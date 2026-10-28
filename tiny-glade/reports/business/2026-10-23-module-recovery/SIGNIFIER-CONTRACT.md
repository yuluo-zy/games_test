# 当前 toolbar 的 signifier 消费接口

本文件记录原证裁决和实现完成条件，不代表当前模块已经验收通过。

## 权威所有者与顺序

原版 `UiState` 捕获的 `Arc<WidgetParams>` 拥有实际
`Vec<SignifierFactory>`。全局流只传递有名字和回调所有权的工厂；create
在 UI 活动时把请求克隆进入捕获参数，随后唤醒实际工厂。追加必须通过
`Arc::get_mut` 的唯一所有者检查，失败保留原 panic，不能隐式 COW。
UI 不活动时原 create 不消费流。

标识是标准 DefaultHasher 对 `(WidgetId, Option<&str>)` 的哈希。
`SignifierId` 同时保存哈希和完整 WidgetId。`None` 和 `Some("")`
必须保持区别。已有实体哈希是 create 执行开始的不可变快照；同帧同名请求
可以重复唤醒，不能用新建实体更新快照来去重。

mark 从捕获参数里的真实工厂列表重建活跃集合，不能另造流内 live 表。
寿命条件为 `!(minimum > elapsed)`，包括原 UCOMISS 的 NaN 行为。
despawn 仅删除无动画或动画 `t == 0` 的已标记实体，不能省略退出动画。

原 set6 顺序：hover → debug volumes → debug roof UVs → ui_update →
create → mark → animation_t → animation_transform → despawn，
`chain_ignore_deferred`。本阶段只接当前 toolbar 必需的已恢复消费者，
不以 `.chain()` 提前刷新命令队列替代原顺序。

动画的 scale 门和 parent-origin 门彼此独立。parent-origin 只在标记存在、
完整 WidgetId 匹配当前活动 UI 时运行：Sphere 取 center，SphereCloud
取真实交点索引对应的 center（保留越界失败），PlaneDisc 取 point，其他
原 volume 类型保持 translation。现有核心渲染继续沿用。

## 后序模块的阻断接口

现有 `UiAppendSignifier::Raw` 是早期数字载荷适配，不是原版有名字与闭包的
完整工厂记录。不得将其 FNV 哈希、生成只有 SignifierId 的空实体，或把数字
tag 猜成可视工厂。原证台账：
`reverse-engineering/tiny-glade/evidence/round12/signifier-create-full/unrestored-raw-producer-inventory.json`。
独立原证审计另将实际生产调用归入 15 个后序源文件；旧台账还包含适配器和测试。

后序 terrain、wall、stairs、decorator、clutter 等模块每个生产点进入其恢复阶段时：

1. 依据原证恢复完整工厂名字、捕获参数、网格/线、shader、动画、pass 和真实回调。
2. 通过 `SignifierFactory::from_closure`、`WidgetParams::with_signifiers` 或真实
   `UiSignifierStream::write_factory` 接入同一个所有者；禁止并行假资源。
3. 删除旧数字 Raw 请求；实体身份统一由 create 在工厂返回后附加，构建器不伪造 ID。
4. 核验真实输入、实体/几何/渲染同步、相同标识复用、取消/退出动画、瞬态网格释放、
   会话卸载和再次进入；不得以工厂单测替代消费者链。

这些后序未恢复生产点应保留明确失败边界，不能因此宣称后序模块已可运行。
toolbar 已核实使用 Replay deny 的真实 Factory，不依赖这些 Raw 生产点。

## 原证范围

原体有界反汇编与哈希：
`../business-module-recovery-2026-10-23-toolbar/signifier-bodies.json` 及对应 asm。
create/mark/hash 原证与标准库哈希 oracle：
`reverse-engineering/tiny-glade/evidence/round12/signifier-create-full/REPORT.md`。
原几何跳表只含 8 个合法 volume 类型；旧反编译所谓 8..16 分支属于损坏输出，
不得按损坏伪 C 补算法。宿主 OnExit 会话清理属于工程修复，与原算法差分分开记录。
