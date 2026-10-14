# terrain-water 域 round2 U 项闭合报告

8 个 U 项的处置（O=闭合；I=有界推断；U=仍开放，注明卡点与已排除路径）。全部机器码为准。

## 1. +0x29b 再上传脏字节的置位点 —— I（收窄到计算寻址/GPU 写回，直接寻址空间已穷尽排除）

**排除性证据（O）**：对 .text 全段做了 disp32=0x29b 的穷举扫描（含 `c6`/`88`/`80 or/and`/`fe inc`/`d0,d1`/SIB/REX 前缀/`66 c7` word/`c7` dword（disp 0x290..0x2a0）/`48 c7` qword（disp 0x294..0x29b）/lea +0x298/`0f 11` 16B 向量写（disp 0x288..0x29b）/`c5 fc` 32B 向量写）：
- **+0x29b 的全部访问点**：读=water_rasterize_from_cpu(0x140a12721)、garden_rasterize_from_cpu(0x140a221e1)、compute_path_masks(0x14090177f/0x140901790/0x14090291a)；写=同函数内 **只写 0**（清除），及 WorldRaster::new 的 memset 0x204 清零。**不存在任何直接寻址的置 1/置非零指令**。
- 覆盖 +0x29b 的宽写入（dword/qword/向量）也全部为 0 或无关结构初始化（0x140c25541 的 `66 c7 86 98020000 0101` 是 2 字节写 0x101=selector+staging 脏，只到 +0x299；属于某工具参数结构初始化，false positive）。
**结论**：置位方必在 (a) 计算寻址（基址+索引循环/函数指针 helper）或 (b) GPU→CPU 映射缓冲写回。协议已 O：置位后由 water/garden/path-masks 三个 rasterize_from_cpu 消费——同步 SSBO 版本（+0x5c ← 库 +0x5b0）、清 +0x29b、memset(+0x98,0,0x200)。已知的其它上传脏标志：**+0x299 由 AllWorldRasterWrite::write 置位（O，round1）**，+0x298/+0x29a 由 copy_to_staging 写。卡点：若要穷尽需对 tabloid 回调链（publish 0x1425db920 的订阅记录 +0x20 闭包槽）做动态跟读——属依赖边界，按项目规则不复原。

## 2. 0x07954dbb 瓦片状态字 —— O（闭合）

- **初始化点**：WorldRaster::new（0xa15530）从 .rdata 0x142ac7e50..5c（4×u32 全 0x07954dbb）填充全部 4096 个瓦片状态字（0x4000B 位图缓冲，SIMD 四字循环 + 标量尾）。
- **清除点**：impl$2::clear_all\<512>（0xa15da0，行 74）同样以 0x07954dbb 重置瓦片状态（SIMD 16B 四字路径 + `*puVar17 = 0x7954dbb` 标量路径，0x140a16092）。
- **语义（I）**：作为 f32 读 = 7.7e-35（denormal ≈ 0）→ "空瓦片/无水" 魔数哨兵（非精确 0.0f，可与真实零值区分）；作为 u32 = 0x07954dbb。瓦片状态消费者在 tabloid DirtyTiles 依赖内（retain/publish），不复原。

## 3. WorldRaster::new 原型来源 —— I（闭合到类型层面）

- new 的 DATA xref 0x1436953d0 所在表 = **{函数指针, 调整量, 类型描述符 RVA} 三元组序列**（相邻条目 {0xa15530, 0xa157a8, 0x032d760c, …}），即 `WorldRaster<512>` 的 trait-impl vtable（动态分发）。无 CALL xref（调用经 vtable 间接）。
- param_2 的 56B 拷贝 = tabloid DirtyTiles 头模板（mutex@+0x60/68 + 订阅者 Vec@+0x78/80/88 等 7×u64），由泛型默认构造路径（`insert_resource<…Raster>` → Default → vtable 分发）提供。
- 卡点：vtable 所属 trait 名与调用方（Bevy resource 初始化泛型）无法从静态表定名；类型层面已闭合。

## 4. subscribe 记录消费方 —— I（tabloid publish 迭代，闭包槽 +0x20）

