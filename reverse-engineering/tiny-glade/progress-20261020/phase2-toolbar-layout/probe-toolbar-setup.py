from pathlib import Path
import pefile,struct,json,hashlib
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
r=Path('D:/game');ev=r/'reverse-engineering/tiny-glade/progress-20261020/phase2-toolbar-layout';data=(r/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes();pe=pefile.PE(data=data,fast_load=True);im=pe.get_memory_mapped_image();ib=pe.OPTIONAL_HEADER.ImageBase
all_results=[]
for demo in [0,1]:
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(ib,(len(im)+4095)&~4095);u.mem_write(ib,im);u.mem_map(0x100000,0x2000000);heap=[0x200000];
 def q(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
 def ret(v=0):
  sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RAX,v);u.reg_write(UC_X86_REG_RIP,q(sp));u.reg_write(UC_X86_REG_RSP,sp+8)
 def hook(u,a,size,user):
  if a==0x140613c10:
   n=u.reg_read(UC_X86_REG_RCX);p=heap[0];heap[0]+=(n+31)&~31;ret(p)
  elif a==0x142923c00:
   dst=u.reg_read(UC_X86_REG_RCX);src=u.reg_read(UC_X86_REG_RDX);n=u.reg_read(UC_X86_REG_R8);u.mem_write(dst,bytes(u.mem_read(src,n)));ret(dst)
  elif not 0x142345968<=a<0x142346e68:
   raise RuntimeError(f'unexpected external {a:x}')
 u.hook_add(UC_HOOK_CODE,hook);u.mem_write(0x14362c098,struct.pack('<Q',0x110000));u.mem_write(0x110000,bytes([demo]));
 for reg,v in [(UC_X86_REG_RBP,0x180000),(UC_X86_REG_RSP,0x17ff80),(UC_X86_REG_R12,0x130000),(UC_X86_REG_R13,0x140000),(UC_X86_REG_RSI,0x150000),(UC_X86_REG_R14,42),(UC_X86_REG_R15,0x160000)]:u.reg_write(reg,v)
 u.emu_start(0x142345968,0x142346e68,count=20000)
 def item(p):
  b=bytes(u.mem_read(p,64));k=b[0];label=q(p+0x20);ln=q(p+0x28);out={'kind':k,'label':bytes(u.mem_read(label,ln)).decode() if ln else '', 'size':struct.unpack_from('<ff',b,0x30),'show_only_fpv':b[0x38]}
  if k in [4,5]:out['children']=[item(q(p+0x10)+64*i) for i in range(q(p+0x18))]
  elif k==0:out['custom_data']=q(p+8);out['custom_vtable']=hex(q(p+16));out['custom_bytes']=bytes(u.mem_read(q(p+8),32)).hex() if q(p+8)>8 else None
  elif k==1:out['mode_raw']=b[8:24].hex()
  elif k in [2,3]:out['sub']=b[1]
  return out
 p=u.reg_read(UC_X86_REG_R12);categories=[]
 for i in range(11):
  a=p+48*i;lab=q(a);ln=q(a+8);cnt=q(a+40);cats={'label':bytes(u.mem_read(lab,ln)).decode(),'tag':u.mem_read(a+16,1)[0],'items':[item(q(a+32)+64*j) for j in range(cnt)]};categories.append(cats)
 result={'demo':bool(demo),'binary_sha256':hashlib.sha256(data).hexdigest(),'bounded_original_range':['0x142345968','0x142346e68'],'categories':categories};all_results.append(result);print('demo',demo,[(x['tag'],x['label'],len(x['items'])) for x in categories])
(ev/'native-toolbar-setup-trees.json').write_text(json.dumps(all_results,indent=2),encoding='utf-8')
