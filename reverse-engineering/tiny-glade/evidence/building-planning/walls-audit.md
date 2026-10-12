# 墙主体系统缺口审计（只读规划，2026-10-11）

## 判定口径

基于 native-rust-reconstruction、AGENTS/DEPENDENCY_FILTERING、当前 Rust 与保存证据；不实现代码，不修改现有报告/ledger。当前运行基线 B，最终目标 C；A 用于追溯。符号/原路径是候选归属，不是作者源码或行为恢复。ABC共享的 arch_walker.rs、B/C共享的 valid_enclosure/mod.rs 仍是游戏业务，不能因相同排除。std/Vec/Arc/HashMap、Bevy事件/查询/调度、glam/half/fastrand以及公共碰撞库内部直接复用，仅逆向游戏调用策略。

## 已完成的连续链及边界

- 已有真实 raw Curve2 → 原重采样/地形采样 → PublicWall发布 → circle/rectangle基础缓存及B逐arch山墙 → Roof/Nani CPU链，移动/删除维护64×64空间格的 wall-ID membership。
- RecalculateWallHoles reader转OnWallChanged → 四类门窗resolved位置/asset碰撞metadata贡献raw洞 → finalize跨帧、周期复制、排序/指纹缓存 → 门窗/宿主删除清理已接World。
- Rim显式账本即时事务提交/undo/redo/分叉已接门窗重算。不是所有wall/decorator/roof历史owner闭环。
- `regular_wall_height_cache` 对shape.tag>=2明确返回 UnsupportedShape，不能说freeform已恢复。raw采集能接任意Curve不等于freeform完整高度/主体输出。
- 当前存储的AABB仅wall-space开口；尚无construct_walls/WallConstructor的砖体输出及实际三维裁切、碰撞mesh。
- `RemoveWall`为重建事件，清PublicWalls/raw/index/opening storage；不是原delete_wall→archivist→所有附属对象/GPU资源/历史的完整级联。
- RoofSpire/Support已注册ZST marker，不能当作实际柱/托架/梁生成。

## 模块缺口表

| 子系统/原模块 | 当前证据等级 | 必须恢复的内容 | 紧邻输入/输出与优先级 |
|---|---|---|---|
| country-core resources/walls/inner_wall_state、InnerWalls，systems/wall/shapes、draw/finish/merge/split/delete | raw-publication投影和部分高度数值已证实；原命令/作者对象schema未闭环 | 完整WallState、StructureType/材料/shape/relativeheight、circle/rectangle创建尺寸旋转及freehand编辑/闭合/分割/合并，高度flag实际producer，ID/删除所有权 | 原命令或已恢复历史 → authority →发布；P0/P1，优先开口消费者可先沿既有B输入推进 |
| init_wall_height_and_arch_segment_height、calc_shape_height | regular/gable部分已验收，freeform未实现 | freehand follow-terrain/constant height、每段height曲线、基础高度；C石基础offset/transition语义 | wall raw+terrain →全形状height/cache；P0，与墙几何并行 |
| wall_state/spatial_hash_update、wall_spatial_hash | 64×64 ID membership已证实；segment metadata/query未恢复 | 段U/terrain Y/normal metadata与circle/segment/rectangle查询，墙相交/重叠依赖、变更/移除双向传播 | raw/published墙→空间段→相交关系；P0，为分段/支撑输入 |
| segmentation/{sample_wall_path_intersections,wall_path_segment,calculate_wall_path_segmentation,split_wall_path_segment_into_platform_stairs}、arch_walker | ABC源/符号候选，未行为实现 | 道路/其他墙相交采样、分段/评分、台阶区段与arch walker调用者完整策略 | 曲线+空间metadata+世界邻接 → WallPathSegments/arch proposals；P1，不能把通用库相交函数算成果 |
| construct_walls、wall_constructor、utils/{trim_rows,split_into_random_rows,split_horizontally,resolve_hole_overlap,perturb_row_ends} | construct_walls是已证实finalizer caller；主体尚未恢复 | Hole padding/类别/周期副本消费、随机行/砖布局、洞重叠处理、逐砖裁切、墙变形、最终记录/mesh及collider更新/销毁 | 现有发布cache+FinalizedWallHoles →实际砖体/mesh/collision；P0最高消费者收益 |
| construct_arches、construct_lintels/place_support_beams、place_pillars | 有原入口/模块，未实现 | arch profile/跨度/帧、过梁梁提案、柱放置/stone pillar、controls命令→变化传播 | segment+opening+height→arch/lintel/pillar业务geometry；P1，必须验证删改重建 |
| construct_elevation_supports全链 | construct_brick_columns(848B)、wooden_pillar_w_stone_base(4327B)确证边界但无恢复报告 | overhang overlap检测、circle分类/底砖/柱/塔、rectangle底砖/柱/大小托架、真实colliders、tabloid输入与清理 | publishedwall+terrain+其他建筑 →支撑实例+碰撞；P1，不以两leaf证明全部支撑 |
| plaster_systems、curved_halftimber、utils/{half_timber,wall_skirts,wall_corners}；独立systems/timberframe四源 | 原模块候选，未实现 | plaster sides/mesh/entities、窗下/重叠窗木构图案、屋顶轮廓、角梁/墙裙/弯墙木构、circle/rectangle timberframe和beam-count音频 | wall/material+holes+roof →表面/梁实例及清理；P1/P2，原Source存在不等于规则已确认 |
| construct_gates、construct_fences、construct_floors | 原完整模块候选，未实现；Roof StoneFloor CPU不是这些模块 | gate索引/门扇与洞、角度拆分/栅栏布局、结构楼面；创建/编辑/删除/碰撞 | StructureType/segment/terrain →gate/fence/floor实例；P1，完整建筑范围必做 |
| construct_platform_stairs、construct_playermade_stairs；decorator door_stairs | 已定位原输入/CPU入口但未恢复 | 台阶分段/砖/拱墙/悬浮砖/柱/托架及collision，stairs hole贡献到墙 | 地形+平台/门/曲线→stairs geometry/collision+hole；P1，由装饰agent细化 |
| C construct_wooden_walls、stone_foundation*、wooden_arch_frame、wooden stairs/support | 仅C原source/符号候选，目前Rust无对应模块 | 木板wall-space区域/水平竖直/圆墙/gable规划、window cut lists、arch openings/toprim remap、石基础表面/cutouts/window frame、木拱角件、木楼梯转角/斜坡collision/托架 | C真实WallMaterial/foundation/holes→C建筑主体；P1，不能等B工程全部做完才发现目标范围变化 |
| collision world/intersect_meshes & 游戏注册/调度 | 公共库复用，游戏调用/geometry未闭环 | 实际墙可拾取/碰撞对象注册与replacement、异步取消/失效、消费者顺序、fork必要差异 | CPU输出→游戏collision/render request；P1，GPU后置不等于可跳过CPU schema |

