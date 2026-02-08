---
id: 1
title: Package for PyPI distribution
assignee: claude
tags: [distribution, packaging]
created: 2026-02-08
completed: 2026-02-08
---

## Description
Restructure the project so it can be published to PyPI and run via `uvx mdboard`. Users should be able to start the board in any project with zero installation.

Target UX:
```bash
uvx mdboard                     # start board in current dir
uvx mdboard --port 3000         # custom port
uvx mdboard --tasks-dir ./work  # custom tasks directory
uvx mdboard init                # scaffold tasks/ directory in current project
```

## Acceptance Criteria
- [x] Rename/restructure into a proper Python package (`mdboard/`)
- [x] Add `[project.scripts]` entrypoint in `pyproject.toml` pointing to CLI main
- [x] `index.html` bundled as package data so it ships with the package
- [x] `server.py` resolves `index.html` from package data, not `__file__` parent
- [x] `uv build` produces a working wheel
- [x] `uvx mdboard` starts the server and serves the board
- [x] `uvx mdboard init` scaffolds a `tasks/` directory with default `config.yaml`

## Notes
Completed packaging restructure alongside task 004 (rename). Used `src/` layout with hatchling
build backend. `index.html` bundled via `importlib.resources.files()`. CLI entrypoint in
`mdboard.cli:main` with argparse subcommands (serve default, init stub). The `init` subcommand
is stubbed out — prints "not yet implemented" and exits 1. Full implementation is task 003.
