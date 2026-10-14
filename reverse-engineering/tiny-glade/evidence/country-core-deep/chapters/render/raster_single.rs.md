# render/raster_single.rs

> 文件作用：**待推断**（由章节 agent 按函数名+证据填写）

函数总计 3：业务 3 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…raster_single_object<1,ColorAttachment[1],final_post::closure_env$1>` | 0x1409ce780 | 17 | 渲染/GPU | O：raster_single_object 泛型实例（final_post 单物体光栅化；raster 单例 pass 包装）【render-loop 域已覆盖 final_post】 | render-loop\decomp（final_post 族） |
| `…raster_single_object<1,ColorAttachment[1],resolve_to_swapchain::closure_env$0>` | 0x1409d00a0 | 17 | 渲染/GPU | O：同上（resolve_to_swapchain 变体；渲染图单例光栅化） | render-loop\decomp |
| `…raster_single_object<1,ColorAttachment[1],mei_pass::closure_env$0>` | 0x1409cdec0 | 17 | 渲染/GPU | O：同上（mei_pass 变体；ui_2d_pass 域单物体光栅化） | render-loop\decomp |
