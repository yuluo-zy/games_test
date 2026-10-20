"""Record original PE ranges and the source candidate; no target modification."""
from pathlib import Path
import hashlib
import json
import struct

root = Path("D:/game")
out = root / "reverse-engineering/tiny-glade/evidence/round12/audio-completion"
original = root / "ljxsj_92385/Tiny Glade/tiny-glade.rne"
data = original.read_bytes()
sha = lambda b: hashlib.sha256(b).hexdigest()
assert sha(data) == "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
pe = struct.unpack_from("<I", data, 0x3C)[0]
sections, opt_size = struct.unpack_from("<H12xH", data, pe + 6)
image_base = struct.unpack_from("<Q", data, pe + 24 + 24)[0]
section_offset = pe + 24 + opt_size

def memory(va, size):
    rva = va - image_base
    for i in range(sections):
        at = section_offset + 40 * i
        virtual_size, virtual, raw_size, raw = struct.unpack_from("<IIII", data, at + 8)
        if virtual <= rva and rva + size <= virtual + raw_size:
            return data[raw + rva - virtual:raw + rva - virtual + size]
    raise ValueError(hex(va))

ranges = [
    ("AudioCore::new", 0x140A54A40, 5339, "PDB full function"),
    ("AudioSamplesLoadedPromise::recv", 0x140A57100, 318, "PDB full function"),
    ("delayed loader closure", 0x140A520B0, 1258, "PDB full function"),
    ("std sync_channel", 0x1408D9D30, 777, "PDB full function"),
    ("std array recv prefix", 0x141679D70, 0x390, "bounded dependency policy check"),
    ("std array send prefix", 0x14167B9B0, 0x3B0, "bounded dependency policy check"),
]
report = {
    "original_path": str(original), "original_sha256": sha(data),
    "ranges": [dict(name=n, va=hex(v), length=s, scope=k, sha256=sha(memory(v, s))) for n,v,s,k in ranges],
    "sources": {}, "cargo_run_by_this_agent": False,
    "acceptance": "pending central single-owner validation",
}
for relative in [
    "tiny-glade/crates/country-core/src/resources/native_audio.rs",
    "tiny-glade/crates/country-core/src/resources/audio_resources.rs",
]:
    report["sources"][relative] = sha((root / relative).read_bytes())
(out / "evidence.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print(json.dumps({"original_sha256": report["original_sha256"], "ranges": len(ranges), "status": report["acceptance"]}))
