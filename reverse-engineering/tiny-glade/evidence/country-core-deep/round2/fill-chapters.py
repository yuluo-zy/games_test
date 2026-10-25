# -*- coding: utf-8 -*-
"""Round-3 chapter filler for history domain (agent tg_a1).
Fills evidence/country-core-deep/chapters/{systems,resources}/history/*.md
with file roles + per-function inferences from DIGEST/CLOSURE/decomp evidence.
Only writes chapters/ files (evidence), never reconstruction sources."""
import json, os, re, sys

ROOT = r"D:/game/reverse-engineering/tiny-glade"
CH = os.path.join(ROOT, "evidence", "country-core-deep", "chapters")
SM = json.load(open(os.path.join(ROOT, "evidence", "country-core-deep", "SOURCE_MAP.json"), encoding="utf-8"))
DECOMP_DIR = os.path.join(ROOT, "evidence", "country-core-deep", "history", "decomp")
DECOMP2_DIR = os.path.join(ROOT, "evidence", "country-core-deep", "round2", "hist-closure")

decomp_ok = set()
for d in (DECOMP_DIR, DECOMP2_DIR):
    if os.path.isdir(d):
        for f in os.listdir(d):
            m = re.match(r"decomp-(0x[0-9a-f]+)\.txt", f)
            if m:
                decomp_ok.add(m.group(1).lower())

def ev(va):
    """evidence link for a va"""
    if va and va.lower() in decomp_ok:
        return f"history\\decomp\\decomp-{va}.txt"
    if va:
        return f"（无反编译）v2:{va}"
    return "内联（无独立符号）"

