---
id: 5
title: Publish to PyPI
assignee: ""
tags: [distribution, release]
created: 2026-02-08
---

## Description
Once packaging is complete and tested, publish the first release to PyPI so `uvx mdboard` works for anyone.

## Acceptance Criteria
- [ ] Register `mdboard` on PyPI
- [ ] `uv build` produces clean sdist and wheel
- [ ] `uv publish` uploads successfully
- [ ] `uvx mdboard` works from a clean environment
- [ ] `uvx mdboard init` scaffolds correctly

## Notes
Blocked until packaging tasks are done. Requires PyPI account and API token.
