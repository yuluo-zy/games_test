import pathlib,struct,re,bisect,json,sys
b=pathlib.Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();p=struct.unpack_from('<I',b,60)[0];n=struct.unpack_from('<H',b,p+6)[0];s=p+24+struct.unpack_from('<H',b,p+20)[0];sections=[]
for i in range(n):
 off=s+40*i;name=b[off:off+8].rstrip(b'\0').decode();size,rva,rawsize,raw=struct.unpack_from('<IIII',b,off+8);sections.append((name,rva,raw,rawsize))
tx=next(x for x in sections if x[0]=='.text');code=b[tx[2]:tx[2]+tx[3]];pd=next(x for x in sections if x[0]=='.pdata');f=[struct.unpack_from('<III',b,pd[2]+i) for i in range(0,pd[3]-11,12)];starts=[x[0] for x in f]
def owner(a):
 i=bisect.bisect_right(starts,a-0x140000000)-1
 return hex(starts[i]+0x140000000) if i>=0 and a-0x140000000<f[i][1] else None
wanted={int(x,16) for x in sys.argv[1:]}
refs=[]
for m in re.finditer(b'(?=(\xe8....))',code,re.S):
 a=0x140000000+tx[1]+m.start();t=a+5+struct.unpack_from('<i',code,m.start()+1)[0]
 if t in wanted:refs.append(dict(from_va=hex(a),target=hex(t),owner=owner(a),kind='call'))
for m in re.finditer(b'[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]....',code,re.S):
 a=0x140000000+tx[1]+m.start();t=a+7+struct.unpack_from('<i',code,m.start()+3)[0]
 if t in wanted:refs.append(dict(from_va=hex(a),target=hex(t),owner=owner(a),kind='lea'))
print(json.dumps(refs,indent=2))
