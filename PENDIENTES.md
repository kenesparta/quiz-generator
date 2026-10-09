# Pendientes

Lo que quedó sin hacer de la revisión de código de octubre de 2026 (informe en `docs/`, que no se
versionó y se eliminó). Los IDs (`SEC-05`, `R-063`...) y la numeración del plan (`4.6`) son los del
informe; el anexo A resume el plan y el anexo B da el título de cada ID que citan los commits y el
código.

## Funcionalidades (requieren una decisión de producto)

| ID | Qué falta | Estado actual y propuesta |
|---|---|---|
| SEC-05 / R-003 | Contraseña inicial del postulante | Sigue siendo los últimos 4 dígitos del documento, que es también el usuario, y no se puede cambiar (`PUT /postulantes` la conserva). Propuesta: contraseña aleatoria devuelta una sola vez en el 201 (o un código de acceso por asignación), cambio obligatorio en el primer inicio de sesión, endpoint de cambio de contraseña y reinicio de las contraseñas existentes. Ya mitigado en parte: límite de intentos de login (SEC-08). El e2e del frontend depende de la regla actual (`e2e/support/seed.js`). |
| R-031 (plan 1.10) | `DELETE /postulantes/{id}` | Responde 501. Falta decidir qué pasa con las hojas de respuestas del postulante: borrarlas, anonimizarlas o impedir el borrado si tiene alguna. |
| R-009 | Unicidad del documento entre colecciones | Dentro de cada colección ya hay índice único (DAT-06). Un mismo documento puede existir como admin, psicólogo y postulante; el login busca en ese orden y toma el primero. |
| R-022 (plan 2.7) | Paginación de los listados | Las proyecciones ya están; falta `limit`/`skip` o cursor. |
| R-037 | Agregar preguntas de forma idempotente | `PUT /examenes/{id}` agrega preguntas, no reemplaza. Propuesta: `POST /examenes/{id}/preguntas` (201 con los ids); opcionales `POST /evaluaciones/{id}/examenes` y `PATCH /evaluaciones/{id}/estado`. El alta duplicada ya responde 409. |
| R-085 | `/ready` | `/health-check` solo dice que el proceso vive. Falta un `/ready` público que haga ping a MongoDB y Redis en paralelo con un tiempo límite (2 s). |
| R-099 | Propiedad por objeto | Cualquier psicólogo puede editar cualquier examen o evaluación. Confirmar la regla; si aplica, guardar `creado_por` y filtrar por él (el admin sin restricción). |
| plan 4.6 | Fechas en UTC | Se guardan como texto en hora de Lima (el lector acepta tres formatos) y `""` cuando faltan. Propuesta: `DateTime` de BSON en UTC y `Option`. Requiere migrar los datos. |

## Refactors (sin cambio de comportamiento)

| ID | Qué falta | Notas |
|---|---|---|
| R-063 | Encapsular las entidades | Tienen campos `pub` y los adaptadores las arman campo a campo. Campos privados y un constructor de rehidratación explícito (`desde_persistencia`) que no repita las validaciones de entrada. |
| R-021 | Reglas de negocio en los casos de uso | La asignación hace sus cuatro comprobaciones dentro de `RespuestaMongo::asignar_evaluacion` y el caso de uso solo reenvía. Moverlas sin perder la atomicidad: el índice único y los compare-and-set siguen siendo la garantía. |
| R-017 | Un solo tipo por documento | El esquema de examen y pregunta se repite en `mongo/examen/pregunta_dto.rs`, `mongo/respuesta/documento.rs` y varios `doc!`. Un tipo serde por documento con `From`/`TryFrom` hacia el dominio. |
| plan 4.4 | Lecturas tipadas | Falta el listado de asignaciones (`mongo/respuesta/read.rs`, `listar`), que sigue con `get_str(...).unwrap_or_default()`. Esos valores por defecto sostienen documentos antiguos: pasarlo a un tipo con `#[serde(default)]` equivalente. |
| R-056 | Clave de corrección en cada hoja | El `puntaje` se copia en cada hoja y el secreto depende de los DTO (hay tests e2e de que el postulante no ve puntos). Opcional: excluirlo con una proyección en la lectura del postulante. |
| R-071 | Tiempo límite por petición | Los plazos de conexión de MongoDB y Redis ya están. Falta un límite por petición (`wrap_fn` + `timeout`, 503/504) y `max_time` en los listados. |
| plan 4.9 | Casbin → `match` (opcional) | Quitaría casbin, rhai y smartstring, y con ellos el aviso RUSTSEC-2026-0249 ignorado en `deny.toml`. Mantener Casbin si personas que no programan editan la política. |
| — | `Sesiones` sin `async_trait` (opcional) | Es el único puerto que se usa como `dyn`. Alternativa: un enum con despacho estático. Ganancia mínima. |
| R-079 | Comentarios que contradicen el código | Muchos se reescribieron en los cambios, pero no se hizo una pasada completa. |

