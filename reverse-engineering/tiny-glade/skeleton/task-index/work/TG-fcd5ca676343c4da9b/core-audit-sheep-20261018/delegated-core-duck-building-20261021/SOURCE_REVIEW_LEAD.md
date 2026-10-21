# 核心业务负责人独立源码审查：Duck audio/planner v2 + set_parameter leaf

审查人：core_business_lead，2026-10-21。结论：本次有界源码候选通过，可交 root 成组整合与实际 Cargo/运行验证；当前状态仍 implemented_unverified。审查不等价于 A/C 或原程序运行验收。

## 冻结输入

- audio.rs：before 2c8eed6ab3be8ee09dabe2586f169fb0a072b871cab214ee2ce2540da1e7dd45；after 74b461adbb01042d25067115de3f2af73b350d382c0be11b36fbf75632af1c1a。
- duck.rs：before 2c8de6382e69362bb97ef70f26e7a4a71b57000cd40602251805568a50f3a6f4；after 955a0737371a4d2d14d957d7d87457abbd768e10b6d85425ed268c877dd6e2f8。
- audio_resources.rs：before 0c404ca50c1d0f1f21b60472dfd9a50a01897f2a84afd39ac15e0104761164d4；after 0b84223d757be5fa383211695f9f8ce20c0a49ed29cfc6ee764eedf951db00b3，仅 set_parameter 方法。

已重新读取三份最终 diff、delivery.json、原身份绑定合同、drain/RNG/复制合同、参数 formatter 和错误调用者盘点。三份 after 文件实算 SHA-256 与冻结清单一致。

## 独立核对

1. Duck::new 构造按原 Quack→raw Pitch→TakeOff→Land→四实例 Flap 环。实例创建失败 panic 并由现 RAII 释放成功前缀；Pitch 参数失败进入原 warn-once 后继续。Flap 复用 canonical AudioLoop 的实时钟、当前实例、轮换与延迟停止，未把单次事件当环，也没有复制第二套环算法。
2. 真实 planner 对外 Vec 使用 drain(..)，保留原 allocation；逐条先抽 Pitch 并完成构造，再 extend_from_slice+reverse，随后第二次 uniform_f32 乘原 -0.3 写出生进度。原空点轨迹合法，未额外跳过或伪造首点。错误的 base→phase 写入已移除。此范围不宣称其余规划几何或姿态已等价。
3. 移动消费使用唯一 WaterAudioGrid 及既有 sampler；Land/TakeOff 的门与原相符，Flap 只消费原 landing_t/is_taking_off。MAXSS 零/NaN 行为及 progress unordered 进入 stop 的分支保留。已有 grid sampler 已由 root 合入并测试，候选不造第二 owner。
4. set_parameter 完整原 1838B 证明使用 log-once 0.3.1 的原宏静态表及 Debug formatter；非 GLOBAL_AUDIO_PARAM_ERRORS。精确文案、参数顺序、插入先于日志级别判断由原锁定依赖复用。当前 Result API 保留 Err，真实调用链均 discard 后继续；作者的 62 处盘点含多行调用、0 直接 unwrap/expect，未把 AudioCore/global API 混同实例 API。
5. 两个 root-host-only hunk 复用 Native 同一 AudioCore 或 Disabled 的真实 None Studio。必须与工厂一同整合，否则 mandatory factory 按设计失败。其他 critter 的旧 instantiate 接口保留；该包不宣称其他动物音频已恢复。
6. 七项构造/前缀失败/参数失败/禁用/缺工厂/原环/真实 planner→同鸭→建筑事件测试，加一项实际 move ECS 测试，覆盖真实 producer/consumer。测试尚未由本 worker 运行；rustfmt parse 不能替代类型检查。测试使用 pin Bevy 0.16 的 Events::send。

## 整合与验收条件

- root 一次应用最终 v2 audio.rs + duck.rs + set_parameter 窄方法，保留 root 同源 DisabledStudio 新增代码。
- root workspace/country-core 添加 exact log-once =0.3.1 并更新 lock；原校验 aba6f00890254f37df9778e2df6042249f543ab1789cbe001c50b4debe264a8c，已保留一致发布包。
- root 把工厂接入真实 Native/Disabled 宿主；root-host-factory-only.patch 为两个最窄 hunk，宿主变化时重定位，不覆盖整个源。
- 运行本包 tests、实际 App 计划/移动装配及无音频分支；保留失败日志。用户 dirty startup_sheep.rs 不动。
- 不能据此宣告 duck 整个规划/Quat/删除历史全部恢复；旧 trajectory/姿态/retention 差异仍由原 duck.rs 注释明确保留，不以本窄闭包遮盖。
