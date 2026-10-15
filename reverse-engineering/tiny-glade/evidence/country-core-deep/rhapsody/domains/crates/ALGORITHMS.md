# rhapsody crates 域 ALGORITHMS.md（shader-pipeline / gpu-profiler / vk-sync-rs / aftermath-rs / dxgi-enumerate-gpus）

- 日期：2026-10-15。基线：B patched EXE（SHA cf048e27...）；S_PUB32 权威 = `evidence/render-architecture/public-symbols.json`。
- 范围：rhapsody workspace 的 5 个独立子 crate（crates.json，46 compiled / 28 有地址）。
- 标注：**O**=机器码/PDB 实测（含反编译、S_PUB32、skeleton 依赖注解），**I**=推断，**U**=未知。每行结论带标注。
- 反编译：`domains/crates/decomp/`（28 个批量）+ `extra-*.txt`（本轮补取 7 个：load_compiled ×2、retrieve_results、ActiveProfiler::end_frame、Aftermath::new、Aftermath::Drop::drop NotFound、3 个 helper）。
- rebuild 对照：`reconstruction/rhapsody-rebuild/src/shader_pipeline/`（镜像实现）；gpu-profiler/vk-sync-rs/aftermath-rs/dxgi **rebuild 无对应实现**（Cargo.toml/Cargo.lock 均无这 4 个依赖，亦无镜像模块）。

## 0. 域结论速览

1. **shader-pipeline 是"编译产物文件加载 + 反射存储 schema + 管线描述"三件套**，只负责 `.bin` 文件的读取/解码与描述构造，**不启动编译**（编译在外部 shader-compiler/重建候选 shader_compiler 中）。核心链：`load_compiled<D>`（0x1409e3c50/0x1409e4160）= `compiled_file_path` → `std::fs::read` → `CompiledShaderPipeline::read_from_buffer`（helper 0x1409e4670）→ Ok 载荷 0x140B。
2. **rebuild shader_pipeline/ 与原始大体对得上，但有两处结构偏差**：rebuild `RasterPipelineDesc` 是 4 字段（source_path/vertex_entry/fragment_entry/defines），原机器码是 **3×24B = 0x48B**（`new_vertex_fragment` 把 path 克隆两份 @+0x00 与 @+0x30，@+0x18 是空 defines {0,8,0}）；rebuild `CompiledShaderPipeline` 用 `trailing_bytes` 保存未消费尾字节，与原 speedy 解码后保留尾字节的行为一致（字段建模方向对，但 0x140B 载荷内 reflection 字段序未逐字段 O 证）。
3. **gpu-profiler 的 fork 与公开 gpu-profiler crate 结构同源**（Profiler→ActiveProfiler→ProfilerSubmittedFrame→retrieve_results 四层），但 **marker 是 `&str` 16B 条目且 create_marker 不做名字去重**（直接 push，返回序号）；timestamp query 池 1024 个、每帧 marker 上限 0x3ff=1023。rebuild 未引入 gpu-profiler，只有候选 `raw/profiler.rs::GpuProfiler`（QueryPool + span 名），与原链不等价。
4. **vk-sync-rs 是 h3r2tic/vk-sync-rs 的分叉（`vk_sync_fork`）**：AccessType 用 **u8 字符编码**（is_write = (c-0x28)<0x10，即写类型占 0x28..0x37），三个 barrier 转换函数产出**自定义 32/56/72B 结构**（非裸 Vulkan 类型），由 device vtable +0x368 统一下发。
5. **aftermath-rs 是 NVIDIA Aftermath 的 Rust 封装**：`Aftermath::new`（0x140176e70）分配 0x38B 对象并调用 Aftermath 初始化函数注册 4 个 C 回调（GpuCrashDump/ShaderDebugInfo/GpuCrashDumpDescription/ResolveMarker），回调转发到 `NvAftermathDelegate` trait 对象 vtable +0x18/+0x20/+0x28/+0x30。游戏侧接线 = `tiny_glade::aftermath::NvAftermathDelegate`（tiny_glade 域，非本域）。rebuild 无 aftermath。
6. **dxgi-enumerate-gpus**：`enumerate`（0x1418b29c0）= CreateDXGIFactory → EnumAdapters1 → GetDesc1 → 逐 GPU 生成 `DxgiAdapter`（40B 元素：name String + id 字段），输出 `Result<Vec<DxgiAdapter>, DxgiError>`。rebuild 无 dxgi。

## 1. 模块总览

### 1.1 shader-pipeline

文件→职责：
- `compiled.rs`：`CompiledShaderPipeline`（发行 `.bin` 解码产物）、`ShaderCompilerArtifacts::load_compiled<D>`（文件加载入口，泛型于 PipelineDesc）。
- `pipeline_desc.rs`：`ComputePipelineDesc` / `RasterPipelineDesc`（编译输入描述 + `compiled_file_path` 文件键）。
- `reflection.rs`：`ShaderReflection`（存储反射 schema）、`BindingNameHash`（绑定名→u64 哈希键）。
- `reflection_ty.rs`：`Type`/`ScalarType`/`VectorType`/`MatrixType`/`ArrayType`/`StructType` + `nbyte()` 尺寸查询。
- `shader_defines.rs`：`ShaderDefines`（定义列表，`Vec<(String,String)>`，只 clone/eq，无独立 hash）。

**CompiledShaderPipeline 载荷 0x140B（O：load_compiled Ok 路径 `memcpy(param_1, &stk, 0x140)`，decomp extra-0x1409e3c50）**。CALL-CHAIN 记 0x130B 载荷/0x150B Arc 块，两值不同源；0x140 是本次实测的 Ok 复制长度。wire 结构（skeleton `impl$2::read_from` line 11 与 `impl$4::read_from` line 18 两个 speedy Readable）：
- stages（`Vec<CompiledShaderStage>`，stage 枚举+entry_point String+spirv Vec<u32>）、source_dependencies（Vec<String>）、reflection（ShaderReflection）。
- 尾字节保留（`impl$4::read_from` 读 2×read_u8 哨兵/收尾；rebuild 用 `trailing_bytes` 建模）。

**ShaderReflection 字段（rebuild 对齐；原为 HashMap 语义，rebuild 用有序 Vec<Vec<(u32,Binding)>> 保持发行顺序）**：descriptor_sets、uniform_block(Option)、binding_names、binding_name_hashes、vertex_inputs、local_size(Option<[u32;3]>)、push_constant_size。skeleton `impl$94::read_from`(line 88) 用 `read_key_value_collection`/HashMap from_iter + 3 层闭包，证实原为 HashMap；`impl$16::eq`(line 15)+`impl$0::hash`(line 19) 证实原 ShaderReflection 实现 Eq+Hash（rebuild 只 derive Eq，无 Hash，差异点）。

