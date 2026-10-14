# -*- coding: utf-8 -*-
"""Round3 chapter fill for session-save domain (tg_a3). Writes chapters/*.md in place."""
import re, os, sys

BASE = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters"
DG = r"session-save\DIGEST.md"
R2C = r"round2\session-save-CLOSURE.md"

data = {}

data["resources/session.rs.md"] = (
 "会话资源层：GladeType 主题判别（0=Summer/1=Olden/2=Flowery/3=Autumn/4=Winter，O）、"
 "SessionId 与保存目录协议（saves()/backups()/会话文件三件套 history.json+screenshot.jpg+meta.json）、"
 "SessionMeta 元数据（meta.json，serde 字段 {state,glade,mutators,image}）、NextSession 会话请求（0xa0B：on_load 命令 Vec + kind niche + uuid）、"
 "SavedSession(List) 会话列表/删除（deleted_saves 两段删除）。",
 {
 "0x1403b5a20": ("U（serde visit_str 派生胶水，非业务）", "—"),
 "0x141328d00": ("U（serde Value 反序列化胶水）", "—"),
 "0x1403b52d0": ("U（serde SliceRead 反序列化胶水）", "—"),
 "0x1403b5500": ("U（serde Serializer 胶水）", "—"),
 "0x141328bb0": ("U（serde Value Serializer 胶水）", "—"),
 "0x140a9fa70": ("O：tag==1 判定（cmp byte[rcx],1; sete）", DG),
 "0x14092e6a0": ("O：tag==2 判定", "ui-input\\decomp\\decomp-0x14092e6a0.txt"),
 "0x14092e6c0": ("O：tag==3 判定", DG),
 "0x140ae8320": ("O：tag==4 判定（与 BackupSaveFileKind::is_corrupted 同址折叠）", "session-save\\decomp\\decomp-0x140ae8320.txt"),
 "0x140916420": ("O：uuid v4 生成（call 0x140c102e0）", DG),
 "0x140b46000": ("O：paths::saves()(0x140b9db60)/uuid", R2C),
 "0x140b46180": ("O：paths::backups()(0x140b9db90)/uuid（独立根目录，非 saves 子目录）", "session-save\\decomp\\decomp-0x140b46180.txt, " + R2C),
 "0x140b46300": ("O：构建 {history.json, screenshot.jpg, meta.json} 三条完整路径", "session-save\\decomp\\decomp-0x140b46300.txt"),
 "0x140b46370": ("O：\"assets/glade\"+主题表[+0x58]（summer/olden/flowery/autumn/winter）", "round2\\decomp-0x140b46370.txt"),
 "0x140b463c0": ("O：save_folder_path+\"meta.json\"+save_session_meta_to_file(0x1417522e0)", "round2\\decomp-0x140b463c0.txt"),
 "0x140b46480": ("O：读文件+serde 反序列化；字段名 {state,glade,mutators,image}（首字节分发表）", "session-save\\decomp\\decomp-0x140b46480.txt"),
 "0x140b46860": ("O：any(b==0) 扫描", DG),
 "0x140b46890": ("O：any(b<=1) 扫描（纠错：wp13 合同写 >1 方向相反）", DG),
 "0x140b468c0": ("O：any(b==2) 扫描", "session-save\\decomp\\decomp-0x140b468c0.txt"),
 "0x140b46920": ("O：{cap-first 空 Vec(cap=8), kind=0x8000000000000002, uuid@+0x20, ignore_snapshot=false}", "session-save\\decomp\\decomp-0x140b46920.txt"),
 "0x140b46960": ("O：96B 模板(rdx)+32B(r8) 构造；kind@+0x18=模板首 qword；Vec 清空", R2C),
 "0x140b46a20": ("O：写 +0x98 bool + memcpy 0xa0（builder）", DG),
 "0x1403b5150": ("O：push 3 条 MetaCommand（块 0xa8B）；载荷 U", "—"),
 "0x141328ac0": ("O：push 0x20B 条目 {u32 0, u32 0x80000000, u64 payload}", "session-save\\decomp\\decomp-0x141328ac0.txt"),
 "0x140b46a50": ("O：save_folder_path+\"loading_background.jpg\"（v2 名；v1 名 screenshot_path 错位）", "session-save\\decomp\\decomp-0x140b46a50.txt"),
 "0x140b46b00": ("O：清空后 paths::saves() readdir 重建", "round2\\decomp-0x140b46b00.txt"),
 "0x140b46d60": ("O：deleted_saves()/uuid 标记文件+rename 两段删除；失败日志 Error deleting glade: ", "session-save\\decomp\\decomp-0x140b46d60.txt"),
 "0x140b47840": ("I：SessionFlowPlugin 域 build（Bevy 事件发送胶水：component.rs/event registry 路径证据）", "session-save\\decomp\\decomp-0x140b47840.txt"),
 },
 {
 "GladeType::assets_folder": "I：主题资产目录名（与 assets_root 叶名表同源）",
 "GladeType::theme_id": "U：主题 id 映射（未见调用点）",
 "GladeType::audio_id": "U：主题音频 id（未见调用点）",
 "SessionId::snapshots_folder_path": "I：v2 已改名 session_file_list（0x140b46300）；本名疑为旧符号",
 "ActiveSession::new": "O：邻域机器码 0x140b468f0 = {+0x00:1, 字节字段空, +0x18:i64::MIN(Option None), +0x58:glade=0}",
 "ActiveSession::assets_root": "I：同 SessionMeta::assets_root 模式",
 "ActiveSession::is_winter": "O：委托 GladeType::is_winter（+0x58==4 选冬季水面，wp7）",
 "ActiveSession::is_autumn": "O：委托 GladeType::is_autumn",
 "ActiveSession::is_olden": "O：委托 GladeType::is_olden",
 "ActiveSession::is_flowery": "O：委托 GladeType::is_flowery",
 "impl$6::from": "U：GladeType 转换（判别字节→枚举）",
 "impl$7::from": "U：GladeType 转换",
 "NextSession::load_latest": "I：构造加载最新会话请求（v2 无独立符号；并入会话流）",
 "NextSession::append_on_load": "O：泛型母版；两实例已证（0x1403b5150/0x141328ac0）",
 "SavedSession::screenshot_path": "I：同 loading_background_path 模式 = save_folder+\"screenshot.jpg\"",
 "load_saved_session_list": "O：wp13 已证按 SystemTime 排序（sort_by_key）",
 })

