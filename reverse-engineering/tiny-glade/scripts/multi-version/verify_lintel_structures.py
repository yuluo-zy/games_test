"""Complete special-lintel CPU business chain; original caller preparation and third writer.
Native ECS EventReader/Query shell is outside this CPU proof, not replaced by game bridges.
"""
import sys,struct,json,math,random,subprocess,hashlib
from pathlib import Path
from verification_provenance import ROOT,candidate_provenance
sys.path.insert(0,str(ROOT/'evidence/building-structures'));import native_structures as native
sys.path.insert(0,str(ROOT/'evidence/building-segmentation'));from native_platform_segmentation import process
from verify_wall_structures import line,pack,f32

def raw_output(c,p284,p288,wall_id,third,prepared_only=False):
 native.materialize(**c);views=native.last_materialized_views[:]
 prepared=[native.prepare_current_lintel(v,c['points'],p284,p288)for v in views]
 third=native.current_third_accumulation(views,third) if views else third
 sources=[(*v,i)for i,v in enumerate(prepared,1)]
 if prepared_only:return sources,third
 return native.produce_support_batch(sources,wall_id,c['closed']),third

def compare(cases,lines,want):
 run=subprocess.run([str(ROOT/'reconstruction/target/release/wall-structures-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True);actual=run.stdout.splitlines();assert len(actual)==len(lines),(len(actual),len(lines),run.stderr)
 fail=[]
 for i,((rows,third),text)in enumerate(zip(want,actual)):
  if text.startswith('error'):fail.append(dict(case=i,input=cases[i],actual=text));continue
  count,payload,thirdpayload=text.split(' ',2);got=[bytes.fromhex(b)for b in payload.split(',')if b];assert int(count)==len(got)
  expectedthird=struct.pack('<'+'f'*len(third),*third).hex()
  diff=[dict(record=j,offsets=[k for k,(a,b)in enumerate(zip(e,g))if a!=b],native=e.hex(),rust=g.hex())for j,(e,g)in enumerate(zip(rows,got))if e!=g]
  if len(rows)!=len(got)or diff or expectedthird!=thirdpayload:fail.append(dict(case=i,input=cases[i],native_count=len(rows),rust_count=len(got),differences=diff[:3],native_third=expectedthird,rust_third=thirdpayload))
 return fail

def main():
 source=json.loads((ROOT/'evidence/building-structures/B-segment-materialization-verification.json').read_text())['input_cases'];cases=[];lines=[];want=[]
 for i,c in enumerate(source):
  p284=f32([.3,.8,1.4,2.1][i%4]);p288=f32([-.3,1.3,.6,2.7][i%4]);wallid=100000 if i%7==0 else 20015998343868+i
  third=[f32((j%3)*.2)for j in range(len(c['points']))];params=dict(source=c,public_284=p284,public_288=p288,wall_id=wallid,initial_third=third)
  cases.append(params);lines.append('lintel '+line(c)+' '+pack([p284])+' '+pack([p288])+' '+str(wallid)+' '+pack(third));want.append(raw_output(c,p284,p288,wallid,third))
 rawcases=len(cases)
 # True original calc -> platform splitting -> materializer chain, with the
 # Rust probe independently producing masks from the same prepared authority.
 chain_mask_count=0;chain_step_count=0
 for i in range(80):
  n=[9,13,25,37][i%4] if i<64 else [65,129,257,129][i%4];shape=i%2 if i<64 else 2
  points=[[f32(3.*math.cos(j*2.*math.pi/(n-1))),f32([0.,.3,1.2,2.1][(j+i)%4]),f32(3.*math.sin(j*2.*math.pi/(n-1)))]for j in range(n)];points[-1]=points[0]
  if i>=64:points=[[f32(j*.6),f32((j%3)*.1),f32(.2*math.sin(j*.3))]for j in range(n)]
  values=[f32([0.,.2,.8,1.5][(j+i)%4])for j in range(n)]
  if i>=64:values=[f32(.8+.4*(j%3))for j in range(n)]
  maximum=f32([2.5,3.7,4.2,5.5][i%4]);arch=[f32(.96+(j%5)*.3)for j in range(n)];us=[f32(j/(n-1))for j in range(n)];third=[f32(.1*(j%2))for j in range(n)]
  masks,steps=process(points,values,shape,1,maximum,0);chain_mask_count+=len(masks);chain_step_count+=len(steps);sources=[];p284=f32([.8,1.3,2.,2.5][i%4]);p288=f32([1.3,1.1,2.5,2.][i%4]);wallid=20015998344000+i
  initial=third[:]
  for m in masks:
   raw=bytes.fromhex(m['coords']);coords=[struct.unpack_from('<If',raw,j)for j in range(0,len(raw),8)]
   c=dict(flags=list(bytes.fromhex(m['flags'])),coords=coords,loop_range=list(struct.unpack('<3i',bytes.fromhex(m['range']))),segment_values=values,arch=arch,points=points,us=us,regular=shape<2,closed=shape!=2)
   rs,third=raw_output(c,p284,p288,wallid,third,True);sources.extend(rs)
  rows=native.produce_support_batch(sources,wallid,shape!=2)
  cases.append(dict(actual_calc_platform=True,points=points,values=values,maximum=maximum,shape=shape,public_284=p284,public_288=p288,wall_id=wallid,arch=arch,us=us,initial_third=initial,processed_masks=masks,step_ranges=steps));want.append((rows,third));lines.append('chain '+str(shape)+' '+pack([maximum])+' '+pack([p284])+' '+pack([p288])+' '+str(wallid)+' '+pack([f for p in points for f in p])+' '+pack(us)+' '+pack(values)+' '+pack(arch)+' '+pack(initial))
 failures=compare(cases,lines,want)
 sources=[Path(__file__),ROOT/'evidence/building-structures/native_structures.py',ROOT/'evidence/building-production/native_production.py',ROOT/'evidence/building-constructor/native_fixture.py',ROOT/'evidence/building-segmentation/native_segmentation.py',ROOT/'evidence/building-segmentation/native_platform_segmentation.py']
 report=dict(ok=not failures,baseline='B',binary_sha256=native.b['binary']['sha256'],cases=len(cases),raw_mask_cases=rawcases,actual_calc_platform_chain_cases=len(cases)-rawcases,actual_processed_mask_count=chain_mask_count,actual_step_count=chain_step_count,ordered_64B_records=sum(len(r)for r,t in want),cross_page_cases=sum(len(r)>32 for r,t in want),maximum_record_count=max(len(r)for r,t in want),failure_count=len(failures),failures=failures[:20],input_cases=cases,oracle_sources={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in sources},candidate_binary_sha256=hashlib.sha256((ROOT/'reconstruction/target/release/wall-structures-probe').read_bytes()).hexdigest(),scope='Raw/actual-produced masks + prepared arrays -> original14122ff90 -> originalcaller14076e65c..86e (height replacement and141204400) -> entire141242530/RDP/ArchWalker/diagonal -> actualSparseWriter64B ordered arrays. Native caller14076f1f4..2e7 actual third-height write included. Full outerECS Query/EventReader/Resource insertion and C wooden-arch owners remain outside CPU proof.',**candidate_provenance())
 path=ROOT/'evidence/building-structures/B-lintel-chain-verification.json';path.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:report[k]for k in ['ok','cases','raw_mask_cases','actual_calc_platform_chain_cases','actual_processed_mask_count','actual_step_count','ordered_64B_records','failure_count']}));print(json.dumps(failures[:1]));assert not failures
if __name__=='__main__':main()