# ---- per-file role (章首“文件作用”) ----
ROLES = {
 "systems/history/finish_replay.rs": "回放自然完成/非活跃回放的收尾系统：检测 ReplayBuffer 与待处理命令双空后把 RestoreAppTo 记录的 AppMode 写回、解锁游戏指挥官（MetaCommander 哨兵弹栈）、恢复工具栏模式，并向绵羊系统发 CmdUnfreezeSheep。日志“History replay has naturally finished…”/“History replay was not active…”。",
 "systems/history/history_process.rs": "历史命令中枢系统：消费 HistoryCmd 事件流（undo/redo/interrupt），移动 History 游标后对目标 MultiEdit 调用 replay_edit 展开 ReplayCmd 桶（undo 时反转），并把桶 push 进 ReplayBuffer；同时消费第二事件流（16B 记录，byte[+9] 跳表 0x142ae5420）做命令校验/动画即时化/移除，录制中仅记日志。generate_replay_cmds_from_edit 为按 HistoryEdit.tag 分派（跳表 0x142c5f248）的共享核心。",
 "systems/history/history_recording.rs": "输入录制启停系统族：history_start_recording / history_finish_recording 驱动 History::start_input_recording / finish_input_recording（0x1409dd790 为 finish，多事件游标 + AppMode 检查）。",
 "systems/history/mod.rs": "history 系统族注册面：add_history_systems / insert_history_resources / add_history_events 三个 Bevy App 扩展胶水（注册 ~10 个回放/录制系统、插入 History/ReplayBuffer 等资源、添加 HistoryCmd 等事件）。AppMode 回放判别 0xd 的边界定义也在此。",
 "systems/history/replay_auto_clutter.rs": "自动杂物回放系统：消费前桶判别 9 命令，undo==0 插墙集（HashMap<WallId,()>），!=0 从 DeletedAutoClutter erase (DecoratorId,DecoratorId)；instant/动画节奏。",
 "systems/history/replay_garden.rs": "花园光栅回放：replay_raster<GardenRasterStagingSsbo, WorldRaster<512>, GardenFilter> 的薄包装 + 命令生成闭包（generate_replay_cmds_from_edit 宿主 0x14143d2c0）。",
 "systems/history/replay_mode.rs": "回放节奏系统：检测 AppMode 进出 0xd(Replay)（log started/finished）、RestoreAppTo 记录回放前模式、长按快进（HoldToFastReplay）时发 redo HistoryCmd、驱动 ReplayModeElapsedTime 计时。",
 "systems/history/replay_path.rs": "路径/鹅卵石光栅回放：replay_raster 的 4 个领域实例（DirtPath/Cobblestone1-3）+ generate_path_replay_cmds（0x1416286d0，niche 判别 0..3 过滤后尾调共享核心 0x1414433xx）。",
 "systems/history/replay_pop_bucket.rs": "空桶弹出系统：前桶 cmds 空时 ReplayBuffer::pop_front，应用桶的装饰 diff（restore_decorators，FUN_14155ba30）与楼梯 diff（restore_backup/apply_diff 链 1408ff210→140900ba0→14157cec0→140b50cc0→140b50d60→1408fefe0），Arc drop 收尾并 flush colliders/archivist。",
 "systems/history/replay_raster.rs": "光栅回放泛型系统 replay_raster<T,R,F>（Path/C1/C2/C3/Garden/Water 六实例）：过滤前桶命令，undo 清缓冲+重绘存储 stroke，redo 画 stroke 并发绘制事件；panic “!cmd.edit.stroke.points.is_empty()”。附 RasterReplayCmd（forward/backward、stroke 切段 get_stroke_from_0_to_t、时长 duration_based_on_stroke_length）与 RasterReplayResources SystemParam。",
 "systems/history/replay_stairs.rs": "楼梯回放系统：前桶判别 10 命令，undo==0 插边集（StairsRemovedSupports），!=0 从边集 remove_entry；查附挂装饰（DecoratorStorage::lookup_decorator_info + CachedDecoratorTransforms）并发 PlayStairsAudioCmd。",
 "systems/history/replay_terrain.rs": "地形回放系统：前桶判别 3 命令；TerrainReplayCmd::forward/backward 构造 duration=clamp(Σ点距*0.01,0.4,3.0)（无点 0.3，stroke_length 有限断言）；undo(+0x1c)==0→reapply，!=0→revert；动画 ease_out。",
 "systems/history/replay_wall.rs": "墙回放系统：前桶判别 4 命令，每 WallReplayEditCmd（0x18B）经 process（0x140abfb30，自递归，8 参）分派应用（instant/动画两路径）；done 标志 @+0x15；全部完成时经 niche vtable 0x142ab4910 收集已改墙。附 _deprecated_morph_curve（已弃用形态曲线应用）。",
 "systems/history/replay_water.rs": "水面光栅回放：replay_raster<WaterRasterStagingSsbo, WorldRaster<512>, WaterFilter> 薄包装 + 命令生成闭包（宿主 0x141444c70）。",
 "systems/history/signifier_replay_active.rs": "回放激活指示器系统：ReplayModeElapsedTime（enum2$ 判别）活跃时累加，对每个带 SignifierReplayIndex 组件的实体把 Transform 朝光标目标做弹性抖动（ease_out_elastic_jiggle=0.5*exp2(-10t)*sin(7.853982t)）；register_required_components 注册必需组件。",
 "resources/history/auto_clutter_edit.rs": "自动杂物编辑负载：AutoClutterEdit（墙派生装饰记录；host wall_id + decorator_ids）的手写 serde visitor 族（serialize 0x14003d300 / deserialize 0x140a18500）。",
 "resources/history/color_edit.rs": "颜色编辑负载：ColorEdit（目标 Wall/Roof/Halftimber/Decorator/StairRailing/FlatRoof/StairLadder/Brightness + from/to + 目标 id；字段 chimney_h_offset/allow_connection）手写 serde visitor 族（serialize 0x14003c5d0 / deserialize 0x1417befe0，visit_str 0x1417bf8a0）。",
 "resources/history/decorator_edit.rs": "装饰历史编辑域：DecoratorHistoryEdit/Diff/State、DecoratorHistoryState::From（0x140b3f930，preview 类型 panic）、is_default_* 谓词（lantern_brightness==stair_width==1.0f，0x140b3f910 ICF）、convert_before_after_states_to_changes（0x140b3fab0，按 style 字节分变体比较浮点输出变更集）、send_state_diff_events（0x1417bdae0，replay_pop_bucket 消费）、cull_allow_list；手写 serde visitor 族。",
 "resources/history/mod.rs": "权威历史缓冲核心（S05）：History 0x78B 布局（edit_chain/snapshots/pending/计数/nonce/cursor/history_start）、undo/redo/录制/快照方法族、MultiEdit/HistoryEdit/DecoratorDiffTable/StairsDiffTable、HistoryChainIndex/HistoryEditChain、HistorySnapshotId/MaybePendingSnapshotId/HistoryChainSnapshots、EditType 与 MultiEdit 的 serde 巨函数（deserialize 合并区 0x140b3ad60-0x140b3bc83）。",
 "resources/history/path_edit.rs": "路径/鹅卵石光栅编辑负载：RasterEdit（points/brush_size/is_additive/blend_mode；ReplayCmd 判别 0/1/2 的 Arc 负载）+ RasterEditSerialized 手写 serde（serialize 0x1404896f0/0x1416e8340、deserialize 0x1416e8510、From 0x140a976b0）。",
 "resources/history/replay_cmd_buffer.rs": "回放命令缓冲：ReplayBuffer（桶 Vec cap-first 元素 0x40 + 活跃计数）、ReplayBucket 0x40B（decorators/stairs Arc + cmds Vec 0x30B）、ReplayCmd 0x30B（判别@0/instant@8/duration@c/负载@10/elapsed@18/undo@1c）、pop_front/front_mut_bucket/make_animations_instant/len。",
 "resources/history/roof_edit.rs": "屋顶编辑负载：RoofEdit 操作枚举（FlatRoofRimStyleChange/SwitchRoofRidgeDir 等 tag）手写 serde visitor 族（serialize 0x140259e30、deserialize 0x140b38b80、visit_str 0x140b392d0/0x140b39400/0x140b394f0/0x140b395d0）。重建侧缺此模块。",
 "resources/history/stairs_history_edit.rs": "楼梯历史编辑负载：StairsHistoryEdit/StairsHistoryDiff（边对 before/after + hash；运行时 StairsDiffTable 96B = 两 Vec<u32> 边对 cap-first）手写 serde（serialize 0x140259af0、deserialize 0x140a18ee0）。",
 "resources/history/terrain_edit.rs": "地形编辑负载：TerrainEdit（判别 0/1/2：New/Edit/Move stroke；duration=clamp(Σ点距*0.01,0.4,3.0) 内联、revert 0.99 进度阈值+stroke 表清 active、reapply、update_terrain_curve 0x140859740 hash 查表+点集拷贝+fold+全局时间 lerp）手写 serde（serialize 0x1404f41b0/0x1417c1590、deserialize 0x1417c1ac0、visit_str 0x1417c23e0）。",
 "resources/history/tree_edit.rs": "树编辑负载：TreeEdit（Place/Move + tree{id,position,scale,rotation_degrees,prefab_name,is_preplaced}）、missing_prefab 0x1409e6210、TreeCanopyMaxCullDistance 手写 serde visitor 族（serialize 0x14003d4e0/0x1417c2770/0x1417c2600/0x1417c2a50、deserialize 0x1417c2f60）。",
 "resources/history/wall_compound_edit.rs": "墙复合历史编辑域：WallHistoryEdit 操作枚举（ChangeFreehandHeightType/PlasterPeelAmount/ChangePlasterPeelAmount/MorphPremadeShape/MorphFreehandWall…，niche 判别首字段）、WallHistoryCompoundEdit（0x108B 负载）、ConcurrentEdits、CompoundEditBuilder（new_edit_sequence/next/build 0x140a82d50）、panic_deprecated_operation 0x140a82e20、update_compatibility（内联）、gather_wall_ids；手写 serde visitor 族（deserialize 0x141401960 等 ~40 变体）。",
 "resources/history/wall_history.rs": "墙历史事件接口：History 的 WallHistory trait 实现（ev_new_wall 0x140a83300：断言 curve 长度非零、构建 0x108B WallHistoryEdit + FUN_141696f50 校验 → History::add(tag 7)；ev_height_changed 0x140a83f80：EditType::new_height_changed（0x140a83fb0，niche+2 判别）→ History::add）。",
}

