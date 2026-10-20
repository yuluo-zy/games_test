from pathlib import Path
here=Path(__file__).parent
visual=here.parent/'decorator-visual'
script=(visual/'door-stairs-native.py').read_text(encoding='utf-8')
script=script.replace('range(2000)','range(192)')
begin=script.index('    # Dense gate')
end=script.index('    state.clear()',begin)
script=script[:begin]+'''    case.update(q=[0.,0.,0.,1.],p=[1.,0.,2.],bottom=-1.,top=1.,width=2.,heights=[2.,.5,.5,.5],coverage=1.,old=bool(i%2))
    # All four terrain NaNs, all signs of infinity/zero, quaternion and width.
    lane=(i//12)%8
    value=[float('nan'),float('inf'),float('-inf'),0.,-0.,-2.,.1,1.5][lane]
    target=i%12
    if target<4: case['heights'][target]=value
    elif target==4: case['coverage']=value
    elif target==5: case['width']=value
    elif target==6: case['bottom']=value
    elif target==7: case['top']=value
    elif target==8: case['p'][1]=value
    elif target==9: case['q']=[0.,value,0.,1.]
    elif target==10: case['p'][0]=value
    else: case['q']=[0.,0.,value,1.]
    q=case['q']
'''+script[end:]
(here/'door-stairs-independent-native.py').write_text(script,encoding='utf-8')
harness=(visual/'build-door-stairs-harness.py').read_text(encoding='utf-8')
(here/'build-door-stairs-independent-harness.py').write_text(harness,encoding='utf-8')
print('prepared independent 192-case all-lane NaN/infinity/zero probe')
