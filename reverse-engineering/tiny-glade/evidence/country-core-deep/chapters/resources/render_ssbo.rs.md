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

> **5 处 typed-gap 载体的真实元素形态（render-gap §2.2，重建 HLSL O）**：
> `DofAutofocusSsbo`=`DofFocus`{focus0/1,lock_bias0/1 f32,lock_pos_ws float3,is_locked u32,interp_lock_pos_ws float3,use_tilt_shift u32,interp_lock_bias0/1 f32,focus_scale f32,pad2}=64 B（`inc/coc.hlsl:12`）；
> `DofFocusReadbackSsbo`=`DofFocusReadback`{lock_pos_ws float3,lock_reponse u32}=16 B（`inc/coc.hlsl:28`，成员名原拼写 `lock_reponse`）；
> `ShadowMaskHistogramSsbo`=u32[256]（`RWStructuredBuffer<uint>`，`InterlockedAdd` clamp 0..255）；
> `ShadowMaskMeanSsbo`=f32（`RWByteAddressBuffer Load<float>(0)`）；
> `WallColorIdSsbo`=NAME "WallColorIds"、SsboId 0xaf42315d263aa22a（O，`bind_resources.rs:214`）。
> 均可补真实元素类型 + StorageElement；NAME/BUFFER_LEN 的机器码核对留待各 SSBO owner（U）。
