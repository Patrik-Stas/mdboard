"""Local markdown-based project management board — HTTP server.

Zero dependencies beyond the Python standard library.
Reads/writes tasks/ directory: directories = columns, .md files = tasks.
"""

import http.server
import json
import os
import re
import shutil
import socketserver
from datetime import date, datetime
from importlib.resources import files
from pathlib import Path
from urllib.parse import unquote


# ---------------------------------------------------------------------------
# YAML-lite parser (handles the subset we need: strings, lists, booleans, ints)
# ---------------------------------------------------------------------------

def parse_yaml_value(raw: str):
    """Parse a single YAML value — string, list, bool, int, or empty."""
    val = raw.strip()
    if not val or val in ('""', "''"):
        return ""
    # Boolean
    if val.lower() in ("true", "yes"):
        return True
    if val.lower() in ("false", "no"):
        return False
    # Inline list  [a, b, c]
    if val.startswith("[") and val.endswith("]"):
        inner = val[1:-1]
        if not inner.strip():
            return []
        return [item.strip().strip('"').strip("'") for item in inner.split(",")]
    # Number
    try:
        return int(val)
    except ValueError:
        pass
    # Strip surrounding quotes
    if (val.startswith('"') and val.endswith('"')) or (val.startswith("'") and val.endswith("'")):
        return val[1:-1]
    return val


def parse_yaml(text: str) -> dict:
    """Minimal YAML parser — flat key: value pairs + nested mappings for config."""
    result = {}
    current_key = None
    current_list = None
    for line in text.splitlines():
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            continue
        indent = len(line) - len(line.lstrip())
        # List item under a key
        if stripped.startswith("- ") and current_key is not None and indent > 0:
            item_val = stripped[2:].strip()
            # Could be a mapping item (key: value on same line or subsequent lines)
            if ":" in item_val:
                # Inline mapping  - name: foo
                mapping = {}
                # Parse first pair
                parts = item_val.split(":", 1)
                mapping[parts[0].strip()] = parse_yaml_value(parts[1])
                if current_list is None:
                    current_list = []
                current_list.append(mapping)
                result[current_key] = current_list
            else:
                if current_list is None:
                    current_list = []
                current_list.append(parse_yaml_value(item_val))
                result[current_key] = current_list
            continue
        # Continuation key: value inside a list-item mapping
        if current_list and indent >= 4 and ":" in stripped and not stripped.startswith("- "):
            k, v = stripped.split(":", 1)
            current_list[-1][k.strip()] = parse_yaml_value(v)
            result[current_key] = current_list
            continue
        # Top-level or nested key: value
        if ":" in stripped:
            k, v = stripped.split(":", 1)
            k = k.strip()
            v = v.strip()
            if not v:
                # Section header
                current_key = k
                current_list = None
                if k not in result:
                    result[k] = {}
            else:
                if indent > 0 and current_key and isinstance(result.get(current_key), dict):
                    result[current_key][k] = parse_yaml_value(v)
                else:
                    result[k] = parse_yaml_value(v)
                    current_key = None
                    current_list = None
    return result


def dump_yaml_value(val) -> str:
    """Serialize a value back to YAML-ish string."""
    if isinstance(val, bool):
        return "true" if val else "false"
    if isinstance(val, list):
        items = ", ".join(str(v) for v in val)
        return f"[{items}]"
    if val is None or val == "":
        return '""'
    return str(val)


# ---------------------------------------------------------------------------
# Frontmatter helpers
# ---------------------------------------------------------------------------

def parse_frontmatter(content: str) -> tuple[dict, str]:
    """Split markdown content into (frontmatter_dict, body_str)."""
    if not content.startswith("---"):
        return {}, content
    parts = content.split("---", 2)
    if len(parts) < 3:
        return {}, content
    fm = parse_yaml(parts[1])
    body = parts[2].lstrip("\n")
    return fm, body


