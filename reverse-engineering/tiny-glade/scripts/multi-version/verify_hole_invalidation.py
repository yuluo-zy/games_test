"""Original publisher's RecalculateWallHoles reader; only std lookup bridged."""
import json,random,struct,subprocess
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
from verify_parameters import cpu,DATA,STACK,ROOT,baseline
from verify_building_state import reset
from verification_provenance import candidate_provenance
present=False
STOP=0x140a8bb7f
def hook(c,pc,size,user):
 if pc==0x140a8bbcf:c.reg_write(UC_X86_REG_RIP,0x140a8bc90 if present else STOP)
 if pc==STOP:c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,hook)
def original(wall,exists):
 global present
 present=exists;reset();sp=STACK+0x8000;frame=sp+0x80;event=DATA;writer=DATA+0x1000;tick=DATA+0x2000;reader=DATA+0x2100
 cpu.mem_write(event,struct.pack('<QQ',0,wall));cpu.mem_write(writer,bytes(0x300));cpu.mem_write(writer+0x20,struct.pack('<QQQQQ',16,writer+0x100,0,0,0));cpu.mem_write(tick,bytes(4));cpu.mem_write(reader,bytes(8));cpu.mem_write(sp,bytes(0x1100));cpu.mem_write(frame+0xce0,struct.pack('<I',0x12345678));cpu.mem_write(frame+0xce8,struct.pack('<Q',tick))
 for reg,val in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RBP,frame),(UC_X86_REG_RBX,event),(UC_X86_REG_R14,writer),(UC_X86_REG_R13,reader)]:cpu.reg_write(reg,val)
 cpu.emu_start(0x140a8bbc0,0,count=500);assert cpu.reg_read(UC_X86_REG_RIP)==STOP
 assert struct.unpack('<Q',cpu.mem_read(reader,8))[0]==1
 count=struct.unpack('<Q',cpu.mem_read(writer+0x30,8))[0];assert count==int(exists)
 assert struct.unpack('<I',cpu.mem_read(tick,4))[0]==(0x12345678 if exists else 0)
 return f'{struct.unpack("<Q",cpu.mem_read(writer+0x108,8))[0]:016x}' if count else '-'
def main():
 rng=random.Random(20261011);rows=[(rng.getrandbits(64),bool(i%2)) for i in range(512)];expected=[original(*r) for r in rows];inputs=[f'{w:016x} {int(p)}' for w,p in rows]
 actual=subprocess.run([str(ROOT/'reconstruction/target/release/hole-invalidation-probe')],input='\n'.join(inputs)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(actual)==len(expected)
 fail=[{'case':i,'expected':e,'actual':a}for i,(e,a) in enumerate(zip(expected,actual))if e!=a]
 report={'ok':not fail,'cases':len(rows),'failures':fail,'binary_sha256':baseline['binary']['sha256'],'scope':'Actual RecalculateWallHoles reader forwards OnWallChanged only when InnerWalls key exists; missing skip, reader consumed and event/tick writes executed. This is one link, not opening geometry acceptance.','range':['0x140a8bbc0','0x140a8bb7f'],'bridges':['InnerWalls std map contains lookup resolved at140a8bbcf..140a8bc90; event Vec preallocated'],**candidate_provenance()}
 (ROOT/'evidence/building-holes/B-invalidation-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not fail,'cases':len(rows)}));assert not fail
if __name__=='__main__':main()
