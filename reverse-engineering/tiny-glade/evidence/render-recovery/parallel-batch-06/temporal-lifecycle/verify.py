"""Original compile take prefix and injected post-export restore ownership slices."""
from pathlib import Path
import hashlib
import importlib.util
import itertools
import json
import random
import struct
import subprocess
import sys
sys.path.insert(0, 'C:/Users/liyu/AppData/Local/Packages/PythonSoftwareFoundation.Python.3.12_qbz5n2kfra8p0/LocalCache/local-packages/Python312/site-packages')
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[3]
SOURCE = ROOT / 'reconstruction/render/work-packages/temporal-lifecycle-r8'
BASE_PATH = ROOT / 'evidence/render-recovery/parallel-batch-03/barrier-record/verify.py'
spec = importlib.util.spec_from_file_location('temporal_native', BASE_PATH)
base = importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)
CPU = base.CPU
GRAPH = base.MEMORY
STATE = {}

def hook(cpu, address, size, user):
    if address in [0x140166617, 0x140166e19]:
        STATE['cut'] = address
        cpu.emu_stop()
    elif address == 0x14289aa40:
        base.ret() # Compiler stack probing; preserve RAX stack-size argument.
    elif address == 0x142923c00:
        dest, source, size = [cpu.reg_read(x) for x in [UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_R8]]
        cpu.mem_write(dest, bytes(cpu.mem_read(source, size)))
        base.ret(dest)
    elif address == 0x14004ceb0:
        # Existing standard HashMap drop is external. Capture the game call boundary.
        STATE['events'].append('drop-old-records')
        base.ret()

CPU.hook_add(UC_HOOK_CODE, hook)

def native(case):
    mode = case[0]
    base.setup([0, 0, 0, 0, 256, 0, 0, 4, 0, 0, 1, 0])
    base.STATE['cut'] = base.STOP
    STATE.clear()
    STATE.update(events=[])
    stack = base.STACK + 0x10000
    if mode == 2:
        base.write(GRAPH + 0x80, '<I', case[1])
        base.write(stack, '<Q', base.STOP)
        CPU.reg_write(UC_X86_REG_RSP, stack)
        CPU.reg_write(UC_X86_REG_RCX, GRAPH)
        CPU.emu_start(0x140976880, base.STOP, count=20)
        return str(CPU.reg_read(UC_X86_REG_RAX) & 255)
    _, graph_present, temporal_present, marker, new_marker = case
    graph = bytearray([marker] * 144)
    struct.pack_into('<Q', graph, 0, 3 if graph_present else 1 << 63)
    temporal = bytearray([marker] * 96)
    struct.pack_into('<Q', temporal, 0, temporal_present)
    if mode == 1:
        temporal[56:64] = bytes(8) # Empty old parity map avoids generic allocator internals.
    CPU.mem_write(GRAPH, bytes(graph + temporal))
    if mode == 0:
        base.write(stack, '<Q', base.STOP)
        base.write(stack + 0x28, '<Q', base.MEMORY + 0x5000)
        CPU.reg_write(UC_X86_REG_RSP, stack)
        CPU.reg_write(UC_X86_REG_RCX, base.MEMORY + 0x3000)
        CPU.reg_write(UC_X86_REG_RDX, GRAPH)
        CPU.reg_write(UC_X86_REG_R8, base.DEVICE)
        CPU.reg_write(UC_X86_REG_R9, base.INNER)
        CPU.emu_start(0x140166510, base.STOP, count=2000)
        if 'cut' in STATE:
            frame = CPU.reg_read(UC_X86_REG_RBP)
            graph_copy = bytes(CPU.mem_read(frame + 0x14b0, 144))
            # First qword taken separately; tail layout is at frame+0x1568.
            temporal_copy = struct.pack('<Q', base.read(frame + 0x18c8, '<Q')[0]) + bytes(CPU.mem_read(frame + 0x1568, 88))
            detached = graph_copy + temporal_copy
            events = ['taken']
        else:
            assert base.STATE['panic'] != 'ok'
            detached = b''
            events = ['temporal-missing' if graph_present else 'graph-missing']
    else:
        frame = stack - 0x4000
        CPU.reg_write(UC_X86_REG_RBP, frame)
        CPU.reg_write(UC_X86_REG_RSP, frame - 0x80)
        CPU.reg_write(UC_X86_REG_RDI, GRAPH)
        CPU.reg_write(UC_X86_REG_R15, GRAPH + 144)
        replacement = bytearray([new_marker] * 96)
        struct.pack_into('<Q', replacement, 0, 1)
        CPU.mem_write(frame + 0x1650, bytes(replacement))
        CPU.emu_start(0x140166d2a, base.STOP, count=1000)
        assert STATE['cut'] == 0x140166e19
        events = STATE['events']
        detached = b''
    return f"{','.join(events)}|{bytes(CPU.mem_read(GRAPH, 144)).hex()}|{bytes(CPU.mem_read(GRAPH + 144, 96)).hex()}|{detached.hex()}"

