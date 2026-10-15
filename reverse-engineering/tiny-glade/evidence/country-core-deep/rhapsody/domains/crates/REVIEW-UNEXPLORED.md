# rhapsody crates 域 REVIEW-UNEXPLORED（审查未探究清单）

- 审查日期：2026-10-15。审查对象：`domains/crates/ALGORITHMS.md`（228 行）对照骨架 `domains/crates.json`（46 compiled / 28 有地址）。
- 审查性质：**只产出审查清单，不改任何源码和已交付文档内容**。结论标注沿用 **O**（机器码/PDB 实测）/ **I**（推断）/ **U**（未知）。
- 核对方法：从 crates.json 提取全部 46 个 compiled_procedure，逐一对照 ALGORITHMS.md §2 逐函数表；37 个反编译（28 批量 + 9 extra）逐一核对 evidence 引用；rebuild 对照逐行复验。
- 一句话结论：**ALGORITHMS.md 主体扎实（BindingNameHash=O 全解、vk-sync barrier 三结构=O、gpu-profiler 四层链=O、aftermath 注册链=O、dxgi 枚举链=O 是本域真正亮点），但存在 6 个 compiled_procedure 无分析行、1 处悬空证据引用（extra-0x140c107b0）、50KB 的 read_from_buffer 反编译完全未分析、vk-sync get_access_info 全表与 gpu-profiler 时间戳换算两个核心算法停留在 NotFound/I。**

---

## 一、未覆盖函数（骨架 compiled 无 ALGORITHMS.md 对应行）

PLAYBOOK §5 规则是 "compiled_procedure 全覆盖"。以下函数在 crates.json 中是 `compiled_procedure`，但 ALGORITHMS.md §2 逐函数表**没有任何对应行**：

| # | 骨架函数 | 文件/行 | form | 卡点 | 建议补法 |
|---|---|---|---|---|---|
| 1 | `shader_pipeline::reflection::impl$33::fmt` | reflection.rs:46 | compiled_procedure（无地址） | 反射某类型的 fmt（Debug/Display）编译实例；line 46 邻近 impl$37::read_from（line 46），应属同一类型族 | 从 reflection.rs line 46 的类型推断是哪个反射类型（对照 rebuild reflection.rs 的 derive Debug 集），用 S_PUB32 查同名；fmt 是胶水可能性高，但需登记并标注"derive 直接复用" |
| 2 | `shader_pipeline::reflection::impl$41::fmt` | reflection.rs:56 | compiled_procedure（无地址） | 同上；line 56 邻近 impl$45::read_from | 同上 |
| 3 | `shader_pipeline::reflection_ty::impl$1::fmt` | reflection_ty.rs:0 | compiled_procedure（无地址） | line 0 的 fmt 实例（ScalarType 族 fmt） | 同上；reflection_ty 是 Type 枚举族，fmt 应为各类型 Debug，需按邻接 impl$9::fmt/impl$19::read_from 分组 |
| 4 | `shader_pipeline::reflection_ty::impl$9::fmt` | reflection_ty.rs:0 | compiled_procedure（无地址） | 同上 | 同上 |
| 5 | `shader_pipeline::reflection_ty::impl$5::fmt` | reflection_ty.rs:174 | compiled_procedure（无地址） | MatrixType 族 fmt（line 174 邻近 MatrixType::nbyte line 169） | 同上 |
| 6 | `shader_pipeline::reflection_ty::impl$12::fmt` | reflection_ty.rs:347 | compiled_procedure（无地址） | Type 枚举顶级 fmt（line 347 邻近 impl$12 在 Type::nbyte line 325 之后） | 同上 |

**补充（inline 但有真实业务角色的未覆盖项）**——PLAYBOOK §5 "inline 里角色重要的也要解"：

