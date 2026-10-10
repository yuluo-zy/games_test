"""从原作匹配 PDB 提取 Roof 方法的精确过程范围，不扫描其他工程。"""
from pathlib import Path
import subprocess, re, json

root = Path('D:/game/reverse-engineering/tiny-glade')
out = root / 'evidence/pdb-structure'
pdb = Path('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb')
llvm = 'C:/Program Files/LLVM/bin/llvm-pdbutil.exe'
base = 0x140001000
addresses = [0x1408e31c0, 0x1408e3250, 0x1408e33b0]
result = subprocess.run([llvm, 'dump', '--section-contribs', str(pdb)], check=True, capture_output=True, text=True, encoding='utf-8')
contributions = []
for line in result.stdout.splitlines():
    m = re.search(r'mod = (\d+), (\d+):(\d+), size = (\d+)', line)
    if not m:
        continue
    mod, sec, offset, size = map(int, m.groups())
    if sec == 1 and any(base + offset <= address < base + offset + size for address in addresses):
        contributions.append(dict(module=mod, section=sec, offset=offset, size=size, raw=line))
(out / 'roof-method-contributions.json').write_text(json.dumps(contributions, indent=2), encoding='utf-8')
data = json.loads((out / 'procedures.json').read_text(encoding='utf-8'))
found = []
for mod in sorted({c['module'] for c in contributions}):
    result = subprocess.run([llvm, 'dump', '--symbols', f'--modi={mod}', str(pdb)], check=True, capture_output=True, text=True, encoding='utf-8')
    file = out / f'roof-methods-module{mod}.txt'
    file.write_text(result.stdout, encoding='utf-8')
    lines = result.stdout.splitlines()
    for i, line in enumerate(lines):
        if 'S_GPROC32 ' not in line and 'S_LPROC32 ' not in line:
            continue
        name = re.search(r'`([^`]+)`', line)
        address = re.search(r'addr = (\d+):(\d+), code size = (\d+)', lines[i + 1])
        if not name or not address:
            continue
        sec, off, size = map(int, address.groups())
        if sec == 1 and base + off in addresses:
            found.append(dict(name=name.group(1), section=sec, section_offset=off, va=hex(base+off), code_size=size,
                              end_exclusive=hex(base+off+size), evidence=str(file), line=i+1, pdb_type_record=lines[i+2].strip()))
data['procedures'] = list({(p['name'], p['va']): p for p in data['procedures'] + found}.values())
(out / 'procedures.json').write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding='utf-8')
print(json.dumps(found, ensure_ascii=False, indent=2))
