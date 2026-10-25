"""Unaccepted 16-chain failure diagnostic. Reuse frozen binaries; no source mutation."""
import sys,struct,json,subprocess,hashlib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'scripts/multi-version'))
from verify_wall_platform_stairs_consumer import evaluate,prepare_steps,partitions,render_produce,decode_segment,word

def parse(line):
 p=iter(line.split());f=lambda:struct.unpack('<f',struct.pack('<I',int(next(p),16)))[0]
 shape=int(next(p));special=int(next(p));maximum=f();extra=f();salt=int(next(p));varying=int(next(p));fix=int(next(p));lo=f();hi=f();rc=int(next(p));n=int(next(p));nr=int(next(p));raw=[[f(),f()]for _ in range(rc)];a=[[f()for _ in range(6)]for _ in range(n)];rows=[bytes.fromhex(next(p))for _ in range(nr)]
 return shape,special,maximum,extra,salt,varying,fix,lo,hi,raw,[x[:3]for x in a],[x[3]for x in a],[x[4]for x in a],[x[5]for x in a],rows

def main():
 failed=json.loads((ROOT/'evidence/building-segmentation/B-platform-stairs-consumer-first-failure.json').read_text());cases=[];inputs=[];wants=[]
 correct=struct.pack('<I6f',1,1.,0.,0.,0.,6.,4.);wrong=struct.pack('<I6f',1,0.,0.,.6,0.,6.,4.)
 for failure in failed['failures']:
  shape,special,maximum,extra,salt,varying,fix,lo,hi,raw,points,us,values,arch,rows=parse(failure['input']);encoded=[((j%512)*131+(j//512)*71 if varying else 0)+salt&65535 for j in range(262144)];blockers=[]
  for row in rows:
   out=evaluate(raw,row,encoded,lo,hi,fix);blockers.extend([(list(v[:2]),v[2])for v in struct.iter_unpack('<3f',bytes.fromhex(out['payload']))])
  segs,steps=prepare_steps(raw,points,us,values,arch,shape,special,maximum,blockers,(.2,6.,4.));cells=[]
  for si,s in enumerate(segs):
   pos,su=decode_segment(s);coords,length=partitions(pos,su,points,us)
   for ci,pair in enumerate(coords):
    cell,gpu,views=render_produce(points,pair,maximum,extra,encoded,lo,hi,fix,correct,27,True)
    suffix=f' 27 {salt} {varying} {fix} {word(lo)} {word(hi)} {word(maximum)} {word(extra)} '+' '.join(f'{a} {word(t)}'for a,t in pair)+' '+' '.join(word(v)for point in points for v in point)
    inputs.extend([correct.hex()+suffix,wrong.hex()+suffix]);wants.append(';'.join(r.hex()for r in gpu));cells.append({'source_segment':si,'pair':pair,'rise_count':cell['rise_count'],'native_records':len(gpu),'native_constructor_calls':views['constructor_calls']})
  cases.append({'case':failure['case'],'maximum':maximum,'extra':extra,'special':special,'segments':len(segs),'cells':cells})
 probe=ROOT/'reconstruction/target/release/wall-platform-stair-rows-probe';run=subprocess.run([str(probe)],input='\n'.join(inputs)+'\n',capture_output=True,text=True);assert run.returncode==0,run.stderr;out=run.stdout.splitlines();assert len(out)==len(inputs)
 cursor=0;failures=[];wrong_errors={};records=0
 for case in cases:
  for cell in case['cells']:
   correct_out=out[cursor*2];wrong_out=out[cursor*2+1];cell['correct_shape_matches_native']=correct_out==wants[cursor];cell['zero_axes_outcome']=wrong_out if wrong_out.startswith('error')else'non-error';wrong_errors[cell['zero_axes_outcome']]=wrong_errors.get(cell['zero_axes_outcome'],0)+1;records+=cell['native_records']
   if correct_out!=wants[cursor]:failures.append({'case':case['case'],'pair':cell['pair'],'rust':correct_out,'original':wants[cursor]})
   cursor+=1
 report={'ok':not failures,'cases':len(cases),'cells':cursor,'native_records96':records,'correct_shape_failures':failures,'zero_axes_outcomes':wrong_errors,'scope':'Frozen shared rows binary fed same original cell/shape/terrain as native. Draft full consumer probe mistakenly initialized rectangle axes to zero and centerX=.6, unlike native axes1/0 center0/0. Original failure retained. This diagnostic does not repair draft wholeconsumer or establish publisher reachability.','candidate_binary_sha256':hashlib.sha256(probe.read_bytes()).hexdigest(),'cases_detail':cases}
 (ROOT/'evidence/building-segmentation/B-platform-consumer-shape-diagnostic.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:report[k]for k in ['ok','cases','cells','native_records96','zero_axes_outcomes']}))
if __name__=='__main__':main()
