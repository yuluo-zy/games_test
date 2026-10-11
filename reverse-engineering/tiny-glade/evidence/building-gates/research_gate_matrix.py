"""Research only: full original Gate returns, canonical CPU output projections.
No Rust comparison or completion claim. Dependency handles/entity IDs excluded.
"""
import json,hashlib,struct
from pathlib import Path
import native_gates_research as n
R=Path(__file__).resolve().parent
cases=[]
for width in [1.3,2.7,2.70001,4.,6.1]:
 for profile in ['flat','sloped']:
  points=[[0.,0.,0.],[width*.3,.2 if profile=='sloped'else 0.,.7 if profile=='sloped'else 0.],[width,.9 if profile=='sloped'else 0.,0.]]
  heights=[1.5,2.8,2.1]if profile=='sloped'else[2.,2.,2.]
  cases.append(dict(name=f'{profile}-{width}',segment=dict(positions=points,heights=heights,us=[0.,.3,1.]),scalar=.7 if profile=='sloped'else 0.,wall_id=100011 if width>4 else 12345,ordinal=3,pub29a=2 if profile=='sloped'else 1))
results=[]
for c in cases:
 seg=c['segment'];v=n.run_complete(seg,seg['positions'],seg['us'],scalar=c['scalar'],wall_id=c['wall_id'],ordinal=c['ordinal'],pub29a=c['pub29a'])
 canonical=dict(bricks=v['bricks'],meshes=[{k:m[k]for k in['indices','attributes']}for m in v['mesh_inputs']],gate_views=[{k:x for k,x in g.items()if k not in['mesh_handle','shader_handle']}for g in v['gate_views']])
 result=dict(input=c,actual_return=True,bricks=len(v['bricks']),meshes=len(v['mesh_inputs']),vertices=[len(bytes.fromhex(m['indices']))//4 for m in v['mesh_inputs']],nani_commands=len(bytes.fromhex(v['command_queue']))//136,constructor_calls=v['constructor_calls'],canonical_sha256=hashlib.sha256(json.dumps(canonical,sort_keys=True).encode()).hexdigest())
 results.append(result);print(c['name'],result['bricks'],result['vertices'],result['nani_commands'],flush=True)
 (R/'B-full-gate-owner-research-matrix.json').write_text(json.dumps(dict(status='original CPU research; not candidate validation',source_sha256="f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03",oracle_sha256=hashlib.sha256((R/'native_gates_research.py').read_bytes()).hexdigest(),cases=results),indent=2))
