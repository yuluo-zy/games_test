#!/usr/bin/env python3
"""G06 whole-original wooden-door CPU vs Rust projection.
Runtime/entity lifecycle and the full system resource envelope remain separate.
"""
import sys,json,struct,subprocess,hashlib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'evidence/building-gates'))
import native_gates_research as native
exe=ROOT/'reconstruction/target/release/gate-construction-probe'
proc=subprocess.Popen([str(exe)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
fh=lambda vs:struct.pack('<'+'f'*len(vs),*vs).hex()
flat=lambda ps:[x for p in ps for x in p]
results=[]
cases=[]
for width in [1.3,2.7,2.70001,4.,6.1]:
 for profile in ['flat','sloped']:
  points=[[0.,0.,0.],[width*.3,.2 if profile=='sloped' else 0.,.7 if profile=='sloped' else 0.],[width,.9 if profile=='sloped' else 0.,0.]]
  heights=[1.5,2.8,2.1]if profile=='sloped' else [2.,2.,2.];us=[0.,.3,1.];scalar=.7 if profile=='sloped' else 0.;wid=100011 if width>4 else 12345;byte=2 if profile=='sloped'else 1
  cases.append((width,profile,points,heights,us,scalar,wid,byte))
import math
from gate_mesh_reference_research import curve,F
for ordinal in range(18):
 count=[2,3,4,7,9,13][ordinal%6];width=[1.8,3.2,5.9][ordinal%3];angle=ordinal*.427
 points=[[F(-3.+(width*i/(count-1))*math.cos(angle)),F(.35*i/(count-1)),F(2.+(width*i/(count-1))*math.sin(angle)+(.15 if 0<i<count-1 else 0))]for i in range(count)]
 heights=[F(3.9+.4*math.sin(i+.2*ordinal))for i in range(count)];_,us,_=curve(points)
 cases.append((width,f'rotated-high-{ordinal}',points,heights,us,.2,100011 if ordinal%2 else 12345,5))
for width,profile,points,heights,us,scalar,wid,byte in cases:
  segment=dict(positions=points,heights=heights,us=us)
  original=native.run_complete(segment,points,us,scalar,wid,3,byte)
  line=' '.join([fh(flat(points)),fh(heights),fh(us),fh(flat(points)),fh(us),fh([scalar]),str(byte),str(wid),'3'])
  proc.stdin.write(line+'\n');proc.stdin.flush();candidate=json.loads(proc.stdout.readline())
  expected=[]
  for m,g in zip(original['mesh_inputs'],original['gate_views']):
   v={k:x for k,x in g.items()if k not in['mesh_handle','shader_handle','transform']};v['indices']=m['indices'];v['attributes']={a['name']:a['data']for a in m['attributes']};expected.append(v)
  queue=bytes.fromhex(original['command_queue']);nanis=[(queue[i+8:i+48]+queue[i+56:i+60]+queue[i+64:i+73]).hex()for i in range(0,len(queue),136)]
  errors=[]
  if candidate.get('nanis')!=nanis:errors.append(dict(field='nanis',expected=nanis,actual=candidate.get('nanis')))
  if candidate.get('bricks')!=original['bricks']:errors.append(dict(field='support_bricks',expected=original['bricks'],actual=candidate.get('bricks')))
  if 'error'in candidate:errors.append(candidate)
  elif len(candidate['leaves'])!=len(expected):errors.append('leaf count')
  else:
   for i,(a,b)in enumerate(zip(candidate['leaves'],expected)):
    for field in b:
     if a[field]!=b[field]:errors.append(dict(leaf=i,field=field,expected=b[field],actual=a[field]))
  result=dict(width=width,profile=profile,original_return=True,support_records=len(original['bricks']),nani_records=len(nanis),mesh_records=len(expected),vertices=[len(bytes.fromhex(m['indices']))//4 for m in original['mesh_inputs']],errors=errors)
  results.append(result);print(width,profile,'match'if not errors else[(e.get('leaf'),e.get('field'))for e in errors if isinstance(e,dict)],flush=True)
  files=['reconstruction/building/gates.rs','reconstruction/building/wall_production.rs','reconstruction/building/wall_constructor.rs','reconstruction/building/wall_collection.rs','reconstruction/building/wall_construction_sources.rs','reconstruction/src/bin/gate-construction-probe.rs','scripts/multi-version/verify_gate_construction.py','evidence/building-gates/native_gates_research.py','evidence/building-arches/native_arches.py','evidence/building-structures/native_structures.py','evidence/building-production/native_production.py','evidence/building-constructor/native_fixture.py']
  (ROOT/'evidence/building-gates/B-gate-mesh-draft-verification.json').write_text(json.dumps(dict(status='whole wooden-door CPU scoped proof; system/runtime review pending',scope=['mesh positions/normals/colors/UV/indices','top positions','source seed/id/observed29a','prepared endpoint directions','support96B rows','complete head/footer Nani meaningful payload'],not_verified=['system resource acquisition and reader envelope','actualWorld gate add/update/delete'],cases=results,source_sha256='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03',candidate_binary_sha256=hashlib.sha256(exe.read_bytes()).hexdigest(),bindings={f:hashlib.sha256((ROOT/f).read_bytes()).hexdigest()for f in files}),indent=2))
proc.stdin.close();proc.wait();assert proc.returncode==0
assert all(not c['errors']for c in results),'Native differences retained; no acceptance'
