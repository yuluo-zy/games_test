"""有界楼板原几何输入；输出由Rosetta原entry→Trimesh::builder边界获取。"""
from pathlib import Path
import math,random,struct,copy,json
p=Path(__file__).resolve();rows=[];bits=lambda f:struct.unpack('<I',struct.pack('<f',f))[0]
def row(n,kind=0,slope=0.,adjust=(None,None)):
 points=[[bits(i*.7),bits(i*slope),bits(.1*math.sin(i*.2))]for i in range(n)]
 aux=[0. if kind==0 else .15 if kind==1 else .2 if kind==2 else [.0,.3][i%2]if kind==3 else .3*i/max(n-1,1)if kind==4 else .3*(1.-i/max(n-1,1))for i in range(n)]
 return dict(widths=[bits(1.+.03*(i%3))for i in range(n)],tangents=[[bits(1.),bits(0.)]for _ in range(n)],positions=points,auxiliary=list(map(bits,aux)),source=100003,endpoints=[None if v is None else bits(v)for v in adjust])
for n in [2,3,4,8,17,65,129]:
 for kind in range(6):
  for slope in [-.4,0.,.4]:
   for adjust in [(None,None),(.0,.1)]:rows.append(row(n,kind,slope,adjust))
rng=random.Random(0x7dc170)
for i in range(80):
 n=rng.randrange(2,13);r=row(n,i%6,.13,([None,.1,.3][i%3],[None,.0,.2][(i+1)%3]))
 r['widths']=[bits(rng.uniform(.2,2.))for _ in range(n)];r['positions']=[[bits(j*.6),bits(rng.uniform(-1.,2.)),bits(rng.uniform(-.3,.3))]for j in range(n)];r['auxiliary']=[bits(rng.uniform(-.1,.4))for _ in range(n)];rows.append(r)
base=row(4,3,.3,(.0,.1))
for key in ['widths','tangents','positions','auxiliary']:
 for count in [0,1,2,3]:
  r=copy.deepcopy(base);r[key]=r[key][:count];rows.append(r)
for value in [0,0x80000000,0x7f800000,0xff800000,0x7fc12345,0xffc54321,0x7f7fffff,0x00000001]:
 for key in ['widths','tangents','positions','auxiliary','endpoints']:
  r=copy.deepcopy(base)
  if key in ['tangents','positions']:r[key][1][0]=value
  else:r[key][1]=value
  rows.append(r)
with p.with_suffix('.json').open('x')as f:json.dump(rows,f,indent=2)
with Path('/tmp/m01-stair-floors-input.bin').open('wb')as f:
 for i,r in enumerate(rows):
  f.write(struct.pack('<6I',i,len(r['widths']),len(r['tangents']),len(r['positions']),len(r['auxiliary']),r['source']))
  f.write(struct.pack('<4I',*[v for x in r['endpoints']for v in [int(x is not None),0 if x is None else x]]))
  for key in ['widths','tangents','positions','auxiliary']:
   words=[x for p in r[key]for x in p]if key in ['tangents','positions']else r[key];f.write(struct.pack('<'+'I'*len(words),*words))
print(len(rows))
