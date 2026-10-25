"""M1 original RNE replay -> compiled Rust CLI differential verification.

No candidate formula is used as expected output: expected bytes come from CPU
emulation. Native library allocations are excluded by preallocated ABI inputs.
"""
import hashlib
import json
import random
import struct
import subprocess
from pathlib import Path
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

OUT = Path(__file__).resolve().parent
ROOT = next(p for p in OUT.parents if (p / "tracking/baselines.json").exists())
GAME = ROOT.parents[2]
TERRAIN = ROOT / "reconstruction/terrain"
PROBE = TERRAIN / "target/release/terrain-probe"
raw = (GAME / "ljxsj_92385/Tiny Glade/tiny-glade.rne").read_bytes()
original_sha = hashlib.sha256(raw).hexdigest()
assert original_sha == "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
cpu = Uc(UC_ARCH_X86, UC_MODE_64)
end = max(s.VirtualAddress + max(s.Misc_VirtualSize, s.SizeOfRawData) for s in pe.sections)
cpu.mem_map(base, (end + 4095) & ~4095)
for s in pe.sections:
    if s.SizeOfRawData:
        cpu.mem_write(base + s.VirtualAddress, raw[s.PointerToRawData:s.PointerToRawData + s.SizeOfRawData])
DATA, STACK, STOP = 0x20000000, 0x30000000, 0x40000000
cpu.mem_map(DATA, 0x10000)
cpu.mem_map(STACK, 0x10000)
cpu.mem_map(STOP, 0x1000)
EDIT, WRITER, RESOURCE, EVENTS, CHANGE = [DATA + x for x in (0x100, 0x1000, 0x2000, 0x3000, 0x4000)]
active = [0]
returned = [False]

def hook(uc, address, size, user):
    if address == STOP:
        returned[0] = True
        uc.emu_stop()
    elif not active[0] <= address < active[0] + 0x500:
        raise RuntimeError(f"Unexpected external execution at {address:#x}")

cpu.hook_add(UC_HOOK_CODE, hook)

def bits(value):
    return struct.unpack("<I", struct.pack("<f", value))[0]

rng = random.Random(20261011)
cases = []
edges = [0, 0x80000000, bits(-2.5), bits(0.4), bits(1.0), bits(2.5), bits(20.0), bits(24.0)]
for i in range(256):
    values = [bits(rng.uniform(-30, 30)) for _ in range(4)]
    if i < 64:
        values = [edges[(i + j) % len(edges)] for j in range(4)]
    if i % 7 == 0:
        values[1] = values[0]
        values[3] = values[2]
    progress = bits([0, 0.25, 0.5, 0.75, 1, -0.25, 1.25][i % 7])
    cases.append((i, values, progress))
inputs, expected = [], []
for reverse, address in [(False, 0x1408592d0), (True, 0x140858d30)]:
    active[0] = address
    for index, values, progress in cases:
        cpu.mem_write(DATA, bytes(0x10000))
        cpu.mem_write(STACK, bytes(0x10000))
        cpu.mem_write(EDIT, struct.pack("<QI4I", 0x8000000000000001, index, *values))
        cpu.mem_write(WRITER, struct.pack("<Q", RESOURCE))
        cpu.mem_write(WRITER + 0x10, struct.pack("<Q", CHANGE))
        cpu.mem_write(WRITER + 0x1c, struct.pack("<I", 123))
        cpu.mem_write(RESOURCE + 0x20, struct.pack("<QQQ", 64, EVENTS, 0))
        cpu.mem_write(RESOURCE + 0x40, struct.pack("<Q", 17))
        rsp = STACK + 0x8000
        cpu.mem_write(rsp, struct.pack("<Q", STOP))
        cpu.mem_write(rsp + 0x28, struct.pack("<Q", WRITER))
        cpu.mem_write(rsp + 0x30, struct.pack("<I", progress))
        for register, value in [(UC_X86_REG_RCX, EDIT), (UC_X86_REG_RDX, DATA + 0x5000),
                                (UC_X86_REG_R8, WRITER), (UC_X86_REG_R9, DATA + 0x6000),
                                (UC_X86_REG_RSP, rsp), (UC_X86_REG_MXCSR, 0x1f80)]:
            cpu.reg_write(register, value)
        returned[0] = False
        cpu.emu_start(address, STOP + 1, count=10000)
        assert returned[0], "instruction limit reached before return"
        assert struct.unpack("<Q", cpu.mem_read(RESOURCE + 0x30, 8))[0] == 1
        expected.append(bytes(cpu.mem_read(EVENTS, 40)).hex())
        inputs.append(f"replay {'backward' if reverse else 'forward'} {index} " + " ".join(f"{v:08x}" for v in values) + f" {progress:08x} 17")
result = subprocess.run([str(PROBE)], input="\n".join(inputs) + "\n", capture_output=True, text=True, check=True)
actual = result.stdout.splitlines()
assert len(actual) == len(expected), "probe output count mismatch"
failures = [{"case": i, "input": inputs[i], "expected": e, "actual": a} for i, (a, e) in enumerate(zip(actual, expected)) if a != e]
(OUT / "replay-inputs.txt").write_text("\n".join(inputs) + "\n")
(OUT / "replay-native-output.txt").write_text("\n".join(expected) + "\n")
(OUT / "replay-rust-output.txt").write_text(result.stdout)
report = {"ok": not failures, "cases": len(inputs), "failures": failures,
          "original_sha256": original_sha, "probe_sha256": hashlib.sha256(PROBE.read_bytes()).hexdigest(),
          "cargo_lock_sha256": hashlib.sha256((TERRAIN / "Cargo.lock").read_bytes()).hexdigest(),
          "rust_sources": {str(p.relative_to(TERRAIN)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted((TERRAIN / "crates/tg-terrain/src").rglob("*.rs"))},
          "scope": "B Edit reapply/revert finite inputs through original function entry/return vs Rust CLI observation; exact valid 40-byte event payload",
          "excluded": ["New/Move", "NaN/Inf", "queue allocation", "ECS resolution", "UI", "GPU", "system transaction", "A/C"],
          "target_execution": False, "original_instructions_emulated": True, "candidate_rust_cli_executed": True}
(OUT / "rust-replay-diff.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
print(json.dumps({"ok": not failures, "cases": len(inputs), "failures": len(failures)}))
if failures:
    raise SystemExit(1)
