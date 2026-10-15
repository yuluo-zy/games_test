# roofs-rt 域算法级深挖（2026-10-15）

> 配套：DIGEST.md + 本文件（算法级）。证据：`decomp/` + llvm-objdump 原指令 + rne-read 常量/串。
> 本轮新裁决（机器码）：rt 三系统完整状态机、bvh_builder_worker 状态字节实际偏移（+0xa0 非 +0x14）、
> roof_noise/visual_shape 常量更正（振幅 0.3、噪声尺度 0.5、smoothstep (3−2s)·s²、噪声系数 −0.3）、
> roof_uv_and_point_at_ws_pos 全管线。

---

## 1. 处理链图（总览）

```
[生产者] 墙/网格系统 → RtWorld(哈希表, 16B 条目) + DirtyCow<T>（refcount copy-on-write）
                      │ Events<UpdateRtWorldEvent> + 帧差
                      ▼
[update_rt_world 系统 0x1408bc650]
  事件游标差 → 世界脏位（资源+0x38 字节）→ 日志 "UpdateRtWorldEvent requested"（帧>4）
  → 新条目两数组 walk（0x10 步长）→ 每条目 hash 查找 + DirtyCow 唯一所有权检查 + 脏标记
  → 脏位置位时全表扫描（16B 组占用掩码 + 脏字节数组）
  → 抢 worker（busy@+0x10: LOCK cmpxchg 0→0x3fffffff）→ 装配工作包（脏字节 + 条目引用）
    → state@+0xa0 != 2 时 FUN_1400524e0(0x20/0x10) 触发构建 → 释放抢注
                      ▼
[bvh_builder_worker 线程 0x140b24d50]
  循环：抢 busy（断言空闲）→ 全局停止标志 0x14362c420（低 63 位非零 → fastrand 随机停止）
  → state@+0xa0: 置 2; 先前 == 2 → 释放 + sleep 1ms（FUN_140d55980(0, 1000000)）
  → 否则: 处理标志@+0xa8 = 1 → 从脏列表(+0x68/+0x70/+0x98) 组装三角缓冲(0x28 步长)
    → build_gpu_bvh(0x140b0da50) → 交换新 BVH 进 glue(+0x20..+0x60) + 释放旧缓冲
    → 处理标志 = 0 → 释放抢注（+0x18 状态字节非零则退出并 Arc drop）
                      ▼
[upload_rt_world 系统 0x1408bcff0]
  抢 worker → 断言 glue 状态 → 快照 pending 列表（+0x20..+0x5c）→ 写 None 哨兵
  0x8000000000000000@+0x20 → 释放抢注 → 容量守卫（<0x40000 / <0x10000）
  → 按名查 3 个 SSBO（FUN_140969720 字典，名 @0x142c2e3d8/270/350，len+memcmp 校验，不匹配 panic）
  → vtable 取缓冲 → 三角/元数据/节点 三路 Vec append（stride 0x50/4/0x30）→ SSBO 上传
                      ▼
[GPU] rt_triangles / rt_triangle_meta / rt_bl_bvh（软光追 BVH）
```

---

## 2. rt/mod.rs 逐函数算法

### 2.1 update_rt_world（0x1408bc650）— O 全解

