import json, collections
from pathlib import Path
OUT=Path(__file__).resolve().parent
BASE=OUT.parents[1]
deps=json.loads((BASE/'original-source-dependencies.json').read_text('utf-8'))
raw=json.loads((OUT/'rawproof.json').read_text('utf-8'))
inventory=json.loads((BASE/'original-shader-inventory.json').read_text('utf-8'))
byfile={m['file']:m for m in inventory}
main_paths=set()
for r in raw:
 m=byfile[r['file']]
 for p in m['source_dependencies']:
  if Path(m['file']).name.startswith('_'+p.removeprefix('shaders/').replace('/','_')+'_'): main_paths.add(p)
main_paths|={d['source'] for d in deps if d['source'].startswith(('shaders/dof/','shaders/gi/','shaders/deferred/','shaders/light_binning/','shaders/screenshot/'))}
main_paths|={'shaders/fxaa.inc.hlsl','shaders/tgsr_common.inc.hlsl','shaders/resolve_to_swapchain.inc.hlsl','shaders/fullscreen_triangle_vert.inc.hlsl'}
UP='https://github.com/EmbarkStudios/kajiya/blob/9fdec0f074f9f822d395f9cfd041b58749e32303/assets/shaders/'
results=[]
for d in deps:
 p=d['source']
 if p not in main_paths: continue
 matches=[r for r in raw if r['file'] in d['files']]
 evidence=[]
 for r in matches:
  own=r['source_ops'].get(p,[])
  if own:
   evidence.append({'file':r['file'],'stage':r['stage'],'module_offset':r['module_offset'],'spirv_sha256':r['spirv_sha256'],'source':p,'ops':len(own),'source_lines':sorted(set(x['line'] for x in own)),'instruction_examples':own[:6]})
 emitted=sorted(set(s for r in matches for s in r['source_ops'] if s.startswith('shaders/')))
 names=sorted(set(n for r in matches for n in r['names']))
 item={'source':p,'status':'generic_unresolved','origin':'未确定','confidence':'low','scope':'当前源文件主体；引用通用或上游include不计作整个文件来源','proof':{'source_dependency':d,'actual_source_emission':evidence,'emitted_shared_sources':emitted,'resource_and_type_names':names[-35:]},'limitations':'只有原编译指令/源行，没有原HLSL正文；未找到上游不能证明从零原创。','upstream_url':None,'proof_file':'render/rawproof.json'}
 if p in ('shaders/blur_downsample.cs.hlsl',):
  item.update(status='confirmed_upstream',origin='Kajiya blur.hlsl 同源实现，游戏资源/入口及MSAA适配',confidence='high',scope='两遍共享内存高斯降采样主体；接口改写',upstream_url=UP+'blur.hlsl')
  item['proof']['specific_fingerprint']='原raw vblur_out[138]，64x1x1工作组，128x2输入tile，两遍11tap，Exp(-d*d*0.16)，与上游kernel_radius=5/group_width=64/vblur_window_size=138和gaussian_wt完整组合一致。'
 elif p=='shaders/blur_pyramid/blur.cs.hlsl':
  item.update(status='game_modified',origin='Kajiya共享内存两遍blur派生',confidence='high',scope='同一vblur_out/tile结构，游戏改为136共享槽/9tap，动态mip尺寸与边界mask',upstream_url=UP+'blur.hlsl')
 elif p=='shaders/calculate_reprojection_map.cs.hlsl':
  item.update(status='game_modified',origin='Kajiya重投影map派生',confidence='high',scope='重投影/有效性主流程来自共同源码家族，游戏改变速度坐标、历史/阈值/相机规则',upstream_url=UP+'calculate_reprojection_map.hlsl')
  item['proof']['specific_fingerprint']='原raw:32767 SNORM量化、GatherRed wzxy，quad validity以1/2/4/8编码后÷15，prev_clip_to_prev_view、-0.2截断、视图深度比较；Kajiya同文件包含这组具体流程。'
 elif p=='shaders/fxaa.inc.hlsl':
  item.update(status='likely_derived',origin='NVIDIA Timothy Lottes FXAA 3.11 PC Quality 源码家族',confidence='high',scope='FXAA核心include，精确编辑版本未定',upstream_url='https://github.com/GameTechDev/CMAA2/blob/master/Projects/CMAA2/FXAA/Fxaa3_11.h')
  item['proof']['specific_fingerprint']='原include1530LF，源行708/881..1236发射实际代码；Green gather、0.166/0.75阈值/亚像素参数、二阶edge梯度、1/12平滑项、1/1.5/2/4/12质量步距及双向终点搜索/跨度符号拒绝；上游PC Quality preset12对应全组控制流。原文件被改名/裁剪，不能逐字diff。'
 elif p=='shaders/fxaa.cs.hlsl':
  item.update(status='game_adapter',origin='游戏FXAA compute入口接线；核心include很可能NVIDIA3.11',confidence='high',scope='资源绑定、入口与输出；FXAA核心另列fxaa.inc.hlsl')
 elif p in ('shaders/generate_hzb.cs.hlsl','shaders/generate_mips.cs.hlsl'):
  item.update(status='game_adapter',origin='FidelityFX SPD原源码+游戏归约/load/store回调',confidence='high',scope='SPD循环核来自ffx_spd.h；min深度或average颜色与资源绑定由游戏配置',upstream_url='https://github.com/GPUOpen-Effects/FidelityFX-SPD/blob/master/ffx-spd/ffx_spd.h')
 elif p=='shaders/ssgi/ssgi.cs.hlsl':
  item.update(status='game_adapter',origin='XeGTAO主算法include+游戏SSGI调度和绑定',confidence='high',scope='XeGTAO指令源行实际发射；游戏入口和资源/参数流程单列',upstream_url='https://github.com/GameTechDev/XeGTAO/blob/master/Source/Rendering/Shaders/XeGTAO.hlsli')
 elif p=='shaders/ssgi/upsample.cs.hlsl':
  item.update(status='likely_derived',origin='Kajiya ssgi/upsample.hlsl 候选改写',confidence='medium',scope='共同3x3低分辨率深度门控/高分辨率重建；-200深度因子，新增半分辨率法线，不能仅标准公式证明源码拷贝',upstream_url=UP+'ssgi/upsample.hlsl')
 elif p=='shaders/ssgi/temporal_filter.cs.hlsl':
  item.update(status='game_modified',origin='Kajiya SSGI temporal-filter同源核心加游戏改写',confidence='high',scope='box_size=.5/n_deviations=5与1/8历史混合继承；游戏改3x3核/历史门控/资源绑定',upstream_url=UP+'ssgi/temporal_filter.hlsl')
 elif p.startswith('shaders/tgsr'):
  item.update(status='game_specific',origin='Tiny Glade TGSR；官方称为自有超分方案',confidence='high',scope='游戏自有时域超分主体/接线；数学和共享过滤include可能复用',upstream_url='https://steamcommunity.com/app/2198150/announcements/')
  item['proof']['specific_fingerprint']='原raw保留reprojected_history_tex_lum/chroma、blurred_luminance_tex、aux_tex、tile_luminance，以及游戏专用history-valid/sharpen策略；官方公告说TGSR是own thing。公告TGSR2.0不能拿来证明本B编译版本已有神经网络。'
 elif p.startswith('shaders/dof/'):
  item.update(status='game_specific',origin='游戏多分辨率DOF/自动对焦方案，非Kajiya公开gather原样拷贝',confidence='medium',scope='原raw tile分类、dispatch列表、ring_samples、半/全分辨率和双焦点/锁定等组合；底层通用CoC数学另查')
  item['proof']['specific_fingerprint']='原raw DofTileOffsets/offsets、tile_coords_buffer、ring_samples和DofFocus(双focus/lock/tilt-shift)结构；multires.inc.hlsl发射70..591行代码。Kajiya公开dof/gather是GOLDEN_ANGLE/RAD_SCALE=0.4逐像素螺旋，没有上述tile/indirect/multires流程。'
 elif p=='shaders/taa.cs.hlsl':
  item.update(status='game_specific',origin='游戏TAA实现；共享Kajiya工具并不证明TAA主体复制',confidence='medium',scope='游戏主体CatmullRom+邻域双统计/钳制+共享tile锐化；独立于当前Kajiya多pass confidence-based TAA')
  item['proof']['specific_fingerprint']='原raw资源color_tex/history_tex/reprojection_tex/history_output_tex/output_tex，tile_luminance,bypass_neighborhood_clamping/sharpen_amount/history_valid；Kajiya公开taa.hlsl使用input_prob_tex、smooth_var_history_tex、velocity_history_tex等多pass结构，原本B不存在这些binding。'
 elif p.startswith('shaders/deferred/') or p.startswith('shaders/gi/') or p.startswith('shaders/water/'):
  item.update(status='game_adapter',origin='游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码',confidence='medium',scope='入口/材质/terrain/light/水冰资源规则属于游戏组合；实际emitted共享源另列，不外推整pass源码来源')
 elif p.startswith('shaders/ssgi/'):
  item.update(status='game_adapter',origin='游戏SSGI时域处理组合；精确原始算法源未全确定',confidence='medium',scope='游戏时域资源/绑定和历史规则')
 elif p.startswith(('shaders/ui_2d','shaders/egui','shaders/text.','shaders/custom_cursor','shaders/timer_circle','shaders/loading_screen','shaders/logo','shaders/session_selection/','shaders/lut_gallery/')):
  item.update(status='game_adapter',origin='游戏UI/选择界面/字形/颜色按钮资源接线',confidence='medium',scope='原资源/打包顶点/字形流/界面参数布局是游戏适配；纹理采样或三角公式太通用，无法确认其全部正文原创')
 elif p.startswith('shaders/light_binning/'):
  item.update(status='game_adapter',origin='游戏灯光网格/桶/前缀合并接线',confidence='medium',scope='游戏网格/灯光资源布局与处理策略；通用prefix/bucket代码具体来源未证')
 elif p in ('shaders/cloud.raster.hlsl','shaders/skydome.raster.hlsl','shaders/final_post.raster.hlsl','shaders/apply_screen_shadows.cs.hlsl','shaders/apply_volumetrics.cs.hlsl','shaders/terrain_lighting_texture.cs.hlsl','shaders/lut/water_noise.cs.hlsl','shaders/rdi/temporal.cs.hlsl'):
  item.update(status='game_adapter',origin='游戏美术/光照/天气/后处理组合',confidence='medium',scope='主题/太阳天空/颜色库/terrain资源/自有灯光与帧规则；通用算法具体来源可能混合')
 results.append(item)
(OUT/'classification.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n','utf-8')
print(len(results),dict(collections.Counter(x['status'] for x in results)))
print('\n'.join(x['source'] for x in results))
