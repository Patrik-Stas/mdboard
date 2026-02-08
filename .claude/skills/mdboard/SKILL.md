---
name: mdboard
description: Operate the markdown task board. Use at session start to scan for assigned tasks, and when creating or managing tasks in tasks/.
---

## Task Board

This project uses mdboard — a markdown-based task board in `tasks/`. Run `uvx mdboard` to view the web UI.

### Structure
- `tasks/{column}/{id:03d}-{slug}.md` — each task is a markdown file
- Columns: `backlog/` → `todo/` → `in-progress/` → `review/` → `done/`
- Config in `tasks/config.yaml`

### Task file format
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

### Workflow
1. Scan `tasks/backlog/`, `tasks/todo/`, and `tasks/in-progress/` for tasks where `assignee: claude`
2. If a task has `branch: X`, only pick it up when on that branch
3. Move task to in-progress: `mv tasks/todo/XXX.md tasks/in-progress/`
4. Work on it — check off `- [ ]` items in Acceptance Criteria as you go
5. Append notes under `## Notes` with what you did and decisions made
6. When complete: add `completed: YYYY-MM-DD` to frontmatter, then `mv tasks/in-progress/XXX.md tasks/done/`
7. If you discover bugs or new work, create task files in `tasks/backlog/`
8. Commit task file changes alongside code changes
