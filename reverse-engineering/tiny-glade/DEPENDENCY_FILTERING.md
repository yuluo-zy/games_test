# 三版本依赖排除与建筑业务恢复指导

本指导按用户 2026-10-11 的要求执行。先核对 A/B/C 依赖来源，再选择游戏业务函数。当前重点是完整建筑系统：墙体权威状态 → 屋顶生命周期 → 墙/砖/支撑/开口 → 门窗与附着 → 历史事务及更新传播。场景和 GPU 继续后置。

## 每个实现入口必须先过归属检查

2026-10-13 用户纠正宏派生方法的进度口径：Default/fmt/Debug/Clone/Eq/Hash、serde生成visitor及Bevy Component执行胶水不逐个列为“尚未实现业务”。确认后用derive/原库；自定义字段默认值、枚举、格式属性、required-components及错误模板按类型/schema合同合并记录。仅凭名字不能确认宏生成，手写默认状态或包含业务校验的trait仍保留；未知归属另列待分类，不混入已确认业务缺口。共享基础609原记录保持不变，当前分类见[INTERFACE_CLASSIFICATION.md](INTERFACE_CLASSIFICATION.md)。

1. 对照各版 Cargo.lock、PDB 源路径/校验和、函数定义所属模块和调用点。A 未提供锁文件，用 PDB 中 registry/git/rustc 路径确认依赖来源；缺失版本不从 B/C 填造。
2. 已确认 registry/git 库或 Rust 标准库实现，标记 `dependency_reuse`，直接复用原版本、commit 和所需 API，不生成它的 Rust 替代实现。游戏包调用该库的策略、参数、组合顺序仍需恢复。
3. local/path 包只表示构建来源。已知 SDK、引擎、渲染器、分配器仍是依赖（索引明确排除已确认的 FidelityFX、XeGTAO、GPU-Raytracer、gpu-allocator、vk-sync-fork）；私有仓库路径也不自动证明是游戏业务。先确认模块归属和目标功能。
4. `country-core` 的 walls/roofs/decorators、建筑系统及自有 utils 是业务候选。确认游戏私有源路径与控制流后，恢复其规则；不能因它们也出现在 Cargo 中而全部排除。
5. 查看函数的**定义所属模块**。Vec/Arc/serde/Bevy 泛型实例即使名字含 Roof/Wall，也不是游戏实现。派生 schema 的字段/枚举映射属于游戏格式；serde 编解码器本身属于依赖。
6. 反编译器可能把共享代码/别名误标成其他泛型函数。用实际调用、指令和类型复核，不能把自动命名当作源码归属证据。

## 三版对比的用法与限制

- 同源库的版本/commit、PDB registry/git 路径和定义模块可用于排除库函数。不同版本地址变化、编译器差异及内联造成的机器码变化，不能据此把库函数重新认定为游戏规则。
- 游戏私有源文件在三版相同，不代表它是库。现有例子：B/C 的 `country-core/src/systems/wall/valid_enclosure/mod.rs` 源校验和相同，仍是建筑业务。
- 去掉 Rust hash 的同名函数仅是映射候选；需要比较类型、控制流、常量和业务调用后才能复用结论。B 验证不能替代 A/C 行为验证。
- Bevy 分叉保留用户已提供的约束，只研究影响游戏调用的必要差异；不重写 ECS 查询、实体表、事件容器、scheduler 或 Commands。具体分叉未知时登记接口缺口，不补造底层引擎。

## 当前建筑链的明确边界

| 项目 | 处理 |
|---|---|
| glam/fastrand/half/serde/slotmap/hashbrown/Bevy/std | 复用依赖；不写替代向量、RNG、序列化器、哈希表或 ECS 内存布局 |
| PublicWalls::get/roof_entity | 恢复游戏的缺失对象处理和关联语义；内部 map/query 用依赖 API，排除展开的库实现 |
| update_roof_spatial | 恢复墙高、形状、样式到屋顶及附属组件的更新、重建通知和触发条件 |
| create/edit/delete/switch/Flat 屋顶 | 恢复游戏参数、初始化、规则、对象关系和事件；依赖分配与容器胶水排除 |
| 墙轮廓、砖列、支撑、开口、门窗 | 沿权威输入/状态/实例输出恢复完整业务链，而非持续堆积孤立数学叶函数 |

每批交付必须包含业务 Rust 模块、原作归属/调用证据、与已有模块的联接和相应原指令验证。依赖分类、脚本数量及反编译数量不计为建筑系统完成度。确认的依赖不再进入后续核心实现待办。

机器可读索引：[dependency-exclusions.json](tracking/dependency-exclusions.json)。来源：三版 `evidence/multi-version/*-dependencies.json` 与 `*-sources.json`。该索引区分包来源、PDB 路径和业务候选，不宣称完整函数地址归属已解决。

## 实体联接与数值边界

新增系统用真实Bevy的Component/Resource/Event/Commands/ApplyDeferred；原调用的传参、筛选、事件顺序属于游戏恢复，Bevy注册容器/调度器实现仍排除。显式重建的合理依赖顺序必须和已经取证的原作调度图分开登记。一个共享地址有多个PDB别名时，以调用点和业务值确认，不能直接实现Ghidra任选的库标签。

跨SSE/NEON或host CRT差异保留失败报告，先定位边界；库版本相同不保证跨架构每位一致，不以依赖源码大规模重写来补差。三版私有墙/屋顶算法共享仍应恢复，而业务投影/字段变更不能靠依赖版本等同推断。
