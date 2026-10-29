# M01 两种砖实例的 canonical ECS 与真实 GPU 验收

本轮仅补齐既有 Nani 架构中 SolidVertexColorBricks 与 GothicWindowBricks 的必要生产消费者，不新建 renderer，不改变 rhapsody 数值或实例存储算法。先实际读取 B `crates/country-core/src/render/nani/systems.rs`、`instance_data.rs` 和现有手工拆分 `instance_updates.rs`，保留已有代码目录。

## 原方法、组件与注册

| 原方法 | PDB 完整跨度 | 原 required Query | 实例 |
| --- | --- | --- | --- |
| `nani_update_solid_vertex_color_bricks@142076230` | 3084B，源码393行 | ActiveInstance、Transform、BrickColor | kind6 / 64B |
| `nani_update_gothic_window_bricks@142076e40` | 3794B，源码425行 | ActiveInstance、Transform、BrickColor、GothicWindowBrickDeformation | kind7 / 112B |

精确 PDB 参数、完整地址跨度身份及原RNE/分析EXE逐字节一致性保存在 `m01-bricks-nani-identity.json`。原方法完整反汇编保存在 `m01-bricks-nani-complete.asm`；fresh Ghidra 只识别2209/2880B，且有类型传播警告，不能以不完整 body 为原算法边界。

原组件位于 `country_core::components`。BrickColor 的原注册/clone 为 `14084d310/14084da00`，GothicWindowBrickDeformation 为 `14084cc10/14084d300`。本轮把已有 builder 内的本地 raw 类型替换为 canonical core 类型的 pub use，仍使用24B/60B原字段载荷；生成器和 Nani Query 现在共用同一个 TypeId，调用路径保持原有 builder 结构。

原两 updater 都先锁实际 subset，再遍历全部 Query，没有 Changed 过滤，不读取 ColorId 或 SemiTransparent。空 Query 仍取得 subset；原本没有每帧清空整个容量的循环。卸载由 maintain 的 removed-component 流和 dense condenser 完成，不能把“归零”误写为额外全量清缓冲。

原 set44 注册父链为 Backend `140942c80` → `1415f5310` chain_ignore_deferred → maintain `14206ba60` → 原20项 updater tuple `14206bc10` → flush `14206b8a0`。两砖 updater 在原 tuple 位于 WindowGlass 后、StoneFloor 前。现有 `add_mapped_nani_instance_systems` 保留同一 maintain、显式 ApplyDeferred、unordered updates、flush，已恢复的16个 leaf 保留原相对注册顺序；没有据此宣称原20项全部完成。

## 实现合同

`components::BrickColor([u8;24])` 与 `GothicWindowBrickDeformation([u8;60])` 仅做明确的小端 u32 投影。Nani 复用已存在的完整 `build_solid_vertex_color_bricks` / `build_gothic_window_bricks`，不重写数值算法。

生产维护 preflight 要求真实 Transform、相应 required 组件和唯一正确 marker。保留已有 Smoke12、Lantern13 marker，在第14项尾部使用嵌套二元组承载两砖 marker，适配 Bevy QueryData 的15项限制，不改变其它 leaf 的 marker 索引。实际 updater 复用 `write_mapped_update`、原 InstanceKey 和同一 typed subset，使用 bypass_change_detection 维修 dense hint，不制造原只读 ActiveInstance 的 Changed。

64B实例的 word14 是原 `(brick_word5 << 31) | brick_word1`，不擅自清理颜色/ID高位；112B实例 word15 读取原 byte20 的全部8位，不把它归一化为bool，也不读padding21..23。Gothic 60B变形的全部原字段保留，原内部清零字节也由既有完整 builder 写出。

## 原机器码差分与真实 ECS

`probe_m01_bricks_nani_native.py` / `probe_m01_bricks_nani_x86.c` 映射原RNE PE sections，在 Rosetta x86_64 执行原两方法完整 per-row 数值块：`1420767df..142076980`、`14207756e..142077820`；原 pack2f16 与 half helpers 也直接执行。只在输入寄存器和输出出口做 instrumentation；这不是执行整个原 Bevy Query。

`tests/fixtures/native/m01-bricks-nani.json` 保存512组原输出（每种256），包含四分量非零单位四元数、带符号零/负缩放、非零平移、half中点/subnormal/Inf/quiet与signaling NaN、原raw标识和flag字节、60B变形数据。probe输入/输出/可执行文件及fixture SHA见 `m01-bricks-nani-native.json`。