1. **puffin scope**：模块 `crates/country-core/src/resources/rt/mod.rs`（0x142aa0995）、scope 名 @0x142aa05f9（0x2c=44 字符）。
2. **双事件游标消费**（SystemParam [0]=cursor、[1]/[3]=Events 缓冲）：`uVar28 = cur − prev1`、`uVar30 = cur − prev2`（下溢饱和 0）；`bVar35/34 = diff ≤ len` → 有效剩余 = len − diff；游标更新 = total − (剩余1 + 剩余2)。**任一流有新事件** → 若全局帧 `0x14362c3a8 > 4` → `log!("UpdateRtWorldEvent requested")`（串 @0x142aa09f8，源 rt/mod.rs）→ 资源 +0x38 字节 |= 脏位。
3. **新条目 walk**：两条数组区间（len 差 vs +0x18/+0x38，基址 +0x8/+0x28，16B 条目）；逐条：计数 `*puVar6 += 1` → hash 查找（FUN_1405b14b0，表 @param_1+4）→ FUN_140b78550（get/create，状态枚举 {…,3}）→ **DirtyCow refcount 块**（LOCK；`(*Arc)[−1] + 0xf` 唯一性判定；归零 → FUN_140b5cad0 释放旧值）→ 置资源 +0x38 = 1（脏）。
4. **全表扫描**（两条数组耗尽且脏位置位时）：16B 组占用掩码（每字节高位取反 = hashbrown 占用位图）→ 逐占用桶查脏字节数组（负偏移 `entry − 8 − bit*0x20`）→ 非零则处理（同上 DirtyCow 块）。
5. **worker 握手**：`LOCK; busy@+0x10 == 0 → 置 0x3fffffff`（抢注，非空闲 → panic）；全局停止标志 0x14362c420 检查（低 63 位非零 → fastrand 随机置 +0x18=1）；`+0x18 != 0 → panic "called `Result::unwrap()` on an `Err` value"`（0x142aa03b8）；装配工作包（脏字节 uVar36 + 条目引用 +0x68..+0xa7 快照）→ **state@+0xa0 != 2 时 `FUN_1400524e0(工作包, 0x20, 0x10)`**（BVH 构建触发/通知辅助，两侧对称调用）→ 随机置 +0x18 → 释放抢注（`busy −= 0x3fffffff`，下溢 → panic）。

状态字段（update 侧视图）：`+0x10 busy（0x3fffffff 抢注标记）、+0x18 退出/停止字节、+0x68/+0x70 条目 Arc 对、+0x78..+0x9c 计数 u32 对、+0x98 数据计数、+0xa0 状态/脏字节（+0xa1..+0xa7 7 个脏字节）`。

### 2.2 upload_rt_world（0x1408bcff0）— O 全解

1. puffin scope（scope 名 @0x142aa05ca）。
2. **抢 worker**（busy@+0x10，0→0x3fffffff，非空闲 panic）；停止标志/随机停止检查；`+0x18 != 0 → panic`（0x142aa03b8/0x142aa0a58）。
3. 快照 pending 列表：`+0x20..+0x5c` 十组 u32（计数/尺寸对）+ `[0xc]@+0x60`（条目表 ptr）；**`+0x20 = 0x8000000000000000`（None 哨兵/占用标记）**。
4. 释放抢注。
5. **容量守卫**：`(count1 | count2) < 0x40000 && count2 < 0x10000`（超限走 panic 路径）。
6. **三 SSBO 按名解析**（FUN_140969720 字典 ×3，键名 @0x142c2e3d8/0x142c2e270/0x142c2e350，期望长度 0x2c/0x2f/0x26）：长度+名称逐字节校验，不匹配 → panic（FUN_1428cab41）；`(**(code **)(entry[1] + 0x18))(entry)` 虚调取缓冲。
7. 三路数据 append（FUN_140beefa0 = Vec 扩容/追加，容量检查）：三角（stride 0x50）、元数据（u32）、BVH 节点 —— 与 render-passes 的 rt_triangles/rt_triangle_meta/rt_bl_bvh 对应。

### 2.3 DirtyCow / RtWorld — 算法要点

- `DirtyCow::get_mut`（0x1415626f0）：LOCK；refcount==1 → 原地 mut（改计数语义）；否则克隆 RtMesh；返回网格指针（字段 +0x24/+0x2c/…/+0x64/+0x6c）。
- `RtWorld::get_mut<WallId>`（0x1412259c0）/`<RtMeshHandle>`（0x1400cce50）：键转换（impl$5::from = 恒等/零 key）→ FUN_1405b14b0 hash → 桶探测（16B 条目）。
- 条目/脏布局：16B 条目 = {key, value}；ctrl/占用高位置于条目区（迭代经 pmovmskb 提取每字节高位）；脏字节存负偏移数组（`entry − 8 − bit*0x20`）。

---

## 3. rt/bvh.rs build_gpu_bvh（0x140b0da50）— O 全解

