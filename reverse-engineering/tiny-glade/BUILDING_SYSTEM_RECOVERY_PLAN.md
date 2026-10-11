# 完整建筑系统攻关计划

日期：2026-10-11。状态：已进入实施，第一批 CPU 消费者已接入真实 Bevy World。主 agent 管共享契约和连续验证，三个 agent 分别恢复墙构造、屋顶/门窗和历史/高度命令。本计划更新建筑子系统的后续顺序；[三版本总体计划](THREE_VERSION_RUST_PLAN.md)继续约束完整游戏目标。

## 实施增量与下一断点

原始 raw Curve2 → terrain mapped outline → 四路 PathRasters → prepared Curve3/segment scalar → regular/freeform/gable caches → 原墙开口 → 完整 from_curve → 96 字节砖实例已接入真实 Bevy World。显式 SetResolvedWallProposals 输入已删除；seed 使用初始 mapped 点数，构造、cache、门窗 subtype 使用原 prepared 曲线。缺少实际路径资源时撤除旧输出并保持 pending，不补造零资源。

正式原指令证明：普通分支85例/6476记录，其他 kind/top/fence 分支304例/21846记录；空路径128、非空路径128，自由墙 cache1536及 shape-height384；独立阶段 review 另覆盖生产320例/23360记录、路径152例/2236点与策略544例。范围见[第二阶段审查](evidence/building-integrate/stage-reviews/2026-10-11-stage2.md)。报告随源码变更必须重执行，旧 hash 不代表当前全工程验收。

当前已接原始 mask 和三项 material-stat 数据，656例/3608 mask 的完整 producer 已有原码证明。Roof 删除连续场景暴露最终 cache 新而 CPU brick 仍旧的调度 P1，已修正消费者顺序并增加同帧与下一 idle 帧一致性回归。正在扩展运行链 oracle，区段不能仅靠局部 mask 通过来验收。

### 下一里程碑

G05 regular/freehand arches 已由实际 raw/raster/cache/masks 接入 World，96字节结构输出、final arch/third 和真实120字节 ConstructGateCmd 均按原码比较；依赖EventId交Bevy。G03实际80字节装饰源→台阶cells/碰撞几何/96字节渲染记录/holes/stats→完整墙砖已接入World；扩大consumer研究仍保留输入schema不一致的失败，不据局部证明宣告G03系统全完成。G14统一Wall/Roof外层buckets和墙内组barrier，作者创建/直接删除与完整bulldoze收缩动效、source恢复和撤销/重做已接运行链。阶段4源码冻结全检查已结束：54测试/67限定证明。新并行批修改后的工程暂不验证，冻结证据保持历史快照。

下一最高收益：关闭扩大stairs consumer输入边界，恢复G06真实Gate120→DoorMesh/TransientMesh/Nani和实体增删消费者，继续矩形enclosure finish/cancel、真实UI placement/startup及未知published字段，不用通知或占位mesh代替最终消费者。

随后推进 corners/skirts、楼板/栅栏/门、其他装饰/楼梯/材质与各历史 owner；C 迁移、原调度/任务、存档与 GPU 均保留最终验收要求。G00–G19 尚无整包全系统验收，system_acceptances=0。

## 目标与完成边界

当前实现与主要原指令证明以 B 为基线；最终交付目标仍为 C v1.16.0，A 用于追溯。完整建筑系统必须贯通：

`编辑输入/历史命令 → 权威对象与 ID → 墙/屋顶/附着关系 → 高度/相交/开口 → 砖、结构与门窗输出 → 注册/任务/渲染消费者 → 删除、撤销、保存与恢复`

源码阶段先完成权威状态与 CPU 链，GPU 执行和原作运行对照后置但属于最终必做。地形、道路、水和环境整套系统不纳入本次建筑实现范围；建筑依赖的地形标高、道路开口、楼梯落点、碰撞与相关历史操作必须有真实接口和消费者，不能用永久常量替代。

