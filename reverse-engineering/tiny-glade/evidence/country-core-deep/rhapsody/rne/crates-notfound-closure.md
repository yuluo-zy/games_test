# rhapsody crates 域 NotFound 补证 rne-read 原字节（agent=rhapsody-crates-closure，2026-10-15）

程序：B patched `workspace/input/tiny-glade.exe`（SHA cf04827e…）。读法：`scripts/rne-read.py --hex <va> <n>` + capstone 反汇编。
覆盖 crates 域 P0/P1 卡点：vk-sync `get_access_info` 全表、gpu-profiler 时间戳换算、`Type::nbyte` case 表、
`Aftermath::Drop::drop`、`with_suffix_dims`。S_PUB32 权威名以 `evidence/render-architecture/public-symbols.json` 为准。

## 1. vk_sync_fork::get_access_info — 0x140c10dd0（O 全解，三表查询）
Ghidra NotFound。原字节（0x30 行）：
```
48 89 c8 0f b6 ca c1 e1 02 48 8d 15 e8 98 f0 01 8b 14 11
4c 8d 05 de 99 f0 01 46 8b 04 01 4c 8d 0d d3 9a f0 01 42 8b 0c 09
89 10 48 89 40 04 48 89 08 c3
```
手解：
```
mov rax, rcx                    ; rax = 输出指针
movzx ecx, dl                    ; ecx = 字符 c
shl ecx, 2                       ; ecx = c<<2（u32 下标）
lea rdx, [rip+0x1f098e8]         ; 表1 = 0x142b1a6c8
mov edx, [rcx+rdx]               ; out[0] = 表1[c]   ← STAGE 掩码
lea r8,  [rip+0x1f099de]         ; 表2 = 0x142b1a7c8 (=表1+0x100)
mov r8d, [rcx+r8]                ; out[1] = 表2[c]   ← ACCESS 掩码
lea r9,  [rip+0x1f09ad3]         ; 表3 = 0x142b1a8c8 (=表1+0x200)
mov ecx, [rcx+r9]                ; out[2] = 表3[c]   ← image layout
mov [rax], edx; mov [rax+4], r8d; mov [rax+8], ecx; ret
```
- **写 3 个 u32**：out+0=stage 掩码、out+4=access 掩码、out+8=layout。三个表是同一 256×u32 数组（0x142b1a6c8）的
  三段：`[0x00..0x3f]=stage`、`[0x40..0x7f]=access`、`[0x80..0xbf]=layout`；`[0xc0..0xff]` 后是 Rust panic 串。
- **角色判定修正（对照 ALGORITHMS §1.3/§2.3"首字节=access、第二字节=write 才计入的 stage"）**：out[0]=**stage**、out[1]=**access**
  （以 get_memory_barrier 输出 `param_1[0]=uVar10=Σout[0]`、`param_1[6]=uVar12=Σ(写字符的 out[1])` 与公开 h3r2tic/vk-sync-rs
  `AccessInfo{stage_mask, access_mask, image_layout}` 字段序对表验证：char 0x1a=(0x800,0x20) = ComputeShaderReadOther
  的 stage=COMPUTE(0x800)+access=SHADER_READ(0x20)）。**表角色 = {stage, access, layout}**。
- 有效字符域 0x00..0x3f（64 变体）；读 0x00..0x27、写 0x28..0x37（is_write=(c-0x28)<0x10 实测 0x140c10e10）、0x38..0x3f 高位掩码读。
- 64 项全表见本仓 crates/decomp 追加 `crates-get-access-info-table.md`（O）。