## 五个可执行攻关包

### W1（P0）：开口数据落到实际墙砖消费者

注意：construct_brick_columns 定义在 construct_elevation_supports/util_pillar_construction，不应仅凭名字认定它是主墙brick loop；先从construct_walls实际CALL确认。该leaf用于W3支撑锚点，与W1主体输出的关系仍待取证。

入口：B construct_walls 0x140757800；原CALL finalize 0x140757e2e/0x140758d1a；WallConstructor::from_curve B0x140769760；C construct_walls0x140803290、normalize_holes0x1408105d0、split_brick_by_holes0x140810670。C from_curve存在多个共享/特化地址，必须查实际CALL选择，不直接用首地址。

输入：已恢复PublicWall/height/arch、FinalizedWallHoles与实际材料/RNG seed参数。输出：原墙砖CPU记录/geometry、wall-space→3D映射、洞裁切结果及collider/render intentions，接入同一Bevy链。

步骤：从实际finalizer AL后的分支向前/后取证，找到layout/normalization→row/brick loop→hole split→output writer；字段定名和buffer ownership确认后恢复游戏规则。先连续B链再比较C不同的normalization/split，A追溯旧规则。std/RNG/glam/mesh库内核复用。

验收：原函数/实际消费者差分覆盖零洞、跨端洞、多洞重叠、洞完全包砖/边缘擦碰/跨行、重复洞、圆/矩形、删门窗复原、改变height/rim/roof、wall删除；compare meaningful原记录/数值/ID/随机末状态。运行连续World，实际砖/碰撞随洞变化，缓存同值不重建；不是只看到OnWallChanged或AABB变化。

风险：B/C的hole consumers可能行为不同；padding/unknown字段不补造；output shader变形与CPUmesh职责待确认；本包不能冒称所有墙材质/形状完成。

### W2（P0，可并行）：真实墙权威编辑、freeform高度与删除闭环

入口：create_circle B0x14097e560/C0x140a3e110；create_rectangle B0x140a93790/C0x140a3f790；adjust_wall_height_impl B0x140ac1f20/C0x140be6d90；calc_shape_height B0x140ac80e0/C calc_shape_height_and_stone_foundation0x140bec1e0；InnerWalls::update_wall_state_height B0x14085ad60/C0x140c03dd0；delete_wall B0x140aac1e0/C0x140af85a0。freehand edit B0x140ac98e0/C0x140b85f30。

输入：原create/edit/split/merge/delete业务命令与terrain；输出：authoritative WallState/Curve2/shape/height/material→现有采集/发布→所有下游失效；Wall历史扩展接口交给历史agent。

验收：circle/rectangle/freeform创建和缩放/旋转/height/move→W1主体、Roof、门窗连续更新；freeform terrain-follow/constant与不同端点/闭合模式；delete实证级联；undo恢复同ID关系及geometry。不允许继续手工给完整PublicWall projection或arch heights。

风险：special flag不是已命名语义；C WallMaterial/foundation新增不能套B enum；UI手势和最终authoritative命令可分批但必须登记未完成部分。

### W3（P1，先补metadata）：相交分段、arches/lintels与elevation supports

