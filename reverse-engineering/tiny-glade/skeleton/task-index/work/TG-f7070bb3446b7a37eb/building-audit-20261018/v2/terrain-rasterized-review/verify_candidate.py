from pathlib import Path
import json,hashlib,tempfile,subprocess,gzip,capstone,pefile
ROOT=Path('/Users/yuluo/Desktop/game');P=Path(__file__).resolve().parent;W=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-e73d16b0f4be6e76bf/core-terrain-render-20261018';M=W/'frozen-inputs.json';assert hashlib.sha256(M.read_bytes()).hexdigest()=='ef73953ac96aae1927af4370db167393685974e516146534c5da3641e0905ddd';x=json.loads(M.read_text())
for r in x['inputs']:
 b=Path(r['path']).read_bytes();assert len(b)==r['bytes'];assert hashlib.sha256(b).hexdigest()==r['sha256']
rne=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';assert hashlib.sha256(rne.read_bytes()).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03';pe=pefile.PE(str(rne),fast_load=True)
for r in x['original_ranges']:
 b=pe.get_data(int(r['va'],16)-0x140000000,r['size']);assert b.hex()==r['bytes'];assert hashlib.sha256(b).hexdigest()==r['sha256']
a=json.load(gzip.open(ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B/enrichment/address-aliases.json.gz','rt'));aliases=a[str(0x1532740)];assert any('insert_resource<country_core::resources::terrain_editing::TerrainEditingData>' in r['name'] for r in aliases)
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);ident=[]
for va,n in [(0x14153278a,27),(0x1408c71c2,35),(0x140940393,53),(0x1408c72a8,27),(0x1408c895b,11)]:
 b=pe.get_data(va-0x140000000,n);rows=[{'va':hex(i.address),'bytes':i.bytes.hex(),'asm':i.mnemonic+' '+i.op_str} for i in md.disasm(b,va)];ident.append({'va':hex(va),'bytes_hex':b.hex(),'instructions':rows})
(P/'resource-identity.json').write_text(json.dumps({'insert_resource_aliases':aliases,'ranges':ident},indent=2)+'\n')
with tempfile.TemporaryDirectory(prefix='tg-terrain-flag-review-') as d:
 t=Path(d)
 for f in (W/'base').rglob('*.rs'):
  out=t/'games_test/tiny-glade'/f.relative_to(W/'base');out.parent.mkdir(parents=True,exist_ok=True);out.write_bytes(f.read_bytes())
 for check in [True,False]:
  r=subprocess.run(['git','apply']+(['--check'] if check else [])+[str(W/'candidate.patch')],cwd=t,capture_output=True,text=True);assert r.returncode==0,r.stderr
 for f in (W/'candidate').rglob('*.rs'):assert (t/'games_test/tiny-glade'/f.relative_to(W/'candidate')).read_bytes()==f.read_bytes()
(P/'verification.json').write_text(json.dumps({'status':'passed','frozen_inputs':15,'exact_PE_ranges':4,'same_batch_apply_and_candidate_equality':3,'original_executed':False,'cargo_executed':False,'production_changed':False},indent=2)+'\n')
for r in ident[:2]: print(r['va'],r['instructions'][:3])
print('passed')
