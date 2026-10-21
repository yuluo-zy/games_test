# R32G32_UINT 混合 warning 的证据边界

## 原世界和宿主

- 当前 country-core systems/render_loop/render_world.rs 的 formats::GBUFFER=0x65，原注明 PC0x1404ec404；Vulkan 101 为 R32G32_UINT。
- RasterPassHost 原 default state=0x1ffc00d0（原 PC0x140be6750）。rhapsody raw/shader.rs blend_mode 读取位29..30，因此默认 blend_mode=0，get_vk_blend_mode 对模式0产生 blendEnable=false。
- 当前 ShaderHost 创建的实际管线 key.selector 来自每个 DrawPacket.state.0，并保留全部实际 pass color formats；没有把默认 blend1用于所有pass的代码证据。仅有默认值不能替代每个 packet 的实际输入审计。
- 真实 world30 日志 /private/tmp/tg-world-normalized-thirty-20261021.log 的75条 R32G32_UINT blending warning 全在frame0以前，不能推断每帧发生75次或绘制已错误。

## 官方 MoltenVK v1.4.2 分支

官方源： https://raw.githubusercontent.com/KhronosGroup/MoltenVK/v1.4.2/MoltenVK/MoltenVK/GPUObjects/MVKPipeline.mm
函数 MVKGraphicsPipeline::addFragmentOutputToPipeline，1800..1836行。

源先读取颜色格式能力 supportsBlend。分支条件为：

```cpp
if (mtlPixFmt && supportsBlend) { /* assigns pCA->blendEnable */ }
else if (mtlPixFmt && !supportsBlend) { /* reports warning */ }
```

第二分支不检查 pCA->blendEnable，直接报告混合已启用的文字。故该源码中合法关闭混合的整数附件也会走此 warning；单凭消息文字不能证明应用请求了混合。实际驱动为本机MoltenVK1.4.2，但本文不声称已将安装dylib逐指令对应此官方tag。

## 正确下一步

先在真实 ShaderHost 交给 compiler 的 descriptor 处采 shader identity、selector、每个 format 与 blendEnable（同一次 create 输入），并对照原 flags/表；或真实 Vulkan 验证层检查对应 VUID。仅采样不改固定状态、shader、绘制或格式。若均为blend0，此消息归为驱动误报；如果存在UINT+blend1，须逆向该实际packet状态来源修正，无通用按format屏蔽候选。
