"""Independent bounded execution of original UI helpers; no Rust codegen."""
from pathlib import Path
import hashlib, json, math, struct
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

OUT = Path(__file__).resolve().parent
SRC = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
raw = SRC.read_bytes()
assert hashlib.sha256(raw).hexdigest() == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
image = pe.get_memory_mapped_image()
md = Cs(CS_ARCH_X86, CS_MODE_64)
cpu = Uc(UC_ARCH_X86, UC_MODE_64)
cpu.mem_map(base, (pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095)
cpu.mem_write(base, image)
stack, stop = 0x70000000, 0x72000000
cpu.mem_map(stack, 0x10000)
cpu.mem_map(stop, 0x1000)
stop_pcs = {stop}
cpu.hook_add(UC_HOOK_CODE, lambda c,p,n,u: c.emu_stop() if p in stop_pcs else None)
sp = stack + 0x8008

def float_bits(v):
    return struct.unpack('<I', struct.pack('<f', v))[0]

def native_inside(lx, ly, ux, uy, px, py):
    # Directly observed machine operand contract: XMM0/1 lower bounds,
    # XMM2/3 upper bounds, stack+28/+30 point. No game helper is hooked.
    for reg, value in zip([UC_X86_REG_XMM0,UC_X86_REG_XMM1,UC_X86_REG_XMM2,UC_X86_REG_XMM3], [lx,ly,ux,uy]):
        cpu.reg_write(reg, float_bits(value))
    cpu.mem_write(sp, struct.pack('<Q', stop))
    cpu.mem_write(sp+0x28, struct.pack('<f', px))
    cpu.mem_write(sp+0x30, struct.pack('<f', py))
    cpu.reg_write(UC_X86_REG_RSP, sp)
    cpu.reg_write(UC_X86_REG_RAX, 0x1122334455667788)
    cpu.emu_start(0x14123ffc0, stop, count=100)
    assert cpu.reg_read(UC_X86_REG_RIP) == stop
    return cpu.reg_read(UC_X86_REG_RAX)&255

vals = [-math.inf, -1., -0., 0., .25, 1., 2., math.inf, math.nan]
cases = []
for px in vals:
    for py in vals:
        args = [0.,0.,1.,1.,px,py]
        observed = native_inside(*args)
        expected = 0. < px < 1. and 0. < py < 1.
        assert observed == expected, (args,observed,expected)
        cases.append({'input_bits':[f'{float_bits(v):08x}' for v in args], 'native_result':observed})
for bounds in [(1.,0.,0.,1.),(0.,1.,1.,0.),(0.,0.,0.,0.),(-math.inf,-math.inf,math.inf,math.inf)]:
    args = [*bounds,.25,.25]
    observed = native_inside(*args)
    expected = bounds[0]<.25<bounds[2] and bounds[1]<.25<bounds[3]
    assert observed == expected
    cases.append({'input_bits':[f'{float_bits(v):08x}' for v in args], 'native_result':observed})

# Independently execute both actual inline history predicates. Stop before the
# History method call, so this proves the caller gate and owner address only.
mode_ptr, ui_ptr, arc_ptr, out_ptr = stack+0x100, stack+0x200, stack+0x400, stack+0x600
gates=[]
for entry, call, skip in [(0x141734605,0x141734637,0x14173463f),(0x1414e182e,0x1414e1860,0x1414e1868)]:
    stop_pcs.update({call,skip})
    count=0
    for mode in [0,12,13]:
        for tag in range(4):
            for phase in range(5):
                for flags in [0,1,2,3,4,5,254,255]:
                    cpu.mem_write(mode_ptr,struct.pack('<I',mode))
                    cpu.mem_write(ui_ptr+0x18,bytes([tag,phase]))
                    cpu.mem_write(ui_ptr+0x20,struct.pack('<Q',arc_ptr))
                    cpu.mem_write(arc_ptr+0xb9,bytes([flags]))
                    for reg,value in [(UC_X86_REG_R14,mode_ptr),(UC_X86_REG_R9,ui_ptr),(UC_X86_REG_R10,out_ptr),(UC_X86_REG_RSP,sp)]:cpu.reg_write(reg,value)
                    cpu.emu_start(entry,stop,count=50)
                    reached = cpu.reg_read(UC_X86_REG_RIP)
                    expected = mode!=13 and tag>=2 and phase==3 and bool(flags&1)
                    assert reached == (call if expected else skip),(entry,mode,tag,phase,flags,hex(reached))
                    count+=1
    gates.append({'entry':hex(entry),'cases':count,'mismatches':0,'scope':'gate only; actual History method not executed'})
    stop_pcs.difference_update({call,skip})

# Execute the actual game lookup helper rather than substituting the entire
# get_ui_volume. Only standard keyed-hash and lock instrumentation are bridged.
def return_from_call(value=None):
    if value is not None:cpu.reg_write(UC_X86_REG_RAX,value)
    rsp=cpu.reg_read(UC_X86_REG_RSP)
    target=struct.unpack('<Q',cpu.mem_read(rsp,8))[0]
    cpu.reg_write(UC_X86_REG_RSP,rsp+8)
    cpu.reg_write(UC_X86_REG_RIP,target)

def std_bridge(c,pc,size,user):
    if pc==0x1405b0d70:return_from_call(0) # fixed standard hasher output for prepared bucket
    elif pc in [0x140c81c70,0x140c81e10]:return_from_call() # parking_lot debug instrumentation
cpu.hook_add(UC_HOOK_CODE,std_bridge)
panic=0x1428d9310
stop_pcs.add(panic)
lookup_cases=[]
ctrl=stack+0x1000
for query,stored,n in [(11,11,1),(22,22,1),(11,22,1),(22,11,1),(11,11,0)]:
    cpu.mem_write(ui_ptr,bytes(0x100))
    cpu.mem_write(ui_ptr+0x68,struct.pack('<4Q',ctrl,3,0,n))
    cpu.mem_write(ctrl,bytes([0]+[255]*31))
    cpu.mem_write(ctrl-16,struct.pack('<QQ',stored,arc_ptr))
    cpu.mem_write(arc_ptr,struct.pack('<QQ',100,1))
    cpu.mem_write(sp,struct.pack('<Q',stop))
    for reg,value in [(UC_X86_REG_RCX,ui_ptr),(UC_X86_REG_RDX,query),(UC_X86_REG_RSP,sp)]:cpu.reg_write(reg,value)
    cpu.emu_start(0x140b2dc60,stop,count=3000)
    end=cpu.reg_read(UC_X86_REG_RIP)
    present=n!=0 and query==stored
    assert end==(stop if present else panic),(query,stored,n,hex(end))
    strong=struct.unpack('<Q',cpu.mem_read(arc_ptr,8))[0]
    assert strong==(101 if present else 100)
    assert struct.unpack('<Q',cpu.mem_read(ui_ptr+0x60,8))[0]==(0 if present else 16)
    if present:assert cpu.reg_read(UC_X86_REG_RAX)==arc_ptr
    lookup_cases.append({'query':query,'stored':stored,'size':n,'result':'Arc cloned' if present else 'unwrap panic reached','strong_count':strong})
stop_pcs.remove(panic)

spans = [('is_inside_bbx',0x14123ffc0,44),('no_history_recording_prefix',0x1409a8f10,64),('history_start_inline_a',0x141734605,0x3a),('history_start_inline_b',0x1414e182e,0x3a),('clearing_canvas_flag_store',0x14092e0e2,9),('get_ui_volume',0x140b2dc60,503)]
manifest=[]
for name, va, size in spans:
    data=image[va-base:va-base+size]
    lines=[f'{i.address:x}: {i.bytes.hex():24s} {i.mnemonic} {i.op_str}' for i in md.disasm(data,va)]
    (OUT/(name+'.asm')).write_text('\n'.join(lines)+'\n',encoding='utf-8')
    manifest.append({'name':name,'va':hex(va),'size':size,'sha256':hashlib.sha256(data).hexdigest()})
(OUT/'native-aabb-fixtures.json').write_text(json.dumps(cases,indent=2)+'\n',encoding='utf-8')
(OUT/'native-verification.json').write_text(json.dumps({'source':str(SRC),'source_sha256':hashlib.sha256(raw).hexdigest(),'aabb_cases':len(cases),'history_start_gates':gates,'get_ui_volume_cases':lookup_cases,'mismatches':0,'bridges':'explicit stop hooks; get_ui_volume reuses only fixed std hash output and parking_lot debug-hook bridges','spans':manifest},indent=2)+'\n',encoding='utf-8')
print(json.dumps({'original_aabb_cases':len(cases),'history_start_gate_cases':sum(g['cases'] for g in gates),'mismatches':0,'span_count':len(spans)}))
