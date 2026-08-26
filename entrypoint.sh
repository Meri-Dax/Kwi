#!/usr/bin/env bash

set -e

echo "Running migrations..."
diesel setup \
	--migration-dir /migrations \
	--config-file /diesel.toml

echo "Starting app..."
exec /kwi
