"""Read static jump tables, typed calls, strings and returned enum constants.

This inventories data/control-flow edges; it is not an instruction emulator or
an original behavioral oracle, and runs neither a native process nor JIT.
"""
from pathlib import Path
import hashlib, json
import capstone, pefile

WORK = Path(__file__).resolve().parent
ROOT = next(p for p in WORK.parents if (p / 'games_test/tiny-glade').is_dir())
BIN = ROOT / 'ljxsj_92385/Tiny Glade/tiny-glade.rne'
pe = pefile.PE(str(BIN), fast_load=True)
md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
md.detail = True
base = pe.OPTIONAL_HEADER.ImageBase

def instructions(va):
    return list(md.disasm((WORK / f'native/{va:x}.bin').read_bytes(), va))

names = ['dst', 'ty', 'rank', 'color_id', 'seed', 'clv', 'h_offset',
         'allow_connection', 'window_light_color', 'lantern_brightness',
         'chimney_smoke', 'stair_width', 'ladder_color', 'railing',
         'stairs_supports', 'magic_color', '<unknown>']
types = ['DecoratorDst', 'DecoratorType', 'Rank{x:u8}', 'u32', 'u32', 'u8',
         'f32', 'bool', 'u32', 'f32', 'bool', 'f32', 'u32',
         'None|Low|High|Wooden', 'Full|None', 'u32', 'IgnoredAny']
defaults = [None, None, None, 0, None, 0, 0.0, True, 0, 1.0, True, 1.0, 0,
            'None', 'Full', 0, None]
offsets = [0, 0x4a, 0x48, 0x28, 0x2c, 0x49, 0x30, 0x4c, 0x34, 0x38,
           0x4d, 0x3c, 0x40, 0x4b, 0x4e, 0x44, None]
expected_calls = [0x140a4a8a0, 0x140a480d0, 0x1415a2d40, 0x14159cfa0,
                  0x14159cfa0, 0x14159a180, 0x14159a370, 0x14159d4f0,
                  0x14159cfa0, 0x14159a370, 0x14159d4f0, 0x14159a370,
                  0x14159cfa0, 0x140a490d0, 0x140a49820, 0x14159cfa0,
                  0x1415a3940]
table = 0x142c7b808
calls = json.loads((WORK / 'native/1415ca5f0.calls.json').read_text())
fields = []
for index, name in enumerate(names):
    entry = table + int.from_bytes(pe.get_data(table-base + 4*index, 4), 'little', signed=True)
    call = next(c for c in calls if int(c['site'], 16) >= entry)
    assert int(call['target'], 16) == expected_calls[index], (name, call)
    fields.append({'selector': index, 'name': name, 'aliases': ['chimney_h_offset'] if name == 'h_offset' else [],
        'type': types[index], 'required': index in (0, 1, 2, 4),
        'missing_default': defaults[index], 'projected_offset': offsets[index],
        'jump_table_entry_va': hex(table + 4*index), 'case_start': hex(entry),
        'typed_decode_call': call,
        'default_join': '0x1415caf0d..0x1415cb0d3',
        'field_selector_function': '0x1417c1170'})
(WORK / 'state-field-bindings.json').write_text(json.dumps(fields, indent=2) + '\n')

