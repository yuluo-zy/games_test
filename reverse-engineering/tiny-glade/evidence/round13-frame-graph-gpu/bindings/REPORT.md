# Shader binding → graph content range 候选

本专项仅修改候选，不改共享生产源码，不运行 Cargo。`candidate.patch` 由 root 统一集成。状态为 `implemented_unverified`；rustfmt 通过，新增回归尚未执行，不宣称真实 GPU 或完整游戏完成。T13 排除。

## 具体错误及修复

`raster_host::bindless_parameters` 从每个 native pass slot 的 `TextureViewDesc` 创建精确描述符；但 `submit_raster_pass` 原先对同一访问只登记 `AccessRange::Whole`。一个读取 mip 2 / layer 1 的真实图像描述符，被图错误解释为读取所有层和 mip：会拒绝未被该 shader 读取的未初始化子资源，也会把同 owner 不同 view 的依赖混在一起。

候选沿当前架构使用已存在的 `compute_view_range` 投影，将其提升为 crate 可见，不另造 range 或渲染后端。按原声明顺序一一匹配 native resource/code 与 pass slot，跳过构造时先登记的附件访问。每个 slot 仅匹配一次；保留 access code 与 synchronization group；具体视图范围验证失败、view 指向 buffer、绑定索引超出 u8 或绑定缺少访问都进入既有正式帧错误回调。原生的无绑定依赖（例如间接参数）保留自己的访问。

`raster_content_intents` 原先还按整个 resource ID 合并 active/inactive。候选改为按 pass slot 合并：同 slot 的 active alias 仍收紧为真实读取；不同 view 的内容意图由图按其实际范围求交。不会发布 initialized import，也不补清零、空资源或扩 heap。

## 原证与当前消费者

`original-evidence.json` 绑定原 RNE 与 patched B 的完整 SHA，4 个 PDB 完整函数范围逐字节一致，原码和 LLVM disassembly 一并保存：

- `DrawBuilder::bind_texture_to_draw_builder @ 0x1408ee800`：1992 B。原 `0x1408eec11` 登记访问后 `0x1408eec64` 调用 `Ref::bind_view`；写分支分别为 `0x1408eeb64`、`0x1408eebb7`。说明访问与带视图的绑定应一起传播。
- `Ref::bind_view @ 0x140bd6c00 / 0x140bd6c30`：各 47 B，完整搬运 0x3c B 视图、资源号置 +0x3c、读写标志置 +0x40。此处只取证消费者合同，不复刻旧 rhapsody ABI。
- `RasterPass::new_impl @ 0x140867970`：2560 B。与当前 `draw_stream::RasterPass::new` 一起核对附件访问先于 DrawBuilder 的绑定访问。
- 复用原证 `evidence/country-core-audit/B_a4_draw.md` 与实际当前 `draw_builder.rs::bind_rg_texture`。原 TextureBindingKey 按 view 去重，故宿主应保留 slot/view，不能按 owner ID 合并。

本轮 Windows 独立 MCP worker 在 Ghidra LaunchSupport 试图写 `java_home.save` 时被文件权限阻断；没有新反编译或 overlay。错误保存在 `mcp-backend-failure.log`，继续使用原 RNE 和已有身份绑定的 PDB 完整范围，不重试修工具。

## 必要验证与边界

新增 `raster_binding_range_tests` 三项：同 owner 不同 mip/layer 保留独立 slot；附件访问不吞 shader slot；真实 selected-subresource producer → GraphDefinition 可接受，而 Whole 或错误 layer 仍必须拒绝；跨 packet 的 active alias 收紧相同 slot；越界/missing-access/wrong-kind 失败保留。

现有 `original_surface_raster_intent_tests` 随 key 从 resource ID 改 slot 更新。测试只覆盖真实当前图定义内容验证；不把空 producer 或合成初始化作为成功条件。root 应执行 manifest 中的目标测试和非空帧 GPU 验收。

剩余明确开放：附件自身范围仍为 Whole（深度 layer 与 opaque `ColorViewWords` 需要另一个完整附件生产/视图/图投影闭包）；外部 SSBO 反射没有 readonly qualifier，保守 RW 本轮不凭名字改成只读；frame_world 的 typed borrowed ResourceView 身份及 compute callback 参数机制本次未发现足够原证支持的改动。完整非空 world 和 GPU 输出仍待中央验收。
