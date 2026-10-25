# RaycastMeshData shared shape 与 Clone 窄恢复合同

合法任务 TG-f7070bb3446b7a37eb，live owner 保持 building-audit-20261018。root 授权 water 独立候选 writer，仅旧 work 新 delegated-water-sharedshape-20261021；生产 root 唯一 writer。本轮冻结 current world before SHA-256 `38593f8e24bb93aae26fdd4d30e910937b106c7065a2df5ca2972cf371eefe62`，保留 root bounded query / convex 已有修复，不改 Cargo/lib/共享统计/其它query算法。

## 双重原证：shape 从来不是 Box 独占

原 B RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03，PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。完整六构造+两SharedShape依赖+86B direct adapter 在本目录 native-pack，report9requests全ok，逐指令verified original bytes、overhang0。PDB originals6：new_convex557B496:55528；new_indexed2435B496:56536；new_unindexed2067B496:62336；new_cuboid290B496:66504；new_sphere267B496:66840；new_compound347B496:67176。没有恢复 parry 算法内部，锁定原依赖复用。

1. 真实构造身份：new_sphere `1408d0166` 调 `1416bb920 SharedShape::ball`，92B完整callee分配0x18，`1416bb947/94e`写strong1/weak1，shape Ball数据+10；二字返回存mesh+20/+28。new_cuboid调用SharedShape::cuboid1416bb980；两TriMesh调用SharedShape::trimesh1416bba10；compound调用SharedShape::compound1416bb810。convex虽inline，也在1408cece4/cecec置strong/weak各1，1408ced18分配0xd0 ArcInner后拷值，最后二字写mesh+20/+28。旧“Box0xd0/vtable”注释误读了 ArcInner，必须更正。
2. Clone 共享身份：完整trap7587B原包（TG-dfec84.../core-visual-data-20261021/before-frozen-20261021-canonical-visual-data/native-pack/tg-0e52e2ec64528325e0）中，`1407bf000..bf21f`分别alloc/memcpy真实4类缓冲并有None/empty门；`1407bf21f/bf223`取mesh+20/+28，**bf227 LOCK INC** 原shape Arc strong并检查overflow；原指针/metadata直接写clone+20/+28。`bf2f7..bf322`为clone值再造outerArc；另一个outer Arc保存原值。故两个outer不同、innerShape同一个，不能Arc::clone一个outer、不能clone_dyn重造shape、更不能裸拷4缓冲导致double-free。
3. direct consumer 86B `140b26d20` 同样从真实mesh+20/+28读，**140b26d4d LOCK INC** 后复制到OffsetColliderTransform shape，不克隆几何。这是独立原消费证据。

## 目标表示与六构造

现 native-physics `shape:Option<Box<dyn Shape>>` 改为 `Option<SharedShape>`。parry3d0.17.0锁定API为 `#[derive(Clone)] pub struct SharedShape(pub Arc<dyn Shape>)`，Deref Target=dyn Shape；复用此依赖，不能自建shapeArc兼容wrapper。`Option<SharedShape>`仍为原16B@+20，mesh当前0x78/其它offset不变，root执行已有layout测试确认。非native壳 `_shape_pad:[u64;2]` 和 Default None 保留，不引入未提供native依赖。

- new_convex：现真实Vec3 bits hash、try_convex_hull、from_convex_mesh、失败None都保留；最后仅一次SharedShape::new(convex)。
- new_cuboid_at（compatibility new_cuboid仍转发）：现rawhash/full dims*0.5/localtranslation全部保留，SharedShape::cuboid(halfx,halfy,halfz)一次。正负/NaN输入不新加guard。
- new_sphere：现radiusbits/translationbits hash和localpose保留，SharedShape::ball(radius)一次。
- new_indexed / new_unindexed：现输入、Option哈希区分、索引/flatten、法线/uv、ownedrawbuffers全部保留，SharedShape::trimesh真实points/indices一次，不造第二geometry。
- new_compound：现先hash_input后逐childhash折叠、child.localpose及outerlocalpose保留；each child.shape.take直接move SharedShape进parts，不再Arc::from(Box)，依赖SharedShape::compound一次持有children。原Vec<RaycastMeshData> stride0x80因原16B对齐，当前Rust重构布局0x78与该Vec步长不宣称ABI直接互换。

