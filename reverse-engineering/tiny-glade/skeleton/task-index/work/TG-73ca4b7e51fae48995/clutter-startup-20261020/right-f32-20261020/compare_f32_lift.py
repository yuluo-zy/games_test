"""Bounded scalar lift of B 1412898e0; not Rust/native execution."""
from pathlib import Path
import hashlib
import json
import math
import re
import struct

ROOT = Path('/Users/yuluo/Desktop/game/games_test')
WORK = Path(__file__).parent

def f32(value):
    try:
        return struct.unpack('<f', struct.pack('<f', value))[0]
    except OverflowError:
        return math.copysign(math.inf, value)

def from_bits(value):
    return struct.unpack('<f', struct.pack('<I', value))[0]

def describe(value):
    return {'bits': struct.unpack('<I', struct.pack('<f', value))[0],
            'value': 'NaN' if math.isnan(value) else str(value)}

def divide(a, b):
    if b == 0.0:
        return math.nan if a == 0.0 or math.isnan(a) else math.copysign(math.inf, a*b)
    return f32(a/b)

def length(a, b):
    x = f32(b[0]-a[0])
    z = f32(b[1]-a[1])
    return f32(math.sqrt(f32(f32(z*z)+f32(x*x))))

def original(points, segment, t, distance):
    if not distance >= 0.0:
        return {'panic': True}
    if segment+1 >= len(points):
        return {'segment': segment, 't': describe(t)}
    edge_length = length(points[segment], points[segment+1])
    residual = f32(f32(f32(1.0-t)*edge_length)-distance)
    while True:
        if residual >= 0.0:
            return {'segment': segment, 't': describe(f32(1.0-divide(residual,edge_length)))}
        t = 1.0
        if segment+2 >= len(points):
            return {'segment': segment, 't': describe(t)}
        segment += 1
        edge_length = length(points[segment], points[segment+1])
        residual = f32(residual+edge_length)

def old_right(points, segment, t, distance):
    if not distance >= 0.0:
        return {'panic': True}
    if distance == 0.0:
        return {'segment': segment, 't': describe(t)}
    remaining = float(distance)
    updated = None
    for i in range(segment, len(points)-1):
        squared_length = length(points[i],points[i+1])
        x = f32(points[i+1][0]-points[i][0])
        z = f32(points[i+1][1]-points[i][1])
        squared = f32(f32(x*x)+f32(z*z))
        if (math.isfinite(squared) and abs(squared) >= from_bits(0x00800000)) or points[i] == points[i+1]:
            edge_length = float(squared_length)
        else:
            edge_length = math.hypot(float(points[i+1][0])-points[i][0],float(points[i+1][1])-points[i][1])
        if edge_length == 0.0:
            if remaining == 0.0:
                break
            updated = {'segment': i, 't': describe(1.0)}
            continue
        start_t = float(t) if updated is None else 0.0
        available = (1.0-start_t)*edge_length
        if remaining <= available:
            updated = {'segment': i, 't': describe(f32(min(start_t+remaining/edge_length,1.0)))}
            break
        updated = {'segment': i, 't': describe(1.0)}
        remaining -= available
    return updated or {'segment': segment, 't': describe(t)}

