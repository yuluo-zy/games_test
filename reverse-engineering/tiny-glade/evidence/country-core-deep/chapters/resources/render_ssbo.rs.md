# resources/render_ssbo.rs

> 文件作用：**帧 pass 的 SSBO 载体集中地（原版分散在 render::lantern、compute_mouse_depth_value、compute_passes、indirect_params；wp5 归并此处再导出）。LanternSsbo 等库侧类型已按机器码补齐。**

函数总计 0：业务 0 / 内联无独立符号 0 / 派生胶水 0（纯类型/StorageElement 实现）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `LanternSsbo` (类型) | (数据型) | — | 灯笼 SSBO：{position f32×3, intensity_color_id u32=half16 强度\|color_id<<16}，16B；容量 0x8000；SsboId=xxh3_64("LanternSsbo")=0x8b2da1ef1bff9619 | O | reconstruction/country-core/src/resources/render_ssbo.rs（机器码注释） |
| `MouseDepthValueSsbo` (类型) | (数据型) | — | 鼠标深度值 SSBO（StorageElement+库侧 SsboType 已补） | O | 同文件 |
| `SpatialResolveOffsetsSsbo` (再导出) | (数据型) | — | 空间解析偏移 SSBO（pub use resources::render） | O | 同文件 |
| `indirect_params` (再导出) | (数据型) | — | 四个间接缓冲（AtomicCounter/DrawIndirect/DispatchIndirect/DrawIndexedIndirect）= pub use indirect_params::* | O | 同文件 |
