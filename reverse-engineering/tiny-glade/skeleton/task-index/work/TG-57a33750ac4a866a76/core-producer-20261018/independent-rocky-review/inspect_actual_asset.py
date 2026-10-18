from pathlib import Path
import json,struct,hashlib,math
ROOT=Path('/Users/yuluo/Desktop/game');P=Path(__file__).resolve().parent;A=ROOT/'ljxsj_92385/Tiny Glade/assets/data/rocky_terrain.json';raw=A.read_bytes();assert hashlib.sha256(raw).hexdigest()=='0f63a7250df5a3382017dbdc3652748f33899750e3a65c5aef3727415b9183b7';j=json.loads(raw);assert set(j)=={'vertices','triangles'}
vhash=hashlib.sha256();thash=hashlib.sha256();selected={};tri_selected={}
for i,v in enumerate(j['vertices']):
 assert set(v)=={'position','cell_bby','cell_id','dir_to_centroid','is_corner'}
 assert len(v['position'])==len(v['dir_to_centroid'])==3
 assert type(v['cell_id']) is int and 0<=v['cell_id']<=0xffffffff
 assert type(v['is_corner']) is int and 0<=v['is_corner']<=0xffffffff
 for f in [*v['position'],v['cell_bby'],*v['dir_to_centroid']]:assert isinstance(f,(int,float)) and math.isfinite(f)
 words=[*[struct.unpack('<I',struct.pack('<f',f))[0] for f in v['position']],struct.unpack('<I',struct.pack('<f',v['cell_bby']))[0],*[struct.unpack('<I',struct.pack('<f',f))[0] for f in v['position']],v['cell_id'],*[struct.unpack('<I',struct.pack('<f',f))[0] for f in v['dir_to_centroid']],v['is_corner']]
 vhash.update(struct.pack('<12I',*words))
 if i in [0,1,len(j['vertices'])//2,len(j['vertices'])-1]:selected[str(i)]={'source':v,'gpu_words':words}
for i,t in enumerate(j['triangles']):
 assert set(t)=={'neighbours','is_top'} and len(t['neighbours'])==3
 assert all(type(n) is int and -(1<<31)<=n<1<<31 for n in t['neighbours'])
 assert type(t['is_top']) is int and 0<=t['is_top']<=0xffffffff
 words=[*[n&0xffffffff for n in t['neighbours']],t['is_top'],0,0,0,0];thash.update(struct.pack('<8I',*words))
 if i in [0,1,len(j['triangles'])//2,len(j['triangles'])-1]:tri_selected[str(i)]={'source':t,'gpu_words':words}
negative=next((i,t) for i,t in enumerate(j['triangles']) if any(n<0 for n in t['neighbours']))
i,t=negative;tri_selected[str(i)]={'source':t,'gpu_words':[n&0xffffffff for n in t['neighbours']]+[t['is_top'],0,0,0,0]}
report={'artifact':str(A),'sha256':hashlib.sha256(raw).hexdigest(),'bytes':len(raw),'vertices':len(j['vertices']),'triangles':len(j['triangles']),'vertices_packed_bytes':48*len(j['vertices']),'triangles_packed_bytes':32*len(j['triangles']),'vertices_packed_sha256':vhash.hexdigest(),'triangles_packed_sha256':thash.hexdigest(),'selected_vertices':selected,'selected_triangles':tri_selected,'validation':'Python actual asset schema/all-row f32 pack; not original or Rust execution'}
(P/'actual-asset-schema-and-packed-fixtures.json').write_text(json.dumps(report,indent=2)+'\n');print({k:v for k,v in report.items() if not k.startswith('selected')})
