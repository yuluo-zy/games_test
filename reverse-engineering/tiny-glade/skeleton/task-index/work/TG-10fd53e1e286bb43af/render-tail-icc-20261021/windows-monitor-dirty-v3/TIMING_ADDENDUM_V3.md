# V3 不可变时序补充（替代 V2 的非零门后发布位置）

独审进一步确认：原dirty查询在run_event_loop OS批次尾，独立于game-loop零尺寸门，不能延迟到窗口恢复。V2全部原Moved/cache/rate/Weak/错误合同保留；仅修当前主线程批次hook位置。

当前frame已完成handoff、flush outgoing、dispatch捕获OS批次之后，读取一次现canonical FrameRenderConfig extent0；先保既有工程zero+Close立即返回false（不用查询阻碍关闭），其余批次立即publish_pending消费真实dirty，然后若zero沿旧sleep1ms+apply_pending_video_settings返回。非零仍在真实App.update之前发布。

OS query只GetICM返回路径并发已有Events，不执行核心Update/LCMS/GPU；zero时事件在World真实排队，直到下一实际Update消费。查询失败在zero当时消费dirty并warn，不推迟到恢复；同一batch App命名空间变化另列工程刷新。

zero+Close优先是当前OS主线程生命周期适配，非原双线程mpsc物理布局；不声称此异常关闭路径仍先query。普通close/nonzero原系统语义不改。V2冻结包保留，V3三源按最窄hunk整合；其他作者Adaptive/Resize/settings候选勿whole覆盖。

root执行验证待做；作者未Cargo/CPU/GPU。必要已有zero identity、zero-close及13纯ICC来源测试与真实WSI归root。
