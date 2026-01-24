# netbox-cli

full-featured cli for the netbox api. covers standard crud resources and a raw mode for any endpoint.

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

credentials can come from (in priority order):
1. CLI flags: `--url`, `--token`
2. Environment variables: `NETBOX_URL`, `NETBOX_TOKEN`
3. Config file profiles

## config profiles

create `~/.config/netbox-cli/config.toml`:

```toml
[default]
url = "https://netbox.example.com"
token_env = "NETBOX_TOKEN"

[prod]
url = "https://netbox.prod.example.com"
token_command = "pass show netbox/prod-token"
timeout = 60
ssl_verify = true
output = "table"
```

profile fields:
- `url` - netbox instance url
- `token` - plain token (not recommended)
- `token_env` - env var containing token
- `token_command` - command to get token (e.g., password manager)
- `timeout` - request timeout in seconds
- `retries` - max retry attempts
- `ssl_verify` - verify ssl certificates (default: true)
- `output` - default output format (json, yaml, table)

use a profile:

```bash
netbox-cli --profile prod dcim devices list
netbox-cli -p lab ipam prefixes list
```

manage config:

```bash
netbox-cli config path      # show config file location
netbox-cli config list      # list available profiles
netbox-cli config show      # show current profile
netbox-cli config validate  # validate profile configuration
```

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

## output formats

```bash
netbox-cli dcim devices list --output json
netbox-cli dcim devices list --output yaml
netbox-cli dcim devices list --output table
```

select a field with a dot path:

```bash
netbox-cli dcim devices list --select results
netbox-cli dcim devices list --select results.name
```

### select path examples

the `--select` flag uses dot notation to navigate nested structures. when traversing arrays, the path applies to each element.

**basic field selection:**
```bash
# get just the results array from a paginated response
netbox-cli dcim devices list --select results

# get the count field
netbox-cli dcim devices list --select count
```

**nested object navigation:**
```bash
# get the site object from each device
netbox-cli dcim devices list --select results.site

# get the site name from each device
netbox-cli dcim devices list --select results.site.name

# get the device type's manufacturer name
netbox-cli dcim devices list --select results.device_type.manufacturer.name
```

**array mapping:**

when the path crosses an array, subsequent segments apply to each element:

```bash
# response: {"results": [{"name": "sw1"}, {"name": "sw2"}]}
# --select results.name produces: ["sw1", "sw2"]

# response: {"results": [{"tags": [{"name": "prod"}, {"name": "dc1"}]}, ...]}
# --select results.tags.name produces: [[\"prod\", \"dc1\"], ...]
```

**combining with output formats:**
```bash
# extract names and output as yaml
netbox-cli dcim devices list --select results.name --output yaml

# extract nested field for table display
netbox-cli dcim devices list --select results --output table
```

table output flattens paginated responses by showing the `results` rows and a summary line with count/next/previous when present.

## table column control

specify which columns to show in table output:

```bash
netbox-cli dcim devices list --output table --columns id,name,status
netbox-cli ipam prefixes list --output table --columns prefix,site,status,vlan
```

change the maximum number of auto-selected columns (default: 6):

```bash
netbox-cli dcim devices list --output table --max-columns 10
netbox-cli dcim devices list --output table --max-columns 3
```

when `--columns` is specified, only those columns are shown. when not specified, columns are auto-selected from preferred fields (id, name, display, status, etc.) up to `--max-columns`.

## dry run

print the request for write operations without sending them:

```bash
netbox-cli dcim devices create --json '{"name":"switch-01","device_type":1,"role":1,"site":1}' --dry-run
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

## graphql

read-only graphql queries:

```bash
netbox-cli graphql --query '{ devices { name } }'
netbox-cli graphql --query-file ./query.graphql --vars '{"limit":5}'
```

## special endpoints

```bash
netbox-cli extras-dashboard get
netbox-cli core-background-queues list
netbox-cli users-config
netbox-cli plugin-branch-action 1 merge --json '{"commit":true}'
```

## ipam availability

list and create from available pools:

```bash
netbox-cli ipam-prefix-available-ips 42 list
netbox-cli ipam-prefix-available-ips 42 create --json '[{"description":"allocated"}]'
netbox-cli ipam-prefix-available-prefixes 42 list
netbox-cli ipam-range-available-ips 10 list
netbox-cli ipam-vlan-group-available-vlans 5 list
netbox-cli ipam-asn-range-available-asns 3 list
```

## background task management

control background tasks:

```bash
netbox-cli core-task-action abc123 enqueue
netbox-cli core-task-action abc123 stop
netbox-cli core-task-action abc123 requeue
netbox-cli core-task-action abc123 delete
netbox-cli core-data-source-sync 7
```

## sync and render operations

sync config contexts, templates, and render configs:

```bash
netbox-cli extras-config-context-sync 5
netbox-cli extras-config-context-profile-sync 3
netbox-cli extras-config-template-sync 2
netbox-cli extras-config-template-render 2
netbox-cli extras-export-template-sync 1
netbox-cli extras-custom-field-choices 4
```

## circuit and cable paths

get path information for terminations:

```bash
netbox-cli circuits-termination-paths 10
netbox-cli circuits-virtual-termination-paths 5
```

## dcim trace

trace cable paths through infrastructure:

```bash
netbox-cli dcim-trace interface 99
netbox-cli dcim-trace console-port 10
netbox-cli dcim-trace console-server-port 5
netbox-cli dcim-trace power-port 20
netbox-cli dcim-trace power-outlet 15
netbox-cli dcim-trace power-feed 8
```

## vm config rendering

render configuration for virtual machines:

```bash
netbox-cli virtualization-render-config 42
```

## output

responses are json, pretty-printed to stdout. errors return non-zero exit codes.

exit codes:
- `0` success
- `1` request or parsing error
- `2` invalid command or arguments

errors include http status, path, and request id when available.

## help

```bash
netbox-cli --help
netbox-cli dcim --help
```
