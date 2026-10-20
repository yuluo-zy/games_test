from pathlib import Path
import json,struct
from native_trigger_oracle import run,make,SHA
out=Path(__file__).resolve().parent
cases=[]
for rect in [False,True]:
 for profile in [0.0,1.0]:
  for h,floor,rot,tip in [(0.10000000149011612,2.3,-6.5,(0.,0.)),(0.10000000894069672,-1.7,7.,(0.9,-0.7)),(0.8,4.,-0.75,(-0.8,0.9))]:
   b=make(rect,h,1.0,profile,dir=1,floor=floor,tip=tip,rotation=rot);v=run(b);cases.append({'roof_hex':b.hex(),**v});print(rect,profile,h,len(v['triangles'])//9,flush=True)
(out/'native-trigger-boundaries.json').write_text(json.dumps({'source_sha256':SHA,'cases':cases},separators=(',',':'))+'\n')
