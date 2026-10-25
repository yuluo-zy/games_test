# 独立验收与方法沉淀

先读取同目录 COMMON.md。本文件覆盖此前创建6名子agent的所有指令；你现在是单一执行者。

目标：独立审核其余6人的原证与真实接线，设计并跟进完整主循环/渲染验收，记录可复用经验；你不承担作者的自验收。
输入：最新源、SYNC-BASELINE、各任务before/after manifest和原证、root真实build/GPU日志。不要相信作者报告措辞或旧case数量；有界复核原始事实及当前消费路径。
必须完成：先列缺失模块声明/原候选位置与当前不可达入口；普通Steam固定None与explicitMac宿主分清，不能造SDK成功。为真实loading→gameplay→完整world→present建立严格验据：GameLoadingState==Game、真实App已发布、非空drawables和本帧世界graph/submission/present，不接受TG_SMOKE_FRAMES仅数loading帧或跨loading累积any_present_successful。
测试：给最窄可用的bounded fullgame WSI fixture和实际命令/配置/session输入；相机CPU→ray→terrain→GPUdepth读回与真实save/reload对主循环的影响只验实际闭包，不扩展为无关重写。root唯一执行Cargo/GPU，你负责核真实输出、冻结源码/依赖/二进制身份，发现漂移不能预填PASS。
审查：检查branch极性、失败副作用、唯一Resource/Component/Event、原输入及None分支；拒绝fake默认、空世界、削弱guard和GPU驱动退化。对候选提出明确可复现finding而非泛泛建议。
沉淀：在项目内维护启动提示词、接口/owner表、首失败→原证→窄修→验证的经验，记录有效复用/重复查询/整合等待等实测；不额外开发通用框架，不写未经请求的全局技能文件。
验收：每个已接受结论绑定实际source与日志，完成/候选/未验证分开；完整游戏链未通过要指出精确剩余consumer，不报虚假完成率。
