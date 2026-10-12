"""Use untouched B serializer call arguments and eave function as Rust oracles."""
from pathlib import Path
import json, struct, hashlib, random, subprocess
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *
from verification_provenance import candidate_provenance

ROOT=Path(__file__).resolve().parents[2]
baseline=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']['B']
raw=Path(baseline['binary']['path']).read_bytes()
assert hashlib.sha256(raw).hexdigest()==baseline['binary']['sha256']
pe=struct.unpack_from('<I',raw,60)[0];opt=pe+24
base=struct.unpack_from('<Q',raw,opt+24)[0];size=struct.unpack_from('<I',raw,opt+56)[0]
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(size+4095)&~4095)
for i in range(struct.unpack_from('<H',raw,pe+6)[0]):
    _,rva,n,offset=struct.unpack_from('<IIII',raw,opt+struct.unpack_from('<H',raw,pe+20)[0]+40*i+8)
    if n:cpu.mem_write(base+rva,raw[offset:offset+n])
DATA,STACK=0x20000000,0x30000000
cpu.mem_map(DATA,0x20000);cpu.mem_map(STACK,0x20000)
observed=[]
serializers={0x1404bdc80:4,0x1404bd170:8,0x1404ba9b0:1,0x1404be720:1}
def ret(c):
    sp=c.reg_read(UC_X86_REG_RSP);target=struct.unpack('<Q',c.mem_read(sp,8))[0]
    c.reg_write(UC_X86_REG_RSP,sp+8);c.reg_write(UC_X86_REG_RIP,target)
def hook(c,pc,sz,user):
    if pc in serializers:
        key=bytes(c.mem_read(c.reg_read(UC_X86_REG_RDX),c.reg_read(UC_X86_REG_R8))).decode()
        value=bytes(c.mem_read(c.reg_read(UC_X86_REG_R9),serializers[pc]))
        observed.append((key,value.hex()));c.reg_write(UC_X86_REG_RAX,0);ret(c)
    elif pc==0x1402b5870:
        # Published Vec/serde output machinery is outside game field mapping.
        c.reg_write(UC_X86_REG_RAX,0);ret(c)
cpu.hook_add(UC_HOOK_CODE,hook)
def setup():
    cpu.mem_write(STACK+0x1000,struct.pack('<Q',STACK))
    cpu.reg_write(UC_X86_REG_RSP,STACK+0x1000)
    cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
def serialize(block):
    observed.clear();setup()
    cpu.mem_write(DATA,block);cpu.mem_write(DATA+0x100,struct.pack('<Q',DATA+0x200))
    cpu.mem_write(DATA+0x200,struct.pack('<QQQ',0x1000,DATA+0x1000,0))
    cpu.reg_write(UC_X86_REG_RCX,DATA);cpu.reg_write(UC_X86_REG_RDX,DATA+0x100)
    cpu.emu_start(0x140259960,STACK,count=3000)
    assert cpu.reg_read(UC_X86_REG_RIP)==STACK
    return dict(observed)
def eave(profile,b):
    block=bytearray(28);struct.pack_into('<I',block,8,profile);struct.pack_into('<I',block,20,b)
    setup();cpu.mem_write(DATA,bytes(block));cpu.reg_write(UC_X86_REG_RCX,DATA)
    cpu.emu_start(0x1408e3c30,STACK,count=40)
    return f'{cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff:08x}'
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def main():
    rng=random.Random(20261010)
    cases=[];inputs=[];expected_fields=[]
    for _ in range(1024):
        fields=[bits(rng.uniform(-2,2)) for _ in range(6)];direction=rng.randrange(2)
        inputs.append('pack '+' '.join(f'{x:08x}' for x in fields)+f' {direction}')
        expected_fields.append(dict(zip(['tip_offset_01','profile_01','height_01','ridge_length_01','eave_length_01','ridge_dir'],
            [struct.pack('<II',*fields[:2]).hex(),*(struct.pack('<I',x).hex() for x in fields[2:]),bytes([direction]).hex()])))
    edges=[bits(x) for x in [0.,-0.,0.1,0.5,1.,2.,1e-20]]
    pairs=[(a,b) for a in edges for b in edges]+[(bits(rng.uniform(0,1)),bits(rng.uniform(-2,2)))for _ in range(2048)]
    inputs+=['eave '+f'{a:08x} {b:08x}' for a,b in pairs]
    binary=ROOT/'reconstruction/target/release/parameter-probe'
    actual=subprocess.run([str(binary)],input='\n'.join(inputs)+'\n',capture_output=True,text=True,check=True).stdout.splitlines()
    assert len(actual)==len(inputs)
    failures=[]
    for i,(line,expected) in enumerate(zip(actual[:1024],expected_fields)):
        block=bytes.fromhex(line)
        fields=serialize(block)
        if any(fields.get(k)!=v for k,v in expected.items()) or block[25:]!=bytes([0xa5])*3:
            failures.append({'case':i,'expected':expected,'observed':fields,'padding':block[25:].hex()})
    for i,((a,b),result) in enumerate(zip(pairs,actual[1024:])):
        original=eave(a,b)
        if original!=result:failures.append({'case':i,'kind':'eave','input':[hex(a),hex(b)],'original':original,'rust':result})
    report={'ok':not failures,'source_sha256':baseline['binary']['sha256'],
        'field_mapping_cases':1024,'eave_cases':len(pairs),'failures':failures,
        'scope':'B field mapping via original serialization call arguments; finite eave function outputs; not full serde output/runtime lifecycle',
        'excluded':['serde/Vec output routines bridged; not reimplemented','nonfinite eave inputs and SSE exception flags','rim default behavior','A/C in-memory layouts'],
        'rust_source_sha256':hashlib.sha256((ROOT/'reconstruction/roof/parameters.rs').read_bytes()).hexdigest(),
        'cargo_lock_sha256':hashlib.sha256((ROOT/'reconstruction/Cargo.lock').read_bytes()).hexdigest()}
    report.update(candidate_provenance())
    (ROOT/'evidence/multi-version/B-parameters-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({k:report[k] for k in ['ok','field_mapping_cases','eave_cases']}))
    if failures:print(json.dumps(failures[:3]));raise SystemExit(1)
if __name__=='__main__':main()
