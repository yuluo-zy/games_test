"""Actualoriginal platform ranges/blocker closure/104B frontend differential."""
import sys,struct,math,json,random,subprocess,hashlib
from verification_provenance import ROOT
sys.path.insert(0,str(ROOT/'evidence/building-segmentation'))
from native_platform_stair_segments import prepare_steps
from native_fixture import b

def f32(v):return struct.unpack('<f',struct.pack('<f',v))[0]
def word(v):return f'{struct.unpack("<I",struct.pack("<f",v))[0]:08x}'
def us_for(points):
 values=[0.];total=0.
 for a,z in zip(points,points[1:]):
  d=[f32(z[i]-a[i])for i in range(3)];length=f32(math.sqrt(f32(f32(f32(d[0]*d[0])+f32(d[1]*d[1]))+f32(d[2]*d[2]))));total=f32(total+length);values.append(total)
 return [f32(v/total)for v in values]
def main():
 rng=random.Random(20261027);cases=[]
 for i in range(192):
  raw=[[-3.,-2.],[-3.,2.],[3.,2.],[3.,-2.],[-3.,-2.]];factor=[1,2,4,8][i%4];points=[]
  for a,z in zip(raw,raw[1:]):
   for j in range(factor):points.append([f32(a[0]+(z[0]-a[0])*j/factor),f32((j%3)*(.15 if i%3==0 else 0)),f32(a[1]+(z[1]-a[1])*j/factor)])
  points.append(points[0].copy());n=len(points);us=us_for(points);values=[rng.choice([0.,.3,.7,1.])for _ in points];values[-1]=values[0];maximum=[1.6,2.5,3.55,4.,6.][i%5];arch=[maximum]*n
  blockers=[([rng.uniform(0,20),rng.uniform(0,2)],rng.choice([.5,1.,2.,4.]))for _ in range(i%4)]
  cases.append((raw,points,us,values,arch,1,i%2,maximum,blockers))
 expected=[];lines=[];total=0;stepcount=0
 for raw,points,us,values,arch,shape,special,maximum,blockers in cases:
  out,steps=prepare_steps(raw,points,us,values,arch,shape,special,maximum,blockers);expected.append(';'.join(x.hex()for x in out));total+=len(out);stepcount+=len(steps)
  parts=[word(v)for p in raw for v in p]+[word(v)for p,u,h,a in zip(points,us,values,arch)for v in [*p,u,h,a]]+[word(v)for coord,width in blockers for v in [*coord,width]];lines.append(f'{shape} {special} {word(maximum)} {len(raw)} {len(points)} {len(blockers)} '+' '.join(parts))
 probe=ROOT/'reconstruction/target/release/wall-platform-stairs-probe';run=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True);assert run.returncode==0,run.stderr[:2000];actual=run.stdout.splitlines();assert len(actual)==len(expected)
 fail=[{'case':i,'input':lines[i],'original':w,'rust':got}for i,(w,got)in enumerate(zip(expected,actual))if w!=got]
 files=[ROOT/'reconstruction/building/wall_platform_stairs.rs',ROOT/'reconstruction/src/bin/wall-platform-stairs-probe.rs',ROOT/'reconstruction/building/wall_state.rs',ROOT/'reconstruction/wall/height_rules.rs',ROOT/'reconstruction/building/wall_structures.rs',ROOT/'reconstruction/building/wall_segmentation.rs',ROOT/'reconstruction/building/wall_platform_segmentation.rs',ROOT/'reconstruction/building/decorator_placement.rs',ROOT/'reconstruction/roof/surface.rs',ROOT/'reconstruction/Cargo.lock',ROOT/'reconstruction/Cargo.toml']
 report={'ok':not fail,'cases':len(cases),'source_platform_steps':stepcount,'materialized_segments':total,'failure_count':len(fail),'failures':fail[:20],'scope':'Original actualcalc→platformsteps→WallCoordFromWorld/WallSpace distance blocker closure141206070→threshold1411ff790/process_segment span10000→original104B materializer. Allcanonical arrays/sourceIndices/range compared. No captured mask/segment result input. Blocker items explicit sourcevalues; raw80B decorator/terrain filter and lateractualrows/mesh/holes/cleanup not acceptedhere.','binary_sha256':b['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'candidate_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in files},'oracle_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in [ROOT/'scripts/multi-version/verify_wall_platform_stair_segments.py',ROOT/'evidence/building-segmentation/native_platform_stair_segments.py',ROOT/'evidence/building-segmentation/native_platform_segmentation.py',ROOT/'evidence/building-segmentation/native_segmentation.py',ROOT/'evidence/building-structures/native_structures.py',ROOT/'evidence/building-production/native_production.py',ROOT/'evidence/building-constructor/native_fixture.py']}}
 name='B-platform-stair-segments-verification.json'if not fail else 'B-platform-stair-segments-failure.json';(ROOT/'evidence/building-segmentation'/name).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not fail,'cases':len(cases),'segments':total,'failures':len(fail),'first':fail[:1]}));assert not fail
if __name__=='__main__':main()
