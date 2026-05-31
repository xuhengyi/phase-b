# ch5 Base Output Oracle

- Date: 2026-05-09
- Scope: smoke L1, `ch5` base group.
- Evidence: `tg-rcore-tutorial-checker --ch 5` expected-pattern report recorded in `logs/checker-ch5-base-empty.log`.

## Required Observable Output

A conforming `ch5` smoke snapshot must emit output matching these user-visible patterns at least once:

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

## Forbidden Observable Output

A conforming `ch5` smoke snapshot must not emit:

- `FAIL: T.T`
- `Test sbrk failed!`

## Boundary

This oracle is only a checker-level smoke oracle. It supports L1 user-test consistency and does not replace the semantic `capability@chapter` specs used for L2/L3 comparison.
