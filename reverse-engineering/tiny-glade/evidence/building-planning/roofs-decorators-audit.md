# 屋顶、门窗、装饰与资产前段规划审计（只读，2026-10-11）

此审计没有修改生产 Rust、验证报告或现有进度台账。最终目标为 C(v1.16.0)，B 是当前主要行为证明基线。PDB 路径/校验和、direct publics 和已保存调用证据用于筛选候选；新增候选地址不是新增行为验证。

## 已完成与实际边界

| 业务段 | 已恢复的内容 | 尚未闭合的内容/优先度 |
|---|---|---|
| 屋顶形状与 CPU 瓦片 | B Circle/Rectangle 曲面、Curve2、脊线、边瓦/脊瓦/面瓦/补片、CPU记录；真实World Changed Roof触发生成 | 仅证实0/1两种有效宿主形状，不凭空列第三种屋顶形状；原assemble系统的special_mode选择、资源所有权和GPU消费未证明；ARM case14补片Y仍差1ULP(P1) |
| 屋顶权威状态/生命周期 | raw墙→发布→roof初始化/关联/同步→gable缓存→动画/删除；10创建组件及空required图 | 原fork调度图、RoofTiles真实backend、完整屋顶支持/尖顶实体生成、C创建/组件/布局逐链迁移(P1) |
| 屋顶参数编辑 | edit_rules已证明resolved参数编辑核心；方向、FlatStyle、Rim消费者实际接World | VisualEdit→实际edit_roof消费者、参数录制/历史owner/撤销链尚未接World；搜索runtime源码无edit_rules调用(P0/P1) |
| 平顶/斜顶切换 | ty/height阈值和当前CPU适配，Flat style数据赋值 | 原switch_roof_type实体/组件/资源切换；flat底板、屋底、rim实际几何/输出完整性(P1) |
| 屋顶触发面/交互与瓦片音效 | TriggerVolume仅确认空创建及结构schema，TileSimulation初始化和wall_height_dt已接 | calculate_roof_trigger_volumes生成三角面/法线/UV、raycast位置消费；simulate_roof_tile_for_audio实际消费者未接；它们影响原屋顶装饰定位/拾取(P0/P1，音效P2) |
| Spire/Support | RoofSpire、RoofSupportBeams已确认ZST和登记 | 标记并非生成。place_spires、roof visual/place_support_beams实际实例生产与删除；不得用墙lintel的同名函数代替(P1) |
| 四族门窗开口 | B cottage/gothic wall window、balcony door，resolved中心/size/collision→raw holes；实际事件/缓存/finalize/清理 | 资产key/rank/subtype选择、collision读取、dst/WallSpace、屋顶感知形变、视觉多实体Bundle(P0) |
| Decorator权威对象与附着 | OpeningAuthority为明确adapter，处理宿主变化/消失/恢复及贡献清理 | 原DecoratorStorage/Op/target/dst、移动/吸附/拆分/合并、锚点跟随、culling、archive/restore/replay和原删除策略(P0/P1) |
| 门窗视觉实体 | 已保存WindowVisualEntityBuilder::build反编译，未实现 | 独立玻璃、窗框/砖、门及其组件/descriptor/transform/颜色/发光数据；WallAttachedDecoTracker diff add/remove和实体所有权(P1) |
| 其他有证据开口族 | PDB/源路径证实corner window、trap door、arrow slit | 完整producer、collision/定位、墙砖消费、state/history/删除；不是四族producer自然覆盖(P1) |
| Dormer | B/C cottage/gothic dormer、profile、bounds、tile generator、DormerHoleSsbo有明确入口 | 屋顶切口/墙洞组合、full/half dormer形变、局部瓦片和清理；C pack_window_cut_list/stone_foundation_cutouts变化(P1) |
| 烟囱/旗/晾衣线/杂物 | game-owned源及generator候选明确存在 | roof/wall chimney+brick/smoke，flag连接/clothline，autoclutter、lantern和lights业务输出、history/清理(P2；非建筑主体闭环的替代) |
| 建筑材质/颜色/灯光 | B/C FlatStyle和Rim resolved赋值已证明 | Roof ColorID首循环、wall/decorator color、window light、material映射/风格变化、GPU color写入与对应历史(P1/P2) |
| Nani | B屋顶StoneFloor分支64byte CPU记录、descriptor选择、真实World Create/Update/Remove意图 | 尚无GPU active handle；Floor普通分支、门窗各subset payload、mesh atlas生成、C Roof分支新增wall/count/style/margin输入(P1后CPU，P3 GPU) |

