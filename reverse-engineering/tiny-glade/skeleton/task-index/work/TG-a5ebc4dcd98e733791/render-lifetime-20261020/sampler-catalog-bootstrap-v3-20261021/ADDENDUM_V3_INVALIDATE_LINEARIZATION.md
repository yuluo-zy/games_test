# V3 invalidate 最终线性化补充

V2 实际组先 finish 的设计合同继续有效，V2 初版源码因独立审查发现并发 stamp 缺口不得整合。V3 保留第一次 epoch++/清 stamp，逐项移出同名 lease 并锁外 Drop；当一次同 cache 锁确认已无该 name 时，再在同一锁内 epoch++/清 stamp。故在删除间隙捕获新 epoch 的准备也不能在最终删除后留下错误完整 stamp；最终检查后新准备看到真实空项并可合法重新补齐。u64::MAX 仍永久禁 stamp 不环绕。

begin_invalidation / finish_invalidation 是现私有页操作的提取，生产 invalidate 连续调用二者；新增精确测试直接在两步间插入实际 prepare/materialize/extras/commit，复现「新epoch + 尚未移除项」后错误 stamp 的时序。测试不是任意模拟新业务，不新增生产运行开关/等待/锁序。旧 owner 仍全部锁外 Drop。

V3 同时恢复 V2 作者机械改动误触的两处 RenderPassEntry table.push 原 Arc 包装，最终 patch 不触该表或 root create_buffer 策略。V3 是唯一请求整合候选。
