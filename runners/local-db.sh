#!/usr/bin/env bash

# Launch postgres server at localhost.
# DATABASE_URL=postgres://user:password@localhost:5432/database
docker run -d --rm --name postgres -p 5432:5432 \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_USER=user \
  -e POSTGRES_DB=database \
  postgres:alpine
