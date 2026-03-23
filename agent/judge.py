from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import os
import pty
import select
import shlex
import subprocess
import time

from .config import Settings
from .manifests import ChapterConfig, CrateConfig
from .workspace import TrialWorkspace, apply_oracle_payload


@dataclass(slots=True)
class CommandResult:
    command: list[str]
    returncode: int
    stdout: str
    stderr: str
    timed_out: bool
    duration_seconds: float


@dataclass(slots=True)
class JudgeResult:
    stage: str
    success: bool
    summary: str
    command_results: list[CommandResult]
    missing_patterns: list[str]
    forbidden_patterns: list[str]


def run_base_crate(
    settings: Settings,
    workspace: TrialWorkspace,
    crate_cfg: CrateConfig,
) -> JudgeResult:
    apply_oracle_payload(workspace, settings, crate_cfg)
    command_results: list[CommandResult] = []

    command_results.append(run_command(crate_cfg.check, workspace.run_root))
    for command in crate_cfg.tests:
        command_results.append(run_command(command, workspace.run_root))

    success = all(result.returncode == 0 and not result.timed_out for result in command_results)
    summary = "all base crate checks passed" if success else _summarize_failure(command_results)
    return JudgeResult(
        stage=crate_cfg.name,
        success=success,
        summary=summary,
        command_results=command_results,
        missing_patterns=[],
        forbidden_patterns=[],
    )


def run_chapter(
    settings: Settings,
    workspace: TrialWorkspace,
    crate_cfg: CrateConfig,
    chapter_cfg: ChapterConfig,
) -> JudgeResult:
    command_results: list[CommandResult] = [run_command(crate_cfg.check, workspace.run_root)]
    if command_results[0].returncode != 0 or command_results[0].timed_out:
        return JudgeResult(
            stage=crate_cfg.name,
            success=False,
            summary=_summarize_failure(command_results),
            command_results=command_results,
            missing_patterns=[],
            forbidden_patterns=[],
        )

    if chapter_cfg.interactive:
        qemu_result = run_interactive_command(
            chapter_cfg.command,
            workspace.run_root,
            timeout_seconds=chapter_cfg.timeout_seconds,
            input_bytes=chapter_cfg.input_bytes.encode("utf-8"),
            input_delay_seconds=chapter_cfg.input_delay_seconds,
            stop_when_patterns=chapter_cfg.required_patterns,
        )
    else:
        qemu_result = run_command(
            chapter_cfg.command,
            workspace.run_root,
            timeout_seconds=chapter_cfg.timeout_seconds,
        )

    command_results.append(qemu_result)

    combined_output = "\n".join([result.stdout + "\n" + result.stderr for result in command_results])
    missing_patterns = [pattern for pattern in chapter_cfg.required_patterns if pattern not in combined_output]
    forbidden_hits = [pattern for pattern in chapter_cfg.forbidden_patterns if pattern in combined_output]
    success = (
        command_results[0].returncode == 0
        and not command_results[0].timed_out
        and not qemu_result.timed_out
        and not missing_patterns
        and not forbidden_hits
    )
    if success:
        summary = "chapter judge passed"
    else:
        parts = []
        if missing_patterns:
            parts.append(f"missing patterns: {missing_patterns}")
        if forbidden_hits:
            parts.append(f"forbidden patterns present: {forbidden_hits}")
        if qemu_result.returncode not in (0, -9):
            parts.append(f"qemu returncode={qemu_result.returncode}")
        if qemu_result.timed_out:
            parts.append("qemu timed out")
        if not parts:
            parts.append(_summarize_failure(command_results))
        summary = "; ".join(parts)

    return JudgeResult(
        stage=crate_cfg.name,
        success=success,
        summary=summary,
        command_results=command_results,
        missing_patterns=missing_patterns,
        forbidden_patterns=forbidden_hits,
    )


def run_command(command: list[str], cwd: Path, timeout_seconds: int | None = None) -> CommandResult:
    start = time.monotonic()
    try:
        completed = subprocess.run(
            command,
            cwd=cwd,
            capture_output=True,
            text=True,
            timeout=timeout_seconds,
        )
        return CommandResult(
            command=command,
            returncode=completed.returncode,
            stdout=completed.stdout,
            stderr=completed.stderr,
            timed_out=False,
            duration_seconds=time.monotonic() - start,
        )
    except subprocess.TimeoutExpired as exc:
        return CommandResult(
            command=command,
            returncode=-9,
            stdout=(exc.stdout or ""),
            stderr=(exc.stderr or ""),
            timed_out=True,
            duration_seconds=time.monotonic() - start,
        )


def run_interactive_command(
    command: list[str],
    cwd: Path,
    timeout_seconds: int,
    input_bytes: bytes,
    input_delay_seconds: float,
    stop_when_patterns: list[str] | None = None,
) -> CommandResult:
    start = time.monotonic()
    master_fd, slave_fd = pty.openpty()
    process = subprocess.Popen(
        command,
        cwd=cwd,
        stdin=slave_fd,
        stdout=slave_fd,
        stderr=slave_fd,
        text=False,
        close_fds=True,
    )
    os.close(slave_fd)
    output = bytearray()
    sent_input = False
    timed_out = False
    satisfied = False

    try:
        while True:
            now = time.monotonic()
            if not sent_input and now - start >= input_delay_seconds and input_bytes:
                os.write(master_fd, input_bytes)
                sent_input = True

            ready, _, _ = select.select([master_fd], [], [], 0.2)
            if ready:
                try:
                    chunk = os.read(master_fd, 4096)
                except OSError:
                    chunk = b""
                if chunk:
                    output.extend(chunk)
                    if _patterns_satisfied(output, stop_when_patterns):
                        satisfied = True
                        process.kill()
                        break

            if process.poll() is not None:
                while True:
                    ready, _, _ = select.select([master_fd], [], [], 0.05)
                    if not ready:
                        break
                    try:
                        chunk = os.read(master_fd, 4096)
                    except OSError:
                        break
                    if not chunk:
                        break
                    output.extend(chunk)
                break

            if now - start > timeout_seconds:
                timed_out = True
                process.kill()
                break
    finally:
        try:
            os.close(master_fd)
        except OSError:
            pass
        if process.poll() is None:
            process.kill()
        process.wait(timeout=5)

    text = output.decode("utf-8", errors="replace")
    return CommandResult(
        command=command,
        returncode=0 if satisfied else (process.returncode if process.returncode is not None else -9),
        stdout=text,
        stderr="",
        timed_out=timed_out and not satisfied,
        duration_seconds=time.monotonic() - start,
    )


def format_command_result(result: CommandResult) -> str:
    command = shlex.join(result.command)
    parts = [
        f"$ {command}",
        f"returncode={result.returncode}, timed_out={result.timed_out}, duration={result.duration_seconds:.2f}s",
    ]
    if result.stdout:
        parts.append("stdout:\n" + result.stdout)
    if result.stderr:
        parts.append("stderr:\n" + result.stderr)
    return "\n".join(parts)


def _summarize_failure(results: list[CommandResult]) -> str:
    for result in results:
        if result.returncode != 0 or result.timed_out:
            return f"command failed: {shlex.join(result.command)}"
    return "unknown failure"


def _patterns_satisfied(output: bytearray, patterns: list[str] | None) -> bool:
    if not patterns:
        return False
    text = output.decode("utf-8", errors="replace")
    return all(pattern in text for pattern in patterns)
