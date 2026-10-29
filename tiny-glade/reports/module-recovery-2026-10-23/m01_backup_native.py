"""B1408238a0原控制流；只桥接容器clone/get/clear、同步Vec与stairs owner方法。"""
from pathlib import Path
import json,hashlib,struct,itertools
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;blob=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(blob)+4095)&~4095);u.mem_write(base,blob)
D,S,STOP=0x20000000,0x30000000,0x200ff000
u.mem_map(D,0x100000);u.mem_map(S,0x100000)
def q(a,x):u.mem_write(a,struct.pack('<Q',x))
def uq(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
def ret(value=None,second=None):
 sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,uq(sp));u.reg_write(UC_X86_REG_RSP,sp+8)
 if value is not None:u.reg_write(UC_X86_REG_RAX,value)
 if second is not None:u.reg_write(UC_X86_REG_RDX,second)
def alloc(n):
 global heap
 p=heap;heap=(heap+n+31)&~31;assert heap<D+0xf0000;return p
def bucket(ptr,value):
 u.mem_write(ptr,bytes(0x90)); ctrl=alloc(0x100)+0x80
 u.mem_write(ctrl,b'\0'+b'\xff'*15);q(ptr+0x30,ctrl);q(ptr+0x38,15);q(ptr+0x48,1)
 u.mem_write(ctrl-0x50,bytes(80));u.mem_write(ctrl-0x50,struct.pack('<I',7))
 u.mem_write(ctrl-0x48+0x38,struct.pack('<I',value));u.mem_write(ctrl-0x48+0x47,b'\1')
def copy_bucket(dst,src):
 value=struct.unpack('<I',u.mem_read(uq(src+0x30)-0x48+0x38,4))[0]
 bucket(dst,value)
def hook(uc,a,size,_):
 global panic
 r=uc.reg_read;cx,dx=r(UC_X86_REG_RCX),r(UC_X86_REG_RDX)
 if a in (0x140a1cf30,0x140a1c2c0,0x140a1cde0):
  addresses.append({'callee':hex(a),'address_kind':dx,'wall':r(UC_X86_REG_R8)if dx==0 else None})
 if a==0x140a1cf30:trace.append('get_backup_mut');ret(saved);return
 if a==0x140a1c2c0:
  if cx==live:trace.append('get_live');ret(0 if case['missing_live'] else live_bucket)
  else:trace.append('get_backup');ret(saved if case['saved'] else 0)
  return
 if a==0x140a1cde0:trace.append('get_live_mut');ret(0 if case['missing_live'] else live_bucket,D+0xc000);return
 if a==0x1412573b0:trace.append('copy_bucket_to_backup');copy_bucket(cx,dx);ret();return
 if a==0x141257900:trace.append('clear_live_bucket');ret();return
 if a==0x140a1b430:ret();return
 if a==0x141256e40:
  trace.append('restore_saved_bucket'if dx==saved else 'restore_archived_bucket');copy_bucket(cx,dx);ret();return
 if a==0x1400885d0:ret(uq(cx));return
 if a==0x140a1b1e0:trace.append('clear_preview_state');ret(uq(cx));return
 if a in (0x141262fb0,0x141263210,0x141262b00):
  name={0x141262fb0:'subtypes',0x141263210:'blueprints',0x141262b00:'transforms'}[a]
  trace.append('copy_'+name);u.mem_write(cx,bytes(u.mem_read(dx,32)));ret();return
 if a==0x1408335a0:
  trace.append('copy_events');count=uq(dx+24);payload=alloc(max(1,count)*136)
  if count:u.mem_write(payload,bytes(u.mem_read(uq(dx+16),count*136)))
  q(cx,0);q(cx+8,count);q(cx+16,payload);q(cx+24,count);ret();return
 if a==0x14125e710:
  trace.append('copy_records_tail');n=(r(UC_X86_REG_R8)-dx)//32
  if n:
   assert uq(cx)>=uq(cx+16)+n
   u.mem_write(uq(cx+8)+uq(cx+16)*32,bytes(u.mem_read(dx,n*32)));q(cx+16,uq(cx+16)+n)
  ret();return
 if a==0x1408ff210:trace.append('restore_stairs');ret();return
 if a==0x140824680:
  trace.append('reset_default');u.mem_write(cx,bytes(0x288));q(cx+8,8);q(cx+0x270,8);ret();return
 if a==0x140a1b760:trace.append('clear_backup_storage');u.mem_write(cx,bytes(0x1b8));ret();return
 if a in (0x140613c20,0x1405f7140):ret();return
 if a==0x1428d9310:panic=hex(a);trace.append('panic_missing_live');uc.emu_stop();return
u.hook_add(UC_HOOK_CODE,hook)
def vec(owner,value,stride):
 data=alloc(2*stride);q(owner,2);q(owner+8,data);q(owner+16,2)
 for i in range(2):u.mem_write(data+i*stride,struct.pack('<QQQQ',i,value,0x1234,0xabcd))
def queue(owner,value):
 data=alloc(136);u.mem_write(data,bytes([value])*136);q(data,1)
 q(owner,0);q(owner+8,1);q(owner+16,data);q(owner+24,1)
