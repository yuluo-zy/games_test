# 已锁定的下一组游戏自有源码入口

只研究发行文件内 `country_core` / `system_roof` 的游戏规则。原 Cargo.lock 提供依赖版本；std、Bevy、serde、glam、fastrand、half 使用已有库/API，不继续反推其实现。用户说明原作为 Bevy 分叉；分叉提交及补丁集合仍未知，PDB 的 `bevy_ecs-0.16.0` 路径不足以推出它们。

这次实际使用 RE-MCP stdio SDK 打开原 RNE 的独立复制件，关闭全局 auto-analysis，读取数据库信息与两个目标函数，然后保存并关闭会话。记录在 [mcp-read.json](mcp-read.json)；原目录未写入。边界另外用原 PDB S_GPROC32 与 RNE 指令验证，避免把 Ghidra 猜测的签名当成真实 Rust ABI。

| 优先级 | 明确函数 | 原作范围（结束地址不含） | 原始源码候选路径 | 具体产出 |
| --- | --- | --- | --- | --- |
| P0 | clamp_wall_height | `0x140ac7f30..0x140ac80d6`，422 B | `country-core/src/systems/wall/adjust_wall_height/ui_adjust_wall_height_mode.rs` | 已还原数值高度限制；接下来确认特殊标志的生产者与 wall/roof 查询前置条件 |
| P0 | system_roof::edit_roof::edit_roof | `0x1421ab7c0..0x1421ac427`，3175 B | `systems/roof/src/edit_roof.rs` | 明确 EditRoofCmd 的 None/Set/Add 编码、各字段 clamp 顺序及屋顶类型/山墙变化触发器 |
| P1 | history_record_adjust_wall_height | `0x140ac79e0..0x140ac7f25`，1349 B | 同上述 wall-height UI 文件 | 连接实际高度编辑和 `new_height_changed → History::add`，确定 before/after 与输入记录分组 |
| P1 | system_roof::replay_roof::replay_roof | `0x141a37ad0..0x141a37fcd`，1277 B | `systems/roof/src/replay_roof.rs` | 恢复 Roof 历史变体到 Create/Delete/Edit/Switch/RimStyle 游戏命令的字段复制；使用 Bevy EventWriter API |
| P1 | PublicWallState::flat_roof_y | `0x140b2a6e0..0x140b2a73a`，90 B | `country-core/src/resources/walls/public_wall_state.rs` | 完整叶函数已还原；继续找 max_y / max_y_gable 缓存的发布生产者，接通墙和屋顶高度 |

完整源码原路径、文件 SHA256、PDB module/记录行和边界在 [high-yield-targets.json](high-yield-targets.json)。源码归属来自命名空间和同 module 源文件记录，尚不是已恢复的源码正文或精确源码行定位。

## 有证据的调用关系

- `clamp_wall_height` 在 `0x140ac803b` 调用游戏的 `PublicWalls::roof_component`（`0x140a923a0`），随后按已解析的 optional Roof 与特殊标志确定高度上下限。
- `history_record_adjust_wall_height` 在 `0x140ac7b8d` / `0x140ac7cdf` 读取 `PublicWalls::roof_component`；`0x140ac7d6d` 调用游戏 `EditType::new_height_changed`（`0x140a83fb0`）；`0x140ac7d7f` 调用游戏 `History::add`（`0x140a84ec0`）。
- `edit_roof` 在 `0x1421aba5b` 解析 `PublicWalls::roof_entity`（`0x140a922b0`）；三个直接调用 `Roof::is_gable` 的位置为 `0x1421abd97` / `0x1421abddb` / `0x1421abe30`；随后使用 `Roof::pivot_3d` 生成相关游戏消息数据。ECS 对象布局、事件通道的命名仍要从游戏注册代码对齐，而不是反推 Bevy。
- `replay_roof` 的 PDB 游戏系统签名携带 CreateRoofCmd、DeleteRoofCmd、EditRoofCmd、SwitchRoofRidgeDirCmd 与 SetFlatRoofRimStyleCmd 的 EventWriter。应恢复匹配分支和字段复制，写事件的方法留给 Bevy。

所有直接 CALL 的地址在 [call-relationships.json](call-relationships.json)。未出现在选择的游戏符号索引中的目标标为 `outside_selected_game_index`，不自动宣称它是第三方依赖。

## 本轮已新增游戏规则源码

[height_rules.rs](../../reconstruction/wall/height_rules.rs) 已可编译，包含如下原作规则：

| 已解析状态 | 最小高度 | 最大高度 |
| --- | --- | --- |
| 有 Roof，height_01 > 0.1（NaN 的原分支也在这里） | f32 0.7 | 常规 14.0 / 特殊标志非零 2.5 |
| 有 Roof，height_01 <= 0.1 | f32 1.6 | 同上 |
| 无 Roof，特殊标志为零 | 0.5 | 14.0 |
| 无 Roof，特殊标志非零 | f32 0.3 | 2.5 |

`max_y` 从 `PublicWallState +0x108` 的 presence bit 和 `+0x10c` 数值读取；`max_y_gable` 对应 `+0x110/+0x114`；cache 缺失时原作 panic。完整 `flat_roof_y` 返回 cached max_y 加 f32 `-0.7900000214576721`。这些物理偏移与函数命名已得到机器码证据，生产者的完整字段类型尚未恢复。

[wall-rules-verification.json](wall-rules-verification.json)：6624 个原机器码差分案例，0 差异，其中 resolved clamp policy 4680、三个完整叶函数各 648。覆盖阈值相邻值、符号零、次正规值、正负无穷、NaN、cache bit 与非零特殊标志。clamp 的数值后半段从 `0x140ac8040` 开始执行，查询/hash 前段明确不在这项验证中，不把局部规则冒充完整 clamp 函数。

`special_flag` 的确切游戏字段名称未知。源码仍保留此观察名称，不能提前命名成围栏/半木/模式等未经生产者证实的语义。

## 依赖边界收尾

矩形面瓦原先的手工四元数旋转、cross 已替换为实际 `glam 0.29.3` 的 `Quat * Vec3A` / `Vec3A::cross`；脊线旋转使用实际 `Mat2 * Vec2`。面瓦 29 个原机器码向量共 11591 条实例仍完全一致，[glam-faces-regression.json](glam-faces-regression.json) 为证据。脊线重新执行 14776 个原机器码案例仍 0 差异。公共 helper 同样使用锁定版本的 fastrand/half/glam，不保留第三方算法的手写替身。

复现新增规则：使用 `workspace/python/Scripts/python.exe` 运行 `scripts/scope-priorities/verify-wall-rules.py`。
