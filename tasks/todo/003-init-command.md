---
id: 3
title: Implement `mdboard init` scaffolding command
assignee: claude
tags: [cli, distribution]
created: 2026-02-08
---

## Description
Add an `init` subcommand that bootstraps a project with the mdboard structure. Running `uvx mdboard init` in any project directory should create the `tasks/` directory, default config, column directories, and optionally the Claude Code skill.

## Acceptance Criteria
- [ ] `mdboard init` creates `tasks/` with default column directories
- [ ] Creates `tasks/config.yaml` with default column config
- [ ] Copies `.claude/skills/mdboard/SKILL.md` into the project
- [ ] Skips files that already exist (safe to re-run)
- [ ] Prints a summary of what was created
- [ ] Works when run via `uvx mdboard init`

## Notes
