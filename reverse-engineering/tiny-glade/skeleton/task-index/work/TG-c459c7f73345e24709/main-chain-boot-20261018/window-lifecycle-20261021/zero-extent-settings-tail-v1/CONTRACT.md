# 零尺寸原公共设置尾与资源身份 V1

root 2026-10-21 明确授权完整 main-loop 必要闭包；本包只恢复已有原 B 零尺寸分支所漏尾段，合法 TG-c459 work continuation，不改原 claim。保现 zero-close 主线程终止适配优先，不因关闭多应用设置或启动 App。

原证复用 round13/frame/zero-extent 的三个真实 RNE 有界 asm：Resize发布 FrameRenderConfig.extent_0（1404dad4f/51）；zero门真实输入为相同资源（1404dc863..960），任一零时 sleep(1ms) 后 dc971 跳公共 de84d，随后 de8c5..de938 取设置 flags。当前 WindowSize 门混了资源身份，且 sleep 后直接 return 使设置尾永不消费。最小反例是 extent_0=[0,720]、WindowSize非零，或者任一零尺寸且待处理显示设置已挂起；当前会错误模拟或永远留 flags。

候选复用历史已取证 helper 语义读取必需 FrameRenderConfig，不增加资源默认值。普通非零路径时序不动。zero-close立即退出的现主线程适配不动。其余zero sleep后只调用已有apply_pending_video_settings；没有App update/egui/输入clear/帧推进/render/FPS。错误直接原样传播给runner cleanup。

新增一个实际资源身份回归故意让WindowSize与FrameRenderConfig矛盾，锁width0/height0/both0及恢复后门，不以复制条件测试宣称OS生命周期通过。真正零尺寸+设置恢复/OS尺寸变化/Swapchain generation还需root真实harness验证。原正常frame dc980后的Adaptive/VRAM/分辨率生成另包完整恢复；本包不先删除目前extent8/10的兼容写，防止留下无producer。

来源限定B，不能外推索引C/A。生产/Cargo/GPU无写无执行；实际编译与验证仅root。
