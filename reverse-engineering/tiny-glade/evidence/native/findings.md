# Tiny Glade 原作第一轮函数还原

研究二进制 SHA256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。所有地址是这个 Windows x64 映像的静态 VA。函数范围来自匹配的 PDB；反编译数据库为独立分析副本。

## 屋顶创建器

`system_roof::create_roof::create_roof` 位于 `0x1421aaea0`，PDB 记录 2075 字节。完整限定反汇编中的 26 个直接 call 站点已映射；初步伪代码见 [create_roof](tg_system_roof_create_roof_create_roof.c)，机器可读记录见 [slice-report.json](slice-report.json)。

当前可追溯的控制和数据流：

1. 遍历两段输入范围，记录步长为 `0x40`。输入中偏移 `+8` 的值被传入 `PublicWalls::get`；输入容器和事件具体类型尚未还原。
2. 两次墙查询返回值分别检查为空，缺失时进入 panic 路径。随后读取其中一个结果的 `+0x258` 字段，值大于 1 时进入另一个 panic 路径；字段的枚举语义尚未知。
3. 从墙结果读取若干字段，并调用 `PublicWallState::max_y`。随后调用 `Roof::new`、`Roof::ty` 和 `Roof::is_gable`。其中墙样式辅助函数地址有多个公开符号别名，不能仅凭首个别名定其语义。
4. `is_gable` 返回值控制一个输出集合的写入；该集合观察到 `0x10` 字节记录步长。之后调用 `Commands::spawn`。
5. 随后调用 `ShapeParameters::center`，并写入另外两组输出集合，观察到 `0x28`、`0x20` 字节记录步长。各输出集合的真实类型、事件用途和调度先后仍需验证。

这支持“创建器读取墙状态、构造屋顶状态并发布实体/后续变化”的结构解释，但不能据此宣称屋顶网格算法已经还原。几何核心应继续追踪 `Roof::new`、屋底生成、空间更新及其下游。

## 历史 mutation 收尾

`country_core::resources::history::History::finish_potential_mutation` 位于 `0x140a86980`，PDB 记录 420 字节。伪代码见 [finish_potential_mutation](tg_country_core_resources_history_History_finish_potential_mutation.c)。

- 第二个输入值为零时，观察到的正常路径直接返回。
- 非零时，读取第一个输入对象的 `+0x20` 指针、`+0x28` 数量，将 `+0x28` 写为零；沿 `0x10` 字节记录步长遍历，并执行原子引用计数递减，必要时调用 `Arc::drop_slow`。
- 调用 `std::thread::local::LocalKey::with` 后，把返回值写入对象 `+0x60`。
- 过程还包含 panic/unwind 清理。PDB 范围内存在加载器产生的辅助入口；限定分析在自有库中将其合并，具体入口记录在 slice-report。

目前不能把输入标志直接命名为“提交成功”，不能把被清理的集合直接认定为 redo 历史，也不能把 `+0x60` 认定为时间戳。下一步是对照 start_input_recording、mutation token 产生处与调用者，确认字段生命周期。

## 已通过差分验证的原函数

`WindowSize::aspect_ratio` 位于 `0x140949c20`，完整大小 20 字节。它读取输入指针偏移 0、4 的两个 32 位无符号值，分别转换为 f32，再相除；指令中没有零分母检查。

还原代码：[window_aspect.rs](../../reconstruction/math/window_aspect.rs)。验证：[window-aspect-verification.json](window-aspect-verification.json)。参照结果来自发行文件中真实机器码的 Unicorn 模拟，Rust 候选由本机 rustc 编译。256 组边界交叉输入加 1024 组固定种子随机输入，共 1280 组，结果的 32 位表示全部一致。

测试涵盖零值、零分母、典型尺寸、f32 整数精度边界、u32 高位和最大值；模拟条件为 MXCSR `0x1f80`。这里没有运行完整游戏。字段原名、完整 WindowSize 大小与其他浮点控制模式仍未知，结论只覆盖这个局部数值函数。

## 下一步

优先定位 `Roof::new` 的真实字段与类型分支，并联接原作历史样本中的 `CreateRoof`、`VisualEdit`、`SwitchRoofRidgeDir`；同时还原 HistoryMutationToken 与历史容器生命周期。完成这两条数据链后，再进入实际屋顶几何和跨对象重构验证。
