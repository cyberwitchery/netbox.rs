#!/usr/bin/env bash
# Fetch OpenAPI schema from a running NetBox instance
#
# Usage:
#   ./scripts/fetch_schema.sh [NETBOX_URL]
#
# Environment variables:
#   NETBOX_URL: URL of the NetBox instance (default: http://localhost:8000)
#   NETBOX_TOKEN: Optional API token for authenticated access

set -euo pipefail

NETBOX_URL="${1:-${NETBOX_URL:-http://localhost:8000}}"
SCHEMA_FILE="scripts/openapi-schema.json"

echo "Fetching OpenAPI schema from ${NETBOX_URL}/api/schema/..."

# Construct curl command with optional auth header
CURL_CMD="curl -fsSL"
if [ -n "${NETBOX_TOKEN:-}" ]; then
    CURL_CMD="$CURL_CMD -H 'Authorization: Token ${NETBOX_TOKEN}'"
fi

# Fetch and pretty-print the schema
eval "$CURL_CMD '${NETBOX_URL}/api/schema/?format=json'" | \
    jq '.' > "${SCHEMA_FILE}"

echo "Schema saved to ${SCHEMA_FILE}"
echo "Schema info:"
jq -r '.info | "  Version: \(.version)\n  Title: \(.title)"' "${SCHEMA_FILE}"
