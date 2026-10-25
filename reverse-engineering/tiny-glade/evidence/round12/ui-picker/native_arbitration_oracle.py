import native_picker_oracle as n
from native_picker_oracle import cpu,DATA,STACK,HEAP,STOP,put,u64,ROOT
import struct,json
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
END=0x140838eec
cpu.hook_add(UC_HOOK_CODE,lambda c,pc,size,user:c.emu_stop()if pc==END else None)
def run(ordinary,canvases,origin=(0,0,0),direction=(0,0,1),view=None,depth=0):
 n.heap=HEAP;n.trace.clear();cpu.mem_write(DATA,bytes(0x10000));cpu.mem_write(STACK,bytes(0x200000))
 vec,items,canvas,viewref,camera,depthref,value=[DATA+i*0x1000 for i in range(7)]
 put(vec,'<3Q',128,items,len(ordinary));cpu.mem_write(items,b''.join(bytes.fromhex(h)for h in ordinary))
 for i,(layer,h)in enumerate(canvases):
  lp=DATA+0x8000+i*8;cpu.mem_write(lp,bytes([layer]));put(canvas+i*64,'<Q',lp);cpu.mem_write(canvas+i*64+8,bytes.fromhex(h))
 put(viewref,'<Q',camera);put(depthref,'<Q',value);put(value+4,'<f',depth)
 if view is None:view=[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1]
 put(camera+0x80,'<16f',*view)
 sp=STACK+0x100000;bp=sp+0x80;put(bp-0x30,'<6f',*origin,*direction);put(bp+0x20,'<Q',viewref);put(bp+0x140,'<Q',depthref);put(bp+0x68,'<Q',canvas);put(bp+0x60,'<Q',0)
 for r in (UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15):cpu.reg_write(r,0)
 for r,v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RBP,bp),(UC_X86_REG_RBX,vec),(UC_X86_REG_RSI,depthref),(UC_X86_REG_R14,len(canvases)),(UC_X86_REG_R15,DATA+0x9000),(UC_X86_REG_MXCSR,0x1f80),(UC_X86_REG_RFLAGS,0x202)]:cpu.reg_write(r,v)
 try:cpu.emu_start(0x140838bfd,END,count=1000000)
 except Exception as e:raise RuntimeError(f'{e} @ {cpu.reg_read(UC_X86_REG_RIP):x}')
 assert cpu.reg_read(UC_X86_REG_RIP)==END,hex(cpu.reg_read(UC_X86_REG_RIP))
 # Final length still in RDX (odd tail) or RDX/RCX? Original stores RSI? At END
 # outer writes rdx? inspect exact instruction rather than infer container len.
 length=cpu.reg_read(UC_X86_REG_RDX)
 # END instruction is mov [rbx+10],rdx per native listing below.
 return [bytes(cpu.mem_read(items+i*56,56)).hex()for i in range(length)]
def hit(id,d,occlude=True,flags=1):
 b=bytearray(56);struct.pack_into('<I3fQQ4fBB',b,0,1,0,1,0,id,0,d,5,6,7,occlude,flags);return b.hex()
if __name__=='__main__':
 cases=[]
 for depth in [-2,-1,0,1,2,3]:
  for occlude in [False,True]:
   a=[hit(1,3,occlude),hit(2,1,occlude),hit(3,2,occlude)];kw=dict(depth=depth,direction=(0,0,-1));res=run(a,[],**kw);cases.append(dict(name=f'depth{depth}-occ{occlude}',ordinary=a,canvases=[],inputs=kw,result=res))
 for flags in [1,5]:
  a=[hit(1,-3,False,flags),hit(2,1,False),hit(3,0,False),hit(4,-0.,False)];res=run(a,[]);cases.append(dict(name=f'negative{flags}',ordinary=a,canvases=[],inputs={},result=res))
 a=[hit(1,2,False),hit(2,2,False),hit(3,1,False)];c=[(4,hit(4,3.4028234663852886e38,False)),(1,hit(5,3.4028234663852886e38,False)),(4,hit(6,3.4028234663852886e38,False))]
 cases.append(dict(name='stable-layers-and-distance',ordinary=a,canvases=c,inputs={},result=run(a,c)))
 for origin in [(1,2,3),(-3,-2,-1)]:
  for depth in [-10,0,10]:
   view=[1,0,.25,0,0,1,-.5,0,0,0,2,0,0,0,-7,1]
   a=[hit(1,3,True),hit(2,1,True),hit(3,2,True),hit(4,4,False)];kw=dict(origin=origin,direction=(.2,-.3,-.4),view=view,depth=depth)
   cases.append(dict(name=f'view-{origin}-{depth}',ordinary=a,canvases=[],inputs=kw,result=run(a,[],**kw)))
 (ROOT/'native-arbitration-fixtures.json').write_text(json.dumps(cases,indent=2));print('arbitration original cases',len(cases),[[struct.unpack_from('<Q',bytes.fromhex(x),16)[0]for x in c['result']]for c in cases])
