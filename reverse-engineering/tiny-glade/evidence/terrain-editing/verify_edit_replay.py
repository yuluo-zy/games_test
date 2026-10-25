"""Observe the original EditTerrainStroke replay branch in Unicorn.

Preallocated resolved EventWriter boundary; no ECS/Vec/serde implementation is
reconstructed. Does not execute the game or modify the original binary/DB.
"""
import hashlib
import json
import random
import struct
from pathlib import Path

import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import (
    UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_R8, UC_X86_REG_R9,
    UC_X86_REG_RSP, UC_X86_REG_MXCSR,
)

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[1]
GAME = ROOT.parents[2]
BINARY = GAME / "ljxsj_92385/Tiny Glade/tiny-glade.rne"
raw = BINARY.read_bytes()
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
cpu = Uc(UC_ARCH_X86, UC_MODE_64)
end = max(s.VirtualAddress + max(s.Misc_VirtualSize, s.SizeOfRawData) for s in pe.sections)
cpu.mem_map(base, (end + 4095) & ~4095)
for section in pe.sections:
    if section.SizeOfRawData:
        cpu.mem_write(base + section.VirtualAddress, raw[section.PointerToRawData:section.PointerToRawData + section.SizeOfRawData])
DATA, STACK, STOP = 0x20000000, 0x30000000, 0x40000000
cpu.mem_map(DATA, 0x10000)
cpu.mem_map(STACK, 0x10000)
cpu.mem_map(STOP, 0x1000)
EDIT, WRITER, RESOURCE, EVENTS, CHANGE = [DATA + x for x in (0x100, 0x1000, 0x2000, 0x3000, 0x4000)]
active = [0]

def code_hook(uc, address, size, user):
    if address == STOP:
        uc.emu_stop()
    elif not active[0] <= address < active[0] + 0x500:
        raise RuntimeError(f"Unexpected external execution at {address:#x}")

cpu.hook_add(UC_HOOK_CODE, code_hook)

def f32(value):
    return struct.unpack("<f", struct.pack("<f", value))[0]

def word(value):
    return struct.unpack("<I", struct.pack("<f", value))[0]

def mix(previous, new, progress, reverse):
    inverse = f32(1.0 - progress)
    if reverse:
        return f32(f32(previous * progress) + f32(new * inverse))
    return f32(f32(new * progress) + f32(previous * inverse))

rng = random.Random(20261011)
cases = []
edges = [-0.0, 0.0, -2.5, 0.4, 1.0, 2.5, 20.0, 24.0]
for i in range(256):
    values = [f32(rng.uniform(-30, 30)) for _ in range(4)]
    if i < 64:
        values = [f32(edges[(i + j) % len(edges)]) for j in range(4)]
    if i % 7 == 0:
        values[1] = values[0]
        values[3] = values[2]
    progress = f32([0.0, 0.25, 0.5, 0.75, 1.0, -0.25, 1.25][i % 7])
    cases.append((i, values, progress))
failures = []
completed = 0
for reverse, address in [(False, 0x1408592d0), (True, 0x140858d30)]:
    active[0] = address
    for index, values, progress in cases:
        previous_height, new_height, previous_falloff, new_falloff = values
        cpu.mem_write(DATA, bytes(0x10000))
        cpu.mem_write(STACK, bytes(0x10000))
        # Native enum niche: Edit = 0x8000000000000001.
        cpu.mem_write(EDIT, struct.pack("<QIffff", 0x8000000000000001, index, *values))
        cpu.mem_write(WRITER, struct.pack("<Q", RESOURCE))
        cpu.mem_write(WRITER + 0x10, struct.pack("<Q", CHANGE))
        cpu.mem_write(WRITER + 0x1c, struct.pack("<I", 123))
        cpu.mem_write(RESOURCE + 0x20, struct.pack("<QQQ", 64, EVENTS, 0))
        cpu.mem_write(RESOURCE + 0x40, struct.pack("<Q", 17))
        rsp = STACK + 0x8000
        cpu.mem_write(rsp, struct.pack("<Q", STOP))
        cpu.mem_write(rsp + 0x28, struct.pack("<Q", WRITER))
        cpu.mem_write(rsp + 0x30, struct.pack("<f", progress))
        for register, value in [(UC_X86_REG_RCX, EDIT), (UC_X86_REG_RDX, DATA + 0x5000),
                                (UC_X86_REG_R8, WRITER), (UC_X86_REG_R9, DATA + 0x6000),
                                (UC_X86_REG_RSP, rsp), (UC_X86_REG_MXCSR, 0x1f80)]:
            cpu.reg_write(register, value)
        cpu.emu_start(address, STOP + 1, count=10000)
        actual = bytes(cpu.mem_read(EVENTS, 40))
        expected = struct.pack("<IfIfQQ", int(new_falloff != previous_falloff),
                               mix(previous_falloff, new_falloff, progress, reverse),
                               int(new_height != previous_height),
                               mix(previous_height, new_height, progress, reverse),
                               0, index << 32) + struct.pack("<Q", 17)
        state = struct.unpack("<Q", cpu.mem_read(RESOURCE + 0x30, 8))[0]
        counter = struct.unpack("<Q", cpu.mem_read(RESOURCE + 0x40, 8))[0]
        tick = struct.unpack("<I", cpu.mem_read(CHANGE, 4))[0]
        completed += 1
        if actual != expected or (state, counter, tick) != (1, 18, 123):
            failures.append({"reverse": reverse, "case": index,
                             "values_bits": [hex(word(v)) for v in values],
                             "progress_bits": hex(word(progress)),
                             "expected": expected.hex(), "actual": actual.hex(),
                             "observed_queue_state": [state, counter, tick]})

report = {
    "ok": not failures, "case_count": completed, "failure_count": len(failures),
    "failures": failures,
    "binary_sha256": hashlib.sha256(raw).hexdigest(), "baseline": "B",
    "target_execution": False, "original_instructions_emulated": True,
    "functions": ["0x1408592d0", "0x140858d30"],
    "scope": "Only EditTerrainStroke branch, original function entry through return; finite f32 interpolation/changed flags/event payload and preallocated writer len/id/change tick. No engine/container calls.",
    "excluded": ["New/Move branches", "NaN/Inf", "ECS resource resolution", "queue growth", "GPU", "system schedule", "full undo/redo transaction", "A/C equivalence"],
    "candidate_model": "f32 operation-ordered interpolation checked against original instructions, not a delivered Rust implementation",
}
(OUT / "B-edit-replay-verification.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
print(json.dumps({"ok": report["ok"], "cases": completed, "failures": len(failures)}))
if failures:
    raise SystemExit(1)
