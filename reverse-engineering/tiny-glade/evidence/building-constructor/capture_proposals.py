"""Actual original owner: initial proposal -> hole/seed/horizontal split outputs.
Raw production/noise outside this captured stage remains explicitly unresolved.
"""
from native_fixture import *
groups=[];current=None
def q(rb):
 cap,p,head,n=struct.unpack('<4Q',cpu.mem_read(rb+0x428,32));return [bytes(cpu.mem_read(p+(head+j)%cap*40,38)).hex()for j in range(n)]
def capture_proposal(uc,pc,size,u):
 global current
 if pc not in [0x14076ba71,0x14076bde7,0x14076c386,0x14076c08f]:return
 rb=cpu.reg_read(UC_X86_REG_RBP)
 if pc==0x14076ba71:
  cap,p,n=struct.unpack('<3Q',cpu.mem_read(data,24));current={'brick':q(rb)[0],'holes':[bytes(cpu.mem_read(p+j*16,16)).hex()for j in range(n)],'values':[fl(u64(rb+0x2a0)),fl(u64(rb+0x1a0)),fl(u64(rb+0x24c)),fl(cpu.reg_read(UC_X86_REG_XMM6))],'split_enabled':cpu.mem_read(rb+0x4b7,1)[0],'records':[]}
 elif pc==0x14076bde7 and current is not None:current['expected']=q(rb);groups.append(current);current=None
 elif pc==0x14076c386:
  c={'ends':list(struct.unpack('<2f',cpu.mem_read(rb+0xa0,8)))+list(struct.unpack('<2f',cpu.mem_read(rb+0x1b0,8))),'u':[fl(u64(rb+0x460)),fl(u64(rb+0x4a0))],'scale':[fl(u64(rb+0x2a0)),fl(u64(rb+0x1a0))],'width_padding':fl(u64(rb+0x328)),'depth_axis':list(struct.unpack('<4f',cpu.reg_read(UC_X86_REG_XMM15).to_bytes(16,'little')))[:3],'surface':[fl(cpu.reg_read(UC_X86_REG_XMM12)),fl(cpu.reg_read(UC_X86_REG_XMM10)),fl(cpu.reg_read(UC_X86_REG_XMM4))],'material':struct.unpack('<I',cpu.mem_read(rb+0x22c,4))[0]}
  if 'context' in groups[-1]:assert groups[-1]['context']==c, 'context changes across pieces'
  else:groups[-1]['context']=c
 elif pc==0x14076c08f:
  p=cpu.reg_read(UC_X86_REG_RAX)+cpu.reg_read(UC_X86_REG_RCX);groups[-1]['records'].append(bytes(cpu.mem_read(p,96)).hex())
cpu.hook_add(UC_HOOK_CODE,capture_proposal)