**BindingNameHash（O 全解，decomp 0x140c87660 + 0x141937610）**：
- `new(name)`：检查末尾 `_dims`（len≥5 且 char[len-5..len-1]==`_dims`）→ limit=len-5 且结果 +1；否则 limit=len（全串）+0。边界 `_dim`（无 s）→ 全串 +0（机器码与 rebuild `strip_suffix("_dims")` 完全一致）。
- 哈希函数 = **FNV-1a 64**（0x141937610：offset basis 0xcbf29ce484222325、prime 0x100000001b3，按 4B 展开+尾部逐字节；第三个参数 bit0=是否用 limit，第四个=limit），与 rebuild `const_fnv1a_hash::fnv1a_hash_64(name, limit)` 一致。✓✓
- `with_suffix_dims`（0x140c87650，Ghidra NotFound 短函数）＝ `hash.wrapping_add(1)`（rebuild 同）。

**RasterPipelineDesc 0x48B（O，decomp 0x1409e3980）** = 3×24B：`@+0x00` String（path 克隆，经 helper 0x140d7bd90=String clone）、`@+0x18` 空 defines（game-Vec {cap=0, ptr=dangling(8), len=0}）、`@+0x30` String（path 克隆）。`new_vertex_fragment(path)`：分配 len 字节拷贝 path 两次。字段语义 I（`@+0x00` 与 `@+0x30` 谁是 source_path/entry 未定）。country-core 合约 `{path, entry: String, defines}` 是 3×24B 形状，但**机器码是"两份 path 字符串 + 空 defines"**，与"path/entry/defines"的语义映射 U。

**pipeline_desc.rs line 3 `impl$8::read_from`**：读单个 u32 的 newtype（I：可能是文件键/入口枚举标签），字段 U。

**ComputePipelineDesc**：`new`(line 55) / `with_entry`(line 63)。`with_entry<ref<str>>`（0x140539370，另实例 0x1409e3ab0）机器码：若 entry 为空则用默认 "cs_main"（字面量 `\x6e69616d\x6d5f7363` = "cs_main" 逆向存），否则 `str.to_owned`；结构 = {source_path String, entry String, defines}。文件键 `compiled-shaders/{source_path.replace('/','_')}_{entry}_{defines_hash:x}.bin`。

**defines 哈希**：原 `calculate_shader_defines_hash`(compiled.rs line 76) + `calculate_hash<tuple<String,String>>`(line 87)，均为 inline/无地址（不能直接 O 证算法）；rebuild 用 std `DefaultHasher`（SipHash）对每对 (String,String) 哈希后 XOR。哈希算法与键语义 **U**（MODULE_STATUS：38 非零 compute 原 key 已核对、170 宏 tuple 未独证）。

**反射类型 nbyte**：`Type::nbyte`（0x1420bda00）机器码：判别字 `*p ^ 0x8000000000000000` 分发；Scalar/Vector/Matrix → 子表派发（ScalarType::nbyte 0x1420bd380 = 分派表，Ghidra NotFound）；Image/CombinedImageSampler/SampledImage/StorageImage/Sampler/SubpassData/AccelStruct → 0；Array/Struct/DeviceAddress/DevicePointer → 递归（Struct 沿 members 指针链递归，StructMember=0x50B，ty 字段 @+0x18）。**case→variant 的精确映射 I**（枚举顺序未 O 证），递归累加细节 U。rebuild `Type::nbyte()` 走 `convert(true)`→spirq 的 `nbyte()`，与机器码结构一致但尺寸来源不同（rebuild 经 spirq 计算，原为自算）。

### 1.2 gpu-profiler（shared.rs + backend/ash.rs）

**MarkerId**（shared.rs line 11/15）= u32 包装；`from_u64`（0x140c10e20）：u64≠0xffffffffffffffff → 截断低 32 位返回；==0xffffffffffffffff → panic（u32::MAX 是非法 marker 哨兵）。`as_u64` 有 **1177 个直接调用方**（O，skeleton enrichment），是全域最热类型之一。

**FrameMarkers**（shared.rs line 82/92）= {cap@0, ptr@8, len@0x10}，元素 16B = `&str`（ptr,len）。`marker_name`（0x140c10ee0）：`return *(ptr + idx*0x10)`（返回 name 字符串指针；越界 panic）。`create_marker` = push，无去重（内联进 ActiveProfiler::create_marker，见下）。

**Profiler（backend::ash，0x140c10f20 new）** = {query_index_buf_ptr@0, capacity 0x3ff@8, query_pool@0x10, used_count@0x18}（O）。new：`vkCreateQueryPool`（device vtable +0xf0；sType=QUERY_POOL_CREATE_INFO、queryType=TIMESTAMP=11、queryCount=0x400=1024），分配 0x1ff8B 固定缓冲（0x3ff×8+8，写标记时存 query 索引，非时间戳），失败 panic。

**Profiler::begin_frame**（0x140c11090）：`vkCmdResetQueryPool(cmd, pool, 0, 0x400)`（vtable +0x380），used_count=0，返回 ActiveProfiler 视图 {cap=0, ptr=8, len=0, profiler*@0x18}。

**ActiveProfiler::create_marker**（0x140c110f0，游戏侧在 record_batch 内调用）= {cap@0, ptr@8, len@0x10, profiler@0x18}；满时 grow（FUN_1418cf470）；push (name_ptr, name_len) 16B，返回 index（u32）。**返回即 MarkerId**。

**ActiveProfiler::write_marker**（0x140c11150）：LOCK/UNLOCK 原子自增 used_count；把 query index 写入 timestamp_buf[count]；调 `vkCmdWriteTimestamp`（vtable +0x388，stage=0x2000）。（注：写的是索引数组，非时间戳。）

**ActiveProfiler::end_frame<ProfilerBuffer>**（0x1418c3910）：`vkCmdCopyQueryPoolResults(cmd, pool, 0, count, dstBuffer=*param_5, offset=0, stride=8, flags=64_BIT|WAIT=3)`（vtable +0x390）；从 Profiler 固定缓冲构建 query_indices Vec；输出 ProfilerSubmittedFrame {markers Vec@0, param_5 拷贝@0x18..0x68, query_indices Vec@0x70, count@0x78}（偏移 I）。

