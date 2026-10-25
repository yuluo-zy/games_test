# COMPILE-GREEN — country-core 测试目标编译修复

> 角色：compile-green（country-core 测试目标编译修复 restorer）。
> 命令：`cargo check -p country-core --tests`。
> 范围：只做编译级最小改动（字段/路径/签名适配），不改业务逻辑、不删测试、
> 不触碰他 agent 正在编辑的文件（先 `git status` 查冲突）。
> 日期：2026-10-17。

## 结果

- **测试目标错误全部清零，`cargo check -p country-core --tests` 达 0 error**（lib / lib-test /
  集成测试三态全部通过，仅剩警告）。
- 初始现场：55 个 lib-test 错误 + 3 个 test 二进制编译失败（wall_authority / wall_highlight /
  core_foundation），全部修复。
- 期间并行 agent 数次引入/修复 lib 错误（steam / ui / onboarding / renderloop 在途），均随其收口
  消失；最后一个阻塞项（onboarding.rs 枚举缺 `#[default]`）由本角色以最小编译级改动解除。
- 最终验证（2026-10-17，cargo 自带锁）：`Finished dev profile`，0 error。

---

## 1. 错误→修复清单（测试目标）

### 1.1 `system_roof` crate/module 找不到（E0433，wall_authority.rs:9）

- 现象：集成测试 `use system_roof::surface::Curve2;`，workspace 无该 crate。
- 修复：改引 country-core 内部路径 `country_core::resources::roofs::geometry::surface::Curve2`。
- 文件：`crates/country-core/tests/wall_authority.rs`。

### 1.2 `CursorTerrainRaycast` 缺 `delta`/`speed` 字段（E0063，2 处）

- 现象：地形重建给结构体加了 `delta: Vec3`、`speed: f32`（O 布局 32B），测试初始化未更新。
- 修复：补 `delta: Vec3::ZERO, speed: 0.0`。
- 文件：`crates/country-core/tests/wall_highlight.rs`、`crates/country-core/tests/core_foundation.rs`。

### 1.3 `GraphicsDevice`/`Device` 从 tuple 占位改为真实设备包装（E0423 ×46、E0609 ×2、E0599 ×1）

- 现象：rebuild-renderloop 把 `rhapsody_contract::GraphicsDevice(pub u64)` 改为
  `struct GraphicsDevice { inner: Arc<rhapsody::device::Device> }`（`inner` 私有），测试代码的
  `Device(1)` / `GraphicsDevice(1)` tuple 构造与 `self.0` 字段读取全部失效；`default()` 也不存在。
- 方案：新增共享测试占位设备 `render::test_device::TestDevice(pub u64)`（`#[cfg(test)]`），承载原
  tuple 序列号语义，并实现 4 个 render 层设备 trait（ssbo `rhapsody_boundary::GraphicsDevice`、
  `load_texture::boundary::TextureDevice`、`shader::boundary::ShaderBackend`、
  `asset_libraries::texture_library::TextureLibraryDevice`）。
  `rhapsody_contract::GraphicsDevice` 自身的 `#[cfg(test)]` trait 实现保留（供
  `texture_library.rs` 常开 `TextureLibraryDevice: TextureDevice` 超 trait 编译），`self.0` 改取固定 0
  （CPU 测试不再构造该类型，id 无消费）。
- 改动：
  - 新文件 `crates/country-core/src/render/test_device.rs`；`render/mod.rs` 增
    `#[cfg(test)] pub mod test_device;`。
  - 18 处构造点 `Device(1)`/`GraphicsDevice(1)`/`GraphicsDevice(n)` →
    `TestDevice(1)`/`TestDevice(n)`（或经 `as Device` 别名改 import 一行切换）：
    `asset_libraries/{color_library, placement_data_library, shader_library, ssbo_library}.rs`、
    `resources/render_ssbo.rs`、`systems/render_loop/compute_passes/{compute_garden, spawn_boats,
    bin_lights, reset_indirect_args, pass_kit/test_env}.rs`、
    `systems/render_loop/draw_passes/transparency_pass/surface_tests.rs`、
    `systems/render_loop/resources/critters.rs`、`render/{load_texture, shader,
    pass_builder/compute_pass_builder, pass_builder/pass_resource_access, ssbo}.rs`、
    `systems/garden/tests.rs`、`systems/terrain/tests.rs`。
  - `render/ssbo.rs:1078` `GraphicsDevice::default()` → `TestDevice::default()`。

