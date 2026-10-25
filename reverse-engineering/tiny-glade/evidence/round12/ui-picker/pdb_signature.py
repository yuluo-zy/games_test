from pathlib import Path
import sys,struct,json
sys.path.insert(0,str(Path(__file__).parents[2]/'utils-deep-followup-2026-10-14'))
from pdb_reader import Pdb,z
p=Pdb('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb');t={i:(k,b)for i,k,b in p.types(2)};ids={i:(k,b)for i,k,b in p.types(4)}
def decode(i,seen=0):
 if seen>7:return hex(i)
 k,b=t.get(i,(0,b''))
 if k==0x1002:return dict(pointer=decode(struct.unpack_from('<I',b)[0],seen+1))
 if k==0x1008:
  ret,cc,attr,n,args=struct.unpack_from('<IBBHI',b);return dict(return_type=decode(ret,seen+1),n=n,args=decode(args,seen+1))
 if k==0x1201:return [decode(x,seen+1)for x in struct.unpack_from('<'+'I'*struct.unpack_from('<I',b)[0],b,4)]
 if k in (0x1504,0x1505):return dict(index=hex(i),kind=hex(k),tail=b[16:].hex(),readable=z(b[18:]))
 return dict(index=hex(i),kind=hex(k),body=b.hex())
out={}
for i in (0x65de,0x8225,0x8224):
 k,b=ids[i];out[hex(i)]=dict(raw=b.hex(),signature=decode(struct.unpack_from('<I',b,4)[0]))
Path(__file__).with_suffix('.json').write_text(json.dumps(out,indent=2),encoding='utf8')
print(json.dumps(out,indent=2))