data["resources/loading_screen.rs.md"] = (
 "启动加载屏：LoadingScreenPlugin（注册 LoadingScreenProgress + Startup/Update 系统）+ 加载屏网格与着色器。"
 "startup_loading_screen 加载 mesh \"loading_screen\"(hash 0x42ac8c61) + shader \"loading_screen/loading_screen.raster.hlsl\""
 "并 spawn (DrawableMeshBundle, DesignatedRenderPass(0x12), DontCastShadow)；update_loading_screen 为内联函数（无独立符号），逐帧推进加载进度。",
 {
 "0x140a1a190": ("O：Plugin::build（v2 名；v1 名 update_loading_screen 错位）= Bevy 系统注册 + hashbrown 表维护（键=占位 UUID 33333333-3333-3333-3333-333333333333，0xd0B 条目 progress@+0x58）", "session-save\\decomp\\decomp-0x140a1a190.txt, " + R2C),
 "0x140a1a1e0": ("O：Startup 系统：once 初始化 → load_mesh_into_library(\"loading_screen\") + load_shader_into_library(\"loading_screen/loading_screen.raster.hlsl\") → spawn 0x88B bundle（vt 0x1412da5d0, +0x48=0x12 渲染 pass, +0x58=0 DontCastShadow）", "session-save\\decomp\\decomp-0x140a1a1e0.txt"),
 },
 {
 "update_loading_screen": "I（内联）：每帧推进加载进度；宿主签名 (ResMut<LoadingScreenProgress>, Res<LogoAnimation>, FilteredWallTime) 为 wp 早期证据；键=占位 UUID 已证（build 内）",
 })

