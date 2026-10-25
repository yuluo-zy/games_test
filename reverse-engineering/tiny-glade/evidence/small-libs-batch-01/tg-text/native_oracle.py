"""Bounded B RNE CPU calls; game startup/GPU never executed.
Original input layouts are test adapters, not Rust public ABI claims.
"""
from pathlib import Path
import ctypes as c, struct as s, hashlib, json, math
ROOT=Path(r'D:/game/reverse-engineering/tiny-glade')
OUT=ROOT/'evidence/small-libs-batch-01/tg-text'
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
append=fn(0xb92340,None,c.c_void_p,c.c_void_p,c.c_void_p,c.c_void_p)
scaled=fn(0xb92090,c.c_void_p,c.c_void_p,c.c_void_p,c.c_void_p)
def_scale=fn(0xb06780,c.c_float)
new_shape=fn(0xa82b20,c.c_void_p,c.c_void_p)
new_raw=fn(0xb92320,c.c_void_p,c.c_void_p)
clear=fn(0xb92590,None,c.c_void_p)
is_empty=fn(0xb92210,c.c_bool,c.c_void_p)
cases=[]
resolver_calls=[]
@c.WINFUNCTYPE(c.c_uint32,c.c_void_p,c.c_void_p,c.c_uint64)
def resolve(ctx,ptr,n):
 name=c.string_at(ptr,n).decode();resolver_calls.append(name);return 0x12345
vt=(c.c_uint64*4)(0,0,0,c.cast(resolve,c.c_void_p).value)
keep=[]
def record(ops,prefix=()):
 b=c.create_string_buffer(max(32,len(ops)*32));keep.clear()
 for i,op in enumerate(ops):
  o=i*32
  if op['kind']=='fill':s.pack_into('<QI',b,o,0x8000000000000002,op['value'])
  elif op['kind']=='radius':s.pack_into('<QI',b,o,0x8000000000000004,op['bits'])
  elif op['kind']=='image':
   s.pack_into('<QI',b,o,0x8000000000000001,op['index']);s.pack_into('<B',b,o+24,op['scaling'])
  else:
   text=op['name'].encode();st=c.create_string_buffer(text);keep.append(st)
   s.pack_into('<QQQ',b,o,len(text),c.addressof(st),len(text));s.pack_into('<B',b,o+24,op['scaling'])
 stream=(c.c_uint64*3)(len(ops),c.addressof(b),len(ops))
 arr=(c.c_uint32*1024)();arr[:len(prefix)]=prefix
 rawvec=(c.c_uint64*3)(1024,c.addressof(arr),len(prefix))
 resolver_calls.clear();append(rawvec,stream,None,vt)
 words=list(arr[:rawvec[2]])
 cases.append(dict(ops=ops,prefix=list(prefix),words=words,resolver_calls=list(resolver_calls)))
for x in [0,1,0x10203040,0xffffffff,0x800000ff]:record([dict(kind='fill',value=x)])
for bits in [0,0x80000000,0x3f000000,0x3ff33333,0x437effff,0x437f0000,0x43800000,0xbf800000,0x7f800000,0xff800000,0x7fc00000,0xffc00001]:record([dict(kind='radius',bits=bits)])
for index in [0,1,255,256,65535,65536,0xffffffff]:
 for scaling in range(4):record([dict(kind='image',index=index,scaling=scaling)])
for scaling in range(4):record([dict(kind='name',name='menu/paper-bg',scaling=scaling)])
record([],[7,8])
record([dict(kind='fill',value=0xdeadbeef),dict(kind='radius',bits=0x41280000),dict(kind='name',name='one',scaling=3),dict(kind='image',index=65537,scaling=2)],[99])
metrics=[]
for scale in [0.,.5,1.,2.,-1.,float('inf'),float('nan')]:
 for a,z in [(12.,20.),(0.,-0.),(-2.,3.)]:
  family=c.create_string_buffer(b'Baloo 2');entry=c.create_string_buffer(32)
  s.pack_into('<QQQf',entry,0,7,c.addressof(family),7,scale)
  m=(c.c_float*2)(a,z);out=c.create_string_buffer(24);scaled(out,entry,m)
  metrics.append({'scale_bits':s.unpack('<I',s.pack('<f',scale))[0],'input_bits':list(s.unpack('<II',bytes(m))),'output_bits':list(s.unpack('<II',out.raw[16:24])),'family_preserved':s.unpack_from('<QQ',out,0)==(c.addressof(family),7)})
v=(c.c_uint64*3)();new_shape(v);shape_new=list(v);empty_new=bool(is_empty(v));new_raw(v);raw_new=list(v);v[0]=17;v[1]=0x12345678;v[2]=8;clear(v);raw_clear=list(v)
result=dict(binary_sha256=sha,target_execution=True,kind='original native instruction execution in owned allocation',game_launched=False,gpu_executed=False,capacity_growth_executed=False,serde_entry_executed=False,cases=cases,metrics_cases=metrics,default_scale=def_scale(),shape_new=shape_new,raw_new=raw_new,empty_new=empty_new,raw_clear=raw_clear,bridges=['owned preallocated input/output memory','explicit image resolver callback'],scope='append/clear/new/is_empty/default_scale/scaled; allocation growth and serde failures excluded')
(OUT/'native-oracle.json').write_text(json.dumps(result,indent=2)+'\n')
k.VirtualFree(base,0,0x8000)
print(json.dumps({'cases':len(cases),'metrics':len(metrics),'default_scale':result['default_scale'],'ok':True}))
