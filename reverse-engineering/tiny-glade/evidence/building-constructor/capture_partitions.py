"""Capture actual B row horizontal partition owner from full original calls.
Includes original seed mixing and private random split; no game RNG bridge.
"""
from native_fixture import *
partitions=[];source_seed=None
def capture_partition(uc,pc,size,u):
 global source_seed
 if pc==0x140769760:
  source_seed=u64(cpu.reg_read(UC_X86_REG_RSP)+0x58)
 if pc==0x14076acb6:
  rb=cpu.reg_read(UC_X86_REG_RBP);p=u64(rb+0x310);n=u64(rb+0x318);rowptr=cpu.reg_read(UC_X86_REG_R13);y0,y1,u0,u1=struct.unpack('<4f',cpu.mem_read(rowptr+16,16));index=u64(rb+0x460);entry_sp=rb+0x5a8
  # Vertical boundary count remains in original owner local+108? saved+100 cap; split vec cap+c8/ptr+d0/len+d8.
  boundaries=u64(rb+0xd8)
  partitions.append({'source_seed':source_seed,'boundary_count':boundaries,'row_index':index,'row_u':[u0,u1],'length':fl(u64(rb+0x2a0)),'expected':bytes(cpu.mem_read(p,n*4)).hex()})
cpu.hook_add(UC_HOOK_CODE,capture_partition)
