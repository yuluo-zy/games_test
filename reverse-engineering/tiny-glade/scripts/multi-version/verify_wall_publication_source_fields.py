"""Execute original publisher loads/stores to identify raw-owned publication fields.
This is schema binding evidence, not fullpublisher or candidateRust acceptance.
"""
import sys,json,struct,random
from verification_provenance import ROOT
from verify_parameters import cpu,DATA,STACK,baseline
from unicorn.x86_const import *

def main():
 rng=random.Random(20261020);frame=STACK+0x1000;cpu.mem_write(frame+0xd40,struct.pack('<Q',DATA+0x70));cpu.reg_write(UC_X86_REG_RBP,frame)
 for i in range(512):
  s=rng.randbytes(112);cpu.mem_write(DATA,s)
  cpu.emu_start(0x140a8ce58,0x140a8ce6f,count=5)
  cpu.emu_start(0x140a8d81e,0x140a8d84b,count=8)
  cpu.emu_start(0x140a8d904,0x140a8d928,count=5)
  cpu.emu_start(0x140a8db6c,0x140a8db81,count=3)
  cpu.emu_start(0x140a8db88,0x140a8db97,count=2)
  assert bytes(cpu.mem_read(frame+0x788,4))==s[0x5c:0x60]
  assert bytes(cpu.mem_read(frame+0x794,1))==s[0x64:0x65]
  assert bytes(cpu.mem_read(frame+0x795,1))==s[0x66:0x67]
  assert bytes(cpu.mem_read(frame+0x796,1))==s[0x65:0x66]
  assert bytes(cpu.mem_read(frame+0x790,4))==s[0x60:0x64]
  assert bytes(cpu.mem_read(frame+0x798,1))==s[0x67:0x68]
  assert bytes(cpu.mem_read(frame+0x799,2))==s[0x68:0x6a]
 report={'ok':True,'cases':512,'scope':'Original publisher instruction source-load/scratch-publication-copy binding only; no Rust candidate/fullpublisher claim','binary_sha256':baseline['binary']['sha256'],
  'source_base':'RBP+d40 holds RawInnerWall end =source+70','publication_base':'RBP+500 scratch PublicWallState',
  'bindings':[{'raw_offset':'0x64','public_offset':'0x294','name':'observed arch-global selection flag, original name/default not asserted','range_load':['0x140a8d81e','0x140a8d84b'],'range_store':['0x140a8d904','0x140a8d928']},{'raw_offset':'0x65','public_offset':'0x296','name':'observed special flag','range_load':['0x140a8ce58','0x140a8ce6f'],'range_store':['0x140a8db74','0x140a8db81']},{'raw_offset':'0x66','public_offset':'0x295','name':'ClutterVersion observed byte','range_load':['0x140a8d81e','0x140a8d84b'],'range_store':['0x140a8d904','0x140a8d928']},{'raw_offset':'0x5c','public_offset':'0x288','name':'observed path scalar, original type/name not asserted','range_load':['0x140a8d81e','0x140a8d84b'],'range_store':['0x140a8d904','0x140a8d928']}],
  'additional_bindings':[{'raw_offset':'0x60','public_offset':'0x290','load':'0x140a8d831','store':'0x140a8db88','semantic_name':'unknown scalar bits'}, {'raw_offset':'0x67','public_offset':'0x298','load':'0x140a8d841','store':'0x140a8db90','semantic_name':'unknown byte'}, {'raw_offset':'0x68','public_offset':'0x299','load':'0x140a8d846','store':'0x140a8db6c','semantic_name':'style'}, {'raw_offset':'0x69','public_offset':'0x29a','load':'0x140a8d846','store':'0x140a8db6c','semantic_name':'raw metadata; new_wall depends on DEV_OVERRIDE_IS_DEMO_MODE'}],
  'excluded':['Creation defaults','Public map insertion/storage copy','Original source field semantic naming','A/C source/public offsets','Full candidatepublication chain']}
 (ROOT/'evidence/building-production-authority/B-publication-source-fields.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':True,'cases':512,'bindings':8}))
if __name__=='__main__':main()
