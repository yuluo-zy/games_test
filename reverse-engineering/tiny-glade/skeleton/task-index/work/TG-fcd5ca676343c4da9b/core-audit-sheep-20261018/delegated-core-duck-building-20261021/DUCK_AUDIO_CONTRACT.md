# Duck::new 音频闭包合同（中央委派，等待接口裁定）

live claim维持TG-fcd5ca676343c4da9b/core-audit-sheep-20261018（原country-core/resources/critters.rs）与TG-fef7f4493739b5bc2d/render-surface-world-20261018（当前critter/duck.rs）。delegation.json记录root经core_business_lead的最窄委派；只产本子目录候选。

原B Duck::new0x140f85bb0，PDBmodule1175 record5652 code_size1920。原字节完整pack在TG-ec39e67771c9fa33c3/core-duck-building-20261021/native-evidence-v3（ok），字符串与常量bounded-constant-data.json绑定同RNE SHA。Flap与frame参数原数据已保存在TG-fef7旧work/delegated-core-duck-building-20261021/bounded-flap-data.json；plan原实参31B与move音频455B块均在同目录native-audio-callsite，原字节核验ok。A/C未验证，不将完整Duck运动宣称完成。

## 构造完整顺序

1. plan_duck_trajectories 每条key==0轨迹临出生先调用一次PerchanceContext.uniform_f32作为Pitch（复制完成后还有独立的出生progress抽样，见下），然后原寄存器XMM0原样movaps→XMM2传Duck::new。无乘加、无clamp、无额外随机抽样，Pitch范围由原uniform_f32给出[0,1)。构造异常之后不得再消耗后续轨迹随机数。
2. Quack event:/SFX/Ducks/Quack（VA142bad218，22B）instantiate；原AudioCore内为空时创建Disabled；有Core则get_event/create_instance，Err立即unwrap panic，不继续创建后缀。成功后立即set_parameter("Pitch",原传入f32)，参数名VA142bad278为大写P。原AudioInstance::set_parameter0x140a57670失败执行全局错误map去重后Warn并继续；原格式字符串已核为`Failed to set parameter named {parameter} on FMOD event {event_name}\n{error}`，见instance-parameter-warning-format.json。当前core方法仅返回Err、blanket适配静默discard，此公共依赖需root补统一原report入口；本adapter不能另建局部去重镜像。
3. TakeOff event:/SFX/Ducks/TakeOff（142bad27d，24B）instantiate，同样Err即panic。
4. Land event:/SFX/Ducks/Land（142bad2b0，21B）instantiate，同样Err即panic。
5. Flap不是上述三个one-shot的别名。原描述name=event:/SFX/Ducks/Flap（142bad2e0，21B），additional count=3，stop delay=50ms。调用AudioCore::instantiate_loop0x140a59220并unwrap Result，实际四个owned实例，按现core.try_instantiate_loop原版完整环构造/首实例loop参数probe/disabled分支，错误时前缀实例按RAII释放。不能用一实例或Vec<AudioHandle>冒充原AudioLoop。
6. 前三个实例存Duck+0 Quack、+0x30 TakeOff、+0x60 Land，Flap ring在+a8。原disabled没有Core也依次走同一三个事件和环描述，不是跳过整个构造；无FMOD宿主应注入真正AudioCore<DisabledStudio>{studio:None}。
7. 所有已创建实例由Duck持有并随实体生命周期drop；active原state初值2，不擅自stop_on_drop。后续构造失败同样drop前缀，不留漏句柄。

## 消费侧必须同步纠错

建筑反应只启动Quack（Duck+0）。原移动轨迹系统：正向落地条件(f_f4>0.3且state<=1)启动Land（+60）；反向起飞(f_f4>0.5且state==3)启动TakeOff（+30）；两者写真实位置，再把WaterAudioGrid水采样max0后sqrt作为音量。当前均错用quack并未采水音量。

Flap由独立AudioLoop管理：progress<0.01或progress>=0.99或state==2→stop；否则start_if_not_playing→current.set_position→current.set_parameter("landing_t",f_f4)→current.set_parameter("is_taking_off",state==3 ? 1.0 : 0.0)。原参数名VA14304aca0(9B)与14304aca9(13B)。不能继续让DuckAudio.start_loops启动Quack/TakeOff/Land，或让stop_loops立即停止三个one-shot。

## 当前架构最窄适配建议（需中央复核）

保留Duck/DuckAudio/AudioHandle手工重构。DuckAudio增加独立flap AudioHandle；私有泛型sink仅转发已有core AudioLoop<AudioInstance<E>>（其start/stop/current负责真实环逻辑），不重写环算法。创建入口通过真实AudioCore<S>构造三个AudioInstance及AudioLoop后擦除具体后端类型进入现句柄。测试用原Studio trait记录器，不mock掉AudioCore与环。

现CritterAudioCore.instantiate为Option<fn(&str)->AudioHandle>且从未插入，既无法捕获root实际共享音频core，也不能表达has_parameter/raw start的环构造。可在其已有文件里增加完整duck构造factory闭包，root用实际同engine的AudioCore共享句柄安装；plan依赖mandatory Res<CritterAudioCore>并调用factory，缺factory明确失败。另一方案泛型plan/插件会牵连共享lib/App，应由root裁定，不由worker擅自建新owner。

不允许把None callback或缺resource当Enabled静音返回，也不为成功测试切换Disabled路径。真实禁用来自AudioCore::studio=None，实际Native时必须可解析/实例化上述七个事件实例。

## 同World/实际owner验收

- 记录Studio验证创建顺序Quack→Pitch→TakeOff→Land→Flap×4→首Flap.loop probe；四环count/threshold、原参数值与没有start副作用。
- 第0..6创建失败分别验证panic、精确prefix release且无后缀调用；Pitch失败策略按原方法；loop probe失败按已有core原算法日志/continue策略。
- None Studio构造保留真实Disabled实例与四环状态，不调用任何假后端。
- key0真实planner产生Duck→真实构造factory→active句柄→建筑编辑事件→Quack；key!=0不创建新音频不额外抽Pitch随机。
- 移动Land/TakeOff与Flap参数/停止环轮换使用实际同Duck实体，不能用一次source-only测试宣布宿主已闭合。

当前尚未写跨源实现；接口/AudioCore共享factory安装由root协调，root唯一Cargo执行者。


## 必需生产闭包修正

见TG-fef7旧work/delegated-core-duck-building-20261021/SPAWN_DRAIN_CONTRACT.md。原外Vec::drain保cap，不是每帧借用重放，也不是mem::take；inner points原复制到Duck后释放，不move。每row还有constructor后第二uniform_f32*-0.3写Duck+ec出生progress；empty合法，phase三浮点全0，首目标timer0。候选与真实三row两tick同World测试已同步，不把单帧出生通过当长稳。
