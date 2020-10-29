#!/bin/env bash

db_deps () {
  export DB_DIR="$HOME/.cache/library_management/db"
  mkdir -m 777 -p "$DB_DIR"
}

case "$1" in
  db)
    db_deps
    docker-compose up db
  ;;
  *)
    if [ "$1" ]; then
      echo "Invalid argument '$1'"
      exit 1
    else
      db_deps
      docker-compose up
    fi
  ;;
esac
