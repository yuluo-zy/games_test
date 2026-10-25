from pathlib import Path
import json,hashlib,subprocess,pefile,capstone
ROOT=Path('/Users/yuluo/Desktop/game');W=Path(__file__).resolve().parent
S=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-9ae382a5028cc58515/terrain-curve-20261018';manifest=S/'continuation-frozen-inputs.json';m=json.loads(manifest.read_text())
rows=[];packs=[]
p=pefile.PE(str(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'),fast_load=True)
for f in m['files']:
 q=Path(f['path']);b=q.read_bytes();assert hashlib.sha256(b).hexdigest()==f['sha256'];assert len(b)==f['bytes'];rows.append({'path':str(q),'sha256':f['sha256'],'size':len(b),'match':True})
 if q.suffix=='.json':
  x=json.loads(b)
  if isinstance(x,dict) and 'instructions' in x and 'range' in x:
   size=0
   for i in x['instructions']:
    a=int(i['va'],16);raw=bytes.fromhex(i['bytes']);assert raw==p.get_data(a-0x140000000,len(raw));assert len(raw)==i['size'];size+=len(raw)
   packs.append({'path':str(q),'instructions':len(x['instructions']),'decoded_bytes':size,'all_original_bytes_match':True,'range':x['range']})
r=subprocess.run(['git','apply','--check',str(S/'continuation-candidate.patch')],cwd=ROOT,capture_output=True,text=True)
# Retain the separately inspected dependency leaf without executing it.
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);a=0x141469a20;raw=p.get_data(a-0x140000000,0xc2);leaf=[{'va':hex(i.address),'bytes':i.bytes.hex(),'mnemonic':i.mnemonic,'operands':i.op_str} for i in md.disasm(raw,a)]
(W/'projection-finite-leaf.json').write_text(json.dumps({'target':'B','entry':hex(a),'length':len(raw),'sha256':hashlib.sha256(raw).hexdigest(),'instructions':leaf,'evidence':'1469a77 ucomiss self/jp;1469a85 exponent Infinity test before successful ret1469a92'},indent=2)+'\n')
report={'target':'B','upstream_manifest_sha256':hashlib.sha256(manifest.read_bytes()).hexdigest(),'frozen_files':rows,'native_spans':packs,'native_instructions':sum(x['instructions'] for x in packs),'native_decoded_bytes':sum(x['decoded_bytes'] for x in packs),'patch_current_base_apply_check':{'returncode':r.returncode,'stderr':r.stderr},'open_finding':'invalid false removes authority before color lookup in native; candidate29 reverses this order; awaiting minimal delta','compiled':False,'native_executed':False}
(W/'verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'hash_files':len(rows),'native_spans':len(packs),'native_instructions':report['native_instructions'],'native_bytes':report['native_decoded_bytes'],'patch_check':r.returncode}))
