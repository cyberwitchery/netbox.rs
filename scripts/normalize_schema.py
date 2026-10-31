#!/usr/bin/env python3
"""normalize the netbox openapi schema so openapi-generator emits usable rust.

usage: normalize_schema.py <schema.json> <normalized.json>
"""
import json
import re
import sys

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

def drop_integer_enums(obj: dict, allow_null: bool = False) -> None:
    if obj.get("type") != "integer":
        return
    enum = obj.get("enum")
    if not isinstance(enum, list):
        return
    if not enum or not all(isinstance(item, int) or (allow_null and item is None) for item in enum):
        return
    obj.pop("enum", None)
    obj.pop("x-spec-enum-id", None)

def drop_enum(obj: dict) -> None:
    obj.pop("enum", None)
    obj.pop("x-spec-enum-id", None)

# fields whose NetBox ChoiceSet has a `key`, so FIELD_CHOICES can replace or extend it
CONFIGURABLE_CHOICES = [
    ("Circuit", "status"),
    ("CircuitGroupAssignment", "priority"),
    ("Cluster", "status"),
    ("CoolingFeed", "status"),
    ("CoolingSource", "status"),
    ("CoolingSource", "type"),
    ("Device", "airflow"),
    ("Device", "status"),
    ("InventoryItem", "status"),
    ("IPAddress", "status"),
    ("IPRange", "status"),
    ("JournalEntry", "kind"),
    ("L2VPN", "status"),
    ("Location", "status"),
    ("Module", "status"),
    ("ModuleType", "airflow"),
    ("PowerFeed", "status"),
    ("PowerOutlet", "status"),
    ("Prefix", "status"),
    ("Rack", "airflow"),
    ("Rack", "status"),
    ("RackReservation", "status"),
    ("Site", "status"),
    ("Tunnel", "status"),
    ("VirtualDeviceContext", "status"),
    ("VirtualMachine", "start_on_boot"),
    ("VirtualMachine", "status"),
    ("VLAN", "status"),
    ("WirelessLAN", "status"),
]

# the model behind a schema name: Site, WritableSiteRequest, PatchedBulkSiteRequest, ... -> Site
model_schema_re = re.compile(r"(?:Patched)?(?:Bulk)?(?:Writable)?(?:Brief)?(\w+?)(?:Serializer_)?(?:Request)?")

def is_configurable(schema: str, field: str) -> bool:
    match = model_schema_re.fullmatch(schema)
    return match is not None and (match.group(1), field) in CONFIGURABLE_CHOICES

def open_choice_fields(schemas: dict) -> None:
    for name, schema in schemas.items():
        for field, prop in (schema.get("properties") or {}).items():
            choice = prop.get("properties") if prop.get("type") == "object" else None
            if isinstance(choice, dict) and set(choice) == {"value", "label"}:
                drop_enum(choice["label"])
                if is_configurable(name, field):
                    drop_enum(choice["value"])
                else:
                    drop_integer_enums(choice["value"], allow_null=True)
            elif is_configurable(name, field):
                drop_enum(prop)

def collapse_string_one_of(obj: dict) -> None:
    # NetBox models blank-able fields (dns_name, color, email, url) as
    # oneOf [constrained string, empty string]. openapi-generator turns an
    # inline oneOf into an empty struct, which fails to deserialize, so
    # collapse it to a plain string.
    branches = obj.get("oneOf")
    if not isinstance(branches, list) or not branches:
        return
    if not all(isinstance(b, dict) and b.get("type") == "string" and "$ref" not in b for b in branches):
        return
    obj.pop("oneOf")
    obj["type"] = "string"
    lengths = [b["maxLength"] for b in branches if "maxLength" in b]
    if lengths:
        obj["maxLength"] = max(lengths)

def is_ref(branch) -> bool:
    if not isinstance(branch, dict):
        return False
    if "$ref" in branch:
        return True
    all_of = branch.get("allOf")
    return isinstance(all_of, list) and len(all_of) == 1 and "$ref" in all_of[0]

def collapse_fk_one_of(obj: dict) -> None:
    # NetBox models writable foreign keys as oneOf [integer id, nested brief
    # object]. openapi-generator turns that into an empty struct that
    # serializes as {}, so collapse it to the integer id.
    branches = obj.get("oneOf")
    if not isinstance(branches, list) or len(branches) != 2:
        return
    ints = [b for b in branches if isinstance(b, dict) and b.get("type") == "integer" and len(b) == 1]
    refs = [b for b in branches if is_ref(b)]
    if len(ints) != 1 or len(refs) != 1:
        return
    obj.pop("oneOf")
    obj["type"] = "integer"
    if refs[0].get("nullable") is True:
        obj["nullable"] = True

def normalize(obj):
    if isinstance(obj, dict):
        drop_readonly_required(obj)
        drop_integer_enums(obj)
        collapse_string_one_of(obj)
        collapse_fk_one_of(obj)
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


def main() -> None:
    src, dst = sys.argv[1], sys.argv[2]
    with open(src, "r", encoding="utf-8") as handle:
        data = json.load(handle)
    open_choice_fields(data["components"]["schemas"])
    normalize(data)
    with open(dst, "w", encoding="utf-8") as handle:
        json.dump(data, handle, ensure_ascii=True)


if __name__ == "__main__":
    main()
