# 半木 SSBO 旧测试字面纠错

root 收束前必要旧 fixture 修复，TG2b6b4af01a4999b89e 保留 main-chain-frame-20261018 claim，worker 仅合法 delegated candidate。原 f507 PE 142c815de 21B 字面为 HalftimberColorIdSsbo，read_storage_buffer141630190/756B真实传该指针/长度，get_rg_buffer1412ca520/810B真实 ID=0x0cf3b7d1045fdc98。实际来源与完整绑定复用只读 color/grading 报告 e6d1433b。

现生产 StorageElement::NAME 和 draw 标签已同原值，只有 cfg(test) 一个旧 expect 和全复数注释陈旧。本候选仅改这两行，不改变 const、hash算法、注册、draw、断言方式或容差。原相邻 halftimber hash 测试在 root 上一轮已通过；本候选尚未 Cargo/运行测试。完整原证报告中也指出 source 旧注释把141630190误称 get_rg_buffer，本轮不扩大生产注释修订范围。
