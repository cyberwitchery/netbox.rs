# compatibility

maps `netbox.rs` releases to the NetBox upstream versions they are tested against.
the integration CI pins to a specific patch release; compatibility across the minor
series is inferred from the absence of breaking api changes in that range.

| netbox.rs | netbox  | notes                                |
|-----------|---------|--------------------------------------|
| 0.3.x     | 4.4.x   | CI pinned to v4.4.2                  |

older client releases have not been retroactively tested.

the authoritative pin lives in `.github/workflows/integration.yml`.
when the upstream drift check opens an issue, update the pin, run CI, and add
a row here before closing the issue.
