# shader-pipeline / shader-compiler 本轮交付

三个 agent 已共同完成选定的 B 发行合同，并循环互审：pipeline 审 compiler，compiler 审 runtime，runtime 审 pipeline。统一证据为 [integration.json](integration.json)。整个原包与完整游戏尚未验收。

## 已接通

- `shader-pipeline`：完整发行 `.bin` 类型、反射、UBO 字段偏移、顶点输入、工作组、push constant、名字/哈希索引和文件键；404 文件、638 模块完整读取与逐字节回写通过。
- `shader-compiler`：从真实 `OpModuleProcessed(330)` 提取并重建原 DXC 请求，核对阶段与入口。源文件包含便利进程适配，但本机没有 DXC，实际重编译未验收。
- 运行时：真实发行容器 → 共享解码 → 保留 stored metadata → descriptor/push 布局 → 现有 ShaderLibrary 与 Vulkan compute；生产路径不使用外部 catalog 偏移或重新反射代替存储字段。

实现集中在现有 `reconstruction/rhapsody-rebuild`，没有新增孤立候选 crate。Speedy、FNV、SPIR-Q 与相关新增依赖身份匹配原 Cargo.lock；编解码器和公共解析库直接复用。

## 实际验证

15 项受影响测试全部通过：pipeline 5、compiler 3、runtime 3、既有 shader production 4。严格 all-targets Clippy 通过。

Intel UHD 630 实际 Vulkan 探针通过：原 fill-buffer shader 的 64 个 word 均读回为 43981；原 egui 容器创建了实际布局，阶段可见性为 31，错误注册后仍保留旧布局。egui 验证只覆盖布局创建，没有宣称 raster draw 或整帧成功。

## 已发现并修正的差异

互审修正了绑定 tag2、UBO 与 push constant 区分、raster 阶段可见性和 map 重复键覆盖。真实测试发现 DXC 命令来自 OpModuleProcessed，而非 OpString。真实 GPU 测试发现存储名 `output` 与 SPIR-V 调试名 `__alias_output` 不同，资源槽现按存储名绑定。

全量独立审查的 2577 条阶段绑定记录中，2362 条名字相同，215 条恰去掉 `__alias_` 前缀，其余差异为零。这是 B 发行产物映射证据，未据此补造任意输入的规范化算法或异常规则。

## 后续缺口

原包公开来源、完整加载错误/缓存所有权、描述对象全部 optional 字段、compiler 预处理/反射生成/变体打包/临时文件清理仍待恢复。原 AssetShaderLibrary 与 pass 的选择和生命周期、真实 raster draw、完整帧以及 A/C 等价仍未验收。便利构造函数和候选进程适配不计作原行为已经恢复。

原分析数据库和发行文件保留；三个独立 Windows worker 的分析副本未同步原 seed。源文件快照与首次整合失败日志均保留。

交付与互审：[pipeline](pipeline/delivery.json)、[compiler](compiler/delivery.json)、[runtime](runtime/runtime-contract.json)、[pipeline → compiler](pipeline/review-compiler.md)、[compiler → runtime](compiler/review-runtime.md)、[runtime → pipeline](runtime/pipeline-peer-review.json)。
