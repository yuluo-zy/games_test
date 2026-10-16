# RE-IVY-ALGO: ivy 主算法证据（eco re-analyst，2026-10-16）

> 会话：eco re-analyst。范围：grower 10 相 / spawner 随机根地形格 / pruner 空间哈希 / send_to_gpu 48B 映射 / ivy SsboId 名字来源。
> 依据：机器码（RNE oracle 字节级 + capstone 反汇编）+ fresh 反编译 + xxhash-rust 复算。
> 记号：O=机器码证实 / I=推断 / U=未恢复。**已挖透的指认章节，不重挖**。

## 1. grower 10 相主生长算法（O 结构 / 逐相体 U→部分 O）

**章节指认**：`chapters/systems/ivy/ivy_grower.rs.md` + `collision-eco/ALGORITHMS.md` §4.1（10 相跳表 @0x142ab3be0，分派 `*(u32*)(world+0x118)`；状态结构 {+0x10 相值写回, +0x1c 相值, +0x48 守卫 bit0, +0x11c 计时 f32}）。

**本轮 capstone 逐相体补证（O，0x14094a910 反编译仅能恢复跳表结构，相体需反汇编）**：
- **相 1/3/4 主入口 0x14094AC20**：`jne 0x14094ac51`（守卫 bit0 置位 → 守卫体 0x14094ac51）否则 `jmp 0x14094c0c0`（主体）。
- **相 2 计时入口 0x14094AC31**：`ucomiss xmm0,[rcx+0x11c]` → 计时 ≥0 走主体。
- **主体 0x14094C0C0 生长管线（O，反汇编调用序）**：`call 0x142923ee0` ×2（RNG）→ `call 0x14094c800`（**closest_segment_index**）→ `call 0x1405b0d70`（PublicWalls 哈希查墙）→ 逐株 `add_point`（0x140869460）。常量区 0x14094adcf.. 载入生长参数（RIP 相对 f32）。
- **相 0 守卫翻转 0x14094C09F / 相 5 二级表 0x14094F2F2 / 相 6-9 panic 入口 0x14094F306..**（章节已 O）。
- **逐相完整指令体（相体全长 ~0x2f0B）U**：需 rne-disasm 分段落盘（0x14094C0C0..0x14094f2f2），供 restorer 逐相实现。restorer 可用已知管线（last_point→get_direction→closest_segment_index→墙跳→add_point）+ 相转移条件先实现。

## 2. spawner 随机根地形格（O，路径 B）

- **章节指认**：`chapters/systems/ivy/ivy_spawner.rs.md`（路径 B = 随机墙 `perchance usize_less_than(墙数)`@0x14094d2e7 + 随机 u `uniform_f32`@0x14094d394 + `Curve::get_coord_at_u` + FastNoise 密度闸门）。
- **"地形格"澄清（O）**：路径 B 的格采样对象是**墙空间哈希（WallSpatialHash 64×64 格）**，非地形高度格——格结构 `{ptr@+8, len@+0x10}`、格步长 0x30B、`*(qword)(grid+0x48)`=dims / `+0x50`=坐标；**32/65 换算**（0x492307693=0.4923）为世界→格映射（与 utils dirty_tiles 的 fmaf(w,32/65,32) 同族）。路径 B 经 `usize_less_than(墙数)` 直接选墙后随机 u，再跳共享墙 loop（0x14094d49f）→ 侧向掷硬币（get_bool@0x14094d6b0）→ 距校验（2.56@0x142ab3214）→ spawn_ivy。
- 路径 A（光标旁）的格采样：RNG 格下标 + closest_segment_index(max=0.5f)——同章已 O。

## 3. pruner 空间哈希（O，章节指认）

`chapters/systems/ivy/ivy_pruner.rs.md`：ivy_pruner（0x140997410，70KB）= 墙空间哈希校验（**"Spatial Hash returned a wall id "** @0x142abb028）+ 逐点 `is_point_still_valid`（0x140998dc0，内含 closest_segment_index 复检）+ `ivy_pruner_cmd`（0x140999330，RemoveIvyCmd）+ `remove_ivy_at_location`（0x140999860）。**无需重挖**（已 O 全链）。

