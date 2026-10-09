# Quiz Generator (Quizz API)

A Rust workspace that exposes an HTTP API for creating exams, composing evaluations, assigning them to candidates (postulantes), collecting their answers, and grading them. It uses Actix Web for the API, MongoDB for persistence, Redis for session management, and JWT + Casbin for authentication and RBAC.

- Language: Rust (2024 edition)
- Web: Actix Web
- Database: MongoDB
- Session store: Redis
- Auth: JWT + Casbin (RBAC)
- Runtime: Tokio
- Workspace crates: core domain, common, auth, and API (cmd/api)


## Repository layout

- `bctx/core`, `bctx/common`, `bctx/auth`: core and supporting domain crates organized by bounded context
- `cmd/api`: HTTP API service (binary name: `quizz`)
- `rbac/`: Casbin RBAC model (`model.conf`) and policies (`policy.csv`)
- `configuration.yaml(.example)`: application configuration
- `docker-compose.dev.yml`: local MongoDB and Redis for development
- `Makefile`: dev helpers (format, test, compose, etc.)


## Prerequisites

- Rust toolchain (rustup) and Cargo
- Docker and Docker Compose (for local MongoDB and Redis)

Recommended developer tooling:
- `rustup component add clippy`
- `rustup component add rustfmt`
- `rustup component add llvm-tools-preview`
- `cargo install cargo-llvm-cov`
- `cargo install cargo-audit`


## Quick start (local dev)

1) Start MongoDB and Redis with Docker Compose (development profile):

```bash
make dev
# or
docker compose -f docker-compose.dev.yml up -d --build
```

This starts MongoDB on `localhost:27017` (credentials `quizz`/`quizz`, database `quizz`) and Redis on `localhost:6379`.

2) Create your app configuration from the example and set a JWT secret:

```bash
cp configuration.yaml.example configuration.yaml
export QUIZZ_JWT__SECRET="$(openssl rand -base64 48)"
```

`configuration.yaml.example` documents every key. Any key can be overridden with a `QUIZZ_<SECTION>__<KEY>` environment variable (for example `QUIZZ_DATABASE__PASSWORD`); in deployed environments, pass secrets that way. The server refuses to start with a JWT secret shorter than 32 bytes.

3) Run the API:

```bash
cargo run -p quizz-api --bin quizz
```

The server binds to `application_host:application_port` (defaults to `0.0.0.0:8008`).

4) Health check:

```bash
curl -i http://localhost:8008/health-check
```


## Authentication and Authorization

The API uses a **single universal login** endpoint and JWT-based sessions tracked in Redis.

- `POST /login` accepts `{ "documento": "...", "password": "..." }` and searches across `admin → psicologo → postulante` collections to find the user. Returns a JWT containing the appropriate role.
- `POST /login` gives the same `401` (and takes the same time) for an unknown `documento` and for a wrong password. Attempts are limited per client IP and failures per `documento` (defaults: 20 attempts and 10 failures per 15 minutes, see `login:` in `configuration.yaml.example`); over the limit it answers `429` with `Retry-After`. Passwords longer than 72 bytes are rejected (bcrypt would ignore the rest).
- Every token carries a session id (`jti`). Redis keeps the open session of each user (`sesion:{user id}`), and every protected request checks it: a token is accepted only while its session is open. Logging in again replaces the previous session.
- `POST /logout` requires `Authorization: Bearer <token>` and closes that session, so the token is rejected (`401`) from then on, even before it expires. It responds with `204` even if the token is already expired or closed (so clients can clean up local state).

Authorization is enforced by an Actix middleware that verifies the JWT and consults a **Casbin RBAC enforcer** built from `rbac/model.conf` and `rbac/policy.csv`. Roles:

- `admin` — full access to all resources
- `psicologo` — manage exams, evaluations, candidates, and reviews
- `postulante` — read/write/update own `respuestas` only

**Public routes** (no auth): `/health-check`, `/login`.
**Protected routes** (JWT + RBAC): everything else.


## API overview

Routes are grouped by scope. List endpoints return HATEOAS-style responses embedding `_links` for navigation.

- `GET /health-check`
- `/examenes`
  - `GET /examenes` — list exams
  - `POST /examenes/{id}` — create an exam
  - `PUT /examenes/{id}` — add a question to an exam
