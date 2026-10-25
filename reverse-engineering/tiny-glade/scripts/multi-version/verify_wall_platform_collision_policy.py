"""Original upsert pose gate and whole tag7 mesh comparison."""
import sys,struct,json,random,subprocess,hashlib
from verification_provenance import ROOT
sys.path.insert(0,str(ROOT/'evidence/building-platform-stairs'))
from native_platform_collision_policy import original_action,b,cpu,original_mesh_eq

def main():
 rng=random.Random(20261105);lines=[];expected=[];counts={};identity=[0,0,0,0,0,0,0x3f800000];pool=[0,0x80000000,0x3f800000,0x7fc00001,0x7f800000,0xff800000]
 for i in range(512):
  exists=i%11!=0;valid=i%7!=0;old=identity.copy()if i%3 else[rng.choice(pool)for _ in range(7)];new=old.copy();old_wall=rng.getrandbits(64);new_wall=old_wall if i%13 else rng.getrandbits(64);a=rng.getrandbits(64);z=a if i%4 else rng.getrandbits(64)
  if i%5==0:new[i%7]=rng.choice(pool)
  if i%17==0:old[:3]=[0x80000000]*3;new[:3]=[0]*3
  out=original_action(exists,valid,a,z,old,new,old_wall,new_wall);expected.append(str(out));counts[out]=counts.get(out,0)+1
  lines.append(f'policy {int(exists)} {int(valid)} {a:x} {z:x} {old_wall} {new_wall} '+' '.join(f'{v:08x}'for p in [old,new]for v in p))
 probe=ROOT/'reconstruction/target/release/wall-platform-collider-probe';run=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True);assert run.returncode==0,run.stderr;got=run.stdout.splitlines();assert len(got)==len(expected);failures=[{'case':i,'input':lines[i],'original':w,'rust':g}for i,(w,g)in enumerate(zip(expected,got))if w!=g]
 # Original table destinations are data, not patched Ghidra switch inference.
 jump=0x142aa44f4+struct.unpack('<i',cpu.mem_read(0x142aa44f4+7*4,4))[0]
 assert jump==0x1408cea79;assert original_mesh_eq(42,42,100,101)==False
 files=['reconstruction/building/wall_platform_collision.rs','reconstruction/src/bin/wall-platform-collider-probe.rs','reconstruction/building/wall_platform_stairs.rs','reconstruction/building/wall_state.rs','reconstruction/Cargo.toml','reconstruction/Cargo.lock'];oracles=['scripts/multi-version/verify_wall_platform_collision_policy.py','evidence/building-platform-stairs/native_platform_collision_policy.py','evidence/building-platform-stairs/native_platform_collision_hash.py','evidence/building-constructor/native_fixture.py'];digest=lambda xs:{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in xs}
 report={'ok':not failures,'cases':len(lines),'outcome_counts':counts,'failures':failures,'original_tag7_eq_jump':hex(jump),'scope':'Resolved outer-map presence and original get_collider pose output -> actual original141245ade..141245c94/141246026 ordered pose gate -> full originalMesh::eq1408cea30(tag7). Includes signedzero/nonfinite quaternion/translation, missing oldpose, different wall IDs and fingerprints. Tag7 original jump table compares source+8 WallId despite patched Ghidra C claiming unitlike variant. Original map/physics acquisition, dependency insertion/removal, dirtyAABB cells and ColliderDiff lifecycle are outside this numeric/action proof and require real backend integration. Absent outer-map insert follows static originalbranch; no map implementation is reconstructed.','binary_sha256':b['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'candidate_sources':digest(files),'oracle_sources':digest(oracles)}
 (ROOT/'evidence/building-platform-stairs/B-collision-policy-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(lines),'counts':counts,'failures':len(failures),'first':failures[:1]}));assert not failures
if __name__=='__main__':main()
