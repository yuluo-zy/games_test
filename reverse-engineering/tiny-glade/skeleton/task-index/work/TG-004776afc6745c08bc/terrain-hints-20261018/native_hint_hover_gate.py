from pathlib import Path
import json,struct,hashlib
OUT=Path(__file__).resolve().parent
source=(OUT/'native_hint_terrain.py').read_text()
exec(compile(source.split('\ncases=')[0],str(OUT/'native_hint_terrain.py'),'exec'),globals())
active_gate=None
gate_hit=False
GATES={'layering':0x142283ef3,'focus':0x142283b81,'steepness':0x14228384a}
def gate_hook(uc,address,size,data):
    global gate_hit
    if address==active_gate:
        gate_hit=True
        uc.emu_stop()
u.hook_add(UC_HOOK_CODE,gate_hook)
results=[]
for kind in ENTRIES:
    for hover_tag in [0,1,2]:
        case(kind,[],False)
        u.mem_write(MODE,struct.pack('<I',10))
        u.mem_write(HOVER,struct.pack('<2Q',hover_tag,99))
        if kind=='layering':u.mem_write(STATE,struct.pack('<QB',2,0))
        elif kind=='steepness':u.mem_write(STATE,b'\x01\x00')
        sp=STACK+0x10008
        u.mem_write(sp,struct.pack('<Q',STOP))
        u.reg_write(UC_X86_REG_RSP,sp)
        u.reg_write(UC_X86_REG_RCX,param(0x00,MODE))
        u.reg_write(UC_X86_REG_RDX,param(0x40,PROFILE))
        u.reg_write(UC_X86_REG_R8,param(0x80,STATE))
        u.reg_write(UC_X86_REG_R9,READER)
        active_gate=GATES[kind];gate_hit=False
        u.emu_start(ENTRIES[kind],STOP,count=3000000)
        assert gate_hit==(hover_tag!=0),(kind,hover_tag,gate_hit)
        assert u.reg_read(UC_X86_REG_RIP) in [STOP,active_gate]
        active_gate=None
        results.append({'kind':kind,'hover_tag':hover_tag,'passed_display_gate':gate_hit})
(OUT/'native-hint-hover-gate-fixtures.json').write_text(json.dumps({'target':'B','binary_sha256':hashlib.sha256(raw).hexdigest(),'cases':results,'boundary':'Original full entry and display guards run with valid Res<HoveredTerrainStrokeId>. Stop before display-builder/backend boundary; only that gate is accepted, no display payload or glyph/binding behavior.'},indent=2)+'\n')
print({'original_hover_gate_cases':len(results)})
