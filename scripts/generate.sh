#!/usr/bin/env bash
# generate Rust bindings from NetBox OpenAPI schema using openapi-generator
#
# prerequisites:
#   - openapi-generator-cli (via npm or docker)
#   - jq for JSON processing
#
# usage:
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
import re
import sys

src, dst = sys.argv[1], sys.argv[2]

with open(src, "r", encoding="utf-8") as handle:
    data = json.load(handle)

anchor_re = re.compile(r'<a href="(https?://[^"]+)"[^>]*>([^<]+)</a>')
url_re = re.compile(r'(?<!<)(https?://[^\s<>()]+)(?P<punct>[)\].,;:]?)')

def sanitize_doc_text(text: str) -> str:
    text = anchor_re.sub(r'\2 (<\1>)', text)

    def wrap_url(match: re.Match) -> str:
        url = match.group(1)
        punct = match.group("punct") or ""
        return f"<{url}>{punct}"

    return url_re.sub(wrap_url, text)

def drop_readonly_required(obj: dict) -> None:
    required = obj.get("required")
    props = obj.get("properties")
    if not isinstance(required, list) or not isinstance(props, dict):
        return
    filtered = []
    for name in required:
        prop = props.get(name)
        if isinstance(prop, dict) and prop.get("readOnly") is True:
            continue
        filtered.append(name)
    obj["required"] = filtered

def drop_integer_enums(obj: dict) -> None:
    if obj.get("type") != "integer":
        return
    enum = obj.get("enum")
    if not isinstance(enum, list):
        return
    if not enum or not all(isinstance(item, int) for item in enum):
        return
    obj.pop("enum", None)
    obj.pop("x-spec-enum-id", None)

def normalize(obj):
    if isinstance(obj, dict):
        drop_readonly_required(obj)
        drop_integer_enums(obj)
        for key, value in obj.items():
            if key == "enum" and isinstance(value, list):
                obj[key] = ["none" if item == "---------" else item for item in value]
            elif key in {"description", "summary", "title"} and isinstance(value, str):
                obj[key] = sanitize_doc_text(value)
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

# prefer Docker for a pinned, reproducible generator version.
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
if [ -d "${HOST_OUTPUT_DIR}/src" ]; then
    echo "Cleaning previous generated code..."
    rm -rf "${HOST_OUTPUT_DIR}/src"
fi

# save Cargo.toml before generation — the generator overwrites it with bad metadata
CARGO_TOML_BACKUP="$(mktemp)"
cp "${HOST_OUTPUT_DIR}/Cargo.toml" "${CARGO_TOML_BACKUP}"

# Generate the code
$GENERATOR_CMD \
    -i "$SCHEMA_FILE" \
    -g "$GENERATOR" \
    -o "$OUTPUT_DIR" \
    --additional-properties=packageName=netbox-openapi,packageVersion="${PACKAGE_VERSION}"

# restore Cargo.toml
cp "${CARGO_TOML_BACKUP}" "${HOST_OUTPUT_DIR}/Cargo.toml"
rm "${CARGO_TOML_BACKUP}"

echo "Applying generated crate lint settings..."
python3 - <<'PY' "${HOST_OUTPUT_DIR}/src/lib.rs"
import sys

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as handle:
    content = handle.read()

attrs = "\n".join(
    [
        "#![allow(clippy::all)]",
        "#![allow(non_snake_case)]",
        "#![allow(non_camel_case_types)]",
        "#![allow(non_upper_case_globals)]",
        "#![allow(unexpected_cfgs)]",
        "",
    ]
)

if attrs not in content:
    content = attrs + content
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(content)
PY

echo "Gating per-tag API modules behind cfg(not(docsrs))..."
python3 - <<'PY' "${HOST_OUTPUT_DIR}/src/apis/mod.rs"
import re
import sys

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as handle:
    content = handle.read()

pattern = re.compile(r"^pub mod ([a-z_]+_api);$", re.MULTILINE)
replacement = r"#[cfg(not(docsrs))]\npub mod \1;"
content = pattern.sub(replacement, content)

with open(path, "w", encoding="utf-8") as handle:
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
seen_version = False

with open(path, "r", encoding="utf-8") as handle:
    for line in handle:
        if line.strip() == "[dependencies.reqwest]":
            in_reqwest = True
            seen_default_features = False
            seen_features = False
            seen_version = False
            lines.append(line)
            continue

        if in_reqwest:
            if line.startswith("[") and line.strip() != "[dependencies.reqwest]":
                if not seen_version:
                    lines.append('version = "^0.12"\n')
                if not seen_default_features:
                    lines.append("default-features = false\n")
                if not seen_features:
                    lines.append('features = ["json", "multipart", "rustls-tls"]\n')
                in_reqwest = False
                lines.append(line)
                continue

            if line.strip().startswith("version"):
                lines.append('version = "^0.12"\n')
                seen_version = True
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
    if not seen_version:
        lines.append('version = "^0.12"\n')
    if not seen_default_features:
        lines.append("default-features = false\n")
    if not seen_features:
        lines.append('features = ["json", "multipart", "rustls-tls"]\n')

with open(path, "w", encoding="utf-8") as handle:
    handle.writelines(lines)
PY

echo "Generating generic-FK field table from schema..."
python3 - "${NORMALIZED_SCHEMA_FILE}" "${HOST_OUTPUT_DIR}/src/generic_fk.rs" "${HOST_OUTPUT_DIR}/src/lib.rs" <<'PY'
import json, re, sys

