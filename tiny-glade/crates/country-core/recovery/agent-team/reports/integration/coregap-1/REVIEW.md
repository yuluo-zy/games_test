# reviewer-coregap-1 — core-type-gap / core-api-missing 第一组 REVIEW

日期：2026-10-17
角色：core 集成 reviewer（coregap-1）
范围：独立核验 restorer-coregap-1 在 `crates/country-core/src/` 补齐的 10 个 core 类型/API，逐条 fresh 反编译对回二进制真实语义（硬标准：不发明字段）。

方法：全部 fresh 反编译（ghidra `--worker shared`，证据落
`reports/integration/coregap-1/review-evidence/batch{1..3}.json` + `decomp/*.c`），
与 RESTORE.md 实现清单及 src 逐字段/逐语句对照。仅评审，未改 src。

**`cargo check -p country-core`：通过（Finished dev，18.6s；10 warnings 全在
save/snapshot.rs 等非本批文件）**。

## 裁决汇总（10 条）

| # | 类型/API | 裁决 | 关键证据（fresh） |
|---|---|---|---|
| 1 | `PlayUiSound` 变体 #4 | **判等** | `feedback_input_out_of_border` 0x141be2280 事件写：`* (lVar7+lVar11*0x20)=4`（判别 4@+0、stride 0x20）+ `* (lVar7+0x18+…)=uVar6`（id@+0x18）|
| 2 | `SignifierFactory` | **判等** | 0x14089d8b0：`memcpy` + `[0]=len [1]=ptr [2]=len [3]=&inner [4]=marker`（5×qword，40B）|
| 3 | `SignifierEntityBuilder` 链 | **需修** | 0x14089d9b0/dad0/dc90/db00/dd50（详见 §1）|
| 4 | `MoveTreeCmd` | **判等** | 0x142267430 消费（事件迭代 stride 0x20：index@+8 作 `TreeStorage::get` 键、pos@+0x10）；0x14226b1a0 发射（0x20B 槽 {id,index,3×f32}）|
| 5 | `ReplayBucket.timing`/`advance_timing` | **需修** | 0x141bc34f0（详见 §2）|
| 6 | `HighlightColor.to_rgb` + `HighlightRoofCmd` | **判等** | 0x140b34f30（`==1→{0.1,0.03,0}` else `{0.07,0.08,0.07}`，XMM 掩码逐位）；0x141a31280（事件 stride 0x18：wall@+8、color u8@+0x10、调 `to_rgb`）|
| 7 | `GraphicsDeviceResource` | **判等** | 0x140de50a0 系统签名 `Res<GraphicsDeviceResource>` → `impl$0::deref()` → `SparseInstanceBuffer<RoofTileSsbo>::gpu_flush`；Deref 面成立。+0x188 管线缓存槽登记 GAP（不冒充）|
| 8 | `WoodenPlankSsbo`/`SparseSsbo`/`SsboType` | **判等** | 0x141a41ba0：`AssetSsboLibrary::get` memcmp 全名 `…roof_buffers::WoodenPlankSsbo`、`SparseInstanceBufferWriter::push<WoodenPlankSsbo>`、`SparseInstanceBufferAllocation<WoodenPlankSsbo>`；Storage=`SparseInstanceBuffer<Self>` 成立。元素 0x20B 尺寸自 struct 布局（12+4+16=0x20）。`CHUNK_COUNT=0x800` 为 sparse_instance_buffer 文档交叉引用（非本次 fresh）|
| 9 | AutoClutter 布局 State/Region/Spawn/Candidate | **判等** | 0x140ee3fd0：State 区域表 @+0x60（RawTable 表项 stride 0x48={key@+0,值@+8}）、pending Vec<u16> @+0x30 cap/+0x38 data/+0x40 len、epoch@+0x90；Region 0x40B（kind@+0、+0x8 Arc、7×f32@+0x18..+0x30、i32@+0x34、u8@+0x38/+0x39、candidate_index@+0x3a、epoch@+0x3c）；`is_equivalent` 逐 7×f32+i32+2×u8 判（NaN 不相等、不含 candidate/epoch）。0x140ee4bf0：候选数组 @+0x20 ptr/+0x28 len、元素 0x98B、kind@+0==4 Arc 释放、+0x88 region_key。Spawn 0x68 布局交叉引用 interaction/clutter RESTORE §类型合同（0x1422ce260 写块）|
| 10 | `utils::calculate_hash` re-export | **判等** | 0x1405bdda0：`Sip13Rounds` write×2 = std `DefaultHasher`（SipHash-1-3 零键）；`country_core::utils` re-export `render::utils::calculate_hash` 成立 |

## 冲突跳过核验

