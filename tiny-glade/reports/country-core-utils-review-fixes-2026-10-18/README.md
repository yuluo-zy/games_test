# utils 再次审查问题修复（2026-10-18）

本轮关闭 [再次审查](../country-core-utils-rereview-2026-10-18/README.md) 的 R1–R3。按用户授权，以当前源码的功能正确性为准，不宣称与原生程序行为等价。

## 修复

- R1：绘制视图名称 `DashedLineLengthView` 改为真实 `components::DashedLineLength` 的再导出。创建与读取具有相同 ECS TypeId，兼容旧导出和构造写法，不复制组件数据。
- R2：透视插值的总权重必须为正；计算结果还须有限且最终裁剪 w 为正。拒绝跨越透视奇点后的背面分支，保留相机前方正常外推。try 接口返回 None，既有滑块便捷入口保持起点。
- R3：`UiSignifierStream` 保存本帧活动 HashSet；创建阶段记录所有请求，包含被去重复用的标识。删除阶段读取同一集合，仅回收不再被请求且已达到寿命阈值的实体。系统组使用 Bevy chain，保证创建、动画、删除标记和销毁的顺序及延迟命令提交。

请求流顺带改用标准 Vec 追加和 mem::take 整批移出，删除无意义的自旋、重复游标及排空时的名字/闭包克隆。所有公开 write/take/drain/committed_len 接口保留。

请求为逐帧合同：生产者须在系统组之前提交；停用交互时丢弃当前批次并清空活动集合，恢复时必须重新提交，避免陈旧闭包重放。调用创建系统所需资源保持不变，删除系统现在需要同一个 `UiSignifierStream` 资源。最短寿命测试验证已有 t/min_lifespan 的删除阈值，不涵盖寿命计时生产者。

## 验证

执行目录为 `games_test/tiny-glade`，目标目录 `/tmp/tg-country-core-utils-review-20261018-target`。

```sh
cargo test -p country-core --lib --locked --offline --target-dir /tmp/tg-country-core-utils-review-20261018-target
```

[全量单元测试日志](tests.log)：2106 passed，0 failed，0 ignored。本轮新增 4 个测试，并扩展已有虚线工厂测试，覆盖：

- 真正的虚线工厂 → 网格/实体 → `read_env57` 绘制输入，检查长度和索引数。
- 真实透视矩阵下的奇点、背面、有效前方外推及实际 Slider3dState 调用。
- World 中运行实际注册系统组，多帧重复请求去重与实体复用、两条线部分撤销、停止请求后销毁，瞬态网格只入回收队列一次。
- 停用时丢弃排队请求，重新启用不重放；最短寿命达到阈值但仍被请求时保留，撤销后回收。

shader 编译边界复用项目已有 TestDevice，业务创建、绘制读取与生命周期代码均为真实实现。此结果不覆盖真实 GPU 图像、原作进程或完整游戏调度集成；回收验证到现有瞬态网格队列边界。

修改前快照位于 [before](before/)，差异、当前源码/依赖指纹及静态检查结果见 verification.json 与 changes.patch。历史反例脚本保留缺陷断言，不作为修复后的回归测试运行。

静态检查：`cargo clippy -p country-core --lib --tests --locked --offline --target-dir /tmp/tg-country-core-utils-review-20261018-target --message-format=json` 退出 0，修改的 7 个 Rust 文件无诊断；其他文件共输出 555 条 warning 诊断（含 lib/test 重复），未扩展修改这些模块。详见 [Clippy 汇总](clippy-summary.json)、[日志](clippy.log)。7 个修改文件的 `rustfmt --check --edition 2024 --config skip_children=true` 通过。
