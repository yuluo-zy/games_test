import json,struct,collections,re
from pathlib import Path
BASE=Path(__file__).resolve().parents[2]
ROOT=Path('D:/game/ljxsj_92385/Tiny Glade')
INV=json.loads((BASE/'original-shader-inventory.json').read_text('utf-8'))
def decode_string(words): return struct.pack('<%dI'%len(words),*words).split(b'\0')[0].decode('utf-8',errors='replace')
def extract(m,s):
 data=(ROOT/m['file']).read_bytes(); words=struct.unpack_from('<%dI'%s['spirv_words'],data,s['module_offset']); p=5; names=[]; constants=[]; site=None; ops=collections.defaultdict(list); layouts=[]
 strings={}; defs={}
 while p<len(words):
  wc,op=words[p]>>16,words[p]&65535; a=words[p+1:p+wc]
  if op==7: strings[a[0]]=decode_string(a[1:])
  if op==5: names.append(decode_string(a[1:]))
  if op==6: names.append(decode_string(a[2:]))
  if op==8: site=(strings.get(a[0],str(a[0])),a[1])
  if op==43 and len(a)==3: constants.append({'id':a[1],'bits':hex(a[2]),'float':struct.unpack('<f',struct.pack('<I',a[2]))[0]})
  if op in (16,28,29,30,59): layouts.append({'word_offset':p,'opcode':op,'operands':a})
  if op not in (2,3,5,6,7,8,317) and site: ops[site[0]].append({'line':site[1],'word_offset':p,'opcode':op,'operands':a})
  p+=wc
 return {'file':m['file'],'sha256':m['sha256'],'stage':s['stage'],'spirv_sha256':s['spirv_sha256'],'module_offset':s['module_offset'],'names':names,'constants':constants,'layout_ops':layouts,'source_ops':dict(ops)}
selected=[]
extra={ 'shaders/'+p for p in ['ice_plane.raster.hlsl','solid_vertex_color.raster.hlsl','solid_vertex_color_albedo_map.raster.hlsl','vertex_color.raster.hlsl','water/ice_cracks.raster.hlsl','water_plane.raster.hlsl','water_plane_refraction_copy.raster.hlsl','water_snow_plane.raster.hlsl','water_x_paths.raster.hlsl','waterfall.raster.hlsl','waterfall_refraction_copy.raster.hlsl']}
support_extra={'shaders/water/'+p for p in ['ice_shared.inc.hlsl','water_object_push.inc.hlsl','water_surface_frag.inc.hlsl','waterfall_vert.inc.hlsl']}
for m in INV:
 main=next((p for p in m['source_dependencies'] if Path(m['file']).name.startswith('_'+p.removeprefix('shaders/').replace('/','_')+'_')),m['source_dependencies'][0])
 if main in extra or support_extra.intersection(m['source_dependencies']) or any(k in main for k in ['fxaa','taa.cs','tgsr','dof/','blur','gi/','deferred/','water/deferred','water/ice_deferred','water/ice_refl','water/ice_refr','water/calc_deferred','calculate_reprojection','final_post','generate_mips','generate_hzb','light_binning','cloud.raster','skydome','egui','ui_2d','text.raster','lut/','ssgi/','copy_depth','blit/','screenshot/','session_selection/','lut_gallery/','apply_screen','apply_volumetrics','capture_panic','fill_buffer','fsr_','fullscreen_triangle','extract_half_res','prefix_scan/','primary_rt_debug','rdi/','resolve_to_swapchain','shader_refactoring','shadow_mask_mean','terrain_lighting_texture','upload_cpu_raster','loading_screen','logo','custom_cursor','dashed_line','normal_debug','texture_test','timer_circle']):
  for s in m['stages']: selected.append(extract(m,s))
(Path(__file__).parent/'rawproof.json').write_text(json.dumps(selected,ensure_ascii=False,indent=2,allow_nan=True),'utf-8')
for m in selected:
 if m['stage']=='Vertex': continue
 own=[(k,len(v),min(x['line'] for x in v),max(x['line'] for x in v)) for k,v in m['source_ops'].items() if k==m['file'].replace('compiled-shaders/_','shaders/').replace('_cs_main_0.bin','') or any(q in k for q in ['fxaa','taa.cs','tgsr','dof/','blur.cs','reprojection','ssgi'])]
 print(m['file'],m['stage'], own,'\n names=',','.join(n for n in m['names'] if not n.startswith(('type.','in.','gl.')))[-1300:])