def get_queue(owner):return list(u.mem_read(uq(owner+16),136))if uq(owner+24)else[]
def get_records(owner):return [list(struct.unpack('<QQQQ',u.mem_read(uq(owner+8)+i*32,32)))for i in range(uq(owner+16))]
def run(c):
 global case,trace,addresses,panic,heap,live,saved,live_bucket
 case=c;trace=[];addresses=[];panic=None;heap=D+0x20000;u.mem_write(D,bytes(0x10000));u.mem_write(S,bytes(0x100000))
 archive,live,subtypes,blueprints,transforms,events,history,backup,stairs=[D+0x1000+i*0x1000 for i in range(9)]
 live_bucket=D+0xb000;saved=D+0xb100;arch_ctrl=D+0xd000
 u.mem_write(arch_ctrl,b'\0'+b'\xff'*15);q(archive,arch_ctrl);q(archive+8,15);q(archive+24,1);q(arch_ctrl-0x98,7)
 bucket(arch_ctrl-0x90,4);bucket(live_bucket,3);bucket(saved,8)
 if c.get('empty_archives'):
  q(archive+24,0);u.mem_write(arch_ctrl,b'\xff'*16)
 if c.get('specials'):
  bucket(archive+0xc0,4);q(archive+0xc0,D+0xe000)
  bucket(archive+0x150,4);q(archive+0x150,D+0xe000)
 for ptr in [subtypes,blueprints,transforms]:q(ptr,3)
 for offset in [0x230,0x1d0,0x200]:q(backup+offset,8)
 vec(history,3,32);vec(backup,8,32);queue(events,3);queue(backup+0x260,8)
 u.mem_write(history+0x48,bytes([c['pending']]));u.mem_write(backup+0x280,bytes([c['reset'],c['created'],c['state']]))
 wrappers=[]
 for i,owner in enumerate([archive,live,subtypes,blueprints,transforms,events,history,backup,stairs]):
  w=D+i*64;wrappers.append(w);q(w,owner);q(w+16,D+0x800);u.mem_write(w+28,struct.pack('<I',77))
 sp=S+0xf0008;q(sp,STOP)
 for offset,wrapper in zip([0x28,0x30,0x38,0x40,0x48],wrappers[4:]):q(sp+offset,wrapper)
 for reg,value in [(UC_X86_REG_RCX,wrappers[0]),(UC_X86_REG_RDX,wrappers[1]),(UC_X86_REG_R8,wrappers[2]),(UC_X86_REG_R9,wrappers[3]),(UC_X86_REG_RSP,sp)]:u.reg_write(reg,value)
 try:u.emu_start(0x1408238a0,STOP,count=100000)
 except Exception:print('rip',hex(u.reg_read(UC_X86_REG_RIP)),c,trace);raise
 assert panic or u.reg_read(UC_X86_REG_RIP)==STOP,(hex(u.reg_read(UC_X86_REG_RIP)),c,trace)
 live_info=uq(live_bucket+0x30)-0x48
 return {'input':c,'panic':panic,'trace':trace,'addresses':addresses,'flags':list(u.mem_read(backup+0x280,3)),
  'live_maps':[uq(p)for p in [subtypes,blueprints,transforms]],'backup_maps':[uq(backup+o)for o in [0x230,0x1d0,0x200]],
  'live_info_value':struct.unpack('<I',u.mem_read(live_info+0x38,4))[0],'live_info_state':u.mem_read(live_info+0x47,1)[0],
  'live_events':get_queue(events),'backup_events':get_queue(backup+0x260),'live_records':get_records(history),'backup_records':get_records(backup)}
rows=[]
for state,created,reset,pending,saved_flag,missing in itertools.product([0,1,2,255],*([[False,True]]*5)):
 rows.append(run({'state':state,'created':created,'reset':reset,'pending':pending,'saved':saved_flag,'missing_live':missing}))
special_rows=[run({'state':state,'created':True,'reset':False,'pending':True,'saved':True,'missing_live':False,'specials':True})for state in [1,2]]
assert all([v['address_kind']for v in r['addresses']]==([0,0,1,1,2,2]if r['input']['state']==1 else[0,1,2,0,0,1,1,2,2])for r in special_rows)
empty_rows=[run({'state':state,'created':True,'reset':False,'pending':True,'saved':True,'missing_live':False,'empty_archives':True})for state in [1,2]]
assert all(not r['addresses']for r in empty_rows)
Path(__file__).with_name('m01-backup-native.json').write_text(json.dumps({'binary_sha256':sha,'entry':'0x1408238a0','bridges':['storage lookup','bucket clear/clone','map clone','locked event Vec clone','record tail extension','stairs restore','default/drop'],'rows':rows,'special_chain_rows':special_rows,'empty_archives_rows':empty_rows},indent=2)+'\n')
print(json.dumps({'cases':len(rows),'panics':sum(bool(x['panic'])for x in rows)}))
lines=['// 原1408238a0执行结果；由m01_backup_native.py生成，非Rust实现自算。','const NATIVE_BACKUP_CASES: &[NativeBackupCase] = &[']
for r in rows:
 c=r['input']; boolean=lambda x:str(bool(x)).lower()
 fields=['state:'+str(c['state'])]+[k+':'+boolean(c[k])for k in ['created','reset','pending','saved','missing_live']]
 fields+=['panic:'+boolean(r['panic']),'flags:'+str(r['flags']),'live_maps:'+str(r['live_maps']),'backup_maps:'+str(r['backup_maps']),
  'info_value:'+str(r['live_info_value']),'info_state:'+str(r['live_info_state']),
  'live_event:'+str(r['live_events'][8]),'backup_event:'+('Some('+str(r['backup_events'][8])+')'if r['backup_events']else'None'),
  'live_record:'+str(r['live_records'][0][1]),'backup_record:'+('Some('+str(r['backup_records'][0][1])+')'if r['backup_records']else'None'),
  'restored_stairs:'+boolean('restore_stairs'in r['trace'])]
 lines.append('NativeBackupCase{'+','.join(fields)+'},')
lines.append('];')
Path(__file__).with_name('m01-backup-native-cases.rs').write_text('\n'.join(lines)+'\n')
