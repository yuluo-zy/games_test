"""Read-only PE byte validation; no tool database or installed file is mutated."""
from pathlib import Path
import hashlib
import json
import struct

HERE = Path(__file__).resolve().parent
SOURCE = Path("D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne")
data = SOURCE.read_bytes()
pe = struct.unpack_from("<I", data, 0x3C)[0]
assert data[pe:pe + 4] == b"PE\0\0"
sections, optsize = struct.unpack_from("<H", data, pe + 6)[0], struct.unpack_from("<H", data, pe + 20)[0]
optional = pe + 24
assert struct.unpack_from("<H", data, optional)[0] == 0x20B
base = struct.unpack_from("<Q", data, optional + 24)[0]
table = optional + optsize

def read(va, size):
    rva = va - base
    for index in range(sections):
        entry = table + index * 40
        virtual_size, start, raw_size, raw = struct.unpack_from("<IIII", data, entry + 8)
        if start <= rva and rva + size <= start + raw_size:
            return data[raw + rva - start:raw + rva - start + size]
    raise ValueError(f"VA {va:x} is not fully file-backed")

expected = json.loads((HERE / "original.json").read_text())
assert hashlib.sha256(data).hexdigest() == expected["sha256"]
records = []
for first, end, name, digest in expected["function_ranges"]:
    first, end = int(first, 16), int(end, 16)
    actual = hashlib.sha256(read(first, end - first)).hexdigest()
    assert actual == digest, name
    records.append({"name": name, "start": hex(first), "end": hex(end), "sha256": actual})
assert struct.unpack("<f", read(0x142925988, 4))[0] == 5.0
# Fresh independent facts used by the Rust branch, beyond the retained C text.
assert read(0x140a8a4ba, 3) == bytes.fromhex("418b41")  # changed tick before state test
assert read(0x140a8a4cc, 4) == bytes.fromhex("4180f902")  # cmp state,2
assert read(0x140a8a552, 3) == bytes.fromhex("c60701")    # interrupt -> state1
assert read(0x14234b73c, 4) == bytes.fromhex("80780801")  # last Finished payload1
assert read(0x14234b749, 4) == bytes.fromhex("c6400a00")  # photo.hide_ui=false
assert read(0x14234b757, 6) == bytes.fromhex("41c644240101")  # screen.enabled=true
assert read(0x14234b799, 7) == bytes.fromhex("c704c813000000")  # request Photomode tag19
out = {"source": str(SOURCE), "sha256": expected["sha256"], "functions": records,
       "multiplier": 5.0, "raw_instruction_assertions": 7,
       "claim": "original-byte identity and branch facts; not runtime game equivalence"}
(HERE / "independent-byte-verification.json").write_text(json.dumps(out, indent=2) + "\n")
print(json.dumps({"functions": len(records), "raw_instruction_assertions": 7, "multiplier": 5.0}))
