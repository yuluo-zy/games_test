# RESTORE — gap/rhapsody-resources（restorer-gap-rhapsody-resources，2026-10-17）

> 角色：restorer-gap-rhapsody-resources（gap 组，深度 gap 中依赖 rhapsody/渲染的类型）。
> 范围：① rhapsody 回调注册表（maintain_dormer_tiles GPU 退役 `remove_entry<u64,Box<dyn FnOnce>>` 面）；
> ② RoofTileSsbo 的 SparseSsbo 面（gpu_flush_buffers 阻塞）；③ NaniBundle/DrawableMeshBundle 结构；
> ④ InsertTreeCmd 0x48 布局。
> 硬标准：每项 fresh 反编译 → 真实布局/语义；在途文件 → 合同 + 登记，不硬写；可落部分只在非在途文件。
> 工具：ghidra `--worker shared` fresh 反编译（本会话 10 函数，证据见本目录 `*.c`）。
> 冲突回避为主：rhapsody 正被他组大重构（REFACTOR_PROGRESS.md 全部里程碑 IN_PROGRESS），
> history 组持有 tree_edit.rs，render 组在途 roof_tiles_gravity.rs / ssbo_library.rs / render_loop 族。

## 判定总表

| 条目 | 判定 | 说明 |
|---|---|---|
| rhapsody 回调注册表（`remove_entry<u64, Box<dyn FnOnce(*mut c_void, bool) + Send>>`） | **合同** | rhapsody 在途（`crates/rhapsody` src 由重构组持有，禁止修改）；rhapsody_contract.rs 亦在途（M）。出合同（§1）。 |
| RoofTileSsbo SparseSsbo 面（gpu_flush_buffers） | **合同** | RoofTileSsbo 定义在 `roof_tiles_gravity.rs`（render 组在途 M）；现 `ssbo_carrier!` 已实现 `SsboType::Storage=TypedStorageBuffer`，补 SparseSsbo 面 = 同类型重复 impl（编译冲突）→ 必须由持有方改。出合同（§2）。 |
| NaniBundle / DrawableMeshBundle 结构 | **合同（布局）** | 消费方 decorator visual build / switch_roof_type / signifier_factory 在途；字段布局部分观测、部分 U。出布局合同（§3），不落地近似体。 |
| InsertTreeCmd 0x48 布局 | **合同 + 登记** | tree_edit.rs 由 history 组在途持有（M），未触碰。布局已由 reviewer 核过，本会话 fresh 复核写块。出合同 + 登记（§4）。 |

**本批实现 0 项**：四个落点全部在途或布局未定，无可安全落地代码。下列均为合同，供持有方照落。

---

## §1 合同 — rhapsody 回调注册表（maintain_dormer_tiles GPU 退役面）

### 证据（本会话 fresh，0x1407ce920）

`system_decorator::decorator_visual::cottage_dormers::maintain_dormer_tiles` 每瓦主循环：

```
hash_one<RandomState, &GlobalId>();                      // key = u64（GlobalId 的哈希）
hashbrown::raw::RawTable::remove_entry<
   (u64, Box<dyn FnOnce(*mut c_void, bool) -> () + Send>)  // tuple 载荷
>();
if (removed != 0) {
    drop_in_place<SparseInstanceBufferAllocation<RoofTileSsbo>>();  // 条件 drop 返回的 Box
}
```

### 合同要点

1. **注册表形态**（O）：`hashbrown::raw::RawTable<(u64, Box<dyn FnOnce(*mut c_void, bool) -> () + Send>)>`。
   key 类型 `u64`，由 `hash_one<RandomState, &GlobalId>` 对游戏侧 `GlobalId`（`country_core::systems::ui::GlobalId`）哈希得到。
   值类型 `Box<dyn FnOnce(*mut c_void, bool) + Send>`——GPU 退役回调，参数 `(*mut c_void, bool)`，
   返回 `()`（`Output=tuple_<>`）。
