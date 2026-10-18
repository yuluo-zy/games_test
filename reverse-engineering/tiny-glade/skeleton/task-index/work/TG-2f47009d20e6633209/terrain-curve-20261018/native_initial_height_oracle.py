"""B state1 branch with resolved new-stroke/world-library/hash lookups."""
import json,struct,hashlib
from pathlib import Path
OUT=Path(__file__).resolve().parent
recipe=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py')
s=recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(s,str(recipe),'exec'),globals());OUT=Path(__file__).resolve().parent
TERRAIN=INPUT+0x5000;PARAM=INPUT+0x6000;TICK=INPUT+0x7000;CONTROL=INPUT+0x8000;BUCKET=CONTROL-48;MOUSE_META=INPUT+0x9000;MASK_META=INPUT+0xa000;VTABLE=INPUT+0xb000;MOUSE_VIEW=INPUT+0xc000;MASK_VIEW=INPUT+0xd000;MOUSE_RAW=INPUT+0xe000;CURSOR=INPUT+0xf000;UI_PARAM=INPUT+0x10000;UI=INPUT+0x11000;WORLD_PARAM=INPUT+0x12000;DRAG=INPUT+0x13000;CHANGED=INPUT+0x14000;PLACEMENT=INPUT+0x15000;QUEUE1=INPUT+0x16000;QUEUE2=INPUT+0x17000;BUF1=INPUT+0x18000;BUF2=INPUT+0x19000;MASK_DATA=HEAP+0xc00000;ASANY=STOP+0x100
mouse_missing=False
def bridge(uc,address,size,data):
    if address==0x140924a10:
        ret(1);u.reg_write(UC_X86_REG_RDX,9)
    elif address==0x140969720:
        key=u.reg_read(UC_X86_REG_RDX);ret((0 if mouse_missing else MOUSE_META) if key==0x142c2e408 else MASK_META)
    elif address==0x142923bf0:
        a,b,n=[u.reg_read(r)for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]];ret(0 if u.mem_read(a,n)==u.mem_read(b,n) else 1)
    elif address==ASANY:ret(u.reg_read(UC_X86_REG_RCX))
    elif address==0x1405b0ed0:ret(0)
u.hook_add(UC_HOOK_CODE,bridge)
def case(mouse_index,rock,drag):
    global mouse_missing
    reset();mouse_missing=mouse_index is None
    for p in [TERRAIN,PARAM,TICK,CONTROL,MOUSE_META,MASK_META,VTABLE,MOUSE_VIEW,MASK_VIEW,MOUSE_RAW,CURSOR,UI_PARAM,UI,WORLD_PARAM,DRAG,CHANGED,PLACEMENT,QUEUE1,QUEUE2,BUF1,BUF2]:u.mem_write(p,bytes(512))
    u.mem_write(BUCKET,bytes(48));u.mem_write(CONTROL,bytes([0])+bytes([255])*31);u.mem_write(TERRAIN+8,struct.pack('<Q',CONTROL));u.mem_write(TERRAIN+0x20,struct.pack('<Q',1));u.mem_write(TERRAIN+0x38,struct.pack('<Q',10));u.mem_write(BUCKET,struct.pack('<I',9));u.mem_write(BUCKET+0x28,pack([2.5]))
    for p,data in [(PARAM,TERRAIN),(UI_PARAM,UI),(CHANGED,QUEUE1),(PLACEMENT,QUEUE2)]:
        u.mem_write(p,struct.pack('<Q',data));u.mem_write(p+0x10,struct.pack('<Q',TICK));u.mem_write(p+0x1c,struct.pack('<I',77))
    for queue,buf in [(QUEUE1,BUF1),(QUEUE2,BUF2)]:u.mem_write(queue+0x20,struct.pack('<3Q',32,buf,0));u.mem_write(queue+0x40,struct.pack('<Q',42))
    u.mem_write(WORLD_PARAM,struct.pack('<Q',INPUT+0x1a000));u.mem_write(VTABLE+0x18,struct.pack('<Q',ASANY))
    u.mem_write(MOUSE_META,struct.pack('<4Q',MOUSE_VIEW,VTABLE,0x142a9df5b,0x6c));u.mem_write(MASK_META,struct.pack('<4Q',MASK_VIEW,VTABLE,0x142a9d4c1,0x3f))
    u.mem_write(MOUSE_VIEW,struct.pack('<2Q',MOUSE_RAW,1));u.mem_write(MOUSE_RAW+0x10,struct.pack('<i',-1 if mouse_index is None else mouse_index));u.mem_write(MASK_VIEW,struct.pack('<2Q',MASK_DATA,8192));u.mem_write(MASK_DATA,(b'\xff' if rock else b'\x00')*(8192*4))
    position=[3.0,drag,4.0];u.mem_write(CURSOR,struct.pack('<I',1)+pack(position));u.mem_write(DRAG,pack([drag]));frame=STACK+0x8000
    u.mem_write(frame+0x48,pack(position));u.mem_write(frame+0x58,struct.pack('<Q',WORLD_PARAM));u.mem_write(frame+0x128,struct.pack('<Q',UI_PARAM));u.mem_write(frame+0x130,struct.pack('<Q',CHANGED));u.mem_write(frame+0x160,struct.pack('<Q',PLACEMENT))
    u.reg_write(UC_X86_REG_RBP,frame);u.reg_write(UC_X86_REG_RCX,PARAM);u.reg_write(UC_X86_REG_R14,DRAG);u.reg_write(UC_X86_REG_R15,CURSOR)
    u.emu_start(0x140aa00e9,0x140aa076d,count=100000)
    return {'mouse_index':mouse_index,'rock':rock,'drag_bits':bits(drag),'height_bits':struct.unpack('<I',u.mem_read(BUCKET+0x28,4))[0],'editing_active':struct.unpack('<I',u.mem_read(TERRAIN,4))[0],'ui_active':u.mem_read(UI+0x1e0,1)[0],'changed_count':struct.unpack('<Q',u.mem_read(QUEUE1+0x30,8))[0],'placement_count':struct.unpack('<Q',u.mem_read(QUEUE2+0x30,8))[0],'placement_tag':struct.unpack('<I',u.mem_read(BUF2+8,4))[0]}
cases=[case(idx,rock,drag)for idx in [None,-1,7]for rock in [False,True]for drag in [-1.0,0.0,1.0,10.0,25.0,float('nan')]]
(OUT/'native-initial-height-fixtures.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(raw).hexdigest(),'range':'0x140aa00e9..0x140aa076d','method':'Original state1 instructions execute; new_stroke resolves to an existing bucket, AssetSsboLibrary/world lookups/as_any and standard hash/memcmp are bridged. Original optional mouse -1 check, live mask bit indexing, height clamp, UI active and event writes execute.','cases':cases},indent=2)+'\n')
print({'initial_height_cases':len(cases)})