## Frontend (`quiz-generator-front`)

- No muestra los tipos de pregunta `si_o_no` y `libre` (`QuestionCard` y el tipo `Question` solo conocen `alternativa_unica`, `alternativa_peso` y `sola_respuesta`).
- Los formularios no restringen el documento a 4–20 letras o dígitos; la API responde 400 y ahora se ve su mensaje.
- Un error al guardar una respuesta (`useRespuestaEvaluacion.updateResponse`) reemplaza toda la evaluación por la pantalla de error.
- Biome: 3 errores de accesibilidad en `src/components/admin/Revision.tsx` (`noSvgWithoutTitle`, `useButtonType` ×2) y el formato de `.claude/settings.local.json`.
- Su `CLAUDE.md` describe endpoints que ya no existen (`/login/postulante`, `/respuesta/...`).

## Antes de desplegar

- **Secreto JWT** de al menos 32 bytes (el `configuration.yaml` local tenía 14): `QUIZZ_JWT__SECRET="$(openssl rand -base64 48)"`. Cambiarlo si el de desarrollo se usó fuera de localhost: los tokens que estaban en los `.http` siguen en el historial de git.
- **Límite de login**: 20 intentos por IP cada 15 minutos, contando los exitosos. Subir `login.max_intentos_por_ip` al tamaño de una sala de examen (comparten IP pública). `login.detras_de_proxy: true` solo si el proxy reescribe `X-Forwarded-For`; si no, todos los clientes comparten la IP del proxy.
- **CORS**: `QUIZZ_CORS__ALLOWED_ORIGINS` con el dominio del frontend, sin `/` final. Por defecto solo `http://localhost:3000`.
- **Sesiones**: cambió su formato en Redis, todos deben volver a iniciar sesión. Hay una sola sesión por usuario: iniciar sesión en otro equipo cierra la anterior.
- **Datos existentes**, antes de que el arranque cree los índices únicos: documentos duplicados en cada colección de usuarios, asignaciones duplicadas (postulante + evaluación), preguntas sin `_id` y documentos que no cumplan `^[A-Za-z0-9]{4,20}$` (esas cuentas no pueden iniciar sesión).
- Los valores de `estado` son snake_case en todas las respuestas (`creado`, `en_proceso`, `finalizado`).

## Archivos locales que todavía nombran `docs/`

Sin versionar: `AGENTS.md` (su sección 12 está desactualizada) y `.claude/agents/rust-engineer.md` y `rust-reviewer.md`, que mencionan `docs/CODE_REVIEW_AND_PLAN.md`.

## Anexo A — Plan de mejora

Numeración que citan los commits ("plan 3.1", "fase 4.3"). Texto original del informe.


**Phase 0 — Day 1: close the hole, turn the gates green**

| # | Qué |
|---|---|
| 0.1 | SEC-01 minimal fix + middleware tests |
| 0.2 | `cargo update`; commit `Cargo.lock`; bump `async-trait = "0.1.92"` in the manifests |
| 0.3 | Fix the 2 `assert!(true/false)` in `pregunta.rs` |
| 0.4 | `cargo fmt --all` in its own commit |
| 0.5 | Remove `print!` in `documento.rs:21`; stop logging `documento` |
| 0.6 | `rust-toolchain.toml` (1.99) + `rust-version = "1.99"` |

**Phase 1 — Week 1: remaining security bugs**

