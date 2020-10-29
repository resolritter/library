#!/bin/env bash

set -e

while [[ "$#" -gt 0 ]]; do
  case $1 in
    --db-port) export APP_DB_PORT=$2; shift ;;
    --instance) export APP_INSTANCE="$2"; shift ;;
    --signal-file) SIGNAL_FILE="$2"; shift ;;
    --dir) export APP_DIR="$2"; shift ;;
    --listen) export APP_LISTEN_ADDR="$2"; shift ;;
    test|test_server|server|db|test_db) CMD="$1";;
    *) echo "Unknown parameter: $1"; exit 1 ;;
  esac
  shift
done

PROJECT_DIR="$HOME/.cache/resolritter/library"
TEST_DB_PORT_FILE="$PROJECT_DIR/test_db_port"
if ! [ "$APP_INSTANCE" ]; then export APP_INSTANCE="default"; fi
if ! [ "$APP_DIR" ]; then export APP_DIR="$PROJECT_DIR/$APP_INSTANCE"; fi
if ! [ "$APP_LOG_DIR" ]; then export APP_LOG_DIR="$APP_DIR/log"; fi
if ! [ "$APP_DB_DIR" ]; then export APP_DB_DIR="$APP_DIR/db"; fi
if ! [ "$APP_LISTEN_ADDR" ]; then export APP_LISTEN_ADDR="127.0.0.1:8080"; fi
if ! [ "$APP_DB_PORT" ]; then
  if [ "$CMD" = "test_server" ]; then
    export APP_DB_PORT=$(cat "$TEST_DB_PORT_FILE")
  fi
  if ! [ "$APP_DB_PORT" ]; then
    export APP_DB_PORT=5432
  fi
fi


mkdir -m 777 -p "$APP_DIR"
export DB_URL="postgresql://localhost:$APP_DB_PORT/$APP_INSTANCE?user=$USER"

logging_deps () {
  mkdir -m 777 -p "$APP_LOG_DIR"
}

db_deps () {
  mkdir -m 777 -p "$APP_DB_DIR"
}

get_available_port () {
  read lowest highest < /proc/sys/net/ipv4/ip_local_port_range
  local taken_ports=( $(ss -lntu | tail -n +2 | awk '{ m=match($5, /([0-9]+)$/, ms); if (m) { print ms[1] } }' | uniq) )

  for port in $(seq $lowest $highest); do
    for taken_i in $(seq 0 ${#taken_ports[@]}); do
      if [ "${taken_ports[$taken_i]}" = "$port" ]; then
        continue 2
      fi
    done

    echo "$port"
    break
  done
}

case "$CMD" in
  test_db)
    export APP_DB_PORT="$(get_available_port)"
    echo "$APP_DB_PORT" > "$TEST_DB_PORT_FILE"
    docker-compose up --force-recreate --renew-anon-volumes db
  ;;
  test_server)
    logging_deps
    if ! [ "$SIGNAL_FILE" ]; then
      echo "Signal file should be specified before running the test."
      exit 1
    fi

    cargo run -- --reset-before-run --log-format="test" --signal-file="$SIGNAL_FILE"
  ;;
  db)
    db_deps
    docker-compose run --volume "$APP_DB_DIR:/bitnami/postgresql" db
  ;;
  server)
    cargo run -- "$@"
  ;;
  *)
    run_server
  ;;
esac
