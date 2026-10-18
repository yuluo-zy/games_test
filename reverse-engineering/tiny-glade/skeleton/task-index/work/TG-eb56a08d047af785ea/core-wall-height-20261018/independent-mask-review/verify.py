from pathlib import Path
import json,hashlib,tempfile,subprocess,shutil,sys
import pefile,capstone
ROOT=Path('/Users/yuluo/Desktop/game')
W=Path(__file__).resolve().parent
S=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-094b63a2d2e8f1aec0/building-masks-20261018'
manifest=S/'frozen-inputs.json';m=json.loads(manifest.read_text());checks=[]
for row in m['inputs']:
 p=Path(row['path']);b=p.read_bytes();assert hashlib.sha256(b).hexdigest()==row['sha256'];assert len(b)==row['size'];checks.append({'path':str(p),'sha256':row['sha256'],'size':len(b),'match':True})
p=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';assert hashlib.sha256(p.read_bytes()).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(str(p),fast_load=True);md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);e=json.loads((S/'evidence/native-inputs.json').read_text());native=[]
for f in e['functions']:
 a=int(f['entry'],16);blob=pe.get_data(a-0x140000000,f['length']);assert hashlib.sha256(blob).hexdigest()==f['sha256'];assert blob==(S/'evidence'/f'{a:x}.bin').read_bytes()
 generated=[f'{i.address:x}: {i.mnemonic} {i.op_str}'.strip() for i in md.disasm(blob,a)];retained=[x.rstrip() for x in (S/'evidence'/f'{a:x}.asm').read_text().splitlines()];assert generated==retained,(f['entry'],len(generated),len(retained));assert len(generated)==f['instructions']
 native.append({'entry':hex(a),'bytes':len(blob),'instructions':len(generated),'all_bytes_and_assembly_match':True})
keys=[]
for address,key in [(0x1412c2eaa,0x4652455297dab420),(0x1412c182a,0x8dbc70d368e43c84),(0x1412c56da,0x6fd8113d23595a99)]:
 raw=pe.get_data(address-0x140000000,10);assert raw[:2]==bytes.fromhex('48ba');assert int.from_bytes(raw[2:10],'little')==key
 keys.append({'site':hex(address),'key':hex(key),'immediate_matches':True})
names=[]
for n in e['names']:
 site=int(n['site'],16);target=int(n['target'],16);raw=pe.get_data(site-0x140000000,len(bytes.fromhex(n['bytes'])));assert raw.hex()==n['bytes']
 # All retained NAME sites are x86-64 LEA r8,[rip+disp32].
 assert raw[:3]==bytes.fromhex('4c8d05');assert site+7+int.from_bytes(raw[3:7],'little',signed=True)==target
 text=pe.get_data(target-0x140000000,n['length']).decode();assert text==n['name'];names.append({'site':hex(site),'target':hex(target),'name':text,'rip_and_string_match':True})
with tempfile.TemporaryDirectory(prefix='tg-mask-review-') as t:
 td=Path(t)
 for claim in ['TG-094b63a2d2e8f1aec0','TG-a0fb89a9f7cc9770b9','TG-91fe8570b8ea4df799']:
  cw=S.parent.parent/claim/'building-masks-20261018';base=json.loads((cw/'source-base.json').read_text());relative=Path(base['path']).relative_to(ROOT);out=td/relative;out.parent.mkdir(parents=True,exist_ok=True);shutil.copy(cw/'base.rs',out)
 r=subprocess.run(['git','apply','--check',str(S/'owned-files.patch')],cwd=td,capture_output=True,text=True);assert r.returncode==0,r.stderr
current=[]
for claim in ['TG-094b63a2d2e8f1aec0','TG-a0fb89a9f7cc9770b9','TG-91fe8570b8ea4df799']:
 cw=S.parent.parent/claim/'building-masks-20261018';base=json.loads((cw/'source-base.json').read_text());path=Path(base['path']);candidate=(cw/'candidate.rs').read_bytes();actual=path.read_bytes();current.append({'path':str(path),'sha256':hashlib.sha256(actual).hexdigest(),'matches_candidate':actual==candidate})
report={'target':'B','manifest_sha256':hashlib.sha256(manifest.read_bytes()).hexdigest(),'frozen_inputs':checks,'native_functions':native,'names':names,'add_new_keys':keys,'native_bytes':sum(x['bytes'] for x in native),'native_instructions':sum(x['instructions'] for x in native),'combined_patch_against_captured_base':'passed','production_read_snapshot':current,'compiled':False,'native_executed':False}
(W/'verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'hash_inputs':len(checks),'native_bytes':report['native_bytes'],'native_instructions':report['native_instructions'],'patch_check':True,'production_candidates_match':all(x['matches_candidate'] for x in current)}))
