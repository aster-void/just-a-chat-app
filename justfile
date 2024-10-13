default: start
dev $DATABASE_URL=(LOCAL_DB): dev-db
    just watch


watch:
    (trap 'kill 0' EXIT; just watch-web & just watch-server & wait)

watch-web:
    cd web; bun run watch
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
    cd web; bun install

LOCAL_DB := "postgres://user:password@localhost:5432/database"

dev-db:
    docker kill postgres || true
    ./runners/local-db.sh
    until docker exec postgres pg_isready -U user -d database; do \
        sleep 1; \
    done
    cd server; sqlx mig run
drop-db:
    cd server; sqlx mig revert || echo 'WARNING: Failed to revert sqlx migration'
    docker kill postgres