2. **消费语义**：每瓦（dormer tile 记录，stride 0x18）写 GlobalId u32（`[tile-builder+0x1c]` → `[+0x10]` 数组，游标同步）后，
   `remove_entry` 该 GlobalId 的注册项；**返回的 Box 非空则 drop**——Box 内承载
   `SparseInstanceBufferAllocation<RoofTileSsbo>`，drop 即把句柄归还 `SparseInstanceBuffer::waste`
   （见 sparse_instance_buffer.rs `SparseInstanceBufferAllocation<T>::drop`，0x1405de840）。
3. **游标写序**（reviewer-wave3-toolbar-decorator 已核）：循环前写 `cursor = total - lenB' - lenA'`，每瓦 `+1`，
   循环后末值 = `total`（不是 `total-(lenA'+lenB')`）。
4. **归属**：注册表本体属 rhapsody（在途重构）。游戏侧只需以下契约：
   - 注册/注销 API 按 u64 key；
   - `remove_entry(u64) -> Option<Box<dyn FnOnce(*mut c_void, bool) + Send>>` 返回所有权（供条件 drop）；
   - maintain_dormer_tiles 重建为 bevy 系统后，把每瓦的退役回调注册到该表（注册点=GPU 退役时机，
     见总账 §6 integration-registration 中 maintain_dormer_tiles 待注册条目）。
5. **落地位置（待 rhapsody 侧）**：`crates/rhapsody`（重构组）；游戏侧 `rhapsody_contract.rs`（在途 M，未触碰）——
   重构成型后按本合同核对。

---

## §2 合同 — RoofTileSsbo SparseSsbo 面 + gpu_flush_buffers

### 证据（本会话 fresh，0x140de50a0；系统签名 O）

```
bevy_ecs::system::function_system::FunctionSystem<
  fn(Res<country_core::render::GraphicsDeviceResource>,
     ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)>
```

### 逻辑（全可观测，约 5 行）

1. `tick = ssbo.tick@+0x5b0; ssbo.tick += 1`（u32 帧计数）。
2. `gpu` present 检查（缺 → `expect_failed`）。
3. 双资源槽校验：`idx1=[gpu+8]`、`idx2=[gpu+0x10]`，均须 < `ssbo.count@+0x218` 且
   `table@+0x210[idx] != 0` 且 `entries@+0x1e0 + 0x20 + ~idx*0x60`（stride 0x60 条目）非空；
   否则 panic（串：`"country_core::asset_libraries::ssbo_library::AssetSsboLibrary"` / 缺资源串）。
4. 通过后：写 tick 到 `entry + 0x5c`；`AssetSsboLibrary::get_mut<RoofTileSsbo>()`（None → `unwrap_failed`）；
   `GraphicsDeviceResource::deref()` → `SparseInstanceBuffer::<RoofTileSsbo>::gpu_flush()`；
   写 tick 到 `gpu + 0x150`；返回。

### 合同要点

1. **RoofTileSsbo 的 SparseSsbo 面**（O，sparse_instance_buffer.rs 文档交叉引用）：
   - `unsafe impl SparseSsbo for RoofTileSsbo { const CHUNK_COUNT: usize = 0x2000; }`（RoofTile=0x2000，
     BUFFER_LEN = 0x2000*32 = 0x40000，与影子数组长度吻合）；
   - `impl SsboType for RoofTileSsbo { type Storage = SparseInstanceBuffer<Self>; }`——替换现
     `ssbo_carrier!` 的 `TypedStorageBuffer<RoofTileSsbo>`（render 组已登记"存储形态与原版不符"）。
   - **阻塞**：`RoofTileSsbo` 定义在 `roof_tiles_gravity.rs`（render 组在途），且现 `SsboType` impl 已存在；
     补 SparseSsbo 面必须由持有方在同文件改（本批不动）。
2. **GraphicsDeviceResource 缺字段**（device.rs，coregap-1 已落、未提交）：本函数读取 `+8/+0x10`（两个资源槽索引）
   与 `+0x150`（tick 写面）、`+0xb8`（panic 缓冲）——现 device.rs 只持 `device` + `_pass_cache_gap`，未建模。
   落地 gpu_flush_buffers 时需补槽位（登记 U，不冒充）。
