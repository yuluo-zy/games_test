# 相机与小生物并行逆向：停止状态与推送交接

停止原因：用户要求“全部停止并推送当前状态”。
快照时间（UTC）：2026-10-11T09:39:52.792963+00:00。
状态：本会话相机、小生物两个 agent 均已通过 TaskStop 成功停止；未重启，未继续逆向或重新运行测试。停止后进程快照未发现标识为两个专题的残留验证进程；这不是全机无其他研究进程的保证。

## 结论

**两个模块都未完成系统验收。当前提交用于保存中断现场，不表示模块实现完成或验证全通过。**

本次仅提交本会话调查、两个专题的 owned 文件和本停止快照。工作区还有其他会话正在变化的建筑/地形/共享工程文件，保留但不纳入本次提交。先前已有专题材料已存在于父提交，本次提交保存停止时新增/修改的差量。

## P01 相机

已落盘：

- 三版符号库存、38 个业务候选的只读反编译材料，以及原 RNE 范围/跳表取证。
- 独立 Rust 工程 `reconstruction/camera/Cargo.toml`；源码 `schema.rs`、`camera_body.rs`、`normal.rs`、`free.rs`、`walk.rs`、`butterfly.rs` 与 `mod.rs`。
- 包含模式/schema、镜头/矩阵、普通相机平移/旋转/缩放，以及自由、行走、蝴蝶模式的局部行为恢复代码。
- `evidence/parallel-camera/B-constructor-native.json` 记录原 MainCamera constructor 在带 allocation/memory/hostlibm bridges 的 oracle 中返回。**原 constructor 成功运行不等于 Rust constructor 等价或相机系统验收。**

重要证据限制：既有 B DB 对部分跳表函数的边界不准，反编译可能跨入其他函数。需要原指令实际可达控制流和跳表验证，不能以导出成功计为 analyzed。PE unwind/PDB 范围同样不是完整逻辑函数的单独证明。

未完成/未验收：完整 MainCamera 编排、输入与模式切换/插值链、真实 terrain/collision producer 联接、全相机原码差分、C 适配、视觉/系统验收。相机编译/测试结果本次未重跑，未查得完整验收报告；当前源码必须视为停止时的未验收实现。

依赖与接口记录：[相机 integration](evidence/parallel-camera/integration.md)。B game glam0.29.3 与 dolly0.6.0 内部 glam0.27.0 分开使用。

## P02 小生物

已落盘：

- 三版 24 个 critters 私有源文件库存，B public 函数反编译及内部模块资料。
- `reconstruction/critters/firefly.rs`：B 萤火虫目标、velocity target 与平滑速度的语义适配实现；不是原 ECS ABI 或完整生成/销毁系统。
- 独立 `reconstruction/critters/harness/`、probe 与 `scripts/parallel-critters/verify_firefly.py`。
- 原版 `perchance=0.4.0` RNG 复用；目标重选严格三次 uniform_f32，比较后续随机值 fingerprint，未手写 RNG。

### 已保存原码验证结果（整体未通过）

以实际落盘 JSON 为准：[B-firefly-verification.json](evidence/parallel-critters/B-firefly-verification.json)。

| 范围 | 案例数 |
|---|---:|
| goal | 780 |
| velocity | 786 |
| smooth | 536 |
| 合计 | 2102 |

报告 `ok=false`，有 **1 个失败案例**（同一 smooth 案例的两个分量 NaN 符号位不同）：原始结果 `ffc00000 ffc00000 7f800000`，Rust `7fc00000 7fc00000 7f800000`。失败保留，不能称逐 bit 全通过。其余案例未记录失败。host libc/Windows CRT 差异、采样输入域、异常标志未覆盖等限制见原报告。

agent 报告 `cargo test` 2/2 通过；未找到持久 test log，本会话停止后未独立重跑，因此作为 agent 报告而非新增复验记录。

报告记录的六个源码/配置/验证脚本 SHA256 与停止时文件比对：**全部一致**。完整比对见机器快照。

[B-boundaries.json](evidence/parallel-critters/B-boundaries.json)：55 个 PDB-owned 范围全部 original/patched 字节一致；3 个入口存在范围外直接 jump，7 个入口含 indirect jump（共11条 indirect jump 指令）。该一致性仅证明所取范围的字节，不证明完整 CFG/归属或反编译可信。

未完成：butterfly/duck/fish/wren/sheep 等其他小生物完整业务链、真实 Bevy App 与生命周期联接、terrain/water/building 查询 producer、删除/replay 接口、C 适配、GPU/视觉与系统验收。生成/build caller 的取证不等于完整 Rust startup/调度已实现。

依赖/共享接口：[小生物 integration](evidence/parallel-critters/integration.md)，其 preliminary 内容不能替代后续实际证据。以源码、原码报告和此停止快照记录最新边界。

## 工具与耗时背景

早期默认 agent API400、后续 HTTP502 与自动权限检查器异常造成中断。既有 DB 已通过只读 DomainObject 取证；未写共享 DB。函数边界问题、Rust泛型/调度/ABI、原作依赖复用与原码 oracle 也产生真实研究成本。无论原因，未完成项不能记为完成。

## 接续方式

用户本次已要求停止，不自动接续。未来明确恢复后，先检查最新分支、此快照、report源hash及两份 integration；不要重跑已保存库存，不把旧DB反编译成功当可信业务。优先形成原输入→状态→实际消费者的可验证链，再扩展剩余范围。保持各专题 owned 路径，共享 workspace/类型需集中协调。

机器快照：[parallel-modules-stopped-snapshot.json](tracking/parallel-modules-stopped-snapshot.json)，包含文件清单/hash、验证结果和未完成项。快照不包含 target、__pycache__ 或原发行二进制。
