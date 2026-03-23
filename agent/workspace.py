from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any
from datetime import datetime, timezone
import json
import shutil
import subprocess

from .config import Settings
from .manifests import CrateConfig


@dataclass(slots=True)
class TrialWorkspace:
    name: str
    root: Path
    run_root: Path
    state_dir: Path
    logs_dir: Path
    metrics_dir: Path
    reports_dir: Path


def create_trial(settings: Settings, name: str, force: bool = False) -> TrialWorkspace:
    trial_root = settings.paths.trial_root / name
    template_root = settings.paths.candidate_template

    if not template_root.exists():
        raise FileNotFoundError(f"candidate template does not exist: {template_root}")

    if trial_root.exists():
        if not force:
            raise FileExistsError(f"trial already exists: {trial_root}")
        shutil.rmtree(trial_root)

    shutil.copytree(template_root, trial_root)
    state_dir = trial_root / ".rebuild"
    logs_dir = state_dir / "logs"
    metrics_dir = state_dir / "metrics"
    reports_dir = state_dir / "reports"
    logs_dir.mkdir(parents=True, exist_ok=True)
    metrics_dir.mkdir(parents=True, exist_ok=True)
    reports_dir.mkdir(parents=True, exist_ok=True)

    workspace = TrialWorkspace(
        name=name,
        root=trial_root,
        run_root=trial_root,
        state_dir=state_dir,
        logs_dir=logs_dir,
        metrics_dir=metrics_dir,
        reports_dir=reports_dir,
    )
    ensure_oracle_installed(workspace, settings)
    write_json(workspace.state_dir / "trial.json", {"trial": name, "status": "initialized"})
    save_trial_state(
        workspace,
        {
            "trial": name,
            "status": "initialized",
            "mode": None,
            "target": None,
            "last_attempted_crate": None,
            "last_completed_crate": None,
            "last_error": None,
            "pretest": {"ran": False, "success": False, "summary": None},
            "updated_at": utc_now(),
        },
    )
    return workspace


def load_trial(settings: Settings, name: str) -> TrialWorkspace:
    trial_root = settings.paths.trial_root / name
    if not trial_root.exists():
        raise FileNotFoundError(f"trial does not exist: {trial_root}")
    state_dir = trial_root / ".rebuild"
    logs_dir = state_dir / "logs"
    metrics_dir = state_dir / "metrics"
    reports_dir = state_dir / "reports"
    logs_dir.mkdir(parents=True, exist_ok=True)
    metrics_dir.mkdir(parents=True, exist_ok=True)
    reports_dir.mkdir(parents=True, exist_ok=True)
    return TrialWorkspace(
        name=name,
        root=trial_root,
        run_root=trial_root,
        state_dir=state_dir,
        logs_dir=logs_dir,
        metrics_dir=metrics_dir,
        reports_dir=reports_dir,
    )


def crate_root(workspace: TrialWorkspace, crate_cfg: CrateConfig) -> Path:
    return workspace.run_root / crate_cfg.dir


def ensure_oracle_installed(workspace: TrialWorkspace, settings: Settings) -> None:
    marker = workspace.state_dir / "oracle_installed.json"
    if marker.exists():
        return

    install_script = settings.paths.oracle_root / "install_oracle_tests.sh"
    if not install_script.exists():
        raise FileNotFoundError(f"oracle install script does not exist: {install_script}")

    completed = subprocess.run(
        ["bash", str(install_script), str(workspace.run_root)],
        capture_output=True,
        text=True,
        check=False,
    )
    if completed.returncode != 0:
        raise RuntimeError(
            "failed to install oracle tests:\n"
            f"stdout:\n{completed.stdout}\n"
            f"stderr:\n{completed.stderr}"
        )

    write_json(
        marker,
        {
            "installed": True,
            "script": str(install_script),
            "stdout": completed.stdout,
            "stderr": completed.stderr,
        },
    )


def apply_oracle_payload(workspace: TrialWorkspace, settings: Settings, crate_cfg: CrateConfig) -> None:
    _ = crate_cfg
    ensure_oracle_installed(workspace, settings)


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat()


def read_json(path: Path) -> Any | None:
    if not path.exists():
        return None
    return json.loads(path.read_text(encoding="utf-8"))


def trial_state_path(workspace: TrialWorkspace) -> Path:
    return workspace.state_dir / "state.json"


def load_trial_state(workspace: TrialWorkspace) -> dict[str, Any]:
    state = read_json(trial_state_path(workspace))
    if isinstance(state, dict):
        return state
    return {
        "trial": workspace.name,
        "status": "initialized",
        "mode": None,
        "target": None,
        "last_attempted_crate": None,
        "last_completed_crate": None,
        "last_error": None,
        "pretest": {"ran": False, "success": False, "summary": None},
        "updated_at": utc_now(),
    }


def save_trial_state(workspace: TrialWorkspace, state: dict[str, Any]) -> None:
    state["updated_at"] = utc_now()
    write_json(trial_state_path(workspace), state)


def patch_trial_state(workspace: TrialWorkspace, **updates: Any) -> dict[str, Any]:
    state = load_trial_state(workspace)
    state.update(updates)
    save_trial_state(workspace, state)
    return state


def write_json(path: Path, payload: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
