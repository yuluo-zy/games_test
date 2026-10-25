"""Original B inline event writers; inputs are native locals/typed event buffers.
No business callback is hooked. Starts after existing producer calculations, so
this validates writer schema and native EventId placement, not producer guards.
Inactive enum storage/alignment padding is deliberately seeded and not claimed.
"""
from pathlib import Path
import json,struct,random,hashlib
ROOT=Path('/Users/yuluo/Desktop/game')
HERE=Path(__file__).resolve().parent
recipe=(ROOT/'games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py').read_text().split('def spline(')[0]
recipe=recipe.replace("D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne",str(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'))
exec(compile(recipe,'shared-native-memory-harness','exec'),globals())
OUT=HERE
rng=random.Random(20261018)
def w64(p,v):u.mem_write(p,struct.pack('<Q',v))
def r64(p):return struct.unpack('<Q',u.mem_read(p,8))[0]
def event_buffer(seed):
 wrapper=INPUT+0x1000;events=INPUT+0x2000;tick=INPUT+0x3000;buf=INPUT+0x4000
 u.mem_write(events,bytes(0x50));w64(events+0x20,16);w64(events+0x28,buf);w64(events+0x30,2);w64(events+0x40,seed)
 u.mem_write(wrapper,struct.pack('<QQQII',events,0,tick,0,0x10203040));u.mem_write(buf,bytes([0xa7])*0x1000)
 return wrapper,events,tick,buf
cases={'deco':[],'stair':[],'finish':[]}
for n in range(12):
 sp=reset()-8;u.reg_write(UC_X86_REG_RSP,sp);u.mem_write(sp,bytes([0xd3])*0x100)
 seed=0x1234567800000000+n;wrapper,e,t,b=event_buffer(seed)
 dst=bytes(rng.getrandbits(8)for _ in range(24));pos=bytes(rng.getrandbits(8)for _ in range(12));rot=rng.getrandbits(32);ty=rng.randrange(256)
 u.mem_write(INPUT,dst);u.mem_write(INPUT+0x100,pos)
 for reg,val in [(UC_X86_REG_RBP,INPUT),(UC_X86_REG_RSI,INPUT+0x100),(UC_X86_REG_RDI,ty),(UC_X86_REG_R13,wrapper),(UC_X86_REG_XMM6,rot)]:u.reg_write(reg,val)
 u.emu_start(0x140977080,0x140977120,count=1000)
 data=bytes(u.mem_read(b+2*0x48,0x48))
 assert data[:24]==dst and data[0x29]==3 and data[0x2c:0x30]==struct.pack('<I',rot)and data[0x30:0x3c]==pos and data[0x3c]==ty
 assert struct.unpack('<Q',data[0x40:])[0]==seed and r64(e+0x40)==seed+1 and r64(e+0x30)==3
 assert bytes(u.mem_read(t,4))==struct.pack('<I',0x10203040)
 cases['deco'].append({'destination':dst.hex(),'position':pos.hex(),'rotation_bits':rot,'ty':ty,'event_id':seed,'output':data.hex()})
for n in range(12):
 reset();frame=STACK+0x12000;local=bytes(rng.getrandbits(8)for _ in range(0x900));u.mem_write(frame,local);u.reg_write(UC_X86_REG_RBP,frame)
 seed=0x2234567800000000+n;wrapper,e,t,b=event_buffer(seed)
 field8=rng.getrandbits(64);rotation=rng.getrandbits(32);height=rng.getrandbits(32);pair=rng.getrandbits(64);flag_a=rng.randrange(256);flag_b=rng.randrange(256);index=rng.getrandbits(32)
 for reg,val in [(UC_X86_REG_RDI,wrapper),(UC_X86_REG_R14,field8),(UC_X86_REG_XMM8,rotation),(UC_X86_REG_XMM6,height),(UC_X86_REG_XMM7,pair),(UC_X86_REG_R15,flag_a),(UC_X86_REG_RBX,flag_b),(UC_X86_REG_RSI,index)]:u.reg_write(reg,val)
 u.emu_start(0x1407a0db8,0x1407a0e94,count=1000)
 data=bytes(u.mem_read(b+2*0x78,0x78));expected=bytearray([0xa7]*0x78)
 expected[:8]=local[0x668:0x670];expected[8:0x10]=struct.pack('<Q',field8);expected[0x10:0x38]=local[:0x28];expected[0x38:0x44]=local[0x490:0x49c]
 expected[0x44:0x54]=struct.pack('<IIQ',rotation,height,pair);expected[0x54:0x56]=bytes([flag_a,flag_b]);expected[0x56:0x5e]=local[0x580:0x588];expected[0x5c:0x64]=local[0x586:0x58e];expected[0x64:0x68]=struct.pack('<I',index);expected[0x68]=0x3f;expected[0x69:0x6d]=local[0x240:0x244];expected[0x6c:0x70]=local[0x243:0x247];expected[0x70:0x78]=struct.pack('<Q',seed)
 assert data==expected and r64(e+0x40)==seed+1 and r64(e+0x30)==3
 cases['stair'].append({'locals':local.hex(),'field8':field8,'rotation_bits':rotation,'height_bits':height,'pair_bits':pair,'flags':[flag_a,flag_b],'index':index,'event_id':seed,'output':data.hex()})
for n in range(12):
 reset();frame=STACK+0x12000;u.mem_write(frame,bytes(0x900));u.reg_write(UC_X86_REG_RBP,frame)
 seed=0x3234567800000000+n;wrapper,e,t,b=event_buffer(seed)
 w64(frame+0xb8,t);u.mem_write(frame+0x2bc,struct.pack('<I',0x10203040));w64(frame+0x1e8,e)
 u.emu_start(0x142294871,0x1422948c9,count=1000)
 data=bytes(u.mem_read(b,32));assert data==bytes([0xa7])*16+struct.pack('<Q',seed)+bytes([0xa7])*8
 assert r64(e+0x40)==seed+1 and r64(e+0x30)==3
 cases['finish'].append({'event_id':seed,'buffer':data.hex()})
report={'original_sha256':hashlib.sha256(raw).hexdigest(),'method':__doc__,'ranges':{'deco':['0x140977080','0x140977120'],'stair':['0x1407a0db8','0x1407a0e94'],'finish':['0x142294871','0x1422948c9']},'cases':cases}
(HERE/'native-insta-writer-fixtures.json').write_text(json.dumps(report,indent=2)+'\n')
print({k:len(v)for k,v in cases.items()})
