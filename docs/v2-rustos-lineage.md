# V2 RustOS lineage

This note explains the V2 RustOS attempt now stored in this repository.

## Repository paths

- `experiments/v2/spec-v2/`: neutral OS capability and chapter specs.
- `experiments/v2/bundles/`: chapter bundles used by the V2 generation run.
- `experiments/v2/rustos/ch2` through `ch8`: accepted RustOS chapter snapshots.
- `experiments/v2/user/`: shared user-space test sources used as build inputs.
- `experiments/v2/tg-rcore-tutorial-{console,signal-defs,syscall}/`: small support crates required by the user tests.

## What V2 is

V2 is a compact, per-chapter RustOS generation attempt. Each chapter is an
independent QEMU/RISC-V teaching OS snapshot with a monolithic `src/main.rs`,
`build.rs`, `linker.ld`, `test.sh`, and an acceptance `report.md`.

V2 is not the same artifact as `trial-workspaces/generated-rust-os/`. The main
Phase B workspace keeps the structured, crate-oriented Rust generation line.
V2 is included as a historical consistency artifact showing that the same
neutral spec and bundle set can drive a compact Rust implementation through the
common base tests.

## Acceptance matrix

| Chapter | V2 RustOS result |
| --- | --- |
| ch2 | `Test PASSED: 5/5` |
| ch3 | `Test PASSED: 4/4` |
| ch4 | `Test PASSED: 6/6` |
| ch5 | `Test PASSED: 14/14` |
| ch6 | `Test PASSED: 15/15` |
| ch7 | `Test PASSED: 18/18` |
| ch8 | `Test PASSED: 22/22` |

## Bundle hashes

| Chapter | Bundle SHA-256 |
| --- | --- |
| ch2 | `f07cd97a5f07493a993f8b6d27736369295c5f54e7774e903c681a037d7592ed` |
| ch3 | `7c98e4585b28f9a31be73252d038fe2426b9add279db2eba2886a045e06d62dd` |
| ch4 | `cb5f15e0b28b169cd3f7d8003b68ee1fdfd2a1ef626889fde42eec9f847524c0` |
| ch5 | `b3863a02704b965f224d46cf2156023bb67e1624ce3f3bb71b8637608ba0f9fe` |
| ch6 | `03cab4e83eb20b66789af32f8411181f0aed5c55ee8e9973caacb79dbae29cb8` |
| ch7 | `b7b033912b0fe6eee1fd8ceddf9416d46456d216759b035962101956ea543795` |
| ch8 | `181f48bf5ac97ea62aefdf002027a17ab98e1e327f9ae89dbd10256978663f0d` |

## Scope

The V2 matrix proves only the accepted base user-visible behavior. It does not
claim full equivalence with tg-rcore, Phase B's modular workspace, exercise
tests, negative paths, or all robustness properties.

