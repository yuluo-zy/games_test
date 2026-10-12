"""Original resolved terrain delta-edit fragments, not whole ECS system.

Observed game decisions only. Hash lookup/event allocation are outside scope;
preallocated resolved pointers are inputs, not a reimplementation of libraries.
"""
import hashlib
import json
import random
import struct
from pathlib import Path
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[1]
raw = (ROOT.parents[2] / "ljxsj_92385/Tiny Glade/tiny-glade.rne").read_bytes()
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
cpu = Uc(UC_ARCH_X86, UC_MODE_64)
end = max(s.VirtualAddress + max(s.Misc_VirtualSize, s.SizeOfRawData) for s in pe.sections)
cpu.mem_map(base, (end + 4095) & ~4095)
for s in pe.sections:
    if s.SizeOfRawData:
        cpu.mem_write(base + s.VirtualAddress, raw[s.PointerToRawData:s.PointerToRawData + s.SizeOfRawData])
DATA, STACK = 0x20000000, 0x30000000
cpu.mem_map(DATA, 0x10000)
cpu.mem_map(STACK, 0x10000)
CMD, BUCKET_END, AUDIO, EVENTS, TICK = [DATA + n for n in (0x100, 0x800, 0x1000, 0x2000, 0x3000)]
RBP = STACK + 0x8000
STOP = [0]


def hook(uc, address, size, user):
    if address == STOP[0]:
        uc.emu_stop()
    elif not 0x14094eb76 <= address < 0x14094ef1b:
        raise RuntimeError(f"Unexpected external path {address:#x}")

cpu.hook_add(UC_HOOK_CODE, hook)


def f32(value):
    return struct.unpack("<f", struct.pack("<f", value))[0]


def bits(value):
    return struct.unpack("<I", struct.pack("<f", value))[0]

rng = random.Random(20261012)
failures = []
completed = 0
edges = [(-0.0, 0.0), (0.0, 0.1), (0.0, 0.0), (0.0, -1.0),
         (2.5, -2.0), (1.0, -0.01), (20.0, 1.0), (1.0, -1.0),
         (0.4, 0.6), (0.4, -0.4), (1.0, 0.0), (-2.5, 1.0)]
for kind, start, stop in [("falloff", 0x14094eb76, 0x14094ec2d),
                          ("height", 0x14094ecf9, 0x14094ef1b)]:
    STOP[0] = stop
    for case in range(256):
        old, delta = edges[case % len(edges)] if case < 96 else (rng.uniform(-4, 24), rng.uniform(-5, 5))
        old, delta = f32(old), f32(delta)
        cpu.mem_write(DATA, bytes(0x10000))
        cpu.mem_write(STACK, bytes(0x10000))
        cpu.mem_write(BUCKET_END - 0x10, struct.pack("<f", old if kind == "falloff" else 0.4))
        cpu.mem_write(BUCKET_END - 8, struct.pack("<f", old if kind == "height" else 2.5))
        cpu.mem_write(CMD + (4 if kind == "falloff" else 12), struct.pack("<f", delta))
        cpu.mem_write(CMD + 0x10, struct.pack("<ff", 3.0, 4.0))
        cpu.mem_write(CMD + 0x18, struct.pack("<f", 5.0))
        cpu.mem_write(AUDIO + 0x20, struct.pack("<QQQ", 64, EVENTS, 0))
        cpu.mem_write(AUDIO + 0x40, struct.pack("<Q", 17))
        cpu.mem_write(RBP + 0xd0, struct.pack("<Q", TICK))
        cpu.mem_write(RBP + 0xf0, struct.pack("<I", 123))
        for register, value in [(UC_X86_REG_RSI, CMD), (UC_X86_REG_R15, BUCKET_END),
                                (UC_X86_REG_RDI, AUDIO), (UC_X86_REG_RBP, RBP),
                                (UC_X86_REG_RSP, STACK + 0x6000), (UC_X86_REG_MXCSR, 0x1f80),
                                (UC_X86_REG_XMM7, 0), (UC_X86_REG_XMM8, bits(1.0)),
                                (UC_X86_REG_XMM9, int.from_bytes(struct.pack("<IIII", *([0x7fffffff] * 4)), "little")),
                                (UC_X86_REG_XMM10, bits(20.0))]:
            cpu.reg_write(register, value)
        cpu.emu_start(start, stop + 1, count=2000)
        if kind == "falloff":
            expected_value = f32(min(1.0, max(0.0, f32(old + delta))))
            tags = [1] if old != expected_value else []
            expected_difference = f32(abs(f32(old - expected_value)))
        else:
            before = 1.0 if delta > 0.0 and old == 0.0 else old
            expected_value = f32(min(20.0, max(0.0, f32(before + delta))))
            if expected_value < 1.0:
                expected_value = 0.0
            tags = [2] if before != expected_value else []
            if before > 0.0 and expected_value == 0.0:
                tags.append(4)
            expected_difference = f32(abs(f32(before - expected_value)))
        actual_bits = struct.unpack("<I", cpu.mem_read(BUCKET_END - (16 if kind == "falloff" else 8), 4))[0]
        count = struct.unpack("<Q", cpu.mem_read(AUDIO + 0x30, 8))[0]
        actual_tags = [struct.unpack("<I", cpu.mem_read(EVENTS + i * 40 + 8, 4))[0] for i in range(count)]
        diff_offset = 28 if kind == "falloff" else 32
        actual_difference = struct.unpack("<I", cpu.mem_read(EVENTS + diff_offset, 4))[0] if tags else None
        completed += 1
        if actual_bits != bits(expected_value) or actual_tags != tags or (tags and actual_difference != bits(expected_difference)):
            failures.append({"kind": kind, "case": case, "old_bits": hex(bits(old)), "delta_bits": hex(bits(delta)),
                             "expected_value_bits": hex(bits(expected_value)), "actual_value_bits": hex(actual_bits),
                             "expected_tags": tags, "actual_tags": actual_tags,
                             "expected_difference_bits": hex(bits(expected_difference)),
                             "actual_difference_bits": None if actual_difference is None else hex(actual_difference)})
report = {"ok": not failures, "case_count": completed, "failure_count": len(failures), "failures": failures,
          "baseline": "B", "binary_sha256": hashlib.sha256(raw).hexdigest(),
          "target_execution": False, "original_instructions_emulated": True,
          "scope": "Finite f32 Delta branches only, resolved bucket/event buffer pointers; native final state, audio tags and actual difference magnitude",
          "fragments": ["0x14094eb76..0x14094ec2d", "0x14094ecf9..0x14094ef1b"],
          "excluded": ["Keep/Absolute", "NaN/Inf", "hash lookup", "event growth", "UI", "history recording", "recalculate/readback notifications", "GPU", "A/C", "full system acceptance"]}
(OUT / "B-delta-edit-verification.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
print(json.dumps({"ok": report["ok"], "cases": completed, "failures": len(failures)}))
if failures:
    raise SystemExit(1)
