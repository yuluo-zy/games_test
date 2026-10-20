from pathlib import Path
import mmap,struct,json
p=Path('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb');f=p.open('rb');m=mmap.mmap(f.fileno(),0,access=mmap.ACCESS_READ);u=lambda o:struct.unpack_from('<I',m,o)[0];bs=u(32);ds=u(44);blocks=(ds+bs-1)//bs;bm=u(52)*bs;db=struct.unpack_from('<'+'I'*blocks,m,bm);directory=b''.join(m[b*bs:(b+1)*bs] for b in db)[:ds];ns=struct.unpack_from('<I',directory)[0];sizes=struct.unpack_from('<'+'I'*ns,directory,4);at=4+ns*4;tpi=None
for i,n in enumerate(sizes):
 if n==0xffffffff:continue
 nb=(n+bs-1)//bs;ids=struct.unpack_from('<'+'I'*nb,directory,at);at+=nb*4
 if i==2:tpi=b''.join(m[b*bs:(b+1)*bs] for b in ids)[:n];break
h,begin,end=struct.unpack_from('<3I',tpi,4);pos=h;idx=begin;records={}
while pos<len(tpi) and idx<end:
 n,leaf=struct.unpack_from('<2H',tpi,pos);records[idx]=(leaf,tpi[pos:pos+n+2]);pos+=n+2;idx+=1
