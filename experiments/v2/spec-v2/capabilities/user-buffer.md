# Capability: user-buffer

- Inputs: `spec-v2/architecture/boot-trap-syscall.md`, `spec-v2/capabilities/address-space.md`
- Prompt version: redo-step1-complete-os-capability.v1
- Model/Codex version: Codex session with local `codex-cli 0.130.0`
- Date: 2026-05-09

## Contract

System services that copy bytes or structured values between user memory and kernel state must validate the current caller's user address range and copy through the chapter memory model.

## Observable Requirements

- `write(fd=1/2, buf, len)` copies exactly the requested user bytes to console/debug output and returns the byte count.
- ch6 file reads/writes copy bytes between file state and user buffers without corrupting unrelated memory.
- ch5 wait writes child exit code to a valid user pointer when provided.
- ch7 signal action and signal return structures are copied through user memory.
- ch8 thread/sync calls that take user addresses must not let invalid user pointers corrupt kernel state.

## Error Behavior

- Invalid user buffer pointers or lengths return a negative value or terminate the current user task according to the chapter's fault policy.
- The kernel must not panic or continue with unchecked host/raw pointers.

## Out Of Scope

The current full base L1 does not enumerate an exhaustive invalid-pointer matrix; canonical comparison may later classify exact error codes where tests expose them.
