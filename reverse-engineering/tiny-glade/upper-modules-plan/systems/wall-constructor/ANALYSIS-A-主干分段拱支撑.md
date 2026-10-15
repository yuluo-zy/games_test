# system_wall_constructor 子范围分析（B 版）：主干/分段/拱/过梁/立面支撑

> 由批次 3 分析 agent 产出，父级代为落盘。本文件覆盖模块 A 子范围；门/栅栏/地板/楼梯/半木/抹灰/UI/utils 见 `ANALYSIS-B-门栅栏楼梯半木utils.md`。

证据源：B patched EXE（sha256 cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323），Ghidra 共享只读 get_functions 反编译。地址为文件内 VA（0x140000000 基址）；RVA<0x10000000 → VA=0x140000000+RVA；RVA≥0x10000000 → VA=0x141000000+余量。

标注：O=可复用（已有恢复实现/证据）；I=已反编译分析、无完整实现；U=骨架/内联/空反编译待定位。依赖（glam/fastrand/perchance/smallvec/utils/lerp/itertools/hashbrown/parking_lot/frozen/puffin/bevy_ecs）直接复用。

## 1. 主干

### 1.1 WallConstructor::from_curve — O @0x140769760
逻辑：曲线+segment_values+FinalizedWallHoles → 规范化 holes → 行切分/裁剪/扰动 → 96B WallBrickInstance。内部 normalize_holes、select_native_row_holes（max-x 序后缀最小 x 提前退出）、split_plain_row_horizontal（0.46m 行随机切分，seed wrap 派生）、consume_wall_proposal（hole 相交→resolve_hole_overlap 破坏性裁、padding 0.46 调制 seed%10、span>0.46 随机行裂）、emit_resolved_wall_brick（CPU 实例）。算法：曲线等距行切 + AABB 四角分类裁切 + fastrand 行裂。常量：砖高 0.46、perturb_row_ends 1.99、split_horizontally t=0.4d+0.3。可复用 reconstruction/building/wall_constructor.rs + evidence/building-constructor/。勿重推导。

### 1.2 construct_walls — O @0x140757800
逻辑：Bevy system，EventReader<OnWallChanged>→PublicWalls/PublicWallState→from_curve→96B 砖→Roof 分支→InstancedWall new/update→WallHoleStorage finalize→half_timbering/add_wall_corners/add_skirts/stairs_signifiers_new→WallTriggerVolume recompute→发事件。未知：spawn bundle 字段、closure$1..7。可复用同 1.1。勿重推导。

### 1.3 ArchWalker::new — O @0x14074c2d0（24B）
写 16B 结构：[0]=in_R8D(u32)、[1]=in_XMM3_Da(f32)、[2]=in_XMM1_Da(f32 @8,=step；regular 传 3f0f5c29≈0.56，lintels 传 40499999≈3.15)、[3]=0x3dcccccd(f32 @12,=0.1 tail_ratio)。未知：字段 0/1 语义（walk 内不直接读）。

### 1.4 ArchWalker::walk<construct_arches::closure_env$3> — O @0x141232bb0
逻辑：total=Σ‖Δ‖；count=max(1,floor(total/step+.5))；step'=total/count；逐 windows(2)：step<residual+dist 时循环 traversed=(step-residual).max(0)、u 经 utils::wall_space subtract+project_clamp 周期投影→closure$3 发 96B；尾 step*0.1<residual 补砖。算法：沿曲线等距行走+WallSpace 周期 u+尾比 0.1。可复用 wall_arches.rs::walk_profile。勿重推导。

### 1.5 ArchWalker::walk<place_support_beams::...> — O @0x141233380
同 1.4 行走骨架，每次构造 64B WoodenPlankSsbo：端点 -0.225Y、方向=normalize(Δ 含 -0.225)、三轴 dir×len/side×0.3375/third×0、索引=(x*1000)as i32+(z*1000)as i32+base。可复用 wall_structures.rs::walk_lintel/emit_plank。勿重推导。

