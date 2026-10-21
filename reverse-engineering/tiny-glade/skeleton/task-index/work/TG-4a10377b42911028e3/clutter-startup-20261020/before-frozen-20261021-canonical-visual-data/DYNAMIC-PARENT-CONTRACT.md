# 农舍老虎窗 canonical 动态碰撞消费者补充合同

本合同续接本 work 的十源授权、冻结 before 与原完整 B body；只改本 mapped parent，collision_geo 的 TG-afa181a144d4516c69 由 water worker 唯一写。原 owner clutter-startup-20261020 保持。root 唯一生产/Cargo/GPU 执行者。

## 原输入及先后次序

原完整 B1407c7ac0/11257B，集中证据 `TG-dfec84bcd0f1e9c7c5/core-visual-data-20261021/before-frozen-20261021-canonical-visual-data/native-pack/tg-4a10377b42911028e3.{json,asm.txt}`，B RNE/PDB SHA 与该 manifest 身份一致。外圈实际 wall/roof 不存在则跳过、Roof::is_flat 为 true 跳过。7c8190 required ID subtype，7c819f 比较 subtype==2；非本 subtype 不消费 cache/尺寸。7c81c4 required CachedDecoratorTransforms.transform(id)。7c81d3..81eb 对 Info.rank 做 dec/cmp3，rank1..3以 (rank-1) 索引 f32 原表142a87238；原表 bytes `cdcc4c3f0000c03fcdcc0c40`，即 width[0.8,1.5,2.2]。不可读取 loader Sphere 的 bbox 替代，不使用 rank0 fallback。

7c8229 是原蓝图 owner 入口，紧接7c8238..8266调用唯一 helper14128a0b0，参数为：&cached transform、width、height=1.0、rank、padding=0.2、&真实 Roof、Local collision cache。helper 按 water 取证后的真实 API/量化 key/convex 构造实现；parent 不复制采样算法、构造新 key 或另造 global registry。7c826b clone 返回 Arc 一次，82af 的两个 slot 接收同一 outer Arc；world pose 为 cached.translation 和 cached.rotation，不乘 cached.scale。

重要校正：7c8190..82af 的完整原控制流只有 subtype/rank/required owner，**不存在 is_out_of_roof_bounds 调用或其高度、section、perimeter inline 筛选**。7c8320 之后的 Roof section 才进入后续 visual 计算。旧 parent 先用 legacy get_collision_bbx_ls 再 is_out_of_roof_bounds/continue 会错误拒绝原合法路径，本次应移掉这个额外 gate；该独立叶函数本体及其原输出测试保留。

## 最窄现代适配

consumer 仅需 ID map，拟保 `Res<DerivedDecoratorSubtypes>`，改为真实 Copy subtype 的 `.get(id).0`，删除不再消费的 `Res<DecoratorMeshLibrary>` 与 DecoratorLibKey import。与六个纯库视觉消费者不同，此 caller 没有真实 mesh library 读，强加一个只为 as_view 的库资源借用会扩大现代系统依赖；如 lead/root 要统一 SystemParam 可以协调，但不得再读库几何替动态 helper。

在本既有 mapped 文件私有 `cottage_dormer_width(rank)` 或现 `cottage_dormer_dims(info)` 恢复三项尺寸表。优先恢复现 dims 接口为 rank required 断言后返回 Some((原width,1.0))，保现公共签名避免越界修改其它 caller；parent 消费同一 helper结果而不重复表。增加 `Local<super::collision_geo::DormerCollisionCache>`，实际 `get_collision_mesh` 返回真实 Arc<RaycastMeshData>，调用 `set_positioned_colliders(Arc::clone(&mesh),mesh,&translation,&rotation)`。

原的蓝图入口发生在 helper 之前，现唯一 canonical `DecoratorBlueprints::insert_for` 可直接复用；在恢复尺寸表之后、调用 helper 之前执行 `insert_for(id, DecoratorStorageAddr::Wall(wall), info.state())`，随后直接写同一 entry，不再本地 new/末尾 legacy insert。参数依据：7c8204 的1408bc520经 matching PDB识别为 `From<WallId> for DecoratorStorageAddr`，7c8209保存返回RDX，8226令R8=RAX；7c8213读 bucketInfo末字节 (-1，Info+0x47=当前state()) 并8217写stack20第五参，不猜为0。由此保 entry缓冲复用、storage metadata、完整flags和 helper panic前部分mutation。其余窗面板、屋顶瓦片、outline 装配、WindowVisualEntityBuilder 的已有未完成部分不扩写、不计完整 visual 恢复。现 get_dormer_panel_xforms None 不作为成功验收。

## 验证

以 water 的真实 Roof/transform fixture 运行 parent 实际 ECS System，使用无 loader collision entry 的真实资源，验证 rank1/2/3 确实经原尺寸输入形成非空 convex，同 Arc 两槽、非零 cached pose、非单位 scale 不混入几何；同系统第二次运行必须复用 Local cache Arc。缺 ID/cache 与 rank0/4 保 required 失败且不产生 blueprint；若 helper 合法输入可被旧 bounds gate 拒绝，明确回归覆盖它。测试不声称所有 visual side effects 完成。候选依赖 water helper 最终 after hash；独立源码复核后交 root 编译和真实物理验收。
