"""Bounded target-specific direct-call search, validated against PE runtime spans."""
from pathlib import Path
import bisect
import hashlib
import json
import struct
import sys
sys.path.insert(0, 'C:/Users/liyu/AppData/Local/Packages/PythonSoftwareFoundation.Python.3.12_qbz5n2kfra8p0/LocalCache/local-packages/Python312/site-packages')
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[3]
TARGET = ROOT / 'workspace/input/tiny-glade.exe'
RAW = TARGET.read_bytes()
EXPECTED = 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
assert hashlib.sha256(RAW).hexdigest() == EXPECTED
pe = struct.unpack_from('<I', RAW, 60)[0]
opt = pe + 24
base = struct.unpack_from('<Q', RAW, opt + 24)[0]
sections = []
for i in range(struct.unpack_from('<H', RAW, pe + 6)[0]):
    at = opt + struct.unpack_from('<H', RAW, pe + 20)[0] + i * 40
    name = RAW[at:at + 8].split(b'\0')[0].decode()
    vsize, rva, size, offset = struct.unpack_from('<IIII', RAW, at + 8)
    sections.append((name, rva, size, offset))

def file_offset(rva):
    for _, start, size, offset in sections:
        if start <= rva < start + size:
            return offset + rva - start
    raise ValueError(hex(rva))

exception_rva, exception_size = struct.unpack_from('<II', RAW, opt + 112 + 3 * 8)
exception_offset = file_offset(exception_rva)
spans = [struct.unpack_from('<III', RAW, exception_offset + i)[:2] for i in range(0, exception_size, 12)]
starts = [s for s, e in spans]
md = Cs(CS_ARCH_X86, CS_MODE_64)
targets = {int(v, 16) for v in ['0x1405fc3d0', '0x14060d340', '0x1405f9f50', '0x140976880']}
found = []
decode_cache = {}
unbounded = []
for name, rva, size, offset in sections:
    if name != '.text':
        continue
    blob = RAW[offset:offset + size]
    for opcode in [b'\xe8', b'\xe9']:
        position = 0
        while True:
            position = blob.find(opcode, position)
            if position < 0 or position + 5 > len(blob):
                break
            site = base + rva + position
            target = site + 5 + struct.unpack_from('<i', blob, position + 1)[0]
            position += 1
            if target not in targets:
                continue
            index = bisect.bisect_right(starts, site - base) - 1
            start, end = spans[index]
            if not start <= site - base < end or end - start > 100000:
                unbounded.append({'site': hex(site), 'target': hex(target)})
                continue
            if start not in decode_cache:
                at = file_offset(start)
                decode_cache[start] = list(md.disasm(RAW[at:at + end - start], base + start))
            instructions = decode_cache[start]
            ins_index = next((i for i, ins in enumerate(instructions) if ins.address == site), None)
            if ins_index is None:
                continue
            ins = instructions[ins_index]
            if ins.mnemonic not in ['call', 'jmp'] or ins.op_str != hex(target):
                continue
            window = instructions[max(0, ins_index - 10):ins_index + 6]
            found.append({'site': hex(site), 'target': hex(target), 'runtime_start': hex(base + start), 'runtime_end': hex(base + end), 'kind': ins.mnemonic, 'window': [f'{v.address:#x}: {v.mnemonic} {v.op_str}' for v in window]})

report = {'binary_sha256': EXPECTED, 'targets': [hex(x) for x in sorted(targets)], 'search_scope': '.text direct E8/E9 encodings to four selected targets, validated on PE RUNTIME_FUNCTION instruction boundaries; indirect, inlined and unbounded code excluded', 'sites': found, 'unbounded_candidates': unbounded, 'decoded_function_count': len(decode_cache)}
(OUT / 'direct-callers.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
for x in found:
    print(x['runtime_start'], x['site'], x['target'], x['kind'])
print(json.dumps({'sites': len(found), 'decoded_functions': len(decode_cache), 'unbounded': len(unbounded)}))
