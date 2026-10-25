"""Observe prepared inputs and outputs for the actual four face loops."""
import capture_rectangular as orig
import struct,json
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
faces=[]; pending={};active=None
def curve(ptr):
 cap,points,n,ucap,us,un=struct.unpack('<6Q',orig.cpu.mem_read(ptr,48))
 assert n<256 and n==un,(hex(ptr),cap,hex(points),n,un)
 return {'points':bytes(orig.cpu.mem_read(points,n*8)).hex(),'u':bytes(orig.cpu.mem_read(us,un*4)).hex(),'length_bits':bytes(orig.cpu.mem_read(ptr+48,4)).hex()}
def observe(cpu,pc,length,user):
 global active
 if pc==0x140dce930:
  rbp=cpu.reg_read(UC_X86_REG_RBP);out=cpu.reg_read(UC_X86_REG_RCX);env=cpu.reg_read(UC_X86_REG_RDX);ps=struct.unpack('<7Q',cpu.mem_read(env,56))
  pending.update({'out':out,'rbp':rbp,'corners':[bytes(cpu.mem_read(p,8)).hex() for p in ps[2:6]],'profile':curve(ps[1]),'world_profile':curve(rbp+0x2b8),
   'row_count_bits':bytes(cpu.mem_read(ps[0],4)).hex(),'rng_before':struct.unpack('<Q',cpu.mem_read(rbp+0x3a8,8))[0],
   'ordinal_before':struct.unpack('<I',cpu.mem_read(rbp+0x474,4))[0],'planes':bytes(cpu.mem_read(rbp+0x280,32)).hex()})
 elif pc==0x1421b0ff2:
  cap,ptr,n=struct.unpack('<QQQ',cpu.mem_read(pending['out'],24));assert n<256
  active=dict(pending,rows=bytes(cpu.mem_read(ptr,n*24)).hex(),tile_records=[],callsites=[]);faces.append(active)
 elif pc==0x1421b2ca0 and active is not None:
  rsp=cpu.reg_read(UC_X86_REG_RSP);ret=struct.unpack('<Q',cpu.mem_read(rsp-8,8))[0];ptr=cpu.reg_read(UC_X86_REG_RDX)
  if ret-5 in [0x1421b1bf2,0x1421b1e8e]:
   active['tile_records'].append(bytes(cpu.mem_read(ptr,64)).hex());active['callsites'].append(hex(ret-5))
   rbp=active['rbp'];active['rng_after']=struct.unpack('<Q',cpu.mem_read(rbp+0x3a8,8))[0]
cpu=orig.cpu;cpu.hook_add(UC_HOOK_CODE,observe)
def run_case(shape,roof,flag):
 global active
 faces.clear();pending.clear();active=None
 result=orig.case(shape,roof,flag)
 return result,list(faces)
if __name__=='__main__':
 ref=json.loads((orig.ROOT/'evidence/roof-ridge/rectangular-reference.json').read_text(encoding='utf8'))
 result,observed=run_case(bytes.fromhex(ref['rectangle_bytes']),bytes.fromhex(ref['roof_bytes']),0)
 report=dict(face_count=len(observed),faces=observed,rectangle_bytes=ref['rectangle_bytes'],roof_bytes=ref['roof_bytes'],completed=result.get('completed'),panic=result.get('panic'),scope='Prepared inputs observed from the complete original rectangular entry, all curve/math executed in RNE')
 (orig.ROOT/'evidence/roof-ridge/face-reference.json').write_text(json.dumps(report,indent=2),encoding='utf8')
 print(json.dumps({'completed':report['completed'],'faces':len(observed),'rows':[len(x['rows'])//48 for x in observed],'tiles':[len(x['tile_records']) for x in observed]},indent=2))