# Serde diagnostics give independently length-bound required/duplicate keys.
diagnostics = []
for row in json.loads((WORK / 'native/ranges.json').read_text()):
    va = int(row['va'], 16)
    ptr = length = None
    for i in instructions(va):
        ops = i.operands
        if i.mnemonic == 'lea' and len(ops) == 2 and ops[0].type == capstone.x86.X86_OP_REG and ops[0].reg == capstone.x86.X86_REG_RCX and ops[1].type == capstone.x86.X86_OP_MEM and ops[1].mem.base == capstone.x86.X86_REG_RIP:
            ptr = (i.address, i.address + i.size + ops[1].mem.disp)
        if i.mnemonic == 'mov' and len(ops) == 2 and ops[0].type == capstone.x86.X86_OP_REG and ops[0].reg == capstone.x86.X86_REG_EDX and ops[1].type == capstone.x86.X86_OP_IMM:
            length = (i.address, ops[1].imm)
        if i.mnemonic == 'call' and ops[0].type == capstone.x86.X86_OP_IMM and ops[0].imm in (0x1428f9bc0, 0x1428f9db0):
            assert ptr and length
            data = pe.get_data(ptr[1]-base, length[1])
            diagnostics.append({'function': hex(va), 'call': hex(i.address),
                'kind': 'missing_field' if ops[0].imm == 0x1428f9bc0 else 'duplicate_field',
                'string_lea': hex(ptr[0]), 'string_va': hex(ptr[1]),
                'length_site': hex(length[0]), 'length': length[1], 'key': data.decode()})
            ptr = length = None
(WORK / 'required-duplicate-key-bindings.json').write_text(json.dumps(diagnostics, indent=2) + '\n')

# The visitor's actual error-list order agrees with the independently recovered
# serializer. Return sites expose every discriminant in the complete reader.
mesh_table = 0x142cda7c8
reader_stores = {}
for i in instructions(0x14179b870):
    o = i.operands
    if i.mnemonic == 'mov' and len(o) == 2 and o[0].type == capstone.x86.X86_OP_MEM and o[0].mem.base == capstone.x86.X86_REG_RSI and o[0].mem.disp == 1 and o[1].type == capstone.x86.X86_OP_IMM:
        reader_stores[o[1].imm] = hex(i.address)
assert set(reader_stores) == set(range(56))
writer = ROOT / 'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-57a33750ac4a866a76/core-producer-20261018/snapshot-domain-candidate/clutter-mesh-string-bindings.json'
writer_names = [r['value'] for r in json.loads(writer.read_text())['bindings']]
variants = []
for index in range(56):
    record = pe.get_data(mesh_table-base + index*16, 16)
    ptr = int.from_bytes(record[:8], 'little')
    length = int.from_bytes(record[8:], 'little')
    name = pe.get_data(ptr-base, length).decode()
    assert name == writer_names[index]
    variants.append({'index': index, 'name': name, 'list_entry_va': hex(mesh_table+index*16),
        'string_va': hex(ptr), 'length': length, 'reader_return_store': reader_stores[index]})
(WORK / 'clutter-variant-bindings.json').write_text(json.dumps(variants, indent=2) + '\n')

# Direct constant compare leaves, not inferred type names.
enum_constants = []
for va in [0x1415d1bb0, 0x1415d3c70]:
    for i in instructions(va):
        for o in i.operands:
            if o.type == capstone.x86.X86_OP_MEM and o.mem.base == capstone.x86.X86_REG_RIP:
                at = i.address + i.size + o.mem.disp
                data = pe.get_data(at-base, o.size)
                # Side compares the first four bytes and the fifth byte in two
                # adjacent instructions; retain the complete five-byte literal.
                if va == 0x1415d3c70 and o.size == 4:
                    data = pe.get_data(at-base, 5)
                try: text = data.decode('ascii')
                except UnicodeDecodeError: continue
                if text in ('Full', 'None', 'Inner', 'Outer'):
                    enum_constants.append({'function': hex(va), 'instruction': hex(i.address), 'va': hex(at), 'bytes': data.hex(), 'text': text})
(WORK / 'enum-literal-bindings.json').write_text(json.dumps(enum_constants, indent=2) + '\n')
assert {'Full', 'None', 'Inner', 'Outer'} <= {r['text'] for r in enum_constants}
print(json.dumps({'state_selectors': len(fields), 'required_duplicate_diagnostics': len(diagnostics), 'clutter_variants': len(variants), 'enum_literal_compares': len(enum_constants), 'native_execution': False}))