- publish<DirtyTiles>（0x1425db920，tabloid 依赖）：version 自增 → FUN_141bb0250（发布回调）→ memcpy 512B 位图 → 遍历订阅记录（每记录：parking_lot 锁 +0x10、**Atomically 换装记录 +0x20 的 Arc 闭包**（LOCK inc 新/dec 旧，旧归零 func_0x000140b5cb30 释放））→ 并发订阅安全（grow FUN_140562870 + 尾追加）。
- 即订阅记录 = {Arc 计数@0, lock@0x10, Arc<闭包>@0x20}；**消费方式 = publish 时换装闭包**（非直接调用）。游戏侧具体回调内容在 tabloid 边界，不复原。subscribe 本身 O（0x28B Arc 记录压入 +0x78 Vec；secondary panic 串已录）。

## 5. solidify_fluid_into 固化数学 —— O（闭合）

- 签名 (points_ptr, points_len, dst_vec)。流程：std 迭代器适配（`FUN_141308760` 建视图，来源 /rustc/…/iterator.rs）→ **FUN_141467fa0(视图, &视图, 0.5) = 0.5 世界单位定距重采样**（_UNK_142925870=0.5）→ 剩余 <2 点走单点拷贝；结果 memcpy 进 dst_vec（cap 检查 + FUN_1428a9a60 扩容）或 FUN_1412f9e10 追加（尾点合并）。即"固化"=把流体输入点列重采样为 0.5 间距折线并入目标 Vec。

## 6. ui_terrain_focus_mode 闭包体 —— I（边界 O，语义框架）

- closure$1（0x9a8350，4.4KB）+ closure$1::closure$0（0x9a3620，682B：FUN_140c97e10(…, 0.5, [p+0x10]) 曲线采样辅助）、closure$2（0x9a8640）、closure$3（0x9a88c0）、closure$4（0x9a8b40）均反编译落盘。
- 主函数 0x9a4fb0（167KB）几乎全内联（alloc/memcpy/HashMap helper 0x140c9cde0×9），闭包为聚焦模式进入/退出/高亮状态机（OutlineWallCmd/PublicWalls 相关，契约已列签名）。
- 卡点：闭包内部状态转换的语义命名需 ui-input 域上下文（该域 owns ui_edit_terrain 等）；本域给出地址+边界+调用结构，不臆造状态名。

## 7. ActiveRasterStroke +0x00/+0x08 —— O（修正 round1 布局）

- **+0x00**：u64 活动/流体标志（add_point 内 `*p1 == 1` 门控 solidify 分支，start_new 置 0）。
- **+0x08**：**流体游标指针**（add_point 读为指针与 1 比较判非空；solidify 后置 0 或 `剩余点数`，行 97/119 写 `lVar7`/`lVar7+2`）——round1 曾标"配置 u64（max points）"为误读，已更正。
- 修正后 InProgressRasterStroke 视窗：{flag u64@+0x00, cursor@+0x08, 固化曲线 Vec@+0x10, 输入点 Vec@+0x28, 弧长 f32@+0x40, …}。

## 8. drawing_input_smoother 平滑核 —— O（闭合）

- DrawingInputSmoother::process（0x145c350）= **弧长均匀重采样**：沿输入折线以 t=i/N 参数步进 lerp 取样（`fVar18 = t*fVar1 + (fVar5-t)*prev.x`），带最小间距门控（`fVar6 < dist && fVar7 < dist*fVar11` 跳过过密点），输出 8B (x,z) 对入 Vec（`CONCAT44(fVar12,fVar18)`），并回写 `param_2`（首个输出点）+ 置位标志。非拉普拉斯平滑——是重采样+抽稀。

## 未闭合残留（写入 DIGEST U 节）

- +0x29b 置位的最终动态路径（tabloid 回调链，依赖边界）。
- 0x07954dbb 的精确编码含义（魔法数语义 I 级）。
- ui_terrain_focus_mode 闭包状态命名（ui-input 域）。
- subscribe 回调的游戏侧语义（tabloid 边界）。

## 复用技巧登记（协调方 CU 边界表）

PE .rdata 的 CU 函数边界表（0x14369a000/0x143682f74/0x143697308 等，rne-read --hex 可读）对"Ghidra NotFound / S_PUB32 无符号"函数的边界枚举有效；本域后续如需枚举 compute_path_masks（0x1409014e0）邻接的私有小函数（+0x29b 协议辅助）可复用。