建筑内容包括矩形/圆形/自由墙，斜顶/平顶及已观察样式，砖墙/木墙/半木结构，拱/梁/支撑/楼板/栅栏/门，普通窗/阳台门/转角窗/屋顶窗/活动门等已观察装饰，烟囱/旗连接、门阶和平台/玩家楼梯，建筑颜色、材质与灯光状态。源路径出现只证明候选存在；具体行为、版本可用性和依赖关系仍需消费者确认。

## 当前确实完成到哪里

| 环节 | 已接业务范围 | 当前剩余边界 |
| --- | --- | --- |
| 基础工程 | 阶段4冻结 workspace 和真实 Bevy World；原依赖身份有历史对照 | 本批新增 Rapier、门、编辑、装饰模块尚未整体验证；原 fork 与 C 迁移 |
| 墙与砖 | Raw → terrain/raster/prepared → cache/masks → openings → noise/seed/height → 完整96B砖输出；显式proposal输入已删除 | 完整 UI/多墙几何连接、墙角裙边及所有结构族 |
| 拱与台阶 | 规则/自由拱、过梁、最终第三高度；实际80B装饰源→台阶几何/记录/holes/stats接World | 更广结构/门阶/玩家楼梯；平台碰撞新运行链未验证 |
| 屋顶 | 创建/关联/方向/Flat/Rim/参数消费者、CPU瓦、山墙、invalid/fade/delete | TriggerVolume生成、尖顶/梁实几何、模拟、所有参数录制、GPU |
| 门 | Gate120→CPU木门/支撑/地下post fallback；新mesh/entity/Nani增删消费者已登记 | 本批实现未验证；原MeshLib/ShaderLib/SSBO/GPU资源所有权 |
| 装饰开口 | 四族实际资产/subtype→holes→墙砖；新增Info72原始字段→原坐标consumer | 新字段和storage绑定未验证；非墙宿主/其他装饰、实际anchor更新 |
| 编辑与历史 | 已验收范围内Wall/Roof buckets、作者创建/删除/bulldoze与undo/redo | 新矩形finish/cancel/Enclosure、free-wall pending、装饰存储恢复未验证；其余owner/存档 |
| 验证 | 最近完整快照阶段4：54 tests / 67 scoped verifiers | 用户要求当前并行批暂缓验证；system_acceptances仍0 |

当前并行批遵循用户最新指令：三个agent分别推进Gate/栅栏、Rapier/DirtyTiles/查询、矩形/Decorator历史与放置；父管理共享类型、Bevy登记、Nani和CLI。冲突及未解析字段写入 `tracking/parallel-implementation-conflicts`，新代码统一标为 implemented_unverified。不将局部静态恢复、旧原码报告或登记完成计为完整模块验收。

保留未关闭失败：ARM 矩形铺瓦 case14 的一条 filler Y 相差 1ULP。不得降低比较精度、改 oracle 或手写 glam 掩盖失败。旧 Windows 报告与当前构建报告分开。

当前 `tracking/module-coverage.json` 的 46 个 crate 目录仍多为旧的静态 `not_assessed`，不能直接当作本轮缺口表；同样不能用 crate、源文件或函数数量计算完成率。本计划按业务链补充清单，不篡改旧实现/验收状态。

## 二十个攻关包

下列包是完整攻关清单；当前状态以顶部实施增量和机器台账为准，原规划审计表保留当时的起点。原模块均为证据中的路径/命名空间，不代表取回了作者源码文本。详细候选、地址及三版差异放在审计证据中。

### G00：原模块与版本契约清单（P0，父 agent）

- 将 InnerWall/PublicWall、Roof、Decorator、Stairs、WallMaterial、History owner 和各 CPU 输出列成接口表；标明原字段、已解析输入、adapter 字段、未知值和版本差异。
- 每个待实现入口先查三版 Cargo/PDB 定义与 `dependency-exclusions.json`。std、Bevy、glam、fastrand、Serde、容器、通用碰撞库复用；同 checksum 的 private 游戏规则仍需恢复。私有 local 包不能自动认定为游戏业务。
- 退出：每个包有权威输入、实际消费者、删除策略、版本范围、Rust 映射和验收场景。未闭合边进入清单，不通过猜默认值消失。

