# 独审修正：旧公开helper复用canonical

范围只含两个旧公开wrapper及其测试。保持`Result`、points长度、length正值、坐标/归一化坐标有限和roof tag/六字段有限/正面积前提；没有新增单位axis guard。

`wall_coord_to_world`直接委托`WallSpaceCoord::to_world_space`；`closest_roof_edge`以真实观测axis/center/size字段构造Rectangle2d值并委托`get_closest_edge_id`。不再复制数值循环。原负坐标方向保持现canonical前向切线相减；重复端点外推时旧手写只改XZ、错误保留有限Y，新路径按原canonical零乘NaN传播Y。

保留5旧测试和完整producer9测试，新加2测试覆盖canonical同值、负坐标、零段NaN、非单位axis不加guard、原Result失败。16个源码测试；未执行Cargo/rustc/GPU。完整producer其它业务不改。

基于先前f9a433 freeze的独审追加；原freeze不修改。narrow patch与从原6ce9a006开始的complete patch同时提供，root择一应用避免重复。
