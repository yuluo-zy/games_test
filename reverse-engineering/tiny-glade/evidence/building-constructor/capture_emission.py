"""Full original B from_curve invocation, capture real resolved row emission inputs.
Stage evidence only: final scalar shader/flags are observed, not recovered owners.
"""
from native_fixture import *
contexts=[];pending={}
def capture(uc,pc,size,u):
 rb=cpu.reg_read(UC_X86_REG_RBP)
 if pc==0x14076c386:
  p=cpu.reg_read(UC_X86_REG_RDI);brick=bytes(cpu.mem_read(p,38)).hex()
  pending.clear();pending.update({'brick':brick,'ends':list(struct.unpack('<2f',cpu.mem_read(rb+0xa0,8)))+list(struct.unpack('<2f',cpu.mem_read(rb+0x1b0,8))),'u':[fl(u64(rb+0x460)),fl(u64(rb+0x4a0))],'scale':[fl(u64(rb+0x2a0)),fl(u64(rb+0x1a0))],'width_padding':fl(u64(rb+0x328)),'depth_axis':list(struct.unpack('<4f',cpu.reg_read(UC_X86_REG_XMM15).to_bytes(16,'little')))[:3],'surface':[fl(cpu.reg_read(UC_X86_REG_XMM12)),fl(cpu.reg_read(UC_X86_REG_XMM10)),fl(cpu.reg_read(UC_X86_REG_XMM4))],'material':struct.unpack('<I',cpu.mem_read(rb+0x22c,4))[0],'flags':cpu.reg_read(UC_X86_REG_R12)&0xffffffff})
 if pc==0x14076c08f:
  p=cpu.reg_read(UC_X86_REG_RAX)+cpu.reg_read(UC_X86_REG_RCX);contexts.append({**pending,'output':bytes(cpu.mem_read(p,96)).hex()})
cpu.hook_add(UC_HOOK_CODE,capture)
if __name__=='__main__':
 call();(ROOT/'evidence/building-constructor/B-emission-cases.json').write_text(json.dumps(contexts,indent=2)+'\n');print('capture',len(contexts));print(contexts[:1])
