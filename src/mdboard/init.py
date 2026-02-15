"""mdboard init — scaffold .mdboard/ directory in the current project."""

from __future__ import annotations

import shutil
from pathlib import Path

DEFAULT_CONFIG = """\
columns:
  - name: backlog
    label: "Backlog"
    color: "#6b7280"
  - name: todo
    label: "To Do"
    color: "#3b82f6"
  - name: in-progress
    label: "In Progress"
    color: "#f59e0b"
  - name: review
    label: "Review"
    color: "#8b5cf6"
  - name: done
    label: "Done"
    color: "#10b981"

settings:
  auto_increment_id: true
  default_column: backlog

scopes: [global]
"""

COLUMNS = ["backlog", "todo", "in-progress", "review", "done"]

DEFAULT_SKILL = """\
---
name: mdboard
description: Operate the markdown task board. Use at session start to scan for assigned tasks, and when creating or managing tasks, prompts, or documents.
---

## mdboard

This project uses mdboard — a markdown-based project management tool. Run `uvx mdboard` to view the web UI.

### Directory Structure

ALL mdboard data lives under `.mdboard/` in the project root. Nothing else. Here is the complete layout:

```
.mdboard/
├── port.json                          # runtime: {"port": N, "pid": P} — written by server
├── tasks/                             # TASKS — kanban board
│   ├── config.yaml                    # column definitions, board settings
│   ├── backlog/                       # column directories (one per column)
│   │   └── 001-my-task.md             # task files
│   ├── todo/
│   ├── in-progress/
│   ├── review/
│   ├── done/
│   └── comments/                      # task comments
│       └── {task_id}/                  # one dir per task
│           └── 20260210-143000-author.md
├── prompts/                           # PROMPTS — reusable templates
│   └── 001-my-prompt/                 # one dir per prompt
│       ├── current.md                 # latest version
│       └── revisions/                 # auto-created on each edit
│           ├── 001.md
│           └── 002.md
└── documents/                         # DOCUMENTS — reports, specs, research
    └── 001-my-doc/                    # one dir per document
        ├── current.md                 # latest version
        └── revisions/
            ├── 001.md
            └── 002.md
```

Key facts:
- `config.yaml` lives inside `.mdboard/tasks/`, NOT at `.mdboard/` root
- Prompts and documents are siblings of `tasks/` — all three are direct children of `.mdboard/`
- There is NO `documents/` or `prompts/` directory inside `tasks/` — they are separate
- `port.json` is the only file at the `.mdboard/` root level (auto-generated, not committed)

### Port Discovery

The server auto-assigns a port (range 10600-10700) and writes `.mdboard/port.json` on startup. To find the running instance: `cat .mdboard/port.json` → `{"port": N, "pid": P}`.

### Tasks

Tasks live in `.mdboard/tasks/{column}/{id:03d}-{slug}.md` as markdown files with YAML frontmatter.

Columns: `backlog/` → `todo/` → `in-progress/` → `review/` → `done/` (defined in `.mdboard/tasks/config.yaml`).

Task file format:
```
---
id: {number}
title: {title}
assignee: claude
scopes: [{scopes}]
created: YYYY-MM-DD
due: YYYY-MM-DD          # optional
branch: {branch-name}    # optional, only pick up when on this branch
completed: YYYY-MM-DD    # set when moving to done
---

## Description
{what needs to be done}

## Acceptance Criteria
- [ ] {criterion}

## Notes
{append decisions and progress here}
```

Filenames are zero-padded: `001-slug.md`, `002-slug.md`. IDs auto-increment across all columns.

#### Task workflow
1. Scan `.mdboard/tasks/backlog/`, `.mdboard/tasks/todo/`, and `.mdboard/tasks/in-progress/` for tasks where `assignee: claude`
2. If a task has `branch: X`, only pick it up when on that branch
3. Move task to in-progress: `mv .mdboard/tasks/todo/XXX.md .mdboard/tasks/in-progress/`
4. Work on it — check off `- [ ]` items in Acceptance Criteria as you go
5. Append notes under `## Notes` with what you did and decisions made
6. When complete: add `completed: YYYY-MM-DD` to frontmatter, then `mv .mdboard/tasks/in-progress/XXX.md .mdboard/tasks/done/`
7. If you discover bugs or new work, create task files in `.mdboard/tasks/backlog/`
8. Commit task file changes alongside code changes

### Prompts & Documents

Prompts and documents are revision-tracked markdown resources stored as siblings of `tasks/` under `.mdboard/`.

- **Prompts**: `.mdboard/prompts/{id:03d}-{slug}/current.md` — reusable prompt templates
- **Documents**: `.mdboard/documents/{id:03d}-{slug}/current.md` — reports, specs, research, decisions

Each resource is a directory containing `current.md` (the latest version) and a `revisions/` subdirectory with numbered snapshots (`001.md`, `002.md`, etc.) created automatically on each edit.

Resource file format:
```
---
id: {number}
title: {title}
created: YYYY-MM-DD
updated: YYYY-MM-DD
revision: {number}
scopes: [{scopes}]
---

{markdown content}
```

#### Creating a prompt or document
1. Pick the next available ID in the directory (check existing `{id:03d}-*` folders)
2. Create the directory: `.mdboard/prompts/{id:03d}-{slug}/` or `.mdboard/documents/{id:03d}-{slug}/`
3. Write `current.md` with frontmatter (revision: 1) and body content
4. Create `revisions/001.md` as the initial snapshot (same content as current.md)

#### Scopes

Scopes are a bounded set of work area labels used to categorize tasks, prompts, and documents. They replace the old "tags" concept with a more intentional grouping.

Guidelines:
- Prefer existing scopes over creating new ones
- Keep the scope set small and intentional (e.g., api, ui, shared, testing, infra)
- Examples: microservice names, feature areas, or cross-cutting concerns
- Tasks can have multiple scopes but avoid overspecifying
- Scopes are filterable via clickable badges in the board UI

#### When to use prompts vs documents
- **Prompts**: templates you'll reuse — code review checklists, PR templates, analysis frameworks
- **Documents**: one-off or evolving content — research findings, architecture decisions, meeting notes, reports
"""


