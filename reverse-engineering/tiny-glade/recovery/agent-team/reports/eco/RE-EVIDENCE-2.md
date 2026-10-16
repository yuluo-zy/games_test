# RE-EVIDENCE-2: E2-E6 定点取证合集（eco re-analyst，2026-10-16）

> 会话：eco re-analyst（2026-10-15/16）。覆盖 E2（DecoratorInfo +0x45）/ E3（sheep kind4 事件名）/ E4（ivy/garden 48B SSBO 打包）/ E5（NameHash secret 裁决）/ E6（噪声阈值 res+0x78）。
> 依据：B patched EXE 机器码（RNE oracle）+ fresh 反编译（eco-reanalyst/decomp/）+ S_PUB32。
> 记号：O=机器码证实 / I=推断 / U=未恢复。不改 reconstruction 源码。

## E2. DecoratorInfo 72B 字段表 + +0x45 rank_ty 区（O 部分）

**Fresh 反编译证据**：
- `apply_decorator_place`（0x140a20c70，fresh 243B + wall/decomp 完整版读面）：placement 记录 param_2 字段读面 = `+0x00 u64（dst 组分，传 get_or_insert_default）, +0x08 u64, +0x10 u64, +0x44 u32 rank, +0x48 u32 type, +0x4c u32, +0x50 u32, +0x55 u8, +0x56 u8`。
- `apply_decorator_remove`（0x140a20670，fresh 537B）：移除拷贝输出 0x48B = `{8×u64 @+0x00..+0x40, u8 state@+0x47}`（`local_89` = hashbrown remove 状态字节，`=='\x02'` 未命中 panic）。

**+0x45 rank_ty 区结论（I，两读面交叉）**：
- 章/合同已证：DecoratorInfo **rank@+0x44、type@+0x46**（decorator.rs 章 §判别访问）。
- apply_decorator_place 读面 rank@+0x44 一致；type 读面 +0x48 与 +0x46 并存（两处读同一 u32 区高低字节的两种取法，I）。
- **+0x45 = rank 与 type 之间 1 字节**（+0x44..0x47 4B 簇 = rank/`?`/type/state）；结合 apply_decorator_remove 的 u8@+0x47 = state 字节，`+0x45` 极可能为 rank 高字节或 state 前导（**精确语义 U，需 decorator 渲染/实体消费端读面**——见 E2 未决）。
- 遗留：DecoratorInfo 全 72B 逐字段语义（ExtraInfo 联合@+0x11 已证、dst tag@+0x22、id@+0x2c 已证；+0x00..0x20 与 +0x24..0x44 剩余 U）。

## E3. sheep kind4 loop 事件名（O，CONFIRMED）

**事件名字符串表（rne-read @0x142ac23b0）**：7 个一次性事件，**无 "Baah"**：
```
kind 0 @0x142ac23b0 event:/SFX/Sheep/Aww
kind 1 @0x142ac23c4 event:/SFX/Sheep/Think
kind 2 @0x142ac23da event:/SFX/Sheep/So
kind 3 @0x142ac23ee event:/SFX/Sheep/Angry
kind 4 → case 4: goto code_r0x0001409d61b0（帧尾循环分支，非事件名）
kind 5 @0x142ac2404 event:/SFX/Sheep/Idea
kind 6 @0x142ac2419 event:/SFX/Sheep/Eyes
kind 7 @0x142ac242e event:/SFX/Sheep/Confused
```
（decomp switch `*(byte*)(rec+3)` 分派证实 kind 编号；kind4 分支 0x1409d61b0 打包循环句柄 → goto 0x1409d5f60 帧尾 start_if_not_playing(FUN_140a596b0)/stop(FUN_140a59430) 公共块。）

**"Baah" 候选 = CONFIRMED 为 kind4 循环**：一次性表无 Baah；startup_sheep（0x140a093e0）载入 `event:/SFX/Sheep/Baah` + 参数 `Pitch`（0x142ac2460 = "Pitch" 串，一次性公共块也经 FUN_140a57670 设 Pitch）——kind4 的连续 "Baah" 循环实例由 startup 创建、每帧经 kind4 分支启停。**结论：kind4 = Baah 循环（独立于 7 一次性事件）**。

## E4. ivy/garden 实体→SSBO 48B 打包（O 框架 / U 逐字段）

