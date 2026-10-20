from native_picker_oracle import *
cases=json.loads((ROOT/'native-picker-fixtures.json').read_text())
def add(name,w,**kw):cases.append(dict(name=name,widgets=[dict(id=i,bytes=b.hex(),extra=e)for i,b,e in w],inputs=kw,result=run(w,**kw)))
for z in [-3,-1,0,1,3]:
 add(f'sphere-z{z}',[(11,widget(0,{0x8c:[1],0x90:[0,0,z]}),None)])
add('sphere-zero-direction',[(11,widget(0,{0x8c:[1],0x90:[0,0,3]}),None)],direction=(0,0,0))
add('sphere-cloud-empty',[(12,widget(1,{0x8c:[1]}),[])])
add('sphere-cloud-tie',[(12,widget(1,{0x8c:[1]}),[(0,0,3),(0,0,3)])])
add('sphere-cloud-behind',[(12,widget(1,{0x8c:[1]}),[(0,0,-3),(0,0,3)])])
add('triangles-empty',[(13,widget(2,{}),[])])
add('triangles-tie',[(13,widget(2,{}),[(-1,-1,2,1,-1,2,0,1,2),(-1,-1,2,1,-1,2,0,1,2)])])
for x in [-1.001,-1,0,1,1.001]:
 add(f'screen-edge{x}',[(14,widget(3,{0x8c:[.25],0x90:[-1,-1,1,1]}),None)],screen=(x,0))
for pos in [None,(5,4,10),(5.001,4,10),(5,4,10.001),(-5,4,-10)]:
 add(f'canvas-edge{pos}',[(15,widget(4,{0x8c:[10,20]},layer=2),None)],terrain=pos)
for x in [0,1.999,2,2.001]:
 add(f'plane-edge{x}',[(16,widget(6,{0x8c:[0,0,3,0,0,-1,2]}),None)],origin=(x,0,0))
add('plane-parallel',[(16,widget(6,{0x8c:[0,0,3,0,0,-1,2]}),None)],direction=(1,0,0))
add('plane-behind',[(16,widget(6,{0x8c:[0,0,-3,0,0,-1,2]}),None)])
for direction in [(0,0,2),(.1,.2,1),(-.3,.1,1)]:
 add(f'sphere-direction{direction}',[(17,widget(0,{0x8c:[2],0x90:[0,0,3]}),None)],direction=direction)
(ROOT/'native-picker-all-fixtures.json').write_text(json.dumps(cases,indent=2),encoding='utf8');print('original cases',len(cases))
