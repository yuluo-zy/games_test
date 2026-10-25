# groupD/misc — decorator 杂项族恢复记录（D4 restorer tg_d4）

日期：2026-10-16。范围：`crates/systems/decorator` 杂项族 + lib.rs 统一注册。
证据落盘：本目录 `decomp/`（22 业务 VA fresh RE-MCP + 12 辅助函数反编译 +
RNE 常量/字符串直读）。
状态：**源码已落码可编译**（`cargo check -p system-decorator` 仅剩 D1 在途
4 错误：decorator_visual/cottage_wall_window、utils/visual_entity_builder
Transform-Bundle、utils/mod.rs RaycastMeshData::new_convex —— 均归 D1）。
验收：待 reviewer 独立核对（签名面/参数面重点：音频三系统、window_lights、
cull_oob 族）。

## 0. 模块清单与业务函数覆盖

| 文件 | 业务函数 | VA | 状态 |
|---|---|---|---|
| audio/op_confirm_audio.rs | op_confirm_audio / place_sound | 0x140822530 / 0x140822cf0 | O 结构（type→事件分派 + 帧队列） |
| audio/op_preview_audio.rs | op_preview_audio / move_sound(内联) | 0x140822e00 | O 结构（type→kind + 木地板特殊路径） |
| audio/op_stairs_audio.rs | op_stairs_audio | 0x14081d9b0 | O 结构（163KB decomp 结构扫描；I 细节） |
| window_lights.rs | window_lights | 0x1407b5980 | O 结构（表维护 + 开灯区间常量） |
| remove_ivy_under_windows.rs | remove_ivy_under_windows | 0x140814860 | O 结构（窗→RemoveIvyCmd 盒） |
| dormer_holes.rs | upload_dormer_holes_to_gpu + 2 内联 | 0x1407b1d40 | O 结构（15×60 网格 + cells 表 + render handoff） |
| collision_detection.rs | intersect_wall_decorators ×2 单态 + project_to_wall_space(内联) | 0x141fe5900 / 0x141fe5c20 | O 结构（泛型宿主 + 谓词基元） |
| cull_oob_decorators.rs | is_decorator_location_valid / auto_allow / cull_oob / cull_stair_impl×2 / cull_stair×2 / should_snap_to_corner / check_if_dormer_ridge | 0x14080f470 / 0x14080d3d0 / 0x14080d540 / 0x1412b6c30 / 0x1412b6420 / 0x14080ec10 / 0x14080f040 / 0x140811900 / 0x140811440 | O 结构 |
| chimney_smoke_anim_thickness.rs | chimney_smoke_anim_thickness | 0x140818df0 | O 结构（厚度表 + rng 目标 + [0,1] 钳制） |
| move_decorators_following_anchors.rs | move_decorators_following_anchors + 2 内联 | 0x14081c790 | O 结构（重锚 + 已删墙） |
| utils/mod.rs | subdivide_wall_decorator(内联) | — | O（with_offset_unbounded 行 26） |
| lib.rs | snapping_direction / try_merge_decorators(内联) + add_decorator_systems | — | O 结构 + 注册链 |

utils/visual_entity_builder（WindowVisualEntityBuilder + build ×2 单态 0x1412a9b30/
0x1412ab270）已由 D1 落码（D1 消费契约），本 restorer 复核其证据
（fresh decomp 54KB 同构双单态）。

## 1. 关键 O 事实（机器码/常量，可直接复用）

- **DecoratorType 值域**（core O）：0x38/0x39/0x3a 窗族、0x3b、0x3c..0x3e 楼梯顶、
  0x3f 角吸附。音频 kind 分派 = (type-0x38) 跳表（place_sound 0x142a910c0、
  preview 0x142a91168/0x142a91198）。
- **开灯区间**（window_lights）：t∈[0.44(olden)|0.5097, 0.7199) → lights_on
  （0x142a83cb8=0.44 / 0x142a83cbc=0.5097 / 0x142a80d44=0.7199；`is_olden`
  0x140a9fa70 选界）。