## 三版本归属：哪些必须恢复，哪些直接复用

- country-core walls/roofs、system-roof、system-decorator、system-color及自有utils/wall_space是游戏业务候选，依据定义模块与实际消费者进一步确认。
- `resources/walls/decorator_collision.rs`、`roof/visual/place_spires.rs`为ABC同源；`utils/wall_space.rs`、roof支撑及多项decorator ownership模块为BC同源。相同不能作为依赖排除理由。
- glam/fastrand/half/std/HashMap/Vec/Arc/Bevy/Serde/itertools等继续复用。游戏规定的rank→lib key、asset选择、world projection、component字段/实体关系和调用顺序仍需恢复。
- WindowVisualEntityBuilder的反编译包含许多NaniBundle generic spawn和多个不同subset，且地址别名可合并。先逐分支确认实际descriptor和写入，不把全部generic实例、Serde派生或容器函数算作核心。
- 屋顶与墙lintel各有`place_support_beams.rs`；B direct publics只给出wall-constructor的函数，roof同名候选目前仅源路径。后续通过roof模块contribution/locals与真实caller查找，禁止错连。
- A锁文件缺失，不从B/C填造版本。C Bevy0.17.3/Message、组件/数据布局和材质行为与当前B Bevy0.16接线必须分别验证。

## 可执行攻关包

### RD1（最高收益）资产→原Decorator位置→现有开口闭环

目标：移除现有resolved collision和wall-center输入，保留已验证开口和storage核心。

- 原模块：country-core/render/mesh.rs、asset_libraries/mesh_library.rs、resources/walls/{decorator,decorator_subtype,decorator_collision,cached_decorator_transforms,public_wall_state}.rs；system-decorator/{derive_decorator_subtype,cache_decorator_transforms,ui_systems/calculate_decorator_dst}.rs；utils/wall_space.rs。
- 候选：B Mesh::collision_bbx `140a07a70`，C `140c29e20`；B DecoratorLibKey::new `140b25c90`；B derive subtype `140813d00`/`140812b20`，C `1408d8cb0`/`1408d9350`；B raycast hit→dst `14127c990`，C `1414d5ea0`；B cache transforms `1408153d0`；B wall_space `140b2a740`，C `140bf1480`；B WallSpaceCoord from/to world `140c91fb0`/`140c92090`，C `140de60f0`/`140de5e30`。
- 真实输入：发布墙/roof状态、Decorator target/dst/rank/variant及资产mesh/outline JSON；输出：DerivedDecoratorInfo、碰撞界、原wall-space coord和transform→四族raw holes→finalized cache。
- 先分析资源选择/包围盒写入，使用JSON库解码，复用std map；读取B/C全尺寸碰撞资产，不手算或注入期望hole。再执行原WallSpace投影与decorator消费者，并同时接迁移/宿主更新/删除。
- 验收：circle/rectangle/freehand、首尾周期、墙高/轮廓/地形/roof/rank改变、missing asset/host失败、同帧移动换宿主、资产重载、删除及恢复；实际World连续输出与原码对照。CLI不再要求用户提供collision_extent/vertical_bounds与最终wall_center。
- 前置：现有raw墙/roof发布已满足；资产key/schema、Decorator权威对象需要此包恢复。风险：公开index没有隐藏project_clamp/unbounded入口，须从保存的cottage producer caller定点补证；B/C source相同不能代替数值验证。

### RD2 原Decorator对象、附着diff与视觉多实体所有权

目标：将OpeningAuthority adapter升到真实Decorator业务状态，创建和删除整个门窗对象及其所有派生实体。

- 原模块：decorator_storage、apply_decorator_op、decorator_on_{move_shape,rectangle_edited,wall_height_changed,freehand_wall_edited}、move_decorators_following_anchors、cull_oob_decorators、deco_differential_state、utils/visual_entity_builder、maintain_visual_entities、restore_archived_decorators、replay_decorator。
- B候选：apply op `141276e30`；anchors `14081c790`；移动/矩形/高度/自由手/圆变化 `140811fc0`/`140819c00`/`14081b5e0`/`14081bac0`/`14081c060`；cull `14080d540`；builder `1412a9b30`/`1412ab270`，C `14149d890`/`14149a970`；B clear visuals/holes `1407f9260`/`1407f98c0`；B replay decorator `14080c8d0`，load `1407b66e0`，restore `140817a40`。
- 输入：原Decorator Op、Storage diff、宿主变化/删除、archive/replay；输出：权威dst/subtype→visual blueprint→多个真实Bevy实体/holes/灯光数据，关联和清理。
- 前置RD1；历史组要求history-agent恢复对应Decorator mutation owner。先只建原component/CPU payload和descriptor，backend成功后才允许ActiveInstance，不伪造GPU handle。
- 验收：创建/移动/换宿主/大小/风格改变→清旧贡献/实体→建新贡献；同值改动、host out-of-bounds、host删除与undo恢复、多个子实体和cache无残留；同一权威操作历史回放与普通操作结果一致。
- 风险：原删除/culling/archive策略尚未证明；当前adapter在host消失后保留authority，不能未经证据把它推广到原游戏。泛型builder不同instantiation必须分开确认。

