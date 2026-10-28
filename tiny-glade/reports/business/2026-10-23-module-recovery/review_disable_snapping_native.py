"""完整原关闭吸附提示consumer；仅桥接Rust分配器，未跳过业务或alternate。"""
from pathlib import Path
import hashlib,json,struct,itertools,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();digest=hashlib.sha256(raw).hexdigest();assert digest=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;im=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(im)+4095)&~4095);u.mem_write(base,im)
data,stack,stop=0x20000000,0x30000000,0x2000f000;u.mem_map(data,0x10000);u.mem_map(stack,0x10000)
def qw(addr,v):u.mem_write(addr,struct.pack('<Q',v))
def hook(uc,addr,size,ud):
 if addr==0x140613c10:
  assert uc.reg_read(UC_X86_REG_RCX)==96 and uc.reg_read(UC_X86_REG_RDX)==8
  rsp=uc.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',uc.mem_read(rsp,8))[0];uc.reg_write(UC_X86_REG_RAX,data+0x8000);uc.reg_write(UC_X86_REG_RSP,rsp+8);uc.reg_write(UC_X86_REG_RIP,ret)
u.hook_add(UC_HOOK_CODE,hook)
def res(addr,body):
 qw(addr,body);qw(addr+0x10,data+0xe000)
def writer(addr,body,items):
 res(addr,body);qw(body+0x20,16);qw(body+0x28,items)
def run(mode,shape,pdone,sdone,events,split,key):
 u.mem_write(data,bytes(0xf000));u.mem_write(stack,bytes(0x10000))
 res(data,data+0x1000);u.mem_write(data+0x1000,struct.pack('<IB',mode,shape))
 res(data+0x40,data+0x2000);u.mem_write(data+0x21fc,bytes([pdone]));u.mem_write(data+0x2120,struct.pack('<III',*key))
 res(data+0x80,data+0x3000);u.mem_write(data+0x3000,bytes([sdone]))
 reader,ev,items1,items2=data+0xc0,data+0x4000,data+0x4200,data+0x4400
 qw(reader,data+0x4100);qw(reader+8,ev)
 qw(ev+8,items1);qw(ev+0x10,split);qw(ev+0x18,0)
 qw(ev+0x28,items2);qw(ev+0x30,len(events)-split);qw(ev+0x38,split);qw(ev+0x40,len(events))
 for i,b in enumerate(events):u.mem_write((items1+i*16) if i<split else (items2+(i-split)*16),struct.pack('<QB7x',i,b))
 writer(data+0x100,data+0x5000,data+0x6000);writer(data+0x140,data+0x5200,data+0x7000)
 rsp=stack+0xf008;qw(rsp,stop);qw(rsp+0x28,data+0x100);qw(rsp+0x30,data+0x140)
 for reg,val in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RCX,data),(UC_X86_REG_RDX,data+0x40),(UC_X86_REG_R8,data+0x80),(UC_X86_REG_R9,data+0xc0)]:u.reg_write(reg,val)
 u.emu_start(0x14227e140,stop,count=10000);assert u.reg_read(UC_X86_REG_RIP)==stop
 done=u.mem_read(data+0x3000,1)[0];completed=struct.unpack('<Q',u.mem_read(data+0x5030,8))[0];shown=struct.unpack('<Q',u.mem_read(data+0x5230,8))[0]
 assert completed in [0,1] and shown in [0,1]
 if completed:assert u.mem_read(data+0x6008,1)[0]==3
 placement,query,outkey=0,0,[195,0,0]
 if shown:
  placement=struct.unpack('<I',u.mem_read(data+0x7000,4))[0];assert struct.unpack('<H',u.mem_read(data+0x7048,2))[0]==0x300
  assert struct.unpack('<I',u.mem_read(data+0x7018,4))[0]==0xca
  assert struct.unpack('<Q',u.mem_read(data+0x7030,8))[0]==2
  pointer=struct.unpack('<Q',u.mem_read(data+0x7028,8))[0]
  assert struct.unpack('<I',u.mem_read(pointer,4))[0]==0xc9
  query=struct.unpack('<H',u.mem_read(pointer+4,2))[0]
  assert struct.unpack('<I',u.mem_read(pointer+48,4))[0]==0xc6
  outkey=list(struct.unpack('<III',u.mem_read(pointer+52,12)))
 return bool(done),completed,shown,placement,query,outkey
if __name__=='__main__':
 sequences=[[],[False],[True],[False,True],[True,False],[False,False],[True,True]]
 keys=[[1,0,0],[20,0,0],[195,0,0],[0,0,0],[0,1,4294967295],[0,3|(65535<<16),0]]
 rows=[]
 for mode,shape,pdone,sdone,events,key in itertools.product([0,6,7,10],range(3),[False,True],[False,True],sequences,keys):
  for split in range(len(events)+1):
   result=run(mode,shape,pdone,sdone,events,split,key)
   rows.append((mode,shape,pdone,sdone,events,split,key,result))
 def ron(v):
  if isinstance(v,bool):return str(v).lower()
  if isinstance(v,(tuple,list)):return ('(' if isinstance(v,tuple) else '[')+','.join(map(ron,v))+(')' if isinstance(v,tuple) else ']')
  return str(v)
 fixture=root/'tiny-glade/tests/fixtures/native/hint-disable-snapping.ron';fixture.write_text('[\n'+''.join(ron(row)+',\n' for row in rows)+']\n')
 report={'binary_sha256':digest,'function':'0x14227e140','return':'0x14227e3d4','whole_function':True,'bridges':[{'address':'0x140613c10','boundary':'std allocator alloc(size96,align8), return isolated zeroed mapped storage'}],'cases':len(rows),'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'scope':'原consumer和packed/alternate业务完整执行；4模式×3形状×2档案标志×2local标志，7种事件序列及双缓冲全部切分，命名/None/三类原生键语义。比较local完成、Complete(3)和Display请求计数、placement/SteamQuery/原KeyCode12字节；padding不比较。无OS/Steam SDK/GPU声明。'}
 report_path=root/'tiny-glade/reports/business/2026-10-23-module-recovery/hint-disable-snapping-native.json';report_path.write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
 print(len(rows),'cases')
