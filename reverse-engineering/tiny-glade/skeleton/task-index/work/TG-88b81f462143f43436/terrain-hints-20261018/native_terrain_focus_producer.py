"""Bounded original B ui_edit_focus entry: actual terrain-focus producer.

UI widget insertion and collision lookup are resolved dependencies. Selection,
action/hover/focus guards, f32 distance comparison, and both queue writes execute
from the original function. No game branch is replaced by the harness.
"""
from pathlib import Path
import json, struct, hashlib
ROOT=Path('/Users/yuluo/Desktop/game')
OUT=Path(__file__).resolve().parent
recipe=ROOT/'games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py'
source=recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(source,str(recipe),'exec'),globals())
OUT=Path(__file__).resolve().parent
MODE,UI,NAV,ACTIONS,HOVER,CURSOR,CAMERA,WALLS,ORIGIN,PARAM,REQ,REQBUF,NOTIFY,NOTIFYBUF,CANCEL,CANCELBUF,EMPTY,HOVERTUPLE,NOTIFYTUPLE,TICK=[INPUT+i*0x1000 for i in range(1,21)]
dependency={}
def readq(p):return struct.unpack('<Q',u.mem_read(p,8))[0]
def readi(p):return struct.unpack('<I',u.mem_read(p,4))[0]
def queue(q,b):
    u.mem_write(q,bytes(128));u.mem_write(q+0x20,struct.pack('<3Q',32,b,0));u.mem_write(q+0x40,struct.pack('<Q',77))
def param(index,p):
    a=PARAM+index*0x40;u.mem_write(a,struct.pack('<Q',p));u.mem_write(a+0x10,struct.pack('<Q',TICK));u.mem_write(a+0x1c,struct.pack('<I',7));return a
def resolved_dependencies(uc,address,size,data):
    if address==0x1409236d0:
        name=bytes(u.mem_read(u.reg_read(UC_X86_REG_RDX),u.reg_read(UC_X86_REG_R8))).decode()
        dependency['widget_name']=name
        dependency['widget_params_hex']=bytes(u.mem_read(u.reg_read(UC_X86_REG_R9),0xb0)).hex()
        ret(UI);u.reg_write(UC_X86_REG_RDX,123)
    elif address==0x1408ce180:
        sp=u.reg_read(UC_X86_REG_RSP)
        dependency['query_mask']=readi(sp+0x38)
        dependency['query_max_toi_bits']=readi(sp+0x30)
        dependency['query_ray_bits']=list(struct.unpack('<6I',u.mem_read(u.reg_read(UC_X86_REG_R8),24)))
        target=u.reg_read(UC_X86_REG_RCX)
        tag=dependency['hit_tag'];u.mem_write(target,bytes(40));u.mem_write(target,bytes([tag]))
        u.mem_write(target+24,pack(dependency['geometry_point']))
        ret(target)
u.hook_add(UC_HOOK_CODE,resolved_dependencies)
def case(phase=0,timer=0,hover_tag=1,cursor_tag=1,nav_focus=0,cancel_phase=4,cancel_timer=0,geometry_tag=17,geometry_point=(0,0,100),terrain_point=(3,4,5),camera_origin=(0,0,0)):
    global dependency
    sp=reset();u.mem_write(INPUT,bytes(0x16000))
    dependency={'hit_tag':geometry_tag,'geometry_point':list(geometry_point)}
    u.mem_write(MODE,struct.pack('<I',10));u.mem_write(NAV+0x10,struct.pack('<I',nav_focus))
    for i in range(22):u.mem_write(ACTIONS+i*8,struct.pack('<If',4,0))
    u.mem_write(ACTIONS+8,struct.pack('<If',phase,timer));u.mem_write(ACTIONS+0x90,struct.pack('<If',cancel_phase,cancel_timer))
    u.mem_write(HOVER,struct.pack('<QQ',hover_tag,17));u.mem_write(CURSOR,struct.pack('<I',cursor_tag)+pack(terrain_point))
    u.mem_write(CAMERA,pack([*camera_origin,0,0,1]))
    for q,b in [(REQ,REQBUF),(NOTIFY,NOTIFYBUF),(CANCEL,CANCELBUF)]:queue(q,b)
    u.mem_write(HOVERTUPLE,struct.pack('<Q',HOVER));u.mem_write(HOVERTUPLE+0x20,struct.pack('<Q',CURSOR))
    u.mem_write(NOTIFYTUPLE,struct.pack('<Q',NOTIFY));u.mem_write(NOTIFYTUPLE+0x10,struct.pack('<Q',TICK));u.mem_write(NOTIFYTUPLE+0x20,struct.pack('<Q',ORIGIN))
    values=[MODE,UI,NAV,ACTIONS,REQ,CANCEL,EMPTY,CAMERA,EMPTY,WALLS,EMPTY,EMPTY,EMPTY]
    pointers=[param(i,p) for i,p in enumerate(values)]+[HOVERTUPLE,NOTIFYTUPLE]
    for r,p in zip([UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9],pointers):u.reg_write(r,p)
    for i,p in enumerate(pointers[4:]):u.mem_write(sp+0x28+i*8,struct.pack('<Q',p))
    run(0x1409771f0)
    nr=readq(REQ+0x30);nn=readq(NOTIFY+0x30)
    return {'input':{'phase':phase,'timer_bits':bits(timer),'hover_tag':hover_tag,'cursor_tag':cursor_tag,'nav_focus':nav_focus,'cancel_phase':cancel_phase,'cancel_timer_bits':bits(cancel_timer),'geometry_tag':geometry_tag,'geometry_point_bits':[bits(v) for v in geometry_point],'terrain_point_bits':[bits(v) for v in terrain_point],'camera_origin_bits':[bits(v)for v in camera_origin]},'requests':[{'tag':readi(REQBUF+i*24),'id':readi(REQBUF+i*24+4),'event_id':readq(REQBUF+i*24+16)}for i in range(nr)],'hint_tags':[u.mem_read(NOTIFYBUF+i*16+8,1)[0]for i in range(nn)],'cancel_count':readq(CANCEL+0x30),'origin_byte':u.mem_read(ORIGIN,1)[0],'dependencies':dependency.copy()}
cases=[case(phase=p,timer=t)for p,t in [(0,0),(1,0),(2,-1),(2,0),(2,1),(2,float('nan')),(3,0),(4,0)]]
cases += [case(hover_tag=h)for h in [0,1]]
cases += [case(nav_focus=n,cancel_phase=0)for n in [0,1]]
cases += [case(cursor_tag=tag,geometry_tag=6,geometry_point=p)for tag,p in [(0,(1,0,0)),(1,(1,0,0)),(1,(3,4,5)),(1,(3,4,6)),(1,(float('nan'),0,0))]]
cases += [case(terrain_point=p,geometry_tag=6,geometry_point=(3,4,6))for p in [(float('nan'),0,0),(3,4,5),(-0.0,0,0)]]
report={'target':'B','binary_sha256':hashlib.sha256(raw).hexdigest(),'entry':'0x1409771f0','original_entry_cases':len(cases),'method':__doc__,'cases':cases}
(OUT/'native-terrain-focus-producer-fixtures.json').write_text(json.dumps(report,indent=2)+'\n')
print({'cases':len(cases),'hint_tags':[c['hint_tags']for c in cases]})
