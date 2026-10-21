# 真实世界 ICC Some / 替换 / 取消 / final_post GPU 验收 V1

## 授权与增量

root授权在既有original_gameplay_chain.rs追加独立GPU测试。测试原文件（含原30帧全部阈值/采样、GPU timing、wire及Terrain精确三f32修正）作为完整前缀逐字保留。仅追加私有测试模块；既有Cargo dev-dependencies增加固定lcms2 5.6.0 static直接依赖，用于真实测试profile生成，目标Mac诊断依赖原样保留。

## 真实输入输出及验收

1. 复用既有boot_shipped_session，全App、已发行intro存档、原shader/生产真实Device/bridge。TimeUpdateStrategy::ManualDuration(0)只固定时间，完整系统继续更新。
2. 原warm_up_gameplay、accept warmup后真实默认None帧。原ScreenshotPurpose::MenuBackground采集final_post之后世界色彩。原remaining=3，四次finish最终read_bytes执行GPU写入同步、真实writer送回RGBA；不是伪CPU像素。
3. 固定LCMS2.15生成线性Rec709/D65 profile，真实事件→原Update→Active Some。完整真实world图执行，并通过actual receipt逻辑资源、backend token、Arc指针、Final blit sampled access证明当前native ICC由实际final_post读取。
4. 线性帧提交后，尚未finish截图/collect CPU frame owner时，真实发送已序列化sRGB替换事件。要求host旧slot已注销、新id不同，旧native Weak仍能升级（证明当前已提交未退役frame owner保活；不声称硬件尚未完成）。然后真实读回已绑定旧profile的线性帧，不把替换后的纹理冒作旧capture。
5. 新profile真实import/export declaration→compile→drop触发现cancel_prepared；Device提交序不增加，Active/hostowner保持。此为导入/取消owner闭包，不冒称整个final_post callback取消已动态覆盖。
6. 新profile完整world帧再次证明Final blit exact native sampled owner。采集baseline/linear/restored像素，相同尺寸/RGB MAD；新ICC专用门linear>10，restored<linear*0.25。不假设序列化sRGB逐位等同内存profile。可选TG_ICC_OUT在断言前写profile及三PNG，保实际失败图像。
7. 正常同步仅有限timeout的真实submission等待，不wait_idle。保留首Result后始终执行真实GPU完成、截图shutdown、save-thread shutdown、App/bridge/shader/context drop；再检查首错误及清理错误，旧native Weak最终必须失效。截图shutdown仅沿原显式shutdown路径，不能把它当逐帧同步。异常挂起仍由root外部时间边界处理。

## 原证与平台界限

原B update@0x14093d830（完整3290B）/grid@0x1413036c0（1690B）；final_post@0x14090d000/0x1409ce780；screenshot opportunity0x14031d140/finish0x14031ea20/readback0x14031f2e0/write0x1403206a0。既有ICC_CORE_PIPELINE_V1_MANIFEST和本工作native-pack绑定原RNE/PDB。测试是Mac真实backend工程验收；不是原Windows OS ICC查询验证，也不是CAMetalLayer scanout或全部尾链视觉完成证明。

## 执行边界

作者只静态读API与生成候选，未Cargo/编译/GPU。root独审后负责实际no-run、显式此test GPU运行，留PNG/日志。强MAD门必须按实际画面失败调查；不得为通过删除真实pass/放宽原30阈值或替换默认图。