```
build_gpu_bvh(out, triangles(0x28 步长), count):
  分配 count*0x30 节点缓冲（0x140613c10, align 0x10）
  if count == 0: 内联写退化根 3 节点 {0,4,0}×3（9 u32）→ 返回（不分配）
  主循环：每 2 个输入三角（各 10 u32: 3×Vec3 位置 + meta u32@+0x24）→ 2 个 0x30B 节点
     节点 = 3 顶点 × vec4（{x,y,z,z} 复制填充，12 u32）
     余 1 个奇数三角单独处理
  FUN_1417c93d0(out_vec, node_buf, count, …) — 计算根 AABB/包装输出 Vec
  FUN_141309ae0 — 三角索引重排（排序后索引表 lStack_198/lStack_190）
     索引表逐项: 越界 (idx ≥ count) → panic（0x142c65638）
     重映射: meta[i] = 输入三角[idx].meta u32（param_2 + 0x24 + idx*0x28）
  out 结构（≥0x48B）：{节点 Vec{ptr,len,cap} @+0x00,
                      AABB min @+0x18 (2×u32) / max @+0x20..0x28,
                      +0x28 辅助, 索引计数 @+0x30, 索引 ptr @+0x38, 索引 len @+0x40}
  释放临时缓冲（0x140613c20 {size, align}）
  （帧 > 4 时 puffin scope 日志 @0x142ae7c10/0x142ae7c50）
```

---

## 4. rt/bvh_builder.rs bvh_builder_worker（0x140b24d50）— O 全解

**偏移更正（raw bytes 0x140b24d50 前导复核）**：`+0x10 busy（lock cmpxchg 0→0x3fffffff）、+0x18 退出/停止字节（movzx [r13+0x18]）、+0xa0 状态字节（movzx [r13+0xa0]; mov byte [r13+0xa0], 2）、+0xa8 处理标志`—— DIGEST/章内 "+0x14 state/+0x15 processing" 为旧读，**更正为 +0xa0/+0xa8**。

```
worker(glue):
  loop:
    抢 busy: LOCK; cmpxchg [glue+0x10]: 0 → 0x3fffffff; 非零 → panic（FUN_1428d8c10）
    停止标志 0x14362c420（首 qword & 0x7fffffffffffffff）: 非零 → bStack = fastrand()^1（随机）
    if glue+0x18 != 0: break（退出）
    state = [glue+0xa0]; [glue+0xa0] = 2
    if state 先前 == 2（空闲）: 释放抢注; sleep(0, 1_000_000ns = 1ms); continue
    # 处理路径：
    [glue+0xa8] = 1（处理中）
    读脏列表: {ptr@+0x68, len@+0x70, count@+0x98, 计数@+0x78..+0x94, 脏字节@+0xa1..0xa7}
    占用掩码（16B 组）→ FUN_14130d1b0 → 组装三角缓冲（0x28 步长）
    build_gpu_bvh(三角缓冲, count) → 新 BVH
    再抢 busy; 停止检查
    if glue+0x18 == 0（继续运行）:
      交换: 旧 {tri@+0x20, tri_len@+0x28, node@+0x38, node_len@+0x40, meta@+0x50, meta_len@+0x58}
            ← 新 BVH（+0x60 等 12 组 u32 写回）
      释放旧缓冲（tri stride 0x50 / node stride 0x18 / meta u32）
      [glue+0xa8] = 0
      随机置 +0x18; 释放抢注
    else: 释放新缓冲; 释放抢注
    释放临时三角缓冲（stride 0x28）
    FUN_1400524e0(&脏列表, &计数, 0x20, 0x10)   # 与 update_rt_world 对称的完成通知
  exit: 帧>2 日志（0x142ae9aa8 区）→ 随机置 +0x18 → 释放抢注
        LOCK; [glue+0x00] −= 1; 归零 → FUN_140b5c7a0（Arc drop，释放 glue）
```

---

## 5. roof_noise / roof_visual_shape — O 全解 + 常量更正

### 5.1 gpunoise::noise（0x140a152c0）— O 全解

