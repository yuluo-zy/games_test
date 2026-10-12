"""Build the deterministic R5A/R5B/R5C rhapsody recovery contract."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

PACKAGES = {
    "R5A-barrier-record": ["0x1402c1d40", "0x1402c3d00"],
    "R5B-sequence-timeline": ["0x140becfd0", "0x140bf0cc0", "0x140c04e20", "0x1418c3300", "0x1418c7ae0"],
    "R5C-temporal-state": ["0x1404f7ca0", "0x14096c310", "0x14096c890", "0x14096ce90", "0x14096d490", "0x140beced0"],
}

def read_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))

def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()

def build(root: Path) -> dict:
    workgraph = root / "evidence/render-recovery/workgraph"
    functions_path = workgraph / "functions.json"
    calls_path = workgraph / "calls.json"
    batch_02_path = root / "evidence/render-recovery/parallel-batch-02/integration.json"
    functions = read_json(functions_path)
    calls = read_json(calls_path)
    by_va = {item["va"]: item for item in functions}
    missing = sorted(set().union(*map(set, PACKAGES.values())) - set(by_va))
    if missing:
        raise ValueError(f"missing workgraph functions: {missing}")
    all_selected = set().union(*map(set, PACKAGES.values()))
    packages = {}
    for package, addresses in PACKAGES.items():
        entries = []
        for address in addresses:
            item = by_va[address]
            decompilation = root / "evidence/render-architecture" / f"{address}.c"
            entries.append({
                "va": address,
                "code_size": item["code_size"],
                "names": item["names"],
                "work_package": item["work_package"],
                "existing_decompilation": str(decompilation.relative_to(root)).replace("\\", "/") if decompilation.is_file() else None,
                "indirect_call_sites": len(calls["functions"][address]["indirect_calls"]),
            })
        packages[package] = {"entry_count": len(entries), "entries": entries}
    edges = [edge for edge in calls["edges"] if edge["source"] in all_selected and edge["target"] in all_selected]
    return {
        "schema_version": 1,
        "status": "batch_03_started",
        "identity": {
            "binary_sha256": "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03",
            "functions_sha256": sha256(functions_path),
            "calls_sha256": sha256(calls_path),
            "batch_02_integration_sha256": sha256(batch_02_path),
        },
        "packages": packages,
        "selected_internal_edges": edges,
        "interfaces": [
            {"from": "R2-graph-compiler", "to": "R5A-barrier-record", "contract": "compiled resource transitions and batches become deterministic backend events"},
            {"from": "R5A-barrier-record", "to": "R5B-sequence-timeline", "contract": "recorded command buffers expose command-sequence ownership before queue submission"},
            {"from": "R5B-sequence-timeline", "to": "R3-gpu-lifecycle", "contract": "command-sequence buckets map to queue completion without merging numbering domains"},
            {"from": "R5C-temporal-state", "to": "R2-graph-compiler", "contract": "typed temporal payload and parity state feed the next graph compilation"},
        ],
        "acceptance": {
            "R5A-barrier-record": ["original-entry instruction oracle or bounded structural oracle covers transition branches", "backend-event ordering agrees with the batch-02 compiled snapshot contract", "Vulkan calls remain explicit unexecuted backend events unless runtime evidence exists"],
            "R5B-sequence-timeline": ["producers and consumers of both numbering domains are identified", "same-resource ownership is traced across submit and reclaim boundaries", "unknown cross-domain mapping is retained as unknown rather than equated"],
            "R5C-temporal-state": ["texture and buffer single/pair payload variants are represented", "parity, export, replacement, reset and drop are separate state transitions", "multi-frame tests cover missing, hit, replace and reset paths"],
        },
        "shared_limits": ["PDB void signatures are placeholders", "static calls do not prove runtime execution", "public dependency implementations are reused, not reconstructed", "candidate tests do not establish full Vulkan or GPU equivalence", "observed, inferred and unknown claims remain separately labelled"],
    }

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--project-root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    root = args.project_root.resolve()
    output = args.output or root / "evidence/render-recovery/parallel-batch-03/contract.json"
    result = build(root)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({name: data["entry_count"] for name, data in result["packages"].items()}))

if __name__ == "__main__":
    main()
