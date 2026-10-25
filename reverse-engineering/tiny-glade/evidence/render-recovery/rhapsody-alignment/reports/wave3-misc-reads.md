# 第三批：raw 域零散未读项闭合（wave3-misc-reads）

图例：✅原版对齐[证据地址] / ⚠️候选 / ❌读不出。工具：`/tmp/dis.py`（oracle `.rne`）、`/tmp/xref.py` `/tmp/callers.py` `/tmp/locscan.py`（自写，扫 .text 的 lea/call 与 .rdata 的 panic Location）。

## 1. 管线创建 Err 分支文案
✅ raster（0x1416a3720 起，冷尾段 0x1416a4494..0x1416a46f1）逐个 panic 点读出（Location = shader.rs:行:列）：
* 796:18 `Result::unwrap()`（"called `Result::unwrap()` on an `Err` value"，0x142c88708，43 字节）——`call [device+0x210]` = vkCreatePipelineLayout，Err=vk::Result。
* 838:26 `panic!("Unsupported vertex attribute type: {:?}", ty)`（片段 0x142c88be0，35 字节；Debug fn 0x1420bdb40）。
* 840:47 `Option::expect("vertex input attrib byte count")`（0x142c88b78，30 字节；`Type::nbyte()`=None，调用点 0x1416a3ba2）。
* 972:18 `.expect("Unable to create graphics pipeline")`（0x142c88b3c，34 字节；Err=ash 的 `(Vec<Pipeline>, vk::Result)`）。
* 825/827/932 为 `panic_bounds_check`，857:87 为 `handle_alloc_error(8,0x98)`（无用户文案）。
✅ compute（冷尾段 0x1416a4f1a..0x1416a50fe，Location shader.rs:397..460）：
* 397:9 `assert!(compiled.reflection.inputs.is_empty())`（0x142c88c30，55 字节）；460:60 `.expect("a valid group size")`（0x142c88cf8，18 字节，`local_size` 为 None）；453:39 `panic!("Failed creating shader pipeline {name}: {err:?}")`（片段 0x142c88d50 = "Failed creating shader pipeline " / ": "）；426/437/428:78 为 `unwrap()` on Err，428/434 为 `[0]` 越界。
落码（`raw/shader.rs`）：`original_panics` 常量模块；`InterfaceError::{VertexInputByteCount, UnsupportedVertexAttributeType, ComputeHasVertexInputs, MissingGroupSize}`（Display 逐字）；`vertex_input_from_reflection` 先 `nbyte()` 再分派；`validate_compute_reflection`（先 assert 后 expect）接入 compute 的 compiled 分支。
✅ render pass：`vkCreateRenderPass` 失败 = 普通 `unwrap()`（render_pass.rs:124:18，Location 0x142b16b08，调用点 0x140bfc438；常量 `CREATE_RENDER_PASS_FAILURE_LOCATION` 上批已有）。关闭上批 ❌。
✅ framebuffer 两条 panic 文案接入 `Framebuffer::from_desc`：新增 `try_extent_from_attachment_sizes`（Err 文案=原 panic 文案），从附件（按 base_mip 缩放）推尺寸，无附件/尺寸不一致返回该 Err（原版 panic）。⚠️ 仍不用推导出的 extent 覆盖调用方 extent。
测试：`tests/wave3_shader_panics.rs`（2）、`framebuffer::original_panic_tests::result_form_carries_the_original_messages`。

## 2. 字符串/字节语义
* `RenderPassAttachmentDesc +0x0d..0x0f`：✅ 结论更正——是 `{u32,u32,u32,u8}` 的**尾部填充**。`new`（0x140be6780）只写 `[+0]`、qword `[+4..+0xc)=0`、byte `[+0xc]=1`，**不清零**这三字节（上批"被 new 清零"作废）；builder（0x140be67a0/67c0）按字段拷贝。⚠️ 缓存键哈希是否按字节含填充未证实（读到的消费端均不读）。文档已改。
* `RasterPipelineDesc@0x30`：⚠️ 构造器 0x1409e3980 把 path 克隆两份（+0x00 经 0x140d7bd90、+0x30 原样 memcpy），+0x18 defines 空 Vec；loader 0x1409e4160 只把 desc 交给 key 函数 0x141796080（读 +0/+0x18），**不读 +0x30**；四个调用点（0x1414b4bb7 等）把 0x48B desc 装入 0xd8B 载荷（疑似枚举变体）交 0x140936ef0。没找到读者，未穷尽全 .text → "仅发现构造写入，读者未证实"，结构未改。compute 的名字 `{name}` 只在 453:39 的失败文案里使用（可能即 name 的来源，未证实）。
* `ComputePipelineDesc/RasterPipelineDesc::new` 字段语义：无新增读出。

