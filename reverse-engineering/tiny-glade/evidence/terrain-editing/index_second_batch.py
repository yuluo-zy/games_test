"""Verify second-batch body ranges and extract game field schema constants.

Serde implementations are dependencies; only game names/field offsets are indexed.
"""
import hashlib
import json
from pathlib import Path
import pefile

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[1]
GAME = ROOT.parents[2]
raw = (GAME / "ljxsj_92385/Tiny Glade/tiny-glade.rne").read_bytes()
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
checks = []
for folder in ["B-second-dossier", "B-helper-dossier"]:
    dossier = json.loads((OUT / folder / "dossier.json").read_text())
    for fn in dossier["functions"]:
        for body in fn["ranges"]:
            address = int(body["start"], 16)
            exported = (OUT / folder / body["file"]).read_bytes()
            original = pe.get_data(address - base, len(exported))
            checks.append({"function": fn["address"], "start": body["start"],
                           "bytes": len(exported), "matches_original": exported == original,
                           "original_sha256": hashlib.sha256(original).hexdigest()})
function_count = len({row["function"] for row in checks})

def text(address, length):
    return pe.get_data(address - base, length).decode("ascii")

schema = {
    "source_function": "0x1417c1590",
    "offset_evidence": "B-second/1417c1590.asm.txt",
    "discriminant_policy": "Native niche encoding, not a normal u32 tag",
    "variants": [
        {"name": text(0x142ce1bd0, 16), "first_qword": "0x8000000000000000",
         "fields": {"index": "0x20 u32", "stroke_pts": "0x08 Vec<Vec3> (cap=+8, ptr=+16, len=+24)", "height_ws": "0x24 f32"}},
        {"name": text(0x142ce1be0, 17), "first_qword": "0x8000000000000001",
         "fields": {"index": "0x08 u32", text(0x142ce1d35, 14): "0x0c f32",
                    text(0x142ce1d43, 13): "0x10 f32", text(0x142ce1d50, 12): "0x14 f32",
                    text(0x142ce1d5c, 11): "0x18 f32"}},
        {"name": text(0x142ce1bf1, 17), "first_qword": "Vec allocation capacity; niche fallback branch",
         "fields": {text(0x142ce1686, 2): "0x20 u32",
                    text(0x142ce1c4d, 19): "0x00 Vec<Vec3> (cap=+0, ptr=+8, len=+16)",
                    text(0x142ce1c60, 8): "0x18 Vec2"}},
    ],
    "limitations": ["Native layouts specific to B", "index/id not generally interchangeable",
                    "Vec/serde implementation reused; no parser reconstruction", "No snapshot format acceptance"],
}
report = {"target_execution": False, "database_modified": False,
          "binary_sha256": hashlib.sha256(raw).hexdigest(),
          "decompilation_count": function_count, "range_checks": checks,
          "all_ranges_match_original": all(x["matches_original"] for x in checks),
          "scope": "Byte/source verification, not behavior acceptance"}
(OUT / "B-second-provenance.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
(OUT / "B-history-schema.json").write_text(json.dumps(schema, ensure_ascii=False, indent=2) + "\n")
print(json.dumps({"functions": function_count, "ranges": len(checks),
                  "all_ranges_match": report["all_ranges_match_original"]}))
if not checks or not report["all_ranges_match_original"]:
    raise SystemExit(1)
