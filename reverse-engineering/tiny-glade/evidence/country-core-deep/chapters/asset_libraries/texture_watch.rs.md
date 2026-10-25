# asset_libraries/texture_watch.rs

> 文件作用：**纹理文件热重载监视器**——`TextureWatch { hotwatch: Option<GenericHotwatch>, changed:
> Arc<Mutex<HashSet<RelativePathBuf>>> }`：`watch(name: String, path: &Path)` 注册后，Create/Write/Rename 事件把
> **被监视的名字**（RelativePathBuf）插入共享集合；消费端读取并重载（消费过程不在本模块，U）。证据：
> `evidence/country-core-audit/A3_texture.md` §TextureWatch（O）。

函数总计 4：业务 2 / 内联无独立符号 0 / 派生胶水 2

## 布局表（O，A3_texture）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | `Option<GenericHotwatch>`（判别 None） | new 0x1408bab30 |
| +0x08 | `changed: Arc<Mutex<HashSet<RelativePathBuf>>>`（Arc 块 0x48 B） | new 0x1408bab30 |

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `TextureWatch::new` | 0x1408bab30 | 17 | 猜测：构造；证实：按 use_watch 建 hotwatch（100 ms） | **O**：`use_watch` → `GenericHotwatch::new_with_custom_delay(secs 0, nanos 0x5f5e100)` = **100 ms**（旧版 50 ms 错），失败 `expect("hotwatch failed to initialize")`；`changed: Arc<Mutex<HashSet<RelativePathBuf>>>` 空（Arc 块 0x48 B） | A3_texture.md |
| `TextureWatch::watch` | 0x1408bad70 | 29 | 猜测：注册监视；证实：**签名 `(&self, name: String, path: &Path)`** | **O**：`rdx=&String, r8:r9=&Path`；先 clone Arc；hotwatch None → drop Arc 与 name 返回；否则 `hotwatch.watch(path, 闭包{name, arc})`，失败 `expect("failed to watch file!")`。旧版把参数当 PathBuf 并在回调里读文件字节——原版不读文件 | A3_texture.md |

## 派生胶水（2 项：closure）

- `impl$0::watch_path::closure$0` 0x14091edb0（O）：`DebouncedEvent`（notify 4.0.17）仅 Create(2)/Write(3)/Rename(6)（掩码 0x4c）处理：Mutex 锁（poison→unwrap），`insert(RelativePathBuf(name.clone()))`；插入的是**监视时的名字**而不是事件路径。变体序号按 notify 4.0.17 源码顺序对应（I，由 `0x4c` 掩码 + 变体数 9 推出）。
- `impl$0::new::closure$0`：无独立符号（I）。

## 遗留（U）

- 消费端（读取并重载 changed 集合）不在三个过程里——**round3 已闭合**：`systems::backend::texture_watch` = **0x140aab370**（PDB 2640B，decomp 23774B 有效，`backend-misc/decomp/decomp-0x140aab370.txt`；oa2 w9 逐串复核），算法见下方 §消费端。
- 与 ConfigWatch/MeshWatch 同族：100 ms 延迟、回调写注册名而非事件路径（O 共性）。
- `FUN_140b73a50` 身份（u16 位图载体细节）留待后续 worker（I）。

## 算法要点（本批深挖 2026-10-15，decomp: asset-libraries/decomp/）

证据：tg_m1 反编译 `decomp-0x1408bad70`（watch 1928 B）、`decomp-0x14091edb0`（closure 4399 B）。

### 热重载全算法（O）

1. **注册**：`watch(&self, name: String, path: &Path)`（rdx=&String，r8:r9=&Path）：先 clone Arc；hotwatch None → drop Arc 与 name 返回（静默）；否则 `hotwatch.watch(path, 闭包{name, Arc})`，失败 `expect("failed to watch file!")`。**不读文件**。
2. **回调闭包**（0x14091edb0）：`DebouncedEvent` 判别 = `tag ^ 0x8000000000000000`；**变体 < 7 且掩码 `0x4c` 位命中 = {2 Create, 3 Write, 6 Rename}**（0x4c = 0b01001100：bit2/bit3/bit6）才处理，其余 drop。处理：锁 `changed`（poison → unwrap panic）→ `insert(RelativePathBuf(name.clone()))`——**插入的是监视时的名字而非事件路径**。Rename(6) 变体有专门分支（携带 from/to 两 PathBuf；闭包内 destructure 后仍插 name）。
3. **消费端（round3 闭合，O）**：见下方 §消费端热重载循环——`systems::backend::texture_watch` 0x140aab370（有效 decomp 23774B，w9 逐串复核）锁集合 → 逐名字 NameHash 查 texture_library by_name → 重载。

### 关键常量

| 常量 | 值 | 含义 |
|---|---|---|
| 监视延迟 | 100 ms（0x5f5e100 ns） | new_with_custom_delay（旧版 50 ms 错） |
| 事件掩码 | 0x4c（位 2/3/6） | Create/Write/Rename（notify 4.0.17 变体序，I 由掩码+变体数 9 推出） |
| 布局 | 0x10 B：Option<GenericHotwatch>@0、Arc<Mutex<HashSet<RelativePathBuf>>>@8（Arc 块 0x48 B） | TextureWatch |
| 队列元素 | RelativePathBuf(name) | 名字（非路径）进集合，按名查库 |

### 消费端热重载循环（round3 闭合，O；systems::backend::texture_watch 0x140aab370，2640B/23774B decomp）

`use_watch` 门控 → 每帧：

1. 锁 `changed` 集合（ArcInner Mutex，futex@+0x10；poison → `Result::unwrap` panic，0x142adcf70 串）。
2. 集合非空且日志级别 > 2 → log（"Texture watch: detected changes: "，0x142add810，len 0x21；级别 3）。
3. **hashbrown SSE 组遍历**（FUN_140b73a50 u16 位图载体 ×2）：逐占用桶取名字 → **`NameHash::compute`（FUN_140c9cd80）→ 查 texture_library `by_name` 表（@+0x30 hashbrown）**（FUN_140917860 校验/比较；`func_0x00014003bb80` 为 1B ret no-op 空 drop 占位）→ 重载纹理。
4. 任一失败 → log "Error processing textures: "（0x142add878）；一次消费清空。

即消费端 = **按名字（NameHash）查 texture_library by_name → 重载**（此前章节推断成立，现为 O）。`FUN_140b73a50` 身份（位图载体细节）留待后续 worker。证据：`backend-misc/decomp/decomp-0x140aab370.txt`（23774B 有效）+ `round3/oa2-results/w9_audio_backend.json`（verdict 一致，逐串核对）。

### 模块间调用

- `TextureWatch::watch(name, path)` ← 纹理加载方（加载时按名注册）；→ 消费端 `systems::backend::texture_watch`（0x140aab370，round3 闭合）→ texture_library by_name（NameHash 查表）。与 ConfigWatch/MeshWatch 同族：100 ms 延迟、回调写注册名而非事件路径（O 共性）。
