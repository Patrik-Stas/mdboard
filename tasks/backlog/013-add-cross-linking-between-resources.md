---
id: 13
title: Add cross-linking between resources
assignee: ""
tags: [backend, frontend, linking]
created: 2026-02-09
---

## Description
Tasks, prompts, and reports currently exist as isolated silos. In real LLM workflows, relationships matter: a report may be generated using a specific prompt against specific tasks.

Add a `related` frontmatter field that supports cross-references:
```yaml
related: [prompt:001, task:003, report:002]
```

The UI should render these as clickable links in the sidebar. The backend should parse and validate the references.

## Acceptance Criteria


## Notes
