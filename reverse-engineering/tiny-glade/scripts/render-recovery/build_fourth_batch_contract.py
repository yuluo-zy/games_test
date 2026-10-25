"""Build the R6 device/record/pipeline recovery contract and shader inventory."""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter
from pathlib import Path


ENTRIES = {
    "R6A-device-counters": ["0x140bf2560"],
    "R6B-record-submit": ["0x1402c1d40", "0x140bebdf0", "0x140c04e20"],
}


def read_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build(root: Path, original_root: Path) -> dict:
    functions_path = root / "evidence/render-recovery/workgraph/functions.json"
    calls_path = root / "evidence/render-recovery/workgraph/calls.json"
    batch_03_path = root / "evidence/render-recovery/parallel-batch-03/integration.json"
    functions = read_json(functions_path)
    by_va = {item["va"]: item for item in functions}
    missing = sorted(set().union(*map(set, ENTRIES.values())) - set(by_va))
    if missing:
        raise ValueError(f"missing workgraph functions: {missing}")

    shader_root = original_root / "compiled-shaders"
    shaders = sorted(shader_root.glob("*.bin"), key=lambda path: path.name.lower())
    kinds = Counter("compute" if ".cs.hlsl_" in path.name else "raster" if ".raster." in path.name else "unknown" for path in shaders)
    inventory = [{"name": path.name, "size": path.stat().st_size, "sha256": sha256(path)} for path in shaders]
    inventory_digest = hashlib.sha256(json.dumps(inventory, sort_keys=True).encode()).hexdigest()

    packages = {}
    for package, addresses in ENTRIES.items():
        packages[package] = {
            "entries": [{"va": address, "code_size": by_va[address]["code_size"], "names": by_va[address]["names"], "work_package": by_va[address]["work_package"]} for address in addresses]
        }
    packages["R6C-pipeline-shader-interfaces"] = {
        "shader_root": str(shader_root),
        "shader_count": len(shaders),
        "kinds": dict(sorted(kinds.items())),
        "inventory_sha256": inventory_digest,
    }

    return {
        "schema_version": 1,
        "status": "batch_04_started",
        "identity": {"binary_sha256": "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03", "functions_sha256": sha256(functions_path), "calls_sha256": sha256(calls_path), "batch_03_integration_sha256": sha256(batch_03_path)},
        "packages": packages,
        "interfaces": [
            {"from": "R6A-device-counters", "to": "R5B-sequence-domain", "contract": "counter initialization and queue identity establish the legal global u64 bucket key policy"},
            {"from": "R5A-barrier-record", "to": "R6B-record-submit", "contract": "post-barrier pass recording produces command buffers and submit backend events"},
            {"from": "R6C-pipeline-shader-interfaces", "to": "country-core passes", "contract": "reflected descriptor/push/stage interfaces identify compatible game pass consumers without reconstructing shader algorithms"},
        ],
        "acceptance": {
            "R6A-device-counters": ["counter producer and initial values are observed or left unknown", "queue identity and collision policy are explicit", "no live Vulkan execution claim"],
            "R6B-record-submit": ["record tail and submit boundary produce deterministic backend events", "uncovered indirect pass callbacks remain explicit", "R5A/R5B invariants remain valid"],
            "R6C-pipeline-shader-interfaces": ["all shipped shader files enter a deterministic catalog", "interfaces are grouped by reflected signature", "reflection failure and opaque payloads are counted", "shader algorithms are not claimed recovered"],
        },
        "shared_limits": ["PDB void signatures are placeholders", "static evidence does not prove runtime execution", "ash/Vulkan/compiler libraries are reused", "CPU/interface candidates are not image equivalence"],
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--project-root", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--original-root", type=Path, default=Path(r"D:/game/ljxsj_92385/Tiny Glade"))
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    root = args.project_root.resolve()
    output = args.output or root / "evidence/render-recovery/parallel-batch-04/contract.json"
    result = build(root, args.original_root.resolve())
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({name: data.get("shader_count", len(data.get("entries", []))) for name, data in result["packages"].items()}))


if __name__ == "__main__":
    main()