| # | 骨架函数 | 角色 | 卡点 | 建议补法 |
|---|---|---|---|---|
| 7 | `compiled::ShaderCompilerArtifacts::load_compiled::closure$0/closure$1`（line 30/38） | 错误上下文闭包（拼 path 进 "Failed opening/Failed deserializing" 文案） | ALGORITHMS.md 只给了错误文案结论，闭包如何构造 anyhow::Error 未解 | 从 extra-0x1409e3c50/0x1409e4160 反编译中定位两个闭包体，确认错误类型构造（anyhow::Error::msg 的格式串/上下文） |
| 8 | `reflection_ty::impl$45/53/61/70::read_from::closure$0/1`（line 147/191/229/235） | Matrix/Array/Struct 族的 speedy 解码闭包（read_key_value_collection / String 读取） | ALGORITHMS.md 只对 reflection.rs:88 的 3 层闭包叙述，reflection_ty 的闭包未提 | 对照 rebuild reflection_ty.rs 的 Readable derive 逐个确认字段读取序 |
| 9 | `reflection::impl$58::eq`、`impl$60::hash`、`impl$74::fmt`、`impl$76::eq`、`impl$48::fmt`、`impl$56::fmt`（line 62–75） | 反射域多类型的 Eq/Hash/fmt 编译实例 | ALGORITHMS.md 只记了 impl$16::eq（line 15）+impl$0::hash（line 19）；line 62–75 的 eq/hash/fmt 属另一类型族（含 Hash 的 key 类型） | 识别这些 impl 属于哪个反射类型（最可能是 `(u32, Binding)` 映射键或 VertexInput 族），补 Hash 语义登记 |
| 10 | `gpu_profiler::backend::ash::impl$1::retrieve_results::closure$0`（line 78） | retrieve_results 的配对复制闭包（u32 timestamp × u64 值 → 16B 条目） | ALGORITHMS.md §2.2 retrieve_results 行未提该闭包 | 从 extra-0x140294cf0 的 joined_r0x000140294ddc 循环（每轮复制 2 个 8B 条目）反推闭包形态 |
| 11 | `gpu_profiler::backend::ash::impl$3::end_frame::closure$0`（line 202） | end_frame 打包 SubmittedFrame 的闭包 | 同上 | 从 extra-0x1418c3910 的栈拷贝段（param_5[0..10] → param_1[3..0xe]）反推 |

> 结论：**46 个 compiled_procedure 中 6 个完全没有分析行**（全部是 reflection 域 fmt），加上 5 组有业务角色的 inline 闭包未覆盖。需按上表补齐后才能在 MODULE-DEEP-DIG 登记"全覆盖"。

---

## 二、表面分析（有行但 role/算法/证据空泛）