data["resources/logo_animation.rs.md"] = (
 "动画 logo 资源与系统：5 阶段揭示状态机（progress[5]@+0..+0x14 + durations[5]@+0x14..+0x28），"
 "阶段 0 播 FMOD event:/UI/Logo/Scribble、阶段 2 播 event:/UI/Logo/Branches；wait/finish 握手（+0x0c 等待进度、+0x10 完成揭示）；"
 "startup_animated_logo 加载 tiny_glade_logo/logo_branches JSON 网格与 raster.hlsl 着色器。",
 {
 "0x140b1a6b0": ("O：Update 系统 (ResMut<LogoAnimation>, Res<Time>, Res<AudioCore>)：找首个进度<1 阶段 i，stage[i]=clamp(stage[i]+dt/duration[i],0,1)；阶段 0 起播 Scribble、阶段 2 起播 Branches（串 0x142ae945b/0x142ae9472）", "session-save\\decomp\\decomp-0x140b1a6b0.txt"),
 "0x140b19d00": ("O：play = 清零 +0x00..+0x14（5 阶段进度）", "session-save\\decomp\\decomp-0x140b19d00.txt"),
 "0x140b19d10": ("O：is_waiting = (+0x0c)>0.0（movss+ucomiss+seta）", "session-save\\decomp\\decomp-0x140b19d10.txt"),
 "0x140b19d20": ("O：is_finished = (+0x10)>=1.0（常量@0x142ae8b10；实代码 0x140b19d22，v2 地址按对齐）", DG),
 "0x140b19d30": ("O：finish_wait = +0x0c:=1.0f（实代码 0x140b19d33；v1 名/地址整体错位一位）", DG),
 "0x140b19d40": ("O：Plugin::build = insert_resource(LogoAnimation 默认：progress 全 0 + durations{const×4, 0.5f}) + Startup startup_animated_logo + Update animate_logo", DG),
 "0x140b19d90": ("O：Startup 系统：load JSON 网格 tiny_glade_logo/logo_branches + 着色器 logo_animation.raster.hlsl/logo_branches_animation.raster.hlsl + spawn (KeepOnLevelUnload, DrawableMeshBundle, DesignatedRenderPass, DontCastShadow)", "session-save\\decomp\\decomp-0x140b19d90.txt"),
 },
 {})

data["resources/prefab.rs.md"] = (
 "prefab 库资源：hashbrown HashMap<PrefabName, Prefab>（RandomState；桶 0x28B = hash 8B + 键 16B + 值 16B；"
 "Prefab 值为 {ptr,len} 胖指针类——reconstruction 的 Vec 布局存疑）+ Bevy 组件注册。",
 {
 "0x1408e2540": ("O：HashMap::insert（被替换旧值逐 PrefabPart drop）", "round3\\decomp-0x1408e2540.txt"),
 "0x141552d70": ("O：hashbrown 查表（0x1405b1a70 哈希 + SIMD group probe + memcmp）返回 &Prefab", DG),
 "0x141552ff0": ("O：同 get 返回 bool（contains_key）", DG),
 "0x1408e5d10": ("O：Bevy 组件注册（PrefabInstance/PrefabInstanceData 等）", "round3\\decomp-0x1408e5d10.txt"),
 },
 {
 "PrefabLibrary::iter": "I：遍历 entry（名称推断）",
 })

data["systems/prefab.rs.md"] = (
 "prefab 实例化系统族：spawn（命令→库查找→RT 实例+音频触发）、update（GPU 实例数据刷新）、"
 "despawn（内容清理+实体销毁）、hot_reload（watch→加载 JSON→despawn→audio→rt 全链重建）；PrefabPlugin 注册 2 个 Bevy 类型。",
 {
 "0x140840430": ("O：build = 注册 2 个 Bevy 类型（type-id 0xa73802c6af7eaf33/0x188dddb3c672d859 与 0x98f655b4397350da/0x107a31ef8cad0cb1，I：PrefabLibrary+PrefabsModifiedCmd）+ 系统配置", "session-save\\decomp\\decomp-0x140840430.txt"),
 "0x140002370": ("O：clone_behavior = xor eax,eax; ret（返回 0/false；13B 小函数）", DG),
 "0x1408446b0": ("O：Bevy 注册胶水（ComponentId/registry/drop，非业务）", "session-save\\decomp\\decomp-0x1408446b0.txt"),
 "0x140840610": ("O：spawn_prefabs = 事件消费→库查找→实例创建（panic 路径 prefab.rs @0x142a94158；Vec grow 0x140562ab0）", "round3\\decomp-0x140840610.txt"),
 "0x140841550": ("O：create_prefab_rt = Prefab→位置查表(变体)→四元数旋转矩阵+缩放→GPU 实例缓冲 push+颜色打包 u32(b<<16|g<<8|r)→条件 spawn（+0x84 标志 &1）", "session-save\\decomp\\decomp-0x140841550.txt"),
 "0x140841b00": ("O：create_prefab_gpu_instances = 音频触发创建链（audio_trigger_library 0x140a382d0 + 0x1408b9f20；v1 批名 create_prefab_audio_trigger 错位）", "session-save\\decomp\\decomp-0x140841b00.txt"),
 "0x140841d60": ("O：update_prefabs = GPU 实例数据刷新（触 audio_trigger_library.rs 路径）", "session-save\\decomp\\decomp-0x140841d60.txt"),
 "0x140842d60": ("O：despawn_prefabs = 逐条目（0x58 步长）despawn_prefab_contents + Commands::despawn(0x140d14770)", "session-save\\decomp\\decomp-0x140842d60.txt"),
 "0x140843390": ("O：despawn_prefab_contents = 内容清理（0x58 步长条目）", "session-save\\decomp\\decomp-0x140843390.txt"),
 "0x140843740": ("O：hot_reload_prefabs = watch 事件→加载 JSON（0x140b712b0 族）→despawn_prefab_contents→create_prefab_audio_trigger→create_prefab_rt 全链重建", "session-save\\decomp\\decomp-0x140843740.txt"),
 },
 {
 "create_prefab_audio_trigger": "I（内联）：音频触发创建（并入 create_prefab_gpu_instances 0x140841b00 链）",
 })

