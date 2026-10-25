"""Read bounded Rust fmt string pieces from original B RNE, never execute it."""
import hashlib
import json
from pathlib import Path
import struct

source = Path(r"D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne")
raw = source.read_bytes()
digest = hashlib.sha256(raw).hexdigest()
assert digest == "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
pe = struct.unpack_from("<I", raw, 0x3C)[0]
sections_count, optional_size = struct.unpack_from("<H12xH", raw, pe + 6)
optional = pe + 24
base = struct.unpack_from("<Q", raw, optional + 24)[0]
sections = []
for index in range(sections_count):
    offset = optional + optional_size + 40 * index
    vsize, rva, size, position = struct.unpack_from("<4I", raw, offset + 8)
    sections.append((rva, max(vsize, size), position))

def read(va, size):
    if size == 0:
        return b""
    rva = va - base
    for start, extent, position in sections:
        if start <= rva and rva + size <= start + extent:
            return raw[position + rva - start:position + rva - start + size]
    raise ValueError(hex(va))

# Address/count comes from run @ 0x140618310's fmt::Arguments.pieces.
formats = [
    ("server_date", 0x142A0A7A8, 3, "Debug", 4),
    ("local_skew", 0x142A0A858, 2, "Debug", 4),
    ("invalid_date", 0x142A0A8B0, 1, "Warn", 2),
    ("etag", 0x142A0A938, 1, "Debug", 4),
    ("events_fetched", 0x142A0A998, 1, "Debug", 4),
    ("events_304", 0x142A0AA20, 1, "Debug", 4),
    ("events_status", 0x142A0AA78, 1, "Error", 1),
    ("download", 0x142A0AAB8, 1, "Info", 3),
    ("blob_fetched", 0x142A0AB70, 2, "Info", 3),
    ("blob_status", 0x142A0ABF0, 2, "Error", 1),
    ("cache_hit", 0x142A0AC50, 2, "Debug", 4),
    ("trim_start", 0x142A0ACA0, 1, "Info", 3),
    ("trim_complete", 0x142A0AD00, 1, "Info", 3),
    ("entry_error", 0x142A0AD50, 2, "Error", 1),
    ("rogue_removed", 0x142A0ADA8, 1, "Warn", 2),
    ("old_removed", 0x142A0ADF0, 1, "Info", 3),
    ("corrupt_removed", 0x142A0AE58, 3, "Warn", 2),
    ("trim_error", 0x142A0AEC0, 1, "Error", 1),
    ("fetch_error", 0x142A0AF00, 1, "Error", 1),
    ("shutdown", 0x142A0AF48, 1, "Info", 3),
    ("blob_error", 0x142A0AF90, 1, "Error", 1),
]
result = []
for name, va, count, level, discriminant in formats:
    pieces = []
    for i in range(count):
        pointer, length = struct.unpack("<QQ", read(va + 16 * i, 16))
        assert length < 256
        pieces.append({"va": hex(pointer), "length": length, "text": read(pointer, length).decode("utf-8")})
    result.append({"name": name, "pieces_va": hex(va), "pieces": pieces, "level": level, "native_level": discriminant})
Path(__file__).with_name("log-formats.json").write_text(json.dumps({"source": str(source), "sha256": digest, "address_basis": "original B run.c pieces pointer/count and explicit level words", "formats": result}, indent=2), encoding="utf-8")
