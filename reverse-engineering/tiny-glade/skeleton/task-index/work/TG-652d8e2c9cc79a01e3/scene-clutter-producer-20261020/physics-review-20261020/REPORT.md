# Clutter cuboid dependency: dimensions and mesh-local translation

结论：这是当前源码中的真实合同错误，不能只改参数名/哈希后继续使用。B `RaycastMeshData::new_cuboid` 的第一Vec3是完整尺寸，原函数自己乘0.5后构造Cuboid；第二12B是用于局部平移的浮点向量，同时参与几何哈希。当前 `new_cuboid(Vec3,UVec3)` 保留了部分位型哈希，却没有保存/消费正确的mesh-local pose，直接产生中心在0的Cuboid。原查询消费者明确读mesh+0x50的局部变换，所以这不是未使用的装饰字段。

范围：只读依赖审查。collision/world.rs文件组TG-f7070bb3446b7a37eb仍属于building-audit-20261018；未改锁、未写该生产文件、未创建覆盖它的新candidate。报告保存在root已claim的decorator helper工作目录中，不取得collision文件owner。没有Cargo/GPU/原进程运行。本报告仅B原证，不声称C等价。

## 原机器码与类型事实

本目录native-manifest.json绑定发行RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03与B PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。所有保存的范围均按完整已知长度解码，不沿用截断反编译。

* new_cuboid@0x1408cfff0完整290B，原字节SHA751506853b9dd302e1ad0b4b7a7b214793f2612ad3dffcaae4b448629937fe0c；PDB procedure记录66504、module496、source line1710。
* 入口rcx是sret，rdx指第一12B向量，r8指第二12B向量。第二向量在0x1408d0010..1a按原12B拷入hash临时值，第一向量也逐位拷入；call0x14145a540的PDB名字是calculate_hash<tuple<array<u32,3>,UVec3>>。这证明哈希载体是位型，不证明第二参数只是随机seed。
* 0x1408d0037从0x142925870读0x3f000000=0.5；0x1408d003f..52对第一向量三分量各乘0.5；0x1408d0058调用PDB绑定的Parry SharedShape::cuboid@0x1416bb980。因此第一输入是完整dimensions，当前half_extents命名不正确，但当前内部乘0.5这一步本身应保留。
* 0x1408d006c..73把第二向量的原指针传给IsoTransform::from_translation@0x140c91b30；该函数完整34B读x/y/z为f32、写translation，并从0x14293e0c0读[0,0,0,1]作为identity quaternion。其第四平移SIMD lane不是另一个业务参数，不应猜为尺寸/种子。
* 0x1408d00b3..bf把返回的完整32B写到RaycastMeshData+0x50与+0x60；shape在+0x20/+0x28，hash在+0x70。原struct类型全文未从PDB恢复，故这里只断言已观察的局部变换字节区和语义，不虚构完整作者Rust签名或padding字段。
* 原into_transformed_shape impl45@0x140b26d20完整86B：+0x20取tuple内mesh指针，0x140b26d2d生成mesh+0x50指针，0x140b26d36调用完整239B IsoTransform::nested_in@0x140c91b80，随后与shape Arc一起输出。局部pose确实影响原查询，不是仅被hash。nested_in按实际rdx/r8参数组合变换；不能凭常见变换习惯交换左右乘顺序。本目录保存该完整range供后续owner裁决。
* 原to_offset_collider_builder impl23@0x1408d0390完整241B保留shape并把已有Isometry装入公开Rapier ColliderBuilder；原路径无需手写物理或三角网格算法。

## 当前源码反例

当前collision/world.rs:506把第二参数命名/定型为UVec3 hash_input，并把它与dimensions混装到_pad0/_pad1/_pad2，scale=0。这里既没有正确的identity quaternion，也没有可调用的局部pose合同。shape是以0为中心的Cuboid。

现shape_ref只返回shape。RaycastMesh::raycast把该shape交给Isometry::identity；upsert_collider只用其外部position/rotation构造Collider；into_transformed_shape_iso只做transform.nested_in(parent)，完全未读mesh局部pose；new_compound对子shape使用Isometry::identity。故只在构造器塞回三项translation仍不足以修复真实consumer。启用正确的local-pose读取还必须核这些消费者与其它初始化器，不能在未知_pad数据上直接启用统一解码。

