#!/bin/env bash

set -e

root_dir="$(dirname $(realpath "$0"))"

if ! [ "$MEMCACHE_ADDR" ]; then MEMCACHE_ADDR="127.0.0.1"; fi
if ! [ "$MEMCACHE_PORT" ]; then MEMCACHE_PORT="11211"; fi
# wrapper for memcached usage in the command line
# reduced and tweaked extract of https://gist.github.com/goodevilgenius/11375877
mc_sendmsg() { echo -e "$*\r\nquit\n" | nc $MEMCACHE_ADDR $MEMCACHE_PORT; }
mc_get() { mc_sendmsg "get $1" | awk "/^VALUE $1/{a=1;next}/^END/{a=0}a" ;}
mc_doset() {
	command="$1"
	shift
	key="$1"
	shift
	exptime="$1"
	shift
	val="$*"
	let bytes=$(echo -n "$val"|wc -c)
	mc_sendmsg "$command $key 0 $exptime $bytes\r\n$val"
}
mc_set() { mc_doset set "$@";}
mc_delete() { mc_sendmsg delete "$*";}

get_available_port () {
  read lowest highest < /proc/sys/net/ipv4/ip_local_port_range
  local taken_ports=( $(ss -lntu | tail -n +2 | awk '{ m=match($5, /([0-9]+)$/, ms); if (m) { print ms[1] } }') )

  for port in $(seq $lowest $highest); do
    for taken_i in $(seq 0 ${#taken_ports[@]}); do
      if [ "${taken_ports[$taken_i]}" = "$port" ]; then
        continue 2
      fi
    done

    key="port_taken_$port"
    if [ "$1" = "sync" ] && [ "$(mc_get "$key")" ]; then
      continue
    fi

    mc_set "$key" 0 1

    echo "$port"
    break
  done
}

while [[ "$#" -gt 0 ]]; do
  case $1 in
    # options
    --db-port) export APP_DB_PORT=$2; shift ;;
    --instance) export APP_INSTANCE="$2"; shift ;;
    --dir) export APP_DIR="$2"; shift;;
    --port) export APP_PORT="$2"; shift;;
    # forwarded arguments
    --listen|--admin-credentials-for-test|--signal-file) export RUN_SERVER_EXTRA="$RUN_SERVER_EXTRA $1=$2"; shift;;
    --reset-before-run) RUN_SERVER_EXTRA="$RUN_SERVER_EXTRA $1";;
    # commands
    get_port)
      get_available_port
      exit $?
    ;;
    get_port_sync)
      get_available_port sync
      exit $?
    ;;
    free_port)
      mc_delete "port_taken_$2"
      exit $?
    ;;
    test|test_server|server|db|test_db) CMD="$1";;
    # fallthrough
    *) echo "Unknown parameter: $1"; exit 1 ;;
  esac
  shift
done

PROJECT_DIR="$HOME/.cache/resolritter/library"
TEST_DB_PORT_FILE="$PROJECT_DIR/test_db_port"
if ! [ "$APP_INSTANCE" ]; then export APP_INSTANCE="library"; fi
if ! [ "$APP_DIR" ]; then export APP_DIR="$PROJECT_DIR/$APP_INSTANCE"; fi
if ! [ "$APP_DB_DIR" ]; then export APP_DB_DIR="$APP_DIR/db"; fi
if ! [ "$LOG_DIR" ]; then LOG_DIR="$APP_DIR/log"; fi
if ! [ "$APP_DB_PORT" ]; then
  if [ "$CMD" = "test_server" ]; then
    export APP_DB_PORT=$(cat "$TEST_DB_PORT_FILE")
  fi
  if ! [ "$APP_DB_PORT" ]; then
    export APP_DB_PORT=5432
  fi
fi


mkdir -m 777 -p "$APP_DIR"
DB_URL="postgresql://localhost:$APP_DB_PORT/$APP_INSTANCE?user=$USER"

logging_deps () {
  mkdir -m 777 -p "$LOG_DIR"
}

db_deps () {
  mkdir -m 777 -p "$APP_DB_DIR"
}

run_server () {
  cd "$root_dir/server"
  cargo run -- --db-url="$DB_URL" $RUN_SERVER_EXTRA
}

clean_test_server () {
  mc_delete "port_taken_$APP_PORT"
}

case "$CMD" in
  test_db)
    export APP_DB_PORT="$(get_available_port | tail -n +2)"
    echo "$APP_DB_PORT" > "$TEST_DB_PORT_FILE"
    docker-compose up --force-recreate --renew-anon-volumes db
  ;;
  test_server)
    RUN_SERVER_EXTRA="--log-dir="$LOG_DIR" --log-format="test" $RUN_SERVER_EXTRA"
    trap clean_test_server EXIT HUP INT QUIT TERM
    run_server
  ;;
  db)
    db_deps
    docker-compose run --service-ports --volume "$APP_DB_DIR:/bitnami/postgresql" db
  ;;
  *)
    run_server
  ;;
esac
