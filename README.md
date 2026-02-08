# .mdboard

A lightweight kanban board where every task is a `.md` file in your repo.

Built for solo developers and AI-assisted workflows. No database, no SaaS, no dependencies -- just markdown files that live alongside your code.

## Why not a cloud board?

Cloud project management tools are built for teams coordinating across time zones on work that stretches over days and weeks. But when it's you and your AI agents on one machine, shipping tasks that take minutes to hours, a cloud subscription is overhead. You're paying for collaboration features you don't use, managing another account per project, and context-switching to a browser tab that's disconnected from the code you're actually working on.

**mdboard puts the board in the repo.** Tasks are markdown files checked into git. No subscription, no sync issues, no per-project setup. Clone the repo and the board is already there.

**The board travels with the branch.** Your `main` branch has a roadmap. Your `experiment/new-parser` branch has its own set of tasks that don't exist on main. When you merge, the task history merges too. When you abandon the branch, the tasks disappear with it. The `tasks/` directory becomes a built-in audit trail of what was planned, what was done, and what was left behind -- scoped to the exact line of work.

**Markdown is the universal interface.** Humans read it. LLMs read it. Git diffs it. Your editor previews it. No proprietary format, no API to learn -- just files.

**AI agents need a local work surface.** Give an agent a `tasks/` directory and it can create work items, track progress, check off acceptance criteria, and leave an audit trail -- all through basic file operations. No auth tokens, no external services, no context window wasted on API docs.

## Quick start

```bash
uvx mdboard init    # scaffold tasks/ directory and config
uvx mdboard         # start the board
```

Open [http://localhost:8080](http://localhost:8080).

No install required -- `uvx` runs directly from PyPI. You just need [uv](https://docs.astral.sh/uv/getting-started/installation/).

## How it works

Tasks are markdown files with YAML frontmatter. Columns are directories. Moving a task = moving a file.

```
tasks/
├── backlog/
├── todo/
│   └── 002-setup-ci-pipeline.md
├── in-progress/
│   └── 004-add-api-rate-limiting.md
├── review/
├── done/
│   └── 001-implement-user-authentication.md
└── comments/
```

A task file:

```markdown
---
id: 2
title: Setup CI pipeline
assignee: claude
tags: [devops, ci]
created: 2026-02-08
branch: feature/ci
---

## Description
Configure GitHub Actions for linting, testing, and deployment.

## Acceptance Criteria
- [ ] Lint on every push
- [x] Run test suite on every PR
- [ ] Auto-deploy to staging on merge to main

## Notes
Decided to use composite actions for reusability.
```

You can manage tasks from the web UI, from the command line, or from an AI agent -- they're all just reading and writing files.

```bash
# move a task to in-progress
mv tasks/todo/002-setup-ci-pipeline.md tasks/in-progress/

# create a task from the command line
cat > tasks/backlog/007-fix-login-bug.md << 'EOF'
---
id: 7
title: Fix login bug
assignee: claude
tags: [bug]
created: 2026-02-08
---

## Description
Login fails when password contains special characters.
EOF
```

## AI agent workflow

The board is designed to be operated by AI coding agents. `mdboard init` installs a Claude Code skill at `.claude/skills/mdboard/SKILL.md` that teaches agents how to operate the board. Invoke it with `/mdboard` in Claude Code, or let the agent discover it automatically.

A typical agent workflow:

1. Agent scans `tasks/todo/` for tasks assigned to it
2. Moves the task file to `tasks/in-progress/`
3. Does the work, checking off acceptance criteria as it goes
4. Appends notes about decisions and changes made
5. Moves the task to `tasks/done/` and adds `completed: 2026-02-08` to frontmatter
6. Creates new task files in `tasks/backlog/` for anything discovered along the way

Everything is committed alongside the code changes. The git log tells you what was done and why.

## Features

- **Zero dependencies** -- Python standard library only
- **Git-native** -- tasks are version-controlled with your code
- **Branch-aware** -- each branch carries its own task state
- **Drag-and-drop UI** -- web-based kanban board
- **Filtering** -- by assignee or tag
- **Inline editing** -- edit raw markdown in the browser
- **Checkbox tracking** -- card progress from `- [x]` items
- **Comments** -- per-task comment threads, also stored as markdown
- **Keyboard shortcuts** -- `N` new task, `Esc` close

## Server options

```bash
mdboard --port 3000           # custom port
mdboard --tasks-dir ./other   # different tasks directory
```

## API

JSON API for scripting:

| Method | Endpoint | Description |
|---|---|---|
| `GET` | `/api/board` | Full board state |
| `POST` | `/api/task` | Create a task |
| `PUT` | `/api/task/{col}/{file}` | Update a task |
| `DELETE` | `/api/task/{col}/{file}` | Delete a task |
| `PATCH` | `/api/task/move` | Move between columns |
| `GET/POST/DELETE` | `/api/comments/{id}` | Task comments |

## Development

```bash
git clone https://github.com/yourusername/mdboard.git
cd mdboard
uv run mdboard
```

## Requirements

Python 3.12+
