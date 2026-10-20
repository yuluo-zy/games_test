"""Original inline control arithmetic140f7df84..fef only. Admission ends
before the wrapper's Bevy change-tick bookkeeping; all game instructions,
comparisons, branches, MAXSS/MINSS and stores execute unchanged.
"""
from pathlib import Path
import ctypes,struct,sys,json
O=Path(__file__).resolve().parent;sys.path.insert(0,str(O.parent/'wall-gpu'))
from native_cpu_image import NativeCpuImage
im=NativeCpuImage();buffers=[]
def mem(n):
 b=ctypes.create_string_buffer(n);buffers.append(b);return ctypes.addressof(b)
tod,control,time,meta=[mem(128) for _ in range(4)]
# The span has no prologue; a small ABI adapter sets the original reference
# registers. Its only end marker is the first non-business instruction.
ctypes.memmove(im.address(0x140f7dff0),b'\xc3',1)
code=(b'\x4c\x8b\xd1\x4c\x8b\xca\x49\x8b\xd0'+b'\x49\xb8'+struct.pack('<Q',meta)
      +b'\x48\x83\xec\x28\x48\xb8'+struct.pack('<Q',im.address(0x140f7df84))
      +b'\xff\xd0\x48\x83\xc4\x28\xc3')
tramp=im.k.VirtualAlloc(None,4096,0x3000,0x40);assert tramp;ctypes.memmove(tramp,code,len(code))
fn=ctypes.WINFUNCTYPE(None,ctypes.c_void_p,ctypes.c_void_p,ctypes.c_void_p)(tramp)
factors=[0x00000000,0x3d4ccccc,0x3d4ccccd,0x3f4ccccc,0x3f4ccccd,0x3f800000,0x7fc12345,0x7f800000,0xff800000]
deltas=[0x00000000,0x3c888889,0x3f800000,0x41200000,0xbf800000,0x7fc54321]
initial=[0,0x80000000,0x3f000000,0x3f800000,0x40000000,0x7fc12345]
cases=[]
for factor in factors:
 for dt in deltas:
  for old in initial:
   birds=len(cases)%2
   ctypes.memmove(tod+0x24,struct.pack('<I',factor),4)
   ctypes.memmove(time+0x28,struct.pack('<I',dt),4)
   ctypes.memmove(control,struct.pack('<IBBBB',old,1,birds,0x35,0xa7),8)
   fn(tod,control,time)
   out=ctypes.string_at(control,8)
   assert out[5:]==bytes([birds,0x35,0xa7])
   cases.append([factor,dt,old,struct.unpack('<I',out[:4])[0],out[4],birds])
(O/'original-control-fixtures.json').write_text(json.dumps({'original_sha256':im.sha,'scope':__doc__,'cases':cases},indent=2),encoding='utf8')
p=O/'control_original_b.rs';p.write_text('const CONTROL_CASES:[[u32;6];'+str(len(cases))+']=[\n'+''.join('['+','.join(hex(v) for v in c)+'],\n' for c in cases)+'];\n',encoding='utf8')
print('original control cases',len(cases));im.k.VirtualFree(tramp,0,0x8000);im.close()
