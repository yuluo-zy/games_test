"""Timer's original state producer values plus interval subtraction entries."""
from pathlib import Path
import json,random,struct
import native_spline_rdp as o
from unicorn.x86_const import *
OUT=Path(__file__).parent
def timer(raw_values):
    o.reset();wrapper=o.INPUT;time=o.INPUT+0x100;local=o.OUTPUT;buffer=o.INPUT+0x4000
    o.u.mem_write(wrapper,struct.pack('<Q',time)+bytes(24)+struct.pack('<Q',local))
    o.u.mem_write(local,struct.pack('<4Qif',4,buffer,0,0,1,1/60))
    result=[]
    for raw in raw_values:
        o.u.mem_write(time+0x58,o.pack([raw]))
        sp=o.STACK+0x10008;o.u.reg_write(UC_X86_REG_RSP,sp);o.u.mem_write(sp,struct.pack('<Q',o.STOP));o.u.reg_write(UC_X86_REG_RCX,wrapper)
        o.run(0x141799e70)
        cap,ptr,head,n,warm,prev=struct.unpack('<4QiI',o.u.mem_read(local,40))
        samples=[struct.unpack('<I',o.u.mem_read(ptr+((head+i)%cap)*4,4))[0] for i in range(n)]
        result.append({'raw':o.bits(raw),'result':o.u.reg_read(UC_X86_REG_XMM0)&0xffffffff,'warmup':warm,'previous':prev,'samples':samples})
    return result
def subtract(start,end,cut):
    o.reset();buf=o.INPUT;body=o.OUTPUT;c=o.INPUT+0x1000
    o.u.mem_write(buf,struct.pack('<ffB3x',start,end,False));o.u.mem_write(body,struct.pack('<QQQ',1,buf,1));o.u.mem_write(c,struct.pack('<ffB3x',*cut))
    o.u.reg_write(UC_X86_REG_RCX,body);o.u.reg_write(UC_X86_REG_RDX,c);o.run(0x141fdc350)
    cap,ptr,n=struct.unpack('<3Q',o.u.mem_read(body,24))
    result=[struct.unpack('<IIB3x',o.u.mem_read(ptr+i*12,12)) for i in range(n)]
    return {'range':[o.bits(start),o.bits(end)],'cut':[o.bits(cut[0]),o.bits(cut[1]),bool(cut[2])],'result':result}
rng=random.Random(1301324)
series=[[.03,.03,.0001,.001,.02,.1,.1,.1,0,.2,.016]*4,[rng.uniform(-.1,1) for _ in range(96)]]
fixtures={'timer':[timer(seq) for seq in series],'subtract':[]}
for _ in range(150):
    a=rng.uniform(-5,5);b=a+rng.uniform(0,10);c=rng.uniform(-10,10);d=c+rng.uniform(0,10)
    fixtures['subtract'].append(subtract(a,b,(c,d,bool(rng.randrange(2)))))
for c in [(0,1,False),(0,1,True),(-1,0,False),(-1,0,True),(1,2,False),(1,2,True),(0,0,False),(1,1,False)]:
    fixtures['subtract'].append(subtract(0,1,c))
(OUT/'small-native-fixtures.json').write_text(json.dumps({'original_sha256':'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03','method':o.__doc__,'fixtures':fixtures},indent=2)+'\n',encoding='utf8')
src=OUT.parents[2]/'reconstruction/shared-foundation/crates/utils/src'
text=(src/'timer.rs').read_text(encoding='utf8')
text=text[:text.rfind('\n#[cfg(test)]')] if '\n#[cfg(test)]' in text else text
test='''
#[cfg(test)]
#[allow(clippy::type_complexity)]
mod native_oracle {
    use super::*;
    #[test]fn original_b_filter_sequence() {
        let fixtures:&[&[(u32,u32,i32,u32,&[u32])]]=&[
'''
for seq in fixtures['timer']:
    test+='            &[\n'
    for row in seq:test+=f"                ({row['raw']}, {row['result']}, {row['warmup']}, {row['previous']}, &{row['samples']!r}),\n"
    test+='            ],\n'
test+='''        ];
        for seq in fixtures {
            let mut timer=FilteredWallTime::default();
            for (raw,result,warmup,previous,samples) in *seq {
                assert_eq!(timer.delta_secs(f32::from_bits(*raw)).to_bits(),*result);
                assert_eq!(timer.warmup_remaining,*warmup);
                assert_eq!(timer.previous_delta.to_bits(),*previous);
                assert_eq!(timer.samples().iter().map(|v|v.to_bits()).collect::<Vec<_>>(),*samples);
            }
        }
    }
}
'''
(src/'timer.rs').write_text(text+test,encoding='utf8')
text=(src/'subtractive_1d_boolean.rs').read_text(encoding='utf8')
index=text.rfind('\n#[cfg(test)]')
text=text[:index] if index>=0 and 'mod native_oracle' in text[index:] else text
test='''
#[cfg(test)]
#[allow(clippy::type_complexity)]
mod native_oracle {
    use super::*;
    #[test]fn original_b_intervals() {
        let fixtures:&[([u32;2],(u32,u32,bool),&[(u32,u32,bool)])]=&[
'''
for row in fixtures['subtract']:
    c=row['cut'];test+=f"            ({row['range']!r}, ({c[0]}, {c[1]}, {str(c[2]).lower()}), &["
    test+=','.join(f'({a},{b},{str(bool(flag)).lower()})' for a,b,flag in row['result'])+']),\n'
test+='''        ];
        for (range,cut,expected) in fixtures {
            let mut result=SubtractiveBoolean1D::new_range(f32::from_bits(range[0]),f32::from_bits(range[1]));
            result.subtract(Interval{start:f32::from_bits(cut.0),end:f32::from_bits(cut.1),upper_exclusive:cut.2});
            let actual:Vec<_>=result.result().iter().map(|v|(v.start.to_bits(),v.end.to_bits(),v.upper_exclusive)).collect();
            assert_eq!(actual,*expected);
        }
    }
}
'''
(src/'subtractive_1d_boolean.rs').write_text(text+test,encoding='utf8')
print('timer steps',sum(map(len,fixtures['timer'])),'subtract cases',len(fixtures['subtract']))
