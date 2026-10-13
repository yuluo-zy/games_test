"""Execute full original splitter and both original RowTrimmerSink callbacks."""
from pathlib import Path
import sys, json, struct, random
OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[2]
sys.path.insert(0, str(ROOT / 'evidence/shared-foundation-2026-10-13/geometry-curves'))
import native_oracle as n
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import UC_X86_REG_RDX, UC_X86_REG_XMM2, UC_X86_REG_R9
trace = []
def observe(cpu, pc, size, user):
    if pc in (0x14123fab0, 0x14123fb70):
        trace.append({'kind': 'leave' if pc == 0x14123fab0 else 'enter',
                      'segment': cpu.reg_read(UC_X86_REG_RDX),
                      't': cpu.reg_read(UC_X86_REG_XMM2) & 0xffffffff})
n.cpu.hook_add(UC_HOOK_CODE, observe)
def run(heights, coords, threshold):
    trace.clear(); n.setup()
    n.cpu.mem_write(n.DATA + 0x1000, struct.pack('<' + 'f'*len(heights), *heights))
    n.cpu.mem_write(n.DATA + 0x2000, struct.pack('<' + 'f'*len(coords), *coords))
    n.cpu.mem_write(n.DATA + 0x300, struct.pack('<6Q', len(coords), n.DATA+0x2000,
                      len(coords), len(coords), n.DATA+0x2000, len(coords)))
    n.cpu.mem_write(n.DATA + 0x200, struct.pack('<3Q', 128, n.DATA+0x4000, 0))
    n.cpu.mem_write(n.DATA + 0x100, struct.pack('<3Q2f', n.DATA+0x300, n.DATA+0x200, 0, 2, 3))
    n.arguments(n.DATA+0x1000, len(heights)); n.cpu.reg_write(UC_X86_REG_XMM2, n.bits(threshold))
    n.cpu.reg_write(UC_X86_REG_R9, n.DATA+0x100)
    try:
        n.call(0x141fe0750)
        count=n.u64(n.DATA+0x210)
        assert count<=128
        return {'regions':[n.floats(n.DATA+0x4000+i*32+24,2) for i in range(count)], 'callbacks':trace.copy()}
    except RuntimeError as e:
        if 'original panic' not in str(e): raise
        return {'panic':str(e),'callbacks':trace.copy()}
rows=[]
paths=[[0],[1],[0,1,0],[1,1,1],[0,0,1,1,0,1],[1,0,0,1],[-1,0,1,0,-1],
       [float('nan')],[float('nan'),float('nan')],[0,float('nan'),1],[float('nan'),0],
       [0,float('inf')],[float('inf'),0]]
rng=random.Random(20261013)
paths += [[rng.uniform(-3,3) for _ in range(rng.randint(2,12))] for _ in range(45)]
src=['use super::*;', 'fn f(b:u32)->f32{f32::from_bits(b)}', '#[test]fn full_original_b_region_and_callback_output(){']
for h in paths:
    coords=[0.13+0.74*i/max(1,len(h)-1) for i in range(len(h))]
    for threshold in (0.,0.5,1.):
        expected=run(h,coords,threshold)
        row={'heights':[n.bits(x) for x in h], 'point_u':[n.bits(x) for x in coords],
             'threshold':n.bits(threshold), **expected};rows.append(row)
        if 'panic' in expected:
            src.append('{let h='+json.dumps(row['heights'])+';let u='+json.dumps(row['point_u'])+';assert!(std::panic::catch_unwind(||split_heightmap_regions(&h.map(f),&u.map(f),f('+str(row['threshold'])+'))).is_err());}')
            continue
        src.append('{let h='+json.dumps(row['heights'])+';let u='+json.dumps(row['point_u'])+';let got=split_heightmap_regions(&h.map(f),&u.map(f),f('+str(row['threshold'])+'));')
        src.append('let expected:Vec<[u32;2]>=vec!'+json.dumps(expected['regions'])+';assert_eq!(got.iter().map(|r|[r.start_u.to_bits(),r.end_u.to_bits()]).collect::<Vec<_>>(),expected);}')
src.append('}')
# Test callback order independently from the concrete receiver/payload.
src += ['#[test]fn callbacks_keep_original_order(){',
        'struct Trace(Vec<(u8,usize,u32)>);impl SplitReceiver for Trace{fn above_sample(&mut self,_:usize){}fn below_to_above(&mut self,i:usize,t:f32){self.0.push((0,i,t.to_bits()));}fn above_to_below(&mut self,i:usize,t:f32){self.0.push((1,i,t.to_bits()));}}']
for row in rows:
    if 'panic' in row:continue
    exp=','.join(f'({0 if e["kind"]=="enter" else 1},{e["segment"]},{e["t"]})' for e in row['callbacks'])
    src.append('{let mut r=Trace(vec![]);split_heightmap_into(&'+json.dumps(row['heights'])+'.map(f),f('+str(row['threshold'])+'),&mut r);let expected:Vec<(u8,usize,u32)>=vec!['+exp+'];assert_eq!(r.0,expected);}')
src += ['}', '#[test]fn original_receiver_rejects_nan_crossings(){assert!(std::panic::catch_unwind(||split_heightmap_regions(&[0.,f32::NAN,1.],&[0.,0.5,1.],0.)).is_err());}',
        '#[test]fn empty_heightmap_and_short_coordinate_view_reject(){assert!(std::panic::catch_unwind(||split_heightmap_regions(&[],&[],0.)).is_err());assert!(std::panic::catch_unwind(||split_heightmap_regions(&[1.],&[],0.)).is_err());}']
(OUT/'heightmap-fixtures.json').write_text(json.dumps({'binary_sha256':n.SHA,'callbacks_executed_original':True,'cases':rows},indent=2)+'\n',encoding='utf-8')
(OUT/'heightmap_tests.rs').write_text('\n'.join(src)+'\n',encoding='utf-8')
print(json.dumps({'cases':len(rows),'native_panics':sum('panic' in r for r in rows)}))
