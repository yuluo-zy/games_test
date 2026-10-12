"""Actual resolved 80B decorator row → original terrain/variant filter."""
import sys, json, struct, random, subprocess, hashlib
from verification_provenance import ROOT
sys.path.insert(0, str(ROOT/'evidence/building-segmentation'))
from native_platform_blockers_research import evaluate
from native_fixture import b

def word(v): return '%08x' % struct.unpack('<I',struct.pack('<f',v))[0]
def main():
 rng=random.Random(20261030); lines=[]; expected=[]; fixtures=[]
 for i in range(384):
  raw=[[-3.,-2.],[-3.,2.],[3.,2.],[3.,-2.],[-3.,-2.]] if i%3 else [[-20.,0.],[0.,5.],[15.,-3.]]
  salt=rng.randrange(65536);varying=i%2;fix=(i//2)%2;lo=[-2.,0.,1.][i%3];hi=[4.,8.,15.][i%3]
  encoded=[((j%512)*131+(j//512)*71 if varying else 0)+salt &65535 for j in range(512*512)]
  rows=[]; payload=''; invalid=False
  for j in range(i%4+1):
   row=bytearray(rng.randbytes(80)); row[0x4e]=[63,63,63,0,62,255][(i+j)%6];row[0x2a]=[0,1,2,3,4,5,255][(i+j)%7];row[0x49]=(rng.randrange(32)<<3)|((i+j)%8)
   struct.pack_into('<2f',row,8,rng.uniform(-8.,40.),[-2.,1.,3.,8.,20.][(i+j)%5]);struct.pack_into('<f',row,0x3c,[0.,.5,1.2,3.,-2.][(i+j)%5]);rows.append(row)
   if not invalid:
    result=evaluate(raw,row,encoded,lo,hi,fix)
    if result.get('invalid_extra'):invalid=True
    else:payload+=result['payload']
  expected.append('invalid_extra' if invalid else payload)
  line=f'{len(raw)} {salt} {varying} {fix} {word(lo)} {word(hi)} '+' '.join(word(v) for point in raw for v in point)+' '+' '.join(row.hex() for row in rows);lines.append(line);fixtures.append({'line':line,'original':expected[-1]})
 probe=ROOT/'reconstruction/target/release/wall-platform-blockers-probe'; run=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True);assert run.returncode==0,run.stderr
 actual=run.stdout.splitlines();assert len(actual)==len(expected); failures=[dict(case=i,**fixtures[i],rust=got)for i,(want,got)in enumerate(zip(expected,actual))if want!=got]
 files=['reconstruction/building/wall_platform_stairs.rs','reconstruction/src/bin/wall-platform-blockers-probe.rs','reconstruction/building/decorator_placement.rs','reconstruction/building/wall_collection.rs','reconstruction/roof/surface.rs','reconstruction/building/wall_state.rs','reconstruction/wall/height_rules.rs','reconstruction/Cargo.lock','reconstruction/Cargo.toml']
 oracles=['scripts/multi-version/verify_wall_platform_blockers.py','evidence/building-segmentation/native_platform_blockers_research.py','evidence/building-segmentation/native_platform_segmentation.py','evidence/building-constructor/native_fixture.py']
 digest=lambda paths:{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in paths}
 report={'ok':not failures,'cases':len(lines),'rows':sum(i%4+1 for i in range(384)),'failures':failures[:20],'failure_count':len(failures),'scope':'Resolved observed80B rows → actual original140774f3d filter, original rawCurve2 builder/to_world/TerrainHeightSsbo sampler/as_stair variant getter. Complete selected XY+width byte order compared. Preallocated std Vec capacity4; singleton iteration resolved at query boundary. Does not accept ECS row acquisition, full stair geometry/collider/holes/cleanup. Reuses existing TerrainHeightGrid512 sampling dependency adapter; no float_grid implementation added.','binary_sha256':b['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'candidate_sources':digest(files),'oracle_sources':digest(oracles)}
 name='B-platform-blockers-verification.json' if not failures else 'B-platform-blockers-failure.json';(ROOT/'evidence/building-segmentation'/name).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(lines),'failures':len(failures),'first':failures[:1]}));assert not failures
if __name__=='__main__':main()
