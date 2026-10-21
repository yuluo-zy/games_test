"""Read-only bounded x64 disassembly; never execute the game or edit a database."""
import hashlib
import json
from pathlib import Path

import capstone
import pefile

ROOT = Path(__file__).resolve().parents[2]
INPUT = ROOT / "workspace/input/tiny-glade.exe"
OUT = Path(__file__).resolve().parent
EXPECTED = "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
data = INPUT.read_bytes()
assert hashlib.sha256(data).hexdigest() == EXPECTED, "use the original B executable"
pe = pefile.PE(data=data, fast_load=True)
decoder = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
functions = {
    "async-loader": (0x1400D7440, 2826),
    "render-outer": (0x1404E8010, 16279),
    "compile-main": (0x140161CD0, 10961),
    "texture-new-2d": (0x140BD52A0, 49),
}
records = []
for name, (address, size) in functions.items():
    body = pe.get_data(address - pe.OPTIONAL_HEADER.ImageBase, size)
    instructions = list(decoder.disasm(body, address))
    text = "\n".join(f"{i.address:#x}: {i.mnemonic} {i.op_str}" for i in instructions) + "\n"
    path = OUT / (name + ".asm")
    path.write_text(text, encoding="utf-8")
    records.append({"name": name, "address": hex(address), "size": size,
                    "body_sha256": hashlib.sha256(body).hexdigest(),
                    "disassembly": path.name,
                    "disassembly_sha256": hashlib.sha256(text.encode()).hexdigest()})
(OUT / "bounded-native-manifest.json").write_text(json.dumps({
    "original_sha256": EXPECTED, "input": str(INPUT),
    "evidence_type": "static original instruction decoding only; no runtime",
    "functions": records,
    "limits": ["No complete Rust/PDB prototype reconstruction.",
               "No game code, GPU graph or Rust test was executed."]
}, indent=2) + "\n", encoding="utf-8")