data["systems/save/load_completion_checker.rs.md"] = (
 "读档完成检查：SessionLoadReadiness（is_ok/deny_load 常量折叠）、is_session_ok_to_load / check_load_completion"
 " = 会话文件存在性检查（save_folder_path + File::create 探测）。",
 {
 "0x140a9f810": ("O：check_load_completion = save_folder_path + join + FUN_140d69590 + File::create×2（文件存在性/就绪检查；精确条件 I）", "session-save\\decomp\\decomp-0x140a9f810.txt"),
 "0x140833230": ("O：SessionLoadReadiness::is_ok（is_initial 同址折叠）", DG),
 "0x140a9fa70": ("O：deny_load = tag==1 判定（与 GladeType::is_olden 同址折叠）", DG),
 "0x140a9fa80": ("O：is_session_ok_to_load = save_folder_path×2 + join×2 + File::create×4（文件存在性检查）", "session-save\\decomp\\decomp-0x140a9fa80.txt"),
 },
 {})

data["systems/save/load_game.rs.md"] = (
 "读档主系统：LoadGameManager 状态机（LoadGameState::LoadGameData）→ 写 loading.tmp → NextSessionKind 解码"
 "（1=parse_saved_game_data 含 upgrade 备份、2=on_load 命令回放+内存快照）→ PostGameLoadCommand 回放（SwitchAppMode 忽略）"
 "→ 旧版颜色 ID 升级（{1→2,2→5,4→8,5→3,8→4}）→ extract_max_wall_id_from_history。",
 {
 "0x14146d030": ("O：主系统（13+ 段 SystemParam）：状态机+loading.tmp+kind 解码+命令回放+升级+wall id 恢复（详见 DIGEST §3）", "session-save\\decomp\\decomp-0x14146d030.txt"),
 "0x141470d00": ("O：parse_saved_game_data（内联于 load_game）：Making save file backup for upgrade→Reading file data→Parsing JSON→Version→BackwardCompatibility→Camera→parsing camera→Time of day（串块 0x142c64450）", "session-save\\decomp\\decomp-0x14146d030.txt"),
 },
 {
 "LoadGameManager::new": "I：构造（状态机初态）",
 "apply_parsed_game_data": "O（内联）：解析数据应用到世界（load_game 内联段：资源拷贝 0x1b0/0x280 memcpy + init_game_state_from_snapshot 调用）",
 "apply_backward_compatibility_overrides": "O（内联）：版本迁移覆盖（Version/BackwardCompatibility 节消费）",
 "upgrade_window_color": "O（内联）：颜色 ID 映射 {1→2,2→5,4→8,5→3,8→4}（load_game 反编译行 1068-1143；日志 Detected old version of window color IDs. Upgrading...）",
 "extract_max_wall_id_from_history": "O（内联）：遍历 MultiEdit（变体 7/8/9，niche 解码）取最大 wall id 写计数器 +0xc0（行 1179-1268）",
 })

data["systems/save/mod.rs.md"] = (
 "save 系统族入口：add_save_load_systems / add_save_load_events_and_resources 注册存档系统与事件。",
 {
 "0x1409443b0": ("O：add_save_load_systems = 注册 save_game/load_game/save_thread/check_load_completion 等", "round3\\decomp-0x1409443b0.txt"),
 "0x1409447e0": ("O：add_save_load_events_and_resources = 注册事件/资源（SaveGameCmd/SavingGameComplete/LoadingGameComplete 等）", "session-save\\decomp\\decomp-0x1409447e0.txt"),
 },
 {})

