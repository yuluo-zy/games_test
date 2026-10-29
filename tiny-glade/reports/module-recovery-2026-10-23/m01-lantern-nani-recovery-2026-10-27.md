# M01 灯笼 Nani 发光实例的实际 ECS 更新

本轮补齐已有 Nani 重构中的灯笼 leaf，不重做 renderer、rhapsody 或实例存储。组件和灯光 SSBO 维护由 rectangle owner 负责，Smoke leaf 由 hints owner 负责；本轮只修改现有 `render/nani/systems.rs` 的 Lantern 输入映射、更新函数和对应测试，以及 `instance_updates.rs` 的真实组件投影。生产仍用同一个 maintain、InstanceKey、子集缓冲和 flush。

## 原路径及注册合同

先实际检查 B 骨架 `skeleton/B/crates/country-core/src/render/nani/systems.rs`、`instance_data.rs` 和 `systems/backend/mod.rs`。当前 `instance_updates.rs` 是已经存在的手工拆分文件，保留该结构，不为重现 B 文件另造重复模块。

原 updater 为 `14207a4f0`，PDB procedure 记录 code_size=3474、源码行700。精确参数记录位于 B `systems.rs:16319`：

```text
Query<(&NaniActiveInstance, &Transform, &LanternLightParams,
       Option<&SemiTransparent>, Option<&ColorId>),
      With<InstanceDataMarker<LanternEmissiveInstanceData>>>, Res<GpuWorld>
```

原函数先锁 `GpuWorldSubsetKind::LanternEmissive`（原 `14207a777 mov dl,12h`），随后遍历全部符合条件的实体，没有 Changed 过滤；空 Query 同样先取得实际子集锁。颜色可缺省，不能因没有 ColorId 跳过整个实例。

原注册 `Backend::add_backend_systems@140942c80`：

- `140942e28` 调用 `1415f5310` 构造 chain_ignore_deferred；`140942e2d` 写 `2ch`，即 set44；`e49/e58/e9c` 完成 intern、in_set、add_systems。
- chain 内 `1415f5663→14206ba60` 为 maintain 配置，`1415f5684→14206bc10` 为原20项更新 tuple，`1415f5690→14206b8a0` 为 flush 配置，`1415f590c→1416a00a0` 使用 IgnoreDeferred。
- 本项目生产 `gameplay_startup.rs` 已调用 `add_mapped_nani_instance_systems(..., GameSystemSet(44))`，沿现有 maintain → 显式 ApplyDeferred → unordered updates → flush 接入。这里只新增已恢复的 Lantern 项，不把其余原20项也声明完成。

`m01-lantern-nani-complete.asm` 来自原 RNE 的完整 PDB 地址跨度。Ghidra 当前只识别2907B body，且数值传播有警告，不能用该反编译输出代替3474B完整指令；保留 fresh C/ASM 供比照，数值验证直接执行原机器码。

## 组件到 GPU 字段

canonical 类型为 `components::drawable::LanternLightParams([u8;36])`，与生成器、灯光维护共用 TypeId。Nani 只读取局部灯位 f32@12/16/20、强度 f32@24、原字节 flag@32。世界灯位@0、装饰ID@28和padding不参与该实例载荷；这些字段由其它消费者读取。

实例实际写入64B `LanternEmissiveInstanceData`：

| 字段 | 业务 |
| --- | --- |
| words0..11 | 原旋转/缩放3×3和真实平移 |
| word12 | 原 pack2f16(局部x, 局部y) |
| word13 | 原 pack2f16(局部z, 强度) |
| word14 | 原始 ColorId 或0，OR 半透明presence<<31，OR 原flag字节<<30 |
| word15 | 0 |

不把 flag 简化成 bool，不清理 ColorId 高位，也不增加窗口玻璃分支才有的颜色边界断言。实例 position 使用 Transform.translation；sort key 保持既有 `trs_packed` 的原 max/NaN 指令语义。

