# local netbox quickstart

this repo does not run netbox for you. use the official netbox-docker setup.

## setup

follow the instructions at:
https://github.com/netbox-community/netbox-docker

## confirm it is running

ensure `http://localhost:8000` responds and you have a token.

## smoke test

run the local smoke test against your instance:

```bash
NETBOX_TOKEN=... NETBOX_URL=http://localhost:8000 cargo test -p netbox --test smoke_local -- --ignored
```
