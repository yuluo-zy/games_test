> 第二轮复审与新增修复见 [复审报告](../../../../reports/asset-libraries-rereview-2026-10-18/README.md)。

# 修复状态（2026-10-18）

下文是修复前的历史审查。六项问题已在本轮工程化中处理，原行号和“当前未实现”描述只适用于审查时源码，不能继续当作最新缺口。

| 原问题 | 当前处理 |
|---|---|
| CPU 影子强转 | NoUninit + bytemuck 检查，句柄按字段构造 |
| GPU 空更新 | Result 错误传播，TextureHost 接真实上传、视图和 BindlessHeap |
| D16 长度 | 复用渲染库格式表与受检查长度 |
| 描述丢失 | 完整描述随预设保留，导入接口显式传入 |
| 热重载 / VAO 孤儿 | runtime 实际消费者与 PostUpdate 调度 |
| 瞬态回收孤儿 | 真实宿主初始化 + 每帧回收与物理缓冲释放 |

实现变化、回归结果及剩余边界见 [最新报告](../../../../reports/asset-libraries-refactor-2026-10-18/README.md)。旧报告的源码指纹及差分范围不覆盖本轮改动。

---

# asset_libraries 代码审查（2026-10-18）

审查范围：本目录 13 个 Rust 文件，以及资源类型、设备/图边界、startup、backend、prefab 和 workspace 调用点。本轮按当前源码审查；未重新查询原二进制或升级原作等价结论。业务说明见 [README.md](README.md)。未修改业务实现。

P1 表示应优先修复的安全性或关键功能问题，P2 表示明确的正确性或集成问题。以下将库内缺陷和运行期集成缺口分别说明。

## 1. [P1] CPU 影子的 u32 转换缺少无填充约束

位置：[ssbo_library.rs](ssbo_library.rs) 第 91–113 行，`Ssbo for ShadowedBuffer<T>::cpu_shadow_words`。

实现只检查 `size_of::<T>() == 4`、`align_of::<T>() == 4`，随后将 `Box<[T]>` 强转为 `&[u32]`。但 [StorageElement](../render/ssbo.rs) 只要求 `Copy`；`Copy` 不保证对象每个字节都已初始化。

例如完全合法的安全 Rust 类型：

```rust
#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct Padded(u8);
impl StorageElement for Padded {
    const NAME: &'static str = "Padded";
}
```

该类型大小和对齐均为 4，但有 3 个填充字节。它可由安全调用方作为 `ShadowedBuffer<T>` 的元素满足当前 blanket impl；读取返回的 u32 会读取填充字节，违反 Rust 初始化要求。注释声称的“位模式类型”并未由 trait 约束保证。

建议：只对已证实的 u32/无填充类型提供该视图，或引入并强制相应的无未初始化字节约束，使用受检查的转换接口。不要仅增加 size/alignment 断言。

验证：静态安全性分析，未执行会读取未初始化内存的探针；未声称已在当前游戏输入触发或通过 Miri 复现。

## 2. [P1] 生产纹理适配未执行更新，却允许调用方按成功推进

位置：[texture_library.rs](texture_library.rs) 第 159–182 行及第 490–517 行；相关边界 [rhapsody_contract.rs](../rhapsody_contract.rs) 第 177–213 行。

`TextureLibraryDevice for GraphicsDevice` 存在三个关联问题：

- `get_vk_image_view` 将资产 `texture.id` 直接当作 Vulkan image view 返回，未查询真实 view。
- `update_texture` 丢弃纹理和数据参数，直接 `Ok(())`，因此调用清零接口无法保证 GPU 被清零。
- `update_persistent_descriptor_set` 丢弃 dset/updates，只记日志；`bindless_textures_dset` 此前已取走 pending，随后返回描述符集，更新任务不会因未完成而保留。

此外，当前 `DescriptorSet` 是零大小占位，创建桥也只返回占位。这是已登记但仍影响功能的集成缺口；日志并不能让上层获知失败。

