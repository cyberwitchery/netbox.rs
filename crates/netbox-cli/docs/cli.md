# cli

this is a full-featured cli client for the netbox api.

all standard crud resources are exposed as commands. use `raw` for non-standard endpoints.

## install

run from the workspace:

```bash
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN --help
```

## auth

required flags:
- `--url`
- `--token`

you can also set `NETBOX_URL` and `NETBOX_TOKEN` to avoid passing flags.

## resources

list all resources or list a group:

```bash
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN resources
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN resources dcim
```

## list examples

```bash
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN dcim devices list
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN ipam prefixes list
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN vpn tunnels list
```

## create and update

payloads are json strings or files.

```bash
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN \
  circuits circuits create --json '{"cid":"CIR-1001","provider":1,"type":1}'

cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN \
  virtualization virtual-machines update 42 --file vm-update.json
```

## raw requests

`raw` is the escape hatch for any endpoint.

```bash
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN \
  raw --method GET --path dcim/devices/ --query "name=leaf-1" --query "limit=5"

cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN \
  raw --method POST --path ipam/vrfs/ --json '{"name":"blue","rd":"65000:100"}'
```

notes:
- `--path` is an api-relative path, e.g. `dcim/devices/`
- the cli strips a leading `api/` if you include it
- use `--query key=value` for repeated query params

## special endpoints

```bash
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN extras-dashboard get
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN core-background-queues list
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN users-config
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN plugin-branch-action 1 merge --json '{"commit":true}'
```

## tips

- run `netbox-cli --help` to see all subcommands.
- the output is raw json formatted with `serde_json::to_string_pretty`.
