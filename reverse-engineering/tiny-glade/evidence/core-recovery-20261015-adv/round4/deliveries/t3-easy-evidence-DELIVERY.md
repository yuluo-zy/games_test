# T3 易取证项 DELIVERY（2026-10-16 第四轮 ②，① 契约 8 项落地 / 2 项待仲裁 / 若干待取证）

> 实现对象：① T3 契约（round4/contracts/t3-*，10 份）。② 文件权内落地 8 项；
> save 域（域采集器 + PostGameLoadCommand::Apply）仲裁已申请待批准；待取证项登记不造。
> 整 crate `cargo check` **零错误**（10 warning 全为既有/用户重构域）。

## 真实代码接线

1. **VERTICAL_MIN 定案 = 0.0**（`systems/ivy/ivy_spawner.rs`，① t3-ivy-constants §1）：
   0.000928482（字节 0x3a73655f 全 EXE 0 次出现，编造值）→ **0.0**（0x142960690）。
   模块 doc、`vertical_u` doc、`VERTICAL_MIN` 常量 + 公式注释（1.5/-0.75/1.0/0.0 常量表）
   全部对齐。
2. **0.68 vs 0.63 定案 = 0.68**（同契约 §2）：`RANDOM_ROOT_OFFSET=0.68` 正确，补 0x142ab31f0
   = 0.68f / 0x142ab3210 = -0.68f 注释；0x1429258d4=0.63f 标注为不同常量（非根偏移）。
3. **flag_8c 清理谓词**（`systems/ivy/ivy_grower.rs`，同契约 §3）：`GrowerState` 加
   `affected_ids: Vec<u64>`（跨帧持留，native 局部 drop 语义待取证注释）；`run_growth_pass`
   取上轮列表（`mem::take`）作谓词 `flag_8c 置位 且 id ∈ 列表 → 清零`（0x14094ae63..96）；
   `propose_new_segment` 洞命中时 `flag_8c=1 + affected_ids.push(ivy.id)`（0x14094b7be..b86e）。
   `+0x60` 字段语义待取证，以 ivy id 建模标注。
4. **EditType tag 双错修正**（`resources/history/mod.rs`，① t3-edittype-terrain-tag §2，
   **中央仲裁批准**第 14 条）：Terrain tag 15→**0**（visit_str case7 "Terrain"→0）、
   Tree tag 0→**12**（case4 else 0x142aee14b="Tree"→0xc）；变体序按原生对齐（Terrain 首变体、
   Tree 移至 tag12 槽）。其余 13 变体不动。deferred_terrain_history **不经 EditType** 定案
   （0x140ac9770 0x38B 记录 tag=1 直入 History）；crate `EditType::Terrain`+`ReplayCommand::Terrain`
   保持本地 CPU 等效模型 + 标注。
5. **ui_draw_terrain jumptable 分支**（`systems/terrain/ui_draw_terrain.rs`，① t3 契约 §2）：
   state 0=无动作 / 1=点按 new_stroke（已落地）/ **2=移动续笔**：`get_last_mut_stroke` +
   `insert_or_morph_last_point([cursor], MORPH_SPACING)`（0x140924900/0x140a97880；morph 间距
   常量待取证，MORPH_SPACING=1.0 接入标注 I）/ 3=释放读笔：当前输入模型（TerrainHoverInput）
   无 release 事件，state 3 不可达，语义登记待取证。
6. **update_mod_key_state producer**（`systems/backend/input/mod_key.rs` + `input/mod.rs`，
   ① t3-input §1/§2/§5）：`ModKeyConfig` 适配资源（`[config+0x78]` 状态 / `[config+0x7c]`
   阈值，身份待取证）+ 表1 跳表（0x142ae5f50）分派：组 0=强制 ON / 1,3,4=强制 OFF /
   2=阈值 `0.0>=threshold` setae / 5,6,7=动作键组（键码映射待取证，按 OFF 登记不造）+
   `setb 字节@0` 门（状态<3 → disable_snapping）。注册 ModKeyConfig + 挂 update_mod_key_state。
7. **sheep 5 宿主组件登记 + required 集冲突**（`resources/critters/sheep.rs`，① t3-sheep §1/§2）：
   Sheep（已实现）+ SheepAudio/UmbrellaHolder/SheepAccessory/FloatieHolder（待恢复，组件名串
   0x142c3652c/384ea/36a9f/37bb5/36fd6 登记）。**required 集冲突**：① 契约 §2 推断
   Sheep>=require(Transform,CritterLocomotion)（标注 I），但 crate 自有恢复 components/registry.rs
   （0x6f0/0x700 单态模板，151 注册函数无跨组件调用）断言原版 required=空集——按"不伪造"
   本组件**不补 #[require]**，待①定点后按真值落。