触发条件：把已有纹理加入库并请求 bindless 描述符，或在建立预设后通过该设备适配调用 clear。生产纹理创建目前会明确返回 unsupported，所以默认完整启动链更早就会受阻；本条不声称已经在真实启动中观察到错误画面。

建议：接入真正持有 ManagedTexture、image view 和 BindlessHeap 的宿主适配。在未接通时返回明确错误；描述符更新接口需能报告失败，并在确认成功后提交队列状态。

验证：直接检查生产实现和队列控制流。GPU 未执行；单元测试使用 MockDevice，不能验证这些生产路径。

## 3. [P2] 预设 10 的清零数据长度是正确值的两倍

位置：[texture_library.rs](texture_library.rs) 第 219–225 行、416–432 行。

[TexturePreset](../resources/texture_presets.rs) 的预设 10 格式为 `0x7c`（D16_UNORM）。当前 `format_bytes_per_texel` 没有覆盖它，走 `_ => 4`；仓内已恢复的 `rhapsody::raw::format::observed::try_vk_format_info(...).word_at_04()` 为 2。

512×512×1 的清零应提供 524,288 字节，当前提供 1,048,576 字节。真实后端可能拒绝数据布局或产生与预期不一致的上传；当前空更新适配掩盖了该差异。helper 对所有未知格式猜测 4 也会继续隐藏问题。

建议：复用锁定渲染依赖的已恢复格式信息，或明确覆盖所支持格式并拒绝未知值。无需再维护一个不完整的独立格式表。

验证：遍历全部 25 项预设，与仓内 observed 表比较，唯一当前预设差异是 `(preset=10, format=124, actual=4, expected=2)`。这是当前源码与既有原证据表的比较，不是本轮重新执行原二进制。

## 4. [P2] 图导入会丢失纹理 mip、数组层数与用途信息

位置：[texture_library.rs](texture_library.rs) 第 194–202 行；[rhapsody_types.rs](../rhapsody_types.rs) 的 `Texture` 第 189–197 行。

创建时设备接收到完整 `TextureDesc`，库保存的 `Texture` 却只有 id、宽高和 format。`TextureRenderGraph for RenderGraph` 导入时使用 `TextureDesc::new_2d` 重建描述，mip、数组层、sampled/storage/附件用途均回到默认值。

具体反例：预设 8 原有 5 个 mip，`get_rg_texture(...).desc.mip_levels` 返回 0；预设 0x11 原有 3 个数组层，导入后返回 0，kind 从数组的 3 变成普通 2D 的 2。预设 0x12 同样受影响。因此消费端取得的描述与已创建资源不一致，即使导入后端已连接也无法弥补这次信息丢失。

建议：保存完整且权威的资源描述，或通过真实资源注册表查询。图导入应携带纹理描述和物理资源身份，而不是仅把纹理 ID 传给 buffer 形状的接口。

验证：使用真实库公开接口、注入 CPU 设备和 `BufferGraphBackend` 执行，两项断言分别得到 `0 != 5`、`0 != 3`。不依赖 GPU。

## 5. [P2 / 集成缺口] 热重载通知与 mesh 修改记录没有生产消费者

位置：[configwatch.rs](configwatch.rs) 第 112–115 行、[meshwatch.rs](meshwatch.rs) 第 106–110 行、[texture_watch.rs](texture_watch.rs) 第 163–166 行、[mesh_library.rs](mesh_library.rs) 第 121–123 行；[backend/mod.rs](../systems/backend/mod.rs) 当前只声明四个子模块。

在 workspace 的 Rust 源码中检查队列访问和方法调用后：`take_pending`、`changed_paths`、TextureWatch 的变更集合只有定义/测试用途；`modified_meshes` 只有生产写入及测试断言，没有生产重建/清空逻辑。原注释中的 `systems::backend::{configwatch,meshwatch}` 与 `vao_update::rebuild_vaos` 当前没有对应实现或注册。

结果是即使文件事件成功入队，也没有现有运行链自动重载；CPU mesh 被修改后，已创建 VAO 不会自动刷新，修改列表也会持续积累。这属于模块外调度与消费者缺失，不是回调过滤实现错误。

