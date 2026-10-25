"""Original private caller entry oracle; backend creation/begin are fixture boundaries."""
from pathlib import Path
import hashlib
import importlib.util
import itertools
import json
import struct
import subprocess
import sys
sys.path.insert(0, 'C:/Users/liyu/AppData/Local/Packages/PythonSoftwareFoundation.Python.3.12_qbz5n2kfra8p0/LocalCache/local-packages/Python312/site-packages')
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[3]
SOURCE = ROOT / 'reconstruction/render/work-packages/queue-callers-r7'
spec = importlib.util.spec_from_file_location('queue_base', ROOT / 'evidence/render-recovery/parallel-batch-03/barrier-record/verify.py')
base = importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)
CPU = base.CPU
EXECUTOR = base.MEMORY
COMMANDS = base.MEMORY + 0x3000
DEVICE = base.MEMORY + 0x8000
STATE = {}

def hook(cpu, address, size, user):
    if address == 0x140BEF4A0:
        out = cpu.reg_read(UC_X86_REG_RCX)
        family = base.read(cpu.reg_read(UC_X86_REG_RSP) + 0x28, '<I')[0]
        STATE['events'].append(f'create:{family}')
        cpu.mem_write(out, bytes([STATE['fill']]) * 88)
        base.ret(out)
    elif address == 0x140BED230:
        out = cpu.reg_read(UC_X86_REG_RCX)
        tag = cpu.reg_read(UC_X86_REG_R8) & 0xffffffff
        STATE['events'].append(f'begin:{tag}')
        assert tag == 0
        base.write(out, '<QQII', 1, STATE['sequence'], tag, 0)
        base.write(out + 86, '<B', 0)
        base.ret()
    elif address == 0x140562F30:
        vec = cpu.reg_read(UC_X86_REG_RCX)
        base.write(vec, '<QQQ', 1, COMMANDS, 0)
        base.ret()

CPU.hook_add(UC_HOOK_CODE, hook)

def native(case, capacity):
    length, family, fill, sequence = case
    base.setup([0, 0, 0, 0, 256, 0, 0, 4, 0, 0, 1, 0])
    base.STATE['cut'] = base.STOP
    CPU.mem_write(EXECUTOR, bytes(0x4000))
    base.write(EXECUTOR + 0x40, '<QQQ', capacity, COMMANDS, length)
    base.write(EXECUTOR + 0x178, '<Q', DEVICE)
    base.write(DEVICE + 0xad4, '<I', family)
    CPU.mem_write(COMMANDS, bytes([fill]) * max(88, length * 88))
    STATE.clear()
    STATE.update(fill=fill, sequence=sequence, events=[])
    stack = base.STACK + 0x10000
    base.write(stack, '<Q', base.STOP)
    CPU.reg_write(UC_X86_REG_RSP, stack)
    CPU.reg_write(UC_X86_REG_RCX, EXECUTOR)
    CPU.emu_start(0x140BEBC90, base.STOP, count=2000)
    assert CPU.reg_read(UC_X86_REG_RIP) == base.STOP
    count = base.read(EXECUTOR + 0x50, '<Q')[0]
    payload = bytes(CPU.mem_read(COMMANDS, count * 88)).hex()
    return f"{count}|{','.join(STATE['events'])}|{payload}"

def main():
    cases = []
    for length, family, fill, sequence in itertools.product([0, 1, 2, 4], [0, 1, 9, 0xffffffff], [0, 0xa5, 0xff], [1, 42, 0xffffffffffffffff]):
        for capacity in ([0, 4] if length == 0 else [4]):
            cases.append(([length, family, fill, sequence], capacity))
    expected = [native(case, cap) for case, cap in cases]
    subprocess.run(['cargo', 'build', '--release', '--offline', '--manifest-path', str(SOURCE / 'Cargo.toml')], check=True)
    actual = subprocess.run([str(SOURCE / 'target/release/tiny-glade-queue-callers-r7.exe')], input=''.join(' '.join(map(str, case)) + '\n' for case, _ in cases), capture_output=True, text=True, check=True).stdout.splitlines()
    failures = [{'case': case, 'capacity': cap, 'native': a, 'rust': b} for (case, cap), a, b in zip(cases, expected, actual) if a != b]
    assert len(actual) == len(expected)
    functions_path = ROOT / 'evidence/render-recovery/workgraph/functions.json'
    calls_path = ROOT / 'evidence/render-recovery/workgraph/calls.json'
    functions = json.loads(functions_path.read_text())
    calls = json.loads(calls_path.read_text())
    by = {x['va']: x for x in functions}
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    inventory = []
    for target in ['0x140bed230', '0x140c04e20']:
        for va, record in calls['functions'].items():
            sites = [x for x in record['direct'] if x['target'] == target]
            if not sites:
                continue
            proc = by[va]
            instructions = list(md.disasm(bytes(CPU.mem_read(int(va, 16), proc['code_size'])), int(va, 16)))
            for site in sites:
                index = next(i for i, ins in enumerate(instructions) if ins.address == int(site['site'], 16))
                window = instructions[max(0, index - 12):index + 1]
                write = next((ins for ins in reversed(window[:-1]) if ins.op_str.startswith(('r8d,', 'r8,')) and ins.mnemonic in ['xor', 'mov', 'lea']), None)
                primary = write is not None and write.mnemonic == 'xor' and write.op_str == 'r8d, r8d'
                inventory.append({'source': va, 'names': proc['names'], 'target': target, 'site': site['site'], 'observed_queue_tag': 0 if primary else None, 'window': [f'{ins.address:#x}: {ins.mnemonic} {ins.op_str}' for ins in window]})
    (OUT / 'callsite-inventory.json').write_text(json.dumps({'scope': 'direct call sites from 450 indexed rhapsody addresses; not whole-program or indirect-call absence', 'functions_sha256': hashlib.sha256(functions_path.read_bytes()).hexdigest(), 'calls_sha256': hashlib.sha256(calls_path.read_bytes()).hexdigest(), 'sites': inventory}, indent=2))
    (OUT / 'native-oracle.json').write_text(json.dumps([{'input': case, 'capacity': cap, 'output': result} for (case, cap), result in zip(cases, expected)], indent=2))
    report = {'status': 'verified_partial' if not failures else 'failed', 'target_sha256': base.EXPECTED_SHA, 'entry': '0x140bebc90', 'original_entry_cases': len(cases), 'mismatches': len(failures), 'failures': failures[:5], 'direct_begin_sites': sum(x['target'] == '0x140bed230' for x in inventory), 'direct_submit_sites': sum(x['target'] == '0x140c04e20' for x in inventory), 'primary_tag_sites': sum(x['observed_queue_tag'] == 0 for x in inventory), 'source_sha256': {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted((SOURCE / 'src').glob('*.rs'))}, 'harness_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(), 'limits': ['create_command_buffer and CommandBuffer::begin are captured fixture boundaries; their full entry implementation is not validated here', 'Vec reserve is delegated; capacity ABI, OOM, unwinding and GPU execution are excluded', 'callsite inventory is bounded to rhapsody workgraph; inlined callers and indirect targets remain unknown', 'Ghidra saved xrefs are incomplete and are not used as negative caller evidence', 'no source-level atomic ordering or actual multiqueue reachability conclusion']}
    (OUT / 'validation.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({k: report[k] for k in ['status', 'original_entry_cases', 'mismatches', 'direct_begin_sites', 'direct_submit_sites', 'primary_tag_sites']}))
    assert not failures

if __name__ == '__main__':
    main()
