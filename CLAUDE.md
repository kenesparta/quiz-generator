# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Quiz Generator is a Rust-based HTTP API for creating exams, composing evaluations, assigning them to candidates (postulantes), and collecting/grading their answers. The system uses a bounded context architecture with MongoDB for persistence, Redis for session management, and JWT authentication.

**Stack:** Rust 2024 edition, Actix Web, MongoDB, Redis, Tokio runtime

## Workspace Structure

The repository uses a Cargo workspace organized by bounded contexts (bctx):

- `bctx/core` - Core domain logic (examen, pregunta, evaluacion, postulante, psicologo, admin, respuesta)
- `bctx/auth` - Authentication domain (universal login/sessions and authorization)
- `bctx/common` - Shared utilities and types
- `cmd/api` - HTTP API service (binary: `quizz`)

### Core Domain Organization

Each domain module in `bctx/core/src/` follows a consistent structure:
- `domain/entity/` - Core business entities
- `domain/value_object/` - Value objects and IDs
- `domain/error/` - Domain-specific errors
- `provider/repositorio.rs` - Repository trait definitions (ports): native async traits that return the module's own error, declared as `fn x(&self, ..) -> impl Future<Output = Result<T, ModuloError>> + Send` and implemented with `async fn`
- `use_case/` - Application use cases (business operations): generic over their ports (static dispatch, e.g. `CrearExamen<R: RepositorioExamenEscritura>`) with an inherent `async fn ejecutar`

The only `dyn` port is `Sesiones` (`bctx/auth`, still `#[async_trait]`): the API shares it as `web::Data<dyn Sesiones>` and the tests swap in an in-memory one.

Question types are the `TipoPregunta` enum (`alternativa_unica`, `alternativa_peso`, `si_o_no`, `libre`, `sola_respuesta`); the composition rules of each type are one `match` in `pregunta/domain/entity/pregunta.rs` and the scoring rule is `corregir_respuesta` in `respuesta/domain/entity/pregunta.rs` (one answer per question).

## Development Commands

### Local Development Setup

```bash
# Start MongoDB (localhost:27017) and Redis (localhost:6379)
make dev
# or: docker compose -f docker-compose.dev.yml up -d --build

# Create configuration file
cp configuration.yaml.example configuration.yaml

# Run the API (binds to 0.0.0.0:8008 by default)
cargo run -p quizz-api --bin quizz
```

MongoDB credentials (dev): username `quizz`, password `quizz`, database `quizz`

### Testing and Quality

```bash
# Everything CI requires (format check, clippy on all targets, tests)
make check

# Format code
cargo fmt --all

# Lint with clippy, including tests (fail on warnings)
cargo clippy --workspace --all-targets -- -D warnings

# Tests (the e2e test in cmd/api/tests/api_e2e.rs needs Docker)
cargo test --workspace

# Coverage (requires llvm-tools-preview and cargo-llvm-cov)
make cobertura

# Dependency advisories, licenses and sources
cargo deny check
```

### Running Single Tests

```bash
# Run a specific test by name
cargo test test_name

# Run tests in a specific module
cargo test --package quizz-core --lib pregunta::domain

# Run with output visible
cargo test test_name -- --nocapture
```

## Architecture Notes

### API Layer (`cmd/api/src/`)

- `main.rs` - Entry point, loads configuration and starts server
- `startup.rs` - Server setup with route configuration
- `configuration.rs` - Config loading from `configuration.yaml`
- `mongo.rs` / `cache.rs` - MongoDB and Redis connections (one shared Redis `ConnectionManager`)
- `mongo/<module>/` - the MongoDB adapters implementing the domain ports: one struct per collection (`ExamenMongo`, `PostulanteMongo`, `RespuestaMongo`...) implements every port backed by that collection, with the documents it reads and writes; `mongo/repositorio.rs` gives each adapter its collection, plus `es_clave_duplicada` (E11000)
- `cache/sesiones.rs` - the Redis adapter of the `Sesiones` port
- `indices.rs` - MongoDB indexes created at startup (unique: one assignment per candidate and evaluation, one account per `documento`)
- `controller/` - HTTP handlers organized by domain (examen, evaluacion, postulante, psicologo, admin, respuesta, revision, auth)
- `controller/error.rs` - `ApiError`: the single error → HTTP status mapping. Handlers return `Result<HttpResponse, ApiError>` and use `?`; add new domain error variants there (the `match` is exhaustive on purpose)
- `controller/cifrado.rs` - the only bcrypt adapter (runs on `spawn_blocking`)
- `controller/hateoas.rs` - `Link`/`Links` and `enlaces::*`, the link builders of each resource (a test checks every advertised link exists)
- `prueba_http.rs` (tests only) - the real app (routes, auth, RBAC) without a database, for in-process HTTP tests