**ProfilerSubmittedFrame::retrieve_results<ProfilerBuffer>**（0x140294cf0，游戏侧 render R6 调用）：从 frame 的 u32 timestamp 数组（frame+0x70，count@0x78）与一个 profiler 全局 u64 数组（0x140bd3c80 取址）配对复制为 16B 条目 Vec；含 per-frame 边界校验（cap 越界 panic：消息 0x5a=90 字符 @0x1429ab580、容量 @0x1429ab5f8）；输出 Result 结构含 Vec + 元数据。FUN_1405c3580/0x1405c75b0 = 对结果做转换/排序（I：ticks→nanoseconds 用 timestamp period）。

**FUN_140bd3c80** = profiler 全局读取器（0x140c113c0 原子取指，空则 panic），语义 I（timestamp period 或结果缓冲）。

处理链：`Profiler::new`（建 pool）→ `begin_frame`（重置+取 ActiveProfiler）→ 逐 pass `create_marker(name)` + `write_marker`（push 名字/写查询索引 + vkCmdWriteTimestamp）→ `end_frame`（CopyQueryPoolResults + 打包 SubmittedFrame）→ 游戏 `retrieve_results`（取回 16B 条目供 puffin）。rebuild 的 `raw/profiler.rs::GpuProfiler` 只覆盖"new + 时间戳池 + span 名"，无 marker 链/结果回读。

### 1.3 vk-sync-rs（`vk_sync_fork`）

- `AccessType` = **u8 字符枚举**（O：is_write = `(byte)(c - 0x28) < 0x10`，0x140c10e10）。写类型判别字 0x28..0x37；读类型 0x00..0x27。字符到 (accessMask, stageMask) 的映射在 `get_access_info`（0x140c10dd0，NotFound，两个输出 u32：首字节=access、第二字节=write 才计入的 stage）。
- `get_memory_barrier`（0x140c107c0）：把名字串逐字符 get_access_info 后 OR 汇总；输出 32B 自定义 MemoryBarrier {accessMask(+0 防零), stageMask(默认 0x2000=TRANSFER), sType 0x2e(=VK_STRUCTURE_TYPE_MEMORY_BARRIER_2), pNext=0, srcAccess=0, dstAccess=0, srcStage=汇总, dstStage=write 汇总}。
- `get_buffer_memory_barrier`（0x140c10940）：同上 + 尾部拷贝输入 buffer 字段（+0x20..+0x38 → 输出 +0x28..+0x40）；sType 0x2c(=BUFFER_MEMORY_BARRIER_2)；输出 0x48B 结构。
- `get_image_memory_barrier`（0x140c10af0）：同上 + oldLayout/newLayout（输入 +0x44/+0x45/+0x46 字节布尔判别，newLayout 有 '\'' → 0x3b9acdea 哨兵）、queue family、subresourceRange（输入 +0x30..+0x38 → 输出 +0x38..+0x40）、image（输入 +0x40 → 输出 +0x48）；sType 0x2d(=IMAGE_MEMORY_BARRIER_2)；输出 0x50B 结构。
- `cmd::pipeline_barrier`（0x1418afd20，7.6KB）：输入 3 组 barrier（memory×0x38、buffer×0x40、image×0x48），逐组转换、OR 汇总 access/stage，一次调 device vtable +0x368 下发；含 Vec 增长（FUN_1418b0310/0x140...）。

产出结构不是裸 Vulkan 类型而是 fork 自有的 "barrier 描述"，由 rhapsody `GraphicsDevice` 侧统一翻译下发——这是 fork 对公开 vk-sync-rs 的主要改造。rebuild 无 vk-sync 依赖；`rg/resource_rules.rs:41` 注释明确"原 vk_sync_fork::AccessType::is_write；本地依赖版本未核实，故显式接收分类器"——即 rebuild 未恢复 vk-sync 枚举，属**候选替代**。

### 1.4 aftermath-rs

- `Aftermath::new<NvAftermathDelegate>`（0x140176e70）：分配 0x38B Aftermath 对象并整体拷贝 delegate/user-data（0x38B）；分配 0x10B user_data 包装 {data_ptr, vtable(UNK_1429603f8)}；调 Aftermath 初始化函数（0x142923580，opcode 0x217，版本参数 2,1）注册 4 个回调 + user_data + vtable。
- 回调（全部转发到 delegate trait 对象 vtable）：
  - `GpuCrashDumpCallback`（0x140c106c0）：C `(pDump, size, userData)` → `vtable[+0x18](self, pDump, size)`。
  - `ShaderDebugInfoCallback`（0x140c10710）→ `vtable[+0x20](self, pInfo, size)`。
  - `GpuCrashDumpDescriptionCallback`（0x140c10760）→ `vtable[+0x28](self, &pDescription)`（GFSDK_Aftermath_GpuCrashDump_AftermathEvent 透传）。
  - `ResolveMarkerCallback`（0x14003bb80）→ Ghidra NotFound、ICF 折叠（S_PUB32 437 条同址）；按序应为 `vtable[+0x30]`（I）。游戏侧 `NvAftermathDelegate::description` 也折叠到 0x14003bb80（S_PUB32，O）。
- `Status::get`（line 55，646 个直接调用方）：读取 Aftermath status 值（I：GFSDK_Aftermath_GetStatus 包装）。
- `Status::wait_for_status`（0x140c104e0）：循环轮询 status，目标值 2/4（success 判别），每轮 sleep 50ms（0x140d55980），总超时 1s 或调用者 u64 纳秒超时；超时 panic（消息 0x1e=30 字符 @0x142b1a5d0）。
- `get_shader_debug_info_identifier`（0x140c10620）：调 Aftermath 函数（0x142923870，opcode 0x217）传 (shader_binary, size) → identifier {low,high}；非 Success 构建错误（0x140d5caa0）返回 Err；Ok 判别字=1，载荷 16B（Result 0x18B）。
- `sys.rs` line 41：struct derive Default/Debug/Hash/PartialEq（I：`ShaderDebugInfoIdentifier` Aftermath 标识类型）。
- 游戏侧接线（非本域但为下游）：`tiny_glade::aftermath::NvAftermathDelegate::{description, shader_debug_info, dumped, default}`（0x14003bb80/0x140245340/0x140244790/0x140244460）；`rhapsody::raw::device::GraphicsDevice::log_gpu_crash_info` 内联宿主一个 aftermath 函数（skeleton enrichment，O）。MODULE_STATUS「GPU诊断」行：原 shader 诊断 producer、vendor dump、实际 device-lost 未验收。

### 1.5 dxgi-enumerate-gpus

