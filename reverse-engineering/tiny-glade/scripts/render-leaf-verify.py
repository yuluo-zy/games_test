"""Untouched original leaf entrypoints compared to compiled private Rust code."""
from pathlib import Path
import json, struct, random, subprocess, hashlib
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *
ROOT = Path('D:/game/reverse-engineering/tiny-glade')
OUT = ROOT / 'evidence/render-leaves'
raw = (ROOT/'workspace/input/tiny-glade.exe').read_bytes()
pe = struct.unpack_from('<I',raw,60)[0]; opt = pe+24
base = struct.unpack_from('<Q',raw,opt+24)[0]
size = struct.unpack_from('<I',raw,opt+56)[0]
cpu = Uc(UC_ARCH_X86,UC_MODE_64); cpu.mem_map(base,(size+4095)&~4095)
for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
    vs,va,rs,ro = struct.unpack_from('<IIII',raw,opt+struct.unpack_from('<H',raw,pe+20)[0]+40*i+8)
    cpu.mem_write(base+va,raw[ro:ro+rs])
STACK=0x20000000; STOP=0x30000000
cpu.mem_map(STACK,0x20000); cpu.mem_map(STOP,4096)
cpu.mem_write(STOP,b'\xcc')
panic_seen = False
def hook(c,pc,sz,user):
    global panic_seen
    if pc == 0x1428d9390:
        panic_seen=True; c.emu_stop()
cpu.hook_add(UC_HOOK_CODE,hook)
def original(kind,value):
    global panic_seen
    panic_seen=False
    sp=STACK+0x10000
    cpu.mem_write(sp,struct.pack('<Q',STOP)+bytes(0x100))
    for reg in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RDI,UC_X86_REG_RSI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]: cpu.reg_write(reg,0)
    cpu.reg_write(UC_X86_REG_RCX,value); cpu.reg_write(UC_X86_REG_RSP,sp)
    cpu.emu_start({'b':0x140bd4b00,'i':0x140bd49d0,'p':0x140c10e10}[kind],STOP,count=500)
    return 'panic' if panic_seen else format(cpu.reg_read(UC_X86_REG_RAX)&(255 if kind=='p' else 0xffffffff),'x')

cargo='C:/Users/liyu/.cargo/bin/cargo.exe'; rustc='C:/Users/liyu/.cargo/bin/rustc.exe'
build=subprocess.run([cargo,'build','--release','--locked','--manifest-path',str(ROOT/'reconstruction/render/Cargo.toml'),'--lib','--message-format=json'],capture_output=True,text=True,encoding='utf8',check=True)
ash=next(p for line in build.stdout.splitlines() for r in [json.loads(line)] if r.get('reason')=='compiler-artifact' and r['target']['name']=='ash' for p in r['filenames'] if p.endswith('.rlib'))
exe=ROOT/'workspace/render-leaf-runner.exe'
native_lib=next(Path('C:/Users/liyu/.cargo/registry/src').glob('*/windows_x86_64_msvc-0.52.6/lib/windows.0.52.0.lib')).parent
compile_result=subprocess.run([rustc,'--edition=2024','-O',str(ROOT/'scripts/render-leaf-runner.rs'),'-o',str(exe),'-L','dependency='+str(Path(ash).parent),'-L','native='+str(native_lib),'--extern','ash='+ash],capture_output=True,text=True,encoding='utf8')
if compile_result.returncode:
    raise RuntimeError(compile_result.stderr)

rng=random.Random(20261012)
consumed=[0,1,2,3,5,6,11,12,15,16]
buf=[sum(1<<bit for j,bit in enumerate(consumed) if n>>j&1) for n in range(1<<len(consumed))]
buf += [1<<b for b in range(32)] + [0xffffffff,0x80000000,0x7fffffff]+[rng.getrandbits(32) for _ in range(5000)]
img=list(range(1<<17)) + [1<<b for b in range(17,32)] + [0xffffffff,0x80000000,0x7fffffff]+[rng.getrandbits(32) for _ in range(5000)]
cases=[('b',v) for v in buf]+[('i',v) for v in img]
payload=''.join(f'{k} {v:x}\n' for k,v in cases)
actual=subprocess.run([str(exe)],input=payload,capture_output=True,text=True,check=True).stdout.splitlines()
expected=[original(k,v) for k,v in cases]
fail=[{'mode':k,'input':hex(v),'original':a,'rust':b} for (k,v),a,b in zip(cases,expected,actual) if a!=b]
pred={str(v):original('p',v) for v in range(256)}
report={'ok':not fail and len(actual)==len(expected),'source_sha256':hashlib.sha256(raw).hexdigest(),'cases':len(cases),'buffer_cases':len(buf),'image_cases':len(img),'failure_count':len(fail),'failures':fail[:20],
 'predicate_binary_only':{'cases':256,'true_discriminants':[int(v) for v,x in pred.items() if x=='1'],'reimplemented':False,'dependency':'vk-sync-fork 0.4.0, original lock path package'},
 'image_returning_inputs_in_exhaustive_low17':{hex(v):x for v,x in zip(img,expected[len(buf):]) if v<1<<17 and x!='panic'},
 'candidate_source_sha256':hashlib.sha256((ROOT/'reconstruction/render/resource_usage.rs').read_bytes()).hexdigest(),
 'renderer_cargo_lock_sha256':hashlib.sha256((ROOT/'reconstruction/render/Cargo.lock').read_bytes()).hexdigest(),
 'oracle':'Unicorn original function entries, no replaced core calls; stop at actual original Rust panic entry 0x1428d9390 for invalid image masks',
 'coverage':['all 1024 combinations of the ten consumed buffer bits','all 32 single bits and full32 boundary masks','5000 deterministic random u32 masks per converter','exhaustive 131072 low17 image masks','all 256 predicate byte values observed, dependency not reconstructed'],
 'limitations':['No live Vulkan device or game execution','Panic occurrence compared; panic text/backtrace/allocator/exception ABI excluded','Unknown input flag names remain raw numeric masks','No arbitrary high32 exhaustive enumeration; unknown bits tested individually and randomly','Original vk_sync predicate observed only as caller boundary, excluded from private Rust recovery']}
(OUT/'verification.json').write_text(json.dumps(report,indent=2))
print(json.dumps({k:report[k] for k in ['ok','cases','buffer_cases','image_cases','failure_count']}))