def build_frontmatter(meta: dict) -> str:
    """Serialize a dict back to YAML frontmatter block."""
    lines = ["---"]
    # Preserve a sensible key order
    key_order = ["id", "title", "assignee", "tags", "created", "due", "branch", "completed"]
    seen = set()
    for k in key_order:
        if k in meta:
            lines.append(f"{k}: {dump_yaml_value(meta[k])}")
            seen.add(k)
    for k, v in meta.items():
        if k not in seen:
            lines.append(f"{k}: {dump_yaml_value(v)}")
    lines.append("---")
    return "\n".join(lines)


def slugify(text: str) -> str:
    slug = text.lower().strip()
    slug = re.sub(r"[^\w\s-]", "", slug)
    slug = re.sub(r"[\s_]+", "-", slug)
    slug = re.sub(r"-+", "-", slug).strip("-")
    return slug


# ---------------------------------------------------------------------------
# Board operations
# ---------------------------------------------------------------------------

class Board:
    def __init__(self, tasks_dir: str):
        self.root = Path(tasks_dir).resolve()
        self.config = self._load_config()
        self._ensure_columns()

    def _load_config(self) -> dict:
        cfg_path = self.root / "config.yaml"
        if cfg_path.exists():
            return parse_yaml(cfg_path.read_text())
        return {
            "columns": [
                {"name": "backlog", "label": "Backlog", "color": "#6b7280"},
                {"name": "todo", "label": "To Do", "color": "#3b82f6"},
                {"name": "in-progress", "label": "In Progress", "color": "#f59e0b"},
                {"name": "review", "label": "Review", "color": "#8b5cf6"},
                {"name": "done", "label": "Done", "color": "#10b981"},
            ],
            "settings": {"auto_increment_id": True, "default_column": "backlog"},
        }

    def _ensure_columns(self):
        for col in self.columns():
            (self.root / col["name"]).mkdir(parents=True, exist_ok=True)

    def columns(self) -> list[dict]:
        cols = self.config.get("columns", [])
        if isinstance(cols, list):
            return cols
        return []

    def column_names(self) -> list[str]:
        return [c["name"] for c in self.columns()]

    def settings(self) -> dict:
        s = self.config.get("settings", {})
        return s if isinstance(s, dict) else {}

    def _next_id(self) -> int:
        max_id = 0
        for col in self.column_names():
            col_dir = self.root / col
            if not col_dir.is_dir():
                continue
            for f in col_dir.glob("*.md"):
                match = re.match(r"(\d+)-", f.name)
                if match:
                    max_id = max(max_id, int(match.group(1)))
        return max_id + 1

    def get_board(self) -> dict:
        board = {"columns": []}
        for col in self.columns():
            col_dir = self.root / col["name"]
            tasks = []
            if col_dir.is_dir():
                for f in sorted(col_dir.glob("*.md")):
                    fm, body = parse_frontmatter(f.read_text())
                    tasks.append({
                        "filename": f.name,
                        "meta": fm,
                        "body": body,
                        "column": col["name"],
                    })
            board["columns"].append({
                **col,
                "tasks": tasks,
            })
        return board

    def get_task(self, column: str, filename: str) -> dict | None:
        path = self.root / column / filename
        if not path.exists():
            return None
        fm, body = parse_frontmatter(path.read_text())
        return {"filename": filename, "column": column, "meta": fm, "body": body}

    def create_task(self, data: dict) -> dict:
        col = data.get("column", self.settings().get("default_column", "backlog"))
        if col not in self.column_names():
            col = self.column_names()[0] if self.column_names() else "backlog"
        (self.root / col).mkdir(parents=True, exist_ok=True)

        task_id = self._next_id()
        title = data.get("title", "Untitled")
        slug = slugify(title)
        filename = f"{task_id:03d}-{slug}.md"

        meta = {
            "id": task_id,
            "title": title,
            "assignee": data.get("assignee", ""),
            "tags": data.get("tags", []),
            "created": str(date.today()),
        }
        if data.get("due"):
            meta["due"] = data["due"]
        if data.get("branch"):
            meta["branch"] = data["branch"]

        description = data.get("description", "")
        body = f"\n## Description\n{description}\n\n## Acceptance Criteria\n\n\n## Notes\n"

        content = build_frontmatter(meta) + "\n" + body
        (self.root / col / filename).write_text(content)
        return {"filename": filename, "column": col, "meta": meta, "body": body}

    def update_task(self, column: str, filename: str, data: dict) -> dict | None:
        path = self.root / column / filename
        if not path.exists():
            return None
        # If raw content provided, write directly
        if "content" in data:
            path.write_text(data["content"])
            fm, body = parse_frontmatter(data["content"])
            return {"filename": filename, "column": column, "meta": fm, "body": body}
        # Otherwise update meta fields
        fm, body = parse_frontmatter(path.read_text())
        for key in ("title", "assignee", "tags", "due", "branch", "completed"):
            if key in data:
                fm[key] = data[key]
        if "body" in data:
            body = data["body"]
        content = build_frontmatter(fm) + "\n" + body
        path.write_text(content)
        return {"filename": filename, "column": column, "meta": fm, "body": body}

    def move_task(self, filename: str, from_col: str, to_col: str) -> bool:
        src = self.root / from_col / filename
        if not src.exists():
            return False
        dst_dir = self.root / to_col
        dst_dir.mkdir(parents=True, exist_ok=True)
        shutil.move(str(src), str(dst_dir / filename))
        return True

    def delete_task(self, column: str, filename: str) -> bool:
        path = self.root / column / filename
        if not path.exists():
            return False
        path.unlink()
        return True

    def task_count(self) -> int:
        total = 0
        for col in self.column_names():
            col_dir = self.root / col
            if col_dir.is_dir():
                total += len(list(col_dir.glob("*.md")))
        return total

    # ── Comments ──

    def _comments_dir(self, task_id: int) -> Path:
        return self.root / "comments" / str(task_id)

    def get_comments(self, task_id: int) -> list[dict]:
        cdir = self._comments_dir(task_id)
        if not cdir.is_dir():
            return []
        comments = []
        for f in sorted(cdir.glob("*.md")):
            fm, body = parse_frontmatter(f.read_text())
            comments.append({"filename": f.name, "meta": fm, "body": body})
        return comments

    def add_comment(self, task_id: int, data: dict) -> dict:
        cdir = self._comments_dir(task_id)
        cdir.mkdir(parents=True, exist_ok=True)
        ts = datetime.now().strftime("%Y%m%d-%H%M%S")
        author = data.get("author", "anonymous")
        slug = slugify(author)
        filename = f"{ts}-{slug}.md"
        meta = {
            "author": author,
            "created": datetime.now().strftime("%Y-%m-%d %H:%M"),
        }
        body = data.get("body", "")
        content = build_frontmatter(meta) + "\n" + body + "\n"
        (cdir / filename).write_text(content)
        return {"filename": filename, "meta": meta, "body": body}

    def delete_comment(self, task_id: int, filename: str) -> bool:
        path = self._comments_dir(task_id) / filename
        if not path.exists():
            return False
        path.unlink()
        return True


