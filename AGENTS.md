# AGENTS.md — Rust service harness

Operating manual for **people and AI coding agents** (Claude Code, Gemini CLI, Antigravity, Codex, Cursor) working on Rust backend services.

It has three tiers, on purpose:

| Tier | Meaning | If it doesn't fit |
|---|---|---|
| **Guardrails** (§2) | Must hold. Each one exists because breaking it caused a real, proven bug. | Change the rule in a reviewed PR — never work around it silently. |
| **Defaults** (§1, §3–§11) | The normal way to do things here. | Deviate, and write the reason in one line (commit message, PR, or a comment next to the code). |
| Everything else | Your judgment. | — |

Project facts (crates, commands, domain glossary) live in `CLAUDE.md` and `README.md`. Detailed Rust patterns live in the `rs-*` skills when they are installed; where a skill disagrees with this file or `CLAUDE.md` (web framework, database, crate layout, English-only names, MSRV), this file and `CLAUDE.md` win. Known problems and the improvement plan live in `docs/CODE_REVIEW_AND_PLAN.md` — **don't copy a pattern that document lists as a problem**.

---

## 1. The working loop

1. **Orient.** Read the code around the change and the closest existing example. Check the plan document for known problems first.
2. **Plan small.** State the goal, the files you will touch, and how you will verify. If the change touches more than ~5 files or a public contract (HTTP API, DB schema, port trait), say so before starting.
3. **Change in small steps that compile.** Keep unrelated refactors out of the change.
4. **Verify** with the loop in §10. A bug fix starts with a test that fails without the fix.
5. **Report:** what changed, why, how it was verified (commands + results), and what is left. **Never claim a check you did not run.**

## 2. Guardrails (must)

| # | Guardrail | Why (evidence from this codebase) |
|---|---|---|
| G1 | `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace` pass before a change is "done" (§10). If a check was already red before your change: add no new failure, list the pre-existing ones (`path:line`) in the report, and don't fix or reformat unrelated files in the same change. | A red baseline hides new problems: 45 warnings and a failing fmt check went unnoticed. |
| G2 | No panic reachable from a request or from stored data: no `unwrap`/`expect`/`panic!`/`todo!`/`unimplemented!`/unchecked indexing outside tests. Startup may fail fast, by returning an error from `main`. The one exception is a call that cannot fail at run time (e.g. a literal regex), marked `#[expect(clippy::expect_used, reason = "...")]`. | One malformed document should be one 4xx/5xx, not a crashed request. |
| G3 | Authorization is **deny-by-default** and declared next to the route. Never derive it by parsing URL strings. | Parsing the raw path let a candidate become admin via `/%61dmins`. |
| G4 | Every read **and write** of user-owned data is scoped to the caller: an owner's request filters by the owner taken from the token (e.g. `postulante_id = claims.sub`), never only by an id from the path or body. Staff roles that may act on other users' data do so through their own endpoint or use case, where that permission is explicit. | Writes filtered only by `_id`; a candidate could answer another candidate's exam. |
| G5 | Secrets and personal data never reach logs, stdout or error bodies (tokens, passwords, hashes, national IDs), and secrets are never committed (deployed environments read them from the environment or a secret store; a git-ignored local file is fine for development). | A stray `print!` wrote every national ID to stdout. |
| G6 | Never block the async executor. CPU-heavy work (password hashing) and blocking IO go to `tokio::task::spawn_blocking` / `web::block`. Every network call has a timeout. | Inline bcrypt (~240 ms) stalled unrelated requests for 1.85 s and caused spurious 500s. |
| G7 | Invariants that must hold across concurrent requests are enforced by the database (unique index, conditional update, transaction) — never by "check, then write". | 10 concurrent assignments stored 10 duplicates. |
| G8 | `Cargo.lock` is committed (applications); `cargo deny check` / `cargo audit` is clean, or each ignore has a written reason. | A stale, git-ignored lock kept 14 known vulnerabilities on one machine. |

## 3. Design principles (from *A Philosophy of Software Design*)

