# Tiny Glade 项目记忆

## 用户已确定的目标与边界

- 原作研究路径：`D:/game/ljxsj_92385/Tiny Glade`。分析目录：`D:/game/reverse-engineering/tiny-glade`。Rust工程：`reconstruction/Cargo.toml`。
- 最终交付完整 Rust 游戏实现；当前先恢复源码、类型、状态与 CPU 算法。场景回放和 GPU 研究后置。
- 不分析用户自己的 demo。不得把已有通用 Rust 依赖也当作游戏算法重新逆向/手写。
- 引擎为 **Bevy 的分叉**：这是用户明确提供的项目事实。随包 Cargo.lock/PDB 独立确认 `bevy_app/bevy_ecs/bevy_tasks` 等为 0.16.0；分叉代码仓库、具体commit及补丁范围尚未核实。不能据此猜测完整渲染等同上游 Bevy。

## 原作依赖基线：直接复用

唯一基线是原作 `build-info/Cargo.lock`，SHA256 `9e0cb4a0fa173122fa7cd552aa639f5ee977abc22c74aec92398808c707184bd`，901 个 package（多版本分别记录）。不能用最新版本覆盖这份基线。

| 依赖 | 原作使用版本/来源 | 当前处理 |
|---|---|---|
| Bevy family | 0.16.0；引擎分叉按用户说明 | 复用；只研究游戏调用及必要分叉改动 |
| glam | 游戏core/roof/utils用0.29.3；锁文件另有0.20.5/0.25.0/0.27.0 | 复用0.29.3数学API，不还原glam实现 |
| fastrand | 游戏core/roof/utils用1.8.0；另有2.3.0 | 复用1.8.0，不能误选2.3.0 |
| half | 2.4.1 | 调用f16转换，不手写转换器 |
| serde / serde_json | 1.0.200 / 1.0.85 | 复用；逆向的是游戏结构/格式和消费者 |
| ron | PounceLight/ron 0.8.1，commit `2787d8c05250c4dd706dfb358a2e9f16969d2968` | 复用原fork，不重写RON解析 |
| rkyv | PounceLight/rkyv 0.7.39，commit `031d58108ccf8c0e125b06b017f72b4afd67e9a3` | 复用原fork，恢复游戏存档schema |
| winit | PounceLight/winit 0.29.15，commit `d03de0a0a881be9422c360083151b0e7a8a2582e` | 复用原fork，窗口/平台研究后置 |
| slotmap / smallvec | 1.0.6 / 1.13.2 | 复用容器；恢复游戏ID和字段语义 |
| ash | 0.38.0+1.3.281 | 复用Vulkan接口；GPU后置 |

全部 package及来源应保存到 `tracking/dependency-policy.json` 和 `tracking/original-Cargo.lock`；原锁文件用于比对，不直接替换重建工程自己的锁文件。

## 排除范围与待归属范围

1. crates.io、公开git依赖与Rust标准库：读取公开源码/文档并使用匹配API，不反编译还原其实现。
2. 无source字段的52个path/local条目不能一律当作游戏独有代码。FidelityFX、XeGTAO、驱动/分配器等先确认已有上游；不要因为路径是local就重新实现。
3. `country-core`、`system-roof`、`system-wall-constructor`、`system-decorator` 等游戏模块为主逆向对象。游戏自有 `utils::CurveU/random_splits` 可以恢复；其中调用的glam、RNG、f16仍复用依赖。
4. 反编译器展开的Vec/Arc/ECS/Serde泛型胶水仅用于理解游戏数据与调用边界，不能计作游戏源码恢复成果。

## 已完成与证据界限

屋顶曲面/Curve2、脊线、圆形与矩形CPU铺瓦源码已存在。此前临时手写的随机数、f16转换、四元数/向量运算已改为原版本库调用，重建工程6个依赖包的版本和checksum与原锁文件一致。替换后重新验证：矩形33输入/19024条记录、seed和最终RNG一致；圆形37输入/11467条记录逐字节一致。具体见 `evidence/roof-tiles/dependency-boundary.json` 和 `dependency-verification.json`。

新增墙高约束数值后段与3个完整高度缓存叶函数，通过6624个机器码案例。约束前段的对象查询、特殊flag生产者仍待恢复，不能把局部数值规则记成完整墙系统。

新增 `roof/edit_rules.rs`：已解析Roof/Event的编辑后段、变化量、change tick、gable重建和数值反馈，通过4514个原指令案例；tip长度限制复用glam公开API。原ECS解析/调度/失败/队列扩容未恢复，不能记成完整屋顶编辑系统。记住height Absolute下限0、Delta下限0.1；eave/ridge/tip Absolute保留raw，Delta才限制；变化量不是实际最终差值。细节和边界见 `evidence/source-edits/findings.md`。

发行树/PDB未取回作者原Rust文本；当前是原机器码约束下的行为恢复源码。不能声称获得作者原始源代码、完整ABI或整个游戏完成。

## 工具与推进方式

- 使用 `reverse-engineer-anything` skill 的原作证据流程。
- REA、ReVa、RE-MCP已安装；会话没有直接暴露其工具时，通过官方MCP SDK连接已配置stdio服务。连接、工具catalog和实际调用结果须落证据，不把doctor成功等同实际连接。
- 复用持久Ghidra数据库；按原作函数/引用/类型检索，不全程序重复分析十万条符号。
- 从高收益游戏规则和权威状态链推进：Roof编辑消费者/字段映射 → Wall形状/高度/生成 → 历史提交/撤销消费者 → 门窗附着等。
- 里程碑与源码验收分开记；依赖复用排除在逆向覆盖统计之外。
