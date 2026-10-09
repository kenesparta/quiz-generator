# GEMINI.md

The project rules are tool-agnostic and live in AGENTS.md; project facts live in
CLAUDE.md. Gemini CLI inlines both files through the import lines below.
Antigravity CLI reads AGENTS.md on its own and treats the import lines as plain
path references, so nothing is loaded twice.

@./AGENTS.md

@./CLAUDE.md

## Gemini CLI notes

- Subagents live in `.gemini/agents/` (Gemini CLI 0.36 or newer). Use
  `rust-reviewer` for read-only reviews and `rust-engineer` for changes. Force
  one with `@rust-reviewer <task>`.
- Run `/memory show` to check that AGENTS.md and CLAUDE.md are loaded once.
  Gemini CLI does not deduplicate imported files: while the import lines above
  exist, do not also list AGENTS.md in `context.fileName` or import it from
  CLAUDE.md, or it is loaded twice (if CLAUDE.md starts importing AGENTS.md,
  delete the AGENTS.md import line here instead).
- Antigravity CLI (`agy`) uses `.agents/agents/` instead.
