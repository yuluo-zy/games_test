# RE-ANALYSIS: resources/ivy + systems/ivy（生态组 · ivy）

> 会话：eco re-analyst（2026-10-15）。范围：`resources/ivy/*` 4 文件 + `systems/ivy/*` 9 文件。
> 依据：B patched EXE 机器码（RNE oracle）+ 既有 collision-eco decomp + 本轮新反编译与 rne-read。
> 记号：O=机器码证实 / I=有界推断 / U=未恢复；「旧结论 X → 新证据 Y」=本章修正。
> 不改 reconstruction 源码。

## 1. 函数清单表（本轮定点深挖项）

| 函数 | 地址 | 状态 | 行为一句话 |
|---|---|---|---|
| ivy_spawner | 0x14094cf90 | O（38KB 全解） | **生产者根**：双路径生成根株（光标旁/随机墙），株数上限≈40000，模式门 AppMode==1 |
| IvyStorage::spawn_ivy | 0x1408696f0 | O（全解） | perchance RNG 采样 u → 建新株 root 点（0x28B）入 HashMap，next_ivy_id@+0x40 自增 |
| Ivy::add_point | 0x140869460 | O（全解） | push 40B IvyPoint{id,pos,dir,flags}，point_id 计数@+0x58 |
| IvyLeafStorage::add | 0x1408fe280 | O（全解） | 按 ivy_id 查 HashMap<usize,Vec<IvyLeaf 104B>>，未命中建 Vec |
| closest_segment_index | 0x14094c800 | O（机器码复核） | 56B 段投影；**过滤语义纠正**（见 §4.1） |
| ivy_send_to_gpu_leaf | 0x1408c1580 | O（全解） | all_leaves collect → **IvyLeafSsbo(0x9e3cfa7f2993f406) + IvyFlowerSsbo(0x1d6de5e69ac50d2a)** 48B 元素 |
| ivy_send_to_gpu_segments | 0x140a9b4e0 | O（全解） | total_segment_count collect → **IvySegmentSsbo(0x2002d2fbdff3d80f)** 48B 元素 |
| add_ivy_systems / insert_ivy_resources / add_ivy_events | 0x140944070/0c0/280 | O | 22 系统注册；IvySpawnNoise(FastNoise 0.3/2.0)；3 事件 TypeId |

## 2. 数据布局（供 restorer 直接落 Rust）

- **IvyPoint = 0x28B（40B）**：`{ id u64@0, pos Vec3(x,y@8, z@0x10), dir Vec3@0x14, u32 flags@0x20=0 }`
  （add_point 0x140869460 + spawn_ivy root 点写入逐字段证实）。
- **Ivy = 可增长 40B 元素容器 + point_id 计数 @+0x58**（add_point id=[self+0x58]+1 后自增）。
- **IvyStorage = HashMap<usize,Ivy> 表体 @+0 + perchance RNG @+0x30 + next_ivy_id @+0x40**（spawn_ivy 插入路径证实）。**RNG = perchance-0.4.0**（range 断言 `end >= start` @0x142aa5710，/home/h3/.cargo/.../perchance-0.4.0/src/lib.rs）。
- **IvyLeaf = 0x68B（104B）**：`{u64@0, u64@8, ivy_id u64@0x10, u64@0x18, pos Vec3@0x20, 法向? Vec3@0x2c, 上向? Vec3@0x38, u32@0x44, u64=0@0x48, u32@0x50, u32@0x54, Vec3@0x58}`（IvyLeafStorage::add 逐字段证实）。GPU 打包后每叶 48B。
- **SSBO 键（O rne-read）**：IvyLeafSsbo=0x9e3cfa7f2993f406、IvyFlowerSsbo=0x1d6de5e69ac50d2a（与 compute_garden 章互证）、IvySegmentSsbo=0x2002d2fbdff3d80f。名表在 0x142a9d65d 区（`country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo` 等全名）。

## 3. ivy_spawner 算法（O，生产者根）

```
入口: mode(AppMode)字节==1 才运行；查询株数>39999+0x12 → return（上限≈40000）
双路径（uVar21 = IvyStorage 计数@param_5+0x120）:
  A. uVar21<3（株数<3）= try_ivy_root_next_to_cursor：
     光标位(Option<Vec3>) → 墙空间哈希格(RNG 采样，0.4923=32/65 世界→格换算，格 0x30B)
     → closest_segment_index(xz, segments, max_height=0.5f)；found!=1 → return
     → 段投影 t∈[0,1] → PublicWalls hashbrown 查墙(步长 0x2a8；curve_len<2 或墙高<=0 跳过)
     → 曲线采样 + func_0x000140c9cdc0 ×N（墙段几何采样）→ 根位 = 墙点 ± 0.34(0.68×0.5) 法向偏移
     → 距既有根校验：|Δpos|²<0.25（距 0.5）→ 过近 return
     → 垂直采样 (rand×1.5 − 0.75)/墙高 + 段高，钳 [0.000928482, 1.0]，非法 → 跳过该次
     → spawn 循环（≤3 次尝试）→ IvyStorage::spawn_ivy
  B. uVar21>=3 = try_random_ivy_root：逐 PublicWalls 随机 u∈[0,1] → 墙点采样 → spawn_ivy
IvyStorage::spawn_ivy(0x1408696f0):
  参数 (self, root_pos, dir, 插值端点, u∈[start,end), ...)；perchance range + f32
  → 建 root 点 0x28B(id=0) → FUN_141527820 插 HashMap → next_ivy_id@+0x40 自增
  断言 end>=start（perchance RNG）
```

