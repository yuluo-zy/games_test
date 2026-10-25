from pathlib import Path
import json,collections
HERE=Path(__file__).resolve().parent
OUT=HERE.parent/'file-classification-2026-10-14/common'
original=json.loads((HERE.parent/'original-source-dependencies.json').read_text(encoding='utf-8-sig'))
cmp=json.loads((HERE/'path-and-identifiers-comparison.json').read_text())
candidates={x['original']:x for x in cmp['normalized_path_matches']}
stats=json.loads((OUT/'raw-source-instruction-stats.json').read_text())
root=HERE.parent
shaderroot='https://github.com/EmbarkStudios/kajiya/blob/9fdec0f074f9f822d395f9cfd041b58749e32303/assets/shaders/'
game_fields={
 'foliage_exclusion.hlsl':['foliage_exclusion_mask','foliage_exclusion_mask_dims'],
 'grass_blade_data.hlsl':['GrassBladeDataPacked'],
 'lanterns.hlsl':['LanternPacked','lantern_buf','lantern_count'],
 'roof_colors_include.hlsl':['roof_color_ids','roof_colors','roof_color_1','roof_color_2'],
 'terrain_heightmap.inc.hlsl':['terrain_heightmap','terrain_heightmap_dims','visual_terrain_heightmap'],
 'wall_color_ids.hlsl':['wall_color_ids'],
 'window_color_customization.hlsl':['window_colors'],
 'wall_color_customization.hlsl':['wall_color_ids'],
 'wind.hlsl':['wind_strength','in.var.wind_bend'],
}
allnames=json.loads((HERE/'original-spv-names-members-constants.json').read_text())
def names_in_bins(bins,desired):
 out=[]
 for bn in bins:
  for mod in allnames.get(bn,[]):
   hit=set(desired)&set(mod['names']+mod['members'])
   if hit:out.append({'file':bn,'module_offset':mod['offset'],'names':sorted(hit)})
 return out[:6]
