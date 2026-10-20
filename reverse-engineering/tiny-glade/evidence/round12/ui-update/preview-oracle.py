import importlib.util,pathlib,json,struct
p=pathlib.Path('reverse-engineering/tiny-glade/evidence/round12/ui-update-oracle.py');spec=importlib.util.spec_from_file_location('u',p);u=importlib.util.module_from_spec(spec);spec.loader.exec_module(u)
cases=[]
for tag,phase in [(1,1),(2,3),(2,4)]:
 for shape in range(8):
  for preview in range(3):
   for flags in [0,8]:
    kw=dict(tag=tag,phase=phase,action=1,preview_tag=preview,geometry_tag=shape,flags=flags)
    r=u.run(**kw);b=bytes.fromhex(r['brush'][0]);cursor=int.from_bytes(bytes.fromhex(r['cursor'][0])[:4],'little')
    expected_origin=preview==2 and shape in [0,1,6];position=(0.,0.,0.) if preview!=2 else (41.,52.,63.) if shape==1 else (11.,22.,33.) if shape in [0,6] else (1.,2.,3.)
    assert b[8]==expected_origin,(kw,r)
    if preview==2:assert struct.unpack_from('<3f',b,12)==position,(kw,r,position)
    assert b[24]==(7 if preview else 0),(kw,r)
    assert cursor==(2 if flags==8 else 3),(kw,r)
    cases.append(dict(inputs=kw,result=r))
for kw in [dict(tag=2,phase=3,action=1,registry=1),dict(tag=2,phase=4,action=1,registry=1),dict(tag=2,phase=4,action=1,focus=[0,1]),dict(tag=2,phase=4,action=1,focus=[1,0]),dict(tag=1,phase=1,action=2,timer=0.1),dict(tag=1,phase=1,action=2,timer=float('nan'))]:
 result=u.run(**kw)
 if 'timer' in kw:kw['timer_bits']=struct.unpack('<I',struct.pack('<f',kw.pop('timer')))[0]
 cases.append(dict(inputs=kw,result=result))
(u.ROOT/'native-preview-ownership-fixtures.json').write_text(json.dumps(cases,indent=2));print(len(cases),'preview/ownership completed')
