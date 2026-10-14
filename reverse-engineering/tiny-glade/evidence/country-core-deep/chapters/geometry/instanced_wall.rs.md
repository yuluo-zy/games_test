# geometry/instanced_wall.rs

> 文件作用：**砖实例墙的几何载体与 GPU 上传**（MODULE_GAP 判 0% 覆盖，本节补齐）。`BrickSourceId`（墙体砖 vs 楼梯瞬态砖）、`InstancedWall::update`（换缓冲）、`upload_to_storage_buffer`（砖变换→SparseInstanceBuffer，stride 0x60）。布局 O 见各函数。

函数总计 4：业务 4 / 派生胶水 0。

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `BrickSourceId::from_wall_id` | 0x140858750 | 24 | 砖源 id 构造 | O：`mov eax,ecx; lea ecx,[rax-0x186a0(=100000)]; cmp ecx,0xb; ja …` —— **判别式枚举编码**：wall_id∈[100000,100011] 为楼梯瞬态边（TransientStairEdge=100000+idx），否则 Wall；`bt/ca` 置判别位后返回（Ghidra NotFound，手解） | geometry-gap\decomp（手解 0x140858750） |
| `BrickSourceId::with_transient_stair_edge_id` | 0x140858780 | 29 | 瞬态楼梯边 id 构造 | O：`mov edx,ecx; shl edx,4; or eax,edx; or eax,0x40; ret` —— **把 edge_id 编码进判别位 + 0x40 位**（区分瞬态边家族） | geometry-gap\decomp（手解 0x140858780） |
| `InstancedWall::update` | 0x140858830 | 34 | 换缓冲 | O：若旧数据非空（+8 有 len）先释放（func_0x140613c20 拷贝/归还，stride 0x60）；再把新 Vec{ptr@0,len@8,cap@0x10} 拷入 self+0x8/+0x10、len→+0x18 —— **整体换入新实例缓冲**（cap-first Vec 三字段 + 0x18=len） | geometry-gap\decomp\decomp-0x140858830.txt |
| `InstancedWall::upload_to_storage_buffer` | 0x140858880 | 41 | 砖变换上传 | O：读 self+0x8（len）负数 panic；调 SparseInstanceBuffer 族（FUN_140c81c70/0x140c81e10/0x1412e4450/0x1412e4760），panic 串 `Out of bricks.`（0x142a94b90）；对每实例 0x60B 写入（位置/旋转/缩放/可见位等，字段 U）；来源 `geometry/instanced_wall.rs`，缓冲 `render/sparse_instance_buffer.rs` | geometry-gap\decomp\decomp-0x140858880.txt |

## 内联/无独立符号函数

无。

## 关键结论

- **BrickSourceId 判别编码**：Wall=0x0/0x40 判别位，墙 id 在 [100000,100011] 映射为瞬态楼梯边（TransientStairEdge=100000+idx），`with_transient_stair_edge_id` 置 0x40 位——reconstruction 现有 `enum BrickSourceId { Wall(u64), TransientStairEdge(u64) }` 语义对，但**编码含 +0x40 位与 100000 偏移**未建模。
- `InstancedWall` 本体 = cap-first Vec{ptr@8,len@0x10(+0x18),cap@0x0?}，元素 stride **0x60=96B**（BrickTransform 24×f32 对应）。`update` 换入新缓冲；`upload_to_storage_buffer` 写 SparseInstanceBuffer 且可 panic `Out of bricks.`。
