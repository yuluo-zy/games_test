"""Compare read-only Ghidra exports with original RNE and index native evidence.

Writes only terrain-editing evidence; does not modify the database or target.
"""
import hashlib
import json
from pathlib import Path

import capstone
import pefile

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
# Resolve from the project rather than persisted foreign-platform path strings.
GAME = ROOT.parents[2]
BINARY = GAME / "ljxsj_92385/Tiny Glade/tiny-glade.rne"
pe = pefile.PE(str(BINARY), fast_load=True)
image_base = pe.OPTIONAL_HEADER.ImageBase
native = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
native.detail = True

report = {
    "target_execution": False,
    "database_modified": False,
    "binary": str(BINARY),
    "binary_sha256": hashlib.sha256(BINARY.read_bytes()).hexdigest(),
    "validation_scope": "Native bytes/disassembly, not behavioral or system acceptance",
    "functions": [],
}
core = json.loads((OUT / "B-core/report.json").read_text())
for row in core["functions"]:
    address = int(row["address"], 16)
    name = f"{address:x}"
    memory = OUT / "B-memory" / f"memory-{name}.bin"
    if not memory.exists():
        continue
    exported = memory.read_bytes()
    original = pe.get_data(address - image_base, len(exported))
    differences = [i for i, (a, b) in enumerate(zip(original, exported)) if a != b]
    lines = []
    calls = []
    branches = []
    instructions = list(native.disasm(original, address))
    for ins in instructions:
        lines.append(f"{ins.address:x} {ins.bytes.hex()} {ins.mnemonic} {ins.op_str}")
        if ins.mnemonic == "call":
            calls.append({"address": hex(ins.address), "operand": ins.op_str})
        if ins.mnemonic.startswith("j") and ins.operands:
            operand = ins.operands[0]
            if operand.type == capstone.x86.X86_OP_IMM:
                target = operand.imm
                if not address <= target < address + len(original):
                    branches.append({"address": hex(ins.address), "target": hex(target)})
    (OUT / f"B-original-{name}.asm.txt").write_text("\n".join(lines) + "\n")
    report["functions"].append({
        "address": row["address"],
        "name": row["name"],
        "ghidra_body_bytes": row["body_bytes"],
        "compared_contiguous_bytes": len(exported),
        "original_length": len(original),
        "database_bytes_match_original": original == exported,
        "difference_offsets": differences,
        "native_sha256": hashlib.sha256(original).hexdigest(),
        "decoded_bytes": sum(ins.size for ins in instructions),
        "calls": calls,
        "out_of_range_branches": branches,
    })

dossier = json.loads((OUT / "B-dossier/dossier.json").read_text())
range_checks = []
for function in dossier["functions"]:
    for body_range in function["ranges"]:
        address = int(body_range["start"], 16)
        exported = (OUT / "B-dossier" / body_range["file"]).read_bytes()
        original = pe.get_data(address - image_base, len(exported))
        range_checks.append({
            "function": function["address"],
            "start": body_range["start"],
            "bytes": len(exported),
            "matches_original": exported == original,
            "original_sha256": hashlib.sha256(original).hexdigest(),
        })
report["database_function_range_checks"] = range_checks
report["all_database_function_ranges_match_original"] = all(row["matches_original"] for row in range_checks)

sources = {}
for version in "ABC":
    data = json.loads((ROOT / f"evidence/multi-version/{version}-sources.json").read_text())
    sources[version] = {row["path"]: row["sha256"] for row in data["files"]}
paths = sorted({path for index in sources.values() for path in index if
    "country-core" in path and any(token in path for token in [
        "/resources/terrain_", "/history/terrain_edit", "/systems/terrain/",
        "/compute_passes/rasterize_terrain_stroke", "/compute_passes/read_terrain_data",
    ])})
ownership = {
    "scope": "PDB private source ownership candidates; not full function-level call attribution",
    "dependency_policy": "Confirmed dependencies reused regardless of cross-version machine-code differences",
    "sources": [
        {"path": path, "classification": "game_business_candidate",
         "versions": {v: sources[v].get(path) for v in "ABC"},
         "missing_source_entry_is_not_feature_absence": True}
        for path in paths
    ],
    "always_reuse": ["std", "core", "alloc", "Bevy", "glam", "fastrand", "half",
                     "serde", "rkyv", "ron", "Vec", "Arc", "HashMap", "public noise/math", "allocator"],
    "rule": "Classify by definition source/crate and callsite, not Terrain generic argument or automatic label",
}
(OUT / "native-provenance.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
(OUT / "source-ownership.json").write_text(json.dumps(ownership, ensure_ascii=False, indent=2) + "\n")
print(json.dumps({"functions": len(report["functions"]),
                  "all_compared_bytes_match": all(row["database_bytes_match_original"] for row in report["functions"]),
                  "source_candidates": len(paths)}, ensure_ascii=False))
