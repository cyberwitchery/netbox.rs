#!/usr/bin/env bash
# Generate Rust bindings from NetBox OpenAPI schema using openapi-generator
#
# Prerequisites:
#   - openapi-generator-cli (via npm or docker)
#   - jq for JSON processing
#
# Usage:
#   ./scripts/generate.sh

set -euo pipefail

SCHEMA_FILE="scripts/openapi-schema.json"
OUTPUT_DIR="crates/netbox-openapi"
GENERATOR="rust"
OPENAPI_GENERATOR_VERSION="${OPENAPI_GENERATOR_VERSION:-v6.6.0}"
OPENAPI_GENERATOR_IMAGE="${OPENAPI_GENERATOR_IMAGE:-openapitools/openapi-generator-cli:${OPENAPI_GENERATOR_VERSION}}"
PACKAGE_VERSION="${PACKAGE_VERSION:-$(rg -n '^version = ' Cargo.toml | head -n 1 | sed -E 's/.*\"([^\"]+)\".*/\1/')}"
NORMALIZED_SCHEMA_FILE="scripts/openapi-schema.normalized.json"
HOST_OUTPUT_DIR="${OUTPUT_DIR}"

if [ ! -f "$SCHEMA_FILE" ]; then
    echo "Error: Schema file not found: $SCHEMA_FILE"
    echo "Run ./scripts/fetch_schema.sh first"
    exit 1
fi

echo "Generating Rust bindings from ${SCHEMA_FILE}..."

echo "Normalizing schema (replace invalid enum values)..."
python3 - <<'PY' "$SCHEMA_FILE" "$NORMALIZED_SCHEMA_FILE"
import json
import sys

src, dst = sys.argv[1], sys.argv[2]

with open(src, "r", encoding="utf-8") as handle:
    data = json.load(handle)

def normalize(obj):
    if isinstance(obj, dict):
        for key, value in obj.items():
            if key == "enum" and isinstance(value, list):
                obj[key] = ["none" if item == "---------" else item for item in value]
            else:
                normalize(value)
    elif isinstance(obj, list):
        for item in obj:
            normalize(item)

normalize(data)

with open(dst, "w", encoding="utf-8") as handle:
    json.dump(data, handle, ensure_ascii=True)
PY

SCHEMA_FILE="${NORMALIZED_SCHEMA_FILE}"

# Prefer Docker for a pinned, reproducible generator version.
if command -v docker &> /dev/null; then
    echo "Using Docker image for openapi-generator (${OPENAPI_GENERATOR_IMAGE})..."
    GENERATOR_CMD="docker run --rm -v ${PWD}:/local ${OPENAPI_GENERATOR_IMAGE} generate"
    SCHEMA_FILE="/local/${SCHEMA_FILE}"
    OUTPUT_DIR="/local/${OUTPUT_DIR}"
elif command -v openapi-generator-cli &> /dev/null; then
    GENERATOR_CMD="openapi-generator-cli generate"
    CLI_VERSION="$(openapi-generator-cli version 2>/dev/null || true)"
    if [ -n "${CLI_VERSION}" ] && [ "${CLI_VERSION}" != "${OPENAPI_GENERATOR_VERSION#v}" ]; then
        echo "Warning: openapi-generator-cli version ${CLI_VERSION} differs from recommended ${OPENAPI_GENERATOR_VERSION#v}."
    fi
else
    echo "Error: Neither docker nor openapi-generator-cli found"
    echo "Install one of:"
    echo "  - Docker (preferred for the pinned generator image)"
    echo "  - npm install -g @openapitools/openapi-generator-cli"
    exit 1
fi

# Clean previous generated code (keep Cargo.toml)
if [ -d "${OUTPUT_DIR}/src" ]; then
    echo "Cleaning previous generated code..."
    rm -rf "${OUTPUT_DIR}/src"
fi

# Generate the code
$GENERATOR_CMD \
    -i "$SCHEMA_FILE" \
    -g "$GENERATOR" \
    -o "$OUTPUT_DIR" \
    --additional-properties=packageName=netbox-openapi,packageVersion="${PACKAGE_VERSION}"

echo "Applying generated crate lint settings..."
python3 - <<'PY' "${HOST_OUTPUT_DIR}/src/lib.rs"
import sys

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as handle:
    content = handle.read()

attrs = "\n".join(
    [
        "#![allow(non_snake_case)]",
        "#![allow(non_camel_case_types)]",
        "#![allow(non_upper_case_globals)]",
        "",
    ]
)

if attrs not in content:
    content = attrs + content
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(content)
PY

