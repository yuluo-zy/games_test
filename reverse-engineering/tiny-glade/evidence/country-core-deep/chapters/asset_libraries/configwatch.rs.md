# asset_libraries/configwatch.rs

> 文件作用：**配置热重载监视器**——`ConfigWatch { hotwatch: Option<GenericHotwatch>, pending: Arc<Mutex<Option<String>>> }`：
> 注册路径后由 hotwatch 回调把**被监视路径名**（不是事件路径）写入 pending，消费端 `systems::backend::configwatch`
> （0x14085b1c0）take 后调用 `update_color_library` 重载。证据：`evidence/country-core-audit/A3_shader.md` §configwatch（O）。

函数总计 4：业务 2 / 内联无独立符号 0 / 派生胶水 2

## 布局表（O，A3_shader）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | `Option<GenericHotwatch>`（首 dword==3 即 None） | new 0x1408ba0f0 |
| +0x30 | `Arc<std::sync::Mutex<Option<String>>>`（Arc 块 0x30 B：计数 1/1、futex +0x10、poison +0x11、Option<String> +0x18，cap niche = i64::MIN） | new 0x1408ba0f0 |

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `ConfigWatch::new` | 0x1408ba0f0 | 13 | 猜测：构造监视器；证实：按 `use_watch` 建 hotwatch（100 ms 延迟）或 None | **O**（405 B）：`use_watch`(dl) 真 → `GenericHotwatch::new_with_custom_delay(Duration{0, 0x5f5e100 ns})` = **100 ms**；Err → `unwrap_failed("hotwatch failed to initialize", loc configwatch.rs:16:18)`；假 → None。Arc 载荷 `Mutex(None)` | A3_shader.md |
| `ConfigWatch::watch` | 0x1408ba290 | 25 | 猜测：注册监视路径；证实：clone 路径 + hotwatch.watch + 回调捕获 | **O**（499 B）：`Arc` 强计数 +1 → `String::clone(path)` → watcher None 静默释放返回；否则 `hotwatch.watch(path, closure{watched: String, pending: Arc})`，非 Ok → `unwrap_failed("failed to watch file!", :37:14)`。**不读文件** | A3_shader.md |

## 派生胶水（2 项：closure）

- `impl$0::watch::closure$0` 0x14091e430（O，875 B）：事件判别 `== 0x8000000000000003`（I：`DebouncedEvent::Write`，notify 4.x 变体序）才处理，其余 drop；处理 = `log::info!(target="country_core::asset_libraries::configwatch", "Configwatch: {} has changed", watched)`（pieces @0x142aadc80）→ `pending.lock()`（poison → unwrap panic）→ `*guard = Some(watched.clone())`。**写入的是闭包捕获的注册路径**。
- `impl$0::new::closure$0`：无独立符号（I，空构造闭包）。

## 纠错/差异（A3_shader，O）

- 旧版错：50 ms（实 100 ms）；吞错误造空壳；队列类型 `Option<Vec<u8>>` 且把变更文件字节读进队列（原版根本不读文件）。
- `take_pending` 非本模块过程：消费者 `systems::backend::configwatch` 0x14085b1c0 对同一 Mutex lock+take 后调 `update_color_library`（0x14093b530）。