# ---- per-function override: name-pattern -> (作用, 结论, 证据extra) ----
# key: unique substring match on the short name (after last '::')
OVERRIDES = [
 # systems/history/history_process
 ("generate_replay_cmds_from_edit", "按 HistoryEdit.tag 经跳表 0x142c5f248 分派到各领域命令生成闭包的共享核心（4 单态仅闭包环境不同，316B 跳板）", "O", "DIGEST §3.5；decomp-0x14143fb50/0x14143d2c0/0x1414423e0/0x141444c70.txt"),
 ("HistoryCmd::interrupt", "构造中断命令字 0x02000200（mov eax,0x00020002;ret）", "O", "原字节 0x140af5b20"),
 ("PendingHistoryCommands::is_empty", "待处理回放命令队列判空（ReplayBuffer 消费侧门控）", "I", "内联"),
 ("PendingHistoryCommands::new", "空队列构造", "I", "内联"),
 ("history_process", "见文件作用；undo/redo 游标+replay_edit+桶 push+第二事件流+log", "O", "decomp-0x140af5b30.txt；DIGEST §3.3"),
 ("replay_edit", "对目标 MultiEdit 展开 ReplayCmd 桶（undo 反转、Arc clone 两表、去重校验）并 push ReplayBuffer", "O", "decomp-0x140af6170.txt；DIGEST §3.4"),
 ("replay_edit::closure", "replay_edit 内联闭包（逻辑并入 replay_edit）", "I", "宿主 0x140af6170"),
 ("raster_edit_to_replay_cmd", "RasterEdit → ReplayCmd 转换辅助", "I", "内联"),
 ("schedule_replay_commands", "把生成的 ReplayCmd 排程进 ReplayBuffer（含 3 层闭包）", "I", "内联；逻辑并入 history_process 桶 push 段"),
 ("history_clear_previous_state", "清理回放前状态（语义 U）", "U", "内联"),
 # history_recording
 ("history_start_recording", "启动输入录制（委托 History::start_input_recording；无独立符号=内联）", "I", "CLOSURE §1"),
 ("history_finish_recording", "结束输入录制（多事件游标+AppMode 检查 → History::finish_input_recording）", "O", "decomp-0x1409dd790.txt"),
 # systems/history/mod
 ("add_history_systems", "Bevy App 扩展：注册 history 系统集合（~10 系统）", "O", "decomp-0x1409438a0.txt"),
 ("insert_history_resources", "插入 History/ReplayBuffer 等资源", "O", "decomp-0x140943e10.txt"),
 ("add_history_events", "添加 HistoryCmd 等事件类型", "O", "decomp-0x140943f40.txt"),
 # replay_auto_clutter
 ("replay_auto_clutter::replay_auto_clutter", "见文件作用", "O", "decomp-0x140aa4a60.txt"),
 # replay_garden / water / path
 ("replay_garden::replay_garden", "replay_raster<Garden> 薄包装", "I", "单态 0x1409f6ff0"),
 ("replay_garden::closure", "命令过滤/生成闭包", "I", "宿主见行号"),
 ("replay_water::replay_water", "replay_raster<Water> 薄包装", "I", "单态 0x1409f4650"),
 ("replay_water::closure", "命令过滤/生成闭包", "I", "宿主见行号"),
 ("generate_path_replay_cmds", "Path/Cobblestone 编辑 → ReplayCmd：niche 判别 0..3 过滤后尾调 0x1414433xx 共享核心", "O", "原字节 0x1416286d0；DIGEST §3.10"),
 ("replay_path::replay_path", "replay_raster<Path> 领域包装", "I", "单态 0x1409febf0"),
 # replay_pop_bucket
 ("replay_pop_bucket::replay_pop_bucket", "见文件作用", "O", "decomp-0x140b38250.txt"),
 ("replay_pop_bucket::closure", "restore_decorators 闭包", "I", "宿主 0x140b38250"),
 # replay_raster
 ("RasterReplayCmd::forward", "构造前向命令：{ptr,+0x8=0,duration f32@+0xc,+0x10=2}", "O", "decomp-0x141700430.txt"),
 ("RasterReplayCmd::backward", "构造反向命令（同构）", "O", "decomp-0x141700620.txt"),
 ("RasterReplayCmd::forward_instant", "instant 前向命令", "I", "内联；74 宿主点"),
 ("RasterReplayCmd::backward_instant", "instant 反向命令", "I", "内联"),
 ("RasterReplayCmd::new", "命令构造（duration 默认）", "I", "内联"),
 ("get_param", "RasterReplayResources SystemParam 的 get_param", "O", "decomp-0x140ae5830 待补"),
 ("init_state", "SystemParam init_state", "I", "Bevy 胶水"),
 ("validate_param", "SystemParam validate_param", "I", "Bevy 胶水"),
 ("impl$8::default", "RasterReplayResources 默认", "I", "内联"),
 ("replay_raster<", "见文件作用（六单态）", "O", "decomp-0x1409febf0/0x1409fc290/0x1409f1cf0/0x1409f9930/0x1409f6ff0/0x1409f4650.txt"),
 ("replay_raster::closure", "命令过滤/重绘闭包", "I", "宿主见行号"),
 ("get_stroke_from_0_to_t", "按进度 t 切取 stroke 点段", "I", "内联；与 0x140859740 的 fold 同族"),
 ("duration_based_on_stroke_length", "按 stroke 长度算动画时长（与 TerrainEdit::duration 同公式族）", "I", "内联"),
 ("impl$6::eq", "RasterEdit 判等（derive）", "派生", "derive"),
 ("impl$0::from", "From 转换（RasterEdit 构造）", "I", "内联"),
 # replay_stairs
 ("replay_stairs::replay_stairs", "见文件作用", "O", "decomp-0x140af75b0.txt"),
 ("replay_stairs::closure", "边集应用闭包", "I", "宿主 0x140af75b0"),
 # replay_terrain
 ("replay_terrain::replay_terrain", "见文件作用", "O", "decomp-0x140995150.txt"),
 ("TerrainReplayCmd::forward", "前向命令：duration=clamp(Σ点距*0.01,0.4,3.0)（空 0.3）+ stroke_length 有限断言", "O", "decomp-0x140994f30.txt；DIGEST §2.4"),
 ("TerrainReplayCmd::backward", "反向命令（同构）", "O", "decomp-0x140995040.txt"),
 ("TerrainReplayCmd::new", "命令构造", "I", "内联"),
 ("ease_out", "缓出函数（系数歧义 U）", "U", "内联；reconstruction 用 1-(1-t)^3"),
 # replay_wall
 ("WallReplayEditCmd::process<", "见文件作用（自递归 8 参分派）", "O", "decomp-0x140abfb30.txt"),
 ("replay_wall::replay_wall::closure$0", "墙命令应用闭包（replay_wall 内联宿主）", "I", "decomp-0x140abfaa0.txt"),
 ("replay_wall::replay_wall", "见文件作用", "O", "decomp-0x140951b80.txt"),
 ("_deprecated_morph_curve", "已弃用的形态曲线应用", "O", "decomp-0x140952570.txt"),
 ("is_single_adjust_height_cmd", "单 AdjustHeight 命令判定（元素数==1）", "I", "内联"),
 ("WallReplayCmd::new", "构造墙回放命令（自 ConcurrentEdits）", "I", "内联；源行 158"),
 ("impl$1::process_instant::closure", "process_instant 内联闭包", "I", "内联；源行 353/502"),
 ("process_instant", "instant 路径应用（cubic_in_out 后按判别分派）", "I", "内联；源行 307"),
 ("process_animated", "动画路径应用", "I", "内联；源行 618"),
 # signifier
 ("register_required_components", "注册 SignifierReplayIndex 的必需组件", "O", "decomp-0x140b1d680.txt"),
 ("signifier_replay_active", "见文件作用", "O", "decomp-0x140b1b860.txt"),
 ("ease_out_elastic_jiggle", "弹性抖动 0.5*exp2(-10t)*sin(7.853982t)", "O", "内联；DIGEST §3.7"),
 # replay_mode
 ("replay_mode::replay_mode", "见文件作用", "O", "decomp-0x140a7c550.txt"),
 ("impl$2::default", "RestoreAppTo 默认", "I", "内联"),
 ("RestoreAppTo::set", "记录回放前 AppMode", "I", "内联；源行 24"),
 ("RestoreAppTo::get", "取回放前 AppMode", "I", "内联；源行 50"),
 # finish_replay
 ("finish_replay::finish_replay", "见文件作用", "O", "decomp-0x1408b36a0.txt"),
 ("finish_replay::closure", "收尾闭包", "I", "宿主 0x1408b36a0"),
 # ===== resources
 # mod.rs
 ("HistoryIdentity::generate", "fastrand u64 全范围随机身份", "O", "decomp-0x140a84220.txt"),
 ("HistoryIdentity::mutate", "重随机身份", "I", "内联"),
 ("MaybePendingSnapshotId::get", "取待定快照 id", "U", "内联"),
 ("MaybePendingSnapshotId::assign", "分配快照 id", "U", "内联"),
 ("MaybePendingSnapshotId::assign_invalid", "标记无效 id", "U", "内联"),
 ("HistoryChainSnapshot::snapshot_id", "从快照负载解析 id", "U", "内联"),
 ("history_chain_index", "取快照的 chain_index 字段", "I", "内联（+0x08 i32）"),
 ("impl$8::from", "快照负载 From 转换", "U", "内联"),
 ("impl$9::try_from", "ResolvedHistoryChainSnapshot TryFrom（真实过滤谓词，规则 U）", "O", "符号 0x140a84260"),
 ("HistorySnapshotId::from_snapshot_blob", "blob → id 解析", "U", "内联"),
 ("HistorySnapshotId::file_name", "id → 文件名", "U", "内联"),
 ("HistorySnapshotId::from_file_stem", "文件 stem → id", "U", "内联"),
 ("i_promise_there_were_no_mutations", "消费 mutation token（置 inactive）", "O", "内联进 finish_input_recording"),
 ("impl$12::drop", "token Drop 守卫（active 未消费 panic；0x14362c420 panic-in-progress 门控）", "O", "DIGEST §6"),
 ("HistoryChainIndex::new", "恒等 u32 封装", "O", "符号 0x140833250"),
 ("HistoryChainIndex::prev", "-1", "I", "内联"),
 ("HistoryChainIndex::next", "+1", "I", "内联"),
 ("impl$2::fmt", "Display fmt（写 u32 十进制）", "O", "decomp-0x140b31c90.txt"),
 ("impl$3::index", "Index usize", "I", "内联"),
 ("HistoryEditChain::new_from_inner", "Vec 封装构造", "I", "内联"),
 ("HistoryEditChain::get", "下标访问", "I", "内联"),
 ("HistoryEditChain::is_empty", "判空", "I", "内联"),
 ("HistoryEditChain::truncate", "截断", "I", "内联"),
 ("HistoryEditChain::push", "push 元素 0x28", "I", "内联；grow_one 0x28"),
 ("HistoryEditChain::iter", "切片迭代", "O", "符号 0x140b31cf0（iter_mut 同址族）"),
 ("HistoryEditChain::iter_mut", "可变切片迭代", "O", "decomp-0x140b31cf0.txt"),
 ("HistoryEditChain::insert_at_start", "首部插入（memmove 右移）", "I", "内联进 insert_starting_edit"),
 ("is_decorator_history_diff_empty", "diff+0x18==0", "O", "decomp-0x140a84290.txt"),
 ("is_stairs_history_diff_empty", "两 Vec cap（+0x30/+0x48）==0", "O", "decomp-0x140a842a0.txt"),
 ("MultiEdit::iter", "edits 迭代", "I", "内联"),
 ("MultiEdit::is_empty", "四条件空判定", "O", "内联进 finish_input_recording"),
 ("EditType::Path::constructor", "Path 变体构造", "O", "符号族；serde visitor"),
 ("EditType::Cobblestone1::constructor", "Cobblestone1 构造", "O", "符号族"),
 ("EditType::Cobblestone2::constructor", "Cobblestone2 构造", "O", "符号族"),
 ("EditType::Cobblestone3::constructor", "Cobblestone3 构造", "O", "符号族"),
 ("EditType::Water::constructor", "Water 构造", "O", "符号 0x1416bb7f0"),
 ("EditType::Garden::constructor", "Garden 构造", "O", "符号 0x1416960d0"),
 ("impl$14::fmt", "EditType Debug::fmt（跳表分派）", "O", "decomp-0x140a842c0.txt"),
 ("History::new_from_params", "（reconstruction 别名；真名 insert_starting_edit）", "O", "见下"),
 ("History::insert_starting_edit", "断言 cursor>-2；cursor+1；首部插入起始编辑；写 nonce；count=0", "O", "decomp-0x140a844f0.txt"),
 ("History::edit_chain_mut", "edit_chain 可变访问器", "I", "内联"),
 ("History::current_snapshot", "对 snapshots 二分 chain_index<=cursor；仅当 ==cursor 才 Arc clone 返回", "O", "decomp-0x140a84650.txt"),
 ("History::set_current_snapshot", "断言 current_snapshot().is_none() 后有序插入（不替换）", "O", "decomp-0x140a84760.txt"),
 ("History::iter_resolved_snapshots", "snapshots 切片 filter_map 迭代", "O", "decomp-0x140a84a40 区；符号 0x140a84a40"),
 ("History::closest_snapshot", "最近快照二分辅助（非公开方法）", "I", "内联；CLOSURE §2"),
 ("History::closest_resolved_snapshot", "filter_map 构建 Resolved Vec 后二分 chain_index<=cursor 最近者", "O", "decomp-0x140a84a50.txt"),
 ("History::remove_snapshot_record", "移除快照记录（无独立符号=内联；sysres 名址错位）", "U", "CLOSURE §1"),
 ("History::undo", "游标守卫三重检查后 cursor-1", "O", "内联进 history_process；DIGEST §3.3"),
 ("History::redo", "cursor>=-1 断言；target 守卫；cursor=target", "O", "内联进 history_process"),
 ("History::debug_display", "格式串 History: {} / {}. Start at {}", "O", "decomp-0x140a84ba0.txt"),
 ("History::debug_set_history_start", "mov [rcx+0x6c],edx", "O", "原字节 0x140a84c40"),
 ("History::nuke_future_states", "cursor!=len-1 时整组重置或 truncate+retain；写 nonce", "O", "decomp-0x140a84c50.txt"),
 ("History::add_decorator_diff", "4 参：未录制 panic；nuke 判定+重建桶；楼梯 checksum 校验；两 u32 边对循环 push tag 0xE/1；Arc 替换两表", "O", "decomp-0x140a854a0.txt；DIGEST §3.2"),
 ("History::add", "pending 未录制 panic；空桶+cursor-len<=-2 先 nuke；tag7 墙族校验；push 24B 记录；第 3 参=track-caller 文件", "O", "decomp-0x140a84ec0.txt"),
 ("History::start_input_recording", "AppMode::Replay(0xd) 断言；drop 旧桶；新桶（64B 表+96B 表+全局序列）", "O", "decomp-0x140a86270.txt"),
 ("History::try_start_input_recording", "非 Replay 且未录制才启动，返回 bool", "O", "decomp-0x140a86510.txt"),
 ("History::finish_input_recording", "取出 pending；空则 drop；push 链、cursor+1、断言、nonce、snapshots 追加 48B 记录", "O", "decomp-0x140a86730.txt"),
 ("History::is_recording", "pending+0x30>=0（哨兵判别）", "O", "原字节 0x140a86970"),
 ("History::new", "History 构造（i64::MIN 哨兵 + cursor=-1 + tail=0 初始化）", "O", "CLOSURE §1（0x140a84e60 机器码）"),
 ("History::pending_undo_count", "待撤销计数", "U", "内联"),
 ("History::finish_potential_mutation", "was_active 为真则清 snapshots（Arc drop）+nonce；panic-in-progress 门控 0x14362c420", "O", "decomp-0x140a86980.txt"),
 ("closest_history_snapshot", "（sysres 名址双错；真实逻辑=current_snapshot/closest_resolved + save::snapshot 域）", "纠错", "CLOSURE §2"),
 ("impl$16::deserialize", "MultiEdit::deserialize（serde 巨函数；与 visit_seq/visit_map 合并区）", "O", "decomp-0x140b36460/0x140b365e0/0x140b36840.txt"),
 ("impl$2::deserialize", "HistoryChainSnapshots::deserialize", "O", "符号 0x140b363e0"),
 # wall_history
 ("ev_new_wall", "断言 curve 长度非零；构建 0x108B WallHistoryEdit+FUN_141696f50 校验 → History::add(tag 7)", "O", "decomp-0x140a83300.txt"),
 ("ev_height_changed", "new_height_changed(buf,wall,h0,h1) → History::add（track-caller）", "O", "原字节 0x140a83f80"),
 ("EditType::new_height_changed", "写 24B 记录 {1,1,niche+2@+0x10,wall,h0,h1}+0x108B 拷贝+FUN_141696f50", "O", "decomp-0x140a83fb0.txt"),
 # wall_compound_edit
 ("WallHistoryCompoundEdit::new", "0x108B 负载构造", "I", "内联"),
 ("CompoundEditBuilder::new_edit_sequence", "开始新编辑序列", "O", "符号 0x140a82b20"),
 ("CompoundEditBuilder::next_empty", "当前组判空", "I", "内联"),
 ("CompoundEditBuilder::with_concurrent", "加入并发组", "I", "内联"),
 ("CompoundEditBuilder::next", "推进到下一组", "O", "符号 0x140a82b40"),
 ("CompoundEditBuilder::build", "构建 Option<WallHistoryCompoundEdit>", "O", "符号 0x140a82d50"),
 ("panic_deprecated_operation", "已弃用操作 panic", "O", "符号 0x140a82e20"),
 ("WallHistoryEdit::update_compatibility", "兼容性更新（无独立符号=内联）", "U", "CLOSURE §1"),
 ("WallHistoryEdit::gather_wall_ids", "收集涉及的墙 id", "U", "内联"),
 ("impl$4::fmt", "WallHistoryEdit Debug::fmt（niche 跳表 0x142ad888c）", "O", "decomp-0x140a82eb0.txt"),
 ("impl$5::fmt", "WallHistoryCompoundEdit Display（写串 \"WallHistoryCompoundEdit\"+参数）", "O", "decomp-0x140a87970.txt"),
 # decorator_edit
 ("is_default_chimney_smoke", "ICF 多符号同址 0x1408c5680（含 is_default_allow_connection 等）", "O(ICF)", "符号 0x1408c5680"),
 ("is_default_lantern_brightness", "f32==1.0f（与 is_default_stair_width ICF 同址）", "O", "原字节 0x140b3f910"),
 ("is_default_stair_width", "f32==1.0f", "O", "符号 0x140b3f910"),
 ("is_default", "通用默认值判定", "U", "内联"),
 ("impl$1::from", "DecoratorHistoryState::From（type@+0x41==8 preview panic）", "O", "decomp-0x140b3f930.txt"),
 ("send_state_diff_events", "diff → 状态变更事件（replay_pop_bucket 的 restore_decorators 闭包宿主）", "O", "符号 0x1417bdae0"),
 ("convert_before_after_states_to_changes", "before/after → 变更集（按 style 字节分变体比较浮点，输出 changed 布尔）", "O", "decomp-0x140b3fab0.txt"),
 ("DecoratorHistoryDiff::is_empty", "diff 判空", "I", "内联"),
 ("cull_allow_list", "装饰 cull 允许列表（to_arc_slice）", "I", "内联"),
 ("impl$25::fmt", "Display/Debug：写串 \"DecoratorStorageAddr…\"（装饰存储地址键）", "O", "decomp-0x140b30a10.txt"),
 # terrain_edit
 ("impl$0::fmt", "TerrainEdit Debug::fmt（niche 判别 0/1/2）", "O", "decomp-0x140858c80.txt"),
 ("TerrainEdit::duration", "clamp(Σ点距*0.01,0.4,3.0)，空 0.3（内联进 TerrainReplayCmd::forward）", "O", "decomp-0x140994f30.txt；CLOSURE §1"),
 ("TerrainEdit::revert", "0.99 进度阈值 panic + hashbrown 48B 表清 active 标志", "O", "decomp-0x140858d30.txt"),
 ("TerrainEdit::reapply", "对称重放应用", "O", "decomp-0x1408592d0.txt"),
 ("TerrainEdit::update_terrain_curve", "hash 查 stroke 表+点集拷贝+fold+全局时间 lerp 进度回写", "O", "round2 反编译 decomp-0x140859740.txt（原名 reapply::closure$0 错）"),
 # path_edit
 ("impl$0::from", "RasterEdit 构造 From", "I", "内联"),
 ("impl$1::from", "RasterEditSerialized From", "O", "符号 0x140a976b0"),
 # tree_edit
 ("missing_prefab", "prefab 缺失回退：返回 24B {0x142ac4708,0x142ac4710,\"FAB_NAME\"}（串 MISSING_TREE_PREFAB_NAME/TREE_PREFAB_NAME 区；树 prefab 回退名候选）", "O", "decomp-0x1409e6210.txt"),
 ("TreeCanopyMaxCullDistance::get_dist", "取树冠最大剔除距离", "I", "内联"),
 # replay_cmd_buffer
 ("ReplayCmd::make_animation_instant", "单命令即时完成", "I", "内联；make_animations_instant 0x1409f1ab0 内联"),
 ("ReplayBuffer::new", "空缓冲构造", "I", "内联；insert_history_resources 内联"),
 ("ReplayBuffer::add", "尾部 push 桶 + counter++", "I", "内联；replay_edit 0x140af6170 内联"),
 ("ReplayBuffer::pop_front", "弹前桶 + counter++", "O", "符号 0x1409f18f0"),
 ("ReplayBuffer::len", "桶数", "O", "符号 0x140949170"),
 ("ReplayBuffer::is_empty", "判空", "I", "内联；finish_replay/history_process 内联"),
 ("ReplayBuffer::front_mut_bucket", "前桶可变引用", "O", "符号 0x1409f1a80"),
 ("ReplayBuffer::current_replay_cmd", "前桶首命令", "I", "内联；signifier_replay_active 内联"),
 ("ReplayBuffer::make_animations_instant", "全部命令即时完成", "O", "符号 0x1409f1ab0"),
 ("ReplayBuffer::modification_counter", "修改计数访问器", "I", "内联；save_game 读取"),
 # inline closures (name-based inference)
 ("impl$1::serialize::closure", "HistoryIdentity serde 数字序列化闭包", "I", "内联"),
 ("impl$1::serialize", "HistoryIdentity serde Serialize（u64 数字）", "I", "内联（闭包并入）"),
 ("current_snapshot::closure", "二分比较闭包（chain_index 键）", "I", "内联进 0x140a84650"),
 ("set_current_snapshot::closure", "有序插入位置闭包", "I", "内联进 0x140a84760"),
 ("iter_resolved_snapshots::closure", "filter_map 谓词（TryFrom 0x140a84260 ok 过滤）", "I", "内联进 0x140a84a40"),
 ("remove_snapshot_record::closure", "移除谓词闭包（随宿主内联，无独立符号）", "U", "CLOSURE §1"),
 ("nuke_future_states::closure", "retain 谓词（chain_index<=cursor）", "I", "内联进 0x140a84c50"),
 ("duration::closure", "点距累加 fold 闭包（相邻点 distance，Σ*0.01 clamp(0.4,3.0)）", "I", "内联进 0x140994f30"),
 ("revert::closure", "撤销应用命令构造闭包（EditTerrainStrokeCmd 族）", "I", "内联进 0x140858d30"),
 ("reapply::closure", "重放应用闭包（与 revert 对称）", "I", "内联进 0x1408592d0"),
 ("update_terrain_curve::closure", "进度 lerp 闭包（fold+全局时间，(1-t)*u+t）", "I", "宿主 0x140859740"),
 ("impl$0::new::closure$0::closure", "WallReplayCmd::new 从 ConcurrentEdits 展开的嵌套闭包", "I", "内联；源行 170/174"),
 ("wall_compound_edit::impl$0::new::closure", "WallHistoryCompoundEdit::new 内组处理闭包", "I", "内联"),
 ("impl$0::new::closure", "构造器内组处理闭包", "I", "内联"),
 # roof_edit / stairs / auto_clutter / color / tree serde families handled by generic rule
]

