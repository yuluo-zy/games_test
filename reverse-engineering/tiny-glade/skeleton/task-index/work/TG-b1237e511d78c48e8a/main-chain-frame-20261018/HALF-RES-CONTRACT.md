# 半分辨率深度/模板视图合同

范围为 B 的 `extract_half_res_depth_normals@0x1408f6180`，PDB 完整函数 1850 B；共享 Ghidra 首段仅 1346 B，因此完整 RNE 指令另存。RNE SHA-256 为 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，逐函数指纹见 `half-res-rne-manifest.json`；只读 patched B 查询见 `half-res-original.json`。没有修改数据库或 rhapsody。

## 已观察到的真实行为

- 输入为现有 GBuffer 的 gbuffer/depth 两个图资源；深度尺寸逐轴向上除以 2。生产 0x64 的 half-res depth 与 0x26 的 half-res normals。
- 深度输入在 `0x1408f637b` 以 access 0x19 绑定默认视图。
- `0x1408f63a4` 调用 TextureViewDesc::new；`0x1408f63ba` 设置 r8d=4；`0x1408f63c0` 调用 aspect_mask；`0x1408f6421` 将该视图连同同一个深度资源绑定为 stencil_tex，仍是 access 0x19。
- 随后写 depth_output_tex、读 gbuffer_input_tex、写 normals_output_tex，按半尺寸 dispatch。拥有者与访问顺序没有改变。

## 现实现差异与修正

旧手写实现把 stencil_tex 也绑定为默认视图，丢失真实模板 aspect。现有 ComputePass::bind_texture_with_view 已完整保存该参数，故直接修正游戏侧消费者，复用当前所有权、资源访问登记、编译与执行架构。没有新建旧版图形后端或复制原 ABI。

现有构图测试补充真实回归条件：同一深度资源必须产生 default 与 aspect 4 两个不同的只读 ImageView；原尺寸、资源顺序和 access 断言仍保留。执行结果另行记录；CPU 构图不等于此 shader 的真实 GPU/完整世界验收。

## 边界

本修改只关闭 W35 视图语义缺口。完整世界 FramePasses、SSBO 生产、深度资源真实产生、后续 SSGI 等消费者仍需各自接线与验证。A/C 没有据此升级状态。