| # | 函数（骨架名） | 地址 | ALGORITHMS.md 现状 | 表面程度 | 建议补法 |
|---|---|---|---|---|---|
| 1 | `aftermath_rs::impl$1::drop`（Aftermath Drop） | 0x140c107b0 | §1.4/§2.4 行："Ghidra NotFound"、verdict I、evidence="extra-0x140c107b0（NotFound）" | **悬空证据**：`decomp/extra-0x140c107b0.txt` **不存在**（已核实目录）。drop 的析构行为（是否调用 Aftermath 反初始化、是否释放 0x38B/user_data）完全未解 | ① 补跑该地址反编译或 rne-read 原字节；② 修 ALGORITHMS.md 悬空引用（要么落盘真实证据，要么改标 U 不引文件）。头注"补取 7 个"清单含此项但文件从未保存 |
| 2 | `gpu_profiler::shared::NanoSecond::from_raw_ns` | 0x14067dd80 | NotFound、verdict O/I、语义 I | 纯 NotFound，无 rne-read 手解；纳秒包装是 141 条 S_PUB32 折叠符号 | rne-read 原字节手解（应是一两条指令的 newtype 包装），确认折叠身份 |
| 3 | `gpu_profiler::shared::GpuTick::duration_since` | 0x140c10e50 | NotFound、verdict I | 时差算法（两 u64 相减？含 wrap？）未解 | rne-read 手解；对照公开 gpu-profiler crate 的 duration_since 语义 |
| 4 | `gpu_profiler::shared::GpuTickDuration::nanoseconds` | 0x140c10e60 | NotFound、verdict I | ticks→ns 的换算公式未解（这是 retrieve_results 的下游） | rne-read 手解；重点确认 period 乘数来源（见 §3 缺口 2） |
| 5 | `shader_pipeline::reflection_ty::ScalarType::nbyte` | 0x1420bd380 | "分派表（Ghidra NotFound）"、verdict O/I | Type::nbyte 的 case 0 子表派发到此处，但标量尺寸表内容未解 | rne-read 手解 0x1420bd380 + 分派表 UNK_142fd8fe4 的 24B 表；对照 spirq 的 ScalarType::nbyte 逐 case 验等价 |
| 6 | `vk_sync_fork::get_access_info` | 0x140c10dd0 | NotFound、verdict O/I、"完整映射表 U"（卡点 12） | **本域核心算法空泛**：字符→(accessMask, stageMask) 两 u32 的完整映射表未解 | 优先 rne-read 手解 0x140c10dd0（应是查表返回两个掩码）；或从 get_*_barrier 反编译里逐字符输出反推（get_memory_barrier 有逐字符循环）。见 §3 缺口 4 |
| 7 | `pipeline_desc::impl$8::read_from` | — | "读单个 u32；类型语义 U" | 一行结论，newtype 身份未尝试识别 | 从 RasterPipelineDesc 0x48B 布局与 with_entry 的字段序反推该 u32 newtype 是文件键还是 entry 标签；对照 skeleton line 3 上下文 |
| 8 | `reflection::impl$3::calculate_name_hashes::closure$0/1`、`len_without_suffix` | — | verdict I | 绑定名→BindingNameHash 的映射算法（含 `_dims` 后缀截断复用）只给一句话 | 对照 rebuild shader_compiler/reflect.rs 的 generate_reflection（binding_name_hashes 生成逻辑）+ BindingNameHash::new 的 O 证，把 calculate_name_hashes 行为推 I→O |
| 9 | `gpu_profiler::backend::ash::ProfilerOutputBuffer::new`、`Profiler::write_marker` | — | verdict I、无内容 | inline 无地址，但 write_marker 的"索引入固定缓冲"由 ActiveProfiler::write_marker 覆盖，Profiler::write_marker 是不同实例 | 低优先；确认 Profiler 层 write_marker 与 ActiveProfiler 层是否同体（骨架 line 108 vs 157） |
| 10 | `dxgi_enumerate_gpus::enumerate` 的 DxgiAdapter 40B 字段 | 0x1418b29c0 | 枚举链 O、字段映射 I | DXGI_ADAPTER_DESC1 0x130B 拷贝后哪些字段进 40B 元素未映射 | 手解 0x1418b29c0 中 GetDesc1 后的字段搬运段（vendor/device/subSys/rev/dedicated memory 哪个被取）；见 §3 缺口 6 |
| 11 | `CompiledShaderPipeline::read_from_buffer`（helper） | 0x1409e4670（**50KB 反编译**） | §0.1/§2.1 只作调用目标引用一次 | **最大反编译完全未分析**：50KB 是 load_compiled 的解码主干，`.bin` 的 wire 布局全部由此来；ALGORITHMS.md 对 0x140B 载荷字段序只按 rebuild 建模、明标"未逐字段 O 证"（卡点 3/13） | 分段读 50KB：先定位 read_vec/read_u32/String 调用点 → 确定 CompiledWire 三段（stages/source_dependencies/reflection）的实际读取序与尾字节处理。见 §3 缺口 1 |

---

## 三、业务逻辑缺口（处理链断点与算法未深挖）

### 缺口 1：shader 加载/反射/打包链——`.bin` wire 布局未 O 证（最高价值）

- 现状：`load_compiled<D>`（0x1409e3c50/0x1409e4160）→ `compiled_file_path` → `std::fs::read` → **read_from_buffer（0x1409e4670）** → Ok 0x140B。0x140B 载荷 = stages（`Vec<CompiledShaderStage>`=stage 枚举+entry_point String+spirv Vec<u32>）+ source_dependencies（Vec<String>）+ reflection（ShaderReflection）。**以上字段序全部来自 skeleton 名 + rebuild 镜像，不是来自 50KB 反编译**。
- 断点：① 50KB 的 extra-0x1409e4670 只被引用未分析；② 0x140B（本次实测复制长度）vs CALL-CHAIN 记录的 0x130B/0x150B 两数值来源不一致未裁决（卡点 13）；③ impl$4::read_from 的"2×read_u8 哨兵/收尾"是推断，trailing 行为 U（卡点 13）；④ `CompiledShaderPipeline` 反射字段序未逐字段 O 证（卡点 3）。
- 建议补法：分段解 50KB 反编译，标记每个 speedy Readable 读取点（read_vec_length/read_u32/String 的 helper 调用），把 CompiledWire 三段读取序钉死；用 `speedy::Readable::read_from_buffer` 的已知序列行为（前导长度 + 尾字节保留）与反编译对表，确认 0x140B 内不含额外头/尾。

