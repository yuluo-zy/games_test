"""Original B UI recording guards and finish lifecycle with resolved diff leaves.

Start executes the original allocator/initializer, bridging only std random-key
TLS. Finish resolves the existing independent decorator/stair diff owners and
add_decorator_diff; its real input-recording commit leaf and event write execute.
This proves lifecycle gating/order, not those independently recovered diff leaves.
"""
from pathlib import Path
import json,struct,hashlib
ROOT=Path('/Users/yuluo/Desktop/game');OUT=Path(__file__).resolve().parent
recipe=ROOT/'games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py'
source=recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(source,str(recipe),'exec'),globals());OUT=Path(__file__).resolve().parent
MODE,UI,HISTORY,WIDGET,STORAGE,PARAM,KEYS,TICK,ARC1,ARC2,LEDGER,EDITS,INPUTQ,INPUTBUF,READER1,READER2,READER3,EVENTS1,EVENTS2,EVENTS3,CURSOR1,CURSOR2,CURSOR3=[INPUT+i*0x800 for i in range(1,24)]
observed={}
EMPTY_AUX1=bytes(64);EMPTY_AUX2=bytes(96)
def q(p):return struct.unpack('<Q',u.mem_read(p,8))[0]
def i(p):return struct.unpack('<I',u.mem_read(p,4))[0]
def param(index,p):
    a=PARAM+index*0x40;u.mem_write(a,struct.pack('<Q',p));u.mem_write(a+0x10,struct.pack('<Q',TICK));u.mem_write(a+0x1c,struct.pack('<I',7));return a
def dependencies(uc,a,size,data):
    if a==0x1400d9b20:ret(KEYS)
    elif a==0x14132ade0:ret(55)
    elif a==0x140b2cb30:
        observed['calls'].append('extract_decorator_diff');p=u.reg_read(UC_X86_REG_RCX);u.mem_write(p,bytes(48));ret(p)
    elif a==0x140900010:
        observed['calls'].append('extract_stairs_diff');p=u.reg_read(UC_X86_REG_RCX);u.mem_write(p,bytes(80));ret(p)
    elif a==0x140a854a0:observed['calls'].append('add_decorator_diff');ret()
    elif a==0x140a86270:observed['calls'].append('start_input_recording')
    elif a==0x140a86730:observed['calls'].append('finish_input_recording')
u.hook_add(UC_HOOK_CODE,dependencies)
def seed(pending=True,edit_count=1):
    global observed
    sp=reset();u.mem_write(INPUT,bytes(0x10000));observed={'calls':[]}
    u.mem_write(HISTORY,struct.pack('<3Q',32,LEDGER,0));u.mem_write(HISTORY+0x68,struct.pack('<i',-1))
    u.mem_write(HISTORY+0x30,struct.pack('<5Q',edit_count if pending else 0x8000000000000000,EDITS,edit_count,ARC1,ARC2))
    u.mem_write(ARC1,EMPTY_AUX1);u.mem_write(ARC2,EMPTY_AUX2)
    for a in [ARC1,ARC2]:u.mem_write(a,struct.pack('<2Q',2,2))
    # A valid preallocated Terrain Arc/EditType; the finish leaf transfers
    # this ownership without inspecting the independently recovered payload.
    terrain_arc=INPUT+0xc000;terrain_points=INPUT+0xd000
    u.mem_write(terrain_arc,struct.pack('<2Q',2,2)+struct.pack('<4QIf',0x8000000000000000,1,terrain_points,1,17,2.5))
    u.mem_write(terrain_points,pack([3,0,4]));u.mem_write(EDITS,struct.pack('<2Q',0,terrain_arc))
    u.mem_write(INPUTQ,struct.pack('<9Q',0,8,0,0,32,INPUTBUF,0,0,77))
    u.mem_write(UI+0x20,struct.pack('<Q',WIDGET));return sp
def start_case(mode,hover,phase,flags,pending=False):
    seed(pending,0);sp=STACK+0x10000;u.reg_write(UC_X86_REG_RSP,sp)
    u.mem_write(MODE,struct.pack('<I',mode));u.mem_write(UI+0x18,bytes([hover,phase]));u.mem_write(WIDGET+0xb9,bytes([flags]))
    u.mem_write(sp+0x90,struct.pack('<Q',MODE));u.reg_write(UC_X86_REG_R14,MODE);u.reg_write(UC_X86_REG_R9,UI)
    u.reg_write(UC_X86_REG_RAX,HISTORY);u.reg_write(UC_X86_REG_R10,TICK);u.reg_write(UC_X86_REG_RSI,7);u.reg_write(UC_X86_REG_RCX,PARAM)
    u.emu_start(0x141734605,0x14173463f,count=10000)
    return {'mode':mode,'hover':hover,'phase':phase,'flags':flags,'was_recording':pending,'started':bool(observed['calls']),'pending_words':list(struct.unpack('<3Q',u.mem_read(HISTORY+0x30,24))),'old_arc_refs':[q(ARC1),q(ARC2)]}
def reader(p,e,c,n):
    u.mem_write(p,struct.pack('<2Q',c,e));u.mem_write(e,struct.pack('<9Q',32,EDITS,n,0,0,8,0,n,n))
def finish_case(hover,phase,flags,first,second,forced,pending=True,edit_count=1):
    sp=seed(pending,edit_count);u.mem_write(UI+0x18,bytes([hover,phase]));u.mem_write(WIDGET+0xb9,bytes([flags]))
    for p,e,c,n in [(READER1,EVENTS1,CURSOR1,first),(READER2,EVENTS2,CURSOR2,second),(READER3,EVENTS3,CURSOR3,forced)]:reader(p,e,c,n)
    params=[param(0,UI),param(1,STORAGE),param(2,HISTORY),param(3,0),param(4,0),param(5,INPUTQ),READER1,READER2,READER3,param(6,0),param(7,0)]
    for r,p in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],params):u.reg_write(r,p)
    for j,p in enumerate(params[4:]):u.mem_write(sp+0x28+j*8,struct.pack('<Q',p))
    run(0x1409dd790)
    n=q(INPUTQ+0x30)
    return {'hover':hover,'phase':phase,'flags':flags,'events':[first,second,forced],'pending':pending,'edit_count':edit_count,'calls':observed['calls'],'reader_cursors':[q(c)for c in[CURSOR1,CURSOR2,CURSOR3]],'recording_after':q(HISTORY+0x30)!=0x8000000000000000,'ledger_len':q(HISTORY+0x10),'cursor':struct.unpack('<i',u.mem_read(HISTORY+0x68,4))[0],'storage_immutable':u.mem_read(STORAGE+0x1b0,1)[0],'input_notifications':[{'niche':hex(q(INPUTBUF+j*48)),'tag_word':q(INPUTBUF+j*48+8),'event_id':q(INPUTBUF+j*48+40)}for j in range(n)]}
