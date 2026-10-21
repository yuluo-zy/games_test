# 活板门动态碰撞体：只读独立复核

本附录仅为 TG0e52 唯一 writer tree_commands 提供消费者合同；不领取、不修改其 work/source，不把任何库 bbox/Sphere 代替原几何。

## 身份与精确源包

原B RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03；matching PDB SHA 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。完整原函数 B1407be730/7587B，peer native-pack/tg-0e52e2ec64528325e0.{json,asm.txt}，其绝对路径/hash 已存 evidence-index.json。peer report 每请求 status=ok；不是 runtime 验证。本文 collider 范围每条指令 byte 又直接对发行 RNE 验证。

## 数值、分配与所有权

1. 1407bea8d `MOVSD xmm9,[142a85d70]` 读取两个 f32 `[0.0,1.3]`；1407bef89 写入 dimensions 的 X/Y，1407bef92 写 Z=`0x3fa66666`。因此 full dimensions 精确为 **(0.0,1.3,1.3)**，不得猜 X=1.3、不得把零宽改为 epsilon。1407bef9c/efa6 清 local translation 三lane为 +0。1407befc3 调原 RaycastMeshData::new_cuboid1408cfff0。
2. 原 cuboid 构造器的1408d0037..58 乘0.5建立半尺寸，因此业务调用不得再除2。1408d006c..73 对第二实参执行 from_translation，local rotation 为 identity；hash 喂原 full dimensions 与 translation bit words。当前 `new_cuboid_at(Vec3::new(0.0,1.3,1.3),Vec3::ZERO)` 正是已恢复的同一构造接口。
3. 1407befc8..bf0cf clone vertices/index 两个 owned slice；bf0d4..bf21a clone normals/UV 的 Option slice；bf21f..bf22b 对原 SharedShape 内 Arc 加强引用（不是按库key另查网格）；bf231..bf2e9 复制 mesh-local rigid pose 与 hash。此 cuboid buffers 实为长度0，不能把原 inlined clone 错读为新 indexed mesh。
4. 1407bf2f7..bf322 新分配0x90B外 Arc，counts=1/1，存入 cloned RaycastMeshData。1407bf335..bf3d9 第二次新分配外 Arc，move 原完整 data。**两外 Arc 不同且几何/hash相同，内部原 SharedShape 引用相同**。不能只 `Arc::clone` 一个外 Arc 冒充原 pointer 差异。不存在缩放、shrink、padding或第二组碰撞尺寸。
5. 1407bf43c set_colliders 顺序：RDX=clone-first Arc，R8=moved-original Arc；R9=&cached translation，stack+20=&cached rotation。1407bee6a 已 required 读取同一 CachedDecoratorTransforms.transform(id)，rotation在 stack120，position在130。scale 部分没有乘进任何 collider dimensions/local pose 或传给 set_colliders；不得把视觉 full transform 的 scale 自动用于碰撞。

## 与后续hole的边界

1407bf796..bf79f 写 `[0.0,1.125,1.125]`；1407bf7a9..bf823 为 cached quaternion 作用局部 +Z 的算序；1407bf85b 调 DormerHoleSsbo::cottage1412866e0。**1.125 是后续洞数据，非 collider size**。本附录不替代 hole 完整producer恢复。

## 当前架构适配与待协调点

现 core RaycastMeshData.shape 是 `Box<dyn Shape>`，没有 data Clone；原则是保真实物理几何与两外 Arc，同时明确内 shape 共享差异。最窄无 world 改动方案为按同尺寸/零 local translation 构造两份真实 cuboid，再分别 Arc::new；这样几何/hash/local pose 一致、outer Arc 独立，额外创建一次固定 primitive，原内部 SharedShape共享不能声明已保留。是否使用此适配由 lead/root 裁决；不可在此 source 偷改全局 Box→SharedShape或自行扩通用buffer clone实现。

Parry3d0.17.6 Cuboid::new 实际直接存 half_extents，没有正值断言；其文档正值说明不能授权把原 X=0 改值。只读核当前依赖源码，未执行 Cargo/native/runtime。

## 建议验收

在实际候选里核两 `Arc::ptr_eq` 为 false、hash相等、shape half_extents=(0,.65,.65)、mesh local pose 零translation/identity rotation。以非单位 cached scale 检查 collider dimensions 不变化；世界pose只取真实 cached translation/rotation。真实物理query与宿主验证由root统一执行。
