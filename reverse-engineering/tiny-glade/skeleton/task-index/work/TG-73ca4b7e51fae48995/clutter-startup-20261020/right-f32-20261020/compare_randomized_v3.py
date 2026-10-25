"""Evaluate every existing randomized right input using the bounded scalar lift."""
from pathlib import Path
import json
import math

WORK=Path(__file__).parent
namespace={'__file__':str(WORK/'compare_f32_lift_v2.py')}
exec((WORK/'compare_f32_lift_v2.py').read_text().split('\ncases=[]')[0],namespace)
f32=namespace['f32']; describe=namespace['describe']; original=namespace['original']; from_bits=namespace['from_bits']

seed=0x02e75a31861f4221
def sample():
    global seed
    seed=(seed*6364136223846793005+1442695040888963407)&((1<<64)-1)
    return f32(f32(f32((seed>>40)/float(1<<24))-.5)*20.0)

def arc(points, coord):
    segment,t=coord['segment'],from_bits(coord['t']['bits'])
    return sum(math.dist(a,b) for a,b in zip(points[:segment],points[1:segment+1]))+t*math.dist(points[segment],points[segment+1])

zero_changes=[];nan_cases=[];finite_arc_failures=[];split_failures=[]
zero_expected_bits=[]
max_arc_error=0.0;max_split_error=0.0;cases=0
for case in range(64):
    zero_expected_bits.append([])
    points=[(0.0,0.0)]
    for index in range(12):
        previous=points[-1]
        points.append(previous if (case+index)%5==0 else (f32(previous[0]+sample()),f32(previous[1]+sample())))
    total=sum(math.dist(a,b)for a,b in zip(points,points[1:]))
    for t in map(f32,[0.0,1e-8,.2,.5,1.0]):
        segment=case%12;coord={'segment':segment,'t':describe(t)}
        origin=arc(points,coord)
        for distance in map(f32,[0.0,1e-6,.1,3.0,18.0,1e5]):
            cases+=1
            actual=original(points,segment,t,distance)
            split=original(points,segment,t,f32(distance*.5))
            split=original(points,split['segment'],from_bits(split['t']['bits']),f32(distance*.5))
            record={'case':case,'segment':segment,'t':describe(t),'distance':describe(distance),'original':actual,'split_twice':split}
            if distance==0.0:zero_expected_bits[case].append(actual['t']['bits'])
            if distance==0.0 and actual!=coord:zero_changes.append(record)
            right=arc(points,actual);split_right=arc(points,split)
            if math.isnan(right):
                record['zero_length_initial_edge']=points[segment]==points[segment+1]
                nan_cases.append(record)
            else:
                error=abs(right-min(origin+distance,total));max_arc_error=max(max_arc_error,error)
                if not error<3e-5:finite_arc_failures.append({**record,'error':error})
                error=abs(split_right-right);max_split_error=max(max_split_error,error)
                if not error<3e-5:split_failures.append({**record,'error':error})

out={'kind':'bounded_Python_f32_lift_not_Rust_or_original_execution','existing_randomized_inputs':cases,
     'zero_distance_expected_t_bits_case_then_t':zero_expected_bits,
     'zero_distance_exact_identity_changes':zero_changes,'nan_cases':nan_cases,
     'finite_single_arc_max_error':max_arc_error,'finite_single_arc_failures_at_existing_3e-5':finite_arc_failures,
     'finite_split_arc_max_error':max_split_error,'finite_split_arc_failures_at_existing_3e-5':split_failures,
     'zero_distance_in_normal_1000_edge_t1e-8':original([(0.0,0.0),(1000.0,0.0)],0,f32(1e-8),0.0),
     'distance5e-6_in_normal_1000_edge_t1e-8':original([(0.0,0.0),(1000.0,0.0)],0,f32(1e-8),f32(5e-6))}
(WORK/'randomized-observations-v3.json').write_text(json.dumps(out,indent=2,allow_nan=False)+'\n')
print(json.dumps({k:(len(v) if isinstance(v,list) else v)for k,v in out.items()},indent=2))
