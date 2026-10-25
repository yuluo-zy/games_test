"""Read bounded B RNE PE data; assert original identity before interpretation."""
from pathlib import Path
import hashlib
import json
import struct
import subprocess

report = Path(__file__).resolve().parent
workspace = report.parents[1]
oracle = workspace.parents[1] / "ljxsj_92385/Tiny Glade/tiny-glade.rne"
data = oracle.read_bytes()
identity = hashlib.sha256(data).hexdigest()
assert identity == "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
pe = struct.unpack_from("<I", data, 0x3C)[0]
section_count = struct.unpack_from("<H", data, pe + 6)[0]
optional_size = struct.unpack_from("<H", data, pe + 20)[0]
optional = pe + 24
image_base = struct.unpack_from("<Q", data, optional + 24)[0]
sections = []
for index in range(section_count):
    start = optional + optional_size + index * 40
    virtual_size, address, raw_size, offset = struct.unpack_from("<IIII", data, start + 8)
    sections.append((address, min(virtual_size, raw_size), offset))

def read(va, size):
    rva = va - image_base
    for address, length, offset in sections:
        if address <= rva and rva + size <= address + length:
            return data[offset + rva - address:offset + rva - address + size]
    raise ValueError(f"Unmapped file data at {va:x}")

result = {"oracle_path": str(oracle), "oracle_sha256": identity,
          "image_base": hex(image_base), "observations": []}
for va, size in [(0x142AA0CE8, 20), (0x142AA0CE8, 9), (0x142AA0CF1, 11),
                 (0x142AA152C, 20), (0x142925904, 4)]:
    raw = read(va, size)
    row = {"va": hex(va), "size": size, "hex": raw.hex()}
    if va in (0x142AA0CE8, 0x142AA0CF1):
        row["ascii"] = raw.decode()
    else:
        row["f32_le"] = struct.unpack("<" + "f" * (size // 4), raw)
    result["observations"].append(row)
(report / "original-audio-data.json").write_text(json.dumps(result, indent=2) + "\n")
with (report / "original-load-glade-audio.asm").open("w") as output:
    subprocess.run(["objdump", "--disassemble", "--start-address=0x1408be360",
                    "--stop-address=0x1408bee10", str(oracle)], check=True, stdout=output)
print("B RNE identity, bounded audio instructions and data recorded")