**InsertTreeCmd 0x48 —— 登记准确（冲突跳过成立）**：
- 我 fresh 反编译 `place_tree` 0x142269970，事件写块 stride 0x48 逐字段：`+0 Arc<str> prefab`、`+8 name len`、`+0x10 position`、`+0x20 {scale,u32}`、`+0x28 {variant,rotation}`、`+0x30 flags=1`、`+0x38 id(=next_index)`、`+0x40 height` —— 与总账登记的布局合同一致。
- `resources/history/tree_edit.rs` 现已被 history 组落库语义版 `InsertTreeCmd {id,position,scale,prefab_name}`（非 0x48 字节精确，但那属 history 组域）；coregap-1 未触碰该文件，冲突跳过行为正确。

## §1 ③ SignifierEntityBuilder —— 需修（3 项）

字段/单点方法大多判等：`set_ui_pass`（+0x8a=pass^1、拷 0x90）、`animation_duration`（写+0x78、拷 0x90）、`build` 只 spawn+插 SignifierAnimation 并把组件入队登记 GAP（诚实，不冒充）均 O。

需修项（逐条对回 0x14089d9b0/db00）：

1. **`new` 的 48B 网格载荷含 kind 于首 qword，Rust 未拆分**。二进制 `new` 把 in_RDX[0..5] 整块拷到 +0x10..+0x37（`in_RCX[2..7] = in_RDX[0..5]`），`build` 的 switch 键读 +0x10（即载荷[0]=mesh_kind）。Rust `new(payload)` 把 6 qword 全写进 `mesh_payload`（源序 +0x18）而 `mesh_kind` 保持 0 —— build 键语义丢失，且 `{mesh_kind@+0x10}+mesh_payload[6]@+0x18` 与二进制 48B 块（+0x10..+0x37）差 8 字节。应 `mesh_kind = payload[0]`、载荷区 = payload[1..6]（或等价 48B 块建模）。
2. **`default`/`new` 未安装默认 shader 名 "vertex_color_shader"**。二进制 `new` 分配 19 字节串并写 `{len=0x13@+0x60, ptr@+0x68, cap=0x13@+0x70}`（0x14089d9b0 的 `0x635f786574726576/0x6168735f726f6c6f/0x72656461` 解码 = "vertex_color_shader"）；Rust `default` 置 `shader_len/ptr/cap=0`，`shader_name()` 返回 None。语义差。
3. **`custom_shader` 未释放旧串**。二进制 `if (len@+0x60 != 0) __rust_dealloc(old_ptr)` 后替换；Rust `shader_name_replace` 直接覆盖指针、旧盒泄漏（Drop 只释放当前指针）。对单次使用 builder 影响小，但语义不等。

## §2 ⑤ ReplayBucket.timing / advance_timing —— 需修（1 项）

0x141bc34f0 时序记录偏移全部判等：`flags byte@+8`、`delay f32@+0xc`、`elapsed f32@+0x18`（`==0 && !NaN` 动画门控）、`started byte@+0x1c`；`ReplayBucket` 平行 `Vec<CommandTiming>` 承载（"平行"设计诚实）。

**`advance_timing` 位极性反转**。二进制逐命令时序推进：
```
if ((record[8] & 1) == 0)   → 加入待移除列表（bit0 清 → 移除）
else {                       → delay/speed（下限 0.05）< elapsed 则加入待移除列表；elapsed += delta }
```
即 **elapsed 只对 bit0 置位的命令推进；bit0 清位命令被移除**。Rust `advance_timing` 却 `if flags&1 != 0 { continue }`（跳过置位）、仅推进清位 —— 与二进制相反。需把推进目标改为 `flags.bit0 置位`，并修正 doc 注释 "按 flags.bit0 跳过已完成命令"。

## 观察

- 所有"未做/待证/GAP"登记均诚实（build 组件入队、+0x188 管线缓存槽、PublicWalls 表、RenderInstance 字段序 U、WoodenPlankSsbo 字段序 I），无冒充。
- 类型以 repr(Rust) 语义承载、少数关键事件布局依赖字段序（MoveTreeCmd/PlayUiSound 实测事件步长 0x20 命中；PlayUiSound 为 repr(C)）。如需字节精确/跨 FFI 承接，建议对 byte-critical 类型补 `#[repr(C)]`（不影响当前语义判定）。

## 统计

判等 8 / 需修 2（③ SignifierEntityBuilder、⑤ advance_timing）/ 冲突跳过核验 1（InsertTreeCmd 登记准确）。
cargo check：通过。

## 证据

- `review-evidence/job{1..3}-*.json`（job）+ `batch{1..3}.json`（结果，fresh 反编译）
- `review-evidence/decomp/*.c`：0x14089d8b0/9b0/dad0/dc90/db00/dd50、0x140b34f30/80、0x1405bdda0、0x140de50a0、0x141a31280、0x141be2280、0x142267430、0x14226b1a0、0x142269970、0x141bc34f0、0x140ee3fd0/48c0/4bf0/4a70/4ee0、0x141a41ba0、0x141a284b0
