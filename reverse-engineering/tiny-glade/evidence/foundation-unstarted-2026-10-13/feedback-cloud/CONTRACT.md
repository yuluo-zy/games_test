# B feedback-reporter 与 geforcenow

业务源码在 shared-foundation/crates/{feedback-reporter,geforcenow}/src/lib.rs，保持原单文件模块。原作源路径归属经 B PDB 核对；函数范围和 SHA 在 function-ranges.json，四处 RNE 与 patched EXE 区域字节相同。反编译持久库为独立 feedback-cloud-unstarted-20261013；没有同步历史 Mac 或原数据库。

## Observed

GFN 载入 GfnRuntimeSdk.dll，依次获得 gfnInitializeRuntimeSdk、gfnShutdownRuntimeSdk、gfnIsRunningInCloud；先解析所有导出再 initialize(0)，非零码原样返回，不调用 query/shutdown。成功则 query → shutdown（结果丢弃）→卸载。缺 DLL 或缺导出传播加载错误；不把这些状态改成 false。334 字节原入口的 18 个注入边界用例通过，详见 gfn-native-validation.json。

反馈：阻塞 HTTP Client 只设置 connect_timeout 5 秒；text/meta 必有，随后按顺序可选 screenshot(image/jpg)、save(application/zip)、benchmark(application/zip)，不添加文件名。原字节中的 obfstr 常量已离线解出到 feedback-endpoint-observed.json，未发送。2xx 成功且不读 body；网络失败返回 Connection failed；非 2xx 返回 body 文本，body 读取失败返回 Error while waiting for a response。

zip_dir 仅按请求的相对文件清单遍历，不递归目录；前缀为 root 的 UTF-8 文件名。不存在的项跳过，存在的非文件进入原 unimplemented 分支；文件完整读入并写入原 zip 0.6.6，权限 0755。发送方 Cursor 的两字 ABI 占 R8/R9（R9=position 0），后续压缩参数 1 为 Deflated，不可把 R9=0误读成 Stored。benchmark.puffin 与 info.txt 固定名；GPU文本读成功用 GPU_UTILIZATION: Ok\n 前缀，失败用 Err(...)，benchmark打包失败仅记录并省略附件，save打包失败传播。

等待分支：save 存在时依次检查两个原 Path::is_file，短路；缺文件且 counter <=600 时递增并等待；counter >=601 强行取出。文件齐或没有 save 直接取出。计数 wrapping。48 个原分支切片用例通过；替换的是路径依赖边界，不是游戏私有条件。Native slice 不包括完整 Bevy 容器/发送段。

## Inferred

公开 Rust 字段名为可审查消费者模型，不是原 PDB 缺失字段的原名或 ABI。两个 readiness 路径的语义名未知，因此由生产者明确传入；不凭目录名猜文件名。FeedbackBenchmarkInfo 的输入文件字段、PendingFeedback 的类型组合、GFN 原错误枚举标签/Display 仍属推定。原Bevy事件/Update/Local策略已接线，额外 FeedbackTransport Resource 是复现依赖边界的测试/接入适配。

## Unknown 与验收边界

完整原 report_player_feedback/zip_dir 入口尚未 oracle；真实 HTTP/GFN SDK 与 country-core 原生产者未执行。原B精确直接依赖使用原锁；共享Cargo gate由父整合。Rust tests/Clippy 必须以实际共享门禁报告为准。C tg-mods 按用户新指示已暂停，只有原库存证据，没有声称恢复任何C业务。

## 重跑

原分支：python gfn_native_probe.py；python feedback_wait_native_probe.py。GFN 真实适配 fixture：fixtures/build.ps1 创建本地测试 DLL，再在共享 workspace 执行 cargo test -p geforcenow native_dll_fixture -- --ignored。该 DLL 只有测试状态和三次调用计数，不接入外部 SDK 或服务。默认 Rust tests 使用注入 Transport。

交叉review修正：FeedbackReporterPlugin现init_resource默认HttpReportTransport，构造不请求网络；已有fixture/custom provider保留。资源注册是重建依赖边界glue，不声称原build另有该Res注册。独立add_plugins后无需额外资源即可执行Update。

原目录分支错误文案经 .rdata 0x142bc8c38 二次校正为精确 not yet implemented；源码使用显式panic保留作者未实现分支，非恢复任务的占位成功实现。

## 最终共享门禁

真实 shared-foundation --offline --locked test：反馈8/8、GFN普通2/2通过；all-targets Clippy -D warnings通过。GFN ignored-only cargo复验因其他agent的共享target锁阻塞180秒而超时，未计通过；直接执行同一次已通过共享锁构建的unit exe指定ignored测试通过1/1，内部14组初始化码/云状态组合仅加载本地fixture DLL。最终源SHA、原case边界和测试二进制SHA见validation.json/source-hashes.json。10+1 Rust tests与66原指令案例是不同验证口径，不合并成完整原入口等价率。

专属Ghidra/ReVa worker已正常stop，worker-stop.log记录running=false。只保存专属副本。