def match_override(name):
    short = name.split("::")[-1]
    for pat, role, concl, evid in OVERRIDES:
        if pat in name:
            return role, concl, evid
    # generic serde/derive rules
    if re.search(r"impl\$\d+::(fmt|clone|eq|default)$", name):
        return ("derive 胶水（Debug/Clone/Eq/Default）", "派生", "复用 derive，不计业务")
    if re.search(r"_::impl\$\d+::(serialize|deserialize)(::impl\$|$|$<)", name):
        return ("serde 手写/derive visitor 胶水（本类型 serde 的 monomorph/visitor 入口）", "O(入口)", "见文件作用所列 serde 地址")
    if "visit_str" in name or "visit_enum" in name or "visit_map" in name or "visit_seq" in name or "expecting" in name or "visit_newtype_struct" in name:
        return ("serde visitor 胶水（字段/枚举 tag 解析；字段名见 DIGEST §3.11/CLOSURE）", "O(胶水)", "serde 族；不单独恢复")
    if "::deserialize<" in name or "::serialize<" in name:
        return ("serde 主入口 monomorph（serde_json::Value / Serializer 实例）", "O(入口)", "见文件作用所列 serde 地址")
    if "::from" in name:
        return ("From 转换（负载构造/归一）", "I", "内联")
    return None

