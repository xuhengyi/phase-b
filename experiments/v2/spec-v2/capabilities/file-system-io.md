# Capability: file-system-io

## Metadata

- id: file-system-io
- introduced_in: ch6
- applies_to: ch6, ch7, ch8
- prompt_version: direct-codex-smoke-v1
- model_or_codex: current Codex session
- date: 2026-05-09

## Purpose

Specify user-visible file operations introduced in ch6: opening files by name, reading and writing file content, closing descriptors, and exercise extensions for hard links and file status.

## Public Surface

- `open(path, flags)`
- `read(fd, buffer)`
- `write(fd, buffer)`
- `close(fd)`
- `link(oldpath, newpath)` as exercise extension
- `unlink(path)` as exercise extension
- `fstat(fd, stat_ptr)` as exercise extension

## State Model

- `file`: named persistent byte content in the teaching file system.
- `file_handle`: open reference with readability, writability, and current offset.
- `file_offset`: byte position advanced by reads and writes on that open handle.
- `directory_entry`: name-to-file reference in the root directory scope.
- `link_count`: number of names pointing to the same file for exercise hard-link support.

## Requirements

### Requirement: open creates or finds a file

- introduced_in: ch6
- applies_to: ch6
- requirement: `open(path, flags)` returns a descriptor for an existing file, or creates a file when the create flag is present. Opening a missing file without create fails.
- observable_behavior: Tests assert returned descriptors are positive for successful opens and negative after unlink in exercise scope.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-syscall/src/user.rs:37`
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/main.rs:538`
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/fs.rs:52`
  - `user/src/bin/filetest_simple.rs:16`

### Requirement: read and write transfer bytes and report counts

- introduced_in: ch6
- applies_to: ch6
- requirement: `write(fd, bytes)` copies user bytes to a writable file or console and returns the number of bytes written; `read(fd, buffer)` copies bytes from a readable file or input source and returns the number of bytes read, including `0` at file end.
- observable_behavior: `filetest_simple` writes a string, closes, reopens, reads, and compares exact content; `cat_filea` loops until `read` returns `0`.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/main.rs:457`
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/main.rs:497`
  - `tg-rcore-tutorial/tg-rcore-tutorial-easy-fs/src/file.rs:153`
  - `tg-rcore-tutorial/tg-rcore-tutorial-easy-fs/src/file.rs:172`
  - `user/src/bin/filetest_simple.rs:19`
  - `user/src/bin/cat_filea.rs:21`

### Requirement: close invalidates one descriptor

- introduced_in: ch6
- applies_to: ch6
- requirement: `close(fd)` succeeds for an open descriptor and makes that descriptor unavailable for subsequent operations in the same process.
- observable_behavior: Tests close file descriptors after write/read flows and rely on later opens to produce usable descriptors.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/src/main.rs:571`
  - `user/src/bin/filetest_simple.rs:20`

### Requirement: hard-link and file-status exercise extension

- introduced_in: ch6
- applies_to: ch6
- status: exercise_extension
- requirement: `link` creates another name for the same file unless source and destination names are the same; `unlink` removes one name and eventually deletes the file when no links remain; `fstat` reports file type and link count through a user-writable status object.
- observable_behavior: `ch6_file1`, `ch6_file2`, and `ch6_file3` assert file type, link count, same file identity, and open failure after unlink.
- evidence:
  - `tg-rcore-tutorial/tg-rcore-tutorial-ch6/exercise.md:5`
  - `user/src/bin/ch6_file1.rs:19`
  - `user/src/bin/ch6_file2.rs:18`
  - `user/src/bin/ch6_file3.rs:17`
- uncertainty: current checked-in ch6 implementation logs these operations as not implemented and returns `-1`.

## Out Of Scope

- Nested directories.
- Permissions beyond the teaching readable/writable flags.
- Full file metadata beyond fields required by exercise tests.
- Concurrent file access ordering beyond the selected tests.

## Uncertainty

- Base file I/O is directly supported by implementation evidence. Hard-link and fstat behavior is exercise-spec evidence and requires implementation before exercise tests can pass.
