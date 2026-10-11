import sys,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];sys.path.insert(0,str(ROOT/'scripts/multi-version'))
import verify_wall_arches_freehand as v
n=v.native
from unicorn import UC_HOOK_MEM_INVALID

def bad(cpu,typ,address,size,value,data):print('BAD',hex(cpu.reg_read(n.UC_X86_REG_RIP)),hex(address),size,flush=True);return False
n.cpu.hook_add(UC_HOOK_MEM_INVALID,bad)
source=json.loads((ROOT/'evidence/building-structures/B-segment-materialization-verification.json').read_text())['input_cases']
for i,s in enumerate(source):
 if s['closed']:continue
 c=dict(shape=2,flag=0,roof=True,allocation=19+i,wall_float=v.f32([.5,1.3,5.,14.][i%4]),public_288=0.,clutter=1,wall_id=20015998343868+i,source_length=12.3,points=s['points'],us=s['us'],values=s['segment_values'],arch=s['arch'],third=[0.]*len(s['points']),masks=[dict(flags=s['flags'],coords=s['coords'],range=s['loop_range'],length=0.)])
 print('case',i,flush=True)
 try:r=v.original(c)
 except Exception as e:print(e);(ROOT/'evidence/building-arches-freehand/B-oracle-fixture-failure.json').write_text(json.dumps(dict(case=i,input=c,error=repr(e)),indent=2));raise