### 1.6 拱砖 96B 发射 closure$3 — O @0x141231af0
Affine3Packed::from_mat3_translation→push<BrickTransformSsbo>。seed=ordinal+index；material=BrickSourceId::from(WallId)；wall_distance=[us0*len,end_u*len](周期+1)；deformation0/surface[-100;3]/flags0（default 1408587c0）。可复用 emit_arch_brick。

### 1.7 init_wall_height_and_arch_segment_height — I @0x14078b550（4467B，UM-WALL-04 未开始）
ECS，遍历待初始化墙（GlobalId→0x2a8B 条目），写两处 {count:u32;height:f32}（条目 -0x198/-0x194 与 -400/-0x18c）+ 高度数组(-0x1c0/-0x1b8)：halftimber 分支=max(逐点y,-2.5)+height 钳0.96；普通分支再 -0.46 钳0.96 且数组逐点 max(y+height,0.5)；矩形+gable 屋顶：非半木 eave_length+inverse_lerp 混合，powf(1-(2|t-0.5|)/fVar40,_Y) roof 剖面叠加，末段 +=Roof::height+0.46。常量：-2.5/-0.46/0.96/0.5/0.28/0.95。未知：两对 {count,height} 权威语义（候选 WallHeightAuthority 顶/底）、-0x48 计数。

### 1.8 debug_force_update_arches — U @0x14003bb80
0x14003bb80 首字节 C3(ret)+0xCC 填充；Ghidra 建 1B 函数改名 finish<MainCameraPlugin>；find_functions(force_update_arches)=0。B 中该符号为 1B ret 存根。未知：真实语义/是否注册。

### 1.9 gpu_clear_wall_gate_pieces — I @0x140773c00
FrozenMap<SsboId,BufferState>（label 0x3d）→锁→遍历门片 HashMap（GlobalId→0x2a8B）→SparseInstanceBufferWriter::clear<WoodenPlankSsbo>。GPU 后置（rhapsody）。

### 1.10 gpu_flush_plank_buffers — I @0x140774010
保持 count→gpu_flush<WoodenPlankSsbo>→gpu_flush<SparkleSsbo>。GPU 后置。

## 2. 分段

### 2.1 calculate_path_wall_segmentation — O @0x14074f310
外入口：MaterialStats init_this_update+set_path_segmentation_count，Vec<Mask> 按 WallId 插 HashMap（标准插入复用）。可复用 wall_segmentation.rs + evidence/building-segmentation/findings.md。

### 2.2 calc_path_wall_segmentation — O @0x14074f7a0
纯 calc：prepared 点+path float+shape+special；path 加到 prepared Y，正 run 两侧带 1 点，closed 首尾合并负 start LoopRange，逐 run process_segment。可复用 wall_segmentation.rs::wall_path_segmentation。

### 2.3 process_segment — O @0x140750420
flags[0]=2/末=4；候选内部点 score_split_candidate>0.4 取最高|1；未标记跨度 add_splits 锚点；首尾坐标驱动长度。可复用 wall_segmentation.rs::process_segment。

### 2.4 add_splits_in_empty_range — O @0x140751240
XZ Curve2，count=floor(len/span)，count==1 且 len<1.5span 则 0；i in 1..=count coord_at_u(i/(count+1))，fraction>0.5 选下槽，flags bit8。可复用 add_splits。

### 2.5 score_split_candidate — O @0x140751610
左右 2 单位弦（反向迭代器自 index+1），flags&0xf 停；投影钳制；(dist_XZ+chordY-currY)*0.5，后 -0.4/clamp0，最高胜出。可复用 score。

### 2.6 calculate_wall_path_segments_impl — O @0x14122ff90
136B mask→104B WallPathSegment（positions/heights/points_u/range/source_coords/flags）。可复用 wall_structures.rs::materialize_wall_segments。

