# RT 实际命中才消费删除合同 V1

TG-81dc0cd75bc98816f4 旧 owner main-chain-frame-20261018 保留；root/lead 已明确授权本轮 invalidate direct-helper 窄候选，live task receipt 随包。原 mapped resources/rt/mod.rs 对应当前 resources/rt.rs；当前 before-rt.rs SHA 0ad785ddb22e24aac0d0874e19c417434d3cbfd5b4bcbbb13265e60516ad66b0。

原证唯一必要片段为完整 invalidate 140b37180/4290B 内 140b379dc..37a57，B RNE f507a6ce/PDB2d9686b8：构造tag1与packedidentity，真实 RtWorld.meshes.remove_entry，None无写dirty，Some先drop DirtyCow<RtMesh>的Arc然后 +38 topology_dirty=1。原删除为内联业务，不假称新增Rust方法同名存在原程序。

新增唯一 inherent public method `remove_mesh_if_present(&mut self, handle:&RtMeshHandle)->bool`：对同一个 self.meshes.remove；None returnfalse且不写任何字段；Some先 `drop(mesh)`，再 `self.topology_dirty=true`，returntrue。消费删除值而不是把Option交外部延迟Drop；不cloneArc，不借get_mut创建missing，不重建mesh表或改计数/pendingupload。

保留当前 remove_mesh/remove_meshes 完整字节语义（无条件先dirty再返回Option）；当前直接caller update_rt_world、prefab、bvh_builder 以及旧单元测试不改。新增方法仅 invalidate 消费，不能全局替换旧调用。

同文件4测试：missing默认不dirty且counter/pending/别tag保持；hit真实Arc最后强owner在返回前销毁且topologydirtytrue；同payload tag0/tag2与其它edge保留；已经dirty时missing不清dirty，并断言旧remove_mesh missing原无条件dirty仍保持。测试可读取本模块private topology_dirty与实际Arc/Weak；不跑Cargo，由root最终执行。

候选仅旧合法work追加，生产/其它RT方法/BVH算法/锁定依赖/注册不改。合同独审通过才写source；输出最窄patch、before/after、F0重放与parse，仍implemented_unverified。