### 1.1 写变体（0x28..0x37，S_PUB32 + 公开 vk-sync-rs 枚举序 + country-core 常量对照，O/I）
| char | stage | access | layout | 变体 |
|---|---|---|---|---|
| 0x28 | 0x20000 | 0x40000 | 0 | CommandBufferWriteNVX |
| 0x29 | 0x8(VS) | 0x40(SHADER_WRITE) | 1(GENERAL) | VertexShaderWrite |
| 0x2a | 0x10(TC) | 0x40 | 1 | TessControlShaderWrite |
| 0x2b | 0x20(TE) | 0x40 | 1 | TessEvalShaderWrite |
| 0x2c | 0x40(GEOM) | 0x40 | 1 | GeometryShaderWrite |
| 0x2d | 0x80(FS) | 0x40 | 1 | FragmentShaderWrite |
| 0x2e | 0x400(COLOR_OUT) | 0x100(COLOR_W) | 2(COLOR_ATTACH) | ColorAttachmentWrite |
| 0x2f | 0x600 | 0x300(DEPTH_R\|W) | 3(DEPTH_ATTACH) | DepthStencilAttachmentWrite |
| 0x30 | 0x600 | 0x300 | sentinel(0x3b9c9309) | DepthAttachmentWriteStencilReadOnly |
| 0x31 | 0x600 | 0x300 | sentinel(0x3b9c9308) | StencilAttachmentWriteDepthReadOnly |
| 0x32 | 0x800(COMPUTE) | 0x40 | 1 | ComputeShaderWrite（country-core `ACCESS_CPU_MAPPED_WRITE`=0x32 O） |
| 0x33 | 0x10000(ALL_CMDS) | 0x40 | 1 | AnyShaderWrite |
| 0x34 | 0x1000(TRANSFER) | 0x1000(TRANSFER_W) | 7(TRANSFER_DST) | TransferWrite（country-core `ACCESS_TRANSFER_WRITE`=0x34 O） |
| 0x35 | 0x4000(HOST) | 0x4000(HOST_W) | 1 | HostWrite |
| 0x36 | 0x400 | 0x180(COLOR_R\|W) | 2 | ColorAttachmentReadWrite |
| 0x37 | 0x10000 | 0x18000(MEM_R\|W) | 1 | General（country-core `ACCESS_WRITE`=0x37 O） |

### 1.2 读变体（0x00..0x27，锚点 = 公开枚举序 + layout 列 + country-core 常量）
- 0x00 Nothing；0x01 CommandBufferReadNVX；0x02 **IndirectBuffer**（country-core `ACCESS_INDIRECT_READ`=0x02 O，
  access=0x21=INDIRECT_COMMAND_READ\|SHADER_READ，stage=0x10000）；0x03 IndexBuffer（VERTEX_INPUT, INDEX_READ）；
  0x04 VertexBuffer（VERTEX_INPUT, VERTEX_ATTRIBUTE_READ）。
- 逐 stage 三连（uniform→0x20, sampled→0x20+layout5, other→0x20+layout1，fork 对 uniform/other 读统一用 SHADER_READ=0x20）：
  0x05/06/07=VS、0x08/09/0a=TC、0x0b/0c/0d=TE、0x0e/0f/10=GEOM、0x11/12/15=FS、0x13=FS color input(layout5)、
  0x14=FS depth/stencil input(layout4)、0x18/19/1a=COMPUTE（0x1a=**ComputeShaderReadOther**=country-core `ACCESS_READ`=0x1a O）、
  0x1b..0x24=AnyShader 六连（含 fork 新增 6 个读变体，精确名 U，layout 列 0/5/4/1 可见）。
- 0x16 ColorAttachmentRead（COLOR_OUT, COLOR_READ, layout2）；0x17 DepthStencilAttachmentRead（EARLY\|LATE, DEPTH_READ, layout4）；
  0x25 TransferRead（layout6=TRANSFER_SRC）；0x26 HostRead（layout1）；0x27 Present（layout 哨兵 0x3b9acdea，access/stage 0）。
- **fork 与公开 vk-sync-rs 差异**：写区间由公开的 0x22..0x31 整体后移 +6 到 0x28..0x37（中间插入 6 个读变体）；
  uniform/input 读合并为 SHADER_READ；stage 掩码沿用 stage-1 位值（0x2000=BOTTOM_OF_PIPE 默认、0x10000=ALL_COMMANDS）。