data["systems/save/save_backup_list.rs.md"] = (
 "备份存档列表：SaveBackupList::new_from_session 扫描备份目录（文件名前缀 history-auto）建列表；"
 "BackupSaveFileKind::is_corrupted（tag==4）。",
 {
 "0x140ae8320": ("O：is_corrupted = tag==4 判定（与 GladeType::is_winter 同址折叠；v1 批名 is_auto 错位）", "session-save\\decomp\\decomp-0x140ae8320.txt"),
 "0x140ae8330": ("O：iter_backup_files = 迭代备份文件", "round3\\decomp-0x140ae8330.txt"),
 "0x140ae8470": ("O：new_from_session = 扫备份目录建列表；错误日志 Error reading save backup directory / Error creating backup save folder: ；前缀 history-auto", "round3\\decomp-0x140ae8470.txt"),
 },
 {
 "BackupSaveFileKind::is_auto": "I（内联）：自动备份判定（v2 无独立符号；文件名前缀判定候选）",
 "SaveBackupList::into_files": "I：Vec 消费转换",
 "SaveBackupList::backup_metronome_counter": "I：备份节拍计数（关联 SaveBackupMetronome）",
 })

data["systems/save/save_backup_metronome.rs.md"] = (
 "备份节拍器（全内联文件，无独立符号）：累积活动游戏时间，达阈值触发备份。"
 "O 数值（内联于 save_game 0x140a3f470 行 507-591）：{counter u32@+0x140, 累计 f32@+0x144 clamp[0,1], 阈值 f32@+0x148}；"
 "仅阈值<60.0 时累计；档位表 {300,600,3600,7200}s（5min/10min/1h/2h）@0x142a9f348；断言 active_game_delta_seconds >= 0.0。",
 {},
 {})

data["systems/save/save_file_identity.rs.md"] = (
 "存档文件身份识别：try_approx_parse 读文件搜字面量 \"Identity\" 后解析 JSON 值；"
 "derive_fallback 失败回退（文件元数据派生）。",
 {
 "0x141e58f20": ("O：try_approx_parse = 0x800B 缓冲→ReadFile→搜 \"Identity\"（串 0x142f92a40 len 10）→解析其后 JSON 为 SaveFileIdentity", "session-save\\decomp\\decomp-0x141e58f20.txt, " + R2C),
 "0x141673140": ("O：derive_fallback = 解析失败回退：SystemTime + 二次尝试 + 元数据派生（字段 U）", "session-save\\decomp\\decomp-0x141673140.txt"),
 },
 {})

