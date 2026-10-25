"""Execute unmodified original verify code in an isolated loader container, never the shipped process."""
import ctypes as C, hashlib, json, os, pathlib, struct, sys, time
source=pathlib.Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
work=pathlib.Path(__file__).resolve().parent
rva=0x67c3d0
if sys.argv[1]=='prepare':
 d=bytearray(source.read_bytes()); original=hashlib.sha256(d).hexdigest(); assert original=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
 e=struct.unpack_from('<I',d,0x3c)[0]; opt=e+24; base=struct.unpack_from('<Q',d,opt+24)[0]
 sections=[struct.unpack_from('<8sIIII',d,opt+struct.unpack_from('<H',d,e+20)[0]+40*i) for i in range(struct.unpack_from('<H',d,e+6)[0])]
 def off(r):
  for n,vsz,va,rs,ro in sections:
   if va<=r<va+max(vsz,rs): return ro+r-va
  raise ValueError(hex(r))
 def text(r):
  a=off(r);return bytes(d[a:d.index(0,a)]).decode()
 io=off(struct.unpack_from('<I',d,opt+120)[0]); kept=[]; removed=[]
 for a in range(io,io+4096,20):
  vals=struct.unpack_from('<IIIII',d,a)
  if not vals[3]:break
  name=text(vals[3])
  if name.lower() in ('steam_api64.dll','gfsdk_aftermath_lib.x64.dll','fmodstudio.dll','fmod.dll'):removed.append(name)
  else:kept.append(bytes(d[a:a+20]))
 descriptors=b''.join(kept)+bytes(20);d[io:a+20]=descriptors+bytes(a+20-io-len(descriptors))
 patches=[]
 def patch(offset,fmt,value,reason):
  before=bytes(d[offset:offset+struct.calcsize(fmt)]).hex();struct.pack_into(fmt,d,offset,value);patches.append({'offset':offset,'before':before,'after':bytes(d[offset:offset+struct.calcsize(fmt)]).hex(),'reason':reason})
 patch(e+22,'<H',struct.unpack_from('<H',d,e+22)[0]|0x2000,'DLL loader container')
 patch(opt+16,'<I',0,'No EXE entrypoint or DllMain execution')
 tls=off(struct.unpack_from('<I',d,opt+112+9*8)[0]);patch(tls+24,'<Q',0,'Suppress application TLS callbacks; loader still supplies TLS template/index')
 dll=work/'original-verify-container.dll';dll.write_bytes(d)
 ranges=[]
 for begin,end,_ in struct.iter_unpack('<III',bytes(d[off(next(x[2] for x in sections if x[0].rstrip(b'\0')==b'.pdata')):off(next(x[2] for x in sections if x[0].rstrip(b'\0')==b'.pdata'))+next(x[3] for x in sections if x[0].rstrip(b'\0')==b'.pdata')-next(x[3] for x in sections if x[0].rstrip(b'\0')==b'.pdata')%12])):
  if begin<=rva<end:ranges.append({'rva':hex(begin),'size':end-begin,'sha256':hashlib.sha256(d[off(begin):off(begin)+end-begin]).hexdigest(),'unchanged_from_original':d[off(begin):off(begin)+end-begin]==source.read_bytes()[off(begin):off(begin)+end-begin]})
 report={'source':str(source),'original_sha256':original,'container_sha256':hashlib.sha256(d).hexdigest(),'patches':patches,'import_descriptors_removed':removed,'original_verify_ranges':ranges,'scope':'Original verify code in DLL loader container; not original shipped-process execution; no hash/parser/filesystem code replacements'}
 (work/'native-container-identity.json').write_text(json.dumps(report,indent=2));print(json.dumps(report));sys.exit()
root=pathlib.Path(sys.argv[2]).resolve();assert root.is_relative_to(work/'fixtures') or (sys.argv[1]=='readonly-package' and root==source.parent.resolve())
os.chdir(root)
mod=C.WinDLL(str(work/'original-verify-container.dll'))
print(json.dumps({'loaded':True,'base':hex(mod._handle),'fixture':root.name}),flush=True)
out=(C.c_uint64*16)(*[0xa5a5a5a5a5a5a5a5]*16)
fn=C.WINFUNCTYPE(C.c_void_p,C.c_void_p)(mod._handle+rva)
class Mem(C.Structure):
 _fields_=[('cb',C.c_uint32),('faults',C.c_uint32)]+[(n,C.c_size_t) for n in ('peak_ws','ws','peak_paged','paged','peak_nonpaged','nonpaged','pagefile','peak_pagefile')]
k=C.WinDLL('kernel32');k.GetCurrentProcess.restype=C.c_void_p;k.K32GetProcessMemoryInfo.argtypes=[C.c_void_p,C.POINTER(Mem),C.c_uint32]
def memory():
 m=Mem();m.cb=C.sizeof(m);assert k.K32GetProcessMemoryInfo(k.GetCurrentProcess(),C.byref(m),m.cb);return m
baseline=memory().ws;started=time.perf_counter();result=fn(C.byref(out));elapsed=(time.perf_counter()-started)*1000;usage=memory();assert result==C.addressof(out)
raw=list(out); tag=raw[0]
if tag>=0x8000000000000000:
 kind={0:'checksum',1:'read',2:'parse',3:'version',4:'ok'}[tag-0x8000000000000000]; report={'kind':kind}
 if kind=='version':report['version']=raw[1]&0xffffffff
 if kind in ('read','parse') and raw[2]>65535 and raw[3]<1000000: report['message']=C.string_at(raw[2],raw[3]).decode('utf8')
else:
 def vec(i):
  cap,ptr,n=raw[i:i+3];assert n<=cap and n<100000
  rows=(C.c_uint64*(n*3)).from_address(ptr) if n else []
  return [C.string_at(rows[k*3+1],rows[k*3+2]).decode('utf8') for k in range(n)]
 report={'kind':'files','lists':[vec(0),vec(3),vec(6)]}
report['elapsed_ms']=elapsed;report['baseline_ws']=baseline;report['ending_ws']=usage.ws;report['process_peak_ws']=usage.peak_ws;report['raw_output_words']=[hex(x) for x in raw[:9]];print(json.dumps(report),flush=True)
