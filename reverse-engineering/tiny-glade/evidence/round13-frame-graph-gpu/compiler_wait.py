import ctypes as c,json,sys
from ctypes import wintypes as w
class Lock(c.Structure): _fields_=[('name',w.WCHAR*128),('timeout',c.c_longlong),('alertable',w.BOOL)]
class Thread(c.Structure): _fields_=[('pid',w.DWORD),('tid',w.DWORD),('wait_ms',w.DWORD),('switches',w.DWORD)]
class Data(c.Union): _fields_=[('lock',Lock),('thread',Thread)]
class Node(c.Structure): _fields_=[('type',c.c_int),('status',c.c_int),('data',Data)]
a=c.WinDLL('advapi32',use_last_error=True)
a.OpenThreadWaitChainSession.argtypes=[w.DWORD,c.c_void_p];a.OpenThreadWaitChainSession.restype=c.c_void_p
a.CloseThreadWaitChainSession.argtypes=[c.c_void_p]
a.GetThreadWaitChain.argtypes=[c.c_void_p,c.c_size_t,w.DWORD,w.DWORD,c.POINTER(w.DWORD),c.POINTER(Node),c.POINTER(w.BOOL)];a.GetThreadWaitChain.restype=w.BOOL
h=a.OpenThreadWaitChainSession(0,None)
try:
 for tid in map(int,sys.argv[1].split(',')):
  n=w.DWORD(16);nodes=(Node*16)();cycle=w.BOOL();ok=a.GetThreadWaitChain(h,0,7,tid,c.byref(n),nodes,c.byref(cycle));v=[]
  for x in nodes[:n.value if ok else 0]:
   v.append({'type':x.type,'status':x.status,**({'pid':x.data.thread.pid,'tid':x.data.thread.tid,'wait_ms':x.data.thread.wait_ms} if x.type==8 else {'name':x.data.lock.name})})
  print(json.dumps({'tid':tid,'ok':bool(ok),'error':c.get_last_error() if not ok else 0,'cycle':bool(cycle.value),'nodes':v}),flush=True)
finally:a.CloseThreadWaitChainSession(h)
