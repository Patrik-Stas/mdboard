---
id: 1
title: mdboard Evaluation: LLM Workflow Fit
tags: [evaluation, llm-workflows, meta]
created: 2026-02-09
updated: 2026-02-09
revision: 2
---
## Executive Summary



mdboard is a zero-dependency, file-based project management tool that stores tasks as markdown files on disk. This report evaluates its strengths, weaknesses, and — most critically — whether it actually makes sense as infrastructure for LLM-driven workflows.

**Verdict:** The core idea is sound and genuinely useful. The file-on-disk model is the single best design decision here — it makes mdboard one of the few task boards that an LLM agent can natively operate without API wrappers, browser automation, or MCP servers. But the current implementation has gaps that limit its real-world utility.

---

## What Works Well

### 1. Files Are the API

This is the killer feature. Tasks are plain markdown files in directories. An LLM agent (Claude, Cursor, Copilot) can:
- Read tasks with standard file operations
- Create tasks by writing files
- Move tasks between columns with `mv`
- Update status by editing frontmatter

No HTTP client needed. No authentication. No SDK. The filesystem IS the interface. This is a genuinely good insight that most task management tools miss entirely.

### 2. Zero Dependencies

The entire server is stdlib Python. No pip install, no node_modules, no Docker. `uvx mdboard` and you're running. This matters because LLM agents operate in diverse environments — the fewer setup steps, the fewer failure modes.

### 3. Convention Over Configuration

The directory-as-column model, zero-padded IDs, and YAML frontmatter are all simple conventions that an LLM can learn from a single example. The CLAUDE.md + SKILL.md scaffolding teaches the agent the conventions automatically.

### 4. Human-Readable Audit Trail

Every task, prompt, and report is a markdown file you can `cat`, `grep`, or open in any editor. No database to query, no export step. Git diff shows you exactly what changed.

---

## What Doesn't Work Well

### 1. The Web UI Is View-Only in Practice

The web UI exists but it's a secondary citizen. The real workflow is: agent writes files, human views the board. The UI's edit mode gives you raw frontmatter + markdown in a textarea — this is fine for developers but it's not a real editing experience. The create-task modal is basic (no due date picker, no markdown preview).

**Impact:** Humans default to the web UI and find it underwhelming compared to Linear/Notion/Trello. The value proposition only clicks when you realize the files are the point.

### 2. No Real-Time Updates

The board doesn't auto-refresh. If an agent creates a task while you're viewing the board, you see nothing until you reload. For a tool whose primary use case is "watch what the agent is doing," this is a significant gap.

### 3. Prompts & Reports Revision Model Is Awkward

The revision tracking saves full snapshots on every edit. This means:
- Storage grows linearly with edits (every save duplicates the full document)
- There's no diff view — you can only look at full snapshots side-by-side
- The revision is created BEFORE the edit is saved, so revision N contains the content from BEFORE the Nth edit. This is correct but counterintuitive.

Git already does this better. The revision system is reinventing version control poorly.

### 4. No Cross-Linking

Tasks can't reference prompts. Reports can't link to the tasks they summarize. In real LLM workflows, these relationships matter: "this report was generated using prompt #3 against tasks #7-#12." Without linking, the three resource types are isolated silos.

### 5. YAML Parser Is Fragile

The hand-rolled YAML parser handles the happy path but will break on multiline strings, anchors, or any non-trivial YAML. This is fine for machine-generated frontmatter but risky when humans edit files directly.

---

## Does It Make Sense for LLM Workflows?

### Where It Fits Perfectly

**Agent task management.** The CLAUDE.md workflow (scan for assigned tasks, move to in-progress, check off criteria, move to done) is genuinely practical. It gives structure to agent sessions without requiring the agent to interact with external services. This is better than:
- GitHub Issues (requires API calls, authentication)
- Linear/Jira (requires MCP servers or API wrappers)
- TodoWrite-style ephemeral lists (lost between sessions)

**Prompt versioning for iteration.** When you're developing a system prompt through multiple rounds of testing, having numbered revisions on disk is useful. You can `diff` versions, roll back, and the agent can read the prompt file directly.

### Where It Doesn't Fit

**Team collaboration.** There's no multi-user support, no permissions, no notifications. This is a single-developer + agent tool.

**Complex project management.** No dependencies between tasks, no estimates, no sprint planning, no burndown. If you need these, use a real PM tool.

**Report generation.** The reports feature stores markdown documents with revisions, but there's no templating, no data aggregation, no way to auto-generate a report from task data. It's just a versioned markdown store. The name "reports" implies analytical capability that isn't there.

### The Core Tension

mdboard tries to be two things:
1. **A file-based protocol for agent task management** (great idea)
2. **A web-based project management UI** (mediocre execution)

The first use case is where the real value lives. The web UI should be optimized for *observing agent activity*, not for *managing tasks manually*. Think of it less as "Trello but files" and more as "a dashboard for watching your agent work."

---

## Recommendations

1. **Add auto-refresh / SSE** — the board should update live when files change on disk
2. **Add task linking** — `related: [prompt:001, task:003]` in frontmatter
3. **Drop revision snapshots, lean into git** — a `git log` view in the UI would be more useful than the current revision system
4. **Add a read-only "activity feed"** — show recent file changes across tasks/prompts/reports as a timeline
5. **Rename "Reports" to "Documents"** — sets more accurate expectations
6. **Add markdown preview in create/edit modals** — side-by-side editing

