from pathlib import Path
import collections
import hashlib
import json
import random
import struct
import subprocess
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_R8, UC_X86_REG_R9, UC_X86_REG_RAX, UC_X86_REG_RSP, UC_X86_REG_RIP

PROJECT = Path(__file__).resolve().parents[4]
BINARY = PROJECT.parent.parent / "ljxsj_92385/Tiny Glade/tiny-glade.rne"
OUTPUT = Path(__file__).resolve().parent
raw = BINARY.read_bytes()
digest = hashlib.sha256(raw).hexdigest()
assert digest == "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
pe = struct.unpack_from("<I", raw, 60)[0]
optional = pe + 24
base = struct.unpack_from("<Q", raw, optional + 24)[0]
size = struct.unpack_from("<I", raw, optional + 56)[0]
cpu = Uc(UC_ARCH_X86, UC_MODE_64)
cpu.mem_map(base, (size + 4095) & ~4095)
for section in range(struct.unpack_from("<H", raw, pe + 6)[0]):
    header = optional + struct.unpack_from("<H", raw, pe + 20)[0] + 40 * section
    virtual_size, rva, raw_size, raw_offset = struct.unpack_from("<IIII", raw, header + 8)
    cpu.mem_write(base + rva, raw[raw_offset:raw_offset + raw_size])
DATA, STACK, STOP = 0x20000000, 0x30000000, 0x40000000
cpu.mem_map(DATA, 0x100000)
cpu.mem_map(STACK, 0x100000)
cpu.mem_map(STOP, 0x1000)
state = {}
panic_entries = {0x1428d9518, 0x1428d9760, 0x1428d9390, 0x1428d9430}

def return_dependency():
    stack = cpu.reg_read(UC_X86_REG_RSP)
    target = struct.unpack("<Q", cpu.mem_read(stack, 8))[0]
    cpu.reg_write(UC_X86_REG_RSP, stack + 8)
    cpu.reg_write(UC_X86_REG_RIP, target)

def hook(machine, address, instruction_size, user):
    if address in panic_entries:
        state["panic"] = hex(address)
        machine.emu_stop()
    elif address == 0x140c10dd0:
        code = machine.reg_read(UC_X86_REG_RDX) & 255
        state["calls"].append(code)
        mask = state["masks"][code]
        machine.mem_write(machine.reg_read(UC_X86_REG_RCX), struct.pack("<IIII", 0, mask, 0, 0))
        return_dependency()
    elif address == 0x1418adc70:
        state["exports"].add(machine.reg_read(UC_X86_REG_RDX) & 65535)
        machine.reg_write(UC_X86_REG_RAX, 0)
        return_dependency()
    elif address == 0x142923c20:
        destination = machine.reg_read(UC_X86_REG_RCX)
        machine.mem_write(destination, bytes([machine.reg_read(UC_X86_REG_RDX) & 255]) * machine.reg_read(UC_X86_REG_R8))
        machine.reg_write(UC_X86_REG_RAX, destination)
        return_dependency()
    elif address in {0x1428a29e0, 0x140562e70, 0x140563600, 0x140b5f870}:
        raise RuntimeError("unexpected standard Vec allocation")

cpu.hook_add(UC_HOOK_CODE, hook)

def words(values):
    return " ".join(format(value, "x") for value in values)

def record_words(records):
    return " ".join(",".join(format(value, "x") for value in record) for record in records)

def vector_records(info_offset, format_code, stride):
    pointer, length = struct.unpack("<QQ", cpu.mem_read(DATA + info_offset + 8, 16))
    return [struct.unpack(format_code, cpu.mem_read(pointer + stride * entry, struct.calcsize(format_code))) for entry in range(length)]

def native(origins, exports, masks, passes):
    cpu.mem_write(DATA, bytes(0x100000))
    for offset, pointer, capacity in [(0, 0x1000, 1024), (0x18, 0x7000, 512), (0x30, 0x9000, 512), (0x48, 0xc000, 512), (0x60, 0xe000, 512), (0x78, 0x10000, 1024)]:
        cpu.mem_write(DATA + offset, struct.pack("<QQQ", capacity, DATA + pointer, 0))
    declaration = DATA + 0x20000
    origin_pointer = DATA + 0x21000
    export_pointer = DATA + 0x25000
    pass_pointer = DATA + 0x28000
    access_pointer = DATA + 0x30000
    cpu.mem_write(declaration, struct.pack("<QQQQQQ", len(origins), origin_pointer, len(origins), len(exports), export_pointer, len(exports)))
    for index, origin in enumerate(origins):
        blob = bytearray(72)
        struct.pack_into("<Q", blob, 0, origin[0])
        if origin[0] == 0:
            blob[0x36] = origin[1]
            struct.pack_into("<I", blob, 0x18, origin[2])
            struct.pack_into("<I", blob, 0x20, origin[3])
        else:
            blob[8] = origin[1]
        cpu.mem_write(origin_pointer + index * 72, bytes(blob))
    for index, export in enumerate(exports):
        blob = bytearray(48)
        blob[0] = export[0]
        struct.pack_into("<I", blob, 4 if export[0] == 0 else 0x18, export[1])
        blob[0x28] = export[2]
        cpu.mem_write(export_pointer + index * 48, bytes(blob))
    for index, accesses in enumerate(passes):
        cpu.mem_write(pass_pointer + index * 64, struct.pack("<QQQ", len(accesses), access_pointer, len(accesses)) + bytes(40))
        for resource, group, code in accesses:
            cpu.mem_write(access_pointer, struct.pack("<IIB3x", resource, group, code))
            access_pointer += 12
    state.clear()
    state.update(exports=set(), calls=[], masks=masks)
    stack = STACK + 0x80008
    cpu.reg_write(UC_X86_REG_RSP, stack)
    cpu.mem_write(stack, struct.pack("<Q", STOP))
    for register, value in [(UC_X86_REG_RCX, DATA), (UC_X86_REG_RDX, declaration), (UC_X86_REG_R8, pass_pointer), (UC_X86_REG_R9, len(passes))]:
        cpu.reg_write(register, value)
    cpu.emu_start(0x140530430, STOP, count=2000000)
    if "panic" not in state:
        assert cpu.reg_read(UC_X86_REG_RIP) == STOP
    accesses = vector_records(0, "<IIIIB", 20)
    ranges = vector_records(0x18, "<II", 8)
    lifetimes = []
    for first_tag, first_pass, last_tag, last_pass, first_access, last_access in vector_records(0x30, "<IIIIII", 24):
        lifetimes.append((first_tag, first_pass if first_tag == 1 else 0, last_tag, last_pass if last_tag == 1 else 0, first_access, last_access))
    return "|".join([
        "panic" if "panic" in state else "ok", record_words(accesses), record_words(ranges), record_words(lifetimes),
        words([entry[0] for entry in vector_records(0x48, "<I", 4)]),
        words([entry[0] for entry in vector_records(0x60, "<I", 4)]),
        words([entry[0] for entry in vector_records(0x78, "<I", 4)]),
        words(sorted(state["exports"])), words(state["calls"]),
    ])

