"""原insert_point_ui完整执行；仅固定标准HashMap的hash桶，业务函数全部执行原指令。"""
import json,hashlib,struct,random
from pathlib import Path
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D,S,STOP=0x20000000,0x30000000,0x200ff000;u.mem_map(D,0x100000);u.mem_map(S,0x100000)
OWNER,CTRL,POINTS,TANGENTS,WIDTHS,CURSOR,OUT=D+0x1000,D+0x3000,D+0x4000,D+0x5000,D+0x6000,D+0x7000,D+0x8000
q=lambda a,v:u.mem_write(a,struct.pack('<Q',v))
w=lambda a,v:u.mem_write(a,struct.pack('<I',v))
def hook(uc,at,size,data):
 if at==0x1405b0d70:
  sp=u.reg_read(UC_X86_REG_RSP);target=struct.unpack('<Q',u.mem_read(sp,8))[0]
  u.reg_write(UC_X86_REG_RAX,0);u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,target)
 if at==0x1428d9430:raise RuntimeError('unexpected original panic')
u.hook_add(UC_HOOK_CODE,hook)
def run(name,points,widths,tangents,cursor,present=True):
 table=OWNER+0x138;u.mem_write(CTRL,b'\x00'+b'\xff'*31)
 q(table,CTRL);q(table+8,15);q(table+24,int(present));q(CTRL-0x78,(7<<32)|19)
 for ptr,items,fmt in [(POINTS,points,'fff'),(TANGENTS,tangents,'ff'),(WIDTHS,[(x,)for x in widths],'f')]:
  if items:u.mem_write(ptr,b''.join(struct.pack('<'+fmt,*x)for x in items))
 for off,ptr,n in [(0x68,POINTS,len(points)),(0x50,TANGENTS,len(tangents)),(0x38,WIDTHS,len(widths))]:q(CTRL-off,ptr);q(CTRL-off+8,n)
 u.mem_write(CURSOR,struct.pack('<fff',*cursor));u.mem_write(OUT,b'\xa5'*64)
 sp=S+0xff008;q(sp,STOP);q(sp+0x28,OWNER)
 for reg,val in [(UC_X86_REG_RCX,OUT),(UC_X86_REG_RDX,7),(UC_X86_REG_R8,19),(UC_X86_REG_R9,CURSOR),(UC_X86_REG_RSP,sp),(UC_X86_REG_MXCSR,0x1f80),(UC_X86_REG_RFLAGS,0x202)]:u.reg_write(reg,val)
 u.emu_start(0x14079bc20,STOP,count=50000)
 tag=struct.unpack('<I',u.mem_read(OUT,4))[0]
 return dict(name=name,points=points,widths=widths,tangents=tangents,cursor=cursor,present=present,output_words=list(struct.unpack('<13I',u.mem_read(OUT,52))),tag=tag)
rows=[run('missing',[],[],[],[1,2,3],False),run('empty',[],[],[],[1,2,3]),run('single',[[0,0,0]],[2],[[1,0]],[1,2,3])]
r=random.Random(726)
for i in range(96):
 n=2+i%6;points=[[r.uniform(-8,8)for _ in range(3)]for _ in range(n)];widths=[r.uniform(-1,4)for _ in range(n)];tangents=[[r.uniform(-2,2),r.uniform(-2,2)]for _ in range(n)]
 if i%8==0:points[1]=points[0][:]
 if i%9==0:tangents=[[0,0]for _ in range(n)]
 if i%10==0:widths=widths[:-1]
 rows.append(run(str(i),points,widths,tangents,[r.uniform(-10,10)for _ in range(3)]))
for name,points in [('degenerate',[[0,0,0],[0,0,0]]),('tie',[[0,0,0],[2,0,0],[0,0,0]])]:rows.append(run(name,points,[2]*len(points),[[1,0]]*len(points),[1,1,0]))
out=Path(__file__).with_name('m01-root-insert-point-native.json')
with out.open('x')as f:json.dump({'binary_sha256':sha,'entry':'0x14079bc20','bridge':'standard HashMap hash_one returns fixed bucket zero; no game helper bridges','rows':rows},f,indent=2);f.write('\n')
print(json.dumps({'file':str(out),'cases':len(rows)}))