### 2.7 sample_wall_path_intersections — O @0x14074c720
四 512² 栅格（u8→u16*257）+relative heights+Public5c→衰减采样/平滑(±1±2 权重 3f1b4598/3e0a9555)/阈值 crossing/去重/重建 Curve3。可复用 wall_path_preparation.rs::prepare_wall_path。

### 2.8 split_wall_path_segment_into_platform_stairs — I @0x140751960（4216B）
ECS，OnWallChanged→sort+dedup；墙段计数(local_80+0x4b)<2 且 roof 存在且 is_flat：逐 0x88B WallPathSegment 收集高度(closure5)→zip(closure6)→utils::threshold_range(closure7/8) 切平台台阶段→追加 0x88B 输出+0x20B 对→写 HashMap<WallId,Vec<Mask>>+DecoratorId→Vec<PreferredNodeAlignment>；≥2 段则 clone 插入。未知：threshold_range 阈值/闭开语义、与 recovered split_platform_wall_masks（mask 级）关系。

## 3. 拱/过梁

### 3.1 construct_arches — O @0x14075f680
墙事件 consumer；Public+296 非零跳过；shape0/1→construct_regular_wall_arches（regular_profile+arch cap）；shape2→construct_freehand_wall_arches（trim/third 重平滑）；has_roof&&单段→profile 变异 104B 段入 120B ConstructGateCmd；third/arch 累积+closed 首尾复制。可复用 wall_arches.rs + wall_arches_freehand.rs + evidence/building-arches|arches-freehand。勿重推导。

### 3.2 create_stone_arch_profile — O @0x1407645f0
regular：clamp(h/Pub284)*.5 再 ×.5 加回；cubic 中心加权（立方先乘）；lower=max(highest-3.5,ground)-1-groundY 钳0；min 全局拱高。freehand：边界 trim .1875/length cap.95（双端 cap.475）+slice 重采样保点数。可复用 regular_profile/profile+slice。

### 3.3 ArchFunction::remap_t — I @0x140765300（248B）
if(*in_RCX!=0){ inverse_lerp(); }。ArchFunction 为 u8 判别枚举，0 直通/非 0 inverse_lerp 重映射 t。未知：枚举完整取值。

### 3.4 construct_lintels — O/I @0x14076d9f0
顶部过梁生产者：prepare_lintel_support（highest+groundY、base=(284+0.5).max(288)、heights=((gY-py)-0.2).max(0)+base、raised=public+height*Y、flags[0/末]=8、source_coords）→lintel_support_instances（Grower propose/consume+join+对角+rdp walk）→64B 木板→段高回写→accumulate_lintel_third_heights（third=heights-0.2 maxnum）；喂 PillarPlacementProposal+MaterialStats。未知：外 ECS shell。可复用 wall_structures.rs::construct_lintel_supports 等。勿重推导。

### 3.5 place_support_beams<closure_env$2> — O @0x141242530
双 Grower 0/end 向中 propose，cost 小者 consume；len≥1.0 发射/≤3.15/join；分开时各 len*0.85 对角，join 且 total≥1.4175 双侧 0.5。可复用 lintel_support_instances。

### 3.6 add_diagonal_support — O @0x141243740
extend_xz 延展到 (index,fraction)；锚高 sample_height-0.2；方向=采样点差(Δy-0.2)；轴 side×0.2/dir×(len-0.2)/third×0.2；push WoodenPlankSsbo。可复用 add_diagonal。

### 3.7 SupportBeamBuilder::emit_support_beams — O @0x141243c20
rdp_support_indices→索引对→walk_lintel。可复用 emit_range。

### 3.8 SupportBeamGrower::new — U（内联，可复用 Grower::init）

### 3.9 SupportBeamGrower::propose — O @0x141fe09c0
flags[next]&0xf≠0→stopped；方向=点差归一化；已有方向投影；cost=len*(‖Δdir‖+0.5)（len≥1）。可复用 wall_structures.rs::propose。

## 4. 立面支撑 / place_pillars

