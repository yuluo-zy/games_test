"""Execute untouched original GCC serializer and field visitor with all valid enum branches.
Only std allocation and CRT memory boundaries are host callbacks. No game/serde stubs.
"""
import ctypes as C,struct,json,hashlib,itertools
from pathlib import Path
OUT=Path(__file__).resolve().parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,0x3c)[0];opt=pe+24
base=struct.unpack_from('<Q',raw,opt+24)[0];size=struct.unpack_from('<I',raw,opt+56)[0];headers=struct.unpack_from('<I',raw,opt+60)[0]
sec=opt+struct.unpack_from('<H',raw,pe+20)[0];count=struct.unpack_from('<H',raw,pe+6)[0]
k=C.WinDLL('kernel32',use_last_error=True);k.VirtualAlloc.argtypes=[C.c_void_p,C.c_size_t,C.c_uint32,C.c_uint32];k.VirtualAlloc.restype=C.c_void_p;k.VirtualFree.argtypes=[C.c_void_p,C.c_size_t,C.c_uint32]
actual=k.VirtualAlloc(base,size,0x3000,0x40);assert actual==base,'Refuse relocated execution'
allocations=[]
def read(a,n):return C.string_at(a,n)
def write(a,b):assert base<=a and a+len(b)<=base+size;C.memmove(a,b,len(b))
def patch(a,fn):write(a,b'\x48\xb8'+struct.pack('<Q',C.cast(fn,C.c_void_p).value)+b'\xff\xe0')
@C.CFUNCTYPE(C.c_void_p,C.c_uint64,C.c_uint64)
def alloc(n,align):
 assert n<0x1000000 and align<0x10000
 b=C.create_string_buffer(max(n,1)+max(align,1));allocations.append(b);return (C.addressof(b)+align-1)&-align
