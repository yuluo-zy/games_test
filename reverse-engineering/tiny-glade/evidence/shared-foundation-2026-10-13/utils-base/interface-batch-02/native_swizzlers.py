"""Run original adapters, keeping all f32 payload bits; no glam reimplementation."""
from pathlib import Path
import hashlib,json,random,struct,sys
sys.path.insert(0,str(Path(__file__).parent.parent))
import native_spline_rdp as o
from unicorn.x86_const import *
OUT=Path(__file__).parent;rows=[];rng=random.Random(131302)
inputs=[[0,0x80000000,0x7fc12345,0x7f800000],[0x7f812345,0x7f7fffff,0xff800000,0xffc12345]]+[[rng.getrandbits(32) for _ in range(4)] for _ in range(62)]
for tag,address in [('x0y',0x140c9cdc0),('xvy',0x140c9cde0),('xvz',0x140c90010),('x0z',0x140c90030)]:
    for input in inputs:
        o.reset();o.u.mem_write(o.INPUT,struct.pack('<4I',*input));o.u.reg_write(UC_X86_REG_RCX,o.OUTPUT);o.u.reg_write(UC_X86_REG_RDX,o.INPUT);o.u.reg_write(UC_X86_REG_XMM2,input[3]);o.run(address)
        result=list(struct.unpack('<3I',o.u.mem_read(o.OUTPUT,12)))
        rows.append({'interface':tag,'input':input,'result':result})
transport=[]
for n in [0,1,2,3,8,64]:
    o.reset();o.u.reg_write(UC_X86_REG_RCX,o.INPUT);o.u.reg_write(UC_X86_REG_RDX,n);o.run(0x14067dd80)
    result=[o.u.reg_read(UC_X86_REG_RAX),o.u.reg_read(UC_X86_REG_RDX)]
    assert result==[o.INPUT,n]
    transport.append({'pointer':o.INPUT,'length':n,'result':result,'scope':'borrowed transport only; projected consumer semantics unverified'})
(OUT/'swizzler-native-fixtures.json').write_text(json.dumps({'original_sha256':hashlib.sha256(o.raw).hexdigest(),'coordinate_cases':rows,'transport_cases':transport,'method':'Original B instruction execution; adapter code contains only loads/stores. Return type/consumer representation of as_vec2 not inferred from shared identity entry.'},indent=2)+'\n',encoding='utf8')
test='''use super::*;
#[test]fn original_coordinate_adapters() {
    let cases:&[(u8,[u32;4],[u32;3])]=&[
'''
for row in rows:test+=f"        ({['x0y','xvy','xvz','x0z'].index(row['interface'])},{row['input']!r},{row['result']!r}),\n"
test+='''    ];
    for (tag,input,expected) in cases {
        let v=input.map(f32::from_bits);
        let result=match tag {0=>Vec2::new(v[0],v[1]).x0y(),1=>Vec2::new(v[0],v[1]).xvy(v[3]),2=>Vec3::new(v[0],v[1],v[2]).xvz(v[3]),3=>Vec3::new(v[0],v[1],v[2]).x0z(),_=>unreachable!()};
        assert_eq!(result.to_array().map(f32::to_bits),*expected);
    }
}
#[test]fn borrowed_transport_preserves_original_storage() {
    let source=[Vec3::new(1.0,2.0,3.0);64];
    for n in [0,1,2,3,8,64] {
        let slice=&source[..n];let view=slice.as_vec2().as_source();
        assert_eq!(view.as_ptr(),slice.as_ptr());assert_eq!(view.len(),slice.len());assert_eq!(view,slice);
    }
}
'''
(OUT/'swizzler_tests.rs').write_text(test,encoding='utf8')
print('coordinate cases',len(rows),'transport cases',len(transport))
