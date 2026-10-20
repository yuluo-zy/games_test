import pathlib,struct,json,hashlib
base=pathlib.Path('reverse-engineering/tiny-glade');p=base/'evidence/render-passes/shaders/oracle-bin/_final_post.raster.hlsl_b903e43ffb3da915.bin';b=p.read_bytes();start=b.find(bytes.fromhex('03022307'),24);count=struct.unpack_from('<I',b,start-4)[0];w=struct.unpack_from('<%dI'%count,b,start);ops=[];names={};members={};constants={};i=5
while i<len(w):
 wc=w[i]>>16;op=w[i]&65535;a=list(w[i+1:i+wc]);assert wc>0 and i+wc<=len(w);ops.append((i,op,a))
 if op==5:names[a[0]]=struct.pack('<%dI'%(len(a)-1),*a[1:]).split(b'\0')[0].decode(errors='replace')
 if op==6:members[(a[0],a[1])]=struct.pack('<%dI'%(len(a)-2),*a[2:]).split(b'\0')[0].decode(errors='replace')
 if op==43:constants[a[1]]=a[2:]
 if op==44:constants[a[1]]={'constituents':a[2:]}
 i+=wc
hits=[]
for key,name in members.items():
 if 'is_input_valid' in name:hits.append({'member':key,'name':name})
for ident,name in names.items():
 if 'is_input_valid' in name:hits.append({'id':ident,'name':name})
# Read OpSelect with float3 constant 0.18 vector and resolve conditions backwards.
scalar18=[ident for ident,value in constants.items() if value==[struct.unpack('<I',struct.pack('<f',0.18))[0]]];vec18=[ident for ident,value in constants.items() if isinstance(value,dict) and len(value['constituents'])==3 and all(v in scalar18 for v in value['constituents'])]
selects=[q for q in ops if q[1]==169 and any(v in vec18 for v in q[2][3:])];ids=set(q[2][2] for q in selects);chain=[]
for _ in range(5):
 for q in ops:
  if len(q[2])>=2 and q[2][1] in ids and q not in chain:
   chain.append(q)
   if q[1] in [65,66,61,171,170,169,80,168]:ids.update(q[2][2:])
report={'shader':str(p),'container_sha256':hashlib.sha256(b).hexdigest(),'module_offset':start,'module_sha256':hashlib.sha256(b[start:start+count*4]).hexdigest(),'words':count,'is_input_valid_members':hits,'scalar18':scalar18,'vec18':vec18,'selects':selects,'condition_backward_slice':chain,'related_constants':{str(k):v for k,v in constants.items() if k in ids},'related_names':{str(k):v for k,v in names.items() if k in ids}}
(base/'evidence/round13-frame-graph-gpu/review/original-input-valid-shader.json').write_text(json.dumps(report,indent=2));print(json.dumps(report,indent=2))


