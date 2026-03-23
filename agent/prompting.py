from __future__ import annotations

from pathlib import Path
import re
import tomllib

from .config import Settings
from .manifests import ChapterConfig, CrateConfig
from .workspace import TrialWorkspace


SYSTEM_PROMPT = """You are implementing one crate in a Rust teaching OS rebuild experiment.
Return JSON only.

JSON shape:
{
  "summary": "short summary",
  "files": [
    {
      "path": "crate-dir/relative/file.rs",
      "content": "full file content"
    }
  ],
  "backtrack_crates": ["optional-crate-name"],
  "notes": "optional notes"
}

Rules:
- Only write files under the currently allowed crate roots.
- Do not modify user/, xtask/, spec/, oracle-tests/, or any path outside the allowed roots.
- Prefer full-file rewrites over partial edits.
- Preserve Cargo package names, features, and module paths.
- Use the provided spec as the source of truth.
- If build/test feedback indicates a dependency protocol mismatch, you may request a backtrack crate in backtrack_crates.
- Write summary and notes in Chinese.
- Before proposing edits, reason from spec, current file layout, and test feedback instead of rewriting unrelated code.
- Never include markdown fences.
"""


def build_api_prompt(
    settings: Settings,
    workspace: TrialWorkspace,
    crate_cfg: CrateConfig,
    chapter_cfg: ChapterConfig | None,
    previous_failure: str | None,
    allowed_backtrack: list[CrateConfig],
) -> str:
    current_spec = _load_spec_bundle(settings, crate_cfg.spec)
    extra_specs = _load_extra_specs(settings, crate_cfg)
    dependency_specs = _load_dependency_specs(settings, workspace, crate_cfg)
    oracle_tests = _load_oracle_bundle(settings, crate_cfg)
    file_snapshot = _snapshot_crate_files(workspace.root / crate_cfg.dir, settings.generation.max_context_chars)
    gate_text = _gate_description(crate_cfg, chapter_cfg)
    input_summary = describe_generation_inputs(settings, workspace, crate_cfg)

    sections = [
        "Task:\nImplement exactly one crate from spec into working Rust code.",
        "Project context:\n" + _load_project_context(settings),
        f"Target crate: {crate_cfg.name}",
        f"Crate directory: {crate_cfg.dir}",
        f"Cargo package: {crate_cfg.package}",
        f"Allowed edit roots: {', '.join(_allowed_roots(crate_cfg, allowed_backtrack))}",
        "Execution requirements:\n"
        "1. Respect the current module tree and Cargo package layout.\n"
        "2. Prefer the smallest coherent implementation that satisfies the spec and tests.\n"
        "3. Preserve behavior already required by prior crates unless feedback proves it wrong.\n"
        "4. When previous failure points to one concrete issue, fix that issue first instead of broad rewrites.\n"
        "5. JSON only, with full-file contents for every changed file.",
        f"Validation gates:\n{gate_text}",
        "Generation inputs:\n" + _format_input_summary(input_summary),
        "Current crate spec:\n" + current_spec,
    ]
    if extra_specs:
        sections.append("Auxiliary specs:\n" + extra_specs)
    if dependency_specs:
        sections.append("Dependency specs:\n" + dependency_specs)
    if oracle_tests:
        sections.append("Oracle tests used by the judge:\n" + oracle_tests)
    sections.append("Current file snapshot:\n" + file_snapshot)
    if chapter_cfg is not None:
        sections.append("Chapter runtime judge:\n" + _chapter_runtime_notes(chapter_cfg))
    if previous_failure:
        sections.append("Previous failure summary:\n" + previous_failure)
    sections.append(
        "Return JSON only. Write every changed file in full. "
        "If no backtrack is needed, return an empty backtrack_crates list."
    )
    prompt = "\n\n".join(sections)
    return prompt[: settings.generation.max_context_chars]


