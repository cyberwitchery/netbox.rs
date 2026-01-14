#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${NETBOX_TOKEN:-}" ]]; then
  echo "NETBOX_TOKEN is required" >&2
  exit 1
fi

NETBOX_URL="${NETBOX_URL:-http://localhost:8000}"

echo "Running netbox smoke tests against ${NETBOX_URL}"

NETBOX_URL="${NETBOX_URL}" NETBOX_TOKEN="${NETBOX_TOKEN}" NETBOX_INSECURE="${NETBOX_INSECURE:-}" \
  cargo test -p netbox --test smoke_local -- --ignored
