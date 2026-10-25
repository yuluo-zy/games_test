# tg-gpu-selector — B 发射面原函数清单（骨架提取）

来源：`skeleton-enriched/B/crates/tg-gpu-selector/src/{lib,gpu_selection,vulkan_config}.rs.enriched.json.gz`。

## lib.rs

| 原函数 | RVA | VA | callers |
|---|---|---|---|
| `run_gpu_selection_gui` | 6491680 (0x631160) | 0x140631160 | 2 |
| `run_vulkan_configuration_gui` | 6492000 (0x631220) | 0x140631220 | 1 |

（其余 iced_graphics damage = 依赖。）

## gpu_selection.rs

| 原函数 | RVA | VA | callers |
|---|---|---|---|
| `device_driver_advice` | 35382128 (0x21C02F0) | 0x1421C02F0 | 2 |
| `render_error_to_ui` | 35403808 (0x21C3E20) | 0x1421C3E20 | 2 |
| `localize_graphics_init_error` | 35406864 (0x21C46D0) | 0x1421C46D0 | 1 |
| `get_vendor_driver_update_links` | 35403472 (0x21C3D50) | 0x1421C3D50 | 0 |
| `impl$2::update` | 35370816 (0x21BCFC0) | 0x1421BCFC0 | 1 |
| `impl$2::view` | 35371648 (0x21BD200) | 0x1421BD200 | 3 |
| `impl$3::active`/`hovered` | 6481696/6481840 | 0x14062EF20/… | 0 |
| `impl$4::active` | 35406784 (0x21C46C0) | 0x1421C46C0 | 0 |
| `Controls::new` / `impl$0::program_outcome` / `impl$5::fmt` / view closures | — | 内联/无独立实例 | 0 |

## vulkan_config.rs

| 原函数 | RVA | VA | callers |
|---|---|---|---|
| `Controls::new` | 14628320 (0xDF3060) | 0x140DF3060 | 1 |
| `Controls::toggle_allowlist_item` | 14630160 (0xDF3890) | 0x140DF3890 | 1 |
| `impl$2::update` | 14630512 (0xDF39F0) | 0x140DF39F0 | 1 |
| `impl$2::view` | 14630944 (0xDF3B20) | 0x140DF3B20 | 3 |
| `impl$3::workarounds_view::closure$1` | 14638992 (0xDF5E90) | 0x140DF5E90 | 6 |
| `Controls::allowlist_view`/`workarounds_view`/defaults/clones | — | 内联 | 0 |

## 对照结论（当前 Rust coverage）

Rust `lib.rs`：`run_gpu_selection_gui`/`run_vulkan_configuration_gui`（+native probe）。
`gpu_selection.rs`：`Device`/`Vendor`/`DriverLink`/`DriverSupport`/`DriverAdvice`/`device_driver_advice`/
`GraphicsInitError{tag:u64}`/`GraphicsErrorView`/`localize_graphics_init_error`/`Device`/`SelectionReason`/`Controls`。
`vulkan_config.rs`：`AllowlistItem`/`Allowlist{policy_word:u32}`/`Workarounds{6 bools}`/`Layer`/`Configuration`/`Controls`。

库内函数名一一对应；待 fresh 反编译核对 device_driver_advice / render_error_to_ui /
localize_graphics_init_error / Controls 字段与分支语义。

## 关键数值/边界（差分核对点）

`GraphicsInitError` tag：0x0b/0x0c/0x12/0x14/0x15/0x16；`DriverSupport` 六态（Unsupported/
UnsupportedLegacyIntel/Emulated/UnknownVendor/known_good/Supported）；allowlist policy_word
u32 语义；6 workaround bool 位序与默认值；Vulkan allowlist 默认项；vendor driver update links。
