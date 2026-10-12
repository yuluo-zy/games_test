from pathlib import Path
import hashlib
import itertools
import json
import random
import struct
import subprocess
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64
from unicorn.x86_const import UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_RSP

ROOT = Path("D:/game/reverse-engineering/tiny-glade")
OUT = ROOT / "evidence/render-recovery/parallel/compiled-state"
SOURCE = ROOT / "reconstruction/render/work-packages/compiled-state"
raw = (ROOT / "workspace/input/tiny-glade.exe").read_bytes()
assert hashlib.sha256(raw).hexdigest() == "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
pe_offset = struct.unpack_from("<I", raw, 60)[0]
optional_offset = pe_offset + 24
image_base = struct.unpack_from("<Q", raw, optional_offset + 24)[0]
image_size = struct.unpack_from("<I", raw, optional_offset + 56)[0]
cpu = Uc(UC_ARCH_X86, UC_MODE_64)
cpu.mem_map(image_base, (image_size + 4095) & ~4095)
section_count = struct.unpack_from("<H", raw, pe_offset + 6)[0]
optional_size = struct.unpack_from("<H", raw, pe_offset + 20)[0]
for section_index in range(section_count):
    header = optional_offset + optional_size + section_index * 40
    virtual_size, virtual_address, raw_size, raw_offset = struct.unpack_from("<IIII", raw, header + 8)
    cpu.mem_write(image_base + virtual_address, raw[raw_offset:raw_offset + raw_size])
stack = 0x20000000
stop = 0x30000000
state = 0x40000000
elements = state + 4096
cpu.mem_map(stack, 0x20000)
cpu.mem_map(stop, 4096)
cpu.mem_map(state, 0x4000)
cpu.mem_write(stop, b"\xcc")

def invoke(address):
    stack_pointer = stack + 0x10000
    cpu.mem_write(stack_pointer, struct.pack("<Q", stop) + bytes(256))
    cpu.reg_write(UC_X86_REG_RSP, stack_pointer)
    cpu.reg_write(UC_X86_REG_RCX, state)
    cpu.emu_start(address, stop, count=300)

cpu.mem_write(state, bytes([0xa5]) * 64)
invoke(0x140be95e0)
constructor = bytes(cpu.mem_read(state, 41))
assert constructor == struct.pack("<QQQQQB", 0, 8, 0, 0, 0, 0)

boundaries = [0, 1, 2, 5, 31, 255, 65535, 2**32, 2**63, 2**64 - 1]
cases = list(itertools.product(boundaries, [0, 17, 2**64 - 1], [0, 1], boundaries))
generator = random.Random(20261012)
cases += [(generator.getrandbits(64), generator.getrandbits(64), generator.randrange(2), generator.getrandbits(64)) for _ in range(1000)]
expected = []
for start, old_end, flag, boundary in cases:
    cpu.mem_write(state, struct.pack("<QQQQQB", 8, elements, 0, start, old_end, flag) + bytes([0xa5]) * 7)
    cpu.mem_write(elements, bytes(24))
    cpu.reg_write(UC_X86_REG_RDX, boundary)
    invoke(0x140be9600)
    capacity, pointer, length, current_start, current_end, current_flag = struct.unpack("<QQQQQB", bytes(cpu.mem_read(state, 41)))
    if length:
        batch_start, batch_end, batch_flag = struct.unpack("<QQB", bytes(cpu.mem_read(elements, 17)))
        expected.append(f"{length} {current_start} {current_end} {current_flag} {batch_start} {batch_end} {batch_flag}")
    else:
        expected.append(f"0 {current_start} {current_end} {current_flag}")
exe = OUT / "batch-runner.exe"
subprocess.run(["rustc", "--edition=2021", "-Awarnings", str(SOURCE / "runner.rs"), "-o", str(exe)], check=True)
payload = "".join(f"{start} {end} {flag} {boundary}\n" for start, end, flag, boundary in cases)
actual = subprocess.run([str(exe)], input=payload, text=True, capture_output=True, check=True).stdout.splitlines()
failures = [{"input":case, "original":original, "candidate":candidate} for case, original, candidate in zip(cases, expected, actual) if original != candidate]
report = {
    "ok": not failures and len(actual) == len(expected),
    "cases": len(cases),
    "constructor_observed_bytes": constructor.hex(),
    "failure_count": len(failures),
    "failures": failures[:20],
    "oracle": "Unicorn exact original entries 0x140be95e0/0x140be9600 with preallocated Vec capacity; no replaced calls",
    "source_sha256": hashlib.sha256(raw).hexdigest(),
    "candidate_sha256": hashlib.sha256((SOURCE / "lib.rs").read_bytes()).hexdigest(),
    "limits": ["Vec allocation/reservation excluded by sufficient capacity; std reused", "padding bytes excluded", "flush semantic projection only; Vulkan/compile/barrier branches not native validated", "runner executes candidate, target game is not launched"]
}
(OUT / "native-batch-verification.json").write_text(json.dumps(report, indent=2), encoding="utf8")
print(json.dumps({"ok":report["ok"], "cases":report["cases"], "failure_count":len(failures)}))
assert report["ok"]