`enumerate`（0x1418b29c0，6.3KB）：
1. `CreateDXGIFactory(&IID)`；失败 → Err(DxgiError)。
2. 循环 `IDXGIFactory1::EnumAdapters1(factory, index, &adapter)`（0x14213b8e0），index 递增。
3. 每个 adapter：`IDXGIAdapter1::GetDesc1`（0x14213b950）→ 拷贝 0x130B DXGI_ADAPTER_DESC1 到栈；Description WCHAR[128] 扫描首个空项（short==0）求字符串长度（步进 4 short、上限 0x100）。
4. `get_string_from_wide`（inline + closure$0）：WCHAR 数组 → String（uVar8-(uVar8>>1)=字符数）。
5. push `DxgiAdapter`（**40B 元素**：{name String@0, 后续 u32 id 字段 @0x10..0x28，含 vendor/device 等，字段精确映射 I}）；Vec 满则 grow（0x140562ab0）。
6. 结束返回 Ok(Vec)；Factory 创建失败 → Err(DxgiError)。`Result<Vec<DxgiAdapter>, DxgiError>`：Ok = 24B game-Vec {cap,ptr,len} 直写；Err = {判别 0x8000000000000000@0, error@8}（O：与 load_compiled 相同 Err 判别模式）。
- 类型名 O：`DxgiAdapter`（S_PUB32 `drop_in_place<Vec<DxgiAdapter>>` @0x14014a860、`<IntoIter<DxgiAdapter>>` @0x14087b030）。

## 2. 逐函数表

格式：`函数（骨架名）| 地址 | role | 算法/行为要点 | O/I/U | evidence`

### 2.1 shader-pipeline

| 函数 | 地址 | role | 算法要点 | O/I/U | evidence |
|---|---|---|---|---|---|
| `compiled::ShaderCompilerArtifacts::load_compiled<ComputePipelineDesc>` | 0x1409e3c50 | 编译产物加载（compute） | `compiled_file_path`(desc)→fs::read→read_from_buffer(0x1409e4670)→Ok 0x140B；两类失败各带 path 上下文 error（"Failed opening/Failed deserializing compiled shader pipeline from {path}"） | O | extra-0x1409e3c50、skeleton compiled.rs:27–38 |
| `compiled::ShaderCompilerArtifacts::load_compiled<RasterPipelineDesc>` | 0x1409e4160 | 同上（raster） | 与 compute 实例同构同体（Ghidra 输出相同长度） | O | extra-0x1409e4160 |
| `compiled::impl$2::read_from`（line 11） | — | wire 解码 | 读 stages/source_dependencies/reflection（read_vec×2/read_u32×3/String） | O | skeleton compiled.rs:11 |
| `compiled::impl$4::read_from`（line 18） | — | 全结构解码 | 尾字节保留 + HashMap 反射读取 | O | skeleton compiled.rs:18 |
| `compiled::impl$0::compiled_file_path`（line 49） | — | compute 文件键 | `compiled-shaders/{path}_{entry}_{hash:x}.bin`（path `/`→`_`） | I | skeleton compiled.rs:49 |
| `compiled::impl$1::compiled_file_path`（line 63） | — | raster 文件键 | 同上前缀、**无入口名**（rebuild 注释"原 raster 文件键没有入口名"） | I | skeleton compiled.rs:63 |
| `compiled::calculate_shader_defines_hash`（line 76） | — | defines 哈希 | inline；算法 U（rebuild 用 DefaultHasher XOR） | U | skeleton compiled.rs:76 |
| `compiled::calculate_hash<tuple<String,String>>`（line 87） | — | 单对哈希 | compiled_procedure 无地址；算法 U | U | skeleton compiled.rs:87 |
| `pipeline_desc::RasterPipelineDesc::new_vertex_fragment<ref<str>>` | 0x1409e3980 | raster 描述构造 | path 拷贝两份（@+0x00 经 String clone helper 0x140d7bd90、@+0x30 直接），@+0x18 空 defines {0,dangling8,0}；0x48B | O | decomp-0x1409e3980、extra-0x140d7bd90 |
| `pipeline_desc::RasterPipelineDesc::new_vertex_fragment`（inline） | — | 构造（inline 实例×4） | 同 compiled body | O | skeleton pipeline_desc.rs:24 |
| `pipeline_desc::ComputePipelineDesc::new`（line 55） | — | compute 描述构造（inline×30） | source_path.to_owned + entry 默认 + defines 空 | O | skeleton pipeline_desc.rs:55 |
| `pipeline_desc::ComputePipelineDesc::with_entry<ref<str>>` | 0x140539370（另 0x1409e3ab0） | 设入口点 | entry 空→默认 "cs_main"；非空→to_owned；复制其余字段 | O | decomp-0x140539370 |
| `pipeline_desc::ComputePipelineDesc::with_entry::closure$0`（line 64） | — | 闭包 | to_owned 转换 | O | skeleton pipeline_desc.rs:64 |
| `pipeline_desc::impl$8::read_from`（line 3） | — | u32 newtype 解码 | 读单个 u32；类型语义 U | U | skeleton pipeline_desc.rs:3 |
| `pipeline_desc::impl$14::clone` / `impl$16::eq`（line 16） | — | 描述 Clone/Eq | String+Option+shader_defines Vec 全字段比较（原含 Option 字段） | O | skeleton pipeline_desc.rs:16 |
| `shader_defines::impl$1::clone` / `impl$3::eq`（line 3） | — | 定义列表 Clone/Eq | `Vec<(String,String)>` 逐项 | O | skeleton shader_defines.rs:3 |
| `reflection::BindingNameHash::new` | 0x140c87660 | 绑定名哈希键 | 末尾 `_dims`→strip 5 且 +1；否则全串 +0；FNV-1a64 | O | decomp-0x140c87660、extra-0x141937610 |
| `reflection::BindingNameHash::with_suffix_dims` | 0x140c87650 | 哈希+1 | `self.0.wrapping_add(1)`（短函数，Ghidra NotFound） | O/I | 骨架 line 27 + rebuild 同实现 |
| `reflection::impl$0::hash`（line 19） | — | Hash impl | 供反射作为键 | O | skeleton reflection.rs:19 |
| `reflection::impl$16::eq`/`impl$18::read_from`（line 15） | — | 反射 Eq/Readable | — | O | skeleton reflection.rs:15 |
| `reflection::impl$25..impl$94::read_from`（line 31–88） | — | 反射各类型解码 | Binding/BindingArray/UniformBlock/VertexInput/descriptor_sets(HashMap)/...；line 88 用 read_key_value_collection+3 闭包 | O | skeleton reflection.rs |
| `reflection::impl$3::calculate_name_hashes`（line 105/112 闭包） | — | 名字哈希化 | 对 binding_names 计算 BindingNameHash | I | skeleton reflection.rs |
| `reflection::len_without_suffix`（line 117） | — | 去 `_dims` 长度 | — | I | skeleton reflection.rs |
| `reflection_ty::Type::nbyte` | 0x1420bda00 | 类型字节数 | 判别 `*p^0x8000000000000000` 分发；opaque→0；Struct/Array 递归（StructMember 0x50B）；Scalar/Vector/Matrix 走子表 | O（case 映射 I） | decomp-0x1420bda00 |
| `reflection_ty::ScalarType::nbyte` | 0x1420bd380 | 标量尺寸 | 分派表（Ghidra NotFound） | O/I | decomp-0x1420bd380（NotFound）、skeleton line 64 |
| `reflection_ty::{VectorType,MatrixType,ArrayType,StructType}::nbyte` | — | 尺寸 | inline | I | skeleton reflection_ty.rs |
| `reflection_ty::impl$86::read_from<LittleEndian,BufferReader>`（line 284） | — | Type 解码 | compiled_procedure 无地址 | O | skeleton reflection_ty.rs |

