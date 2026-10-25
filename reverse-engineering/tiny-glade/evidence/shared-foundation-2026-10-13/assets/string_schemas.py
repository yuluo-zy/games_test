import pathlib
b=pathlib.Path(r'D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne').read_bytes()
out=[]
for name in ['PrefabConfig','PrefabPartConfig','PrefabRenderConfig','PrefabRtConfig','AudioTriggerConfig','MeshProcessorConfig','TextureGroupConfig']:
 start=0
 while (i:=b.find(name.encode(),start))>=0:
  q=b[max(0,i-150):i+250]; out.append(name+' '+hex(i)+' '+repr(q)); start=i+len(name)
pathlib.Path(__file__).with_name('string-schemas.txt').write_text('\n'.join(out));print('\n'.join(out[:18]))
