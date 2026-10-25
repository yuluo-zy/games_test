# Roof 原指令次序与 sinf 反证

旧候选 SHA23a1dfe2…74cc3b 将调用错误地写成 cos。重新核发行 RNE 的 thunk 和 PE IAT 后证实 **0x142923fd0 实际跳转 sinf**，不是 cosf。本轮已修 work 候选，根任务已收到不能合旧候选。这个错误说明只核参数常量、旧 decomp 的 callee 显示名不够。

独立取证脚本 `../extract-roof-order.py` 读取完整 PDB 边界 initial map@141302d60/2000B、后处理常量及 import table。输出 `../roof-operation-constants.json` 保留 thunk `jmp [rip+0x86f1d2]` → IAT VA1431931a8 → `api-ms-win-crt-math-l1-1-0.dll!sinf`。

原 load_mesh_atlases 的完整 asm 位于 `../14129f400-full-rne.asm`。每个原顶点 x0=±0.5，z0=i/n−0.5：

1. `14129f7e0` 先保存原 x0 到 xmm12。
2. `7e6/7ec` 读 z0，乘 xmm6=PI bits40490fdb。
3. `7f0` 调原 sinf，`7f5` 乘 xmm7=0.5，`7f9` 写回 z1。
4. `7ff..81c` 从原 x0 的 sign bit 得到 ±1；unordered 支路只处理原值 NaN，生成输入恒±0.5。
5. `820` 对 **z1** 自加，`824` 平方；`828..83b` 得到 min(1,max(0,1−(2z1)^2))。
6. `83f/842` sqrt；`846` 乘原 x0 的符号；`84b` 最后乘0.5；`84f` 写回 x1。

正确投影：`z1=0.5*sin(PI*z0)`，`x1=0.5*sign(x0)*sqrt(clamp(1-(2*z1)^2,0,1))`。level0完全跳过该段。当前候选逐个保留上述有依赖的 f32 运算，不用 fma，不用cos，不用变换前 z0 计算横向半径。

测试候选 `original_mesh_atlas_startup.rs` 的固定 level1 点是 z=[−0.5,0,0.5] 且中心 x=±0.5，两端 x=0；level2 四分之一点为±sqrt(1/2)/2。这会直接否定旧cos候选及使用旧z的半径算法。Mac与MSVC sinf不作逐位相等声明，允许1e−6用于该有界样点；原 Windows 进程行为 oracle 尚未运行。

测试另通过真实发行RON的19配置/193项JSON与585609源索引、实际Atlas映射、索引偏移和AssetMeshLibrary稳定句柄；加19个原程序网格后24个非空subset/592419索引。后3族无顶点属性是原encodedVertexID行为，不生成假位置。

本agent未执行Cargo/GPU。根任务负责编译与实际运行测试，未通过前不把候选称为算法已验收。
