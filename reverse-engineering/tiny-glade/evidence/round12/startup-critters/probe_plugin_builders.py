"""Original ordered config builders, with Bevy's trait-object admission as a
recording boundary. No gameplay system is executed or replaced in production.
"""
from pathlib import Path
import sys,ctypes,struct,json,re
O=Path(__file__).resolve().parent;sys.path.insert(0,str(O.parent/'wall-gpu'))
from native_cpu_image import NativeCpuImage
im=NativeCpuImage();im.bridge_allocator();keep=[];records=[]
def mem(n):
 b=ctypes.create_string_buffer(n);keep.append(b);return ctypes.addressof(b)
def word(p):return struct.unpack('<Q',ctypes.string_at(p,8))[0]
def admit(out,state,vtable):
 size=word(vtable+8);assert size<0x10000,size
 names=[]
 for off in range(0,size-15,8):
  ptr,n=struct.unpack('<QQ',ctypes.string_at(state+off,16))
  if im.base<=ptr<im.base+0x3800000 and 0<n<256:
   raw=ctypes.string_at(ptr,n)
   if raw.startswith(b'system_critters::'):names.append({'offset':hex(off),'name':raw.decode()})
 records.append({'vtable':hex(vtable-im.base+0x140000000),'name':names,'state_bytes':size})
 ctypes.memset(out,0,0x70);return out
im.dependency_callback(0x140ce4b10,ctypes.WINFUNCTYPE(ctypes.c_void_p,*([ctypes.c_void_p]*3)),admit,'Bevy IntoSystemConfigs trait-object admission, recording-only leaf')
for p in [0x142923c00,0x142923c10]:im.resolve_math_import(p)
one=mem(0x70);two=mem(0x70)
ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p)(im.address(0x140f7d430))(one)
first=len(records)
ctypes.WINFUNCTYPE(ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p)(im.address(0x140f7e4e0))(two,one)
data={'original_sha256':im.sha,'scope':__doc__,'first_builder_count':first,'records':records,'first_header':ctypes.string_at(one,0x70).hex(),'outer_header':ctypes.string_at(two,0x70).hex(),'bridges':im.bridges}
(O/'original-plugin-configs.json').write_text(json.dumps(data,indent=2),encoding='utf8')
print(json.dumps({'first':first,'all':len(records),'names':[r['name'] for r in records]},indent=2));im.close()