### 缺口 2：gpu-profiler 时间戳换算链（四层链下游 U）

- 现状：四层链 `Profiler::new`（0x140c10f20）→ `begin_frame`（0x140c11090）→ `create_marker`/`write_marker`（0x140c110f0/0x140c11150）→ `end_frame`（0x1418c3910）→ 游戏侧 `retrieve_results`（0x140294cf0）全部有 O 行。**marker 无去重已由 0x140c110f0 确认**（push 不查重，与公开 gpu-profiler crate 的 create_marker 行为不同，fork 差异是有价值的业务发现，但未展开"下游 puffin 同名重复会造成什么"）。
- 断点：① `retrieve_results` 里 u32 timestamp×u64 值配对成 16B 条目的**元数据语义**（param_1[3..0x88] 逐字段）未映射；② `FUN_1405c3580`/`0x1405c75b0`（转换/排序）只有一句 I；③ `0x140bd3c80`（profiler 全局）语义 I——是 timestamp period 还是结果缓冲未定（卡点 6）；④ ticks→ns 的 period 来源 U（GpuTickDuration::nanoseconds 是 NotFound，见表面分析 #4）；⑤ `ProfilerBuffer`（游戏侧 raw/profiler 类型）布局未接。
- 建议补法：① rne-read 手解 GpuTick::duration_since + GpuTickDuration::nanoseconds + 0x140bd3c80 读取器；② 在 extra-0x140294cf0 反编译里把 param_1[3..0x88] 的源字段（param_2[0..0xd]）逐一配对，确定 16B 条目与帧元数据结构；③ 找到 period 写入者（render loop 初始化处）确认 `Profiler::new` 与 period 的关系。

### 缺口 3：BindingNameHash 消费链（`_dims` 后缀的语义未闭合）

- 现状：`BindingNameHash::new`（0x140c87660 O）+ FNV-1a64 helper（0x141937610 O）+ `with_suffix_dims`（0x140c87650 NotFound，verdict O/I 实为 I——decomp 文件是 NotFound JSON）+ `len_without_suffix`（I）。**哈希算法本身已 O 全解（本域最扎实的一块）**。
- 断点：① `with_suffix_dims` 的 O 证缺失（rebuild `wrapping_add(1)` 未与机器码对表）；② `calculate_name_hashes`（reflection.rs:105/112 闭包）把 binding_names → binding_name_hashes 的映射何时应用 `with_suffix_dims`（即 `_dims` 后缀在消费端什么时候 +1）未解；③ `_dims` 后缀的业务语义（数组纹理绑定名约定）未说明。
- 建议补法：rne-read 手解 0x140c87650（短函数，应是把哈希 +1）；在 CALL-CHAIN/raw 域找 `with_suffix_dims` 的调用点（draw_stream.rs PassHost::with_suffix_dims 已列，追踪其入参）把消费链闭合。

### 缺口 4：vk-sync AccessType u8 编码与 barrier 转换——完整字符表 U（核心算法）