## 2. with_suffix_dims — 0x140c87650（O 升级；decomp NotFound 但 rne-read 证实）
decomp 为 `{"error":"No function at 0x140C87650"}`。原字节：
```
48 8d 41 01 c3
```
手解：`lea rax, [rcx+1]; ret` = **`self.0.wrapping_add(1)`**，与 rebuild 完全一致。
S_PUB32：`shader_pipeline::reflection::BindingNameHash::with_suffix_dims`（1 符号）。verdict O/I → **O（rne-read）**。

## 3. gpu-profiler 时间戳换算（P1，O 全解）
| 函数 | VA | 原字节 | 语义 |
|---|---|---|---|
| `GpuTick::duration_since` | 0x140c10e50 | `xor eax,eax; sub rcx,rdx; cmovae rax,rcx; ret` | `if self>=earlier {self-earlier} else {0}` 饱和差 |
| `GpuTickDuration::nanoseconds` | 0x140c10e60 | `movq xmm0,rcx; punpckldq ...; cvtss2sd xmm0,xmm1; mulsd; cvttsd2si; 边界 cmov` | `(self as f64 * period_f32 as f64) as u64`，period 为 **f32 第二参数（xmm1）** |
| `NanoSecond::from_raw_ns` | 0x14067dd80 | `mov rax,rcx; ret` | newtype 恒等（S_PUB32 同址折叠 raw_ns） |
| `MarkerId::from_u64` | 0x140c10e20 | cmp -1 / je panic | 非 -1 截断 32 位；==u64::MAX panic（ALG 已有） |
| `FrameMarkers::marker_name` | 0x140c10ee0 | `mov rdx,[rcx+0x10]; cmp rdx,ecx; jbe panic; ...; shl rcx,4; mov rax,[rdx+rcx]; mov rdx,[rdx+rcx+8]; ret` | 返回**完整 &str {ptr,len}**（纠 ALG §4.7"返回 8B 丢 len"）；len>idx 越界 panic |
| `FUN_140bd3c80`（profiler 数据读取器） | 0x140bd3c80 | `call 0x140c113c0; test; 空则 panic` | 返回全局 profiler 的 u64 时间戳值数组；0x140c113c0 读全局+0x30 有效位/返回+0x18 数据指针 |
| `FUN_1405c3580` / `FUN_1405c75b0` | — | sort 内部 | **纯排序**（小/大样本 std slice sort），非 ticks→ns 转换；条目 16B={u32 marker, u64 tick}，按键=u64 |

`retrieve_results`（0x140294cf0，O）：条目 = `(MarkerId, GpuTick)` 16B，配对复制后按 GpuTick `sort_unstable_by_key` 排序；
结果结构 param_1[0..2]=Vec{cap,ptr,len}+帧元数据拷贝。**period 来源 = rhapsody::raw::profiler::timestamp_period**
（skeleton ash.rs:41 PDB 内联；对应 VkPhysicalDeviceLimits.timestampPeriod，f32/f64 纳秒/刻度，游戏侧渲染初始化查询）——
即 ALG §4.6"ticks→ns 的 period 来源 U"闭合：period 不作为全局常量存在，而是 raw profiler 传入 nanoseconds() 的 f32 参数。

