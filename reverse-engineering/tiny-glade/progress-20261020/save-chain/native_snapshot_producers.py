"""Execute retained B instruction ranges; heap release is a recorded dependency stub.

This is producer-range evidence, not an execution of the complete game or its ECS.
"""
from pathlib import Path
import hashlib
import json
import random
import struct
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

ROOT = Path(__file__).resolve().parents[4]
OUT = Path(__file__).resolve().parent
EXE = ROOT / "reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe"
raw = EXE.read_bytes()
identity = hashlib.sha256(raw).hexdigest()
assert identity == "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
pe = pefile.PE(data=raw, fast_load=True)
BASE = pe.OPTIONAL_HEADER.ImageBase
FN = 0x1417A9C50
SIZE = 4143
function = pe.get_data(FN - BASE, SIZE)
RBP, STACK, LOADED, INPUT, EVENTS, BUFFER, TICK = (
    0x701000, 0x702000, 0x710000, 0x720000, 0x730000, 0x740000, 0x750000
)

def q(uc, address, value):
    uc.mem_write(address, struct.pack("<Q", value))

def d(uc, address, value):
    uc.mem_write(address, struct.pack("<I", value))

def machine():
    uc = Uc(UC_ARCH_X86, UC_MODE_64)
    start = FN & ~0xFFF
    uc.mem_map(start, 0x3000)
    uc.mem_write(FN, function)
    uc.mem_map(0x700000, 0x60000)
    uc.mem_map(0x140613000, 0x1000)
    uc.reg_write(UC_X86_REG_RBP, RBP)
    uc.reg_write(UC_X86_REG_RSP, STACK)
    q(uc, RBP + 0x208, LOADED)
    released = []
    def dependency(uc, address, size, _):
        if address == 0x140613C20:
            released.append((uc.reg_read(UC_X86_REG_RCX), uc.reg_read(UC_X86_REG_RDX)))
            rsp = uc.reg_read(UC_X86_REG_RSP)
            ret = struct.unpack("<Q", uc.mem_read(rsp, 8))[0]
            uc.reg_write(UC_X86_REG_RSP, rsp + 8)
            uc.reg_write(UC_X86_REG_RIP, ret)
    uc.hook_add(UC_HOOK_CODE, dependency)
    return uc, released

def event(uc, owner, buffer, count):
    q(uc, owner + 0x20, max(4, count + 1))
    q(uc, owner + 0x28, buffer)
    q(uc, owner + 0x30, 0)
    q(uc, owner + 0x40, 37)

def execute(uc, start, end):
    uc.emu_start(start, end, count=100000)
    assert uc.reg_read(UC_X86_REG_RIP) == end

rng = random.Random(0xB5A0E)
roof_cases = []
for count in [0, 1, 3]:
    for case in range(8):
        uc, released = machine()
        # Valid canonical fields permit the external Rust test to exercise
        # actual file serde -> World apply -> CreateRoofCmd, not a private leaf.
        source = bytearray(88 * count)
        for i in range(count):
            struct.pack_into("<Q", source, i * 88, rng.getrandbits(64))
            struct.pack_into("<I", source, i * 88 + 8, i & 1)
            for offset in list(range(12, 36, 4)) + list(range(36, 60, 4)) + [76]:
                struct.pack_into("<f", source, i*88+offset, rng.uniform(-10, 10))
            source[i*88+60] = rng.randrange(2)
            source[i*88+61] = rng.randrange(2)
            source[i*88+64] = 2
            source[i*88+65] = rng.randrange(2)
            struct.pack_into("<II", source, i*88+68, rng.getrandbits(32), rng.getrandbits(32))
            struct.pack_into("<I", source, i*88+80, rng.getrandbits(32))
            source[i*88+84] = rng.randrange(2)
        uc.mem_write(INPUT, bytes(source))
        q(uc, LOADED + 0x40, count)
        q(uc, LOADED + 0x48, INPUT)
        q(uc, LOADED + 0x50, count)
        uc.reg_write(UC_X86_REG_RBX, EVENTS)
        event(uc, EVENTS, BUFFER, count)
        q(uc, RBP + 0x150, TICK)
        d(uc, RBP + 0x1D0, 91)
        execute(uc, 0x1417AA042, 0x1417AA196)
        actual = struct.unpack("<Q", uc.mem_read(EVENTS + 0x30, 8))[0]
        assert actual == count
        roof_cases.append({"sources": [bytes(source[i*88:(i+1)*88]).hex() for i in range(count)],
            "payloads": [bytes(uc.mem_read(BUFFER+i*64+8, 53)).hex() for i in range(count)]})

color_cases = []
for count in [0, 1, 3]:
    for case in range(8):
        uc, released = machine()
        inputs = []
        for family, offset in enumerate([0x70, 0x88, 0xA0]):
            entries = [(rng.getrandbits(64), rng.getrandbits(32)) for _ in range(count)]
            pointer = INPUT + family * 0x1000
            inputs.append(entries)
            for i, (wall, color) in enumerate(entries):
                uc.mem_write(pointer+i*16, struct.pack("<QI", wall, color))
            q(uc, LOADED+offset, count)
            q(uc, LOADED+offset+8, pointer)
            q(uc, LOADED+offset+16, count)
            event(uc, EVENTS+family*0x1000, BUFFER+family*0x1000, count)
        uc.reg_write(UC_X86_REG_R14, EVENTS)
        uc.reg_write(UC_X86_REG_RDI, EVENTS+0x1000)
        q(uc, RBP+0x178, EVENTS+0x2000)
        for off in [0x1F8, 0x60, 0x68]:
            q(uc, RBP+off, TICK)
        for off in [0x200, 0x114, 0x118]:
            d(uc, RBP+off, 91)
        execute(uc, 0x1417AA218, 0x1417AA4BA)
        payloads = []
        for family in range(3):
            actual = struct.unpack("<Q", uc.mem_read(EVENTS+family*0x1000+0x30, 8))[0]
            assert actual == count
            payloads.append([bytes(uc.mem_read(BUFFER+family*0x1000+i*24+8, 13)).hex() for i in range(count)])
        color_cases.append({"inputs": inputs, "payloads": payloads})

manifest = {
    "image": str(EXE), "sha256": identity,
    "function": {"address": hex(FN), "size": SIZE, "sha256": hashlib.sha256(function).hexdigest()},
    "ranges": {"roofs": ["1417aa042", "1417aa196"], "colors": ["1417aa218", "1417aa4ba"]},
    "dependency_stub": "Only deallocation140613c20; no grow path (event buffers preallocated).",
    "scope": "Native producer instruction ranges only; not full function, whole game, ECS or Rust acceptance.",
    "roofs": roof_cases, "colors": color_cases,
}
(OUT / "native-producer-fixtures.json").write_text(json.dumps(manifest, indent=2), encoding="utf-8")
print(json.dumps({"roofs": len(roof_cases), "colors": len(color_cases), "function_sha256": manifest["function"]["sha256"]}))
