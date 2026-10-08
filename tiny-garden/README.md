# Tiny Garden · Bevy 模块基础

这是自由建造微缩庭院游戏的**模块工程与第二阶段比例测试场**，不是已经完成的游戏。当前实现建筑参数 → 编辑事务 → 异步布局/网格生成 → Bevy 投影 → 可选 3D 展示。纯核心与默认测试不要求 GPU / Android SDK。

产品边界：原创石木美术、先做 Windows 桌面鼠标版，触摸与手机后续移植；P0 多层建筑外观，不做室内、养宠或游戏手柄。

当前开发顺序和验收以[桌面版开发计划](docs/桌面版开发计划.md)为准，不再以 Android 真机包作为首个交付关口。

## 模块

| crate | 责任 |
|---|---|
| `garden-geometry` | 无引擎矩形运算与退台区域切分 |
| `garden-domain` | 建筑聚合、体块、楼层、屋顶意图、合法性约束 |
| `garden-application` | 命令、预览、撤销重做、版本与最新生成请求 |
| `garden-generation` | 布局、墙面开孔、三屋顶、米制 UV、纯 Mesh 缓冲 |
| `garden-bevy` | Plugin、后台任务、帧预算、投影更新通知、生成错误 |
| `garden-presentation` | 主题/资产合同与检查器；desktop 可选启用 3D 展示 |

说明、依赖方向、扩展方案和后续任务见 [模块架构](docs/模块架构.md)，验证结果见 [验证记录](docs/验证记录.md)。

美术怎样制作、先做什么、由谁交付、如何验收见 [美术资产制作计划](docs/美术资产制作计划.md)；基础窗 Blender 起步模板见 [美术源工作区](art-source/README.md)。

## 运行

Rust ≥ 1.95；Bevy 基础子库固定为 0.19.1，依赖版本记录在 `Cargo.lock`。首次构建可能需要联网下载依赖。

```powershell
cd D:\game\tiny-garden
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
./scripts/check-boundaries.ps1
cargo run -p garden-bevy --example headless --locked
cargo run -p garden-generation --example layout_benchmark --release --locked
cargo run -p garden-presentation --bin art-check --locked
cargo run -p garden-presentation --features desktop --bin showcase --locked
```

`headless` 打印变化与撤销结果，不打开窗口；微基准分别测 CPU 布局和网格，不测 GPU/手机帧率。

`showcase` 打开六种建筑的比例场，包括 1–4 层、三屋顶和退台；点击按钮改变第一栋房屋宽高/屋顶与撤销重做。新增右键拖动旋转、滚轮缩放，以及 View/Tilt/Zoom/Reset view 镜头按钮，不依赖键盘快捷键。面板上不启动镜头拖动或滚轮缩放；失焦/离开窗口取消捕获。当前没有平移、选择拾取、自由拖拽建造、触摸手势或连续画面动画，也没有加载正式 GLB/贴图。

离屏截图不打开窗口，需 GPU 渲染支持；输出路径不得已存在：

```powershell
cargo run -p garden-presentation --features desktop --bin showcase --locked -- --capture D:\game\tiny-garden\art-source\reference\new-review.png
```

主题为当前原创基础色与程序化占位窗框，不是成品画质。模型清单中 26 项仍是 planned；`art-check --strict` 会拒绝通过生产门，这是预期行为。

## 从哪里读

1. `domain/src/lib.rs`：`BuildingDraft` → `Building::try_new` → `Building::edited`。
2. `application/src/lib.rs`：`Editor::execute`、`Preview`、`JobTicket`。
3. `generation/src/lib.rs`：`compile`，看建筑参数怎样产生布局变化。
   再看 `generation/src/mesh.rs`：墙面带状切分、屋顶、法线与 UV。
4. `bevy-adapter/examples/headless.rs`：应用如何使用插件与编辑入口。
5. `bevy-adapter/src/lib.rs`：Commit → Collect → Dispatch 的系统顺序。
6. `presentation/src/renderer.rs`：仅消费更新通知，分材质上传、替换与资源回收。

路径均位于 `crates/` 下。暂时不创建无实现的道路、UI、存储和平台 crate；达到真实依赖边界时再拆分。

接 UI 时必须消费 `EditFeedback::drain()`；反馈满时提交系统会暂停，避免静默丢失命令结果。`QueueFull` 应显示繁忙反馈或保留待提交事务重试，不能把失败误报成编辑成功。
