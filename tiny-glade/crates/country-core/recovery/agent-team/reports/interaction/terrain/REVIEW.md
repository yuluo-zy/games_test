# REVIEW — systems/terrain

审查员: reviewer-interaction (独立核对,以原证据为准)
日期: 2026-10-15
审查范围: 当前 src 已有实现 + 原函数清单逐项核对 + 地形资源(terrain_editing/terrain_height 及缺失的 terrain_stroke/rocky_terrain/terrain_rock_mask)。restorer 尚未提交新增(工具 UI/历史生产),本次审已有现状。

## 核对方法

- inventory.json 枚举原函数 + 章节 + 既有 decomp 全文核对:
  - `terrain_stroke_changed` @0x140a03ba0、`edit_terrain_stroke` @0x14094e000、`delayed_terrain_changed_msg` @0x140aabdd0(既有 decomp 全文)。
  - 夹取常量 read_memory:`0x142a7fa30`=20.0(height)、`0x142925904`=1.0(falloff)、`0x1429cd060`=0.001(magnitude 除数)。
  - `ScheduledTerrainReadback::on_terrain_modified` @0x140aabdc0(既有 decomp NotFound,以 terrain_stroke_changed 内联 LOCK INC + read_terrain_data.rs 注释佐证)。

## 原函数清单(no_formal_mapping ≠ 零实现)

| 原函数 | VA | 当前 src | verdict |
|---|---|---|---|
| `mod::add_terrain_systems` | 0x1409455f0 | 未复现(有 add_terrain_cpu_systems 适配) | PARTIAL |
| `mod::insert_terrain_resources` | 0x1409457c0 | 未复现 | PARTIAL |
| `mod::add_terrain_events` | 0x140945a20 | register_terrain_events(6 事件) | PARTIAL |
| `draw_terrain_stroke::terrain_stroke_changed` | 0x140a03ba0 | draw_terrain_stroke.rs | **OK** |
| `edit_terrain_stroke::edit_terrain_stroke` | 0x14094e000 | edit_terrain_stroke.rs | **OK**(附 schema 待取证) |
| `delayed_terrain_changed_msg::ScheduledTerrainReadback::on_terrain_modified` | 0x140aabdc0 | read_terrain_data.rs | **OK** |
| `delayed_terrain_changed_msg::delayed_terrain_changed_msg` | 0x140aabdd0 | delayed_terrain_changed_msg.rs | **OK** |
| `deferred_terrain_history::history_record_terrain` | 0x140ac9770 | 缺失 | GAP |
| `terrain_hover_idx::terrain_hover_idx` | 0x140aff720 | 缺失 | GAP |
| `ui_draw_terrain::ui_draw_terrain` | 0x140a9fdc0 | 缺失 | GAP |
| `ui_edit_terrain::ui_edit_terrain` | 0x140aef1f0 | 缺失 | GAP |
| `ui_terrain_focus_mode::ui_terrain_focus_mode` | 0x1409a4fb0 | 缺失 | GAP |
| `resources/terrain_stroke.rs`(TerrainStroke::insert_or_morph_last_point 0x140a97880 等) | — | 委托 tg_terrain::stroke | OK(委托) |
| `resources/terrain_height.rs`(sample_at_world_xz 0x1409e9640 / intersect_ray 0x1409e9a20 / copy_from_gpu 0x1409ea020 / load_index_unchecked 0x1408b54c0) | — | terrain_height.rs | OK(限定) |
| `resources/rocky_terrain.rs`(RockyTerrainMesh::load 0x1408bbf50 等) | — | 缺失(src 无 rocky_terrain.rs) | GAP |
| `resources/terrain_rock_mask.rs`(sample_at_world_xz 0x1409ea800) | — | 缺失 | GAP |

## 逐函数核对

### 1. terrain_stroke_changed @0x140a03ba0 — **OK**

既有 decomp(FUN_140a03ba0)核对:
- 排空 EventReader(param_1),任一未读事件存在则:
  1. push RecalculateHeightMapCmd(8B 事件,param_3)
  2. `LOCK; *counter += 1; UNLOCK`(param_2 = ScheduledTerrainReadback 原子计数)
  3. push PlayTerrainSound tag 0(StrokeChanged,param_4,stride 0x28)
- 当前 Rust: `if changed.read().last().is_some()` → write RecalculateHeightMapCmd → on_terrain_modified() → write PlayTerrainSound::StrokeChanged。**顺序与语义一致**(read 消耗全部未读,等价于原版置 reader 游标到尾部)。✓

### 2. edit_terrain_stroke @0x14094e000 — **OK**

既有 decomp(FUN_14094e000)核对:
- **先处理 SetTerrainStrokePtsCmd(param_2,stride 0x30),后处理 EditTerrainStrokeCmd(param_1,stride 0x28)** —— 当前 Rust 同序 ✓。
- **缺失 stroke**:原版在 hashmap 未命中时构造 `log` 记录(UNK_142ab3a40 文件串 + line 0x33),**不 panic、continue**;章节"缺失 panic"是错的,当前 Rust `log::error!` + continue 正确 ✓(本次确认)。
- **点替换**:old=stroke.points[0] 与 new=event.points[0] 差分,magnitude = `delta × √(dx²+dy²+dz²) / 0.001`(除数 = 0x1429cd060 = 0.001,本次 read_memory 证实;当前 Rust `f32::from_bits(0x3a83126f)`=0.001 ✓);先算后 clone ✓;空 points 双方都 panic(原版 `len==0` 显式 panic,当前 [0] 下标 panic,等价)。
- **参数编辑**:委托 tg_terrain::apply_resolved(B 14094eb76..14094ef1b)。用本次读到的常量逐分支核对:
  - height Delta(field@-8):激活基线(旧==0 && delta>0 → 旧=1.0,0x3f800000)+ clamp[0, 20.0](fVar2=20.0)+ `new<1.0 → 0`(NaN 掩码 ~-(new<1.0)) + tag 2 → tg_terrain `if delta>0 && prev==0 {prev=1.0}` + `min(20, max(0,...))` + `if clamped<1.0 {0.0}` + tag 2 ✓ **完全一致**。
  - height Absolute:直写不夹取 + tag 2 ✓。
  - falloff Delta(field@-0x10):clamp[0, 1.0](fVar21=1.0)+ tag 1 ✓。
  - falloff Absolute:直写 + tag 1;new==0 && old>0 → tag 4(removed)✓。
  - tag 1/2/4 与 PlayTerrainSound::Falloff/Height/Removed 映射 **正确**(初读时曾疑 1/2 互换,读常量后证实不互换)。