def fixture_checks():
    source = ROOT/'tiny-glade/crates/utils/tests/oracle/shared-foundation-2026-10-13/geometry-curves/interface-batch-02/slice-native-tests.rs'
    text = source.read_text()
    results=[]
    for block in re.finditer(r'\{let p=points\(&(?P<points>\[.*?\])\);\n(?P<body>.*?)\n\}', text, re.S):
        body = block['body']
        if 'slice.extend_right(' not in body:
            continue
        points = json.loads(block['points'])
        points = [(from_bits(v[0]),from_bits(v[2])) for v in points]
        view = re.search(r'LoopRange::new_loop\((-?\d+),(\d+),(\d+)\)', body)
        if view:
            start,end,period = map(int,view.groups())
            points = [points[i % period] for i in range(start,end)]
        coords = re.search(r'c\(\[(\d+),(\d+)\]\),c\(\[(\d+),(\d+)\]\)',body)
        distance = from_bits(int(re.search(r'slice.extend_right\(f\((\d+)\)\)',body)[1]))
        seg,t = int(coords[3]),from_bits(int(coords[4]))
        expected_panic = 'catch_unwind' in body
        expected = re.search(r'eq_coord\(slice.end\(\),\[(\d+),(\d+)\]\)',body)
        expected = {'panic':True} if expected_panic else {'segment':int(expected[1]),'t':describe(from_bits(int(expected[2])))}
        native = original(points,seg,t,distance)
        old = old_right(points,seg,t,distance)
        def compares(actual):
            if expected_panic:
                return actual == expected
            if 'panic' in actual or actual['segment'] != expected['segment']:
                return False
            a,e = from_bits(actual['t']['bits']),from_bits(expected['t']['bits'])
            return math.isnan(a) if math.isnan(e) else abs(a-e) <= 8.0*from_bits(0x34000000)
        results.append({'line':text.count('\n',0,block.start())+1,'expected':expected,'original_lift':native,'old_source_lift':old,
                        'original_exact_expected':native==expected,'original_fixture_tolerance':compares(native),
                        'old_fixture_tolerance':compares(old)})
    return {'source':str(source),'sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'right_cases':len(results),
            'original_exact_matches':sum(v['original_exact_expected'] for v in results),
            'original_fixture_tolerance_matches':sum(v['original_fixture_tolerance'] for v in results),
            'old_fixture_tolerance_matches':sum(v['old_fixture_tolerance'] for v in results),
            'mismatching_original_cases':[v for v in results if not v['original_fixture_tolerance']],
            'bit_changes_from_old_source':[v for v in results if v['old_source_lift']!=v['original_lift']]}

cases=[]
maximum=from_bits(0x7f7fffff)
for label,points,distance in [
    ('tiny_1e-30',[(0.0,0.0),(f32(1e-30),0.0)],f32(f32(1e-30)*0.5)),
    ('min_positive',[(0.0,0.0),(from_bits(0x00800000),0.0)],f32(from_bits(0x00800000)*0.5)),
    ('large_1e20',[(0.0,0.0),(f32(1e20),0.0)],f32(f32(1e20)*0.5)),
    ('maximum',[(0.0,0.0),(maximum,0.0)],f32(maximum*0.5)),
    ('opposite_maximum',[(-maximum,0.0),(maximum,0.0)],maximum),
    ('tiny_then_regular',[(0.0,0.0),(f32(1e-30),0.0),(f32(1e-30),1.0)],f32(5e-31)),
    ('zero_distance_zero_edge',[(0.0,0.0),(0.0,0.0),(1.0,0.0),(1.0,0.0)],0.0),
]:
    cases.append({'case':label,'points':[[describe(x),describe(z)] for x,z in points],
                  'distance':describe(distance),'old_source_lift':old_right(points,0,0.0,distance),
                  'original_lift':original(points,0,0.0,distance)})
output={'kind':'bounded_python_f32_lift_not_Rust_or_native_execution',
        'body':'B 1412898e0/921B, remapped Vec3 -> Vec2',
        'source_scope':'original arithmetic only; no stopping-predicate execution tested',
        'properties':cases,'local_fixture_comparison':fixture_checks()}
(WORK/'numerical-observations.json').write_text(json.dumps(output,indent=2,allow_nan=False)+'\n')
print(json.dumps({'properties':[{k:v for k,v in c.items() if k not in ('points','distance')} for c in cases],
                  'fixture_summary':{k:v for k,v in output['local_fixture_comparison'].items() if k not in ('bit_changes_from_old_source','mismatching_original_cases')},
                  'fixture_original_mismatches':len(output['local_fixture_comparison']['mismatching_original_cases'])},indent=2))