Each controller module has:
- `route.rs` - Actix Web routes of the module; **the source of truth for the routes** (the list below is a summary)
- `dto.rs` - request/response DTOs
- one file per handler (e.g. `registrar_examen.rs`, `obtener_respuesta.rs`)

### Authentication Flow

Uses JWT tokens whose session (`jti`) is tracked in Redis: `AuthMiddleware` accepts a token only while its session is open (port `Sesiones` in `bctx/auth`, adapter `cache/sesiones.rs`), so logout revokes it immediately. There is a **single universal login endpoint** `POST /login` that accepts `{ "documento": "...", "password": "..." }` and searches across all collections (admin → psicologo → postulante) to find the user and return a JWT with the appropriate role.

Auth is handled in:
- `bctx/auth/src/universal/` - Universal login domain (use case, provider traits, entity, error)
- `cmd/api/src/controller/auth/` - Login controller, JWT provider, and auth middleware
- Users are identified by their `Documento` (DNI) number, validated with the `Documento` VO from `bctx/core`

### Authorization (RBAC with Casbin)

The system uses **Casbin** (`casbin` crate v2) for Role-Based Access Control (RBAC). The authorization layer follows the same DDD/hexagonal architecture as the rest of the project:

**Domain layer** (`bctx/auth/src/autorizacion/`):
- **Value Objects**: `Rol` (Postulante, Psicologo, Admin), `Recurso` (Admin, Examen, Evaluacion, Postulante, Psicologo, Respuesta, Revision), `Accion` (Leer, Escribir, Actualizar, Eliminar)
- **Entity**: `SolicitudAcceso` — represents an access request (sujeto, rol, recurso, accion)
- **Error**: `AutorizacionError`
- **Provider (Port)**: `AutorizacionVerificar` trait — defines the authorization contract (synchronous: the policy is evaluated in memory)

**Infrastructure layer** (`cmd/api/`):
- **Casbin adapter**: `controller/auth/casbin_enforcer.rs` — implements `AutorizacionVerificar` using `casbin::Enforcer`; the model and policy are embedded in the binary with `include_str!`
- **Actix middleware**: `controller/auth/middleware.rs`
  - `AuthMiddleware` wraps the whole protected scope: it requires a valid `Authorization: Bearer` JWT with a known role and stores the `Claims` in the request extensions.
  - `Autorizacion::para(Recurso::X)` wraps **each route scope** and enforces RBAC for the resource that scope declares; the action comes from the HTTP method. Anything that cannot be classified (unknown method, missing claims) is denied.
  - **Never derive the resource from the URL**: the router percent-decodes paths (`/%61dmins` reaches `/admins`), so parsing the raw path let any user bypass RBAC. A new route scope must call `.wrap(Autorizacion::para(...))` and be added to the route table in the middleware tests.
- **RBAC model**: `rbac/model.conf` — standard RBAC model (request, policy, role definitions, matchers)
- **Policies**: `rbac/policy.csv` — defines permissions per role; the expected matrix is pinned by a test in `casbin_enforcer.rs`

**Roles and permissions:**
- `admin` — full access to all resources
- `psicologo` — manage exams, evaluations, candidates, and reviews
- `postulante` — read/write/update own respuestas only

**Public routes** (no auth required): `/health-check`, `/login`
**Protected routes** (JWT + RBAC): all other endpoints

**Configuration**: JWT settings (`secret`, `expiration_seconds`) are under the `jwt:` key. The JWT keys and validation are built once at startup (`JWTProvider`, shared as `web::Data`).

### Configuration

`configuration.yaml.example` is the reference (copy it to `configuration.yaml`; a test checks that it loads). Settings come from `configuration.yaml` (optional) and `QUIZZ_*` environment variables, which win: `QUIZZ_JWT__SECRET`, `QUIZZ_DATABASE__PASSWORD`, `QUIZZ_CORS__ALLOWED_ORIGINS=a,b`... `Settings::validar` rejects at startup a JWT secret shorter than 32 bytes, a token lifetime outside 1 s–7 days and CORS origins with a trailing slash. `database.database_name` selects the MongoDB database (injected as `web::Data<mongodb::Database>`); `database.uri` / `redis.uri` accept full connection URIs (TLS, SRV).

