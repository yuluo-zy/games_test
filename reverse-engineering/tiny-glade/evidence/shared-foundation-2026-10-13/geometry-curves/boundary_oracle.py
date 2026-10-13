import native_oracle as n
from native_oracle import *
import ctypes,math,random
lib=ctypes.CDLL('ucrtbase.dll')
functions={0x142923cf0:('_hypotf',2),0x142923e20:('atan2f',2),0x142923f10:('fmodf',2),0x142923e60:('ceilf',1)}
for iat in pe.DIRECTORY_ENTRY_IMPORT:
    for item in iat.imports:
        name=item.name.decode() if item.name else ''
        if name in ('sinf','cosf','fmodf','atan2f','ceilf','_hypotf'):
            # Patch only IAT dispatch to a dedicated trap address; original
            # private .text bytes remain untouched.
            address=STOP+0x100+len(functions)*8;functions[address]=(name,2 if name in ('fmodf','atan2f','_hypotf') else 1);cpu.mem_write(item.address,struct.pack('<Q',address))
for address,(name,count) in list(functions.items()):
    fn=getattr(lib,name);fn.argtypes=[ctypes.c_float]*count;fn.restype=ctypes.c_float;functions[address]=(fn,count)
def math_hook(c,pc,size,user):
    if pc in functions:
        fn,count=functions[pc];args=[struct.unpack('<f',struct.pack('<I',c.reg_read(r)&0xffffffff))[0]for r in (UC_X86_REG_XMM0,UC_X86_REG_XMM1)[:count]];c.reg_write(UC_X86_REG_XMM0,bits(fn(*args)));ret()
cpu.hook_add(UC_HOOK_CODE,math_hook)
def encoded(x):return [bits(v)for v in x]
def rust(v):return json.dumps(v,separators=(',',':'))
source=['use super::*;use glam::Vec2;','fn f(b:u32)->f32 {f32::from_bits(b)}','fn v(b:[u32;2])->Vec2 {Vec2::new(f(b[0]),f(b[1]))}','fn eq(a:f32,b:u32) {if f(b).is_nan(){assert!(a.is_nan())}else{assert_eq!(a.to_bits(),b)}}']
cases=[]
def blob(a,values):cpu.mem_write(a,struct.pack('<'+'f'*len(values),*values))
def circle(value,u,point):
    setup();blob(DATA,value)
    def run(a):arguments(DATA);cpu.reg_write(UC_X86_REG_XMM1,bits(u));call(a);return [cpu.reg_read(r)&0xffffffff for r in (UC_X86_REG_XMM0,UC_X86_REG_XMM1)]
    pos=run(0x140c975a0);normal=run(0x140c978e0);tangent=run(0x140c97660)
    arguments(DATA);cpu.reg_write(UC_X86_REG_XMM1,bits(point[0]));cpu.reg_write(UC_X86_REG_XMM2,bits(point[1]));call(0x140c976f0);near=cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff
    return {'position':pos,'normal':normal,'tangent':tangent,'near_u':near}
def rect(value,u,other):
    setup();blob(DATA,value);blob(DATA+0x100,other)
    arguments(DATA);cpu.reg_write(UC_X86_REG_XMM1,bits(u));call(0x140c99800);pos=[cpu.reg_read(r)&0xffffffff for r in (UC_X86_REG_XMM0,UC_X86_REG_XMM1)]
    arguments(DATA,DATA+0x100);call(0x140c9b420);overlaps=bool(cpu.reg_read(UC_X86_REG_RAX)&0xff)
    return {'position':pos,'overlaps':overlaps}
def main():
    rng=random.Random(88990);source.append('#[test]fn original_b_circle_boundary(){')
    for i in range(80):
        value=[rng.uniform(-10,10),rng.uniform(-10,10),rng.uniform(0.1,5),rng.uniform(-10,10)];u=rng.choice([0,0.5,1])if i<8 else rng.random();point=value[:2] if i%10==0 else [rng.uniform(-10,10),rng.uniform(-10,10)];expected=circle(value,u,point);cases.append({'kind':'circle','value':encoded(value),'u':bits(u),'point':encoded(point),'expected':expected})
        source.append('{let c=Circle2d::new_rotated(v('+rust(encoded(value[:2]))+'),f('+str(bits(value[2]))+'),f('+str(bits(value[3]))+'));let u=f('+str(bits(u))+');for (a,e)in c.get_pos_at_u(u).to_array().into_iter().zip('+rust(expected['position'])+'){eq(a,e)}for (a,e)in c.get_normal_at_u(u).to_array().into_iter().zip('+rust(expected['normal'])+'){eq(a,e)}for (a,e)in c.get_tangent_at_u(u).to_array().into_iter().zip('+rust(expected['tangent'])+'){eq(a,e)}eq(c.get_closest_u_from_pos(v('+rust(encoded(point))+')),'+str(expected['near_u'])+');}')
    source.append('}\n#[test]fn original_b_rectangle_boundary(){')
    for i in range(80):
        value=[1,0,rng.uniform(-2,2),rng.uniform(-2,2),rng.uniform(0.1,5),rng.uniform(0.1,5)];other=[0.6,0.8,rng.uniform(-2,2),rng.uniform(-2,2),rng.uniform(0.1,5),rng.uniform(0.1,5)];u=rng.choice([0,0.5,1])if i<8 else rng.random();expected=rect(value,u,other);cases.append({'kind':'rectangle','value':encoded(value),'other':encoded(other),'u':bits(u),'expected':expected})
        source.append('{let r=Rectangle2d::new(v('+rust(encoded(value[2:4]))+'),v('+rust(encoded(value[4:]))+'),v('+rust(encoded(value[:2]))+'));let other=Rectangle2d::new(v('+rust(encoded(other[2:4]))+'),v('+rust(encoded(other[4:]))+'),v('+rust(encoded(other[:2]))+'));let p=r.get_pos_at_u(f('+str(bits(u))+'));for (a,e)in p.to_array().into_iter().zip('+rust(expected['position'])+'){eq(a,e)}assert_eq!(r.overlaps_rectangle2d(other),'+str(expected['overlaps']).lower()+');}')
    source.append('}')
    (OUT/'boundary-fixtures.json').write_text(json.dumps({'binary_sha256':SHA,'target_execution':False,'bridges':['original ucrt math imports via host ucrtbase.dll','allocator/free/realloc','memcpy/memmove/memset'],'cases':cases},indent=2)+'\n',encoding='utf-8');(OUT/'boundary_tests.rs').write_text('\n'.join(source)+'\n',encoding='utf-8');print(json.dumps({'cases':len(cases)}))
if __name__=='__main__':main()
