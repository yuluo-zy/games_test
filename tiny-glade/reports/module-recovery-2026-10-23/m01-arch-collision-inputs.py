"""生成有界原construct_arch_geo输入；oracle由Rosetta执行原RNE，非候选算法。"""
from pathlib import Path
import struct,json,random,copy
r=Path(__file__).resolve().parent
bits=lambda f:struct.unpack('<I',struct.pack('<f',f))[0]
rows=[]
for nh in range(9):
 for np in range(9):
  rows.append(dict(heights=[bits(.25+i*.17)for i in range(nh)],positions=[[bits(i*.41),bits((i%3)*.13),bits((i%2)*.29)]for i in range(np)],seed=(nh+np)%3))
rng=random.Random(0x8d7ed0)
for i in range(192):
 n=rng.randrange(2,10)
 rows.append(dict(heights=[bits(rng.uniform(-8,8))for _ in range(n)],positions=[[bits(rng.uniform(-16,16))for _ in range(3)]for _ in range(n)],seed=i%3))
base=dict(heights=[bits(1.),bits(2.),bits(3.)],positions=[[bits(1.),bits(2.),bits(3.)],[bits(2.),bits(3.),bits(4.)],[bits(-2.),bits(.5),bits(5.)]],seed=1)
for value in [0,0x80000000,0x7f800000,0xff800000,0x7fc12345,0xffc54321,0x00000001,0x00800000,0x7f7fffff]:
 for axis in range(4):
  row=copy.deepcopy(base)
  if axis==3:row['heights'][0]=value
  else:row['positions'][0][axis]=value
  rows.append(row)
for n in range(2,8):
 rows.append(dict(heights=[bits(0.)]*n,positions=[[bits(1.),bits(2.),bits(3.)]for _ in range(n)],seed=0))
with Path('/tmp/m01-arch-collision-input.bin').open('wb')as f:
 for i,row in enumerate(rows):
  f.write(struct.pack('<4I',i,len(row['heights']),len(row['positions']),row['seed']))
  for group in [row['heights'],[x for p in row['positions']for x in p]]:
   f.write(struct.pack('<'+'I'*len(group),*group))
with (r/'m01-arch-collision-inputs.json').open('x')as f:json.dump(rows,f,indent=2)
print(len(rows))
