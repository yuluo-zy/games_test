# 第 11 轮 DEC 组交付：`crates/systems/decorator`（B 版）

> 实现 agent：DEC②；复核：DEC③（`evidence/round11/reviews/dec-review.md`，16:04 快照）。
> 目标 crate：`D:/game/tiny-glade/crates/systems/decorator`。契约：
> `evidence/round11/contracts/dec-decorator-CONTRACT.md`。
> 复核结论：NOT PASS（P0×4）。本文件按复核 P0 收口，逐条给修复位置与仍缺项。

---

## 0. P0 收口摘要

| P0 | 状态 | 修复位置 |
|---|---|---|
| 编译（E0614 等） | **PASS**（check ×2 + test 75/0，§5） | `decorator_visual/lantern.rs`、`lib.rs`、`ui_change_chimney_height.rs` |
| 三处必崩 | 已修 | `ui_place_decorator.rs:42-90`、`ui_change_chimney_height.rs:46-120`、`ui_systems/mod.rs:91-104` |
| 生命周期接线 | 已接（仅 `build()`） | `crates/tiny-glade/src/decorator_plugin.rs:21-50` + `systems/decorator/src/lib.rs:127-193` |
| 伪造默认值 | 已删除主要项（§3） | utils/load/cache/ivy/archive 等 |
| 62 空壳 | 仍约 44 项（§4），楼梯装配为硬 U | — |

---

## 1. 编译修复（P0-1）

1. **E0614** `decorator_visual/lantern.rs:258/263/268`：`iter_modified()` 已返回
   `(u32, &DecoratorInfo)`，旧代码 `collect(*id, info)` 对 `u32` 解引用。已改为
   `collect(id, info)`（三处）。
2. **E0283** `ui_systems/mod.rs`：泛型系统注册缺 turbofish。已为
   `ui_change_stair_height::<C>`、`ui_change_chimney_height::<C>` 指定类型。
3. **E0599** `lib.rs`：`.chain()/.after()` 需要 `IntoScheduleConfigs` 在作用域。
   已 `use bevy_ecs::schedule::IntoScheduleConfigs;`。
   `add_decorator_systems` / `register_cull_oob_systems` 改为接收 `&mut bevy_app::App`
   （原 `&mut Schedule` 无调用者）。
4. **E0502** `lantern.rs:273-282`：`lists.diffs` 不可变借用期间写 `lists.seq/output`。
   已 `mem::take` diffs 后再写并回填。
5. **E0614** `ui_change_chimney_height.rs`：`lookup_info` 改为返回
   `(DecoratorStorageAddr, &DecoratorInfo)`，删除 `*dst` 解引用。

### 1.1 三处必崩（P0-2）

- **空指针 IdGen**：`ui_place_decorator.rs` 新增
  `TransientDecoratorIdBase(Resource)`（`pub struct` @42），系统以
  `TransientDecoratorIdGen::new(&mut id_base.0, false)` 生成瞬时 id；
  删除 `std::ptr::null_mut()`。
- **None archivist**：`ui_change_chimney_height` 改泛型 `C`，新增
  `ResMut<DecoratorArchivist<C>>` 与 `Res<DeferredDecoratorOpEvents>`，释放相位传
  `Some(&mut *archivist)` / `&events`；删除 `None::<…>` 与 `&Default::default()`。
  同时修正 op 首字段 = 存储判别（0/1/2），不再写 wall id。
- **缺资源 init**：`ui_systems/mod.rs:91-104` 新增
  `ChimneyVisuals`、`ChimneyPreviewState`、`MoveDecoratorDeltaPositions`、
  `InsertPointState`、`PlaceDecoratorInteractionState`、`TransientDecoratorIdBase`
  的 `init_resource`；`lib.rs` 新增 `LanternLightLists`。

---

## 2. 生命周期接线（P0-3）

`crates/tiny-glade/src/decorator_plugin.rs::build(&mut App)`（中央授权，仅此函数）
现调用：

1. `system_decorator::add_decorator_events_and_resources(app.world_mut())`
2. `init_resource::<LiveDecoratorArchivist>()`（原 DecoratorPlugin 安装点）
3. `add_decorator_systems::<MaybePendingSnapshotId, CollisionPayload>(app)`
4. `register_cull_oob_systems::<CollisionPayload>(app)`
5. `ui_systems::register_ui_systems::<CollisionPayload>(app)`

顺序由 `lib.rs` 的 `.chain()/.after()` 保证：

```
derive_decorator_subtypes → cache_decorator_transforms
  → [clear_out_visual_entities → clear_out_wall_holes → audio×3
     → window_lights → remove_ivy_under_windows → chimney_smoke]
  → [decorator_record_history → create_or_restore_backup
     → handle_external_decorator_cmds]
  → replay_decorator（after history）
cull 族：auto_allow → cull_oob → cull_stair ×2（chain）
```

