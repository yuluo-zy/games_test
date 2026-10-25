# resources/gpu_lut.rs

> 文件作用：**GPU LUT 生成（brdf_fg/water_noise 两条目 tracker + 计算 pass）。**

函数总计 11：业务 3 / 内联无独立符号 2 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| country_core::resources::gpu_lut::impl$0::build | 0x14090e080 | 25 | 0x14090e080：Vec::with_capacity(8)（条目 stride0x30={u64,u64,u64,self,vtable,rendered}）；条目1 vtable 0x142ad6290；非 demo(全局 0x143634d90==0)加条目2(vtable 0x142ad6260)→insert_resource<GpuLutTracker> | O | audio-misc\decomp\decomp-0x14090e080.txt |
| country_core::resources::gpu_lut::GpuLutTracker::render_luts | 0x14090e280 | 95 | 0x14090e280：逐条目[+0x28]==0→[+0x10]=0→vtable[+0x28]()→[+0x28]=1 | O | audio-misc\decomp\decomp-0x14090e280.txt |
| country_core::resources::gpu_lut::impl$3::render_luts::impl$0::compute_impl | 0x14090e310 | 112 | 0x14090e310：vtable[+0x18] 取 URI 入 Vec→FUN_141600570 装 400B ComputePass→memcpy→vtable[+0x18] 计算 | O | audio-misc\decomp\decomp-0x14090e310.txt |

## 实现方式（算法级）（O，DIGEST §2.10 + decomp-0x14090e080/0x14090e280/0x14090e310）

**GpuLut::build @0x14090e080**：`Vec::with_capacity(8)`（cap-first）；条目 stride 0x30 =
`{+0 u64, +8 u64, +0x10 u64, +0x18 self, +0x20 vtable, +0x28 rendered u8}`；条目 1 vtable
0x142ad6290；**非 demo**（进程全局 0x143634d90==0）再加条目 2（vtable 0x142ad6260）→
`insert_resource<GpuLutTracker>`。（O）
**render_luts @0x14090e280**：逐条目 `[+0x28]==0` → `[+0x10]=0` → `vtable[+0x28](...)` →
`[+0x28]=1`（惰性一次性渲染标记）。（O）
**compute_impl @0x14090e310**：`vtable[+0x18]` 取 URI 入 Vec → `FUN_141600570(3,self,vtable,…)`
装 400B ComputePass → memcpy → `vtable[+0x18]` 计算。（O）
- 0x143634d90 与 image_formation 档 2 `+0x3d` 共用同一进程级布尔（跨域 demo 开关）。O

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::gpu_lut::impl$1::compute`（行 63）— 查询/计算
- `country_core::resources::gpu_lut::GpuLutDesc::new`（行 80）— 构造

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项
