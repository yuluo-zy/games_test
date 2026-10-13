"""Bounded B RNE CPU calls; game startup/GPU never executed.
Original input layouts are test adapters, not Rust public ABI claims.
"""
from pathlib import Path
import ctypes as c, struct as s, hashlib, json, math
ROOT=Path(r'D:/game/reverse-engineering/tiny-glade')
OUT=ROOT/'evidence/tg-text-full-2026-10-13'
raw=Path(r'D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=s.unpack_from('<I',raw,60)[0]; opt=pe+24
n=s.unpack_from('<H',raw,pe+6)[0]; sh=opt+s.unpack_from('<H',raw,pe+20)[0]
size=s.unpack_from('<I',raw,opt+56)[0]
k=c.WinDLL('kernel32',use_last_error=True)
k.VirtualAlloc.argtypes=[c.c_void_p,c.c_size_t,c.c_uint32,c.c_uint32];k.VirtualAlloc.restype=c.c_void_p
k.VirtualFree.argtypes=[c.c_void_p,c.c_size_t,c.c_uint32];k.VirtualFree.restype=c.c_int
base=k.VirtualAlloc(None,size,0x3000,0x40)
assert base,c.get_last_error()
for i in range(n):
 o=sh+i*40;vs,rva,rs,off=s.unpack_from('<IIII',raw,o+8)
 c.memmove(base+rva,raw[off:off+rs],rs)
# RIP-relative code/data remain at original distances; absolute/import routes avoided.
def fn(rva,ret,*args):return c.WINFUNCTYPE(ret,*args)(base+rva)
crt=c.CDLL('vcruntime140.dll')
c.c_uint64.from_address(base+0x3193028).value=c.cast(crt.memcpy,c.c_void_p).value
rect=fn(0xb8f510,None,c.c_void_p,c.c_uint64,c.c_uint64,c.c_float,c.c_void_p)
cases=[]
for color in [0.,0.25,0.5,1.,2.]:
 for words in [[],[0,0x11223344],[2],[0,7,2,0x201]]:
  renderer=c.create_string_buffer(0x4e0);primitives=c.create_string_buffer(48*8);shapes=(c.c_uint32*512)();shapes[0]=99
  s.pack_into('<QQQ',renderer,0x410,8,c.addressof(primitives),0)
  s.pack_into('<QQQ',renderer,0x428,512,c.addressof(shapes),1)
  rawwords=(c.c_uint32*max(1,len(words)))(*words)
  rawvec=(c.c_uint64*3)(len(words),c.addressof(rawwords),len(words))
  pos=s.unpack('<Q',s.pack('<ff',3.25,-7.5))[0];extent=s.unpack('<Q',s.pack('<ff',9.5,21.25))[0]
  rect(renderer,pos,extent,color,rawvec)
  cases.append(dict(input_color=color,words=words,primitive_hex=primitives.raw[:48].hex(),primitive_words=list(s.unpack('<12I',primitives.raw[:48])),shape_words=list(shapes[:s.unpack_from('<Q',renderer,0x438)[0]])))
result=dict(binary_sha256=sha,original_instruction_execution=True,game_launched=False,gpu_executed=False,allocation_growth_executed=False,scope='preallocated TextRenderer::rect; packed primitive and ordered word append only',cases=cases)
(OUT/'rect-native-oracle.json').write_text(json.dumps(result,indent=2)+'\n')
k.VirtualFree(base,0,0x8000)
print(json.dumps({'cases':len(cases),'ok':True}))
