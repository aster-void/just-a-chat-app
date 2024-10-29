default: start
# all-in-one script for development.
dev $DATABASE_URL=(LOCAL_DB): dev-db
    just watch

precommit: 
    # precommit must be kept minimal
    cd server; sqlx prepare

check: check-web check-server
    bunx prettier . --check
check-web:
    cd web; bun check
check-server:
    cd server; cargo check
    cd server; cargo clippy
    cd server; cargo fmt --check
fix: fix-web fix-server
    bunx prettier . --write
fix-web:
    # nothing for now
fix-server:
    cd server; cargo fix --allow-staged
    cd server; cargo clippy --fix --allow-staged
    cd server; cargo fmt

watch:
    (trap 'kill 0' EXIT; just watch-web & just watch-server & wait)

watch-web:
    cd web; while true; do bun run watch; done
watch-server:
    cd server; cargo-watch --exec run

start: build
    just serve
build: build-web build-server
build-web:
    cd web; bun run build
build-server:
    cd server; cargo build

serve:
    (trap 'kill 0' EXIT; just serve-web & just serve-server & wait)
serve-web:
    cd web; bun run serve
serve-server:
    cd server; cargo run

sync:
    cd web; bun install --frozen-lockfile

LOCAL_DB := "postgres://user:password@localhost:5432/database"

dev-db:
    docker kill postgres || true
    ./runners/local-db.sh
    until docker exec postgres pg_isready -U user -d database; do \
        sleep 1; \
    done
    cd server; sqlx mig run
    psql "postgres://user:password@localhost:5432/database" -f sql/seed.sql

drop-db:
    cd server; sqlx mig revert || echo 'WARNING: Failed to revert sqlx migration'
    docker kill postgres