### API Routes

- `GET /health-check` - Health check endpoint
- `/examenes` - Create exams, add questions, list
  - `GET /examenes` - List exams
  - `POST /examenes/{id}` - Create an exam
  - `PUT /examenes/{id}` - Add questions to an exam (images: PNG/JPEG/WebP data URIs up to 512 KiB)
- `/evaluaciones` - Create evaluations, associate exams, publish, assign to candidates, list
  - `GET /evaluaciones` - List evaluations
  - `POST /evaluaciones/{id}` - Create an evaluation
  - `PUT /evaluaciones/{id}` - Associate exams with a draft evaluation (`409` once published)
  - `PATCH /evaluaciones/{id}` - Publish an evaluation (once, and only if it has questions)
  - `POST /evaluaciones/{evaluacion_id}/respuestas` - Assign a published evaluation to a candidate (creates a respuesta with estado `creado`, returns its id and `Location`; `409` if already assigned)
- `/postulantes` - CRUD operations for candidates
  - `GET /postulantes` - Staff: by `id` or `documento` (query), or the whole list without them. A postulante always gets their own record
  - `PUT /postulantes` - Update candidate by document (in the body)
  - `POST /postulantes/{id}` - Create candidate
  - `DELETE /postulantes/{id}` - Not implemented yet (`501`): what happens to the candidate's respuestas is undecided
- `/respuestas` - Manage exam lifecycle, submit answers
  - `GET /respuestas` - Unfinished respuestas of a candidate (a postulante gets their own; staff pass `postulante_id`)
  - `GET /respuestas/asignaciones` - List assignments (staff only)
  - `GET /respuestas/{id}` - A respuesta: a postulante only their own and **without points**, and without questions before `empezar`; staff any, complete
  - `PATCH /respuestas/{id}/estado` - Owner only. Body `{"accion":"empezar"}` or `{"accion":"finalizar"}`; compare-and-set, idempotent
    - `empezar`: `creado` → `en_proceso` (sets fecha_tiempo_inicio)
    - `finalizar`: `en_proceso` → `finalizado` (sets fecha_tiempo_fin)
  - `POST /respuestas/{id}/examenes/{examen_id}/preguntas/{pregunta_id}/contestaciones` - Owner only, while `en_proceso`; one answer per question
- `/revisiones` - Grade and review completed evaluations
  - `GET /revisiones` - List revisiones
  - `GET /revisiones/{respuesta_id}` - A graded respuesta, with the psychologist who graded it
  - `POST /revisiones/{respuesta_id}` - Grade a `finalizado` respuesta (also accepts `PATCH`); stores `revisado_por` and `fecha_revision`
- `POST /login` - Universal login (searches admin → psicologo → postulante by documento, returns JWT with role). Same `401` and same timing for unknown documento and wrong password; attempts limited per IP and failures per documento (`429`)
- `POST /logout` - Cierra la sesión. Recibe `Authorization: Bearer <token>` y cierra su sesión en Redis (clave `sesion:{sub}`, valor `jti`): desde ese momento el token responde 401. Devuelve 204 incluso si el token está expirado o la sesión ya estaba cerrada, para que el cliente pueda limpiar su sesión local.

Example HTTP requests are in `cmd/api/http/dev/*.http` files; run `auth/login.http` first (JetBrains HTTP Client stores the tokens as global variables). Never commit tokens.

Error bodies are always `{"error": "..."}` (also for malformed JSON, query or path). Estado values are snake_case everywhere: `creado`, `en_proceso`, `finalizado`.

List endpoints return HATEOAS-style responses (see `cmd/api/src/controller/hateoas.rs`) embedding `_links` for navigation.

## Key Domain Concepts

**Examen (Exam):** A collection of questions (preguntas) grouped together.

**Pregunta (Question):** Individual questions; each `TipoPregunta` has its own composition and scoring rules.

**Evaluacion (Evaluation):** A composition of one or more exams to be assigned to candidates.

**Postulante (Candidate):** The person taking an evaluation.

**Respuesta (Answer):** Tracks a candidate's assigned evaluation, their submitted answers, and completion status. Contains the full evaluation snapshot at assignment time. Follows a state machine with three estados (states):
- **Creado** (`creado`) - Initial state when evaluation is assigned to candidate
- **EnProceso** (`en_proceso`) - Exam in progress (after empezar, fecha_tiempo_inicio is set); only now can the owner answer
- **Finalizado** (`finalizado`) - Exam completed (after finalizar, fecha_tiempo_fin is set); answers are frozen

