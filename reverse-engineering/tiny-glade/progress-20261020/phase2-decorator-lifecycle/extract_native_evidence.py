"""Read only the original PE and bounded retained symbols; no target execution."""
from pathlib import Path
import hashlib
import json
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

ROOT = Path(__file__).resolve().parents[4]
OUT = Path(__file__).resolve().parent
PE_PATH = ROOT / "reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe"
EXPECTED = "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
raw = PE_PATH.read_bytes()
assert hashlib.sha256(raw).hexdigest() == EXPECTED
pe = pefile.PE(data=raw)
base = pe.OPTIONAL_HEADER.ImageBase
md = Cs(CS_ARCH_X86, CS_MODE_64)

ranges = [
    ("decorators_on_glade_unload", 0x140347090, 9451),
    ("registry_unload_interval", 0x1403481B6, 0x14034831E - 0x1403481B6),
    ("storage_clear_modification_tracking", 0x140A1E720, 791),
]
for label, start in [
    ("drop_StairColliderRegistry", 0x14060A9E0),
    ("invalidate_dirty_stair_artifacts", 0x140B37180),
    ("remove_collider_StairGraphEdge", 0x140252AD0),
    ("gen_stairs_collider_and_rt_geo", 0x1407DE8B0),
    ("gen_stairs_structural_collider", 0x1407DEC90),
    # 140178a70 is inside this physical wrapper, not its function entry.
    ("pillars_process_proposals_containing_wrapper", 0x1401788B0),
    ("create_structural_colliders_and_initial_graph", 0x1407D20C0),
    ("upsert_collider_edge_index", 0x141299CA0),
    ("new_convex", 0x1408CEC10),
]:
    entries = [e.struct for e in pe.DIRECTORY_ENTRY_EXCEPTION if e.struct.BeginAddress + base == start]
    assert len(entries) == 1, (label, entries)
    ranges.append((label, start, entries[0].EndAddress - entries[0].BeginAddress))

manifest = {"oracle": str(PE_PATH.relative_to(ROOT)), "oracle_sha256": EXPECTED,
            "scope": "PE byte/disassembly verification; labels from retained B metadata, no original execution", "ranges": []}
for label, start, length in ranges:
    data = pe.get_data(start - base, length)
    instructions = list(md.disasm(data, start))
    path = OUT / f"{label}.asm.txt"
    path.write_text("\n".join(f"{i.address:016x} {i.bytes.hex():<32} {i.mnemonic} {i.op_str}" for i in instructions) + "\n", encoding="utf8")
    manifest["ranges"].append({"label": label, "start": hex(start), "length": length,
        "sha256": hashlib.sha256(data).hexdigest(), "asm": path.name,
        "instructions": len(instructions), "decoded_bytes": sum(i.size for i in instructions),
        "direct_calls": [{"at": hex(i.address), "target": i.op_str} for i in instructions if i.mnemonic == "call"]})
(OUT / "native-byte-manifest.json").write_text(json.dumps(manifest, indent=2), encoding="utf8")
print(json.dumps({"oracle_sha256": EXPECTED, "ranges": [{k: r[k] for k in ("label", "length", "sha256")} for r in manifest["ranges"]]}, indent=2))
