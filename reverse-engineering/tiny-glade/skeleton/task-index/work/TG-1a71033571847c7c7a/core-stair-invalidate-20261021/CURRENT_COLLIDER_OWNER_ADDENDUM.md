# 当前物理网格强 owner 适配勘误

绑定 IMPLEMENTATION_CONTRACT_V1.md SHA e5610fa8e3dfcf832c03eb3ddd5c401fd6d1e5ad163add333ac24057eab8cc26，旧合同保持冻结。

原 remove/retain 返回或持有的网格 Arc 析构行为，在当前手工重构架构中由唯一 RaycastWorld.mesh_sources 实现。实际 ColliderMeshInstance 是 0x18B Copy 句柄记录（generation/index/collider/slot），本身没有 Arc；不能把 V1 的“ownedArc”误读为当前 instance 的字段，更不能 clone/dummy drop 这个 Copy record 假冒释放。

current world::dispose_collider_mesh_instance 在真实Rapier删除和generation校验之后、free-list更新之前，从同一个 mesh_sources.remove(strict_mesh_key(raw)) 取得真实 RaycastMesh；该局部在函数结束释放内层 Arc。remove_edge_colliders依0/1/2调用相同 dispose，第四表retain命中时直接dispose然后false；不改变 world 内部已审时序，不在invalidate创建另一owner/Arc。现代位置与原布局不同属于既有所有权适配，真实物理/receipt/Arc销毁副作用由同owner完成。

测试以 Arc::downgrade 和真实 RaycastWorld receipt/registry为准，不以Drop Copy或虚拟计数作为释放证明。
