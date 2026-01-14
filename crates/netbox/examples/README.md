# examples

all examples use these environment variables:

- `NETBOX_URL` (default: `http://localhost:8000`)
- `NETBOX_TOKEN` (required for auth)
- `NETBOX_INSECURE` (optional, set to `1` to disable tls verification)

run an example:

```bash
cargo run -p netbox --example status
```

```bash
cargo run -p netbox --example create_tag
```