建议：恢复并挂载实际后台消费与 VAO 更新系统，明确资产修改→重建→draw 的依赖顺序及失败重试策略。不要把原注释的消费者当作已接通。

验证：全 workspace 调用点和调度源码静态检索；未声称运行了真实文件监听端到端流程。

## 6. [P2 / 集成缺口] 瞬态组件 Drop 后没有运行期回收调用

位置：[transient_mesh.rs](transient_mesh.rs) 第 48–75 行。

Drop 只将 handle 入队，真正的 `delete_dropped_transient_meshes` 当前只有本文件测试调用，在 workspace 生产源码中没有调用/注册。移除或反复重建实体因此只增加删除队列，CPU mesh 与关联 VAO 会继续留在资产库里；显式调用清理函数本身的逻辑有单元测试，但运行期闭环缺失。

建议：在合适的 ECS 清理阶段注册实际消费者，确保实体移除后处理删除队列，并核对与 mesh 修改/VAO 使用的调度顺序。

验证：全 workspace 调用点检查；没有运行完整游戏来测量内存增长。

## 7. 保留合同与其他边界

以下行为已有明确的原作合同说明，本轮不按一般工程偏好判为待修复 bug：

- `MeshLibrary::remove` 不清路径表；对旧路径再次 reload 可能 panic。
- `get_mut` 在查找之前登记修改，加载失败保留旧 mesh 但仍登记变更。
- raster `force_new`、同名 mesh 覆盖名字后保留旧对象。
- texture 别名覆盖真实名字时先 insert 后 panic。
- bindless 读锁切写锁后不重查，以及注册错误、Mutex 中毒和缺失资源的 panic 策略。

上述行为若将来调整，应先核对原作合同与真实消费者，不能凭代码风格直接修改。

放置数据的 `{x, y}` 仍是明确的 schema 占位；`handle_of` 对未标注 repr 的 `ResourceHandle` 做布局假设，也值得后续消除。前者缺少本轮原 schema 证据，后者本轮没有证明当前编译器已产生错位，故不与六项已明确问题混作同等结论。

## 8. 验证记录

证据目录：[asset-libraries-review-2026-10-18](../../../../reports/asset-libraries-review-2026-10-18/)。

| 检查 | 结果与含义 |
|---|---|
| `cargo test -p country-core --lib asset_libraries:: --locked --offline` | 157 passed，0 failed，0 ignored；[unit-tests.log](../../../../reports/asset-libraries-review-2026-10-18/unit-tests.log) |
| 格式大小反例 | 遍历 25 项预设，预设 10 断言失败，确认现有错误 |
| mip 保留反例 | 预设 8 断言失败，实际 0、期望 5 |
| 数组层保留反例 | 预设 0x11 断言失败，实际 0、期望 3 |

三项反例源文件：[probe.rs](../../../../reports/asset-libraries-review-2026-10-18/probe.rs)；最终输出：[probe.log](../../../../reports/asset-libraries-review-2026-10-18/probe.log)。它们是用于确认尚未修复问题的失败断言，不能报告为通过。为避免向默认测试集加入已知失败项，临时 integration test 已从 `tests/` 移除，只保留证据副本。

首次探针错误使用了生产 `RenderGraph::default()`，两项导入在缺少后端处提前 panic；该次日志保留为 [probe-first-attempt.log](../../../../reports/asset-libraries-review-2026-10-18/probe-first-attempt.log)，不用于证明描述丢失。最终探针显式注入 `BufferGraphBackend` 后才执行到目标断言。

重跑反例：在 workspace 根目录将证据 `probe.rs` 临时复制到 `crates/country-core/tests/asset_libraries_review_probe_20261018.rs`，执行 `cargo test -p country-core --test asset_libraries_review_probe_20261018 --locked --offline`，完成后移除该临时文件。

本轮未执行 GPU、真实文件事件、完整游戏和原进程行为对照；没有修改依赖、锁文件、Rust 业务源码或恢复台账。

历史审查探针只对应当时源码。当前回归以资产库集成测试、真实 GPU 测试和最新重构报告为准；旧行号和失败探针不用于评价已修改接口。
