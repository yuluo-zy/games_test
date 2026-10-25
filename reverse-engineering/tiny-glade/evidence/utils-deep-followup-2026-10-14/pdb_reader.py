"""Read existing MSF/CodeView evidence; never synthesize missing Rust types."""
from pathlib import Path
import mmap,struct

class Pdb:
    def __init__(self,path):
        self.file=open(path,'rb');self.raw=mmap.mmap(self.file.fileno(),0,access=mmap.ACCESS_READ)
        m=self.raw
        assert m[:24]==b'Microsoft C/C++ MSF 7.00'
        self.bs,_,_,ndir,_,bmap=struct.unpack_from('<6I',m,32)
        bs=self.bs;blocks=struct.unpack_from(f'<{(ndir+bs-1)//bs}I',m,bmap*bs)
        d=b''.join(m[b*bs:(b+1)*bs] for b in blocks)[:ndir]
        ns=struct.unpack_from('<I',d)[0];self.sizes=struct.unpack_from(f'<{ns}I',d,4)
        p=4+4*ns;self.blocks=[]
        for size in self.sizes:
            n=(max(0,size if size!=0xffffffff else 0)+bs-1)//bs
            self.blocks.append(struct.unpack_from(f'<{n}I',d,p));p+=4*n
    def stream(self,i):
        bs=self.bs;m=self.raw
        return b''.join(m[b*bs:(b+1)*bs] for b in self.blocks[i])[:self.sizes[i]]
    def types(self,stream):
        data=self.stream(stream);hs,begin,end=struct.unpack_from('<3I',data,4);p=hs
        for index in range(begin,end):
            size,kind=struct.unpack_from('<HH',data,p)
            yield index,kind,data[p+4:p+size+2]
            p+=size+2
    def modules(self):
        d=self.stream(3);size=struct.unpack_from('<i',d,24)[0];p=64;mi=0
        while p<64+size:
            ss,sb=struct.unpack_from('<HI',d,p+34);q=p+64
            n1=d.index(b'\0',q);n2=d.index(b'\0',n1+1)
            yield mi,d[q:n1].decode('utf8','replace'),ss,sb
            p=(n2+4)&~3;mi+=1
    def symbols(self,ss,sb):
        s=self.stream(ss);q=4
        while q<sb:
            ln,kind=struct.unpack_from('<HH',s,q)
            yield q,kind,s[q+4:q+ln+2]
            q+=ln+2

def z(data):return data.split(b'\0',1)[0].decode('utf8','replace')