### G01：墙权威对象与完整编辑（P0）

- 原入口：`country-core/resources/walls/{inner_wall_state,inner_walls,in_progress_wall,wall_attachment}`，`systems/wall/shapes/*`、`adjust_wall_height/*`、`delete_wall`。
- 输入/输出：创建、尺寸、旋转、移动、形变、墙高、删除 → 原墙状态、raw Curve2、对象 ID 与 dirty。CLI upsert 已有 Curve2 不能替代作者编辑规则。
- 恢复高度约束查询前段和未命名 special_flag 的生产者；rectangle/circle/freehand 或 path 的实际类型关系先由判别分支确认。
- 前置 G00；退出：连续编辑与取消、同值写入、缺失对象、宿主删除、恢复 ID 均经过原消费者核对，更新现有发布链。

### G02：自由墙缓存与高度前段（P0）

- 原入口：`init_wall_height_and_arch_segment_height`、`calc_shape_height`、PublicWall 缓存生产者；目前 regular cache 对 tag>=2 明确拒绝。
- 输入/输出：开放/闭合 raw 路径、地形、相对高度、山墙与限制 → 基础/逐段高度及 max-y 缓存。
- 前置 G00/G01 的类型契约，可并行恢复解析后的算法；退出：自由路径不再落 UnsupportedShape，移动/地形/屋顶切换不积累旧高度，缺缓存与无变化行为明确。

### G03：墙相交、分段与连接（P0/P1）

- 原入口：`shape_intersections`，wall-constructor 的 `segmentation/{sample_wall_path_intersections,calculate_wall_path_segmentation,wall_path_segment}`。
- 输入/输出：多墙、地形、屋顶、楼梯与邻接关系 → 可构造区段、可见面、重叠/裁切和支撑提案。空间格候选覆盖并非真实几何相交。
- 复用匹配碰撞/几何依赖，恢复游戏筛选、容差、顺序和传播；退出：相接/重叠/包含/角点/开放端、邻墙移动及删除均更新双方，无残留段。

### G04：开口实际消费者与墙砖主体（P0，第一批主攻）

- 原入口：已保存 WallHoleStorage::finalize 实际 caller `system_wall_constructor::construct_walls`，沿 AL 后实际分支和 `WallConstructor` 追踪主体输出；调查 `resolve_hole_overlap`、`trim_rows`、`split_into_random_rows`、wall corners/skirts 的真实调用。`construct_brick_columns` 定义在 elevation support/pillar 模块，列入 G05，未经 CALL 证据不把它当主体墙砖循环。
- 输入/输出：真实 PublicWall、finalized 开口、分段/高度、材质/seed → 有序砖列/实例、裁切边界、墙体 CPU 几何与输出所有权。开口 AABB 数量通过不能替代此消费链。
- 前置：现有 G00 已知契约与已验证开口核心；自由墙与复杂相交分支分别等待 G02/G03，不阻止先恢复合法矩形/圆墙。
- 退出：创建门窗确实改变墙砖/裁切输出；移除、移动、wrap、多洞相交、同值缓存、宿主删除/撤销恢复均连续验证；保留 RNG 最终状态与输出顺序。

### G05：拱、过梁与悬挑/高架支撑（P1）

- 原入口：`construct_arches`、`arch_walker`、`construct_lintels/place_support_beams`、`construct_elevation_supports/*`（包括 `util_pillar_construction::construct_brick_columns`）、`place_pillars`、stone column colliders。墙过梁与屋顶的同名 `place_support_beams` 分开追踪，不能混用证据。
- 输入/输出：墙区段、孔洞、地形/下方交叠、样式与支撑状态 → 石柱、梁、托架、拱轮廓、碰撞及材质统计。
- 前置 G02/G03/G04；退出：悬空/叠层/山墙/门窗挤占等分支有原规则，支撑增删跟随宿主，ZST marker 不计为几何完成。

### G06：楼板、栅栏、门与墙面层（P1）