### RD3 屋顶参数与类型转换、支撑/尖顶及完整CPU输出

目标：把已验证edit_rules连到真实事件/History/World，补type switch和已存在屋顶视觉分支。

- 原模块：system-roof/edit_roof、UI参数command生产、visual/{switch_roof_type,place_spires,place_support_beams,generate_roof_stone_floor_and_roof_bottom,assemble_roof_tiles}；country-core/resources/roofs/{commands,roof_shape,roof_visual_shape,roof_buffers}。
- 候选：B edit_roof `1421ab7c0` / C `142827a90`；B type switch `1421ac4f0` / C `142828820`；B spires `142196e70` / C `14281a950`；B assembly系统 `141a36700` / C `141f3d850`；B bottom/floor generator `1421a6d40` / C `142821520`；B trigger generator `1421a3220` / C `1428220a0`，B UV→位置 `1408e46d0` / C `140a04ed0`；B瓦片音效模拟 `1421a7090` / C `1428219b0`。roof支撑及roof_snow源路径存在而direct函数入口尚待locals/caller，不误用wall lintel `141242530`。
- 输入：原Roof parameter edits/VisualEdit、发布墙style/height/shape、颜色、动画/删除；输出：Roof权威参数、真实通知/历史、Circle/Rectangle的pitched/flat切换、spire/support/stone-floor/bottom CPU实例、真实屋顶trigger三角面/法线/UV与资源清理。屋顶装饰定位需要这些trigger输出，不能永远注入raycast成功结果。瓦片音效消费单独后置，不能因TileSimulation已登记就记完成。
- 前置：现有roof链；History Roof parameter owner与VisualEdit真实消费者配套。C投影恢复新增PublicWall字段及margin修正，不套B64bytewriter。
- 验收：height穿越0.1、circle↔rectangle宿主改变、ridge方向/长度、eave/tip/profile/style、marked/destroy、连续undo/redo；瓦/底板/支撑/尖顶/trigger旧资源全部移除后正确新建，最终World状态与原码一致。
- 风险：原special_mode来源和RoofType错误值未取证。已知ARM矩形case14仅一个补片Y相差1ULP，必须独立定位SSE/NEON或CRT边界，保留失败，不改成容差通过、不重写依赖。

### RD4 其他开口族、Dormer与屋顶切口

目标：恢复明确存在的未覆盖门窗族及局部屋顶/墙切口贡献，避免将四族holes称为完整开口系统。