8. **ui_clutter parent-map 0x60B 槽位**（`systems/ui_clutter/ui_autoclutter.rs`，① t3 契约）：
   `AutoClutterParent` 扩 O 观测字段 tag@+0x00（<3 门）/ slot_count@+0x10 /
   mesh_ty_arg@+0x30 / mesh_ty_arg2@+0x38（decomp-0x14099ca60 行 480-520）；其余区间
   +0x04..0x0f/+0x14..0x2f/+0x3c..0x5f 标注 U 不填；保留 crate 转换输入字段（I 模型）。
9. **save 域采集器**（`systems/save/snapshot.rs`，① t3-save-domain-collectors §1，
   **中央仲裁批准**第 15 条）：`collect_walls`（0x141308b40，inner_walls HashMap<WallId,WallState>
   → Vec，容器访问面待取证）+ `collect_ids`（0x141309840，HashSet<u64> → Vec<u64>，来源集
   身份待取证）；Tree 已有 `collect_trees`（0x141305170）。采集输出 = 快照节载荷，序列化节
   布局待取证不编造。
10. **PostGameLoadCommand::Apply 桥**（`systems/save/load_game.rs`，FUN_1412e0b90）：
   `load_game_system` 加 `Commands` 参，`ReplayedOnLoadCommand::Apply` → `Commands::
   insert_resource<NextSession>`（0xa8B 命令条目 + FUN_1412db840 call_once；0xa0B 载荷 →
   NextSession 重构字段待取证，以当前资源克隆作桥，① 定点后按真值重构）。

## candidate 链测试

- mod_key.rs：`update_writes_flag_bytes_per_table_one`（组 0 ON / 组 1/3/4 OFF / 组 2 阈值
  setae / 状态>=3 disable_snapping 关）+ 既有谓词测试。
- ivy_grower flag_8c 谓词经 lib 类型检查；ui_draw_terrain state-2 morph 经 lib 类型检查。
- 整 crate `cargo check` 零错误（10 warning 为既有/用户重构域）；本组 T3 文件全零错误。

## 原侧验证

- 常量：0x142960690=0.0、0x1429258d0=1.5、0x142ab3220=-0.75、0x142925904=1.0、
  0x142ab31f0=0.68、0x142ab3210=-0.68、0x1429258d4=0.63（不同常量）；编造值 0x3a73655f
  全文件 0 次出现（① capstone/rne-read）。
- EditType：visit_str 0x140b3b630 全解（Terrain case7 / Tree case4 else 0x142aee14b）；
  deferred_terrain_history 0x140ac9770 0x38B tag=1 记录直入 History。
- ui_draw_terrain：跳表 0x142adc498 4 条目（0x140aa076d epilogue / 0x140aa00e9 点按 /
  0x140aa0235 续笔 / 0x140aa02c7 释放），函数 0x140924a10/0x140924900/0x140a97880。
- update_mod_key_state：capstone 0x140af9c00 全函数（表1 0x142ae5f50 8 条目 + setb 门）。
- sheep：procs.tsv 0x14089ec50/0x14089f340/0x14089fa30/0x1408a0120/0x1408a0810 + 组件名串。
- ui_clutter：decomp-0x14099ca60 行 444-520（0x60B parent 分配 + slot 索引）。

## 仍缺项（待取证，登记不造）

1. ivy count_range.end（SPAWN_COUNT_RANGE 上限，IvyStorage::spawn_ivy 0x1408696f0 载荷待解）。
2. ivy send_to_gpu seed/wall_normal（GPU 后置，不编造）。
3. ui_draw_terrain state 3（释放/读笔）——当前输入模型无 release 事件，语义待取证。
4. update_mod_key_state 组 5/6/7 键码枚举 + config 资源身份 + u32 高 2 字节语义。
5. sheep required 集（registry 证据=空集 vs ①推断 Transform+CritterLocomotion，待①定点）；
   其余 4 宿主组件本体待恢复。
6. ui_clutter 0x60B +0x04..0x0f/+0x14..0x2f/+0x3c..0x5f 区间 U。
7. save 域：`inner_walls` 容器访问面、u64 来源集身份、快照序列化节布局、
   Apply 载荷（0xa0B）→ NextSession 重构字段——均待取证登记（桥已落，重构字段待①定点）。

## 组外申请记录

- `resources/history/mod.rs` EditType tag 修正：**中央批准**（PROGRESS.md 仲裁表第 14 条）。
- `systems/save/snapshot.rs` + `systems/save/load_game.rs`（域采集器 + Apply 桥）：**中央批准**
  （仲裁表第 15 条）。
