# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Quiz Generator is a Rust-based HTTP API for creating exams, composing evaluations, assigning them to candidates (postulantes), and collecting/grading their answers. The system uses a bounded context architecture with MongoDB for persistence, Redis for session management, and JWT authentication.

**Stack:** Rust 2024 edition, Actix Web, MongoDB, Redis, Tokio runtime

## Workspace Structure

The repository uses a Cargo workspace organized by bounded contexts (bctx):

- `bctx/core` - Core domain logic (examen, pregunta, evaluacion, postulante, respuesta)
- `bctx/auth` - Authentication domain (postulante and psicologo login/sessions)
- `bctx/usermgm` - User management support domain
- `bctx/common` - Shared utilities and types
- `cmd/api` - HTTP API service (binary: `quizz`)

### Core Domain Organization

Each domain module in `bctx/core/src/` follows a consistent structure:
- `domain/entity/` - Core business entities
- `domain/value_object/` - Value objects and IDs
- `domain/error/` - Domain-specific errors
- `provider/repositorio.rs` - Repository trait definitions (ports)
- `use_case/` - Application use cases (business operations)

The `pregunta` domain uses the Strategy pattern with multiple question types (alternativa_unica, alternativa_peso, si_no, libre, sola_respuesta).

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
# Format code
make fmt
# or: cargo fmt

# Check formatting (CI-style)
cargo fmt -- --check

# Lint with clippy (fail on warnings)
cargo clippy -- -D warnings

# Run tests with coverage (requires llvm-tools-preview and cargo-llvm-cov)
make test
# or: cargo llvm-cov --show-missing-lines

# Security audit
cargo audit
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
- `mongo.rs` / `redis.rs` - Database client initialization
- `controller/` - HTTP handlers organized by domain (examen, evaluacion, postulante, respuesta, revision, auth)
- `controller/mongo_repository.rs` - MongoDB repository implementations (adapters)

Each controller module typically has:
- `route.rs` - Actix Web route configuration
- `request.rs` / `response.rs` - DTOs for API
- `handler.rs` - HTTP request handlers

### Authentication Flow

Uses JWT tokens stored in Redis for session management. Auth is handled in:
- `bctx/auth/src/postulante/` - Candidate authentication
- `bctx/auth/src/psicologo/` - Psychologist authentication
- `cmd/api/src/controller/auth/` - Login endpoints

### Configuration

Application settings are in `configuration.yaml`:
```yaml
application_port: 8008
application_host: "0.0.0.0"
database:
  host: "127.0.0.1"
  port: 27017
  username: "quizz"
  password: "quizz"
  database_name: "quizz"
```

### API Routes

- `GET /health-check` - Health check endpoint
- `/examen/{id}` - Create exams, add questions
- `/evaluacion/{id}` - Create evaluations, associate exams, publish
- `/postulante` - CRUD operations for candidates
- `/respuesta` - Assign evaluations to candidates, manage exam lifecycle, submit answers
  - `POST /respuesta` - Assign evaluation to candidate (creates respuesta with estado: Creado)
  - `GET /respuesta/postulante/{postulante_id}` - List candidate's respuestas (excludes finalized)
  - `GET /respuesta/{id}/postulante/{postulante_id}` - Get specific respuesta details
  - `PATCH /respuesta/{id}/empezar` - Start exam (Creado → EnProceso, sets fecha_tiempo_inicio)
  - `PATCH /respuesta/{id}` - Submit answers to questions
  - `PATCH /respuesta/{id}/finalizar` - Finalize exam (EnProceso → Finalizado, sets fecha_tiempo_fin)
- `/revision` - Grade and review completed evaluations
  - `GET /respuesta/revision` - List finalized respuestas awaiting review
- `/login` - Authentication endpoints

Example HTTP requests are in `cmd/api/http/*.http` files (use with VS Code/IntelliJ HTTP Client).

## Key Domain Concepts

**Examen (Exam):** A collection of questions (preguntas) grouped together.

**Pregunta (Question):** Individual questions with different types/strategies for validation and scoring.

**Evaluacion (Evaluation):** A composition of one or more exams to be assigned to candidates.

**Postulante (Candidate):** The person taking an evaluation.

**Respuesta (Answer):** Tracks a candidate's assigned evaluation, their submitted answers, and completion status. Contains the full evaluation snapshot at assignment time. Follows a state machine with three estados (states):
- **Creado** - Initial state when evaluation is assigned to candidate
- **EnProceso** - Exam in progress (after empezar endpoint is called, fecha_tiempo_inicio is set)
- **Finalizado** - Exam completed (after finalizar endpoint, fecha_tiempo_fin is set)

**Revision:** The grading/review process for completed evaluations. Only respuestas with estado "Finalizado" are eligible for revision.

## Docker

Build the API image:
```bash
docker build -t quizz-api:local .
```

Run (ensure MongoDB/Redis are accessible per configuration.yaml):
```bash
docker run --rm -p 8008:8008 \
  -v "$PWD/configuration.yaml":/app/configuration.yaml \
  quizz-api:local
```

Note: `docker-compose.dev.yml` only includes MongoDB and Redis, not the API service.

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

### DDD Structure in bounded-contexts

The `bounded-contexts/` directory follows Domain-Driven Design principles:
- **Value Objects**: Immutable objects compared by value (e.g., `AlternativaClave`, `Puntaje`)
- **Entities**: Objects with identity, compared by ID (e.g., `PreguntaAlternativaUnica`)
- **Sum Types**: Use Rust enums for type-safe variants instead of dynamic dispatch (e.g., `Pregunta` enum)

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