- **Deep modules.** A module's interface should be much simpler than what it hides. Prefer one repository per aggregate that hides the storage layout over many one-method traits that each leak it.
- **Information hiding.** Each design decision (document layout, token format, hashing algorithm) is known by exactly one module. If two adapters build the same BSON path, one of them should own it.
- **Pull complexity down.** Callers should not handle what the module can handle (timeouts, retries, error mapping, offloading blocking work).
- **Define errors out of existence.** Make operations idempotent where it is natural (logout twice is fine), make invalid states unrepresentable (enums, newtypes), declare things where they are used (resource next to route) so they cannot drift.
- **Different layer, different abstraction.** A layer that only passes data through should not exist.
- **Design it twice.** For anything non-trivial, sketch two options, pick one, and write down why (five lines in the PR is enough).
- **Strategic, not tactical.** Spend a small part of every change leaving the touched code a bit better. Not a rewrite.

## 4. Architecture defaults

- **Workspace:** one library crate per bounded context with `domain` + `application` (no IO, no runtime, no framework types); one binary crate holding every adapter (HTTP, database, cache) and the wiring.
- **Adapters grouped by kind** (`http/`, `persistence/`, `cache/`), not mixed into HTTP controllers.
- **Domain:** value objects with private fields and a validating constructor (`new`/`TryFrom`) returning `Result`; entities expose behavior, not setters; closed sets are enums; **time, ids and randomness are injected** (pass `now: DateTime<Utc>`; add a `Clock` port only if several use cases need it).
- **Use cases:** one struct per business intent, generic over the ports it needs (static dispatch), one `async fn` entry point returning `Result<Output, Error>`.
- **Ports:** one repository per aggregate root. Native async traits with an associated error type:
  ```rust
  pub trait Sessions: Send + Sync {
      type Error;
      fn revoke(&self, user_id: &str) -> impl Future<Output = Result<(), Self::Error>> + Send;
  }
  // implementors may still write `async fn revoke(...)`
  ```
  Use `dyn Trait` (and then `#[async_trait]`) only when you need runtime choice or heterogeneous collections.
- **Modules:** `foo.rs` + `foo/` — no `mod.rs`.
- **Persistence:** typed `Collection<T>` / rows mapped to the domain in the adapter (not hand-navigated `Document`s); timestamps as native UTC date-times (local time only for display); state transitions as conditional updates (`filter: {_id, estado: expected}`, `matched_count == 0` → conflict).
- **HTTP:** DTOs at the edge, one error→HTTP mapping, one error body shape, pagination on every list, explicit payload limits.

## 5. Rust defaults

- Toolchain pinned in `rust-toolchain.toml`; `rust-version` in `[workspace.package]`; shared versions in `[workspace.dependencies]`; lints in `[workspace.lints]` with `[lints] workspace = true` in every member.
- **Errors:** a `thiserror` enum per module in libraries; at the binary edge, a small enum or `anyhow`. Keep the cause (`#[source]`, `#[from]`) — `map_err(|_| DatabaseError)` without logging loses the only clue.
- No lossy `as` casts on numbers that come from outside; use `TryFrom`/`try_into()`, and `checked_*` arithmetic for scores and money.
- Prefer `#[expect(lint, reason = "...")]` over `#[allow]`: a stale exception then warns.
- Borrow in parameters (`&str`, `&T`); clone at ownership boundaries only.
- `Debug` on public types, except types that hold secrets (redact them, or use `secrecy::SecretString`).
- Use edition 2024 features (let chains, `let … else`) when they make code clearer, not for novelty.

## 6. Async defaults

- Know the runtime model. In actix-web each worker is a **single-threaded** Tokio runtime; anything that does not `.await` for a long time freezes every request on that worker. An `async fn` with no `.await` that does real work is a smell.
- **Offload** CPU-heavy or blocking work with `spawn_blocking`. Put that decision in one small module (e.g. `cifrado::{cifrar, verificar}`) so no caller can forget it.
- **Run independent IO concurrently** with `tokio::try_join!` (note: MongoDB actions are `IntoFuture`; `tokio::try_join!` accepts them, `futures::try_join!` does not). Keep dependent steps sequential. Concurrency inside a request uses more pool connections.
- **Create clients once** at startup and share them (MongoDB `Client`, Redis `ConnectionManager`); never open a connection per request.
- **Timeouts** on every external call (driver options plus `tokio::time::timeout` for a request budget).
- Never hold a `std::sync::Mutex` guard across `.await`. Don't `tokio::spawn` work you cannot bound.
- Measure before optimizing: `async_trait` boxing costs ~13 ns per call — irrelevant next to one database round trip.

