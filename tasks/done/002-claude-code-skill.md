---
id: 2
title: Create Claude Code skill for board operation
assignee: claude
tags: [distribution, ai, skill]
created: 2026-02-08
completed: 2026-02-08
---

## Description
Create a Claude Code skill that teaches agents how to operate the board. This should be a project-level skill at `.claude/skills/mdboard/SKILL.md` that gets picked up automatically when Claude Code works in a project using mdboard.

The skill should instruct the agent on the workflow: scanning for assigned tasks, moving files between columns, checking off acceptance criteria, appending notes, and creating new tasks for discovered work.

## Acceptance Criteria
- [x] `.claude/skills/mdboard/SKILL.md` exists with proper YAML frontmatter
- [x] Skill describes the full task lifecycle (pick up, work, complete, discover)
- [x] Skill explains the file format and directory structure
- [x] Skill is user-invocable (e.g. `/mdboard` to get instructions)
- [x] `mdboard init` copies the skill into the target project's `.claude/skills/`

## Notes
Skill template embedded in `src/mdboard/init.py` as `DEFAULT_SKILL` constant. Uses Claude Code
skill frontmatter: `name: mdboard` (makes it `/mdboard` invocable), `description` for auto-discovery.
Content is concise and actionable — structure, file format, and step-by-step workflow. Defaults to
`user-invocable: true` and model-invocable so Claude sees the description automatically at session start.
