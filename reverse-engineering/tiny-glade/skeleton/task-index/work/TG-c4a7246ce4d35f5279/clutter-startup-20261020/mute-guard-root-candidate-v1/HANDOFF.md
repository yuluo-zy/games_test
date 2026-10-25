# 原单位静音事件入口门

原证复用 preview task / mute-reader-20261020 完整 READER-CONTRACT-V1 与 typed reader addendum；preview140822e00/2144B、stairs14081d9b0/18411B，真实独立 MutePreviewDecoratorOpSounds EventReader 已锁。保持原 owner/claim，不写生产。

只有入口门：定义单位事件在原 preview 模块，两个实际系统最先消费全部本 reader 未读 mutes 后返回；不触及普通操作 reader、全局 Events 或音频帧状态。不只 next 一条、不清主事件，不造取消ID队列。其余旧音频业务缺口明确保留，不因此注册整套 audio/旧近似 installer。

必须配对 preview+stairs，并在实际 App 唯一注册 Events<MutePreviewDecoratorOpSounds>。TG69 validate 真实拒绝且 backup.created 时设置 state2 后只发一个单位事件。root 唯一生产整合和运行验证。两个 actual-system schedule 源码测试在 preview：三通知只跳一次、两 reader 独立、普通 op 下次仍消费、原音频状态不被改、正常 Events 两次 update 后过期。仅 rustfmt parse/格式化；未 Cargo/rustc/GPU，测试未跑。
