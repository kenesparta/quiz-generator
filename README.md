# Quiz Generator (Quizz API)

A Rust workspace that exposes an HTTP API for creating exams, composing evaluations, assigning them to candidates (postulantes), and collecting their answers. It uses Actix Web for the API and MongoDB for persistence.

- Language: Rust (2024 edition)
- Web: Actix Web
- Database: MongoDB
- Runtime: Tokio
- Workspace crates: core domain, common, auth, usermgm, and API (cmd/api)


## Repository layout

- bctx/core, bctx/common, bctx/auth, bctx/usermgm: core and supporting domain crates
- cmd/api: HTTP API service (binary name: `quizz`)
- configuration.yaml(.example): application configuration
- docker-compose.dev.yml: local MongoDB for development
- Makefile: dev helpers (format, test, compose, etc.)


## Prerequisites

- Rust toolchain (rustup) and Cargo
- Docker and Docker Compose (for local MongoDB)

Recommended developer tooling:
- rustup component add clippy
- rustup component add rustfmt
- rustup component add llvm-tools-preview
- cargo install cargo-llvm-cov
- cargo install cargo-audit


## Quick start (local dev)

1) Start MongoDB with Docker Compose (development profile):

```bash
make dev
# or
docker compose -f docker-compose.dev.yml up -d --build
```

This starts MongoDB on localhost:27017 with credentials quizz/quizz and database quizz.

2) Create your app configuration from the example:

```bash
cp configuration.yaml.example configuration.yaml
```

Default example values:

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

3) Run the API:

```bash
cargo run -p quizz-api --bin quizz
```

The server binds to application_host:application_port (defaults to 0.0.0.0:8008).

4) Health check:

```bash
curl -i http://localhost:8008/health-check
```


## API overview

Routes are grouped by scope:

- GET /health-check
- /examen/{id}
  - POST: create an exam
  - PUT: add questions to an exam
- /evaluacion/{id}
  - POST: create an evaluation
  - PUT: associate exams with an evaluation
  - PATCH: publish an evaluation
- /postulante
  - GET /postulante: get candidate by document (see controller for expected query fields)
  - POST /postulante/{id}: create candidate
  - PUT /postulante/{id}: update candidate
  - DELETE /postulante/{id}: remove candidate
- /respuesta
  - POST /respuesta: assign an evaluation to a candidate
  - GET /respuesta/{id}: get a candidate’s evaluation/answers
  - POST /respuesta/{id}: submit answers for a question in an evaluation

Example requests are provided as HTTP files you can use with VS Code/IntelliJ HTTP Client:

- cmd/api/http/examen1.http
- cmd/api/http/examen2.http
- cmd/api/http/respuesta.http

Set the base URL to http://localhost:8008 and follow the examples.


## Development

- Format code:
  - cargo fmt
  - To check formatting only: `cargo fmt -- --check`

- Lint with clippy (CI-style: fail on warnings):
  - `cargo clippy -- -D warnings`

- Tests and code coverage:
  - Ensure llvm-tools-preview and cargo-llvm-cov are installed
  - `cargo llvm-cov` (use `make test` to include missing-lines report)

- Security audit (advisories):
  - `cargo audit`

Make targets:
- make fmt — format the workspace
- make test — run coverage with missing lines
- make dev — start MongoDB dev container
- make exec — open a shell in the compose service defined for API (if present)


## Docker

A Dockerfile is provided for building the API image. Example build and run (adjust as needed):

```bash
# Build from the workspace root
docker build -t quizz-api:local .

# Run (ensure MongoDB is reachable per configuration.yaml)
docker run --rm -p 8008:8008 \
  -v "$PWD/configuration.yaml":/app/configuration.yaml \
  quizz-api:local
```

Note: The provided docker-compose.dev.yml only includes MongoDB. The API is intended to run locally via Cargo during development.


## Troubleshooting

- Connection refused to MongoDB:
  - Ensure `docker compose -f docker-compose.dev.yml up -d` is running and check port 27017
  - Verify configuration.yaml matches the compose credentials and host

- Address already in use on port 8008:
  - Change `application_port` in configuration.yaml or stop the blocking process

- 404/405 errors on API calls:
  - Verify the route and HTTP method match the route definitions listed above


## License

This project is provided as-is. Add your license information here if applicable.
