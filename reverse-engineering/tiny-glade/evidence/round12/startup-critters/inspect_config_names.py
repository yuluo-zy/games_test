from pathlib import Path
import contextlib,io,runpy,re,json,subprocess
O=Path(__file__).resolve().parent
with contextlib.redirect_stdout(io.StringIO()):v=runpy.run_path(str(O/'native_extract.py'))
sources=[0x140f7d430,0x140f7e4e0,0x140f801d0,0x140f7d9e0,0x14067d810]
constructors=[0x141da6cd0,0x141da76b0,0x141da88b0,0x140f7fc80,0x141d8fa10,0x141da95c0,0x141dabd90,0x140f7cf20,0x141dabbe0,0x141dac8f0,0x141dabf40,0x141dad2e0,0x141dadcc0,0x140f7cb50,0x141dae6b0,0x141db2d10,0x141db5920]
rows=[]
for a in sources+constructors:
 n=v['procs'][hex(a)][0]['size']
 asm=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={a:#x}',f'--stop-address={a+n:#x}',str(v['X'])],check=True,stdout=subprocess.PIPE).stdout.decode('utf8')
 (O/f'{a:x}.original.asm').write_text(asm,encoding='utf8')
 refs=[]
 for line in asm.splitlines():
  m=re.search(r'#\s*(0x[0-9a-f]+)',line)
  if not m:continue
  try:b=v['read'](int(m[1],16),192)
  except ValueError:continue
  if b.startswith(b'system_critters::'):
   text=re.match(rb'[A-Za-z0-9_:]+',b)[0].decode()
   refs.append({'instruction':line.strip(),'candidate_ascii_prefix':text,'raw192':b.hex()})
 rows.append({'constructor_va':hex(a),'pdb':v['procs'][hex(a)],'name_references':refs})
(O/'plugin-config-name-evidence.json').write_text(json.dumps(rows,indent=2),encoding='utf8')
for r in rows:print(r['constructor_va'],[(s['instruction'],s['candidate_ascii_prefix']) for s in r['name_references']])