- 现状：`is_write`（0x140c10e10）= `(byte)(c-0x28)<0x10`（写 0x28..0x37）O 证；`get_memory_barrier`（0x140c107c0 32B）`get_buffer_memory_barrier`（0x140c10940 0x48B）`get_image_memory_barrier`（0x140c10af0 0x50B）结构/sType（0x2e/0x2c/0x2d）O 证；`cmd::pipeline_barrier`（0x1418afd20）O 证。
- 断点：① **`get_access_info`（0x140c10dd0）完整字符→(accessMask, stageMask) 表 U**——这是 vk-sync 的核心算法，is_write 只给了写区间，读类型字符集（0x00..0x27）与每字符的双掩码值完全未解（卡点 12）；② 三个 barrier 结构经 device vtable +0x368 下发前的**翻译函数在 raw 域**（`GraphicsDevice::issue_pipeline_barrier`），fork 结构→真实 `VkPipelineBarrier2` 的映射不在本域，跨域契约未交接；③ stage 汇总语义（srcStage=汇总、dstStage=write 汇总、默认 0x2000=TRANSFER）只有描述，未与字符表交叉验证。
- 建议补法：**最高优先 rne-read 手解 0x140c10dd0**（查表函数，32 项内），从 get_memory_barrier 逐字符循环（0x140c107c0 有按名字串逐字符 get_access_info 的循环）里反推每字符掩码，产出完整 AccessType 字符表；再与公开 vk-sync-rs 的 AccessType 枚举对照（fork 改造点）。

### 缺口 5：aftermath 回调链——初始化函数身份与 Drop 全空

- 现状：`Aftermath::new`（0x140176e70 O）= 0x38B 拷贝 + 0x10B user_data{vtable} + 调 0x142923580（opcode 0x217, 版本 2,1）注册 4 回调（0x140c106c0/+0x18、0x140c10710/+0x20、0x140c10760/+0x28、0x14003bb80/+0x30 I）；`Status::wait_for_status`（0x140c104e0 O）轮询 2/4 + 50ms sleep；`get_shader_debug_info_identifier`（0x140c10620 O）。**注册链主体 O，是本域亮点之一**。
- 断点：① **`Aftermath::Drop::drop`（0x140c107b0）零分析**（见表面分析 #1，还带悬空证据）；② `0x142923580`（初始化函数）是 GFSDK_Aftermath DLL 导出还是封装函数未核实，opcode 0x217 与版本 (2,1) 语义 U（卡点 9）；③ `ResolveMarkerCallback` 0x14003bb80 ICF 折叠 437 符号、vtable+0x30 为推断（卡点 8）；④ `wait_for_status` 目标值 2/4 对应 GFSDK_Aftermath 枚举名 U（卡点 10）；⑤ `get_shader_debug_info_identifier` 调的 0x142923870（opcode 0x217）是哪个 GFSDK 入口未核实。
- 建议补法：补跑 0x140c107b0（Drop）与 0x142923580/0x142923870（opcode 0x217 分派函数）反编译或 rne-read；用 Aftermath SDK 头（GFSDK_Aftermath.h）对 GFSDK_Aftermath_ResultCode 枚举值 2/4、以及初始化入口原型，把封装函数身份钉死。

### 缺口 6：dxgi 枚举——40B 元素字段映射 U

- 现状：`enumerate`（0x1418b29c0 O）= CreateDXGIFactory → EnumAdapters1（0x14213b8e0）→ GetDesc1（0x14213b950）→ WCHAR 扫描 → String → push `DxgiAdapter`（40B {name String@0 + u32 id 字段 @0x10..0x28}）→ Ok(Vec)/Err。枚举链 O，是本域亮点之一。
- 断点：① **40B 元素 @0x10..0x28 的 u32 字段精确映射 U**（vendor/device/subSys/rev/dedicated memory 哪些被取、顺序如何，卡点 11）；② `EnumAdapters1` 循环终止条件（返回 `DXGI_ERROR_NOT_FOUND` 才停？）未述；③ `DxgiError` 类型与 Err 判别 `0x8000000000000000` 的 Rust 语义（判别与 Ok 载荷共用 offset 0 的 niche）未定（卡点 11）。
- 建议补法：手解 0x1418b29c0 中 GetDesc1 0x130B 拷贝后的字段搬运段（定位 mov 序列搬运 DXGI_ADAPTER_DESC1 的 VendorId/DeviceId/SubSysId/Revision/DedicatedVideoMemory），确定 40B 元素布局；对照 raw/physical_device 的枚举消费点验证。

### 缺口 7：rebuild 反射生产链（shader_compiler 候选）未纳入分析

