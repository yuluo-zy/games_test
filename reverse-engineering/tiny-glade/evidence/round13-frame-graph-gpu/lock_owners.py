import ctypes as c,json,os
from ctypes import wintypes as w
class FT(c.Structure): _fields_=[('low',w.DWORD),('high',w.DWORD)]
class UP(c.Structure): _fields_=[('pid',w.DWORD),('start',FT)]
class PI(c.Structure): _fields_=[('process',UP),('app',w.WCHAR*256),('service',w.WCHAR*64),('type',c.c_int),('status',w.ULONG),('session',w.DWORD),('restartable',w.BOOL)]
r=c.WinDLL('rstrtmgr')
paths=['C:/Users/liyu/.cargo/.package-cache','C:/Users/liyu/.cargo/.package-cache-mutate','D:/game/tiny-glade/target-round12/debug/.cargo-lock']
for p in paths:
 h=w.DWORD();key=c.create_unicode_buffer(33);started=r.RmStartSession(c.byref(h),0,key)
 try:
  a=(w.LPCWSTR*1)(p);err=r.RmRegisterResources(h,1,a,0,None,0,None);need=w.UINT();n=w.UINT();why=w.DWORD();err2=r.RmGetList(h,c.byref(need),c.byref(n),None,c.byref(why));items=(PI*max(need.value,1))();n.value=need.value;err3=r.RmGetList(h,c.byref(need),c.byref(n),items,c.byref(why));print(json.dumps({'path':p,'start':started,'register':err,'list':err3,'owners':[{'pid':x.process.pid,'app':x.app} for x in items[:n.value]]}),flush=True)
 finally:r.RmEndSession(h)
