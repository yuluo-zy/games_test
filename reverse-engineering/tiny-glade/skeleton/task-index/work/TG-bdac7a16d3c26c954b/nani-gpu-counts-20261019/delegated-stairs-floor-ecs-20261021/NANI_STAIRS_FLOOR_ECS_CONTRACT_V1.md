# 玩家楼梯 floor 真 Nani ECS 消费者合同

此为当前楼梯同链必要 direct consumer，不新模块/registry。TG-bdac7a16d3c26c954b owner nani-gpu-counts-20261019 保原claim，lead唯一candidatewriter，原legalwork delegated-stairs-floor-ecs-20261021；生产/Cargo/GPU只有root写/跑。基线依赖已独审Gothic canonical Nani afteraf8faaba3d5d373136f2e33af1668d87360500f16a973485a6d5b25473efced0；新增叶必须以此精确增量，不覆盖早先两砖系统。Tree visualafter262f提供唯一真实StairsFloorMeshChunk/instance_words/Arc<StairsMeshBuffer>，不复制组件。

## 完整原证

B RNE f507a6ce/PDB2d9686b8。原14207fcc0/2430B，PDB module3098 record240364，native-pack stairs-floor-full 两者身份绑定，637条完整instructions，collector单请求ok。EXACT_QUERY_AND_PDB_RANGE.json记录原skeleton/B systems.rs16429精确函数指针：Query<(&NaniActiveInstance,&StairsFloorMeshChunk),(With<Marker<StairsFloorInstanceData>>,Or<(Changed<StairsFloorMeshChunk>,Changed<NaniActiveInstance>)>)>，另Res<GpuWorld>。原ff21 kind0x1a先调用owned_subset_mut，空query也取锁；之后遍历Bevy ChangedOR domain（标准query内核用pin0.16，不抄迭代器）。原803d5..3e8逐字复制chunk+8的9word，原3eb/3f3末3word置0；原3fa..401 position三分量0，xmm6=1作sortkey，41d真实update调用，循环完才放subset锁。

## 现架构完整调用

新增正式 nani_update_stairs_floor_ecs，Query同原With+ChangedOR，数据使用&mut InstanceKey仅沿已有sameWorld adapter进行dense-index hint修复，并对key写用bypass_change_detection，不制造Changed<Key>。不要求Transform/descriptor出现在updateQuery，不把它混成已有通用Row的TRS系统；按同Owned subset26取一次锁，再每行调已恢复build_stairs_floor(chunk.instance_words())→write_mapped_update→StairsFloorInstanceData。9words按Tree同组件逐位输出，保NaN/负零和地址高低字，不默认0、不忽略Info.u28。原profile/query framework复用现Nani family宿主政策，不把依赖迭代器/trace胶水当新增业务算法。

当前maintain的宿主completeness guard增加唯一15th marker与Has<StairsFloorMeshChunk>：kind26必须真chunk和真marker，保单marker约束；创建/descriptor存续时仍要求现maintain原triplet的Transform，不从updateQuery虚构变换。两者门不同：update本身不需要Transform。无missingpayload fallback、无parallel keys。add_mapped_nani_instance_systems将该函数加同一无序updates tuple，维持maintain→ApplyDeferred→updates→flush原宿主链，不额外run_if。startup_nani已注册真实Owned subset26，沿同GpuWorld/flush/remove路径。

有意义tests：实际World+真实MeshAtlasLibrary+Owned subset26、由实际NaniBundle/Tree FloorChunk创建key，核9word原输入+3zeros、positionZERO/sort1；clear_trackers+flush后未Changed两者应无data dirty，分别改chunk或标key Changed均更新，再证明hint修复bypass不伪造Changed。去掉descriptor/Transform而保marker后真实Changedchunk仍在原update domain；missingchunk/marker/ambiguous在maintain发布前失败；空query仍要求Owned26；despawn释放唯一sameWorld实例、chunkArc clone drop次数由Tree原本体测试覆盖。测试用已有CPUdevice边界，不冒真实GPU。

## 严格不完成声明与实际 GPU 前提

FloorChunk首Arc持真core Buffer、Tree RAII最后owner会回同BufferHost.release；仅更新9word BDA不能证明在途使用安全。真实draw还必须把这个Buffer经同host解析为ManagedBuffer并导入本帧used-resources/lease，root当前尾部唯一writer负责证明；可经FloorChunk.buffer.buffer()取代际句柄，不新增地址镜像registry。该叶的实际CPU通过也不能越过GPU lease门或挂尚未完整的genfloors/two62KBmain/railings/Info63宿主。整个渲染链能否呈现须root原资产+真GPU场景验证。
