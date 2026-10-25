# 正式 bindless、资源与窗口领域验证

此处保存合并 FrameStorage/typed RecordingId/补齐 CPU 资源预算之前的可执行文件、构建源指纹和领域结果。**不覆盖随后源码，也不提升 M0–M9 为 PASS**；最终统一矩阵、长跑与性能对照仍需执行。

- `gpu-checks.json` / `gpu-repairs.json`：正式 shader ABI、逻辑 ABI 热重载、在飞帧池、设备丢失、计时、完整上传及 tiny 计算消费者。最初两个新 fixture 的失败保留：可选 uniform metadata 被强 unwrap；PGO 负载变化前未预热全部 worker。修复后的 ABI 3/3、frame pools 4/4 通过。tiny 导出缺失导致正确裁剪的初始失败在相邻 tiny-bindless-consumers 目录保留，明确 HOST_READ 导出后 10/10 通过。
- `shader-assets/results.json`：本机素材 361 包 / 575 stage 全部经正式转换并通过 spirv-val；`shader-assets-b/results.json`：B 证据目录 404 包 / 638 stage 同样通过。每份记录实际输入、输出和转换二进制 SHA256；二者来自不同素材集合，不混称一个版本。
- `window-domain-checks.json`：raster 热重载、temporal、Exclusive/Concurrent 三族、32 个大附件的别名 off/on，各 360 帧呈现和读回通过，关闭完成且保留 WSI 代际 0。初始验收正则误匹配无错误汇总 `validation errors: 0`，初始归档保留并明确重分类；正式脚本已加单词边界。
- 大附件 native 保留内存 off=576 MiB / 3 blocks，on=320 MiB / 2 blocks；未增加依赖，720 次共享 backing 分配/释放配平。完整原始数值见 summary.json，资源区间与 native allocation 分开。

所有运行均使用 Apple M3 / MoltenVK，Khronos 与同步校验开启。屏幕像素/键盘与 Windows 仍独立 BLOCKED，不以 GPU 读回冒充。
