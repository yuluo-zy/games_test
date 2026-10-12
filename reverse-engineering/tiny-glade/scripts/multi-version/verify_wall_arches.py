"""Regular B arches whole CPU owner: masks->native profile->walker->96B and cache/deferred writes.
This intentionally does not claim shape2/freehand endpoint and cache-tail owners.
"""
import sys,json,struct,math,random,subprocess,hashlib
from pathlib import Path
from verification_provenance import ROOT,candidate_provenance
sys.path.insert(0,str(ROOT/'evidence/building-arches'));import native_arches as native
sys.path.insert(0,str(ROOT/'evidence/building-segmentation'));from native_platform_segmentation import process

def f32(x):return struct.unpack('<f',struct.pack('<f',x))[0]
def pack(v):return struct.pack('<'+'f'*len(v),*v).hex()
def mask_from_native(m):
 raw=bytes.fromhex(m['coords']);return dict(flags=list(bytes.fromhex(m['flags'])),coords=[struct.unpack_from('<If',raw,i)for i in range(0,len(raw),8)],range=list(struct.unpack('<3i',bytes.fromhex(m['range']))),length=struct.unpack('<f',bytes.fromhex(m['length']))[0])
def mask_line(m):return ','.join([bytes(m['flags']).hex(),b''.join(struct.pack('<If',i,t)for i,t in m['coords']).hex(),struct.pack('<3i',*m['range']).hex(),pack([m['length']])])
def line(c):return 'regular '+' '.join([str(c['shape']),str(c['flag']),str(int(c['roof'])),pack([c['wall_float']]),str(c['wall_id']),pack([c['source_length']]),pack([f for p in c['points']for f in p]),pack(c['us']),pack(c['values']),pack(c['arch']),pack(c['third']),';'.join(mask_line(m)for m in c['masks'])or'none'])
def original(c):
 allsegments=[];alltrims=[];gate_commands=[];event_ids=[];event_count=c['native_event_count'];ordinal=0
 for m in c['masks']:
  args=dict(flags=m['flags'],coords=m['coords'],loop_range=m['range'],segment_values=c['values'],arch=c['arch'],points=c['points'],us=c['us'],regular=True,closed=True)
  native.materialize(**args);views=native.structures_native.last_materialized_views[:];trims,segments=native.prepare_profiles_current(views,c['arch'],c['shape'],c['wall_float'],c['flag'],len(c['points']))
  if c['roof']and len(segments)==1:
   event=native.observe_gate_event_send(views[0],c['wall_id'],ordinal+1,event_count);gate_commands.append(event[:-8]);event_ids.append(struct.unpack('<Q',event[-8:])[0]);event_count+=1
  ordinal+=len(segments);allsegments.extend(segments);alltrims.extend(trims)
 arch,third=native.accumulate_regular_arch_profiles(allsegments,alltrims,c['arch'],c['third']);records=native.walk_arch_profiles(allsegments,c['wall_id'],c['source_length'])
 return dict(records=records,arch=arch,third=third,segments=ordinal,native_next_event_count=event_count,native_event_ids=event_ids,gate_commands=gate_commands)