- **烟柱区间**（chimney_smoke）：t∈[0.7199, 0.9) → 烟激活；厚度钳 [0,1.0]
  （0x142925904=1.0）；目标系数 3.0/4.0（0x1429cd078/0x142934694）；rng =
  `utils::rng::init_rng` + fastrand f32（0x140c96240/0x140caf380，种子来源 I）。
- **dormer_holes 网格**：单元索引 = row*15+col（15 列，计数缓冲 900 u32）；
  坐标换算 `floor((x + 65.0) / 8.6667)`（0x142a83814=65.0 / 0x142a83818=8.6667；
  0x1429a49bc=2.147e9 钳制）；cells[225][50] × 0x30B。
- **snap_to_corner 阈值**：`dist < threshold*0.5 + (halftimbered ? 3.0 : 0.46)`
  （0x1429cd068=3.0 / 0x1429a0764=0.46 / 0x142925870=0.5）。
- **panic/断言串**（RNE 原文 + 地址）：
  - `assertion failed: decorator_archivist.is_decorator_cull_allowed(locator.decorator)`
    @0x142a8e8e3 / 0x142c2be40（cull_stair_attached_decorators_impl，源
    cull_oob_decorators.rs）。
  - `internal error: entered unreachable code` @0x142a90fae（place_sound case 3）。
  - `assertion failed: (0.0..=1.0).contains(&t)` @0x142c236fd（utils curve_u.rs
    CurveUBoundless clamp；move_decorators/碰撞投影共用）。
  - `assertion failed: *range.end() < points.len()` @0x142c28c90。
  - puffin profile_scope 名（0x142a83d70/0x142a905f8/0x142a90030/0x142a8e683/
    0x142a8e63b/0x142a83afc 等 = `system_decorator::<fn>::f`，puffin-0.14.1
    源路径 `/home/h3/.cargo/registry/.../puffin-0.14.1/src/lib.rs` 证实）。
  - `"oob backup"` @0x142a8e950（cull_oob 归档标签，archive_storage_decorators
    0x1412b2a70 的 &str 实参）。

## 2. 跨依赖登记（中央协调）

### core 侧缺口（等待 core 恢复/接入）
1. **`country_core::resources::window_lights`**（GlobalWindowLightsControl /
   WindowLight）仅胶水（章节 window_lights.rs.md）；本族在 decorator
   `window_lights` 模块本地承载类型，待 core 恢复后迁移。
2. **`PublicWalls::roof_component`**（0x140a923a0）：op_preview_audio 本地等价
   `roof_entity + Query<&Roof>` 组合；core 恢复后替换。
3. **`DormerHoleSsbo`**（core `resources::walls::dormers`）：D1 在 utils 模块
   承载（dummy 0x141286660 / gothic / cottage）；SSBO 名产生字符串 0x142c215f8
   待取证。计数缓冲（u32[900]）与 cells 区为同一缓冲双写（0x140beefa0 ×2）；
   本面 cells 经 InstanceBuffer<DormerHoleSsbo> 写，计数区布局挂账。
4. **`WorldRaster::as_float_grid`**（op_stairs_audio 依赖）：core `paths.rs`
   `PathRastersResources::as_float_grid` 存在但本系统采样面未接线（I 挂账）。
5. **`time_continuous_lerp`**（window_lights/chimney 共用，原 utils crate
   0x140c904e0）：本地实现于 window_lights 模块；F = 0x142923ec0（std 导入
   thunk）身份 U（以 sin 振荡语义承载并登记）。
6. **`DecoratorSubtypeLibrary` 查表**（0x14125cf90）：remove_ivy / move_decorators
   / collide 谓词的 dims 查表；Rust 侧用 core `DecoratorSubtypeLibrary::
   dims_wall_space(key)` 与记录自带 subtype 字节承载，未命中 fmt panic
   （0x142c21c80/0x142c21c90）语义保留。

### D1-D3 侧（同 crate 模块）
1. **apply_decorator_op**（D3）：cull_oob 移除链调用
   `apply_decorator_op(op, 0, dst, storage, Some(archivist), None, events)`；
   events（`DeferredDecoratorOpEvents` 共享队列）在 core 非 Resource → 本面以
   默认空队列承载（I 挂账，D3 接线后替换）。
