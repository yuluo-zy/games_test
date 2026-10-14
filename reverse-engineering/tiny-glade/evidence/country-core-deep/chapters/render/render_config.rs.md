# render/render_config.rs

> 文件作用：**画质/显示设置枚举与小函数（layer A1）：`FullscreenConfig` / `UpscalingPreset` / `ImageQualityPreset` / `UpscalingMode` / `MonitorMatch` + 序列化/克隆/From 实现。**

函数总计 9：业务 3 / 自定义 From 1 / 派生胶水 5

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `render_config::impl$7::from` | 0x140968c40 | — | **`From<&GraphicsDevice>` 类转换 → `{String uuid, String driver}`（48 B）**：String A=`format!("{}", physical_device)`（`rhapsody::physical_device::impl$3::fmt`，源 @+0x58），String B=`uuid.to_string()`（`uuid::fmt::impl$1::fmt`，源 @+0x78）；失败 → panic "a Display implementation returned an error unexpectedly" | O（2026-10-14 反编译） | render-gap\decomp\decomp-0x140968c40.txt |

> 语义（I）：产出 `LastKnownGraphicsDevice{uuid,driver}`（render_config 序列化串含 "LastKnownGraphicsDevice"/"uuid"/"driver"，见下）。
> reconstruction 未实现；建议补该 From。`FullscreenConfig/UpscalingPreset/ImageQualityPreset/UpscalingMode/MonitorMatch` 本体见 L0_NOTES（O）。

## 派生胶水（derive/宏生成；不计独立业务）

- `render_config serialize` 0x140039c10：serde `Serialize` for `MonitorMatch`（`serialize_struct("MonitorMatch",3)`，字段 `name/physical_position/physical_size`；串 0x14292b810 区含 "MonitorMatch"、全 `VideoSettings` 字段名、`FullscreenConfig{Best/ClosestMatch}`、`UpscalingPreset{UltraPerformance..UltraQuality}`、`LastKnownGraphicsDevice{uuid,driver}`、`VideoMode{resolution,bit_depth,refresh_rate_millihertz}`）。
- `render_config deserialize visit_str` 0x14003ae40：serde `Deserialize` visitor（`visit_str<ron::error::Error>`，8 字节窗口比较；Ghidra NotFound 已反汇编核实）。
- `render_config::impl$38::clone` 0x1404d7d50：derive(Clone) 展开（逐字段拷贝 ~0xc0 B 配置结构 + 2 字段专用 clone + `i64::MIN` niche）。
- `FullscreenConfig/UpscalingPreset` 等 fmt/eq/hash 其余胶水。

> 全部确认后标注**不补**（render-gap §2.1）。
