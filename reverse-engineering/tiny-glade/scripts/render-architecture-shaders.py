from pathlib import Path
import struct,json,collections,hashlib
r=Path('D:/game/reverse-engineering/tiny-glade')
s=Path('D:/game/ljxsj_92385/Tiny Glade/compiled-shaders')
g=json.loads((r/'skeleton/_tools/spirv.core.grammar.json').read_text())
ops={i['opcode']:i['opname'] for i in g['instructions']}
enum={i['kind']:{int(v['value'],0) if isinstance(v['value'],str) else v['value']:v['enumerant'] for v in i.get('enumerants',[])} for i in g['operand_kinds']}
def st(a):return struct.pack('<'+'I'*len(a),*a).split(b'\0')[0].decode('utf8',errors='replace')
result=[]
for p in sorted(s.glob('*.bin')):
 if not any(p.name.startswith(x) for x in ['_nani_cull_and_bucket.cs','_nani_cull_and_bucket_roof_tile','_nani_draw_indexed','_gi_','_ssgi_','_tgsr.','_taa.','_extract_half','_calculate_reprojection']):continue
 b=p.read_bytes();cursor=0;modules=[]
 while True:
  at=b.find(b'\x03\x02\x23\x07',cursor)
  if at<0:break
  n=struct.unpack_from('<I',b,at-4)[0];payload=b[at:at+4*n];w=struct.unpack('<'+'I'*n,payload)
  assert len(payload)==4*n and w[0]==0x07230203
  i=5;names={};deco=collections.defaultdict(list);variables={};entries=[];modes=[];caps=[];count=collections.Counter()
  while i<n:
   length,op=w[i]>>16,w[i]&65535;assert length>0 and i+length<=n
   a=w[i+1:i+length];name=ops.get(op,str(op));count[name]+=1;i+=length
   if op==5:names[a[0]]=st(a[1:])
   elif op==71:deco[a[0]].append({'kind':enum['Decoration'].get(a[1],str(a[1])),'args':list(a[2:])})
   elif op==59:variables[a[1]]={'type':a[0],'storage':enum['StorageClass'].get(a[2],str(a[2]))}
   elif op==15:entries.append({'stage':enum['ExecutionModel'].get(a[0],str(a[0])),'entry':st(a[2:])})
   elif op in [16,331]:modes.append({'mode':enum['ExecutionMode'].get(a[1],str(a[1])),'args':list(a[2:])})
   elif op==17:caps.append(enum['Capability'].get(a[0],str(a[0])))
  bindings=[{'name':names.get(k),'storage':v['storage'],'decorations':deco[k]} for k,v in variables.items() if any(d['kind'] in ['Binding','DescriptorSet'] for d in deco[k]) or v['storage']=='PushConstant']
  modules.append({'offset':at,'sha256':hashlib.sha256(payload).hexdigest(),'entries':entries,'execution_modes':modes,'capabilities':caps,'bindings':bindings,'opcode_histogram':dict(count),'debug_names':[v for v in names.values() if any(x in v.lower() for x in ['bvh','ray','radiance','instance','indirect','gbuf','history','depth','normal','accel'])]})
  cursor=at+4*n
 result.append({'file':str(p),'sha256':hashlib.sha256(b).hexdigest(),'modules':modules})
out=r/'evidence/render-architecture/shader-reflection.json';out.write_text(json.dumps({'files':result,'limitations':['SPIR-V declarations and opcode presence only; no runtime bindings or complete algorithm recovery']},indent=2),encoding='utf8')
for f in result:
 print(Path(f['file']).name)
 for m in f['modules']:print(' ',m['entries'],m['execution_modes'],'caps',m['capabilities'],'bindings',[v['name'] for v in m['bindings']],'names',m['debug_names'][:24])
