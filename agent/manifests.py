from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import tomllib


@dataclass(slots=True)
class CrateConfig:
    name: str
    kind: str
    dir: str
    package: str
    spec: str
    extra_specs: list[str]
    check: list[str]
    tests: list[list[str]]
    chapter: str | None


@dataclass(slots=True)
class ChapterConfig:
    name: str
    command: list[str]
    timeout_seconds: int
    interactive: bool
    input_delay_seconds: float
    input_bytes: str
    required_patterns: list[str]
    forbidden_patterns: list[str]


@dataclass(slots=True)
class ManifestBundle:
    order: list[str]
    crates: dict[str, CrateConfig]
    chapters: dict[str, ChapterConfig]


def _read_toml(path: Path) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def load_manifests(phase_root: Path) -> ManifestBundle:
    crates_path = phase_root / "agent/manifests/crates.toml"
    chapters_path = phase_root / "agent/manifests/chapters.toml"
    crates_raw = _read_toml(crates_path)
    chapters_raw = _read_toml(chapters_path)

    crates: dict[str, CrateConfig] = {}
    for name, cfg in crates_raw["crates"].items():
        crates[name] = CrateConfig(
            name=name,
            kind=cfg["kind"],
            dir=cfg["dir"],
            package=cfg["package"],
            spec=cfg["spec"],
            extra_specs=list(cfg.get("extra_specs", [])),
            check=list(cfg["check"]),
            tests=[list(command) for command in cfg.get("tests", [])],
            chapter=cfg.get("chapter"),
        )

    chapters: dict[str, ChapterConfig] = {}
    for name, cfg in chapters_raw["chapters"].items():
        chapters[name] = ChapterConfig(
            name=name,
            command=list(cfg["command"]),
            timeout_seconds=int(cfg["timeout_seconds"]),
            interactive=bool(cfg.get("interactive", False)),
            input_delay_seconds=float(cfg.get("input_delay_seconds", 0.0)),
            input_bytes=str(cfg.get("input_bytes", "")),
            required_patterns=list(cfg.get("required_patterns", [])),
            forbidden_patterns=list(cfg.get("forbidden_patterns", [])),
        )

    return ManifestBundle(order=list(crates_raw["order"]), crates=crates, chapters=chapters)