### 1.4 `mod_key` `update_mod_key_state` 改 Bevy 系统签名（E0308 ×5，mod_key.rs 测试）

- 现象：函数签名由 `(&ModKeyConfig, &mut Modifiers)` 改为 `(Res<ModKeyConfig>, ResMut<Modifiers>)`。
- 修复：测试模块加 `apply_mod_key_state` 适配（临时 `World` + `Schedule::run` 走系统，再回写），
  5 个调用点改调该适配。
- 文件：`crates/country-core/src/systems/backend/input/mod_key.rs`。

### 1.5 `GameKeyBindings.bindings` 改 `[KeyBinding; 40]`（E0308，game_profile.rs 测试）

- 现象：测试 `bindings: vec![(1, 2)]` 与新的 40 槽固定数组不符。
- 修复：改 `[KeyBinding::default(); 40]` + 首槽 `KeyBinding { key: 1, value: 2 }`。
- 文件：`crates/country-core/src/game_profile.rs`。

### 1.6 `GameProfile` 结构体扩展（E0063，game_profile.rs 测试）

- 现象：save/域类型 agent 为 `GameProfile` 增补 20 个 hint/完成标志字段，测试初始化缺字段。
- 修复：测试加 `..GameProfile::default()`。
- 文件：`crates/country-core/src/game_profile.rs`。

### 1.7 `ReplayBucket` 新增 `timing: Vec<CommandTiming>`（E0063 ×8，history 测试）

- 现象：history agent 给 `ReplayBucket` 加时序头字段（`commands`/`timing` 长度一一对应），
  8 处测试初始化缺字段。
- 修复：补 `timing: vec![Default::default(); commands.len()]`（空命令桶为 `Vec::new()`）。
- 文件：`systems/history/{finish_replay, replay_stairs, replay_wall}.rs`、
  `systems/ui_clutter/tests.rs`。

---

## 2. 方法说明

- 每条只改编译错误本身：字段/路径/签名适配，未动任何业务函数体。
- 未删除任何测试；未新增测试。
- `TestDevice` 为共享测试支撑（`#[cfg(test)]`），不进入生产路径。
- 冲突规避：开工先 `git status` 核对在途文件；rhapsody crate、`rhapsody_contract.rs` 结构体、
  render 域生产文件均未改（仅新增 `render/test_device.rs` 与 `render/mod.rs` 一行注册）。

## 3. 并行 agent 在途干扰（观察，均已随其收口解决）

工作期间多个 agent 并行编辑，`lib` 反复出现其各自的在途编译错误，均在其收口后消失：

| 错误 | 位置 | 归属 agent | 处置 |
|---|---|---|---|
| E0433/E0277 rhapsody 在途（`reads_previous` 补字段、`pass_api` Command 非穷尽、`result_cache` 借用） | `crates/rhapsody/src/**` | rebuild-renderloop | 不触碰，agent 自修 |
| E0428 `steam` 定义重复 | `resources/mod.rs:64` | rebuild-steam | 不触碰，agent 自修 |
| E0432 `bevy_ecs::world::Command` | `systems/ui/signifiers/signifier_factory.rs:20` | rebuild-ui | 不触碰，agent 自修 |
| E0277 `GraphicsDevice` 无 `Debug` | `render/device.rs:19` | rebuild-renderloop | 不触碰，agent 自修 |
| E0277 `[u8; 55]: Default` | `systems/ui_clutter/autoclutter_state.rs:92` | rebuild-ui | 不触碰，agent 自修 |
| E0277 `SteamInput: FromWorld` / E0596 借用 | `systems/steam/mod.rs` | rebuild-steam | 不触碰，agent 自修 |
| E0665 枚举无 `#[default]` | `resources/onboarding.rs:92` | 域类型/onboarding | 本角色补 `#[default] Tutorial = 0`（最后阻塞项） |

最终现场：上述 7 个 lib 错误全部消失，`cargo check -p country-core --tests` 0 error。
