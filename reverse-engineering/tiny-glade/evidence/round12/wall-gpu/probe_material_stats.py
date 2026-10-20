"""Execute only original B pure MaterialStats methods, never the EXE entry.
PE section layout and RIP-relative constants are copied unchanged. No game
helper is replaced, patched or executed by the argmax entry.
"""
import ctypes, hashlib, json, math, pathlib, random, struct
root=pathlib.Path('D:/game')
path=root/'ljxsj_92385/Tiny Glade/tiny-glade.rne'
b=path.read_bytes()
assert hashlib.sha256(b).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',b,60)[0]
n=struct.unpack_from('<H',b,pe+6)[0]
opt=pe+24
size=struct.unpack_from('<I',b,opt+56)[0]
headers=struct.unpack_from('<I',b,opt+60)[0]
section=opt+struct.unpack_from('<H',b,pe+20)[0]
k=ctypes.WinDLL('kernel32',use_last_error=True)
k.VirtualAlloc.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_uint32,ctypes.c_uint32]
k.VirtualAlloc.restype=ctypes.c_void_p
k.VirtualFree.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_uint32]
base=k.VirtualAlloc(None,size,0x3000,0x40)
assert base
ctypes.memmove(base,b,headers)
for i in range(n):
    vsize,rva,rawsize,raw=struct.unpack_from('<IIII',b,section+i*40+8)
    if rawsize: ctypes.memmove(base+rva,b[raw:raw+rawsize],rawsize)
argmax=ctypes.WINFUNCTYPE(ctypes.c_uint8,ctypes.c_void_p)(base+0xa887d0)
init=ctypes.WINFUNCTYPE(None,ctypes.c_void_p,ctypes.c_uint32,ctypes.c_uint8)(base+0xa88280)
rng=random.Random(0x19a887d0)
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
cases=[]
for i in range(520):
    prev=[bits(rng.uniform(-20,20)) for _ in range(7)]
    cur=[bits(rng.uniform(-20,20)) for _ in range(7)]
    if i<40:
        prev=[0]*7;cur=[0]*7
        if i<7: cur[i]=bits(3.)
        elif i<14: cur[i-7]=0x7fc00000
        elif i<21:cur[i-14]=0xffc00000
        elif i<28:cur[i-21]=0x7f800000
        elif i<35:cur[i-28]=0xff800000
        else:cur=[bits(3.)]*7
    state=ctypes.create_string_buffer(0xd0)
    struct.pack_into('<7I',state,0x28,*prev);struct.pack_into('<7I',state,0x90,*cur)
    cases.append(dict(previous=prev,current=cur,material=argmax(ctypes.addressof(state))))
init_cases=[]
for special in (0,1):
    for x in (0.,-0.,5.,float('inf'),float('nan')):
        state=ctypes.create_string_buffer(0xd0)
        ctypes.memset(ctypes.addressof(state)+0x88,0xa5,0x48)
        struct.pack_into('<7I',state,0x28,*[bits(float(i)) for i in range(7)])
        init(ctypes.addressof(state),bits(x),special)
        init_cases.append(dict(length=bits(x),special=special,current=list(struct.unpack_from('<7I',state,0x90)),previous=list(struct.unpack_from('<7I',state,0x28)),counts=list(struct.unpack_from('<8I',state,0xac))))
out=root/'reverse-engineering/tiny-glade/evidence/round12/wall-gpu/material-stats-native-cases.json'
out.write_text(json.dumps(dict(source_sha256=hashlib.sha256(b).hexdigest(),entries=['140a887d0','140a88280'],cases=cases,init_cases=init_cases),indent=2))
assert k.VirtualFree(base,0,0x8000)
print(json.dumps(dict(original_calls=len(cases)+len(init_cases),output=str(out))))