```
noise(x, y):
  x0 = floor(x), y0 = floor(y)（饱和截断: NaN→0, >0x4effffff→0x7fffffff）
  fx = x − x0, fy = y − y0
  4 角 Wang 型哈希链（坐标 v 与 y 组合）:
    h  = ((v*0x401) >> 6 ^ v*0x401) * 9
    h  = (h >> 0xb ^ h) * 0x8001
    h  = (y·0x19660d + h + 0x19660d) * 0x19660d + 0xae135d3      # 0xae135d3 = 0x142ac7e10/e14 种子同值
    角种子表 @0x142ac7e10..e1c（观察值: e10/e14 = 0x0ae135d3, e18/e1c = 0 —— 第 3/4 角哈希退化为常量种子，机器码如此）
  finalizer: h ^ (h & 0x13a58ad) << 7;  h ^ h << 0xf & 掩码;  h ^ h >> 0x12
  u01(h) = (bits | 0x3f800000) − 1.0                          # [0,1) 归一
  x 轴 smoothstep: sx = (3 − 2·fx)·fx²
  双线性: lerp(lerp(v00, v10, sx), lerp(v01, v11, sx), (3 − 2·fy)·fy²)
  return [0, 1]
```

### 5.2 apply_roof_noise（0x1412348f0）— O（常量更正）

```
apply_roof_noise(vertex.xyz, strength):
  s = clamp(strength, 0, 1)
  n = noise(vertex.x · 0.5, vertex.z · 0.5)          # 尺度 0.5（_UNK_142925870；章内 "0.1" 错）
  vertex.y += n · 0.3 · (3 − 2s) · s²                # 振幅 0.3（_UNK_1429258d8；章内 "1.5" 错）
                                                     # smoothstep (3−2s)s²（章内 "(1−2s)s²" 错）
```
常量：0.5 = 噪声采样尺度、0.3 = 振幅、3.0 = smoothstep 系数（_UNK_1429cd078）、1.0 = clamp 上限（_UNK_142925904）。

### 5.3 get_height_01_unclamped_from_ws（0x1408e4790）— O 全解（公式级）

```
h01_unclamped(pos):
  n  = noise(pos.x·0.5, pos.z·0.5)
  y  = pos.y + n · (−0.3)                     # 噪声系数 −0.3（_UNK_142925900；章内 "0.12?" 错）
  if height_01(+0x30) > 0.1:
    if tag(+0x8) bit0 == 0 (circle):  dim_n = (radius(+0x14) − 0.9) / 7.1    # −0.9/7.1
                                      base  = 16.0                           # _UNK_142934604
    else (rect):                     dim_n = (max(dim_x(+0x1c), dim_z(+0x20)) − 2.25) / 12.75
                                      base  = 9.0                            # _UNK_142a7ba84
    t = clamp(dim_n / 0.4, 0, 1)
    smooth = (3 − 2t)·t²
    cap_base = smooth · 12.0 + (1 − smooth) · base            # 12.0 = _UNK_1429cd2d4
    if cap_base < 0 → panic（0x140d8b480, 串 @0x142aa5190）   # assert 高度上限非负
    cap_h = height_01 · 12.0 + (1 − height_01) · 0.4          # 0.4 = _UNK_142925970
    cap   = min(cap_h, cap_base)
  else: cap = 0.0（未越阈值路径直接以 0 为界）
  return (y − floor(+0x4c)) / (cap − floor)
```
（circle 分支的 `(r−0.9)/7.1` 与 rect 的 `(d−2.25)/12.75` 归一后夹取，同 Roof::height 的 smoothstep 上限族 —— 与 reconstruction/roof/surface.rs 的 height 同源。）

---

## 6. roof_trigger_volume / roof_uv 管线

### 6.1 RoofTriggerVolume（roof_trigger_volume.rs）— O 全解

- `triangles()`（0x1408e4560）：`len@+0x10 == 0 → panic "assertion failed: !self.triangles.is_empty()"`（0x142aa5958, 0x2c=44 字符）；否则返回迭代器 `{start=ptr@+8, end=ptr@+8+len·0x24, idx}`（stride 0x24=36B）。
- `set_triangles`（0x1408e45b0）：drop 旧（0x140613c20 系）→ 整块拷入新 Vec（cap-first）。
- `set_normals`（0x1408e45f0）：stride 0x24；断言 `triangles.len() == normals.len()`（0x142aa59e0）。
- `set_uvs`（0x1408e4660）：stride 0x18；断言 `triangles.len() == uvs.len()`（0x142aa5a38）。
- `get_position_from_mesh_uv`（0x1408e46d0）：经 FUN_1416f34a0（utils position_from_mesh_uvs）三角/uv → 位置；返回 `{found, pos}`，found=0 丢弃输出。