- **GPU 元素 = 0x30B（48B）**（O）：send_to_gpu_leaf/segments `count × 0x30` 分配（plVar16/uVar15），与 CPU 侧 IvyLeaf 0x68B=104B（IvyLeafStorage::add）不同——**上传即打包 104B→48B**。
- **写缓冲原语（O）**：`FUN_140beefa0` = **`rhapsody::raw::device::GraphicsDevice::update_buffer_bytes`**（S_PUB32 确认，1147B）：逐 {offset,size} 区间从 staging memcpy 到 GPU buffer（需 `(in_R8+0x16)==1` 类型断言 + `(in_R8+0x14)==device` 断言），缺失时 create_buffer_impl。**仅 raw 拷贝，不负责打包**。
- **打包位置（O 框架）**：`FUN_1412fa4d0` = `Iterator::collect`（fat ptr → core/src/iter/traits/iterator.rs）——all_leaves/all_segments 迭代 + **map 闭包 104B→48B 打包**；打包后 staging Vec 交给 update_buffer_bytes。**48B 逐字段映射 = 该 map 闭包内部（U）**：需渲染域 `ivy_leaf_shader`（0x1408dc1c0）/HLSL 实例布局交叉推（CPU 104B 字段已知：{u64,u64,ivy_id,u64,pos Vec3@0x20,法向?@0x2c,上向?@0x38,u32@0x44,u64=0@0x48,u32@0x50,u32@0x54,Vec3@0x58}；48B 仅 12 槽 → 必然舍 pad/部分字段）。

## E5. ivy 三键 NAME 串 + NameHash secret 裁决（O，DEFAULT xxh3 短名）

- **0x140c9cd80 = `utils::name_hash::NameHash::compute`**（S_PUB32 确认）：46B，`lea rax,[常数名]` + `mov [rsp+0x20],0xc0`(len) + `lea r9,[secret]` + **`xor r8d,r8d`（seed=0）** + `call xxhash_rust::xxh3::xxh3_64_internal`（S_PUB32 确认目标 0x140e6a0c0）。
- **裁决：DEFAULT xxh3_64（seed 0，默认 secret），无 custom-secret**。
- **★ 2026-10-16 修正（旧结论 X → 新证据 Y，见 RE-IVY-ALGO.md §5）**：`SsboId = xxh3_64(StorageElement::NAME 短名)`，**非全类型路径**（`xxh3_64("Cobblestone3RasterStagingSsbo")=0x802ff23dee42ab47` ✓ 短名；全路径 `0xd6ba93c1577a0705` ≠ 键）。默认 secret @0x142b2ebb0（标准 192B，rne-read 字节核对）。**ivy 三键 ≠ 结构名**（`xxh3_64("IvyLeafSsbo")=0x853c6c5aec19be7a` ≠ 0x9e3cfa7f2993f406；Flower/IvySegment 同理）——ivy 的 `StorageElement::NAME` 常量 ≠ 结构名，**实际 NAME 串值 U**（get_rg_buffer 内 memcmp 的全类型名@0x142a9eb87 是类型校验非键；与 RENDER-U-CLOSURE 的 Flower03Ssbo NAME 穷尽一致）。
- **登记：utils NameHash::compute = `xxh3_64(name,0)`（默认）正确无需修**；restorer 复算 SsboId 用 `xxh3_64(NAME)`，NAME 值待槽名表 0x142c8151d 系/绑点名交叉。

## E6. 噪声阈值 res+0x78（O 门控 / 默认值 U）

- **门控（O，ivy_spawner 路径 B）**：`fVar29 = FastNoise::get_noise(res)`（FUN_141743760 = `bracket_noise::fastnoise::FastNoise::get_noise`，S_PUB32）+ `if (fVar29 < *(f32*)(res+0x78) || fVar29 == *(f32*)(res+0x78)) return`——res = `*param_2`（IvySpawnNoise 类资源，首字段即 FastNoise），阈值 f32@+0x78。
- **构造（O 部分，insert_ivy_resources 0x1409440c0）**：IvySpawnNoise 构造含 **FastNoise 频率 0.3（0x3e99999a）+ lacunarity 2.0（0x40000000）**（构造序列内常量，rne-read 证实）；`FUN_14152eee0` = `insert_resource<IvyLeafStorage>`（S_PUB32，非噪声），噪声资源插入为其前一步。
- **+0x78 默认值（U）**：精确阈值数值需 IvySpawnNoise 构造函数逐字段（构造序列 0x1409440c0 内 +0x78 写入未直读；建议 restorer 以构造序列 + `get_direction` 消费面交叉）。FastNoise 参数（0.3/2.0）已 O。

## 未决汇总（U）

1. E2：DecoratorInfo +0x45 精确语义；72B 全字段表（+0x00..0x20 / +0x24..0x44）。
2. E4：48B GPU 元素逐字段 ↔ 104B CPU 字段映射（打包 map 闭包内部；建议 shader/HLSL 交叉）。
3. E6：IvySpawnNoise +0x78 阈值默认数值。

## 证据落盘（本轮新增）

- eco-reanalyst/decomp/decomp-0x140c9cd80.txt（NameHash::compute）、decomp-0x140a20c70.txt（apply_decorator_place fresh）、decomp-0x140a20670.txt（apply_decorator_remove fresh）、decomp-0x140beefa0.txt（update_buffer_bytes）、decomp-0x14152eee0.txt（insert_resource<IvyLeafStorage>）。
- rne-read：sheep 事件名表 0x142ac23b0 区、ssbo 名表 0x142a9d65d 区、噪声常量 0x3e99999a/0x40000000。