data["systems/save/save_game.rs.md"] = (
 "主存档系统：save_game（6 段 SystemParam；节拍器内联；capture→Arc→set_current_snapshot(0x140a84760)→"
 "发送 AsyncSaveCmd::CaptureSnapshot(9)）；SaveFileSection 10 变体节协议（json_key/padding 表 @0x142ace328/0x142ace378/0x142ace3c8）；"
 "save_game_to_file 手工拼 JSON（'{'、\"History\":{、edit 间 \",\\n\"）；ensure_current_snapshot 增量历史写（mark_dirty 协议）；"
 "to_json_string 族 = 各节 serde 载荷（Colors=GladeColorCustomization、Identity=HistoryIdentity、Camera=CameraRigParametersCache、"
 "TimeOfDay、Sheep=Option<SheepControlSnapshot>、chain_index=HistoryChainIndex、snapshot=HistoryChainSnapshots）。",
 {
 "0x140a3f470": ("O：主系统（节拍器+快照捕获+AsyncSaveCmd 发送；错误串 Error while performing a synchronous save / Sync response from save thread / Failed writing session meta）", "session-save\\decomp\\decomp-0x140a3f470.txt"),
 "0x140241ca0": ("O：save_game_to_file<File>（手工 JSON：'{' 0x7b、\"History\":{、\",\\n\" 0xa2c 分隔）", "session-save\\decomp\\decomp-0x140241ca0.txt"),
 "0x141750c90": ("O：save_game_to_file<Cursor<Vec<u8>>>（同 File 实例）", "session-save\\decomp\\decomp-0x141750c90.txt"),
 "0x140242b40": ("O：write_json_sections<File>（节级序列化+padding）", "session-save\\decomp\\decomp-0x140242b40.txt"),
 "0x14085ee90": ("O：to_json_string<GladeColorCustomization> = serde_json::to_string（Colors 节载荷）", "session-save\\decomp\\decomp-0x14085ee90.txt"),
 "0x140b38810": ("O：to_json_string<HistoryChainSnapshots>（snapshot 节载荷）", "session-save\\decomp\\decomp-0x140b38810.txt"),
 "0x140b385e0": ("O：to_json_string<HistoryIdentity>（Identity 节载荷）", "session-save\\decomp\\decomp-0x140b385e0.txt"),
 "0x1408aa7a0": ("O：to_json_string<HistoryChainIndex>（chain_index 节载荷）", "session-save\\decomp\\decomp-0x1408aa7a0.txt"),
 "0x140b389b0": ("O：to_json_string<TimeOfDay>（Time of day 节载荷）", "session-save\\decomp\\decomp-0x140b389b0.txt"),
 "0x1416f8470": ("O：to_json_string<CameraRigParametersCache>（Camera 节载荷；相机 schema 见 camera-time 域）", "session-save\\decomp\\decomp-0x1416f8470.txt"),
 "0x140a51d40": ("O：to_json_string<Option<SheepControlSnapshot>>（Sheep 节载荷）", "session-save\\decomp\\decomp-0x140a51d40.txt"),
 "0x140a43810": ("O：json_key = tag 字节→len 表(0x142ace328)+ptr 表(0x142ace378) 双查表（NotFound 手解：movzx/shl 3/双 mov）", DG),
 "0x140a43830": ("O：padding_bytes = 表 0x142ace3c8[tag] = [0,16,0,128,32,128,64,8,8,2048]", DG),
 "0x140a43840": ("O：AutoSaveState::close_file = 清 +0x120 字段+清 cap-first 列表（v1 批名 json_key 错位）", DG),
 "0x141e59860": ("O：create_backup_of<PathBuf>（备份目录创建）", "session-save\\decomp\\decomp-0x141e59860.txt"),
 "0x140a439a0": ("O：last_error_state（错误状态访问）", "session-save\\decomp\\decomp-0x140a439a0.txt"),
 "0x141751c90": ("O：write_json_sections<Cursor>", "session-save\\decomp\\decomp-0x141751c90.txt"),
 "0x140a439b0": ("O：add_history_edit_to_save_file / v1 双名 ensure_current_snapshot = 追加 edit 计数+序列化 0x80B+\",\\n\"(0xa2c)+ShadowFile::mark_dirty(start,end)", "session-save\\decomp\\decomp-0x140a439b0.txt"),
 "0x1417522e0": ("O：save_session_meta_to_file<PathBuf>（SessionMeta 落盘，被 write_to_disk 调用）", "session-save\\decomp\\decomp-0x1417522e0.txt"),
 },
 {})

