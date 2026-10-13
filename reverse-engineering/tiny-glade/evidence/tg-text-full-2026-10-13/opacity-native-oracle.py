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
scale_color=fn(0xc918d0,c.c_uint32,c.c_uint32,c.c_float)
cases=[]
scale_bits=[0,0x80000000,0x3e800000,0x3f000000,0x3f800000,0x40000000,0xbf800000,0x7f800000,0xff800000,0x7fc00000,0xffc12345]
for alpha in [0,1,7,63,127,128,254,255]:
 for bits in scale_bits:
  color=0x1e80f300|alpha
  scale=s.unpack('<f',s.pack('<I',bits))[0]
  cases.append(dict(color=color,scale_bits=bits,result=scale_color(color,scale)))
(OUT/'opacity-native-oracle.json').write_text(json.dumps(dict(binary_sha256=sha,original_instruction_execution=True,scope='96-byte private scale_u32_packed_srgba_color scalar; public float clamp/conversion not reconstructed',cases=cases),indent=2)+'\n')
k.VirtualFree(base,0,0x8000)
print(json.dumps(dict(cases=len(cases),ok=True)))
