"""执行B原完整Intent及角度函数；仅panic/CRT边界桥接，保留活跃载荷差分。"""
import argparse,hashlib,subprocess
parser=argparse.ArgumentParser();parser.add_argument('--output-prefix',default='/tmp/m01-intent-stair-replay');args=parser.parse_args()
from pathlib import Path
root=Path(__file__).resolve().parents[3]
# reports/module-recovery位于tiny-glade内，上溯三层回workspace。
original=root/'B版本内容/tiny-glade.rne'
assert hashlib.sha256(original.read_bytes()).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
prefix=Path(args.output_prefix)
from pathlib import Path
import random,struct,json,math
r=random.Random(273117)
def f(x):return struct.unpack('<f',struct.pack('<f',x))[0]
def bits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
rows=[];source=[]
for k in range(130):
 row=[0]*128;row[0]=k;row[1]=3 if k<128 else 2;row[2]=0x3c if k!=129 else 0x38
 pts=[[f(-4.),f(-2.)],[f(0.8),f(r.uniform(-4,4))],[f(5.),f(2.)]] if k%3 else [[-4.,-2.],[5.,2.]]
 tans=[[f(r.uniform(-1.2,1.2)),f(r.uniform(-1.2,1.2))] for _ in pts]
 if k%17==0:tans=[[0.,0.] for _ in pts]
 widths=[f(r.uniform(0.4,3.7)) for _ in pts]
 x,y,z=f(r.uniform(-8,8)),f(r.uniform(-2,7)),f(r.uniform(-6,6));q=[0.,0.,0.,1.]
 dist=0.;pu=[0.]
 for a,b in zip(pts,pts[1:]):
  dx=f(a[0]-b[0]);dy=f(a[1]-b[1]);dist=f(dist+f(math.sqrt(f(f(dx*dx)+f(dy*dy)))));pu.append(dist)
 pu=[f(x/dist) for x in pu]
 row[3:6]=map(bits,[x,y,z]);row[6:10]=map(bits,q);row[10]=len(pts);row[11]=bits(dist)
 for i,p in enumerate(pts):row[16+i*2:18+i*2]=map(bits,p)
 for i,p in enumerate(tans):row[32+i*2:34+i*2]=map(bits,p)
 row[56:56+len(pts)]=map(bits,widths);row[64:64+len(pts)]=map(bits,pu)
 rows.append(row);source.append(dict(row=k,points=pts,tangents=tans,widths=widths,points_u=pu,length=dist,position=[x,y,z]))
Path(str(prefix)+'-input.bin').write_bytes(b''.join(struct.pack('<128I',*row) for row in rows));Path(str(prefix)+'-input.json').write_text(json.dumps(source))
for k in range(96):
 row=[0]*128;row[0]=130+k;row[1]=16
 q=[f(r.uniform(-4,4)) for _ in range(4)]
 if k==0:q=[0.,0.,0.,0.]
 if k==1:q=[-0.,0.,-0.,1.]
 if k==2:q=[f(float('nan')),0.,0.,1.]
 if k==3:q=[0.,float('inf'),0.,1.]
 if k==4:q=[-float('inf'),0.,0.,1.]
 row[6:10]=map(bits,q);rows.append(row)
Path(str(prefix)+'-input.bin').write_bytes(b''.join(struct.pack('<128I',*row) for row in rows))

source=Path(__file__).with_name('probe_m01_intent_stair_native.c')
subprocess.run(['clang','-arch','x86_64','-O2',str(source),'-o',str(prefix)],check=True)
subprocess.run(['arch','-x86_64',str(prefix),str(original),str(prefix)+'-input.bin',str(prefix)+'-output.bin'],check=True)
observed=list(struct.iter_unpack('<26I',Path(str(prefix)+'-output.bin').read_bytes()))
expected=json.loads(Path(__file__).with_name('m01-intent-stair-direct-replay.json').read_text())['output_words']
assert len(observed)==226
for i,(actual,old) in enumerate(zip(observed,expected)):
 assert actual[1]==old[1]==0
 if i<128: assert actual[14:16]==tuple(old[14:16]),(i,actual[14:16],old[14:16])
 elif i<130: assert actual[2]==old[2],(i,actual[2],old[2])
 else: assert actual[2:4]==tuple(old[2:4]),(i,actual[2:4],old[2:4])
print('PASS 128 stair active coordinates /2 terrain tags /96 atan inputs; Darwin atan2 bridge remains an explicit boundary')
