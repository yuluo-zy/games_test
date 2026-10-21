# 窗灯—玻璃同 World 候选 V2（精确重基 fa53）

状态：已实现、待独立源码审查与 root 编译/CPU/GPU 验证。当前未写 production，未运行 Cargo、测试或 GPU，未更新任务完成统计。

## 调用闭包

- `country_core::resources::window_lights` 是唯一组件/资源权威：`WindowLight { key:usize, decorator_id:u32 }`、`LightInfo`、std HashMap 全局控制器。
- `VisualEntityBuilder::build` 复用原缓存，直接插入 canonical `WindowLight`；新同 World 测试实际执行 builder Commands、灯系统及删除，不造第二套组件。
- `window_lights` 使用原六参 Query/TimeOfDay/GameTime/ActiveSession/EventReader；保留最后未读事件、frame+map ordinal 本地 fastrand1.8、真实 live retain、MAXSS 第二操作数、原 expf 连续插值与手动闪亮语义。
- Nani 复用实际 `InstanceKey`、owned subset、原 `build_window_glass` 数学与既有 flush；入场校验要求 canonical light/color/Transform/唯一 marker；指定未知贴图按原失败，灯键缺失亮度为零。
- App 窄 hunk 将窗灯放原 set36；现有 `gameplay_startup.rs:197` 把实际 Nani 维护→ApplyDeferred→更新→flush 挂 set44。原 OnExit(Game) 窗灯片段只替换本资源，不冒称完整 decorators unload 已挂。

## 原证与设计边界

已独立审查的 contract/lifecycle addendum 绑定原 B：Window system 0x1407b5980/3333B、Glass 0x142075070/4529B、retain 0x141268bd0/599B、lerp 0x140c904e0/81B、unload 0x140347090/9451B。未新增二进制结论。原 UCRT expf 在本机目标的末位等价待验证。

## 验证范围

新增 14 项待执行测试（系统6、helper3、builder1、真实 Nani owner4），保留已有系统2项及原 Nani 测试。Nani 四项使用现有 MockDevice/真实 AssetTextureLibrary/同 World owners：亮度与实际描述符索引、barrier/脏区上传/卸载、缺输入不发布、未知纹理失败、空查询仍借用目标 subset。

仅执行 `rustfmt --emit stdout --config skip_children=true --edition 2024`，8 个源均解析成功。此结果不是类型检查或通过测试。生产状态未经本候选改写。

## 仍未闭合

当前 decorator_visual 没有真实设置 builder.window_light 的上游调用；现有 typed builder 闭包尚不能证明窗场景已生成。真实 LightWindowCmd UI producer 待恢复；system-color 的 SetWindowLightCmd 是颜色写回并非此事件。完整装饰物 editing/placement、全部卸载 owner、场景资源与真实 GPU 场景验证独立推进，不能记作本候选已完成。

## 整合约束

`run_game.rs` 和 resources/events/utils 只逐 hunk 应用；root 同时改 App 音频构建，禁止整文件覆盖。before hash 保留本轮读取基线，manifest 同时记录冻结时生产 hash。claim 不改；root 为唯一 production/Cargo/GPU/Git 整合 owner。
