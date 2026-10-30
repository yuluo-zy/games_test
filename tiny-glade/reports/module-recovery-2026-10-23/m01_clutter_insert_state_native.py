"""只验证 B generate_decorator_clutter 的真实 insert 调用块，不代替完整生成算法。"""
from pathlib import Path
import hashlib,struct,json,itertools,subprocess
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D,S=0x20000000,0x30000000;u.mem_map(D,0x100000);u.mem_map(S,0x100000)
q=lambda a,v:u.mem_write(a,struct.pack('<Q',v))
uq=lambda a:struct.unpack('<Q',u.mem_read(a,8))[0]
def hook(uc,a,size,_):
 global capture
 if a==0x1408258b0:
  sp=uc.reg_read(UC_X86_REG_RSP)
  capture={'blueprints':uc.reg_read(UC_X86_REG_RCX),'id':uc.reg_read(UC_X86_REG_RDX),'addr_words':[uc.reg_read(UC_X86_REG_R8),uc.reg_read(UC_X86_REG_R9)],'state':u.mem_read(sp+0x28,1)[0]};uc.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
rows=[]
for poisoned_stack_state,identifier,addr in itertools.product([0,1,2,255],[0,91,0xffffffff],[(0,0),(1,0xfedcba9876543210),(2,7)]):
 u.mem_write(D,bytes(0x10000));u.mem_write(S,bytes(0x100000));bp=S+0xe0000;sp=S+0xf0000
 q(bp+0x140,D+0x1000);q(bp+0x148,D+0x2000);q(bp+0x1a0,addr[0]);q(bp+0x1a8,addr[1]);u.mem_write(bp+0x28c,struct.pack('<I',0x12345678));u.mem_write(sp+0x20,bytes([poisoned_stack_state]))
 for reg,value in [(UC_X86_REG_RBP,bp),(UC_X86_REG_RSP,sp),(UC_X86_REG_RDI,identifier)]:u.reg_write(reg,value)
 capture=None;u.emu_start(0x1407e3b94,0x1408258b0+1,count=1000)
 assert capture=={'blueprints':D+0x1000,'id':identifier,'addr_words':list(addr),'state':0}
 assert struct.unpack('<I',u.mem_read(D+0x2000,4))[0]==0x12345678
 rows.append({'poisoned_stack_byte':poisoned_stack_state,'id':identifier,'addr_words':list(addr),'call':capture})
report={'binary_sha256':sha,'function':'0x1407e3370','entry':'0x1407e3b94','state_writer':'0x1407e3bb1','insert_call':'0x1407e3bbf','callee':'0x1408258b0','scope':'actual call block after original filters; no full producer/native container execution','rows':rows}
Path(__file__).with_name('m01-clutter-insert-state-native.json').write_text(json.dumps(report,indent=2)+'\n')
Path(__file__).with_name('m01-clutter-insert-state-call.asm').write_text(subprocess.check_output(['/opt/homebrew/opt/llvm/bin/llvm-objdump','-d','--start-address=0x1407e3b84','--stop-address=0x1407e3be8',str(root/'B版本内容/tiny-glade.rne')],text=True))
print(json.dumps({'call_block_cases':len(rows),'all_actual_state':0}))
