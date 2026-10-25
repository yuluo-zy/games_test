# edit_roof 游戏参数规则源码恢复

[edit_rules.rs](../../reconstruction/roof/edit_rules.rs) 已实现可运行的 **resolved Roof/Event 编辑核心**。复用父任务已经保留的真实 RE-MCP 反编译结果 `scope-priorities/mcp-read.json` 与 `mcp_0x1421ab7c0.c`，逐分支核对原 ASM；不重复全程序分析，不重写 Bevy、HashMap 或 glam。

原过程 `system_roof::edit_roof::edit_roof`：`0x1421ab7c0`，匹配 PDB 过程 3,175 字节。新模块对应实体/组件解析成功后的 `0x1421abb2c` 后缀。原作 SHA-256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。

## 输入与赋值规则

Event 的 64 字节观察接口：profile mode/value `0/4`，height `8/12`，ridge `16/20`，eave `24/28`，wall ID 的 64 位载荷 `32`，tip mode `40`、Vec2 `44/48`，附带原标志字节 `52`。合法模式为 0 不变、1 绝对、2 增量。

| 编辑字段 | Roof 偏移 | 绝对模式写入 | 增量模式写入 | 用于触发的变化量 |
|---|---|---|---|---|
| profile | `0x2c` | clamp(raw,0,1) | clamp(old+raw,0,1) | Abs 为 raw-old；Delta 为 raw |
| height | `0x30` | clamp(raw,**0**,1) | clamp(old+raw,**0.1**,1) | Abs 为 raw-old；Delta 为 raw |
| ridge | `0x34` | **raw 原值** | clamp(old+raw,0,1) | Abs 为 raw-clamp(old,0,1)；Delta 为 raw |
| eave | `0x38` | **raw 原值** | clamp(old+raw,0,1) | Abs 为 raw-clamp(old,0,1)；Delta 为 raw |
| tip | `0x24/28` | **raw Vec2 原值** | 长度超 1 时归一化 old+raw | Abs 为 length(raw-limit(old))；Delta 为 length(raw) |

注意：变化量不是最终写入值减原值。特别是 clamp 后结果不变时，原代码仍可能产生非零变化量和反馈；零增量也仍触发 changed tick。profile、height 先写，tip、eave 次之，最后写 ridge。

每个非 0 模式均写入组件 changed tick，不先比较数值是否实际变化。Rust `changed_fields` 记录原指令实际写入字段的掩码，和原内存写入 hook 独立比较。所有成功 resolved 的事件，包括全部 Keep，均请求固定的 resolved_update 通知。

非法模式 3 也有真实指令分支：profile 除 1 外按增量，其余除 2 外按绝对。为了忠于原后缀，接口保留原 u32 discriminant，并验证了此 fallback；不暗示正常游戏会生成非法 enum。

## 触发与数值反馈

`is_gable` 在更新 profile/height/tip/eave 后、ridge 写入前后分别采样，**不是整个编辑前后的状态**。它使用已确认的原高度阈值和 ridge==1 规则。

- gable 前后不同时，或者编辑后为 gable 且 profile 非零变化/height 模式活跃/eave 非零变化，请求 gable 重建。
- before=false、after=true 时产生原 tag 10；before=true、after=false 时产生 tag 11。
- profile/height/ridge/eave 的变化量任一个按原 UCOMISS 判断为非零，产生 tag 2。载荷含 `ceil(abs(height_delta))`、`ceil(abs(eave_delta)) + (ceil(abs(ridge_delta))+ceil(abs(profile_delta)))`；height 的 ceil 为正时 weight 取 abs(height_delta)，否则取 `(abs(ridge_delta)+abs(profile_delta))+abs(eave_delta)`。
- tip 变化长度大于零时产生 tag 3，其载荷为当前 tip 长度和请求变化长度。
- old height >0.1 且 new<=0.1 产生 Down；old<=0.1 且 new>0.1 产生 Up。原输入标志字节 52 保留用于 ECS 载荷，Rust core 只恢复 crossing 类型。

UCOMISS 的 `SETNE/JNE` 在 NaN 下不会把它视为非零；因此普通 Rust `v!=0` 不能直接替代。clamp 按原 MAXSS/MINSS 第二操作数选择保留 NaN 和 signed-zero 行为。这些边界已经进入源码校验。tip 限长直接使用公开 `glam 0.29.3 Vec2::clamp_length_max(1.0)`；已检查安装版本 `src/f32/vec2.rs:764`，其超限路径为 `max*(self/sqrt(length_squared))`，与原 DIVPS 的除法路径一致，未恢复依赖内部实现。替换为公开 API 后已重新执行全部 4,514 cases，零差异。

反馈 pivot 使用原 `Roof::pivot_3d` 规则（0x1408e3220，38 字节）：根据 shape tag 读取 XZ，Y 读取 `Roof+0x4c`。数值存储用锁定 `glam 0.29.3` 的 Vec2/Vec3；条件、赋值顺序和这些游戏规则在新模块内恢复。

## 源码校验

[verification.json](verification.json) 为 **4,514 cases、0 差异**，登记当前 edit_rules.rs、runner.rs 和 Cargo.lock 的 SHA-256，后续改源码须重新校验：

| 覆盖组 | cases |
|---|---:|
| 五种模式组合（0/1/2） | 243 |
| 标量 old/request 边界，包含 NaN/Inf 与模式 3 | 2,028 |
| tip 边界、长度超限、NaN/Inf、模式 3 | 243 |
| 混合随机事件 | 2,000 |

比较完整 Roof 88 字节、changed tick/mask、5 种变化量、旧/新高度、gable 状态与触发数量、所有反馈数值载荷和 pivot、height crossing，全部使用精确 bits。

[verify.py](../../scripts/source-edits/verify.py) 用原 RNE 指令从 `0x1421abb2c` 执行直到单事件完成。实体和 Roof 已解析，消息队列是足够容量的 prepared 合法内存；原 `is_gable`、pivot、参数数学和消息记录写入全部实际执行，不替换任何核心算法。唯一数学 bridge 是原 CRT 导入 `ceilf` 到当前 Windows 的实际 ucrtbase.dll。Rust runner 经共享 `scripts/dependency_build.py` 链接 Cargo 锁定依赖。

**未完成边界**：全函数 Bevy ECS 查询、原事件读取调度、实体查找失败、队列扩容、事件 ABI ownership、未初始化 padding、SSE 异常标志。该模块和报告不应被登记为完整 ECS edit_roof 系统，也没有场景回放、GPU 或渲染校验。

公开入口：`RoofEditEvent::from_observed`、`apply_edit`、`apply_resolved_roof_edit`。输入可使用已恢复的 Roof 88 字节和原 Event 64 字节；结果是明确的 `EditOutcome`、`EditChanges`、`EditFeedback`、`HeightCrossing`。
