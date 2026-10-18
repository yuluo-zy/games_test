# country-core utils 修复与工程化重构

当前入口：[业务说明](../../crates/country-core/src/utils/README.md)、[修复复核](../../crates/country-core/src/utils/REVIEW.md)。用户授权优先保证功能和正确性，不要求保留逆向直译的错误设计。

已处理原 F1/F2：栅格整数回绕及热重载发布错误位置类型。额外修复矩形末行/末列漏标、负坐标/NaN 点查询、投影 Y 偏移与齐次 w 丢失、音频计时回绕。调试线改为类型化 Mesh + 真实绘制组件和瞬态回收，去掉裸指针载荷；标准哈希完整包含实际颜色、通道和虚线模式。中文注释只说明当前合同，原地址/偏移说明留在历史记录。

## 最终验证

| 检查 | 结果 |
|---|---|
| 完整 `country-core --lib` 单元测试 | **2100 passed，0 failed / ignored**，其中本目录 **92** 项 |
| Clippy（lib、no-deps） | 命令退出 0；本目录 **0 项诊断**，其他目录仍有 250 个既有/并行修改警告，不宣称包级严格无警告 |
| rustfmt | 本目录检查通过 |
| 早期限定源码检查 | 115 passed，输入指纹在该次运行前后一致；此后以完整 core 最终检查为准 |

完整测试覆盖实际调试工厂唤醒后的网格、shader、单位缩放、瞬态回收凭据和实体销毁入队；坏网格重载保留原资产及修改列表；极值栅格、巨型斜线、有界输出、屏幕中心和无效透视；以及现有 core 调用方测试。不是只检查替身或复制后的算法。

首次整体重建被目录外保存系统注册的过渡接口错误阻断，后续该处更新后完整测试通过。增强调试生命周期断言时曾遗漏一个元组解构字段，已修正并完整重跑。失败日志保留，没有隐藏失败或沿用旧缓存 core 作为最新源码验证。

## 性能范围

矩形置位从逐格操作改为最多 64 次整行 OR；笔画先限制输出范围，每段最多 4096 个格，无网格外枚举。极端世界坐标可能触及世界时保守全图重建，避免精度损失漏标。JSON 向量直接反序列化为 glam，曲线重接复用 Vec，调试名字不再格式化整条点列。

`rustc -O` 直接包含旧/新 DirtyTiles 源码，各场景 100,000 次、5 轮中位数（ns/op，含新建空位图）：

| 矩形 | 旧实现 | 当前实现 | 本次比值 |
|---|---:|---:|---:|
| 完整世界 | 2896 | 58 | 49.93× |
| 中等区域 | 525 | 45 | 11.67× |
| 很小区域 | 89 | 123 | 0.72× |

很小区域增加约 34ns 的校验/范围计算成本，并非所有输入都提速。微基准受同机任务和缓存影响，早轮数值也保留；这里只支持矩形路径的局部结论，不推断完整帧率。旧实现完整矩形只标记 63×63，当前正确覆盖 64×64，因此不是逐位等价对照。

## 行为与接线边界

- 原指令差分及旧源码哈希已过期。本轮是产品正确性重构，不升级原作等价状态，不修改依赖版本或逆向数据库。
- 采样开关已接入取得有效存档的 `load_game_session` 辅助路径。完整 World 加载入口正在由其他工作恢复，当前遇到未恢复的世界应用会返回错误；不会为了写开关而把失败加载宣称成功，也没有在失败时覆盖当前会话采样策略。
- 调试线已提供真实 CPU 绘制实体和资源生命周期；球/三维矩形新增 show API、其他调用方的原生布局构建器及实际 GPU/FMOD 设备画面/播放不在本轮新增实现中。
- 保留 `ChildWall.a/b` 和 AudioLoop 的现有公开字段以兼容消费方，补充含义及 current_mut；没有批量改动其他音频域文件。
- 加载源码同时含其他工作修改；本任务只归属新增的有效存档采样策略应用语句，不把整个文件的其他变化算作本次交付。

## 可复验入口与文件

从 `games_test/tiny-glade` 根运行：

```sh
cargo test --offline --locked --target-dir /tmp/tg-country-core-utils-review-20261018-target -p country-core --lib
cargo clippy --offline --locked --target-dir /tmp/tg-country-core-utils-review-20261018-target -p country-core --lib --no-deps
rustfmt --edition 2024 --check crates/country-core/src/utils/mod.rs
python3 reports/country-core-utils-refactor-2026-10-18/run_source_checks.py
python3 reports/country-core-utils-refactor-2026-10-18/run_benchmark.py
```

| 文件 | 用途 |
|---|---|
| [final-core-tests.log](final-core-tests.log) | 最终完整 core 测试 |
| [final-clippy.log](final-clippy.log) / `final-clippy.jsonl` | 最终 Clippy；按诊断主位置区分本目录与外部警告 |
| [final-benchmark.log](final-benchmark.log) / [benchmark.rs](benchmark.rs) | 最终性能测量及直接源码基准 |
| [benchmark-baseline.rs](benchmark-baseline.rs) | 原位图实现，仅用于基准，不参与生产构建 |
| [sources.json](sources.json) / [baseline.json](baseline.json) / [utils.patch](utils.patch) | 当前源码指纹、初始快照与仅 utils 范围的差异 |
| [verification.json](verification.json) | 命令、退出码、摘要与证据指纹 |
| [previous-REVIEW.md](previous-REVIEW.md) / [previous-README.md](previous-README.md) | 保留前次问题及旧合同，不能当作当前状态 |

基准依赖产物由早期源码检查记录 SHA256；重放脚本拒绝不匹配产物，需先重新做限定源码检查。限定检查排除 debug_viz、顶层 helper、完整调度及保存加载接线，最终完整 core 测试覆盖的范围更大。