### 2.2 gpu-profiler

| 函数 | 地址 | role | 算法要点 | O/I/U | evidence |
|---|---|---|---|---|---|
| `shared::MarkerId::as_u64` | — | 标记→u64 | inline；1177 调用方 | O | skeleton shared.rs:11 |
| `shared::MarkerId::from_u64` | 0x140c10e20 | u64→标记 | 非 0xffffffffffffffff 截断 32 位；否则 panic（u32::MAX 哨兵） | O | decomp-0x140c10e20 |
| `shared::NanoSecond::from_raw_ns` | 0x14067dd80 | 纳秒包装 | Ghidra NotFound（141 条 S_PUB32 折叠）；语义 I | O/I | decomp-0x14067dd80（NotFound） |
| `shared::GpuTick::duration_since` | 0x140c10e50 | 时差 | NotFound | I | decomp-0x140c10e50（NotFound） |
| `shared::GpuTickDuration::nanoseconds` | 0x140c10e60 | 时长转纳秒 | NotFound | I | decomp-0x140c10e60（NotFound） |
| `shared::FrameMarkers::create_marker`（line 82） | — | 名字入表 | push &str 16B；无去重；内联宿主=ActiveProfiler::create_marker | O | skeleton shared.rs:82 |
| `shared::FrameMarkers::marker_name` | 0x140c10ee0 | 序号→名 | `*(ptr + idx*0x10)` 返回名字指针；越界 panic | O | decomp-0x140c10ee0 |
| `backend::ash::ProfilerOutputBuffer::new`（line 38） | — | 输出缓冲 | inline | I | skeleton ash.rs:38 |
| `backend::ash::Profiler::new` | 0x140c10f20 | 分析器建池 | vkCreateQueryPool(TIMESTAMP,1024)+0x1ff8B 固定缓冲；{buf,0x3ff,pool,0} | O | decomp-0x140c10f20 |
| `backend::ash::Profiler::write_marker`（line 108） | — | 写标记 | inline | I | skeleton ash.rs:108 |
| `backend::ash::Profiler::begin_frame` | 0x140c11090 | 帧开始 | vkCmdResetQueryPool(0,1024)+count=0；返回 ActiveProfiler 视图 | O | decomp-0x140c11090 |
| `backend::ash::ActiveProfiler::create_marker` | 0x140c110f0 | 建标记 | 满则 grow；push(name_ptr,len)16B；返回 index(u32) | O | decomp-0x140c110f0 |
| `backend::ash::ActiveProfiler::write_marker` | 0x140c11150 | 写时间戳 | 原子自增；索引入固定缓冲；vkCmdWriteTimestamp(stage 0x2000) | O | decomp-0x140c11150 |
| `backend::ash::ActiveProfiler::end_frame<ProfilerBuffer>` | 0x1418c3910（另实例无地址） | 帧收尾 | vkCmdCopyQueryPoolResults(64_BIT\|WAIT,stride 8)；打包 SubmittedFrame（markers+query_indices+meta） | O | extra-0x1418c3910 |
| `backend::ash::ProfilerSubmittedFrame<Buffer>::retrieve_results` | 0x140294cf0 | 结果回读（游戏侧 R6） | u32 timestamps×u64 值→16B 条目 Vec；校验/panic；0x140bd3c80 全局取源 | O | extra-0x140294cf0、extra-0x140bd3c80 |

### 2.3 vk-sync-rs

| 函数 | 地址 | role | 算法要点 | O/I/U | evidence |
|---|---|---|---|---|---|
| `cmd::pipeline_barrier` | 0x1418afd20 | 屏障下发 | 3 组 barrier 转换汇总后一次 vtable+0x368；Vec 增长 | O | decomp-0x1418afd20 |
| `lib.rs impl$1::fmt` / `impl$5::eq`（line 21） | — | AccessType Debug/Eq | — | O | skeleton lib.rs:21 |
| `get_memory_barrier` | 0x140c107c0 | 内存屏障 | 字符逐 access 汇总；32B 结构{sType 0x2e,...} | O | decomp-0x140c107c0 |
| `get_buffer_memory_barrier` | 0x140c10940 | 缓冲屏障 | 同上前缀+拷贝输入字段；sType 0x2c；0x48B | O | decomp-0x140c10940 |
| `get_image_memory_barrier` | 0x140c10af0 | 图像屏障 | +layout/queue/subresource/image；sType 0x2d；0x50B；'\''→0x3b9acdea | O | decomp-0x140c10af0 |
| `get_access_info` | 0x140c10dd0 | 字符→掩码对 | NotFound；输出 2 u32（access, write-stage） | O/I | decomp-0x140c10dd0（NotFound） |
| `AccessType::is_write` | 0x140c10e10 | 写类型判断 | `(byte)(c-0x28)<0x10`（写=0x28..0x37） | O | decomp-0x140c10e10 |

### 2.4 aftermath-rs

