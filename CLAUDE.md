## Task Management

This project uses a local markdown-based task board in `tasks/`. It consists of a zero-dependency Python HTTP server (`server.py`) serving a single-page kanban UI (`index.html`).

### Running the Board

```bash
uv run server.py [--port PORT] [--tasks-dir DIR]
# → http://localhost:8080 (default)
# → tasks dir defaults to ./tasks
```

### Structure
- `tasks/{column}/{id:03d}-{slug}.md` — each task is a markdown file (e.g. `001-pypi-packaging.md`)
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
