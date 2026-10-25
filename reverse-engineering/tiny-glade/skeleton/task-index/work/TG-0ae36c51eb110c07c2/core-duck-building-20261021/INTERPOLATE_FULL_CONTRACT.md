# 原完整楼梯插值、比较、采样历史生命周期合同（待独审）

## 身份与实际输入

B1407cf500/PDB437:5700完整8622B（native-pack/interpolate-full.asm.txt/json），manifest绑定RNE f507a6ce、PDB2d9686b8。C行为未独立验证。已有named C仅导航，原ASM/PDB governing。原1196B run_unsafe wrapper140004710/PDB2:34676已在interpolate-required-readonly/native-pack绑定：6资源依次 ResMut<Requests>、Res<GameTime>、Res<StairsVisualState>、Res<StairsRemovedSupports>、Res<DecoratorStorage>、Res<UiSignifierStream>。第6项body优化后无读取，现代仍保只读依赖；现生产_old/_new slices→bool是假面，替换为真实系统，无返回值。

GameTime+28 delta、rate9.0(bits41100000@142a7ba84)，原cf7dea调用time_continuous_lerp_factor140c90540/40B。原factor先翻delta符号、再乘rate、expf、1减，不钳/不拒非finite。必要canonical叶在TG1d8旧cloth委派candidate，after8011a64c，待独审root集成；不在stairs私造或用四参lerp(0,1)增加浮点操作。常量interpolate-required-readonly/constants.json绑定 width tolerance142a87790=1e-4(bits38d1b717)、motion squared tolerance142a87794=5e-5(bits3851b717)。

## 入口、旧表与段类型历史

先factor，再mem::take同Requests.previous.sampled_points旧表；在self留下同类型新表。原std RandomState/default/hasher内核复用锁定依赖，不自建哈希算法。先以本轮requests iter.map(segment identity,sub_segments.clone) extend同previous.sub_segments，不清旧表；typed fold141293420/356B在segment-history-native-pack，原key140959e70为packed(minID<<32|maxID)，不是dirty集合两u32 hash。使用clone/extend保持SmallVec原inline/spilled所有权；这只是有界游戏callback，不能复刻std/hashbrown。

依本轮Vec请求顺序逐项执行；arch_wall_supports不被此函数改动。loop末尾drop旧sampled表剩余条目，因此消失identity真实删除/释放四Vec；不能简单保旧map或只append。

## 每请求当前值与可缺元数据

先 zip request.curve.points(XZ)与heights，min长度，生成Vec<Vec3>(x,height,z)。原381B map from_iter1402bd1e0/PDB1503:25736完整pack已存；同地址另有ICF alias不能误归别模块。随后以segment第一个原ID四次optional storage.lookup_decorator_info：Info+28 bits（None→0），try_as_stair.extra+08 bits（None/非Stair→0），extra+10 byte（None→0），extra+0c bits（None→0），该顺序保留。try_as_stair直接检查原extra首判别<=1再取StairExtra，其余任意byte都None；现代读取应先用canonical info.extra_disc()<=1，再调用info.extra()/as_stair，不能对未知disc先调用会panic的enum parser。此处缺Info是明确原默认0分支，不能requiredpanic；不是给上游pointfold加fallback。

读真实StairsRemovedSupports.contains(StairGraphEdge)，结果为removed_support。形成StairSamplesMetadata {info_28:u32, extra_08:u32, extra_0c:u32, removed_support:bool, extra_10:u8}，按原rawbits而不是f32==（NaN payload/±0必须区分）。然后从taken旧sampled表remove_entry(packed identity)，同identity仅可消费一次；new表重复identity按真实insert替换/drop旧值。

## old有值时的插值

克隆old.points给真实 **Curve3XZ::try_new_from_points(points,true)**（Vec3 + CurveCoordSemantic2D，原1412699d0/PDB1506:10828/782B），只在原Ok分支插值；Err释放clone/old四Vec并进入fresh分支，不产生默认Curve。新Vec3长度由前述zip决定。循环0..min(request.curve.points.len,request.curve.points_u.len)，每i先required当前Vec3[i]，再required request.tangents[i]，保native索引panic先后。

若 old.points.len==request.curve.points.len，取old.points[i]、old.tangents[i]（依次真实bounds）；否则以旧 **XZ度量** Curve.get_coord_at_u(request.curve.points_u[i])求index,t，依次required旧points[index]/[index+1]、旧tangents[index]/[index+1]，按old+(next-old)*t线性插值。不能用均匀i比例，也不把两条不同len Vec直接zip截短。旧Curve3XZ坐标策略复用canonical，已知共享极端f32/提前finite域差异须root协调，不在本源另造Curve。

位置 current[i]=old+(desired-old)*factor。dirty-motion比较使用**尚未乘factor的desired-old**，平方顺序(dx²+dy²)+dz²，原cf...ff55..6f有序严格 `deltaSquared > 5e-5`，NaN false，逐项OR，不能比较插值后微小delta或加clamp。

