# netbox-cli

full-featured cli for the netbox api. covers standard crud resources and exposes a raw mode for any endpoint.

## install

from crates.io:

```bash
cargo install netbox-cli
```

from this repo:

```bash
cargo install --path crates/netbox-cli
```

## quickstart

```bash
netbox-cli --url https://netbox.example.com --token $TOKEN dcim devices list
```

## auth

required:
- `--url`
- `--token`

or set:
- `NETBOX_URL`
- `NETBOX_TOKEN`

## resources

list resource groups:

```bash
netbox-cli resources
netbox-cli resources dcim
```

## common commands

list:

```bash
netbox-cli dcim devices list
netbox-cli ipam prefixes list
netbox-cli vpn tunnels list
```

get by id:

```bash
netbox-cli dcim devices get 42
```

create/update with json:

```bash
netbox-cli circuits circuits create --json '{"cid":"CIR-1001","provider":1,"type":1}'
netbox-cli virtualization virtual-machines update 42 --file vm-update.json
```

## raw requests

use `raw` for any endpoint:

```bash
netbox-cli raw --method GET --path dcim/devices/ --query "name=leaf-1" --query "limit=5"
netbox-cli raw --method POST --path ipam/vrfs/ --json '{"name":"blue","rd":"65000:100"}'
```

notes:
- `--path` is api-relative, e.g. `dcim/devices/`
- a leading `api/` is stripped if present
- repeat `--query key=value` for multiple params

## special endpoints

```bash
netbox-cli extras-dashboard get
netbox-cli core-background-queues list
netbox-cli users-config
netbox-cli plugin-branch-action 1 merge --json '{"commit":true}'
```

## output

responses are json, pretty-printed to stdout. errors return non-zero exit codes.

## help

```bash
netbox-cli --help
netbox-cli dcim --help
```
