# tg-package-integrity / tg-text 续轮

2026-10-14，当前开发工程 `D:/game/tiny-glade`，基线 B Windows x86_64。直接完善当前两个 crate；历史 `reconstruction/shared-foundation` 和导入快照保留，不将早期恢复范围当作当前代码范围。

## 实质改动

| 范围 | 已确认事实与改动 | 原证据 |
| --- | --- | --- |
| 完整性文件/扫描策略 | 11 个新 Windows 原 verify 案例全部匹配当前库；冻结大小写、分隔符、self 别名、root tmp 文件、目录条目、Unicode 与孤立 surrogate 的结果；库内生产算法不改 | [完整性报告](../integrity-followup-2026-10-14/REPORT.md)，verify `0x14067c3d0`、逐文件闭包 `0x140f77f30`、extra `0x140f79650` |
| 完整性诊断 example | 修正 verify 之前 `read/parse expect` 导致提前 panic；先走 read→checksum→parse→version→files，再收集可选诊断 metadata | [5 阶段示例结果](../integrity-followup-2026-10-14/example-results.json)；示例修复不计作新恢复原算法 |
| renderer getter / consume | `render` 读最近 consume 保存的 primitive 数量，并借用 atlas；consume 先保存数量，回调 unwind 保留流且不推进 frame | getter `0x140b8f670`，consume `0x140fac810`；[文字报告](../text-followup-2026-10-14/REPORT.md) |
| metrics 编辑状态 | 原作搬移 lines 到 `Buffer::new_empty`，再构造 Editor；替换此前只调用依赖 `set_metrics` 的实现。保留文本、属性和独立动作队列，重置光标、选择、scroll、size、wrap 与编辑设置 | `0x140b90780` 的原字节与 fresh MCP 反编译；[新合同测试](../../../../tiny-glade/crates/tg-text/tests/followup_contract.rs) |
| scale 与 atlas 增长 | 原 setter 不清 overflow flag；清缓存时保留已请求的增长，下一 upload 执行倍增 | `0x140b8f3f0` 与 upload `0x140b8f680` |
| glyph 内容 | 原 upload tag 0 展开 alpha→白色 RGBA，非 0 直接使用 RGBA；移除 SubpixelMask 的 RGB3/max-alpha 臆造转换 | upload fold `0x14180585e`；原 swash 的 RGBA32 内容合同；当前 cosmic fork 固定 Alpha 格式 |

## 联合门禁

在当前 `tiny-glade` workspace 实际运行，使用 `--offline --locked`，未修改 Cargo 依赖；[verification.json](verification.json) 绑定当前源码与日志 SHA，并确认检查期间选定源码/Cargo 上下文未变。

- 两库 `cargo test --all-targets`：**31 通过、0 失败、0 忽略**（integrity 8，text 23）。
- 两库 `cargo clippy --all-targets -- -D warnings`：通过。
- `cargo check -p tg-text --features gpu --all-targets`：通过，仅为可选适配器编译。
- 两库 `cargo fmt --check`：通过。
- [依赖闭包](dependency-identities.json)：默认 CPU 的 120 个外部 package version/source/checksum 与原 B 锁完全匹配；不把依赖检查计作游戏算法恢复。

原指令证据另计：integrity 11 个原 verify loader 容器案例零差异；text 19 个限定 getter/align/max-width 指令案例通过。完整性 loader 的 header/import/TLS patch、原函数范围未变 SHA 及运行边界见专项报告；没有运行发行 EXE main/GUI。文字构造、metrics 重建、scale 清理与上传内容主要为静态原字节/反编译合同，不冒称完整原函数运行差分。失败的初次 MCP job 已保留，修正 job 真正通过，且只保存到本轮独立 Windows worker。

## 当前未知与适配边界

- `render` 的 `&AtlasDescriptor` 是原 `&rhapsody::raw::Texture` 的 CPU 适配；真实 GPU 纹理/设备所有权和 toolbar/SSBO 消费者仍未闭合。
- fresh `set_family` 证实原接受角色字节并经 `Arc<FontTable>` 查找；当前 `set_family(String)` / `set_font_role` 仍是适配。原角色状态与共享表合同未恢复，不再将名称相同当接口等价。
- 原 atlas stamp/frame 为 u32，当前适配为 u64，并带额外失效策略；同帧清理顺序和 rollover 未等价验收。原 Bevy 分叉身份仍未知。
- NaN 工具链位模式、完整依赖 fork 等价、完整 OS/并发/异常路径、真实游戏启动消费者、A/C 与跨架构仍保留。已明确排除的四个报告/云平台包只留历史证据，不进入后续恢复待办。

本轮是确认规则驱动的源码修复与限定验收，两个模块继续为部分恢复，不宣称整库/整游戏/GPU 等价。当前开发源码 hash 登记于 `tiny-glade/provenance/foundation-followup-2026-10-14.json`；原作者文本、历史 Git 来源与历史验收 hash 不覆盖。其他聊天对全局 workspace 的并发修改不归本轮成果。
