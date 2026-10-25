"""B resolved event consumer: original writes, three notifications, pivot+height."""
import json,random,struct,subprocess
from verify_parameters import cpu,DATA,STACK,ROOT,baseline
from verify_building_state import reset
from verification_provenance import candidate_provenance
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
STOP=0x141a3a8e1
cpu.hook_add(UC_HOOK_CODE,lambda c,pc,size,user:c.emu_stop() if pc==STOP else None)
def original(roof,wall,entity,direction):
 reset();sp=STACK+0x8000;cpu.mem_write(sp,bytes(0x400));cpu.mem_write(DATA,roof)
 event=DATA+0x100;rt=DATA+0x200;gt,wt,st=[DATA+o for o in [0x220,0x224,0x228]]
 cpu.mem_write(event,struct.pack('<QQB',0,wall,direction));cpu.mem_write(rt,bytes(48))
 g,w,s=[DATA+o for o in [0x1000,0x2000,0x3000]]
 for q in [g,w,s]:cpu.mem_write(q,bytes(0x500));cpu.mem_write(q+0x20,struct.pack('<QQQQQ',16,q+0x100,0,0,0))
 for offset,value in [(0x38,g),(0x98,gt),(0x90,wt),(0x88,s),(0x80,st)]:cpu.mem_write(sp+offset,struct.pack('<Q',value))
 for offset in [0x34,0x30,0x2c,0x28]:cpu.mem_write(sp+offset,struct.pack('<I',0x12345678))
 for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RBP,DATA),(UC_X86_REG_R14,0),(UC_X86_REG_RCX,0),(UC_X86_REG_RDX,rt),(UC_X86_REG_R12,event),(UC_X86_REG_R13,entity),(UC_X86_REG_RSI,w)]:cpu.reg_write(reg,val)
 cpu.emu_start(0x141a3aa2b,0,count=3000);assert cpu.reg_read(UC_X86_REG_RIP)==STOP
 for q,t in [(g,gt),(w,wt),(s,st)]:
  assert struct.unpack('<Q',cpu.mem_read(q+0x30,8))[0]==1
  assert struct.unpack('<Q',cpu.mem_read(q+0x40,8))[0]==1
  assert struct.unpack('<I',cpu.mem_read(t,4))[0]==0x12345678
 assert struct.unpack('<QQ',cpu.mem_read(g+0x108,16))==(entity,wall)
 assert struct.unpack('<Q',cpu.mem_read(w+0x108,8))[0]==wall
 kind=cpu.mem_read(g+0x118,1)[0];sound=cpu.mem_read(s+0x108,1)[0]
 pivot=struct.unpack('<III',cpu.mem_read(s+0x118,12))
 tick=struct.unpack('<I',cpu.mem_read(rt,4))[0]
 return f'{bytes(cpu.mem_read(DATA,88)).hex()} {tick:08x} {kind} {sound} {entity:016x} {wall:016x} '+','.join(f'{v:08x}' for v in pivot)
def main():
 rng=random.Random(20261012);inputs=[];expected=[]
 for i in range(1024):
  roof=bytearray(rng.randbytes(88));shape=i%2;wall=rng.getrandbits(64);entity=0x100000029;direction=(i//2)%2
  struct.pack_into('<Q',roof,0,wall);struct.pack_into('<I',roof,8,shape)
  struct.pack_into('<6f',roof,12,1,0,rng.uniform(-30,30),rng.uniform(-30,30),rng.uniform(1,14),rng.uniform(1,14))
  struct.pack_into('<6f',roof,0x24,0,0,rng.uniform(0,1),[0,.1,.5,1][i%4],rng.randrange(2),rng.uniform(0,1))
  struct.pack_into('<f',roof,0x4c,rng.uniform(-2,15));roof[0x3c]=direction if i%3==0 else 1-direction
  inputs.append(f'direction {roof.hex()} {wall:016x} {entity:016x} {direction}');expected.append(original(bytes(roof),wall,entity,direction))
 actual=subprocess.run([str(ROOT/'reconstruction/target/release/lifecycle-probe')],input='\n'.join(inputs)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(actual)==len(expected)
 failures=[{'case':i,'input':inputs[i],'original':e,'rust':a} for i,(e,a) in enumerate(zip(expected,actual)) if e!=a]
 report={'ok':not failures,'cases':len(inputs),'failures':failures[:10],'failure_count':len(failures),'binary_sha256':baseline['binary']['sha256'],'original_range':['0x141a3aa2b','0x141a3a8e1'],'scope':'Entire resolved direction write/Changed tick, geometry(kind1), gable wall-ID and sound(kind7) pivot+Roof::height; both shapes/directions, unchanged-byte writes and finite valid parameters. Original private pivot/height helpers executed.','bridges':['Preallocated standard event Vec buffers; lookup/query successful resolution supplied at entry'],'excluded':['Original PublicWalls/ECS lookup execution, invalid shapes/enum/NaN parameters, writer allocation failure, whole fork scheduling, A/C execution'],**candidate_provenance()}
 (ROOT/'evidence/building-edits/B-direction-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not failures,'cases':len(inputs),'failures':len(failures)}));assert not failures
if __name__=='__main__':main()
