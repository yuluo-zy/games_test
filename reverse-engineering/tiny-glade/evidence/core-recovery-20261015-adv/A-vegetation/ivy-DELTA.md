# ivy 核对 DELTA（A 植被历史核对，2026-10-16）

对照：`A-vegetation/ivy-SPEC.md`（2026-10-15）vs 当前源码 `crates/country-core/src/{resources/ivy,systems/ivy.rs,systems/ivy}`（commit `f510c7e7`+merge）。
机器码依据：`evidence/core-recovery-20261015/ivy/grower.asm.txt`、`spawn.asm.txt`、`insert.asm.txt`、
`evidence/country-core-deep/collision-eco/ivy-grower-disasm/`（2026-10-16 新反汇编）。
状态判定：**真实实现**＝有真实业务代码+真实消费者+原机器码一致；**有水分**＝实现存在但语义/常量/接线偏离原侧或占位；**仍缺**＝未实现；**待取证**＝无法从现有证据证实。

| SPEC 项 | 当前状态 | 证据（源码位置 / 原地址） | 备注 |
|---|---|---|---|
| G1.1 系统门控（+0x118 跳表 / +0x48 bit0 / +0x11c timer） | 有水分 | `systems/ivy/ivy_grower.rs` 相位机 `GrowerState{phase,guard,timer}`（0x14094ac20 读 `[r14]+0x48`、0x14094ac31 ucomiss [rcx+0x11c]、0x14094c09f XOR toggle，grower.asm 180-191/1233-1241 已核） | 相位分派/定时结构真实；但**守卫字节建模为 Local 私有 toggle，未读 `IvyStorage.flag_48`**（`resources/ivy/ivy_storage.rs:97` 字段存在但全库无消费者）。原侧守卫字节持久在资源 +0x48；若外部系统写该字节，本实现会忽略。行为节奏（隔帧推进）等效，但 SPEC 明言"grower 读取它"。 |
| G1.2 记录布局（+0x68/+0x84/+0x88/+0x8c） | 真实实现 | `Ivy{pending_growth, growth_progress, growth_delay, flag_8c}`（ivy_storage.rs:38-45） | 字段语义与原生偏移一致（`flag_8c` 仅 +0x8c 占位，见 G6 仍缺）。非 repr(C)，无 ABI 约束。 |
| G1.3 生长推进（scalar_88>0.001 递减 / progress+=delta/0.075 / lerp / ≤0.99 continue） | 真实实现 | `advance_growth`（ivy_grower.rs:98-120）+ `run_growth_pass`（:366-369）；常量 `SETTLED_THRESHOLD 0.001`(0x1429cd060)、`SEGMENT_DURATION 0.075`(0x142a9abd4)、`GROWTH_COMPLETE 0.99`(0x142a948d8) | 单点 ivy 清 pending（0x14094aedb）、末点位置写回、clamp 顺序全部与 asm 一致（0x14094aea9/adf5/b1bb/b25a）。 |
| G1.4 完成段事件（state=0;progress=0; last_segment_point_ids → 事件行） | 真实实现 | `advance_growth` 完成分支（ivy_grower.rs:110-119）；`IvySegmentDoneGrowingMsg` 24B（ivy_leaf_spawner.rs:11-16） | 24B payload（a,b,ivy）+ EventId 前缀=原生 32B 行，SPEC 确认保持现状。测试 `growth_completes_and_emits_one_event_per_segment` 覆盖。 |
| G1.5 proposal 路径（cell 采样 / closest_segment / 0.5 高度 / 接受/方向/高度带 / 落点） | 部分真实/有水分 | `propose_new_segment`（ivy_grower.rs:394-473）；`cell_index_of_world_point` floorf×2（asm 0x14094af75/af82）、0x5 max(0x3f000000 @0x14094b054)、0.365 接受(0x142ab3190 @0x14094b080)、高度带 0.2/−0.46/−0.79+0.2（0x14094b73d..b77a，代码 `wall_height_band` :211-219） | 结构/门/常量已验证。**两处偏离**：①`proposer.get_direction(tail, ivy.source_60)`（:432）用 +0x60，SPEC 1.2/1.5 明示 seed=+0x20 ivy id；②`check_for_wall_jump`（:263-285）是全墙最近段遍历，非原生空间哈希 `query_with_segment`(0x140aa5850)+闭包(0x1412f6b60/0x141308470)，代码自标待取证。0.42 落点门（见 G6）未实现。 |
| G1.6 常量总表 | 真实实现 | 见 G1.3/G1.5 常量 | 0.21(DIRECTION_SCALE)、1.0(PROGRESS_MAX) 均在；0.42 未消费。 |
| G2.1 门控与预算（[res+0x1c]==1 / total_point_count+0x13<0x9c40 / 每帧 0x46 上限） | 有水分/仍缺 | `ivy_spawner`（ivy_spawner.rs:250-268）`mode.native_tag()!=1` 与 `storage.next_id()>=40000` | ①门资源身份 SPEC 待取证，实现解析为 `AppMode::Editing(Wall)` tag 1——合理但未证；②**预算用 ivy 计数 next_id()，非原生 total_point_count**；③**每帧 0x46(70) 点上限未实现**。 |
| G2.2 两条根路径（cursor/random） | 部分真实/待取证 | `try_ivy_root_next_to_cursor`（:165-185）、`try_random_ivy_root`（:192-240） | 噪声门 `density<=threshold`(-0.02，insert 0x1409441cb 证实)与 batch 真实；**random 根法向偏移 0.68 与 SPEC 的 0.63(0x1429258d4) 冲突**（代码引用 RE-ANALYSIS2 未在本仓库取证到，标待取证）。 |
| G2.3 批量生成参数（batch=2+rand%3 / count 0x14(20) / scalar cursor→[0,3] random→[0,1] / source_60=wall id） | 有水分 | `spawn_validated_root`（ivy_spawner.rs:244-246）`spawn_ivy(pos, dir, 1.0..2.0, 4..16, 0)` | **batch `2+usize_less_than(3)`**（:201）=2..4 真实；但 **①count_range 用 4..16，SPEC 栈参 [rsp+0x28]=0x14(20)**；**②scalar_range 恒 1.0..2.0，未按 path 区分 cursor→[0,3]/random→[0,1]**；**③source_60 传 0，SPEC 明示 wall id [rsp+0xf8]**。三处均与 SPEC 冲突，源码注释自认待取证。 |
| G2.4 UI/ActionSet 门 | 仍缺 | `ivy_spawner` 参数无任何 `DigitalActionState`/ActionSet | SPEC 待取证键；实现仅用 `CursorTerrainRaycast` 取光标位，未读 pressed 门。 |
| G3.1 pruner 结构 | 部分真实 | `ivy_pruner`（ivy_pruner.rs:147-178） | 消费 `OnWallChanged`（原侧 dirty-wall deque 的等价替代，注释承认）；逐 ivy 重验。 |
| G3.2 is_point_still_valid（0x140998dc0） | 待取证 | `is_point_still_valid`（ivy_pruner.rs:104-139） | 空间哈希 cell + closest_segment + 0.365(0x142ab3190) + 高度>0 + `interpolated ∈ (-0.01/h, 1+0.01/h)` 带。**SPEC 3.2 判据是 wall_y+带常数(0.2/−0.79/−0.46) vs point.y**，与实现语义不同；code 的带判据出处未在本仓库取证到，标待取证（两版都是"高度带"检查，但非同一检查）。 |
| G3.3 末点失效删尾 | 部分真实/偏离 | `ivy_pruner` while 循环（:161-176） | **SPEC 只重验末点与根点**；实现从 index 0 起逐点重验，首个失效点截断整个后缀并清叶子/生长（与 RemoveIvyCmd 路径同构）。更激进但覆盖原行为超集；SPEC 的"末点-only"未精确复现。 |
| G3.4 根点失效弹根 | 仍缺 | `ivy_pruner` 无 `pop_front` 路径 | SPEC 3.4 的 `front=(front+1) mod cap` + root 副本更新 + 叶子清理未实现。`Ivy::remove_root` 存在（ivy_storage.rs:86-88）但无消费者。 |
| G3.5 循环出口/临时数组 | 部分 | 叶子清理内联（`delete_affected_leaves` :174，0x1412f5490 对应） | 未收集被删 id 数组清 leaf map 键；SPEC 本身标"整 ivy 删除"待取证。 |
| G3.6 pruner_cmd / remove_ivy_at_location | 真实实现 | `ivy_pruner_cmd`/`apply_remove_ivy_cmd`（ivy_pruner.rs:65-96） | 已实现基线，SPEC 说无需改动。 |
| G4 root/branch 删除策略 | 仍缺（部分） | 见 G3.4 | 根部删除路径缺；删尾路径以"全点重验+后缀截断"近似。 |
| G5 UI/ActionSet 门（grower 模式判别） | 待取证 | `GrowerState` phase + guard | +0x118/+0x120 资源身份未证实；grower 不读 AppMode，SPEC 亦未断言。 |
| G6 wall jump / hole 约束 | 仍缺 | grower 无 `PrevWallHoles`/`flag_8c` 写入；`check_for_wall_jump` 机制不同（见 G1.5） | **洞测试整段缺失**：原生 0x14094b7be..b86e `PrevWallHoles::wall_holes`(0x140f86d00) + 盒包含(0x141240140) + `flag_8c=1`(0x14094b869) 未实现；`flag_8c` 消费（0x14094ae63..96 leaf map 查+清零）未实现；**0.42 落点门（0x142ab31b0 @0x14094bd38/bf26）未实现**。 |
| G7 ivy_leaf_spawner GladeSettings +1c 旗标 | 真实实现 | `ivy_leaf_spawner`（ivy_leaf_spawner.rs:88-110）`Res<GladeSettings>` 读 `settings.ivy_leaves`（glade_settings.rs:13 首字段，注释锁定"Native +1c"） | 门=0 时直接 return **不推进 EventReader**（符合 SPEC）；测试 `glade_settings_controls_actual_ivy_leaf_event_consumption` 验证。SPEC 建议的注释锁定已做。 |
| G8 GPU 上传/渲染边界 | 部分真实（无消费者） | `ivy_send_to_gpu_leaf.rs` / `ivy_send_to_gpu_segments.rs` | 48B 布局与 shader `instanced_ivy_leaf/branch.raster.hlsl` 对齐；SSBO 键 rne-read 证实。**但 `arch_height=0.0`、`seed=frame` 为占位（自标待取证），且 upload 函数无 Bevy 系统消费者**（`registration.rs:71-84 add_ivy_systems` 未挂 send_to_gpu，注释"待接线"）。SPEC 边界本只要求 CPU storage 字段，此实现超出边界但不可验收闭合。 |
| G9 原全局调度（add_ivy_systems 22 系统链） | 仍缺（部分） | `registration.rs:71-84` 链 7 系统（grower/spawner/pruner/cmd/leaf_spawner/动画×2） | 原生 0x140944070 `0x16`=22 系统链；缺 ivy_audio、send_to_gpu、精确 set/顺序（SPEC 待取证）。 |

