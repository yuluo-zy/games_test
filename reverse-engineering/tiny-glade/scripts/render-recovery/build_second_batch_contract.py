"""Build the deterministic R2/R3/R4 rhapsody recovery contract."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path


SEEDS = {
    "R2-graph-compiler": [
        r"ResourceInfo::calculate",
        r"ResourceRegistry::populate_resources",
        r"ResourceRegistry::into_exports",
        r"CompiledRenderGraph.*::(?:new|record_batch|add_pre_pass_barriers)",
        r"BatchCompiler::",
        r"RenderGraphState::compile",
    ],
    "R3-gpu-lifecycle": [
        r"RawManagedHandle|managed::handle::impl\$(?:4|5|6)::(?:track|clone|drop)",
        r"ManagedResources::take_out_dropped_waste",
        r"SubmissionEngine::update_queue_pending_submissions",
        r"CommandBufferSeqTracker::track",
        r"CommandBufferDisposalBuckets::(?:remove|insert)",
        r"SequentialDisposal::dispose_in_order",
        r"GraphicsDevice::queue_submit",
        r"GraphicsDevice::(?:wait_submit_done|wait_duration_or_until_submit_done)",
    ],
    "R4-executor-temporal": [
        r"GraphExecutionRecorder::(?:execute|execute_compiled|ensure_active_cb|submit_pending_cbs|end_frame)",
        r"GraphExecutor::(?:new|begin_frame|wait_gpu_queue_trim|last_known_frame_duration_ns)",
        r"ExecutedRenderGraph::(?:new|retire)",
        r"ExportedRenderGraphResources::take_resource",
        r"TemporalRenderGraphState::get_and_flip_parity",
        r"TemporalRenderGraphRecorder.*::export_temporal",
    ],
}


def read_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build(root: Path) -> dict:
    workgraph = root / "evidence/render-recovery/workgraph"
    functions_path = workgraph / "functions.json"
    calls_path = workgraph / "calls.json"
    integration_path = root / "evidence/render-recovery/parallel/integration.json"
    functions = read_json(functions_path)
    calls = read_json(calls_path)
    by_va = {item["va"]: item for item in functions}
    selected: dict[str, set[str]] = {}
    for package, patterns in SEEDS.items():
        regexes = [re.compile(pattern) for pattern in patterns]
        selected[package] = {
            item["va"]
            for item in functions
            if any(regex.search(" | ".join(item["names"])) for regex in regexes)
        }

    all_selected = set().union(*selected.values())
    edges = [
        edge
        for edge in calls["edges"]
        if edge["source"] in all_selected and edge["target"] in all_selected
    ]
    packages = {}
    for package, addresses in selected.items():
        entries = []
        for address in sorted(addresses, key=lambda value: int(value, 16)):
            item = by_va[address]
            decompilation = root / "evidence/render-architecture" / f"{address}.c"
            entries.append(
                {
                    "va": address,
                    "code_size": item["code_size"],
                    "names": item["names"],
                    "work_package": item["work_package"],
                    "existing_decompilation": (
                        str(decompilation.relative_to(root)).replace("\\", "/")
                        if decompilation.is_file()
                        else None
                    ),
                    "indirect_call_sites": len(calls["functions"][address]["indirect_calls"]),
                }
            )
        packages[package] = {
            "entry_count": len(entries),
            "entries": entries,
        }

    return {
        "schema_version": 1,
        "status": "batch_02_started",
        "identity": {
            "binary_sha256": "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03",
            "functions_sha256": sha256(functions_path),
            "calls_sha256": sha256(calls_path),
            "batch_01_integration_sha256": sha256(integration_path),
        },
        "packages": packages,
        "selected_internal_edges": edges,
        "interfaces": [
            {
                "from": "R2-graph-compiler",
                "to": "R4-executor-temporal",
                "contract": "CompiledRenderGraph snapshot, batches, initial/pre/export transition plans",
            },
            {
                "from": "R4-executor-temporal",
                "to": "R3-gpu-lifecycle",
                "contract": "command buffers, submit sequence/timeline, tracked managed handles, completion token",
            },
            {
                "from": "R3-gpu-lifecycle",
                "to": "R4-executor-temporal",
                "contract": "completed sequence, reclaimable resources and command buffers",
            },
            {
                "from": "R4-executor-temporal",
                "to": "R2-graph-compiler",
                "contract": "retired remaining resources, exports, temporal parity/history state",
            },
        ],
        "acceptance": {
            "R2-graph-compiler": [
                "one DSL input produces a deterministic compiled graph snapshot",
                "batch-01 ledgers remain cross-package consistent",
                "GPU creation and Vulkan calls are explicit backend events",
            ],
            "R3-gpu-lifecycle": [
                "create-track-drop-submit-complete-reclaim event trace closes",
                "CPU drop never implies premature GPU destruction",
                "generation/sequence failure boundaries are explicit",
            ],
            "R4-executor-temporal": [
                "compiled batches map to command buffers and submit events",
                "retire returns resources/exports only after the established completion boundary",
                "temporal parity/history transition is represented without guessing payload schema",
            ],
        },
        "shared_limits": [
            "PDB void signatures are placeholders",
            "static calls do not prove runtime execution",
            "public dependency implementations are reused, not reconstructed",
            "country-core monomorphized closure content is not generic rhapsody source",
            "candidate tests do not establish full Vulkan equivalence",
        ],
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--project-root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    root = args.project_root.resolve()
    output = args.output or root / "evidence/render-recovery/parallel-batch-02/contract.json"
    result = build(root)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({name: data["entry_count"] for name, data in result["packages"].items()}))


if __name__ == "__main__":
    main()
