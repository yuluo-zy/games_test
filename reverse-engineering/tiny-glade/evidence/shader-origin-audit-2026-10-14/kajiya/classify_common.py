from pathlib import Path
import json,struct,collections
HERE=Path(__file__).resolve().parent
OUT=Path('D:/game/reverse-engineering/tiny-glade/evidence/shader-origin-audit-2026-10-14/file-classification-2026-10-14/common')
OUT.mkdir(parents=True,exist_ok=True)
BASE=json.loads((HERE/'path-and-identifiers-comparison.json').read_text())
targets={x['original'] for x in BASE['normalized_path_matches']}
original_sources=json.loads((HERE.parent/'original-source-dependencies.json').read_text(encoding='utf-8-sig'))
targets.update(x['source'] for x in original_sources if x['source'].startswith('shaders/inc/'))
traces={}
stats={s:{'opcode_counts':collections.Counter(),'emitted_source_lines':set(),'modules':[],'float_constant_bits':set()} for s in targets}
for path in Path('D:/game/ljxsj_92385/Tiny Glade/compiled-shaders').glob('*.bin'):
 data=path.read_bytes();cur=0
 while True:
  at=data.find(b'\x03\x02\x23\x07',cur)
  if at<4:break
  n=struct.unpack_from('<I',data,at-4)[0]
  if n<5 or at+4*n>len(data):cur=at+4;continue
  w=struct.unpack_from('<'+'I'*n,data,at);i=5;strings={};const={};comps={};ins=[];active=None;typed={};floattypes=set()
  while i<n:
   wc,op=w[i]>>16,w[i]&65535;a=list(w[i+1:i+wc])
   if wc==0 or i+wc>n:break
   if op==7:strings[a[0]]=struct.pack('<'+'I'*(len(a)-1),*a[1:]).split(b'\0')[0].decode('utf-8','replace')
   if op==22 and a[1]==32:floattypes.add(a[0])
   if op==43 and len(a)==3:const[a[1]]=(a[0],a[2])
   if op==44:comps[a[1]]=a[2:]
   if op==8:active=(strings.get(a[0]),a[1],a[2])
   if op==317:active=None
   if active and active[0] in targets and op not in [8,317]:ins.append((active,op,a,at+4*i))
   i+=wc
  def float_bits(value):
   if value in const:return [const[value][1]] if const[value][0] in floattypes else []
   if value in comps:return [b for v in comps[value] for b in float_bits(v)]
   return []
  mod_sources=set()
  for (source,line,col),op,a,off in ins:
   st=stats[source];st['opcode_counts'][str(op)]+=1;st['emitted_source_lines'].add(line);mod_sources.add(source)
   # Computational instructions use result type/id then operand ids; not a full grammar decoder.
   if op in [12,81,109,110,111,112,115,124,126,127,128,129,130,131,132,133,134,135,136,137,138,139,140,141,142,143,144,145,146,147,148,170,171,172,173,174,175,176,177,178,179,180,181,182,183,184,185,186,187,188,189,190,191]:
    operands=a[4:] if op==12 else a[2:3] if op==81 else a[2:]
    for value in operands:st['float_constant_bits'].update(float_bits(value))
  for source in mod_sources:stats[source]['modules'].append({'bin':path.name,'module_offset':at})
  for source in ['shaders/ssgi/temporal_filter.cs.hlsl','shaders/calculate_reprojection_map.cs.hlsl','shaders/inc/unjitter_taa.hlsl']:
   if source in mod_sources and source not in traces:
    traces[source]={'bin':path.name,'module_offset':at,'instructions':[{'line':line,'column':col,'opcode':op,'operands':a,'offset':off} for (s,line,col),op,a,off in ins if s==source],'constants':{str(k):{'type_id':t,'bits':hex(v),'float32_interpretation':struct.unpack('<f',struct.pack('<I',v))[0] if t in floattypes else None} for k,(t,v) in const.items()},'constant_composites':comps}
  cur=at+4*n
for source,st in stats.items():
 st['opcode_counts']=dict(st['opcode_counts']);st['emitted_source_lines']=sorted(st['emitted_source_lines'])
 st['float_constants']=[{'bits':hex(b),'float32':struct.unpack('<f',struct.pack('<I',b))[0]} for b in sorted(st.pop('float_constant_bits'))]
(OUT/'raw-source-instruction-stats.json').write_text(json.dumps(stats,indent=2),encoding='utf-8')
(OUT/'selected-instruction-traces.json').write_text(json.dumps(traces,indent=2),encoding='utf-8')
for source in ['shaders/ssgi/temporal_filter.cs.hlsl','shaders/calculate_reprojection_map.cs.hlsl','shaders/inc/unjitter_taa.hlsl','shaders/inc/image.hlsl','shaders/inc/bilinear.hlsl','shaders/inc/color/helmholtz_kohlrausch.hlsl']:
 st=stats[source];print(source,'lines',st['emitted_source_lines'],'floats',[x['float32'] for x in st['float_constants']])
