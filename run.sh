#!/bin/env bash

APP_DIR="$HOME/.cache/library_project"

logging_deps () {
  export APP_LOG_DIR="$APP_DIR/log"
  mkdir -m 777 -p "$APP_LOG_DIR"
}

db_deps () {
  export APP_DB_DIR="$APP_DIR/db"
  mkdir -m 777 -p "$APP_DB_DIR"
}

db_vars () {
  export DB_URL="postgresql://localhost:5432/library?user=postgres"
}

run_server () {
  db_vars
  cargo run
}

case "$1" in
  db)
    db_deps
    docker-compose up db
  ;;
  server)
    run_server
  ;;
  seed)
    RESET_AND_SEED=1 run_server
  ;;
  logged)
    logging_deps
    run_server
  ;;
  *)
    if [ "$1" ]; then
      echo "Invalid argument '$1'"
      exit 1
    else
      run_server
    fi
  ;;
esac
