# tg-package-integrity：真实业务与恢复边界

本轮恢复原作 B 的小库校验业务，不要求作者源码、Rust ABI 或调度方式 1:1。实现位于 reconstruction/shared-foundation/crates/tg-package-integrity。本文件的静态结论来自实际官方 MCP SDK / Ghidra worker small-integrity-20261013，程序 /tiny-glade.exe，SHA256 cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323；原指令范围用发行 tiny-glade.rne 的 PE RUNTIME_FUNCTION 和原字节 SHA256 绑定。原发行文件、原数据库与其他 worker 没有改写。

## Observed

| 规则 | 原依据 | 候选入口与验证 |
|---|---|---|
| 原私有入口属于 crates/tg-package-integrity/src/lib.rs；PDB source SHA256 b52f8642f8d60fddef21bdefeb7e0c285b7dcb8d269560720f8c73d127291fc0 | skeleton/B/crates/tg-package-integrity/src/lib.rs，verify 0x14067c3d0 | 原私有策略复原，库实现直接复用 |
| 从当前工作目录读取 build-info/manifest.json，读取错误先返回显示化 IO 错误 | verify 原指令 / 0x142a16af0 字面量；first-result.json / prepare-result.json | verify_package_manifest(root) 显式 root 参数替代进程 CWD |
| 在 JSON 解析前校验原始 bytes，空格也在覆盖范围内 | verify 先调用 blake3::hash 0x140db6090，再调用 Hash<[u8;32]> 0x140f737f0 | is_valid_manifest_checksum；原 264376-byte 样本通过，追加空格失败 |
| FNV 初始状态为 0xf52dad71b327e45a；Rust 数组 Hash 写入 64-bit 长度 32 前缀，然后写入 32-byte BLAKE3 digest；只检查最终低 16 位是否为零 | helpers-prepare-result.json / 0x140f737f0.c，verify 的 TEST word branch；机器码下的原 Hash trait 包装 | fnv::FnvHasher::with_key + Hash::hash；不得把常量当作期望完整 FNV 值，不得省略数组长度前缀 |
| 清单为 version + files；files 为 (String, String) 列表，version 只有 1 进入检查 | 原 manifest.json 和 PackageManifest PDB 关系；verify version==1 分支 | serde::Deserialize / serde_json；未实现通用替代清单 |
| 逐条独立读取文件，没有单独大小字段或 size 短路；BLAKE3 的小写完整 hex 文本与期望字符串精确比较 | closure 原消费者 0x140f77f30，blake3 Hash display 0x140db5320，长度比较 + memcmp | blake3::hash(...).to_hex；大写 hash 记 corrupt，不“修正”输入 |
| 只有恰好等于 build-info/manifest.json 的 24-byte 路径跳过逐文件 hash；自身的 checksum 已在解析前校验 | 0x140f77f30 的字面量 SIMD 精确比较及 Valid 分支 | 不依靠真实文件 hash 自引用，不归一化 self 路径 |
| 所有文件读取失败统一归 Missing/unreadable；成功读取但 hash 不匹配归 Incorrect | closure CheckOutcome 0/1/2，switch-result.json；original-report-strings.json | Missing(String)/Corrupt(String)，不是只对 NotFound 归 missing |
| files 是顺序列表；重复条目仍独立检查并可重复报告，不先将它们去重 | easy-parallel::each 消费 tuple Vec，closure shim，真实聚合跳表 | HashSet 只用于 extra membership；磁盘测试保留重复 missing |
| 使用 easy-parallel 逐文件返回结果，然后聚合为三个列表；不存在发现一个错误就停止的策略 | each 0x140f77ad0、run 0x140f76810、verify 原跳表 0x14067c9b8 | 候选仍用 easy-parallel，所有失败聚合 |
| 从 "." 递归查额外文件；路径用 relative-path join；与根相对路径 tmp 等价的条目整树忽略 | find_extra_files 0x140f79650 / 0x140f78630；0x142bab8a1 字面量 tmp；relative-path Components::eq | RelativePath join/equality；nested sub/tmp 不忽略，file.tmp 不忽略 |
| symlink/name-surrogate reparse 条目忽略；无法转 UTF-8 的名字、read_dir/DirEntry/file_type 错误忽略 | find_extra 原 Windows FileType 位条件，OsStr::to_str 0x140d58e70 的失败分支 | std FileType::is_symlink、to_str、read_dir；平台异常实测范围见 Unknown |
| 普通目录递归，目录本身不报 extra；非目录名称不在原 files 的 String set 中才报 extra，membership 为原文本字符串 | extra 原控制流及 String hash set 比较 | std HashSet<String> + relative-path，不还原 hashbrown 内部实现 |
| 只有 missing/incorrect/extra 三列表都为空才成功 | verify 0x14067cb76 附近三个 length 检查；report caller 0x1402552b0 | FileIssues 三类，空才 Ok |
| 游戏的 integrity report 和 crash reporter 都调用库入口 | PDB skeleton 原直接引用 0x2552f5 / 0x3b040f | 本轮只闭合库与只读 example，不调用报告发送或游戏入口 |

