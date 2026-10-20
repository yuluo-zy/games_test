from pathlib import Path
import ctypes,struct,json,math
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE,UC_HOOK_MEM_UNMAPPED
from unicorn.x86_const import *
here=Path(__file__).parent
exec((here.parent/'extract_startup_followup.py').read_text().split('addresses=')[0])
crt=ctypes.CDLL('ucrtbase');floor=crt.floorf;floor.argtypes=[ctypes.c_float];floor.restype=ctypes.c_float
bits=lambda x:struct.unpack('<I',struct.pack('<f',x))[0]
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(0x60000000,0x100000);u.mem_map(0x70000000,0x10000);u.mem_map(0x50000000,0x1000)
owner=0x60000000;headers=0x60010000;rows=0x60060000
u.mem_write(owner,struct.pack('<QQQ',4096,headers,4096));u.mem_write(owner+0x48,struct.pack('<IIff',64,64,130,130))
for i in range(4096):u.mem_write(headers+i*48,struct.pack('<QQQ',1,rows+i*64,1)+bytes(24))
def mem(uc,access,addr,size,value,user):
 page=addr&~4095
 if 0x140000000<=page<0x144000000:
  try:b=read(page,4096)
  except ValueError:return False
  uc.mem_map(page,4096);uc.mem_write(page,b);return True
 return False
def code(uc,addr,size,user):
 if addr==0x50000000:uc.emu_stop()
 elif addr==0x142923ee0:
  x=struct.unpack('<f',struct.pack('<I',uc.reg_read(UC_X86_REG_XMM0)&0xffffffff))[0];uc.reg_write(UC_X86_REG_XMM0,bits(floor(x)));sp=uc.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',uc.mem_read(sp,8))[0];uc.reg_write(UC_X86_REG_RSP,sp+8);uc.reg_write(UC_X86_REG_RIP,ret)
u.hook_add(UC_HOOK_MEM_UNMAPPED,mem);u.hook_add(UC_HOOK_CODE,code)
def query(x,z):
 sp=0x70008008;u.mem_write(sp,struct.pack('<Q',0x50000000));u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_RCX,owner);u.reg_write(UC_X86_REG_XMM1,bits(x));u.reg_write(UC_X86_REG_XMM2,bits(z));u.emu_start(0x140aa5730,0,count=5000)
 return None if u.reg_read(UC_X86_REG_RDX)==0 else (u.reg_read(UC_X86_REG_RAX)-rows)//64
values=[-math.inf,-1e20,-65.00001,-65,-64.99999,-62.96875,-1.015625,-.000001,0,1.015625,62.96875,64.99999,65,65.00001,1e20,math.inf,math.nan]
cases=[{'point_bits':[bits(x),bits(z)],'cell':query(x,z)} for x in values for z in values]
(here/'native-point-query.json').write_text(json.dumps({'original_sha256':sha,'scope':'Full original query140aa5730/273B, real native bucket headers, std floorf delegated to ucrtbase; no game query branch mocked','cases':cases},indent=2),encoding='utf-8')
source='''use country_core::{resources::walls::WallId,systems::wall::wall_state::wall_spatial_hash::{WallSpatialHash,WallSpatialSegment}};
#[test]fn full_original_point_query_boundaries_and_no_wall_dedup() {
 let mut hash=WallSpatialHash::default();
 for i in 0..4096usize {
  let p=[-65.0+(i%64) as f32*(130.0/64.0)+1.015625,-65.0+(i/64) as f32*(130.0/64.0)+1.015625];
  hash.add_segment(WallSpatialSegment {endpoints:[p,p],source_u:[0.0,1.0],relative_heights:[0.0;2],terrain_heights:[0.0;2],wall:WallId(i as u64)});
 }
 let cases:&[([u32;2],Option<u64>)]=&[
'''
for c in cases:source+=' ([%d,%d],%s),\n'%(*c['point_bits'],'None' if c['cell'] is None else 'Some(%d)'%c['cell'])
source+=''' ];
 for &(point,expected) in cases {let walls:Vec<_>=hash.segments_at_world_xz(point.map(f32::from_bits)).map(|s|s.wall.0).collect();assert_eq!(walls,expected.into_iter().collect::<Vec<_>>(),"point bits {point:?}");}
 let p=[1.0,1.0];let row=WallSpatialSegment {endpoints:[p,p],source_u:[0.0,1.0],relative_heights:[0.0;2],terrain_heights:[0.0;2],wall:WallId(9000)};
 hash.add_segment(row);hash.add_segment(row);assert_eq!(hash.segments_at_world_xz(p).filter(|s|s.wall.0==9000).count(),2,"original cell preserves both rows of one wall");
}
'''
(here/'point-query.tests.rs').write_text(source,encoding='utf-8');print('full native point query',len(cases),'cases')
