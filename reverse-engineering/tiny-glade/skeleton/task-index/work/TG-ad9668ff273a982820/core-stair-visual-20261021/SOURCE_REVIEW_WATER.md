# Stairs visual lazy flag 独立源码复核

复核者：water_activity_chain（不是候选作者）。结论：**通过本次窄范围源码复核和零 fuzz 补丁复现**。此结论只覆盖同一 edge owner 的 request byte/getter、existing-only mark、reset 清零及真实缓冲影子测试；没有执行 Cargo/rustc/GPU，不构成完整楼梯视觉恢复验收。

## 绑定交付

- 源：`tiny-glade/crates/country-core/src/resources/stairs/visual.rs`
- before SHA-256：`d8e8e0da1f5db76dabd715743be6a588f9149f68bfd930b879ea8d4d7ceca7d2`
- candidate SHA-256：`aa5e84f63204e496991be005ae6a312a80f3ade509a3135322bdef6e96449bec`
- patch SHA-256：`16ff174904f8050e4e8929bbd2fe8a0089d60f70627a6811df7403d9be646dec`
- 相对本目录交付：`before-frozen-20261021-lazy-flag/delivery.json`、同目录 `candidate.patch` 与完整 candidate。
- 合同独审：`CONTRACT_REVIEW_DUCK.md`，本次另外逐项读取候选全文/patch、实际 SparseInstanceBuffer/MeshAtlasAllocation API、原 store inline 包与 reset 汇编。

## 原证与源码对照

1. 原 `force_rebuild` 的 store 位于 `140b379c3`、`140b37c10`、`140b37df0`，三条均为六字节写 byte 1，最内 PDB 帧为 `StairsVisualState::force_rebuild`，精确 source Line134。原 producer 表探测命中键才写，miss 跳过；bucket end-8 对应 value+0x100。候选只在 `edges.get_mut(&edge)` 返回 Some 时写现有私有 bool；没有插入、reset、几何删除、allocation 创建或全局 dirty 写入。getter 读取同一字段，没有第二权威。
2. `force_rebuild_with` 保留现宿主兼容形参，仅转发 mark；`let _ = (first, second, magic)` 丢弃的是借用，不会释放其 owner。原三个 buffer/magic owner 不被调用。这消除了先前把 force 等同 reset 的副作用。
3. 原 reset 完整 body 为 `140aa8a60/2322B`。已读第一 writer clear `140aa8c1b`、magic clear `140aa8c31`、第二 writer clear `140aa8ce0`，随后包含原实体/cache 清理；`140aa8e26` 与 fresh 分支 `140aa9067` 写 word0，清 +0x100/+0x101。候选在现有两个 writer/magic 清理之后清 request，Default/fresh/reset_edge 也为 false。只恢复已证 +0x100，不凭相邻字节创建第二业务字段。现 reset 未恢复的实体/cache 部分仍未恢复，不能由本 hunk 的清零宣称补齐。
4. 现 `reset_edge`/`reset_edge_with` 的已有 global dirty 行为和 remove owner 行为没有被这次窄修法扩大或删除。`force_rebuild` 对 dirty 初始 true/false 都保持原值；missing 仍不创建 entry。没有发现本次候选范围内的阻断问题。

原证均复用本 work 的 `flag-stores-inline-pack/force-store-{1,2,3}.json`、`native-pack/reset-edge-full.asm.txt`，以及合同已绑定的完整 producer；没有新做全程序采集。原身份为 RNE SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，PDB SHA-256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。

## 测试源码有效性检查

- 测试走现有 `SparseInstanceBuffer<Elem>` CPU 影子、真实 `MeshAtlasAllocation::allocation()` 指针和 `writer_for().push`，没有自己建 allocation 镜像。设备测试替身只供原有 buffer 构造，不是 GPU 渲染验证。
- 数据、byte_map、active/dirty 原子位图、free chunks、waste len 全部从真实对象快照；两 slot allocation 指针及 magic 两 slot/三个计数均有核对。以 global dirty 两种初值执行 plain/adapter force；已有写入 71/72 保持。missing 测试还核对不新建 magic slot、无表项和无缓冲变化。
- reset 测试在 request 为 true 后执行真实 reset，验证清零及已写实例释放；plain storage reset 验证 Default false；remove 验证同 owner 消失。源码接口/类型与当前 SparseInstanceBuffer 暴露字段一致，`Elem` 已有 `PartialEq`，快照比较有意义。
- 这些测试未在本审查中运行；后续由 root 在授权的统一构建中执行。该测试不证明尚未恢复的 RT entity/cache 生命周期。

## 独立补丁复现

详见 `PATCH_REPRO_WATER.json`：从实际 production before 复制到独立 `/private/tmp/tg-lazy-flag-source-review-water-2etjtynx`，执行 `patch -N -f -F0 -p1 -i <交付绝对路径>`，返回码 0，无 stderr/fuzz；产物 SHA-256 为上述 after，逐字节等于候选。复现前后 production SHA-256 均为上述 before，未写生产源。

## 保留的验收边界

外部 invalidate 的三个邻接 mark 调用、RT/实体/cache 删除和重建、共享 laziness global、完整 request/cache/compute/interpolate 链仍须其唯一 writer 和 root 后续整合。该候选提供真实 consumer 可读取的同 owner flag，不能单独声称整条楼梯 lazy 路径或游戏主链已完成。
