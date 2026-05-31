# Prompt 30: Generate Or Repair OS

Prompt version: 30_generate_or_repair_os.v1

## Task

Generate or repair one chapter snapshot for RustOS or COS from a chapter bundle.

## Required Variables

- Implementation: `rustos` or `cos`
- Chapter: `chN`
- Mode: `generate` or `repair`
- Output directory: `generated/<impl>/<chapter>/`

## Allowed Inputs

- `agent/SKILL.md`
- `bundles/<chapter>.bundle.md`
- `spec-v2/manifest.json`
- `spec-v2/architecture/*.md`
- `inputs/user-tests-manifest.json`
- Current `generated/<impl>/<chapter>/` files, if repairing
- Current implementation/chapter build or test logs
- Candidate template if explicitly provided under this experiment directory

## Forbidden Inputs

- Any `tg-rcore-tutorial/tg-rcore-tutorial-ch*/src/*` source file.
- Any tg-rcore implementation detail not present in the bundle.
- Canonical spec diffs from Phase D.

## Shared Test Input Boundary

The generated OS may build and run the shared user test programs as Step 3 inputs, but the OS kernel implementation must not depend on tg-rcore kernel source. If `test.sh` uses `tg-rcore-tutorial-user` or another shared test source directory to build user programs, report it explicitly as test input, not implementation input.

## Output Files

- Source files under `generated/<impl>/<chapter>/`
- `generated/<impl>/<chapter>/report.md`

## Requirements

- Implement the chapter snapshot behavior described by the bundle.
- The artifact must be a complete independent RISC-V OS chapter snapshot, not a host conformance runner and not a minimal oracle-emitting kernel.
- `./test.sh base` must build a RISC-V kernel image or ELF and run it with `qemu-system-riscv64`.
- Do not satisfy the checker by running a host binary or by printing oracle lines from the host.
- Do not satisfy the checker by embedding handwritten user snippets whose main purpose is to print oracle lines. User programs must be independently built/loaded from the chapter test manifest or an equivalent app image format.
- Do not claim full OS success unless the implementation provides the chapter's required trap/syscall/process/memory/file/thread semantics and participates in the full `ch2-ch8` OS line.
- Record whether this chapter is standalone or inherited from a previous chapter.
- Record bundle path and hash in report.
- In repair mode, cite the exact log signature and spec entry that motivated each change.
- Do not exceed the configured repair budget.
- If the target cannot be completed, record honest unresolved risks; do not invent fallback behavior.

## Report Fields

- Inputs
- Prompt version
- Model/Codex version
- Date
- Bundle path and hash
- Snapshot type
- Source entry points
- Complete independent OS gate: target triple, QEMU command, kernel entry, app image/user loader, trap/syscall path, scheduler/process/memory/file/thread subsystem status as applicable
- Modified files
- Generation or repair summary
- Build/test result
- Unresolved risks