rng = random.Random(20261012)
cases = []
for count in [0, 1, 2, 4]:
    cases.append(([(0, 0, 0, 0), (1, 0, 0, 0), (2, 1, 0, 0)], [], [0x20], [[] for _ in range(count)]))
cases.extend([
    ([(0, 8, 0, 0)], [], [0x20], [[(0, 0, 0)]]),
    ([(0, 0, 0, 0)], [], [0], [[(0, 0, 0)]]),
    ([(0, 7, 0, 0)], [], [0], [[(0, 0, 0)]]),
    ([(1, 2, 0, 0)], [], [0], [[(0, 0, 0)]]),
    ([], [(0, 65535, 0)], [0], []),
    ([], [(1, 65536, 0)], [0], []),
    ([], [(0, 0, 0)], [0x20], []),
    ([(0, 0, 0, 0)], [], [0x20], [[(1, 0, 0)]]),
])
image_masks = [0x20, 0x40, 0x18000, 0x80, 0x100, 0x180, 0x200, 0x400, 0x600, 0x220, 0x800, 0x1000]
for case_index in range(500):
    count = rng.randrange(1, 10)
    origins = []
    for resource_index in range(count):
        tag = rng.randrange(3)
        kind = rng.randrange(3) if tag else rng.randrange(2)
        origins.append((tag, kind if tag else 7 if kind else 0, rng.getrandbits(32), rng.getrandbits(32)))
    masks = image_masks + [0, 0xdeadbeef]
    passes = []
    for pass_index in range(rng.randrange(0, 9)):
        accesses = []
        for access_index in range(rng.randrange(0, 8)):
            resource = rng.randrange(count)
            kind = origins[resource][1] if origins[resource][0] else 1 if origins[resource][1] == 7 else 0
            code = rng.randrange(len(image_masks)) if kind == 0 else rng.randrange(len(masks))
            accesses.append((resource, rng.getrandbits(32), code))
        passes.append(accesses)
    exports = []
    for export_index in range(rng.randrange(0, 6)):
        tag = rng.randrange(2)
        exports.append((tag, rng.randrange(count), rng.randrange(len(image_masks)) if tag == 0 else rng.randrange(len(masks))))
    cases.append((origins, exports, masks, passes))
lines, expected = [], []
for origins, exports, masks, passes in cases:
    lines.append("|".join([record_words(origins), record_words(exports), words(masks), ";".join(record_words(accesses) for accesses in passes) if passes else "-"]))
    expected.append(native(origins, exports, masks, passes))
runner = PROJECT / "reconstruction/render/work-packages/resource-info/target/debug/resource-info-snapshot.exe"
process = subprocess.run([str(runner)], input="\n".join(lines) + "\n", text=True, capture_output=True, check=True)
actual = process.stdout.splitlines()
assert len(actual) == len(expected)
for index, (native_output, rust_output) in enumerate(zip(expected, actual)):
    if native_output != rust_output:
        raise AssertionError(f"case {index}\ninput={lines[index]}\nnative={native_output}\nrust={rust_output}")
report = {
    "status": "verified_semantic_snapshot",
    "binary_sha256": digest,
    "entry": "0x140530430",
    "cases": len(cases),
    "outcomes": dict(collections.Counter(entry.split("|")[0] for entry in expected)),
    "fields_compared": ["accesses", "pass_access_ranges", "lifetime_endpoints", "first_last_access_indices", "image_usage", "buffer_usage", "lifetime_events", "exported_index_set", "dependency_access_info_call_order"],
    "reuse": "candidate reuses one ResourceInfo across all cases; original allocations preallocated and reset per case",
    "dependency_boundaries": ["vk_sync_fork::get_access_info outputs supplied as explicit masks, not reimplemented", "hashbrown insert supplied as exported-index set side effect", "memset dependency", "standard Vec capacity preallocated"],
    "limitations": ["not full graph integration", "no actual dependency AccessType enum validation", "None payload and padding canonicalized", "panic observed at entry without unwind", "fewer than 21 lifetime events use untouched emitted insertion sort; large std sort path not repeated"],
}
(OUTPUT / "verification.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
print(json.dumps(report, indent=2))