| 函数 | 地址 | role | 算法要点 | O/I/U | evidence |
|---|---|---|---|---|---|
| `Aftermath::new<NvAftermathDelegate>` | 0x140176e70 | 初始化+注册回调 | 0x38B 对象拷贝 delegate；0x10B user_data{vtable}；Aftermath 初始化注册 4 回调 | O | extra-0x140176e70 |
| `Aftermath as Drop::drop` | 0x140c107b0 | 析构 | Ghidra NotFound | I | extra-0x140c107b0（NotFound） |
| `impl$4::fmt` / `impl$6::eq`（line 45） | — | 错误类型 Debug/Eq | — | O | skeleton lib.rs:45 |
| `Status::get`（line 55） | — | 读状态 | 646 调用方 | O | skeleton lib.rs:55 |
| `Status::wait_for_status` | 0x140c104e0 | 等待状态 | 轮询 status→2/4；50ms sleep；超时 panic | O | decomp-0x140c104e0 |
| `get_shader_debug_info_identifier` | 0x140c10620 | 着色器诊断 ID | Aftermath 函数(0x217)+错误构建；Result{flag,16B} | O | decomp-0x140c10620 |
| `callbacks::GpuCrashDumpCallback` | 0x140c106c0 | C 回调 | 转发 delegate vtable+0x18 | O | decomp-0x140c106c0 |
| `callbacks::ShaderDebugInfoCallback` | 0x140c10710 | C 回调 | 转发 vtable+0x20 | O | decomp-0x140c10710 |
| `callbacks::GpuCrashDumpDescriptionCallback` | 0x140c10760 | C 回调 | 转发 vtable+0x28 | O | decomp-0x140c10760 |
| `callbacks::ResolveMarkerCallback` | 0x14003bb80 | C 回调 | ICF 折叠（437 符号同址，含 NvAftermathDelegate::description）；vtable+0x30（I） | O/I | decomp-0x14003bb80（NotFound）、S_PUB32 |
| `sys.rs impl$0/3/4/6`（line 41） | — | GFSDK 标识 derive | Default/Debug/Hash/Eq | O | skeleton sys.rs:41 |

### 2.5 dxgi-enumerate-gpus

| 函数 | 地址 | role | 算法要点 | O/I/U | evidence |
|---|---|---|---|---|---|
| `enumerate` | 0x1418b29c0 | GPU 枚举 | CreateDXGIFactory→EnumAdapters1→GetDesc1→WCHAR 名扫描→String→DxgiAdapter 40B 入 Vec→Ok/Err | O（字段映射 I） | decomp-0x1418b29c0 |
| `get_string_from_wide`（line 36） | — | 宽串转换 | 扫描首个 null WCHAR；内联进 enumerate | O | skeleton lib.rs:36 |
| `get_string_from_wide::closure$0`（line 37） | — | 逐字符 | inline | O | skeleton lib.rs:37 |

## 3. 原版 ↔ rebuild 对照表

| 原模块/函数 | rebuild 对应 | 判定 | 差异说明 |
|---|---|---|---|
| `shader_pipeline`（整个子 crate） | `rhapsody-rebuild/src/shader_pipeline/`（并入 rhapsody crate，非独立 crate） | ✓（半） | 同名文件全镜像；反射/描述/文件键对得上。差异：`RasterPipelineDesc` 4 字段 ≠ 原 3×24B（原 = 2×path String + 空 defines，@+0x18 字段语义 U）；`ShaderReflection` 原 Hash（impl$0::hash line 19）rebuild 未 derive；defines 哈希算法（原 `calculate_shader_defines_hash` 无地址、rebuild 用 DefaultHasher XOR）未 O 证。 |
| `CompiledShaderPipeline` | `shader_pipeline/compiled.rs` | ✓ | `trailing_bytes` 建模原 read_from 未消费尾字节（正确方向）；0x140B 载荷/反射字段序未逐字段 O 证。 |
| `BindingNameHash::new/with_suffix_dims` | `shader_pipeline/reflection.rs` | ✓✓ | 机器码逐字节对（`_dims` strip+1、FNV-1a64 = const-fnv1a-hash）。 |
| `Type/ScalarType/...::nbyte` | `reflection_ty.rs::nbyte()` | ✓（半） | 结构/0x50B StructMember 对齐；rebuild 尺寸经 spirq 计算 vs 原自算分派表（0x1420bd380），尺寸语义等价性未逐 case O 证。 |
| `load_compiled<D>` | `CompiledShaderPipeline::load_compiled(path)` | 半 | 原泛型于 PipelineDesc（`compiled_file_path` 生成路径）；rebuild 是 `path` 直传（调用方自行拼键）。原返回 Result<0x140B, anyhow>（Err 判别 0x8000000000000000）；rebuild 返回 Result<Self,String>。原游戏侧经 `ShaderBackend::load_compiled_*` + Arc 使用，rebuild 未接。 |
| `gpu_profiler`（shared + backend::ash 全部） | `raw/profiler.rs::GpuProfiler`（候选，非依赖） | ✗ | rebuild 未引入 gpu-profiler 依赖；候选 `GpuProfiler` 只有 new+时间戳池+span 名，**无 marker 链（create_marker/write_marker/begin_frame/end_frame/retrieve_results）、无 SubmittedFrame 结构、无 0x140294cf0 等价回读**。CALL-CHAIN §3.2 标"rebuild 用真实 gpu-profiler 依赖 ✓"与实测不符（Cargo.toml/lock 均无 gpu-profiler）。 |
| `vk_sync_fork`（全部） | 无 | ✗ | rebuild 无 vk-sync 依赖；`rg/resource_rules.rs:41` 显式接收外部分类器替代 `AccessType::is_write`。原 barrier 结构（32/0x48/0x50B）与 vtable+0x368 下发链未镜像。 |
| `aftermath_rs`（全部） | 无 | ✗ | rebuild 无 aftermath。MODULE_STATUS「GPU诊断」验收部分覆盖（marker/debug packet 读回），但原 Aftermath 注册链/vendor dump/device-lost 未恢复。 |
| `dxgi_enumerate_gpus::enumerate` | 无（原 `raw::physical_device` 自己枚举 vk data） | ✗ | rebuild 的物理设备枚举是 `enumerate_vk_data`（0x1418cd4d0 原侧另有函数），不是 dxgi 层；原 enumerate 的 `DxgiAdapter` 40B 结果结构未镜像。 |

## 4. U 卡点清单