def migrate_legacy_dirs(root: Path) -> bool:
    """Migrate legacy top-level tasks/, prompts/, documents/ into .mdboard/.

    Returns True if any migration occurred.
    """
    data_dir = root / ".mdboard"
    migrated = []

    for name in ("tasks", "prompts", "documents"):
        legacy = root / name
        target = data_dir / name
        if not legacy.is_dir():
            continue
        if target.exists():
            print(f"  Warning: both {name}/ and .mdboard/{name}/ exist — skipping {name}/")
            continue
        data_dir.mkdir(parents=True, exist_ok=True)
        shutil.move(str(legacy), str(target))
        migrated.append(name)

    if migrated:
        print(f"  Migrated to .mdboard/: {', '.join(migrated)}")
    return bool(migrated)


def run_init() -> None:
    """Scaffold a .mdboard/ directory and Claude Code skill in the current project."""
    root = Path.cwd()

    # Migrate old layout if present
    migrate_legacy_dirs(root)

    created = []
    skipped = []

    # Create .mdboard/tasks/ and column subdirectories
    for col in COLUMNS:
        col_dir = root / ".mdboard" / "tasks" / col
        if col_dir.exists():
            skipped.append(f".mdboard/tasks/{col}/")
        else:
            col_dir.mkdir(parents=True, exist_ok=True)
            created.append(f".mdboard/tasks/{col}/")

    # Create .mdboard/prompts/ and .mdboard/documents/ directories
    for resource_dir in ("prompts", "documents"):
        rdir = root / ".mdboard" / resource_dir
        if rdir.exists():
            skipped.append(f".mdboard/{resource_dir}/")
        else:
            rdir.mkdir(parents=True, exist_ok=True)
            created.append(f".mdboard/{resource_dir}/")

    # Create .mdboard/tasks/config.yaml
    config_path = root / ".mdboard" / "tasks" / "config.yaml"
    if config_path.exists():
        skipped.append(".mdboard/tasks/config.yaml")
    else:
        config_path.parent.mkdir(parents=True, exist_ok=True)
        config_path.write_text(DEFAULT_CONFIG)
        created.append(".mdboard/tasks/config.yaml")

    # Write .claude/skills/mdboard/SKILL.md — always update to latest
    skill_path = root / ".claude" / "skills" / "mdboard" / "SKILL.md"
    skill_path.parent.mkdir(parents=True, exist_ok=True)
    existing_skill = skill_path.read_text() if skill_path.exists() else ""
    if existing_skill == DEFAULT_SKILL:
        skipped.append(".claude/skills/mdboard/SKILL.md")
    else:
        skill_path.write_text(DEFAULT_SKILL)
        label = "~ .claude/skills/mdboard/SKILL.md" if existing_skill else ".claude/skills/mdboard/SKILL.md"
        created.append(label)

    # Print summary
    if created:
        print("Created:")
        for item in created:
            print(f"  + {item}")
    if skipped:
        print("Already exists:")
        for item in skipped:
            print(f"  - {item}")
    if not created and skipped:
        print("\nNothing to do — board already initialized.")
    elif created:
        print(f"\nBoard initialized. Run `uvx mdboard` to start.")
