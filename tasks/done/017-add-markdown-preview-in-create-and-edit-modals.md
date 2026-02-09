---
id: 17
title: Add markdown preview in create and edit modals
assignee: claude
tags: [frontend, ux, editor]
created: 2026-02-09
completed: 2026-02-09
---

## Description
The create task modal and all edit modes show a raw textarea with no preview. Add a side-by-side or toggle preview so users can see rendered markdown while editing.

The markdown renderer already exists (`renderMarkdown()`), so this is primarily a layout change:
- Split the edit area into textarea + preview pane
- Or add a preview/edit toggle button
- Apply to: task create modal, task edit mode, resource create modal, resource edit mode

## Acceptance Criteria


## Notes
- Added side-by-side split layout: textarea left, rendered preview right
- Applied to all 4 edit contexts: task create, task edit, resource create, resource edit
- Task edit strips frontmatter from preview (shows only body)
- Preview updates on every input event via `renderMarkdown()`
