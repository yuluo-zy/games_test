"""Recheck the retained B camera evidence and the startup correction ranges."""
import hashlib
import json
import struct
from pathlib import Path

ROOT = Path(__file__).resolve().parents[5]
OUT = Path(__file__).resolve().parent
ORIGINAL = ROOT / "ljxsj_92385/Tiny Glade/tiny-glade.rne"
PATCHED = ROOT / "reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe"
PACK = ROOT / "reverse-engineering/tiny-glade/skeleton/task-index/work/TG-8ed1ee9951d9d8c4b0/oschain-20261018"


def image_reader(path):
    raw = path.read_bytes()
    pe = struct.unpack_from("<I", raw, 0x3C)[0]
    count = struct.unpack_from("<H", raw, pe + 6)[0]
    opt = struct.unpack_from("<H", raw, pe + 20)[0]
    sections = [struct.unpack_from("<IIII", raw, pe + 24 + opt + i * 40 + 8) for i in range(count)]

    def read(va, length):
        rva = va - 0x140000000
        for virtual_size, virtual_address, raw_size, raw_offset in sections:
            if virtual_address <= rva < virtual_address + virtual_size:
                start = raw_offset + rva - virtual_address
                assert start + length <= raw_offset + raw_size
                return raw[start:start + length]
        raise ValueError(hex(va))

    return raw, read


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


original, read = image_reader(ORIGINAL)
patched, patched_read = image_reader(PATCHED)
report = {"original_sha256": sha(original), "patched_sha256": sha(patched), "camera_ranges": []}
for name in ("camera-native-manifest.json", "camera-registration-projection-native-manifest.json"):
    manifest = json.loads((PACK / name).read_text(encoding="utf-8"))
    for fn in manifest["functions"]:
        body = read(int(fn["va"], 16), fn["length"])
        report["camera_ranges"].append({
            "va": fn["va"], "length": fn["length"], "sha256": sha(body),
            "matches_frozen_manifest": sha(body) == fn["raw_sha256"],
            "patched_bytes_identical": body == patched_read(int(fn["va"], 16), fn["length"]),
            "manifest": name,
        })
report["grading_constants"] = {
    "cinema_lut": read(0x142AA0E00, 22).decode(),
    "cinema_params": struct.unpack("<4f", read(0x142AA0190, 16)),
    "theme_root": read(0x142AEF51A, 12).decode(),
    "theme_file": read(0x142AA0DA7, 12).decode(),
}
report["snapshot_ui_ranges"] = []
for va, length in ((0x140148CB0, 3120), (0x1401453D0, 1159), (0x140143040, 5073)):
    body = read(va, length)
    report["snapshot_ui_ranges"].append({"va": hex(va), "length": length,
        "sha256": sha(body), "patched_bytes_identical": body == patched_read(va, length),
        "length_source": "evidence/country-core-audit/data/procs.tsv"})
report["startup_ranges"] = []
for va, length in ((0x1408639F0, 2611), (0x1408BF9C0, 2346)):
    body = read(va, length)
    report["startup_ranges"].append({"va": hex(va), "length": length,
        "sha256": sha(body), "patched_bytes_identical": body == patched_read(va, length),
        "length_source": "evidence/country-core-audit/data/procs.tsv"})
report["all_camera_ranges_match"] = all(
    row["matches_frozen_manifest"] and row["patched_bytes_identical"] for row in report["camera_ranges"]
)
(OUT / "native-verification.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
print(json.dumps({"ranges": len(report["camera_ranges"]), "all_match": report["all_camera_ranges_match"],
                  "original_sha256": report["original_sha256"]}))