`restore_archived_decorators` **暂不注册**：原系统带历史游标守卫（阶段 6.3，
0x140817a40 步骤 2；尚未闭合），每帧无条件运行会用 archivist 内容反复覆盖
存储；待守卫取证后以 `run_if` 单独注册。

`system-decorator.workspace = true` 已加入 `crates/tiny-glade/Cargo.toml`（接线必需）。
**仍缺**：原 113KB `impl$0::build` 的完整 schedule/set/run_if 表；生成族
（`generate_*`/`instantiate_*`）与 `decorators_on_glade_unload` 未注册（§4）。

---

## 3. 删除伪造默认值/日志冒充（P0-4）

| 位置 | 旧行为 | 现行为 |
|---|---|---|
| `utils/mod.rs:122` | `new_cuboid_collider` → `Some(RaycastMeshData::default())` | `None`（显式 U；`new_cuboid` 在 core `native-physics` feature） |
| `utils/mod.rs:128` | `clutter_mesh_ty_data` → `0` | `Option<u8>` = `None`（显式 U） |
| `load_decorators.rs:84` | 读 bbx 后返回 default 网格 | `None`（显式 U，不再伪造几何） |
| `cache_decorator_transforms.rs:108/122` | 墙 `Vec3::ZERO`、屋顶 `Vec2::ZERO` | 均返回 `None`（U：公式未闭合），不再伪造变换 |
| `remove_ivy_under_windows.rs:75` | width/height = 1.0/3.0 | `DecoratorLibKey::new(ty,rank)` → `collision_mesh().dims_wall_space()`；未命中 panic（同原 fmt panic） |
| `apply_decorator_op.rs:168` | 非墙地址 `log::warn` + skip | 地形→`archivist.special1`、楼梯→`archivist.special2` 真实归档 |
| `ui_change_stair_height.rs` | 处理器 2/3 语义对调、空体 | tag2=屋顶/tag3=地形，机器码调用面 + 测试；0/1/4 = `unreachable!()` |
| `merge_proposal_*` | 本地重复 `try_merge_decorators`/`snapping_direction` | 已删除，统一 `lib.rs` |
| `ui_rotate_lantern.rs` | spurious `dst_addr` | 已删除 |

---

## 4. 62 空壳现状（复核口径）

- **R（真实实现）**：`blueprint::new`、`compare_light_lists`、`replay_decorator`
  （`ReplayCommand::Decorator` + `bucket.remove_command` + `cull_allow` take，含 3 测试）。
- **P（部分）**：`maintain_lantern_lights`（新旧 0x1cB 列表/分类/0x28B 输出已落；
  记录 +4..+0xc 与 GPU `update_buffer_bytes` U）、`record_to_history_edit`
  （48B 前导两个 1 未证明）、`ui_change_stair_height` 屋顶/地形处理器 + op。
- **U0（原作不可达）**：#56 `change_stair_height_wall`：跳表 idx0/idx3 =
  `0x1407ae26f` = `unreachable!()`（复核 §4.1 机器字节一致；契约
  `0x1407ade6f` 为 +0x400 笔误）。
- **D（删除/统一）**：#49、#51、#60、#62。
- **E（仍空壳）**：约 **44 项**，集中在：
  - 楼梯视觉 7（`stairs.rs` #18–24，仅 `derive_stair_node_params` 索引公式已修）；
  - 楼梯装配 16（`stairs_assemble.rs` #25–40；#29 `playermade_stairs_assemble`
    **硬 U**：4 个闭包 0x140284b0/0x140275b0/0x140286d0/0x14027840 无反编译，未写空体）；
  - 扶手 4（#41–44）；
  - 差分重装 3（#9–11）；
  - 老虎窗/阳台门/窗 8（#3–8、#14、#15）；
  - 旗帜/布绳 2（#12–13）；
  - `brick_source`、`find_best_*_merge_position`、split 处理器 2、
    旋转墙/楼梯处理器 2、`preview_ratio` 等。
  （`is_half_dormer` 已由同一 crate 的并发续跑在 17:55 以机器码分支 + 测试闭合，
  不再计入 E。）
- **本轮新闭合的“无迹象”项**：
  - `blueprint::clear_blueprints`（modified/removed + removed storages 桶删除）
    与 `copy_blueprints_to_prev`（prev 基线 clone）；
  - `restore_archived_decorators::overwrite_decorator_state_from_archivist`
    （归档条目解码 → 桶 insert → state==2 d2s insert/update，返回缺失=false）；
  - `dormer_holes::build_dormer_hole_grid`（计数/单元表 CPU 生产者，原计数被丢弃）。
