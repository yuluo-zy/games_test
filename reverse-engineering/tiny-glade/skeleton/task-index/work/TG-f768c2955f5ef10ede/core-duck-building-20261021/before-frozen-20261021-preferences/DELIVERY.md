# 完整 find_alignment_preferences1196B 源码候选

复用现source/typedStairPoint与canonical AlignKind20B；真实显式addr/id按原顺序查Storage，不调用忽略addr的简化入口。None normal首先空返回；source Info required→dst墙required→target Info required→source addr.as_wall；samewall短距离不读PublicWalls。三个有序候选不排序，折扣常量/完整turn与score符号翻转、samewall单候选原coord差/-diff.signum/两侧XOR完整恢复。

6新测试从实际storage/cache/graph/PublicWalls发布链构造StairPoint；验证None提前门、同墙短距离门、三项顺序/正dot折扣/负号位、两侧墙normal/u差、NaN有序短距离门、source/target/addr失败顺序、samewall负canonicalNaN。测试未运行，父独审后root CPU。

同墙周期差仍复用唯一canonical WallSpace.subtract，半周期f64重算与原f32反例边界已明确交root，不造本地policy或声称全输入等价。leaf fixture沿现Info.ty0，只证明本leaf原不读ty；实际楼梯Info producer完整类型/id/state仍需后续真证，不凭rawtransport3f改字段。

此source保此前node/best/compute/walk/flags叶；after-walk-flags.patch相对b42f8928，whole patch适用freeze内actualbefore。完整上游两30KB请求及历史/cache/mesh并未因此完成。