### 4.1 detect_overhang_support_segments — I @0x1407867d0
ECS：OnWallChanged→HashSet；tabloid Subscriber<DirtyTiles>；脏→OverhangSupportFinderCache::new→遍历墙表(0x2a8B)：fVar4@-0x20==0→clear+ElevationSupports insert+closure$2(140787d80 清两 WoodenPlankSsbo)+bucket；否则过滤集且 Circle/Rect aabb 与 DirtyTiles 相交→closure$2+find_overhang_support_segments+bucket。未知：fVar4 判据、ElevationSupports 字段。

### 4.2 overhang_subscribe_to_tabloids — I @0x140786440
8×RaycastWorld::subscribe_to_changes()+1×tabloid<DirtyTiles>，句柄 push。纯订阅胶水。

### 4.3 construct_circle_supports — I @0x140788160
遍历 0x28B 队列→0x2a8B 墙记录→is_circle；clear_and_get_mut→circle_categorize：true→bottom_layer+indexmap bucket+circle_pillars；false→turret；写 WallId→ElevationSupports。

### 4.4 construct_rectangle_columns — I @0x140788a80
同骨架 is_rectangle→clear+rect_bottom_layer+rectangle_pillars+ElevationSupports。

### 4.5 construct_rectangle_brackets — I @0x1407896b0
矩形→small_brackets；墙高(0x2a8-0x1c)≥1.6 再 large_brackets。

### 4.6 circle_categorize — I @0x14124af60
fVar8=*(rcx+8)≥1.75→ceil(2fVar8) 网格双层扫；Circle2d::is_point_inside；内部逐 0x50B 墙 get_wall_shape+GladeBorder::is_pos_inside。未知：输出副作用、≥1.75/ceil(2r) 语义。

### 4.7 construct_circle_bottom_layer — I @0x14124b2c0
halftimber 早退；count=ceil(perimeter/0.69)；random_splits_into；逐对 get_pos_at_u→add_bottom_brick。常量 0.69、+0.28。

### 4.8 add_bottom_brick — I @0x141fe1de0
fastrand f32 抖动+push<BrickTransformSsbo>+dirty。

### 4.9 construct_circle_pillars — I @0x14124ba30
0x298==0 守卫；count=round(inverse_lerp(perimeter)→t*7+(1-t)*4)；逐 u get_pos_at_u+terrain sample，terrain+0.05≤墙高(0x280)；raycast；smoothstep 剖面((h-y0-5)/5)钳(3-2t)t²；柱位法线外移 -0.4；0x40B 柱记录；rng 后 halftimber→wooden_pillar_w_stone_base 否则 brick_columns。常量：4..7、+0.05、-0.4、2.5。

### 4.10 construct_turret — I @0x14124cc30
Circle2d/Aabb2/Curve/CurveU/random_splits_into/Nani instance_data/StairsAssemblyParams→圆顶塔楼砖。未知：层数/收分率。

### 4.11 construct_rect_bottom_layer — I @0x14124eb80
halftimber→四角 WoodenPlankSsbo；否则 5 段边：count=ceil(len/0.69)、jitter=0.32/len、random_splits_into→get_point_u×2+lerp→add_bottom_brick。

### 4.12 construct_rectangle_large_brackets — I @0x141253fc0
Rectangle2d 角+RayBoxIntersection::is_hit+TerrainHeightsData+valid_enclosure+Curve→角部大托架(WoodenPlankSsbo)。未知：托架几何参数。

### 4.13 construct_rectangle_pillars — I @0x14124fcb0
4 矩形角：terrain+0.05≤墙高→glade/wall 检查→3×legacy_raycast_w_params 最小距离；minDist≥2.3→find_stone_column_start_y→smoothstep 剖面(3-2t)t²；0x40B 柱记录；rng 后 halftimber 二路。常量 2.3、+0.05。

### 4.14 find_stone_column_start_y — I/U @0x141250720
raycast_simple_exclude_wall_id+TerrainHeightsData+Rectangle2d。Ghidra jumptable 坏数据截断（case 0x12..0x16）。须 reva 定点。

