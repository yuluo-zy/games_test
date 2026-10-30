"""原Mode10的Widget参数块与hover高亮门；没有算法/事件结果桥接。"""
from pathlib import Path
import hashlib,json,struct,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D=0x20000000;u.mem_map(D,0x10000);sp=D+0x8000;state=D+0x1000
rows=[]
for volume in [4,7]:
 for camera in [0,1]:
  u.mem_write(sp,bytes([0xcc])*0x300);u.mem_write(sp+0x70,bytes([1-camera]))
  u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_RAX,volume)
  u.emu_start(0x140aef812,0x140aef8a1,count=200)
  block=bytes(u.mem_read(sp+0xf0,0xb0))
  active=b''.join(block[o:o+n] for o,n in [(0x18,4),(0x30,4),(0x48,4),(0x60,4),(0x78,2),(0x88,2),(0x8c,8),(0xa8,2)])
  rows.append(f'P {volume} {camera} {active.hex()}')
outcome=[]
def stop(_,address,size,data):
 if address in [0x140aef8f1,0x140aef980,0x140aef9a0]:
  outcome.append(int(address==0x140aef8f1));u.emu_stop()
hook=u.hook_add(UC_HOOK_CODE,stop)
for tag in range(4):
 for phase in range(6):
  for matched in [0,1]:
   for hovered in [-1,0]:
    for camera in [0,1]:
     u.mem_write(state,bytes(0x80));u.mem_write(state+0x18,bytes([tag,phase]))
     u.mem_write(state+0x38,struct.pack('<Q',42 if matched else 99))
     u.mem_write(sp+0x2b,bytes([camera]));u.mem_write(sp+0x2c,struct.pack('<i',hovered))
     u.reg_write(UC_X86_REG_RAX,state);u.reg_write(UC_X86_REG_R14,42);u.reg_write(UC_X86_REG_R15,state)
     before=len(outcome);u.emu_start(0x140aef8bc,0x140aef9a1,count=100)
     assert len(outcome)==before+1
     rows.append(f'H {tag} {phase} {matched} {hovered} {camera} {outcome[-1]}')
u.hook_del(hook)
def retain(path,content):
 if path.exists():assert path.read_text()==content, f'原证已存在且内容不一致：{path}'
 else:path.write_text(content)
target=Path(__file__).with_suffix('.txt');retain(target,'\n'.join(rows)+'\n')
metadata=Path(__file__).with_suffix('.json')
retain(metadata,json.dumps({'binary_sha256':sha,'parameter_cases':4,'hover_gate_cases':192,
 'ranges':['140aef812..140aef8a1 original active WidgetParams stores','140aef8bc..140aef8f1/980/9a0 original hover gate'],
 'bridges':[],'fixture_sha256':hashlib.sha256(target.read_bytes()).hexdigest(),
 'inactive_bytes':'four sound inactive payloads, preview inactive padding excluded',
 'boundary':'bounded original parameter stores + highlight branch; no complete Mode10/main or device input claim'},indent=2)+'\n')
print(len(rows))