- 原模块：decorator_visual/{cottage_corner_window,trap_doors,arrow_slit,gothic_dormer,cottage_dormers/*}、dormer_holes、country-core/walls/dormers、gothic_window_profile、C roof_buffers/stone_foundation_frames与system-decorator/stone_foundation_cutouts。
- B/C候选：corner `1407f66b0`/`1408cb150`；trap `1407be730`/`140879f40`；arrow `1407bbee0`/`140877690`；gothic dormer `1407c5650`/`140880600`；cottage dormer `1407c7ac0`/`140882d10`；bounds `1407ca6c0`/`1408826d0`；local roof tiles `1407cb1a0`/`140885900`及assembler `14128db20`/`1414e28c0`；dormer upload `1407b1d40`/`14086d850`；C window cut packing `140c71410`、foundation extension/upload `141506aa0`/`1408db060`。
- 输入：RD1/2原dst/subtype/asset、Roof profile/ridge/bounds、wall与foundation材质；输出：洞与变形profile、DormerHoleSsbo/packed window cuts、局部瓦片/可见性和清理。
- 前置：RD1/2定位与ownership、RD3参数传播；wall-agent必须提供真正砖裁切消费者，不能到SSBO intention就宣称切孔显示完成。
- 验收：full/half dormer与壁窗转换、roof边界内外、rank1/2/3、斜/平顶、墙高/材质变化、重叠多个opening、移除/undo恢复；对比几何/profile/payload后再做实际World与固定场景。
- 风险：source文件B/C差异广，C新增foundation/cut机制须按C原码证明；bounded A/B索引找不到C方法不能视为旧版绝对无功能。

### RD5 材质、颜色、灯光与附属装饰；C验收门槛

目标：补建筑外观权威状态/传播/历史，使door/window/roof输出体现用户编辑；附属装饰分批恢复。

- 原模块：system-color/{set_roof_customization,set_decorator_color_id,set_window_light,material_color_mapping,style_changes,write_roof_color_id_to_gpu,replay_color}；country-core color/appearance/material/atlas；system-decorator window_lights、lantern/flag/flag_connections/generate_clothline/chimney_wall/chimney_roof/chimney_assemble_bricks/decorator_clutter。
- B/C候选：roof customization `142299dd0`/`1428ac4f0`（只后两循环已证明，ColorID首循环未恢复）；decorator color `1422af0c0`/`1428ae820`；window light `142298260`/`1428ab8a0`；roof color write `142298520`/`1428abef0`；replay color `141c7f270`/`1421730a0`；C FlatRoofStyle appearance `140be7b40`，WallMaterial gable style `140c55770`。
- 附属分组依据明确PDB：B/C window_lights `1407b5980`/`1408714f0`，lantern `14078d320`/`140843b50`，roof chimney `1407c0910`/`14087bfb0`，wall chimney `1407c21b0`/`14087d590`，clutter `1407e3370`/`14088d0c0`，flag connection `1407df470`/`14088a500`，C generate_flags `1408cea00`。不把它们和屋顶主形状混在一个完成指标。
- 输入：颜色/材质/亮度/灯开关/flag连接/chimney smoke command及历史；输出：权威值、构造失效与真实mesh/subset payload、材质/灯光buffer，后续GPU写入另验。
- 前置：RD2真实Decorator组件、wall material schema、相应history owner。先Roof ColorID/window light→CPU消费者闭环，再烟囱/旗与自动杂物；不能由shader资产推断业务规则。
- 验收：同值与变化命令的ticks/通知/历史不同政策、墙/roof/装饰目标missing、材质影响Dormer/flat style、undo/redo与删除；C目录数据/Message事件/组件迁移、原码对照、完整建筑连续场景和最终GPU固定视角逐层验收。
- 风险：C增加WallMaterial和resolved appearance等目标版本规则；registry身份匹配不是fork/feature/材料语义证明。资源格式复用Serde/ron，SSBO/GPU writer到最终呈现必须单列。

## 来源与下一轮可直接使用的证据

以下路径均在 `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade` 内；JSON保留原SHA/地址，不以本文候选代替oracle：

- `evidence/building-planning/roof-decorator-business-function-candidates.json`：ABC已有聚焦index中的业务候选；index漏掉system-color/自有utils，因此不能依据其负结果排除。
- `evidence/building-planning/roof-decorator-direct-public-candidates.json`：本次ABC完整publics的有界direct符号，包括WallSpace和roof视觉系统。Serde派生保留作schema但实现排除。
- `evidence/building-planning/roof-decorator-color-candidates.json`：本次ABC完整publics恢复system-color直接入口。
- `evidence/building-planning/roof-decorator-library-and-function-schemas-B.json`：B schema包含EventReader/Query类型；std/core wrapper自身是依赖，仅参数/消费者类型为证据。
- `evidence/building-planning/roof-decorator-source-candidates.json`：ABC source路径/checksum比较，含roof/wall同名支撑、材质/颜色、cut与自有WallSpace。
- `evidence/building-holes/{findings,opening-findings,opening-three-version-map}.md/json`：原四族resolved界限及World实际链。
- `evidence/building-holes/B-opening-producers/{1407ebfd0,1412a9b30}.c`：定点补WallSpace调用、collision lib key/lookup、实际visual bundle branches入口。
- `evidence/building-nani/findings.md`、`evidence/building-runtime/B-roof-pipeline-host-verification.json`：B组件/CPU和C差异、保留1ULP失败。
- 实际B碰撞资源例：`/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/assets/meshes/decorators/window_cottage_1x1_collision.json` 与 `balcony_door_rank1_collision.json`。已直接检查，都是Vertex_Position float3+typed index buffer，8顶点/36索引；资产读取本身不证明原collision加工/选择政策。

建议父计划将RD1作为下一完整业务交付，RD3与历史owner恢复并行，RD2在RD1类型固定后接线，RD4/5依据前置逐批。每包必须原输入→原业务规则→实际World传播→消费者输出→编辑/重放→删除场景闭合；C迁移不能靠B来源相同代验。GPU后置但完整建筑最终验收必须包含GPU/资源与可见效果。
