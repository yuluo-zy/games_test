from pathlib import Path
import pefile,struct,json
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path('D:/game/reverse-engineering/tiny-glade');EV=ROOT/'progress-20261020/phase2-toolbar-focus';pe=pefile.PE(str(ROOT/'workspace/input/tiny-glade.exe'),fast_load=True);IM=pe.get_memory_mapped_image();IB=pe.OPTIONAL_HEADER.ImageBase
class Native:
 def __init__(self,rows):
  self.u=u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(IB,(len(IM)+4095)&~4095);u.mem_write(IB,IM);u.mem_map(0x100000,0x3000000);self.heap=0x300000;self.trace=[];self.states={};self.tables=[];self.rows=rows
  def hook(u,a,size,data):
   self.trace.append(a);self.trace=self.trace[-25:]
   if a==0x140c16a00:self.ret(0)
   elif a==0x1400d9b20:self.ret(0x110000)
   elif a==0x140613c10:self.ret(self.alloc(u.reg_read(UC_X86_REG_RCX)))
   elif a==0x140613c20:self.ret(0)
   elif a==0x140613c30:
    old=u.reg_read(UC_X86_REG_RCX);sz=u.reg_read(UC_X86_REG_RDX);p=self.alloc(u.reg_read(UC_X86_REG_R9));u.mem_write(p,bytes(u.mem_read(old,sz)));self.ret(p)
   elif a==0x142923c20:
    dst=u.reg_read(UC_X86_REG_RCX);val=u.reg_read(UC_X86_REG_RDX)&255;n=u.reg_read(UC_X86_REG_R8);u.mem_write(dst,bytes([val])*n);self.ret(dst)
   elif a==0x142923c00:
    dst=u.reg_read(UC_X86_REG_RCX);src=u.reg_read(UC_X86_REG_RDX);n=u.reg_read(UC_X86_REG_R8);u.mem_write(dst,bytes(u.mem_read(src,n)));self.ret(dst)
  u.hook_add(UC_HOOK_CODE,hook);self.w(0x110000,struct.pack('<QQ',1234,5678));self.world=0x120000;meta=0x130000;tables=0x140000;self.wq(self.world+8,meta);self.wq(self.world+0x10,max((x['id']&0xffffffff for x in rows),default=0)+1);self.wq(self.world+0x1a8,tables)
  for tableid,kind in enumerate([0,1,2]):
   rs=[x for x in rows if (0 if 'parent' not in x else 2 if x.get('fpv') else 1)==kind];self.tables.append(rs)
   t=tables+tableid*0x48;entities=self.alloc(8*len(rs));columns=self.alloc(0x90);mapping=self.alloc(24);state=self.alloc(3*len(rs));parents=self.alloc(8*len(rs));ticks=self.alloc(4*len(rs));self.wq(t+8,entities);self.wq(t+0x10,len(rs));self.wq(t+0x18,columns);self.wq(t+0x38,mapping);self.wq(t+0x40,3);self.w(mapping,struct.pack('<QQQ',0xffffffffffffffff,0xfffffffffffffffe if kind else 0,0xfffffffffffffffd if kind==2 else 0));self.wq(columns+0x10,state);self.wq(columns+0x28,ticks);self.wq(columns+0x40,parents);self.wq(columns+0x58,ticks);self.wq(columns+0x70,0x160000)
   for j,x in enumerate(rs):
    eid=x['id'];self.wq(entities+8*j,eid);self.w(state+3*j,bytes(x['state']));self.states[eid]=state+3*j;self.wq(parents+8*j,x.get('parent',0));self.w(meta+(eid&0xffffffff)*20,struct.pack('<IIIII',eid>>32,tableid,j,tableid,j))
  self.eventdata=0x170000;self.eventbuf=0x180000;self.eventarg=0x190000;self.wq(self.eventdata+0x20,512);self.wq(self.eventdata+0x28,self.eventbuf);self.wq(self.eventarg,self.eventdata);self.wq(self.eventarg+0x10,0x190100);self.w(self.eventarg+0x1c,struct.pack('<I',42));self.local=0x1a0000;self.w(self.local,IM[0x142bb2730-IB:0x142bb2730-IB+32]+struct.pack('<QQ',111,222))
 def alloc(self,n):p=self.heap;self.heap+=(max(n,1)+63)&~63;return p
 def w(self,a,b):self.u.mem_write(a,b)
 def wq(self,a,x):self.w(a,struct.pack('<Q',x))
 def q(self,a):return struct.unpack('<Q',self.u.mem_read(a,8))[0]
 def ret(self,val):
  sp=self.u.reg_read(UC_X86_REG_RSP);self.u.reg_write(UC_X86_REG_RAX,val);self.u.reg_write(UC_X86_REG_RIP,self.q(sp));self.u.reg_write(UC_X86_REG_RSP,sp+8)
 def query(self,kind,components):
  state=self.alloc(0x130);arg=self.alloc(32);ids=[i for i,rs in enumerate(self.tables) if rs and (kind=='all' or kind=='nested' and i!=0 or kind=='roots' and i==0)];ptr=self.alloc(4*len(ids));self.w(ptr,b''.join(struct.pack('<I',x) for x in ids));self.wq(state+0xf8,ptr);self.wq(state+0x100,len(ids));self.w(state+0x108,b'\x01');bits=self.alloc(8);self.wq(bits,sum(1<<x for x in ids));self.wq(state+0x28,bits);self.wq(state+0x38,3)
  for i,c in enumerate(components):self.wq(state+0x110+i*8,c)
  self.wq(arg,state);self.wq(arg+8,self.world);self.w(arg+0x14,struct.pack('<I',42));return arg
 def run(self,entry,args):
  self.wq(0x2ffff8,0x100000);self.u.reg_write(UC_X86_REG_RSP,0x2ffff8)
  for reg,v in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],args):self.u.reg_write(reg,v)
  self.u.emu_start(entry,0x100000,count=1000000)
  if self.u.reg_read(UC_X86_REG_RIP)!=0x100000:raise RuntimeError('instruction bound reached')
 def result(self):return {str(k):bytes(self.u.mem_read(a,3)).hex() for k,a in self.states.items()}
if __name__=='__main__':
 eid=lambda i:(1<<32)|i
 rows=[{'id':eid(0),'state':[4,0,8]},{'id':eid(1),'state':[5,0,9],'parent':eid(0)},{'id':eid(2),'state':[6,1,10],'parent':eid(1)},{'id':eid(3),'state':[7,0,11],'parent':eid(1),'fpv':True}]
 for name,entry,ks in [('notify',0x14233d680,('all',[0])),('begin',0x14233a170,('all',[0])),('parent',0x14233dcb0,('nested',[1,0])),('show',0x14233ce00,('nested',[1,2]))]:
  n=Native(rows);args=[n.query(*ks)]
  if name in ['notify','begin']:args.append(n.eventarg)
  if name=='begin':args.append(n.local)
  if name=='parent':args.append(n.query('roots',[0]))
  if name=='show':args.extend([n.query('all',[0]),0x1b0000]);n.w(0x1b0009,b'\x01')
  try:n.run(entry,args);print(name,n.result(),'events',n.q(n.eventdata+0x30))
  except Exception as e:print(name,type(e).__name__,str(e),'rip',hex(n.u.reg_read(UC_X86_REG_RIP)), 'trace',[hex(x) for x in n.trace]);raise
