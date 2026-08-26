#!/usr/bin/env bash

set -e

echo "Running migrations..."

# Migrations are in `./migrations` because of dev. env. volume mounts
diesel setup \
	--migration-dir ./migrations \
	--config-file /diesel.toml

echo "Starting app with hot reload [dev mode]..."
exec cargo watch -x run
