# M01 全局射线结果与即时放置 UV 链（2026-10-27）

本轮限定恢复既有 `world.rs` 的完整全局射线结果，复用上一轮五表/十二参 producer，以及已恢复的 canonical pose converter。未重建 World、renderer 或另一条 raycaster。

## 原件与完整范围

先重读 B 同路径 `country-core/src/systems/collision/world.rs`。原 PDB module496明确 `Rapier::raycast@1408cdc60` 物理786B（record44596）、`RaycastWorld::raycast@1408cc870`39B、`raycast_w_terrain@1408cd430`383B和转换器 `1408ce4a0`1398B。四段 RNE/分析EXE逐字相同；RNE SHA `f507a6…d03`，PDB SHA及每段 SHA见[身份记录](m01-global-ray-identity.json)。全部原 ASM按VA保存在同前缀文件，fresh完整C在[m01-global-ray-fresh.json](m01-global-ray-fresh.json)。procedure的void()仍是调试占位，实际隐藏结果/World/Ray/Option<WallId>/TOI/mask参数按原完整机器码与包装调用核定，不把候选签名冒充原Rust类型。

## 恢复的业务与边界

原17槽按mask遍历，每次有效命中收紧后续TOI，等距后槽覆盖。**循环内只保留winner，循环结束才校验最终physical/receipt并转换**。现在保留这个原顺序：更早但较远的坏网格被后来的近命中覆盖时，不提前读取其三角索引或错误地panic。

原global predicate1408cab80与winner1408cdeea都以`raw_generation | 1`校验真实注册表。恢复全局这一读取合同；显式`get_collider_mesh`及其它独立消费者仍保留其严格接口，不能根据全局的OR1规则放宽所有owner。原排墙位图0x105只含source0/2/8，验证receipt发生在同墙排除之前。

最终winner直接传其真实mesh、完整24B source、原TOI/Feature、物理Collider平移和旋转到canonical converter。三角网格Face正常插值UV/法线；反面Face索引按原扣一次三角数；非Face网格结果为miss2；primitive不要求三角/UV，保留无UV命中0。原正常mesh无UV还保留barycentric尾分量。primitive未写extra、miss未写其余union字段，验收只比较原活动字段，不能把Rust安全初始化视为原零填充。

`RaycastWorldResult::is_hit`修为0/1均成立。否则UV命中1会在terrain wrapper中被错误当作miss，允许更远地形覆盖近处屋顶，camera/walk使用同一helper也会漏命中。原terrain wrapper对两种hit都传已有TOI作为地形上限，真实地形返回point直接复制，不再重算丢掉该字段；既有地形求交算法与选择次序不变。

原转换器按法线/UV的Option指针是否0判断，不能用slice为空替代。获批准仅改两处presence条件，保留全部已验证数值体：None正常fallback/无UV；Some(empty)继续原bounds panic。原normal先于UV的读取和失败顺序保留。

按中央要求，增加现World上的薄接口 `raycast_simple_with_max_toi_excluding_wall_id`，直接调用既有simple查询，保留方向长度、TOI、mask及WallId排除；非native明确失败。未复制simple算法。

## 原执行与真实World证据

[有界原探针](m01-global-ray-native.py)执行原完整global、predicate、terrain wrapper、转换器及原barycentric。共82个global输入（含10个原panic）、204个predicate输入、16个terrain包装输入。**没有算术桥**；QueryPipeline候选相交、ColliderSet查询、Terrain服务返回和panic捕获是明确隔离边界。它不证明原QBVH/arena/terrainmarch自身实现，与真实Rust依赖验证分开记录。

新增8项真实测试包括：

- 70组原global活动word，对真实Rapier World的全部17槽、mask、最近/等距后槽、yaw姿态/平移、primitive、缺法线/UV逐位对照；mesh活动16字，primitive不比较原未写extra。
- 204组原predicate，对真实World验证所有source0..16、None/同墙/异墙及raw世代0..3；有UV命中也正确判为hit。
- 75组原最终Feature/bounds路径复用同一canonical converter，涵盖反面、非Face、坏索引和Some(empty)。这些resolved数值/失败样本与70组有重叠，不是覆盖率分母。
- 原16组terrain wrapper确认UV/无UV均限距、等距可覆盖、mask分支、显式caller上限、真实point字段复制。
- 真完整十二参producer中的两屋顶→全局最近UV→近/等/远真实Terrain合并→Added删除/despawn/Arc释放；不会为了避开完整App其它未恢复模块而删除App系统。
- 真物理世界中，较远早槽坏索引不在近winner之前提前读取；真正选中坏数据才失败。mask未选时不读错误receipt；错误index/旧世代失败；退役释放；复用后的新偶raw世代仍仅OR1到正确新世代。
- 薄simple适配器在非单位方向下保留最大TOI和同墙排除。

最终`world`组 **71/71**，屋顶同文件组 **25/25**，其中原完整转换器259组逐word数值回归通过。命令、最终日志与来源哈希见[最终收据](m01-global-ray-validation.json)。不将这些重叠组数相加当作游戏还原百分比。

## 保留的失败与修正

首次旧global真实测试错误断言偶raw必须失败；原predicate与winner的OR1指令及新204组反证后，修为全局归一化成功，显式receipt getter仍严格。首次terrain夹具把默认高度范围误当0，实际默认min为-2.5；改为真实`set_height_range(0,0)`明确测试平面，没有改业务迎合断言。

屋顶旧World fixture的`uv=false`产生空Vec，而真实Roof producer始终传`Some(volume.uvs())`：这属于Some(empty)，不是None。新原边界使旧期望NoUv码4失效，按批准更正该fixture为bounds panic并改测试名。真正None的NoUv码4仍由原完整屋顶控制流22组证明；本轮真实全局None-UV网格亦按原命中0验证，未将生产空UV吞成None。259组数值体没有改变。共享Focus旧测试迁移曾使屋顶回归编译失败，待owner收敛后完整重跑25/25；失败日志保留。

## 性能与剩余范围

保留现17槽原遍历和Rapier QueryPipeline；仅最终winner转换一次，不克隆mesh/Arc、构造临时物理世界或分配每帧Vec。独立simple、spherecast、all-intersections族没有借此宣称全验，保留其各自既有实现与后续边界审查。原射线服务/SDK本体未直接在完整Windows游戏中运行；生产即时Place上层和主窗口整游戏验收由中央继续。冻结的十二参main与五实体map业务体未改动。
