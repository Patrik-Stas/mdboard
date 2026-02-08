---
id: 3
title: Implement `mdboard init` scaffolding command
assignee: claude
tags: [cli, distribution]
created: 2026-02-08
completed: 2026-02-08
---

## Description
Add an `init` subcommand that bootstraps a project with the mdboard structure. Running `uvx mdboard init` in any project directory should create the `tasks/` directory, default config, column directories, and optionally the Claude Code skill.

## Acceptance Criteria
- [x] `mdboard init` creates `tasks/` with default column directories
- [x] Creates `tasks/config.yaml` with default column config
- [x] Copies `.claude/skills/mdboard/SKILL.md` into the project
- [x] Skips files that already exist (safe to re-run)
- [x] Prints a summary of what was created
- [x] Works when run via `uvx mdboard init`

## Notes
Implemented in `src/mdboard/init.py`. Templates for config.yaml and SKILL.md are embedded
as string constants. Tested: fresh init creates all files, re-running skips everything.
Server starts correctly with scaffolded board. Also checked off the remaining criterion
on task 001 (init scaffolding).
