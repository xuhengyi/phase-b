from __future__ import annotations

from pathlib import Path
import argparse
import sys

from .config import load_settings
from .manifests import load_manifests
from .runner import ExperimentRunner


def _phase_root() -> Path:
    return Path(__file__).resolve().parents[1]


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Phase B spec-to-OS automation")
    parser.add_argument("--config", type=Path, help="Optional TOML override for phase-b/agent/manifests/defaults.toml")

    subparsers = parser.add_subparsers(dest="command", required=True)

    init_parser = subparsers.add_parser("init-trial", help="Create a new trial workspace from phase-b/candidate-template")
    init_parser.add_argument("trial")
    init_parser.add_argument("--force", action="store_true")

    api_parser = subparsers.add_parser("run-api-crate", help="Run one crate with the model API runner")
    api_parser.add_argument("trial")
    api_parser.add_argument("crate")

    coding_parser = subparsers.add_parser("run-coding-crate", help="Run one crate by orchestrating a coding CLI tool")
    coding_parser.add_argument("trial")
    coding_parser.add_argument("crate")

    api_phase = subparsers.add_parser("run-api-phase", help="Run crates in manifest order until the target crate")
    api_phase.add_argument("trial")
    api_phase.add_argument("target")

    coding_phase = subparsers.add_parser("run-coding-phase", help="Run crates in manifest order until the target crate")
    coding_phase.add_argument("trial")
    coding_phase.add_argument("target")

    pretest = subparsers.add_parser("run-pretest", help="Run cargo pretest inside an initialized trial")
    pretest.add_argument("trial")

    run_all = subparsers.add_parser("run-all", help="Run the whole crate sequence, pretest, and chapter sequence")
    run_all.add_argument("trial")
    run_all.add_argument("--mode", choices=("api", "coding"), default="api")
    run_all.add_argument("--through", default="ch8")

    resume = subparsers.add_parser("resume", help="Resume a stopped trial from the first incomplete crate")
    resume.add_argument("trial")
    resume.add_argument("--mode", choices=("api", "coding"))
    resume.add_argument("--through")

    report = subparsers.add_parser("report", help="Summarize trial progress and write a report artifact")
    report.add_argument("trial")
    report.add_argument("--mode", choices=("api", "coding"))
    report.add_argument("--json", action="store_true")

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    raw_argv = list(sys.argv[1:] if argv is None else argv)
    if "--config" in raw_argv:
        index = raw_argv.index("--config")
        if index + 1 < len(raw_argv) and index != 0:
            pair = raw_argv[index : index + 2]
            del raw_argv[index : index + 2]
            raw_argv = pair + raw_argv
    args = parser.parse_args(raw_argv)
    phase_root = _phase_root()
    settings = load_settings(phase_root, args.config)
    manifests = load_manifests(phase_root)
    runner = ExperimentRunner(settings, manifests)

    if args.command == "init-trial":
        workspace = runner.init_trial(args.trial, force=args.force)
        print(f"initialized trial at {workspace.root}")
        return 0

    if args.command == "run-api-crate":
        outcome = runner.run_api_crate(args.trial, args.crate)
        print(f"{outcome.crate}: success={outcome.success} rounds={outcome.rounds} summary={outcome.summary}")
        return 0 if outcome.success else 1

    if args.command == "run-coding-crate":
        outcome = runner.run_coding_crate(args.trial, args.crate)
        print(f"{outcome.crate}: success={outcome.success} rounds={outcome.rounds} summary={outcome.summary}")
        return 0 if outcome.success else 1

    if args.command == "run-api-phase":
        outcomes = runner.run_order(args.trial, args.target, mode="api")
        for outcome in outcomes:
            print(f"{outcome.crate}: success={outcome.success} rounds={outcome.rounds} summary={outcome.summary}")
        return 0 if outcomes and outcomes[-1].success else 1

    if args.command == "run-coding-phase":
        outcomes = runner.run_order(args.trial, args.target, mode="coding")
        for outcome in outcomes:
            print(f"{outcome.crate}: success={outcome.success} rounds={outcome.rounds} summary={outcome.summary}")
        return 0 if outcomes and outcomes[-1].success else 1

    if args.command == "run-pretest":
        result = runner.run_pretest(args.trial)
        print(f"pretest: success={result.success} summary={result.summary}")
        return 0 if result.success else 1

    if args.command == "run-all":
        result = runner.run_all(args.trial, mode=args.mode, through=args.through, resume=False)
        for outcome in result.crate_outcomes:
            print(f"{outcome.crate}: success={outcome.success} rounds={outcome.rounds} summary={outcome.summary}")
        if result.pretest_result is not None:
            print(f"pretest: success={result.pretest_result.success} summary={result.pretest_result.summary}")
        print(f"run-all: success={result.success} next_crate={result.next_crate}")
        return 0 if result.success else 1

    if args.command == "resume":
        result = runner.resume(args.trial, mode=args.mode, through=args.through)
        for outcome in result.crate_outcomes:
            print(f"{outcome.crate}: success={outcome.success} rounds={outcome.rounds} summary={outcome.summary}")
        if result.pretest_result is not None:
            print(f"pretest: success={result.pretest_result.success} summary={result.pretest_result.summary}")
        print(f"resume: success={result.success} next_crate={result.next_crate}")
        return 0 if result.success else 1

    if args.command == "report":
        trial_report = runner.build_report(args.trial, mode=args.mode)
        if args.json:
            import json
            print(
                json.dumps(
                    {
                        "trial": trial_report.trial,
                        "status": trial_report.status,
                        "status_zh": runner._trial_status_zh(trial_report.status),
                        "mode": trial_report.mode,
                        "target": trial_report.target,
                        "completed_crates": trial_report.completed_crates,
                        "next_crate": trial_report.next_crate,
                        "pretest_success": trial_report.pretest_success,
                        "last_error": trial_report.last_error,
                        "crate_metrics": trial_report.crate_metrics,
                    },
                    indent=2,
                    ensure_ascii=False,
                )
            )
        else:
            print(f"试验={trial_report.trial}")
            print(f"状态={trial_report.status}")
            print(f"模式={trial_report.mode}")
            print(f"目标={trial_report.target}")
            print(f"已完成 crate 数={len(trial_report.completed_crates)}")
            print(f"下一个 crate={trial_report.next_crate}")
            print(f"pretest 通过={trial_report.pretest_success}")
            print(f"最近错误={trial_report.last_error}")
            for metric in trial_report.crate_metrics:
                print(
                    f"{metric['crate']}: 成功={metric['success']} 轮次={metric['rounds']} "
                    f"模式={metric['mode']} 失败分类={metric.get('failure_category_zh', metric.get('failure_category'))} 摘要={metric['summary']}"
                )
        return 0

    parser.print_help()
    return 1


if __name__ == "__main__":
    sys.exit(main())
