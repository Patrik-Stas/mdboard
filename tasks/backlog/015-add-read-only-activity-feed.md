---
id: 15
title: Add read-only activity feed
assignee: ""
tags: [frontend, ux]
created: 2026-02-09
---

## Description
Add a timeline/activity feed view showing recent file changes across all resource types (tasks, prompts, reports). This supports the primary use case of observing agent activity.

Could be implemented as:
- A fourth tab showing recent changes sorted by mtime
- Entries like: "Task 012 created", "Prompt 001 updated to rev 3", "Task 007 moved to done"
- Sourced from filesystem timestamps or git log

## Acceptance Criteria


## Notes
