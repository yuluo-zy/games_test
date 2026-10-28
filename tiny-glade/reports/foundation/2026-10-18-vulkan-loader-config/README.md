# 本机 Vulkan 加载配置修复与严格验证

日期：2026-10-18。范围仅为本机加载配置、已有 rhapsody 初始化入口和现成 core/rhapsody 链测试。没有改游戏业务或驱动安装，也没有继续其他恢复任务。

## 修复

本机已安装 `/opt/homebrew/lib/libvulkan.dylib`、`/opt/homebrew/lib/libMoltenVK.dylib`；ICD 位于 `/opt/homebrew/etc/vulkan/icd.d/MoltenVK_icd.json`。先前“本机缺少 Vulkan”是错误的环境判断。

1. [workspace Cargo 配置](../../../.cargo/config.toml) 设置默认 `RHAPSODY_VULKAN_LIBRARY=/opt/homebrew/lib/libvulkan.dylib`，`force=false` 保留用户覆盖。
2. [GraphicsInstanceLoader::new](../../../crates/rhapsody/src/raw/instance.rs) 读取该变量并传入既有 `from_candidates`；指定路径不可用时仍保留系统加载器 fallback。平台路径不写死进 Rust 初始化算法。
3. 最初只设置 `DYLD_FALLBACK_LIBRARY_PATH` 的尝试未生效：Cargo 重写了测试进程的动态库搜索路径。失败日志保留，不把该尝试算作修复成功。

从 workspace 运行 Cargo 会自动应用配置。直接执行已编译二进制时，需要显式传入 `RHAPSODY_VULKAN_LIBRARY`，因为 `.cargo/config.toml` 不是系统级环境配置。

## 实测

| 检查 | 结果 |
|---|---|
| 原 dyld-only 配置 + 沙箱严格链测试 | 失败，仍为 loader 未找到；[日志](strict-bridge-sandbox.log) |
| 修复后 + 沙箱严格链测试 | loader 已找到，失败变为 `ERROR_INCOMPATIBLE_DRIVER`；沙箱无 Metal；[日志](strict-explicit-loader-sandbox.log) |
| 沙箱 vulkaninfo | 已能找到 MoltenVK，但 Metal 不可用；[日志](vulkaninfo-sandbox.log) |
| GPU 可访问宿主 vulkaninfo | 成功枚举 Apple M3、MoltenVK 1.4.2、设备 API 1.4.357；[日志](vulkaninfo-host.log) |
| GPU 可访问宿主严格 core/rhapsody 链 | **2 passed / 0 failed / 0 ignored / 0 SKIP**；[日志](strict-bridge-host.log) |

严格命令：

```sh
RHAPSODY_REQUIRE_VULKAN=1 cargo test -p tiny-glade --test core_render_bridge --locked --offline -- --nocapture
```

两条实际链分别验证上传页/资源导入→提交→等待→回读/退役，以及构图失败/忽略错误/异常展开→取消退役→下一帧复用。日志中的 `game builder panic` 是第二条测试主动注入并捕获的异常，不是未处理失败。

此处“宿主执行”指允许访问 Metal GPU 的执行环境；同一命令在普通沙箱中仍会失败。这与 loader/ICD 是否安装是不同事实。不得通过关闭严格标志、返回 SKIP 或改为 CPU 替身来宣称 GPU 验证通过。

本次不覆盖完整游戏帧、窗口呈现、全平台驱动或原作行为等价。旧日志仅保留当次事实；上层旧报告将 loader 查找失败写成“本机缺 lib”的解释由本报告纠正。

源码/日志指纹与结果见 [verification.json](verification.json)。长期记忆已写入 [逆向索引](../../../../reverse-engineering/tiny-glade/README.md)。
