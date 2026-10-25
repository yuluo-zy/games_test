# 水声活动格候选独立复核

复核者：core_business_lead。已直接核对当前生产 `Raycasting`/`RaycastWorld` 接口、water 候选、原 `1408cc320` 完整字节与调用点 `141baf494..141baf4a5`。未运行编译、测试、原程序或 GPU。

候选的实际修复成立：生产宿主只安装一个 `RaycastWorld`，原借用参数不能被改成没有 producer 的独占 `Res<Raycasting>`。原调用传入 sample 起点、eye-sample 方向、max_toi=1.0、mask=0x32a3；同一 bound 在原 callee 同时传给地形与 Rapier，地形命中即短路。当前 f32::MAX 丢失该有限线段，因此相机后方碰撞体会误遮挡。候选借用真实 owner 并显式保留 bound，不制造第二物理世界。

保持已有100格/64位/i16滚动算法，修正旧注释的8格、Local镜像和260采样窗口误述。三项真实 World 测试有针对性，覆盖64帧滚动和清空、相机后方遮挡反例、阈值/NaN；均待 root 执行。`world-bounded-proposal.patch` 是跨 owner 的最窄建议，不能当成水 worker 已获 world 写权。

允许 root 配对整合 water 候选与经 world owner 复核的方法 hunk；water 单源 before=`db55e5c6584f68ab054543ca0fdb356524eea421862237798333961d30ed88b4`，最终 after=`18b95d290f625cf7a9486b851f49382312303242b363fb2f7f7140fe8faa4878`。首次交付后的唯一修订是测试 fixture 显式把地形设为 -2.5，防止把默认数据误当空地。helper 使用现 native-physics 能力，根代理需使 system-water 单包 feature 与实际 App 能力一致，不添加空返回分支。

音频 Startup/Update 注册由 root 接线。demo 全局指针身份另存原证，当前候选不无证变更门控。上述工作不代表 FMOD 实机或 C 版本验收通过。
