import ctypes,hashlib,json,struct
from pathlib import Path
root=Path(__file__).parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,0x3c)[0]
st=pe+24+struct.unpack_from('<H',raw,pe+20)[0]
def bytes_at(rva,size):
 for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
  off=st+40*i
  vs,va,rs,ro=struct.unpack_from('<IIII',raw,off+8)
  if va<=rva<va+max(vs,rs):return raw[ro+rva-va:ro+rva-va+size]
 raise ValueError(rva)
k=ctypes.WinDLL('kernel32',use_last_error=True)
k.VirtualAlloc.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_uint32,ctypes.c_uint32];k.VirtualAlloc.restype=ctypes.c_void_p
k.VirtualFree.argtypes=[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_uint32]
mem=k.VirtualAlloc(None,4096,0x3000,0x40)
assert mem
# Copy original writer success/pending-error paths, plus the original u64 AsTag.
code=bytearray(bytes_at(0x1552820,0x132));assert code[0x42]==0xe8
struct.pack_into('<i',code,0x43,0x800-(0x42+5))
ctypes.memmove(mem,bytes(code),len(code));ctypes.memmove(mem+0x800,bytes_at(0x885d0,4),4)
ctypes.memmove(mem+0x400,bytes_at(0x87abf0,0xd1),0xd1)
Word=ctypes.c_uint64
Add=ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.POINTER(Word),ctypes.POINTER(Word),Word)(mem)
Drop=ctypes.WINFUNCTYPE(None,ctypes.POINTER(Word))(mem+0x400)
report=[]
for start in [0,3,31,1024]:
 for n in [0,1,7,64,2048]:
  backing=ctypes.create_string_buffer(b'\xa5'*4096)
  vec=(Word*3)(4096,ctypes.addressof(backing),4096)
  writer=(Word*3)(ctypes.addressof(vec),start,0)
  result=(Word*3)(0,0,0)
  tag=0x6867666564636261
  Add(result,writer,tag)
  assert result[0]==ctypes.addressof(writer) and result[1]==ctypes.addressof(writer)+16 and result[2]==start
  payload=bytes((i*31)%256 for i in range(n))
  ctypes.memmove(ctypes.addressof(backing)+writer[1],payload,n);writer[1]+=n
  Drop(result)
  actual=ctypes.string_at(ctypes.addressof(backing)+start,16+n)
  assert actual==struct.pack('<QQ',tag,n)+payload
  assert writer[1]==start+16+n and writer[2]==0
  report.append({'start':start,'payload_len':n,'tag':tag,'position':writer[1],'header_hex':actual[:16].hex(),'payload_sha256':hashlib.sha256(payload).hexdigest()})
writer=(Word*3)(0,123,0x2500000003);result=(Word*3)(0,0,0)
Add(result,writer,7)
assert result[0]==0 and result[1]==0x2500000003 and writer[2]==0 and writer[1]==123
(root/'writer-oracle.json').write_text(json.dumps({'source_sha256':sha,'writer_rva':'0x1552820','drop_rva':'0x87abf0','cases':report,'pending_error_case':True,'original_instruction_execution':True,'relocation':'AsTag call displacement retargeted to copied original 4-byte identity leaf','external_calls_executed':False,'coverage':'preallocated Cursor<Vec> success and pending-error path only; no allocation, panic, generic failure or full game execution'},indent=2)+'\n')
k.VirtualFree(mem,0,0x8000)
print(json.dumps({'cases':len(report),'pending_error_case':True}))
