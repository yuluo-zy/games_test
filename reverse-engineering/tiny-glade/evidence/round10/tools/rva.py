import sys,struct
BIN=r"D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe"
data=open(BIN,'rb').read(); BASE=0x140000000
e=struct.unpack_from('<I',data,0x3c)[0]; n=struct.unpack_from('<H',data,e+6)[0]; os_=struct.unpack_from('<H',data,e+20)[0]
sec=[]
for i in range(n):
    o=e+24+os_+i*40
    vs,va,rs,rp=struct.unpack_from('<IIII',data,o+8); sec.append((va,vs,rp,rs))
def off(va):
    r=va-BASE
    for va0,vs,rp,rs in sec:
        if va0<=r<va0+vs: return rp+(r-va0)
    raise ValueError(hex(va))
for spec in sys.argv[1:]:
    if ':' in spec: a,l=spec.split(':'); a=int(a,16); l=int(l,16)
    else: a=int(spec,16); l=0x40
    b=data[off(a):off(a)+l]
    print(hex(a),l,repr(b.decode('utf-8','replace')))