## 4. Type::nbyte / ScalarType::nbyte（P1，case 表 O）
`Type::nbyte`（0x1420bda00，O）：`xor rax,0x8000000000000000` 判别后跳表（15 项，default=case 10）。case 表：
| case | 变体 | 行为 |
|---|---|---|
| 0 | Scalar(ScalarType) | 子跳表 → (1, 宽度) |
| 1 | Vector(VectorType) | 子跳表 → (1, 分量尺寸×nscalar) |
| 2 | Matrix(MatrixType) | 子跳表 → (1, 尺寸) |
| 3..8 | Image/CombinedImageSampler/SampledImage/StorageImage/Sampler/SubpassData | (0,0) |
| 9 | Array(ArrayType) | 读 [rcx+8] Option 标记 / [rcx+0x10]；返回 1（语义 I） |
| 10 | Struct(StructType) | **递归**：members 沿 +0x50 步长，member.ty@+0x18（=0x50-0x38）递归累加，返回 (1, Σ) |
| 11 | AccelStruct | (0,0) |
| 12 | DeviceAddress | (1, 8) |
| 13 | DevicePointer | (1, 8) |
- **rebuild Type 枚举序 = 机器序**（0..13 完全一致，O 证实）。
- 返回约定 = (rax 非零标记, rdx 尺寸/累加)；对 opaque=0、复合/地址=1、DeviceAddress/Pointer=8。
- `ScalarType::nbyte`（0x1420bd380，O）：跳表 Void(0)→0、Boolean(1)→1、Signed/Unsigned/Float(2/3/4)→**返回 payload u32（宽度字段原值）**。
  字节：`mov eax,[rcx]; lea rdx,[rip+0xf1bbd3]; movsxd r8,[rdx+rax*4]; add r8,rdx; jmp r8`。
  bits-vs-bytes 契约：payload 由未恢复的原 producer 写入（spirq 存位宽）；Boolean=1/DeviceAddress=8 暗示 payload 为字节数（I），
  消费端 = raw `GraphicsDevice::create_raster_pipeline`（0x16a3b40 调 ScalarType::nbyte、0x16a3ba2 调 Type::nbyte，S_PUB32 静态直调）。

## 5. Aftermath Drop 与初始化函数身份（P1，O）
- `Aftermath::Drop::drop` **@0x140c107b0 = `jmp 0x142923880`（5B 尾调）**；0x142923880 = import thunk
  `jmp qword [IAT]`（IAT 槽 0x143192d38 → 目标 0x3194aaa）。S_PUB32：0x142923880 = **`GFSDK_Aftermath_DisableGpuCrashDumps`**。
  → Drop = **尾调 GFSDK_Aftermath_DisableGpuCrashDumps()**；0x38B Aftermath 对象内存释放由 std drop 胶水另行处理
  （`core::ptr::drop_in_place<AftermathUserData>` @0x1400c04b0）。
  **修正 ALG §2.4/§4.8 悬空引用**：`decomp/extra-0x140c107b0.txt` 不存在；证据为本文 rne-read 原字节（O）。
- `Aftermath::new` 的初始化函数 0x142923580（S_PUB32）= **`GFSDK_Aftermath_EnableGpuCrashDumps`**；调用
  `0x142923580(0x217, 2, 1, GpuCrashDumpCb, ShaderDebugInfoCb, DescriptionCb, ResolveMarkerCb, userData)`——
  ALG §4.9"opcode 0x217"实为第一参数（Aftermath API 版本/特性位），(2,1) 为版本；S_PUB32 定死函数身份。
- `get_shader_debug_info_identifier` 目标 0x142923870（S_PUB32）= **`GFSDK_Aftermath_GetShaderDebugInfoIdentifier`**。
- `wait_for_status`（0x140c104e0）轮询目标 = 0x142923860（S_PUB32）= **`GFSDK_Aftermath_GetCrashDumpStatus`**；
  退出值 2/4 = `GFSDK_Aftermath_CrashDumpStatus` 枚举成员（FinishedNoCrashDump/NotEnabled 等，精确名对照 Aftermath SDK 头为 I）。

## 6. CompiledShaderPipeline wire 布局（P0，O 分段反编译 extra-0x1409e4670）
- **无 magic/版本头**：缓冲区首 4 字节即 stages 计数（`*(uint*)param_2`），不扫描 SPIR-V magic（rebuild 注释"绝不搜索任意偏移的 SPIR-V magic"与机器码一致）。
- 最小长度门 `param_3<0x1e`（30B）= 空流水线编码：stages 4 + source_deps 4 + descriptor_sets 4 + uniform_block tag 1 +
  binding_names 4 + binding_name_hashes 4 + vertex_inputs 4 + local_size tag 1 + push_constant_size 4 = **30**（O，逆向印证反射字段序）。