## 7. Security defaults

- Authorization: deny by default; resource declared per route scope; ownership checked for **every** read and write; a guard that branches on several query parameters must cover all of them.
- Sessions: if logout must revoke, the request path must consult the session store (store a token id, not the token).
- Passwords: Argon2id or bcrypt, hashed off the executor; reject inputs bcrypt would truncate (> 72 bytes); never derive initial passwords from personal data; rate-limit login; compare against a dummy hash when the user does not exist (no timing oracle).
- Input: validate at the boundary into domain types; reject unknown or oversized payloads.
- Dependencies: `cargo deny check` in CI; disable features you don't serve (e.g. HTTP/2 when there is no TLS).

## 8. Comments and documentation

- Comments describe **what is not obvious from the code**: intent, invariants, units, edge cases, and *why* a decision was made. Don't restate the code (`// extract token` above `extraer_token()` adds nothing).
- Every public type, trait and function that is not self-evident gets an **interface comment**: what it does, what it guarantees, which errors it returns (`# Errors`), and side effects — not how it works.
- If an interface comment is hard to write, the interface is probably too complicated — fix the design, not the comment.
- Use one language for comments and logs per repository. Domain terms stay in the domain language (ubiquitous language).
- A `TODO` needs an issue link or an owner; otherwise fix it or delete it.

Example of a good module comment (explains *why*, not *how*):

```rust
//! bcrypt outside the async executor.
//!
//! bcrypt is deliberately slow CPU work (~240 ms at cost 12). Each actix worker is a
//! single-threaded runtime, so hashing inline would freeze every request on that worker.
```

## 9. Testing defaults

- **Domain:** fast unit tests; pin boundaries (exactly-at-limit cases); no wall clock — inject the date.
- **Use cases:** in-memory fakes of the ports; assert effects, not just `is_ok()`.
- **HTTP and authorization:** `actix_web::test::init_service` + `TestRequest`, including hostile inputs (encoded paths, other users' ids, missing/expired tokens).
- **Adapters and end-to-end:** real MongoDB/Redis through testcontainers; random ids; no shared fixtures.
- A test that cannot fail is worse than no test: check it fails when you break the code (a quick manual mutation).
- Make code testable instead of working around it: configurable paths and database names, an `App` factory, injected clocks.

## 10. Verification loop and definition of done

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace                 # or: cargo nextest run --workspace
cargo deny check                       # or: cargo audit — locally when Cargo.toml/Cargo.lock changed; CI runs it on every push
```

**Done** = the loop is green · new behavior has tests (bugs: a test that failed before the fix) · no new `unwrap`/`print!`/secret in logs · docs and comments updated where behavior changed · a short report with the commands you ran and their results.

## 11. Over-engineering alarms

Stop and simplify when you see:

- a trait with one implementation and no test fake;
- a generic parameter that never varies;
- a layer or struct that only forwards calls;
- a config option nobody sets;
- a builder for a two-field struct;
- an event bus, outbox, CQRS split or new service with no concrete consumer;
- a rewrite proposed where a small fix removes the risk.

## 12. This repository

- Commands, crates and domain glossary: `CLAUDE.md`. Local services: `make dev` (MongoDB + Redis). Where `CLAUDE.md` gives a different quality command (its `cargo clippy -- -D warnings` skips test code), §10 wins.
- Known problems and the phased plan: `docs/CODE_REVIEW_AND_PLAN.md`. Until those items are fixed, **do not copy** these patterns: path-parsing authorization, `_id`-only write filters, check-then-insert, inline bcrypt, a Redis connection per request, `Box<dyn>` ports, manual `Document` navigation, `mod.rs`.
- Authorization still derives the resource from the URL path, so G3 is not met yet (plan items 0.1 and 1.8). Until 1.8 lands, a new top-level route must also be added to `Recurso::desde_ruta` in the same change, with an encoded-path test; otherwise it is unprotected (before 0.1) or always denied (after 0.1).
- Agents: `rust-reviewer` (read-only review) and `rust-engineer` (implementation) in `.claude/agents/`, `.gemini/agents/` and `.agents/agents/`.
- Comments and logs (§8): this code mixes Spanish and English and no language has been chosen yet. Until the team picks one, write new comments and logs in the language of the file you are editing.
