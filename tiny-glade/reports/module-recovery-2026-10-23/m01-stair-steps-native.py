"""B原1407da0b0完整梯步；标准分配/CRT/writer边界，terrain与random_splits原指令执行。"""
from pathlib import Path
import struct,json,hashlib,pefile,math
from unicorn import *
from unicorn.x86_const import *
root=Path('/Users/yuluo/Desktop/game');raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);u.mem_map(0x100000,0x200000);u.mem_map(0x400000,0x2000000)
def floats(xs):return struct.pack('<'+'f'*len(xs),*xs)
def words(raw,n):return [list(struct.unpack('<'+'I'*n,raw[i:i+4*n])) for i in range(0,len(raw),4*n)]
def ret():
 sp=u.reg_read(UC_X86_REG_RSP);ip=struct.unpack('<Q',u.mem_read(sp,8))[0];u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,ip)
def alloc(n,alignment=16):
 global heap
 heap=(heap+alignment-1)&-alignment;p=heap;heap+=max(16,n);assert heap<0x2300000;u.mem_write(p,bytes(n));return p
records=[];terrain_samples=[];matrices=[];heap=0x1000000;panicked=False
pe.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_IMPORT']]);imports={i.address:i.name.decode() for d in pe.DIRECTORY_ENTRY_IMPORT for i in d.imports if i.name};math_hooks={}
for va in range(0x142923000,0x142925000):
 code=bytes(u.mem_read(va,6))
 if code[:2]==b'\xff\x25':
  name=imports.get(va+6+struct.unpack('<i',code[2:])[0],'')
  if name in ['ceilf','floorf','memcpy','memmove','memset']:math_hooks[va]=name
def hook(uc,ip,size,data):
 global panicked
 if ip==0x140613c10:u.reg_write(UC_X86_REG_RAX,alloc(u.reg_read(UC_X86_REG_RCX),u.reg_read(UC_X86_REG_RDX)));ret()
 elif ip==0x140613c20:ret()
 elif ip==0x14128cdd0:records.append(list(struct.unpack('<24I',u.mem_read(u.reg_read(UC_X86_REG_RDX),96))));ret()
 elif ip==0x1407da562:terrain_samples.append(u.reg_read(UC_X86_REG_XMM0)&0xffffffff)
 elif ip==0x140c90ed0:matrices.append(dict(input=[u.reg_read(UC_X86_REG_XMM1)&0xffffffff,u.reg_read(UC_X86_REG_XMM2)&0xffffffff],out=u.reg_read(UC_X86_REG_RCX)))
 elif ip==0x1407dad20:matrices[-1]['matrix']=list(struct.unpack('<9I',u.mem_read(matrices[-1]['out'],36)));del matrices[-1]['out']
 elif ip in math_hooks:
  name=math_hooks[ip]
  if name in ['ceilf','floorf']:
   x=struct.unpack('<f',u.reg_read(UC_X86_REG_XMM0).to_bytes(16,'little')[:4])[0];v=float((math.ceil if name=='ceilf' else math.floor)(x)) if math.isfinite(x) else x;u.reg_write(UC_X86_REG_XMM0,int.from_bytes(floats([v])+bytes(12),'little'));ret()
  else:
   dst=u.reg_read(UC_X86_REG_RCX);src=u.reg_read(UC_X86_REG_RDX);n=u.reg_read(UC_X86_REG_R8);u.mem_write(dst,bytes([src&255])*n if name=='memset' else bytes(u.mem_read(src,n)));u.reg_write(UC_X86_REG_RAX,dst);ret()
 elif ip in [0x1428d9518,0x1428d9310,0x1428d9000]:panicked=True;u.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
# 原u16 terrain owner头+采样范围；非Bevy容器。实际bilinear_sample原函数照常执行。
heights=[int((i%512)/511*65535) for i in range(512*512)];u.mem_write(0x800000,struct.pack('<'+'H'*len(heights),*heights));terrain=bytearray(0x60);struct.pack_into('<3Q',terrain,0,len(heights),0x800000,len(heights));terrain[0x50:0x58]=floats([-2.5,24.]);u.mem_write(0x408000,bytes(terrain))
rows=[]
cases=[(5,slope,flags,reduced,half_height,1.,.2,False) for slope in [.125,.375,.75,-.375] for flags in [0,1,256,257] for reduced in [0,1] for half_height in [0,1]]
cases += [(n,slope,flags,reduced,half_height,width_scale,aux_base,True) for n in [2,3,6] for slope in [0.,.125,-.75] for flags in [0,1,256,257] for reduced in [0,1] for half_height in [0,1] for width_scale in [.25,2.,4.] for aux_base in [0.,.2]]
for n,slope,flags,reduced,half_height,width_scale,aux_base,curved in cases:
 points=[[i*.25-1.,i*slope+2.5,i*.125-2.+(i*i*.015625 if curved else 0.)] for i in range(n)];widths=[(.75+i*.125)*width_scale for i in range(n)];tangents=[[.875-i*.03125 if curved else .875,.4375+i*.0625 if curved else .4375] for i in range(n)];aux=[aux_base+i*.0625 for i in range(n)];heap=0x1000000;sp=0x1ff008;u.mem_write(sp,struct.pack('<Q',0x100000));u.reg_write(UC_X86_REG_RSP,sp)
 for addr,vals in [(0x401000,sum(points,[])),(0x402000,widths),(0x403000,sum(tangents,[])),(0x404000,aux)]:u.mem_write(addr,floats(vals))
 u.mem_write(0x405000,struct.pack('<Q',0x123456789abcdef0));u.reg_write(UC_X86_REG_RCX,0x402000);u.reg_write(UC_X86_REG_RDX,n);u.reg_write(UC_X86_REG_R8,0x403000);u.reg_write(UC_X86_REG_R9,n)
 args=[(0x28,0x401000),(0x30,n),(0x38,0x405000),(0x40,0x406000),(0x48,42),(0x50,0x407000),(0x58,flags),(0x60,reduced),(0x68,struct.unpack('<I',floats([.1]))[0]),(0x70,0x408000),(0x78,0x404000),(0x80,n),(0x88,half_height)]
 for off,val in args:u.mem_write(sp+off,struct.pack('<Q',val))
 records=[];terrain_samples=[];matrices=[];panicked=False
 try:u.emu_start(0x1407da0b0,0x100000,count=1000000)
 except UcError as e:raise RuntimeError(f'{e} RIP={u.reg_read(UC_X86_REG_RIP):x}')
 assert panicked or u.reg_read(UC_X86_REG_RIP)==0x100000,hex(u.reg_read(UC_X86_REG_RIP))
 rows.append(dict(n=n,width_scale=width_scale,aux_base=aux_base,curved=curved,slope=slope,flags=flags,reduced=reduced,half_height=half_height,points=words(floats(sum(points,[])),3),widths=[x[0] for x in words(floats(widths),1)],tangents=words(floats(sum(tangents,[])),2),aux=[x[0] for x in words(floats(aux),1)],terrain_samples=terrain_samples,matrices=matrices,panic=panicked,records=records,final_seed=struct.unpack('<Q',u.mem_read(0x405000,8))[0]))
Path('/tmp/m01-stair-steps-native.json').write_text(json.dumps(dict(original_sha256=sha,scope='full4617B1407da0b0; original terrain sample+random_splits; only allocator/deallocator/CRT and 96B writer output',rows=rows),indent=2));print('cases',len(rows),'records',sum(len(r['records']) for r in rows),'panic',sum(r['panic'] for r in rows))
