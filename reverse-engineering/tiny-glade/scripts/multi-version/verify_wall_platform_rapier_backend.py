"""Source cell -> whole original indexed mesh/Parry query + real backend lifecycle."""
import sys,json,struct,subprocess,random,hashlib,math
from verification_provenance import ROOT
sys.path.insert(0,str(ROOT/'evidence/building-platform-stairs'))
sys.path.insert(0,str(ROOT/'evidence/building-segmentation'))
from research_native_mesh_query import query,full
from native_platform_geometry import produce
from native_fixture import b

def f(v):return struct.unpack('<f',struct.pack('<f',v))[0]
def word(v):return f'{struct.unpack("<I",struct.pack("<f",v))[0]:08x}'
def main():
 lines=[];expected=[];details=[]
 for i in range(64):
  angle=[0.,math.pi/2,.3,-.7][i%4];x=f(math.cos(angle));z=f(math.sin(angle));length=[1.,2.,4.,6.][i%4];a=[f(3.*(i%3)),0.,f(-2.*(i%5))];points=[a,[f(a[0]+x*length),0.,f(a[2]+z*length)]];height=[1.6,2.5,3.5,6.][i%4];extra=[.5,.8,1.6][i%3];wall=27+i
  cell=produce(points,[(0,0.),(0,1.)],height,extra,[0]*262144,0.,3.,False);vertices=list(struct.iter_unpack('<3f',bytes.fromhex(cell['vertices'])));triangles=list(struct.iter_unpack('<3I',bytes.fromhex(cell['indices'])));owner=full(vertices,triangles);fingerprint=struct.unpack_from('<Q',owner,0x70)[0]
  center=[0.,0.,0.]
  for p in vertices:
   for j in range(3):center[j]=f(center[j]+f(p[j]/len(vertices)))
  hit,toi,entry=query(vertices,triangles,[center[0],f(height+10.),center[2]],[0.,-1.,0.]);assert hit
  expected.append(f'{fingerprint:016x} 0,1,0,1 4 {toi}');lines.append(f'{wall} {word(height)} {word(extra)} '+' '.join(word(v)for p in points for v in p));details.append({'case':i,'entry':hex(entry),'original_toi_bits':hex(toi)})
 probe=ROOT/'reconstruction/target/release/wall-platform-rapier-probe';run=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True);assert run.returncode==0,run.stderr;actual=run.stdout.splitlines();assert len(actual)==len(expected);failures=[{'case':i,'input':lines[i],'original':w,'rust':g}for i,(w,g)in enumerate(zip(expected,actual))if w!=g]
 files=['reconstruction/building/wall_platform_collision.rs','reconstruction/building/wall_platform_rapier_backend.rs','reconstruction/src/bin/wall-platform-rapier-probe.rs','reconstruction/building/wall_platform_stairs.rs','reconstruction/building/wall_collection.rs','reconstruction/building/wall_state.rs','reconstruction/Cargo.toml','reconstruction/Cargo.lock'];oracles=['scripts/multi-version/verify_wall_platform_rapier_backend.py','evidence/building-platform-stairs/research_native_mesh_query.py','evidence/building-platform-stairs/research_native_indexed_mesh.py','evidence/building-platform-stairs/native_platform_collision_hash.py','evidence/building-segmentation/native_platform_geometry.py','evidence/building-constructor/native_fixture.py'];digest=lambda xs:{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in xs}
 report={'ok':not failures,'cases':len(lines),'failures':failures,'native_meshes':len(lines),'operational_insert_keep_replace_remove_sequences':len(lines),'library_ray_queries':len(lines)*3,'rasterized_aabb_callbacks':len(lines)*4,'scope':'Actual source prepared endpoints/public scalars/real512 zero encodedterrain -> original cell geometry -> whole original RaycastMeshData::new_indexed including ParrySharedShape::trimesh -> actual original vtable cast_local_ray (option inAL,TOI XMM0). Candidate actual lockedRapier/Parry/Slotmap backend constructs indexed sensor mesh, checks native fingerprint, realquery TOI bitexact. Candidate inserts, keeps samehandle without diff/raster, replaces with generational oldkeys invalidated, realquery still hits, removes and verifies actualquery miss, duplicate remove no-op; original delta sequence0,1,0,1 and four real AABB sinkcalls asserted. Backend query scope is isolated platform subworld7; original all-subworld gamefilters/ECSWorld acquisition and DirtyTiles cell rasterization are not proved, caller raster sink is explicit and required. No geometry output injection in candidate. RaycastWorld whole original insertion/removal calls are not executed: business action policy separately original-verified and real dependency lifecycle checked here.','binary_sha256':b['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'candidate_sources':digest(files),'oracle_sources':digest(oracles),'details':details}
 name='B-rapier-backend-verification.json'if not failures else'B-rapier-backend-first-failure.json';(ROOT/'evidence/building-platform-stairs'/name).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(lines),'failures':len(failures),'first':failures[:2]}));assert not failures
if __name__=='__main__':main()
