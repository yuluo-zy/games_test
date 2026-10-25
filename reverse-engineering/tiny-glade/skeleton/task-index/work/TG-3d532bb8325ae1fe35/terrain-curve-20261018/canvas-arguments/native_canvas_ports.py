"""Original B terrain canvas caller ports and original constructor parameters.

Only bilinear sampling, string formatting/hash and the nested UI registry are
resolved dependency boundaries. Native caller gates and original argument,
layer, extent and history flags construction execute from identity-bound RNE.
"""
from pathlib import Path
import json,struct,hashlib
ROOT=Path('/Users/yuluo/Desktop/game');OUT=Path(__file__).resolve().parent
recipe=ROOT/'games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py'
s=recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(s,str(recipe),'exec'),globals());OUT=Path(__file__).resolve().parent
FRAME=INPUT+0x1000;PARAM=INPUT+0x2000;CURSOR=INPUT+0x3000;SOUNDS=INPUT+0x4000;UI=INPUT+0x5000;NAME=INPUT+0x6000
kind='';calls=[];water_value=0
def dependencies(uc,addr,size,data):
 if kind=='caller' and addr==0x140a14df0:
  u.reg_write(UC_X86_REG_XMM0,bits(water_value));ret(0)
 elif kind=='caller' and addr==0x14092df50:
  sp=u.reg_read(UC_X86_REG_RSP);n=u.reg_read(UC_X86_REG_R8)
  calls.append({'name':bytes(u.mem_read(u.reg_read(UC_X86_REG_RDX),n)).decode(),'name_length':n,'argument':u.reg_read(UC_X86_REG_R9)&255,'layer':u.mem_read(sp+0x28,1)[0]})
  ret(UI);u.reg_write(UC_X86_REG_RDX,123)
 elif kind=='constructor' and addr==0x140d79f90:
  target=u.reg_read(UC_X86_REG_RCX);u.mem_write(target,struct.pack('<3Q',0,NAME,0));ret(target)
 elif kind=='constructor' and addr==0x1405bdda0:ret(123)
u.hook_add(UC_HOOK_CODE,dependencies)
def caller(water,hit,distance):
 global kind,calls,water_value
 sp=reset();u.mem_write(INPUT,bytes(0x10000));kind='caller';calls=[];water_value=water
 u.mem_write(FRAME+0x118,struct.pack('<Q',PARAM));u.mem_write(PARAM,struct.pack('<Q',UI));u.mem_write(CURSOR,struct.pack('<I3f',int(hit),0,distance,0))
 u.reg_write(UC_X86_REG_RBP,FRAME);u.reg_write(UC_X86_REG_R15,CURSOR);u.reg_write(UC_X86_REG_XMM7,0)
 u.emu_start(0x140a9fe80,0x140a9ff4e,count=10000)
 return {'water_bits':bits(water),'hit':hit,'distance_bits':bits(distance),'calls':calls.copy()}
def constructor(arg,layer):
 global kind
 sp=reset();u.mem_write(INPUT,bytes(0x10000));kind='constructor';u.mem_write(NAME,b'terrain_draw')
 for r,v in[(UC_X86_REG_RCX,UI),(UC_X86_REG_RDX,NAME),(UC_X86_REG_R8,12),(UC_X86_REG_R9,arg)]:u.reg_write(r,v)
 u.mem_write(sp+0x28,bytes([layer]));u.mem_write(sp+0x30,struct.pack('<Q',SOUNDS))
 u.emu_start(0x14092df50,0x14092e0eb,count=10000)
 param=u.reg_read(UC_X86_REG_RBP)+0x60;b=bytes(u.mem_read(param,0xb0))
 return {'argument_input':arg,'layer_input':layer,'argument_79':b[0x79],'volume_tag_88':b[0x88],'layer_89':b[0x89],'extent_bits':list(struct.unpack_from('<2I',b,0x8c)),'occlude_a8':b[0xa8],'record_history_a9':b[0xa9]}
callers=[caller(w,h,d)for w,h,d in[(0,False,0),(1,False,0),(1,True,.5),(1,True,.6),(float('nan'),False,0)]]
ctors=[constructor(a,l)for a,l in[(0,1),(3,2),(7,9),(255,255)]]
assert[[(c['argument'],c['layer'])for c in r['calls']]for r in callers]==[[(3,2)],[(0,1),(3,2)],[(0,1),(3,2)],[(3,2)],[(3,2)]]
assert all(c['argument_79']==c['argument_input']and c['layer_89']==c['layer_input']and c['volume_tag_88']==4 and c['extent_bits']==[bits(130),bits(130)]and c['occlude_a8']==0 and c['record_history_a9']==1 for c in ctors)
report={'target':'B','binary_sha256':hashlib.sha256(raw).hexdigest(),'caller_range':'0x140a9fe80..0x140a9ff4e','constructor_range':'0x14092df50..0x14092e0eb','caller_cases':callers,'constructor_cases':ctors,'method':__doc__}
(OUT/'native-canvas-ports.json').write_text(json.dumps(report,indent=2)+'\n')
print({'caller_cases':len(callers),'constructor_cases':len(ctors),'all_assertions':True})
