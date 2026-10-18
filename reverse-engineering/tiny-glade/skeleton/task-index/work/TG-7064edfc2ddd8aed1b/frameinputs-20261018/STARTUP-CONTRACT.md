# B StairsState 启动资源与校验开关

本文件族 owner `frameinputs-20261018`；只恢复已证初始化，不重构其他墙/楼梯历史算法。原 B RNE 身份与完整指令范围见 `evidence/native-manifest.json`、共享启动证据 `TG-379258a80656c7937d/main-chain-frame-20261018/os-evidence/startup-producers/native-manifest.json`。

## 唯一校验开关

原 `VERIFY_STAIR_CHECKSUMS@14362c011` 属于 `.data` 的 initialized raw 区。section RVA 362b000、VirtualSize 10328、RawSize 9c00；目标 section offset 1011，文件 offset 3629811，实际字节 `01`。因此 release 初值是 true，不能按 BSS 假定 false。`evidence/release-statics-section-proof.json` 保留 PE 边界和实际字节。

生产只新增一个 canonical `pub static VERIFY_STAIR_CHECKSUMS: AtomicBool = AtomicBool::new(true)`。load_game owner 已按原版本门 `Version>0x1a` 写此同一 static，没有另造资源或复制开关。

## 原构造 site

完整 `DecoratorPlugin::build@140342c10` RNE 已先前保存。本次直接复用完整范围，不因共享 Ghidra BufferMgr 错误重开/改数据库。

- `14034420e/215` 和 `14034421a/221` 分别调用同一个 `GraphMap::default@140244140`，随后两次复制 0x90B 到资源 +0x28 和 +0xb8。该构造函数完整 392B 在本任务 evidence 中，容器长度为 0，实际 RandomState 使用原标准依赖 producer。
- `140344275` 写资源 +0x178 的 backed_up 为 false。
- `14034427c` 写资源 +0 的 checksum Option 判别为 None。
- `140344287` 写资源 +0x10 endpoint 判别值 3，即现有 endpoint 消费合同中的 unset。
- `140344292/29a` 从原 rdata `1429b0020/30` 复制空 HashSet 控制字到 +0x148/+0x158；`1403442a2` 写入新 RandomState seeds 到 +0x168。原空表 ctrl 指针 1429b0010，mask/growth/items 均 0。
- `1403442aa` 取资源基址，`1403442b4` 调 `App::insert_resource<StairsState>@14011af50`。

因此当前 owned 类型的两空图、空 dirty set、None checksum、false backed_up 可以复用依赖构造器；endpoint 的合法初始状态必须是 tag3。新增 `StairsState::new()/Default` 与 `ObservedStairEndpoint::unset_value()`。后者初始化 owned 内存的 inactive bytes 为 0，仅为安全承载，不声称原 Rust enum 未使用 payload 的原始字节也是 0。其实际 active endpoint schema 仍留显式证据边界。

`StairsState::empty(endpoint)` 兼容入口保留。未新增假楼梯、节点、边、checksum 或端点坐标。boot owner 可直接使用 canonical new 构造，补入原插件启动路径。

rustfmt 通过；未由本 agent 运行 Cargo/GPU，主任务统一验证。生产快照/哈希在 `frame-inputs-manifest.json`。
