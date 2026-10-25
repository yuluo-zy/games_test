"""Execute only two leaf B RNE builders in isolated fixture buffers; no game startup/IO."""
import ctypes, hashlib, json
from pathlib import Path
ROOT=Path(r'D:/game/reverse-engineering/tiny-glade')
image=Path(r'D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(image).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=int.from_bytes(image[0x3c:0x40],'little'); sh=pe+24+int.from_bytes(image[pe+20:pe+22],'little')
sections=[]
for i in range(int.from_bytes(image[pe+6:pe+8],'little')):
 o=sh+i*40
 sections.append((int.from_bytes(image[o+12:o+16],'little'),int.from_bytes(image[o+8:o+12],'little'),int.from_bytes(image[o+20:o+24],'little')))
def raw(rva,n):
 for a,size,o in sections:
  if a<=rva<a+size:return image[o+rva-a:o+rva-a+n]
 raise ValueError(rva)
k=ctypes.WinDLL('kernel32',use_last_error=True)
k.VirtualAlloc.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_uint32,ctypes.c_uint32];k.VirtualAlloc.restype=ctypes.c_void_p
k.VirtualFree.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_uint32]
mem=k.VirtualAlloc(None,4096,0x3000,0x40)
assert mem
try:
 newcode=raw(0x631ce0,55);levelcode=raw(0x631d20,66)
 ctypes.memmove(mem,newcode,len(newcode));ctypes.memmove(mem+256,levelcode,len(levelcode))
 new=ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_uint32,ctypes.c_uint32)(mem)
 level=ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_uint8)(mem+256)
 cases=[]
 for width in [0,1,640,1920,0xffffffff]:
  for height in [0,1,480,1080,0xffffffff]:
   for l in range(3):
    src=ctypes.create_string_buffer(bytes([0xa5])*64,64);dst=ctypes.create_string_buffer(64)
    assert new(src,width,height)==ctypes.addressof(src)
    initial=bytes(src)
    assert int.from_bytes(initial[48:52],'little')==width
    assert int.from_bytes(initial[52:56],'little')==height
    assert initial[56:58]==b'\0\0'
    assert level(dst,src,l)==ctypes.addressof(dst)
    state=bytes(dst)
    assert state[56]==l and state[57]==0
    assert state[:56]==initial[:56]
    cases.append({'width':width,'height':height,'level':l,'title_len':int.from_bytes(state[16:24],'little'),'description_len':int.from_bytes(state[40:48],'little'),'buttons':state[57]})
 out={'scope':'Original B RNE MessageDialog new/set_level leaf entry; no IO or game startup','original_rne_sha256':hashlib.sha256(image).hexdigest(),'cases':cases,'mismatches':0,'native_leaf_entries':len(cases)*2,'code_hashes':{'new':hashlib.sha256(newcode).hexdigest(),'set_level':hashlib.sha256(levelcode).hexdigest()}}
 (ROOT/'evidence/foundation-unstarted-2026-10-13/ui/original-dialog-cases.json').write_text(json.dumps(out,indent=2),encoding='utf8')
 print(json.dumps({'native_leaf_entries':len(cases)*2,'cases':len(cases),'mismatches':0}))
finally:k.VirtualFree(mem,0,0x8000)