有限正数反例：dimensions=(2,4,6)、translation=(0,2,0)、外部placement为identity。原几何AABB是min=(-1,0,-3)、max=(1,4,3)；当前源得到min=(-1,-2,-3)、max=(1,2,3)。这由原构造/消费与当前公开Cuboid几何可直接推导，属于静态反例，未称运行测量或原进程对照结果。

## clutter实际调用者

main_graph_team给出的完整generate_clutter原6631B证据已保留在TG-b3ec56669fe9d97baf/scene-clutter-producer-20261020/evidence/generate_clutter.asm，本报告manifest绑定其当前哈希并保存调用点片段。

第一call@0x1407e3d7b：metadata bbox的width/depth和top-bottom形成dimensions=(w,h,d)；中心按原运算顺序为((h*0.0)*0.5,h*0.5,(h*0.0)*0.5)，不包含bbox bottom。第二call@0x1407e3e21使用缩小后的dimensions，但仍用原h的同一个中心；不得把center改成缩小高度一半，也不得再次把输入dimensions折半。具体NaN/MAXSS/MAXPS规则由clutter合同owner保留，本审查不改其算法。对于非有限h，h*0并不等于可任意替换的字面零。

当前decorator helper new_cuboid_collider仍显式None，因此没有已执行的clutter错位测试可报告；本结论是在恢复其真实producer时发现的依赖反例，不把helper缺口制造成默认成功。

## 复用现依赖的设计裁决

当前Cargo.lock固定Rapier0.22.0的PounceLight fork rev7b5ece73c45a4d39bb983c48c283e773d7241050（本地checkout HEAD已核相同），Parry3d0.17.0（checksum33f1a44b6ced345a7bb52f5df9912fecae07c6e22d231ba1bab7abf807f52ec9）。首次查看过本机另有的0.17.6源码，不作为本报告依据；最终所有API引用和哈希均绑定实际锁定0.17.0。

优先方案是交原collision owner有界修复现canonical RaycastMeshData局部pose，并同时连接原消费者。逻辑接口宜是dimensions:Vec3、translation:Vec3，哈希仍使用两向量的原位型；不得用Vec3.as_uvec3数值转换替代f32::to_bits。复用当前utils::IsoTransform表达局部pose，按原caller参数顺序组合，再把最终Isometry交给现Rapier/Parry公开查询或ColliderBuilder::position。形状保持Cuboid(dimensions*0.5)；不手写物理、不引入第二碰撞世界。实际存储/API形式由该文件owner选择，不要求复制原ABI。new_sphere/new_compound与统一identity初始化不能由此报告直接推定全部正确，若启用公共pose读取必须一并核其合法初始化，而非泛化本轮scope。

若root在不写foreign collision源码的前提下，需要限定的clutter-helper适配，现Parry公开SharedShape::compound(Vec<(Isometry,SharedShape)>)可以将一个Cuboid(dimensions*0.5)放在已证translation处，得到几何上平移的形状。它只用现物理依赖，不造网格；实际entity/world placement继续照原保留。此时baked shape的mesh-local pose必须约定为identity，不能未来又组合一次相同translation。该方案改变primitive shape kind/feature-id路径并增加一个Compound/QBVH结构，不能未验证就声称所有原消费者等价，也不能把它混入其它mesh构造器。若当前RaycastMeshData没有明确表示identity local-pose的公共入口，不能靠忽略其错误pads把这一缺口宣布闭合；应先由owner给出可依赖的canonical入口。

因此当前最安全的集成门槛：先明确唯一local-pose语义和所有实际clutter消费者，再决定owner修复或明确受限adapter。仅调用当前new_cuboid(dimensions,translation_bits)或只改参数类型，均不通过合同。

## 最小后续验证建议（未执行）

由root/合法collision owner执行少量有意义的真实依赖测试：非零Y及XYZ translation的AABB/射线/overlap；同dimensions不同translation的hash区分（原位型，含signed zero/NaN只按原允许范围）；原外部placement叠加，按原nested_in参数顺序核非identity旋转；clutter外/内两个尺寸共享原中心。若选择Compound adapter，额外验证直接RaycastMesh与世界collider两消费路径及FeatureId相关分支、避免double translation。现identity-only测试不能发现本反例。这里没有新增测试或运行任何CPU/GPU验证。