1. **`RasterPipelineDesc` 字段语义**：机器码 0x48B = 2×path String（@+0x00、@+0x30）+ 空 defines（@+0x18 {0,8,0}）。`@+0x00` 与 `@+0x30` 谁是 source_path / entry、为何克隆两份 path，未定。country-core 合约 `{path, entry, defines}` 的字段序与 72B 机器布局映射 U。卡点：`new_vertex_fragment` 只设置 path，未设置第二个字符串的不同默认；需要 raster 带 entry 的构造实例（如 variants 消费路径）佐证。
2. **defines 哈希算法**：原 `calculate_shader_defines_hash` / `calculate_hash<tuple<String,String>>` 无地址（inline），rebuild 用 std DefaultHasher(SipHash) XOR-fold；原是否同 FNV-1a（BindingNameHash 用 FNV）未知。卡点：无 compiled 实例可反编译；需从发行 `.bin` 文件名 key 反推（MODULE_STATUS：38 非零 key 已核对、170 宏 tuple 未独证）。
3. **`ShaderReflection` 字段序与 Hash**：原 impl$0::hash（line 19）证实 ShaderReflection 实现 Hash（rebuild 未 derive）；原 HashMap<u32,HashMap<u32,Binding>> 字段在 0x140B 载荷内的精确布局/序未逐字段 O 证（只按 rebuild 顺序建模）。
4. **`pipeline_desc.rs` line 3 u32 newtype**：`impl$8::read_from` 只读一个 u32；类型语义 U。
5. **`Type::nbyte` 的 case→variant 映射与递归算法**：判别字 `^0x8000000000000000` 的枚举编码、Struct 递归的累加方式（只观察到尾成员递归）、case 9/0xc/0xd 返回 1 的语义，均未解清。卡点：需逐 variant 构造反例或对照 spirq 尺寸表。
6. **gpu-profiler 时间戳换算**：`retrieve_results` 里 0x140bd3c80（profiler 全局）+ FUN_1405c3580/0x1405c75b0（转换/排序）语义 I；ticks→ns 的 period 来源 U。
7. **`FrameMarkers::marker_name` 返回值**：返回 8B（名字字符串指针），若名字是 `&'static str` 则丢 len；实际存储形态 U（16B 条目 = (ptr,len) 但返回只取 ptr）。
8. **aftermath `ResolveMarkerCallback`（0x14003bb80）**：ICF 折叠 437 符号，Ghidra 无函数；转发 vtable+0x30 为推断（I）。`Aftermath::Drop::drop`（0x140c107b0）NotFound，析构行为 U。
9. **`Aftermath::new` 的 Aftermath 初始化函数（0x142923580, opcode 0x217）**：是 Aftermath DLL 导出（GFSDK_Aftermath_*）还是封装函数未核实；版本参数 (2,1) 语义 U。
10. **`Status::wait_for_status` 目标值**：轮询退出条件为 status∈{2,4}；2/4 对应的 GFSDK_Aftermath 枚举名 U。
11. **`DxgiAdapter` 40B 字段映射**：name String@0 + @0x10..0x28 的 u32 id 字段（vendor/device/subSys/rev 等）精确对应 U；Err 判别 `0x8000000000000000` 与 load_compiled 相同的 Result 布局（判别与 Ok 载荷共用 offset 0）的 Rust 语义未定（可能是自定义 Vec 指针 niche）。
12. **`get_access_info`（0x140c10dd0）**：字符→(access, stage) 两 u32 的**完整映射表** U（Ghidra NotFound；需从 get_*_barrier 输出反推每个字符位）。
13. **`CompiledShaderPipeline` 0x140B 载荷 vs CALL-CHAIN 0x130B**：两数值来源不一致（0x140=本次 load_compiled Ok 复制长度；0x130/0x150=既有记录）；0x140 内是否含 trailing 字段 U。
14. **rebuild 接入缺口（跨域）**：rebuild 的 shader_pipeline 无人调用（country-core 从未 use rhapsody）；`ShaderCompilerArtifacts::load_compiled` 游戏侧经 `ShaderBackend::load_compiled_compute/raster` trait 契约，rebuild 未接上该 trait 面。

---

## 5. 审查轮闭合（agent=rhapsody-crates-closure，2026-10-15）

按 REVIEW-UNEXPLORED crates 部分 P0/P1 逐项闭合。证据：`rne/crates-notfound-closure.md` + `decomp/crates-get-access-info-table.md`。
除 §0/§1/§2 既有结论行外，本节只追加、不改写既有行。

### 5.1 P0：get_access_info 全表手解（0x140c10dd0）—— O 全解

- 函数实为**三表查询**（rne-read 原字节 + capstone）：`out[0]=stage`、`out[1]=access`、`out[2]=image layout`，
  三张 u32 表 = 同一 256×u32 数组 0x142b1a6c8 的三段（stage@[0x00..0x3f]、access@[0x40..0x7f]、layout@[0x80..0xbf]）。
- **修正 §1.3/§2.3 角色**：首字节是 **stage**（非 access），第二字节是 **access**（非"write 才计入的 stage"）；
  三个 barrier 转换函数把 `out[0]`（stage）OR 进 `param_1[0]`、把写字符的 `out[1]`（access）OR 进 `param_1[6]`。
- 有效字符 0x00..0x3f 共 64 变体；读 0x00..0x27、写 0x28..0x37、0x38..0x3f 高位掩码读。**全表**见 `decomp/crates-get-access-info-table.md`。
- 变体名锚点：写变体 16 个与公开 h3r2tic/vk-sync-rs 枚举逐一对上（ColorAttachmentWrite=0x2e、TransferWrite=0x34、
  HostWrite=0x35、ColorAttachmentReadWrite=0x36、General=0x37 等，stage/access/layout 三列全对）；读变体锚点 =
  layout 列（sampled=5、other=1、color=2、depth=4、transfer_src=6、present=哨兵）+ country-core 常量
  （ACCESS_INDIRECT_READ=0x02、ACCESS_READ=0x1a、ACCESS_DRAW_READ=0x1f、ACCESS_CPU_MAPPED_WRITE=0x32、
  ACCESS_TRANSFER_WRITE=0x34、ACCESS_WRITE=0x37 全部对上）。
- **fork 改造点**：写区间由公开的 0x22..0x31 整体后移 +6 到 0x28..0x37（中间插入 6 个读变体，名 U）；uniform/input 读合并为
  SHADER_READ(0x20)；stage/access 沿用 stage-1 位值。

### 5.2 P0：read_from_buffer wire 布局（extra-0x1409e4670）—— O 分段反编译

- **无 magic/版本头**：首 4 字节即 stages 计数（`*(uint*)param_2`），无 SPIR-V magic 扫描；rebuild 注释与机器码一致。
- 最小长度门 `0x1e`=30B = 空流水线编码之和（stages 4 + source_deps 4 + descriptor_sets 4 + uniform_block tag 1 +
  binding_names 4 + binding_name_hashes 4 + vertex_inputs 4 + local_size tag 1 + push_constant_size 4）——
  **O 印证 ShaderReflection 七字段序**（descriptor_sets → uniform_block(Option) → binding_names → binding_name_hashes →
  vertex_inputs → local_size(Option) → push_constant_size）。