# ---------------------------------------------------------------------------
# HTTP Handler
# ---------------------------------------------------------------------------

class BoardHandler(http.server.BaseHTTPRequestHandler):
    board: Board
    html_path: object  # Path or importlib.resources Traversable

    def log_message(self, fmt, *args):
        # Quieter logging
        pass

    def _send_json(self, data, status=200):
        body = json.dumps(data, default=str).encode()
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def _send_html(self, path: Path):
        content = path.read_bytes()
        self.send_response(200)
        self.send_header("Content-Type", "text/html; charset=utf-8")
        self.send_header("Content-Length", str(len(content)))
        self.end_headers()
        self.wfile.write(content)

    def _send_error(self, status, msg):
        self._send_json({"error": msg}, status)

    def _read_body(self) -> dict:
        length = int(self.headers.get("Content-Length", 0))
        if length == 0:
            return {}
        raw = self.rfile.read(length)
        return json.loads(raw)

    def _route(self, method: str):
        path = unquote(self.path)

        # Serve frontend
        if method == "GET" and path == "/":
            return self._send_html(self.html_path)

        # API routes
        if path == "/api/board" and method == "GET":
            return self._send_json(self.board.get_board())

        if path == "/api/config" and method == "GET":
            return self._send_json(self.board.config)

        if path == "/api/task" and method == "POST":
            data = self._read_body()
            result = self.board.create_task(data)
            return self._send_json(result, 201)

        if path == "/api/task/move" and method == "PATCH":
            data = self._read_body()
            ok = self.board.move_task(data["filename"], data["from_column"], data["to_column"])
            if ok:
                return self._send_json({"ok": True})
            return self._send_error(404, "Task not found")

        # /api/comments/{task_id}
        cm = re.match(r"^/api/comments/(\d+)$", path)
        if cm:
            task_id = int(cm.group(1))
            if method == "GET":
                return self._send_json(self.board.get_comments(task_id))
            if method == "POST":
                data = self._read_body()
                result = self.board.add_comment(task_id, data)
                return self._send_json(result, 201)

        # /api/comments/{task_id}/{filename}
        cm2 = re.match(r"^/api/comments/(\d+)/([^/]+)$", path)
        if cm2:
            task_id = int(cm2.group(1))
            filename = cm2.group(2)
            if method == "DELETE":
                ok = self.board.delete_comment(task_id, filename)
                if ok:
                    return self._send_json({"ok": True})
                return self._send_error(404, "Comment not found")

        # /api/task/{column}/{filename}
        m = re.match(r"^/api/task/([^/]+)/([^/]+)$", path)
        if m:
            column, filename = m.group(1), m.group(2)
            if method == "GET":
                task = self.board.get_task(column, filename)
                if task:
                    return self._send_json(task)
                return self._send_error(404, "Task not found")
            if method == "PUT":
                data = self._read_body()
                result = self.board.update_task(column, filename, data)
                if result:
                    return self._send_json(result)
                return self._send_error(404, "Task not found")
            if method == "DELETE":
                ok = self.board.delete_task(column, filename)
                if ok:
                    return self._send_json({"ok": True})
                return self._send_error(404, "Task not found")

        self._send_error(404, "Not found")

    def do_GET(self):
        self._route("GET")

    def do_POST(self):
        self._route("POST")

    def do_PUT(self):
        self._route("PUT")

    def do_PATCH(self):
        self._route("PATCH")

    def do_DELETE(self):
        self._route("DELETE")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def run_server(port: int = 8080, tasks_dir: str = "tasks") -> None:
    """Start the HTTP server."""
    tasks_path = Path(tasks_dir).resolve()

    # Auto-scaffold on first run
    if not tasks_path.exists():
        from mdboard.init import run_init
        run_init()

    html_path = files("mdboard").joinpath("_assets", "index.html")

    board = Board(str(tasks_path))

    BoardHandler.board = board
    BoardHandler.html_path = html_path

    # Allow port reuse
    socketserver.TCPServer.allow_reuse_address = True

    with socketserver.TCPServer(("", port), BoardHandler) as httpd:
        col_count = len(board.column_names())
        task_count = board.task_count()
        print(f"")
        print(f"  mdboard")
        print(f"  ─────────────────────────────────")
        print(f"  URL:      http://localhost:{port}")
        print(f"  Tasks:    {tasks_path}")
        print(f"  Columns:  {col_count}    Tasks: {task_count}")
        print(f"  ─────────────────────────────────")
        print(f"")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nShutting down.")


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description="mdboard server")
    parser.add_argument("--port", type=int, default=8080)
    parser.add_argument("--tasks-dir", default="tasks")
    args = parser.parse_args()
    run_server(port=args.port, tasks_dir=args.tasks_dir)