3. **落地位置**：`gpu_flush_buffers` 系统 → `systems/render_loop/mod.rs`（render 组在途）；Ssbo 面 → `roof_tiles_gravity.rs`。
   本批均不写。

---

## §3 合同（布局）— NaniBundle<T> / DrawableMeshBundle

### 证据（本会话 fresh）

- `WindowVisualEntityBuilder::build<Transient>`（0x1412ab270）/`<Persistent>`（0x1412a9b30）：
  mesh 名分派 → `Commands::spawn<NaniBundle<{Uber,Cloth,Smoke,BalconyDoor,LanternEmissive,SolidVertexColor,
  GothicWindowBricks,SolidAlbedoMap,SolidVertexColorWindow,SolidVertexColorBricks,SolidAlbedoMapEmissive,
  SolidVertexColorWindowGlass}InstanceData>>`（可配 `(NaniBundle<T>, NaniSolidTexture)` 元组）。
  默认分支：`AssetMeshLibrary::get_handle_by_name` + `AssetShaderLibrary::get_raster_handle_by_name`
  （None → `expect_failed`）→ `Commands::spawn<DrawableMeshBundle>`。
- `switch_roof_type`（0x1421ac4f0，既有 evidence）：insert `NaniBundle<StoneFloorInstanceData>` / remove
  `DrawableMeshBundle` / `AssetMeshLibrary::try_get_handle_by_name` / remove `Handle<Vao>` /
  `TransientMeshRegistry::deferred_delete_mesh`。
- `add_decorator_visual_entity` impl$5（0x140811c30，本会话 fresh）：向
  `HashMap<DecoratorId, Vec<DisplayChildAutoClutterInfo>>` 插入，Vec 条目 **8B**（`+idx*8 = u64 载荷`）。
- `DrawableMeshBundle::build`（0x14082bd70）：纯尾调用共享 Bundle build（无自身逻辑）。
- `startup_quad` 证据：`spawn_empty().insert(bundle)`，bundle 含 `Transform(IDENTITY)` + 网格句柄 + shader 句柄 +
  `DrawableMeshBundle` 内一个 `bool=false`（stars/clouds `+0x58`、billboard `+0x48`）。

### 布局要点（观测值，逐条标注）

**`NaniBundle<T>`**（bevy Bundle，泛型于实例数据）：
- 序列化/closure-env（0x50B，0x1412ab270 插入闭包写块）：
  `+0x00` 命令 fn 指针；`+0x08..0x18` 4×f32（builder `+0xc0/+0xc4/+0xc8/+0xcc` 位置/缩放）；
  `+0x18` 2×f32（`+0xd0` 打包旋转）；`+0x20` f32（`+0xd8` 旋转 z）；
  `+0x24` u64（`+0xdc` mesh 名/句柄）；`+0x2c` u32（`+0xe4` rank/tag）；`+0x30` u64（`+0xe8`）；
  `+0x38` 0；`+0x40` u64；`+0x48` tag 字节（SolidVertexColorWindowGlass=0x10、SolidAlbedoMapEmissive=5）。
- 实例数据泛型参数已在 `render/nani/instance_data.rs` 落库（16B 族：Uber/SolidVertexColor*/Smoke/…）。

**`DrawableMeshBundle`**：
- 构造 = mesh handle + raster shader handle + Transform + bool（`+0x58`/billboard `+0x48`）。
- `Handle<Vao>` 是 `AssetMeshLibrary::get_handle_by_name` 的句柄类型；shader 经
  `AssetShaderLibrary::get_raster_handle_by_name -> Option<Handle<RasterShaderProgram>>`。
- 字段精确布局 **U 待证**（mesh/shader 句柄相对偏移、bool 语义 U）；消费方在途
  （decorator visual / switch_roof_type / signifier_factory build GAP），落库由消费组按本合同 + fresh 取证落实。

**`DisplayChildAutoClutterInfo`**（impl$5 附带合同）：8B u64；表 `HashMap<DecoratorId, Vec<Self>>` 归
`decorator_visual_state`（在途）。

