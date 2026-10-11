"""Full B original WallConstructor::from_curve -> complete96B instance array.
Source inputs only; no captured proposal/context/RNG draw may enter candidate.
"""
import sys,json,random,struct,math,hashlib,subprocess
from pathlib import Path
from verification_provenance import ROOT,candidate_provenance
sys.path.insert(0,str(ROOT/'evidence/building-production'))
from native_production import produce,b
from try_cases import make_line
from try_variants import variant_line

def f32(v):return struct.unpack('<f',struct.pack('<f',v))[0]
def prepare(c):
 for key in ['us','arch','second','third']:c[key]=list(map(f32,c[key]))
 c['points']=[list(map(f32,p))for p in c['points']];c['length']=f32(c['length'])
 for key in ['base_height','perturb','width_padding','thickness_scale','wall_float','holes_length']:
  if key in c and c[key]is not None:c[key]=f32(c[key])
 c['holes']=[(list(map(f32,a)),k,o)for a,k,o in c['holes']];return c

def verify_branches():
 source=json.loads((ROOT/'evidence/building-production/B-wall-production-verification.json').read_text())
 cases=[]
 for index in [0,1,6,14,17,21,32,36,42,55,72,78]:
  template=source['input_cases'][index]
  for tag in [0,1,2]:
   for fit in [False,True]:
    for kind in [0,1,2,3]:
     cases.append(prepare({**template,'base_height':0.,'kind':kind,'top_bounds':[f32(max(template['arch'])*.5),.15,.4,.65,.9,tag],'fence_fit':fit}))
 # Exact duplicated height bands and interval endpoints require preserving row
 # indices and random draws even when a zero-height row emits no instance.
 for height in [0.,max(source['input_cases'][1]['arch'])]:
  for tag in [0,1]:
   for kind in [0,1,2,3]:
    cases.append(prepare({**source['input_cases'][1],'base_height':0.,'kind':kind,'top_bounds':[height,0.,.5,.5,1.,tag],'fence_fit':False}))
 lines=[variant_line(c)for c in cases];expected=[]
 for index,c in enumerate(cases):
  expected.append(produce(**c))
  if (index+1)%24==0:print(json.dumps({'native_returned':index+1,'cases':len(cases)}),flush=True)
 run=subprocess.run([str(ROOT/'reconstruction/target/release/wall-production-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True);got=run.stdout.splitlines();assert len(got)==len(expected)
 failures=[]
 for i,(want,row)in enumerate(zip(expected,got)):
  if row.startswith('error'):failures.append({'case':i,'source':cases[i],'input':lines[i],'actual':row,'expected_count':len(want)});continue
  count,rest=row.split(' ',1);actual=[bytes.fromhex(x)for x in rest.split(',')if x]
  assert int(count)==len(actual)
  diff=[{'record':j,'offsets':[k for k,(a,b)in enumerate(zip(e,g))if a!=b],'expected':e.hex(),'actual':g.hex()}for j,(e,g)in enumerate(zip(want,actual))if e!=g]
  if len(actual)!=len(want)or diff:failures.append({'case':i,'source':cases[i],'input':lines[i],'expected_count':len(want),'actual_count':len(actual),'differences':diff[:3]})
 deps=[Path(__file__),ROOT/'evidence/building-production/native_production.py',ROOT/'evidence/building-production/try_cases.py',ROOT/'evidence/building-production/try_variants.py',ROOT/'evidence/building-constructor/native_fixture.py']
 flags=sorted({struct.unpack('<I',record[92:96])[0]for records in expected for record in records})
 report={'ok':not failures,'baseline':'B','binary_sha256':b['binary']['sha256'],'cases':len(cases),'instance_records':sum(map(len,expected)),'native_instance_flags':flags,'failure_count':len(failures),'failures':failures,'input_cases':cases,'candidate_binary_sha256':hashlib.sha256((ROOT/'reconstruction/target/release/wall-production-probe').read_bytes()).hexdigest(),'oracle_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in deps},'scope':'Full original from_curve returned -> entire ordered96B output arrays, all decoded construction kind0..3/top tag0..2/fence-fit false-or-true combinations. Nonuniform three authority arrays, real rectangles/circles/open paths, multiple holes and source options. Candidate accepts source input only; every native case asserts RIP == stop.','limitations':['Behavior is proven against B; A/C module provenance excludes cargo dependencies but does not establish identical branch behavior.','True World Roof/top owner binding is separate from this explicit ABI producer proof.','Finite valid per-point arrays and normalized intervals are required; arbitrary corrupt or invalid original ABI states are not claimed.'],**candidate_provenance()}
 (ROOT/'evidence/building-production/B-wall-production-branches-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(cases),'records':report['instance_records'],'failures':len(failures),'first':failures[:1]}),flush=True);assert not failures

def main():
 rng=random.Random(20261013);cases=[]
 for i in range(72):
  if i%3==0:
   radius=rng.uniform(.18,3.);n=8+i%13;points=[[radius*math.cos(j*2*math.pi/n),radius*math.sin(j*2*math.pi/n)]for j in range(n)]+[[radius,0.]]
  else:
   x,z=rng.uniform(.2,5),rng.uniform(.1,3);points=[[-x,-z],[-x,z],[x,z],[x,-z],[-x,-z]]
  pts=[list(map(f32,p))for p in points];lens=[f32(math.hypot(f32(pts[j+1][0]-pts[j][0]),f32(pts[j+1][1]-pts[j][1])))for j in range(len(pts)-1)];dist=[0.]
  for le in lens:dist.append(f32(dist[-1]+le))
  length=dist[-1];us=[f32(u/length)for u in dist];n=len(pts);height=rng.uniform(.2,4.);arch=[height]*n if i<12 or i%4==0 else[rng.uniform(.05,height)for _ in pts];arch[-1]=arch[0];second=[0.]*n if i<12 or i%5==0 else[rng.uniform(-.3,height*.8)for _ in pts];second[-1]=second[0];third=[0.]*n if i<12 or i%7==0 else[rng.uniform(0,.7)for _ in pts];third[-1]=third[0]
  holes=[]
  for j in range(i%5):
   x=rng.uniform(-.3,length);y=rng.uniform(-.1,height);holes.append(([x,y,x+rng.uniform(.03,min(2,length)),y+rng.uniform(.05,2)],j%3,0))
  holes.sort(key=lambda h:h[0][2]);seed=[27,1,13,0xffffffff,0x123456789abc,0xffffffffffffffff][i%6]
  c=dict(points=pts,us=us,length=length,arch=arch,second=second,third=third,holes=holes,seed=seed,material=[23,0,0x80000000,0xffffffff][i%4],closed=bool(i%4!=1),base_height=0. if i%5 else rng.uniform(-.3,.1),perturb=0. if i%2 else rng.uniform(.1,1.),width_padding=None if i%3 else rng.uniform(-.015,.12),thickness_scale=None if i%3 else rng.uniform(.3,2.),wall_float=[0.,.7,1.,.2][i%4]);cases.append(prepare(c))
 # Actual open polylines include distinct endpoints, not merely closed=false holes.
 for n in [2,3,5,8]:
  for seed in [27,0xffffffff,0x123456789abc]:
   pts=[[f32(j*.7),f32((j%3)*.3)]for j in range(n)];dist=[0.]
   for a,bp in zip(pts,pts[1:]):dist.append(f32(dist[-1]+f32(math.hypot(f32(bp[0]-a[0]),f32(bp[1]-a[1])))))
   ll=dist[-1];us=[f32(x/ll)for x in dist];ar=[f32(.5+(j%3)*.4)for j in range(n)];second=[f32((j%2)*.25)for j in range(n)];third=[f32(.1*(j%3))for j in range(n)];cases.append(prepare(dict(points=pts,us=us,length=ll,arch=ar,second=second,third=third,holes=[([-.1,.1,.7,.6],0,0)],seed=seed,material=0x80000000,closed=False,base_height=0.,perturb=.6,width_padding=.015,thickness_scale=1.2,wall_float=1.)))
 # Keep small no-output and periodic-boundary cases within real finite source domain.
 empty={**cases[1],'arch':[0.]*len(cases[1]['points']),'second':[0.]*len(cases[1]['points']),'third':[0.]*len(cases[1]['points']),'holes':[]};cases.append(empty)
 lines=[make_line(c)for c in cases];expected=[produce(**c)for c in cases];run=subprocess.run([str(ROOT/'reconstruction/target/release/wall-production-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True);got=run.stdout.splitlines();assert len(got)==len(expected)
 failures=[];total=0
 for i,(want,row)in enumerate(zip(expected,got)):
  total+=len(want)
  if row.startswith('error'):failures.append({'case':i,'input':lines[i],'actual':row,'expected_count':len(want)});continue
  count,rest=row.split(' ',1);actual=[bytes.fromhex(x)for x in rest.split(',')if x];diff=[{'record':j,'offsets':[k for k,(a,b)in enumerate(zip(e,g))if a!=b],'expected':e.hex(),'actual':g.hex()}for j,(e,g)in enumerate(zip(want,actual))if e!=g]
  if len(actual)!=len(want)or diff:failures.append({'case':i,'input':lines[i],'expected_count':len(want),'actual_count':len(actual),'differences':diff[:3]})
 out=ROOT/'evidence/building-production/B-wall-production-verification.json';report={'ok':not failures,'baseline':'B','binary_sha256':b['binary']['sha256'],'cases':len(cases),'instance_records':total,'failure_count':len(failures),'failures':failures[:12],'input_cases':cases,'candidate_binary_sha256':hashlib.sha256((ROOT/'reconstruction/target/release/wall-production-probe').read_bytes()).hexdigest(),'oracle_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in [Path(__file__),ROOT/'evidence/building-production/native_production.py',ROOT/'evidence/building-production/try_cases.py',ROOT/'evidence/building-constructor/native_fixture.py']},'scope':'Original full from_curve legal plainkind2/toptag2/fencefitfalse returns -> entire ordered96byte BrickTransformSsbo array. Raw source inputs only, candidate computes rows, row candidates, horizontal partitions, all RNG/noise, authority walking, rawproposals, context and actual cut/split/emission.','bridges':['PinnedB PE loaded; original privatehelpers run, no geometry/game/seed branch substitutions.','IAT-identified generic allocation/free/reallocate/memory and floorf/ceilf/sinf/truncf/roundf; profiler disabled.','Rust reuses fastrand1.8 and existingCurve2 instead of copying dependencies.'],'limitations':['Otherconstruction kinds/toptag0-or1/fencefittrue not accepted by plain API.','Bevy source-owner binding and lifecycle acceptance are parent integration work.','Explicit per-point array/finalized-hole metadata validity belongs to their true source owners; tests do not claim every arbitrary valid ABI fixture is emitted byWorld.'],**candidate_provenance()};out.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(cases),'records':total,'failures':len(failures),'first':failures[:1]}));assert not failures
if __name__=='__main__':
 if '--branches' in sys.argv:verify_branches()
 else:main()