- 现状：ALGORITHMS.md §0.1 一句"编译在外部 shader-compiler/重建候选 shader_compiler 中"，但**从未分析重建候选** `reconstruction/rhapsody-rebuild/src/shader_compiler/{preprocess,reflect,pack}.rs`。该候选是编译侧 producer（reflect.rs::generate_reflection 正是原反射 schema 的生成逻辑，含 canonical_name、$Globals uniform 块、BindingNameHash::new 应用、跨 stage 合并冲突检测），与 crates.json 的 reflection 族逐函数对应。
- 断点：① shader_compiler 候选与原始 reflection producer 链无逐项对照行；② 候选与 ALGORITHMS.md 的 reflection 域结论（BindingNameHash=O、ShaderReflection 字段序、descriptor_sets HashMap）未交叉引用；③ pack.rs 的 `trailing_bytes: Vec::new()` 与 CompiledShaderPipeline 尾字节语义冲突未裁决。
- 建议补法：把 shader_compiler 三文件纳入 rebuild 对照表（§四），用 MODULE_STATUS 的 404 包/638 阶段/397 typed 反射相等数值核对候选反射产出。

---

## 四、rebuild 对照缺口

任务提示的 rebuild 对照四项复核结果：

| # | 对照项 | ALGORITHMS.md 现状 | 缺口 | 建议补法 |
|---|---|---|---|---|
| 1 | **rebuild `RasterPipelineDesc` 4 字段 vs 原 3×24B** | §3/§4 已记：rebuild `{source_path, vertex_entry, fragment_entry, defines}`=4×24B vs 原 0x48B=`{path String@0, 空 defines@0x18, path String@0x30}`；卡点 1 已列 | 已覆盖 ✓，但**修复方向卡死**：@+0x00 与 @+0x30 谁是 source_path/entry、为何克隆两份 path 未定，rebuild 无法据此改字段 | 找 raster 带 entry 的构造实例（variants 消费路径）或再解 new_vertex_fragment 的调用点，把两 String 语义定案 |
| 2 | **rebuild 未引 gpu-profiler 依赖 vs 原本地 crate** | §3.2/§3 已记：rebuild 无 gpu-profiler 依赖，仅候选 `raw/profiler.rs::GpuProfiler`（QueryPool+span 名，无 marker 链/无 SubmittedFrame/无 retrieve_results）；**已指出 CALL-CHAIN §3.2"rebuild 用真实 gpu-profiler 依赖 ✓"与 Cargo.toml/lock 实测不符** | 已覆盖 ✓ | 已在 ALGORITHMS.md 指正；审查确认属实。rebuild 候选与四层链等价性未做，需在 MODULE_STATUS 登记该候选的差距 |
| 3 | **shader_pipeline 反射 210/404** | §3 反射行只给"✓（半）"（原 Hash 未 derive、字段序未 O 证）；**未引用 rebuild 侧 MODULE_STATUS 的 404 包/638 阶段/397 typed 反射相等数值** | 反射对照停留在结构级，缺数量级交叉验证 | 在 §3 反射行补 rebuild 验证数值（404/638/397），并把"210"所指（疑为 reflection 域骨架或 S_PUB32 计数，见下注）核对清楚 |
| 4 | **aftermath/vk-sync/dxgi rebuild 缺失** | §3 全记 ✗：aftermath 无、vk-sync 无（`rg/resource_rules.rs:41` 显式接收外部分类器替代 `AccessType::is_write`）、dxgi 无（原 `raw::physical_device` 自己枚举 vk data） | 已覆盖 ✓ | 已记录；vk-sync 的替代分类器需与 get_access_info 全表对齐（见 §三缺口 4） |

**补充对照缺口（ALGORITHMS.md §3 未覆盖）：**