- **仍缺的系统级生成族**：`instantiate_decorator_colliders`、
  `instantiate_blueprint_colliders`、`instantiate_blueprints`、`validate_blueprints`、
  `show_max_wall_y`（需要 RaycastWorld/collider upsert 与实体 seal 完整接线）。

---

## 5. 编译/测试（独立 target，2026-10-19 18:2x）

命令：

```
CARGO_TARGET_DIR=/d/game/tiny-glade/target-agent-dec cargo check -p system-decorator --offline
CARGO_TARGET_DIR=/d/game/tiny-glade/target-agent-dec cargo check -p tiny-glade --offline
CARGO_TARGET_DIR=/d/game/tiny-glade/target-agent-dec-final cargo test -p system-decorator --lib --offline
```

| 门 | 结果 | 日志 |
|---|---|---|
| `cargo check -p system-decorator` | **PASS（exit 0）**，5 warnings | `/tmp/dec-final-check2.log` |
| `cargo check -p tiny-glade` | **PASS（exit 0）**，7 warnings | `/tmp/tg-final.log`（18:16）/ `/tmp/tg-final2.log`（18:48） |
| `cargo test -p system-decorator --lib` | **PASS：75 passed / 0 failed** | `/tmp/dec-final-test4.log`（18:23） |

> 18:2x 启动的最终复跑（覆盖测试/注释改动）也已通过：`DEC_EXIT:0`、
> `TG_EXIT:0`（`target-agent-dec`；日志 `/tmp/dec-final-check2.log`、
> `/tmp/tg-final2.log`）。

- 共享 `target/` 被其它 agent 的 cargo 锁占用（“Blocking waiting for file
  lock”），以上均用独立 target。
- 编译期曾出现 `country-core` 工作树并发编辑导致的 3 处 E0308
  (`publication_fields::WallId` vs `walls::WallId`)；该编辑方修复后复跑通过。
- 测试期唯一并发文件 `derive_decorator_subtype.rs` 的测试面（E0425/E0382）由
  其 owner 在 17:55 的版本修正；本轮另修：
  - `decorator_on_move_shape` 测试期望从“旧锚+delta”改为机器码写面
    “xvy(elem+event)，覆盖 x/z、保留 y”（0x140811fc0 O）；
  - `replay_decorator` 测试补 `pop_empty_replay_bucket` 链并删除空桶后再读
    aux 的断言。

### 5.1 新增测试

- `apply_decorator_op.rs`：哨兵原地更新（u30/载荷同步、不 remove）、迁移路径
  （源 removed / 目标 insert / d2s）、非法 rank 判别 panic。
- `ui_change_stair_height.rs`：地形打包 xz、op 编码（u30/哨兵/core 载荷）、
  `sync_core_stair_extra` 保留边 id、地址判别。
- `backup.rs`：TrackedDecorators 往返、pending 门控 state 清除。
- `cull_oob_decorators.rs`：tag 2 屋顶槽、tag 3/4 恒无效、缓存缺失无效。
- `lantern.rs`：归并分类、0x28b 输出布局、0.45..0.72 门。
- `replay_decorator.rs`：cull_allow take、timed floor/帧推进。
- `window_lights.rs`：0.51/0.44 下界 + 0.72 严格上界。
- `cache_decorator_transforms.rs` / `decorator_visual/stairs.rs`：地形 xz、跳表索引。
- `blueprint.rs`：prev 基线 clone、modified/removed 蓝图清理。
- `dormer_holes.rs`：footprint→网格单元/计数、世界→网格坐标。
- `restore_archived_decorators.rs`：归档条目恢复与缺失条目 false 路径。

---

## 6. 跨 crate 缺口（本轮未越权）

1. `DecoratorStorage::apply_decorator_change` 只实现同址 remove/reinsert；正常
   源≠目标迁移与哨兵原地路径已在 DEC 侧按机器码实现，建议上收 core。
2. `DecoratorInfo::extra()` 编码（dst_payload/u18/u20）与原生 `+0x30` 布局的偏移
   偏离：DEC 侧双写 `u30` 与 core 载荷首 f32（`sync_core_stair_extra`）。
3. `RaycastMeshData::new_cuboid` 仅在 `country-core/native-physics`；
   system-decorator 未启用该 feature，`new_cuboid_collider`/`load_collision_as_bbx`
   保持显式 U。
4. `PlayermadeStairsAssemblyRequests`/`AdjustedStairEdge`/`StairSubSegment`/
   `OverhangSupportFinderCache`/`StairsLazySubscriber`/`SampledStairsPoints`/
   `DecoratorBlueprint` 完整 0x140B 布局（契约 §4.2 七项）仍缺。
