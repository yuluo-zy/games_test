"""Execute the whole B wall_highlight with controlled Bevy read inputs.

The public hash computation is bridged to zero and the hashbrown table has a
matching control byte. Native last-event selection, key comparison, early exits,
state writes and tick writes all execute. No game output is supplied by hooks.
"""
import argparse
import hashlib
import json
import struct
from pathlib import Path

import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import (
    UC_X86_REG_RAX, UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_R8,
    UC_X86_REG_R9, UC_X86_REG_RSP,
)

SHA = "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
BASE, DATA, STACK, STOP = 0x140000000, 0x20000000, 0x30000000, 0x40000000


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("binary", type=Path)
    ap.add_argument("output", type=Path)
    args = ap.parse_args()
    raw = args.binary.read_bytes()
    assert hashlib.sha256(raw).hexdigest() == SHA
    pe = pefile.PE(data=raw)
    uc = Uc(UC_ARCH_X86, UC_MODE_64)
    for section in pe.sections:
        size = (max(section.Misc_VirtualSize, section.SizeOfRawData) + 4095) & ~4095
        if size:
            uc.mem_map(BASE + section.VirtualAddress, size)
            uc.mem_write(BASE + section.VirtualAddress, section.get_data())
    uc.mem_map(DATA, 0x4000)
    uc.mem_map(STACK, 0x10000)
    uc.mem_map(STOP, 4096)
    uc.mem_write(0x1405B0D70, b"\xc3")

    def bridge(machine, address, size, context):
        if address == 0x1405B0D70:
            machine.reg_write(UC_X86_REG_RAX, 0)

    uc.hook_add(UC_HOOK_CODE, bridge)

    def put(offset, fmt, *values):
        uc.mem_write(DATA + offset, struct.pack("<" + fmt, *values))

    out = {"binary_sha256": SHA, "entry": "0x140981390", "exit": "ret@0x1409815cb",
           "bridges": ["0x1405b0d70 public hash computation -> controlled hash 0"], "cases": []}
    event_sets = [[], [(1, 1)], [(1, 0)], [(1, 2)], [(1, 255)], [(2, 1)],
                  [(1, 1), (2, 2)], [(2, 2), (1, 1)], [(1, 1), (1, 2)]]
    for events in event_sets:
        for split in range(len(events) + 1):
            for consumed in range(len(events) + 1):
                for enabled in [False, True]:
                    uc.mem_write(DATA, bytes(0x4000))
                    # ResMut {value@0, changed_tick@16, this_tick@28}.
                    for resource, target, tick in [(0, 0x800, 0x900), (0x40, 0x840, 0x904)]:
                        put(resource, "Q", DATA + target)
                        put(resource + 16, "Q", DATA + tick)
                        put(resource + 28, "I", 17)
                    put(0x800, "QQB", (1 << 32) | 7, 9, 3)
                    put(0x840, "fffB", 0.3, 0.4, 0.5, enabled)
                    # EventReader pointer pair; both buffers keep 24-byte EventInstances.
                    put(0x80, "QQ", DATA + 0x100, DATA + 0x180)
                    put(0x100, "Q", consumed)
                    put(0x180, "QQQQ", split, DATA + 0x1000, split, 0)
                    put(0x1A0, "QQQQ", len(events) - split, DATA + 0x1800, len(events) - split, split)
                    put(0x1C0, "Q", len(events))
                    for i, (wall, ty) in enumerate(events):
                        offset = 0x1000 + i * 24 if i < split else 0x1800 + (i - split) * 24
                        put(offset, "QQB", i, wall, ty)
                    put(0xC0, "Q", DATA + 0x600)
                    put(0x600, "QQQQ", DATA + 0xA00, 15, 14, 1)
                    uc.mem_write(DATA + 0xA00, b"\x00" + b"\xff" * 31)
                    put(0x9F0, "QQ", 1, (1 << 32) | 42)
                    sp = STACK + 0x8008
                    uc.mem_write(sp, struct.pack("<Q", STOP))
                    for reg, value in [(UC_X86_REG_RCX, DATA), (UC_X86_REG_RDX, DATA + 0x40),
                        (UC_X86_REG_R8, DATA + 0x80), (UC_X86_REG_R9, DATA + 0xC0), (UC_X86_REG_RSP, sp)]:
                        uc.reg_write(reg, value)
                    uc.emu_start(0x140981390, STOP, count=10000)
                    assert uc.reg_read(UC_X86_REG_RSP) == sp + 8
                    entity, wall, ty = struct.unpack("<QQB", uc.mem_read(DATA + 0x800, 17))
                    color = list(struct.unpack("<III", uc.mem_read(DATA + 0x840, 12)))
                    out["cases"].append({"events": events, "split": split, "consumed": consumed,
                        "enabled": enabled, "entity_bits": entity, "wall": wall, "ty": ty,
                        "color_bits": color, "enabled_after": bool(uc.mem_read(DATA + 0x84C, 1)[0]),
                        "cursor_after": struct.unpack("<Q", uc.mem_read(DATA + 0x100, 8))[0],
                        "wall_tick": struct.unpack("<I", uc.mem_read(DATA + 0x900, 4))[0],
                        "editing_tick": struct.unpack("<I", uc.mem_read(DATA + 0x904, 4))[0]})
    args.output.write_text(json.dumps(out, indent=2) + "\n")
    print(json.dumps({"cases": len(out["cases"]), "output": str(args.output)}))


if __name__ == "__main__":
    main()
