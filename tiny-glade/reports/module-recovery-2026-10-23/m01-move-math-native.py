from pathlib import Path
import argparse,subprocess
parser=argparse.ArgumentParser();parser.add_argument("--prefix",default="/tmp/m01-move-math-replay");args=parser.parse_args();prefix=Path(args.prefix)
import struct,hashlib,random,json
root=Path('/Users/yuluo/Desktop/game');raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
def pack(xs):return struct.pack('<'+'f'*len(xs),*xs)
def bits(xs):return list(struct.unpack('<'+'I'*len(xs),pack(xs)))
def f(x):return struct.unpack('<f',pack([x]))[0]
r=random.Random(0x147a4850)
rows=[]
def add(kind,values):rows.append([len(rows),kind]+values)
for v in [[0.,0.,0.],[1.,2.,3.],[-1.,0.,0.],[0.,1.,-1.],[1e-30,0.,-1e-30],[1e30,0.,1e30],[float('inf'),0.,1.],[0.,float('nan'),1.]]+[[f(r.uniform(-10,10)),f(r.uniform(-10,10)),f(r.uniform(-10,10))]for _ in range(120)]:add(1,bits(v))
for _ in range(128):add(2,bits([f(r.uniform(-2,2))for _ in range(8)]))
for _ in range(128):add(3,bits([f(r.uniform(-3,3))for _ in range(64)]+[f(r.uniform(-10,10))for _ in range(3)]+[f(r.uniform(100,3000)),f(r.uniform(100,2000))]))
for ty in [1,0x3f]:
 for tag in [0,2,3]:
  for height in [0.,.001,.01,1.,2.,float('nan')]:
   for cursor in [-2.,1.,2.]:add(4,[ty,tag]+bits([height,1.,cursor]))
Path(str(prefix)+'.input.bin').write_bytes(b''.join(struct.pack('<128I',*(row+[0]*(128-len(row))))for row in rows))
Path(str(prefix)+'.input.json').write_text(json.dumps(rows))
print('original input',len(rows))

source=Path(__file__).resolve().parent
subprocess.run(['clang','-arch','x86_64','-O1','-fno-omit-frame-pointer',str(source/'m01-move-math-native.c'),str(source/'m01-move-math-native.S'),'-o',str(prefix)],check=True)
subprocess.run(['arch','-x86_64',str(prefix),str(root/'B版本内容/tiny-glade.rne'),str(prefix)+'.input.bin',str(prefix)+'.output.bin'],check=True)
results=list(struct.iter_unpack('<6I',Path(str(prefix)+'.output.bin').read_bytes()))
assert len(results)==len(rows)==492
for row,result in zip(rows,results):assert result[0]==row[0] and result[1]==0
code_hashes={}
for name,start,length in [('rotation',0xc91640,619),('projection',0xac8780,174),('product_segment',0x7a665b,96),('grip_entry',0x7a523e,140)]:
 import pefile
 native=pefile.PE(data=raw,fast_load=True)
 code_hashes[name]=hashlib.sha256(native.get_data(start,length)).hexdigest()
Path(str(prefix)+'.json').write_text(json.dumps(dict(original_sha256=sha,code_hashes=code_hashes,scope='128 full rotation +128 full projection +128 original quaternion blocks +108 original grip blocks; no database, no emulated resource lookup',rows=rows,results=results),indent=2))
print('verified original outputs',len(results))