入口：spatial_hash_update B0x140a8fd90/C0x140b95a50；sample_wall_path_intersections B0x14074c720/C0x1407f7df0；calculate_path_wall_segmentation B0x14074f310/C0x1407fca90；detect_overhang_support_segments B0x1407867d0/C0x140837c20；construct_arches B0x14075f680/C0x1407eab60；construct_lintels B0x14076d9f0/C0x1408108e0；construct_brick_columns A0x141774c80/B0x141fe3240/C0x1424ec5e0。

输入：W2真实墙+terrain/相邻墙+opening；输出：实际segments/proposals→arches/lintels/pillars/brackets/bottomlayers及colliders。由spatial metadata契约起步，后可拆独立圆墙/矩形支撑agent。

验收：原segments+排序/评分/proposals+geometry meaningful bytes；两墙相交/重叠/移动离开/删除、悬挑上下方支持、有/无地形支撑、窗门跨度改变；删除无旧colliders/instances。柱leaf只作为native数值锚点，完成包必须到全链输出。

风险：库锁/分配/HashSet展开与游戏metadata区分；ArchWalker多特化别名需实际caller；C support材料/木结构扩展独立验证。

### W4（P1/P2，主体后）：完整结构变体和饰面

入口：construct_gates B0x140756370/C0x140801000、update_gates_hashmap B0x140755690/C0x1408022b0；construct_fences B0x140765830/C0x14080d800；construct_floors B0x1407661e0/C0x14080e4c0；plaster generate B0x1412269b0/C0x141463410；curved_halftimber B0x1407706d0/C0x1408136f0；timberframe四source为归属候选，现index尚无目标函数映射应先查完整PDB，不猜地址。

输入：W1-W3墙/洞/分段+StructureType/material；输出：gate/fence/floor实体，plaster mesh，halftimber/timberframe梁及墙裙/角/窗框，collider/render requests。

验收：实际结构变体创建→编辑→重建→material/style转换→删除→历史恢复；原geometry/proposal差分。Roof StoneFloor记录不能替代floor consumer验收。楼梯接口与另agent交叉核对，所有scope明确登记。

风险：原视觉资源metadata需资产agent交付；timberframe可能独立plugin注册，不能只实现construct_walls调用到的分支；GPU职责后置但CPU材料/atlas IDs不能补造。

### W5（C目标必做，契约提前冻结）：木墙/石基础/木拱/木楼梯扩展

入口：C wooden wall boards0x141456b30、WindowCutLists0x1424e34e0、maintain_wooden_freeform_wall_boards0x140816af0；maintain_stone_foundation_surfaces0x14083acb0、stone_foundation_decorator_frames0x141460930；wooden_arch_frame系统0x140819120；construct_wooden_platform_stairs0x14141a990；construct_playermade_wooden_stair_supports0x140822d60。

输入：C WallMaterial/stone-foundation offset+surface+cutouts、C权威shape/height+holes+stairs；输出：木板分片/窗切口/石基mesh、木拱框角件、木阶角件/底立板/支撑/collision。

步骤：先从C实际callers/descriptor/存档schema确认新原模块行为与材料字段，原sources显示C新增只是候选。做C专属schema而非把所有B数据改名。由W1/W2共享抽象接入，实际差异留version adapter。

验收：C原指令输出/输入guard与完整连续场景：circle/rectangle/freeform wood、horizontal/vertical boards、gable/rim、windowcut/foundation高度与跨墙terrain变化、wood/stone转换、删除及历史恢复。B只提供旧版本对照，不能代C验收。

风险：target C比B范围大；C所有from_curve别名不能按同名复用；新增子模块渲染输出/mesh归属和scope必须原caller确认。

## 建议实施次序

父agent统一原模块→重建模块/API/evidence映射；第一批W1实际consumer、W2 authority/freeform、资产/Decorator前段并行，先打通一面带门窗墙的actual CPU实体/geometry，而不是重复扩展开口通知。W3 metadata/相交随后并行圆/矩形支撑；W4主体饰面/变体完整覆盖；W5 C新增schema应前置审计而实现可按依赖滚动。每包附原命令/实际消费者差分及真实World增删改/undo/reload。全建筑验收仍需父agent补Roof Flat/Floor/Spire、History各owner/snapshot/timed、实际调度及之后GPU/visual/拾取，不能此报告单独判完整。

## 证据绝对路径

- /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/tracking/version-map.json（ABC原symbols/私有sources/checksums；候选非行为等价）
- /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/tracking/dependency-exclusions.json
- /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/building-holes/B-finalizer-callers/references.json（实际construct_walls→finalize传参/调用/AL）
- /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/building-holes/findings.md
- /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/building-collection/findings.md（仅ID membership范围）
- /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/building-gather/findings.md
- /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/pdb-structure/procedures.json（construct_brick_columns/wooden_pillar exact范围）
- /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/scope-priorities/findings.md（height suffix scope）
- /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/building/wall_cache.rs（shape>=2 UnsupportedShape）
- /Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/crates/tg-building-runtime/src/lib.rs（当前RemoveWall/regular缓存适配）