实际非空 World 测试从 canonical NaniBundle 与原fixture经生产 maintain → ApplyDeferred → updater，读取实际 typed buffer 的16/28个u32和sort key逐位比较；验证64/112B flush、数据与剔除dirty清除、只改参数的下一帧更新、无 key Changed、dense删除/交换/ID保持、最终实例count0、required组件失败和空Query要求实际subset。

- `cargo test -p country-core --lib bricks_ecs_tests --features native-physics --offline`：3通过，512组逐位相同。
- `cargo test -p country-core --lib render::nani::systems --features native-physics --offline`：32通过。
- `cargo check -p country-core --offline`：通过。

已有三window fullvisual测试补上逐实体canonical类型核对：builder的原BrickColor presence（byte20!=2）及完整24B、GothicWindowBrickDeformation完整60B，必须由core同一TypeId取回。五种subtype×三rank×正式/preview共30组真实生成→fullvisual→两Rapier表→差分→卸载通过（单个复合测试1通过；日志 `m01-bricks-nani-window-typeid-tests.log`），不以只手工spawn组件替代生成器输出合同。共享编译中其它owner的临时缺方法/旧曲线类型均待其修复后才计本项通过。

## 本机真实 GPU 验收状态

在现有 `crates/tiny-glade/tests/original_nani_gpu.rs` 尾部追加有界测试，复用真实 CoreRenderBridge、GraphicsDeviceResource、BufferHost、GpuWorld 与生产 flush，未改变旧 master/fine cull测试和生产bridge。诊断设备只在测试描述中添加 TRANSFER_SRC，读回使用真实 Vulkan copy、资源barrier、提交完成点与CPU读取，不使用CPU recording替代GPU。

实际物理设备为 Apple M3（vendor106b、device1a030209、driver28a2），MoltenVK Vulkan，无OS窗口/输入。两种kind各2实例，经16帧复杂四元数和原flag案例，实际数据16/28words及剔除平移逐字读回与原fixture一致；同一物理缓冲和宿主资源数量保持稳定。先删除每种第一个实体，dense修复后保留实例ID、实际GPU载荷正确；再删除全部，实际count为0。

最终严格验收通过（`m01-bricks-nani-gpu-tests.log`，1通过/0失败）。保持 bridge 和宿主仍然存活，四个data/cull raw Buffer 的Weak strong均为0；宿主 live buffers 精确6→2，保留其它两个实际缓冲；完成回收后retirement pending=0，idle_waits始终为0。未改rhapsody生产代码，没有 wait_idle、吞Timeout、扩大预算或跳过失败。

定位时保留失败的 v1/v2/v3/v6 日志。最初在最后一次despawn后直接检查完成值，四缓冲各strong1；完整取证确认原 `OwnedGpuWorldSubset::remove` 即使删除最后元素也执行 positions写回、dirty.add与data.copy（`gpu_world.rs:661..674`）。因此最后一次零实例flush仍接受四个UploadQueue::Job，测试漏了这批尚未提交job的宿主poll。完成值只覆盖已提交工作，不能证明CPU admission队列已空。最终测试补齐同一宿主的最后poll，并对四缓冲做真实Vulkan copy完成和逐字读回，再释放handle和检查Weak。保持该原dirty逻辑，不为通过测试修改算法或取消上传。

GPU测试中的required组件失败是有意catch_unwind case；日志会打印原preflight panic，但测试随后继续完成全部实际GPU验证。初始化由 RHAPSODY_REQUIRE_VULKAN=1 强制，后端不可用会失败，不能将SKIP算通过。

## 经验与界限

1. 存在数值 helper 不代表实际生成实体能被 Query 消费；必须统一组件TypeId、marker、required参数、维护者和flush。
2. 原raw flag/NaN/half的位语义要用原机器码对照，不能按字段名称猜bool、颜色范围或padding。
3. typed buffer 的CPU对照、真实GPU字节读回、最终像素输出是不同验收层，本轮分别记录，不混称完成。
4. GPU所有提交完成不等于CPU admission队列为空；检查原最后remove的dirty传播，再证明队列提交、GPU完成和owner释放三步，避免把尚未提交job误诊为rhapsody泄漏。

剩余边界：两个leaf的原数值→canonical ECS→真实GPU缓冲→dense卸载及物理owner释放，以及生成器30组真实World的canonical组件合同均已验收；整世界绘制像素、其它原20项leaf、历史恢复和OS真实输入由中央负责，不随两个leaf自动通过。