def main():
    cases = [[mode, gp, tp, marker, new_marker] for mode, gp, tp, marker, new_marker in itertools.product([0, 1], [0, 1], [0, 1], [0, 1, 0x55, 0xff], [0, 0xa5, 0xff])]
    rng = random.Random(20261012)
    frame_words = [0, 1, 2, 0xffffffff, 0x80000000] + [rng.getrandbits(32) for _ in range(1000)]
    cases += [[2, word] for word in frame_words]
    expected = [native(case) for case in cases]
    subprocess.run(['cargo', 'build', '--release', '--offline', '--manifest-path', str(SOURCE / 'Cargo.toml')], check=True)
    actual = subprocess.run([str(SOURCE / 'target/release/tiny-glade-temporal-lifecycle-r8.exe')], input=''.join(' '.join(map(str, case)) + '\n' for case in cases), text=True, capture_output=True, check=True).stdout.splitlines()
    assert len(expected) == len(actual)
    failures = [{'input': c, 'native': n, 'rust': r} for c, n, r in zip(cases, expected, actual) if n != r]
    paths = [SOURCE / 'src/lib.rs', SOURCE / 'src/main.rs', SOURCE / 'Cargo.toml', SOURCE / 'Cargo.lock', Path(__file__).resolve(), BASE_PATH, OUT / 'job.json', OUT / 'mcp-report.json', OUT / 'direct-callers.json', ROOT / 'evidence/render-recovery/parallel/compiled-state/0x140166510.c', ROOT / 'evidence/render-architecture/0x140161cd0.c']
    report = {'status': 'verified_partial' if not failures else 'failed', 'target_sha256': base.EXPECTED_SHA, 'cases': len(cases), 'take_prefix_cases': 48, 'restore_injected_slice_cases': 48, 'frame_reset_leaf_cases': len(frame_words), 'mismatches': len(failures), 'failures': failures[:4], 'hashes': {str(p.relative_to(ROOT)).replace('\\', '/'): hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}, 'limits': ['compile body and unwind are not executed; take prefix starts at original entry and stops before game closure', 'restore starts at injected post-export frame, assumes empty old parity map, captures standard HashMap drop without restoring its internals', 'FrameConstants reset word semantics beyond equals-zero are unknown; predicate does not itself clear temporal maps', 'whole-state drop observed via original standard monomorphized glue but not counted as private source recovery', 'unused-key eviction and explicit full-cache reset remain unresolved']}
    (OUT / 'native-oracle.json').write_text(json.dumps([{'input': c, 'native': n} for c, n in zip(cases, expected)], indent=2) + '\n', encoding='utf-8')
    (OUT / 'validation.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
    print(json.dumps({k: report[k] for k in ['status', 'cases', 'mismatches', 'failures']}))
    assert not failures

if __name__ == '__main__':
    main()
