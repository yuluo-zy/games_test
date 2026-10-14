# tg-building-runtime

将已恢复建筑 CPU 算法接到真实 Bevy 0.16 World 的开发适配层。提供事件、资源、headless 测试与探针；这里的系统顺序是重建工程排程，尚未证明等于原引擎分叉图。

## 来源

原工程 `reverse-engineering/tiny-glade/reconstruction/crates/tg-building-runtime/{src,tests,fixtures}`。过程包含 Git `c3625e1d`、`531b7d32`、`0f9ded43`；逐文件实际提交和哈希在 [building-audit.json](../../provenance/building-audit.json)。证据入口：原 `evidence/building-integrate/findings.md`、`building-runtime/findings.md`、`building-gather/findings.md` 和 `reconstruction/README.md`。

## 已完成

墙采集/发布到屋顶创建、关联、动画/移除的 World 联接；路径、编辑、台阶、洞口、装饰、拱门和历史事务的开发事件消费者；Nani CPU 注册请求及连续 World/历史链 fixtures。业务直接调用 country-core::recovered_building，不重复恢复依赖，不生成虚假 GPU 对象。

## 还差什么

原引擎 fork 身份和准确调度、完整源资产输入、未恢复 freeform/结构消费者、实际 GPU/backend、全历史/场景/画面验收。fixtures 是显式集成输入，不能充当原场景证据。Bevy 胶水不累计为新逆向成果。

## 验证与开发

`cargo test -p tg-building-runtime`，或使用 building-run/nani-probe。默认 facade 未自动依赖此适配包；本包显式启用 country-core/native-physics 使用真实 Rapier。测试所需15个原 B 碰撞 JSON 固定于 tests/assets，并保留哈希；TG_B_ASSETS 可显式覆盖，默认不读取原发行安装位置。部分旧 CLI JSON 仍保留原输入路径，运行时由调用者明确提供。

Lint 仅为 Bevy 显式系统参数允许 too_many_arguments/type_complexity；publication 与 Nani 的负比较使用函数级许可保留 NaN 行为。当前结果见 [validation.json](../../provenance/validation.json)。
