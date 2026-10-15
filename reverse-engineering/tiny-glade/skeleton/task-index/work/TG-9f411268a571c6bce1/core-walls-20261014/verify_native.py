"""Execute bounded B wall business instructions; emit independent Rust fixtures.

Constructor/map insertion and notification use dependency bridges. Setter tails
start AFTER the successful lookup/notification and execute the actual stores.
This proves those stores, not the original ECS scheduler or complete publisher.
"""
import argparse
import hashlib
import json
import struct
from pathlib import Path

import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import (
    UC_X86_REG_RAX, UC_X86_REG_RBX, UC_X86_REG_RCX, UC_X86_REG_RDX,
    UC_X86_REG_RSI, UC_X86_REG_R8, UC_X86_REG_R9, UC_X86_REG_R14,
    UC_X86_REG_R15, UC_X86_REG_RSP, UC_X86_REG_XMM2,
)

SHA = "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
BASE = 0x140000000
DATA = 0x20000000
STACK = 0x30000000
STOP = 0x40000000
INSERT = 0x141529BB0
NOTIFY = 0x1402CA2F0
PANIC = 0x1428D9390
STORE_RANGES = {
    "height": (0x140AAAE64, 0x140AAAE6F),
    "elevation": (0x140AAAC84, 0x140AAAC8F),
    "arch_height": (0x140AAA8D4, 0x140AAA8DF),
    "plaster": (0x140AAA6F4, 0x140AAA6FF),
    "style": (0x140AAA514, 0x140AAA51C),
    "freehand": (0x140AA9BA4, 0x140AA9BAC),
    "arch_type": (0x140AAA074, 0x140AAA07C),
    "supports": (0x140AA99C4, 0x140AA99CC),
    "enclosure": (0x140AA97D4, 0x140AA97E7),
    "non_enclosed": (0x140AAAAA7, 0x140AAAAB0),
}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("binary", type=Path)
    ap.add_argument("output", type=Path)
    args = ap.parse_args()
    binary = args.binary.read_bytes()
    assert hashlib.sha256(binary).hexdigest() == SHA, "wrong native oracle"
    pe = pefile.PE(data=binary)
    uc = Uc(UC_ARCH_X86, UC_MODE_64)
    pages = set()

    def map_code(va, size=1):
        for page in range(va & ~4095, (va + size + 4095) & ~4095, 4096):
            if page not in pages:
                uc.mem_map(page, 4096)
                data = pe.get_data(page - BASE, 4096)
                if data:
                    uc.mem_write(page, data)
                pages.add(page)

    for a, b in STORE_RANGES.values():
        map_code(a, b - a)
    for va, size in [(0x14085A370, 0x230), (0x1408C5690, 0x30),
                     (INSERT, 1), (NOTIFY, 1), (PANIC, 1),
                     (0x142A94910, 16), (0x142A8CC60, 4),
                     (0x143634D90, 1)]:
        map_code(va, size)
    uc.mem_map(DATA, 0x4000)
    uc.mem_map(STACK, 0x10000)
    uc.mem_map(STOP, 4096)
    uc.mem_write(INSERT, b"\xc3")
    uc.mem_write(NOTIFY, b"\xc3")
    captured = {}

    def external(machine, address, size, context):
        if address == INSERT:
            captured["state"] = bytes(machine.mem_read(machine.reg_read(UC_X86_REG_R9), 112))
            captured["id"] = machine.reg_read(UC_X86_REG_R8)
            output = machine.reg_read(UC_X86_REG_RCX)
            machine.mem_write(output, struct.pack("<Q", 0x8000000000000000))
        elif address == NOTIFY:
            captured["notice"] = [machine.reg_read(UC_X86_REG_RCX) - DATA,
                                  machine.reg_read(UC_X86_REG_RDX)]
        elif address == PANIC:
            captured["panic"] = True
            machine.emu_stop()

    uc.hook_add(UC_HOOK_CODE, external)

    def stack():
        sp = STACK + 0x8008
        uc.mem_write(sp, struct.pack("<Q", STOP))
        uc.reg_write(UC_X86_REG_RSP, sp)
        return sp

    fixture = {"binary_sha256": SHA, "setters": [], "constructors": [],
               "default_arch_height": [], "reset_max_id": []}
    initial = bytes((i * 17 + 3) & 255 for i in range(112))
    values = [0, 0x80000000, 0x3F800000, 0x40333333, 0x7FC00001,
              0xFF800000, 0x7F800000, 0xAABBCCDD]
    for operation, (start, end) in STORE_RANGES.items():
        for bits in values:
            payload = struct.pack("<I", bits) + bytes(range(4, 28))
            uc.mem_write(DATA, initial)
            uc.mem_write(DATA + 0x1000, payload)
            uc.reg_write(UC_X86_REG_RSI, DATA + 0x1000)
            uc.reg_write(UC_X86_REG_R14, DATA)
            uc.reg_write(UC_X86_REG_R15, 112)
            if operation == "non_enclosed":
                uc.reg_write(UC_X86_REG_RBX, DATA)
                uc.reg_write(UC_X86_REG_R14, 112)
            uc.emu_start(start, end, count=100)
            fixture["setters"].append({"operation": operation, "input": payload.hex(),
                "before": initial.hex(), "after": bytes(uc.mem_read(DATA, 112)).hex(),
                "entry": hex(start), "exit": hex(end)})

    for demo in [False, True]:
        for counter, height in [(0, 0x40333333), (99, 0x80000000),
                                (0xFFFFFFFFFFFFFFFF, 0x7FC00001)]:
            captured.clear()
            uc.mem_write(DATA, bytes(256))
            uc.mem_write(DATA + 0xC0, struct.pack("<Q", counter))
            uc.mem_write(DATA + 0x1000, initial[:56])
            uc.mem_write(0x143634D90, bytes([demo]))
            sp = stack()
            uc.mem_write(sp + 0x28, b"\x05")
            uc.reg_write(UC_X86_REG_RCX, DATA)
            uc.reg_write(UC_X86_REG_RDX, DATA + 0x1000)
            uc.reg_write(UC_X86_REG_XMM2, height)
            uc.reg_write(UC_X86_REG_R9, 2)
            uc.emu_start(0x14085A370, STOP, count=1000)
            fixture["constructors"].append({"counter": counter, "height_bits": height,
                "demo": demo, "style": 2, "clutter_version": 5,
                "id": captured["id"], "notice": captured["notice"],
                "state": captured["state"].hex(),
                "counter_after": struct.unpack("<Q", uc.mem_read(DATA + 0xC0, 8))[0]})

    for bits in values:
        uc.mem_write(DATA, struct.pack("<I", bits))
        uc.reg_write(UC_X86_REG_RCX, DATA)
        stack()
        uc.emu_start(0x1408C5690, STOP, count=100)
        fixture["default_arch_height"].append({"bits": bits,
            "result": bool(uc.reg_read(UC_X86_REG_RAX))})
    for counter in [0, 99, 0xFFFFFFFFFFFFFFFF]:
        before = initial + bytes(144)
        uc.mem_write(DATA, before)
        uc.mem_write(DATA + 0xC0, struct.pack("<Q", counter))
        uc.reg_write(UC_X86_REG_RCX, DATA)
        stack()
        uc.emu_start(0x14085A550, STOP, count=100)
        assert struct.unpack("<Q", uc.mem_read(DATA + 0xC0, 8))[0] == 0
        assert bytes(uc.mem_read(DATA, 112)) == initial
        fixture["reset_max_id"].append(counter)
    args.output.write_text(json.dumps(fixture, indent=2) + "\n")
    print(json.dumps({"sha256": SHA, "setters": len(fixture["setters"]),
                      "constructors": len(fixture["constructors"]),
                      "output": str(args.output)}))


if __name__ == "__main__":
    main()