### 4.15 construct_rectangle_small_brackets — I @0x141250da0
同 4.12 小托架（fastrand Rng::f32、stairs::visual）。

### 4.16 find_overhang_support_segments — I @0x141248620
几何核：Rectangle2d/Circle2d+Curve/CurveU/CurveUCircular+country_core::utils_intersect::line_x_shape_overlap+glade/墙检查+多 closure。未知：输出结构/阈值。

### 4.17 OverhangSupportFinderCache::new — I @0x141248300
遍历墙表：合法 gable 屋顶→push(wall_id, 屋顶数据指针)；段计数(-0x48)<2→push(wall_id, 条目-0x2a0)。两 Vec(16B 元素)。未知：消费方。

### 4.18 maintain_stone_column_colliders — I @0x14078a6e0
RaycastWorld(6 调用)+StoneColumnCollisionStorage+Triangle 相交→维护石柱碰撞体。rapier 后端依赖 core 未闭合。

### 4.19 construct_wooden_pillar_w_stone_base — I @0x141fe2150
锁 SSBO(0x3d)；逐 16B 柱源记录：高度钳1.1、半高轴→WoodenPlankSsbo 立杆；len>1.25 石基（inverse_lerp 2 段、0.3 缩木板斜撑）；再逐 0x18B 记录 construct_crossbrick_pillar。常量 1.1/1.25/0.5/0.3/0.2。

### 4.20 construct_brick_columns — I @0x141fe3240
FrozenMap(0x3a)→锁→writer 末项→0x40B 柱表逐 construct_crossbrick_pillar。

### 4.21 construct_crossbrick_pillar — I @0x142418f50
count=ceil((top-bottom)/(size*0.4))；random_splits→映射 split*top+(1-split)*bottom；逐切点 Rectangle2d 四边+fastrand 抖动+xz_rotation_bad→push<BrickTransformSsbo>。常量 0.4 层高。UM-WALL-25 内联宿主已定位=本函数+4.20。

### 4.22 place_pillars — I @0x140753670
ECS：OnWallChanged→sorted+dedup；每墙查 0x2a8B→遍历 0x88B 柱提案记录（@0x38 提案数组 0x1cB、@0x68 u64 索引、@0x50 u32、@0x20 (i32,f32)、@8 标志、@0x78 LoopRange 类）：LoopRange start/end+CurveSlice extend_left/right→rem_euclid 采样点内插→提案(u8,u32)；lintel 标志(local_100-10==1)→WoodenPlankSsbo 立柱，否则 assemble_stone_pillar；MaterialStats set_arch/lintel_pillars_count+set_pillar_placements。未知：0x88B 字段权威语义。

### 4.23 PillarProposals::add_proposal — I @0x1407534d0
PillarProposals{Vec<PillarPlacementProposal 0x1cB>,Vec<f32> scores,Vec<usize> keys}。新 key 三向量 push；同 key score NaN-aware min，新 score 更小则替换末提案=每墙最优。

### 4.24 assemble_stone_pillar — I @0x14120d670
bottom+0.2≤1.26 单段否则双段且 1.2<|Δ| 标记；逐段 span=(top-bottom)-0.25；count=ceil(span/0.56)；random_splits；逐对 -0.2 偏移+fastrand jitter（0.42/0.7 或顶段固定 0.7/0.84）；bottom+0.2>2.1 盖砖；closure$4 发 96B。常量 1.26/0.56/0.25/0.42/0.7/2.1。

### 4.25 place_pillars::closure$4 — I @0x14120d340
Affine3Packed+push<BrickTransformSsbo>+计数++dirty。

