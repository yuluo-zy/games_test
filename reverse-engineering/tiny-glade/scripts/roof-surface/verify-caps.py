"""Full original rectangle invocation -> isolated cap TileInstance subsequence.
The start endpoints/rng/ordinal are explicitly observed prefix context; this
test does not claim that recovering the parent preamble is already complete.
"""
from pathlib import Path
import importlib.util,json,struct,random,subprocess,hashlib,sys
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path('D:/game/reverse-engineering/tiny-glade')
spec=importlib.util.spec_from_file_location('rect_oracle',ROOT/'scripts/roof-ridge/capture_rectangular.py');oracle=importlib.util.module_from_spec(spec);spec.loader.exec_module(oracle)
seen={}
def xmm(cpu,reg):return struct.unpack('<4f',cpu.reg_read(reg).to_bytes(16,'little'))
def capture(cpu,pc,length,user):
    bp=cpu.reg_read(UC_X86_REG_RBP)
    if pc==0x1421afea8:
        a=xmm(cpu,UC_X86_REG_XMM12);b=xmm(cpu,UC_X86_REG_XMM8)
        seen.update(a=[a[0],a[1],xmm(cpu,UC_X86_REG_XMM13)[0]],b=[b[0],b[1],xmm(cpu,UC_X86_REG_XMM15)[0]],
            is_gable=cpu.reg_read(UC_X86_REG_RAX)&255,rng=struct.unpack('<Q',cpu.mem_read(bp+0x3a8,8))[0],ordinal=struct.unpack('<I',cpu.mem_read(bp+0x474,4))[0])
    if pc==0x1421b062d:
        seen.update(end_rng=struct.unpack('<Q',cpu.mem_read(bp+0x3a8,8))[0],end_ordinal=struct.unpack('<I',cpu.mem_read(bp+0x474,4))[0])
oracle.cpu.hook_add(UC_HOOK_CODE,capture)
def make(profile=.5,height=.4,ridge=.6,eave=.7,tip=(0.,0.),direction=0,style=0,rotation=(1.,0.),dims=(6.,9.),flag=0):
    roof=bytearray(oracle.roof)
    for o,v in [(0xc,rotation[0]),(0x10,rotation[1]),(0x1c,dims[0]),(0x20,dims[1]),(0x24,tip[0]),(0x28,tip[1]),(0x2c,profile),(0x30,height),(0x34,ridge),(0x38,eave)]:struct.pack_into('<f',roof,o,v)
    roof[0x3c]=direction;roof[0x54]=style
    shape=struct.pack('<6f',rotation[0],rotation[1],0.,0.,dims[0],dims[1])
    return shape,roof,flag
cached='--cached' in sys.argv
baseline=json.loads((ROOT/'evidence/roof-surface/caps-verification.json').read_text(encoding='utf-8')) if cached else None
cases=[make()]
for direction in [0,1]:
 for style in [0,1]:
  for ridge in [.0,.3,1.]:cases.append(make(direction=direction,style=style,ridge=ridge,profile=.7,height=.65))
fixtures=json.loads((ROOT/'evidence/roof-fixtures.json').read_text(encoding='utf-8'))['samples']
for i,item in enumerate(fixtures[::24]):
    p=item['params'];cases.append(make(profile=p['profile_01'],height=p['height_01'],ridge=p['ridge_length_01'],eave=p['eave_length_01'],tip=p['tip_offset_01'],direction=i%2,style=(i//2)%2,rotation=(.8,.6),dims=(4.,7.),flag=i%2))
if cached:
    cases=[(bytes.fromhex(c['shape_bytes']),bytes.fromhex(c['roof_bytes']),c['flag']) for c in baseline['cases']]
lines=[];expected=[];sourcecases=[];skips=[]
def bits(v):return f'{struct.unpack("<I",struct.pack("<f",v))[0]:08x}'
for index,(shape,roof,flag) in enumerate(cases):
    seen.clear()
    if cached:
        observed=baseline['cases'][index];seen.update(observed['observed_context']);cap=observed['original_records']
    else:
        result=oracle.case(shape,roof,flag)
        assert result.get('completed') and not result['panic'],result
        cap=[r for r,site in zip(result['tile_records'],result['tile_callsites']) if site in ['0x1421b042c','0x1421b05f0']]
    if 'a' not in seen:
        assert not cap;skips.append(index);continue
    lines.append(' '.join([*(bits(v) for v in seen['a']+seen['b']),bits(struct.unpack_from('<f',roof,0x30)[0]),bits(struct.unpack_from('<f',roof,0x2c)[0]),str(seen['is_gable']),str(roof[0x54]),str(struct.unpack_from('<I',roof)[0]),str(flag),f'{seen["rng"]:016x}',str(seen['ordinal']),shape.hex(),roof.hex()]))
    expected.append(f'{seen["end_rng"]:016x} {seen["end_ordinal"]}'+''.join(' '+r for r in cap))
    sourcecases.append(dict(index=index,roof_bytes=roof.hex(),shape_bytes=shape.hex(),flag=flag,observed_context=dict(seen),caps=len(cap),original_records=cap))
    print(f'case {index} caps {len(cap)}',flush=True)
binary=ROOT/'workspace/rectangular-caps.exe'
import sys
sys.path.insert(0,str(ROOT/'scripts'))
from dependency_build import build_runner
build_runner(ROOT/'scripts/roof-surface/caps_runner.rs',binary)
result=subprocess.run([str(binary)],input='\n'.join(lines)+'\n',text=True,capture_output=True,check=True)
actual=result.stdout.splitlines();failed=[dict(index=i,original=a,candidate=b,input=lines[i]) for i,(a,b) in enumerate(zip(expected,actual)) if a!=b]
report=dict(ok=not failed and len(actual)==len(expected),source_sha256=hashlib.sha256(oracle.raw).hexdigest(),full_game_executed=False,
    oracle='full original assemble_rectangular_roof entry to return using shared capture_rectangular oracle, cap subsequence at exact Writer calls compared',
    core_stubs=False,context_source='endpoints generated independently by Rust from original roof/rectangle bytes and checked against observed prefix endpoints; RNG and ordinal are the common evolving state observed after original edge producer',
    current_run='recheck independent Rust prefix/caps against prior complete original-machine captures' if cached else 'fresh full original-machine capture and Rust comparison',
    endpoint_validation='22 exact endpoint bit comparisons, followed by caps recomputed from independently derived Rust endpoints',
    external_dependency=dict(json.loads((ROOT/'evidence/roof-surface/verification.json').read_text(encoding='utf-8'))['external_dependency'],math_api_bridges=[value[0] for value in oracle.funcs.values()]),
    cap_scope='original gable extension + length/partitions + quaternion + dimensions + position + packed TileInstance + RNG/ordinal updates',
    compared_records=sum(c['caps'] for c in sourcecases),cases=sourcecases,gate_skipped_cases=skips,failure_count=len(failed),failures=failed[:5],
    limitations=['Combined fully Rust rectangular entry must pass its edge producer RNG and ordinal state; this test observes only that evolving state from original producer','Original ownership, Writer GPU upload and rendering not verified','Sampled finite nondegenerate roof geometry; NaN, allocation failure and panic branches excluded','Source history parameters combined with declared synthetic wall rectangle shapes; no scene replay'])
(ROOT/'evidence/roof-surface/caps-verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps(dict(ok=report['ok'],cases=len(lines),records=report['compared_records'],failures=len(failed))))
if failed:raise SystemExit(1)
