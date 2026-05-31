# ch4 Base Output Oracle

- Date: 2026-05-09
- Scope: full L1, `ch4` base group.
- Evidence: `logs/checker-ch4-base-empty.log`.

## Required Observable Output

- `Test write A OK!`
- `Test write B OK!`
- `Test write C OK!`
- `Test sbrk almost OK!`

## Forbidden Observable Output

- `FAIL: T.T`
- `Test sbrk failed!`

## Boundary

This oracle is checker-level L1 evidence only.