- `/evaluaciones`
  - `GET /evaluaciones` — list evaluations
  - `POST /evaluaciones/{id}` — create an evaluation
  - `PUT /evaluaciones/{id}` — associate exams with an evaluation
  - `PATCH /evaluaciones/{id}` — publish an evaluation
  - `POST /evaluaciones/{evaluacion_id}/respuestas` — assign evaluation to a candidate (creates respuesta with estado `Creado`)
- `/postulantes`
  - `GET /postulantes` — search candidate by document (query param)
  - `PUT /postulantes` — update candidate by document (query param)
  - `POST /postulantes/{id}` — create candidate
  - `DELETE /postulantes/{id}` — not implemented yet (`501`)
- `/respuestas`
  - `GET /respuestas` — list respuestas
  - `GET /respuestas/asignaciones` — list assignments with their evaluation context
  - `GET /respuestas/{id}` — get a specific respuesta
  - `PATCH /respuestas/{id}/estado` — transition state (body: `{"accion":"empezar"}` or `{"accion":"finalizar"}`)
    - `empezar`: `Creado → EnProceso` (sets `fecha_tiempo_inicio`)
    - `finalizar`: `EnProceso → Finalizado` (sets `fecha_tiempo_fin`)
  - `POST /respuestas/{id}/examenes/{examen_id}/preguntas/{pregunta_id}/contestaciones` — submit answer to a question
- `/revisiones`
  - `GET /revisiones` — list revisiones
  - `GET /revisiones/{revision_id}` — get a specific revision
  - `POST /revisiones/{revision_id}` — review evaluation for a candidate (also accepts `PATCH`)
- `POST /login` — universal login (returns JWT with role)
- `POST /logout` — close the session (the token stops working)

Example requests are provided as HTTP files you can use with VS Code/IntelliJ HTTP Client under `cmd/api/http/dev/`:

- `examen1.http`, `examen2.http`, `examen3_entrevista.http`
- `evaluacion.http`
- `postulante.http`, `psicologo.http`, `admin.http`
- `respuesta.http`
- `auth/`, `revision/`

Set the base URL to `http://localhost:8008` and follow the examples.


## Development

- Format code:
  - `cargo fmt`
  - Check formatting only: `cargo fmt -- --check`

- Lint with clippy (CI-style: fail on warnings):
  - `cargo clippy -- -D warnings`

- Tests:
  - `cargo test --workspace` runs the unit tests and the end-to-end test `cmd/api/tests/api_e2e.rs`, which starts throwaway MongoDB and Redis containers with testcontainers, so **Docker must be running**. Run it alone with `cargo test -p quizz-api --test api_e2e`.

- Tests and code coverage:
  - Ensure `llvm-tools-preview` and `cargo-llvm-cov` are installed
  - `cargo llvm-cov` (use `make test` to include the missing-lines report)

- Security audit (advisories):
  - `cargo audit`

Make targets:
- `make fmt` — format the workspace
- `make test` — run coverage with missing lines
- `make dev` — start MongoDB and Redis dev containers
- `make exec` — open a shell in the compose service defined for API (if present)


## Docker

A `Dockerfile` is provided for building the API image:

```bash
# Build from the workspace root
docker build -t quizz-api:local .

# Run (ensure MongoDB and Redis are reachable per configuration.yaml)
docker run --rm -p 8008:8008 \
  -v "$PWD/configuration.yaml":/app/configuration.yaml \
  quizz-api:local
```

Note: `docker-compose.dev.yml` only includes MongoDB and Redis. The API is intended to run locally via Cargo during development.


## Troubleshooting

- Connection refused to MongoDB or Redis:
  - Ensure `docker compose -f docker-compose.dev.yml up -d` is running and check ports `27017` and `6379`
  - Verify `configuration.yaml` matches the compose credentials and host

- Address already in use on port 8008:
  - Change `application_port` in `configuration.yaml` or stop the blocking process

- 401/403 errors on protected routes:
  - Make sure you are sending `Authorization: Bearer <token>` and that the role mapped to your JWT has permission for the resource/action in `rbac/policy.csv`

- 404/405 errors on API calls:
  - Verify the route and HTTP method match the route definitions listed above


## License

This project is provided as-is. Add your license information here if applicable.
