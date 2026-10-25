"""原视频页有界指令差分：预设表构造／识别／应用、FPS槽位。
执行发行RNE，不替换业务指令；不执行UI/分配器/设备查询，入口所需Res指针由探针提供。
"""
from pathlib import Path
import hashlib,itertools,json,struct,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
data,stack=0x20000000,0x30000000
u.mem_map(data,0x10000);u.mem_map(stack,0x10000)
bp=stack+0x8000
u.reg_write(UC_X86_REG_RBP,bp);u.reg_write(UC_X86_REG_RSP,stack+0x7000)
def put(addr,fmt,*v):u.mem_write(addr,struct.pack('<'+fmt,*v))
def get(addr,fmt):return struct.unpack('<'+fmt,u.mem_read(addr,struct.calcsize('<'+fmt)))
def fbits(x):return struct.unpack('<I',struct.pack('<f',x))[0]
def execute(start,ends):
 def stop(uc,address,size,_):
  if address in ends:uc.emu_stop()
 h=u.hook_add(UC_HOOK_CODE,stop)
 try:u.emu_start(start,base+len(image),count=10000)
 finally:u.hook_del(h)
 end=u.reg_read(UC_X86_REG_RIP);assert end in ends,hex(end)
 return end
# 表中业务字段完全由原mov指令产生，未使用候选常量。
table=data+0x1000
u.reg_write(UC_X86_REG_RAX,table);execute(0x140247d3b,{0x140247d8e})
qbytes=bytes(u.mem_read(table,48))
qrows=[(qbytes[i*12],get(table+i*12+4,'I')[0],qbytes[i*12+8],qbytes[i*12+9],qbytes[i*12+10]) for i in range(4)]
scales=[fbits(x) for x in [-1.,0.,0.5,1.,1.5,2.]]+[0x7fc01234,0x7f800000]
match=[]
for tag,scale,mode,preset,dof in itertools.product(range(2),scales,range(2),range(5),range(2)):
 cfg=bytearray(0xc8);struct.pack_into('<II',cfg,0x88,tag,scale);cfg[0xab]=mode;cfg[0xc3]=preset;cfg[0xa9]=dof
 u.mem_write(data,bytes(cfg));u.reg_write(UC_X86_REG_RDI,data);u.reg_write(UC_X86_REG_RSI,table);u.reg_write(UC_X86_REG_RAX,data+0x1100)
 end=execute(0x140247dce,{0x140247ea5,0x140247f5d})
 expected=u.reg_read(UC_X86_REG_R8) if end==0x140247ea5 else -1
 match.append((tag,scale,mode,preset,dof,expected))
apply=[]
for quality,maximum,tag,scale,phase in itertools.product(range(4),scales,range(2),[fbits(1.),fbits(1.5),fbits(2.),0x7fc01234],range(4)):
 cfg=bytearray(0xc8);struct.pack_into('<II',cfg,0x88,tag,scale);cfg[0xab]=0;cfg[0xc3]=0;cfg[0xa9]=0
 u.mem_write(data,bytes(cfg));u.mem_write(data+0x2000,b'\x00'*0x1000)
 put(data+0x2000,'QQQI',data,0,data+0x3000,0);put(data+0x201c,'I',42)
 put(data+0x2100,'QQQI',data+0x2200,0,data+0x3004,0);put(data+0x211c,'I',42)
 put(data+0x2300,'QQQI',data+0x2400,0,data+0x3008,0);put(data+0x231c,'I',42)
 put(data+0x2218,'I',fbits(0.7));put(data+0x2400,'Q',phase)
 put(bp+0x318,'Q',data+0x2500);put(bp+0x330,'Q',data+0x2100);put(bp+0x338,'Q',data+0x2300)
 put(bp+0x250,'Q',data+0x2200);put(bp+0x210,'Q',data+0x2400)
 u.reg_write(UC_X86_REG_RDI,data);u.reg_write(UC_X86_REG_R14,data+0x2000);u.reg_write(UC_X86_REG_RBX,table)
 u.reg_write(UC_X86_REG_RCX,quality);u.reg_write(UC_X86_REG_XMM6,maximum)
 execute(0x140248016,{0x1402483c4})
 newtag,newscale=get(data+0x88,'II');frame=get(data+0x2218,'I')[0];newphase=get(data+0x2400,'Q')[0]
 apply.append((quality,maximum,tag,scale,phase,*get(data+0xab,'B'),*get(data+0xc3,'B'),*get(data+0xa9,'B'),newtag,newscale,frame,newphase,get(data+0x2500,'B')[0]))
fps=[]
floats=[-float('inf'),-1.,-0.,0.,14.,15.,15.9,19.9,20.,29.9,30.,59.9,60.,60.9,71.9,72.,79.9,80.,89.9,90.,119.9,120.,120.9,144.,4294967296.,float('inf')]
for bits in [fbits(x) for x in floats]+[0x7fc01234]:
 put(data+0x98,'I',bits);u.reg_write(UC_X86_REG_RDI,data)
 execute(0x140249e29,{0x140249ec6});fps.append((bits,u.reg_read(UC_X86_REG_RAX)))
fixture=root/'tiny-glade/tests/fixtures/native'
def save(name,rows):
 p=fixture/name;p.write_text('[\n'+''.join('('+','.join(map(str,row))+'),\n' for row in rows)+']\n');return hashlib.sha256(p.read_bytes()).hexdigest()
files={name:save(name,rows) for name,rows in [('toolbar-video-quality-match.ron',match),('toolbar-video-quality-apply.ron',apply),('toolbar-video-fps.ron',fps)]}
report={'binary_sha256':hashlib.sha256(raw).hexdigest(),'pdb_sha256':'2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911','preset_records':qrows,'cases':{'quality_match':len(match),'quality_apply':len(apply),'fps':len(fps)},'fixtures':files,'executed_ranges':[['0x140247d3b','0x140247d8e'],['0x140247dce',['0x140247ea5','0x140247f5d']],['0x140248016','0x1402483c4'],['0x140249e29','0x140249ec6']],'bridges':[],'supplied_boundaries':['私有ABI内存和Res指针','预设应用入口已由UI选择的枚举值，跳过UI used检验'],'scope':'仅原预设识别／应用、FPS索引有界业务指令，不覆盖UI、枚举GPU、预算查询或OS输入。'}
Path(__file__).with_name('toolbar-video-values-native.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
print(report['cases'],qrows)