# build chapters
hist_files = sorted(k for k in SM if "/history" in k)
results = []
for path in hist_files:
    sm = SM[path]
    chap = os.path.join(CH, path.replace("/", os.sep) + ".md")
    if not os.path.exists(chap):
        results.append((path, "chapter missing", 0))
        continue
    funcs = sm["functions"]
    role = ROLES.get(path, "待推断")
    biz = [f for f in funcs if f.get("va")]
    inline = [f for f in funcs if not f.get("va")]
    lines = [f"# {path}", ""]
    lines.append(f"> 文件作用：{role}")
    lines.append("")
    lines.append(f"函数总计 {len(funcs)}：业务 {len(biz)} / 内联无独立符号 {len(inline)} / 派生胶水 0（已并入业务表）")
    lines.append("")
    lines.append("## 业务函数")
    lines.append("")
    lines.append("| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |")
    lines.append("|---|---|---|---|---|---|")
    uncovered = 0
    for f in funcs:
        if not f.get("va"):
            continue
        name = f["name"]
        va = f["va"][0]
        ln = f.get("line", "")
        m = match_override(name)
        if m:
            role_f, concl, evid = m
        else:
            role_f, concl, evid = "待推断", "U", "—"
            uncovered += 1
        # evidence: prefer decomp link if exists
        if evid.startswith("符号") or evid.startswith("内联") or evid.startswith("原字节") or evid.startswith("decomp") or evid.startswith("history\\") or evid.startswith("round2") or evid.startswith("（") or evid.startswith("serde") or evid.startswith("derive") or evid.startswith("DIGEST") or evid.startswith("CLOSURE") or evid.startswith("宿主") or evid.startswith("单态") or evid.startswith("v2"):
            pass  # keep
        if va and va.lower() in decomp_ok and not evid.startswith("history\\decomp") and not evid.startswith("DIGEST") and not evid.startswith("CLOSURE") and not evid.startswith("decomp"):
            evid = f"history\\decomp\\decomp-{va}.txt"
        lines.append(f"| `{name}` | {va} | {ln} | {role_f} | {concl} | {evid} |")
    lines.append("")
    if inline:
        lines.append("## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）")
        lines.append("")
        for f in inline:
            name = f["name"]
            m = match_override(name)
            if m:
                role_f, concl, evid = m
            else:
                role_f, concl, evid = "待推断", "U", "—"
                uncovered += 1
            lines.append(f"- `{name}`（行 {f.get('line','')}）— {role_f}（{concl}；{evid}）")
        lines.append("")
    lines.append(f"<!-- 未覆盖函数数：{uncovered} -->")
    with open(chap, "w", encoding="utf-8") as fh:
        fh.write("\n".join(lines) + "\n")
    results.append((path, f"biz={len(biz)} inline={len(inline)}", uncovered))

tot_unc = 0
for path, info, unc in results:
    print(f"{path}: {info} uncovered={unc}")
    tot_unc += unc
print("TOTAL uncovered:", tot_unc)