---

## §4 合同 + 登记 — InsertTreeCmd 0x48 布局

### 证据（本会话 fresh，0x142269970 place_tree 写块 + 0x142267430 process_tree_cmds 读面）

- **写块**（stride 0x48，Events 缓冲 data@+0x28 / len@+0x30 / count@+0x40）：
  - `+0x00` = name `Arc<str>` {ptr, len}（16B，len=sVar10）；`+0x08` = name len；
  - `+0x10` = position（Vec3 面；consumer 读 `+0x18` = z）；
  - `+0x20` = scale（f32，`lerp(0.5,0.9,rand)`）；`+0x28` = {variant, rotation}（variant 由 hash 派生、
    rotation `lerp(0,360,rand)`）；`+0x30` = flags；`+0x38` = id（`TreeStorage::next_index`）；
    `+0x40` = 值来自缓冲 `count@+0x40`（逐插自增；reviewer 核作 height）。
- **读面**（process_tree_cmds Insert 分支）：`+0x18`（位置 z）、`+0x38`（树 id）被读取以索引
  TreeStorage/墙体数据。
- **布局契约（reviewer-wave3-tg-tr / reviewer-coregap-1 已核，本会话复核一致）**：

```
stride 0x48 = {
  +0x00 name: Arc<str>,          // {ptr@+0, len@+8}，16B
  +0x10 position: Vec3,          // x@+0x10 y@+0x14 z@+0x18
  +0x20 scale: f32,
  +0x28 { variant, rotation },
  +0x30 flags,
  +0x38 id: u64,
  +0x40 height,
}
```

### 登记

- tree_edit.rs 的 `InsertTreeCmd` 现为共享语义型 `{id, position, scale, prefab_name}`（0x28 量级）——
  **与真实 0x48 载荷不符**（缺 Arc<str> name 布局、variant/rotation/flags/height）。
- **history 组持有 tree_edit.rs（在途 M），本批未触碰**。照落时：name 改 `Arc<str>` 承载，补
  variant/rotation/flags/height 字段，stride 0x48；process_tree_cmds Insert / place_tree / replay_tree 按此重建。
- 写者：place_tree（0x142269970）、replay_tree（0x141bc34f0）；读者：process_tree_cmds（0x142267430）。

---

## 冲突跳过分列（在途文件均未触碰）

| 落点文件 | 在途原因 | 本批动作 |
|---|---|---|
| `crates/rhapsody/src/*` | rhapsody 重构组（REFACTOR_PROGRESS 全 IN_PROGRESS） | 合同 §1 |
| `src/rhapsody_contract.rs` / `src/rhapsody_types.rs` | 在途（M，D 组接线） | 合同 §1，未改 |
| `src/render/device.rs` | 未提交（coregap-1 新落） | 合同 §2 缺字段登记，未改 |
| `src/systems/render_loop/compute_passes/roof_tiles_gravity.rs` | render 组在途（M） | 合同 §2，未改 |
| `src/asset_libraries/ssbo_library.rs` | render 组在途（M） | 合同 §2，未改 |
| `src/systems/render_loop/mod.rs` | render 组在途（M） | gpu_flush_buffers 落点，未写 |
| `src/resources/history/tree_edit.rs` | history 组持有（M） | 合同 §4 + 登记，未改 |
| `src/resources/walls/decorator_storage.rs` 等 decorator 族 | decorator 组在途（M） | 合同 §3 附带，未改 |

## 本批实现 0 / 合同 4 / 冲突跳过 4

实现 0（无安全落点）；合同 4（§1–§4）；冲突跳过 4（rhapsody 回调注册表 / RoofTileSsbo SparseSsbo 面 /
NaniBundle/DrawableMeshBundle 落库 / InsertTreeCmd 0x48 落库——全部因在途或布局未定跳过硬写，只出合同）。

## cargo check

`cargo check -p country-core`：**通过**（13 条既有 `non_snake_case` warning，均在
`src/systems/save/snapshot.rs`，非本批引入）。未改 src，故本批未引入新编译影响；他组在途改动已在各自轮次自检。