| # | 对照项 | 缺口 | 建议补法 |
|---|---|---|---|
| 5 | **shader_compiler 候选无对照行**（新增候选） | §3 rebuild 对照表只有 shader_pipeline/gpu-profiler/vk-sync/aftermath/dxgi 5 行；`shader_compiler/{preprocess,reflect,pack}.rs` 未进表 | 增行：preprocess.rs::prepare_file（include 展开/#define 注入/#line）与 reflect.rs::generate_reflection（反射 producer）与 pack.rs（容器打包）对标原编译链，判定 ✓/半/✗ |
| 6 | **`Type::nbyte` rebuild 尺寸来源** | §3 已记"rebuild 经 spirq 计算 vs 原自算分派表，等价性未逐 case O 证"（卡点 5） | 用 ScalarType::nbyte 分派表手解结果（§三缺口 4/表面 #5）逐 case 核对 rebuild 的 spirq::nbyte |
| 7 | **`trailing_bytes` 建模** | §3 已记"方向对但原行为未 O 证"（卡点 13） | 解 50KB read_from_buffer 后把尾字节行为钉死（§三缺口 1） |
| 8 | **load_compiled 泛型→path 直传** | §3 已记"原泛型于 PipelineDesc（compiled_file_path 生成路径）；rebuild path 直传；原游戏侧经 ShaderBackend::load_compiled_* + Arc 使用，rebuild 未接"（卡点 14） | 已覆盖 ✓；接链依赖 §三缺口 1 的布局定案后，补 `ShaderBackend` trait 面接线 |

> **注（任务提示 "shader_pipeline 反射 210/404" 的核实）**：在重建侧能找到的明确数值是 `MODULE_STATUS.md:83` 的 **"404 个新包／638 阶段，完整 typed 反射 397/404 相等"**（tests 断言 `(404, 638)`）；"210" 未在 crates 域证据中出现——疑指 reflection 域的骨架函数计数或 S_PUB32 反射符号计数，审查期间无法唯一确定，建议在补查时同时把该数字指认清楚再登记。

---

## 补全优先级排序

| 优先级 | 项 | 理由 |
|---|---|---|
| **P0** | ① 手解 `vk_sync_fork::get_access_info`（0x140c10dd0）完整字符表（rne-read） | vk-sync 核心算法，is_write 之外全 U；直接决定 vk-sync 缺失的 rebuild 替代分类器能否对齐 |
| **P0** | ② 解 50KB `read_from_buffer`（0x1409e4670）→ `.bin` wire 布局逐字段 O 证 | shader-pipeline 主线（load_compiled 载荷 0x140B 字段序/trailing），连锁影响卡点 3/13/14 与 trailing_bytes 判定 |
| **P1** | ③ 补 `Aftermath::Drop::drop`（0x140c107b0）证据 + **修 ALGORITHMS.md 悬空引用 extra-0x140c107b0**（该文件不存在，头注"补取 7 个"与实 9 个不符） | 悬空证据是交付卫生问题，Drop 析构是全空白 |
| **P1** | ④ gpu-profiler 时间戳换算链（GpuTick/GpuTickDuration/0x140bd3c80/FUN_1405c3580/0x1405c75b0） | retrieve_results 下游语义，直接影响 game-side puffin 数据正确性 |
| **P1** | ⑤ `Type::nbyte` case→variant 映射 + `ScalarType::nbyte` 分派表手解 | 连锁 rebuild nbyte 尺寸等价性判定（对照缺口 #6） |
| **P2** | ⑥ defines 哈希算法（`calculate_shader_defines_hash`/`calculate_hash<tuple>`）从发行 .bin 文件名 key 反推 | 文件键语义，影响 .bin 命名/寻址 |
| **P2** | ⑦ `RasterPipelineDesc` 两 String 字段语义定案 | 卡点 1，阻塞 rebuild 结构修正 |
| **P2** | ⑧ dxgi `DxgiAdapter` 40B 字段映射手解 | 卡点 11，收尾 dxgi 域 |
| **P3** | ⑨ 6 个未覆盖 compiled fmt 函数登记（derive 复用标注） | 满足 PLAYBOOK"compiled_procedure 全覆盖"的合规项；fmt 大概率胶水 |
| **P3** | ⑩ shader_compiler 候选入 rebuild 对照表 + 反射 404/638/397 数值交叉引用 | rebuild 对照完整性（对照缺口 #3/#5） |
| **P3** | ⑪ 反射域 inline eq/hash/fmt 与解码闭包组登记（表面 #7/#8、未覆盖 #7-#11） | 完整性兜底 |
