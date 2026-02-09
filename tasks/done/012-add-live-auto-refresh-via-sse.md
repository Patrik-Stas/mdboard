---
id: 12
title: Add live auto-refresh via SSE
assignee: claude
tags: [frontend, ux, polling]
created: 2026-02-09
completed: 2026-02-09
---

## Description
The board does not update when files change on disk. If an agent creates a task while the user is viewing the board, nothing happens until manual reload.

Add Server-Sent Events (SSE) or polling so the board, prompts, and reports views refresh automatically when underlying files change. This is critical since the primary use case is watching agent activity in real time.

Options:
- SSE endpoint that watches filesystem (inotify/kqueue)
- Simple polling (fetch board state every N seconds)
- Hybrid: poll with smart diffing to avoid full re-renders

## Acceptance Criteria


## Notes
- Chose polling approach (zero-dep, simpler than SSE with filesystem watchers)
- Backend: `/api/poll` returns MD5 hashes of file names + mtimes for board, prompts, documents
- Frontend: polls every 3 seconds, compares hashes, re-fetches only changed views
- First poll initializes hashes without triggering reload
