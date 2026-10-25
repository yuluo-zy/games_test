"""Resolved source chain through actual decorator filter/steps/partitions/cells."""
import sys,struct,math,json,random,subprocess,hashlib
from verification_provenance import ROOT
sys.path.insert(0,str(ROOT/'evidence/building-segmentation'))
from native_platform_blockers_research import evaluate
from native_platform_stair_segments import prepare_steps
from native_platform_partitions import partitions
from native_platform_geometry import produce
from native_platform_holes import produce as hole_produce
from native_fixture import b
def f32(v):return struct.unpack('<f',struct.pack('<f',v))[0]
def word(v):return '%08x'%struct.unpack('<I',struct.pack('<f',v))[0]
def decode_segment(raw):
 at=0;arrays=[]
 for width in [4,12,4]:
  n=struct.unpack_from('<Q',raw,at)[0];at+=8;payload=raw[at:at+n*width];at+=n*width
  arrays.append(list(struct.iter_unpack('<3f',payload))if width==12 else list(struct.unpack('<'+'f'*n,payload)))
 return arrays[1],arrays[2]
def us_for(points):
 total=0.;values=[0.]
 for a,z in zip(points,points[1:]):
  d=[f32(z[j]-a[j])for j in range(3)];total=f32(total+f32(math.sqrt(f32(f32(f32(d[0]*d[0])+f32(d[1]*d[1]))+f32(d[2]*d[2])))));values.append(total)
 return [f32(v/total)for v in values]
