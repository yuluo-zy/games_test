import emu,struct,sys,re,collections,json
emu.BCMP=0x142923bf0
from emu import *
import run1,run2
rows=[]
for c in run1.cands:
    l=run2.go(c)
    if not l: continue
    r=run1.run(c)
    k,v,key,fn=r[1]
    rows.append((l,c,k,v,key,fn))
rows.sort()
for l,c,k,v,key,fn in rows: print(hex(l),c,k,v,key,hex(fn))
json.dump(rows,open('/tmp/dg/table.json','w'))
