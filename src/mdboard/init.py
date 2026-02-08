"""mdboard init — scaffold a tasks/ directory in the current project."""

from pathlib import Path

DEFAULT_CONFIG = """\
columns:
  - name: backlog
    label: "📋 Backlog"
    color: "#6b7280"
  - name: todo
    label: "📌 To Do"
    color: "#3b82f6"
  - name: in-progress
    label: "🔨 In Progress"
    color: "#f59e0b"
  - name: review
    label: "👀 Review"
    color: "#8b5cf6"
  - name: done
    label: "✅ Done"
    color: "#10b981"

settings:
  auto_increment_id: true
  default_column: backlog
"""

COLUMNS = ["backlog", "todo", "in-progress", "review", "done"]

DEFAULT_SKILL = """\
## Task Management

This project uses a local markdown-based task board in `tasks/`.

### Running the Board

```bash
uvx mdboard [--port PORT] [--tasks-dir DIR]
# → http://localhost:8080 (default)
```

### Structure
- `tasks/{column}/{id:03d}-{slug}.md` — each task is a markdown file (e.g. `001-setup-ci.md`)
- Columns: backlog → todo → in-progress → review → done (defined in `tasks/config.yaml`)
- Files have YAML frontmatter with: id, title, assignee, tags, created, due, branch, completed
- Comments stored in `tasks/comments/{task_id}/` as timestamped markdown files
- Config in `tasks/config.yaml` (columns, colors, `auto_increment_id`, `default_column`)

### Your Workflow
1. At session start, scan `tasks/backlog/`, `tasks/todo/`, and `tasks/in-progress/` for tasks where `assignee: claude`
2. Before starting a task, move it: `mv tasks/todo/XXX.md tasks/in-progress/`
3. Work on the task, checking off acceptance criteria in the file as you go
4. Append notes under `## Notes` with what you did and any decisions made
5. When complete, move to done: `mv tasks/in-progress/XXX.md tasks/done/`
6. Add `completed: YYYY-MM-DD` to the frontmatter
7. If you discover bugs or necessary refactors, create new task files in `tasks/backlog/`
8. Commit task file changes alongside your code changes
9. If a task has `branch: X`, only pick it up when on that branch

### Creating Tasks
Filename format: `{id:03d}-{slug}.md` (zero-padded 3 digits). IDs auto-increment across all columns.

Use this template:
```
---
id: {next_id}
title: {title}
assignee: claude
tags: [{relevant, tags}]
created: {today}
due: {optional, YYYY-MM-DD}
branch: {optional, only if scoped to a branch}
---

## Description
{what needs to be done}

## Acceptance Criteria
- [ ] {criterion}

## Notes
```
"""


def run_init() -> None:
    """Scaffold a tasks/ directory and Claude Code skill in the current project."""
    root = Path.cwd()
    created = []
    skipped = []

    # Create tasks/ and column subdirectories
    for col in COLUMNS:
        col_dir = root / "tasks" / col
        if col_dir.exists():
            skipped.append(f"tasks/{col}/")
        else:
            col_dir.mkdir(parents=True, exist_ok=True)
            created.append(f"tasks/{col}/")

    # Create tasks/config.yaml
    config_path = root / "tasks" / "config.yaml"
    if config_path.exists():
        skipped.append("tasks/config.yaml")
    else:
        config_path.parent.mkdir(parents=True, exist_ok=True)
        config_path.write_text(DEFAULT_CONFIG)
        created.append("tasks/config.yaml")

    # Create .claude/skills/mdboard/SKILL.md
    skill_path = root / ".claude" / "skills" / "mdboard" / "SKILL.md"
    if skill_path.exists():
        skipped.append(".claude/skills/mdboard/SKILL.md")
    else:
        skill_path.parent.mkdir(parents=True, exist_ok=True)
        skill_path.write_text(DEFAULT_SKILL)
        created.append(".claude/skills/mdboard/SKILL.md")

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
