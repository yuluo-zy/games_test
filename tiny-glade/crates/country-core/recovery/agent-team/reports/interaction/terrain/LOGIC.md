# 逐函数逻辑判等 — systems/terrain + 地形资源

审计员: logic-verifier-terrain
日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md
方法: 全部 fresh 反编译（ghidra shared 只读，fields=signature,decompiled_code,callees，批≤20），重建控制流后逐语句对照 Rust body。未实现函数无 Rust body 不在本次范围。
fresh 反编译证据: recovery/agent-team/work/logic-verifier-terrain/jobs/terrain-systems-result.json、resources-result.json、focus_closures-result.json、disasm-result.json（decomp/ 目录逐文件）。

## 结论摘要

- 已审计业务函数 16：**判等 9**（含 4 个附注）、**未实现-需重做 7**。
- 胶水-非业务（不计审计）: mod.rs 三注册函数（add_terrain_systems/insert_terrain_resources/add_terrain_events）、SSBO 载体类型实现（terrain_height 宏/water_mask/garden_mask/water_exclude_clutter_mask/terrain_rock_mask 的 StorageElement+SsboType）。
- 核心系统（edit_terrain_stroke/terrain_stroke_changed/terrain_hover_idx/ui_draw_terrain/ui_edit_terrain/ui_terrain_focus_mode）**6/7 未判等**，与 RESTORE/REVIEW 的"O 结构"登记不符——原函数内部逻辑大多未还原。

## 函数表

