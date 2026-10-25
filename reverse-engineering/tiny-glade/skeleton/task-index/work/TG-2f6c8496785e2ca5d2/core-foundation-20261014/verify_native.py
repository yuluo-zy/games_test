"""Run complete bounded B leaf entries, without replacing business instructions.

The menu inputs exercise only the observed prefix, not enum payload ownership.
Window conversion, UI defaults, highlight clear and brightness run at their
original entries. JSON field dispatch bridges only the public CRT memcmp;
no game output, allocator or ECS lookup is supplied by the harness.
"""
import argparse
import hashlib
import json
import struct
from collections import deque
from pathlib import Path

import pefile
from unicorn import Uc, UcError, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import (
    UC_X86_REG_RAX, UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_R8,
    UC_X86_REG_RSP, UC_X86_REG_XMM0, UC_X86_REG_XMM1, UC_X86_REG_MXCSR,
    UC_X86_REG_RIP,
)

SHA = "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
BASE, DATA, STACK, STOP = 0x140000000, 0x20000000, 0x30000000, 0x40000000
ENTRIES = {
    "ui_default": 0x140B12F10, "horizontal": 0x140B0E070,
    "ui_deserialize": 0x14142EBA0,
    "window_new": 0x140949BF0, "window_set": 0x140949C00,
    "window_vec2": 0x140949C10, "window_ratio": 0x140949C20,
    "screen_space": 0x140AC8420, "clear": 0x140976890,
    "brightness": 0x140B06770, "brightness_default": 0x140B06780,
    "is_open": 0x14093FBC0, "is_closed": 0x14093FBE0,
    "cast_ui_shadows": 0x14093FC00, "should_show_world": 0x14093FC20,
    "can_limit_frame_rate": 0x14093FC60,
}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("binary", type=Path)
    ap.add_argument("output", type=Path)
    args = ap.parse_args()
    raw = args.binary.read_bytes()
    assert hashlib.sha256(raw).hexdigest() == SHA
    pe = pefile.PE(data=raw)
    machine = Uc(UC_ARCH_X86, UC_MODE_64)
    # Mapping the original image also retains every RIP-relative constant.
    for section in pe.sections:
        size = (max(section.Misc_VirtualSize, section.SizeOfRawData) + 4095) & ~4095
        if size:
            address = BASE + section.VirtualAddress
            machine.mem_map(address, size)
            machine.mem_write(address, section.get_data())
    machine.mem_map(DATA, 0x4000)
    machine.mem_map(STACK, 0x10000)
    machine.mem_map(STOP, 4096)
    machine.reg_write(UC_X86_REG_MXCSR, 0x1F80)
    trace = deque(maxlen=20)
    machine.mem_write(0x142923BF0, b"\xc3")

    def instruction(uc, address, size, context):
        trace.append(hex(address))
        if address == 0x142923BF0:
            length = uc.reg_read(UC_X86_REG_R8)
            left = bytes(uc.mem_read(uc.reg_read(UC_X86_REG_RCX), length))
            right = bytes(uc.mem_read(uc.reg_read(UC_X86_REG_RDX), length))
            uc.reg_write(UC_X86_REG_RAX, ((left > right) - (left < right)) & 0xFFFFFFFF)

    machine.hook_add(UC_HOOK_CODE, instruction)

    def call(name, rcx=DATA, rdx=0, r8=0):
        sp = STACK + 0x8008
        machine.mem_write(sp, struct.pack("<Q", STOP))
        for reg, value in [(UC_X86_REG_RSP, sp), (UC_X86_REG_RCX, rcx),
                           (UC_X86_REG_RDX, rdx), (UC_X86_REG_R8, r8)]:
            machine.reg_write(reg, value)
        try:
            machine.emu_start(ENTRIES[name], STOP, count=100000)
        except UcError:
            print(json.dumps({"entry": name, "pc": hex(machine.reg_read(UC_X86_REG_RIP)),
                              "trace": list(trace)}))
            raise
        assert machine.reg_read(UC_X86_REG_RSP) == sp + 8

    def read32(address):
        return struct.unpack("<I", machine.mem_read(address, 4))[0]

    def xmm(reg):
        return machine.reg_read(reg) & 0xFFFFFFFF

    out = {"binary_sha256": SHA, "entries": {k: hex(v) for k, v in ENTRIES.items()},
           "bridges": [{"entry": "ui_deserialize", "address": "0x142923bf0", "name": "VCRUNTIME140.memcmp"}],
           "windows": [], "menus": [], "clear": [], "brightness": [],
           "horizontal": []}
    machine.mem_write(DATA, b"\xa5" * 16)
    call("ui_default")
    out["ui_default"] = list(machine.mem_read(DATA, 16))
    fields = {"icon_size_mult": 1.5, "toolbar_alignment": "Right",
              "auto_hide_toolbar": True, "input_lag_reduction": False,
              "game_logo": False, "custom_cursor": True, "custom_cursor_size": 32.0}
    out["ui_deserialize"] = []
    for mask in range(128):
        source = json.dumps({key: value for i, (key, value) in enumerate(fields.items())
                             if mask & (1 << i)}, separators=(",", ":"))
        raw_json = source.encode()
        machine.mem_write(DATA + 0x1000, raw_json)
        # serde_json 1.0.85 StrRead: empty scratch Vec, input, offset, recursion.
        machine.mem_write(DATA + 0x100, struct.pack("<QQQQQQB", 0, 1, 0,
            DATA + 0x1000, len(raw_json), 0, 128))
        machine.mem_write(DATA, b"\xa5" * 16)
        call("ui_deserialize", DATA, DATA + 0x100)
        result = list(machine.mem_read(DATA, 13))
        assert result[12] != 2, (mask, "native error")
        out["ui_deserialize"].append({"json": source, "state": result})
    for tag in range(4):
        machine.mem_write(DATA, bytes([tag]))
        call("horizontal")
        out["horizontal"].append(bool(machine.reg_read(UC_X86_REG_RAX) & 255))

    sizes = [(0, 0), (1, 0), (0, 1), (1920, 1080), (1080, 1920),
             (16777215, 16777217), (16777217, 16777215),
             (0xFFFFFFFF, 1), (1, 0xFFFFFFFF), (0xFFFFFFFF, 0xFFFFFFFF)]
    for width, height in sizes:
        call("window_new", width | height << 32)
        constructed = [machine.reg_read(UC_X86_REG_RAX) & 0xFFFFFFFF,
                       machine.reg_read(UC_X86_REG_RDX) & 0xFFFFFFFF]
        call("window_set", rdx=width, r8=height)
        fields = [read32(DATA), read32(DATA + 4)]
        call("window_vec2")
        converted = [xmm(UC_X86_REG_XMM0), xmm(UC_X86_REG_XMM1)]
        call("window_ratio")
        ratio = xmm(UC_X86_REG_XMM0)
        call("screen_space", DATA + 0x100, DATA)
        screen = [read32(DATA + 0x100 + 4 * i) for i in range(4)]
        out["windows"].append({"input": [width, height], "constructed": constructed,
            "fields": fields, "vec2_bits": converted, "ratio_bits": ratio,
            "screen_space_bits": screen})

    for tag in range(12):
        for payload in [0, 1, 0xFFFFFFFF]:
            for flag in [False, True]:
                machine.mem_write(DATA, struct.pack("<IIB", tag, payload, flag))
                case = {"tag": tag, "payload": payload, "flag": flag}
                for name in ["is_open", "is_closed", "cast_ui_shadows",
                             "should_show_world", "can_limit_frame_rate"]:
                    call(name)
                    case[name] = bool(machine.reg_read(UC_X86_REG_RAX) & 255)
                out["menus"].append(case)

    for tick in [0, 17, 0xFFFFFFFF]:
        before = bytes((i * 17 + tick) & 255 for i in range(16))
        machine.mem_write(DATA + 0x100, before)
        machine.mem_write(DATA, bytes(32))
        machine.mem_write(DATA, struct.pack("<Q", DATA + 0x100))
        machine.mem_write(DATA + 0x10, struct.pack("<Q", DATA + 0x200))
        machine.mem_write(DATA + 0x1C, struct.pack("<I", tick))
        call("clear")
        out["clear"].append({"tick": tick, "tick_after": read32(DATA + 0x200),
                             "before": list(before),
                             "after": list(machine.mem_read(DATA + 0x100, 16))})
    for tag in range(3):
        call("brightness", tag)
        out["brightness"].append(xmm(UC_X86_REG_XMM0))
    call("brightness_default")
    out["brightness_default"] = xmm(UC_X86_REG_XMM0)
    args.output.write_text(json.dumps(out, indent=2) + "\n")
    print(json.dumps({"sha256": SHA, "windows": len(out["windows"]),
                      "menus": len(out["menus"]), "fixture": str(args.output)}))


if __name__ == "__main__":
    main()
