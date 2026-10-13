"""Original B inline projections, with explicit caller register bindings.

These fragments are not standalone functions or claimed Rust ABIs. Only the
selected leaf computation and verified branch exits execute; game launch never
occurs. fmodf delegates to the host's original CRT API.
"""
from pathlib import Path
import sys,json,ctypes,random,math,hashlib
OUT=Path(__file__).resolve().parent
sys.path.insert(0,str(OUT.parent))
import native_oracle as n
from native_oracle import *
OUT=Path(__file__).resolve().parent
fmod=ctypes.CDLL('ucrtbase.dll').fmodf
fmod.argtypes=[ctypes.c_float,ctypes.c_float];fmod.restype=ctypes.c_float
active_exits=set();exit_pc=None
PANICS={0x1407c9d95,0x1407c9e06,0x140777c78,0x140777c5b,0x140777c95,0x140777cb2,0x14121e6cb}
def value(reg):return struct.unpack('<f',struct.pack('<I',cpu.reg_read(reg)&0xffffffff))[0]
def put(reg,v):cpu.reg_write(reg,bits(v))
def inline_hook(c,pc,size,user):
    global exit_pc
    if pc in active_exits:exit_pc=pc;c.emu_stop()
    elif pc in PANICS:raise RuntimeError('original panic '+hex(pc))
    elif pc==0x142923f10:
        put(UC_X86_REG_XMM0,fmod(value(UC_X86_REG_XMM0),value(UC_X86_REG_XMM1)));ret()
cpu.hook_add(UC_HOOK_CODE,inline_hook)
def run(begin,exits):
    global active_exits,exit_pc
    active_exits=set(exits);exit_pc=None
    try:
        cpu.emu_start(begin,STOP,count=100000)
    except RuntimeError as e:
        if 'original panic' in str(e):return {'panic':True,'panic_pc':str(e)}
        raise
    assert exit_pc in active_exits,hex(cpu.reg_read(UC_X86_REG_RIP))
    return {'panic':False,'exit':hex(exit_pc)}
def scalar(kind,a,b=0,t=0):
    a,b,t=[struct.unpack('<f',struct.pack('<f',v))[0] for v in (a,b,t)]
    setup();put(UC_X86_REG_XMM6,1)
    if kind=='new':put(UC_X86_REG_XMM0,a);begin,end,out=0x14121b67e,0x14121b6ab,UC_X86_REG_XMM2
    elif kind=='wrap':put(UC_X86_REG_XMM0,a);put(UC_X86_REG_XMM9,1);begin,end,out=0x1407ca887,0x1407ca8b2,UC_X86_REG_XMM12
    elif kind=='add':
        put(UC_X86_REG_XMM10,a);put(UC_X86_REG_XMM11,b);put(UC_X86_REG_XMM9,1);begin,end,out=0x1407ca8b2,0x1407ca8e2,UC_X86_REG_XMM10
    elif kind=='subtract':
        put(UC_X86_REG_XMM10,a);put(UC_X86_REG_XMM11,b);begin,end,out=0x1407ca875,0x1407ca8b2,UC_X86_REG_XMM12
    elif kind=='distance':
        cpu.reg_write(UC_X86_REG_RBP,DATA+0x8000);cpu.mem_write(DATA+0x8360,struct.pack('<4f',a,0,0,0));put(UC_X86_REG_XMM10,b);begin,end,out=0x14121d549,0x14121d56f,UC_X86_REG_XMM2
    elif kind=='lerp':
        put(UC_X86_REG_XMM6,a);put(UC_X86_REG_XMM7,b);put(UC_X86_REG_XMM9,b+1);put(UC_X86_REG_XMM12,1)
        cpu.reg_write(UC_X86_REG_RDX,DATA+0x100);cpu.mem_write(DATA+0x100,struct.pack('<f',t));cpu.reg_write(UC_X86_REG_RBX,DATA+0x200);cpu.mem_write(DATA+0x200,struct.pack('<f',0));cpu.reg_write(UC_X86_REG_RBP,DATA+0x8000)
        begin,end,out=0x14077579a,0x140775876,UC_X86_REG_XMM9
    else:raise ValueError(kind)
    result=run(begin,[end]);
    if not result['panic']:result['bits']=cpu.reg_read(out)&0xffffffff
    return result
def boolean(kind,a,b,p=0):
    setup()
    if kind=='eq':
        put(UC_X86_REG_XMM6,a);put(UC_X86_REG_XMM15,b);result=run(0x1407756d7,[0x1407756df,0x1407756f1]);result['value']=exit_pc==0x1407756df
    elif kind=='contains':
        put(UC_X86_REG_XMM12,a);put(UC_X86_REG_XMM10,b);put(UC_X86_REG_XMM0,p);put(UC_X86_REG_XMM9,1)
        result=run(0x1407ca8ec,[0x1407ca9fd,0x1407ca923,0x1407ca9da]);result['value']=exit_pc==0x1407ca9fd
    elif kind=='from':
        put(UC_X86_REG_XMM7,a);put(UC_X86_REG_XMM6,0);result=run(0x1407c878a,[0x1407c87ad])
    else:raise ValueError(kind)
    return result
