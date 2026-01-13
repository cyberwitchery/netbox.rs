# cli

this is a small helper for exercising the api.

the cli is not a full feature surface for all endpoints. use `raw` when a dedicated command is missing.

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

## list examples

```bash
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN list-circuits
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN list-virtual-machines
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN list-tunnels
```

## create and update

payloads are json strings or files.

```bash
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN \
  create-circuit --json '{"cid":"CIR-1001","provider":1,"type":1}'

cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN \
  update-virtual-machine 42 --file vm-update.json
```

## raw requests

`raw` is the escape hatch for any endpoint.

```bash
cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN \
  raw --method GET --path api/dcim/devices/ --query "name=leaf-1" --query "limit=5"

cargo run --bin netbox-cli -- --url https://netbox.example.com --token $TOKEN \
  raw --method POST --path api/ipam/vrfs/ --json '{"name":"blue","rd":"65000:100"}'
```

notes:
- `--path` is an api-relative path, e.g. `api/dcim/devices/`
- use `--query key=value` for repeated query params

## tips

- run `netbox-cli --help` to see all subcommands.
- the output is raw json formatted with `serde_json::to_string_pretty`.
