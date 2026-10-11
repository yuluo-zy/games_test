import sys,json,subprocess,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'evidence/building-production'))
from native_production import produce
from try_variants import variant_line
c=dict(points=[[0.,0.],[1.,0.]],us=[0.,1.],length=1.,arch=[1.6375]*2,second=[2.085]*2,third=[0.]*2,holes=[],seed=0,material=23,closed=False,base_height=2.085,perturb=.2,width_padding=None,thickness_scale=None,wall_float=-.9075,kind=0,top_bounds=[0.,0.,0.,0.,0.,2],fence_fit=False)
try:
 expected=produce(**c);text=subprocess.run([str(ROOT/'reconstruction/target/release/wall-production-probe')],input=variant_line(c)+'\n',text=True,capture_output=True,check=True).stdout.strip()
 count,payload=text.split(' ',1)if' 'in text else(text,'');got=[bytes.fromhex(p)for p in payload.split(',')if p];result=dict(input=c,native_count=len(expected),rust_count=count,equal=expected==got,native=[r.hex()for r in expected],rust=text)
except Exception as e:result=dict(input=c,error=repr(e))
(ROOT/'evidence/building-arches-freehand/B-kind0-negative-band-research.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))
