from pathlib import Path
import runpy,io,contextlib,struct,json,re,subprocess
O=Path(__file__).resolve().parent
with contextlib.redirect_stdout(io.StringIO()):v=runpy.run_path(str(O/'native_extract.py'))
read,procs=v['read'],v['procs'];tables=[]
for a in [0x142babd60,0x142babe18,0x142bac528]:
 entries=[]
 for off in range(0,0xb8,8):
  p=struct.unpack('<Q',read(a+off,8))[0];symbols=procs.get(hex(p),[])
  if symbols:entries.append({'offset':hex(off),'va':hex(p),'pdb':symbols})
 tables.append({'va':hex(a),'entries':entries})
(O/'initial-vtables.json').write_text(json.dumps(tables,indent=2),encoding='utf8')
for t in tables:
 print(t['va'])
 for e in t['entries']: print(e['offset'],e['va'],e['pdb'][0]['symbol'][:180])
