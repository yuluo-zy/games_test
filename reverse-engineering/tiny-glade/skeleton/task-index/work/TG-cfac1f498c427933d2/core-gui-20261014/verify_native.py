"""Execute B GUI business instructions; bridge only Rust allocation/deallocation.

time_update runs its complete original entry with normal (non-replay) inputs.
The LUT default probe stops before App::insert_resource, not at function return.
Panic probes stop on the original panic entry; unwinding/OS behavior is excluded.
"""
import argparse
import hashlib
import json
import random
import struct
from pathlib import Path

import capstone
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import (
    UC_X86_REG_RAX, UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_R8,
    UC_X86_REG_R9, UC_X86_REG_RSP, UC_X86_REG_RIP, UC_X86_REG_MXCSR,
)

SHA = "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
BASE, DATA, HEAP, STACK, STOP = 0x140000000, 0x20000000, 0x21000000, 0x30000000, 0x40000000
ENTRIES = {
    "positions": 0x14093FC70, "map": 0x141304FB0,
    "default_prefix": 0x1408B04C0, "time_update": 0x140922C30,
    "proxy_label": 0x14093FFD0, "raycast_label": 0x140940170,
    "set": 0x14093FAD0, "get": 0x14093FBB0, "clone": 0x140FE5150,
    "is_open": 0x14093FBC0, "is_closed": 0x14093FBE0,
    "cast_ui_shadows": 0x14093FC00, "should_show_world": 0x14093FC20,
    "can_limit_frame_rate": 0x14093FC60,
}
PANICS = {0x1428D99E0: "divide_by_zero", 0x1428D9A20: "remainder_by_zero"}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("binary", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    raw = args.binary.read_bytes()
    assert hashlib.sha256(raw).hexdigest() == SHA
    pe = pefile.PE(data=raw, fast_load=True)
    uc = Uc(UC_ARCH_X86, UC_MODE_64)
    for section in pe.sections:
        size = (max(section.Misc_VirtualSize, section.SizeOfRawData) + 4095) & ~4095
        if size:
            uc.mem_map(BASE + section.VirtualAddress, size)
            uc.mem_write(BASE + section.VirtualAddress, section.get_data())
    for address, size in [(DATA, 0x10000), (HEAP, 0x100000), (STACK, 0x10000), (STOP, 4096)]:
        uc.mem_map(address, size)
    uc.reg_write(UC_X86_REG_MXCSR, 0x1F80)
    next_alloc = HEAP
    freed, allocations = [], []
    panic = None
    # Return stubs affect public allocation glue only, never GUI instructions.
    for address in [0x140613C10, 0x140613C20]:
        uc.mem_write(address, b"\xc3")

    def hook(machine, address, size, context):
        nonlocal next_alloc, panic
        if address == 0x140613C10:
            length = machine.reg_read(UC_X86_REG_RCX)
            align = machine.reg_read(UC_X86_REG_RDX)
            ptr = (next_alloc + align - 1) & -align
            next_alloc = ptr + length
            assert next_alloc <= HEAP + 0x100000
            machine.mem_write(ptr, b"\xa5" * length)
            machine.reg_write(UC_X86_REG_RAX, ptr)
            allocations.append([ptr, length, align])
        elif address == 0x140613C20:
            freed.append([machine.reg_read(r) for r in
                          [UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_R8]])
        elif address in PANICS:
            panic = PANICS[address]
            machine.emu_stop()

    uc.hook_add(UC_HOOK_CODE, hook)

    def put(address, fmt, *values):
        uc.mem_write(address, struct.pack(fmt, *values))

    def unpack(address, fmt):
        return struct.unpack(fmt, uc.mem_read(address, struct.calcsize(fmt)))

    def call(name, rcx=DATA, rdx=0, r8=0, r9=0, fifth=0, end=STOP):
        nonlocal panic, next_alloc
        panic = None
        next_alloc = HEAP
        allocations.clear()
        freed.clear()
        sp = STACK + 0x8008
        uc.mem_write(STACK, b"\xa5" * 0x10000)
        put(sp, "<Q", STOP)
        put(sp + 0x28, "<Q", fifth)
        for reg, value in [(UC_X86_REG_RSP, sp), (UC_X86_REG_RCX, rcx),
                           (UC_X86_REG_RDX, rdx), (UC_X86_REG_R8, r8), (UC_X86_REG_R9, r9)]:
            uc.reg_write(reg, value)
        uc.emu_start(ENTRIES[name], end, count=1000000)
        assert panic or uc.reg_read(UC_X86_REG_RIP) == end
        if not panic and end == STOP:
            assert uc.reg_read(UC_X86_REG_RSP) == sp + 8

    out = {"binary_sha256": SHA, "entries": {k: hex(v) for k, v in ENTRIES.items()},
           "bridges": ["0x140613c10 Rust allocator", "0x140613c20 Rust deallocator"],
           "menus": [], "metrics": [], "labels": [], "setters": []}
    for tag in range(11):
        for payload in [0, 1]:
            for flag in (range(11) if tag == 7 else range(2)):
                uc.mem_write(DATA, bytes(96))
                put(DATA, "<IIB", tag, payload, flag)
                case = {"tag": tag, "payload": payload, "flag": flag}
                for name in ["is_open", "is_closed", "cast_ui_shadows", "should_show_world", "can_limit_frame_rate"]:
                    call(name)
                    case[name] = bool(uc.reg_read(UC_X86_REG_RAX) & 255)
                # Res<Time>, Res<ShaderRefactoringState>, Res<EscMenuState>,
                # Res<InputRecordReplayState>, ResMut<GameTime>.
                uc.mem_write(DATA + 0x100, bytes(0x800))
                for offset, target in [(0x100, 0x500), (0x120, 0x600), (0x140, 0),
                                       (0x160, 0x700), (0x180, 0x800)]:
                    put(DATA + offset, "<Q", DATA + target)
                put(DATA + 0x510, "<QI", 0, 250000000)
                put(DATA + 0x190, "<Q", DATA + 0x900)
                put(DATA + 0x19C, "<I", 42)
                put(DATA + 0x82C, "<I", 2)
                call("time_update", DATA + 0x100, DATA + 0x120, DATA + 0x140,
                     DATA + 0x160, fifth=DATA + 0x180)
                case["allow_time_passage"] = bool(unpack(DATA + 0x830, "<B")[0])
                case["time_delta_bits"] = unpack(DATA + 0x828, "<I")[0]
                assert case["time_delta_bits"] in [0, 0x3E800000]
                out["menus"].append(case)

    def metric_case(origin, step, count, stride, column_major):
        uc.mem_write(DATA, bytes(0x400))
        put(DATA, "<II", 1, 3)
        put(DATA + 8, "<II", *origin)
        put(DATA + 0x10, "<ff", 0.25, 0.5)
        put(DATA + 0x18, "<II", *step)
        put(DATA + 0x20, "<ff", -7, 19)
        put(DATA + 0x28, "<IIB", count, stride, column_major)
        call("positions", DATA + 0x100, DATA)
        result = {"origin_bits": origin, "step_bits": step, "count": count,
                  "stride": stride, "column_major": bool(column_major), "panic": panic}
        if not panic:
            begin, current, capacity, end = unpack(DATA + 0x100, "<QQQQ")
            assert begin == current and end - begin == count * 8 and capacity == count
            result["positions_bits"] = [list(unpack(begin + i * 8, "<II")) for i in range(count)]
        out["metrics"].append(result)

    for order in [0, 1]:
        for stride in [0, 1, 2, 3, 7, 0xFFFFFFFF]:
            for count in [0, 1, 2, 8, 19]:
                metric_case([0xBE800000, 0x3F000000], [0x3DCCCCCD, 0x3E4CCCCD], count, stride, order)
    rng = random.Random(0x93FC70)
    for _ in range(80):
        bits = [rng.getrandbits(32) for _ in range(4)]
        # Finite, bounded coordinates, including signed zero and subnormals.
        bits = [value & 0xBEFFFFFF for value in bits]
        metric_case(bits[:2], bits[2:], rng.randrange(1, 24), rng.randrange(1, 9), rng.randrange(2))
    for bits in [[0, 0x80000000, 0, 0x80000000],
                 [1, 0x80000001, 0x007FFFFF, 0x807FFFFF],
                 [0xBF800000, 0xBF800000, 0x3EAAAAAB, 0x3F000001],
                 [0xFF800000, 0x7F800000, 0x7F800000, 0xFF800000],
                 [0x7FC12345, 0xFFC23456, 0x3F800000, 0xBF800000],
                 [0x3F800000, 0xBF800000, 0x7FC12345, 0xFFC23456],
                 [0x7FC12345, 0xFFC23456, 0x7FC34567, 0xFFC45678],
                 [0x7F812345, 0xFF823456, 0x7FC34567, 0xFFC45678],
                 [0x7F7FFFFF, 0xFF7FFFFF, 0x7F7FFFFF, 0xFF7FFFFF]]:
        metric_case(bits[:2], bits[2:], 16, 1, 0)

    call("default_prefix", end=0x1408B0533)
    ptr = uc.reg_read(UC_X86_REG_RDX)
    out["default_metrics_bytes"] = list(uc.mem_read(ptr + 0x14, 0x34))
    out["default_prefix_end"] = "0x1408b0533 before App::insert_resource; allocation bridge only"
    for tag in range(5):
        put(DATA, "<B", tag)
        call("proxy_label", DATA + 0x100, DATA)
        capacity, ptr, length = unpack(DATA + 0x100, "<QQQ")
        assert capacity == length
        owned = bytes(uc.mem_read(ptr, length)).decode()
        call("raycast_label")
        ptr, length = uc.reg_read(UC_X86_REG_RAX), uc.reg_read(UC_X86_REG_RDX)
        borrowed = bytes(uc.mem_read(ptr, length)).decode()
        out["labels"].append({"tag": tag, "proxy": owned, "raycast": borrowed})

    # Establish the ownership contract without pretending to know load payload roles.
    for old_tag in range(12):
        for strong in [1, 2]:
            uc.mem_write(DATA, b"\xa5" * 0x400)
            put(DATA, "<Q", old_tag)
            put(DATA + 0x30, "<Q", DATA + 0x200)
            put(DATA + 0x200, "<QQ", strong, 1)
            new_state = struct.pack("<Q", 7) + bytes([1]) + bytes(range(87))
            uc.mem_write(DATA + 0x100, new_state)
            call("set", DATA, DATA + 0x100)
            after = bytes(uc.mem_read(DATA, 96))
            assert after == new_state
            out["setters"].append({"old_tag": old_tag, "strong_before": strong,
                "strong_after": unpack(DATA + 0x200, "<Q")[0],
                "deallocated": bool(freed), "deallocation_size_align": [v[1:] for v in freed],
                "copied_bytes": len(after)})
    cs = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    out["instruction_ranges"] = {}
    for start, length in [(0x14093FAD0, 144), (0x14093FBB0, 192), (0x141305040, 0x6E),
                          (0x140922CBA, 0x3D), (0x140940170, 29), (0x140FE5150, 450)]:
        out["instruction_ranges"][hex(start)] = [
            f"{i.address:#x} {i.bytes.hex()} {i.mnemonic} {i.op_str}"
            for i in cs.disasm(pe.get_data(start - BASE, length), start)]
    args.output.write_text(json.dumps(out, indent=2) + "\n")
    print(json.dumps({"output": str(args.output), "menus": len(out["menus"]),
                      "metrics": len(out["metrics"]), "labels": len(out["labels"]),
                      "setters_observation_only": len(out["setters"])}))


if __name__ == "__main__":
    main()
