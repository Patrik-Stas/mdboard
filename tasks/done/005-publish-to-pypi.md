---
id: 5
title: Publish to PyPI
assignee: ""
tags: [distribution, release]
created: 2026-02-08
completed: 2026-02-08
---

## Description
Once packaging is complete and tested, publish the first release to PyPI so `uvx mdboard` works for anyone.

## Acceptance Criteria
- [x] Register `mdboard` on PyPI
- [x] `uv build` produces clean sdist and wheel
- [x] `uv publish` uploads successfully
- [x] `uvx mdboard` works from a clean environment
- [x] `uvx mdboard init` scaffolds correctly

## Notes
Published v0.1.0 via `uv publish`. Verified from clean environment: `uvx mdboard init` scaffolds
tasks/, config.yaml, and .claude/skills/mdboard/SKILL.md. `uvx mdboard` starts server, UI and API
both functional.
