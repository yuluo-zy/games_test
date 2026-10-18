# input 功能修复与工程化（2026-10-18）

目标：按用户授权优先保证功能性、正确性和可维护性，不再要求逐字段复刻逆向布局。业务文档见 [input/README.md](../../crates/country-core/src/input/README.md)，历史问题及修复映射见 [REVIEW.md](../../crates/country-core/src/input/REVIEW.md)。

## 已落实的修复

| 原问题 | 新实现 | 回归依据 |
|---|---|---|
| F1 同步点只保存工具栏裸地址 | InputSnapshot 持有工具栏列表、索引及类型化相机参数；相机立即恢复并清除旧动画；索引查询使用 get | 清空当前列表、移动相机后仍能完整恢复 |
| F2 文件只追加首帧，加载后没有同步点 | 新格式包含完整队列；解析、验证回退目标后整体替换；失败保留旧录制和资源 | 文件全帧往返、无效版本/索引拒绝、加载失败保留、空回放安全停止 |
| F3 丢失动作且回放写入孤立资源 | 每帧保存实际 MainActionSet，回放覆盖现有 ActionSetStates/ActionSetValues；删除 qword 翻译块、空动作转换和归一除数 | 三帧按下/持续/释放经保存加载后送达同一个真实消费者；相反实时输入不能覆盖回放 |
| F4 没有输入顺序边 | Prepare → Control → Translate → Resolve → RecordReplay → Consume → Finish | 通过完整 Schedule 验证首尾帧恰好消费一次，而非仅检查注册数量 |

## 可维护性和性能

- 数字状态从 disc/val 改为 DigitalActionPhase / timer_seconds；回放索引改为私有 next_frame_index，不再使用 u64::MAX 或减一回绕。
- MainActionSet 在自身声明 Resource；模块不再依赖模式管理器里的隐式实现。
- 文件和运行时帧采用同一语义结构。错误用 Result 返回，命令层记录 last_error；不因用户文件错误让游戏 panic。
- 动作热路径使用固定数组，普通帧没有动作 Vec 分配或工具栏克隆。Serde 直接写稀疏记录；名称解码不再逐键创建 String。
- Recent 策略每 1,111 帧建立分段，最多保留 9,999 帧/9 个同步点；没有历史事件时也能保持近期窗口。All 策略保留完整录制。
- 删除旧分段使用 VecDeque::drain；手工编码可通过 deconstruct_into 复用缓冲。
- 键位表是可配置 Resource，不再每帧创建默认表；设备模式使用宿主单调时间，取消固定零时刻占位。
- 标准 VecDeque、serde、glam、Bevy、tempfile 继续复用；本次输入改动没有增加依赖，也没有承诺未经测量的加速倍数。

## 验证记录

完整命令、最终结果和源码 SHA256 见 [verification.json](verification.json)。新增集成测试为 [input_pipeline.rs](../../crates/country-core/tests/input_pipeline.rs)。

最终源码验证：

| 检查 | 结果 | 日志 |
|---|---|---|
| country-core --lib | **2,098 通过，0 失败** | [tests.log](tests.log) |
| 输入完整流水线集成测试 | **4 通过，0 失败** | [tests.log](tests.log) |
| toolbar --lib 调用方测试 | **22 通过，0 失败** | [toolbar-tests.log](toolbar-tests.log) |
| country-core Clippy（--lib、input_pipeline、--no-deps） | **退出 0；输入实现无诊断**，其他目录既有告警保留 | [clippy.log](clippy.log) |
| 输入源码/集成测试 rustfmt、改动 whitespace 检查 | **通过** | verification.json |

共 2,124 项测试通过。测试期间输入实现及集成测试的指纹保持不变，见 source-snapshot-before-final.json / source-snapshot-after-final.json；这些是本次审查范围的快照，不是整个并行工作区的冻结交付。

中间失败记录保留：缺失 app_loading_state 模块、栅格测试集合类型歧义，以及保存模块系统配置的编译阻塞。缺失模块和保存配置随后由其原任务完成；本任务仅给原集合断言补充 BTreeSet 类型标注，不改算法或期望格点。相关日志是 blocked-*.log，不能当作最终状态。前轮 Clippy 的输入诊断已修复，旧日志保存在 initial-clippy.log。

## 接口与使用边界

- 新录制协议为 format_version=1，包含 stream.sync_points 和 stream.frames。旧 first_frame 文件缺失完整输入，被明确拒绝；未声称兼容原作文件。
- 文件最多 64 MiB；保存采用同目录临时文件后替换。失败不覆盖旧文件。
- 相机锚点恢复正常相机的已支持参数；输入文件不包含完整场景存档。宿主应准备相同场景初始状态。
- 宿主游戏消费者加入 InputSystemSet::Consume，并使用同一套输入阶段；不要重复安装复位或推进系统。原作 CLI 和 crash-report 的旧导出入口没有在本次模块重构中启用。
- 历史逆向审查和缓存探针保留原快照身份，不用于证明新设计等价。本轮以 Rust 功能回归验收，没有执行原版游戏或 GPU 画面验证。