- **记录顺序**:每次修改(含 Keep+Keep)无条件 recalculate + counter inc —— 与 tg_terrain `requests_recalculation: true` 一致 ✓。

**待取证(schema)**:原版事件记录 stride 0x28、stroke id 在 +0x1c、`*piVar33`(offset 0)即 falloff Adjustment 判别 —— 即原 `EditCommand` 字段序似为 `{falloff, height, position, id}`(id 在尾部),而 tg_terrain `EditCommand` 为 `{id, falloff, height, position}`。当前生产/消费同一类型所以内部一致,但**与原生事件字节布局不一致**;待 ui_edit_terrain 恢复时需按原字段序对齐,否则 serde/ABI 不保真。标记待取证,不阻塞当前函数。

### 3. delayed_terrain_changed_msg @0x140aabdd0 — **OK**

既有 decomp(FUN_140aabdd0)核对:
- param_3 = Local<DelayedUpdateWallsMsgBuffer>(delays Vec)—— **章节"delays 在 ScheduledTerrainReadback 内"是错的**,当前把 delays 放独立 Local 正确 ✓。
- 循环逐 delay `-1`(u32 回绕),首个归零且未复制 → 拷贝权威(param_5,stride 0x30 事件)+ 写 OnTerrainChange(param_4)+ 拷贝视觉(param_6),只复制一次 ✓(当前 Rust `wrapping_sub` + `copied` 标志同序)。
- 之后读/清 `copy_issued`(param_2 @+8 bool),为真 → push(2)(延迟 2 帧)✓;最后压实非零项 ✓。
- OnTerrainChange 位于两次拷贝之间 ✓。
- `ScheduledTerrainReadback::on_terrain_modified` = `pending.fetch_add`(与 terrain_stroke_changed 内联 LOCK INC 一致;`pending: AtomicU64` 与 `*(longlong *)*param_2 += 1` 一致)✓。
- SSBO 取值走类型库 get::<TerrainHeightSsbo>/<VisualTerrainHeightSsbo> 而非原版名字 hash 查找 —— 适配,行为等价(尺寸断言 0x3a/0x40 名串在 decomp 可见)。✓

### 4. mod.rs 注册 — PARTIAL

原 add_terrain_systems/insert_terrain_resources/add_terrain_events 未复现;当前 register_terrain_events 只注册 6 个 CPU 事件,add_terrain_cpu_systems 只挂 3 个 CPU 系统(Edit/Changed/Readback 链)。terrain_hover_idx/ui_draw_terrain/ui_edit_terrain/ui_terrain_focus_mode/deferred_terrain_history 及 TerrainHeightsData 双实例、网格资源插入均未挂入。属部分注册。

### 5. 地形资源

- `terrain_editing.rs`:TerrainEditingData 包装 tg_terrain::stroke::TerrainEditingData(Deref/DerefMut)。原 TerrainEditingData::new_stroke@0x140924a10 / get_last_mut_stroke@0x140924900 由 tg_terrain 实现(委托共享基础)。✓
- `terrain_height.rs`:copy_from_gpu@0x1409ea020 / sample_at_world_xz@0x1409e9640 / intersect_ray@0x1409e9a20 有详细证据注释 + 测试,本次抽查 copy_from_gpu 的瓦片脏位图逻辑与 delayed 消费一致;标记 OK(限定),不做全量重审。
- **缺失**:`rocky_terrain.rs`(RockyTerrainMesh::load@0x1408bbf50)、`terrain_rock_mask.rs`(sample@0x1409ea800)在 src 无对应文件 —— GAP。

## 对新增工作(restorer 批次:工具 UI/历史生产)的核对基线

- `deferred_terrain_history::history_record_terrain`@0x140ac9770:PendingTerrainHistory 脏哨兵(0x8000000000000002)→ 首次变化 History::add + DebugHistoryMsg,之后不重复。
- `terrain_hover_idx`@0x140aff720:AppMode 0x421 掩码门 → MouseTerrainRaycastSsbo 读 id@+0x10(-1 哨兵)→ 水位≤0/光标无效/距离≤0.5 不更新 → 写 HoveredTerrainStrokeId。
- `ui_draw_terrain`@0x140a9fdc0 / `ui_edit_terrain`@0x140aef1f0 / `ui_terrain_focus_mode`@0x1409a4fb0:按章节既有 decomp 闭合(ui-input 域细查状态机)。

## 结论

当前已有地形实现(3 个 CPU 系统 + ScheduledTerrainReadback + 委托 tg_terrain)经反编译逐条核对**行为正确**,未发现偷懒/占位冒充。主要缺口是整块 UI/历史生产/rocky_terrain/terrain_rock_mask 未恢复(符合排队状态)。EditCommand 字段序与原事件布局的差异标记待取证。terrain 模块维持 queued,新增工作按基线核对。