## Clone 具体所有权和失败边界

只给有真实native allocation的native-physics RaycastMeshData实现Clone；非native壳保持原无allocator/无Drop行为，不为编译造原不存在的shape。

1. 4类真实owned缓冲按vertices(Vec3)、indices([u32;3])、normals(Vec3)、uvs(Vec2)顺序独立分配复制。None ptr0仍0，Some(empty) ptr4仍4，primitive vertex/index sentinel4 len0不当真实指针解引用；Default全None保持0。
2. 使用Option<Box<[T]>>暂存每份clone副本直到全部复制成功，最后移出rawptr。若中间分配异常，先前已复制Box正常释放；不连续leak_slice后靠裸数字拼装造成失败泄漏。有效ptr/len前提和现Drop一致；不新增unsafe指针来源，不手抄原依赖分配器。
3. 原长度、hash、四个localpose原word和shape metadata不被重新计算。shape使用Option<SharedShape>::clone，仅+1同innerArc。最后为新RaycastMeshData对象，不强制创建outer Arc；真实caller决定两个不同outerArc。
4. Clone Drop各释放自己的4缓冲，并正常减sharedshape；原对象仍可读取、射线/几何值不变。原缓冲ptr不可复制复用，shape不可clone_dyn。

## 直接调用影响审计

当前全crates生产搜索没有外部RaycastMeshData struct literal，仅world.rs自身六构造；所有已有vertices/indices/normals/uvs getter、hash/localpose/public API签名保持。shape_ref现在Option<SharedShape>::as_deref仍正好Option<&dyn Shape>，无需私有Deref层或消费者fallback。当前布局测试全部offset/size继续使用。

必要同文件两处direct adapter目前对shape_ref().clone_dyn：upsert_collider约1929、into_transformed_shape_iso约2633。应改为从required mesh.shape.as_ref().expect(...).0 Arc::clone，继续输出既有OffsetColliderTransform {samepose,shape:Arc<dynShape>}；只更正原本的错误geometry深克隆，不动query/maxTOI/source/registry/world算法。后者有上述86B原证；前者是当前真实接线调用该转换同source，保持唯一innerArc才能闭合shapeowner和性能。父级已明确批准这两处同owner必要direct adapter；不扩大其它query。

影响之外：OffsetColliderTransform本来Arc<dynShape>，其Clone/Builder.shared_shape照旧；new_compound不能额外双建shape；无需增加Cargo依赖、手写parry算法或扩恢复family。已审trap candidate真实Arc::new(data.clone())会因此编译并保持twoouter/oneinner；其余四表/loader真Arc消费者复用同canonical值。

## 必须有的测试源码（root运行）

- indexed含4个非空ownedbuf，clone值相等、4ptr均不同、hash/localpose完全相等、Arc::ptr_eq内shape且strong+1；先drop原再读clone和dropclone，反向也成立。
- indexed None对比Some(empty)保ptr0/4、hash差异，不以空切片统一两者；primitive六构造的4sentinel和defaults，不拿非空TriMesh丢给空几何panic伪覆盖。
- 原cuboid localpose/geometry保持，Arc::new(original.clone()) 与Arc::new(original) outer !ptr_eq、inner ptr_eq；mutation一边rawvertex不影响另一边。
- compound children内shape持有是同原Arc，child drop后compound子shape仍有效，hash折叠和childlocalpose保持。
- shape_ref/Deref仍取真实shape；layout0x78/field offsets保留。若批准direct adapter，验证OffsetColliderTransform形状ptr同inner且strong变化，不只几何数值相同。

没有运行Cargo/rustc/GPU；合同批准后仅在本合法work产最窄candidate/patch，生产root应用。此修复验收不替代整体RaycastWorld其它算法的原版差分。
