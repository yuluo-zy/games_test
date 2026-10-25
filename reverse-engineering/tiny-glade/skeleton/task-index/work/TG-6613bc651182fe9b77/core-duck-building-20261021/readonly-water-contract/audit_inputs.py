from pathlib import Path
import hashlib, json, struct, gzip
ROOT=Path('/Users/yuluo/Desktop/game/games_test')
OUT=ROOT/'reverse-engineering/tiny-glade/skeleton/task-index/work/TG-6613bc651182fe9b77/core-duck-building-20261021/readonly-water-contract'
main=OUT.parent/'native-pack/cache-decorator-transforms-full.json'
peer=ROOT/'reverse-engineering/tiny-glade/skeleton/task-index/work/TG-10dd22b74858bd4c2e/decorator-chain-root-20261021/window-host-contract-v1/native-pack/canonical-cache-full.json'
d=json.loads(main.read_text()); e=json.loads(peer.read_text())
binary=Path(d['artifact']['path']); data=binary.read_bytes(); pe=struct.unpack_from('<I',data,0x3c)[0]; n=struct.unpack_from('<H',data,pe+6)[0]; optional_size=struct.unpack_from('<H',data,pe+20)[0]; sects=[]
for i in range(n):
 o=pe+24+optional_size+40*i; name=data[o:o+8].rstrip(b'\0').decode(); vs,va,rs,rp=struct.unpack_from('<IIII',data,o+8); sects.append((name,va,vs,rs,rp))
def read(va,n):
 rva=va-0x140000000
 for name,v,s,rs,rp in sects:
  if v<=rva and rva+n<=v+rs: return data[rp+rva-v:rp+rva-v+n]
 raise ValueError(hex(va))
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
pack_bytes=bytes.fromhex(''.join(i['bytes'] for i in d['instructions'])); peer_bytes=bytes.fromhex(''.join(i['bytes'] for i in e['instructions']))
body=read(0x1408153d0,9331)
records={}
for va,count in [(0x142960690,1),(0x14293e0c0,4),(0x142a8f9e0,2),(0x142a81460,4),(0x14295a0d8,1),(0x142950d70,1),(0x142a7fdf4,1),(0x142a8c6f0,4),(0x1429cd068,1),(0x142925870,1),(0x1429cd060,1),(0x14295a0dc,1),(0x14295a0e0,1),(0x1429ae340,1),(0x1429345b0,4),(0x142960674,1)]:
 b=read(va,count*4); vals=struct.unpack('<'+'f'*count,b)
 records[hex(va)]={'bytes':b.hex(),'u32_bits':[hex(x) for x in struct.unpack('<'+'I'*count,b)],'f32':list(vals)}
table=0x142a8ff00
jt=[hex(table+x) for x in struct.unpack('<iiii',read(table,16))]
source=ROOT/'tiny-glade/crates/systems/decorator/src/cache_decorator_transforms.rs'
inputs=[main,peer,main.with_suffix('.asm.txt'),source,ROOT/'tiny-glade/crates/systems/decorator/src/cache_roof_orientation.rs',ROOT/'tiny-glade/crates/systems/decorator/src/cache_terrain_support.rs',ROOT/'tiny-glade/crates/country-core/src/resources/walls/cached_decorator_transforms.rs',ROOT/'tiny-glade/crates/utils/src/curve/mod.rs']
index=ROOT/'reverse-engineering/tiny-glade/skeleton/_pdb/B/enrichment/procedure-index.json.gz'
with gzip.open(index,'rt') as f: idx=json.load(f)
if isinstance(idx,dict): print('index keys',list(idx)); idx=idx.get('procedures',idx.get('records',[]))
selected=[]
for rec in idx:
 if rec.get('rva') in (0x8153d0,0xc913e0,0x1269710,0xa1aeb0,0xa1ada0,0xa1b1e0,0xa1b200) or ('cache_decorator_transforms::cache_decorator_transforms' in rec.get('name','') and len(selected)<30): selected.append(rec)
out={'schema':1,'artifact':d['artifact'],'binary_sha_now':hashlib.sha256(data).hexdigest(),'identity_ok':hashlib.sha256(data).hexdigest()==d['artifact']['sha256'],'main_status':d['status'],'main_range':d['range'],'main_bytes_match_binary':pack_bytes==body,'peer_bytes_match':peer_bytes==body,'body_sha256':hashlib.sha256(body).hexdigest(),'constants':records,'kind_jump_table':{'va':hex(table),'bytes':read(table,16).hex(),'entries':jt,'mapping':{'other':jt[0],'2':jt[1],'3':jt[2],'4':jt[3]}},'pdb_records':selected,'inputs':[{'path':str(p),'sha256':sha(p)} for p in inputs]}
(OUT/'INPUT_BINDING_AND_CONSTANTS.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'identity_ok':out['identity_ok'],'native_match':out['main_bytes_match_binary'],'peer_match':out['peer_bytes_match'],'kind':out['kind_jump_table'],'constants':records,'pdb_records':selected},ensure_ascii=False,indent=2))
