import pathlib, subprocess, re, json
p = pathlib.Path(__file__).parent
exe = r'C:\Program Files\LLVM\bin\llvm-pdbutil.exe'
pdb = r'D:\game\ljxsj_92385\Tiny Glade\tiny_glade.pdb'
s = subprocess.check_output([exe, 'dump', '-modules', pdb], text=True, encoding='utf8')
mods = [int(n) for n in re.findall(r'Mod (\d+) \| `(?:asset-|asset_pipe-)', s)]
(p/'modules.json').write_text(json.dumps(mods))
out = []
nr = re.compile(r'^\s*(\d+) \| S_(?:GPROC32|LPROC32)(?:_ID)? \[[^\]]+\] `([^`]+)`')
ar = re.compile(r'addr = (\d+):(\d+), code size = (\d+)')
for mod in mods:
    t = subprocess.check_output([exe, 'dump', '-symbols', '-modi='+str(mod), pdb], text=True, encoding='utf8')
    (p/f'module-{mod}.txt').write_text(t, encoding='utf8')
    pending = None
    for line in t.splitlines():
        m = nr.match(line)
        if m:
            pending = {'module': mod, 'name': m[2]}
        elif pending and (a := ar.search(line)):
            pending.update(section=int(a[1]), va=hex(0x140001000+int(a[2])), size=int(a[3]))
            out.append(pending)
            pending = None
(p/'functions.json').write_text(json.dumps(out, indent=2))
print('modules', len(mods), 'procedures', len(out))
print('\n'.join(r['va']+' '+str(r['size'])+' '+r['name'] for r in out if r['name'].startswith(('asset::', 'asset_pipe::'))))