| VA | 原函数 | Rust 位置 | Verdict | 分歧点 |
|---|---|---|---|---|
| 0x14094e000 | edit_terrain_stroke | edit_terrain_stroke.rs | **未实现-需重做** | ①点集/编辑两循环、id 查找、错误串、magnitude=delta*sqrt(dz²+(dy²+dx²))/0.001、apply_resolved 逻辑、readback.pending LOCK INC 均吻合；②**原版每个事件写 indexmap::Bucket<(DecoratorId,DecoratorId),AlignKind>（StairsPlacementBucket）记录 tag 1/2/3/4**（callee grow_one<indexmap::Bucket<…>>），Rust 完全不写 StairsPlacementBucket；③**原版只写 8B unit 事件（grow_one<EventInstance<…>>，≈RecalculateHeightMapCmd），无 PlayTerrainSound 载荷事件**——Rust 的 PlayTerrainSound::Points/Falloff/Height/Removed 写入系编造（把 stairs bucket 记录误判为音频事件） |
| 0x140a03ba0 | terrain_stroke_changed | draw_terrain_stroke.rs | **未实现-需重做** | 有事件才处理、RecalculateHeightMapCmd 8B 写、readback.on_terrain_modified（LOCK INC）均吻合；**原版另写一条 StairsPlacementBucket 记录 tag=0**（grow_one<indexmap::Bucket<…>>），Rust 写 PlayTerrainSound::StrokeChanged（编造） |
| 0x140aabdd0 | delayed_terrain_changed_msg | delayed_terrain_changed_msg.rs | **判等** | 每元素 wrapping_sub(1)；`delay==0 && !copied` 时 FrozenMap get TerrainHeightSsbo（名长 0x3a assert）→ authority.copy_from_gpu → **两次拷贝间写 OnTerrainChange（8B）** → VisualTerrainHeightSsbo（0x40 assert）→ visual.copy_from_gpu；copy_issued 取走置 0、真则 push 2；末尾 retain 非零（Vec::retain 移除 0）。逐条吻合。Rust 的 library.get::<T> type_name 断言 = 原 memcmp 全路径名等价（已登记） |
| 0x140aabdc0 | ScheduledTerrainReadback::on_terrain_modified | read_terrain_data.rs | **判等** | 机器码 `LOCK INC qword [RCX]` = fetch_add(1, SeqCst) |
| 0x140ac9770 | history_record_terrain | deferred_terrain_history.rs | **判等**（附注） | 哨兵 0x8000000000000002；上帧非哨兵 → alloc 0x38 记录 {tag 1,1, old state, pending[0..3]} → History::add → 写 8B debug 事件。**分歧**: 原版 History::add 后无条件写 debug 事件（忽略返回值）；Rust 加 `if let Err → log+return` 错误分支，add 失败时不再写 debug（原版仍写）。主路径逐条吻合 |
| 0x140aff720 | terrain_hover_idx | terrain_hover_idx.rs | **未实现-需重做** | AppMode 门（disc<0xb && 0x421>>disc &1）、高度≤0/高度>0 两分支、悬停位/distance>0.5 门、写 {tag=1,id} 全吻合；**分歧**: ①原版函数开头**无条件复位输出 tag=0**（miss/门失败帧输出被清），Rust 不清；②**原版输出布局 = tag@+0、id@+8（≥16B）**，Rust HoveredTerrainStrokeId={u8 tag@0,i32 id@4}（8B），id 偏移错；③原版输出为事件（EventWriter 语义），Rust 为 Resource（ui_edit 消费面机制不同） |
| 0x140a9fdc0 | ui_draw_terrain | ui_draw_terrain.rs | **未实现-需重做** | mode==10 门吻合；**画布门反向**: 原版条件调 canvas = `height>0 ∧ (!hovered ∨ dist≤0.5)`（decomp 行 40-41），Rust 在 `height>0 ∧ hovered ∧ dist≤0.5` 才注册——非悬停帧原版仍调 canvas，Rust 不调；**状态机 prev=0 的 u8 回绕分歧**: 原版用 `(byte)(prev-1)>1` / `<2`（prev=0→255 算>2），Rust 用 `prev>2`/`prev<=2`（prev=0 算≤2），两分支结果相反；cVar9==3 分支原版只把 distance 写进栈槽（in_stack_00000070），**new_stroke / insert_or_morph_last_point 来自不可恢复 jumptable（&DAT_142adc498）的目标猜测**（fresh decomp 明确 UNRECOVERED_JUMPTABLE）；状态机由事件队列 subtype（`extraout_RAX+0x19`）驱动，Rust 用编造的 TerrainHoverInput.pressed 模型 |
| 0x140aef1f0 | ui_edit_terrain | ui_edit_terrain.rs | **未实现-需重做** | 仅 mode==10 门吻合。**原版主体全部缺失**: 逐笔画 `UiState::add<String>` widget 注册（type 字节 = `(iVar37!=id)*3+4`）+ `format_inner` 串；事件 subtype switch（&DAT_142ae404c，case 0-4/6/8/0xc/0xf）驱动多路事件写（WindowFocusEvent 型 0x10B、WriteSettingsFileCmd 型 8B、indexmap::Bucket stairs 记录 tag 2）；case 3 写 TerrainEdit 历史哨兵 **0x8000000000000001** 且校验 0x…0002 panic；case 4 仍 UNRECOVERED_JUMPTABLE。**Rust 的 MouseActivityTimer / 16 位脏掩码→EditTerrainStrokeCmd/SetTerrainStrokePtsCmd 是自造近似**（原版脏判定=事件队列非空 `(!bVar63||!bVar65)&&(!bVar62||!bVar64)`，不产这些命令）。HoveredTerrainStrokeId 读 id@+8，Rust 读 +4 |
| 0x1409a4fb0 | ui_terrain_focus_mode | ui_terrain_focus_mode.rs | **未实现-需重做** | fresh 反编译超时（167KB 内联，UNRECOVERED_JUMPTABLE），以 closures 为准: closure$1@0x1409a8350 = "vertex_color_shader" SignifierEntityBuilder::build + bevy command push `insert<DrawTopology>`；closure$2/3/4 = "brush_arrow_flat_to_single_forward"/"brush_arrow_flat_to_w_flat_single_up" 等 signifier 创建。**Rust 仅行为级骨架（门 + 单 TerrainFocusTrigger 事件），signifier 创建/raycast/焦点状态机全部未实现**（RESTORE 已如实登记"主体待取证"，按协议标未实现） |
| 0x1408bbf50 | RockyTerrainMesh::load | rocky_terrain.rs | **判等**（附注） | read_to_string→serde→顶点 0x24→0x30 打包→add_new_with_data×2 结构吻合。分歧: 源→GPU 字段重排语义 I（已登记）；错误类型 anyhow vs Rust io::Error（适配）；triangle_groups 在返回序未见于反编译（按文档 O） |
| 0x1409e9640 | TerrainHeightsData::sample_at_world_xz<TerrainHeightSsbo> | terrain_height.rs | **判等** | divps 130.0、+0.5、USE_FLOAT_GRID_SAMPLING_FIX 条件 clamp（maxps 0/minps 1、NaN 处理）、3×lerp —— 与 Rust 委托 utils::float_grid::bilinear_sample_at_world_xz（extent 130、centered 模式）一致 |
| 0x140257960 | sample_at_world_xz<VisualTerrainHeightSsbo> | terrain_height.rs | **判等** | 直接委托 bilinear_sample_at_world_xz<TerrainHeightFloatGrid>，Rust 泛型委托一致 |
| 0x1409e9a20 | TerrainHeightsData::intersect_ray | terrain_height.rs | **判等** | Aabb3 classify、Inside 时 enter<=0→0、enter<=t_max、step=(end-enter)*0.03125、32 步行进（direction.y*t+origin.y<height 判命中）、7 次二分（y<sampled→hi=mid；sampled<=y→lo=mid）、末次 y<sampled→lo else mid、命中写 {tag=1,t,pos}。**机器码（0x1409e9eec..0x1409ea013）证实 plane/terrain 合并**: `ucomiss plane_t,terrain_t; jbe` → plane_t>terrain_t 取地形，≤ 取平面；无地形命中时平面 fallback 存在（Rust `terrain.or(plane)` 吻合） |
| 0x1409ea020 | TerrainHeightsData::copy_from_gpu | terrain_height.rs | **判等** | min/max(+0x50/+0x54)!=默认 → 复位+位图 memset(0xff)；64×8×64 瓦片 16B 比较，位 `1<<tile_column`（8 sub_row 共位）；changed → tabloid publish。全吻合。Rust 额外 `assert_eq!(gpu.len(),…)`（原版无此断言，防御性新增） |
| 0x1408b54c0 | load_index_unchecked | terrain_height.rs | **判等**（附注） | 机器码证实算法 = u16/65535（DIVSS 0x142a7cd4c）、`v*max+(1-v)*min`（SUBSS 1-v、MULPS/ADDSS）。**分歧**: 原版读 max/min 于 +0x10/+0x18（接收类型或布局与 Rust TerrainHeightsData{max@+0x54} 不一致，待核——可能与 utils FloatGrid view 布局有关） |
| 0x1409ea800 | terrain_rock_mask::sample_at_world_xz | terrain_rock_mask.rs | **未实现-需重做** | x/130+0.5、clamp[0,1]、*512、trunc、min 0x1ff 均吻合；**分歧**: ①原版越界 `panic_bounds_check`（index>>5<len，按 u32 词长），Rust `unwrap_or(0)` 静默返回 0；②bounds check 用 index>>5 表明**按词位读取**（每 u32=32 位），Rust 按整 u32 词返回，疑似位/词语义不符；③x/130 divss vs Rust `x*(1/130)` |
| 0x140924900 | TerrainEditingData::get_last_mut_stroke | tg_terrain stroke.rs（库外 re-export） | **判等**（附注） | next_index-1 哈希查找吻合。分歧: 原版 next_index==0 或未中 → `unwrap_failed` **panic**；Rust 返回 Result Err（错误路径适配） |
| 0x140924a10 | TerrainEditingData::new_stroke | tg_terrain stroke.rs | **判等**（附注） | next_index<0x3fffff 门、自增、TerrainStroke 插入、碰撞 panic_fmt 吻合。分歧: 原版碰撞 panic，Rust 返回 Err(IdCollision)；原版布局 next_index@+0x38（Rust @+0x30 适配） |
| 0x140a97880 | TerrainStroke::insert_or_morph_last_point | tg_terrain stroke.rs | **判等** | len=0 push；len=1 距>0.001 才 push；len≥2 spacing≤dist→push（<0x400 上限），否则 morph last。UCOMISS/JBE 无序语义与 Rust `!(spacing>dist)` 一致。float 分量关联序（x²+z²+y² vs z²+y²+x²）存在舍入级差异待核 |

