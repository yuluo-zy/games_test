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
mem=k.VirtualAlloc(None,16384,0x3000,0x40)
assert mem
try:
 cases=[]
 for name,rva,size,jmp_rva,lea_offset,variants,control_size in [('gpu',0x21bb740,564,0x2ffa910,0x20,6,0x100),('config',0xdf3e70,431,0x2b79150,0x13,5,0x100)]:
  code=bytearray(raw(rva,size))
  code[lea_offset+3:lea_offset+7]=(4096-(lea_offset+7)).to_bytes(4,'little',signed=True)
  ctypes.memmove(mem,bytes(code),len(code))
  tab=raw(jmp_rva,variants*4)
  relocated=b''.join((jmp_rva+int.from_bytes(tab[i*4:i*4+4],'little',signed=True)-rva-4096).to_bytes(4,'little',signed=True) for i in range(variants))
  ctypes.memmove(mem+4096,relocated,len(relocated))
  fun=ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p)(mem)
  if name=='gpu':
   for variant in [0,1,3,5]:
    for val in [0,1,7,0xffffffff,0xffffffffffffffff]:
     output=ctypes.create_string_buffer(256);state=ctypes.create_string_buffer(256);message=ctypes.create_string_buffer(64)
     ctypes.memmove(message,variant.to_bytes(8,'little')+val.to_bytes(8,'little'),16)
     fun(output,state,message)
     data=bytes(state)
     cases.append({'function':name,'variant':variant,'value':val,'outcome_tag':int.from_bytes(data[:8],'little'),'selected':int.from_bytes(data[8:16],'little'),'more_info':data[0x90]})
  else:
   for index in range(6):
    for seed in [0,1]:
     for repeat in [1,2,3]:
      output=ctypes.create_string_buffer(256);state=ctypes.create_string_buffer(bytes([0])*96+bytes([seed])*6+bytes([0])*154,256);message=ctypes.create_string_buffer((4).to_bytes(8,'little')+index.to_bytes(8,'little'),64)
      for _ in range(repeat):fun(output,state,message)
      cases.append({'function':name,'index':index,'seed':seed,'repeat':repeat,'flags':list(bytes(state)[96:102])})
 out={'scope':'Original B RNE gpu update leaf branches 0/1/3/5 and config six workaround XOR branches; original jump tables relocated, no external calls','cases':cases,'native_entries':20+sum(c['repeat'] for c in cases if c['function']=='config'),'original_rne_sha256':hashlib.sha256(image).hexdigest(),'not_whole_update':True}
 (ROOT/'evidence/tg-gpu-selector-completion-2026-10-14/native-update-cases.json').write_text(json.dumps(out,indent=2),encoding='utf8')
 print(json.dumps({'cases':len(cases),'native_entries':out['native_entries']}))
finally:k.VirtualFree(mem,0,0x8000)