## 5. 本子范围逻辑总览
主干链：construct_walls(140757800)→from_curve(140769760)→96B 砖；init_wall_height(14078b550) 未恢复；GPU 辅助后置；debug_force_update_arches(14003bb80)=1B 存根；ArchWalker new(14074c2d0) 定 step@8/tail@12，walk<arches>(141232bb0)/walk<support>(141233380) 等距行走发 96B 拱砖/64B 木板。分段链：calculate(14074f310)→calc(14074f7a0)→process(140750420)→score(140751610)+add_splits(140751240)→136B Mask；materializer(14122ff90)→104B 段；sample(14074c720)；split_into_platform_stairs(140751960) threshold_range 台阶切分（I）；calc 族已闭合(O)。拱/过梁：construct_arches(14075f680)+create_stone_arch_profile(1407645f0)+remap_t(140765300)+120B GateCmd(O)；construct_lintels(14076d9f0)→place_support_beams(141242530)/propose(141fe09c0)/add_diagonal(141243740)/emit_support_beams(141243c20)→64B 木板+third(O，外 shell I)。立面支撑：detect_overhang(1407867d0)+subscribe(140786440)+Cache::new(141248300)→find_overhang(141248620)；circle_supports(140788160)→categorize(14124af60) 分流 turret(14124cc30)/bottom(14124b2c0)+pillars(14124ba30)；rectangle_columns(140788a80)→rect_bottom(14124eb80)+rect_pillars(14124fcb0)；brackets(1407896b0)→small(141250da0)+large(141253fc0)；柱砖 util wooden_pillar(141fe2150)/brick_columns(141fe3240)/crossbrick(142418f50)；place_pillars(140753670)+add_proposal(1407534d0)+assemble_stone_pillar(14120d670)+closure$4(14120d340)；maintain_stone_column_colliders(14078a6e0)。整族 I，未恢复实现。

## 6. 算法清单
沿曲线等距行走（count=max(1,floor(total/step+.5))+尾比0.1）；WallSpace 周期 u（subtract/project_clamp）；分段评分（score_split_candidate 弦投影+符号谷值×.5-.4、add_splits floor(len/span) 锚点）；threshold_range 高度阈值切平台台阶；行切分/裁剪（0.46 行、resolve_hole_overlap AABB 四角分类、split_into_random_rows 0.46/0.046、perturb_row_ends×1.99、split_horizontally 0.4t+0.3）；支撑梁双向生长（Grower 代价=len*(‖Δdir‖+.5)、join、extend_xz 对角）；柱/托架布局（ceil(2r) 格、inverse_lerp 4..7 柱、4 角+3 raycast 净空≥2.3+smoothstep(3-2t)t²、ceil(len/0.69)+random_splits_into、crossbrick 0.4 层高四边随机砖、assemble 1.26 分段+0.56 层+随机砖宽）；悬挑支撑（line_x_shape_overlap+曲线 u 采样+DirtyTiles AABB 过滤）；RNG 抖动（fastrand f32）。

## 7. 未知清单
1. init_wall_height(14078b550) 两对 {count,height} 权威语义、-0x48 计数、与 core wall_height_authority 对齐。2. debug_force_update_arches(14003bb80) 1B 存根语义/调用点。3. gpu_clear_wall_gate_pieces/gpu_flush_plank_buffers 门片 HashMap 写入者、GPU 后置。4. split_wall_path_segment_into_platform_stairs threshold_range 阈值/闭开、closure7/8、与 split_platform_wall_masks 关系。5. ArchFunction::remap_t 枚举取值。6. find_stone_column_start_y(141250720) jumptable 截断须 reva 定点。7. 立面支撑整族多函数细节（raycast 参数、0x40B/0x88B 记录字段、categorize 输出语义、托架几何、ElevationSupports 字段）待定点；maintain_stone_column_colliders 依赖 core rapier 后端未闭合。8. ArchWalker 字段 0/1；walk 另两单态（141231ea0/14048ab50）属子范围 B。9. circle_categorize/construct_circle_pillars 的 0x298 开关与 0x280 墙高字段权威来源。

## 8. 依赖与工具
依赖直接复用。反编译 job：tool-usage/jobs/wall-a/batch1|1b|2|3|4|5.json；结果 /tmp/wall-a-batch*.json；逐函数 /tmp/wall-a-dec/*.c。共享只读 worker，未写库。