def build_coding_prompt(
    settings: Settings,
    workspace: TrialWorkspace,
    crate_cfg: CrateConfig,
    chapter_cfg: ChapterConfig | None,
    previous_failure: str | None,
) -> str:
    current_spec = _load_spec_bundle(settings, crate_cfg.spec)
    extra_specs = _load_extra_specs(settings, crate_cfg)
    dependency_specs = _load_dependency_specs(settings, workspace, crate_cfg)
    oracle_tests = _load_oracle_bundle(settings, crate_cfg)
    gate_text = _gate_description(crate_cfg, chapter_cfg)
    input_summary = describe_generation_inputs(settings, workspace, crate_cfg)
    file_snapshot = _snapshot_crate_files(workspace.root / crate_cfg.dir, settings.generation.max_context_chars // 3)

    sections = [
        "你正在一个 Rust 教学操作系统重建实验中，作为 Codex 风格的 coding agent 工作。",
        "你的目标是把当前 crate 从 spec 落成可通过判题的实现，而不是只做分析。",
        "硬约束：\n"
        f"1. 只允许修改 `{crate_cfg.dir}/`。\n"
        "2. 严禁修改 user/、xtask/、phase-b/spec/、phase-b/oracle-tests/ 或无关 crate。\n"
        "3. 保持现有 Cargo package 名、feature、模块路径和公共表面。\n"
        "4. 优先最小必要改动，不要顺手大规模重构。\n"
        "5. 章节 crate 不需要人工在终端输入；外层 judge 会自动喂入用户测例。",
        "执行流程：\n"
        "1. 先阅读 project context summary、当前 spec、child spec、直接依赖 spec、oracle tests 和下面提供的当前 crate 文件快照。\n"
        "2. 先确认最小可实现的模块边界，再写代码。\n"
        "3. 编辑后必须在当前 workspace 内自行运行下面列出的 validation gates。\n"
        "4. 如果 gate 失败，在同一次 invocation 内继续修复，直到当前 crate 本地通过或出现明确阻塞。\n"
        "5. 结束时输出中文总结：改了哪些文件、解决了什么问题、还剩什么风险。",
        "附加约束：\n"
        "1. 不要扫描整个仓库；除非被明确阻塞，只读取当前 crate 目录和外层 prompt 已给出的信息。\n"
        "2. 优先编辑 autogen.yaml 里要求的模块文件。\n"
        "3. 先实现能让 `cargo test` 通过的最小公共表面，再考虑内部整理。",
        "Project context summary:\n" + _load_project_context_summary(settings),
        f"Target crate: {crate_cfg.name}",
        f"Allowed edit root: {crate_cfg.dir}",
        f"Validation gates:\n{gate_text}",
        "Generation inputs:\n" + _format_input_summary(input_summary),
        "Current crate spec:\n" + current_spec,
    ]
    if extra_specs:
        sections.append("Auxiliary specs:\n" + extra_specs)
    if dependency_specs:
        sections.append("Direct dependency specs:\n" + dependency_specs)
    if oracle_tests:
        sections.append("Oracle tests used by the judge:\n" + oracle_tests)
    sections.append("Current crate file snapshot:\n" + file_snapshot)
    if chapter_cfg is not None:
        sections.append("Runtime judge details:\n" + _chapter_runtime_notes(chapter_cfg))
    if previous_failure:
        sections.append("上一轮失败摘要：\n" + previous_failure)
    sections.append(
        "请直接在工作区中动手实现并自测。不要把回答停留在方案层。"
    )
    return "\n\n".join(sections)[: settings.generation.max_context_chars]


def _gate_description(crate_cfg: CrateConfig, chapter_cfg: ChapterConfig | None) -> str:
    lines = ["- " + " ".join(crate_cfg.check)]
    if chapter_cfg is None:
        lines.extend("- " + " ".join(command) for command in crate_cfg.tests)
    else:
        lines.append("- " + " ".join(chapter_cfg.command))
        if chapter_cfg.required_patterns:
            lines.append("- required output patterns: " + ", ".join(chapter_cfg.required_patterns))
    return "\n".join(lines)


def _allowed_roots(crate_cfg: CrateConfig, allowed_backtrack: list[CrateConfig]) -> list[str]:
    roots = [crate_cfg.dir]
    roots.extend(crate.dir for crate in allowed_backtrack)
    return roots


def _chapter_runtime_notes(chapter_cfg: ChapterConfig) -> str:
    lines = [
        f"Command: {' '.join(chapter_cfg.command)}",
        f"Timeout: {chapter_cfg.timeout_seconds}s",
        f"Interactive: {chapter_cfg.interactive}",
    ]
    if chapter_cfg.input_bytes:
        lines.append("Input bytes:\n" + chapter_cfg.input_bytes)
    if chapter_cfg.required_patterns:
        lines.append("Required patterns:\n- " + "\n- ".join(chapter_cfg.required_patterns))
    if chapter_cfg.forbidden_patterns:
        lines.append("Forbidden patterns:\n- " + "\n- ".join(chapter_cfg.forbidden_patterns))
    return "\n".join(lines)


def _load_spec_bundle(settings: Settings, spec_name: str) -> str:
    parts = []
    for label, path in _discover_spec_files(settings, spec_name):
        parts.append(f"## {label}\n" + path.read_text(encoding="utf-8"))
    if not parts:
        roots = ", ".join(str(root) for root in settings.paths.spec_search_roots)
        return f"(spec missing for {spec_name}; searched under: {roots})"
    return "\n\n".join(parts)


def _load_project_context(settings: Settings) -> str:
    project_md = settings.phase_root / "spec" / "project.md"
    if project_md.exists():
        return project_md.read_text(encoding="utf-8")
    return "(project context missing at spec/project.md)"


def _load_project_context_summary(settings: Settings) -> str:
    _ = settings
    return (
        "- 这是模块化 rCore-Tutorial 工作区，目标是按教学 OS 语义重建 crate 实现。\n"
        "- 保持现有 crate 名、Cargo package 名、feature 和公共 API 稳定。\n"
        "- 目标环境是 Rust 2021、no_std、riscv64gc-unknown-none-elf、QEMU。\n"
        "- 当前 Phase B 直接以 spec/ 目录中的文本规格为输入，不依赖 OpenSpec CLI 运行。\n"
        "- 优先最小必要实现，先满足当前 crate 契约与测试，不要改动无关目录。"
    )


def _load_extra_specs(settings: Settings, crate_cfg: CrateConfig) -> str:
    spec_names = _collect_extra_spec_names(settings, crate_cfg)

    parts = []
    for spec_name in spec_names:
        bundle = _load_spec_bundle(settings, spec_name)
        if not bundle.startswith("(spec missing"):
            parts.append(bundle)
    return "\n\n".join(parts)


def describe_generation_inputs(settings: Settings, workspace: TrialWorkspace, crate_cfg: CrateConfig) -> dict[str, object]:
    current_spec_files = [label for label, _ in _discover_spec_files(settings, crate_cfg.spec)]
    extra_spec_names = _collect_extra_spec_names(settings, crate_cfg)
    extra_spec_files = {name: [label for label, _ in _discover_spec_files(settings, name)] for name in extra_spec_names}
    dependency_specs = _discover_dependency_spec_names(settings, workspace, crate_cfg)

    oracle_roots = []
    seen_oracle_roots = set()
    for root in (
        settings.paths.oracle_root / "unit" / crate_cfg.name,
        settings.paths.oracle_root / "unit" / crate_cfg.dir,
    ):
        if root.exists():
            rel = str(root.relative_to(settings.phase_root))
            if rel not in seen_oracle_roots:
                seen_oracle_roots.add(rel)
                oracle_roots.append(rel)

    return {
        "project_context": "spec/project.md" if (settings.phase_root / "spec" / "project.md").exists() else None,
        "current_spec_files": current_spec_files,
        "extra_spec_names": extra_spec_names,
        "extra_spec_files": extra_spec_files,
        "dependency_specs": dependency_specs,
        "oracle_roots": oracle_roots,
    }


def _format_input_summary(summary: dict[str, object]) -> str:
    lines = []
    project_context = summary.get("project_context")
    if project_context:
        lines.append(f"- project context: {project_context}")

    current_spec_files = summary.get("current_spec_files") or []
    if current_spec_files:
        lines.append("- current spec files: " + ", ".join(str(item) for item in current_spec_files))

    extra_spec_names = summary.get("extra_spec_names") or []
    if extra_spec_names:
        lines.append("- auxiliary spec names: " + ", ".join(str(item) for item in extra_spec_names))

    extra_spec_files = summary.get("extra_spec_files") or {}
    for name, files in extra_spec_files.items():
        if files:
            lines.append(f"- auxiliary spec `{name}` files: " + ", ".join(str(item) for item in files))

    dependency_specs = summary.get("dependency_specs") or []
    if dependency_specs:
        lines.append("- dependency specs: " + ", ".join(str(item) for item in dependency_specs))

    oracle_roots = summary.get("oracle_roots") or []
    if oracle_roots:
        lines.append("- oracle test roots: " + ", ".join(str(item) for item in oracle_roots))

    return "\n".join(lines) if lines else "(no additional inputs discovered)"


def _load_dependency_specs(settings: Settings, workspace: TrialWorkspace, crate_cfg: CrateConfig) -> str:
    dependency_specs = _discover_dependency_spec_names(settings, workspace, crate_cfg)
    spec_texts = []
    for dep_dir in dependency_specs:
        bundle = _load_spec_bundle(settings, dep_dir)
        if not bundle.startswith("(spec missing"):
            spec_texts.append(f"## dependency {dep_dir}\n" + bundle)
    return "\n\n".join(spec_texts)


def _discover_dependency_spec_names(settings: Settings, workspace: TrialWorkspace, crate_cfg: CrateConfig) -> list[str]:
    cargo_toml = workspace.root / crate_cfg.dir / "Cargo.toml"
    if not cargo_toml.exists():
        return []
    with cargo_toml.open("rb") as handle:
        data = tomllib.load(handle)

    dep_paths: list[str] = []
    for table_name in ("dependencies", "build-dependencies", "dev-dependencies"):
        table = data.get(table_name, {})
        for dep_cfg in table.values():
            if isinstance(dep_cfg, dict) and "path" in dep_cfg:
                dep_paths.append(dep_cfg["path"])

    spec_names = []
    for dep_path in dep_paths:
        dep_dir = Path(dep_path).name
        bundle = _load_spec_bundle(settings, dep_dir)
        if not bundle.startswith("(spec missing"):
            spec_names.append(dep_dir)
    return spec_names


def _load_oracle_bundle(settings: Settings, crate_cfg: CrateConfig) -> str:
    parts = []
    seen = set()
    candidates = (
        settings.paths.oracle_root / "unit" / crate_cfg.name,
        settings.paths.oracle_root / "unit" / crate_cfg.dir,
    )
    for root in candidates:
        if not root.exists():
            continue
        resolved = root.resolve()
        if resolved in seen:
            continue
        seen.add(resolved)
        parts.extend(_snapshot_text_tree(root, f"unit/{root.name}"))
    return "\n\n".join(parts)


def _discover_related_specs(settings: Settings, spec_name: str) -> list[str]:
    names: list[str] = []
    for _, path in _discover_spec_files(settings, spec_name):
        if path.name != "autogen.yaml":
            continue
        for candidate in _extract_spec_refs_from_autogen(path):
            if candidate != spec_name and candidate not in names:
                names.append(candidate)
    return names


def _collect_extra_spec_names(settings: Settings, crate_cfg: CrateConfig) -> list[str]:
    spec_names = list(crate_cfg.extra_specs)
    for spec_name in _discover_related_specs(settings, crate_cfg.spec):
        if spec_name not in spec_names:
            spec_names.append(spec_name)
    return spec_names


def _extract_spec_refs_from_autogen(path: Path) -> list[str]:
    names: list[str] = []
    text = path.read_text(encoding="utf-8")
    for key in ("child_specs", "depends_on_specs"):
        inline_pattern = rf"{key}:\s*\[(.*?)\]"
        for match in re.finditer(inline_pattern, text, flags=re.DOTALL):
            raw_items = match.group(1)
            for item in raw_items.split(","):
                name = item.strip().strip("\"'")
                if name and name not in names:
                    names.append(name)

    active_key: str | None = None
    active_indent = 0
    for raw_line in text.splitlines():
        line = raw_line.rstrip()
        if not line.strip():
            continue
        stripped = line.lstrip()
        indent = len(line) - len(stripped)
        if stripped.startswith(("child_specs:", "depends_on_specs:")):
            key = stripped.split(":", 1)[0]
            if "[" in stripped:
                active_key = None
                continue
            active_key = key
            active_indent = indent
            continue
        if active_key is None:
            continue
        if indent <= active_indent:
            active_key = None
            continue
        if not stripped.startswith("- "):
            continue
        name = stripped[2:].strip().strip("\"'")
        if name and name not in names:
            names.append(name)
    return names


def _snapshot_crate_files(crate_root: Path, max_chars: int) -> str:
    files = []
    if not crate_root.exists():
        return "(crate directory missing)"

    text_patterns = re.compile(r"\.(rs|toml|md|ld|txt|json|yaml|yml)$")
    for path in sorted(crate_root.rglob("*")):
        if not path.is_file():
            continue
        if path.name.startswith("."):
            continue
        if not text_patterns.search(path.name) and path.name != "build.rs":
            continue
        try:
            content = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        rel = path.relative_to(crate_root.parent)
        files.append(f"## {rel}\n{content}")

    snapshot = "\n\n".join(files)
    return snapshot[:max_chars]


def _snapshot_text_tree(root: Path, label: str) -> list[str]:
    files = []
    text_patterns = re.compile(r"\.(rs|toml|md|ld|txt|json|yaml|yml)$")
    for path in sorted(root.rglob("*")):
        if not path.is_file():
            continue
        if path.name.startswith("."):
            continue
        if not text_patterns.search(path.name):
            continue
        try:
            content = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        rel = path.relative_to(root)
        files.append(f"## {label}/{rel}\n{content}")
    return files


def _discover_spec_files(settings: Settings, spec_name: str) -> list[tuple[str, Path]]:
    discovered: list[tuple[str, Path]] = []
    seen: set[Path] = set()
    for root in settings.paths.spec_search_roots:
        if not root.exists():
            continue

        direct_dir = root / spec_name
        if direct_dir.is_dir():
            for file_name in ("spec.md", "design.md"):
                candidate = direct_dir / file_name
                if candidate.exists() and candidate not in seen:
                    discovered.append((f"{spec_name}/{file_name}", candidate))
                    seen.add(candidate)

            autogen = direct_dir / "autogen.yaml"
            if autogen.exists() and autogen not in seen:
                discovered.append((f"{spec_name}/autogen.yaml", autogen))
                seen.add(autogen)

            extra_docs = sorted(
                path for path in direct_dir.glob("*.md") if path.name not in {"spec.md", "design.md"}
            )
            for path in extra_docs:
                if path not in seen:
                    discovered.append((f"{spec_name}/{path.name}", path))
                    seen.add(path)

        flat_candidate = root / f"{spec_name}.md"
        if flat_candidate.exists() and flat_candidate not in seen:
            discovered.append((flat_candidate.name, flat_candidate))
            seen.add(flat_candidate)

    return discovered