Every write on a respuesta filters by the owner (`postulante_id` from the token) and, for transitions and answers, by the expected estado in the same MongoDB update (compare-and-set). Points per exam (`puntos_obtenidos`) are computed on read from the questions' points, not stored.

**Revision:** The grading/review process for completed evaluations. Only respuestas with estado `finalizado` can be graded (enforced in the update filter).

## Docker

```bash
docker build -t quizz-api:local .
docker run --rm -p 8008:8008 -v "$PWD/configuration.yaml":/app/configuration.yaml:ro quizz-api:local
```

The image is distroless and non-root; the RBAC policy is embedded in the binary; configuration comes from a mounted `configuration.yaml` or `QUIZZ_*` variables. `docker-compose.dev.yml` only includes MongoDB and Redis, not the API.

## Code Conventions

### Rust Module Organization

When organizing Rust modules, prefer the modern file-based pattern over `mod.rs`:

```
# Preferred (Rust 2018+ style)
my_module.rs           # Module declaration file
my_module/             # Directory for submodules
  submodule_a.rs
  submodule_b.rs

# Avoid (legacy style)
my_module/
  mod.rs               # Don't use mod.rs
  submodule_a.rs
  submodule_b.rs
```

Example: For a `value_object` module containing multiple files:
```
pregunta/
  value_object.rs      # Declares: mod alternativa; mod puntaje; pub use ...
  value_object/
    alternativa.rs
    puntaje.rs
    etiqueta.rs
```

### DDD Structure in bctx

The `bctx/` crates follow Domain-Driven Design principles:
- **Value Objects**: Immutable objects compared by value (e.g., `Documento`, `Nombre`)
- **Entities**: Objects with identity, compared by ID (e.g., `PreguntaEntity`, `Postulante`)
- **Sum Types**: Use Rust enums for type-safe variants instead of dynamic dispatch (e.g., `TipoPregunta`)

`bctx/` is the only domain model. A partial redesign that lived in `bounded-contexts/` was removed
(it was never wired to the API); its good ideas are ported into `bctx/` one at a time.

Prefer static dispatch (enums with match) over dynamic dispatch (`Box<dyn Trait>`) for domain types.

### Error Handling

Use `Result<T, E>` for all fallible operations. Never use `panic!`, `unwrap()`, or `expect()` in production code.

**Guidelines:**
- Use `thiserror` crate for defining custom error types
- Every domain module should have its own error type in `error.rs`
- Avoid functions that can panic; always return `Result<T, E>` or `Option<T>`
- Use `?` operator for error propagation
- Name error types with the suffix `Error` (e.g., `ExamenError`, `PreguntaError`)
- **NEVER use `unwrap()` or `expect()`** in production code; use `?` or explicit error handling
- `unwrap()` is **only acceptable in tests** (`#[cfg(test)]` modules) where panicking on failure is desired behavior

**Example:**
```rust
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ExamenError {
    #[error("Índice fuera de rango: {indice}, máximo: {maximo}")]
    IndiceFueraDeRango { indice: usize, maximo: usize },

    #[error("Pregunta no encontrada: {0}")]
    PreguntaNoEncontrada(Id),
}

// Good: Returns Result
pub fn eliminar_pregunta_por_indice(&mut self, indice: usize) -> Result<Pregunta, ExamenError> {
    if indice >= self.preguntas.len() {
        return Err(ExamenError::IndiceFueraDeRango {
            indice,
            maximo: self.preguntas.len().saturating_sub(1),
        });
    }
    Ok(self.preguntas.remove(indice))
}

// Bad: Can panic
pub fn eliminar_pregunta_por_indice(&mut self, indice: usize) -> Pregunta {
    self.preguntas.remove(indice) // panics if out of bounds!
}
```

### Domain Enums over Hardcoded Strings

**NEVER use hardcoded strings** for values that have a corresponding domain enum. Always use the enum variant and convert with `.to_string()` when a `String` is needed.

This applies to: roles (`Rol`), resources (`Recurso`), actions (`Accion`), and any other domain Value Object that implements `Display`/`FromStr`.

```rust
// Good: use the Rol enum
use crate::autorizacion::domain::value_object::rol::Rol;
let rol = Rol::Psicologo.to_string();

// Bad: hardcoded string
let rol = "psicologo".to_string();
```