data["systems/save/save_thread.rs.md"] = (
 "存档线程：save_thread 主循环（脏 shadow 文件逐 tick flush → mutex+condvar 等待≤1s → 错误列表 diff/日志 → recv 命令）；"
 "execute_save_cmd = AsyncSaveCmd 分发（3=OpenFile/4=CloseFile/5=ChangeSection/6=ReplaceEdits/7=BackupSave/8=ManualBackup/9=CaptureSnapshot/10=Flush/11=Shutdown）；"
 "snapshot_gc_thread 清理未引用快照；CurrentSaveFile 增量历史写（Edit appended to save file + mark_dirty 协议）。",
 {
 "0x141721aa0": ("O：execute_save_cmd = switch{3,4,5,6,7,9,10}+default(含 8=ManualBackup)；OpenFile 体内 join \"snapshots\"+Rewriting history file+Saving the game to a temp file+creating snapshot folder", "session-save\\decomp\\decomp-0x141721aa0.txt"),
 "0x141720680": ("O：save_thread 主循环（condvar 1s；cmd==0xb → flush_deferred_and_sync+退出；Save thread I/O error; affected file count 日志）", "session-save\\decomp\\decomp-0x141720680.txt"),
 "0x14171ece0": ("O：CurrentSaveFile::change_section = 修改当前节+ShadowFile::mark_dirty(0x14091a450)×2（脏区间协议互证）", "round3\\decomp-0x14171ece0.txt"),
 "0x14171f7f0": ("O：flush_deferred_and_sync（游戏线程断开时落盘；错误日志 Error flushing save file on game thread disconnect.）", "session-save\\decomp\\decomp-0x14171f7f0.txt"),
 "0x14171ff80": ("O：UnsavedFileList::add（v2 名；v1 名 backup_save 同址）= 锁+condvar 通知+内联实现", "session-save\\decomp\\decomp-0x14171ff80.txt"),
 "0x1417201a0": ("O：contains_file = 列表查找（0x40B 条目 memcmp）", "session-save\\decomp\\decomp-0x1417201a0.txt"),
 "0x141720490": ("O：impl$2::from（v1 名 contains_file::closure$0）= 文件条目比较辅助", "session-save\\decomp\\decomp-0x141720490.txt"),
 "0x140b92210": ("O：AutosaveErrorState::is_ok = [rcx+0x10]==0 sete（is_empty 折叠，NotFound 手解）", DG),
 "0x14172b280": ("O：snapshot_gc_thread = parse_save_file_snapshots→目录比对→Deleting unreferenced snapshot；竞态守卫 Save file changed before snapshot GC could finish...", "session-save\\decomp\\decomp-0x14172b280.txt"),
 },
 {
 "CurrentSaveFile::replace_edits": "I：替换历史 edit 列表（execute_save_cmd 6 号命令载荷）",
 "CurrentSaveFile::backup_save": "I：备份写盘（BackupSave 命令 7 的执行体；history-auto 前缀）",
 "CurrentSaveFile::perform_manual_backup": "I：手动备份（8 号命令执行体；Copying inactive save file to backup）",
 "execute_open_file_cmd": "I（内联）：OpenFile 命令执行体（并入 execute_save_cmd case 3）",
 "full_write_history_file": "O（内联）：整写历史文件（Rewriting history file 串 0x142ccaf31；并入 OpenFile 流程）",
 "capture_snapshot": "I（内联）：快照捕获执行体（case 9）",
 "nuke_unreferenced_shapshots": "O（内联）：删除未引用快照（GC 线程核心；Deleting unreferenced snapshot）",
 })

data["systems/save/shadow_file.rs.md"] = (
 "影子文件原子写协议：ShadowFile{+0x28/+0x30 数据 (ptr,len), +0x40/+0x48 文件状态（MIN 哨兵+句柄）, +0x68 脏块计数(8 槽 &7)}；"
 "create 初始化；flush = 打开 <path>.tmp→Write data→Setting file length→Sync data→Save file successfully opened and written to disk；"
 "sync_all = Persist temp file（Asking the OS to sync writes to disk / flushed_block_count）；mark_dirty(start,end) 记录脏字节区间。",
 {
 "0x1409184d0": ("O：create = 初始化影子文件（未开状态；ShadowBackendFile is not open 前置）", "round3\\decomp-0x1409184d0.txt"),
 "0x1409193d0": ("O：flush（v2 名；v1 名 unwrap_open_mut 同址）= 未开则开：<path>.tmp 创建→写数据→set_length→sync→成功日志；错误 Could not create temp file for save rewrite: / Error writing save data to temp file: ", "session-save\\decomp\\decomp-0x1409193d0.txt"),
 "0x14091a230": ("O：sync_all（v1 名 flush_impl）= Persist temp file：写盘+同步+flushed_block_count 复位", "session-save\\decomp\\decomp-0x14091a230.txt"),
 "0x14091a450": ("O：mark_dirty = 记录脏字节区间（Marking block range dirty: ..；被 change_section/ensure_current_snapshot 调用）", "session-save\\decomp\\decomp-0x14091a450.txt"),
 },
 {})

