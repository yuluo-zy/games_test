# utils/audio_utils.rs

> 文件作用：**循环音效播放包装**。`AudioLoop` 是**自包含游标结构（非 Entity 非 Resource，≈0x38 B）**，
> 挂在 `AudioCore` resource（resources/audio_resources.rs）下：48 B entries Vec + Instant 计时 +
> 游标索引，驱动 **kira** `set_parameter("loop")`；失败 `warn!("Failed to set parameter named …")`。
> 证据：stop/start_if_not_playing 计时门控 + 游标遍历、current 越界 panic、instantiate_loop 构造。

函数总计 8：业务 4 / 内联无独立符号 3 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `AudioLoop::start_if_not_playing` | 0x140a596b0 | 0 | 幂等启动循环 | **O** | 重置 1e9 哨兵 + flag@+0x34 早退 + 游标遍历 + kira 增益(_UNK_142925904) |
| `AudioLoop::stop` | 0x140a59430 | 0 | 计时门控停止 | **O** | Instant::now() 首调记录 + elapsed≤阈值 早退 + 48B entries 检查 + kira set_parameter |
| `AudioCore::instantiate_loop` | 0x140a59220 | 49 | 由描述构造循环 | **O** | AudioLoopDesc(name fat-ptr@+16, u32@+24) → entries Vec；失败 None 哨兵；内部 unwrap panic |
| `AudioLoop::current` | 0x140a593f0 | 95 | 当前 entry | **O** | `&entries[cursor]`（cursor*0x30 + base@+8），越界 panic |

## 内联/无独立符号函数

- `AudioLoopDesc::new`（行 12）— **O**（构造 {path: String}）
- `AudioCore::instantiate_loop_snappy`（行 65）— **I**（名称推断，snappy 变体）
- `AudioLoop::is_playing`（行 147）— **I**（谓词，名称推断）

## 派生胶水（1 项：closure 等）