**常量表（O rne-read）**：0.5 缩放 @0x142925870/8e0/e4、32/65=0.4923@0x142925874、0.000928@0x142960690、1.5@0x1429258d0、−0.75@0x142ab3220、0.68/−0.68 法向 @0x142ab31e0..f0 / 0x142ab3200..10、距阈值 0.25@0x142925950 / 2.56@0x142ab3214、尝试上限 3@0x142ab3218、i32::MAX@0x1429a49bc。

## 4. 修正记录（旧结论 → 新证据）

### 4.1 closest_segment_index 双过滤（★ 重要纠错）

**旧结论（collision-eco ALGORITHMS §4.2）**：「过滤 1 = h0≤max 或 h1≤max → 跳过（保留两端>max）；两过滤在 t∈[0,1] 下互斥 —— 待调用方数值裁决（U）」。

**新证据（机器码字节级复核 0x14094c8b0..0x14094c960）**：
- 过滤 1 = `ucomiss h0,max; jbe 进体` + `ucomiss h1,max; ja 0x14094c95e（跳过）` → **仅 h0>max 且 h1>max 跳过；h0≤max 或 h1≤max 均进体**（旧 decompiler 的 `h0<=max||h1<=max→break` 为误读）。
- 过滤 2 = 插值高 `h1·t+h0·(1−t) > max → 跳过`（ucomiss;ja 0x14094c95e）。
- **两过滤自洽**：语义 = 只保留生长投影点处墙高 ≤ max_ivy_grow_height 的段；穿过 max 边界的段按 t 裁。无互斥。
- 调用参数序（ivy_spawner 调用点证实）：`(out, xz, 0, segments_ptr, count, max_height)`。

### 4.2 其他修正

- ivy RNG 识别：章节未注 → **perchance-0.4.0**（range 断言串路径证实）。
- send_to_gpu_leaf 章节「104B 叶实例参数打包」→ 精确化：CPU 叶 104B，**GPU 打包 48B**，双 SSBO（Leaf+Flower）。
- IvyStorage 布局补全：HashMap@+0 + RNG@+0x30 + next_ivy_id@+0x40；非 FrozenMap。

## 5. 生产者→GPU 验收链（供 restorer 全链接线）

```
ivy_spawner(0x14094cf90) → spawn_ivy(0x1408696f0) → IvyStorage(HashMap+RNG+next_id)
  → ivy_grower(0x14094a910, 10 相跳表@0x142ab3be0) 主闭包 0x14094ac20 沿墙 add_point
  → 段完成 → IvySegmentDoneGrowingMsg → ivy_leaf_spawner(0x140996080) → IvyLeafStorage::add
  → ivy_send_to_gpu_leaf(0x1408c1580, IvyLeafSsbo+IvyFlowerSsbo) / _segments(0x140a9b4e0, IvySegmentSsbo)
  → render draw_shaders/instanced_ivy（draw_instanced_ivy_branch/leaf/flower 0x1408dbc50/180/1a0、ivy_leaf_shader 0x1408dc1c0）
```

## 6. 未决（U，不臆造）

1. IvySegmentSsbo/叶 SSBO 48B 打包元素各字段 ↔ 104B CPU 叶字段的精确映射（需 shader 侧交叉）。
2. ivy_grower 相 5-9 二级跳表（0x142ab3bf4）目标语义（主跳表已验证）。
3. insert_ivy_resources 内 IvySpawnNoise FastNoise 完整参数（频率 0.3/lacunarity 2.0 已 O，其余参数 U）。

## 7. 证据落盘（本轮新增）

- `evidence/country-core-deep/eco-reanalyst/decomp/decomp-0x14094c800.txt`（closest_segment_index fresh）
- `evidence/country-core-deep/eco-reanalyst/decomp/decomp-0x1416f9*.txt`（sheep 快照 serde，非 ivy）
- `evidence/country-core-deep/eco-reanalyst/rne_local.py`（本地 RNE 读取工具）
- 章节已升级：chapters/systems/ivy/{ivy_spawner,ivy_send_to_gpu_leaf,ivy_send_to_gpu_segments,ivy_grower,mod}.rs.md、chapters/resources/ivy/{ivy_storage,ivy_leaf_storage}.rs.md；collision-eco/{ALGORITHMS,DIGEST}.md §4/§6/§11。