- 原入口：`construct_floors`、`construct_fences`、`construct_gates`、`plaster_systems`、`curved_halftimber`、wall skirts。
- 输入/输出：墙/洞/区段、地形和材质 → 楼面、栅栏段、门口/门组件、抹灰/边缘层与碰撞。
- 前置 G03/G04；退出：各 family 创建/修改/移除都有实际输出与 owner 清理；门/栅栏不能通过门窗四族规则覆盖。

### G07：屋顶全部参数与类型切换（P0/P1）

- 原入口：`system_roof::edit_roof`、`replay_roof`、Roof 参数/visual shape 和 Flat customization 的真实消费者。
- 输入/输出：height/profile/eave/ridge/tip、方向、Flat/Pitched 样式 → 原 Roof 写入、ticks、geometry/gable/hole/sound、录制 mutation 和 CPU 输出。
- 前置 G00，复用已验证 edit_rules/replay 后段；退出：VisualEdit 真正进入 World；参数同值、Absolute/Delta、取消与缺失对象按原规则，切换清理旧瓦/旧平顶输出。
- 已证有效 Roof 宿主 shape 为 circle/rectangle；其他形状必须先找到原消费者，不凭“完整”要求补造。

### G08：屋顶尖顶、梁、模拟与碰撞输出（P1）

- 原入口：`place_spires`、support/roof tile simulation、`calculate_roof_trigger_volumes`、roof buffers、Nani 更新消费者。当前 TriggerVolume 创建为空，尚无原 triangle/normal/UV 生产；当前 TileSimulation 初始化及 wall_height_dt 不等于原 simulate_roof_tile_for_audio 已恢复。
- 输入/输出：屋顶形状与动画、材质、实例/碰撞资源 → 尖顶/梁 CPU 几何、动态瓦片参数、TriggerVolume 和 drawable/instance 更新与释放。
- 前置 G07，必要资源来自 G09；TriggerVolume/UV 分支按 G10 屋顶定位需要提前至 P0，墙面四族先闭环不必等待全部屋顶视觉。退出：真正消费 RoofSpire/SupportBeams marker；阶段、淡出和删除后所有 owner 清理。Nani Create/Update/Remove 意图单独验证，backend 成功由真实执行证明。

### G09：建筑资源与碰撞元数据（P0，第一批并行）

- 原入口：asset/asset-pipe、mesh JSON/RON、decorator collision/outline/subtype、Nani atlas/prefab/资源 ID 的游戏绑定。
- 输入/输出：发行资源与游戏资源 key → 正确资产变体、碰撞尺寸、轮廓、枢轴、mesh subset 和材质 metadata。
- 先提取窗/门、砖、楼梯等当前 consumer 需要的数据；复用 Serde/RON/rkyv/image 等匹配依赖，恢复游戏 schema/坐标/ID 规则。
- 退出：现有四族不再由 fixture 手填碰撞 extent；尺寸、单位、变体、缺资源与 ID 绑定有实际 caller 证据。解析 JSON 本身不等于复现资产管线。

### G10：原 Decorator 放置与附着状态（P0，接续 G09）

- 原入口：WallSpace 映射、DecoratorInfo::dst_as_wall、target/dst、derive_subtype、collision_detection、缓存 transform 和宿主更新。
- 输入/输出：用户/历史的放置与位置、墙/屋顶、资产碰撞 → 宿主解析、UV/墙局部坐标、投影/吸附/避碰、尺寸/变形、真实 Decorator 实体及 raw holes。
- 前置 G01/G07/G09 已知接口；屋顶挂载还需 G08 的原 TriggerVolume/UV→position，不能用空碰撞体或预解析 roof 坐标冒充。退出：移墙、调高、换屋顶、跨角/周期接缝、冲突、改 subtype、删宿主/撤销和重新附着有连续验证；adapter OpeningAuthority 与原 Bundle 分列。

### G11：其他装饰、屋顶窗与洞贡献（P1）

