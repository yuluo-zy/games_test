# systems/backend/input/reset_input_state.rs

> 文件作用：输入状态复位：清零并重排 InputTranslationState（11 qword 置换）、按最后 sync point 修剪双 VecDeque、ButtonInput::reset_all<KeyCode>()（wp20 合同复核）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::input::reset_input_state::reset_input_state` | 0x140942b00 | 9 | 系统:输入处理 | O：wp20 合同复核通过（0x140942b00 无本域 decomp，证据=wp20 analysis） | wp20/analysis/0x140942b00_reset_input_state.txt |

## 实现方式（算法级）（O，wp20/analysis/0x140942b00 复核）

**reset_input_state @0x140942b00（输入状态复位）**：
1. **清零并重排 InputTranslationState**：11 qword（0x58..0xa8 区）按录制帧布局置换/清零。
2. **按最后 sync point 修剪**：InputStream 双 VecDeque（sync_points + frames）按当前 sync
   point 裁剪旧帧（drop 前段）。
3. **`ButtonInput::reset_all<KeyCode>()`**：复位全部键盘按键沿状态。
4. 保留 `InputStream` 结构（mode/index 不动）——仅供录制重来。