schema_file, out_file, lib_file = sys.argv[1], sys.argv[2], sys.argv[3]
data = json.load(open(schema_file, encoding="utf-8"))
schemas = data.get("components", {}).get("schemas", {})


def request_schema_names(op):
    names, stack = set(), [
        op.get("requestBody", {})
        .get("content", {})
        .get("application/json", {})
        .get("schema", {})
    ]
    while stack:
        s = stack.pop()
        if not isinstance(s, dict):
            continue
        if "$ref" in s:
            names.add(s["$ref"].split("/")[-1])
        for key in ("allOf", "oneOf", "anyOf"):
            stack.extend(s.get(key, []))
        if isinstance(s.get("items"), dict):
            stack.append(s["items"])
    return names


def is_string_prop(pdef):
    if not isinstance(pdef, dict):
        return False
    if pdef.get("type") == "string":
        return True
    for key in ("allOf", "oneOf", "anyOf"):
        for sub in pdef.get(key, []):
            if isinstance(sub, dict) and (sub.get("type") == "string" or "$ref" in sub):
                return True
    return False


def generic_fields(name, seen=None):
    # returns {base_field: encoding}, where encoding is one of
    # "Nested" (a `GenericObjectRequest`), "NestedList" (an array of them), or
    # "Split" (a sibling `<field>_type` content type + `<field>_id` pair).
    seen = seen or set()
    if name in seen or name not in schemas:
        return {}
    seen.add(name)
    sdef = schemas[name] or {}
    out = {}
    props = sdef.get("properties") or {}
    for pname, pdef in props.items():
        if not isinstance(pdef, dict):
            continue
        if "GenericObjectRequest" in (pdef.get("$ref") or ""):
            out[pname] = "Nested"
        elif isinstance(pdef.get("items"), dict) and "GenericObjectRequest" in (
            pdef["items"].get("$ref") or ""
        ):
            out[pname] = "NestedList"
    # split-form generic FK: a `<base>_type` content-type string paired with a
    # `<base>_id`. NetBox models e.g. `assigned_object` and `scope` this way
    # instead of as a `GenericObjectRequest`.
    for pname, pdef in props.items():
        if pname.endswith("_type") and is_string_prop(pdef):
            base = pname[:-5]
            if base + "_id" in props and base not in out:
                out[base] = "Split"
    for sub in sdef.get("allOf", []):
        if isinstance(sub, dict) and "$ref" in sub:
            for field, enc in generic_fields(sub["$ref"].split("/")[-1], seen).items():
                out.setdefault(field, enc)
    return out


# key by app_label.model to match how consumers key NetBox types
entries = {}
for path_name, ops in data.get("paths", {}).items():
    m = re.match(r"/api/([^/]+)/", path_name)
    if not m or not isinstance(ops, dict):
        continue
    app = m.group(1)
    for method in ("post", "put", "patch"):
        op = ops.get(method)
        if not isinstance(op, dict):
            continue
        for sname in request_schema_names(op):
            fields = generic_fields(sname)
            if not fields:
                continue
            model = re.sub(r"(Patched|Writable|Request)", "", sname).lower()
            for field, enc in fields.items():
                entries.setdefault((f"{app}.{model}", field), enc)

pairs = sorted(entries.items())
rows = "".join(
    f'    ("{m}", "{f}", GenericFkEncoding::{enc}),\n' for (m, f), enc in pairs
)

content = f'''//! Generic foreign-key fields, derived from the NetBox OpenAPI schema.
//! Auto-generated by `scripts/generate.sh` - do not edit.

/// How a generic foreign key is encoded in a NetBox write payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericFkEncoding {{
    /// A single nested `{{ "object_type": ..., "object_id": ... }}`.
    Nested,
    /// An array of nested `{{ "object_type": ..., "object_id": ... }}`.
    NestedList,
    /// A sibling scalar pair: a `<field>_type` content type and a `<field>_id`.
    Split,
}}

/// `(app_label.model, field, encoding)` for every generic foreign key in the
/// NetBox write API. `field` is the logical base name; for `Split` it expands to
/// `<field>_type` and `<field>_id` on the wire, otherwise it is the property
/// carrying the (array of) `GenericObjectRequest`.
pub const GENERIC_FK_FIELDS: &[(&str, &str, GenericFkEncoding)] = &[
{rows}];

/// Returns the [`GenericFkEncoding`] for `field` on `model` (keyed as
/// `app_label.model`, e.g. `"dcim.cable"`) if it is a generic foreign key.
pub fn generic_fk_encoding(model: &str, field: &str) -> Option<GenericFkEncoding> {{
    GENERIC_FK_FIELDS
        .iter()
        .find(|(m, f, _)| *m == model && *f == field)
        .map(|(_, _, enc)| *enc)
}}

/// Returns `true` if `field` on `model` is a generic foreign key whose value
/// must be encoded as a content-type reference rather than a bare id.
pub fn is_generic_fk(model: &str, field: &str) -> bool {{
    generic_fk_encoding(model, field).is_some()
}}
'''
with open(out_file, "w", encoding="utf-8") as handle:
    handle.write(content)

with open(lib_file, encoding="utf-8") as handle:
    lib = handle.read()
if "pub mod generic_fk;" not in lib:
    with open(lib_file, "a", encoding="utf-8") as handle:
        handle.write("pub mod generic_fk;\n")
PY

echo "Running cargo fmt..."
cargo fmt --all

echo "Code generation complete!"
echo "Generated files are in ${OUTPUT_DIR}"
echo ""
echo "Next steps:"
echo "  1. Review generated code"
echo "  2. Run: cargo build"
echo "  3. Run: cargo test"