## 严重分歧汇总（按模块）

1. **PlayTerrainSound 事件系编造**: edit_terrain_stroke / terrain_stroke_changed 原版把 tag 0/1/2/3/4 音频观测写入 **StairsPlacementBucket（indexmap::Bucket<(DecoratorId,DecoratorId),AlignKind>）**，不产生 PlayTerrainSound 载荷事件。Rust 的 PlayTerrainSound 变体/写入、terrain_audio 消费链均为对原音频桶的误读。
2. **ui_draw_terrain 画布门反向 + 状态机编造**: 门条件与非悬停帧行为相反；new_stroke/续笔来自不可恢复 jumptable 猜测。
3. **ui_edit_terrain 主体未还原**: widget 注册/事件 subtype switch/stairs 桶写/TerrainEdit 哨兵历史全部缺失；"16 位脏掩码→命令"是自造近似。
4. **terrain_hover_idx 输出布局错**: id@+8（原）vs id@+4（Rust）；缺无条件 tag=0 复位；事件 vs Resource。
5. **ui_terrain_focus_mode 仅骨架**: signifier 创建 + DrawTopology insert 未实现。

## 方法备注

- fresh 反编译对 ui_terrain_focus_mode 超时、对 ui_edit_terrain/ui_draw_terrain 尾部 jumptable UNRECOVERED——这些按协议标未实现，不采信 RESTORE 的"行为级骨架/待取证"为通过。
- 判等函数的附注分歧均为错误路径适配或已登记 I 项，不影响主路径逐条判等。
- tg_terrain（共享基础）的三函数按 inventory 归属审计，分歧为 panic-vs-Result 错误路径。