- 原入口：cottage_corner_window、arrow slit、cottage/gothic dormer、dormer_holes/roof tiles、trap_doors、chimney wall/roof/bricks、flag_connections、decorator_clutter。
- 输入/输出：G10 的附着状态、roof-aware 形变与各 family 资源 → 对应孔洞/屋顶切口/门窗视觉 CPU 记录、烟囱和连接状态。
- 前置 G07/G09/G10；退出：按实际分支建立 family 清单；所有贡献进入同一实际消费者，移除能撤销，dormer tile 与主屋顶切口一致。不将多个 raw rectangle 合并猜成完整开口形状。

### G12：门阶、平台与玩家楼梯（P1）

- 原入口：construct_door_stairs、door_stairs_assemble_bricks、platform/playermade_stairs、stairs structural graph/removed_supports/invalidate，C stairs_wall_holes。
- 输入/输出：门口、墙/平台边缘、地形、用户楼梯参数 → 分段、台阶、托架/柱、碰撞、开口贡献及支撑关系。
- 前置 G03/G05/G10，历史由 G14 接入；退出：坡地、边角、交叠、宿主与地形改变、移除/撤销保持图和实例一致，石/木差异单列。

### G13：半木、颜色、材质与建筑灯光（P1）

- 原入口：timberframe circle/rectangle、color set_wall/decorator/gate/halftimber/roof_customization、window_light、material/elevation stats。
- 输入/输出：样式/材质/颜色/窗口灯、墙屋顶与结构数据 → 原状态、dirty、半木梁/颜色 CPU 字段和历史 mutation。
- 前置 G04/G07/G10；退出：样式切换会触发真实重建/切口策略，复用 palette schema；改色/灯光/半木及其撤销覆盖同值和宿主清理。状态字段与 GPU upload 分列。

### G14：全部建筑历史 owner 与复合事务（P0，第一批并行）

- 原入口：wall_history/wall_compound_edit、roof_edit/replay_roof、decorator_edit/replay_decorator、color_edit/replay_color、stairs_history_edit/replay_stairs 和 replay_pop_bucket。
- 输入/输出：各业务消费者产生的真实 mutation → 录制组、commit/cancel、undo/redo 命令展开、cursor/redo 截断与 ID 恢复。
- 复用已证 Rim ledger 核心；先补 Roof VisualEdit/Create/Delete/Direction 和 Wall，再补 Decorator/Color/Stairs。三个 aux owner 非零的排序/展开须恢复，不能将 Rim 一 edit 一 cmd 的反转推广。
- 退出：跨 owner 复合组按原展开顺序撤销/重做；空组/取消/撤销后首次 edit 截 redo、删除重建、no-record 重放和缺对象明确。整个结果再次驱动 G01–G13 的状态与几何。

### G15：快照、保存/装载与版本迁移（P1）

- 原入口：History snapshots、session/world save schema、rkyv/ron 原 fork 调用、旧版 migration/BackwardCompatibility。
- 输入/输出：已有完整快照、事务链与版本 → 同一对象 ID/关系/状态恢复，再触发 derived 重建。
- 前置 G00/G14 的 schema，现有完整样本可立即调查；缺引用不是暂停所有快照研究的理由。
- 退出：已存在样本解码和 roundtrip、保存后重建、派生缓存重生与关联一致；未知/缺失对象和版本走原策略。语料 62 个缺引用（A18/B22/C22），B 同 ID 的 2 个候选未确认可替代，不伪造缺失目标。

### G16：原系统图、输入与异步任务（P1，接口取证穿插）

- 原入口：各 crate lib/plugin 注册、FunctionSystem/Event/Message types、wall/roof/decorator UI 模式、picking/gizmo、history recording/replay_mode/finish_replay。
- 输入/输出：工具开始/拖动/结束/取消、事件/dirty 与 tasks → 原消费者执行顺序、flush、任务结果/取消及录制边界。
- 复用 Bevy 调度/输入/任务 API；恢复游戏注册、filter、before/after、flush 和任务版本策略，现有 19 阶段 adapter chain 不等于原 graph。
- 退出：timed VisualEdit 与连续交互真实调度；快速连续编辑、撤销/删除期间的过期任务不复活对象。原 fork 未知范围仅按实际阻塞调查。

