#!/bin/env bash

set -e

RUN_OP=""
while [[ "$#" -gt 0 ]]; do
  case $1 in
    --db-port) export APP_DB_PORT=$2; shift ;;
    --instance) export APP_INSTANCE="$2"; shift ;;
    --dir) export APP_DIR="$2"; shift ;;
    --listen) export APP_LISTEN_ADDR="$2"; shift ;;
    --test) export RUN_OP="test";;
    *) echo "Unknown parameter passed: $1"; exit 1 ;;
  esac
  shift
done

if ! [ "$APP_DB_PORT" ]; then export APP_DB_PORT=5432; fi
if ! [ "$APP_INSTANCE" ]; then export APP_INSTANCE="default"; fi
if ! [ "$APP_DIR" ]; then export APP_DIR="$HOME/.cache/resolritter/library/$APP_INSTANCE"; fi
if ! [ "$APP_LISTEN_ADDR" ]; then export APP_LISTEN_ADDR="127.0.0.1:8080"; fi

mkdir -m 777 -p "$APP_DIR"

logging_deps () {
  export APP_LOG_DIR="$APP_DIR/log"
  mkdir -m 777 -p "$APP_LOG_DIR"
}

db_deps () {
  export APP_DB_DIR="$APP_DIR/db"
  mkdir -m 777 -p "$APP_DB_DIR"
  export DB_URL="postgresql://localhost:$APP_DB_PORT/$APP_INSTANCE?user=$USER"
}

run_server () {
  db_deps
  cargo run
}

case "$RUN_OP" in
  test)
    db_deps
    docker-compose up db &
    sleep 5
    logging_deps
    run_server
  ;;
  db)
    db_deps
    docker-compose up db
  ;;
  server)
    run_server
  ;;
  seed)
    # TODO turn this into an argument or handle it in rust
    RESET_AND_SEED=true run_server
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
