import json,sys,subprocess,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];sys.path.insert(0,str(ROOT/'evidence/building-production'))
from native_production import produce
from try_variants import variant_line
calls=json.loads((ROOT/'evidence/building-segmentation/B-stair-negative-base-input-research.json').read_text())['view']['constructor_calls'];cs=[dict(c,holes=[],width_padding=None,thickness_scale=None,top_bounds=[0.,0.,0.,0.,0.,2],fence_fit=False)for c in calls];expected=[produce(**c)for c in cs]
r=subprocess.run([str(ROOT/'reconstruction/target/release/wall-production-probe')],input='\n'.join(variant_line(c)for c in cs)+'\n',text=True,capture_output=True,check=True);results=[]
for c,ex,line in zip(cs,expected,r.stdout.splitlines()):
 count,payload=line.split(' ',1)if' 'in line else(line,'');got=[bytes.fromhex(x)for x in payload.split(',')if x]if not line.startswith('error')else[]
 dif=[dict(index=i,native=a.hex(),rust=b.hex(),offsets=[j for j,(x,y)in enumerate(zip(a,b))if x!=y])for i,(a,b)in enumerate(zip(ex,got))if a!=b];results.append(dict(input=c,native_count=len(ex),rust_count=count,equal=ex==got,differences=dif[:3]))
report=dict(ok=all(c['equal']for c in results),cases=results);(ROOT/'evidence/building-production/B-stair-empty-holes-regression.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report));
