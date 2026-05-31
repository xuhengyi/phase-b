# ch7 Base Output Oracle

- Date: 2026-05-09
- Scope: full L1, `ch7` base group.
- Evidence: `logs/checker-ch7-base-empty.log`.

## Required Observable Output

- `Hello, world from user mode program!`
- `Test power_3 OK!`
- `Test power_5 OK!`
- `Test power_7 OK!`
- `Test write A OK!`
- `Test write B OK!`
- `Test write C OK!`
- `Test sbrk almost OK!`
- `exit pass.`
- `hello child process!`
- `child process pid = <non-negative integer>, exit code = <integer>`
- `forktest pass.`
- `file_test passed!`
- `signal_simple: Done`
- `pipetest passed!`
- `pipe_large_test passed!`

## Forbidden Observable Output

- `FAIL: T.T`
- `Test sbrk failed!`

## Boundary

This oracle is checker-level L1 evidence only.