game_domain={
 'foliage_exclusion.hlsl':'排除建筑/路径区域的游戏植被遮罩接口',
 'grass_blade_data.hlsl':'游戏草叶实例的打包布局',
 'lanterns.hlsl':'游戏灯笼实例buffer与数量参数',
 'roof_colors_include.hlsl':'游戏屋顶颜色ID与实例颜色参数',
 'terrain_heightmap.inc.hlsl':'游戏地形高度图资源、采样接口与分辨率参数',
 'wall_color_ids.hlsl':'游戏墙体颜色ID参数',
 'window_color_customization.hlsl':'游戏窗户颜色配置',
 'wall_color_customization.hlsl':'游戏墙体颜色配置',
 'wind.hlsl':'游戏植被实例风弯曲属性与全局风力参数',
 'wall_distortion.hlsl':'游戏墙体形变/材质几何接口',
 'rocky_terrain.inc.hlsl':'游戏岩石地形的共享资源/几何接口',
 'highlight_color.hlsl':'游戏编辑高亮色选择与参数接口',
 'material_eval.inc.hlsl':'游戏canopy/grass/plant/solid材质求值的组合接口',
}
material_scope='游戏canopy/grass/plant/solid材质类型、资源和求值组合；BRDF/标准采样数学不因此判为原创。'
files=[]
for o in original:
 s=o['source']
 if not s.startswith('shaders/inc/') and s not in candidates:continue
 c=candidates.get(s); st=stats[s]
 entry={'source':s,'status':'generic_unresolved','origin':'Unresolved common graphics helper or pass','confidence':'insufficient_for_code_origin','scope':'整个文件的具体源码出处未锁定；仅能识别通用图形任务。','proof':[{'type':'original_source_dependency','source_lf':o['source_linefeed_counts'],'pipeline_count':o['pipeline_count']},{'type':'raw_spirv_source_lines','emitted_line_count':len(st['emitted_source_lines']),'emitted_lines':st['emitted_source_lines']}],'limitations':['通用文件名、标准公式、常见常数或同一目录不足以证明拷贝开源源码；未证明逐字源码一致或从零原创。'],'upstream_url':None,'proof_file':str(OUT/'raw-source-instruction-stats.json').replace('\\','/')}
 if c:
  entry['upstream_url']=shaderroot+c['upstream'];entry['proof'].append({'type':'candidate_only','normalized_same_path':c['upstream'],'upstream_lf':c['upstream_lf'],'shared_raw_identifiers':sorted({n for h in c['original_spv_shared_identifiers'] for n in h['shared_identifiers']})})
 base=s.removeprefix('shaders/inc/')
 if base in game_domain or base.startswith('material/'):
  entry.update(status='game_specific',origin='Tiny Glade game domain data or integration',confidence='medium_for_domain_scope',scope=game_domain.get(base,material_scope))
  hit=names_in_bins(o['files'],game_fields.get(base,[]))
  if hit:entry['proof'].append({'type':'raw_named_game_resources_or_types','hits':hit});entry['confidence']='high_for_named_data_layout_or_parameters'
  entry['limitations']=['game_specific只判游戏领域数据、参数或组合接口；不宣称文件内所有数学/实现从零编写。','OpSource正文空白，无法完整排除藏在接口下的外部算法。']
 if s=='shaders/inc/frame_constants.hlsl':
  entry.update(status='game_modified',origin='Kajiya shared ViewConstants/FrameConstants lineage plus Tiny Glade extensions',confidence='high_for_layout_lineage',scope='继承相机/帧常量的字段族并增添游戏主题、时间、编辑高亮和楼梯边ID等字段；不是整文件逐字copy。')
  entry['proof'].append({'type':'raw_unique_layout_lineage','inherited_fields':['hi_px_subpixels','render_overrides','sample_offset_clip','prev_clip_to_prev_view','prev_view_to_prev_clip'],'game_added_fields':['glade_theme_id','game_time_sec','editing_highlight_color','highlighted_transient_stair_edge_id'],'source_lf_change':[252,439],'raw_fields':names_in_bins(o['files'],['hi_px_subpixels','render_overrides','sample_offset_clip','prev_clip_to_prev_view','prev_view_to_prev_clip','glade_theme_id','game_time_sec','editing_highlight_color','highlighted_transient_stair_edge_id'])})
 if s=='shaders/inc/reservoir.hlsl':
  entry.update(status='confirmed_upstream',origin='Kajiya reservoir helper source lineage',confidence='high',scope='Reservoir1spp/Reservoir1sppStreamState类型布局及stream初始化/累计/finish_stream；具体源码版本与全文diff未知。',proof_file=str(HERE/'reservoir-original-instructions.json').replace('\\','/'))
  entry['proof'].append({'type':'raw_spirv_unique_type_and_dataflow','types':{'Reservoir1spp':['w_sum','payload','M','W'],'Reservoir1sppStreamState':['p_q_sel','M_sum']},'formula':'W = w_sum / max(float32(1e-8), M * p_q_sel)','module_offset':21538,'source_line':94,'float_constant_bits':'0x322bcc77'})
  entry['limitations']=['确认源码家族的实际复用，不等于全文原样复制；原source96LF，上游100LF。','原发行版权声明Kajiya0.2.0，无法锁定游戏私有fork的commit。']
 if s=='shaders/inc/color/helmholtz_kohlrausch.hlsl':
  entry.update(status='likely_derived',origin='Likely Kajiya color helper source family',confidence='medium',scope='独特文件布局同源线索；最终shader没有发射此文件函数，不能说当前执行了H-K算法。')
  entry['proof'].append({'type':'specific_layout_plus_attribution','same_source_path':True,'same_lf':133,'emitted_instruction_lines':[],'original_attribution':'Kajiya0.2.0'})
  entry['limitations']=['133LF与独特文件名支持候选来源，不能代替源码hash或原指令等价。','上游自定义16项颜色数组未在原SPIR-V出现；该include当前未形成可核验执行代码。']
 if s=='shaders/inc/unjitter_taa.hlsl':
  entry.update(status='likely_derived',origin='Likely Kajiya unjitter helper with changes',confidence='medium_high',scope='相同文件族、像素偏移方向约定、颜色权重与统计moment使用不同Gaussian权重；原源码扩展到161LF。',proof_file=str(OUT/'selected-instruction-traces.json').replace('\\','/'))
  entry['proof'].append({'type':'raw_math_constellation_plus_upstream','observed_constants':[-10.0,-1.0,4.0,0.5],'upstream_scheme':'Exp2(-10*dist2*scale) for color and Exp2(-dist2*scale) for ex/ex2; jitter sign float2(1,-1)','raw_source_lines':[105,108,109,110,111,118,120,123,126,136,137,139,140,141,142,143,149,150]})
  entry['limitations']=['多重独特设计与原attribution支持派生，但未为全部函数证明指令等价。','4.0及源LF变化显示可能扩展，未恢复完整diff。']
 if s=='shaders/ssgi/temporal_filter.cs.hlsl':
  entry.update(status='game_modified',origin='Kajiya SSGI temporal-filter design adapted for Tiny Glade',confidence='high_for_adapted_core',scope='复用同一组历史/双输出资源及Gaussian统计、variance clamp与1/8历史混合设计；游戏改采样邻域和历史有效性接口。',proof_file=str(OUT/'selected-instruction-traces.json').replace('\\','/'))
  entry['proof'].append({'type':'raw_instruction_math_constellation','shared_resources':['input_tex','history_tex','reprojection_tex','history_output_tex','final_output_tex'],'raw_used_constants':[0.125,0.25,0.5,5.0,0.22313016653060913,0.4723665416240692,0.2644112706184387],'design':'5 sigma at box_size0.5; lerp center/moment by0.25; lerp clamped history/current by0.125','change':'Upstream k=2 =>5x5, game constants exp(-0.75) and exp(-1.5) and normalized reciprocal0.26441127 correspond k=1=>3x3; original history_valid gate/new dims fields'})
  entry['limitations']=['确认设计与代码家族派生及具体可见改动，不等于全文逐字copy。']
 if s=='shaders/calculate_reprojection_map.cs.hlsl':
  entry.update(status='game_modified',origin='Kajiya reprojection-map lineage with changed validity handling',confidence='high_for_adapted_core',scope='继承15bit运动量化、四样本validity位权打包及-0.2平面偏差设计；游戏调整阈值/资源布局。',proof_file=str(OUT/'selected-instruction-traces.json').replace('\\','/'))
  entry['proof'].append({'type':'raw_instruction_constellation_plus_resources','motion_quantization':[32767.0,3.0518509447574615e-05],'quad_validity_weights':[1,2,4,8],'validity_normalization':0.06666667014360428,'plane_slope_floor':-0.20000000298023224,'game_thresholds':[-0.004999999888241291,-0.0003000000142492354],'matching_upstream_resources':['depth_tex','prev_depth_tex','velocity_tex','output_tex'],'source_lf_change':[142,178]})
  entry['limitations']=['独特组合支持派生；运动重投影本身是通用技术，不把其全部数学判成Kajiya原创。','没有完整上游原样编译diff；具体forkcommit未知。']
 if s in ['shaders/inc/fidelityfx/ffx_a.h','shaders/inc/fidelityfx/ffx_spd.h']:
  entry.update(status='confirmed_upstream',origin='AMD FidelityFX SPD include source family',confidence='high_for_spd_family',scope='SPD/common支持include实际集成；ffx_a的未执行通用部分不由此全部证实。',upstream_url='https://github.com/GPUOpen-LibrariesAndSDKs/FidelityFX-SDK',proof_file=str(root/'REPORT.md').replace('\\','/'))
  entry['proof'].append({'type':'existing_original_audit','original_attribution':'FidelityFX1.0.0','raw_evidence':'ffx_spd.h1297LF;ffx_a.h933LF; SPD256threads/64x64tile/16x16shared/wave reduction and sync, see root REPORT AMD subsection'})
  entry['limitations']=['这里复用既有原始SPD核对，不追加全文原样或精确版本声明。']
 if s=='shaders/inc/rt.hlsl':
  entry.update(status='confirmed_upstream',origin='GPU-Raytracer BVH8 traversal port plus game integration',confidence='high_for_bvh8_core',scope='仅compressed-wide BVH8节点解码/遍历已确认；rt.hlsl其他函数和资源接入不能全归为上游。',upstream_url='https://github.com/jan-van-bergen/GPU-Raytracer/blob/3525cd4f96fba01c8976c7f1a8465dac7bd3d60f/Src/CUDA/Raytracing/BVH8.h',proof_file=str(root/'raytracer/raw_spirv_fingerprints.json').replace('\\','/'))
  entry['proof'].append({'type':'existing_raw_dataflow_audit','stride':80,'formulas':['(meta & (meta << 1)) & 0x10101010','(meta ^ (octant & mask)) & 0x1f1f1f1f','(meta >> 5) & 0x07070707'],'source_report':'raytracer/REPORT.md'})
  entry['limitations']=['同名Kajiya rt.hlsl仅候选；确证BVH8核心来源是GPU-Raytracer，不能把原整个rt.hlsl归Kajiya。','原始include720LF包含适配和其他逻辑，本条范围是指定核心。']
 files.append(entry)
summary={'files_total':len(files),'inc_files':sum(x['source'].startswith('shaders/inc/') for x in files),'normalized_kajiya_candidates':len(candidates),'counts_by_status':dict(collections.Counter(x['status'] for x in files))}
(OUT/'classification.json').write_text(json.dumps({'schema_version':1,'summary':summary,'files':files,'method':'Original shipped SPIR-V names/member names, source dependencies, OpLine-associated computational instructions; compare Kajiya public commit; source body blank. Scoped classifications do not measure source-code ownership percentages.'},ensure_ascii=False,indent=2),encoding='utf-8')
print(summary)
