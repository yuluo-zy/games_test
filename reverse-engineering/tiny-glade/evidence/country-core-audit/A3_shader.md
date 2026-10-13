# A3 shader 组：`shader_library.rs`（6 过程）+ `configwatch.rs`（3 过程）

结论标记：O 机器码直接观察；I 推断；U 未知。测试台 `/tmp/h_shader`（21 个新单测，全 130 通过）。

## asset_libraries/shader_library.rs

布局（O，0xe0 B）：`+0x00 raster FrozenMap`、`+0x40 compute FrozenMap`（`add_compute` 里 `add rsi,0x40`）、
`+0x80 raster_names`（RwLock 字 +0x80、ctrl +0x88、mask +0x90、growth +0x98、items +0xa0、RandomState 密钥 +0xa8/+0xb0，桶 0x18 B）、
`+0xb8 compute_names`（锁 +0xb8、ctrl +0xc0、mask +0xc8、items +0xd8，桶 0x10 B，探测把 key 直接当哈希 = NoHash）。

### new 0x140936c60（656 B）
O：两张表的 `RandomState` 取自线程局部 `KEYS`（标准库，k0 自增），锁字全 0，ctrl 指向静态空组；无其它初始化。

### add_raster 0x140936ef0（1639 B）
签名 O：`(&self, program: *RasterShaderProgram(0xd8 B，按值经指针), force_new: bool)`。名字取自程序 `+0x48/+0x50`（`&'static str`）。
流程 O：写锁（`cmpxchg [+0x80], 8`）→ `!force_new && items != 0` 才查重 → 命中则 `FrozenMap::get(existing).unwrap()` 并逐字段比较
`desc`（+0x08/0x10 path；+0x30/0x38/0x40 `Option<String>`；+0x20/0x28 `Vec<(String,String)>` 元素 0x30 B）→ 全等：放锁、
`drop_in_place<RasterShaderProgram>(新)`、返回已有 handle；不等：`panic_fmt("AssetShaderLibrary: the shader name {} is already taken")`
（pieces @0x142ab10d0，参数 `<str as Display>`，位置 shader_library.rs:62:21）。未命中/跳过：`lock xadd [0x143634cd0],1` 取 handle →
`HashMap::insert(&str, handle)`（返回值丢弃）→ `Box`(0xd8，align 8) → `FrozenMap::insert`，`Err` →
`panic("assertion failed: self.raster_shaders.insert(handle, Box::new(shader)).is_ok()")`（:75:9）→ 放锁 → 返回 handle。
旧版错：形参 `(name, program)`（臆造）、`&mut self`、相等判据臆造为 source_name+compiled_variants、panic 文案臆造、键用拥有的 `String`、
无 `force_new`。I：`force_new` 的语义名。

### add_compute 0x140937560（886 B）
签名 O：`(&self, program: *ComputeShaderProgram(0xa0 B), hash: u64)`。写锁；`items != 0` 且键存在 → `panic_fmt`（同一份 pieces，
参数 = `shader_uri::impl$18::fmt` = derive(Debug) `ComputeShaderSourceHash { hash }`，位置 :89:13），**不比较内容**；
否则 handle（同一全局计数器）→ `HashMap::insert` → Box(0xa0) → `FrozenMap::insert` → 失败断言（:100:9，文案见源码）。
旧版错：文案 `{:016x} … already in the library` 臆造；`&mut self`；键类型当 u64 newtype 无 Debug 形态。

### get_compute_by_name 0x1409378e0（446 B）
O：`(&self, &ComputeShaderSourceHash) -> Option<(Handle, &Program)>`（rax=handle，rdx=程序指针，0 = None）。读锁（含 deadlock 资源登记，
parking_lot 行为）查表 → **放锁** → `FrozenMap::get(handle)`；缺失 → None（不 unwrap）。旧版只返回 handle。

### get_raster_handle_by_name 0x140937aa0（525 B）
O：`(&self, &str) -> Option<Handle>`（rax=命中、rdx=handle）；`items == 0` 直接 None，否则 `hash_one<RandomState,&str>` → 探测（h2 比对 + len + bcmp）。

### debug_raster_get_name 0x140937cb0（352 B）
O：`(&self, Handle) -> Option<&str>`（rax=ptr，0=None）；读锁下对整张表**线性扫描**（SSE 组位图遍历），桶 `[-8]` == handle 时返回桶 `[-0x18]/[-0x10]`（键 &str）。
旧版返回克隆 `String`。

### 旧版臆造入口（已删）
`get_compute_handle_by_name` / `get_compute_shader_name` / `get_raster_by_name` / `get_raster_mut` / `extract_raster_shader_names`：procs.tsv 里 `shader_library::` 只有上述 6 个过程。
保留 `get_raster` / `get_compute`（I，薄封装，消费者内联 `FrozenMap::get`）。

U / I 剩余：`force_new` 语义名（I）；`ComputeShaderSourceHash` 在原版定义于 `shader_uri`（O 符号路径），其 `compute()` 的算法已由第 0 层核对，这里的键类型因 `shader_uri.rs` 里同名单元结构无字段而另置（I）；
`RasterPipelineDesc` 占位的第二字段在原版是 `Option<String>`（O 比较序列：neg/seto niche 判别），占位为 `String`，不影响本文件判据。

## asset_libraries/configwatch.rs

布局（O，0x38 B）：`+0x00 Option<GenericHotwatch>`（首 dword==3 即 None）、`+0x30 Arc<std::sync::Mutex<Option<String>>>`
（Arc 块 0x30 B：计数 1/1、futex 字节 +0x10、poison +0x11、`Option<String>` +0x18，cap niche = i64::MIN）。

### ConfigWatch::new 0x1408ba0f0（405 B）
O：`use_watch`(dl) 真 → `GenericHotwatch::new_with_custom_delay(Duration{0, 0x5f5e100 ns})` = **100 ms**；`Err` →
`unwrap_failed("hotwatch failed to initialize", loc configwatch.rs:16:18)`；假 → None。Arc 载荷 `Mutex(None)`。
旧版错：50 ms；吞错误造空壳；队列类型 `Option<Vec<u8>>`。

### ConfigWatch::watch 0x1408ba290（499 B）
O：`Arc` 强计数 +1（溢出 abort）→ `String::clone(path)` → watcher None：全部释放返回（静默）→ 否则
`hotwatch.watch(path, closure{ watched: String, pending: Arc })`，结果非 Ok → `unwrap_failed("failed to watch file!", :37:14)`。
旧版错：把变更文件**字节**读进队列（O：根本不读文件）。

### watch::{closure#0} 0x14091e430（875 B）
O：事件判别 `== 0x8000000000000003`（I：`DebouncedEvent::Write`，notify 4.x 变体序）才处理，其余 `drop_in_place<DebouncedEvent>`。
处理：`log::info!(target="country_core::asset_libraries::configwatch", "Configwatch: {} has changed", watched)`（pieces @0x142aadc80，级别立即数 3）→
`pending.lock()`（poison → `Result::unwrap` panic，msg 长 0x2b）→ `*guard = Some(watched.clone())`（旧值 drop）→ 解锁（panicking 时置 poison）→ drop 事件 PathBuf。
**写入的是闭包捕获的注册路径，不是事件路径。**

### take_pending（I）
非 ConfigWatch 过程；消费者 `systems::backend::configwatch` 0x14085b1c0 对同一 Mutex lock+take 后调用 `color_library::update_color_library`（0x14093b530）。
