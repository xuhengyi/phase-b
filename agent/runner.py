from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any
from typing import Iterable
import shlex

from .coding_cli import run_coding_exec
from .config import Settings
from .judge import JudgeResult, format_command_result, run_base_crate, run_chapter, run_command
from .manifests import ChapterConfig, CrateConfig, ManifestBundle
from .model_api import EditProposal, ModelClient, proposal_to_jsonable
from .prompting import SYSTEM_PROMPT, build_api_prompt, build_coding_prompt, describe_generation_inputs
from .workspace import (
    TrialWorkspace,
    create_trial,
    ensure_oracle_installed,
    load_trial,
    load_trial_state,
    patch_trial_state,
    read_json,
    write_json,
)


@dataclass(slots=True)
class RunOutcome:
    crate: str
    success: bool
    rounds: int
    summary: str
    judge_result: JudgeResult


@dataclass(slots=True)
class RunAllResult:
    mode: str
    target: str
    success: bool
    resumed: bool
    crate_outcomes: list[RunOutcome]
    pretest_result: JudgeResult | None
    next_crate: str | None


@dataclass(slots=True)
class TrialReport:
    trial: str
    status: str
    mode: str | None
    target: str | None
    completed_crates: list[str]
    next_crate: str | None
    pretest_success: bool
    last_error: str | None
    crate_metrics: list[dict[str, Any]]


