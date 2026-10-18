# tg-theme

主题活动交付模块。将服务端活动列表转换为本地时间表，下载并校验内容包，管理磁盘缓存，向调用方提供当前主题及其有效期。

本模块负责传输、调度和文件完整性。主题文件解析、资源加载、场景应用及界面展示由消费模块负责。

## 功能与结构

| 模块 | 职责 | 主要接口 |
| --- | --- | --- |
| `model` | 合法内容标识、时间区间、活动与消费状态 | `ContentId`、`EventInterval`、`ScheduledEvent`、`ActiveEvent`、`ThemeState` |
| `protocol` | 服务端数据结构及整份列表校验 | `Event`、`EventsResponse::validate` |
| `http` | HTTP 客户端与可替换传输边界 | `Transport`、`HttpTransport`、`Response` |
| `cache` | 有界下载、安全解包、缓存验证与清理 | `Cache`、`CacheLimits`、`TrimReport` |
| `service` | 配置、刷新、重试、下载调度与活动选择 | `Config`、`ThemeClient` |
| `service/subscription` | 后台线程、活动快照、变化通知与退出 | `ThemeSubscription` |

常用接口从包根导出：`Config`、`ThemeClient`、`ThemeSubscription`、`ContentId`、`ActiveEvent`、`ThemeState`。

## 数据合同

活动响应采用以下 JSON 结构。示例 CID 应替换为实际 ZIP 字节的 BLAKE3 摘要。

```json
{
  "events": [
    {
      "start": 1700000000,
      "end": 1700086400,
      "cid": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    }
  ]
}
```

`events` 必填，可为空，保留服务端顺序。`start`、`end` 是有符号64位 Unix 秒，必须满足 `start < end`。`cid` 必须为64个小写十六进制字符，标识整个压缩包。

协议解码接受未知字段，拒绝已知字段缺失、重复或类型错误。业务校验拒绝非法 CID、无效区间、算术溢出和超量活动；任一项无效即拒绝整份响应。

本地时间按 `服务端时间戳 − 服务端时差` 换算。负秒数截到 Unix epoch，截断后不能形成有效区间的活动仍被拒绝。活动有效期为半开区间 `[start_time, end_time)`。

## 业务流程

### 列表刷新

1. 到达刷新时间后请求 `events_url`，已有已接受的 ETag 时发送条件请求。
2. 根据 `Date + Age` 与响应头到达时的本地时间计算时差。
3. 200 响应有界读取正文；存在 Content-Length 时要求长度一致。完成协议与业务校验后，一并提交时间表、时差和 ETag；新响应缺少 ETag 时清除旧标记。
4. 304 要求存在已接受的条件缓存；保留包状态，用新时差重算区间。缺少 Cache-Control 时继承上次接受的刷新策略。
5. 刷新截止时间从响应头到达时计算，扣除 Age，读取和解析正文会消耗新鲜期。刷新间隔限制在1秒至24小时；no-cache/no-store 使用最短1秒轮询，no-store 不保留 ETag。活动数据继续作为应用运行状态使用。

刷新失败保留上一份有效时间表。列表请求和每个内容包分别使用1、2、4…300秒退避；失败期限从操作结束后计算，相应操作成功后重置退避。

### 包下载与活动选择

同 CID 只维护一份下载状态，刷新列表时复用已成功包。未结束活动均可预取，已结束活动不再触发下载。调度选择到达重试期限且最久未尝试的包，首次选择时优先当前活动，失败包不会长期阻塞其他包。

每次 `poll` 至多执行一次列表刷新和一次包获取，包获取可以命中缓存或下载。多个区间同时有效时，服务端列表中最后一个匹配项优先。该活动未就绪或失败时返回其 Pending 或 Failed，不回退到较早的重叠活动。

| 状态 | 含义 |
| --- | --- |
| `Idle` | 已有有效列表，当前没有匹配活动 |
| `Pending` | 尚未获得有效列表，或当前活动包尚未就绪 |
| `Active(ActiveEvent)` | 当前包可用，包含 `data_folder`、`start_time`、`end_time` |
| `Failed(String)` | 当前活动包失败；没有活动时也用于报告列表刷新失败 |

已有可用活动时，列表刷新错误不会覆盖它。普通 Failed 可随重试恢复；worker 异常退出产生的 Failed 是该订阅的最终通知。

## 接入方式

### 后台订阅

```rust,no_run
use std::time::Duration;
use tg_theme::{Config, ThemeClient, ThemeState};

let mut themes = ThemeClient::new(Config::new("theme-cache"))?.spawn()?;
loop {
    match themes.recv_timeout(Duration::from_secs(30)) {
        Some(ThemeState::Active(event)) => {
            println!("Theme files: {}", event.data_folder.display());
            break;
        }
        Some(ThemeState::Failed(error)) => {
            eprintln!("Theme unavailable: {error}");
            break;
        }
        Some(ThemeState::Idle | ThemeState::Pending) => continue,
        None => break,
    }
}
themes.shutdown()?;
# Ok::<(), anyhow::Error>(())
```

订阅保留最新快照，不积累通知队列，只有相对上次接收值变化时才返回状态。订阅根据快照独立处理起止边界，阻塞下载不延长旧主题有效期。先 `poll` 再 `spawn` 会保留已知状态。

