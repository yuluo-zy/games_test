"""Bounded original B instruction/rodata evidence, without executing the target."""
import hashlib
import json
import struct
from pathlib import Path

import capstone
import pefile

ROOT = Path(__file__).resolve().parents[2]
INPUT = ROOT / "workspace/input/tiny-glade.exe"
OUT = Path(__file__).resolve().parent
EXPECTED = "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
data = INPUT.read_bytes()
assert hashlib.sha256(data).hexdigest() == EXPECTED
pe = pefile.PE(data=data, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
decoder = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
records = []
for name, address, size in [
    ("welcome-ui", 0x141EBE130, 2158),
    ("welcome-slot-one", 0x140FC9230, 771),
    ("welcome-slot-two", 0x140FC9540, 691),
    ("menu-hotkey", 0x14067E6D0, 505),
    ("menu-button-pre-update", 0x14067EA20, 45),
]:
    body = pe.get_data(address - base, size)
    lines = [f"{i.address:#x}: {i.mnemonic} {i.op_str}" for i in decoder.disasm(body, address)]
    text = "\n".join(lines) + "\n"
    (OUT / f"{name}.asm").write_text(text, encoding="utf-8")
    records.append(dict(name=name, address=hex(address), size=size,
                        body_sha256=hashlib.sha256(body).hexdigest()))

strings = []
for address, length, expected in [
    (0x142F9AFEA, 7, "welcome"),
    (0x142F9AFF1, 12, "menu/welcome"),
    (0x142F9AFFD, 17, "menu/welcome_demo"),
    (0x142F9B00E, 7, "buttons"),
    (0x142F9B015, 19, "steamdeck/welcome_1"),
    (0x142F9B028, 19, "steamdeck/welcome_2"),
    (0x142BB3EAD, 14, "Welcome.LetsGo"),
    (0x142BB4758, 6, "\u27a1\ufe0f"),
    (0x142BB3EBB, 5, " \U0001f411"),
]:
    value = pe.get_data(address - base, length).decode("utf-8")
    assert value == expected, (hex(address), value, expected)
    strings.append(dict(address=hex(address), byte_length=length, text=value))
metrics_factor = struct.unpack("<f", pe.get_data(0x1429258D0-base, 4))[0]
assert metrics_factor == 1.5
(OUT / "native-manifest.json").write_text(json.dumps(dict(
    original_sha256=EXPECTED, functions=records, strings=strings,
    metrics_factor=dict(address="0x1429258d0", value=metrics_factor),
    limits=["Static decoding only; no original or restored game executed.",
            "No full tooltip restoration or full MenuFlow claim."]), indent=2) + "\n", encoding="utf-8")
