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
- `/respuesta` - Assign evaluations to candidates, submit answers
- `/revision` - Grade and review completed evaluations
- `/login` - Authentication endpoints

Example HTTP requests are in `cmd/api/http/*.http` files (use with VS Code/IntelliJ HTTP Client).

## Key Domain Concepts

**Examen (Exam):** A collection of questions (preguntas) grouped together.

**Pregunta (Question):** Individual questions with different types/strategies for validation and scoring.

**Evaluacion (Evaluation):** A composition of one or more exams to be assigned to candidates.

**Postulante (Candidate):** The person taking an evaluation.

**Respuesta (Answer):** Tracks a candidate's assigned evaluation, their submitted answers, and completion status. Contains the full evaluation snapshot at assignment time.

**Revision:** The grading/review process for completed evaluations.

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
