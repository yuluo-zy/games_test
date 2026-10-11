#!/usr/bin/env python3
"""Original ratio decision and complete interior-post business block.
Surrounding EventReader/hash/resource acquisition is explicitly outside scope.
"""
import sys,json,struct,subprocess,hashlib,math
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];sys.path.insert(0,str(ROOT/'evidence/building-gates'))
from native_gate_fallback_research import project_system_business
from gate_mesh_reference_research import curve,F
proc=subprocess.Popen([str(ROOT/'reconstruction/target/release/gate-construction-probe')],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
fh=lambda vs:struct.pack('<'+'f'*len(vs),*vs).hex()
results=[]
for case in range(64):
 count=2+case%12;step=[.13,.25,.500001,.7][case%4];angle=.271*case;half=case%4==0
 points=[[F(i*step*math.cos(angle)),F(-.9 if not half or i<count//2 else .1),F(i*step*math.sin(angle)+(.04 if i%2 else 0))]for i in range(count)]
 heights=[F(2.3+.2*math.cos(i))for i in range(count)];_,us,_=curve(points);segment=dict(positions=points,heights=heights,us=us)
 original=project_system_business(segment)
 line=' '.join([fh([x for p in points for x in p]),fh(heights),fh(us),fh([x for p in points for x in p]),fh(us),fh([0.]),'5','12345','3','system']);proc.stdin.write(line+'\n');proc.stdin.flush();candidate=json.loads(proc.stdout.readline())
 errors=[]
 if original['branch']=='posts':
  if candidate.get('nanis')!=original['nanis'] or candidate.get('bricks')!=[] or candidate.get('leaves')!=[]:errors.append(dict(expected=original,actual=candidate))
 else:
  if not candidate.get('leaves'):errors.append(dict(expected_branch='wooden',actual=candidate))
 results.append(dict(case=case,count=count,half=half,step=step,branch=original['branch'],nani_count=len(original['nanis']),actual_edge=original['actual_edge'],errors=errors));print(case,original['branch'],len(original['nanis']),'match'if not errors else'FAIL',flush=True)
 files=['reconstruction/building/gates.rs','reconstruction/src/bin/gate-construction-probe.rs','scripts/multi-version/verify_gate_system_fallback.py','evidence/building-gates/native_gate_fallback_research.py','evidence/building-gates/native_gates_research.py']
 (ROOT/'evidence/building-gates/B-gate-system-fallback-verification.json').write_text(json.dumps(dict(status='system game ratio/whole interior-post CPU block; surrounding resource and lifecycle boundary pending',scope='Original140756970..140756e20 and wooden-edge140756d14; full native branching exits asserted',cases=results,bindings={f:hashlib.sha256((ROOT/f).read_bytes()).hexdigest()for f in files}),indent=2))
proc.stdin.close();proc.wait();assert proc.returncode==0
assert all(not c['errors']for c in results),'Source differences preserved'
