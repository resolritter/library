#!/bin/env bash

db_deps () {
  export DB_DIR="$HOME/.cache/library_management/db"
  mkdir -m 777 -p "$DB_DIR"
}

db_vars () {
  export DB_URL="postgresql://localhost:5432/library"
}

case "$1" in
  db)
    db_deps
    docker-compose up db
  ;;
  server)
    db_vars
    cargo run
  ;;
esac
