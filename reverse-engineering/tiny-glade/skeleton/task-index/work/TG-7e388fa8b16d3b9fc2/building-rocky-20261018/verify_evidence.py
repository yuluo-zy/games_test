"""Read-only static PE/source verification. No original/JIT/Rust/Cargo execution."""
from pathlib import Path
import json, hashlib, struct, subprocess, tempfile, shutil, sys, gzip
import pefile, capstone
ROOT=Path('/Users/yuluo/Desktop/game'); P=Path(__file__).resolve().parent; E=P/'evidence'
def sha(path):
 h=hashlib.sha256()
 with path.open('rb') as f:
  for chunk in iter(lambda:f.read(1<<20),b''):h.update(chunk)
 return h.hexdigest()
RNE=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';PDB=RNE.with_name('tiny_glade.pdb')
assert sha(RNE)=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
assert sha(PDB)=='2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911'
pe=pefile.PE(str(RNE),fast_load=True); md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);md.detail=True
primary=json.loads((E/'native-ranges.json').read_text())['ranges'];fields=json.loads((E/'field-bindings/native-ranges.json').read_text())['ranges'];ranges=[(E,r) for r in primary]+[(E/'field-bindings',r) for r in fields];count=0;total=0;instructions={}
for folder,r in ranges:
 va=int(r['va'],16);b=(folder/f'{va:x}.bin').read_bytes();p=json.loads((folder/f'{va:x}.pdb.json').read_text())
 assert b==pe.get_data(va-0x140000000,r['bytes'])
 assert hashlib.sha256(b).hexdigest()==r['sha256']
 assert p['name']==r['name'] and p['code_size']==len(b)==r['bytes']
 decoded=list(md.disasm(b,va));assert sum(x.size for x in decoded)==len(b),(r['va'],'partial decode')
 expected='\n'.join(f'{i.address:#x} {i.bytes.hex()} {i.mnemonic} {i.op_str}' for i in decoded)+'\n'
 assert (folder/f'{va:x}.asm').read_text()==expected
 instructions.update({i.address:i for i in decoded});count+=len(decoded);total+=len(b)
def rip(site,expected,length,meaning):
 i=instructions[site];ops=[o for o in i.operands if o.type==capstone.x86.X86_OP_MEM and o.mem.base==capstone.x86.X86_REG_RIP];assert len(ops)==1
 va=i.address+i.size+ops[0].mem.disp;raw=pe.get_data(va-0x140000000,length);assert raw==expected
 return {'site':hex(site),'target':hex(va),'length':length,'hex':raw.hex(),'meaning':meaning}
data=[rip(0x141611c71,b'Rocky Terrain Vertices',22,'vertex buffer NAME'),rip(0x1416145a3,b'Rocky Terrain Triangles',23,'triangle buffer NAME'),rip(0x140927060,b'assets/data/rocky_terrain.json',30,'startup relative asset path'),rip(0x14176a423,b'country_core::startup::startup_terrain::startup_terrain',55,'actual plain function name'),rip(0x1413fb833,b'country_core::startup::init_color_library::init_color_library',61,'PreStartup candidate is color, not terrain')]
def call(site,target):
 i=instructions[site];assert i.mnemonic=='call' and i.operands[0].imm==target
for site,target in [(0x1408bc0f8,0x1412bc1f0),(0x1408bc241,0x1412bba70),(0x1412bc218,0x141611ba0),(0x1412bba94,0x1416144a0),(0x14092706c,0x1408bbf50),(0x1412d7ab4,0x14205e980),(0x140941130,0x140b67620),(0x14094117f,0x140b67b50),(0x140b67bda,0x14099b8b0),(0x14099baad,0x14176a370)]:call(site,target)
def imm(site,value):
 i=instructions[site];assert any(o.type==capstone.x86.X86_OP_IMM and o.imm&((1<<64)-1)==value for o in i.operands),(hex(site),i.op_str)
for site,value in [(0x1412bc2c7,0x1c5c4204bf6436bb),(0x1412bbb43,0x2a397285cdc5c9db),(0x141611c0b,0x6cfb10),(0x141611c2f,0x6cfae0),(0x141611c37,0x22),(0x141611c3d,1),(0x141611c49,4),(0x1416144f8,0x1837e0),(0x141614569,0x1837c0),(0x141614571,0x22),(0x141614577,1),(0x141614583,4),(0x14205e9cb,0x12146c4ea814069b),(0x14205e9d5,0xfa44538b95ae76c6)]:imm(site,value)
assert 'bevy_app::main_schedule::Startup' in json.loads((E/'140b67b50.pdb.json').read_text())['name']
assert 'bevy_app::main_schedule::PreStartup' in json.loads((E/'140b67620.pdb.json').read_text())['name']
assert 'RockyTerrainMeshVertCount' in json.loads((E/'14205e980.pdb.json').read_text())['name']

