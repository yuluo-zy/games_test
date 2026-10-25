import json,collections
from pathlib import Path
OUT=Path(__file__).resolve().parent
BASE=OUT.parents[1]
raw=json.loads((OUT/'rawproof.json').read_text('utf-8'))
deps=json.loads((BASE/'original-source-dependencies.json').read_text('utf-8'))
rows=json.loads((OUT/'classification.json').read_text('utf-8'))
extra=['ice_plane.raster.hlsl','solid_vertex_color.raster.hlsl','solid_vertex_color_albedo_map.raster.hlsl','vertex_color.raster.hlsl','water/ice_cracks.raster.hlsl','water/ice_shared.inc.hlsl','water/water_object_push.inc.hlsl','water/water_surface_frag.inc.hlsl','water/waterfall_vert.inc.hlsl','water_plane.raster.hlsl','water_plane_refraction_copy.raster.hlsl','water_snow_plane.raster.hlsl','water_x_paths.raster.hlsl','waterfall.raster.hlsl','waterfall_refraction_copy.raster.hlsl']
for tail in extra:
 source='shaders/'+tail
 if any(r['source']==source for r in rows): continue
 d=next(x for x in deps if x['source']==source)
 matches=[r for r in raw if r['file'] in d['files']]
 own=[]
 for m in matches:
  ops=m['source_ops'].get(source,[])
  if ops: own.append({'file':m['file'],'stage':m['stage'],'module_offset':m['module_offset'],'spirv_sha256':m['spirv_sha256'],'instruction_count':len(ops),'source_lines':sorted(set(o['line'] for o in ops)),'examples':ops[:7]})
 names=sorted(set(n for m in matches for n in m['names'] if any(k in n.lower() for k in ['water','ice','snow','refract','ripple','rough','mask','push','wave','crack','shadow','material','semitransparent','flat_lighting','albedo','color','mesh','position','normal','tangent','atlas','instance'])))
 contract='原材质/资源与游戏帧接线；标准光照/顶点变换数学的独立作者未知。'
 if tail.startswith('solid_'): contract='原Globals保留is_semitransparent/is_flat_lighting/bool_render_shadows/model_to_world；游戏渲染模式/材质接线，可复用一般顶点颜色/纹理算法。'
 if 'ice' in tail: contract='原游戏水mask/roughness/refraction与ice_refl计数/像素队列或裂纹材质输入；游戏冰面流程合同，通用反射折射/噪声作者未知。'
 if 'water' in tail: contract='原游戏水mask/refraction/history与water_rt队列/RippleSourceSsbo或WaterfallMesh(position,tangent,normal0,normal1)接线；通用波/BRDF/光线算法来源另列。'
 if tail=='water_x_paths.raster.hlsl': contract='原water_mask/path_mask/buildings_mask/rocky_terrain_texture资源同时参与；游戏水与道路/建筑交界材质合同，通用采样数学未知。'
 status='game_adapter'; confidence='medium'; origin='Tiny Glade游戏水冰/基础材质接线'
 if tail=='vertex_color.raster.hlsl': status='generic_unresolved';confidence='low';origin='通用顶点颜色shader，源码来源待定';contract='原始顶点/颜色输入与光栅输出可观察；基本变换和颜色透传不能用于证明原创。'
 rows.append({'source':source,'status':status,'origin':origin,'confidence':confidence,'scope':contract,'proof':{'source_dependency':d,'actual_source_emission':own,'actual_resource_and_type_names':names,'emitted_shared_sources':sorted(set(s for m in matches for s in m['source_ops'] if s.startswith('shaders/')))},'limitations':'游戏合同/接线归属不等于所有include或数学代码原创；没有原HLSL正文，未证明逐字源码身份。','upstream_url':None,'proof_file':'render/rawproof.json'})
for r in rows:
 if r['source']=='shaders/ssgi/temporal_filter.cs.hlsl':
  r.update(status='game_modified',origin='Kajiya SSGI temporal_filter同源核心加游戏改写',confidence='high',scope='沿用加权均值/方差、box_size=.5/n_deviations=5、history clamp和1/8 temporal blend；游戏改3x3/历史门控/资源绑定',upstream_url='https://github.com/EmbarkStudios/kajiya/blob/9fdec0f074f9f822d395f9cfd041b58749e32303/assets/shaders/ssgi/temporal_filter.hlsl')
  r['proof']['specific_fingerprint']='原source对应实际138指令，raw常量0.2231301665/0.4723665416/0.2644112706=3x3步长2核，current+blur混合0.25，dev*0.5*5，clamp后当前帧0.125；Kajiya使用同一box_size0.5、n_deviations5、box_size平方均值混合、dev×box_size×n_deviations、clamp后1/8混合。主核从k=2改为k=1，新增history_valid/frame_index门控。此改写分类覆盖之前game_adapter粗分类，不与common报告冲突。'
(OUT/'classification.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n','utf-8')
print('paths',len(rows),'statuses',dict(collections.Counter(r['status'] for r in rows)))
for r in rows:
 if r['source'].removeprefix('shaders/') in extra: print(r['source'],r['status'],len(r['proof']['actual_source_emission']))