### 6.2 roof_uv_and_point_at_ws_pos（0x1408e4090，roof_shape.rs）— O 全解（collider 注册表管线）

```
roof_uv_and_point_at_ws_pos(out, key, ws_pos, …):
  if RaycastWorld+0x48 注册表空 → not-found
  h1 = FUN_1405b0d70(world+0x50, key)         # 注册表 hash（hashbrown 双哈希探测:
  #   首轮 16B 组字节比较 + 占用掩码; 未中 → 第二哈希 h2 探测序列（+0x10 步进））
  命中 → value = 条目值（含 {slot u8 区, index u32} 打包 u64）; hi==0 → not-found
  重建 {slot, index} 键 → FUN_1405b0d70(world+0x3e18, …) 竞技场注册表（同探测模式）
  命中 → slot 字节 != 0x11:
    puVar11 = func_0x000140b93940(world + slot·0x390 + 0x20, index, gen)
              # 槽竞技区 = world + slot·0x390 + 0x20（17 槽 0x390 布局复核 ✓）
    有效性 {index < world+0x10, arena@+8, entry+0x20 == gen|1} 否则 panic
              （0x142aa3d50/0x142aa3d98 = world.rs 一致性断言）
    读 entry 命中视图: {mesh ptr @+0x34/+0x3c/+0x44, f32 @+0x4c}（CLOSURE 布局复核 ✓）
    FUN_140c91b60 → 装配网格视图; FUN_140c9cde0 → 由 ws_pos 建 ray
    shape vtable [view+0x38] 虚调 cast_ray_and_get_normal（同 RaycastMesh::raycast 模式）
    命中 → FUN_1408ce4a0（barycentric 插值）→ marker != 2:
      out = {byte0 = 0（found）, byte1 = ?, uv@+0x4, point@+0xc, +0x14}
      （marker&1 → uv 写; 否则 byte1 = 4）
    未命中 → *out = 0x301（not-found 标记）
  out 默认 = {1, 0}（not-found）
```
这确认 roof_shape 的 uv 悬停与碰撞世界共用同一注册表+竞技区+插值管线（跨域接线 O）。

---

## 7. 其余模块算法要点

- **Roof::height / get_roof_shape / profile_curve**：已由 reconstruction/roof 恢复（2,500+ 案例），本域不重挖（roof_shape 章映射）。
- **RoofAnimation（roof_animation.rs）**：`is_being_animated`（0x1408dbc20）= `cmpss [rcx], 常量; movd eax; and 1`（首 f32 ≠ 常量）；`new`（0x1408dbc40）= xorps 双 xmm 零化 16B。
- **RoofTileSimulation::new**（0x140939390）：`+0 state=0; +4 timer=入参; +8..+0x14 零化`。
- **RoofBoundsDims**：+0 tag bit0 = is_rectangle；`as_circle`（0x140a88200）tag bit0 置位 → panic；`as_rectangle`（0x140a88240）bit0 清零 → panic；各拷对应字段。
- **mesh_atlas_library register_meshes**（0x140ae6b90）：逐世界子集（字节 key 哈希表，FUN_1412e7650）注册网格 → 变更时重建图集 + `RtBvhBuilderGlueResource::new`（0x14067dd80）×2 接入 rt；panic "Updating mesh atlas for world subset "/"Not registering atlas for world subset "（0x142ae3438/0x142ae3518）。
- **Affine3Packed**：48B = 3 行 × 16B；`add_y`（0x14089d550）= `addss [rcx+0x28]`（平移 y@+0x28）；`from_cols`/`from_mat3_translation`/`From<Mat4>` = movups/shufps 重排。
- **TreeStorage**：16B 条目哈希表（{base@+0, mask@+8, len@+0x18}，FUN_1405b0d70/FUN_140b76f90）；`iter`（0x140a1b100）= `{cur@+0, next@+8, end@+0x10, occupied@+0x18}`，pmovmskb+NOT 取 16B 槽高位占用位图，步进 0x10；`insert`（0x140a38560）=`max(+0x30, id+1)` + FUN_140eec970 句柄（高 32 位 0 → None）；`next_index`（0x140a38600）= `mov rax,[rcx+0x30]; lea rdx,[rax+1]; mov [rcx+0x30],rdx`。
- **spatial_resolve_offsets default_value**（0x140b31d10）：首字段 1，随后 0x800+B 常量表（`_UNK_142aeca*`，步长 8B {f32, 0} 对）整体拷入 +0x7e*..+0x800。
- **resources/mod.rs 三入口**：insert_rendering_resources（0x140940190）/ init_populate_ssbo_library（0x140909e80，FUN_1412c4*..1412c6f* 30+ 注册函数）/ insert_resources（0x140940d80，逐资源真实构造 0x140b18550 等）—— 全部经 bevy_app App::init_resource/insert_resource 单态化族（func_0x000142923c00/c20 调度辅助）。

