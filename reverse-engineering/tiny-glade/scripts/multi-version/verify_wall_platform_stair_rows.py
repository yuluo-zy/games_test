"""Actual creator cell→shape/rise curves→full constructor→96B SparseWriter."""
import sys,struct,json,subprocess,hashlib,random,math
from verification_provenance import ROOT
sys.path.insert(0,str(ROOT/'evidence/building-segmentation'))
from native_platform_rows_research import produce
from native_platform_segmentation import call
from native_fixture import b,cpu,data,bits,UC_X86_REG_RCX,UC_X86_REG_RDX
def word(v):return '%08x'%struct.unpack('<I',struct.pack('<f',v))[0]
def main():
 rng=random.Random(20261102);lines=[];expected=[];inputs=[];total=0;rises={}
 for i in range(96):
  tag=i%2
  axis=[(1.,0.),(0.,1.),(.6,.8),(-.8,.6),(struct.unpack('<f',struct.pack('<f',math.cos(.7)))[0],struct.unpack('<f',struct.pack('<f',math.sin(.7)))[0])][(i//2)%5]
  shape=struct.pack('<I6f',1,*axis,rng.choice([0.,2.]),rng.choice([0.,-3.]),rng.choice([4.,6.,8.]),rng.choice([3.,4.,5.])) if tag else struct.pack('<I4f8x',0,rng.choice([0.,1.]),rng.choice([0.,-2.]),rng.choice([1.,2.,3.]),rng.choice([0.,.3,-.7]))
  cpu.mem_write(data+0x9000,shape);call(0x1411ff2b0,[(UC_X86_REG_RCX,data+0x9100),(UC_X86_REG_RDX,data+0x9000)]);_,p,n=struct.unpack('<3Q',cpu.mem_read(data+0x9100,24));points=[[x,0.,z]for x,z in struct.iter_unpack('<2f',cpu.mem_read(p,n*8))];idx=(i//2)%(n-2);end=min(idx+1,n-2);
  if i>=32 and i%4==0:idx=n-2;end=0
  if i>=32 and i%8==2:idx=0;end=0
  coords=[(idx,[0.,.25,.6][i%3]),(end,[.5,.75,1.][i%3])]
  maximum=[1.6,2.5,3.5][i%3];extra=[.5,.8,1.2][(i//3)%3];salt=[0,10000,35000][i%3];varying=i%2;fix=(i//2)%2;lo=[-.5,0.][i%2];hi=3.;wall=[27,100000,100011,0xffffffff][i%4];encoded=[((j%512)*131+(j//512)*71 if varying else 0)+salt&65535 for j in range(262144)]
  cell,rows,view=produce(points,coords,maximum,extra,encoded,lo,hi,fix,shape,wall,views=True);total+=len(rows);rises[cell['rise_count']]=rises.get(cell['rise_count'],0)+1;expected.append(';'.join(x.hex()for x in rows));inputs.append(view)
  line=f'{shape.hex()} {wall} {salt} {varying} {fix} {word(lo)} {word(hi)} {word(maximum)} {word(extra)} '+' '.join(f'{idx} {word(t)}'for idx,t in coords)+' '+' '.join(word(v)for p in points for v in p);lines.append(line)
 probe=ROOT/'reconstruction/target/release/wall-platform-stair-rows-probe';run=subprocess.run([str(probe)],input='\n'.join(lines)+'\n',capture_output=True,text=True);assert run.returncode==0,run.stderr;actual=run.stdout.splitlines();assert len(actual)==len(expected)
 failures=[]
 for i,(want,got)in enumerate(zip(expected,actual)):
  if want!=got:
   w=want.split(';')if want else[];g=got.split(';')if got else[];first=next(((j,a,z)for j,(a,z)in enumerate(zip(w,g))if a!=z),None);failures.append({'case':i,'input':lines[i],'original_count':len(w),'rust_count':len(g),'first':first,'source_view':inputs[i]})
 files=['reconstruction/building/wall_platform_stair_rows.rs','reconstruction/src/bin/wall-platform-stair-rows-probe.rs','reconstruction/building/wall_platform_stairs.rs','reconstruction/building/wall_production.rs','reconstruction/building/wall_constructor.rs','reconstruction/building/wall_construction_sources.rs','reconstruction/building/wall_freeform_cache.rs','reconstruction/building/wall_collection.rs','reconstruction/building/wall_state.rs','reconstruction/wall/height_rules.rs','reconstruction/roof/surface.rs','reconstruction/Cargo.lock','reconstruction/Cargo.toml'];oracles=['scripts/multi-version/verify_wall_platform_stair_rows.py','evidence/building-segmentation/native_platform_rows_research.py','evidence/building-segmentation/native_platform_geometry.py','evidence/building-segmentation/native_platform_stair_segments.py','evidence/building-segmentation/native_platform_segmentation.py','evidence/building-segmentation/native_segmentation.py','evidence/building-structures/native_structures.py','evidence/building-production/native_production.py','evidence/building-constructor/native_fixture.py'];digest=lambda xs:{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest()for p in xs}
 report={'ok':not failures,'cases':len(lines),'gpu_records96':total,'rise_counts':rises,'circle_cases':48,'rectangle_cases':48,'axes':'axis-aligned, rotated .6/.8 and -.8/.6, cos/sin .7; actual nativeasCurve2 source points' ,'failure_count':len(failures),'failures':failures[:12],'scope':'Resolved prepared endpoint coords/real512terrain/public scalar/shape/WallId → actual originalcell geometry → whole creator140775ff2..140776f9b: shapeClosestU/asCurve2/circularSlice/cornerWeights/normaloffset/dedup/rises/actual from_curve(kind0/tag2)/GPUInstance default and flags0x10/actual96B SparseWriter. Entire ordered96B records compared; no captured render curve/RNG/proposal/instance output input. Shared G04 producer and G02 shape utilities reused. Original ECS acquisition, frame scheduler, mesh colliderWorld lifetime and final holes resource pass excluded here.','binary_sha256':b['binary']['sha256'],'candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'candidate_sources':digest(files),'oracle_sources':digest(oracles)}
 name='B-platform-stair-rows-verification.json'if not failures else'B-platform-stair-rows-first-failure.json';(ROOT/'evidence/building-segmentation'/name).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(lines),'records':total,'failures':len(failures),'first':failures[:1]}));assert not failures
if __name__=='__main__':main()