- 读取序 `stages → source_dependencies → reflection`；CompiledShaderStage 0x38B = {u32 stage(≤2), String entry_point, Vec<u32> spirv}。
- **尾字节保留**：末字段读完后剩余字节不消费、不报错（speedy read_from_buffer 行为，rebuild `trailing_bytes` 建模正确）。
- 返回 Result<0x140B, anyhow>：Ok={0, 0x140B}、Err={0x8000000000000000, err}；0x140B = {stages@0, source_deps@0x18, ShaderReflection@0x30(0x110)}。
- 0x130/0x150 数值：0x140 为权威 Ok 复制长度；0x130=旧结构/与 GetDesc1 长度同值、0x150=Arc 块，不冲突。

### 5.3 P1：Aftermath Drop + 悬空引用修正—— O，S_PUB32 定死函数身份

- `Aftermath::Drop::drop` @0x140c107b0 = **`jmp 0x142923880`**（5B 尾调）→ import thunk → **`GFSDK_Aftermath_DisableGpuCrashDumps`**
  （S_PUB32；IAT 槽 0x143192d38→目标 0x3194aaa）。0x38B 对象内存释放由 std drop 胶水另行处理（drop_in_place<AftermathUserData>@0x1400c04b0）。
- **修正悬空引用**：`decomp/extra-0x140c107b0.txt` 不存在（头注"补取 7 个"与实 9 个不符——本轮补证据到 `rne/crates-notfound-closure.md` §5，非 decomp 目录）。
- `Aftermath::new` 初始化函数 0x142923580（S_PUB32）= **`GFSDK_Aftermath_EnableGpuCrashDumps`**；调用 `(0x217, 2, 1, 4 回调, userData)`——
  0x217 是第一参数（API 版本/特性位），非"opcode"；`get_shader_debug_info_identifier` 目标 0x142923870 = **`GFSDK_Aftermath_GetShaderDebugInfoIdentifier`**。
- `wait_for_status` 轮询目标 0x142923860 = **`GFSDK_Aftermath_GetCrashDumpStatus`**；退出值 2/4 = CrashDumpStatus 枚举（精确名对照 Aftermath SDK 头，I）。

### 5.4 P1：gpu-profiler 时间戳换算—— O

- `GpuTick::duration_since`（0x140c10e50，rne-read）= `if self>=earlier {self-earlier} else {0}` 饱和差。
- `GpuTickDuration::nanoseconds`（0x140c10e60，rne-read）= `(self as f64 * period_f32) as u64`；**period 为 f32 第二参数（xmm1）**，
  非全局常量。**period 来源 = rhapsody::raw::profiler::timestamp_period**（对应 VkPhysicalDeviceLimits.timestampPeriod，游戏侧渲染初始化查询）。
- `NanoSecond::from_raw_ns`（0x14067dd80）= newtype 恒等（与 S_PUB32 折叠 raw_ns 同址）；`marker_name`（0x140c10ee0）返回**完整 &str {ptr,len}**（修正 §4.7"返回 8B 丢 len"）。
- `retrieve_results`（0x140294cf0）条目 = `(MarkerId, GpuTick)` 16B，配对复制后按 GpuTick **sort_unstable_by_key**；
  FUN_1405c3580（<0x15）/FUN_1405c75b0（更大）= std slice sort 内部（**纯排序，非 ticks→ns 转换**，修正 §2.2"转换/排序"措辞）。
- `0x140bd3c80` = profiler 数据读取器（0x140c113c0 读全局+0x30 有效位/返回+0x18 u64 值数组），非 period。

### 5.5 P1：Type::nbyte case 映射—— O

- 跳表 15 项（判别 `*p^0x8000000000000000`，default=case 10）：0=Scalar、1=Vector、2=Matrix、3..8=Image/CombinedImageSampler/
  SampledImage/StorageImage/Sampler/SubpassData(→0)、9=Array、10=Struct(递归 0x50B/ty@+0x18)、11=AccelStruct(→0)、
  12=DeviceAddress(→(1,8))、13=DevicePointer(→(1,8))。**rebuild Type 枚举序 = 机器序（0..13 全对上）**。
- 返回约定 = (rax 非零标记, rdx 尺寸/累加)；opaque=0、复合/地址=1、DeviceAddress/Pointer=8。
- `ScalarType::nbyte`（0x1420bd380，rne-read）：Void→0、Boolean→1、Signed/Unsigned/Float→**返回 payload u32（宽度字段原值）**；
  bits-vs-bytes 契约 U（Boolean=1/DeviceAddress=8 暗示 payload 为字节数，I）；消费端 = raw `create_raster_pipeline`（S_PUB32 静态直调）。

### 5.6 P1：with_suffix_dims 判定—— decomp NotFound 但 rne-read **O 升级**（非降 I）

- `decomp/0x140c87650.txt` 为 NotFound JSON；但 rne-read 原字节 `48 8d 41 01 c3` = **`lea rax,[rcx+1]; ret`** =
  `self.0.wrapping_add(1)`，与 rebuild 完全一致。verdict O/I → **O（rne-read 证据）**。S_PUB32 确认 1 符号非折叠。

### 5.7 P2：dxgi 40B 字段映射、defines 哈希、shader_compiler 候选

- **DxgiAdapter 40B**（enumerate 0x1418b29c0 元素写，O）= `{name: std String{ptr,len,cap}@0, AdapterLuid u64@0x18, VendorId u32@0x20}`。
  只取 VendorId + AdapterLuid（修正 §4.11"含 vendor/device/subSys/rev/dedicated"）；desc1 栈拷贝基址 = rbp-0x50。
- **defines 哈希**（U 保留）：发行 404 .bin 全部 raster 只有两个 hash `b903e43ffb3da915`/`f0adff76827e73ff`（117+117=每 raster 两变体，
  defines 全局同组、单一宏开/关）；compute hash 恒 0=空 defines，与 rebuild XOR-fold 空列表=0 一致；但 FNV-1a64/SipHash-1-3(0,0)
  对常见 PERM_* 名单暴力测试未复现两值 → 宏名/值未知，算法与输入均 U。
- **shader_compiler 候选对照**（补 §3 行）：`preprocess.rs::prepare_file`（include 展开/#define/#line）+ `reflect.rs::generate_reflection`
  （反射 producer，BindingNameHash 应用、跨 stage 合并冲突检测）+ `pack.rs::package`（prepare_spirv_for_package 重写 header/
  压缩 Source 串 + Speedy 容器打包，`trailing_bytes=Vec::new()`）。MODULE_STATUS 核对：404 包/638 阶段生成、**397/404 typed 反射相等**、
  "210" = 新编译包中**完整反射的包数**（210/404，REVIEW §四#3 疑问指认）；pack.rs `trailing_bytes=Vec::new()` 与 §5.2 尾字节
  保留行为一致（原版允许尾字节，rebuild 候选不产生尾字节不冲突）。
