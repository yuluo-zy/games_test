# systems/backend/icc.rs

> 文件作用：**窗口 ICC 色彩配置。`IccProfilePlugin::build` 注册调度（add_systems 路径）；`update_window_icc_profile` 系统监听显示器 ICC 变更 → lcms2-5.6.0 `cmsOpenProfileFromMemTHR` 读配置文件字节 → `generate_icc_lut`（内联）对具名颜色（terrain_1/terrain_2/wall_moss/wall_bricks/terrain_highlight/night_tint）生成 LUT 写入 ColorLibrarySSBO。**

函数总计 9：业务 2 / 内联无独立符号 1 / 派生胶水 6

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::icc::inner::impl$0::build` | 0x140a58440 | 41 | 薄跳板：`mov rcx,rdx; jmp 0x140b65a80`（尾调用）。实现在 0x140b65a80：`App::add_systems` 类注册（按 TypeId/name-hash 查 `app+0x38` 的 Schedules；找不到 panic `'bevy_ecs::schedule::schedule::SchedulesRequested resource ... does not exist in the World'` 0x142af2cc0）。 | O | 机器码 rne-read --hex 0x140a58440；decomp/decomp-0x140b65a80.txt |
| `country_core::systems::backend::icc::inner::update_window_icc_profile` | 0x14093d830 | 49 | Bevy 系统：取 ICC 变更；lcms2 `func_0x000140bd5310`（`cmsOpenProfileFromMemTHR`）+ `func_0x000140bd5370`；`FUN_1416a71f0` 生成 LUT 写入 `ActiveIccProfile`（`lVar15+0x10`）；`(int)colorspace != 0xb` panic；日志 `'Failed creating an ICC LUT: '`（0x142ab2130）；`generate_icc_lut` 对具名颜色填 SSBO。 | O | decomp/decomp-0x14093d830.txt；rne-read --unks（lcms2-5.6.0 profile.rs 0x142ab1708 / transform.rs 0x142ab17b0、具名颜色 0x142ab2158、`'Failed creating an ICC LUT: '` 0x142ab2130） |

## 实现方式（算法级）

**build @0x140a58440（O）**：2 指令尾跳板（`mov rcx,rdx; jmp 0x140b65a80`），实现在 0x140b65a80 = App 调度注册：按 TypeId/name-hash 查 `app+0x38` 的 Schedules；缺失 → panic `'bevy_ecs::schedule::schedule::SchedulesRequested resource ... does not exist in the World'`（0x142af2cc0）。（O）

**update_window_icc_profile @0x14093d830（系统，O）**：
1. 取 ICC 变更事件；lcms2-5.6.0 `cmsOpenProfileFromMemTHR`（func_0x000140bd5310）+ transform（func_0x000140bd5370）读显示器配置字节。（O）
2. `FUN_1416a71f0` 生成 LUT → 写 `ActiveIccProfile` 资源（+0x10）。（O）
3. `(int)colorspace != 0xb` → panic；失败 → log `'Failed creating an ICC LUT: '`（0x142ab2130）。（O）
4. `generate_icc_lut`（内联）对六具名颜色 `terrain_1/terrain_2/wall_moss/wall_bricks/terrain_highlight/night_tint`（0x142ab2158）填 ColorLibrarySSBO；缺失 → panic `' entry is missing from ColorLibrarySSBO'`。（I，内联）

**数据结构（I）**：`ActiveIccProfile` LUT 区 +0x10；ColorLibrarySSBO 为 LUT 落点。（I）

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::backend::icc::inner::generate_icc_lut`（行 94）— **I**（内联，并入 update_window_icc_profile）：对 terrain_1/terrain_2/wall_moss/wall_bricks/terrain_highlight/night_tint 六个具名颜色，经 lcms2 变换生成 LUT 项写入 ColorLibrarySSBO；`' entry is missing from ColorLibrarySSBO'`（0x142ab2158 串段）为缺失校验。

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 6 项（default 为 ActiveIccProfile 资源默认值；closure 为 update_window_icc_profile / generate_icc_lut 的闭包）

## 备注

- 依赖确认：**lcms2 5.6.0**（registry `/home/h3/.cargo/registry/.../lcms2-5.6.0/src/{profile,transform}.rs`），与 utils crate 同 fork 路径。
- icc 与 configwatch 的 SSBO/ColorLibrary 重载共同构成色彩管线热更新面（跨域线索）。
