---
id: 7
title: Design file format for prompts and reports with revision tracking
assignee: claude
tags: [design, prompts, reports]
created: 2026-02-08
---

## Description
Define the directory structure and file format for two new resource types: prompts and reports. Both are markdown files with revision tracking — each revision stored as a separate file so the audit trail doesn't require digging through git history.

Key decisions:
- Directory layout (e.g. `prompts/{id}-{slug}/` with revisions inside, or flat with naming conventions)
- Revision file naming (timestamps, sequential numbers, or both)
- Frontmatter schema for resources (title, created, updated, current revision number)
- Frontmatter schema for revisions (revision number, created, author, change summary)
- How "current" version relates to revisions (symlink, copy, or latest-wins)

Prompts and reports share the same mechanics — design should be generic enough to support both with minimal duplication.

## Acceptance Criteria
- [ ] Directory structure documented
- [ ] Resource frontmatter schema defined
- [ ] Revision frontmatter schema defined
- [ ] Relationship between current version and revisions is clear
- [ ] Design written up in a notes file or task notes

## Notes
