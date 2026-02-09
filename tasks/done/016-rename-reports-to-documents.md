---
id: 16
title: Rename Reports to Documents
assignee: claude
tags: [naming, ux]
created: 2026-02-09
completed: 2026-02-09
---

## Description
"Reports" implies analytical capability (aggregation, data-driven output) that the feature does not have. It is actually a versioned markdown document store.

Renaming to "Documents" sets more accurate expectations and is a more general label that covers specs, notes, design docs, etc.

This touches: directory name, API routes, UI labels, init scaffolding, CLAUDE.md, SKILL.md.

## Acceptance Criteria


## Notes
- Renamed `reports/` directory to `documents/` on disk
- Updated server.py: ResourceStore instance name, API route prefix `/api/documents`
- Updated init.py: scaffolds `documents/` instead of `reports/`
- Updated index.html: all UI labels, JS variables, hash routing
- Updated MEMORY.md
