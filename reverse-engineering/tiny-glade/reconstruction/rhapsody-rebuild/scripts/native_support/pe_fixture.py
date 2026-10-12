from pathlib import Path
import hashlib
import itertools
import json
import struct
import subprocess
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

ROOT = Path('D:/game/reverse-engineering/tiny-glade')
OUT = Path(__file__).resolve().parent
SOURCE = ROOT / "reconstruction/render/work-packages/barrier-record-r5"
TARGET = ROOT / "workspace/input/tiny-glade.exe"
EXPECTED_SHA = "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
RAW = TARGET.read_bytes()
assert hashlib.sha256(RAW).hexdigest() == EXPECTED_SHA
PE = struct.unpack_from("<I", RAW, 60)[0]
OPTIONAL = PE + 24
BASE = struct.unpack_from("<Q", RAW, OPTIONAL + 24)[0]
SIZE = struct.unpack_from("<I", RAW, OPTIONAL + 56)[0]
CPU = Uc(UC_ARCH_X86, UC_MODE_64)
CPU.mem_map(BASE, (SIZE + 4095) & ~4095)
for section in range(struct.unpack_from("<H", RAW, PE + 6)[0]):
    header = OPTIONAL + struct.unpack_from("<H", RAW, PE + 20)[0] + section * 40
    virtual_size, virtual_address, raw_size, raw_offset = struct.unpack_from("<IIII", RAW, header + 8)
    CPU.mem_write(BASE + virtual_address, RAW[raw_offset:raw_offset + raw_size])
STACK = 0x20000000
STOP = 0x30000000
MEMORY = 0x40000000
CPU.mem_map(STACK, 0x20000)
CPU.mem_map(STOP, 4096)
CPU.mem_map(MEMORY, 0x20000)
COMPILED = MEMORY
REGISTRY = COMPILED + 0xA8
REUSE = MEMORY + 0x1000
INITIAL = MEMORY + 0x2000
BACKING = MEMORY + 0x3000
ACCESSES = MEMORY + 0x4000
RANGES = MEMORY + 0x5000
LIFETIMES = MEMORY + 0x6000
PENDING = MEMORY + 0x7000
DEVICE = MEMORY + 0x8000
INNER = MEMORY + 0x9000
RECORDS = MEMORY + 0xB000
BATCH = MEMORY + 0xD000
COMMAND_VEC = MEMORY + 0xD100
COMMANDS = MEMORY + 0xD200
EXPORTS = MEMORY + 0xE000
PANICS = {
    0x1428D9518: "bounds", 0x1428D9850: "range-start", 0x1428D9840: "range-end",
    0x1428D9310: "generation", 0x1428D9430: "kind", 0x1428CA816: "device",
    0x1428CA902: "managed-kind", 0x1428A21B8: "initial-nonzero",
    0x1428D9390: "command-empty",
}
STATE = {}

def write(address, fmt, *values):
    CPU.mem_write(address, struct.pack(fmt, *values))

def read(address, fmt):
    return struct.unpack(fmt, bytes(CPU.mem_read(address, struct.calcsize(fmt))))

def ret(value=None):
    pointer = CPU.reg_read(UC_X86_REG_RSP)
    address = read(pointer, "<Q")[0]
    CPU.reg_write(UC_X86_REG_RSP, pointer + 8)
    CPU.reg_write(UC_X86_REG_RIP, address)
    if value is not None:
        CPU.reg_write(UC_X86_REG_RAX, value)

def hook(cpu, address, size, user):
    STATE["visited"].add(address)
    if address == STATE["cut"]:
        STATE["cut_reached"] = True
        cpu.emu_stop()
    elif address in PANICS:
        STATE["panic"] = "command-empty" if address == 0x1428D9310 and read(CPU.reg_read(UC_X86_REG_RSP), "<Q")[0] == 0x1402C2F02 else PANICS[address]
        cpu.emu_stop()
    elif address == 0x140C10E10:
        ret(int(CPU.reg_read(UC_X86_REG_RCX) & 255 in STATE["write_codes"]))
    elif address in (0x140BD65E0, 0x140BD6610):
        kind = int(address == 0x140BD6610)
        transition = [kind, CPU.reg_read(UC_X86_REG_R8) & 255, CPU.reg_read(UC_X86_REG_R9) & 255]
        STATE["constructors"].append({"transition": transition, "handle": CPU.reg_read(UC_X86_REG_RDX), "aspect": read(CPU.reg_read(UC_X86_REG_RSP) + 0x28, "<I")[0] if kind == 0 else None})
        write(CPU.reg_read(UC_X86_REG_RCX), "<III", *transition)
        ret()
    elif address in (0x140BD66A0, 0x140BD6810):
        transition = list(read(CPU.reg_read(UC_X86_REG_RDX), "<III"))
        STATE["transitions"].append(transition)
        STATE["scope"].append(transition)
        ret()
    elif address == 0x140BD6630:
        STATE["formats"].append(CPU.reg_read(UC_X86_REG_RCX) & 0xFFFFFFFF)
        ret(1)
    elif address == 0x140BECFD0:
        STATE["tracked"] += 1
        ret()
    elif address == 0x140BD6980:
        ret(int(not STATE["scope"]))
    elif address == 0x140BEE030:
        STATE["emitted"] += 1
        ret()
    elif address == 0x140BD6950:
        STATE["resets"] += 1
        STATE["scope"] = []
        ret()
    elif address in (0x140BF09B0, 0x140BF0B40):
        ret()

CPU.hook_add(UC_HOOK_CODE, hook)

