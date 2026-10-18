# 原 init_populate 前四项：墙砖真实存储

owner `frameinputs-20261018`，已合法领取 `TG-a738caa535351a7991`。原 B RNE/PDB 身份同主链；四个完整 native constructor 范围哈希见 `native-manifest.json`。本轮只恢复缺失的三个 canonical 元素/存储及明确首四项 prefix；不改已有墙几何业务，不修改 rhapsody 或 sparse 架构。

| 原site | 原canonical类型 | 元素/BUFFER_LEN | 存储/desc | None实际行为 |
|---|---|---|---|---|
| 140909eb7 | BrickTransformSsbo | 96B / 0x200000 | 当前真实 SparseInstanceBuffer；GPU0xc000000B，usage22/memory1/align4 | GPU无初始上传；CPU影子逐元素Default，surface三值-100；空分配/可用chunk表/active与dirty bitset各自按原初始化 |
| 140909ec7 | BrickDrawListSsbo | u32 / 0xe00000 | Typed40；0x3800000B，usage22/memory1/align4 | 不上传 |
| 140909ed7 | BrickPrevVisibilityBitmaskSsbo | u32 / 0x10000 | Typed40；0x40000B，usage22/memory1/align4 | 分配65536个word、清零后全上传 |
| 140909ee7 | BrickNextVisibilityBitmaskSsbo | u32 / 0x10000 | Typed40；0x40000B，usage22/memory1/align4 | 分配65536个word、清零后全上传 |

三个缺失类型的 NAME 是各自短名，已从各 create_buffer 调用读取精确 rdata 与长度，见 `native-name-values.json`。现 draw_shaders/ssbo_tags 的 BrickDrawListTag 仅有原类型名和哈希，不是可以注册的元素/存储，不能当作 canonical producer 已存在。新类型使用同一 NameHash/完整类型身份，现消费者不需复制第二份库状态。

## 原构造与当前 API 复用

`BrickDrawListSsbo::new_storage_buffer@141612a90` 完整531B：Some时 len%4、ptr%4、len<0x3800004；None直接传空指针和长度0；desc固定0x3800000、usage22、memory1、align4。恰好容量允许。当前 `TypedStorageBuffer::new_storage_buffer` 的固定分支已保有此同合同，可直接复用，不新造另一buffer层。

Prev constructor `141611000` 与 Next `141611df0` 各完整890B：Some时 len%4、ptr%4、len<0x40004；None时分配0x40000/align4，循环向全部内存写0，再以该数据指针/完整长度创建固定GPU缓冲并释放临时数据。当前 `TypedStorageBuffer::new(device,None)` 的固定多元素分支匹配真实零上传；Some分支则继续用现byte constructor作原对齐/容量检查。策略按这两个原类型显式选择，不能推广成所有固定缓冲都清零。

`BrickTransformSsbo` 原 sparse constructor `1412e5800` 完整1857B，与现有 canonical SparseInstanceBuffer 的主要生产数据吻合：byte_map长0x10001，最后值32；CPU影子0xc000000B，所有96B记录的surface@64/68/72为-100、其余初值0；free列表65536个倒序chunk；active/dirty各0x400个64bitword，均零。调用具体 TypedStorageBuffer 构造 `141618890`。本轮保留现有复杂 sparse 算法，不以填空标记取代，不声称已对整个 sparse 分配/回收重新验收。

## 接口与边界

新增 canonical `BrickDrawListSsbo(pub u32)`、`BrickPrevVisibilityBitmaskSsbo(pub u32)`、`BrickNextVisibilityBitmaskSsbo(pub u32)`，各自 impl library/graph SsboType。新增显式有界 `populate_brick_ssbo_library(lib,device)` 按原顺序调用上述首四项；它不是全72项启动。

完整 producer 应接原全表后续 NormalDebug/MouseDepth/staging 等项，且沿现真实 GraphicsDeviceResource/BufferHost 调用，不另建host或库。已有 wall CPU→sparse GPU 生产者保持原文件内容；只有缺失存储声明附加。Parent 统一 Cargo/GPU，本 agent 不运行。source快照/哈希与候选见本 workdir manifest。
