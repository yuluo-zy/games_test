#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""Generate per-crate REVIEW.md checklists (pre-seeded baseline) for group B review.
   Run with: python -I _gen_groupB_review.py  (reads _groupB_inventory.json in same dir)
"""
import json
import os

BASE = os.path.dirname(os.path.abspath(__file__))
with open(os.path.join(BASE, '_groupB_inventory.json'), encoding='utf-8') as f:
    inv = json.load(f)

notes = {
 'critters': {
   'boids.rs': 'BoidFlock::remove 2 实例(swap_remove+despawn 语义, wren/fish) / frame_update 2 实例(call-site 常量: fish sep_r=0.8 align=0.0584 min/max=0.9|3.0/4.0|8.0; wren sep_r=1.3 align=0.1169 7/14 — RNE 0x142328340/0x140f91600 已抽验) / random_vel_change(fastrand+uniform_sample_cone)',
   'butterfly.rs': '常量已抽验: 0x142f67d14=0.8529, 0x142f67d24=-0.2835, 0x142f67d30=1.77; 跳表 DAT_142f68008/0x142f68018 handler 语义未精读(证据 unknown)',
   'duck.rs': '五段链 0x14230ff00->0x142312200->0x142313b30->0x142314470->0x142314b70; DuckState: 1=轨迹正向 2=自由游(set_vel 只处理==2) 3=轨迹反向末端 despawn; Duck+0xe0/0xe1/0xe4/0xe8/0xfc; 轨迹 Vec@+0x98/+0xa0 进度+0xec',
   'duck_splash_particle.rs': '事件消费链: DespawnDuckCmd 发/收双方 — 消费方待核(证据 unknown)',
   'ducks_react_to_being_inside_buildings.rs': '无既有重建, fresh 反编译',
   'firefly.rs': '惯性 bias: RNE 0x142ccfc58 打包 0x3f000000/0xbf000000 = 有目标+0.5/无目标-0.5 — 证据文档"校正1"(-1.0)是错的, reconstruction 草稿(-0.5)正确; set_vel 海拔下限 pos.y<=1.0 -> dir.y=max(dir.y,0.25)',
   'fish.rs': '出生 200 上限/每帧 20/625 网格[-52,52]; 陆面反弹 pos-=vel*dt, vel=-vel*min_speed; FishSpawnLocations::new 0x140f909f0; fish 0x140f90cd0',
   'lib.rs': 'impl$0::build 0x14067d440 插件注册(系统链顺序!)',
   'shared.rs': 'critters_avoid_collisions 0x1423179a0(legacy_spherecast 10槽序{0,1,3,7,6,4,11,2,12,13}, 球距<1.0保持/<0.1命中发DespawnDuckCmd) / smooth 0x142318c10(retention=powf(rps,dt)) / move 0x142318fa0(查询签名 7 列, y=max(y,0), rot=powf(0.01,dt) 混合, legacy_spherecast)',
   'sheep/mod.rs': 'impl$0::build 0x14067d810',
   'sheep/sheep_behavior.rs': 'update_sheep_goals 0x142315050(Sheep+8==1 跟随: MainCamera mode==3 eye+forward*2 否则光标; +0x30 字节共享刷新; dist2>=0.16) / set_sheep_velocities 0x142315e40 / update_sheep_audio 0x142316490 未精读',
   'sheep/sheep_floatie.rs': '0x142320cf0 未精读',
   'sheep/sheep_input_record_replay.rs': '0x14231b720 未精读',
   'sheep/sheep_pet_animation.rs': '0x14231ac20; 对照 reconstruction/sheep_pet*.rs(C 基线 0x14293a350 五相动画, +0x58 amplitude>0.01=being_petted 已证实)',
   'sheep/sheep_react_to_being_inside_buildings.rs': '0x14231f610/0x14231ff70/0x142320480 未精读',
   'sheep/sheep_terrain.rs': '0x14231bd80: state 2=吸附/4=过渡(lerp+水采样<=0->2)/6=浮空(dy>-0.05->2)/default 行走(+4.0 偏移, clamp[-2,2]/[-5,5], 增益10x3, smooth=1-exp(-3dt)); 常量已抽验',
   'sheep/sheep_umbrella.rs': '0x142321980 未精读',
   'sheep/sheep_unstuck.rs': '0x1423246b0 未精读(消费 legacy_spherecast)',
   'sheep/sheep_walk_animation.rs': '0x14231a630 未精读',
   'sheep/ui_pet_sheep.rs': '0x142324f80 未精读',
   'spawn_ducks.rs': 'DuckSpawnLocations::new 0x140f8ee90(10x10=100 点, x in [-52,52], z 10档) / spawn 0x140f8f210(门控非demo+非corrupted, 活鸭<15 3x速率, timer>=30) / despawn 0x140f8f8b0(DuckState==2 深/浅水越界->离场计划)',
   'ui_pet_ducks.rs': '0x142322e30 未精读',
   'wren.rs': 'update_decorator_bird_spots 0x14231c7d0 / update_roof_bird_spots 0x14231d6c0(BirdSpotManager::create_spots/recycle 消费) / update_wrens 0x14231e390',
   'wren_boids.rs': 'sim_wren_boids 0x142326a80: 80 上限, x=u*130-65 y=u*9+1, 摇树 rng*5+1, 半径 91.9/高 30 保留, RAY_DIRS 32 方向 closure_3; 调用点常量已抽验',
 },
 'clutter': {
   'autoclutter/add_autoclutter_around_gates.rs': '0x1422cc390: 每门 2 放置 gate_right/gate_left, 键 calculate_hash<tuple<&str,u32,WallId>>, 墙点数>2, 墙端中点+法向偏移',
   'autoclutter/add_autoclutter_around_windows.rs': '0x1422cefe0: 每窗 3xadd_region; extend_right_until 延伸; 键 <&str,i32,usize,u32>x1+<&str,usize,u32>x2',
   'autoclutter/add_autoclutter_on_windows.rs': '0x1422cd7d0: 跳表 0x143042370 分派, [R9-0x50]==6 判类型; 反编译质量差 — 需 disasm 精读',
   'autoclutter/add_birdnests.rs': '0x1422c9c70: Subscriber::take<DirtyTiles>->逐脏格 type=4 候选, 0x3e19999a=0.15 偏移, +0x60=1.0 缩放, HashMap<u64,bool> 去重; 发布者之一(DirtyTiles pub/sub)',
   'autoclutter/add_lantern_water_setdresssing.rs': '0x1422d2fa0: as_inner_mut WaterRaster(+0x298 双缓冲)+is_corrupted 门控; DecoratorStorage::iter_terrain_decorators; 每装饰物 5 次 i/5.0+fmodf jitter; 入水才放; 键 <u32,usize,&str>(lantern@0x143042d28)',
   'autoclutter/add_preplaced_autoclutter.rs': '0x141ce5900 未精读',
   'autoclutter/add_tree_vegetation_setdressing.rs': '0x1422d3800: is_dragging/is_startedx4/is_corruptedx4 门控; 每树 12 次 i/12.0; 键 <TreeStorageIndex,usize,&str>(tree@0x143042d91)',
   'autoclutter/autoclutter_animation.rs': '0x1422ccef0 未精读',
   'autoclutter/load_clutter_meshes.rs': '0x1422c0760 未精读(host_geometry 边界确认)',
   'autoclutter/manage_autoclutter_entities.rs': '0x1422d04e0: 现存/目标键集差集; spawn_empty+insert<ColorId>; insert<ClutterMarkedForDeletion>; 四元数朝向(两次叉积+归一); init_rng 抖动; byte@+0x30 分支(==2 或 -3 后 <4 且 !=1)',
   'autoclutter/mod.rs': 'AutoClutterState: 区域表 hashbrown@+0x60 stride 0x48(key@0, 基向量@0x8..0x40, u16@0x42, epoch byte@0x44=state+0x90); 候选 Vec@+0x20 stride 0x98; 待处理 u16@+0x30; 输出@+0x48 stride 0x68(+0x58 计数); add_region 0x140ee3fd0 / mark_dirty 0x140ee48c0 / remove_region_from_lazy_grid 0x140ee4a70 / flush 0x140ee4bf0 / register_required_components 0x140ee4ee0',
   'autoclutter/populate_autoclutter_regions.rs': '0x1422d6ed0(44KB 主驱动): remove_stale_regions retain->flushx2->DirtyTiles new_clear+Subscriber::take->逐位 mark_dirty->filter_clutter_object(水位双采样+terrain_rock_mask, Aabb2+iter_world_space_rect_cells)->switch 种类(==2 水位阈值 0.01)->is_halloween2024 门控; init_state 0x1422e1b40 / autoclutter_subscribe_to_tabloids 0x1422d6930',
   'autoclutter/process_autoclutter_candidates.rs': '0x1422ce260: from_iter 过滤->排序(insertion_sort_shift_left<0x15 否则 driftsort)->遍历: byte@+0x30==6 强制接受否则 Rectangle3d::overlaps_rectangle3d(0x140c98580) 重叠剔除->输出 0x68 记录',
   'autoclutter/read_bush_positions_from_gpu.rs': '0x140eebaa0(发布者之一) 未精读',
   'inter_roof_merlons/construct_inter_roof_merlons.rs': '0x1422d4870: 等距 0.46 城垛砖; ceilf(长/0.46); random_splits_into 切段; 抖动 ((r*.5-(1-r)*.5)*0.21*0.8+0.368)*.5+(宽基+0.23); 门控 is_halftimbered||flat_w_no_rim 然后 ^1; SparseInstanceBufferWriter push',
   'inter_roof_merlons/detect_inter_roof_merlons.rs': '0x141ce3a70: dirty_walls 事件汇入(hashbrown set extend OnWallDeleted/OnWallChanged/RecalculateRoofCmd -> 0x141cd16b0); is_flatx2 门控; roof_shape_at_intersection_y; local_374!=2 时 expandx2+intersect_shapes(0x140edd250); 收尾 WallHoles::add stride 0x28',
   'inter_roof_merlons/inter_roof_merlons_audio.rs': '0x1422d4230 未精读',
   'inter_shape_stitches/detect_intra_shape_corners.rs': '0x141cd1a90: 角点高=max_y+flag*(-0.67)(0xbf2b851f); retain Corner/HashMap<[WallId;2],Vec<Vec2>>/ShapeIntersectionHole; intersect_shapes 0x140edbf60; add_hole_at_shape_intersection 0x141cd3ce0(洞 stride 0x14, kind byte 5/1); get_param 0x141ce3430; dirty_walls 0x141cd16b0',
   'inter_shape_stitches/stitch_bricks.rs': 'spawn_stitches 0x141ce65b0(kind==1 且 +0x04<0.2, 段长 min(1000,2|长|)<=2, 抖动 (r*.5-(1-r)*.5)*0.1+0.4, Δ+(-0.1)) / spawn_stitch_bricks 0x141ce7190(clear->逐条 hash([WallId;2],usize)->push)',
   'lib.rs': 'impl$0::build 0x140652050 插件注册(系统链/chain/in_set 顺序!)',
 },
 'onboarding': {
   'exponential_back_off.rs': '0x142284800',
   'hint_camera_controls.rs': 'camera_pan_hint 0x14227d000 / camera_rotate_hint 0x14227d2f0 / hint_camera_enable_condition 0x14227c780 / hint_camera_display_condition 0x14227c910 / camera_zoom_hint 0x14227d680',
   'hint_camera_progress.rs': 'hint_camera_progress_update 0x14227bd30 / hint_camera_progress_reset 0x14227c070',
   'hint_clutter_focus_edit.rs': 'make_focus_or_camera_binding_hint 0x141c3de00 / hint_clutter_focused_edit_condition 0x141c3d890 / hint_clutter_focused_edit_hint 0x141c3db80',
   'hint_deselect_tool.rs': '0x1422853c0',
   'hint_disable_snapping.rs': '0x14227e140',
   'hint_display_system.rs': '16 业务(最大单文件): hint_animation_system 0x141c4f400 / hint_deletion_cooldown_system 0x141c4f790 / hint_audio 0x141c4fb00 / hint_deletion_system 0x141c4fea0 / hint_highlight_shape 0x141c50120 / hint_position_update 0x141c50510 / hint_draw 0x141c51a60 / render_hints 0x141c52700 / progress_bar_update_system 0x141c530f0 / register_required_componentsx5 / BindingHintDisplay::resolve_steam_impl 0x14227e5e0 / cleanup_impl 0x14227f4a0',
   'hint_focused_edit.rs': 'hint_focused_edit_condition 0x14227da70 / hint_focused_edit_hint 0x14227dd60',
   'hint_orchestrator.rs': '16 业务: hint_orchestrator 0x14227fc30 / register_required_componentsx7 / BindingHintDisplay::alternatex5(0x141c4e4f0/0x141c4e6d0/0x141c4e8b0/0x141c4ead0/0x141c4ecb0) / adjacentx2(0x141c4ee80/0x141c4f060) / together 0x141c4f230 / filter_to_mode_impl 0x1422808d0',
   'hint_roof_shape.rs': 'hint_roof_shape 0x142284ed0 / make_lbm_drag_binding_hint 0x142285270',
   'hint_stairs.rs': '0x142285a20',
   'hint_terrain.rs': '0x142283cc0',
   'hint_terrain_focus.rs': '0x142283980',
   'hint_terrain_steepness.rs': '0x142283640',
   'hint_undo.rs': '0x142284220',
   'hint_unlink_windows.rs': '0x14227d760',
   'hints_startup.rs': '0x14227b450',
   'lib.rs': 'impl$0::add_onboarding 0x140650d50 / prompt_rollercoaster 0x14227bb70',
   'reset_hints.rs': '0x141c1c8d0',
   'tutorial_completion.rs': '0x14227c230',
 }
}

HEADER = [
 '> 状态：**awaiting restorer-B delivery**（2026-10-16 检查 `tiny-glade/crates/systems/<crate>/` 尚无交付）。',
 '> 本表业务函数清单与证据锚点来自 `SOURCE_MAP-systems.json`（skeleton PDB 元数据）+ `evidence/multi-baseline/*` 反汇编级分析 + country-core-deep 章节 + reviewer tg_vB1 独立 RNE 抽验。',
 '> 交付后 reviewer 逐项核对：签名/参数面 vs 机器码；接口契约 vs core 类型；偷懒（空壳/假数据/直接复制 TODO 骨架）；缺失（本表函数未实现）。',
 '> 核对列：P=通过 / F=失败 / Q=存疑待深挖。',
]

for crate in ['critters', 'clutter', 'onboarding']:
    rows = []
    for k in sorted(inv[crate]):
        fns = inv[crate][k]
        biz = [fn for fn in fns if fn[1] == 'business']
        if not biz:
            continue
        rel = k[len(crate) + 1:]
        if rel.startswith('src/'):
            rel = rel[4:]
        note = notes[crate].get(rel, '')
        lines = []
        for b in biz:
            name, _, va = b
            anchor = note if b is biz[0] else ''
            lines.append(f'| `{name}` | {va} | | {anchor} |')
        rows.append((rel, lines))
    md = [f'# {crate} 审查核对表（reviewer-B tg_vB1 预置基线）', '']
    md.extend(HEADER)
    md.append('')
    md.append('| 函数 | VA | 核对 | 证据锚点 / 已知风险 |')
    for rel, lines in rows:
        md.append(f'| **{rel}** | | | |')
        md.extend(lines)
    md.append('')
    out = os.path.join(BASE, 'groupB', crate, 'REVIEW.md')
    with open(out, 'w', encoding='utf-8') as f:
        f.write('\n'.join(md))
    print('written', out)
