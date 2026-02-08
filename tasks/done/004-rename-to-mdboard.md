---
id: 4
title: Rename project from amtheboard to mdboard
assignee: claude
tags: [packaging, branding]
created: 2026-02-08
completed: 2026-02-08
---

## Description
The PyPI package name should be `mdboard`. Update `pyproject.toml`, README, and any internal references from `amtheboard` to `mdboard`. The repo can stay as-is on GitHub but the distributed package should be `mdboard`.

## Acceptance Criteria
- [x] `pyproject.toml` name is `mdboard`
- [x] README title and references updated
- [x] Python package directory named `mdboard/`
- [x] No remaining references to `amtheboard` in distributed code

## Notes
Done alongside task 001 (packaging). Renamed in pyproject.toml, README.md, and uv.lock.
Package directory is `src/mdboard/`. Verified with grep — no "amtheboard" in src/.
