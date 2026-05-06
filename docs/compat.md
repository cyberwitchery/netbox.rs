# compatibility

maps `netbox.rs` releases to the NetBox upstream versions they are tested against.
the integration CI pins to a specific patch release; compatibility across the minor
series is inferred from the absence of breaking api changes in that range.

| netbox.rs | netbox  | notes                                |
|-----------|---------|--------------------------------------|
| main      | 4.5.x   | CI pinned to v4.5.9 (next 0.3.x)     |
| 0.3.3     | 4.4.x   | CI pinned to v4.4.2                  |

older client releases have not been retroactively tested.

the authoritative pin lives in `.github/workflows/integration.yml`. update the
`main` row when the pin changes; rename it to the released version when a
release is cut, then start a fresh `main` row.
