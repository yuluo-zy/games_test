# RESTORE — unlock/nani（restorer-unlock-nani，2026-10-17）

> 角色：restorer-unlock-nani。目标：落库 `NaniBundle<T>` / `DrawableMeshBundle`，解锁
> decorator visual build / roof `switch_roof_type` / toolbar spawn 的类型依赖。
> 硬标准：逐字段 fresh 反编译，不发明；缺证据字段标 typed-gap。
> 工具：ghidra `--worker shared`（只读），本批 13 个函数 fresh 反编译；原始输出在本目录
> `job1-*.c` / `job5-*.c` / `job6-*.c`（`job1-result.json` 等为完整 MCP 结果）。

## 落地结果（1 组 2 类型）

| 类型 | 位置 | 状态 |
|---|---|---|
| `NaniBundle<T>` | `crates/country-core/src/components/drawable.rs` | **已落库**（`#[derive(Bundle)]`，0x48 B） |
| `DrawableMeshBundle` | 同上 | **已落库**（`#[derive(Bundle)]`，0x40 B） |

PDB 全名（`component_ids`/`write_components` 单态化，O）确认归属：
`country_core::components::drawable::{NaniBundle, DrawableMeshBundle}` —— 所以落在
`components/drawable.rs`，与 B 模块路径一致。

---

## §1 证据与判定

### 1.1 `NaniBundle<T>` 的组件集合（O）

`country_core::components::drawable::impl$13::component_ids<
  country_core::render::nani::instance_data::UberInstanceData, …NaniBundle<UberInstanceData>…>`
（0x14267a9e0，fresh）依次调用：

```
ComponentsRegistrator::register_component<components::drawable::NaniInstanceDesc>();
ComponentsRegistrator::register_component<utils::components::transform::Transform>();
ComponentsRegistrator::register_component<render::nani::gpu_world::InstanceDataMarker<UberInstanceData>>();
```

=> 字段**声明序**：`NaniInstanceDesc` → `Transform` → `InstanceDataMarker<T>`，恰好 3 个组件。
`register_info<NaniBundle<StoneFloorInstanceData>>`（0x142197c70）只调这一份 `component_ids` +
`BundleInfo::new`，无额外组件/required 组件。

### 1.2 `NaniBundle<T>` 内存布局（O）

`BundleInfo::write_components<NaniBundle<StoneFloorInstanceData>, …>`（0x141a07f20，fresh）：

- 第一段拷贝读 bundle `+0x30..+0x48`（4×u32 + u64 = 0x18 B）→ `NaniInstanceDesc`；
- 第二段拷贝读 bundle `+0x00..+0x30`（12×f32 = 0x30 B）→ `Transform`；
- 第三段长度 0（ZST）→ `InstanceDataMarker<T>`。

`WindowVisualEntityBuilder::build<Transient>`（0x1412ab270，fresh）insert 闭包写块（cloure env，
bundle 载荷自闭包 `+0x08` 起）逐字段复核同一布局：

```
bundle+0x00..0x30  Transform（rotation 4×f32 @0x00，translation 3×f32 @0x10，scale 3×f32 @0x1c，0x30B）
bundle+0x30..0x48  NaniInstanceDesc（tag/selector @0x00、payload(NameHash) @0x08、subset_kind @0x10）
bundle+0x48        InstanceDataMarker<T>（ZST）
```

结论：`repr(Rust)` 字段重排（`Transform` 对齐 16 排最前）。总大小 **0x48**，对齐 16。
因此**不能**给该结构加 `#[repr(C)]`（会把 desc 挪到 +0x00，与机器码不符）。

`NaniInstanceDesc` 现有实现（`render/nani/systems.rs`，0x18 B：`NaniMeshSelector` 16B +
`GpuWorldSubsetKind` 1B）与上述偏移一致（payload@0x08、kind@0x10）。原版类型路径是
`components::drawable::NaniInstanceDesc`（`register_component` 的 PDB 名，O），本批在
`drawable.rs` 以 `pub use crate::render::nani::systems::NaniInstanceDesc;` 重导出该路径，
未搬动定义（避免改他组文件）；类型同一，布局已核。

### 1.3 `DrawableMeshBundle`（O）

`country_core::components::drawable::impl$3::component_ids<…DrawableMeshBundle…>`
（0x14267c2a0，fresh）依次注册：

```
register_component<asset_libraries::Handle<render::mesh::Mesh>>();
register_component<asset_libraries::Handle<render::shader::RasterShaderProgram>>();
register_component<utils::components::transform::Transform>();
```

`BundleInfo::write_components<DrawableMeshBundle, …>`（0x141fe7aa0，fresh）读偏移：

```
bundle+0x00..0x30  Transform（12×f32）
bundle+0x30        Handle<Mesh>                （8 B）
bundle+0x38        Handle<RasterShaderProgram> （8 B）
```

总大小 **0x40**，对齐 16；无第 4 个字段。

**对旧合同（gap/rhapsody-resources §3）的更正**：早期把 insert 闭包里的一个 `bool=false`
（闭包 `+0x58`；billboard 变体 `+0x48`）算进 `DrawableMeshBundle`。fresh `component_ids` /
`write_components` 证明 bundle 只有 3 个组件/0x40 B，该字节属于闭包的命令/错误处理状态
（闭包尾部还有 `bevy_ecs::error::handler::panic` 指针），**不在 bundle 载荷内**。