## 3. 多宏 defines 折叠 ✅（找到真实样本）
样本：`evidence/render-passes/shaders/oracle-bin/*.bin` 文件名 + `evidence/shader-gap-investigation-2026-10-13/compute/original-compute-tuples.json`（原 producer 的 defines 列表）。发行 compute 共 14 组 (defines→键)，其中 **5 组含 2 个宏**：`gauss_blur`（PERM_fmt="rgba32f"/"r16" × PERM_ty）、`tgsr`（PERM_wave32 × PERM_sharpen，4 组）。用 `ShaderDefines::artifact_hash` 对拍：10 个键（含 4 个单宏）**全部逐位一致**（如 `[("PERM_wave32","0"),("PERM_sharpen","1")]`→0x3ee6a953408e4c4c）。反序给出同键。结论：折叠 = 每对 SipHash-1-3(0,0) 的 XOR，与顺序无关，**原版无需也未观察到排序/去重**（重复对抵消是 XOR 的必然，原版同）。测试 `shader_defines::tests::shipped_multi_define_compute_keys_reproduce`。旧"无多宏样本"作废。

## 4. 访问码 0 / 0x34 / 0x22 ✅
访问码表（`access_table.rs`，0x142b1a6c8/7c8/8c8，已含全部 64 项）直接给出：`0`=无 stage/access/layout 0（TOP_OF_PIPE 回落）；`0x34`=TRANSFER/TRANSFER_WRITE/TRANSFER_DST_OPTIMAL(7)；`0x22`=ALL_COMMANDS(0x10000)/SHADER_READ(0x20)/SHADER_READ_ONLY_OPTIMAL(5)（公开版 AnyShaderReadSampledImage）。屏障经 `get_image_memory_barrier`（0x140c10af0）：src_access 只收写码，dst_access 全收。`setup.rs` 新增 `code_image_barrier` + `cmd_code_barrier`，`init_texture_zero_fill` / `upload_regions` 改为表驱动（0→0x34→0x22；旧内容更新用 0x22→0x34→0x22）。结果与旧"公开掩码近似"数值相同（旧近似恰好正确），偏差说明已删。测试 `raw::setup::access_code_tests`（3，含 0x22→0x34 无 src access）。⚠️ 未改：`upload_regions` 的 update 路径原版 `old_access` 实参未读（初次上传=0）；同步 setup CB 偏差仍在。

## 5. 设备排序键 ✅（手解，无多设备动态样本）
`evidence/**/device-policy*`/`device_counters` 无多设备输入输出样本（仅 `tooling/batch04-device-tests.json`，非原版回放）。逐指令手解 0x1418b9130：类别跳表 0x142d03954 → {0:0x1418b917f Discrete，1:0x1418b91e9 Integrated，2:0x1418b9237 Virtual，3:0x1418b9252 Cpu，4:0x1418b91cc Other}；Discrete/Integrated 先查 "Direct3D12"(len10) 再 "Microsoft Basic Render Driver"(len0x1d)（0x140bd7250，区分大小写子串），命中→-2000，否则 -4000/-3000；Virtual 入口默认 -2000；Cpu -1000；Other 0（`xor esi,esi`）；键 = base − (local `shr 0x1e`)，`cmp; jge` 严格 `<` 前移。与 `device_order_key` 一致，无改动。新增 `tests/wave3_device_order.rs`：5 类别 × 4 名字（含大小写反例）× 6 堆大小全矩阵 + tie 稳定性。⚠️ 仍无原函数动态注入。

