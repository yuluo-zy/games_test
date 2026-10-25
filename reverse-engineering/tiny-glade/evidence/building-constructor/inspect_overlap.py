from native_fixture import *
def resolve(hole,bricks):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x10000));cpu.mem_write(stack,bytes(0x200000));cpu.mem_write(data,struct.pack('<4f',*hole));cpu.mem_write(data+0x100,struct.pack('<QQQQ',512,data+0x1000,0,len(bricks)))
 for i,x in enumerate(bricks):cpu.mem_write(data+0x1000+i*40,struct.pack('<8fIBB',*x[0],*x[1],x[2],x[3],x[4]))
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_RDX,data+0x100);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.emu_start(0x1412411c0,stop,count=1000000)
 cap,p,head,n=struct.unpack('<4Q',cpu.mem_read(data+0x100,32));out=[]
 for j in range(n):out.append(list(struct.unpack('<8fIBB',cpu.mem_read(p+(head+j)%cap*40,38))))
 return out
if __name__=='__main__':
 for hole in [[-.2,-.3,.4,.5],[.6,-.3,1.2,.5],[-.2,.5,.4,1.3],[.6,.5,1.2,1.3],[-1,-1,2,.5],[-1,.5,2,2],[-1,-1,.5,2],[.5,-1,2,2],[.25,.25,.75,.75],[-1,-1,2,2],[2,2,3,3]]:
  print(hole,resolve(hole,[([0,0,1,1],[.1,.2,.3,.4],17,0,0)]))
def rows(heights,scale,seed):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x10000));cpu.mem_write(stack,bytes(0x200000));cpu.mem_write(data+0x1000,struct.pack('<'+'f'*len(heights),*heights));sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.mem_write(sp+0x28,struct.pack('<Q',seed));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_RDX,data+0x1000);cpu.reg_write(UC_X86_REG_R8,len(heights));cpu.reg_write(UC_X86_REG_XMM3,bits(scale));cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.emu_start(0x14123e110,stop,count=1000000);cap,p,n=struct.unpack('<3Q',cpu.mem_read(data,24));return bytes(cpu.mem_read(p,n*4))
def trim(boundaries,us,heights,scale,length,special):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x10000));cpu.mem_write(stack,bytes(0x200000));cpu.mem_write(data+0x1000,struct.pack('<'+'f'*len(boundaries),*boundaries));cpu.mem_write(data+0x2000,struct.pack('<'+'f'*len(us),*us));cpu.mem_write(data+0x3000,struct.pack('<'+'f'*len(heights),*heights));cpu.mem_write(data+0x100,struct.pack('<QQQQQQf',len(us),data+0x4000,len(us),len(us),data+0x2000,len(us),length));sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.mem_write(sp+0x28,struct.pack('<Q',data+0x3000));cpu.mem_write(sp+0x30,struct.pack('<Q',len(heights)));cpu.mem_write(sp+0x38,struct.pack('<f',scale));cpu.mem_write(sp+0x40,struct.pack('<f',length));cpu.mem_write(sp+0x48,bytes([special]));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_RDX,data+0x1000);cpu.reg_write(UC_X86_REG_R8,len(boundaries));cpu.reg_write(UC_X86_REG_R9,data+0x100);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.emu_start(0x14123eef0,stop,count=1000000);cap,p,n=struct.unpack('<3Q',cpu.mem_read(data,24));return bytes(cpu.mem_read(p,n*32))

def normalize(holes,length,height,closed):
 def halt(uc,pc,size,u):
  if pc==0x140769e2a:cpu.emu_stop()
 h=cpu.hook_add(UC_HOOK_CODE,halt)
 try:call(length=length,height=height,holes=holes,closed=closed)
 finally:cpu.hook_del(h)
 cap,p,n=struct.unpack('<3Q',cpu.mem_read(data,24));return bytes(cpu.mem_read(p,n*16))
def row_holes(row,holes,length,height):
 """Execute actual original from_curve collector block and both geometry callees.
 All hole refs/suffix storage preallocated; no game branch is substituted.
 """
 allocation[0]=heap;cpu.mem_write(data,bytes(0x10000));cpu.mem_write(stack,bytes(0x200000))
 cpu.mem_write(data+0x1000,b''.join(struct.pack('<4f',*h)for h in holes));cpu.mem_write(data+0x500,struct.pack('<QQ4f',0,0,row[1],row[3],row[0],row[2]));sp=stack+0x100008;rb=stack+0x100600;cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RBP,rb);cpu.reg_write(UC_X86_REG_R13,data+0x500);cpu.reg_write(UC_X86_REG_RSI,data+0x1000+len(holes)*16);cpu.reg_write(UC_X86_REG_RAX,0);cpu.reg_write(UC_X86_REG_XMM8,bits(f32(.46)/length));cpu.reg_write(UC_X86_REG_XMM9,bits(f32(.46)/height));cpu.reg_write(UC_X86_REG_XMM15,0);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 for o,v in {0x160:data+0x1000,0x340:len(holes)+1,0x348:data+0x6000,0x2b0:len(holes)+1,0x2b8:data+0x7000,0x2c0:0}.items():cpu.mem_write(rb+o,struct.pack('<Q',v))
 wf(rb+0x478,f32(f32(.46)/length)*.5)
 cpu.emu_start(0x14076a293,0x14076a498,count=1000000);n=u64(rb+0x350);refs=[(u64(data+0x6000+i*8)-data-0x1000)//16 for i in range(n)];mins=bytes(cpu.mem_read(data+0x7000,n*4));return refs,mins

def f32(v):return struct.unpack('<f',struct.pack('<f',v))[0]
