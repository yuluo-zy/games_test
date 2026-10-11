"""Whole original indexed mesh -> actual Parry vtable local ray query research."""
from research_native_indexed_mesh import *
def query(points,triangles,origin,direction,max_toi=100.,solid=True):
 owner=full(points,triangles);arc,vtable=struct.unpack_from('<2Q',owner,0x20);entry=u64(vtable+24);ray=data+0x9000;cpu.mem_write(ray,struct.pack('<6f',*origin,*direction));sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop))
 for r,v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,arc+16),(UC_X86_REG_RDX,ray),(UC_X86_REG_XMM2,bits(max_toi)),(UC_X86_REG_R9,int(solid))]:cpu.reg_write(r,v)
 cpu.emu_start(entry,stop,count=5000000)
 return bool(cpu.reg_read(UC_X86_REG_RAX)&255),cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff,entry
if __name__=='__main__':
 try:print(query([[0.,0.,0.],[1.,0.,0.],[0.,0.,1.]],[[0,1,2]],[.2,10.,.2],[0.,-1.,0.]))
 except Exception as e:print(type(e).__name__,str(e),'rip',hex(cpu.reg_read(UC_X86_REG_RIP)),last[-12:])
