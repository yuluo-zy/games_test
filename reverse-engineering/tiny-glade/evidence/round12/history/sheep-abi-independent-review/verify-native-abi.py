from pathlib import Path
import json,struct
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
OUT=Path(__file__).resolve().parent;source=json.loads((OUT/'original-byte-evidence.json').read_text(encoding='utf8'));cpu=Uc(UC_ARCH_X86,UC_MODE_64)
cpu.mem_map(0x1416e0000,0x20000)
for va,r in source['functions'].items():cpu.mem_write(int(va,16),bytes.fromhex(r['raw_hex']))
D=0x180000000;S=0x190000000;cpu.mem_map(D,0x10000);cpu.mem_map(S,0x10000);cpu.mem_map(0x140613000,0x1000);cpu.mem_write(0x140613c20,b'\xc3');frees=[]
def hook(c,pc,size,user):
 if pc==0x140613c20:frees.append([c.reg_read(UC_X86_REG_RCX),c.reg_read(UC_X86_REG_RDX),c.reg_read(UC_X86_REG_R8)])
cpu.hook_add(UC_HOOK_CODE,hook)
def put(a,fmt,*args):cpu.mem_write(a,struct.pack(fmt,*args))
def get(a,fmt):return struct.unpack(fmt,bytes(cpu.mem_read(a,struct.calcsize(fmt))))
env=D;control=D+0x100;frame=S+0x1000;rows=[]
for label,a,b,target in [('Goal',0x1416ee62f,0x1416ee636,1),('Teleport',0x1416ee6c2,0x1416ee6c9,2),('Consumed',0x1416f2179,0x1416f2183,0)]:
 for before in [0,1,2,255]:
  initial=bytes([0xa5])*0x48+bytes([before])+bytes([0x5a])*7;cpu.mem_write(control,initial);put(env+0x10,'<Q',control+0x48);put(frame+0x2f0,'<Q',control+0x48)
  cpu.reg_write(UC_X86_REG_RDI,env);cpu.reg_write(UC_X86_REG_RBP,frame);cpu.emu_start(a,b,count=20)
  result=bytes(cpu.mem_read(control,0x50));assert result[:0x48]==initial[:0x48] and result[0x49:]==initial[0x49:] and result[0x48]==target
  rows.append({'kind':'actual native picking write','label':label,'range':[hex(a),hex(b)],'before':before,'after':target})
for before in [0,1,2,3,2**64-2,2**64-1]:
 put(control+0x40,'<Q',before);cpu.reg_write(UC_X86_REG_RBX,control+0x40);cpu.emu_start(0x1416ee281,0x1416ee2a8,count=30)
 after=get(control+0x40,'<Q')[0];assert after==((before+1)&(2**64-1))%3
 rows.append({'kind':'actual native selected SheepId cycle','range':['0x1416ee281','0x1416ee2a8'],'before':before,'after':after})
for oldcap in [0,1,3,2**63]:
 for newlen in [0,1,3]:
  cpu.mem_write(control,bytes([0xa5])*0x50);put(env+8,'<Q',control+0x18);put(control+0x18,'<QQQ',oldcap,D+0x400,oldcap if oldcap<4 else 0);put(frame+0x150,'<QQQ',newlen,D+0x800,newlen)
  cpu.reg_write(UC_X86_REG_RDI,env);cpu.reg_write(UC_X86_REG_RBP,frame);cpu.reg_write(UC_X86_REG_RSP,S+0xf000);frees.clear();cpu.emu_start(0x1416eeff2,0x1416ef02f,count=80)
  assert bytes(cpu.mem_read(control,0x18))==bytes([0xa5])*0x18
  assert get(control+0x18,'<QQQ')==(newlen,D+0x800,newlen)
  expected=[[D+0x400,oldcap*40,4]] if oldcap in [1,3] else [];assert frees==expected
  rows.append({'kind':'actual native manual snapshot replace','range':['0x1416eeff2','0x1416ef02f'],'old_capacity':oldcap,'new_length':newlen,'frees':list(frees),'primary_snapshot_unchanged':True})
(OUT/'native-abi-fixtures.json').write_text(json.dumps({'original_sha256':source['original_sha256'],'scope':'bounded original instruction slices; no UI/Bevy whole function, no actual Rust implementation equivalence claim','cases':rows},indent=2),encoding='utf8')
print(json.dumps({'cases':len(rows),'actual_original_picking_writes':12,'actual_original_selected_id_cycle':6,'actual_original_manual_snapshot_replacements':12}))
