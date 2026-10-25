"""One-time 2026-10-13 source-ledger import. Refuses to overwrite maintained claims."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'tracking/module-progress-overrides.json'
if OUT.exists():
    raise SystemExit('Maintained overrides already exist; update them directly, never re-import over them.')
source_paths = {r['path'] for r in json.loads((ROOT / 'tracking/version-map.json').read_text(encoding='utf-8'))['source_files']}
config = {
    'schema_version': 1, 'baseline_date': '2026-10-13',
    'policy': '原crate/目录/文件为进度主键；实现、接线和验证独立；不推算整游戏百分比；历史报告不自动升级为当前验收。',
    'modules': [], 'file_bindings': [],
    'candidate_mappings': ['evidence/abc-tools-52-2026-10-11/mapping.json', 'evidence/abc-building-35-2026-10-12/mapping.json'],
}


def add(module, scope, files, functions, gaps, verification, evidence, integration='not_established', status='candidate_partial', next_action=None, ownership='private_game_source'):
    config['modules'].append({'module': module, 'status': status, 'ownership': ownership,
                             'scope': scope, 'restoration_globs': files, 'functions': functions,
                             'gaps': gaps, 'verification': verification, 'evidence': evidence,
                             'integration': integration, 'next_action': next_action or gaps[0]})


def bind(original, local, scope=None, evidence=None, basis='reviewed_partial_module_mapping'):
    if original not in source_paths:
        raise ValueError('Original path absent: ' + original)
    row = {'original': original, 'restoration': local, 'basis': basis}
    if scope:
        row.update(verification_scope=scope, evidence=evidence or [])
    config['file_bindings'].append(row)


cc = 'crates/country-core/src/'
ss = 'crates/systems/'
be = 'evidence/'
br = 'reconstruction/building/'
runtime = 'reconstruction/crates/tg-building-runtime/src/'
roofev = ['evidence/roof-surface/verification.json', 'evidence/roof-ridge/verification.json', 'evidence/multi-version/B-parameters-verification.json']
add(cc+'resources/roofs', 'Roof88、参数/形状、曲面/Curve2、脊线、方向与Flat/Rim部分消费者已恢复。',
    ['reconstruction/roof/{surface,ridge,roof_state,parameters}.rs'],
    ['Roof::new/ty/is_gable', 'profile/截面/Curve2构造、采样、导数和切线', 'RoofRidgeDims、roof_tip_offset_xz、世界点/线段脊线', 'Width/Length参数映射与檐口计算'],
    ['roof_noise/buffers/trigger_volume/tile_sim等全部字段与消费者未闭合', 'A/C布局及完整ABI单独核实', 'ARM矩形case14 filler Y仍有1ULP历史失败'],
    '限定原码报告：surface23810、ridge14776；参数与状态有独立对照。不是全部11文件验收。', roofev,
    '部分由真实Bevy建筑runtime消费', 'integrated_partial')
# Python Path.glob has no brace expansion; expand recorded input patterns explicitly.
config['modules'][-1]['restoration_globs'] = ['reconstruction/roof/'+x+'.rs' for x in ('surface','ridge','roof_state','parameters')]
for orig, locals_ in [('roof_shape',['roof_state','surface','parameters']),('roof_ridge',['ridge']),('roof_visual_shape',['surface'])]:
    for local in locals_:
        bind(cc+'resources/roofs/'+orig+'.rs', 'reconstruction/roof/'+local+'.rs',
             '仅列出的状态/曲面/参数/脊线局部范围；非整个原文件', roofev)

add(cc+'resources/walls', 'Public/InnerWall投影、开口存储、subtype、装饰字段与部分附着候选。',
    [br+x+'.rs' for x in ('wall_state','wall_height_authority','wall_holes','decorator_subtype','decorator_info','wall_decorator_archivist')]+['reconstruction/wall/height_rules.rs'],
    ['PublicWalls::get合法性读取', 'max_y/max_y_gable/flat_roof_y', 'WallHoleStorage周期副本/finalize/cache', 'derive_decorator_subtype和Info72字段投影'],
    ['未知published字段及PublicWall+296生产者', '全部资源字段/布局和wall_attachment跨版ABI', '新Info/storage绑定未统一验收'],
    '高度、hole、subtype有原码局部证明；Info和部分storage为新增候选，不能整体通过。',
    [be+'scope-priorities/wall-rules-verification.json',be+'building-holes/storage-B-verification.json',be+'building-placement/B-subtype-verification.json',be+'building-decorator-info/findings.md'],
    '已验收子集接真实World，新增字段另列未验证', 'integrated_partial')
for orig, local in [('public_wall_state','wall_state'),('public_walls','wall_state'),('inner_wall_state','wall_height_authority'),('wall_holes','wall_holes'),('decorator_subtype','decorator_subtype'),('decorator','decorator_info'),('decorator_archivist','wall_decorator_archivist')]:
    bind(cc+'resources/walls/'+orig+'.rs', br+local+'.rs')
for orig in ('public_wall_state','public_walls'):
    bind(cc+'resources/walls/'+orig+'.rs','reconstruction/wall/height_rules.rs','3个缓存叶函数及数值后段；非整个墙系统',[be+'scope-priorities/wall-rules-verification.json'])

wallfiles = ['wall_collection','wall_publication','wall_spatial_index','wall_cache','wall_freeform_cache','wall_gable','wall_authoring','wall_height_command','wall_bulldoze','wall_rectangle_edit','wall_pending_changed','wall_hole_invalidation','roof_associations']
add(cc+'systems/wall', 'raw Curve2→terrain/raster/prepared→cache/publication→Roof/openings/brick消费者；创建、高度、删除/bulldoze及部分矩形编辑。',
    [br+x+'.rs' for x in wallfiles]+[runtime+x+'.rs' for x in ('wall_authoring','wall_height','wall_bulldoze','wall_rectangle_edit','wall_pending_changed')],
    ['publish_public_wall_state/采集与dirty更新', 'circle/rectangle/freeform缓存和逐arch gable', 'AdjustWallHeightCmd读取、clamp、authority/audio', '创建/直接删除/animate_wall_bulldoze', '矩形finish/cancel/enclosure及PendingWallChanged候选'],
    ['完整UI placement/startup、snapping/merge/splitter未恢复', '新矩形/Enclosure等整批尚未统一验证', '原fork排程与跨writer顺序'],
    '阶段4冻结54测试/67限定证明；新增并行批不继承冻结验收。',
    [be+'building-integrate/stage-reviews/2026-10-11-stage4-frozen.md',be+'building-integrate/stage-reviews/stage4-frozen/summary.json',be+'building-wall-history/findings.md'],
    '已验收链为真实Bevy World；新增链implemented_unverified', 'integrated_partial')
for orig, local in [('wall_state/publish_public_wall_state','wall_publication'),('wall_state/wall_spatial_hash','wall_spatial_index'),('wall_state/spatial_hash_update','wall_spatial_index'),('wall_state/wall_roof_hashmap','roof_associations'),('shapes/calc_shape_height','wall_freeform_cache'),('draw_wall','wall_authoring'),('finish_wall','wall_authoring'),('delete_wall','wall_authoring'),('adjust_wall_height/adjust_wall_height_impl','wall_height_command'),('eraser/animate_wall_bulldoze','wall_bulldoze'),('shapes/rectangle/edit_rectangle','wall_rectangle_edit'),('shapes/rectangle/finish_edit_rectangle','wall_rectangle_edit')]:
    bind(cc+'systems/wall/'+orig+'.rs', br+local+'.rs')

add(cc+'resources/history', 'A/B/C历史格式解析、Wall/Roof事务和游戏命令字段；其他命令保留未知值。',
    ['reconstruction/crates/tg-history/src/lib.rs','reconstruction/history/transaction.rs','reconstruction/history/building_transactions.rs',br+'wall_history_edit.rs'],
    ['History start/add/finish/cursor、分叉截redo', 'Roof五类owner和Wall compound edit', '原历史大整数ID/未知tag保留'],
    ['Path/Garden/Color/Stairs等完整owner和schema消费者', '快照解码、22缺失快照引用与C迁移'],
    '历史静态样本与Rust解析有证明，建筑事务限定链通过；不等于完整存档。',
    [be+'multi-version/history-rust-verification.json',be+'building-history/B-transaction-verification.json',be+'building-history-owners/B-owner-verification.json'],
    '建筑已验收owner接World，其余未建立', 'integrated_partial')
for orig, local in [('mod','reconstruction/history/transaction.rs'),('roof_edit','reconstruction/roof/replay.rs'),('wall_compound_edit',br+'wall_history_edit.rs'),('wall_history',br+'wall_history_edit.rs')]:
    bind(cc+'resources/history/'+orig+'.rs',local)

add(cc+'systems/history', 'Wall/Roof外层bucket与同Wall组barrier、计时和离散回放；terrain/tree/water/auto-clutter局部链。',
    [br+'wall_history_replay.rs',br+'wall_history_transition.rs',runtime+'history.rs','reconstruction/roof/replay.rs','reconstruction/terrain/crates/tg-terrain/src/replay.rs','reconstruction/tree-tool/src/replay_tree.rs','reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/auto_clutter_edit.rs'],
    ['replay_wall/replay_pop_bucket限定范围', 'Roof VisualEdit/create/delete/direction/rim', 'replay_terrain Edit', 'replay_tree/replay_water/replay_raster/replay_auto_clutter候选'],
    ['全部命令owner统一调度、失败展开、snapshot联接', 'Garden/Path/Stairs/Color完整消费者'],
    '建筑冻结链通过；terrain Edit512原作↔Rust；其他新回放候选范围独立。',
    [be+'building-integrate/stage-reviews/2026-10-11-stage4-frozen.md',be+'multi-version/B-replay-verification.json',be+'terrain-editing/milestones/M1/status.md',be+'multi-baseline/README.md'],
    '建筑World已接，独立terrain/tree/clutter不冒称统一游戏接线', 'integrated_partial')
for orig, local in [('replay_wall',br+'wall_history_replay.rs'),('replay_pop_bucket',br+'wall_history_replay.rs'),('history_process','reconstruction/history/transaction.rs'),('replay_terrain','reconstruction/terrain/crates/tg-terrain/src/replay.rs'),('replay_auto_clutter','reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/auto_clutter_edit.rs')]:
    bind(cc+'systems/history/'+orig+'.rs',local)
add(cc+'systems/save','仅建立原文件清单，未登记完整恢复。',[],[],['save_game/load_game/snapshot/snapshot_gc完整状态、格式、后台消费者与错误恢复'], '未建立模块验收。',[],status='not_assessed')
add(cc+'systems/terrain','Stroke schema、点列/ID、已解析编辑与Edit回放；重算/读回事件链已分析。',
    ['reconstruction/terrain/crates/tg-terrain/src/'+x+'.rs' for x in ('schema','stroke','edit','replay','readback')],
    ['stroke点列维护和ID上限', 'Edit Delta/Absolute/Keep规则', 'Edit正反replay', 'delayed_terrain_changed_msg/readback聚合通知'],
    ['New/Move全入口与UI、pending history、真实ECS', '非有限边界、GPU rasterize/readback同步、dirty消费者全链'],
    'M1报告Edit512原作↔Rust；Delta512是早期原片段↔公式，不能升格完整Rust入口。',
    [be+'terrain-editing/milestones/M1/status.md',be+'terrain-editing/second-batch-findings.md',be+'terrain-editing/B-delta-edit-verification.json'], status='candidate_partial')
bind(cc+'systems/terrain/edit_terrain_stroke.rs','reconstruction/terrain/crates/tg-terrain/src/edit.rs')
bind(cc+'resources/terrain_editing.rs','reconstruction/terrain/crates/tg-terrain/src/stroke.rs')
bind(cc+'resources/terrain_stroke.rs','reconstruction/terrain/crates/tg-terrain/src/schema.rs')

add(cc+'systems/camera_rig','normal/free/walk/butterfly/camera_body候选源码；不是完整MainCamera。',
    ['reconstruction/camera/*.rs'], ['camera_body/normal/free/walk/butterfly的候选更新函数'],
    ['完整input/mode transition和MainCamera编排', '真实terrain/collision producer与跳表复核、C适配'],
    '构造器native返回已观察；Rust完整差分/系统未验收。',
    ['tracking/parallel-modules-stopped-snapshot.json',be+'parallel-camera/B-constructor-native.json'],status='draft_partial')
for file in ('normal','free','walk','butterfly','camera_body'):
    bind(cc+'systems/camera_rig/'+file+'.rs','reconstruction/camera/'+file+'.rs',basis='same_source_submodule_candidate')
add(cc+'systems/audio','16事件注册；wall/building/stairs/music/menu/sheep/clutter/water局部事件映射。',
    ['reconstruction/audio/src/*.rs','reconstruction/clutter-audio/crates/tg-clutter-audio/src/audio/*.rs'],
    ['add_audio_events', 'wall/building/stairs kind→FMOD参数', 'music_audio_system richness/time规则', 'update/listener/volume候选'],
    ['FMOD资源类型/实例和真实后端接线', '部分kind、资源字段和环境音公式未钉死；占位不得计恢复'],
    'clutter+audio候选check/build/test5/5；非FMOD运行或原作完整音频等价。',
    [be+'multi-baseline/compile-clutter-audio.json',be+'multi-baseline/audio-events-analysis.md',be+'multi-baseline/audio-extra-analysis.md'],status='candidate_partial')
add(cc+'render','pass_builder、SSBO/Shadowed/SparseBuffer、Mesh/Shader、nani GPU-world/atlas/实例与21 ECS系统有交付记录。',
    ['reconstruction/country-core/src/render/**/*.rs'],
    ['ShaderUri::parse、ComputePass/DrawBuilder全族', 'TypedStorageBuffer/ShadowedBuffer/SparseInstanceBuffer', 'GpuWorld/MeshAtlas/实例更新及nani_flush_instance_updates'],
    ['记录中的country-core/country-core-render交付在当前工作区缺失，先同步并复核', 'pass_resource_access/draw_stream managed/temporal边界、component→T多数字段映射'],
    '项目记忆报告offline check/clippy通过；本机源缺失，不计本机已复核实现。',
    ['PROJECT_MEMORY.md','reconstruction/country-core-render/_integration/INTEGRATION_REPORT.md'],status='reported_missing',next_action='找回原交付与合同，核对身份/源码hash，再接pass到rhapsody真实Command与managed资源')
add(cc+'systems/render_loop','render_world帧顺序、54pass调用、8条件分支及CPU组合有分析/交付记录。',
    ['reconstruction/country-core/src/systems/render_loop/**/*.rs'],
    ['render_world', 'HZB/GI/透明/DoF/TAA或TGSR及呈现调用顺序'],
    ['CPU交付源码本机缺失', '逐pass变体、descriptor/resource handle、GPU时序与整帧画面对照'],
    'B原二进制+404shader静态分析；没有整帧GPU验收。',
    [be+'render-passes/INDEX.md','PROJECT_MEMORY.md'],status='reported_missing')
add(cc+'asset_libraries','实际decorator collision JSON读取与mesh/shader library生产者边界已有局部恢复/分析。',
    [runtime+'decorator_assets.rs',br+'decorator_placement.rs'],
    ['load_decorators/实际collision bounds', 'AssetShaderLibrary四个add/get生产者及raster创建分析'],
    ['完整asset/pass关联、缓存淘汰和资源所有权', '新country-core库接线交付缺失'],
    '18键资产放置等限定World链；ShaderLibrary只读分析非完整Rust恢复。',
    [be+'building-integrate/findings.md',be+'render-recovery/consumer-chain-review-2026-10-12/README.md'], integration='decorator资产子集已接World',status='integrated_partial')
add(cc+'resources','地形、水、树存储、critters/window lights等跨系统资源有局部字段证据；不是整个resources目录恢复。',
    ['reconstruction/terrain/crates/tg-terrain/src/water.rs','reconstruction/tree-tool/src/process_tree_cmds.rs'],
    ['TreeStorage get/insert/remove与next_index', 'WorldRaster<Water> CPU状态与格点映射', 'TerrainHeightsData双实例字段分析'],
    ['每个资源精确字段/原类型和生产者仍待对齐', 'WP7–10 country-core消费资源源码本机缺失'],
    '水模块候选9测试；其他资源主要分析或缺失交付记录。',
    [be+'multi-baseline/README.md','PROJECT_MEMORY.md'],status='candidate_partial')
bind(cc+'resources/tree_storage.rs','reconstruction/tree-tool/src/process_tree_cmds.rs')

add(ss+'roof/src','create/edit/delete/spatial/replay和类型/动画已接World；CPU铺瓦完整核心。',
    ['reconstruction/roof/*.rs']+[br+'roof_'+x+'.rs' for x in ('creation','spatial','components','associations','deletion','lifecycle','direction','customization','invalid_notifications')]+[runtime+'roof_edits.rs',runtime+'roof_invalid.rs'],
    ['create_roof、edit_roof、delete_roof、update_roof_spatial', 'replay_roof/五类owner', 'assemble_circular_roof/assemble_rectangular_roof', 'switch_roof_type和屋顶动画/移除'],
    ['roof_trigger_volume实际几何、place_spires/place_support_beams、snow与瓦片音频模拟完整消费者', 'UI输入与完整视觉所有权', 'ARM矩形case14一处1ULP历史失败'],
    '矩形33输入19024记录/圆形37输入11467记录历史报告；编辑/生命周期/World链各自范围独立。',
    [be+'roof-pipeline-verification.json',be+'roof-tiles/circular-verification.json',be+'source-edits/verification.json',be+'building-integrate/stage-reviews/2026-10-11-stage4-frozen.md'],
    '真实Bevy建筑runtime已接验收子集；GPU/完整系统未验收','integrated_partial')
for orig, local in [('create_roof',br+'roof_creation.rs'),('edit_roof','reconstruction/roof/edit_rules.rs'),('delete_roof',br+'roof_deletion.rs'),('update_roof_spatial',br+'roof_spatial.rs'),('replay_roof','reconstruction/roof/replay.rs'),('utils/assemble_circular_roof','reconstruction/roof/tiles.rs'),('utils/assemble_rectangular_roof','reconstruction/roof/pipeline.rs'),('visual/switch_roof_type',runtime+'roof_edits.rs'),('visual/roof_animation',br+'roof_components.rs')]:
    bind(ss+'roof/src/'+orig+'.rs',local)
for orig, local, ev, scope in [('edit_roof','reconstruction/roof/edit_rules.rs',be+'source-edits/verification.json','4514已解析编辑后段案例，不含完整原ECS前段'),('utils/assemble_circular_roof','reconstruction/roof/tiles.rs',be+'roof-tiles/circular-verification.json','37输入，25prepared/12完整入口，限定CPU记录'),('utils/assemble_rectangular_roof','reconstruction/roof/pipeline.rs',be+'roof-pipeline-verification.json','历史Windows33完整CPU输入；ARM1ULP失败单列')]:
    bind(ss+'roof/src/'+orig+'.rs',local,scope,[ev])

wallroot=ss+'wall-constructor/src'
add(wallroot,'construct_walls/WallConstructor与noise/seed/authority walker→96B砖输出已恢复并接World。',
    [br+x+'.rs' for x in ('wall_constructor','wall_production','wall_construction_sources','wall_path_preparation')]+[runtime+'wall_bricks.rs'],
    ['WallConstructor::from_curve', 'construct_walls输入策略、proposal与96B实例输出', 'noise/seed/height/开口裁切与顺序'],
    ['所有结构family和完整原资源消费者', '原ECS/fork排程及C迁移'],
    '普通85/其他304限定入口及连续World链；冻结后新增源码不继承旧报告。',
    [be+'building-constructor/B-wall-constructor-verification.json',be+'building-production-authority/findings.md',be+'building-integrate/stage-reviews/2026-10-11-stage4-frozen.md'],
    '真实raw/prepared/cache→brick World链', 'integrated_partial')
for orig,local in [('wall_constructor','wall_constructor'),('construct_walls','wall_production')]:
    bind(wallroot+'/'+orig+'.rs',br+local+'.rs')
families=[
 ('segmentation',['wall_segmentation','wall_platform_segmentation','wall_path_preparation'],['calculate_wall_path_segmentation','sample_wall_path_intersections','split_wall_path_segment_into_platform_stairs'], '完整路径mask/material-stat已验限定范围；更多形状/消费者', 'building-segmentation/findings.md','integrated_partial'),
 ('construct_arches',['wall_arches','wall_arches_freehand'],['construct_arches/profile','regular/freeform arch输出、final arch/third和120B ConstructGateCmd'], '更多支撑族、门命令最终consumer', 'building-arches-freehand/findings.md','integrated_partial'),
 ('construct_lintels',['wall_structures'],['construct_lintels/place_support_beams限定CPU链'], '完整梁/悬挑/资源生命周期', 'building-structures/findings.md','integrated_partial'),
 ('construct_platform_stairs',['wall_platform_stairs','wall_platform_stair_rows','wall_platform_collision','wall_platform_rapier_backend'],['80B装饰源→cells/碰撞/96B记录/holes/stats'], '扩大consumer输入边界；新Rapier接线整批未验收', 'building-platform-stairs/B-rapier-backend-verification.json','integrated_partial'),
 ('construct_gates',['gates'],['Gate120→CPU木门/支撑/post fallback、候选mesh/entity/Nani生命周期'], '新增整批未统一验收；原MeshLib/ShaderLib/SSBO所有权', 'building-gates/findings.md','candidate_partial'),
 ('construct_fences',['fences'],['construct_fences/process_segment部分候选'], '完整floor/fence消费链及新代码验收', 'building-fences/findings.md','candidate_partial'),
 ('construct_elevation_supports',[],['construct_brick_columns定位/局部helper候选'], '圆/矩形柱、托架、turret、碰撞完整family', 'building-structures/findings.md','analysis_partial'),
 ('utils',['wall_constructor'],['resolve_hole_overlap、trim_rows、split_into_random_rows限定策略'], 'wall_corners/wall_skirts、half_timber与全部helper来源归属', 'building-constructor/B-wall-constructor-verification.json','candidate_partial')]
for name, locals_, funcs, gap, ev, state in families:
    add(wallroot+'/'+name,'；'.join(funcs),[br+x+'.rs' for x in locals_],funcs,[gap,'未建立整个子目录逐函数等价验收'],
        '仅所列局部/连续链报告；不继承其他family验收。',[be+ev],integration='部分World链或候选，按所列范围',status=state)
for orig,local in [('construct_arches/mod','wall_arches'),('construct_arches/profile','wall_arches'),('construct_lintels/mod','wall_structures'),('construct_platform_stairs/mod','wall_platform_stairs'),('construct_gates/mod','gates'),('construct_gates/systems','gates'),('construct_fences/mod','fences'),('construct_fences/process_segment','fences'),('segmentation/calculate_wall_path_segmentation','wall_segmentation'),('segmentation/sample_wall_path_intersections','wall_path_preparation'),('segmentation/split_wall_path_segment_into_platform_stairs','wall_platform_segmentation')]:
    bind(wallroot+'/'+orig+'.rs',br+local+'.rs')

add(ss+'decorator/src','墙面四族asset/subtype/position→raw holes→wall brick；Info/storage与部分stairs候选。',
    [br+x+'.rs' for x in ('decorator_placement','decorator_subtype','decorator_info','opening_rules','wall_decorator_archivist')]+[runtime+x+'.rs' for x in ('decorator_assets','decorator_placement','decorator_info_binding','openings')],
    ['load_decorators/WallSpaceCoord定位', 'derive_decorator_subtype、四族raw-hole producer', 'apply_decorator_op/archivist/storage部分候选'],
    ['全部decorator_visual家族：dormer/chimney/corner/flag/clothline/lantern等', '真实anchor/拖动与roof宿主形变、跨对象恢复'],
    'placement1668/subtype1326等限定报告；新增Info/storage不冒称当前整批通过。',
    [be+'building-placement/B-placement-verification.json',be+'building-placement/B-subtype-verification.json',be+'building-integrate/findings.md'],integration='墙面子集真实World已接',status='integrated_partial')

add(ss+'tree-tool/src','六个文件的命令/删除/回放/地形跟随/弯曲/预置树恢复草稿。',
    ['reconstruction/tree-tool/src/*.rs'],
    ['process_tree_cmds Move→Insert→Delete', 'despawn_trees/replay_tree', 'recalc_tree_position_on_terrain_height_change/tree_move_bend/convert_preplaced_trees'],
    ['PlaceTreeCmd载荷和TreeEdit跨系统判别值', '完整UI/startup/hide_trees及真实World consumer'],
    'reva/RNE反汇编交叉核对与语义草稿；未登记完整编译/原码差分/系统验收。',
    [be+'multi-baseline/tree-process-cmds-analysis.md',be+'multi-baseline/tree-tool-rest-analysis.md',be+'multi-baseline/tree-tool-extra-analysis.md'],status='draft_partial')
for name in ('process_tree_cmds','replay_tree','recalc_tree_position_on_terrain_height_change','tree_move_bend','convert_preplaced_trees'):
    bind(ss+'tree-tool/src/'+name+'.rs','reconstruction/tree-tool/src/'+name+'.rs',basis='same_source_submodule_candidate')

add(ss+'water/src','WaterRaster CPU状态、world_raster/replay/上传调用、water_audio_grid。',
    ['reconstruction/terrain/crates/tg-terrain/src/water.rs','reconstruction/terrain/crates/tg-terrain/src/water_audio_grid.rs'],
    ['draw_water工具分派分析', 'startup_water、water_rasterize_from_cpu、replay_raster/replay_water', 'world→512网格映射、水位u8/255、audio-grid更新'],
    ['zeno闭包内部、mask SSBO字段和完整draw/history producer', '真实GPU上传、water-render consumer与系统生命周期'],
    '五模块README记录候选9测试；没有完整水域原作运行等价。',
    [be+'multi-baseline/README.md',be+'multi-baseline/water-cpu-analysis.md',be+'multi-baseline/water-extra-analysis.md'],status='candidate_partial')
for orig,local in [('draw_water','water'),('record_water_history','water'),('water_audio_grid','water_audio_grid')]:
    bind(ss+'water/src/'+orig+'.rs','reconstruction/terrain/crates/tg-terrain/src/'+local+'.rs')

add(ss+'critters/src','duck/shared/sheep terrain/wren boids/fish/butterfly候选；firefly和sheep pet独立局部。',
    ['reconstruction/critters/src/*.rs','reconstruction/critters/firefly.rs','reconstruction/critters/sheep_pet*.rs'],
    ['duck plan→move→goals→velocities→ripples', 'shared避让/移动、sheep_walk_on_terrain', 'sim_wren_boids、fish/butterfly更新', 'firefly goal/velocity/smoothing与sheep pet局部'],
    ['CritterLocomotion物种映射、全startup/delete/history/建筑反应', 'firefly2102案例中一处NaN符号差异保留', '真实World/GPU与C物种全链'],
    '新harness记录22测试；firefly原码报告有失败，不记全通过；其他物种新源码未等价验收。',
    [be+'multi-baseline/README.md',be+'multi-baseline/critters-behavior-analysis.md',be+'multi-baseline/critters-extra-analysis.md','tracking/parallel-modules-stopped-snapshot.json'],status='candidate_partial')
for orig,local in [('duck','src/duck'),('shared','src/shared'),('fish','src/fish'),('butterfly','src/butterfly'),('wren_boids','src/wren_boids'),('sheep/sheep_terrain','src/sheep_terrain'),('firefly','firefly')]:
    bind(ss+'critters/src/'+orig+'.rs','reconstruction/critters/'+local+'.rs',basis='same_source_submodule_candidate')

add(ss+'clutter/src','autoclutter区域/筛选/放置/实体管理、merlons/stitches、auto-clutter历史草稿接可编译crate。',
    ['reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/**/*.rs'],
    ['populate_autoclutter_regions/process_autoclutter_candidates', 'add_* gates/windows/birdnests/lantern/tree', 'manage_autoclutter_entities', 'detect/construct_inter_roof_merlons、stitch_bricks', 'AutoClutterEdit/replay与删除记录'],
    ['BrickTransformSsbo/StairsFloatingBrickParams字段与真实资源producer', '22 TODO和3空位；动画/mesh加载/GPU bush读回与完整调度'],
    'clutter+audio check/build/test5/5，使用glue占位；未通过原作完整行为验证。',
    [be+'multi-baseline/compile-clutter-audio.json',be+'multi-baseline/clutter-placement-analysis.md',be+'multi-baseline/clutter-extra-analysis.md'],status='candidate_partial')
for orig,local in [('autoclutter/populate_autoclutter_regions','autoclutter/populate_autoclutter_regions'),('autoclutter/process_autoclutter_candidates','autoclutter/process_autoclutter_candidates'),('autoclutter/manage_autoclutter_entities','autoclutter/manage_autoclutter_entities'),('inter_roof_merlons/construct_inter_roof_merlons','merlons'),('inter_roof_merlons/detect_inter_roof_merlons','merlons'),('inter_shape_stitches/stitch_bricks','stitches')]:
    bind(ss+'clutter/src/'+orig+'.rs','reconstruction/clutter-audio/crates/tg-clutter-audio/src/clutter/'+local+'.rs')

rp='crates/rhapsody/src/'
re='reconstruction/rhapsody-rebuild/'
common_ev=[re+'MODULE_STATUS.md',re+'evidence/missing-functions-completion-2026-10-13.json',re+'evidence/functional-project-validation.json']
add(rp+'raw','设备、buffer/image/view/sampler、descriptor/pipeline/attachments、command/submit/WSI/诊断/格式候选可运行。',
    [re+'src/raw/*.rs'],
    ['GraphicsDevice资源创建/屏障/扩展和create_raw_device', 'RasterPipelineOpts、descriptor/binding/shader/cache', 'CommandBuffer/SubmitInfo/SubmissionEngine/Queue', 'Swapchain acquire→submit→present及诊断/格式原入口'],
    ['get_setup_cb_submit_info/with_setup_cb、init_texture_data/update_texture、fullscreen exclusive', '原设备完整策略、allocator/vk-sync/winit分叉和全pipeline/pass关联'],
    '10/12候选统一64测试/13GPU探针；10/13补接口69测试和严格Clippy、未重跑GPU。报告当前性逐项检查。',
    common_ev+[re+'evidence/raw-resource-implementation.json',re+'evidence/shader-production-validation.json',re+'evidence/swapchain-functional-validation.json'],
    '真实Vulkan候选资源/WSI链；原完整ABI/行为未验收','integrated_partial')
add(rp+'raw/managed','handle/ref/generation/checksum、资源持有/CPU退役/GPU保护与managed资源包装。',
    [re+'src/raw/managed/*.rs'],
    ['GpuOwned全族、ManagedHandle/ManagedRef', 'ManagedResourceMeta/checksum/owning_device/get_maybe_tracked', 'managed buffer/texture/view/swapchain包装'],
    ['原全域producer/序号桶、真实游戏持有关系与性能', 'raw析构保守等待、全ABI和原complete disposal'],
    '限定原生命周期合同和候选测试/GPU链；新增接口不是原完整ABI恢复。',
    common_ev+[re+'evidence/managed-implementation.json',re+'evidence/pass-bindings-functional-validation.json'],
    '候选descriptor持有与GPU消费已接，provider主图桥仍两阶段','integrated_partial')
add(rp+'rg','graph/compiled/registry/pool、访问分析、barrier/record、executor/temporal/import-export/pass API。',
    [re+'src/rg/*.rs','reconstruction/render/execution.rs','reconstruction/render/graph.rs','reconstruction/render/graph_info.rs','reconstruction/render/resource_info.rs','reconstruction/render/resource_usage.rs'],
    ['ResourceInfo::calculate与用途转换', 'RenderGraph/BatchCompiler/CompiledGraph/Registry/Pool', 'RenderPassApi draw/dispatch九命令、recorder、executor', 'Parity/Temporal/import_export/ExecutedRenderGraph/PGO', 'ExecutionChain::execute_graph(_with_context)旧共享入口'],
    ['首个shader callback自动注入真实Command/managed descriptor并同一submit', '原并行/跨族ownership transfer、完整compile/record原入口', 'temporal淘汰/reset/drop、原游戏consumer'],
    '原CPU合同/叶函数限定差分；候选串行session/temporal GPU已运行；全图与原整帧未验收。',
    common_ev+[be+'render-recovery/verification.json',be+'render-recovery/consumer-chain-review-2026-10-12/context-entry-validation.json',re+'evidence/queue-graph-functional-validation.json',re+'evidence/temporal-functional-validation.json'],
    'rg候选到Vulkan串行功能链；旧context入口未自动迁入新工程','integrated_partial',
    '将游戏pass的registry与正在录制的Command接入首个真实shader callback，完成同一提交/回收')
add(rp.rstrip('/'),'crate根与leek入口包装已实现部分候选。',[re+'src/lib.rs',re+'src/leek.rs'],['leek wait_submits_done和顶层接口'],['全部原root/reexports/features/原API闭合'],
    '候选check/test/clippy；不能从根文件存在判断完整crate完成。',common_ev,status='candidate_partial')
for original in sorted(source_paths):
    if original.startswith(rp):
        local = re + 'src/' + original[len(rp):]
        if (ROOT/local).is_file():
            bind(original,local,basis='pdb_structure_aligned_candidate_path; not_original_text')

add('crates/utils/src','游戏自有random_splits与工具候选；通用库调用直接复用。',
    ['reconstruction/roof/tiles.rs'],['utils::random_splits/random_splits_into', '跨版本工具候选详见精确原文件映射'],
    ['52组候选未统一build/test，原签名/跨版ABI未验证', '其余helper先查定义归属，不能由泛型游戏类型判私有'],
    '屋顶CPU限定链证明私有随机策略；52候选只有冻结交付，没有行为验收。',
    [be+'roof-tiles/source-map.json',be+'abc-tools-52-2026-10-11/summary.json'],status='candidate_partial')
add('crates/utils/src/curve','Curve2/profile采样完整CPU子链与CurveU候选。',['reconstruction/roof/surface.rs'],
    ['Curve2构造/弧长/points_u/find/interpolate/tangent', 'CurveU/Circular等跨版候选'],
    ['全部Curve3/slice/异常调用者和候选ABI'], 'surface限定原码23810案例中包含Curve2链；不是整个curve目录。',
    [be+'roof-surface/verification.json',be+'abc-tools-52-2026-10-11/summary.json'],status='candidate_partial')
bind('crates/utils/src/curve/mod.rs','reconstruction/roof/surface.rs','限定Curve2 CPU链',[be+'roof-surface/verification.json'])
bind('crates/utils/src/lib.rs','reconstruction/roof/tiles.rs','私有random_splits调用策略，不包含RNG库实现',[be+'roof-tiles/dependency-boundary.json'])

# No proof is inferred for utility/UI/application crates merely indexed in the PDB.
for prefix in ['crates/asset/src','crates/asset-pipe/src','crates/paths/src','crates/tiny-glade/src','crates/tabloid/src',
               'crates/lang-lib/src','crates/tg-iced/src','crates/tg-text/src','crates/theme-defs/src','crates/theme-fetcher/src','crates/theme-server-proto/src']:
    add(prefix,'仅登记原源码结构；局部调用或适配不等于本crate实现。',[],[],['逐文件/函数恢复范围、类型和实际生产消费链尚未建立'],
        '未登记完整crate验证；源码索引不算恢复。',[],status='not_assessed')
for family in ('color','forest','glade-border','onboarding','timberframe','toolbar'):
    add(ss+family+'/src','索引/零散候选；完整系统范围尚未核对。',[],[],['完整文件/函数、状态/资源和生产者消费者尚未恢复'],
        '未登记完整模块验收。',[],status='not_assessed')
for family in ('gpu-allocator','conservative-line-raster','frozen','leekspin','tg-gpu-selector','shader-variants','turbosloth','usdl','usdl-sys'):
    add('crates/'+family+'/src','先核实Cargo/PDB定义与公开来源；不默认local包都是私有任务。',[],[],['来源/必要分叉差异待核实，公共实现复用'],
        '依赖实现不计游戏逆向覆盖。',['DEPENDENCY_FILTERING.md'],status='dependency_boundary',ownership='dependency_or_unresolved_local_ownership')

config['shader_layer']={
    'root':'reconstruction/shader-source/shaders/',
    'scope':'独立HLSL源码层：404/404原bin有对应重建main；260main/共享include及大算法族已重建。对应覆盖不等于算法验收。',
    'verification':'全树汇总258/260编译、235/260结构关口；两个Affine3Packed冲突后续记录已修复，未见更新后的全树汇总。原码指纹、变体和GPU验证分别看待。',
    'gaps':['17 capability、5 binding和1 local_size汇总残差需逐项核实，不能全归编译器差异','PBA/f0ad变体/大PS结构残差','Shader源码→游戏pass→rhapsody pipeline/descriptor→真实整帧与画面对照'],
    'evidence':[be+'render-passes/integration/wave2-final/completion.json',be+'render-passes/RECOVERY_TRACKER.md'],
    'next_action':'保留历史汇总，先关闭绑定/工作组等语义结构差异，再接最小真实cull/indirect/shadow/main消费链'}
OUT.write_text(json.dumps(config,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'modules':len(config['modules']),'file_bindings':len(config['file_bindings']),'output':str(OUT)},ensure_ascii=False))
