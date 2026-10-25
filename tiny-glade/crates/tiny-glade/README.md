# tiny-glade

## 2026-10-18 core / rhapsody 调用接线

新增宿主侧 `tiny_glade::render_bridge::CoreRenderBridge`，共用一个 `VulkanGraphSession`，连接 core 计算/光栅 pass、权威 shader 句柄和真实资源导入，并提供构图失败取消。接口调查、调用方式、验证范围及剩余阻塞见 [接线报告](../../reports/runtime/2026-10-18-core-rhapsody-bridge/README.md)。这不代表完整 `WorldAccess`、`FramePasses`、游戏主循环或 GPU 帧已经验收。


持续开发的游戏集成入口，目前为 library facade。重导出 country_core、rhapsody、roof、history、terrain 和 utils。

## 来源

此包是整理后的工程接线，没有原作函数体，不计入逆向恢复量。算法来源见各被导出包的 README、[sources.json](../../provenance/sources.json) 和专项审计；整理核对了 Git a205bd8a 之前的恢复过程及已登记工作树内容。

## 已完成

正式 workspace 包、实际模块依赖与统一 facade。根 Cargo.toml 管理持续开发配置；渲染、建筑、自有工具及 HLSL 保存在各功能目录，编译不需要运行逆向工具。

## 还差什么

游戏二进制启动入口、窗口与设备初始化、真实 GPU/backend、原作完整 ECS 调度、源资产加载和全场景恢复。headless 建筑适配可显式使用 tg-building-runtime，当前未自动接入 facade。

## 验证与开发

默认 cargo check 检查此包和依赖；cargo check-all/test-all 检查整 workspace。原 Bevy fork 身份尚未核实，公共0.16用于已声明的开发适配，不冒称原 fork。当前整合结果见 [validation.json](../../provenance/validation.json)。
