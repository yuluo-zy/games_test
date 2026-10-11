"""Original actual creator cell: terrain→rise/depth→collider vertices/indices."""
import sys, struct, random, json, subprocess, hashlib
from verification_provenance import ROOT
sys.path.insert(0,str(ROOT/'evidence/building-segmentation'))
from native_platform_geometry import produce
from native_fixture import b
def word(v):return '%08x'%struct.unpack('<I',struct.pack('<f',v))[0]
def main():
 rng=random.Random(20261031);lines=[];expected=[];counts={};
 for i in range(256):
  points=[[rng.uniform(-70.,70.),rng.uniform(-2.,3.),rng.uniform(-70.,70.)]for _ in range(4)];coords=[(i%3,rng.choice([0.,.1,.5,.75,1.])),((i//3)%3,rng.choice([0.,.25,.6,1.]))];maximum=rng.choice([1.6,2.5,4.,6.,10.,14.]);extra=rng.choice([.3,.8,1.5,3.,7.]);minimum=rng.choice([-2.,0.,1.]);upper=rng.choice([3.,8.,15.]);salt=rng.randrange(65536);varying=i%2;fix=(i//2)%2;offset=[0,8,64,65536,0xfffffffa][i%5]
  encoded=[((j%512)*131+(j//512)*71 if varying else 0)+salt&65535 for j in range(262144)]
  o=produce(points,coords,maximum,extra,encoded,minimum,upper,fix,offset);counts[o['rise_count']]=counts.get(o['rise_count'],0)+1;expected.append(o['top']+o['bottom']+struct.pack('<Q',o['rise_count']).hex()+o['depth']+o['vertices']+o['indices'])
  line=f'{salt} {varying} {fix} {word(minimum)} {word(upper)} {word(maximum)} {word(extra)} {offset} '+' '.join(f'{idx} {word(t)}'for idx,t in coords)+' '+' '.join(word(v)for p in points for v in p);lines.append(line)
 probe=ROOT/'reconstruction/target/release/wall-platform-stair-geometry-probe';run=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True);assert run.returncode==0,run.stderr;actual=run.stdout.splitlines();assert len(actual)==len(expected)
 failures=[{'case':i,'input':lines[i],'original':want,'rust':got}for i,(want,got)in enumerate(zip(expected,actual))if want!=got]
 files=['reconstruction/building/wall_platform_stairs.rs','reconstruction/src/bin/wall-platform-stair-geometry-probe.rs','reconstruction/building/wall_collection.rs','reconstruction/building/wall_state.rs','reconstruction/wall/height_rules.rs','reconstruction/Cargo.lock','reconstruction/Cargo.toml'];oracles=['scripts/multi-version/verify_wall_platform_stair_geometry.py','evidence/building-segmentation/native_platform_geometry.py','evidence/building-segmentation/native_segmentation.py','evidence/building-constructor/native_fixture.py'];digest=lambda xs:{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in xs}
 report={'ok':not failures,'cases':len(lines),'rise_counts':counts,'vertices':len(lines)*8,'indices':len(lines)*36,'failure_count':len(failures),'failures':failures[:20],'scope':'Resolved actual Curve3 endpoint index/fraction → original creator140775920..140775ff2 executes PublicWall max_y getter, TerrainHeightSsbo sampling, rise/depth selection, two tangent/normal conversions, skewed_box_trigger_vol8vertices/36indices. All geometry/scalar bits and wrapping index offsets compared. Does not accept actual query acquisition, horizontal partitions, shape-specific render Curve2,96B rows, holes, collider World lifetime. CRT floorf/allocator/memcopy bridges only; no private geometry output injection. finite nondegenerate horizontal tangents.','binary_sha256':b['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'candidate_sources':digest(files),'oracle_sources':digest(oracles)}
 name='B-platform-stair-geometry-verification.json'if not failures else 'B-platform-stair-geometry-failure.json';(ROOT/'evidence/building-segmentation'/name).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(lines),'rise_counts':counts,'failures':len(failures),'first':failures[:1]}));assert not failures
if __name__=='__main__':main()
