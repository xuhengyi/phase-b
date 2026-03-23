from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import subprocess
import time

from .config import CodingCLIConfig


@dataclass(slots=True)
class CodingCLIInvocationResult:
    returncode: int
    stdout: str
    stderr: str
    duration_seconds: float
    last_message_path: Path


def run_coding_exec(
    prompt: str,
    cwd: Path,
    coding_cli_cfg: CodingCLIConfig,
    last_message_path: Path,
) -> CodingCLIInvocationResult:
    command = [
        _render_token(
            token,
            cwd=cwd,
            model=coding_cli_cfg.model,
            sandbox=coding_cli_cfg.sandbox,
            approval_policy=coding_cli_cfg.approval_policy,
            last_message_path=last_message_path,
        )
        for token in coding_cli_cfg.command
    ]
    if coding_cli_cfg.use_json_stream:
        command.append("--json")

    start = time.monotonic()
    completed = subprocess.run(
        command,
        input=prompt,
        text=True,
        capture_output=True,
    )
    return CodingCLIInvocationResult(
        returncode=completed.returncode,
        stdout=completed.stdout,
        stderr=completed.stderr,
        duration_seconds=time.monotonic() - start,
        last_message_path=last_message_path,
    )


def _render_token(
    token: str,
    *,
    cwd: Path,
    model: str,
    sandbox: str,
    approval_policy: str,
    last_message_path: Path,
) -> str:
    return token.format(
        cwd=str(cwd),
        model=model,
        sandbox=sandbox,
        approval_policy=approval_policy,
        last_message_path=str(last_message_path),
    )
