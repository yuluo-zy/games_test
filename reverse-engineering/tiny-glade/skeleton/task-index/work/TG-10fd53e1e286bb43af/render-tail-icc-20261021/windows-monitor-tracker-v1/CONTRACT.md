# Windows 原 ICC 自动来源到当前 App 更新 V1

root明确委派game_loop field/init/publish三hunk，保原claim；不碰suspend/resize/shutdown及reviewer event_loop/harness。此包依赖core六源和Windows返回长度包先合；icc.rs before明确是长度包candidate，不覆盖原长度修复。

## 原证与工程时序

原B run_event_loop@14026e4a0设置dirty；每批OS消息结束后@14026f5d0..870，dirty&&monitor Some→先清dirty→Win32真实查询→仅成功发送tag0x11。当前主线程适配没有原OS/game mpsc，所以在GameLoop::frame已flush handoff、已dispatch捕获OS批次、已越过原零尺寸门后，当前App.update前发送canonical事件。截图/渲染/关闭/准备join时序不变。

Tracker只在Windows保存真实MonitorHandle和IccTextureDevice的Weak控制块身份。首次、实际monitor改变或App设备命名空间改变才查询；不按任意Moved重建、不持旧App/GPU owner，不把同数字texture id当同App。无变化无LCMS、文件I/O、GPU创建、额外Vec或字符串分配。第一次query失败已消dirty，保持原不会每帧retry；真实下一身份变化可重查。

查询成功发送精确路径到当前World已有Events；缺事件owner明确Err。查询失败warn且不发event/不碰Active资源；核心收到坏路径则先清旧active后解析，二者不可混淆。Monitor None无事件。

非Windows没有Win32来源；Mac实际sRGB swapchain/MoltenVK设置CAMetalLayer并由系统处理显示器转换，保独立平台合同和可选真实身份诊断。不在OS不存在路径返回默认ICC/假事件/伪查询成功。跨平台核心显式ICC事件完整支持，Mac不自动重复display ICC。

原dirty setter具体callback分支仍由窗口owner有界核；本包只恢复已证initial/monitor identity变更与新App命名空间必要刷新，不声称同一monitor配置文件热修改通知已恢复。后者若原有明确dirty事件须后续按证据增量。

## 验收与限制

3个纯测试：query Err无event且不换Active资源；query Ok精确路径只送当前World；缺event owner明确Err。NativeAPI/weak tracker实际Windows、Mac layer身份、core GPU finalpost均需root执行，不把pureevent测试当全链通过。未Cargo/GPU/OS。普通OS关闭、窗口0extent和异步取消源逐字保留。
