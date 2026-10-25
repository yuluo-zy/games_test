"""Execute unmodified original verify code in an isolated loader container, never the shipped process."""
import ctypes as C, hashlib, json, os, pathlib, struct, sys, time
source=pathlib.Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
work=pathlib.Path(__file__).resolve().parent
rva=0x67c3d0
assert sys.argv[1] == 'inspect'
root=pathlib.Path(sys.argv[2]).resolve();assert root.is_relative_to(work/'fixtures')
os.chdir(root)
container=pathlib.Path('D:/game/reverse-engineering/tiny-glade/evidence/small-libs-development-2026-10-13/tg-package-integrity/original-verify-container.dll')
assert hashlib.sha256(source.read_bytes()).hexdigest() == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
assert hashlib.sha256(container.read_bytes()).hexdigest() == '281fc876c5543e3553856fdfc0fb925bbf9be72e2dce375b39ec43aec1771772'
mod=C.WinDLL(str(container))
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

