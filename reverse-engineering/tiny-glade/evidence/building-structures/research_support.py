import sys,struct,subprocess,json,math,random
from pathlib import Path
from native_structures import *
BIN=ROOT/'reconstruction/target/release/wall-structures-probe'
def fh(v):return struct.pack('<'+'f'*len(v),*v).hex()
def ph(v):return fh([x for p in v for x in p])
cases=[]
for n in [2,3,4,7,13,31]:
 for span in [.5,1.,1.4175,2.,3.,3.15,4.,7.,12.]:
  h=[1.3]*n;g=[[span*i/(n-1),0.,0.]for i in range(n)];r=[[p[0],1.3,0.]for p in g];cases.append((h,g,r))
for seed in range(80):
 rng=random.Random(seed);n=rng.choice([3,5,9,17,25]);span=rng.uniform(.5,15);h=[rng.uniform(.6,2.)for _ in range(n)];g=[[span*i/(n-1),rng.uniform(-.2,.2),math.sin(i/(n-1)*rng.choice([1,3,6]))*.3]for i in range(n)];r=[[p[0],p[1]+v,p[2]]for p,v in zip(g,h)];cases.append((h,g,r))
lines=['support 20015998343868 1 '+fh(h)+' '+ph(g)+' '+ph(r)for h,g,r in cases]
out=subprocess.run([str(BIN)],input='\n'.join(lines)+'\n',text=True,capture_output=True,check=True).stdout.splitlines();assert len(out)==len(cases)
fail=[];records=0
for i,((h,g,r),line)in enumerate(zip(cases,out)):
 n=len(h);flags=[8]+[0]*(n-2)+[8];coords=[(0,0)]*(n-1)+[(n-2,1)]
 native,growth=produce_support_instances(h,g,r,flags,coords,20015998343868,1,False);assert growth==0
 a=line.split(' ',1);rust=[bytes.fromhex(v)for v in a[1].split(',')if v];records+=len(native)
 if native!=rust:
  dif=[]
  for j,(b,c)in enumerate(zip(native,rust)):
   if b!=c:dif.append({'record':j,'native':struct.unpack('<12f4I',b),'rust':struct.unpack('<12f4I',c)})
  fail.append({'case':i,'input':dict(heights=h,ground=g,raised=r),'native_count':len(native),'rust_count':len(rust),'diff':dif[:2]})
report=dict(cases=len(cases),records=records,failures=len(fail),first=fail[:4]);(ROOT/'evidence/building-structures/B-support-research.json').write_text(json.dumps(report,indent=2));print(json.dumps(report))
