from pathlib import Path
import json,hashlib,pefile,capstone
R=Path('/Users/yuluo/Desktop/game');P=Path(__file__).resolve().parent;W=R/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-9ae382a5028cc58515/terrain-curve-20261018';M=W/'continuation-frozen-inputs.json';assert hashlib.sha256(M.read_bytes()).hexdigest()=='1d8528f5280fb9811af1b4d807c181da5ae2c0321840b2ef151d44d8244ff827';x=json.loads(M.read_text())
for r in x['files']:
 b=Path(r['path']).read_bytes();assert len(b)==r['bytes'];assert hashlib.sha256(b).hexdigest()==r['sha256']
F=R/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-c4a5b4adcb068a950d/terrain-curve-20261018/continuation-v2/native-pack/suppress-continue-wall.json';a=json.loads(F.read_text());pe=pefile.PE(str(R/'ljxsj_92385/Tiny Glade/tiny-glade.rne'),fast_load=True);md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);n=0
for i in a['instructions']:
 b=bytes.fromhex(i['bytes']);va=int(i['va'],16);assert pe.get_data(va-0x140000000,len(b))==b;ins=list(md.disasm(b,va));assert len(ins)==1 and ins[0].size==len(b);n+=len(b)
assert n==375
thresholds=[]
for va,expected in [(0x142925904,'0000803f'),(0x1429cd070,'00004842')]:
 b=pe.get_data(va-0x140000000,4);assert b.hex()==expected;thresholds.append({'va':hex(va),'bytes':b.hex()})
(P/'verification.json').write_text(json.dumps({'status':'passed','frozen_inputs':29,'scope':'suppression body only; other continuation functions not reviewed here','native_bytes':n,'native_instructions':len(a['instructions']),'thresholds':thresholds,'original_executed':False,'cargo_executed':False,'production_changed':False},indent=2)+'\n');print(n,len(a['instructions']))
