"""mdboard init — scaffold a tasks/ directory in the current project."""

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
"""

COLUMNS = ["backlog", "todo", "in-progress", "review", "done"]

DEFAULT_SKILL = """\
---
name: mdboard
description: Operate the markdown task board. Use at session start to scan for assigned tasks, and when creating or managing tasks, prompts, or documents.
---

## mdboard

This project uses mdboard — a markdown-based project management tool with tasks, prompts, and documents. Run `uvx mdboard` to view the web UI.

### Tasks

Tasks live in `tasks/{column}/{id:03d}-{slug}.md` as markdown files with YAML frontmatter.

Columns: `backlog/` → `todo/` → `in-progress/` → `review/` → `done/` (defined in `tasks/config.yaml`).

Task file format:
```
---
id: {number}
title: {title}
assignee: claude
tags: [{tags}]
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
1. Scan `tasks/backlog/`, `tasks/todo/`, and `tasks/in-progress/` for tasks where `assignee: claude`
2. If a task has `branch: X`, only pick it up when on that branch
3. Move task to in-progress: `mv tasks/todo/XXX.md tasks/in-progress/`
4. Work on it — check off `- [ ]` items in Acceptance Criteria as you go
5. Append notes under `## Notes` with what you did and decisions made
6. When complete: add `completed: YYYY-MM-DD` to frontmatter, then `mv tasks/in-progress/XXX.md tasks/done/`
7. If you discover bugs or new work, create task files in `tasks/backlog/`
8. Commit task file changes alongside code changes

### Prompts & Documents

Prompts and documents are revision-tracked markdown resources for storing reusable prompts, reports, specs, and other project knowledge.

- **Prompts**: `prompts/{id:03d}-{slug}/current.md` — reusable prompt templates
- **Documents**: `documents/{id:03d}-{slug}/current.md` — reports, specs, research, decisions

Each resource is a directory containing `current.md` (the latest version) and a `revisions/` subdirectory with numbered snapshots (`001.md`, `002.md`, etc.) created automatically on each edit.

Resource file format:
```
---
id: {number}
title: {title}
created: YYYY-MM-DD
updated: YYYY-MM-DD
revision: {number}
tags: [{tags}]
---

{markdown content}
```

#### Creating a prompt or document
1. Pick the next available ID in the directory (check existing `{id:03d}-*` folders)
2. Create the directory: `prompts/{id:03d}-{slug}/` or `documents/{id:03d}-{slug}/`
3. Write `current.md` with frontmatter (revision: 1) and body content
4. Create `revisions/001.md` as the initial snapshot (same content as current.md)

#### When to use prompts vs documents
- **Prompts**: templates you'll reuse — code review checklists, PR templates, analysis frameworks
- **Documents**: one-off or evolving content — research findings, architecture decisions, meeting notes, reports
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

    # Create prompts/ and reports/ directories
    for resource_dir in ("prompts", "documents"):
        rdir = root / resource_dir
        if rdir.exists():
            skipped.append(f"{resource_dir}/")
        else:
            rdir.mkdir(parents=True, exist_ok=True)
            created.append(f"{resource_dir}/")

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
