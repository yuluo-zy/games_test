"""Validate cross-package invariants from the parallel rhapsody recovery batch."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path


def read_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def parse_hex(value: str) -> int:
    return int(value, 16)


def named_type(ledger: dict, name: str) -> dict:
    return next(item for item in ledger["types"] if item["name"] == name)


def build(project_root: Path) -> dict:
    parallel = project_root / "evidence/render-recovery/parallel"
    paths = {
        "resource_info": parallel / "resource-info/type-ledger.json",
        "resource_info_validation": parallel / "resource-info/validation.json",
        "registry": parallel / "registry-pool/type-ledger.json",
        "registry_ownership": parallel / "registry-pool/ownership-state-machine.json",
        "registry_validation": parallel / "registry-pool/validation.json",
        "compiled": parallel / "compiled-state/type-ledger.json",
        "compiled_snapshot": parallel / "compiled-state/snapshot-schema.json",
        "compiled_validation": parallel / "compiled-state/verification.json",
    }
    documents = {name: read_json(path) for name, path in paths.items()}
    checks: list[dict] = []

    def check(name: str, condition: bool, detail: str) -> None:
        checks.append({"name": name, "ok": bool(condition), "detail": detail})

    registry_type = named_type(documents["registry"], "ResourceRegistry")
    compiled_type = named_type(documents["compiled"], "CompiledRenderGraph<UserData>")
    compiled_fields = {field["role"]: field for field in compiled_type["fields"]}
    registry_fields = {field.get("type"): field for field in registry_type["fields"]}
    resource_info_type = named_type(documents["resource_info"], "ResourceInfo")
    resource_access = named_type(documents["resource_info"], "ResourceAccessRecord")
    compiled_access = named_type(documents["compiled"], "PassAccessRecord")

    registry_offset = parse_hex(compiled_fields["ResourceRegistry aggregate"]["offset"])
    resource_info_offset = parse_hex(compiled_fields["ResourceInfo embedded at registry+0x48"]["offset"])
    registry_size = registry_type["size"]
    check(
        "compiled_registry_size",
        registry_size == parse_hex(compiled_fields["ResourceRegistry aggregate"]["size"]),
        f"registry={registry_size:#x}, compiled field={compiled_fields['ResourceRegistry aggregate']['size']}",
    )
    check(
        "resource_info_embedding",
        resource_info_offset - registry_offset == registry_fields["ResourceInfo"]["offset"],
        f"compiled delta={resource_info_offset - registry_offset:#x}, registry field={registry_fields['ResourceInfo']['offset']:#x}",
    )
    check(
        "resource_info_vector_prefix",
        [field["offset"] for field in resource_info_type["fields"][:6]]
        == ["0x00", "0x18", "0x30", "0x48", "0x60", "0x78"],
        "six observed Vec headers remain contiguous at 0x18-byte intervals",
    )
    check(
        "resource_access_stride",
        resource_access["size"] == compiled_access["observed_stride"] == "0x14"
        or resource_access["size"] == parse_hex(compiled_access["observed_stride"]),
        f"resource-info={resource_access['size']}, compiled={compiled_access['observed_stride']}",
    )
    resource_fields = [(field["offset"], field["type"].split()[0]) for field in resource_access["fields"][:5]]
    compiled_fields_access = [(field["offset"], field["role"]) for field in compiled_access["fields"]]
    check(
        "resource_access_field_offsets",
        [offset for offset, _ in resource_fields] == [offset for offset, _ in compiled_fields_access],
        "calculate producer offsets match pre-barrier consumer offsets",
    )

    event_type = named_type(documents["resource_info"], "LifetimeEventWord")
    skipped_release = any(
        item["claim"] == "release event inserts resource into local reuse maps"
        and item["status"].startswith("not present")
        for item in documents["registry_ownership"]["non_transitions"]
    )
    check(
        "lifetime_event_encoding",
        "pass_index<<17" in event_type["encoding"] and "0x10000" in event_type["encoding"],
        event_type["encoding"],
    )
    check(
        "release_event_consumer",
        skipped_release,
        "registry population skips bit16 release events; graph-local alias return is not claimed",
    )

    snapshot_layout = documents["compiled_snapshot"]["layout"]
    check(
        "compiled_total_size",
        parse_hex(snapshot_layout["size"]) == 0x218,
        snapshot_layout["size"],
    )
    check(
        "resource_info_native_snapshot",
        documents["resource_info_validation"]["native_snapshot"] == {
            "cases": 512,
            "ok": 507,
            "panic": 5,
            "mismatches": 0,
        },
        json.dumps(documents["resource_info_validation"]["native_snapshot"], sort_keys=True),
    )
    native_batch = documents["compiled_validation"]["native_batch_oracle"]
    check(
        "batch_native_snapshot",
        native_batch["status"] == "passed"
        and native_batch["executed_cases"] == 1600
        and native_batch["failure_count"] == 0,
        f"cases={native_batch['executed_cases']}, failures={native_batch['failure_count']}",
    )
    registry_checks = documents["registry_validation"]["checks"]
    check(
        "registry_candidate_tests",
        any(item["command"] == "cargo test --offline" and item["exit_code"] == 0 for item in registry_checks),
        "candidate ownership/pool tests passed; no native differential is claimed",
    )

    failed = [item for item in checks if not item["ok"]]
    return {
        "schema_version": 1,
        "status": "consistent_partial" if not failed else "inconsistent",
        "scope": "R1 parallel ResourceInfo -> ResourceRegistry/Pool -> CompiledRenderGraph contracts",
        "inputs": {
            name: {
                "path": str(path.relative_to(project_root)).replace("\\", "/"),
                "sha256": sha256(path),
            }
            for name, path in paths.items()
        },
        "checks": checks,
        "failed_checks": [item["name"] for item in failed],
        "accepted_boundaries": [
            "ResourceInfo::calculate semantic snapshot",
            "ResourcePool exact-key moved LIFO candidate",
            "ResourceRegistry ownership/export candidate",
            "BatchCompiler new/flush CPU state",
            "cross-package layout and event-consumer consistency",
        ],
        "not_accepted": [
            "complete populate_resources GPU creation behavior",
            "graph-local alias reuse",
            "full barrier and record_batch native equivalence",
            "queue submission and retirement",
            "temporal state schema",
            "original Rust ABI or source text",
        ],
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--project-root",
        type=Path,
        default=Path(__file__).resolve().parents[2],
    )
    parser.add_argument(
        "--output",
        type=Path,
        default=None,
    )
    args = parser.parse_args()
    project_root = args.project_root.resolve()
    output = args.output or project_root / "evidence/render-recovery/parallel/integration.json"
    result = build(project_root)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"status": result["status"], "checks": len(result["checks"]), "failed": result["failed_checks"]}))
    if result["failed_checks"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
