"""Bounded original-RNE wire/dispatch audit; no database or game-file writes."""
import hashlib
import json
from pathlib import Path
import struct
import subprocess

ROOT = Path(__file__).resolve().parent
TARGET = Path("D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne")
blob = TARGET.read_bytes()
assert hashlib.sha256(blob).hexdigest() == "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
pe = struct.unpack_from("<I", blob, 0x3c)[0]
section_count = struct.unpack_from("<H", blob, pe + 6)[0]
sections_offset = pe + 24 + struct.unpack_from("<H", blob, pe + 20)[0]
sections = []
for index in range(section_count):
    size, rva, raw_size, raw = struct.unpack_from("<IIII", blob, sections_offset + 40 * index + 8)
    sections.append((rva, max(size, raw_size), raw))

def read(va, size):
    rva = va - 0x140000000
    for start, span, raw in sections:
        if start <= rva and rva + size <= start + span:
            return blob[raw + rva - start:raw + rva - start + size]
    raise ValueError(hex(va))

tables = {}
for table, count in [(0x1429bb2fc, 17), (0x142c5badc, 17), (0x1429bb340, 15)]:
    rows = []
    for index in range(count):
        target = table + struct.unpack("<i", read(table + 4 * index, 4))[0]
        code = read(target, 110)
        name = None
        for pos in range(len(code) - 13):
            if code[pos:pos + 3] == b"\x4c\x8d\x05" and code[pos + 7:pos + 9] == b"\x41\xb9":
                address = target + pos + 7 + struct.unpack_from("<i", code, pos + 3)[0]
                length = struct.unpack_from("<I", code, pos + 9)[0]
                name = read(address, length).decode("ascii")
                break
        rows.append({"tag": index, "target": hex(target), "name": name})
    tables[hex(table)] = rows

result = {
    "target": str(TARGET), "sha256": hashlib.sha256(blob).hexdigest(),
    "serializer_tables": tables,
    "supports_fields": {
        hex(address): read(address, size).decode("ascii")
        for address, size in [(0x142c5b69b, 23), (0x142c5b6b2, 22)]
    },
    "functions": {},
}
with (ROOT / "wire-independent-original.asm").open("w", encoding="utf8") as output:
    for address, size in [
        (0x141400220, 5944), (0x140316660, 2813),
        (0x140aaa350, 475), (0x140aaa530, 478), (0x140aa9800, 475),
        (0x140abfb30, 9188),
    ]:
        result["functions"][hex(address)] = {
            "size": size, "sha256": hashlib.sha256(read(address, size)).hexdigest(),
        }
        asm = subprocess.run([
            "C:/Program Files/LLVM/bin/llvm-objdump.exe", "-d",
            f"--start-address={hex(address)}", f"--stop-address={hex(address + size)}", str(TARGET),
        ], check=True, stdout=subprocess.PIPE).stdout.decode("utf8")
        output.write(asm)
(ROOT / "wire-independent-original.json").write_text(json.dumps(result, indent=2), encoding="utf8")
print(json.dumps({"tables": tables, "supports_fields": result["supports_fields"]}, indent=2))