data["systems/save/snapshot.rs.md"] = (
 "快照捕获/加载：capture_game_snapshot 巨型系统（184KB）串化全 Query 为 JSON 文档（节序 Version→Walls→Roofs→Decorators→"
 "WallColors→GateColors→Terrain(flate2 deflate)→Trees→Raster）；capture_game_snapshot_save_data 构建 0x400B 结构并返回 Arc；"
 "validate_and_load_closest_snapshot_data 解析快照 JSON（parsing 全串+Version mismatch）；init_game_state_from_snapshot 应用数据。",
 {
 "0x1417a0430": ("O：capture_game_snapshot_save_data = 从世界数据构建快照结构（~0x400B），返回 Arc（alloc 0x18 {1,1}+LOCK inc）", "session-save\\decomp\\decomp-0x1417a0430.txt"),
 "0x1417a1d00": ("O：capture_game_snapshot 系统体 = 巨型串化（StairsSnapshot{edges,checksum}/TerrainEditingData{strokes,next_stroke_id,next_stroke_index}/DecoratorStorageSnapshot{states} 等）", "session-save\\decomp\\decomp-0x1417a1d00.txt"),
 "0x1417ab290": ("O：init_state（v1 名 get_param 同址）= Bevy SystemParam 胶水（20+ param 函数链，非业务）", "session-save\\decomp\\decomp-0x1417ab290.txt"),
 "0x1417a6d70": ("O：try_chunk = Result<Option<chunk>>（分块/文件读取）", "round3\\decomp-0x1417a6d70.txt"),
 "0x1417a6e50": ("O：validate_and_load_closest_snapshot_data（v1 名 validate_param 错位）= 快照 JSON 解析器：parsing Version/Walls/Roofs/Decorators/WallColors/GateColors/Terrain(decompressing terrain data)/Trees/Raster；Version mismatch; expected , got ；Ignoring unknown raster data in snapshot", "session-save\\decomp\\decomp-0x1417a6e50.txt"),
 "0x1417a9c50": ("O：init_game_state_from_snapshot（v1 名 try_load_snapshot_data）= 应用解析数据到世界（被 load_game 调用）", "session-save\\decomp\\decomp-0x1417a9c50.txt"),
 },
 {})

data["systems/save/snapshot_gc.rs.md"] = (
 "快照 GC 辅助：parse_save_file_snapshots 从存档文件解析快照引用集合，供 snapshot_gc_thread 判定未引用快照。",
 {
 "0x141286730": ("O：parse_save_file_snapshots<BufReader<File>> = 解析存档内快照引用（GC 线程消费；错误 Error parsing history snapshots fromfile）", "session-save\\decomp\\decomp-0x141286730.txt"),
 },
 {})

data["systems/render_loop/session_selection/mod.rs.md"] = (
 "会话选择界面渲染（邻域）：SessionSelectionPlugin 注册；thumbnails_startup/render_session_selection_previews 渲染存档缩略图。",
 {
 "0x1408f68c0": ("O：build = 注册资源(FUN_140b67dd0)+系统(FUN_1415451c0) 小插件", "round2\\decomp-0x1408f68c0.txt"),
 "0x1408f6900": ("O：thumbnails_startup = 缩略图启动系统", "round3\\decomp-0x1408f6900.txt"),
 "0x1408f6c00": ("O：render_session_selection_previews = 会话缩略图渲染系统（120KB；渲染域深挖留待 render 域）", "round2\\decomp-0x1408f6c00.txt"),
 },
 {})


def apply(path, purpose, fns, inlines):
    p = os.path.join(BASE, path)
    txt = open(p, encoding='utf-8').read()
    txt = re.sub(r"^> 文件作用：\*\*待推断\*\*（由章节 agent 按函数名\+证据填写）$",
                 "> 文件作用：" + purpose, txt, flags=re.M)

    def row_sub(m):
        name, addr, line, act, concl, ev = m.groups()
        if addr in fns:
            c, e = fns[addr]
            concl, ev = c, e
        if concl == "待证":
            concl = "I（无直接证据，名称推断）"
        if not ev.strip():
            ev = "—"
        return "| `" + name + "` | " + addr + " | " + line + " | " + act + " | " + concl + " | " + ev + " |"

    txt = re.sub(r"\| (`[^`]+`) \| (0x[0-9a-f]+) \| (\d+) \| ([^|]*) \| ([^|]*) \| ([^|]*)\|",
                 row_sub, txt)

    for name, inf in inlines.items():
        pat = r"(- `" + re.escape(name) + r"`（行 [^）]*）— )[^\n]*"
        txt = re.sub(pat, lambda m: m.group(1) + inf, txt)

    open(p, 'w', encoding='utf-8').write(txt)
    return txt.count("待证")

total_left = 0
for path, (purpose, fns, inlines) in data.items():
    p = os.path.join(BASE, path)
    if os.path.exists(p):
        left = apply(path, purpose, fns, inlines)
        total_left += left
        print(path, "-> remaining 待证:", left)
    else:
        print(path, "MISSING")
print("TOTAL remaining 待证:", total_left)
