import native_oracle as n
from native_oracle import *
import math
def rust(v):return json.dumps(v,separators=(',',':'))
source=['use super::*;use glam::Vec3;','fn f(b:u32)->f32 {f32::from_bits(b)}','fn eq(a:f32,b:u32){if f(b).is_nan(){assert!(a.is_nan())}else{assert_eq!(a.to_bits(),b)}}']
cases=[];events=None
def callback(c,pc,size,user):
    if pc in (0x14123fab0,0x14123fb70):
        events.append({'kind':'above_to_below' if pc==0x14123fab0 else 'below_to_above','segment':c.reg_read(UC_X86_REG_RDX),'t':c.reg_read(UC_X86_REG_XMM2)&0xffffffff});ret()
cpu.hook_add(UC_HOOK_CODE,callback)
def threshold(heights,value):
    global events
    setup();events=[];cpu.mem_write(DATA+0x1000,struct.pack('<'+'f'*len(heights),*heights));cpu.mem_write(DATA+0x300,struct.pack('<6Q',len(heights),DATA+0x2000,len(heights),len(heights),DATA+0x2000,len(heights)));cpu.mem_write(DATA+0x2000,struct.pack('<'+'f'*len(heights),*[float(i)for i in range(len(heights))]));cpu.mem_write(DATA+0x200,struct.pack('<3Q',128,DATA+0x4000,0));cpu.mem_write(DATA+0x100,struct.pack('<4Q',DATA+0x300,DATA+0x200,0,7));arguments(DATA+0x1000,len(heights));cpu.reg_write(UC_X86_REG_XMM2,bits(value));cpu.reg_write(UC_X86_REG_R9,DATA+0x100);call(0x141fe0750);return events
def abs_eq(a,b,epsilon):
    setup();cpu.mem_write(DATA,struct.pack('<10f',*a));cpu.mem_write(DATA+0x100,struct.pack('<10f',*b));arguments(DATA,DATA+0x100);cpu.reg_write(UC_X86_REG_XMM2,bits(epsilon));call(0x140c9d0d0);return bool(cpu.reg_read(UC_X86_REG_RAX)&255)
def main():
    source.append('#[test]fn original_b_threshold_crossings(){')
    heights_list=[[0],[1],[0,1,0],[0,0,1,0,0],[1,1,1],[-1,0,1,0,-1],[0.5,0.5,0.5],[float('nan'),0,1],[0,float('nan'),1],[0,1,float('nan')]]
    for heights in heights_list:
        for value in (0.0,0.5,1.0):
            expected=threshold(heights,value);cases.append({'kind':'threshold_crossings','heights':[bits(v)for v in heights],'threshold':bits(value),'expected':expected})
            source.append('{let heights='+rust([bits(v)for v in heights])+';let got=split_heightmap::split_heightmap_by_threshold(&heights.map(f),f('+str(bits(value))+'));assert_eq!(got.len(),'+str(len(expected))+');')
            for i,event in enumerate(expected):
                variant='AboveToBelow' if event['kind']=='above_to_below' else 'BelowToAbove'
                source.append('match got['+str(i)+']{split_heightmap::ThresholdCrossing::'+variant+'{segment,t}=>{assert_eq!(segment,'+str(event['segment'])+');eq(t,'+str(event['t'])+');},_=>panic!("wrong crossing direction")}')
            source.append('}')
    source.append('}\n#[test]fn original_b_transform_abs_diff_eq(){')
    for epsilon in (0.0,0.01,float('nan')):
        for field in range(10):
            for value in (0.005,0.02,float('nan')):
                a=[0,0,0,1,1,2,3,4,5,6];b=a.copy();b[field]+=value;expected=abs_eq(a,b,epsilon);cases.append({'kind':'transform_abs_diff_eq','a':[bits(v)for v in a],'b':[bits(v)for v in b],'epsilon':bits(epsilon),'expected':expected})
                source.append('{fn t(b:[u32;10])->crate::components::transform::Transform{let b=b.map(f);crate::components::transform::Transform{r:glam::Quat::from_xyzw(b[0],b[1],b[2],b[3]),t:Vec3::new(b[4],b[5],b[6]),s:Vec3::new(b[7],b[8],b[9])}}assert_eq!(t('+rust([bits(v)for v in a])+').abs_diff_eq(&t('+rust([bits(v)for v in b])+'),f('+str(bits(epsilon))+')),'+str(expected).lower()+');}')
    source.append('}')
    (OUT/'edge-fixtures.json').write_text(json.dumps({'binary_sha256':SHA,'target_execution':False,'threshold_scope':'original classification/crossing event projection; RowTrimmerSink callback bodies are intercepted, full consumer span assembly not validated','cases':cases},indent=2)+'\n',encoding='utf-8');(OUT/'edge_tests.rs').write_text('\n'.join(source)+'\n',encoding='utf-8');print(json.dumps({'cases':len(cases)}))
if __name__=='__main__':main()