# The actual serde producer path and scalar types, without executing/restoring serde internals.
fieldcalls=[(0x1408bbfac,0x141405b00),(0x141405b50,0x1414212b0),(0x141410dde,0x140965980),(0x14141099e,0x140965af0),(0x140965a19,0x14140bd70),(0x140965b71,0x14140b6d0),(0x14140be77,0x14142c2a0),(0x14140b7d8,0x141428ea0),(0x14142c474,0x1417c3650),(0x14142c6ab,0x14140d270),(0x14142c675,0x14140d270),(0x1414291ac,0x14140d270),(0x14142c755,0x14140cfd0),(0x14142c71c,0x141413820),(0x14142c6e1,0x141413820),(0x14142916e,0x141413560)]
for site,target in fieldcalls:call(site,target)
A=json.load(gzip.open(ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B/enrichment/address-aliases.json.gz','rt'))
helpertypes={}
for va,needle in [(0x14140d270,'deserialize<i32,'),(0x14140cfd0,'deserialize<f32,'),(0x141413820,'array$<f32,3>'),(0x141413560,'array$<i32,3>')]:
 names=A[str(va-0x140000000)];assert len(names)==1 and needle in names[0]['name'];helpertypes[hex(va)]=names[0]
fieldtables=[]
for va,targets in [(0x142c5dbe8,[0x14142c497,0x14142c5f3,0x14142c545,0x14142c59c,0x14142c4ee,0x14142c64a]),(0x142ce251c,[0x1417c3672,0x1417c36b5,0x1417c3690,0x1417c36fb,0x1417c36fb,0x1417c36fb,0x1417c36fb,0x1417c36fb,0x1417c36db])]:
 raw=pe.get_data(va-0x140000000,4*len(targets));assert [va+struct.unpack_from('<i',raw,4*i)[0] for i in range(len(targets))]==targets
 fieldtables.append({'va':hex(va),'bytes':raw.hex(),'targets':list(map(hex,targets))})
for site,expected,length in [(0x141429054,b'neighbou',8),(0x14142905f,b'rs',2),(0x1414290c7,b'is_t',4),(0x1414290d1,b'op',2)]:data.append(rip(site,expected,length,'triangle key literal piece'))
fieldmap={'helpertypes':helpertypes,'switchtables':fieldtables,'typed_call_sites':[(hex(a),hex(b)) for a,b in fieldcalls],'field_binding_notes':'FIELD-BINDINGS.md'}
if '--save' in sys.argv:(E/'field-bindings/field-binding-map.json').write_text(json.dumps(fieldmap,indent=2)+'\n')

asset=json.loads((E/'actual-asset-schema-and-packed-fixtures.json').read_text());assert sha(Path(asset['artifact']))==asset['sha256']
source=json.loads((P/'source-map.json').read_text());assert sha(P/'base.rs')==source['base_sha256'];assert sha(P/'candidate.rs')==source['candidate_sha256']
# Captured base mirror prevents unrelated concurrent source edits from changing this check.
with tempfile.TemporaryDirectory(prefix='rocky-apply-',dir='/private/tmp') as temp:
 dst=Path(temp)/'games_test/tiny-glade/crates/country-core/src/resources/rocky_terrain.rs';dst.parent.mkdir(parents=True);shutil.copyfile(P/'base.rs',dst)
 checked=subprocess.run(['git','apply','--check',str(P/'candidate.patch')],cwd=temp,capture_output=True,text=True);assert checked.returncode==0,checked.stderr
 applied=subprocess.run(['git','apply',str(P/'candidate.patch')],cwd=temp,capture_output=True,text=True);assert applied.returncode==0,applied.stderr
 assert dst.read_bytes()==(P/'candidate.rs').read_bytes()
fmt=subprocess.run(['rustfmt','--edition','2024','--check',str(P/'candidate.rs')],capture_output=True,text=True);assert fmt.returncode==0,fmt.stderr
result={'target':'B','native_execution':False,'cargo_execution':False,'production_changed':False,'range_count':len(ranges),'primary_ranges':len(primary),'schema_dependency_ranges':len(fields),'raw_bytes':total,'decoded_instructions':count,'all_ranges_equal_PE':True,'all_ranges_complete_PDB_size':True,'ASM_equal_full_decode':True,'data_refs':data,'typed_calls_checked':10+len(fieldcalls),'immediate_constants_checked':14,'actual_asset_sha256_checked':True,'asset_all_row_pack':'see executed inspect_actual_asset.py/result (not native execution)','patch_applies_to_captured_base_and_equals_candidate':True,'rustfmt_check':0,'rust_tests':'five written, not executed; parent owns compilation','current_production_equals_base':Path(source['path']).read_bytes()==(P/'base.rs').read_bytes()}
if '--save' in sys.argv:(E/'static-verification.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