class ExperimentRunner:
    def __init__(self, settings: Settings, manifests: ManifestBundle) -> None:
        self.settings = settings
        self.manifests = manifests

    def init_trial(self, name: str, force: bool = False) -> TrialWorkspace:
        return create_trial(self.settings, name, force=force)

    def load_trial(self, name: str) -> TrialWorkspace:
        return load_trial(self.settings, name)

    def run_api_crate(self, trial_name: str, crate_name: str) -> RunOutcome:
        workspace = self.load_trial(trial_name)
        crate_cfg = self._crate(crate_name)
        chapter_cfg = self._chapter(crate_cfg)
        rounds = self.settings.generation.max_rounds_chapter if crate_cfg.kind == "chapter" else self.settings.generation.max_rounds_base
        client = ModelClient(self.settings.api)
        previous_failure: str | None = None
        self._mark_crate_started(workspace, crate_cfg.name, "api")

        for round_index in range(1, rounds + 1):
            proposal = client.generate_edit_proposal(
                SYSTEM_PROMPT,
                build_api_prompt(
                    self.settings,
                    workspace,
                    crate_cfg,
                    chapter_cfg,
                    previous_failure,
                    self._allowed_backtrack_crates(crate_cfg, previous_failure),
                ),
            )
            self._apply_proposal(workspace, crate_cfg, proposal)
            judge_result = self._judge(workspace, crate_cfg, chapter_cfg)
            self._write_round_log(workspace, crate_cfg.name, round_index, proposal, judge_result, "api")
            if judge_result.success:
                self._write_metric(workspace, crate_cfg.name, round_index, judge_result, "api")
                self._mark_crate_finished(workspace, crate_cfg.name, "api", True, judge_result.summary)
                self._write_crate_report(workspace, crate_cfg, "api", round_index, judge_result)
                self.build_report(trial_name, mode="api")
                return RunOutcome(crate=crate_cfg.name, success=True, rounds=round_index, summary=judge_result.summary, judge_result=judge_result)
            previous_failure = self._failure_feedback(judge_result)

        self._write_metric(workspace, crate_cfg.name, rounds, judge_result, "api")
        self._mark_crate_finished(workspace, crate_cfg.name, "api", False, judge_result.summary)
        self._write_crate_report(workspace, crate_cfg, "api", rounds, judge_result)
        self.build_report(trial_name, mode="api")
        return RunOutcome(crate=crate_cfg.name, success=False, rounds=rounds, summary=judge_result.summary, judge_result=judge_result)

    def run_coding_crate(self, trial_name: str, crate_name: str) -> RunOutcome:
        workspace = self.load_trial(trial_name)
        crate_cfg = self._crate(crate_name)
        chapter_cfg = self._chapter(crate_cfg)
        rounds = self.settings.generation.max_rounds_chapter if crate_cfg.kind == "chapter" else self.settings.generation.max_rounds_base
        previous_failure: str | None = None
        self._mark_crate_started(workspace, crate_cfg.name, "coding")

        for round_index in range(1, rounds + 1):
            prompt = build_coding_prompt(self.settings, workspace, crate_cfg, chapter_cfg, previous_failure)
            log_path = workspace.logs_dir / f"{crate_cfg.name}.coding.round{round_index}.last_message.txt"
            coding_result = run_coding_exec(prompt, workspace.run_root, self.settings.coding_cli, log_path)
            judge_result = self._judge(workspace, crate_cfg, chapter_cfg)
            self._write_coding_round_log(workspace, crate_cfg.name, round_index, prompt, coding_result, judge_result)
            if coding_result.returncode == 0 and judge_result.success:
                self._write_metric(workspace, crate_cfg.name, round_index, judge_result, "coding")
                self._mark_crate_finished(workspace, crate_cfg.name, "coding", True, judge_result.summary)
                self._write_crate_report(workspace, crate_cfg, "coding", round_index, judge_result)
                self.build_report(trial_name, mode="coding")
                return RunOutcome(crate=crate_cfg.name, success=True, rounds=round_index, summary=judge_result.summary, judge_result=judge_result)
            previous_failure = self._coding_failure_feedback(coding_result, judge_result)

        self._write_metric(workspace, crate_cfg.name, rounds, judge_result, "coding")
        self._mark_crate_finished(workspace, crate_cfg.name, "coding", False, judge_result.summary)
        self._write_crate_report(workspace, crate_cfg, "coding", rounds, judge_result)
        self.build_report(trial_name, mode="coding")
        return RunOutcome(crate=crate_cfg.name, success=False, rounds=rounds, summary=judge_result.summary, judge_result=judge_result)

    def run_order(self, trial_name: str, until: str, mode: str) -> list[RunOutcome]:
        outcomes: list[RunOutcome] = []
        for crate_name in self._order_until(until):
            if mode == "api":
                outcome = self.run_api_crate(trial_name, crate_name)
            else:
                outcome = self.run_coding_crate(trial_name, crate_name)
            outcomes.append(outcome)
            if not outcome.success:
                break
        return outcomes

    def run_all(self, trial_name: str, mode: str, through: str | None = None, resume: bool = False) -> RunAllResult:
        workspace = self.load_trial(trial_name)
        ensure_oracle_installed(workspace, self.settings)
        target = through or self.manifests.order[-1]
        crate_names = self._pending_crates(workspace, mode, target, resume=resume)
        patch_trial_state(
            workspace,
            status="running",
            mode=mode,
            target=target,
            last_error=None,
        )

        crate_outcomes: list[RunOutcome] = []
        for crate_name in crate_names:
            outcome = self.run_api_crate(trial_name, crate_name) if mode == "api" else self.run_coding_crate(trial_name, crate_name)
            crate_outcomes.append(outcome)
            if not outcome.success:
                next_crate = crate_name
                patch_trial_state(workspace, status="failed", mode=mode, target=target, last_error=outcome.summary)
                return RunAllResult(
                    mode=mode,
                    target=target,
                    success=False,
                    resumed=resume,
                    crate_outcomes=crate_outcomes,
                    pretest_result=None,
                    next_crate=next_crate,
                )

            if self._should_run_pretest_after_crate(workspace, mode, crate_name, target):
                pretest_result = self.run_pretest(trial_name)
                if not pretest_result.success:
                    patch_trial_state(workspace, status="failed", mode=mode, target=target, last_error=pretest_result.summary)
                    next_crate = self._next_crate_after(crate_name, target)
                    return RunAllResult(
                        mode=mode,
                        target=target,
                        success=False,
                        resumed=resume,
                        crate_outcomes=crate_outcomes,
                        pretest_result=pretest_result,
                        next_crate=next_crate,
                    )
                patch_trial_state(workspace, status="running", mode=mode, target=target, last_error=None)

        pretest_result = None
        if self._target_requires_pretest(target) and self._all_base_successful(workspace, mode):
            existing_pretest = self._load_pretest_result(workspace)
            if existing_pretest is None or not existing_pretest.success:
                pretest_result = self.run_pretest(trial_name)
                if not pretest_result.success:
                    patch_trial_state(workspace, status="failed", mode=mode, target=target, last_error=pretest_result.summary)
                    return RunAllResult(
                        mode=mode,
                        target=target,
                        success=False,
                        resumed=resume,
                        crate_outcomes=crate_outcomes,
                        pretest_result=pretest_result,
                        next_crate=self._first_incomplete_crate(workspace, mode, target),
                    )
            else:
                pretest_result = existing_pretest

        success = self._target_completed(workspace, mode, target) and (
            not self._target_requires_pretest(target) or (pretest_result is not None and pretest_result.success)
        )
        next_crate = None if success else self._first_incomplete_crate(workspace, mode, target)
        patch_trial_state(
            workspace,
            status="completed" if success else "failed",
            mode=mode,
            target=target,
            last_error=None if success else (pretest_result.summary if pretest_result else "incomplete run"),
        )
        return RunAllResult(
            mode=mode,
            target=target,
            success=success,
            resumed=resume,
            crate_outcomes=crate_outcomes,
            pretest_result=pretest_result,
            next_crate=next_crate,
        )

    def resume(self, trial_name: str, mode: str | None = None, through: str | None = None) -> RunAllResult:
        workspace = self.load_trial(trial_name)
        state = load_trial_state(workspace)
        selected_mode = mode or state.get("mode") or self._infer_mode(workspace)
        selected_target = through or state.get("target") or self.manifests.order[-1]
        return self.run_all(trial_name, mode=selected_mode, through=selected_target, resume=True)

    def build_report(self, trial_name: str, mode: str | None = None) -> TrialReport:
        workspace = self.load_trial(trial_name)
        state = load_trial_state(workspace)
        selected_mode = mode or state.get("mode") or self._infer_mode(workspace)
        completed = [name for name in self.manifests.order if self._metric_success(workspace, name, selected_mode)]
        next_crate = self._first_incomplete_crate(workspace, selected_mode, state.get("target") or self.manifests.order[-1])
        pretest_result = self._load_pretest_result(workspace)

        crate_metrics: list[dict[str, Any]] = []
        for name in self.manifests.order:
            payload = self._load_metric(workspace, name, selected_mode)
            if payload is not None:
                crate_metrics.append(payload)

        report = TrialReport(
            trial=trial_name,
            status=str(state.get("status", "unknown")),
            mode=state.get("mode"),
            target=state.get("target"),
            completed_crates=completed,
            next_crate=next_crate,
            pretest_success=bool(pretest_result and pretest_result.success),
            last_error=state.get("last_error"),
            crate_metrics=crate_metrics,
        )
        write_json(
            workspace.state_dir / "report.json",
            {
                "trial": report.trial,
                "status": report.status,
                "status_zh": self._trial_status_zh(report.status),
                "mode": report.mode,
                "target": report.target,
                "completed_crates": report.completed_crates,
                "next_crate": report.next_crate,
                "pretest_success": report.pretest_success,
                "last_error": report.last_error,
                "crate_metrics": report.crate_metrics,
            },
        )
        (workspace.state_dir / "report.md").write_text(self._render_trial_report_markdown(report), encoding="utf-8")
        return report

    def run_pretest(self, trial_name: str) -> JudgeResult:
        workspace = self.load_trial(trial_name)
        ensure_oracle_installed(workspace, self.settings)
        result = run_command(["cargo", "pretest"], workspace.run_root)
        summary = "cargo pretest passed" if result.returncode == 0 and not result.timed_out else f"cargo pretest failed: {result.returncode}"
        judge = JudgeResult(
            stage="pretest",
            success=result.returncode == 0 and not result.timed_out,
            summary=summary,
            command_results=[result],
            missing_patterns=[],
            forbidden_patterns=[],
        )
        write_json(
            workspace.logs_dir / "pretest.json",
            {
                "stage": "pretest",
                "success": judge.success,
                "summary": judge.summary,
                "commands": [format_command_result(result)],
            },
        )
        patch_trial_state(
            workspace,
            pretest={"ran": True, "success": judge.success, "summary": judge.summary},
            status="running" if judge.success else "failed",
            last_error=None if judge.success else judge.summary,
        )
        self.build_report(trial_name)
        return judge

    def _judge(self, workspace: TrialWorkspace, crate_cfg: CrateConfig, chapter_cfg: ChapterConfig | None) -> JudgeResult:
        if crate_cfg.kind == "base":
            return run_base_crate(self.settings, workspace, crate_cfg)
        assert chapter_cfg is not None
        return run_chapter(self.settings, workspace, crate_cfg, chapter_cfg)

    def _crate(self, crate_name: str) -> CrateConfig:
        return self.manifests.crates[crate_name]

    def _chapter(self, crate_cfg: CrateConfig) -> ChapterConfig | None:
        if crate_cfg.chapter is None:
            return None
        return self.manifests.chapters[crate_cfg.chapter]

    def _allowed_backtrack_crates(self, crate_cfg: CrateConfig, previous_failure: str | None) -> list[CrateConfig]:
        if not self.settings.generation.allow_backtrack or previous_failure is None:
            return []
        if crate_cfg.kind != "chapter":
            return []
        allowed = []
        for name in self.manifests.order:
            if name == crate_cfg.name:
                break
            candidate = self.manifests.crates[name]
            if candidate.kind == "base":
                allowed.append(candidate)
        return allowed

    def _apply_proposal(self, workspace: TrialWorkspace, crate_cfg: CrateConfig, proposal: EditProposal) -> None:
        allowed_roots = {crate_cfg.dir}
        for crate_name in proposal.backtrack_crates:
            if self.settings.generation.allow_backtrack and crate_name in self.manifests.crates:
                allowed_roots.add(self.manifests.crates[crate_name].dir)

        for file_edit in proposal.files:
            target = (workspace.run_root / file_edit.path).resolve()
            if not target.is_relative_to(workspace.run_root.resolve()):
                raise ValueError(f"proposal attempted to escape workspace: {file_edit.path}")
            if not any(target.is_relative_to((workspace.run_root / root).resolve()) for root in allowed_roots):
                raise ValueError(f"proposal attempted to edit disallowed path: {file_edit.path}")
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(file_edit.content, encoding="utf-8")

    def _write_round_log(
        self,
        workspace: TrialWorkspace,
        crate_name: str,
        round_index: int,
        proposal: EditProposal,
        judge_result: JudgeResult,
        mode: str,
    ) -> None:
        payload = {
            "mode": mode,
            "crate": crate_name,
            "round": round_index,
            "proposal": proposal_to_jsonable(proposal),
            "judge": {
                "success": judge_result.success,
                "summary": judge_result.summary,
                "failure_category": self._classify_failure(judge_result),
                "missing_patterns": judge_result.missing_patterns,
                "forbidden_patterns": judge_result.forbidden_patterns,
                "commands": [format_command_result(result) for result in judge_result.command_results],
            },
        }
        write_json(workspace.logs_dir / f"{crate_name}.{mode}.round{round_index}.json", payload)

    def _write_coding_round_log(
        self,
        workspace: TrialWorkspace,
        crate_name: str,
        round_index: int,
        prompt: str,
        coding_result,
        judge_result: JudgeResult,
    ) -> None:
        payload = {
            "mode": "coding",
            "crate": crate_name,
            "round": round_index,
            "prompt": prompt,
            "coding_cli": {
                "returncode": coding_result.returncode,
                "duration_seconds": coding_result.duration_seconds,
                "stdout": coding_result.stdout,
                "stderr": coding_result.stderr,
                "last_message_path": str(coding_result.last_message_path),
                "last_message_excerpt": self._read_last_message_excerpt(coding_result.last_message_path),
            },
            "judge": {
                "success": judge_result.success,
                "summary": judge_result.summary,
                "failure_category": self._classify_failure(judge_result),
                "missing_patterns": judge_result.missing_patterns,
                "forbidden_patterns": judge_result.forbidden_patterns,
                "commands": [format_command_result(result) for result in judge_result.command_results],
            },
        }
        write_json(workspace.logs_dir / f"{crate_name}.coding.round{round_index}.json", payload)

    def _write_metric(self, workspace: TrialWorkspace, crate_name: str, rounds: int, judge_result: JudgeResult, mode: str) -> None:
        failure_category = self._classify_failure(judge_result)
        payload = {
            "crate": crate_name,
            "mode": mode,
            "rounds": rounds,
            "success": judge_result.success,
            "summary": judge_result.summary,
            "failure_category": failure_category,
            "failure_category_zh": self._failure_category_zh(failure_category),
        }
        write_json(workspace.metrics_dir / f"{crate_name}.{mode}.json", payload)

    def _failure_feedback(self, judge_result: JudgeResult) -> str:
        parts = [f"Judge summary: {judge_result.summary}"]
        if judge_result.missing_patterns:
            parts.append("Missing required patterns: " + ", ".join(judge_result.missing_patterns))
        if judge_result.forbidden_patterns:
            parts.append("Forbidden patterns present: " + ", ".join(judge_result.forbidden_patterns))
        for result in judge_result.command_results:
            parts.append(self._compact_command_result(result))
        return "\n\n".join(parts)

    def _coding_failure_feedback(self, coding_result, judge_result: JudgeResult) -> str:
        parts = []
        if coding_result.returncode != 0:
            parts.append(f"Coding CLI failed with return code {coding_result.returncode}.")
            stdout_tail = self._extract_coding_cli_tail(coding_result.stdout)
            stderr_tail = self._extract_coding_cli_tail(coding_result.stderr)
            last_message = self._read_last_message_excerpt(coding_result.last_message_path)
            if last_message:
                parts.append("Coding CLI last message tail:\n" + last_message)
            if stdout_tail:
                parts.append("Coding CLI stdout tail:\n" + stdout_tail)
            if stderr_tail:
                parts.append("Coding CLI stderr tail:\n" + stderr_tail)
        parts.append(self._failure_feedback(judge_result))
        return "\n\n".join(parts)

    def _compact_command_result(self, result) -> str:
        parts = [
            f"Command: {' '.join(result.command)}",
            f"Return code: {result.returncode}",
            f"Timed out: {result.timed_out}",
            f"Duration: {result.duration_seconds:.2f}s",
        ]
        stdout_tail = self._tail_lines(result.stdout)
        stderr_tail = self._tail_lines(result.stderr)
        if stdout_tail:
            parts.append("Stdout tail:\n" + stdout_tail)
        if stderr_tail:
            parts.append("Stderr tail:\n" + stderr_tail)
        return "\n".join(parts)

    def _tail_lines(self, text: str) -> str:
        if not text:
            return ""
        lines = text.splitlines()
        tail = lines[-self.settings.generation.failure_tail_lines :]
        return "\n".join(tail)

    def _read_last_message_excerpt(self, path: Path) -> str:
        if not path.exists():
            return ""
        text = path.read_text(encoding="utf-8", errors="replace")
        return self._tail_lines(text)

    def _extract_coding_cli_tail(self, text: str) -> str:
        if not text:
            return ""
        lines = text.splitlines()
        filtered: list[str] = []
        capture = False
        for line in lines:
            if line.startswith("ERROR:") or "invalid_request_error" in line or "unsupported_value" in line:
                capture = True
            if capture:
                filtered.append(line)
        if not filtered:
            filtered = lines[-40:]
        return "\n".join(filtered[-self.settings.generation.failure_tail_lines :])

    def _load_metric(self, workspace: TrialWorkspace, crate_name: str, mode: str) -> dict[str, Any] | None:
        path = workspace.metrics_dir / f"{crate_name}.{mode}.json"
        payload = read_json(path)
        return payload if isinstance(payload, dict) else None

    def _load_round_payloads(self, workspace: TrialWorkspace, crate_name: str, mode: str) -> list[dict[str, Any]]:
        payloads: list[dict[str, Any]] = []
        for path in sorted(workspace.logs_dir.glob(f"{crate_name}.{mode}.round*.json")):
            payload = read_json(path)
            if isinstance(payload, dict):
                payloads.append(payload)
        return payloads

    def _metric_success(self, workspace: TrialWorkspace, crate_name: str, mode: str) -> bool:
        payload = self._load_metric(workspace, crate_name, mode)
        return bool(payload and payload.get("success"))

    def _infer_mode(self, workspace: TrialWorkspace) -> str:
        for mode in ("coding", "api"):
            for name in self.manifests.order:
                if (workspace.metrics_dir / f"{name}.{mode}.json").exists():
                    return mode
        return "api"

    def _base_crate_names(self) -> list[str]:
        return [name for name in self.manifests.order if self.manifests.crates[name].kind == "base"]

    def _first_incomplete_crate(self, workspace: TrialWorkspace, mode: str, target: str) -> str | None:
        for name in self._order_until(target):
            if not self._metric_success(workspace, name, mode):
                return name
        return None

    def _pending_crates(self, workspace: TrialWorkspace, mode: str, target: str, resume: bool) -> list[str]:
        if not resume:
            return list(self._order_until(target))
        start = self._first_incomplete_crate(workspace, mode, target)
        if start is None:
            return []
        names = list(self._order_until(target))
        return names[names.index(start) :]

    def _target_requires_pretest(self, target: str) -> bool:
        base_names = self._base_crate_names()
        return target not in base_names or target == base_names[-1]

    def _all_base_successful(self, workspace: TrialWorkspace, mode: str) -> bool:
        return all(self._metric_success(workspace, name, mode) for name in self._base_crate_names())

    def _load_pretest_result(self, workspace: TrialWorkspace) -> JudgeResult | None:
        payload = read_json(workspace.logs_dir / "pretest.json")
        if not isinstance(payload, dict):
            return None
        return JudgeResult(
            stage=str(payload.get("stage", "pretest")),
            success=bool(payload.get("success")),
            summary=str(payload.get("summary", "")),
            command_results=[],
            missing_patterns=[],
            forbidden_patterns=[],
        )

    def _classify_failure(self, judge_result: JudgeResult) -> str:
        if judge_result.success:
            return "success"
        if judge_result.missing_patterns:
            return "missing_patterns"
        if judge_result.forbidden_patterns:
            return "forbidden_patterns"
        for result in judge_result.command_results:
            command_text = shlex.join(result.command)
            if result.timed_out:
                return "qemu_timeout" if "qemu" in command_text else "timeout"
            if result.returncode == 0:
                continue
            if "cargo check" in command_text:
                return "compile_error"
            if "cargo test" in command_text:
                return "unit_test_failure"
            if "cargo pretest" in command_text:
                return "pretest_failure"
            if "qemu" in command_text:
                return "qemu_runtime"
        return "judge_failure"

    def _write_crate_report(
        self,
        workspace: TrialWorkspace,
        crate_cfg: CrateConfig,
        mode: str,
        rounds: int,
        judge_result: JudgeResult,
    ) -> None:
        metric = self._load_metric(workspace, crate_cfg.name, mode) or {}
        round_payloads = self._load_round_payloads(workspace, crate_cfg.name, mode)
        input_summary = describe_generation_inputs(self.settings, workspace, crate_cfg)
        chapter_cfg = self._chapter(crate_cfg)

        issues = []
        for payload in round_payloads:
            judge = payload.get("judge", {})
            if judge.get("success"):
                continue
            issues.append(
                {
                    "round": payload.get("round"),
                    "category": self._failure_category_zh(str(judge.get("failure_category", "judge_failure"))),
                    "summary": str(judge.get("summary", "")),
                    "status": "已在后续轮次解决" if judge_result.success else "尚未解决",
                }
            )

        round_summaries = []
        for payload in round_payloads:
            judge = payload.get("judge", {})
            coding_cli = payload.get("coding_cli", {})
            round_summaries.append(
                {
                    "round": payload.get("round"),
                    "success": bool(judge.get("success")),
                    "failure_category": str(judge.get("failure_category", "success" if judge.get("success") else "judge_failure")),
                    "summary": str(judge.get("summary", "")),
                    "coding_cli_returncode": coding_cli.get("returncode"),
                    "coding_cli_last_message_excerpt": coding_cli.get("last_message_excerpt"),
                }
            )

        report_payload = {
            "trial": workspace.name,
            "crate": crate_cfg.name,
            "package": crate_cfg.package,
            "dir": crate_cfg.dir,
            "mode": mode,
            "kind": crate_cfg.kind,
            "success": judge_result.success,
            "rounds": rounds,
            "summary": judge_result.summary,
            "failure_category": metric.get("failure_category", self._classify_failure(judge_result)),
            "failure_category_zh": metric.get(
                "failure_category_zh",
                self._failure_category_zh(metric.get("failure_category", self._classify_failure(judge_result))),
            ),
            "validation_gates": {
                "check": crate_cfg.check,
                "tests": crate_cfg.tests if crate_cfg.kind == "base" else [],
                "chapter_command": chapter_cfg.command if chapter_cfg is not None else None,
                "required_patterns": chapter_cfg.required_patterns if chapter_cfg is not None else [],
                "forbidden_patterns": chapter_cfg.forbidden_patterns if chapter_cfg is not None else [],
            },
            "spec_inputs": input_summary,
            "issues": issues,
            "round_summaries": round_summaries,
            "final_resolution": self._final_resolution_zh(judge_result, rounds),
        }
        write_json(workspace.reports_dir / f"{crate_cfg.name}.{mode}.json", report_payload)
        (workspace.reports_dir / f"{crate_cfg.name}.{mode}.md").write_text(
            self._render_crate_report_markdown(report_payload),
            encoding="utf-8",
        )

    def _render_crate_report_markdown(self, payload: dict[str, Any]) -> str:
        lines = [
            f"# {payload['crate']} 生成报告",
            "",
            "## 总览",
            f"- 试验：`{payload['trial']}`",
            f"- 模式：`{payload['mode']}`",
            f"- 类型：`{payload['kind']}`",
            f"- 包名：`{payload['package']}`",
            f"- 目录：`{payload['dir']}`",
            f"- 结果：{'成功' if payload['success'] else '失败'}",
            f"- 轮次：{payload['rounds']}",
            f"- 最终摘要：{payload['summary']}",
            f"- 失败分类：{self._failure_category_zh(str(payload['failure_category']))}",
            "",
            "## 规格输入",
        ]

        spec_inputs = payload.get("spec_inputs", {})
        if spec_inputs.get("project_context"):
            lines.append(f"- 项目上下文：`{spec_inputs['project_context']}`")
        current_spec_files = spec_inputs.get("current_spec_files", [])
        if current_spec_files:
            lines.append("- 当前 crate spec 文件：`" + "`, `".join(current_spec_files) + "`")
        extra_spec_names = spec_inputs.get("extra_spec_names", [])
        if extra_spec_names:
            lines.append("- 自动扩展的辅助 spec：`" + "`, `".join(extra_spec_names) + "`")
        dependency_specs = spec_inputs.get("dependency_specs", [])
        if dependency_specs:
            lines.append("- 直接依赖 spec：`" + "`, `".join(dependency_specs) + "`")
        oracle_roots = spec_inputs.get("oracle_roots", [])
        if oracle_roots:
            lines.append("- Oracle tests：`" + "`, `".join(oracle_roots) + "`")

        lines.extend(
            [
                "",
                "## 过程",
            ]
        )

        round_summaries = payload.get("round_summaries", [])
        if not round_summaries:
            lines.append("- 没有找到 round 日志。")
        for round_summary in round_summaries:
            status = "通过" if round_summary.get("success") else "失败"
            lines.append(
                f"- 第 {round_summary.get('round')} 轮：{status}；"
                f"分类={self._failure_category_zh(str(round_summary.get('failure_category', 'judge_failure')))}；"
                f"摘要={round_summary.get('summary')}"
            )
            excerpt = round_summary.get("coding_cli_last_message_excerpt")
            if excerpt:
                lines.append("  Codex 总结摘录：")
                lines.append("```text")
                lines.append(str(excerpt))
                lines.append("```")

        lines.extend(
            [
                "",
                "## 关键问题与处理",
            ]
        )
        issues = payload.get("issues", [])
        if not issues:
            lines.append("- 没有记录到需要单独列出的失败轮次。")
        for issue in issues:
            lines.append(
                f"- 第 {issue['round']} 轮：{issue['category']}；问题={issue['summary']}；状态={issue['status']}"
            )

        lines.extend(
            [
                "",
                "## 结论",
                f"- {payload['final_resolution']}",
                "",
            ]
        )
        return "\n".join(lines)

    def _render_trial_report_markdown(self, report: TrialReport) -> str:
        lines = [
            f"# Trial {report.trial} 总报告",
            "",
            "## 当前状态",
            f"- 状态：{self._trial_status_zh(report.status)}",
            f"- 模式：`{report.mode}`",
            f"- 目标：`{report.target}`",
            f"- 已完成 crate 数：{len(report.completed_crates)}",
            f"- 下一个 crate：`{report.next_crate}`" if report.next_crate else "- 下一个 crate：无",
            f"- pretest：{'通过' if report.pretest_success else '未通过或未执行'}",
            f"- 最近错误：{report.last_error or '无'}",
            "",
            "## Crate 汇总",
        ]
        if not report.crate_metrics:
            lines.append("- 暂无 crate 结果。")
        for metric in report.crate_metrics:
            lines.append(
                f"- `{metric['crate']}`：{'成功' if metric['success'] else '失败'}；"
                f"轮次={metric['rounds']}；"
                f"分类={self._failure_category_zh(str(metric.get('failure_category', 'judge_failure')))}；"
                f"摘要={metric['summary']}"
            )
        return "\n".join(lines) + "\n"

    def _trial_status_zh(self, status: str) -> str:
        mapping = {
            "initialized": "已初始化",
            "running": "运行中",
            "failed": "失败",
            "completed": "完成",
        }
        return mapping.get(status, status)

    def _failure_category_zh(self, category: str) -> str:
        mapping = {
            "success": "成功",
            "missing_patterns": "缺少预期输出",
            "forbidden_patterns": "命中禁止输出",
            "qemu_timeout": "QEMU 超时",
            "timeout": "命令超时",
            "compile_error": "编译错误",
            "unit_test_failure": "单元测试失败",
            "pretest_failure": "pretest 失败",
            "qemu_runtime": "QEMU 运行时错误",
            "judge_failure": "判题失败",
        }
        return mapping.get(category, category)

    def _final_resolution_zh(self, judge_result: JudgeResult, rounds: int) -> str:
        if judge_result.success:
            return f"该 crate 在第 {rounds} 轮通过外层判题，说明本轮 spec 到实现的闭环已收敛。"
        return f"该 crate 在 {rounds} 轮后仍未通过，当前主要阻塞为：{judge_result.summary}"

    def _should_run_pretest_after_crate(self, workspace: TrialWorkspace, mode: str, crate_name: str, target: str) -> bool:
        if not self._target_requires_pretest(target):
            return False
        last_base = self._base_crate_names()[-1]
        if crate_name != last_base:
            return False
        if not self._all_base_successful(workspace, mode):
            return False
        existing = self._load_pretest_result(workspace)
        return existing is None or not existing.success

    def _target_completed(self, workspace: TrialWorkspace, mode: str, target: str) -> bool:
        return all(self._metric_success(workspace, name, mode) for name in self._order_until(target))

    def _next_crate_after(self, crate_name: str, target: str) -> str | None:
        names = list(self._order_until(target))
        try:
            index = names.index(crate_name)
        except ValueError:
            return None
        if index + 1 >= len(names):
            return None
        return names[index + 1]

    def _mark_crate_started(self, workspace: TrialWorkspace, crate_name: str, mode: str) -> None:
        state = load_trial_state(workspace)
        patch_trial_state(
            workspace,
            status="running",
            mode=mode,
            target=state.get("target") or crate_name,
            last_attempted_crate=crate_name,
            last_error=None,
        )

    def _mark_crate_finished(self, workspace: TrialWorkspace, crate_name: str, mode: str, success: bool, summary: str) -> None:
        patch_trial_state(
            workspace,
            status="running" if success else "failed",
            mode=mode,
            last_attempted_crate=crate_name,
            last_completed_crate=crate_name if success else load_trial_state(workspace).get("last_completed_crate"),
            last_error=None if success else summary,
        )

    def _order_until(self, target: str) -> Iterable[str]:
        for name in self.manifests.order:
            yield name
            if name == target:
                return