| # | Qué |
|---|---|
| 1.1 | Owner + state in every write filter (SEC-02) |
| 1.2 | Fix the PII guard (SEC-03) |
| 1.3 | Random initial credentials (SEC-05) |
| 1.4 | Session check in the middleware (SEC-04) |
| 1.5 | Dependency features (SEC-07 steps 2–3) + `deny.toml` |
| 1.6 | Login hardening (SEC-08): rate limit, dummy hash, 72-byte limit |
| 1.7 | JWT: keys built once, `set_required_spec_claims(&["exp", "sub"])`, secret ≥ 32 random bytes checked at startup, secrets from env |
| 1.8 | Declare the resource on each route scope and delete `Recurso::desde_ruta` (SEC-01 "Better design") |
| 1.9 | Grading integrity (SEC-09, COR-02) |
| 1.10 | Implement or remove `DELETE /postulantes/{id}` (COR-03) |
| 1.11 | Validate plaintext passwords before hashing (SEC-10) |

**Phase 2 — Week 2: async and data integrity**

| # | Qué |
|---|---|
| 2.1 | `cifrado.rs` with `spawn_blocking` (PERF-01) |
| 2.2 | One shared `redis::aio::ConnectionManager` |
| 2.3 | `ensure_indexes` + unique assignment index (DAT-01) |
| 2.4 | Compare-and-set transitions (DAT-02) |
| 2.5 | Timeouts: Mongo `connect_timeout` 5 s, `server_selection_timeout` 10 s; Redis set explicitly |
| 2.6 | `tokio::try_join!` for the 3 independent lookups in `asignar_evaluacion` — **only together with 2.3** |
| 2.7 | Pagination (`limit`/`skip` or cursor) + projections on list endpoints |
| 2.8 | Dev profile: optimize bcrypt/blowfish in debug |
| 2.9 | Keep questions without an image (DAT-03); associate exams only to drafts (DAT-04) |
| 2.10 | Enforce `estado` on grading writes (DAT-05) |
| 2.11 | Unique `documento` per collection, after a duplicate audit (DAT-06) |

**Phase 3 — Weeks 3–4: make regressions visible**

| # | Qué |
|---|---|
| 3.1 | CI: fmt, clippy `--all-targets -D warnings`, tests, `cargo deny` |
| 3.2 | `[workspace.lints]` + `clippy.toml` (§8.2) |
| 3.3 | Middleware/authorization tests with `actix_web::test` |
| 3.4 | e2e harness (testcontainers) with the attack checks from §4 |
| 3.5 | Testability refactors: configurable `rbac` paths and database name, an `App` factory |
| 3.6 | Coverage floor for `cmd/api` (start at the current value, raise it as tests land) |
| 3.7 | Fix or delete the stale Dockerfile/Makefile targets |

**Phase 4 — Ongoing: simplify the design (strategic, in small steps)**

| # | Qué |
|---|---|
| 4.1 | Move Mongo/Redis adapters out of `controller/` into `persistence/` and `cache/` |
| 4.2 | One repository per aggregate instead of one trait per operation |
| 4.3 | Native `async fn` ports with generics (§8.3) |
| 4.4 | Typed `Collection<T>` instead of hand-navigated `Document`s |
| 4.5 | Keep error sources (`#[source]`), log at the boundary |
| 4.6 | Dates as BSON `DateTime` (UTC), `Option` instead of `""` |
| 4.7 | `mod.rs` → file modules (`git mv dir/mod.rs dir.rs`, no content change) |
| 4.8 | Decide `bctx/` vs `bounded-contexts/` (ARC-05) |
| 4.9 | Optional: replace Casbin with an exhaustive `match` |

## Anexo B — IDs citados en los commits y en el código

Títulos originales del informe (en inglés). Los commits citan estos IDs para explicar por qué se hizo cada cambio.