### G17：C 专属木构与风格迁移（P1，贯穿每包）

- 原入口：C construct_wooden_walls、window_cut_lists、stone_foundation_cutouts/surface、wooden_arch_frame、wooden platform/playermade stairs；color palettes/material_color_mapping/style_changes，roof resolved_appearance 与 pack_window_cut_list。
- 输入/输出：C 独立类型/材质/样式及资源 → C 木板/切口/石基础/木拱框/楼梯、颜色和屋顶输出；B 实现保留独立验证。
- 前置相应 G04/G05/G07/G09/G12/G13，静态契约现在开始；退出：C 用自己的 binary、enum、依赖与 Message API 验证对应链，最终 runtime 迁移报告完整。共享源 checksum 不替代执行，B Replay13 不套 C Replay9。

### G18：建筑实例到实际绘制与释放（后期必做）

- 原入口：Nani mesh/atlas、roof/wall/decorator buffers、sparse instance buffer 的游戏调用、draw/shader material 接口和资源所有权。
- 输入/输出：G04–G13/G17 的 CPU 记录与资源 → 实际 allocation/upload/update/remove、材质/mesh/灯光、可见画面和碰撞一致性。
- 复用 Vulkan/渲染依赖；私有 renderer 包先按归属与可用源码分类。shader/GPU 后置，不能把 deferred 请求或 dummy handle 宣称为后端执行。
- 退出：固定相机/种子/资源建筑画面对照，重复增删无残留 allocation，描边/孔洞/玻璃/瓦片与 CPU 数据一致。

### G19：完整建筑验收矩阵（分层持续、最终关闭）

- CPU 门槛：全部已观察 content family 的创建/编辑/取消/删除、跨对象变化、事务与保存恢复均进入真实 App，而非仅 probe。
- 版本门槛：B 每项恢复证明与 C 每项迁移证明分别可追溯，失败报告保留；A 仅对声称兼容的分支验收。
- 系统门槛：矩形/圆形/自由墙，单体/相接/重叠/高架，多洞/跨角/门窗/屋顶窗/楼梯，样式切换、同值、反复编辑和脏更新、撤销分叉、保存装载、缺资源及任务过期都有场景。
- 最终门槛：可运行建筑交互，原作 Windows x64 状态/画面对照、长运行与性能、GPU/资源清理。源码验收和最终产品验收分开，不要求先有 GPU 才能恢复下一段 CPU。

## 执行顺序与并行边界

第一批采用三个 worker 加父 agent，沿三条互不覆盖的源码/证据线开展：

| 批次 | 墙线 worker | 门窗/屋顶线 worker | 历史/调度线 worker | 父 agent 集成与退出 |
|---|---|---|---|---|
| 0：本轮规划 | 墙主体/结构/开口缺口审计 | 屋顶/门窗/资源缺口审计 | 历史/调度/保存缺口审计 | 汇总原模块、类型边与版本契约；只写计划 |
| 1：补最关键断点 | G04 construct_walls 实际消费；G02 契约/自由墙前置 | G09 collision 资源 → G10 原始四族放置链 | G14 扩 Roof/Wall owner，并取 G07 VisualEdit/G16 边界 | G00 管共享接口，集成“原始门窗输入改变真实墙砖，记录组撤销恢复” |
| 2：主体与附着闭环 | G01/G02/G03 完整墙编辑、分段与相交 | G07/G08 参数、平斜切换、尖顶/支撑 | G14 混合 owner、G15 保存快照 | 接宿主变化、跨墙传播及保存后重建，关闭永久 resolved 输入 |
| 3：建筑内容补齐 | G05/G06 拱梁支撑、门/栅栏/楼板 | G11/G12 其他装饰、dormer/楼梯 | G13 状态/历史、G16 timed 与任务取消 | 每个新 family 至少一条完整连续链；维护未观察分支 |
| 4：C 完整迁移 | G17 木墙/石基础/木拱框 | G17 木楼梯/材质/屋顶风格/门窗切口 | C history/Message/fork 契约与重验 | 基于前批穿插取证关闭全部 C 差异，独立 C acceptance |
| 5：最终建筑验收 | 结构/碰撞回归 | G18 资产/GPU/视觉 | 保存、输入、长运行/异常 | G19 状态、画面、性能/清理验收；达标才称完整建筑系统 |