def main():
 source=json.loads((ROOT/'evidence/building-structures/B-segment-materialization-verification.json').read_text())['input_cases'];cases=[]
 for i,s in enumerate(source):
  if not s['closed']:continue
  m=dict(flags=s['flags'],coords=s['coords'],range=s['loop_range'],length=0.)
  cases.append(dict(shape=i%2,flag=[0,1,255][i%3],roof=i%4<2,native_event_count=19+i,wall_float=f32([.5,1.3,5.,14.][i%4]),wall_id=100000+i if i%5==0 else 20015998343868+i,source_length=f32(12.3+(i%3)),points=s['points'],us=s['us'],values=s['segment_values'],arch=s['arch'],third=[f32((j%4)*.2)for j in range(len(s['points']))],masks=[m],actual_calc_platform=False))
 rawcases=len(cases)
 for i in range(80):
  n=[9,13,25,37][i%4] if i<64 else [65,129,257,129][i%4];shape=i%2
  radius=.2 if shape==0 else 3.
  points=[[f32(radius*math.cos(j*2*math.pi/(n-1))),f32([-.3,0.,.3,.8][(j+i)%4]),f32(radius*math.sin(j*2*math.pi/(n-1)))]for j in range(n)];points[-1]=points[0]
  values=[f32([0.,.2,.8,1.5][(j+i)%4])for j in range(n)]
  if i>=64:values=[f32(.8+.4*(j%3))for j in range(n)]
  maximum=f32([2.5,3.7,4.2,5.5][i%4]);roof=i%3!=0
  try:ms,steps=native.process_platform_original(points,values,shape,0,maximum,0 if roof else None)
  except Exception as error:
   print('upstream failure case',i,'count',n,'shape',shape,'RIP',hex(native.cpu.reg_read(native.UC_X86_REG_RIP)));raise
  masks=[mask_from_native(m)for m in ms]
  cases.append(dict(shape=shape,flag=i%2,roof=roof,native_event_count=100+i,wall_float=f32([.5,1.3,5.,14.][i%4]),wall_id=20015998344000+i,source_length=f32(2*math.pi*radius),points=points,us=[f32(j/(n-1))for j in range(n)],values=values,arch=[f32(.96+(j%5)*.3)for j in range(n)],third=[f32((j%4)*.2)for j in range(n)],masks=masks,actual_calc_platform=True,steps=steps))
 expected=[original(c)for c in cases]
 run=subprocess.run([str(ROOT/'reconstruction/target/release/wall-arches-probe')],input='\n'.join(line(c)for c in cases)+'\n',text=True,capture_output=True,check=True);actual=run.stdout.splitlines();assert len(actual)==len(cases),(len(actual),len(cases),run.stderr);fail=[]
 for i,(e,text)in enumerate(zip(expected,actual)):
  if text.startswith('error'):fail.append(dict(case=i,input=cases[i],actual=text));continue
  fields=text.split(' ',5);assert len(fields)==6,text
  count,payload,arch,third,segments,deferred=fields;records=[bytes.fromhex(s)for s in payload.split(',')if s];ds=[]if deferred=='none'else[bytes.fromhex(s)for s in deferred.split(';')]
  differences=[dict(record=j,offsets=[k for k,(a,b)in enumerate(zip(n,r))if a!=b],native=n.hex(),rust=r.hex())for j,(n,r)in enumerate(zip(e['records'],records))if n!=r]
  if len(e['records'])!=len(records)or differences or pack(e['arch'])!=arch or pack(e['third'])!=third or e['segments']!=int(segments)or e['gate_commands']!=ds:
   fail.append(dict(case=i,input=cases[i],native_count=len(e['records']),rust_count=len(records),differences=differences[:3],native_arch=pack(e['arch']),rust_arch=arch,native_third=pack(e['third']),rust_third=third,native_gate_commands=[x.hex()for x in e['gate_commands']],rust_gate_commands=[x.hex()for x in ds]))
 oracle=[Path(__file__),ROOT/'evidence/building-arches/native_arches.py',ROOT/'evidence/building-structures/native_structures.py',ROOT/'evidence/building-production/native_production.py',ROOT/'evidence/building-constructor/native_fixture.py',ROOT/'evidence/building-segmentation/native_segmentation.py',ROOT/'evidence/building-segmentation/native_platform_segmentation.py']
 report=dict(ok=not fail,baseline='B',binary_sha256=native.b['binary']['sha256'],cases=len(cases),raw_mask_cases=rawcases,actual_calc_platform_cases=len(cases)-rawcases,ordered_records=sum(len(e['records'])for e in expected),gate_commands=sum(len(e['gate_commands'])for e in expected),maximum_records=max(len(e['records'])for e in expected),cross_page_cases=sum(len(e['records'])>32 for e in expected),failure_count=len(fail),failures=fail[:20],input_cases=cases,native_only_event_id_observations=[e['native_event_ids']for e in expected],oracle_sources={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()for p in oracle},candidate_binary_sha256=hashlib.sha256((ROOT/'reconstruction/target/release/wall-arches-probe').read_bytes()).hexdigest(),scope='Regular shapes0/1 complete CPU chain, true profile1407645f0 incl authorclosure1412083a0, wholewalker141232bb0/gameclosure141231af0/default/writer96, true caller third+closedcopies and120B ConstructGateCmd canonical Veccontents/counts+range/source pair+WallId/ordinal; native-only Bevy EventId excluded. No claim outerECS/GPU or shape2 endpoint/Clutter initial tail. Actualproducer masks generated by original calc+platform upstream; candidate CPU API consumes safe typedprojection.',**candidate_provenance())
 p=ROOT/'evidence/building-arches/B-regular-arches-verification.json';p.write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:report[k]for k in ['ok','cases','ordered_records','gate_commands','maximum_records','cross_page_cases','failure_count']}));print(json.dumps(fail[:1]));assert not fail
if __name__=='__main__':main()
