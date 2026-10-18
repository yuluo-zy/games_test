"""Bounded, identity-checked read-only evidence extraction for W2/W12 review."""
from pathlib import Path
import hashlib
import json
import struct

import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

ROOT = Path('/Users/yuluo/Desktop/game')
OUT = Path(__file__).parent
RNE = ROOT / 'ljxsj_92385/Tiny Glade/tiny-glade.rne'
RNE_SHA = 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
PROCS = ROOT / 'games_test/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv'
SHADER = ROOT / 'ljxsj_92385/Tiny Glade/compiled-shaders/_gauss_blur_mask.cs.hlsl_cs_main_0.bin'
SHADER_SHA = '3474c437ce66699d60a2aabcbea699c58a48408879e359b3b38b63b44a2d8258'
RETAINED = ROOT / 'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-60ea7c6fe1579df293/render-water-live-20261018/_gauss_blur_mask.cs.hlsl_cs_main_0.bin.spv'
WANTED = {0x140a12680, 0x140a22140, 0x1403d9610, 0x1403d9b30, 0x1404ec110}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    data = RNE.read_bytes()
    assert sha(data) == RNE_SHA
    pe = pefile.PE(data=data, fast_load=True)
    rows = {}
    for line in PROCS.read_text().splitlines():
        cols = line.split('\t')
        try:
            va = int(cols[0], 0)
        except ValueError:
            continue
        if va in WANTED:
            rows[va] = (int(cols[1]), cols[4])
    assert rows.keys() == WANTED
    cs = Cs(CS_ARCH_X86, CS_MODE_64)
    cs.detail = True
    native = []
    constants = {}
    for va, (length, name) in sorted(rows.items()):
        raw = pe.get_data(va - pe.OPTIONAL_HEADER.ImageBase, length)
        instructions = list(cs.disasm(raw, va))
        assert sum(ins.size for ins in instructions) == length
        path = OUT / f'{va:x}.asm'
        path.write_text(''.join(f'{ins.address:x}: {bytes(ins.bytes).hex()} {ins.mnemonic} {ins.op_str}\n' for ins in instructions))
        if va in (0x140a12680, 0x140a22140):
            for ins in instructions:
                if ins.mnemonic != 'movss' or '[rip' not in ins.op_str:
                    continue
                target = ins.address + ins.size + ins.operands[1].mem.disp
                value = pe.get_data(target - pe.OPTIONAL_HEADER.ImageBase, 4)
                constants[hex(target)] = {'bytes': value.hex(), 'float': struct.unpack('<f', value)[0]}
        native.append({'entry': hex(va), 'full_pdb_length': length, 'symbol': name,
                       'range_sha256': sha(raw), 'instructions': len(instructions), 'path': path.name})
    packed = SHADER.read_bytes()
    assert sha(packed) == SHADER_SHA
    # File header is 23 bytes; the original compute stage contains exactly 1478 words.
    spv = packed[23:23 + 1478 * 4]
    assert spv == RETAINED.read_bytes()
    words = struct.unpack('<1478I', spv)
    assert words[0] == 0x07230203
    cursor = 5
    operations = []
    while cursor < len(words):
        count, opcode = words[cursor] >> 16, words[cursor] & 0xffff
        assert count > 0 and cursor + count <= len(words)
        operations.append({'word': cursor, 'opcode': opcode, 'operands': list(words[cursor + 1:cursor + count])})
        cursor += count
    assert cursor == len(words)
    (OUT / 'gauss-blur-original.words.json').write_text(json.dumps(operations, indent=2) + '\n')
    (OUT / 'gauss-blur-original.spv').write_bytes(spv)
    manifest = {'scope': 'Independent B native W2/W12 and packaged shader review; no execution, production edits or Cargo.',
                'rne': str(RNE), 'rne_sha256': RNE_SHA, 'procs': str(PROCS), 'procs_sha256': sha(PROCS.read_bytes()),
                'native': native, 'rip_float_constants': constants,
                'shader': {'file': str(SHADER), 'sha256': SHADER_SHA, 'stage_offset': 23, 'stage_words': len(words),
                           'stage_sha256': sha(spv), 'stage_equals_retained': True, 'instructions': len(operations),
                           'retained': str(RETAINED),
                           'image_fetch': [op for op in operations if op['opcode'] == 95],
                           'image_read': [op for op in operations if op['opcode'] == 98],
                           'image_write': [op for op in operations if op['opcode'] == 99]}}
    (OUT / 'original-manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    print(json.dumps({'native': native, 'rip_float_constants': constants, 'shader': manifest['shader']}, indent=2))


if __name__ == '__main__':
    main()
