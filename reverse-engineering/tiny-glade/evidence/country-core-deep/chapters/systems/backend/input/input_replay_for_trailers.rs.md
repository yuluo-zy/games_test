# systems/backend/input/input_replay_for_trailers.rs

> 文件作用：trailer 用输入录制/回放：配置字节@+0xfc==1 且命中按键表时在 Idle/Recording/Replaying 间切换（'Start recording'/'Finished recording'/'Replay...' 日志），do_input_replay_ui 提供 egui 面板（'Enable input recording'、'`O` - start/stop recording'、tmp/input_replay.json 保存）。

函数总计 3：业务 2 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::input::input_replay_for_trailers::record_and_replay_input_for_trailers` | 0x14083c620 | 18 | 系统:输入处理 | O：record_and_replay_input_for_trailers：键切状态 + MainCamera::store/load_parameters + 恢复 InputTranslationState | ui-input\decomp\decomp-0x14083c620.txt |
| `country_core::systems::backend::input::input_replay_for_trailers::do_input_replay_ui` | 0x14083cdd0 | 67 | egui 面板 | O：do_input_replay_ui：checkbox 写 @+0xfc、'Stream frame count: ' 标签、'`O`/`E`' 键提示、Replay sheep 按钮 | ui-input\decomp\decomp-0x14083cdd0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