2. **DecoratorBlueprints**（D1 blueprint.rs）：cull_oob 用 `map` 字段 + get/remove；
   dormer_holes 用 `map.iter()`（D1 未提供 `iter()` 方法 → 走 pub 字段）。
3. **decorator_visual_state**（D1/D3）：visual_entity_builder 消费
   add_decorator_visual_entity；dormer/cull 不直接依赖。
4. **DecoratorArchivist<C> 泛型**：cull 族系统需宿主 App 以具体 C 注册
   （`register_cull_oob_systems<C>`）；add_decorator_systems 只注册非泛型系统。
5. **gothic_dormer_profile_curve**（decorator_visual，D1）：check_if_dormer_ridge
   的轮廓曲线依赖（0x1407c7500）；本面变体门控承载，屋顶几何 I 挂账。

## 3. lib.rs 注册链（D4 统一）

- 模块声明：D4 追加 audio / chimney_smoke_anim_thickness / collision_detection /
  cull_oob_decorators / dormer_holes / move_decorators_following_anchors /
  remove_ivy_under_windows / window_lights（与 D1-D3 声明共存，未整体覆盖）。
- `add_decorator_systems(schedule)`：注册 audio×3 + window_lights +
  remove_ivy_under_windows + chimney_smoke_anim_thickness（D4 杂项族）。
- `register_cull_oob_systems<C>(schedule)`：cull 族 4 系统（泛型 C 具化由宿主
  App 完成，原 `tiny_glade::decorator_plugin::build` 0x140342c10 装配面）。
- `add_decorator_events_and_resources(world)`：init_resource ×4。
- `snapping_direction`（行 63 inline）/ `try_merge_decorators`（行 77 inline）：
  按 PDB 行号与实例宿主（merge_proposal / interaction_intent / collide 谓词）
  落码；签名 I（reviewer 按调用点核对）。

## 4. 待 reviewer 核对点（签名面/参数面）

1. 音频三系统参数面：decomp 各见第三个 ResMut（tick@+0x1c）——资源身份未定
   （挂账）；kind 值域（4/0x17/0x10/0x1e/0x20/0x23/8/0x19/6/0xc/0x24）语义由
   core decorator_audio kind→loop 表承载（章节 I 部分）。
2. window_lights 参数面：屋顶/墙遍历资源 + 昼夜循环值资源 + 收集资源身份未定；
   `GlobalWindowLightsControl` 字段（cycle_t @+8 / lights 表 / lights_on @+0x30）
   为本地承载。
3. cull_oob 族：`DecoratorArchivist<C>` 泛型参数；archivist 双表（0x30B 段）与
   special 桶映射；`is_decorator_location_valid` 跳表槽位（墙/楼梯/屋顶/地形）
   的墙有效性函数（行 707 内联）为 I 简化。
4. dormer_holes：洞记录载荷（蓝图 +0x0..+0x1c 区）以首个碰撞体 footprint 承载
   （I）；计数区双写布局。
5. move_decorators_following_anchors：非系统辅助（裸 WallId 参数）；调用点 =
   D3 decorator_on_* 族；曲线采样/附着高 lerp 链 I。
6. `intersect_wall_decorators` 谓词：merge（0x141fe5310）与 interaction
   （0x141fe50e0/141fe5530/141fe5720）闭包实参归 D1/D2；本文件只提供泛型宿主 +
   WallSpaceRegion 相交基元。

## 5. 编译状态

- **`cargo check -p system-decorator` 通过**（0 error，10 warning，exit 0；
  2026-10-16 晚终态）。warnings 均为 unused import/变量（core 与 D1-D3 在途
  文件残留 + 本族 1 处 `_graph` 占位）。
- workspace 注册已核：`Cargo.toml` members 含 `crates/systems/decorator`，
  `system-decorator = { path = "crates/systems/decorator" }`，Cargo.lock 已含
  system-decorator 包。
- 中间态注记：D1 在途错误（cottage_wall_window Transform-Bundle /
  visual_entity_builder / RaycastMeshData::new_convex）已随 D1 更新归零。
