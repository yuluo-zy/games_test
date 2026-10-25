"""Original freehand arches fixtures: whole original profile/walker/cache. No game rule bridges."""
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/"evidence/building-arches"))
import native_arches as regular_native
from native_arches import *
def walk_arch_profiles(segments,wall_id,source_length,first_ordinal=1):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000));view=create_brick_writer(256);public=data+0x1000;cpu.mem_write(public+0x28c,struct.pack('<f',source_length));idptr=data+0x200;cpu.mem_write(idptr,struct.pack('<Q',wall_id));ordinalptr=data+0x210;indexptr=data+0x220;growth=data+0x800;env=data+0x900;cpu.mem_write(env,struct.pack('<6Q',ordinalptr,indexptr,idptr,public,view['writer'],growth));options=data+0xa00;cpu.mem_write(options,struct.pack('<Ifff',0,1.,.56,.1))
 for ordinal,s in enumerate(segments,first_ordinal):
  n=len(s['heights']);hp=alloc(n*4);gp=alloc(n*12);up=alloc(n*4);cpu.mem_write(hp,struct.pack('<'+'f'*n,*s['heights']));cpu.mem_write(gp,b''.join(struct.pack('<3f',*p)for p in s['positions']));cpu.mem_write(up,struct.pack('<'+'f'*n,*s['us']));cpu.mem_write(ordinalptr,struct.pack('<Q',ordinal));cpu.mem_write(indexptr,bytes(8))
  sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,options);cpu.reg_write(UC_X86_REG_RDX,hp);cpu.reg_write(UC_X86_REG_R8,n);cpu.reg_write(UC_X86_REG_R9,gp);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
  for offset,v in {0x28:n,0x30:up,0x38:n,0x40:env}.items():cpu.mem_write(sp+offset,struct.pack('<Q',v))
  cpu.emu_start(0x141232bb0,stop,count=10000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop,'arch walker did not return'
 assert bytes(cpu.mem_read(growth,1))[0]==0,'arch writer pool exhausted'
 return brick_records(view)

def accumulate_freehand_arch_profiles(segments,trims,arch,third,prepared,public_284,public_288,clutter):
 """Original caller complete third iteration and freehand15-pass cache owner."""
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000));n=len(segments);sptr=alloc(n*104)
 for i,s in enumerate(segments):
  v=sptr+i*104
  for offset,rows,width in [(0,s['heights'],4),(24,s['positions'],12),(48,s['us'],4)]:
   payload=struct.pack('<'+'f'*len(rows),*rows)if width==4 else b''.join(struct.pack('<3f',*p)for p in rows);p=alloc(len(payload));cpu.mem_write(p,payload);cpu.mem_write(v+offset,struct.pack('<3Q',len(rows),p,len(rows)))
  cpu.mem_write(v+72,bytes.fromhex(s['tail'])+bytes(4))
 tp=alloc(len(trims)*8);cpu.mem_write(tp,b''.join(struct.pack('<2f',*p)for p in trims));ap=alloc(len(arch)*4);cpu.mem_write(ap,struct.pack('<'+'f'*len(arch),*arch));thp=alloc(len(third)*4);cpu.mem_write(thp,struct.pack('<'+'f'*len(third),*third));pubend=data+0x12a0
 pp=alloc(len(prepared)*12);cpu.mem_write(pp,b''.join(struct.pack('<3f',*p)for p in prepared));cpu.mem_write(pubend-0x210,struct.pack('<QQ',pp,len(prepared)));cpu.mem_write(pubend-0x48,struct.pack('<I',2));cpu.mem_write(pubend-0xb,bytes([clutter]));cpu.mem_write(pubend-0x1c,struct.pack('<ff',public_284,public_288));cpu.reg_write(UC_X86_REG_XMM15,bits(.92))
 cpu.mem_write(pubend-0x1c0,struct.pack('<QQ',ap,len(arch)));cpu.mem_write(pubend-0x1a8,struct.pack('<QQ',thp,len(third)))
 bp=stack+0x180000;cpu.mem_write(bp+0x1a8,struct.pack('<Q',sptr+n*104));cpu.mem_write(bp+0x150,struct.pack('<QQ',tp,n));cpu.mem_write(bp+0x128,struct.pack('<Q',sptr));cpu.mem_write(bp+0x2e8,struct.pack('<Q',pubend));cpu.reg_write(UC_X86_REG_RBP,bp);cpu.reg_write(UC_X86_REG_RSP,stack+0x170000);cpu.reg_write(UC_X86_REG_R14,sptr);cpu.reg_write(UC_X86_REG_R13,sptr);cpu.reg_write(UC_X86_REG_R12,pubend);cpu.reg_write(UC_X86_REG_RDI,0);cpu.reg_write(UC_X86_REG_XMM14,0);cpu.reg_write(UC_X86_REG_XMM6,bits(1.));cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 run_to_edge(0x140761288,0x140761770,10000000);assert cpu.reg_read(UC_X86_REG_RIP)==0x140761770,'arch third owner did not finish'
 cpu.reg_write(UC_X86_REG_R13,sptr);cpu.reg_write(UC_X86_REG_R12,pubend);cpu.reg_write(UC_X86_REG_XMM15,bits(.92));run_to_edge(0x140761770,0x1407629b1,30000000);assert cpu.reg_read(UC_X86_REG_RIP)==0x1407629b1,'freehand cache owner did not finish'
 return list(struct.unpack('<'+'f'*len(arch),cpu.mem_read(ap,len(arch)*4))),list(struct.unpack('<'+'f'*len(third),cpu.mem_read(thp,len(third)*4)))