现代 ECS updater 为 `nani_update_lantern_emissive_ecs`。复用 `write_mapped_update` 直接写实际 owned subset；每个系统只锁一次，不构造逐实体 Row Vec 或第二套实例 key。使用既有 `bypass_change_detection` 维修稠密位置提示，避免制造原只读 ActiveInstance 的 Changed。`MappedNaniMarkers` 保留 Smoke 索引12，在尾部追加 Lantern 索引13，preflight 要求真实参数、Transform、唯一正确 marker；可选颜色仍保持可选。

## 原机器码数值对照与实际 World 验收

`probe_m01_lantern_nani_native.py` / `probe_m01_lantern_nani_x86.c` 将原 RNE 按 PE sections 映射，在 Rosetta x86_64 直接执行 `14207ad95..14207af29` 的完整共享数值块。原 `pack_2x16f_uint@140c90df0`、`half::f16::from_f32@1408b9f20` 及其真实依赖同样直接执行；没有替代数值库桥。宿主只提供输入寄存器和原 frame 所需常量，在数值块完成后跳到输出出口，不执行原 Bevy 容器遍历或 update_instance_data。

448组真实输出保存在 `tests/fixtures/native/m01-lantern-nani.json`，包括有效单位四元数、正负/零缩放、非零世界平移、半精度舍入中点、subnormal、Inf、quiet/signaling NaN、颜色高位、8种原flag字节和半透明presence。原始文件、probe、输入、输出、fixture及本机probe可执行文件SHA见 `m01-lantern-nani-native.json`。

正式 Rust 测试不是 LanternRow 纯 helper 自测：同一真实 Bevy World 创建非空 NaniBundle 和 canonical 参数，经生产 maintain、ApplyDeferred、真实 updater 后，读实际 `InstanceDataBuffer<LanternEmissiveInstanceData>` 的 typed shadow，16个u32与原448组逐位匹配，并检查实际 compact position、half sort key、数据/剔除脏区、flush 接口写出的64B范围和脏区清除。

其它实际 ECS case 覆盖：

- 只修改参数、不改 Transform，下一帧仍上传点亮位；颜色缺省得到0，未创造新 key。
- 删除中间实体后，稠密 swap/提示修复保留原 key ID与正确载荷；最终 despawn 后子集 instance_count=0。
- 缺参数、Transform、marker或重复不同 marker，在实例发布前失败；空 Query 仍要求已注册真实 subset。
- GPU World 的 previous_instance_count 是原本帧创建请求计数，不能改成常驻活实例数量。

执行：

- `CARGO_INCREMENTAL=0 cargo test -p country-core --lib lantern_ecs_tests --features native-physics --offline`：3通过，0失败。
- `CARGO_INCREMENTAL=0 cargo test -p country-core --lib render::nani::systems --features native-physics --offline`：29通过，0失败，0忽略。

测试中设备边界为现有 MockDevice；typed buffer 内容、dirty 和上传调用真实走生产实例代码，但不把它称为 Vulkan readback。完整 GPU 画面、rectangle 的生成/灯 SSBO 维护、正式资源与场景重载联合验收仍由中央完成。本轮未执行 OS 输入或新 GPU 场景验证。

## 经验与后续界限

1. 有 build helper 不代表实际 ECS 系统可达；要核真实组件 TypeId、marker preflight、生产 tuple、唯一维护者和实体卸载。
2. Ghidra body 长度和 PDB procedure 范围可以不相等；必须补原完整跨度，不能因 fresh decompile 缺数值写面就遗漏逻辑。
3. 不把可选颜色改为必填，不凭 UI 表现将原raw flag字节提前归一化；位级差分可检出这些看似合理的简化。
4. 数值块直接执行和整个原系统执行是不同证据；本轮明确保留入口/出口 instrumentation 与实际 ECS 对照边界。
5. 共享 Nani 文件并行修复时只做窄 patch；Smoke 与 Lantern 追加独立 marker，保留同一 maintain/flush，不扩散为两套 renderer。

剩余范围：该 leaf 已形成组件→实例载荷→typed buffer/dirty/上传→卸载的闭环；原20项Nani更新中其它未恢复项、独立灯照明SSBO和最终像素效果不随本轮自动通过。