def setup(case, count=1):
    mode, kind, initial, reused, prev, prev_group, prev_write, current, group, current_write, flag, validity = case
    CPU.mem_write(MEMORY, bytes(0x20000))
    CPU.mem_write(STACK, bytes(0x20000))
    STATE.clear()
    STATE.update(visited=set(), transitions=[], constructors=[], scope=[], panic="ok", tracked=0, emitted=0, resets=0, formats=[], cut_reached=False, write_codes=set())
    if prev_write and prev != 256:
        STATE["write_codes"].add(prev)
    if current_write:
        STATE["write_codes"].add(current)
    write(REGISTRY, "<QQQ", count, REUSE, count)
    write(REGISTRY + 0x18, "<QQQ", count, INITIAL, count)
    write(REGISTRY + 0x30, "<QQQ", count, BACKING, count)
    write(REGISTRY + 0x48, "<QQQ", 2 * count, ACCESSES, 2 * count)
    write(REGISTRY + 0x60, "<QQQ", 1, RANGES, 1)
    write(RANGES, "<II", count, 2 * count)
    write(REGISTRY + 0x78, "<QQQ", count, LIFETIMES, count)
    write(COMPILED + 0x200, "<QQQ", count, PENDING, count)
    write(DEVICE + 0x648, "<Q", INNER)
    write(INNER + 0xA50, "<H", 9)
    write(COMMAND_VEC, "<QQQ", 4, COMMANDS, flag if mode == 1 else 1)
    write(BATCH, "<QQB", 0, 1, flag)
    write(COMPILED + 0x20, "<QQ", EXPORTS, count)
    for resource in range(count):
        write(REUSE + resource * 4, "<HB", resource, reused)
        write(INITIAL + resource, "<B", initial)
        payload = BACKING + resource * 64
        record = RECORDS + resource * 256
        write(payload, "<Q", kind * 2)
        write(payload + 16, "<Q", record)
        write(payload + 24, "<IHBB", 8 if validity == 3 else 7, 10 if validity == 1 else 9, (kind ^ 1) if validity == 2 else kind, 0)
        write(record + 0xC, "<I", 7)
        write(record + 0x18, "<I", 37)
        write(record + 0x30, "<Q", 0xB000 + resource)
        write(record + 0x40, "<Q", 0xA000 + resource)
        write(ACCESSES + resource * 20, "<IIIIB", resource, 0xFFFFFFFF, 0xFFFFFFFF, prev_group, prev if prev != 256 else 0)
        write(ACCESSES + (count + resource) * 20, "<IIIIB", resource, resource if prev != 256 else 0xFFFFFFFF, 0xFFFFFFFF, group, current)
        write(LIFETIMES + resource * 24 + 16, "<II", count + resource if prev != 256 else 0xFFFFFFFF, resource if prev != 256 else 0xFFFFFFFF)
        write(PENDING + resource * 4, "<I", resource)
        write(EXPORTS + resource * 48, "<II", kind, resource)
        write(EXPORTS + resource * 48 + 0x10, "<I", 91)
        write(EXPORTS + resource * 48 + 0x18, "<I", resource)
        write(EXPORTS + resource * 48 + 0x28, "<B", current)
    stack_pointer = STACK + 0x10000
    write(stack_pointer, "<Q", STOP)
    CPU.reg_write(UC_X86_REG_RSP, stack_pointer)
    if mode == 0:
        CPU.reg_write(UC_X86_REG_RCX, REGISTRY)
        CPU.reg_write(UC_X86_REG_RDX, 0)
        CPU.reg_write(UC_X86_REG_R8, COMPILED + 0x1B0)
        CPU.reg_write(UC_X86_REG_R9, DEVICE)
        return 0x1402C3D00, STOP
    if mode == 1:
        CPU.reg_write(UC_X86_REG_RCX, COMPILED)
        CPU.reg_write(UC_X86_REG_RDX, BATCH)
        CPU.reg_write(UC_X86_REG_R8, DEVICE)
        CPU.reg_write(UC_X86_REG_R9, 0)
        write(stack_pointer + 0x28, "<QQQQ", 0, COMMAND_VEC, 0, 0)
        return 0x1402C1D40, 0x1402C2007
    frame = stack_pointer + 0x80
    CPU.reg_write(UC_X86_REG_RBP, frame)
    CPU.reg_write(UC_X86_REG_RBX, COMPILED)
    CPU.reg_write(UC_X86_REG_RDI, 1)
    CPU.reg_write(UC_X86_REG_R12, COMMANDS)
    write(frame + 8, "<Q", BATCH)
    write(frame + 0x130, "<Q", REGISTRY)
    write(frame + 0x140, "<Q", COMPILED + 0x1B0)
    write(frame + 0x150, "<Q", DEVICE)
    return 0x1402C26A1, 0x1402C2985

def execute(case, count=1):
    entry, cut = setup(case, count)
    STATE["cut"] = cut
    try:
        CPU.emu_start(entry, STOP, count=30000)
    except Exception as error:
        raise RuntimeError(f"case {case}, RIP {CPU.reg_read(UC_X86_REG_RIP):x}: {error}") from error
    if STATE["panic"] == "ok" and cut != STOP:
        assert STATE["cut_reached"]
    pending_length = read(COMPILED + 0x210, "<Q")[0] if case[0] == 1 else 0
    formatted = ",".join(":".join(str(value) for value in transition) for transition in STATE["transitions"])
    result = f'{STATE["panic"]}|{formatted}|{pending_length}|{STATE["tracked"]}|{STATE["emitted"]}|{STATE["resets"]}'
    return result, {"case": case, "result": result, "constructors": STATE["constructors"].copy(), "formats": STATE["formats"].copy(), "visited": sorted(hex(address) for address in STATE["visited"])}
