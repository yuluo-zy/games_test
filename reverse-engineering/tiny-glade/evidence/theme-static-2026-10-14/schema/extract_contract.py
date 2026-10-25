"""Offline B record/literal extraction. No target execution or service requests."""
import hashlib
import json
from pathlib import Path
import re
import subprocess
import struct

root = Path(__file__).resolve().parents[5]
out = Path(__file__).resolve().parent
pdb = root / 'ljxsj_92385/Tiny Glade/tiny_glade.pdb'
exe = root / 'ljxsj_92385/Tiny Glade/tiny-glade.exe'
data = exe.read_bytes()
nt = struct.unpack_from('<I', data, 0x3c)[0]
count = struct.unpack_from('<H', data, nt + 6)[0]
optional_size = struct.unpack_from('<H', data, nt + 20)[0]
sections = []
for i in range(count):
    at = nt + 24 + optional_size + i * 40
    virtual_size, rva, raw_size, raw_at = struct.unpack_from('<IIII', data, at + 8)
    sections.append((rva, max(virtual_size, raw_size), raw_at))

def read(va, size):
    rva = va - 0x140000000
    for start, length, at in sections:
        if start <= rva < start + length:
            return data[at + rva - start:at + rva - start + size]
    raise ValueError(hex(va))

literal_specs = [('start', 0x142a0a220, 5), ('end', 0x142a0a225, 3),
                 ('cid', 0x142a0a228, 3), ('events', 0x142a0a280, 6),
                 ('active_sequence_expectation', 0x142a0a22b, 34),
                 ('events_sequence_expectation', 0x142a0a286, 37)]
literals = [{'label': name, 'va': hex(va), 'hex': read(va, size).hex(),
             'text': read(va, size).decode('ascii')} for name, va, size in literal_specs]
# All records are parsed, but only concrete crate scopes and their function IDs
# are retained. Build-command strings with --extern theme_defs are not scopes.
ids = subprocess.check_output(['C:/Program Files/LLVM/bin/llvm-pdbutil.exe', 'dump', '--ids', str(pdb)],
                              encoding='utf-8', errors='strict')
records = re.split(r'(?=^\s+0x[0-9A-F]+ \| )', ids, flags=re.M)
scopes = set()
selected = []
for record in records:
    if re.search(r'LF_STRING_ID[^\n]*String: theme_(defs|server_proto)::', record):
        scopes.add(re.search(r'0x[0-9A-F]+', record).group())
        selected.append(record)
for record in records:
    parent = re.search(r'parent scope = (0x[0-9A-F]+)', record)
    if parent and parent.group(1) in scopes:
        selected.append(record)
(out / 'crate-ipi.txt').write_text(''.join(selected), encoding='utf-8')
type_text = subprocess.check_output(['C:/Program Files/LLVM/bin/llvm-pdbutil.exe', 'dump', '--types', str(pdb)],
                                   encoding='utf-8', errors='strict')
report = {
    'target_execution': False, 'service_requests': 0,
    'exe_sha256': hashlib.sha256(data).hexdigest(),
    'pdb_sha256': hashlib.sha256(pdb.read_bytes()).hexdigest(),
    'literals': literals,
    'ipi_scopes': sorted(scopes),
    'ipi_relevant_records': len(selected),
    'tpi_relevant_type_names': re.findall(r'`[^`]*(?:theme_defs|theme_server_proto)[^`]*`', type_text),
    'tpi_rust_fields_not_recovered': True,
}
memory = json.loads((out / 'inventory.artifacts/003.stdout.txt').read_text(encoding='utf-8'))
native_bytes = bytes.fromhex(memory['hex'])
assert memory['address'] == '0x142a0a220'
assert native_bytes == read(0x142a0a220, memory['length'])
report['native_memory_matches_exe_bytes'] = True
report['native_memory_sha256'] = hashlib.sha256(native_bytes).hexdigest()
for crate, index in [('theme_defs', 1), ('theme_server_proto', 2)]:
    inventory = json.loads((out / f'inventory.artifacts/{index:03d}.stdout.txt').read_text(encoding='utf-8'))
    assert inventory['total_matches'] == len(inventory['functions'])
    report[f'{crate}_public_inventory'] = inventory
(out / 'static-extraction.json').write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding='utf-8')
print(json.dumps({'literal_count': len(literals), 'ipi_records': len(selected),
                  'tpi_theme_types': len(report['tpi_relevant_type_names'])}))