父 agent 负责共享 ID/状态/事件/输出 schema、Bevy Plugin 与阶段、workspace/CLI、统一证据索引和集成测试；worker 负责分配的业务文件与专属证据目录，避免共同修改 `lib.rs`、Cargo.lock 和总台账。接口改变先反馈父 agent，禁止各写一套 Wall/Decorator/History 类型。

每批顺序为：三版归属/真实 caller → 最小类型契约 → 原业务算法 → 原指令差分 → 真实 World 输入/更新/删除 → 跨 owner/history/版本验证。依赖库内部不进入 worker 任务，也不进入完成分母。

## 本计划的验证与记录纪律

1. 精确字节/位、事件顺序/重复、ID 与 RNG 状态按原行为比较；容差须事先有依据。库桥接、已解析输入和跳过范围单列，不以 harness 注入预期输出证明整链。
2. CPU 差分执行原始 RNE/各版原 binary；patched EXE 只供静态分析。source/PDB/binary、输入、Rust source/manifest/lock、工具与平台身份绑定。
3. 连续 App 场景至少覆盖创建 → 无变化 → 属性/宿主变化 → commit/cancel → undo/redo/分叉 → 删除 → 清理帧；检查 actual geometry/instances/relationship，而非仅通知数量。
4. 不对旧源码全库格式化；输入或依赖集变化才重跑受影响证明。保留历史 source snapshots、未通过报告与明确范围，不仅改 hash 制造通过。
5. Token 控制：先索引/模块/调用点，后局部函数；完整输出落盘；独立查询批量，依赖操作顺序执行。agent 回报结论、证据、卡点与下一个 consumer，不互相倾倒大反编译。
6. 不估整系统百分比，不承诺未测工期。以包的输入/消费者/生命周期/事务/版本退出条件关闭；多 agent 的交叉审计是规划证据，不自动变为实现验收。

## 证据与审计产物

- [墙体审计](evidence/building-planning/walls-audit.md)
- [屋顶与装饰审计](evidence/building-planning/roofs-decorators-audit.md)
- [历史与调度审计](evidence/building-planning/history-schedule-audit.md)
- [机器可读攻关包](tracking/building-system-plan.json)
- [本轮输入快照](evidence/building-planning/planning-snapshot.json)
- 现有证据：`building-collection`、`building-gather`、`building-holes`、`building-edits`、`building-history`、`building-nani`、`building-runtime`；`tracking/version-map.json` 仍为候选映射，不能视为跨版行为等价表。

下一次实施的首个整链目标：**发行碰撞资源/原 Decorator 放置 → 门窗原始贡献 → 实际墙构造/砖列裁切 → 门窗移动/删除 → 建筑事务撤销/重做恢复**。三条 worker 线各自可先取证和差分，父 agent 在共享契约下集成，不能又停在多个独立 probe。

## 当前仍需取证的决策

- construct_walls 的真实 CPU/实例/碰撞输出 schema、材料和 seed 来源；主体与 shader 变形的职责边界。
- special_flag、未知 History aux owner、原取消/interrupt 对实时预览和 redo 的实际政策。
- 屋顶 special_mode、空 TriggerVolume 的原填充与引用周期、atlas/backend 回执的所有权。
- C 木构/材质/基础字段、派生切口与 Message 排程变化；同名或同 checksum 只建立候选。
- 原 fork/features、私有渲染源码可用性、最终 Windows x64 原作对照环境及未关闭 1ULP。前三类未知不阻止已确认墙面分支继续恢复；最终运行环境只阻碍对应最终验收。

这些是后续 agent 要用原定义/caller/consumer 解决的问题，当前不需要用户猜测字段或算法。
