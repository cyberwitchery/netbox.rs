# roadmap

## 0.3.x (stabilization + cli ux)

- issue #4: cli config profiles + token sources (env/command) with `config show|list|validate`
- issue #5: integration harness + golden outputs for cli (table/json) and client CRUD smoke scenarios
- issue #6: regen script (low priority, optional)
- TODO: cli `--output table` column selection (`--columns`) and max column count override
- TODO: expand `--select` docs/examples for nested arrays/objects

## 0.4.0 (library extensibility + observability)

- issue #1: optional tracing instrumentation (feature-gated, no secrets)
- issue #2: request middleware/hooks or configurable reqwest client injection
- TODO: typed graphql response helper (`query<T>`) once api stabilizes
- TODO: per-request retry/backoff tuning (if middleware is insufficient)
