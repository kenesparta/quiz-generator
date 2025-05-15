install-sqlx-cli:
	cargo install --version="~0.8" sqlx-cli \
      --no-default-features \
      --features rustls,postgres

create-migration:
	export DATABASE_URL=postgres://quizz:quizz@localhost:5432/quizz
	sqlx migrate add create_postulante_table

migrate:
	sqlx migrate run --database-url postgres://quizz:quizz@localhost:5432/quizz

db-create:
	sqlx database create --database-url postgres://quizz:quizz@localhost:5432/quizz

sql-prepare:
	cargo sqlx prepare --database-url postgres://quizz:quizz@localhost:5432/quizz