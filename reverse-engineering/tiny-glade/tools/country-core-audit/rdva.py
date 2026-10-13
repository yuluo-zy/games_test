import struct,sys
sys.path.insert(0,'.')
from cca import EXE,BASE
e=open(EXE,'rb').read()
pe=struct.unpack_from('<I',e,0x3c)[0]; n=struct.unpack_from('<H',e,pe+6)[0]; o=struct.unpack_from('<H',e,pe+20)[0]
secs=[struct.unpack_from('<IIII',e,pe+24+o+40*i+8) for i in range(n)]
def rd(va,k):
    r=va-BASE
    for vs,v,rs,rp in secs:
        if v<=r<v+max(vs,rs): return e[rp+r-v:rp+r-v+k]
def q(va): return struct.unpack('<Q',rd(va,8))[0]
def fmtpieces(va):
    """Arguments-pieces: [ptr,len]* pointed to by va (the array of &str)"""
    out=[]
    i=0
    while True:
        p,l=q(va+16*i),q(va+16*i+8)
        if not(0x140000000<p<0x150000000 and 0<l<200): break
        out.append(rd(p,l)); i+=1
    return out
if __name__=='__main__':
    for a in sys.argv[1:]:
        va=int(a,16); print(hex(va),fmtpieces(va), rd(va,16))
