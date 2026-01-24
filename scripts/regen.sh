#!/usr/bin/env bash
# Regenerate Rust bindings from a running NetBox instance.
#
# Combines fetch_schema.sh and generate.sh into a single workflow
# with idempotency verification.
#
# Usage:
#   ./scripts/regen.sh [NETBOX_URL]
#
# Environment variables:
#   NETBOX_URL: URL of the NetBox instance (default: http://localhost:8000)
#   NETBOX_TOKEN: Optional API token for authenticated access
#   OPENAPI_GENERATOR_VERSION: Generator version (default: v6.6.0)
#   SKIP_IDEMPOTENCY: Set to 1 to skip idempotency check

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

cd "${REPO_ROOT}"

NETBOX_URL="${1:-${NETBOX_URL:-http://localhost:8000}}"

echo "=== NetBox OpenAPI Regeneration ==="
echo "Target: ${NETBOX_URL}"
echo ""

# Step 1: Fetch schema
echo "Step 1/4: Fetching OpenAPI schema..."
"${SCRIPT_DIR}/fetch_schema.sh" "${NETBOX_URL}"
echo ""

# Step 2: Generate bindings
echo "Step 2/4: Generating Rust bindings..."
"${SCRIPT_DIR}/generate.sh"
echo ""

# Step 3: Verify build
echo "Step 3/4: Verifying build..."
cargo check --workspace
echo ""

# Step 4: Idempotency check
if [[ "${SKIP_IDEMPOTENCY:-}" == "1" ]]; then
    echo "Step 4/4: Skipping idempotency check (SKIP_IDEMPOTENCY=1)"
else
    echo "Step 4/4: Verifying idempotency..."

    # Save current state
    TEMP_DIR=$(mktemp -d)
    cp -r crates/netbox-openapi/src "${TEMP_DIR}/src_before"

    # Regenerate
    "${SCRIPT_DIR}/generate.sh" > /dev/null 2>&1

    # Compare
    if diff -rq "${TEMP_DIR}/src_before" crates/netbox-openapi/src > /dev/null 2>&1; then
        echo "  Idempotency check passed: regeneration produces identical output"
    else
        echo "  WARNING: Idempotency check failed - regeneration produced different output"
        echo "  This may indicate non-deterministic generation or schema changes"
        diff -rq "${TEMP_DIR}/src_before" crates/netbox-openapi/src || true
    fi

    rm -rf "${TEMP_DIR}"
fi

echo ""
echo "=== Regeneration complete ==="
echo ""
echo "Next steps:"
echo "  1. Review changes: git diff crates/netbox-openapi"
echo "  2. Run tests: cargo test --workspace"
echo "  3. Update CHANGELOG.md if schema version changed"
