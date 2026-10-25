"""Validate R2/R3/R4 interfaces for rhapsody recovery batch 02."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path


def read_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build(root: Path) -> dict:
    batch = root / "evidence/render-recovery/parallel-batch-02"
    paths = {
        "contract": batch / "contract.json",
        "graph_schema": batch / "graph-compiler/snapshot-schema.json",
        "graph_contract": batch / "graph-compiler/implementation-contract.json",
        "graph_validation": batch / "graph-compiler/validation.json",
        "lifecycle_schema": batch / "gpu-lifecycle/event-schema.json",
        "lifecycle_state": batch / "gpu-lifecycle/lifecycle-state-machine.json",
        "lifecycle_contract": batch / "gpu-lifecycle/implementation-contract.json",
        "lifecycle_validation": batch / "gpu-lifecycle/validation.json",
        "executor_timeline": batch / "executor-temporal/execution-timeline.json",
        "executor_temporal": batch / "executor-temporal/temporal-schema.json",
        "executor_contract": batch / "executor-temporal/implementation-contract.json",
        "executor_validation": batch / "executor-temporal/validation.json",
    }
    docs = {name: read_json(path) for name, path in paths.items()}
    checks: list[dict] = []

    def check(name: str, condition: bool, detail: str) -> None:
        checks.append({"name": name, "ok": bool(condition), "detail": detail})

    graph_fields = docs["graph_schema"]["fields"]
    check(
        "initial_transition_reused_split",
        "reused=false" in graph_fields["initial_transitions"]
        and "reused=true" in graph_fields["pre_transitions"]
        and "initial=0" in graph_fields["pre_transitions"],
        "record_batch handles ordinary initial transitions; pre-pass handles explicit reused resources",
    )
    check(
        "pool_hit_not_graph_alias",
        "reused marker remains false" in graph_fields["pool_hits"]
        and "graph-local alias producer" in docs["graph_contract"]["excluded"],
        "persistent pool reuse is not promoted to graph-local alias evidence",
    )
    check(
        "graph_candidate_boundary",
        docs["graph_validation"]["passed"] is True
        and docs["graph_validation"]["tests"] == 17
        and docs["graph_validation"]["full_original_entry_differential"] is False,
        "17 projection tests pass; full original compile entry remains unverified",
    )

    lifecycle_events = docs["lifecycle_schema"]["events"]
    lifecycle_invariants = docs["lifecycle_state"]["invariants"]
    check(
        "command_sequence_vs_queue_timeline",
        "distinct from command sequence" in lifecycle_events["submit"]["timeline"]
        and "command sequence differs from timeline" in lifecycle_invariants,
        "R3 preserves separate command and submission ordering domains",
    )
    check(
        "cpu_retire_not_completion",
        "no completion guarantee" in lifecycle_events["cpu_pool_return"]["effect"]
        and "CPU return/drop is not GPU completion" in lifecycle_invariants
        and "no completion wait" in docs["executor_contract"]["retire_contract"],
        "CPU pool return is immediate while tracked handles protect GPU lifetime",
    )
    native_cpu = docs["lifecycle_validation"]["native_cpu"]
    check(
        "lifecycle_native_leaves",
        native_cpu["cases"] == 29692 and native_cpu["failures"] == 0,
        f"cases={native_cpu['cases']}, failures={native_cpu['failures']}",
    )
    check(
        "generation_failure_policy",
        docs["lifecycle_state"]["failures"]["stale_track"] == "silent skip"
        and "generation changes at actual disposal" in lifecycle_invariants,
        "stale track is ignored; generation advances only at disposal",
    )

    timeline = docs["executor_timeline"]
    check(
        "executor_decoupled_paths",
        "execute_compiled -> immediate_retire" in timeline["decoupled_paths"]
        and "queue_submit -> submit_completion -> tracked_disposal" in timeline["decoupled_paths"],
        "R4 CPU retirement and R3 GPU disposal are separate paths",
    )
    temporal = docs["executor_temporal"]
    check(
        "temporal_parity_per_key",
        temporal["state"]["container"] == "hash map"
        and temporal["transition"]["missing"] == {"return": 0, "stored": 0}
        and temporal["transition"]["hit"] == {"return": "1-old", "stored": "1-old"},
        "parity is keyed state, not one global bool",
    )
    executor_coverage = docs["executor_validation"]["candidate"]["coverage"]
    check(
        "temporal_export_no_flip",
        "temporal export replaces raw variants without parity flip or reexport" in executor_coverage,
        "export_temporal and get_and_flip_parity remain separate operations",
    )
    check(
        "executor_candidate_tests",
        docs["executor_validation"]["candidate"]["cargo_test"].startswith("passed: 4 tests")
        and docs["executor_validation"]["candidate"]["cargo_fmt"] == "passed",
        docs["executor_validation"]["candidate"]["cargo_test"],
    )

    failed = [item["name"] for item in checks if not item["ok"]]
    return {
        "schema_version": 1,
        "status": "consistent_partial" if not failed else "inconsistent",
        "scope": "R2 compile projection -> R4 executor/temporal -> R3 tracked GPU lifecycle",
        "inputs": {
            name: {
                "path": str(path.relative_to(root)).replace("\\", "/"),
                "sha256": sha256(path),
            }
            for name, path in paths.items()
        },
        "checks": checks,
        "failed_checks": failed,
        "closed_path": [
            "graph declarations to deterministic CPU compiled snapshot",
            "compiled batches to command-buffer and submit events",
            "immediate CPU retirement with deferred tracked GPU disposal",
            "per-key temporal parity and export state projection",
        ],
        "remaining": [
            "full original compile/record/submit entry differential",
            "Vulkan runtime and real GPU execution",
            "graph-local alias producer or proof it is disabled",
            "same-resource cross-domain sequence mapping",
            "full temporal key/payload/reset schema",
            "pipeline/descriptors/shaders and country-core pass integration",
        ],
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--project-root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    root = args.project_root.resolve()
    output = args.output or root / "evidence/render-recovery/parallel-batch-02/integration.json"
    result = build(root)
    output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"status": result["status"], "checks": len(result["checks"]), "failed": result["failed_checks"]}))
    if result["failed_checks"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