`recv_timeout` 返回 None 表示超时，或后台最终状态已消费且线程已停止。Drop 请求取消并唤醒 worker，不等待退出；`shutdown` 取消并 join。worker panic 会发布 Failed、唤醒消费者，显式 shutdown 仍返回退出错误。阻塞网络请求受超时约束，文件系统操作仍需完成。

### 同步与自定义传输

| 接口 | 行为 |
| --- | --- |
| `ThemeClient::new(config)` | 创建默认传输，获取缓存所有权并执行启动清理 |
| `ThemeClient::with_transport(config, transport)` | 注入私有服务、代理或离线传输实现 |
| `poll(wall_time, monotonic_time)` | 执行一次阻塞调度，使用传入的冻结时钟 |
| `state_at(time)` | 只选择指定时刻的活动，不执行 I/O |
| `spawn()` | 转移客户端所有权，由后台线程使用真实时钟持续调度 |

`Transport: Send` 的 `get(url, etag)` 返回独立拥有正文的 `Response`，正文实现 `Read + Send`。传输及操作错误通过 anyhow 保留上下文。自定义传输负责认证、代理、请求与正文读取超时，必须提供有界 I/O 截止时间。

## 配置

| 配置 | 默认值 | 约束或用途 |
| --- | --- | --- |
| `cache_dir` | 调用方指定 | 非空，同根仅允许一个缓存所有者 |
| `events_url` | `https://ryeland.pouncelight.games/v1/events` | 活动列表地址 |
| `blobs_url` | `https://ryeland.pouncelight.games/v1/blob` | 包地址前缀，后接 `/{cid}` |
| `request_timeout` | 30秒 | 大于0且不超过300秒 |
| `default_refresh` | 10小时 | 必须大于0，响应无刷新策略时使用 |
| `cache_max_age` | 14天 | 启动清理的目录修改时间阈值 |
| `max_events` | 1024 | 活动数量上限 |
| `max_events_bytes` | 1 MiB | 活动响应正文上限 |
| `cache_limits.max_archive_bytes` | 64 MiB | 压缩包字节上限 |
| `cache_limits.max_files` | 4096 | ZIP 条目数及展开文件与目录总数各自的上限 |
| `cache_limits.max_file_bytes` | 64 MiB | 单文件展开上限，不得超过总量上限 |
| `cache_limits.max_unpacked_bytes` | 256 MiB | 全包文件展开总量上限 |

端点接受有主机名的 HTTP(S) URL，不允许凭据、查询参数或片段。默认传输不跟随重定向、不进行隐式系统代理发现。HTTP 端点不提供传输认证，需要代理时注入自定义传输；默认服务的实际可用性由部署环境验证。

## 缓存与资源管理

```text
cache_dir/
├── .owner-lock
├── <cid>/
│   ├── .tg-manifest.json
│   └── <theme files>
└── .tmp-<pid>-<sequence>/
    ├── download.zip
    └── payload/
```

下载以32 KiB缓冲写入临时文件并计算 BLAKE3，解包同样分块处理。验证目录空正文及 CRC、实际文件长度与 ZIP 声明一致后，再验证整个提取目录，通过同根重命名发布。失败清理临时内容，已存在目录不能单凭存在性命中缓存。

路径最长4096字节、最多64层，拒绝越界、符号链接、特殊文件、重复项、大小写别名、保留名称和平台不兼容字符。清单保存相对路径、文件长度和摘要；验证先检查完整目录树，再读取文件，拒绝额外项、遗漏父目录和内容损坏。清单大小也受路径数量预算限制。

缓存根使用操作系统文件锁，同实例操作使用互斥锁；进程退出自动释放所有权。启动清理删除遗留临时目录、损坏包和目录修改时间超过阈值的包；未来修改时间仍执行完整校验。清理不是周期任务，命中不会续期。独立缓存使用者可调用 `Cache::trim`，逐项错误由 `TrimReport` 返回。

已接受包在使用期内视为不可变，调用方不得从外部修改缓存。清单不承担来源认证，服务端身份依赖可信传输。旧 `.checksum` 格式不兼容，会清理后重新获取。

## 构建与验证

最低 Rust **1.89**，使用标准库文件锁；无异步运行时或 GUI 依赖。在 workspace 根执行：

```sh
cargo test --offline --locked -p tg-theme --all-targets
cargo test --offline --locked -p tg-theme --doc
cargo clippy --offline --locked -p tg-theme --all-targets --no-deps -- -D warnings
cargo fmt -p tg-theme --check
cargo run --offline --locked -p tg-theme --example replay_schedule -- <events-response.json>
```

测试覆盖输入边界、缓存事务与恢复、ZIP 元数据、限额、HTTP 状态提交、时差、新鲜期、公平重试、活动边界和后台生命周期。`replay_schedule` 只校验保存的响应，不连接网络。真实服务、资源解析及场景应用由集成环境验证。

## 相关文档

- [重构与旧包迁移记录](../../reports/theme-ui-refactor-2026-10-17/README.md)
- [源码复审与修复记录](../../reports/theme-ui-rereview-2026-10-17/README.md)
