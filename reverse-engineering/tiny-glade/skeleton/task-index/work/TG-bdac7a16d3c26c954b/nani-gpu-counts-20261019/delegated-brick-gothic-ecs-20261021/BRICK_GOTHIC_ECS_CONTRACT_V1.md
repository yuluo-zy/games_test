# 真实砖与哥特窗实例 producer→ECS→GPU 合同

仅当前 Gothic producer 必要的 canonical components 与 render/nani/systems 两源，我为唯一候选 writer。live claims TG-abf1305389c44cc264 owner meihost-20261018、TG-bdac7a16d3c26c954b owner nani-gpu-counts-20261019 保持不变；原 legal work delegated-brick-gothic-ecs-20261021 追加。root已明确同链 direct-consumer 委派，不写生产/Cargo。builder TG1298 由 decorator_chain 协调最窄重导出/插入转换，不建立第二 TypeId/第二 GpuWorld。

## 原范围与真实输入

B RNE f507a6ce/PDB2d9686b8。完整 native-pack solid-bricks-full 142076230/3084B、gothic-bricks-full 142076e40/3794B，原 706/815 条指令 collector 两请求 ok；已有 decomp 仅辅助。PDB Query 精确签名见 TG73d/scene-clutter-producer-20261020/gothic-main-chain-20261021/NANI_BRICK_CONSUMER_SOURCE_REFERENCES_V1.json：前者 (&NaniActiveInstance,&Transform,&BrickColor)+With<Marker<SolidVertexColorBricksInstanceData>>，后者再加 &GothicWindowBrickDeformation+对应 Gothic marker，另 Res<GpuWorld>。没有 Changed gate、没有额外 descriptor 条件。

实际 producer 合同见同目录 GOTHIC_PRODUCER_CONTRACT_V1.md：Brick6word=[Info.rawword2c,WallID.low32,(u-width*.5).bits,(width*.5+u).bits,width.bits,lowbytebool]；帽bool是半木，框固定0。Gothic主真实重排为 side4,edge2,first3,center3,second3；gethat返回却是 side4,first3,center3,second3,edge2，不得直接拷贝。这里消费已正确重排的主载荷，不复制 producer。

## canonical 类型与 builder adapter

放回原 mapped components/mod.rs，不新建生产文件：BrickColor 是保留六个 u32 原字的透明新类型，字段不据未知 PDB 名乱命名；公开 from_words/words、from_native_bytes([u8;24]) 固定小端逐字转换。第5word高24位保留，即使两当前 shader consumer 只使用其特定低位，也不擅 mask。GothicWindowBrickDeformation 是有意义的 side:[f32;4],edge:[f32;2],first/center/second:[f32;3] 字段，保持原主十五标量顺序/60B，不用 glam::Vec4 引入16对齐；from_words/words 与 from_native_bytes([u8;60]) 不做浮点运算，保持 NaN/负零 bits。不提供默认空几何或缺 payload 补值。

builder 原 [u8;24]/[u8;60] 字段接口保留用户架构，只把本地 pending raw component 换 canonical pub use，并 insert canonical::from_native_bytes(payload)。原 BrickColor 过滤 payload[20]!=2 分支不变（该字节是原 Option niche 判别），不新增过滤、校验或转换失败分支。canonical 是唯一 ECS 类型，公开旧 pending 导入路径只重导出同类型。

## 实际系统与所有权/失败顺序

复用已存在且按原恢复的 build_solid_vertex_color_bricks/build_gothic_window_bricks 数值与 half packing，不新增私有投影/重复公式。每系统先取得本 same-GpuWorld 的 Owned subset kind6/7 锁（空 Query 也取），后逐 Query 行把 transform/typed words 构造成现 InstanceUpdate，write_mapped_update 到原 OwnedGpuWorldSubset。原共享 key 是只读查询，其 index hint 修复沿现实际 ECS adapter &mut InstanceKey + bypass_change_detection，避免假 Changed；id/kind由同一 maintain 创建，不复制 key owner/row Vec 或逐实体 subset lock。

同一维护 preflight 增加两个 marker 与 canonical payload presence：kind6 require BrickColor；kind7 require BrickColor+Deformation；仍 require Transform、单一 mapped marker。该断言属于现宿主的 completeness 边界，不宣称原分支。缺真实输入在实例发布之前失败，不能默认0或跳过。实际 update Query 保原 With-only + required components，不加 descriptor/Changed。两函数追加同一个 add_mapped_nani_instance_systems 原无序update tuple，保持 maintain→显式 ApplyDeferred→updates→flush 的现宿主原序。启动原 owned kind6/7 已存在 startup_nani。flush/roll 复用现实现，不建立新 upload cache。实体卸载由既有 RemovedComponents<InstanceKey>→Local 同owner→GpuWorld.remove_instance 路径处理；不另建 registry。

## 验收与边界

有意义测试：两 typed 类型原大小/word和NaN负零无运算往返；real World+real MeshAtlasLibrary+two real Owned subsets、maintain/ApplyDeferred/updates 生成唯一 key，核明确原尾字与15f32重排；真实 payload 变更且无 Changed gate、descriptor移除后 With-only 查询、同 key hint修复不制造Changed；真实 flush dirty ranges/instance counts与实体despawn释放。缺每个输入/错误或歧义 marker 必须在发布前失败；空 Query 仍要求自己的 actual owned subset。必要 builder 测试证明插入的 canonical TypeId 可被该 Query消费。root实际 CPU/GPU唯一执行者。

两个 Nani 查询可调用/flush 不代表 Gothic producer 宿主已mount或原资产场景已可达；原全主/hat/helper、set10/25与unload由 decorator_chain 独立闭合，玩家楼梯与窗口生命周期/尾部仍各自验收，不按函数数量冒画面完成。