## Inferred / intentional representation choices

- PackageManifest 由原私有 serde derive 符号、tuple Vec 的 PDB 关系和实际随包 JSON 确认。字段名/数值类型按样本与原版 serde 复用；候选采用标准 derive，并没有声称拿到作者原源码。
- 候选接受显式 package root，原入口无 ABI 证据可直接调用；原函数 output carrier 为 Result<(), PackageVerificationError> 的 PDB 候选。错误 enum 的 Rust 命名是新的可读接口，含义由原消费者字符串核对。
- 原 easy-parallel 在一次 run 中为所有 entries 建 job；候选以 64 个文件为一批调用同一依赖，限制线程峰值。稳定文件系统下仍按输入顺序聚合，不改变规则。并发修改文件时的精确读取时序未承诺等价。
- 原 paths/hex-literal 在 Cargo 中属于直接依赖；本轮没有需要恢复的 literal producer，也不复制 paths 其他 owner 的实现。显式 root API 使 library 能消费原 manifest；依赖裁剪是候选接口选择。
- Extra 文件顺序保持 std read_dir 原枚举顺序，没有人为 sort。测试为了比较无顺序的磁盘 extra 集合才排序测试副本。

## 实现、验证与差异修复

实现使用 blake3 1.5.3 / fnv 1.0.7 / relative-path 1.9.0 / easy-parallel 3.2.0 / serde 1.0.200 / serde_json 1.0.85。dependency-audit.json 的六项 version/source/checksum 与原锁一致；没有手写 FNV、BLAKE3、JSON、路径组件解析、hash map 或线程 scheduler。

4 项候选测试通过，覆盖原 manifest 结构与 raw checksum、缺失/损坏/严格大小写/重复条目/self 排除、root tmp 与 nested tmp 差别、manifest read/checksum/parse/version 各级早退、实际 disk 完整成功后损坏、目录读取失败静默规则。test-msvc-first.json 保留输出。Clippy all-targets -D warnings 通过。构建固定 --offline --locked；父 owner 统一处理 workspace lock，传递依赖审计另见 batch 根 dependency-validation。

回看真实原规则后，已纠正初始分析的“raw FNV 校验”“期望 checksum 常量”“manifest 自身 hash 照常检查”“临时文件后缀跳过”四种错误解释：原流程为 BLAKE3 -> 数组 Hash / seeded FNV -> low16==0；只 self 精确路径跳过；只根 tmp path components 跳过。以上误解均未作为候选最终代码提交，记录见 DIFFERENCES.md。

候选 inspect_package 只读实际 D:/game/ljxsj_92385/Tiny Glade：
1792 entries，checksum=true，missing=0，incorrect=2（steam_api64.dll / tiny-glade.exe），extra=288（额外 controller glyphs、steam_emu.ini、保留 .rne 等）。这是候选对当前混合发行/修补目录的实际读取，不是原入口执行；不会将这种不完整包伪报成功，也没有修补发行物。完整输出 candidate-real-package-result.json。

## Unknown / acceptance boundary

- 未受控执行原 verify/find_extra_files/checksum 入口，未执行实际 report/crash reporter；静态指令匹配、原格式样本与候选测试不等于原入口 runtime 对照。
- 作者完整字段声明、serde attrs 的罕见输入行为（字段别名/default/unknown-field/duplicate-field/sequence form 的逐项原对照）未全闭合；标准 derive 行为是候选，不为未观察 schema 行为编造证据。
- 原 Rust ABI/所有 enum 字段布局未恢复；未做原 DLL/函数直接调用。
- candidate batching 的并发文件修改时序、IO errno 文字完全匹配、线程启动失败与 panic 时的原精确行为未验收。
- Windows symlink/reparse/非法 surrogate、permission denied 等特殊 OS 条件从原控制流恢复策略，但本机这批 disk tests 未逐个制造这些环境。
- Hash array prefix 对应原 x86_64 小端 Rust Hash 语义；尚未验证 32-bit/大端 host。
- A/C 尚未验证；B 的源/地址不能套用其他版本。
- 只在专属持久 worker 准备有限原函数体与 switch targets；没有同步原数据库，也没有给共享 worker 增加签名。原 decompiler 对聚合 indirect jump 仍显示未恢复跳表；真实三分支已单独从原 table/机器码核实，不用它推断“没有业务”。

业务策略与可调用候选校验链已交付。完整原入口行为验收仍待独立受控 oracle，不标记 1:1 或完整游戏接线验收。

补充类型边界：version 的32-bit比较从原入口指令可见，manifest编译单元中也有serde u32解码泛型；然而完整字段TPI声明缺失，候选使用u32仍属于类型推断，尤其超过u32范围/负数输入的原parser对照未验收。Windows控制流没有is_file筛选，只有is_symlink/is_dir分支，因此非目录且非symlink条目候选按未知文件处理；跨平台socket/特殊文件的OS分类尚未运行对照。

测试fixture cleanup已按父review修复：canonicalize目标与本crate target/test-fixtures基目录，验证目标是该基目录的严格子树后再remove_dir_all；不涉及原库行为。