def main():
 rng=random.Random(20261101);lines=[];expected=[];total_cells=0;total_segments=0;total_steps=0;counts={}
 for i in range(64):
  raw=[[-3.,-2.],[-3.,2.],[3.,2.],[3.,-2.],[-3.,-2.]];factor=[1,2,4,8][i%4];points=[]
  for a,z in zip(raw,raw[1:]):
   for j in range(factor):points.append([f32(a[0]+(z[0]-a[0])*j/factor),f32((j%3)*(.15 if i%3==0 else 0)),f32(a[1]+(z[1]-a[1])*j/factor)])
  points.append(points[0].copy());us=us_for(points);values=[rng.choice([0.,.3,.7,1.])for p in points];values[-1]=values[0];maximum=[1.6,2.5,3.5,4.,6.][i%5];extra=[.3,.8,1.6][i%3];arch=[maximum]*len(points);shape=1;special=i%2;salt=rng.randrange(65536);varying=i%2;fix=(i//2)%2;lo=[-2.,0.][i%2];hi=[2.,5.][i%2]
  encoded=[((j%512)*131+(j//512)*71 if varying else 0)+salt&65535 for j in range(262144)];rows=[];blockers=[]
  for j in range(i%4):
   row=bytearray(rng.randbytes(80));row[0x4e]=[63,63,0][j%3];row[0x2a]=[0,1,3][j%3];row[0x49]=(rng.randrange(32)<<3)|(j%2);struct.pack_into('<2f',row,8,rng.uniform(-1.,21.),rng.uniform(-1.,4.));struct.pack_into('<f',row,0x3c,[.5,1.2,3.][j%3]);rows.append(row)
   out=evaluate(raw,row,encoded,lo,hi,fix)
   assert not out.get('invalid_extra')
   blockers.extend([(list(v[:2]),v[2])for v in struct.iter_unpack('<3f',bytes.fromhex(out['payload']))])
  segs,steps=prepare_steps(raw,points,us,values,arch,shape,special,maximum,blockers);total_segments+=len(segs);total_steps+=len(steps);cells=[];holes=[];stairs_length=0.
  for s in segs:
   pos,su=decode_segment(s);holes.append(hole_produce(raw,pos,maximum).hex());coords,length=partitions(pos,su,points,us)
   if length>=.5:stairs_length=f32(stairs_length+length)
   for pair in coords:
    cell=produce(points,pair,maximum,extra,encoded,lo,hi,fix,len(cells)*8);counts[cell['rise_count']]=counts.get(cell['rise_count'],0)+1;cells.append(cell['top']+cell['bottom']+struct.pack('<Q',cell['rise_count']).hex()+cell['depth']+cell['vertices']+cell['indices'])
  total_cells+=len(cells);expected.append(struct.pack('<fQ',stairs_length,len(cells)).hex()+''.join(cells)+struct.pack('<Q',len(holes)).hex()+''.join(holes))
  line=f'{shape} {special} {word(maximum)} {word(extra)} {salt} {varying} {fix} {word(lo)} {word(hi)} {len(raw)} {len(points)} {len(rows)} '+' '.join(word(v)for p in raw for v in p)+' '+' '.join(word(v)for p,u,val,a in zip(points,us,values,arch)for v in [*p,u,val,a])+' '+' '.join(r.hex()for r in rows);lines.append(line)
 probe=ROOT/'reconstruction/target/release/wall-platform-stair-chain-probe';run=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True);assert run.returncode==0,run.stderr;actual=run.stdout.splitlines();assert len(actual)==len(expected);failures=[{'case':i,'input':lines[i],'original':want,'rust':got}for i,(want,got)in enumerate(zip(expected,actual))if want!=got]
 files=['reconstruction/building/wall_platform_stairs.rs','reconstruction/src/bin/wall-platform-stair-chain-probe.rs','reconstruction/building/wall_structures.rs','reconstruction/building/wall_segmentation.rs','reconstruction/building/wall_platform_segmentation.rs','reconstruction/building/wall_holes.rs','reconstruction/building/wall_collection.rs','reconstruction/building/decorator_placement.rs','reconstruction/roof/surface.rs','reconstruction/roof/tiles.rs','reconstruction/building/wall_state.rs','reconstruction/wall/height_rules.rs','reconstruction/Cargo.lock','reconstruction/Cargo.toml'];oracles=['scripts/multi-version/verify_wall_platform_stair_chain.py','evidence/building-segmentation/native_platform_partitions.py','evidence/building-segmentation/native_platform_holes.py','evidence/building-segmentation/native_platform_geometry.py','evidence/building-segmentation/native_platform_blockers_research.py','evidence/building-segmentation/native_platform_stair_segments.py','evidence/building-segmentation/native_platform_segmentation.py','evidence/building-segmentation/native_segmentation.py','evidence/building-structures/native_structures.py','evidence/building-constructor/native_fixture.py'];digest=lambda xs:{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in xs}
 report={'ok':not failures,'cases':len(lines),'platform_steps':total_steps,'materialized_segments':total_segments,'cells':total_cells,'holes':total_segments,'rise_counts':counts,'vertices':total_cells*8,'indices':total_cells*36,'failure_count':len(failures),'failures':failures[:8],'scope':'Resolved prepared curve/U/values/arch + actual80B decorator rows/encoded512terrain → actual original rawcalc/platformsteps/blocker filter/104B frontend → creator104B lengths/init_rng(0)/random_splits → actual Curve3 endpoint selector → actual terrain/rise/depth/8vertices/36indices. All scalar/order/geometry/index bits, actualstairs length sum and actual per104B registry hole center/dimensions/kind3 compared. Original native calls are composed across resolved owner boundaries; no captured mask/segment/partition/cell input. ECS query acquisition, shape-specific render curves/96B rows, WallHoles resource acquisition and colliderWorld lifetime remain outside. Finite nondegenerate tangent domain.','binary_sha256':b['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'candidate_sources':digest(files),'oracle_sources':digest(oracles)}
 name='B-platform-stair-chain-verification.json'if not failures else 'B-platform-stair-chain-failure.json';(ROOT/'evidence/building-segmentation'/name).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(lines),'segments':total_segments,'cells':total_cells,'failures':len(failures),'first':failures[:1]}));assert not failures
if __name__=='__main__':main()