## 4. send_to_gpu 48B↔104B 字段映射（O 框架 / 逐字段 U）

- **写原语 O**：`FUN_140beefa0` = `rhapsody GraphicsDevice::update_buffer_bytes`（raw memcpy 区间，**不打包**）。
- **打包位置 O**：`FUN_1412fa4d0` = `Iterator::collect`（all_leaves/all_segments 迭代 + **map 闭包 104B→48B**）；staging Vec → update_buffer_bytes。
- **逐字段映射 U**：48B 元素 = {position, normal/up/scale 等 12 槽} ↔ 104B CPU IvyLeaf（{u64,u64,ivy_id,u64,pos Vec3@0x20,法向?@0x2c,上向?@0x38,u32@0x44,u64=0@0x48,u32@0x50,u32@0x54,Vec3@0x58}）——建议 **ivy_leaf_shader（0x1408dc1c0）/HLSL 实例布局交叉**（shader_readable_tree 可查）。restorer 可先用 `count×0x30` 缓冲 + CPU 字段顺序打包（I），待 shader 交叉定案。

## 5. ivy SsboId 名字来源（★ 定案：默认 xxh3_64(短 NAME)，NAME 串 U）

- **机制（O，verify_ssbo_id.py 复算）**：`SsboId = xxh3_64(StorageElement::NAME)`——**短名常量**（非全类型路径），**DEFAULT xxh3 secret**（0x142b2ebb0 = 标准 192B 默认 secret，rne-read 字节核对）。
- **自检（O）**：`xxh3_64("Cobblestone3RasterStagingSsbo")=0x802ff23dee42ab47`、`xxh3_64("ColorLibrary")=0xef2a68c12cde3f0d`（用 xxhash-rust 0.8.10 + verify_ssbo_id.py 双确认）。全路径 `country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo` → 0xd6ba93c1577a0705（**≠ 键**，证非全路径）。
- **ivy 键≠结构名（O，直接复算）**：`xxh3_64("IvyLeafSsbo")=0x853c6c5aec19be7a` ≠ 0x9e3cfa7f2993f406；`xxh3_64("IvyFlowerSsbo")/("IvySegmentSsbo")` 同理不等。故 ivy 类型的 `StorageElement::NAME` 常量 ≠ 结构名——**ivy 三键的实际 NAME 串 = U**（与 RENDER-U-CLOSURE 的 Flower03Ssbo NAME 穷尽一致；get_rg_buffer 内 memcmp 用的是全类型名（0x142a9eb87，len 0x3d=61）作**类型校验**，非键）。
- **E5 修正（旧结论 X → 新证据 Y）**：旧「ivy 键可 `xxh3_64(全名,0)` 复算」→ **仅机制对（默认 xxh3_64 短名），NAME 串本身 U**；restorer 以 `xxh3_64(NAME)` 复算验证，NAME 值以 render 侧槽名表（0x142c8151d 系）或 shader 绑定名为待查方向。

## 未决（U）

1. grower 相体全长指令级（0x14094C0C0..0x14094f2f2，建议 rne-disasm 分段落盘）。
2. send_to_gpu 48B 逐字段 ↔ 104B CPU 字段（shader 交叉）。
3. ivy 三键 `StorageElement::NAME` 串值（槽名表 0x142c8151d 系）。
4. 相 5 二级表（0x142ab3bf4）目标体语义。

## 证据

- 章节：chapters/systems/ivy/{ivy_grower,ivy_spawner,ivy_pruner,ivy_send_to_gpu_leaf,ivy_send_to_gpu_segments}.rs.md、collision-eco/{ALGORITHMS,DIGEST}.md。
- 本轮：capstone 反汇编 0x14094AC20（相 1/3/4 入口 + 主体调用序 closest_segment_index 0x14094c800/哈希 0x1405b0d70/RNG 0x142923ee0）；xxhash-rust 复算脚本 `/tmp/xxh3check`（ivy 键≠结构名、Cobblestone 短名=键）；rne-read 0x142b2ebb0（默认 secret 字节核对）。