切向先 dot(desired_tangent,old_tangent)<0（有序）才逐位翻old两分量符号；然后old+(desired-old)*factor，按1/sqrt(x²+y²) finite且>0门归一，失败取**Vec2::X**，不是ZERO。原cfe74 SETA、cff24合并finite门、cff46 ANDN XMM11 fallback（cf926只loadss1，其他0）已绑定。修改request.tangents[i]，不修改old.tangents；desired当前Vec3被原地插值，old.points不被写。

## 两个同Vec3、不同坐标语义的曲线

原旧历史比较用try1412699d0，是Vec3/CurveCoordSemantic2D（水平XZ长度）；写request.interpolated_curve的try141269ce0是Vec3/CurveCoordSemantic3D（真实XYZ长度），PDB1506:13576/702B。native get_coord141269710同地址存在多种ICF alias，不能据alias把两个constructor当同type。现代必须 oldCurve:Curve3XZ，output:Option<Curve3>；否则旧height很大时按new二维u重采会走错segment。此原类型由真实两callee独立锁定，canonical Params OptionCurve3保持正确，不改owner/不加镜像。

## reuse精确门与几何选择（纠正早期checkpoint）

须全部满足：无motion超阈值；removed_support相同；extra_10 byte相同；extra_0c bits、extra_08 bits、info_28 bits相同；request.widths.len==old.widths.len；每个abs(current-old width)不满足ordered >1e-4（NaN比较false，不人为拒）；然后canonical USE_STAIRS_LAZINESS==true（原byte恰1），同StairsVisualState.get(twoU32 edge)命中且force_rebuild_requested()==false。缺visual或global关/force1都fresh，不能建default entry或常false绕过。原slot14362c018→14362c010 init1与visual+100三producer/两清0由原独审合同绑定；实际invalidate宿主仍另闭，不因此宣称mounted。

**reuse的Option<Curve3>输入不是总用当前插值points**：原d0756比较old.points.len与当前Vec3.len；相等时d075b..07e0克隆old.points（RSI源old.ptr）给Curve3Try(true)，并随后释放临时当前Vec3；不等时d078e..07ac接管当前已插值Vec3给Curve3Try。两者成功Some、Err保持None，覆盖/drop request.interpolated_curve旧值。原独立clone旧points已用于比较的旧Curve也drop。然后把原old SampledStairsPoints整个value（四Vec/原metadata）重插新表，保持第四opaque Vec与所有旧值，**不清dirty**：原本请求dirty若已true仍true。

以上旧/新Vec选择替代INTERPOLATE_READONLY_CHECKPOINT.md中“reuse总用current插值points”的未闭概述；不修改历史断点而以此正式合同优先。完整原d0756..08f3可复核。

## fresh分支

old无值、旧Curve无效或任一比较/visual/global门失败，都使用当前Vec3（如旧CurveOk则已经插值），先clone points、clone request.tangents、clone request.widths，第四历史Vec **empty Vec<u32> opaque words**，附本轮真实metadata，insert到新的previous.sampled_points。原fresh显式初始化第四Vec空，不从Params.auxiliary_samples(Vec<f32>)复制或擅改类型。

再把当前Vec3本体移入Curve3Try(true)，成功Some/Err None，覆盖/drop原request.interpolated_curve；最后request.dirty=true。CurveTry Err在这里仍有真实fresh sampled history和dirty副作用，不能return跳过整请求。上游heights/widths不被此系统插值或改写；terrain_contacts/曲线XZ/sub_segments本值/alignments/adjusted_edges均不被改动。

Rust RAII负责上述所有old Curve/new Vec/替换value/未消费旧表释放，不复制原手写析构和容量算术。原new map哈希和分配行为由pinned依赖reuse，不造镜像cache。

## 验收计划与边界

真实同World/authority tests需覆盖：首次fresh+消失edge删除；元数据缺Info零/非Stair；raw ±0/NaNpayload改变dirty；old同数/异数按实际CurveU重采；切向反号/零归一fallbackX；运动平方阈值严格边界与factor原dt；width NaN ordered比较；lazy missing/globaloff/force1分支；reuse同数取old几何、异数取插值几何；第四opaque保留vsfresh清空；fresh CurveTryErr仍history+dirty；旧dirty不被reuse清除。不得为了测试上游尚未完成而默认填Params。

本合同待parent独审才写完整source；只factor40B因root明确窄委派已独立候选。没跑Cargo/rustc/原target，不把源码/pack/F0当runtime或C验证。原两62KB mesh、完整cache和宿主caller未知继续分别登记。

## 共享Curve可缺域补充

原old Curve3XZ782B累长仅读点+0/+8 XZ，141269a67..b0c；没有Y有限性前门。例如不同XZ且NaN的Y，旧XZ参数仍可有效，后续true3D输出可以Err。现canonical Curve::parameters_for_points先point.is_finite提前Err会把原reuse变fresh/dirty，并丢第四旧历史Vec。已报告root TG9cab必要policy，同源不绕私有Curve；这一边界不因输入通常来自有限cache而被宣称等价。
