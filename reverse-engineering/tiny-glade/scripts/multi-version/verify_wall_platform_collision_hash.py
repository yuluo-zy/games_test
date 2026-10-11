"""Native indexed mesh fingerprint schema; hashing algorithms reused from std."""
import sys,json,struct,random,subprocess,hashlib
from verification_provenance import ROOT
sys.path.insert(0,str(ROOT/'evidence/building-platform-stairs'))
from native_platform_collision_hash import original_hash,b

def main():
 rng=random.Random(20261104);lines=[];expected=[]
 for i in range(256):
  np=i%19;nt=i%9;points=[[rng.getrandbits(32)for _ in range(3)]for _ in range(np)];tris=[[rng.randrange(max(np,1))for _ in range(3)]for _ in range(nt)];a=None if i%4==0 else [[rng.getrandbits(32)for _ in range(3)]for _ in range(i%5)];z=None if i%3==0 else [[rng.getrandbits(32)for _ in range(2)]for _ in range(i%7)]
  expected.append(f'{original_hash(points,tris,a,z):016x}');line=f'{np} {nt} {-1 if a is None else len(a)} {-1 if z is None else len(z)} '+' '.join(f'{v:08x}'for p in points for v in p)+' '+' '.join(str(v)for arr in [tris,a or[],z or[]]for p in arr for v in p);lines.append(line)
 probe=ROOT/'reconstruction/target/release/wall-platform-collider-probe';run=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True);assert run.returncode==0,run.stderr;actual=run.stdout.splitlines();assert len(actual)==len(expected);failures=[{'case':i,'input':lines[i],'original':w,'rust':g}for i,(w,g)in enumerate(zip(expected,actual))if w!=g]
 paths=['reconstruction/building/wall_platform_collision.rs','reconstruction/src/bin/wall-platform-collider-probe.rs','reconstruction/Cargo.toml','reconstruction/Cargo.lock'];oracles=['scripts/multi-version/verify_wall_platform_collision_hash.py','evidence/building-platform-stairs/native_platform_collision_hash.py','evidence/building-constructor/native_fixture.py'];digest=lambda ps:{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in ps}
 report={'ok':not failures,'cases':len(lines),'failures':failures,'scope':'Entire original new_indexed1408cee40 hash prefix to1408cef4e executes original std SipHash for triangle [u32;3] indices then point bitcast UVec3 slice, then present optional float3/float2 bitcast folds. None and Some(empty) are exercised. Hash output is observed RDI, never bridged. Raw float words intentionally include nonfinite/signedzero: this hash-only schema proof is not physics input reachability. Newindexed normals/trimesh/physics and RaycastWorld lifecycle remain outside.','binary_sha256':b['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'candidate_sources':digest(paths),'oracle_sources':digest(oracles)}
 (ROOT/'evidence/building-platform-stairs/B-collision-hash-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(lines),'failures':len(failures),'first':failures[:1]}));assert not failures
if __name__=='__main__':main()
