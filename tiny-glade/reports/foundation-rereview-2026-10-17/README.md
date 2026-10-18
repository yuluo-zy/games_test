# 五包第二轮完整源码复审（2026-10-17）

重新审查 lang-lib、asset、asset-pipe、motomoto、paths 的全部源码、测试和 examples，并检查包间合同及 tg-iced 的直接消费。使用三个独立 agent 分担审查，主 agent 联合检查、复现与集成；没有执行逆向。

上一轮69项测试通过仍遗漏了下面的问题。本轮确认9类实现问题及1项测试可靠性问题，均已修复；没有因历史行为、旧说明或兼容测试保留错误。

## 实现发现与修复

| 优先级 | 位置 | 触发/影响 | 修复与证据 |
| --- | --- | --- | --- |
| P2 | `asset/src/texture/desc.rs:10`；`asset-pipe/src/texture/compile.rs:27` | 公开编译入口接受 `../escaped`，可写到 output_root 以外；scanner 的 array_、array_.、array_.. 会产生非canonical逻辑名。只读 list 也未验证路径。 | 共用 validate_logical_name，在扫描、公开编译及只读索引中拒绝空/跳转/非canonical组件、反斜杠和盘符前缀，所有源任务的名称在发布前预检。回归确认不创建输出目录、不写出 escaped.texture，并拒绝非法索引。 |
| P2 | `asset-pipe/src/texture/compile.rs:110` | a.png/A.jpg 或 é.png/é.jpg 产生不同逻辑名，却在常见 macOS/Windows 文件系统写到同一输出，两个任务互相覆盖。 | 对输出名进行大写+Unicode规范分解冲突检查，在所有平台保持相同的可移植规则；源扫描和只读索引复用同一预检。当前文件系统实测大小写与Unicode等价路径均为同一文件；回归在解码/创建输出前报错。 |
| P2 | `asset-pipe/src/texture/image.rs:205` | 透明边缘用 Catmull–Rom 过滤后，依赖夹紧alpha，破坏其与预乘RGB的比例；均匀灰色128实测变成129–137。 | 透明度滤波使用非负Triangle权重，全不透明输入继续使用Catmull–Rom；反预乘后才夹紧输出覆盖率。linear/sRGB均匀灰色边缘回归保持128。 |
| P2 | `asset-pipe/src/texture/image.rs:298` | linear 的16位/浮点样本在平均之前变成8位，改变最终像素；0.0019与0.0020平均本应编码0，原输出1。 | 保留16位/浮点输入为RGBA32F直到最后编码；16位和float的同类样本均回归通过。已有8位无额外整图分配路径保留。 |
| P2 | `asset-pipe/src/texture/image.rs:261` | RGB/alpha含NaN时，依赖在浮点转u8内部unwrap panic；Inf也未形成明确输入错误。 | 入口检查浮点通道，检查可能溢出的颜色变换和滤波阶段，返回Result错误。新测试先在依赖color.rs:435复现panic，修复后覆盖RGB/RGBA、linear/sRGB、NaN/±Inf。 |
| P2 | `motomoto/src/writer.rs:28,57` | mem::forget(ChunkWriter)跳过Drop，随后新增chunk或Writer::finish原会成功，返回未回填的损坏容器。 | 父Writer记录chunk_open，遗失chunk后两个入口均返回InvalidData；空/非空payload及两个入口回归先红后绿。 |
| P2 | `motomoto/src/writer.rs:128` | append File满足Write+Seek，但回填长度仍追加到末尾，原Writer错误报告成功；BufWriter的stream_position还可能隐藏实际追加。 | 回填后seek(Current(0))刷新并验证实际位置，发现错误后永久失败；真实append File/BufWriter回归先红后绿，故障矩阵覆盖新增seek。 |
| P2 | `lang-lib/src/language_library.rs:81` | YAML重复key被loader静默覆盖，甚至掩盖较早出现的非法值。 | 同一次YAML事件流记录根条目数，与最终映射长度比对，继续复用依赖的类型/alias解析。普通、引号、alias重复及非法前值回归先红后绿；合法alias保留。 |
| P2 | `lang-lib/src/diagnostics.rs:16` | 旧重入保护仅覆盖缺key日志；logger回调再次load_locale会递归发出替代/保存等警告，最终栈溢出。 | 所有警告共用线程局部RAII guard，锁外调用logger，panic后恢复；有界日志探针先红后绿，覆盖查询、加载、快照替换和panic恢复。 |

测试可靠性：`asset-pipe/tests/pipeline_contract.rs` 的临时目录此前只使用PID+系统时间。合并测试实际发生AlreadyExists，导致一个scanner测试误失败。增加进程内原子序号，与时间戳/PID组合，后续合并测试通过。该项属于测试基础设施缺陷，不算上表9类实现问题。

`paths` 的目录构造、平台策略、失败重试、一次发布、并发、cwd和非Unicode路径未发现新增确证问题，未修改其实现。

## 验证

最终合并测试82项通过，其中五包77项，直接调用方tg-iced 5项；另对最后的只读索引验证追加运行19项pipeline回归，全部通过。

| 包 | 通过测试数 |
| --- | ---: |
| asset | 21 |
| asset-pipe | 22 |
| lang-lib | 14 |
| motomoto | 12 |
| paths | 8 |
| tg-iced（调用方） | 5 |

[合并测试日志](tests.log)、[严格Clippy日志](clippy.log)。格式检查和git diff空白检查通过。命令使用独立target目录，避免共享构建锁：

```sh
cargo test --offline -p paths -p lang-lib -p asset -p asset-pipe -p motomoto -p tg-iced --all-targets --target-dir /private/tmp/tg-foundation-refactor-target
cargo clippy --offline -p paths -p lang-lib -p asset -p asset-pipe -p motomoto -p tg-iced --all-targets --no-deps --target-dir /private/tmp/tg-foundation-refactor-target -- -D warnings
cargo fmt -p paths -p lang-lib -p asset -p asset-pipe -p motomoto -- --check
```

此外，motomoto的12项Release测试通过；语言探针的10000次翻译命中、10000次缺失缓存命中仍各为0新增分配，1000次解析/释放净保留0字节；五种隔离配置模式通过；388份现有纹理archive字节一致往返通过。

## 行为变化与边界

- pipeline revision由1升为2，旧编译缓存自动重建；文件版本8和字段布局不变。滤波正确性修正会改变相关像素结果。
- 透明图像改用Triangle过滤；16位/浮点linear保留处理精度；非有限浮点明确拒绝。
- 名称校验更严格，大小写/Unicode等价输出名在Linux上也会拒绝，使源目录可跨平台使用；为此增加unicode-normalization 0.1.22直接依赖，版本复用已有锁文件，没有升级依赖版本。
- chunk完成增加一次位置确认seek；遗失chunk和append输出从错误成功改成明确失败，无公开API/有效格式变化。
- 重复YAML现在是输入错误；按既有服务策略，非法目标包仍可整包回退至英文。

本轮在macOS验证，Windows/Linux真机、native-gui可选特性、完整游戏运行和GPU消费没有执行。intel_tex_2预编译对象的platform load command链接警告仍存在，测试和链接成功；没有修改第三方二进制掩盖警告。上述范围内未留有已确认而未修复的问题，但源码审查和测试不构成无缺陷证明。
