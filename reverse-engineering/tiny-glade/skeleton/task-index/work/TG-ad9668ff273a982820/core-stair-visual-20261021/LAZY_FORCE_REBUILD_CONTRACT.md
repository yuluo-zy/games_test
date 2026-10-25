# 楼梯lazy消费的真实force_rebuild字段合同

范围仅mapped country-core/resources/stairs/visual.rs，TG-ad9668ff273a982820由lead live show NULL后normal claim core-stair-visual-20261021。原生产before见source-input.json；只合法work候选，无生产/Cargo/Graph/GPU修改。其它reset/remove的RT实体删除和完整0x108值没有因此恢复，不以此标整楼梯完成。

## 原身份、大小与字段

原B RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03、PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。完整reset PDB562:27896实际2322B（旧源码2099B记述范围不足，含EH尾部不能丢），remove1087B；invalidate PDB586:12512实际4290B（旧3455B不足）。native-pack/producer-native-pack均全范围逐指令验证；原函数大小不是业务实现验收。

reset查map的两个u32 key在bucket_end-110/-10c；value起end-108，stride110含key8。140aa8e26写word[bucket_end-8]=0、e2d写qword[end-10]=0；freshdefault140aa9067同word清0。因此+100与+101是两个清零byte，本轮只恢复有producer/consumer双证的+100，+101业务仍未知，不伪造布局填充。

## 原真正置1 producer

PDB inline名称StairsVisualState::force_rebuild，原source132/133为函数/入口、实际store最内Line134，在invalidate宿主有三处：140b379c3、140b37c10、140b37df0，均 `mov byte[bucket_end-8],1`。全部先对原edges HashMap get_mut，key缺失沿skip分支，**不创建entry、不clear mesh、不移除allocation，不设置全局dirty**。flag-stores-inline-pack含5个最窄原store和source/inline帧；skeleton/B该source的inline记录19676/22844/25088与宿主一致。当前手工force_rebuild错误调用reset_edge，改变这些实际副作用。

## 原消费者与现代接口

duck已锁原interpolate1407d0162读唯一USE_STAIRS_LAZINESS；1407d025d比较该value+100，零时026a→0756复用旧sample/插值路径，非零fallthrough0270重建/dirty。必须读实际canonical状态，不能alwaysfalse或另一表。

现代最窄接口：StairsEdgeVisual增私有bool force_rebuild_requested，Default false对应原fresh清0；公开同名getter `force_rebuild_requested(&self)->bool`只读。StairsVisualState::force_rebuild(edge)只在get_mut命中时设true，missing no-op，不更改globaldirty。既有force_rebuild_with<T>保兼容形参，但委托真实mark，不再错误清宿主；不得给它另造与原force相反的语义。reset_edge现有替换entry的Default已清该flag；reset_edge_with完成现有两writer清除后明确置false，对应原reset零写；其余已知未恢复的RT/实体面继续明确，不宣称完整reset。remove自动移除这同owner字段。

## 验收与待接线

测试须用真实SparseInstanceBuffer CPU影子与已有MeshAtlasAllocation，force后allocation指针/contents/byte_map不变且flag1，globaldirty保持原值；missing force不创建entry；reset_with清原buffer并flag0；remove后不存在。不是只验证布尔setter。root执行Cargo；本工不运行。

当前invalidate手工仍错误把现存边走reset，还缺原三次mark针对哪些邻接边的完整调度及三路physics/RT删除，TG1a710 live NULL。该外部source尚未领取/写候选；本轮合同只锁canonical原producer方法，待root同链恢复actualcaller才能称lazy闭环。全局USE_STAIRS_LAZINESS在foreign frameinputs owner也待root协调；不私建第二全局。两62KBassemble未因本字段恢复而算完成。
