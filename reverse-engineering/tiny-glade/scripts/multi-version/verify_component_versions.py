"""Execute small game initializer/predicate leaves in A/B/C, not library code."""
from pathlib import Path
import hashlib,json,random,struct,subprocess
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
from capstone.x86_const import X86_REG_RIP,X86_OP_MEM
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
from verification_provenance import ROOT,candidate_provenance

def main():
    baselines=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']
    rng=random.Random(20261017);rows=[];failures=[]
    for version,baseline in baselines.items():
        functions=json.loads((ROOT/f'evidence/multi-version/{version}-functions.json').read_text())['functions']
        needles=['roof_tile_sim18RoofTileSimulation3new17','roof_animation13RoofAnimation3new17','roof_animation13RoofAnimation17is_being_animated17']
        chosen=[next(f for f in functions if needle in f['name']) for needle in needles]
        binary=Path(baseline['binary']['path']);pe=baseline['pe'];base=pe['image_base']
        with binary.open('rb') as file:assert hashlib.file_digest(file,'sha256').hexdigest()==baseline['binary']['sha256']
        def read(va,n):
            rva=va-base;s=next(s for s in pe['sections'] if s['rva']<=rva<s['rva']+s['raw_size'])
            with binary.open('rb') as f:f.seek(s['raw_pointer']+rva-s['rva']);return f.read(n)
        cpu=Uc(UC_ARCH_X86,UC_MODE_64)
        end=max(s['rva']+max(s['raw_size'],s['virtual_size']) for s in pe['sections'])
        cpu.mem_map(base,(end+4095)&~4095)
        data,stack=0x20000000,0x30000000;cpu.mem_map(data,0x1000);cpu.mem_map(stack,0x2000)
        code_rows=[]
        for function in chosen:
            va=int(function['va'],16);raw=read(va,128);cpu.mem_write(va,raw)
            decoder=Cs(CS_ARCH_X86,CS_MODE_64);decoder.detail=True;instructions=[]
            for i in decoder.disasm(raw,va):
                instructions.append({'va':hex(i.address),'text':i.mnemonic+' '+i.op_str,'bytes':i.bytes.hex()})
                for operand in i.operands:
                    if operand.type==X86_OP_MEM and operand.mem.base==X86_REG_RIP:
                        address=i.address+i.size+operand.mem.disp;cpu.mem_write(address,read(address,16))
                if i.mnemonic=='ret':break
            assert instructions[-1]['text'].startswith('ret')
            code_rows.append({'function':function,'instructions':instructions})
        def execute(index,raw,argument=0):
            cpu.mem_write(data,raw);cpu.mem_write(stack+0x1000,struct.pack('<Q',stack))
            cpu.reg_write(UC_X86_REG_RSP,stack+0x1000);cpu.reg_write(UC_X86_REG_RCX,data)
            cpu.reg_write(UC_X86_REG_XMM0,0xaaaaaaaa);cpu.reg_write(UC_X86_REG_XMM1,argument)
            cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
            cpu.emu_start(int(chosen[index]['va'],16),stack,count=100)
            assert cpu.reg_read(UC_X86_REG_RIP)==stack
        lines=[];expected=[]
        for _ in range(128):
            raw=rng.randbytes(24);argument=rng.getrandbits(32);execute(0,raw,argument)
            lines.append(f'tile {raw.hex()} {argument:08x}');expected.append(bytes(cpu.mem_read(data,24)).hex())
        result=subprocess.run([str(ROOT/'reconstruction/target/release/lifecycle-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True)
        for i,(want,got) in enumerate(zip(expected,result.stdout.splitlines())):
            if want!=got:failures.append({'version':version,'case':i,'original':want,'rust':got})
        execute(1,bytes(8),0xbbbbbbbb)
        assert cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff==0 and cpu.reg_read(UC_X86_REG_XMM1)&0xffffffff==0
        for value in [0,0x80000000,0x3f800000,0x3f7fffff,0x7fc01234]:
            execute(2,struct.pack('<II',value,0))
            want=int(struct.unpack('<f',struct.pack('<I',value))[0]!=1.0)
            assert cpu.reg_read(UC_X86_REG_RAX)&1==want
        rows.append({'baseline':version,'source_sha256':baseline['binary']['sha256'],'tile_cases':128,
            'animation_new_cases':1,'animation_predicate_cases':5,'source_classification':'private game business',
            'source_paths':['crates/country-core/src/resources/roofs/roof_tile_sim.rs','crates/country-core/src/resources/roofs/roof_animation.rs'],
            'functions':code_rows})
    report={'ok':not failures,'versions':rows,'failure_count':len(failures),'failures':failures,
        'scope':'A/B/C TileSimulation constructor compared to same Rust adapter; animation default/predicate original leaf behavior checked; not complete publisher or lifecycle equivalence',
        'limitations':['Common private source remains game business, never filtered as library merely because it matches',
            'Caller field producers and ABI/component ownership not verified across versions'],**candidate_provenance()}
    (ROOT/'evidence/building-lifecycle/three-version-component-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'ok':report['ok'],'versions':len(rows),'tile_cases':384,'failure_count':len(failures)}))
    if failures:raise SystemExit(1)

if __name__=='__main__':main()
