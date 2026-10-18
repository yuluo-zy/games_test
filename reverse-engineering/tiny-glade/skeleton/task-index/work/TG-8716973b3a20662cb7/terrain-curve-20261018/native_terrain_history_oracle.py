"""B update_terrain_curve runs original instructions with a one-bucket hash map."""
import json,struct,hashlib
from pathlib import Path
OUT=Path(__file__).resolve().parent
recipe=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py')
source=recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(source,str(recipe),'exec'),globals());OUT=Path(__file__).resolve().parent
TERRAIN=INPUT+0x8000;CONTROL=INPUT+0x9000;BUCKET=CONTROL-48;OLD_POINTS=INPUT+0xa000

def library_hash(uc,address,size,data):
    if address==0x1405b0ed0:ret(0)
u.hook_add(UC_HOOK_CODE,library_hash)
def case(source_points,old_points,progress):
    sp=reset();u.mem_write(INPUT,pack([v for p in source_points for v in p]));u.mem_write(OLD_POINTS,pack([v for p in old_points for v in p]));u.mem_write(TERRAIN,bytes(128));u.mem_write(BUCKET,bytes(64));u.mem_write(CONTROL,bytes([0])+bytes([255])*31)
    u.mem_write(TERRAIN+8,struct.pack('<Q',CONTROL));u.mem_write(TERRAIN+0x20,struct.pack('<Q',1));u.mem_write(BUCKET,struct.pack('<I',7));u.mem_write(BUCKET+8,struct.pack('<3Q',32,OLD_POINTS,len(old_points)))
    u.mem_write(sp+0x28,struct.pack('<Q',TERRAIN));u.reg_write(UC_X86_REG_RCX,7);u.reg_write(UC_X86_REG_RDX,INPUT);u.reg_write(UC_X86_REG_R8,len(source_points));u.reg_write(UC_X86_REG_XMM3,bits(progress));run(0x140859740)
    cap,ptr,n=struct.unpack('<3Q',u.mem_read(BUCKET+8,24));assert n<1000
    return {'source':[[bits(v)for v in p]for p in source_points],'old':[[bits(v)for v in p]for p in old_points],'progress':bits(progress),'points':list(struct.unpack('<'+'I'*(n*3),u.mem_read(ptr,n*12))),'height_factor':u.reg_read(UC_X86_REG_XMM0)&0xffffffff}
inputs=[([[0,0,0],[1,0,0],[1,1,1]],[[0,0,0],[1,1,0]]),([[0,0,0],[0,0,0],[2,0,0]],[[0,0,0],[1,0,0]]),([[0,0,0],[0,0,0]],[[0,0,0],[1,0,0]]),([[0,0,0],[1,0,0]],[[3,4,5]])]
cases=[case(source,old,p)for source,old in inputs for p in [0,.1,.25,.5,.99,1]]
report={'binary_sha256':hashlib.sha256(raw).hexdigest(),'entry':'0x140859740','method':'Original business entry through Unicorn. Only standard hash returns bucket0; original map probing, adjacent retain, Curve3 construction, cubic easing dispatch, slice, allocation/CRT bridge and point replacement execute. Known valid one-bucket map; missing lookup/allocator errors excluded.','cases':cases}
(OUT/'native-curve-history-fixtures.json').write_text(json.dumps(report,indent=2)+'\n');print({'native_update_curve_cases':len(cases)})
