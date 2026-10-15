# rhapsody 审查轮合并清单（REVIEW-UNEXPLORED MERGED，2026-10-15）

来源：5 域审查 agent（rg / raw-core / raw-init / managed / crates）各自 `domains/<域>/REVIEW-UNEXPLORED.md`。
方法：对照骨架清单逐函数核对 ALGORITHMS.md + 抽查反编译证据 + rebuild 当前代码复核。
结论口径：✓已分析 / △表面（有行无深度）/ ✗未覆盖。

## 覆盖总况（466 compiled 名义覆盖，深度不均）

| 域 | compiled | ✓ | △ | ✗(无行) | 有地址进表 | 反编译真实性 |
|---|---|---|---|---|---|---|
| rg | 107 | ~86 | ~14 | 7（6 派生+1 业务：into_sampled_rg_texture） | 57/57 | 51 真实 O；11 NotFound 仅 I/U |
| raw-core | 184 | 高 | 中 | 1（query_device_extension_support::closure$0） | 94/94 | 95 份中 **47 是 NotFound**，48 真实 |
| raw-init | 87 | 高 | ~17 | 1（impl$26::fmt） | 47/47（10 NotFound 全 rne-read 补证） | 46+手解 |
| managed | 42 | 全有行 | 深度不达标 | 0 | 12/12 | 12 全成功 |
| crates | 46 | 高 | 11 | 6（reflection fmt 全族） | 37/37 | 28 批量+9 extra（1 悬空） |

**统一结论**：名义覆盖率 >95%，但"有行≠有深挖"——大量 compiled 只到 I/U、证据引用有悬空/错域、部分 rebuild 对照判定已随代码演进过期。

## 已推翻的结论（审查纠错，供集成轮作废）

1. **rg「image_access_mask_to_usage_flags rebuild 全错位」= 误判**：逐值核对 12 对，原机器码与 ash ImageUsageFlags 常量数值完全一致（0x20→4=SAMPLED、0x40→8=STORAGE、0x80→0x10=COLOR_ATTACHMENT、0x200→0x20=DEPTH_STENCIL、0x800→1=TRANSFER_SRC、0x1000→2=TRANSFER_DST）。**作废，勿改 rebuild 该处**。
2. **raw-core「allocate_descriptor_set rebuild 用 BTreeMap 自造」= STALE**：当前 rebuild `descriptor_set.rs` 已是空闲池复用+OUT_OF_POOL/FRAGMENTED_POOL 循环重试（DescriptorPoolQueue/RetiredDescriptorPool），与原版一致。ALG §3 与 a5 §7 抄了旧代码。**应改判 ✓/半**。
3. **raw-core「bind_descriptor_set/bind_persistent_descriptor_set rebuild 缺」措辞过强**：rebuild `binding.rs:89/112` 已有 free fn，缺的只是"GraphicsDevice 方法"身份。

## 跨域 P0（下一轮优先闭合，直接决定业务正确性）

| # | 缺口 | 域 | 建议补法 |
|---|---|---|---|
| P0-1 | **Buffer 40B 生命周期链**：tag@0 的 0xb 语义、create_buffer 80B→40B 截取、+0x20 字段、inner 双属性位 | raw-core | 反编译 create_buffer_impl 0x140bee200 全链 + rne-read 字节级定案（证据已在手） |
| P0-2 | **GraphicsDevice::create 展开**（155KB/2625 行只浓缩 1 行） | raw-core | 分段反编译，产出逐段纪要（这是整个 raw 层核心构造） |
| P0-3 | **退役分派链闭合**：immediate_empty_bucket(0x140bf92f0)/drop_in_place\<DisposalBucket\>(0x140bdfcf0) S_PUB32 身份 + 10×DisposalBucket 桶数组 + tag@0xe 分派表 | managed | 反编译 take_out_dropped_waste(0x1418c0a70) 全链 + rne-read tag 表 |
| P0-4 | **ManagedResource\<T\> 槽布局**：decomp-0x1418aeba0 已暴露 slot{+8 inner,+0x10 checksum,+0x14 device,+0x16 tracked}/inner{+0 seq AtomicU64,+0xc checksum,+0x10 resource} | managed | 从已有反编译落实字段序，空闲链表追插槽路径 |
| P0-5 | **refcount 原子宽度** AtomicI32 vs AtomicU32 | managed | rne-read 0x140bd6b00（clone 的 lock xadd 宽度）定论 |
| P0-6 | **get_access_info 字符表手解**（0x140c10dd0） | crates | rne-read 字符表 + AccessType u8 编码（is_write=(c-0x28)<0x10）全表 |
| P0-7 | **read_from_buffer 50KB wire 布局**（extra-0x1409e4670 完全未分析） | crates | 分段反编译，定 .bin 字段序/trailing |
| P0-8 | **env 链完整化**：apply_vulkan_blocklist ≥19 变量（实测 0x142b19400..196f8：EOS_OVERLAY_DISABLE/VULKAN_WIN64_DISABLE/reshade/Twitch/MANGOHUD/gshade/VK_LAYER_* 通配/VK_LOADER_LAYERS_DISABLE~implicit~ 组合） | raw-init | rne-read 字符串表完整枚举 + 触发条件 |
| P0-9 | **设备排序比较器**（closure$0/$1 未恢复） | raw-init | 反编译排序调用点，定比较准则 |
| P0-10 | **滞回公式 + 快照排布**：refresh_memory_budget 除数实为 max(1,旧/8)；峰值快照字段 +0x620..+0x680 排布 | raw-init | rne-read 公式 + 布局 |
| P0-11 | **NotFound 11 个 rg 函数 rne-read 补证**（BatchCompiler::new/pass_access_range_indices/PassPgoNameId::from_name/end_render_pass/set_line_width/ResourceSyncGroup 三件套等） | rg | 逐个 rne-read 原字节手解，结论从 I/U 升级 |
| P0-12 | **into_sampled_rg_texture**（唯一业务型无行 compiled） | rg | 从调用点 import_export 链反推职责 |