---

## 8. 常量表（本轮 rne-read 证实）

| 常量 | 值 | 用途 |
|---|---|---|
| `0x14362c420` | 全局共享状态指针 | worker/update/upload 停止判定（首 qword 低 63 位非零 → fastrand 随机停止） |
| `0x14362c3a8` | 全局帧计数器 | 日志门限（update: >4；worker exit: >2） |
| `_UNK_142925870` | 0.5f | roof 噪声采样尺度（x/z 均乘） |
| `_UNK_1429258d8` | 0.3f | apply_roof_noise 振幅 |
| `_UNK_142925900` | −0.3f | get_height_01 噪声系数（y 减向） |
| `_UNK_142925970` | 0.4f | 平顶高度（Roof::height 的 (1−h)·0.4） |
| `_UNK_142925984` | 0.1f | is_flat 阈值 |
| `_UNK_1429cd2d4` | 12.0f | 高度上限（Roof::height 的 h·12） |
| `_UNK_1429cd078` | 3.0f | smoothstep 系数 (3−2x) |
| `_UNK_142934604` | 16.0f | circle 高度基 |
| `_UNK_142a7ba84` | 9.0f | rect 高度基 |
| `_UNK_142aa5084/88` | −2.25f / 12.75f | rect 尺寸归一 (d−2.25)/12.75 |
| `_UNK_142aa508c/90` | −0.9f / 7.1f | circle 尺寸归一 (r−0.9)/7.1 |
| `_UNK_1429a49bc` | 0x4efffffff | gpunoise floor 饱和上界 |
| `_UNK_14292e6b0/6d0` | 1.0f / −1.0f | u01 归一 (bits|0x3f800000)−1.0 |
| `_UNK_142ac7e10/e14` | 0x0ae135d3 | 角哈希种子（e18/e1c = 0） |

---

## 9. 与既有证据的差异（本轮新增/裁决）

1. **bvh_builder_worker 状态字节偏移**：raw bytes 证实 `+0xa0`（state，2=空闲）/`+0xa8`（processing），DIGEST/章内 "+0x14/+0x15" 更正；+0x18 为退出/停止字节（旧章未录）；+0x10 busy 抢注为 cmpxchg 0→0x3fffffff 协议。
2. **roof_noise 三常量更正**：尺度 0.1→**0.5**、振幅 1.5→**0.3**、smoothstep (1−2s)s²→**(3−2s)·s²**；get_height_01 噪声系数 "0.12?" → **−0.3**（章内数值全部以机器码为准）。
3. **build_gpu_bvh 输出结构**：章内"输出节点 stride 0x30"正确，补输出面 ≥0x48B 布局（节点 Vec + AABB + 索引重映射表 + 计数）。
4. **update_rt_world 的 +0x38 脏字节**：此前未录（事件游标→脏位的物化点）；worker 握手 0x3fffffff 抢注协议与 worker 侧对称。
5. **upload_rt_world**：容量守卫 0x40000/0x10000、SSBO 名校验（len+memcmp）、None 哨兵写回 —— 章内"逐脏条目置 0x8000000000000000"确认并补 3-SSBO 解析链。