## 合成 agent 应关闭的真实剩余缺口（按验收纪律优先级）

1. **G6 洞/跳墙约束**：实现 `PrevWallHoles` 盒包含（0x140f86d00 + 0x141240140），命中写 `flag_8c=1` 并跳过本段加点；下一主循环消费 flag_8c（leaf 查询/清理，0x14094ae63..96）。同时实现 **0.42 落点门**（0x142ab31b0）。这是 grower 与 SPEC 差距最大的一处。
2. **G2.3 spawner 参数**：count_range 起点恢复 0x14(20)（现 4..16 自认占位）；scalar_range 按 path 区分 cursor→[0,3]/random→[0,1]（现恒 1.0..2.0）；source_60 传墙 id（现 0）。每帧 70 点上限（G2.1）补齐。
3. **G1.5 方向种子**：`get_direction` 用 `ivy.id`（+0x20）而非 `ivy.source_60`（+0x60）。
4. **G1.1 守卫接线**：把守卫字节落到 `IvyStorage.flag_48`（+0x48）读写（若外部系统写该字节则必须；否则删除死字段并登记）。
5. **G3.4 根部弹根路径**：实现 `pop_front` + root 副本更新 + 叶子清理。
6. **G2 预算门**：改 total_point_count（原生 `+0x13 < 0x9c40`），现用 next_id() 仅近似。
7. **G8 消费者接线**：send_to_gpu 的 `arch_height`/`seed` 真值取证；由 D 组（rhapsody 接入）挂系统链时登记。
8. 待取证登记：is_point_still_valid 判据、random 根偏移 0.68 vs 0.63、ActionSet pressed 键、+0x118/+0x120 资源身份、22 系统链精确序。
