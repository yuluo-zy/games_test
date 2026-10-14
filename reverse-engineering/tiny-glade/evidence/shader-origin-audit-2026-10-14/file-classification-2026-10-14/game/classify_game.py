import collections,json,re
from pathlib import Path
OUT=Path(__file__).resolve().parent
DATA=json.loads((OUT/'raw_game_evidence.json').read_text())
EXISTING={row['source']:row for row in json.loads((OUT/'classification.json').read_text(encoding='utf-8'))['files']} if (OUT/'classification.json').exists() else {}

def family(source):
    s=source.removeprefix('shaders/')
    if s.startswith(('flag_','plaster.','smoke.','stairs_floor.','wooden_plank.')):return 'game_props_and_material_layouts'
    if s.startswith(('generate_grass.','grass.','grass_lod','grass_object_push.')):return 'grass'
    if s.startswith(('billboard_forest.','billboard_plant.')):return 'clearing'
    if s.startswith(('demo_terrain_overlay.','mouse_terrain_raycast.','read_terrain_stroke.','timed_terrain_gizmo.')):return 'terrain_editing'
    if s.startswith(('leaf_particle','rain_particle','snow_particle','sparkles.','splash_particle.','star.')):return 'particle_effects'
    if s.startswith(('mei_shadow.','read_mouse_depth_value.')):return 'rendering_adapters'
    if s.startswith(('wall/','wall_')):return 'wall'
    if s.startswith(('roof/','roof_','instanced_roof_')):return 'roof'
    if s.startswith(('buildings/','rasterize_buildings','gauss_blur_building')):return 'buildings'
    if s.startswith(('nani/','nani_')):return 'nani'
    if s.startswith(('terrain_editing/','terrain_','stairs_foliage')):return 'terrain_editing'
    if s.startswith('rocky_terrain/'):return 'rocky_terrain'
    if s.startswith('water/'):return 'water_placement_and_waves'
    if s.startswith(('paths/','path_')):return 'paths'
    if s.startswith(('painterly/','vermeer/')):return 'painterly_and_vermeer'
    if s.startswith(('garden/','grass/','ivy/','clearing/')):return s.split('/')[0]
    if s.startswith(('critters/','sheep.')):return 'critters'
    return 'game_props'

signal=re.compile(r'wall|roof|brick|garden|grass|nani|pebble|ivy|stroke|pet_|landing|taking_off|wren|duck|bush|flower|contouring|foliage|cobblestone|dirt|dormer|halftimber|tile_bundle|grasslet|instance_indirection|atlas_part|mesh_histogram|filtered_instance|firefly|butterfly|wing|trunk|bark|ripple|highlight_active|eraser|brush_position|path_|paths_|terrain_|building|stairs_|water_mask|lilypad|reed|boat|waterwheel|stone_floor|progress_t|history_valid|world_subset|draw_list_index|merge_window|roof_profile|prim_center|prim_normals',re.I)
GENERIC=re.compile(r'/(sdf_utils|isolines|marching_squares\.inc|triangle_adjacency|arch_noise_discard|inc/trimesh)')
ADAPTER=re.compile(r'/(clear_|.*prefix.*\.cs|draw_indexed_indirect_to_dispatch|convolve_target|acquire_target|combine\.raster)|gauss_blur_building_mask')
items=[]
for record in DATA['sources']:
    s=record['source'];fam=family(s);proofs=record['proofs'];p=proofs[0] if proofs else None
    if s in EXISTING:
        items.append(EXISTING[s]);continue
    resources=[];fields=[];types=[]
    for pr in proofs:
        resources.extend(r for r in pr['resources'] if not r.isdecimal() and not r.startswith(('out.var.','in.var.C')) and r not in ['$Globals','frame_constants','frame_constants_storage_buffer'])
        for layout in pr['resource_layouts']:
            if layout.get('type_name') in ['FrameConstants','ViewConstants','type.ConstantBuffer.FrameConstants']:continue
            if layout.get('type_name'):types.append(layout['type_name'])
            fields.extend(m['name'] for m in layout.get('members',[]) if m.get('name'))
    extra_signal=re.compile(r'flag_patterns|flag_palettes|local_start_y|body_end_y|tip_end_y|is_corner|is_photomode|is_topdown_mode|SparkleInstanceData|leaf_placement|particle_emitter_count|total_particle_count|y_axis_stretch|x_axis_stretch|arrow_index',re.I)
    signals=list(dict.fromkeys(x for x in resources+fields+types if signal.search(x) or extra_signal.search(x)))
    if GENERIC.search(s) or s in ['shaders/mei_shadow.raster.hlsl','shaders/star.raster.hlsl']:
        status='generic_unresolved';confidence='medium';reason='原指令有通用几何/采样/遍历辅助实现，但本轮没有能识别精确外部源码的指纹；其接入游戏领域不证明算法源码原创。'
    elif ADAPTER.search(s):
        status='game_adapter';confidence='high' if p else 'medium';reason='通用清零、扫描、卷积、复制或间接参数操作接入本游戏的数据布局；标准算法不能据此判为开发者独创。'
    elif signals and p:
        status='game_specific';confidence='high';reason='原 OpLine 对应代码读取/操作具有游戏领域含义的资源或参数，支持游戏专属数据契约与实现策略；不推断全部数学公式原创。'
    elif p:
        status='game_adapter';confidence='medium';reason='原 OpLine 存在该源码的发射区域，其消费者有游戏领域实现；本文件的原名称/接口不足以判断算法作者，按游戏接入层分类。'
    else:
        status='game_adapter';confidence='low';reason='原容器确认该源码依赖，但没有该文件独立的可归属 OpLine 指令区；可识别游戏shader包装/声明接入，原始正文来源保留未知。'
    scope={'classification_unit':'source file integration/body regions','family':fam,'meaning_of_game_specific':'Game-specific contracts/behavior in shipped implementation; not a claim of original invention of every line or algorithm.','main_or_include':'include_or_declaration' if any(z in s for z in ['.inc.','/inc/','/inc_','/defs.','/user.']) else 'main_or_pipeline_helper','body_attributed_to_original_source':bool(p),'consumer_pipeline_count':record['pipeline_count']}
    proof={'basis':['original distribution raw SPIR-V','original OpString + OpLine source attribution','original OpName/OpMemberName and buffer layout'],'reason':reason,'game_specific_signals':signals[:32],'resources_in_source_regions':list(dict.fromkeys(resources))[:32],'consumer_examples':record['consumers'][:4],'raw_examples':[]}
    for pr in proofs[:2]:
        proof['raw_examples'].append({k:pr[k] for k in ['raw_file','raw_sha256','stage','spirv_sha256','source_line_min','source_line_max','line_count','attributed_instruction_count','access_examples']})
    items.append({'source':s,'status':status,'origin':'Tiny Glade game integration; individual formula/source ancestry unspecified' if status!='generic_unresolved' else 'Unresolved generic helper ancestry','confidence':confidence,'scope':scope,'proof':proof,'limitations':['No recovered HLSL comments/README used as provenance evidence.','Original source text was stripped; game_specific does not certify every line authored from scratch.','Confirmed open-source helper calls/includes are separate emitted regions; dependency closure alone is not actual code contribution.'],'upstream_url':None,'proof_file':'raw_game_evidence.json'})
(OUT/'classification.json').write_text(json.dumps({'files':items,'summary':dict(collections.Counter(x['status'] for x in items)),'classification_boundary':DATA['limitations']},indent=2,ensure_ascii=False),encoding='utf-8')
print(json.dumps({'sources':len(items),'status_counts':dict(collections.Counter(x['status'] for x in items))},ensure_ascii=False))
