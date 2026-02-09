---
id: 14
title: Replace revision snapshots with git log view
assignee: claude
tags: [backend, frontend, diff]
created: 2026-02-09
completed: 2026-02-09
---

## Description
The current revision system saves full-file snapshots on every edit, which:
- Duplicates storage linearly with edits
- Has no diff view (only full snapshots side-by-side)
- Reinvents version control poorly

Consider replacing the revisions/ directory approach with a git-based history view. The UI could show `git log` for a specific file and render diffs between versions. This leverages what git already does well.

Alternative: keep snapshots but add a diff view between consecutive revisions.

## Acceptance Criteria


## Notes
- Chose the "alternative" approach: kept snapshot system, added diff view
- When viewing a revision, a content/diff toggle appears in the banner
- Diff uses LCS algorithm implemented in JS (no deps), shows +/- lines with color coding
- Compares revision body against current body
- Green = added in current, red = removed from revision
