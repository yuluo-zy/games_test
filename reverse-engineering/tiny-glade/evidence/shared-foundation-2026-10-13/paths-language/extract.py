import subprocess,re,json,pathlib
out=pathlib.Path(__file__).parent
pdb='D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb'
mods=pathlib.Path('D:/game/reverse-engineering/tiny-glade/evidence/binary/pdb-modules.txt').read_text()
ids=[int(m.group(1)) for m in re.finditer(r'Mod (\d+) \| `(?:paths-|lang_lib-|theme_defs-)[^`]+`',mods)]
procedures=[]
for i in ids:
 t=subprocess.check_output(['llvm-pdbutil','dump','--symbols','-l',f'--modi={i}',pdb],encoding='utf-8',errors='replace')
 (out/f'module-{i}.txt').write_text(t,encoding='utf-8')
 for m in re.finditer(r'S_[GL]PROC32[^\n]+`([^`]+)`\s+parent[^\n]+addr = (\d+):(\d+), code size = (\d+)',t):
  name,sec,off,size=m.groups(); procedures.append(dict(name=name,section=int(sec),va=hex(0x140001000+int(off)) if sec=='0001' else None,size=int(size),module=i))
(out/'procedures.json').write_text(json.dumps(procedures,indent=2),encoding='utf-8')
print(json.dumps([x for x in procedures if any(x['name'].startswith(y+'::') for y in ['paths','lang_lib','theme_defs'])],indent=2))