## 6. 其它遗留
* `inner+0x7c==2`：⚠️ 新证据：`0x140c13e41..0x140c13e4f` 在函数 0x140c12dc0..0x140c1422b（内含 gpu-allocator 错误串，疑 `allocate`）的错误路径写 `[out]=0; [out+0x44]=2`（out+0x44 ≡ inner+0x38+0x44 = inner+0x7c），随后 `lock dec` 释放 Arc——即 `Result<Allocation,_>` Err 的 niche，与 `create_texture` 的 `!=2` 判断一致。全 .text 再扫 `mov byte [reg+0x44],2` 仅 3 处（另 0x141af1aab、0x141b3b882，未读）。"取走分配"（Option::take）的写者仍未定位。
* `flag@0x16`：✅ 即 `ResourceType` 字节（Debug 0x140bd6bd0，长度表 0x142b10168={7,6,9,11,10,7,23,11}）：Texture0 Buffer1 Swapchain2 TextureView3 RenderPass4 Sampler5 PersistentDescriptorSet6 Framebuffer7，与已证 0/1/6 吻合；已写进 `TextureRecord` 注释。
* crash `log_gpu_debug_info` 其余分支：上批已核对 kind 1..6 与日志行（raw-init-instance-disposal.md）；本批无新增，未再发现未读分支。
* `GraphicsDevicesForSurface::new` 内联体：⚠️ 调用点 0x1405953b7（`available_devices` 0x140bdb820）→0x1405953e6（`missing_devices` 0x140bdb090）→0x140595410（`order_and_filter_devices` 0x140bdbc30）。新发现：**DXGI 枚举 0x1418b29c0 是在 `missing_devices` 内部（0x140bdb18c）调用**，不是 `new` 里先枚举再传入。`Instance::create` 前后的其余内联体未读。
* `DxgiAdapter.device_id`：✅ 原 40B 记录没有它。0x1418b2ad0 把 `DXGI_ADAPTER_DESC`(0x130B) 拷到 rbp-0x50，只取 `Description→name`、`Luid(Low@+0x128,High@+0x12c)→+0x18`、`VendorId@+0x100→+0x20`；DESC+0x104 的 DeviceId 未取，`+0x24` 是未写的尾填充。`device_id` 仅为本实现候选字段（文档已改）。

## 7. `Type::nbyte` 逐 case ✅
跳表 0x142fd8f84（14 项，default=10）逐项解出：0 Scalar、1 Vector、2 Matrix、3..8 opaque→None、9 Array、10 Struct、11 AccelStruct→None、12/13 地址→Some(8)；子表 0x142fd8fbc/0x142fd8fd0/0x142fd8fe4 解 Scalar tag。Vector=`scalar*nscalar`；Matrix=`(stride 有? stride : scalar*nscalar)*nvec`（`imul` 在 0x1420bdb26）；Array=`stride.unwrap_or(0)*nrepeat.unwrap_or(0)`（0x1420bdb09 两个 Option 标签字节）；Struct=末成员 `offset + nbyte.unwrap_or(0)`（无尺寸成员取 0，仍 Some）。`reflection_ty.rs` 新增 `nbyte_tests`（4）：字面向量 + 6 标量×nscalar×nvec×stride 全网格对拍机器码模型 + 与 spirq 实现比对——全部一致，无需改实现。

## 改动文件
`raw/{shader,setup,framebuffer,render_pass,texture,init}.rs`、`crates/shader-pipeline/src/{shader_defines,reflection_ty}.rs`、新测试 `tests/wave3_device_order.rs`、`tests/wave3_shader_panics.rs`。
## 验证
全量命令被他人的 `tests/wave3_submit.rs`（`vk::DeviceFnV1_*` 不存在）编译失败阻塞；排除该目标后：rhapsody（lib+43 目标）294 passed、tiny-glade+country-core+shader-pipeline 12 目标 1574 passed，合计 **1868 passed / 0 failed**，`original_oracle*` 无回归。
## 遗留
RasterPipelineDesc@0x30 读者；`+0x7c==2` 的 take 写者；0x141af1aab/0x141b3b882 两处 `+0x44=2`；update_texture 的 old_access 实参；`Instance::create` 周边内联体；原函数级设备排序动态注入。
