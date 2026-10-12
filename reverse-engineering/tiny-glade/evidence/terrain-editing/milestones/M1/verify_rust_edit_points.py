"""Original resolved edit branches and full point helper vs Rust CLI.
Expected observations come only from RNE instructions, not a copied formula.
"""
import hashlib
import json
import random
import struct
import subprocess
from pathlib import Path
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

OUT = Path(__file__).resolve().parent
ROOT = next(p for p in OUT.parents if (p / 'tracking/baselines.json').exists())
TERRAIN = ROOT / 'reconstruction/terrain'
PROBE = TERRAIN / 'target/release/terrain-probe'
raw = (ROOT.parents[2] / 'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest() == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
cpu = Uc(UC_ARCH_X86, UC_MODE_64)
end = max(s.VirtualAddress + max(s.Misc_VirtualSize, s.SizeOfRawData) for s in pe.sections)
cpu.mem_map(base, (end + 4095) & ~4095)
for s in pe.sections:
    if s.SizeOfRawData:
        cpu.mem_write(base+s.VirtualAddress, raw[s.PointerToRawData:s.PointerToRawData+s.SizeOfRawData])
DATA, STACK, RETURN = 0x20000000, 0x30000000, 0x40000000
cpu.mem_map(DATA, 0x40000)
cpu.mem_map(STACK, 0x10000)
cpu.mem_map(RETURN, 0x1000)
CMD, BUCKET, AUDIO, EVENTS, TICK, POINTS = [DATA+n for n in (0x100,0x800,0x1000,0x2000,0x3000,0x10000)]
RBP = STACK+0x8000
stops = [0]
allowed = [(0, 0)]
reached = [False]

def hook(uc,address,size,user):
    if address == stops[0]:
        reached[0] = True
        uc.emu_stop()
    elif not allowed[0][0] <= address < allowed[0][1]:
        raise RuntimeError(f'unexpected external execution {address:#x}')
cpu.hook_add(UC_HOOK_CODE,hook)

def bits(v): return struct.unpack('<I',struct.pack('<f',v))[0]
def pack(values): return struct.pack('<'+'I'*len(values),*values)

def reset():
    cpu.mem_write(DATA,bytes(0x40000))
    cpu.mem_write(STACK,bytes(0x10000))
    cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
    reached[0] = False

inputs, expected, scopes = [],[],[]
rng = random.Random(20261013)
edges = [0,0x80000000,bits(0.4),bits(1),bits(2.5),bits(20),bits(-3),0x7f800000,0xff800000,0x7fc12345]
# Full resolved numeric dispatch, bypassing only original hash lookup.
for i in range(900):
    old_f, old_h = [edges[(i+j)%len(edges)] if i<300 else bits(rng.uniform(-4,24)) for j in range(2)]
    fv, hv = [edges[(i+j+3)%len(edges)] if i<300 else bits(rng.uniform(-5,5)) for j in range(2)]
    ft, ht = (i//3)%3, i%3
    reset()
    cpu.mem_write(CMD,pack([ft,fv,ht,hv,bits(3),bits(4),bits(5),0]))
    cpu.mem_write(BUCKET-16,pack([old_f,0,old_h]))
    cpu.mem_write(AUDIO+0x20,struct.pack('<QQQ',64,EVENTS,0))
    cpu.mem_write(AUDIO+0x40,struct.pack('<Q',17))
    cpu.mem_write(RBP+0xd0,struct.pack('<Q',TICK))
    cpu.mem_write(RBP+0xf0,struct.pack('<I',123))
    regs = [(UC_X86_REG_RSI,CMD),(UC_X86_REG_R15,BUCKET),(UC_X86_REG_R10,0),
            (UC_X86_REG_RDI,AUDIO),(UC_X86_REG_RBP,RBP),(UC_X86_REG_RSP,STACK+0x6000),
            (UC_X86_REG_XMM7,0),(UC_X86_REG_XMM8,bits(1)),
            (UC_X86_REG_XMM9,int.from_bytes(pack([0x7fffffff]*4),'little')),(UC_X86_REG_XMM10,bits(20))]
    for reg,v in regs: cpu.reg_write(reg,v)
    allowed[0] = (0x14094ead0,0x14094ef1b)
    stops[0] = 0x14094ef1b
    cpu.emu_start(0x14094ead0,stops[0]+1,count=5000)
    assert reached[0]
    f = struct.unpack('<I',cpu.mem_read(BUCKET-16,4))[0]
    h = struct.unpack('<I',cpu.mem_read(BUCKET-8,4))[0]
    count = struct.unpack('<Q',cpu.mem_read(AUDIO+0x30,8))[0]
    audio = []
    for j in range(count):
        tag = struct.unpack('<I',cpu.mem_read(EVENTS+j*40+8,4))[0]
        magnitude_offset = 32 if tag==2 else 28
        magnitude = struct.unpack('<I',cpu.mem_read(EVENTS+j*40+magnitude_offset,4))[0]
        audio.append(f'{tag}:{magnitude:08x}')
    inputs.append(f'edit {old_f:08x} {old_h:08x} {ft} {fv:08x} {ht} {hv:08x}')
    expected.append(f'{f:08x} {h:08x} '+','.join(audio))
    scopes.append('resolved_edit')
# Entire insert_or_morph entry/return, native Vec allocation excluded by capacity.
for i in range(300):
    n = [0,1,2,3,1023,1024][i%6]
    coords = [[bits(rng.uniform(-5,5)) for _ in range(3)] for _ in range(n)]
    point = [edges[(i+j)%len(edges)] if i<100 else bits(rng.uniform(-5,5)) for j in range(3)]
    spacing = edges[(i+3)%len(edges)] if i<100 else bits(rng.uniform(0,20))
    if n and i%5==0: coords[-1] = point[:]
    reset()
    cpu.mem_write(CMD,struct.pack('<QQQ',2048,POINTS,n))
    cpu.mem_write(CMD+0x40,pack(point))
    if coords: cpu.mem_write(POINTS,pack([v for p in coords for v in p]))
    rsp=STACK+0x8000
    cpu.mem_write(rsp,struct.pack('<Q',RETURN))
    for reg,v in [(UC_X86_REG_RCX,CMD),(UC_X86_REG_RDX,CMD+0x40),(UC_X86_REG_XMM2,spacing),(UC_X86_REG_RSP,rsp)]: cpu.reg_write(reg,v)
    allowed[0]=(0x140a97880,0x140a97a25)
    stops[0]=RETURN
    cpu.emu_start(0x140a97880,RETURN+1,count=2000)
    assert reached[0]
    result_n=struct.unpack('<Q',cpu.mem_read(CMD+16,8))[0]
    values=struct.unpack('<'+'I'*(result_n*3),cpu.mem_read(POINTS,result_n*12)) if result_n else []
    inputs.append('points '+' '.join(f'{v:08x}' for v in point+[spacing]+[v for p in coords for v in p]))
    expected.append(str(result_n)+''.join(f' {v:08x}' for v in values))
    scopes.append('point_helper')
result=subprocess.run([str(PROBE)],input='\n'.join(inputs)+'\n',text=True,capture_output=True,check=True)
actual=result.stdout.splitlines()
assert len(actual)==len(expected)
failures=[{'case':i,'scope':scopes[i],'input':inputs[i],'expected':e,'actual':a} for i,(a,e) in enumerate(zip(actual,expected)) if a!=e]
(OUT/'edit-points-inputs.txt').write_text('\n'.join(inputs)+'\n')
(OUT/'edit-points-native.txt').write_text('\n'.join(expected)+'\n')
(OUT/'edit-points-rust.txt').write_text(result.stdout)
report={'ok':not failures,'cases':len(inputs),'resolved_edit_cases':900,'point_cases':300,'failure_count':len(failures),'failures':failures,
        'original_sha256':hashlib.sha256(raw).hexdigest(),'rust_binary_sha256':hashlib.sha256(PROBE.read_bytes()).hexdigest(),
        'rust_sources':{str(p.relative_to(TERRAIN)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted((TERRAIN/'crates/tg-terrain/src').rglob('*.rs'))},
        'cargo_lock_sha256':hashlib.sha256((TERRAIN/'Cargo.lock').read_bytes()).hexdigest(),
        'scope':'Native resolved numeric Keep/Absolute/Delta incl quiet NaN/Inf/signed zero; full point-helper entry-return with reserved capacity vs compiled Rust CLI',
        'excluded':['ID allocation','HashMap lookup','event expansion','recalculate/readback outside fragment','UI','GPU','A/C','system acceptance'],
        'candidate_rust_cli_executed':True,'target_execution':False}
(OUT/'rust-edit-points-diff.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'ok':not failures,'cases':len(inputs),'failures':len(failures)}))
if failures: raise SystemExit(1)
