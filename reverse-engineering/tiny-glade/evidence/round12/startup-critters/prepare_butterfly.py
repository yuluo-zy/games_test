from pathlib import Path
import contextlib,io,runpy,json,hashlib,subprocess
O=Path(__file__).resolve().parent
with contextlib.redirect_stdout(io.StringIO()):v=runpy.run_path(str(O/'native_extract.py'))
targets=[]
for a in [0x141d84d40,0x141d86060,0x141d867b0,0x141d86eb0,0x140f7fc80,0x140f7cf20,0x140f7cb50,0x141db2d10]:
 n=v['procs'][hex(a)][0]['size'];targets.append({'address':hex(a),'size':n,'bytes_sha256':hashlib.sha256(v['read'](a,n)).hexdigest(),'range_basis':'exact PDB author range; original RNE SHA checked'})
 (O/f'{a:x}.original.asm').write_bytes(subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={a:#x}',f'--stop-address={a+n:#x}',str(v['X'])],check=True,stdout=subprocess.PIPE).stdout)
(O/'butterfly-job.json').write_text(json.dumps([{'tool':'prepare_functions','args':{'program':'tiny-glade.exe','targets':targets}},{'tool':'get_functions','args':{'program':'tiny-glade.exe','functions':','.join(t['address'] for t in targets),'fields':'signature,decompiled_code,callees'}}]),encoding='utf8')
print('prepared8 bounded original butterfly/config ranges')
