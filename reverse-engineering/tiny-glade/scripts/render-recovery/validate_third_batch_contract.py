"""Validate cross-package interfaces for rhapsody recovery batch 03."""

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
    batch = root / "evidence/render-recovery/parallel-batch-03"
    paths = {
        "contract": batch / "contract.json",
        "barrier_schema": batch / "barrier-record/oracle-schema.json",
        "barrier_validation": batch / "barrier-record/validation.json",
        "barrier_summary": batch / "barrier-record/validation-summary.json",
        "barrier_search": batch / "barrier-record/negative-search.json",
        "sequence_events": batch / "sequence-domain/event-schema.json",
        "sequence_machine": batch / "sequence-domain/mapping-state-machine.json",
        "sequence_contract": batch / "sequence-domain/implementation-contract.json",
        "sequence_unknowns": batch / "sequence-domain/unknowns.json",
        "sequence_validation": batch / "sequence-domain/validation.json",
        "temporal_schema": batch / "temporal-full/snapshot-schema.json",
        "temporal_machine": batch / "temporal-full/temporal-state-machine.json",
        "temporal_contract": batch / "temporal-full/implementation-contract.json",
        "temporal_unknowns": batch / "temporal-full/unknowns.json",
        "temporal_validation": batch / "temporal-full/validation.json",
    }
    docs = {name: read_json(path) for name, path in paths.items()}
    checks: list[dict] = []

    def check(name: str, condition: bool, detail: str) -> None:
        checks.append({"name": name, "ok": bool(condition), "detail": detail})

    barrier = docs["barrier_validation"]
    barrier_summary = docs["barrier_summary"]
    check("barrier_native_oracle", barrier["passed"] is True and barrier["cases"] == 1344 and barrier["failure_count"] == 0, f"cases={barrier['cases']}, failures={barrier['failure_count']}")
    check("barrier_mode_coverage", barrier["by_mode"] == {"0": 768, "1": 288, "2": 288}, "pre, initial-record prefix and final/export gate covered")
    check("population_original_entry", barrier_summary["native_population_cases"] == 97 and barrier_summary["snapshot_checks"] == 6, "97 population fixtures and six R2 snapshot checks passed")
    check("barrier_partial_boundaries", "full record_batch oracle" in " ".join(barrier["limits"]) and barrier["repeat_initial_no_duplicate"] is True, "partial oracle remains explicit and repeat-prefix behavior is covered")
    search = docs["barrier_search"]
    check("reuse_marker_bounded_search", len(search["positive_marker_stores"]) == 3 and "producer" in search["result"] and "not a whole-program absence claim" in search["result"], "ordinary marker=0 and two marker=1 consumers are observed; producer conclusion remains bounded")

    machine = docs["sequence_machine"]
    sequence_requirements = docs["sequence_contract"]["requirements"]
    check("sequence_timeline_domains", "not interchangeable" in machine["ordering"] and any("distinct u64" in item for item in sequence_requirements), "CB sequence orders disposal; queue timeline gates completion")
    check("sequence_queue_ownership", any("queue identity differs" in item for item in sequence_requirements), "candidate rejects cross-queue CB ownership mismatch")
    sequence_unknowns = docs["sequence_unknowns"]["unknowns"]
    check("sequence_unknown_preserved", any("counter slots" in item["question"] and item["status"] == "unresolved" for item in sequence_unknowns), "counter seed/global collision policy remains unresolved")
    sequence_validation = docs["sequence_validation"]
    check("sequence_no_false_native_claim", str(sequence_validation["native_differential"]).startswith("not run"), "Vulkan-dependent native differential is not claimed")

    temporal = docs["temporal_validation"]
    temporal_machine = docs["temporal_machine"]
    temporal_contract = docs["temporal_contract"]
    temporal_entries = temporal["static"]["new_procedures"] + temporal["static"]["reused"]
    check("temporal_original_entries", len(temporal_entries) == 7 and temporal["static"]["close_database_save_false"] is True, "four new and three reused static entries captured; database closed without save")
    check("temporal_candidate_tests", temporal["candidate"]["result"].startswith("passed:"), temporal["candidate"]["result"])
    machine_text = json.dumps(temporal_machine, ensure_ascii=False)
    contract_text = json.dumps(temporal_contract, ensure_ascii=False)
    check("temporal_raw_export_import", all(token in machine_text + contract_text for token in ["texture", "buffer", "export"]), "raw texture/buffer and exported state represented")
    temporal_unknowns = docs["temporal_unknowns"]["unknowns"]
    check("temporal_reset_unknown_preserved", any("reset/drop" in item["question"] and item["status"] == "unknown" for item in temporal_unknowns), "reset/drop is not guessed")

    failed = [item["name"] for item in checks if not item["ok"]]
    return {
        "schema_version": 1,
        "status": "consistent_partial" if not failed else "inconsistent",
        "scope": "R2 compiled transitions -> R5A barrier/record -> R5B sequence/timeline plus R5C cross-frame temporal state",
        "inputs": {name: {"path": str(path.relative_to(root)).replace("\\", "/"), "sha256": sha256(path)} for name, path in paths.items()},
        "checks": checks,
        "failed_checks": failed,
        "closed_path": ["compiled transition branches to recorded backend events", "command-buffer sequence ownership through timeline-gated reclamation", "temporal single/pair selection and raw/exported cross-frame projection"],
        "remaining": ["full pass-recording and Vulkan runtime differential", "device constructor counter seed/global CB sequence uniqueness", "temporal reset/drop and compile take/restore ownership", "pipeline/descriptors/shaders and country-core pass integration"],
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--project-root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    root = args.project_root.resolve()
    output = args.output or root / "evidence/render-recovery/parallel-batch-03/integration.json"
    result = build(root)
    output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"status": result["status"], "checks": len(result["checks"]), "failed": result["failed_checks"]}))
    if result["failed_checks"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