echo "Patching generated Tag model defaults..."
python3 - <<'PY' "${HOST_OUTPUT_DIR}/src/models/tag.rs"
import sys

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as handle:
    content = handle.read()

needle = '#[serde(rename = "tagged_items")]'
replacement = '#[serde(rename = "tagged_items", default)]'

if needle in content and replacement not in content:
    content = content.replace(needle, replacement)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(content)
PY

echo "Patching generated ConfigContext model nullables..."
python3 - <<'PY' "${HOST_OUTPUT_DIR}/src/models/config_context.rs"
import sys

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as handle:
    content = handle.read()

content = content.replace(
    '#[serde(rename = "data_file")]\n    pub data_file: Box<crate::models::BriefDataFile>,',
    '#[serde(rename = "data_file", default, skip_serializing_if = "Option::is_none")]\n    pub data_file: Option<Box<crate::models::BriefDataFile>>,'\n)

content = content.replace(
    "data_file: crate::models::BriefDataFile,",
    "data_file: Option<crate::models::BriefDataFile>,",
)

content = content.replace(
    "data_file: Box::new(data_file),",
    "data_file: data_file.map(Box::new),",
)

with open(path, \"w\", encoding=\"utf-8\") as handle:
    handle.write(content)
PY

echo "Normalizing generated Cargo.toml dependencies..."
python3 - <<'PY' "${HOST_OUTPUT_DIR}/Cargo.toml"
import sys

path = sys.argv[1]
lines = []
in_reqwest = False
seen_default_features = False
seen_features = False

with open(path, "r", encoding="utf-8") as handle:
    for line in handle:
        if line.strip() == "[dependencies.reqwest]":
            in_reqwest = True
            seen_default_features = False
            seen_features = False
            lines.append(line)
            continue

        if in_reqwest:
            if line.startswith("[") and line.strip() != "[dependencies.reqwest]":
                if not seen_default_features:
                    lines.append("default-features = false\n")
                if not seen_features:
                    lines.append('features = ["json", "multipart", "rustls-tls"]\n')
                in_reqwest = False
                lines.append(line)
                continue

            if line.strip().startswith("default-features"):
                lines.append("default-features = false\n")
                seen_default_features = True
                continue
            if line.strip().startswith("features"):
                if "rustls-tls" in line:
                    lines.append(line)
                else:
                    lines.append('features = ["json", "multipart", "rustls-tls"]\n')
                seen_features = True
                continue

        lines.append(line)

if in_reqwest:
    if not seen_default_features:
        lines.append("default-features = false\n")
    if not seen_features:
        lines.append('features = ["json", "multipart", "rustls-tls"]\n')

with open(path, "w", encoding="utf-8") as handle:
    handle.writelines(lines)
PY

echo "Normalizing rustdoc links in generated sources..."
python3 - <<'PY' "${HOST_OUTPUT_DIR}/src"
import os
import re
import sys

root = sys.argv[1]

doc_line_re = re.compile(r'^(?P<prefix>\s*///\s?|\s*//!\s?)(?P<body>.*)$')
anchor_re = re.compile(r'<a href=\\"(https?://[^\\"]+)\\"[^>]*>([^<]+)</a>')
url_re = re.compile(r'(?<!<)(https?://[^\s<>()]+)(?P<punct>[)\].,;:]?)')

def normalize_doc_line(line: str) -> str:
    line_end = "\n" if line.endswith("\n") else ""
    stripped = line[:-1] if line_end else line
    match = doc_line_re.match(stripped)
    if not match:
        return line

    prefix = match.group("prefix")
    body = match.group("body")

    body = anchor_re.sub(r'\2 (<\1>)', body)

    def wrap_url(match: re.Match) -> str:
        url = match.group(1)
        punct = match.group("punct") or ""
        return f"<{url}>{punct}"

    body = url_re.sub(wrap_url, body)
    return f"{prefix}{body}{line_end}"

for dirpath, _, filenames in os.walk(root):
    for filename in filenames:
        if not filename.endswith(".rs"):
            continue
        path = os.path.join(dirpath, filename)
        with open(path, "r", encoding="utf-8") as handle:
            lines = handle.readlines()
        new_lines = [normalize_doc_line(line) for line in lines]
        if new_lines != lines:
            with open(path, "w", encoding="utf-8") as handle:
                handle.writelines(new_lines)
PY

echo "Code generation complete!"
echo "Generated files are in ${OUTPUT_DIR}"
echo ""
echo "Next steps:"
echo "  1. Review generated code"
echo "  2. Run: cargo build"
echo "  3. Run: cargo test"
