---
id: 8
title: Backend API for prompts and reports
assignee: claude
tags: [backend, prompts, reports, api]
created: 2026-02-08
---

## Description
Add API endpoints for prompts and reports in server.py. Both resource types share the same mechanics: CRUD for the resource itself, plus revision management (create revision, list revisions, view specific revision).

Endpoints needed:
- `GET /api/prompts` — list all prompts, ordered by date
- `POST /api/prompts` — create a new prompt
- `GET /api/prompts/{id}` — get prompt with current content
- `PUT /api/prompts/{id}` — update prompt (creates a new revision automatically)
- `DELETE /api/prompts/{id}` — delete prompt
- `GET /api/prompts/{id}/revisions` — list all revisions
- `GET /api/prompts/{id}/revisions/{rev}` — get specific revision

Same set for `/api/reports`. Consider a shared implementation since the mechanics are identical.

## Acceptance Criteria
- [ ] Prompts CRUD endpoints working
- [ ] Reports CRUD endpoints working
- [ ] Updating a resource creates a new revision file
- [ ] Revision list returns all revisions ordered chronologically
- [ ] Individual revision retrieval works
- [ ] Resources listed in date order

## Notes
