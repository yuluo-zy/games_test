# 原 compute 变体配置恢复（2026-10-13）

已从原 B 发行机器指令、原 LLVM PDB 命名、原 PE x64 .pdata 范围及真实生产/调用点恢复全部 38 个非零 compute key 对应的宏 tuple。不是由文件 hash 猜宏名；宏名字通过立即数写入堆字符串，所以原静态字符串搜索漏掉了它们。通过实际恢复的 ShaderDefines::artifact_hash 复验 38/38，一共 14 个非零 key、8 个宏、6 组配置。

| 配置 | 原宏及取值 | 生产者/原调用证据 |
|---|---|---|
| DOF，5 个入口，10 资产 | PERM_quality=0/1 | 五 typed as_compute_pipeline；calc_dof 入参 byte 扩展为 i32；render_world0x1404ecce7 判断资源 discriminant==19 并 sete |
| Nani cull，9 个入口，18 资产 | PERM_oc_pass=0/1 | 九 typed as_compute_pipeline；gpu_generate_draw_lists0x140987e0a 将入参可选指针!=0置 bool，经0x14098830c/0x140988500/0x1409885b3传入生产者 |
| 灯光分桶，2 资产 | PERM_write=0/1 | 0x1409e7920 与0x1409e811f 原调用明确0/1 |
| Vermeer，2 资产 | PERM_photomode=0/1 | 0x1408a8fc9 读取入参 bool，作为 i32 传到生产者 |
| TGSR，4 资产 | PERM_wave32=0/1、PERM_sharpen=0/1，按此顺序 | unsigned wave size<=32 AND supported；ordered sharpen strength>0，NaN false；宏输入 self+0/+4 |
| Gauss，2 资产 | PERM_fmt="rgba32f"、PERM_ty=float4；或 PERM_fmt="r16"、PERM_ty=float | 原调用0x1408c8161/0x1408c842e；fmt 是包含两个双引号的字符串，fmt::Arguments 两个 pieces均指向0x142ab5688，len1 |

完整逐资产原文件 SHA、source、entry、defines 顺序、producer地址及证据文件见 original-compute-tuples.json。tuple-hash-verification.json 记录实际恢复的原 hash 库 SHA 和38项结果。verify-original-hash-library.log 保留执行输出。

原 RNE SHA256：f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03。持久工作库目标是已核身份的 B patched EXE cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323；每个 prepare_functions 请求均核原代码范围 SHA。24 个原生产/辅助函数 .pdata 全可核界，extract_original_producers.py 已实际复跑24/24，结果 extract-replay.log。asm/ 保存原 LLVM 指令，以及 SDK 请求已覆盖函数的反编译；void占位签名没有作为 ABI 使用。

复跑提取：D:/tools/reverse-engineering/venvs/reverse-engineering-assistant/Scripts/python.exe extract_original_producers.py。直接原库核 key：在 MSVC Rust 环境 rustc verify_original_tuple_hash.rs -o verify_original_tuple_hash.exe，然后执行该 exe。只依赖 std 与仓库实际 ShaderDefines 文件。

SDK 使用独立 worker shader-compute-gap-1013，入口 scripts/reverse_tools/reverse_operator.py；initial-job/prepare-job/caller-prepare-job/config-prepare-job 及 result 均保留。首次 node 子进程缺 APPDATA，补进程环境后又遇 seed owner 缺 USERNAME，真实失败日志保留，最终根正常环境启动 owned backend，随后本 agent 直接复用持久 SDK 成功；没有改 seed、原发行或原数据库 owner。

边界：本轮只修改本 evidence 目录，不修改源 shader 或构建实现。上轮38空宏 baseline 的成功编译不因此变成正确变体。非零 key tuple 已恢复，不证明恢复 HLSL 业务算法等价，也不验证原 DXC fork/版本。数字宏生产者接受 i32，不把发行0/1误写成所有可接受输入；DOF资源 discriminant19的 enum 名和 Nani可选指针具体类型语义仍未独立命名。hash0 compute 本轮未调查，重复 XOR 可抵消不能独证空 tuple。原游戏进程未运行。后续由根整合原 tuple 配置并对实际新编译输出做差分。

早期 tuple-hash-probe.log/tuple_hash.rs 属于调查诊断：首次裸 fmt 没有匹配原 key，旧日志还保留无原调用证据的 fmt 候选。它们不作为任何恢复结论。最终以包含原双引号的 verify_original_tuple_hash.rs、verify-original-hash-library.log 和逐项 tuple-hash-verification.json 为准。

运行时 agent 的独立互审已通过：../interfaces/compute-tgsr-peer-review.json 核原四 SPIR-V 的LocalSize及ControlBarrier；../interfaces/compute-caller-peer-review.json 用原函数push/sub/rbp偏移独立核DOF参数搬运，及Nani输入pointer布尔透传。未猜原UI enum或pointer语义名称。

Pipeline agent 正式复审通过：../build-entry/compute-peer-review.json 独立核38个原资产SHA、38个asm文本中的12750条指令hex逐条与原RNE一致，并核Gauss两个quote片段使用Display而非Debug。quality==19、oc可选指针!=0控制流也复审一致。
