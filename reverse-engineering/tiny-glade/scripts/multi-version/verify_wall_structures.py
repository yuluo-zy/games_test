"""Original136B mask owner -> complete104B segment semantic output differential.
This is the front owner of G05, not acceptance of full lintel/arch output.
"""
import sys,json,struct,subprocess,random,hashlib,math
from pathlib import Path
from verification_provenance import ROOT,candidate_provenance
sys.path.insert(0,str(ROOT/'evidence/building-structures'))
from native_structures import materialize,b,thunk,iat,name

def f32(v):return struct.unpack('<f',struct.pack('<f',v))[0]
def pack(v):return struct.pack('<'+'f'*len(v),*v).hex()
def line(c):
 return f"{int(c['closed'])} {int(c['regular'])} "+' '.join(map(str,c['loop_range']))+' '+bytes(c['flags']).hex()+' '+b''.join(struct.pack('<If',i,t)for i,t in c['coords']).hex()+' '+pack([f for p in c['points']for f in p])+' '+pack(c['us'])+' '+pack(c['segment_values'])+' '+pack(c['arch'])
def main():
 cases=[];rng=random.Random(20261015)
 for i in range(96):
  closed=i%2==0;n=5+i%13;mod=n-1 if closed else n
  points=[[f32(2*math.cos(j*2*math.pi/(n-1))),f32(rng.uniform(-2.,.7)),f32(2*math.sin(j*2*math.pi/(n-1)))]for j in range(n)] if closed else [[f32(j*.6),f32(rng.uniform(-2.,.7)),f32(.2*(j%3))]for j in range(n)]
  if closed:points[-1]=points[0]
  start=-1 if closed and i%4==0 else 0;end=start+n
  flags=[0]*n;flags[0]=8;flags[-1]=8
  for j in range(1,n-1):
   if j%4==i%4:flags[j]=[1,2,4,8][i%4]
  coords=[(j,0.)for j in range(n-1)]+[(n-2,1.)]
  if i%3==0:coords[0]=(0,f32(.23));coords[-1]=(n-2,f32(.71))
  if i%5==0:
   for j in range(1,n-1):
    if flags[j]:coords[j]=(j,f32(.4))
  cases.append(dict(flags=flags,coords=coords,loop_range=[start,end,mod],segment_values=[f32(rng.uniform(-.1,2.))for _ in range(n)],arch=[f32(rng.uniform(.96,2.6))for _ in range(n)],points=points,us=[f32(j/(n-1))for j in range(n)],regular=i%3!=1,closed=closed))
 # Fewer than two anchors returns the complete empty owner result.
 for flags in [[0]*5,[0,0,8,0,0]]:
  c={**cases[0],'points':cases[0]['points'][:5],'us':cases[0]['us'][:5],'segment_values':cases[0]['segment_values'][:5],'arch':cases[0]['arch'][:5],'flags':flags,'coords':[(0,0),(1,0),(2,0),(3,0),(3,1)],'loop_range':[0,5,4]};c['points'][-1]=c['points'][0];cases.append(c)
 want=[materialize(**c)for c in cases];lines=[line(c)for c in cases];run=subprocess.run([str(ROOT/'reconstruction/target/release/wall-structures-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True);actual=run.stdout.splitlines();assert len(actual)==len(cases);failures=[]
 for index,(rows,row)in enumerate(zip(want,actual)):
  if row.startswith('error'):failures.append({'case':index,'source':cases[index],'expected_count':len(rows),'actual':row});continue
  n,s=row.split(' ',1);got=[bytes.fromhex(x)for x in s.split(',')if x];assert int(n)==len(got)
  diff=[{'record':j,'offsets':[k for k,(e,g)in enumerate(zip(a,z))if e!=g],'expected':a.hex(),'actual':z.hex()}for j,(a,z)in enumerate(zip(rows,got))if a!=z]
  if len(rows)!=len(got)or diff:failures.append({'case':index,'source':cases[index],'expected_count':len(rows),'actual_count':len(got),'differences':diff[:3]})
 files=[Path(__file__),ROOT/'evidence/building-structures/native_structures.py',ROOT/'evidence/building-production/native_production.py',ROOT/'evidence/building-constructor/native_fixture.py'];report={'ok':not failures,'baseline':'B','binary_sha256':b['binary']['sha256'],'cases':len(cases),'segment_records':sum(map(len,want)),'failure_count':len(failures),'failures':failures,'input_cases':cases,'oracle_sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in files},'candidate_binary_sha256':hashlib.sha256((ROOT/'reconstruction/target/release/wall-structures-probe').read_bytes()).hexdigest(),'CRT_bridge':{'thunk':hex(thunk),'iat':hex(iat),'name':name},'scope':'Whole original14122ff90 actual returned owner: each ordered segment canonicalized3 ownedVec contents/counts+sourceinterval2usize+LoopRange12. Does not compare allocation addresses/caps or last4 unproven padding. Does not claim G05 lintel support/SSBO owner yet.',**candidate_provenance()};p=ROOT/'evidence/building-structures/B-segment-materialization-verification.json';p.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(cases),'records':report['segment_records'],'failures':len(failures),'first':failures[:1]}));assert not failures
if __name__=='__main__':main()