def main():
    rng=random.Random(20261014);rows=[]
    specials=[-0.,0.,1.,-1.,2.,-.25,.25,.75,1e-45,-1e-45,float('inf'),-float('inf'),float('nan')]
    for k in ['new','wrap']:
        for a in specials+[rng.uniform(-100,100)for _ in range(50)]:rows.append({'kind':k,'a':bits(a),**scalar(k,a)})
    for k in ['add','subtract','distance']:
        for a,b in [(0,0),(0,1),(1,0),(.75,.25),(.25,.75),(float('nan'),0),(0,float('nan')),(.5,float('inf'))]+[(rng.random(),rng.uniform(-5,5) if k!='distance' else rng.random())for _ in range(60)]:rows.append({'kind':k,'a':bits(a),'b':bits(b),**scalar(k,a,b)})
    for a,b,t in [(a,b,t) for a,b in [(.75,.25),(.25,.75),(0,0),(0,1),(1,0)]for t in [-1,0,.25,.5,1,2,float('nan'),float('inf')]]+[(rng.random(),rng.random(),rng.uniform(-2,3))for _ in range(80)]:
        rows.append({'kind':'lerp','a':bits(a),'b':bits(b),'t':bits(t),**scalar('lerp',a,b,t)})
    for a in specials+[-.001,1.001]+[rng.random()for _ in range(30)]:rows.append({'kind':'from','a':bits(a),**boolean('from',a,0)})
    for a,b in [(0,-0.),(.5,.5),(.25,.5),(float('nan'),float('nan')),(float('nan'),0)]+[(rng.random(),rng.random())for _ in range(25)]:rows.append({'kind':'eq','a':bits(a),'b':bits(b),**boolean('eq',a,b)})
    for a,b,p in [(a,b,p) for a,b in [(.75,.25),(.25,.75),(0,0),(0,1),(1,0),(.5,.5)]for p in [0,.25,.5,.75,1,float('nan')]]+[(rng.random(),rng.random(),rng.random())for _ in range(100)]:rows.append({'kind':'contains','a':bits(a),'b':bits(b),'p':bits(p),**boolean('contains',a,b,p)})
    source=['use super::*;','fn f(b:u32)->f32{f32::from_bits(b)}','fn c(b:u32)->CurveUCircular{CurveUCircular{value:f(b),semantic:PhantomData}}','fn eq_bits(a:f32,b:u32){if f(b).is_nan(){assert!(a.is_nan())}else{assert_eq!(a.to_bits(),b)}}']
    for kind in ['new','wrap','add','subtract','distance','lerp','from','eq','contains']:
        source.append('#[test]fn original_b_circular_'+kind+'(){')
        for row in [r for r in rows if r['kind']==kind]:
            a=str(row['a']);b=str(row.get('b',0));t=str(row.get('t',0));p=str(row.get('p',0))
            expr={'new':'CurveUCircular::<super::super::CurveCoordSemantic2D>::new(f('+a+')).get()','wrap':'CurveUCircular::<super::super::CurveCoordSemantic2D>::wrap_around(f('+a+')).get()','add':'c('+a+').add(f('+b+')).get()','subtract':'c('+a+').subtract(f('+b+')).get()','distance':'c('+a+').distance(c('+b+'))','lerp':'c('+a+').wrapping_lerp(c('+b+'),f('+t+')).get()','from':'CurveU::from(c('+a+')).get()','eq':'c('+a+')==c('+b+')','contains':'c('+a+').wrapping_contains(c('+b+'),c('+p+'))'}[kind]
            if row['panic']:source.append('assert!(std::panic::catch_unwind(||{'+expr+'}).is_err());')
            elif 'value'in row:source.append('assert!('+('' if row['value'] else '!(')+expr+('' if row['value'] else ')')+');')
            else:source.append('eq_bits('+expr+','+str(row.get('bits',row['a']))+');')
        source.append('}')
    (OUT/'circular-native-tests.rs').write_text('\n'.join(source)+'\n',encoding='utf-8')
    payload={'original_sha256':SHA,'execution':'unmodified private inline blocks with explicit caller register bindings; not standalone ABI','whole_game_execution':False,'bridges':['host ucrtbase.dll fmodf'],'cases':rows,'counts':{k:sum(r['kind']==k for r in rows)for k in sorted(set(r['kind']for r in rows))}}
    (OUT/'circular-native-fixtures.json').write_text(json.dumps(payload,indent=2)+'\n',encoding='utf-8');print(json.dumps(payload['counts']))
if __name__=='__main__':main()
