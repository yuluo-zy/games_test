# systems/wall/upload_instanced_wall_ssbo.rs

> 文件作用：实例化墙砖 SSBO 上载：AssetSsboLibrary::get_mut<BrickTransformSsbo>（缺失 unwrap panic）；逐 InstancedWall 脏区间（+0x108 compact 标志双路径）upload_to_storage_buffer；SparseInstanceBuffer::gpu_flush。GPU 前段。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::upload_instanced_wall_ssbo::gpu_flush_brick_buffers` | 0x14082c170 | 9 | upload_instanced_wall_ssbo（get_mut<BrickTransformSsbo> unwrap + 脏区间双路径 + gpu_flush） | O | wall\decomp\decomp-0x14082c170.txt |

## 实现方式（算法级）（O，decomp-0x14082c170）

**处理链**：3 参；param_2 = ResMut<AssetSsboLibrary>（+0x1c 回写）；param_3 = Res<InstancedWalls>。

**记录布局**（param_3 deref）：id **连续区间** `{start u32*@+0xf8, end@+0x100}`（非 cap-first Vec）；
**compact 标志 u8@+0x108**（双路径）；**frame 索引 @+0x110**；帧号 u32 = `*(param_3+0x14)`。
二级存储（param_3[1]）：每墙 0xa0B @`+0x100`（+0x58=计数、+0x50=脏索引表、+0x94=帧索引）、
每帧 0x48B @`+0x1a8`（+0x10=计数、+0x18=ptr、+0x38=帧槽指针数组）。

1. **get_mut<BrickTransformSsbo>**（FUN_1412c71b0）：None → panic_str（UNK_142a92100）+ trap。
2. **双路径**（@+0x108）：
   - 非 compact：逐 id：`*(storage+0x100)+id*0xa0` 项计数@+0x58==0 跳过；帧槽 idx=@+0x94；
     `src=*(*(0x48项+0x38)+frame*8)`、dst ptr（+0x10/+0x28 区）；遍历**脏索引表**（@+0x50，0x10 步长，
     u32 idx@+8）：`*(dst+idx*4)=frame`；`FUN_140858880(idx*0x30+src, ssbo)` 拷 **0x30B 砖**。
   - compact：逐 id：计数@0x48项+0x10==0 跳过；同帧槽取址；**顺序拷全部实例** `FUN_140858880(i*0x30+src, ssbo)`。
3. **gpu_flush**：`FUN_14161ab00(ssbo+0x68, …)`（SparseInstanceBuffer 域）+ puffin scope
   （0x142c2e9f0 len 0x50、0x142c2eb10 len 0x38）+ Commands FUN_140d79f90 +
   诊断 fmt panic `FUN_1428d9760(&UNK_142c2e7f0, 0x37, …)`。
