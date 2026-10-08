# Tiny Garden

Rust + Bevy 的原创自由建造微缩庭院项目，先开发 Windows 桌面鼠标版，手机/触摸后续移植。首版聚焦多层建筑外观、自动立面、屋顶、退台与编辑闭环，不做室内、宠物或手柄。

当前为开发中的灰盒，不是已完成游戏。工程包含独立领域/编辑核心、异步布局和 Mesh 生成、桌面镜头，以及正在验证的选择/拖拽建造工具；正式资产、连续画面动画、道路关系和存档尚未完成。

## 运行与检查

Rust ≥ 1.95。Bevy 固定 0.19.1，依赖解析记录在 Cargo.lock。

```powershell
cd tiny-garden
cargo run -p garden-presentation --features desktop --bin showcase --locked
cargo test --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
./scripts/check-boundaries.ps1
```

桌面运行需要兼容的图形设备/驱动；默认纯核心测试不需要 GPU，使用 `cargo test --workspace --all-targets --locked`。平台范围不等于已发布兼容性保证。

## 文档入口

- [工程说明与代码阅读顺序](tiny-garden/README.md)
- [当前桌面开发计划](tiny-garden/docs/桌面版开发计划.md)
- [模块架构](tiny-garden/docs/模块架构.md)
- [验证记录与未完成项](tiny-garden/docs/验证记录.md)
- [美术资产制作计划](tiny-garden/docs/美术资产制作计划.md)
- [产品架构](TinyGlade_类手游_产品架构.md)
- [开发需求](TinyGlade_类手游_开发需求文档.md)
- [建筑变化与资产扩展](TinyGlade_类手游_建筑变化与资产扩展设计.md)
- [美术架构](TinyGlade_类手游_美术架构.md)
- [原实现计划与桌面优先修订](TinyGlade_类手游_实现计划.md)

文件名中的“手游”保留为历史链接；当前执行基线是桌面计划。美术概念图与引擎截图已区分，planned 模型清单不代表获批生产资产。仓库不包含编译产物或工作区其他项目。
