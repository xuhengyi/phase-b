from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any
import tomllib


@dataclass(slots=True)
class ExperimentPaths:
    candidate_template: Path
    trial_root: Path
    spec_root: Path
    spec_search_roots: list[Path]
    oracle_root: Path
    artifacts_root: Path


@dataclass(slots=True)
class APIConfig:
    base_url: str
    endpoint: str
    model: str
    api_key_env: str
    timeout_seconds: int
    temperature: float


@dataclass(slots=True)
class GenerationConfig:
    max_rounds_base: int
    max_rounds_chapter: int
    max_context_chars: int
    max_output_chars: int
    failure_tail_lines: int
    allow_backtrack: bool


@dataclass(slots=True)
class CodingCLIConfig:
    command: list[str]
    model: str
    sandbox: str
    approval_policy: str
    use_json_stream: bool


@dataclass(slots=True)
class Settings:
    phase_root: Path
    paths: ExperimentPaths
    api: APIConfig
    generation: GenerationConfig
    coding_cli: CodingCLIConfig


def _deep_merge(base: dict[str, Any], override: dict[str, Any]) -> dict[str, Any]:
    merged = dict(base)
    for key, value in override.items():
        if isinstance(value, dict) and isinstance(merged.get(key), dict):
            merged[key] = _deep_merge(merged[key], value)
        else:
            merged[key] = value
    return merged


def _read_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def load_settings(phase_root: Path, config_override: Path | None = None) -> Settings:
    defaults_path = phase_root / "agent/manifests/defaults.toml"
    merged = _read_toml(defaults_path)
    if config_override is not None:
        merged = _deep_merge(merged, _read_toml(config_override))

    path_cfg = merged["paths"]
    api_cfg = merged["api"]
    generation_cfg = merged["generation"]
    coding_cli_cfg = merged["coding_cli"]

    paths = ExperimentPaths(
        candidate_template=phase_root / path_cfg["candidate_template"],
        trial_root=phase_root / path_cfg["trial_root"],
        spec_root=phase_root / path_cfg["spec_root"],
        spec_search_roots=[phase_root / item for item in path_cfg.get("spec_search_roots", [path_cfg["spec_root"]])],
        oracle_root=phase_root / path_cfg["oracle_root"],
        artifacts_root=phase_root / path_cfg["artifacts_root"],
    )

    return Settings(
        phase_root=phase_root,
        paths=paths,
        api=APIConfig(
            base_url=api_cfg["base_url"].rstrip("/"),
            endpoint=api_cfg["endpoint"],
            model=api_cfg["model"],
            api_key_env=api_cfg["api_key_env"],
            timeout_seconds=int(api_cfg["timeout_seconds"]),
            temperature=float(api_cfg["temperature"]),
        ),
        generation=GenerationConfig(
            max_rounds_base=int(generation_cfg["max_rounds_base"]),
            max_rounds_chapter=int(generation_cfg["max_rounds_chapter"]),
            max_context_chars=int(generation_cfg["max_context_chars"]),
            max_output_chars=int(generation_cfg["max_output_chars"]),
            failure_tail_lines=int(generation_cfg["failure_tail_lines"]),
            allow_backtrack=bool(generation_cfg["allow_backtrack"]),
        ),
        coding_cli=CodingCLIConfig(
            command=[str(item) for item in coding_cli_cfg["command"]],
            model=coding_cli_cfg["model"],
            sandbox=coding_cli_cfg["sandbox"],
            approval_policy=coding_cli_cfg["approval_policy"],
            use_json_stream=bool(coding_cli_cfg["use_json_stream"]),
        ),
    )
