# Bundle: ch6

## Metadata

- chapter: ch6
- prompt_version: direct-codex-smoke-v1
- model_or_codex: current Codex session
- date: 2026-05-09
- inputs:
  - `spec-v2/inventory/ch6.yaml`
  - `spec-v2/riscv-os-requirements.md`
  - `spec-v2/architecture/boot-trap-syscall.md`
  - `spec-v2/architecture/app-image-loader.md`
  - `spec-v2/architecture/subsystem-contracts.md`
  - `spec-v2/capabilities/syscall-abi.md`
  - `spec-v2/capabilities/process-control.md`
  - `spec-v2/capabilities/fork-exec-wait.md`
  - `spec-v2/capabilities/file-system-io.md`
  - `spec-v2/capabilities/fd-table.md`
  - `spec-v2/chapters/ch6.md`
  - `spec-v2/oracles/ch6-base-output.md`

## Generation Boundary

This bundle is the semantic input for both RustOS and COS `ch6` generation. It may depend on ch5 process behavior but does not require any particular source tree layout.

## OS Architecture Contract

- 该章必须继承 ch5 的进程语义，并新增真实 fd table、普通文件对象和 QEMU 内核可访问的文件系统镜像。
- 用户程序与数据文件必须在 app/fs image 中由内核读取；宿主机不能代替内核执行 open/read/write/exec。
- `exec(path)` 从文件系统或等价 app store 读取程序镜像，report 必须说明路径查找与 ELF/程序装载方式。

## Base Required Behavior

- Preserve all base ch5 process-control behavior.
- Load user programs by name from a teaching file system or equivalent chapter snapshot storage.
- Provide standard descriptors `0`, `1`, and `2` for new processes.
- `open(path, flags)` supports read-only, write-only, read-write, create, and truncate flags in the teaching subset.
- Successful `open` returns a descriptor that can be used by the same process.
- `read(fd, buf, len)` reads file bytes into the user buffer, advances the handle offset, returns bytes read, and returns `0` at file end.
- `write(fd, buf, len)` writes user bytes from the buffer, advances the handle offset, and returns bytes written.
- `close(fd)` succeeds on an open descriptor and invalidates it.
- Invalid descriptors, wrong-direction descriptors, invalid user buffers, and missing files return negative failure values.
- `fork` inherits the descriptor table.
- `exec` preserves the descriptor table while replacing the current program image.
- The implementation must be a QEMU-runnable RISC-V OS chapter snapshot with real trap/syscall entry, process state, file state, and fd table state. It cannot satisfy this bundle by running a host program or by printing checker oracle text directly.

## Base User Case Summary

- `filetest_simple`: create `filea`, write `Hello, world!`, close, reopen read-only, read and compare exact bytes.
- `cat_filea`: open `filea`, repeatedly read chunks until `0`, print chunk content, then close.
- Inherited ch5 base cases continue to validate process behavior.

## Base Oracle

The checker-level oracle is `spec-v2/oracles/ch6-base-output.md`. It defines required and forbidden observable output patterns for Step 3, but it is not a substitute for the RISC-V OS gate. A host conformance runner or host-side oracle printer is invalid.

## Complete Independent OS Gate

`./test.sh base` must build a complete independent RISC-V teaching OS image or ELF and run `qemu-system-riscv64`. The report must identify the kernel entry, independent user app loader/app image, trap/syscall path, process table/state, fd table/file state, and file-system implementation status. A host runner, oracle-only QEMU printer, or embedded-snippet kernel is invalid.

## Exercise Extension

These are included for later exercise targeting.

- `fstat(fd, stat_ptr)` writes file status with file type and link count.
- `link(old, new)` creates a new directory entry for the same file and returns `0`, except same-name links fail with `-1`.
- `unlink(path)` removes one name and returns `0`, or returns `-1` if the path does not exist.
- When the final link is removed, later open by that name fails.

## Evidence

- `user/src/bin/filetest_simple.rs`
- `user/src/bin/cat_filea.rs`
- `user/src/bin/ch6_file0.rs`
- `user/src/bin/ch6_file1.rs`
- `user/src/bin/ch6_file2.rs`
- `user/src/bin/ch6_file3.rs`
- `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/main.rs`
- `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/process.rs`
- `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/fs.rs`
- `tg-rcore-tutorial/tg-rcore-tutorial-easy-fs/src/file.rs`
- `tg-rcore-tutorial/tg-rcore-tutorial-ch6/exercise.md`

## Uncertainty

- Hard-link and fstat behavior is exercise-specified but not confirmed in the checked-in ch6 implementation.
- Offset sharing across fork should be explicitly tested or classified during Phase D if it becomes a comparison target.