@C.CFUNCTYPE(None,C.c_void_p,C.c_uint64,C.c_uint64)
def free(p,n,a):pass
@C.CFUNCTYPE(C.c_void_p,C.c_void_p,C.c_uint64,C.c_uint64,C.c_uint64)
def realloc(p,n,a,m):q=alloc(m,a);C.memmove(q,p,min(n,m));return q
@C.CFUNCTYPE(C.c_void_p,C.c_void_p,C.c_void_p,C.c_uint64)
def move(d,s,n):C.memmove(d,s,n);return d
@C.CFUNCTYPE(C.c_int,C.c_void_p,C.c_void_p,C.c_uint64)
def compare(a,b,n):x=read(a,n);y=read(b,n);return (x>y)-(x<y)
try:
 C.memmove(base,raw[:headers],headers)
 for i in range(count):
  _,va,n,p=struct.unpack_from('<IIII',raw,sec+i*40+8)
  if n:C.memmove(base+va,raw[p:p+n],n)
 original_hash=hashlib.sha256(read(0x14085ee90,880)).hexdigest()
 for address,fn in [(0x140613c10,alloc),(0x140613c20,free),(0x140613c30,realloc),(0x142923c00,move),(0x142923c10,move),(0x142923bf0,compare)]:patch(address,fn)
 serialize=C.CFUNCTYPE(None,C.c_void_p,C.c_void_p)(0x14085ee90)
 visit=C.CFUNCTYPE(None,C.c_void_p,C.c_void_p,C.c_uint64)(0x14085f550)
 default=C.CFUNCTYPE(C.c_void_p,C.c_void_p)(0x1408c5600)
 original_default=C.create_string_buffer(bytes([0xcc])*0x48);default(original_default)
 default_output=C.create_string_buffer(24);serialize(default_output,original_default)
 _,default_ptr,default_len=struct.unpack('<QQQ',default_output.raw)
 default_json=read(default_ptr,default_len).decode()
 value_serialize=C.CFUNCTYPE(None,C.c_void_p,C.c_void_p)(0x14085f200)
 value_deserialize=C.CFUNCTYPE(None,C.c_void_p,C.c_void_p)(0x1415a0130)
 native_roundtrip=[]
 rows=[]
 for style,arch,rim,railing,window in itertools.product(range(3),range(2),range(2),range(4),[None,0,8,0xffffffff]):
  data=bytearray(0x48);struct.pack_into('<II',data,0,int(window is not None),window or 0)
  struct.pack_into('<III',data,8,11,22,33);data[0x14]=railing
  for offset,value in [(0x18,101),(0x1c,102),(0x20,103),(0x24,104),(0x28,105),(0x2c,106),(0x30,107),(0x34,108),(0x38,109)]:struct.pack_into('<I',data,offset,value)
  struct.pack_into('<f',data,0x3c,1.25);data[0x40:0x43]=bytes([arch,rim,style])
  source=C.create_string_buffer(bytes(data));output=C.create_string_buffer(24);serialize(output,source)
  cap,p,n=struct.unpack('<QQQ',output.raw);assert n<4096 and cap>=n
  text=read(p,n).decode();value=json.loads(text)
  assert value['windows']==window and value['flag']==108 and value['fences']==106
  owned_value=C.create_string_buffer(32);value_serialize(owned_value,source)
  assert owned_value.raw[0]==5,owned_value.raw.hex()
  decoded=C.create_string_buffer(80);value_deserialize(decoded,owned_value)
  assert struct.unpack_from('<I',decoded.raw)[0]==0,decoded.raw.hex()
  decoded_gcc=C.create_string_buffer(decoded.raw[4:76]);roundtrip_out=C.create_string_buffer(24);serialize(roundtrip_out,decoded_gcc)
  _,rp,rn=struct.unpack('<QQQ',roundtrip_out.raw);roundtrip_json=read(rp,rn).decode()
  assert roundtrip_json==text
  native_roundtrip.append({'style':style,'arch':arch,'rim':rim,'railing':railing,'windows':window,'original_value_tag':owned_value.raw[0],'original_result_tag':0,'decoded_native_bytes_hex':decoded.raw[4:76].hex()})
  rows.append({'native_bytes_hex':data.hex(),'native_style':style,'native_arch':arch,'native_rim':rim,'native_railing':railing,'native_windows':window,'json':text})
 fields=['freeform_wall','building','roof','building_wall_style','freehand_ty','halftimber','lantern_light','fences','window_light','flag','windows','gates','flat_rim_style','lantern_brightness','stairs','fence','unknown']
 visits=[]
 for i,field in enumerate(fields):
  name=field.encode();source=C.create_string_buffer(name);output=C.create_string_buffer(16);visit(output,source,len(name))
  tag,index=output.raw[:2];assert tag==0 and index==(i if i<15 else 15)
  visits.append({'field':field,'native_result_tag':tag,'native_field_index':index})
 missing=[]
 for field in fields[:15]+['stairs_color','ladder_color','railing_ty','magic_color']:
  allocation_start=len(allocations);owned_value=C.create_string_buffer(32);value_serialize(owned_value,original_default)
  key=field.encode();found=[b for b in allocations[allocation_start:] if len(b.raw)==len(key)+1 and b.raw[:len(key)]==key]
  assert len(found)==1,(field,len(found))
  group=fields[:15] if field in fields[:15] else ['stairs_color','ladder_color','railing_ty','magic_color']
  order=sorted(group);index=order.index(field)
  # Mutate only an owned fixture key, keeping length and lexical order. The
  # complete original map/visitor/ownership algorithms still execute untouched.
  replacement=None
  for shift in [1,-1,2,-2]:
   name=field[:-1]+chr(ord(field[-1])+shift);changed=order.copy();changed[index]=name
   if sorted(changed)==changed and name not in group:replacement=name;break
  assert replacement
  C.memmove(C.addressof(found[0]),replacement.encode(),len(key))
  result_out=C.create_string_buffer(80);value_deserialize(result_out,owned_value)
  result_tag=struct.unpack_from('<I',result_out.raw)[0]
  item={'missing_field':field,'fixture_unknown_key':replacement,'original_result_tag':result_tag}
  if result_tag==0:
   decoded_gcc=C.create_string_buffer(result_out.raw[4:76]);output=C.create_string_buffer(24);serialize(output,decoded_gcc)
   _,ptr,n=struct.unpack('<QQQ',output.raw);item['native_result_json']=read(ptr,n).decode()
  missing.append(item)
 empty_map=C.create_string_buffer(32);C.memmove(empty_map,b'\x05',1);empty_result=C.create_string_buffer(80);value_deserialize(empty_result,empty_map)
 empty_map_tag=struct.unpack_from('<I',empty_result.raw)[0]
 result={'original_sha256':sha,'serializer':{'va':'0x14085ee90','bytes':880,'sha256':original_hash},'default':{'va':'0x1408c5600','bytes':82,'sha256':hashlib.sha256(read(0x1408c5600,82)).hexdigest(),'native_bytes_hex':original_default.raw.hex(),'json':default_json},'visitor':{'va':'0x14085f550','fields':visits},'native_value_serializer':'0x14085f200','native_value_deserializer':'0x1415a0130','original_empty_map_result_tag':empty_map_tag,'native_missing_fields':missing,'native_decode_cases':native_roundtrip,'method':'Execute complete original GCC serializers, Value deserializer and field visitor. Only existing std allocator and CRT memory boundaries replaced; no serde/game logic stubs.','cases':rows}
 (OUT/'original-gcc-full-fixtures.json').write_text(json.dumps(result,indent=2)+'\n')
 first=json.loads(rows[0]['json']);last=json.loads(rows[-1]['json'])
 print(json.dumps({'native_cases':len(rows),'first':first,'last':last,'enum_strings':{key:sorted({json.loads(row['json'])[key] for row in rows}) for key in ['building_wall_style','freehand_ty','flat_rim_style']},'railing_strings':sorted({json.loads(row['json'])['stairs']['railing_ty'] for row in rows})}))
finally:k.VirtualFree(base,0,0x8000)
