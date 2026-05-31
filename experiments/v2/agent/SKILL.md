# tgos-consistency-agent

You are executing the TGOS multi-implementation consistency experiment.

## First Principle

`experiment-design.md` is the controlling document. If a prompt, script, or generated artifact conflicts with the design, follow the design and record the conflict in `progress.md`.

## Hard Rules

- Work only inside this V2 package when replaying the archived experiment.
- Use Codex CLI/exec and local deterministic scripts only. Do not call external model APIs.
- Source layout is evidence, not the consistency definition.
- The build and test unit is a chapter snapshot: `<impl>@<chapter>`.
- The spec extraction and comparison unit is `capability@chapter`.
- `tg-rcore` is a reference implementation and evidence source, not an absolute oracle.
- `spec-v2` must be language-neutral and layout-neutral.
- RustOS/COS generation and repair must not read `tg-rcore` source files.
- RustOS and COS for the same chapter must trace to the same `bundles/chN.bundle.md`.
- RustOS/COS generated artifacts must be QEMU-runnable RISC-V OS chapter snapshots. Host programs, host conformance runners, or direct host-side checker-output printers are invalid for Step 2/3 success.
- Bug discovery and canonical spec comparison are separate result lines; combine them only in cross analysis.
- Every spec, diff, report, and bug entry must cite evidence paths and, when available, logs.
- Claims must be bounded consistency claims. Do not claim formal equivalence.
- If evidence is insufficient, write `inconclusive`.
- If a chapter/capability is not applicable, write `not_applicable` or `not_implemented_in_this_teaching_chapter`; do not silently skip it.
- If budget/quota risk appears, stop at a clean phase boundary and write resume instructions in `progress.md`.

## Language-Neutral Spec Rules

Do not include Rust/C implementation requirements in `spec-v2` or bundles. In particular, do not require:

- Rust crate/module boundaries.
- C file layout.
- Type names such as `Arc`, `Vec`, trait, struct layout, or unsafe block behavior.
- Internal lock strategy, allocator internals, page table representation, or function names as semantic requirements.

Allowed implementation details only in evidence fields:

- `path`
- `line` or line range, when useful
- `reason`
- `source_layout`

## Required Metadata

Every generated Markdown/YAML artifact must include:

- Inputs
- Prompt version
- Model or Codex version when known
- Date
- Evidence
- Uncertainty or unknowns

## Canonical Spec Schema

Canonical specs should normalize to:

- `id`
- `capability`
- `chapter`
- `level`
- `surface`
- `state_model`
- `observables`
- `allowed_variation`
- `out_of_scope`
- `evidence.callers`
- `evidence.callees`
- `uncertainty`

## Diff Categories

Use only these high-level categories unless a prompt narrows them:

- `equivalent`
- `compatible_variation`
- `weaker_in_tgrcore`
- `weaker_in_rustos`
- `weaker_in_cos`
- `stronger_in_tgrcore`
- `stronger_in_rustos`
- `stronger_in_cos`
- `missing_semantics`
- `caller_conflict`
- `spec_ambiguity`
- `bug_evidence_related`
- `inconclusive`

## Phase Review Checkpoints

- CP1 evidence map: paths map to `capability@chapter`, not to spec structure.
- CP2 bundle: bundle can guide RustOS/COS generation without tg-rcore source or layout leakage.
- CP3 user tests: matrix aligns the same chapter snapshot across tg-rcore, RustOS, and COS.
- CP4 canonical diff: comparison is semantic only, with evidence for every diff.

## Complete Independent OS Artifact Gate

A generated RustOS/COS chapter snapshot is valid only if all of the following hold:

- It builds for a RISC-V target, preferably `riscv64gc-unknown-none-elf` for RustOS and `riscv64-unknown-elf-*` for COS.
- Its `./test.sh base` path runs `qemu-system-riscv64` with a RISC-V kernel image or ELF.
- It is independent from tg-rcore source and from the other generated implementation. It may share `spec-v2` and bundles, but not implementation source.
- It has a hardware/kernel entry path, memory initialization, SBI/console path, trap/syscall entry path, user program loader, scheduler/runtime state, and chapter-specific kernel subsystems described in the bundle.
- For chapters with user programs, it must load independently built user binaries or a documented app image format from the chapter test manifest. Handwritten embedded snippets that only emit oracle strings are invalid.
- It must implement the chapter semantics, not merely produce checker-visible strings. Direct oracle printing from host, kernel, or embedded snippets is invalid.
- By the end of Step 2/3, RustOS and COS must each form a complete independent teaching OS across the full `ch2-ch8` target. A partial/minimal chapter snapshot may be logged as a failed attempt, but it cannot be counted as success.
- Reports must include a `Complete independent OS gate` section with build target, QEMU command, kernel entry, user loader/app image path, trap/syscall path, scheduler/process/memory/file/thread subsystem status as applicable, and any incomplete semantics.

Invalid artifacts must be recorded honestly as failed generation attempts, not counted as passing RustOS/COS implementations.