- 读取序：**stages → source_dependencies → reflection**。
  - stages：u32 count × 0x38B 元素；每元素 = {u32 stage(≤2), String entry_point(len+bytes+from_utf8), Vec<u32> spirv(count×4)}。
  - source_dependencies：u32 count × 24B（Vec<String>），每 String len+bytes。
  - reflection = ShaderReflection 七字段：descriptor_sets(外层/内层两级计数) → uniform_block(Option u8 tag) →
    binding_names → binding_name_hashes → vertex_inputs → local_size(Option u8 tag) → push_constant_size u32。
- **尾字节保留**：末字段读完后剩余字节不消费、不报错（speedy read_from_buffer 行为；rebuild `trailing_bytes` 建模正确）。
- 返回 Result<0x140B 结构, anyhow>：Ok={0 判别+0x140B}（load_compiled `memcpy(param_1,&stk,0x140)`）、Err={0x8000000000000000@0, err@8}。
  0x140B 载荷 = {stages Vec@0x00, source_deps Vec@0x18, ShaderReflection@0x30(0x110B)}。
- 与 CALL-CHAIN 0x130/0x150：0x140 为本次 Ok 复制长度（权威）；0x130=旧版结构/GetDesc1 长度同值巧合，0x150=Arc 块，不冲突。

## 7. DxgiAdapter 40B 字段映射（P2，O）
enumerate（0x1418b29c0）元素写（0x1418b2c57..71）：
```
mov [rax+rcx*8], rdx        ; +0x00 = name.ptr
mov [rax+rcx*8+8], rdx      ; +0x08 = name.len
mov [rax+rcx*8+0x10], rdi   ; +0x10 = name.cap
mov [rax+rcx*8+0x18], rsi   ; +0x18 = CONCAT([rbp+0xdc],[rbp+0xd8]) = desc1+0x128 = AdapterLuid (u64)
mov [rax+rcx*8+0x20], r15d  ; +0x20 = [rbp+0xb0] = desc1+0x100 = VendorId (u32)
```
- desc1 栈拷贝基址 = rbp-0x50（WCHAR 扫描 `[rbp+r8*2-0x56]` 对表确认）。**40B 元素 = {name: std String{ptr,len,cap}@0,
  AdapterLuid u64@0x18, VendorId u32@0x20}**（+0x24 补齐到 0x28）。
- **修正 ALG §4.11"u32 id 字段 @0x10..0x28 含 vendor/device/subSys/rev/dedicated"**：只取 **VendorId + AdapterLuid**，
  deviceId/subSysId/Revision/dedicated memory 不入元素。Err 判别 0x8000000000000000 与 load_compiled 相同（判别与载荷共用 offset0 的 niche）。

## 8. defines 哈希（P2，U 保留；输入 defines 未知）
- 发行 `compiled-shaders/` 共 404 个 .bin（与 MODULE_STATUS 404 相符）：compute 键 `{path}_{entry}_0.bin`（hash 恒 0=空 defines）；
  raster 键 `{path}_{hash:x}.bin`，**全部 234 个 raster 只有两个 hash**：`b903e43ffb3da915` / `f0adff76827e73ff`（117+117，
  每 raster 两变体）——defines 为全局同一组、单一宏开/关。
- rebuild `artifact_hash`（DefaultHasher 对每 (String,String) 哈希后 XOR）空列表=0，与 compute hash 0 一致；
  **但 FNV-1a64 与 SipHash-1-3(0,0) 对常见 PERM_* 宏名单/值（0/1/空）暴力测试均未复现两 hash** → 宏名/值本身未知，
  算法（XOR-fold 候选）与输入均 **U**，待真实 defines manifest（MODULE_STATUS：38 非零 compute key 已核对、170 宏 tuple 未独证）。