### 1.4 `NaniBundle<T>` 的 12 类实例数据（O）

`WindowVisualEntityBuilder::build` 的 mesh 名分派（0x1412ab270 / 0x1412a9b30 同构）逐 case
点名（PDB 单态化）：Uber / Cloth / Smoke / BalconyDoor / LanternEmissive / SolidVertexColor /
GothicWindowBricks / SolidAlbedoMap / SolidVertexColorWindow / SolidVertexColorBricks /
SolidAlbedoMapEmissive / SolidVertexColorWindowGlass，共 12 类；默认分支
`Commands::spawn<DrawableMeshBundle>`（0x1412ab270 尾段，O）。其中
SolidAlbedoMap / SolidAlbedoMapEmissive 在 insert 元组里附带独立组件 `NaniSolidTexture`
（`insert::closure_env_0<tuple_<NaniBundle<SolidAlbedoMapInstanceData>, NaniSolidTexture>>`，O）。
`switch_roof_type`（0x1421ac4f0，fresh）插入 `NaniBundle<StoneFloorInstanceData>`、移除
`DrawableMeshBundle`（PDB 证据见同批 job1）。

### 1.5 typed-gap（未落库、不编造）

| 项 | 证据 | 缺口 / 解锁条件 |
|---|---|---|
| `NaniSolidTexture` 类型 | `components::drawable::impl$28`（register 0x140830940 / clone_behavior 0x140831030）；1 B enum，clone 行为 Custom（registry 已登记）；实例更新里按 `u8` 索引 9 项 u32 表（`instance_updates.rs`，O） | 变体**名称/顺序**无源码证据；12 类中仅 SolidAlbedoMap / SolidAlbedoMapEmissive 两个 insert 元组需要它。解锁条件：从其消费者（`nani_update_solid_albedo_map` 0xf/0x10）或 serde/反射名表取到变体名后再落。 |
| `NaniBundle<T>` 字段名 | 机器码只给布局 | 现用语义名 `desc/transform/marker`（U）；布局 O。 |
| `DrawableMeshBundle` 字段名 | 同上 | 现用 `mesh/shader/transform`（U）；布局 O。 |
| 原类型定义在 `drawable.rs` 的 `NaniInstanceDesc` | PDB 路径 O；现实现仍在 `render/nani/systems.rs` | 本批用重导出对齐路径；如需 type_name 级一致，由 render 组在后续整合中搬定义。 |

---

## §2 实现

`crates/country-core/src/components/drawable.rs` 新增：

```rust
pub use crate::render::nani::systems::NaniInstanceDesc;

#[derive(Bundle)]
pub struct NaniBundle<T: Send + Sync + 'static> {
    pub desc: NaniInstanceDesc,      // bundle+0x30（声明序第 1）
    pub transform: Transform,        // bundle+0x00（rustc 重排）
    pub marker: InstanceDataMarker<T>, // bundle+0x48（ZST）
}

#[derive(Bundle)]
pub struct DrawableMeshBundle {
    pub mesh: Handle<Mesh>,          // bundle+0x30
    pub shader: Handle<RasterShaderProgram>, // bundle+0x38
    pub transform: Transform,        // bundle+0x00
}
```

`T: Send + Sync + 'static`：`InstanceDataMarker<T>: Component` 的隐含约束（bevy 0.16 derive），
非业务逻辑；所有实例数据类型均为纯数据，满足该约束。

新增 2 个布局测试：
`nani_bundle_layout_matches_write_components`（0x00/0x30/0x48、0x48 B）与
`drawable_mesh_bundle_layout_matches_write_components`（0x00/0x30/0x38、0x40 B）。

未做（按红线）：未改 rhapsody、未改 render/nani/systems.rs、未改他组在途文件；宏胶水
（`Bundle` derive 的 component_ids/register_info/write_components）不重建，由 derive 生成。

---

## §3 验证

`cargo check -p country-core`：**通过**（exit 0，`Finished dev profile`；仅既有 warning）。
过程记录：本批首次运行时库内报 15 个错误，其中 12 个来自 `NaniBundle` 缺 `Send + Sync` 约束（已修）；
其余为他人在途文件的瞬时错误（`resources/prefab.rs` 的 `Aabb3: Default`、
`intersect_meshes.rs`/`save/*`/`RaycastWorld` 字段），并发轮次间被各自 owner 修复后全绿。
`components/drawable.rs` 全程无 error。

布局测试（`cargo test -p country-core --lib components::drawable`）：**未执行成功**——
构建目录锁 + 磁盘满（`ENOSPC`，`target/debug/.../build-script-build` 链接失败），属环境/并发
构建问题，非代码错误。测试代码已随模块落库，待磁盘恢复后重跑即可。

## 收口

**完成（本批范围）**：`NaniBundle<T>` / `DrawableMeshBundle` 已按 fresh 反编译落库，
布局/组件集合有机器码证据 + 布局测试；解锁 decorator visual build（12 类中的 10 类直接可用）、
roof `switch_roof_type`、toolbar spawn 的 bundle 依赖。`cargo check -p country-core` 通过。
**登记**：`NaniSolidTexture`（变体名缺证据，阻塞 2 类 albedo 分支）；`NaniInstanceDesc`
原模块路径由 render 组后续整合；布局测试待磁盘空间恢复后重跑。
**阻塞**：无。
