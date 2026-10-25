# tg-package-integrity 开发与独立验收

功能清单见crate README。核心API和调度未改，私有helper/派生/泛型均按真实业务归并。库继续直接复用原依赖，独立same-source Cargo验证工程闭包18个公共package全部与原版本/source/checksum一致。共享workspace被其他任务持续修改，因此没有改变根Cargo/lock。

## 已完成

- 原verify 0x14067c3d0 原RNE范围2447 bytes / SHA5646cdb57855ea9bc2b4e28dd93a21b5465c1f42016a6d6d6d9f85bd602e1a39；原调用点与返回carrier写入、所有错误tags及3个Vec布局已核实。
- 原完整函数在Windows loader容器中运行29个独占磁盘fixtures：read/checksum/parse/version/Ok/Files，全匹配。罕见schema、错误阶段优先级、131missing重复与跨批次顺序、corrupt、nestedtmp/extra已实际执行。改动后29项再次运行全匹配，见native-contract-results-streaming.json。
- loader容器仅用于隔离原码执行，未运行游戏EXE入口/GUI/网络；原发行物只读。patch与SHA见native-container-identity.json；不能冒充原发行进程全环境等价。
- 2项有证据的回归测试新增至crate，预保存通过gate的schema输入避免重复nonce搜索；6测试、严格all-targets Clippy、release构建通过。
- release旧版真实目录1792 entries，2697518474 bytes，峰值1494405120 bytes，26.56秒，missing0/corrupt2/extra288。见real-package-candidate-observable.json。
- 修复上述确认的内存成本：以原blake3::Hasher::update_reader(File)替代每任务fs::read整文件；打开/读取错误仍Missing，完全hash文本和输出顺序保持。前后源码与production hash见source-binding.json，原源码快照source-before-streaming.rs。

## 历史诊断与最终结果

- 1792个实体空文件/1792×64KiB/64×2MiB的旧候选与原容器各5次成功；streaming各5次也功能成功、峰值6.4–9.6MB，但本轮系统争用使耗时从几十ms到7.8秒波动。不能宣称速度提升或回归，不能将此与原完整进程perf相比。
- 真实包streaming在45秒期限内未得完整结果，低峰值约8.2MB仅证明受控时段的内存。逐文件reader扫描记录58.46MB EXE约12.54秒、随后274.7MB PDB未在25秒整scan期限完成。与根owner观察到的CIM/编译全局迟滞相符，尚未归因算法。
- 根owner要求暂停启动更多包benchmark，等待系统平稳。后续同文件cache条件下对比fs::read和原库reader吞吐；如确认小块读取有成本，优先实测标准std::io::BufReader 1MiB/4MiB，直接复用依赖而非手写循环。不要未经完整真实包结果/性能复验标修复完成。
- 读取粒度/并发变动文件的时序、线程启动失败与panic、OS异常穷举、A/C与其他平台仍不承诺等价。
- 新工具worker未启动；超时探针用execFile自身deadline结束，身份过滤的CIM查询确认本模块无残留Python/validation-driver进程，没有停止其他worker。

历史batch/review报告及hash保留，本报告不覆盖原成功标记。

## 最终冻结补充

上文45秒期限未完成与暂停benchmark是历史诊断，不是当前结果。真实包after150秒期限内完整成功：103.55秒/9854976-byte峰值；唯一一次warm复核85.71秒/10125312-byte峰值，均missing0/corrupt2/extra288，列表与旧结果匹配。资源汇总final-resource-validation.json；内存相比旧峰值降低约99.32%。耗时高于旧26.56秒必须公开，但cache/并发负载不同，不能宣称确定性能回退或速度提升。

同58.46MB EXE，首次File reader4283ms、随后1MiB35ms/4MiB41ms，反向cache复核File26ms/1MiB31ms，BLAKE3结果全一致。说明首轮环境/cache影响显著，未支持增添BufReader。保留原Hashed reader API，未改调度或依赖。

发布源测试改为合成success contract，不嵌入原B-manifest.json。生产hash保持768120ee182088517ec6e295c3da0570a3e7cbe36014973718267db338ca2491；最终sourcehash见source-binding.json。该测试文本变化交根重新运行6测试/Clippy，不冒充当前已重验。原资产、patched loader DLL、PDB、真实fixtures及target只保留local，不提交。29原合同/6此前测试/Clippy/依赖与最终真实包功能及内存均已交付，本owner不再启动benchmark。
