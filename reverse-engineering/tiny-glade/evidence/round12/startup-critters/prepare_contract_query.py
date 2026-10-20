from pathlib import Path
import contextlib,io,runpy,json,hashlib,subprocess
O=Path(__file__).resolve().parent
with contextlib.redirect_stdout(io.StringIO()):v=runpy.run_path(str(O/'native_extract.py'))
targets=[]
for a,n in [(0x14067d440,966),(0x140f7d430,1450),(0x140f7e4e0,3623),(0x140f801d0,954),(0x140f7d9e0,971),
 (0x140b22db0,1608),(0x1420194d0,2338),(0x14201a0d0,1984),(0x140c909a0,154),(0x140c90700,104)]:
 targets.append({'address':hex(a),'size':n,'bytes_sha256':hashlib.sha256(v['read'](a,n)).hexdigest(),'range_basis':'Exact PDB S_GPROC32 procs.tsv author range; original RNE hash checked'})
 p=O/f'{a:x}.original.asm';p.write_bytes(subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={a:#x}',f'--stop-address={a+n:#x}',str(v['X'])],check=True,stdout=subprocess.PIPE).stdout)
job=[{'tool':'prepare_functions','args':{'program':'tiny-glade.exe','targets':targets}},{'tool':'get_functions','args':{'program':'tiny-glade.exe','functions':','.join(t['address'] for t in targets),'fields':'signature,decompiled_code,callees'}}]
(O/'contract-job.json').write_text(json.dumps(job),encoding='utf8');print('prepared10 bounded functions')