| ID | Hallazgo |
|---|---|
| ARC-05 | Two architectures: `bctx/` runs, `bounded-contexts/` is partial and unused |
| COR-01 | Panics on stored data |
| COR-02 | Scoring adds up every submitted key |
| COR-03 | `DELETE /postulantes/{id}` returns 201 and deletes nothing (`cmd/api/src/controller/postulante/registrar_postulante.rs:198-204`); `eliminar_postulante` is never called |
| DAT-01 | Duplicate assignments (check-then-insert) |
| DAT-02 | State transitions race and overwrite each other |
| DAT-03 | Questions without an image disappear |
| DAT-04 | Associating exams after publishing corrupts the evaluation |
| DAT-05 | Grading is allowed before the exam is finished |
| DAT-06 | One national ID can have several accounts |
| PERF-01 | bcrypt blocks the async executor |
| R-001 | IDOR: GET /postulantes?id=&lt;own&gt;&documento=&lt;victim&gt; returns any candidate's PII |
| R-002 | RBAC middleware fails open on unmapped paths; percent-encoded paths bypass Casbin |
| R-004 | bcrypt (cost 12) hash/verify runs synchronously on actix worker threads |
| R-005 | Scoring sums every submitted key: duplicates/select-all inflate scores; no per-type rules |
| R-006 | Questions without imagen_ref are silently dropped on read and lost when publishing |
| R-007 | Exams can be added after publish: $addToSet corrupts the snapshot and breaks assignment |
| R-008 | State machine unenforced on writes: answers in any estado, revision before finalizado |
| R-009 | documento uniqueness not enforced within or across user collections; login takes first |
| R-010 | GET /respuestas/{id} shows candidates per-question points (answer-key oracle) and verdict |
| R-011 | Logout doesn't revoke JWTs: Redis session is written/deleted but never checked |
| R-012 | No rate limiting, throttling or lockout on POST /login |
| R-013 | JWT HS256 secret is a public placeholder, never validated (length/TTL); no iss/aud |
| R-014 | Respuesta write endpoints (contestaciones, estado) don't check the caller owns the record |
| R-015 | Admin/psicologo password checks run on the bcrypt hash, so empty passwords are accepted |
| R-016 | Shallow abstractions: many 1-method ports, generic RepoErr, async_trait + Box&lt;dyn&gt; |
| R-018 | Postulante read adapter duplicates the document-to-entity mapping three times |
| R-019 | Admin/psicologo/postulante stacks duplicate ports, errors and bcrypt adapters |
| R-020 | Unused parallel domain model (bounded-contexts/) and empty crates are workspace members |
| R-021 | Business rules live in Mongo adapters; use cases are pass-throughs (anemic domain) |
| R-022 | Lists and answer path load whole documents/snapshots: no projection, pagination or limit |
| R-023 | No MongoDB indexes: logins, lists, revisiones and assignment checks do full scans |
| R-024 | Redis: new TCP connection per login/logout; ConnectionManager unused; no startup check |
| R-025 | Example config has null Redis values and docs omit sections: quick start panics |
| R-026 | Mongo database name hard-coded twice; configured database_name is ignored |
| R-027 | Mongo/Redis URIs built with format! from raw credentials; TLS/SRV/options unconfigurable |
| R-028 | Associating exams silently drops malformed IDs and accepts nonexistent ones |
| R-029 | Documento VO slices by byte index: non-ASCII input panics registration |
| R-030 | Name regex range À-ú rejects ü (e.g. Agüero) and accepts × and ÷ |
| R-031 | DELETE /postulantes/{id} is a stub: returns 201 Created and deletes nothing |
| R-032 | Psicologo/admin always get 404 on GET /respuestas/{id} (filter uses staff id as owner) |
| R-033 | Estado stringly typed in 3 casings: list never emits empezar/finalizar links |
| R-034 | POST /evaluaciones/{id}/respuestas returns 201 with empty id and broken self link |
| R-035 | Draft-evaluation assignment rejected only via deserialization failure (500, misleading) |
| R-036 | Lossy `as` casts on points across u32/i32/i64 silently wrap or truncate scores |
| R-037 | PUT /examenes/{id} appends questions (non-idempotent); duplicate POST returns 500 |
| R-038 | HATEOAS links are hand-built strings pointing to routes/methods that don't exist |
| R-039 | Nombre VO surname length checks measure nombre (copy-paste), so surnames are unbounded |
| R-043 | Estado transitions are check-then-write; finalizar/revisar are non-atomic 1+N updates |
| R-044 | Assignment is check-then-insert without unique index: concurrent requests duplicate it |
| R-045 | Timestamps stored as Lima-offset strings with "" sentinel; elapsed time and sort wrong |
| R-046 | Revision stores no reviewer/time; GET /revisiones/{id} shows the viewer as psychologist |
| R-047 | Publish is check-then-act in the adapter with no atomic guard; empty evaluations publish |
| R-048 | Cargo.lock git-ignored; advisories (e.g. bcrypt RUSTSEC-2026-0199) untracked |
| R-049 | README and CLAUDE.md have drifted from the actual code structure |
| R-050 | unwrap() in respuesta Mongo DTO-to-domain From impls panics on malformed stored data |
| R-051 | No ResponseError impl: each handler maps errors to HTTP by hand, often to wrong status |
| R-052 | Adapters discard driver errors via map_err(\|_\| ..); error variants carry no cause |
| R-053 | Login timing (bcrypt only for known documento) enables user enumeration |
| R-054 | DNI (PII) logged at info/error levels and print!ed to stdout in Documento::new |
| R-055 | Question images: unvalidated inline base64 copied into every evaluation and respuesta |
| R-057 | Risky code (cmd/api ~2%: middleware, handlers, adapters, use cases) has no tests |
| R-058 | Use-case tests pass for the wrong reason: mocks ignore config, multiple invalid fields |
| R-059 | Wall clock read directly in domain; FechaNacimiento tests will fail on 2031-02-15 |
| R-060 | Quality gates not enforced: no CI, inert hook, no lint config; fmt/clippy fail on main |
| R-061 | Dockerfile is a leftover that cannot build or run the API (wrong deps, port, toolchain) |
| R-062 | Box&lt;dyn TipoPreguntaStrategy&gt; spreads rules over 7 files; match on TipoPregunta enum |
| R-064 | Dead/placeholder code: unused per-role logins, provider traits, empty modules, variants |
| R-065 | Misleading names: typos, wrong descriptions, one word for several concepts |
| R-067 | controller/ holds all adapters (Mongo, Redis, JWT, middleware); modules shadow crates |
| R-068 | Publish reads exams one by one (N+1); write paths add redundant existence reads |
| R-069 | Independent DB reads awaited sequentially (assignment, revision detail) |
| R-070 | Auth middleware rebuilds JWT key per request; Casbin behind an RwLock nothing writes |
| R-072 | Secrets only from a CWD-relative YAML file; no environment-variable overrides |
| R-073 | Casbin model/policy loaded from CWD-relative files at runtime; policy matrix untested |
| R-074 | CORS allowlist hardcoded to localhost:3000; trailing-slash entries never match |
| R-076 | Alternatives stored in HashMap, so their order is random in DB and JSON |
| R-077 | Unused FechaTiempoValueObject: Display always errors, so to_string() panics |
| R-078 | Domain crates depend on tokio only for tests; versions/metadata repeated across manifests |
| R-080 | Error enums: copy-pasted/wrong messages, overlapping and never-constructed variants |
| R-081 | expect()/assert! in production code (main.rs, shared kernel) despite no-panic rule |
| R-082 | Actix extractor errors bypass the JSON error format; 2 MiB body limit implicit |
| R-083 | Auth: token-expiry distinction discarded, duplicated token extraction, unused variants |
| R-084 | thiserror messages use Debug formatting and print causes twice |
| R-086 | Two logging facades in use; no access log or per-request correlation |
| R-087 | Handlers parse match_info by hand, have typo'd type names, log unvalidated params |
| R-088 | Legacy ID stores a runtime IdType tag duplicating the per-module newtypes |
| R-089 | mod.rs files used widely, contradicting CLAUDE.md file-based module convention |
| R-090 | Debug derived on types holding tokens/password hashes; ID newtypes lack common traits |
| R-091 | Older idioms where Rust 1.70-1.88 features (LazyLock, let chains) would simplify |
| R-092 | Candidates can read all questions while respuesta is still creado (before clock starts) |
| R-093 | Bearer tokens (including admin) committed in .http example files |
| R-094 | Dev compose publishes MongoDB (root quizz/quizz) and password-less Redis on 0.0.0.0 |
| R-095 | Tests that can't fail: constant asserts, fixed-size length checks, missed boundaries |
| R-096 | .http files are stale and assertion-free: expired tokens, missing auth, mixed roles |
| R-097 | Stale Makefile targets and ignore entries copied from other projects |
| RS-01 | Pin the toolchain and the MSRV |
| SEC-01 | Privilege escalation through a percent-encoded path |
| SEC-02 | Writes are not scoped to the owner (IDOR) |
| SEC-03 | Personal data of any candidate readable by any candidate |
| SEC-04 | Logout does not revoke anything |
| SEC-06 | National IDs written to stdout and logs |
| SEC-07 | Vulnerable and stale dependencies |
| SEC-08 | Login can be brute-forced and leaks which documents exist |
| SEC-09 | Candidates can recover the answer key |
| SEC-10 | Empty passwords accepted for admin and psychologist accounts |
