# ch8 Base Output Oracle

- Date: 2026-05-09
- Scope: full L1, `ch8` base group.
- Evidence: `logs/checker-ch8-base-empty.log`.

## Required Observable Output

- `Hello, world from user mode program!`
- `Test power_3 OK!`
- `Test power_5 OK!`
- `Test power_7 OK!`
- `Test write A OK!`
- `Test write B OK!`
- `Test write C OK!`
- `exit pass.`
- `hello child process!`
- `child process pid = <non-negative integer>, exit code = <integer>`
- `forktest pass.`
- `file_test passed!`
- `pipetest passed!`
- `mpsc_sem passed!`
- `philosopher dining problem with mutex test passed!`
- `race adder using spin mutex test passed!`
- `sync_sem passed!`
- `test_condvar passed!`
- `threads with arg test passed!`
- `threads test passed!`

## Forbidden Observable Output

- `FAIL: T.T`
- `Test sbrk failed!`

## Boundary

This oracle is checker-level L1 evidence only. ch8 base no longer requires the ch4 `sbrk` success marker, but still forbids the sbrk failure marker.
