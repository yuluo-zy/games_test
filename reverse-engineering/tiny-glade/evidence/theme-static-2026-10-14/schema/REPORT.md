# B 主题传输/协议合同静态复核，2026-10-14

这份报告针对原 B 发行文件以及实际 `tiny-glade/crates/{theme-defs,theme-server-proto}`，没有启动游戏或请求任何主题服务。恢复范围是 B 客户端已发射的合同，不能代表未发射的作者源码、服务端实现或 A/C。

## 原作身份与工具

`static-extraction.json` 重新计算 EXE SHA `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323` 与 PDB SHA `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。LLVM PDBUtil 读取全部 IPI/TPI，精筛保留 19 项本库 scope/function records；没有本库 Rust TPI 类型字段。单独实读 proto module 2162，只有两项 serde 自动生成的 `expecting`。`crate-ipi.txt` 与 `proto-module-2162.txt` 是静态原始记录。

通过 Windows 官方 Python MCP SDK + ReVa/Ghidra 引擎的独立只读 worker `theme-schema-20261014` 取得 `inventory.json`：身份、限定 public 搜索、原内存 464 字节。与直接 PE 段映射读取逐字节相等。这是新 Windows 持久副本，没有同步旧 Mac overlays，没有修改原数据库。所有查询显式绑定 `tiny-glade.exe`。首次探查误用了历史参数名称、随后使用现场 `pattern` schema 纠正；最终报告使用宽 crate 名匹配，避免 public demangler 将泛型内 `::` 保留为 `..` 导致漏查。全文 artifacts 已按 operator 核验字节数与 SHA。

## theme-defs

IPI 0x3F73/0x3F76 是错误合同 clone/eq，0x3F78/0x3F7A 是活动合同 clone/eq。原 `ThemeFetcher::run` 0x140618310 的生产逻辑与现有 PDB inline bindings（`shared-foundation-2026-10-13/paths-language/theme-consumer-259.txt`）提供两种 unit 状态与 owned String 错误、两个 SystemTime 和 PathBuf。全体 16 个 public 匹配都是 std/ArcSwap 容器胶水，排除业务函数计数。

继续采用三个语义变体 `Pending`、`Error(String)`、`NoActiveEvent` 和三个语义字段 `data_folder`、`start_time`、`end_time`。完整 Result 的比较用于发布变化检测；新 `publication_identity.rs` 覆盖每个活动字段、100 ns 亚秒时间差、单位状态及 String 内容差（Windows SystemTime 的可表示精度为 100 ns）。时间选择半开区间在 fetcher，`is_active_interval` 是已有便于验证的合成 helper，不能称为原独立函数。

作者精确字段/变体名、两个 unit Display 文案仍未知：此发行版 line-tables-only TPI 不含字段/变体，全部 crate IPI 只含上述 clone/eq，没有 Display/Serialize/Deserialize。`thiserror` 和 `serde` 依赖存在不证明某种未发射 wire shape。consumer `update_daily_theme` 0x1403d1750 的 Err 分支直接退出，不补造 Display。保留已有明确标注的可编译候选文本，未把它升级成原版已证实文案。

## theme-server-proto

原 JSON producer `serde_json::from_slice<EventsResponse>` 在 PDB module 258。其中：

| 合同 | B 原证据 | 恢复规则 |
|---|---|---|
| `ActiveEvent` | serde struct visitor 0x140615f70；字面量 0x142a0a220/225/228 | start/end i64，cid String；三个字段必填 |
| 对象分支 | 0x140615f70 原伪码缺失/重复错误调用，unknown 分支调用 IgnoredAny | 对象顺序任意，重复已知字段报错，未知字段递归忽略 |
| 序列分支 | 0x140615f70 首分支接受 `[`；0x142a0a22b `struct ActiveEvent with 3 elements` | 依次 start/end/cid；长度必须为 3 |
| `EventsResponse` | serde struct visitor 0x140617820；0x142a0a280 `events` | 唯一字段 Vec&lt;ActiveEvent&gt;，必填，顺序保留 |
| response 序列 | 0x140617820 首分支接受 `[`；0x142a0a286 `struct EventsResponse with 1 element` | 一个 events 元素；空 events 合法 |
| 数值/字符串策略 | 既有 event-time-map/原 fetcher 消费，加上述类型 deserializer | 协议层接受全部 i64/任意 String；不夹取时间、不校验 CID |

原伪码复用 `foundation-unstarted-2026-10-13/themes/0x140615f70.c`、`0x140617820.c`；本轮仅恢复私有字段和 serde 属性，不重新实现公共解析器。Public inventory 7 项 = 2 个自动 visitor + 5 个 Drop 胶水；没有定位到独立手写协议校验或迁移函数。继续由原 serde 1.0.200 / serde_json 1.0.85 完成解析；Serialize 是已有重建便利接口，B 客户端发射证据主要覆盖 Deserialize，未声明服务器生成代码验证。

新 `b_wire_contract.rs` 有 5 条并行合同测试，覆盖对象/序列、完整 signed 数值边界、类型/溢出/长度错误、所有 required/duplicate 字段、unknown nested value 跳过、数据顺序、空 response、任意 CID 及不合法时间区间仍可解码。测试是候选协议边界回归，不是执行原代码的差分 oracle。

## 集成验证

`theme-server-proto` 新增已有原依赖 `serde_json.workspace` 的 dev dependency，因此首次 `cargo test --offline --locked` 被锁文件门禁拒绝。没有运行网络解锁；集成 owner 已负责仅补增 package 的测试依赖边及重新运行三库 locked/offline tests/Clippy。最终通过结果以父目录 integration 日志为准；本报告不把第一次未运行测试称为通过。

两个 crate 的源码和 README 已更新；未改共享历史恢复副本、全局台账、其他游戏模块。测试依赖没有新实现或新公共库版本。fresh consumer 静态读取已证实的 `langs.json`、`meta.json`、`history.json`、`credits.json` 是 tiny_glade/country_core 的消费合同，不能加进 transport defs 凑模块完整度；具体 schema 属于 integration 报告范围。
