import re,json,subprocess,pathlib
out=pathlib.Path(__file__).parent
mods=pathlib.Path('D:/game/reverse-engineering/tiny-glade/evidence/binary/pdb-modules.txt').read_text()
ids=[int(x) for x in re.findall(r'Mod (\d+) \| `theme_fetcher-[^`]+`',mods)]
found=[]
for i in ids:
 t=subprocess.check_output(['llvm-pdbutil','dump','--symbols',f'--modi={i}','D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb'],encoding='utf-8',errors='replace')
 if 'theme_defs::' in t or re.search(r'S_[GL]PROC32[^\n]+`theme_fetcher::',t):
  (out/f'theme-consumer-{i}.txt').write_text(t,encoding='utf-8')
 for m in re.finditer(r'S_[GL]PROC32[^\n]+`([^`]+)`\s+parent[^\n]+addr = (\d+):(\d+), code size = (\d+)',t):
  name,sec,off,size=m.groups()
  if any(x in name for x in ['theme_defs::','theme_fetcher::','theme_server_proto::']):found.append(dict(name=name,va=hex(0x140001000+int(off)) if sec=='0001' else None,size=int(size),module=i))
(out/'theme-procedures.json').write_text(json.dumps(found,indent=2),encoding='utf-8')
print('modules',len(ids),'filtered functions',len(found))
for f in found:
 if f['name'].startswith('theme_fetcher::'):print(f)