## 跨域 P1/P2（次要补全）

- **rg P1**：OOM 降分辨率路径（compile 失败重试链 identified 未消化）；temporal Record 0x68B 字段序；compile ABI/RenderGraphState 布局。
- **rg P2**：屏障计算 3 方法（compute_batches/add_pre_pass_barriers/gather_batch_preamble_barriers）I→O；SynchronizationScope 0x50B 布局；ResourcePool 回池 insert 调用方接线；ResourceExportHandle Rust 定义。
- **rg 对照**：temporal 复用路径原版 import_unowned vs rebuild import_owned 差异；into_sampled_rg_texture/ResourceSyncGroup::can_be_concurrent_with/validate_read_access 补对照行。
- **raw-core P1**：reclaim_command_buffer_resources（25KB）/wait_idle/wait_idle_and_recycle_resources 大函数展开；17 行 verdict U 升级；30 行 O(消费端) 批量 rne-read 复验；全局退役哈希表 0x1436308f0；init_texture_data 跳表；DynamicConstants 双 region。
- **raw-core P2**：STALE 3 项改判落文档；引用卫生（decomp-0x140bee030/090/150 实为 n1-device 的 .c）；§1.1 文件 compiled 计数修正（texture 26 vs 47 等）；bind_descriptor_set 第 4 实例地址。
- **raw-init P1**：implicit_layers 返回 0x208B 切片语义；disposal 整表锁 0→8 + 1s 慢路径 + hashbrown map 偏移（+0x28 vs +0x8）矛盾裁决；5 处事实偏差回填（NotFound 9→10、dispose_cb_resource 6→7、断言串地址 0x142b12f50）。
- **raw-init P2**：workarounds 完全缺失登记；crash.rs 的 leekspin::RingBuffer 恢复状态。
- **managed P1**：GpuOwnedState 变体与字段序；swapchain 锁字（写0→8/读+0x10）rne-read + Cargo.lock 对照；disposal 对照 ✓→✗/半 改判。
- **managed P2**：三 fmt 身份、Swapchain::new 分节、视图缓存、create_* 下沉。
- **crates P1**：Aftermath Drop 证据 + 悬空引用修正；gpu-profiler 时间戳换算（period 来源）；Type::nbyte case 映射；with_suffix_dims 判定降 I。
- **crates P2**：defines 哈希算法；RasterPipelineDesc 字段语义；dxgi 40B 字段映射；shader_compiler 候选对照行。

## 数据卫生清单（回填/修正 ALGORITHMS.md 与台账）

1. crates：ALG 头注"补取 7 个"实 9 个；extra-0x140c107b0 悬空引用；Aftermath::Drop::drop 证据文件不存在。
2. raw-core：decomp 引用错域（0x140bee030/090/150 → n1-device）；compiled 计数不符。
3. raw-init：NotFound 计数、dispose 实例数、断言串地址。
4. managed：get_maybe_tracked 地址挂在 trait 声明行（应挂单态行）。

## 结论

审查轮的核心价值：**推翻了 1 条错误修正 + 1 条 STALE 对照 + 1 条过强措辞**（避免集成轮改错/改多余的代码），并把"深度不达标"精确定位到 **P0 12 项**（多数证据已在手，只需落实）+ 各域 P1/P2。下一轮 = 按 P0 逐项闭合，P0 完成后重跑受影响反编译，最后汇总全量差距清单。
