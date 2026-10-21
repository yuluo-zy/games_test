# 两主装配器：入口、partition与必要owner合同（完整main仍未闭合）

## 原范围与身份
- true：140008c90/62374B/PDB2:251656；false：140018040/62822B/PDB2:342240。
- native-pack两完整范围与identity已绑定。main-private11*.c只在唯一私有worker作导航，原函数bytes独立匹配RNE；显式保存/重开overlay未证，不当业务完成。
- INPUT_OWNER_CONTRACT.json已锁真实11组/17flat SystemParam；ThreeSSBO见THREE_SSBO_OWNERS.json。原取Brick→Wood→Sparkle，reset按Brick/Sparkle/Wood传三不同类型owner。没有global/default SSBO。

## 按原顺序的每请求入口
1. 外层先取并验证三个SSBO。缺requiredkey/错误type/下转失败按原panic。随后遍历current requests（stride0x150），不clone/drop或drain请求Vec。
2. 原`dirty@148 ==1`，否则跳过；`interpolated_curve@98`必须Some，None跳过。两门在RemovedSupports哈希查找前。
3. true判别：RemovedSupports.contains(segment)时跳下一个request（1400092f1→9330）；仅不含才进入1400093b9。false判别相反：命中140018688→186c7；缺失1400186ae→1fa40（下一request）。**这是真实业务partition，不是任意run_if或duplicate system。**
4. 将两个原IDs调用140959e70打包identity（highfirst）；该值送init_rng_u64（native1400093d8..3e4），保每请求确定性RNG，不用thread RNG或ID简化seed。
5. 解包同一interpolated_curve；由storage对segment第一ID重复lookup：Info.u28缺失时显式原default100003；optional Stair.extra+8缺失0；optional Stair.extra+10缺失0。门是原try_as_stair，无额外parse非法disc panic；当前canonical optional accessor可复用。
6. 原再次计算RemovedSupports.contains为后续状态，不因partition已知真值删lookup或提前改变失败顺序。
7. 两端anchor若Roof tag1捕获各height；其它tag不制造height。随后reset_edge同真实Commands/3SSBO owner，先释放旧产物，取returned borrowed writers/entities/cells/floorpoints/wallholes（不是copy旧state）。
8. 原RT.get_mut句柄tag1+packed edge；获取/创建同RtWorld entry后仅清`vertices@+50`长度，保其它RTowner。不能新局部RT表。

## 每subsegment原范围规则与const差异
- 原SubSegment.count给当前end=start+count，使用inclusive范围[start..=end]，points/tangents/widths逐个真实bounds检查；相邻段复用接缝端点（下一start=end）。没有全数组预先等长检查。
- first=start==0，last=end==interpolated_points.len-1；各原bool进入后续endpoint业务，不把count当点数而漏+1。
- auxiliary_samples若非空先按同inclusive范围取值；全值有序>0时可能将kind1转kind2（原true140009d00..14000a2b4），保NaN非positive。空值不在入口统一制造默认。
- false的缺失辅助样本panic不是入口无条件assert：在该段非ladder路径实际CurveTry/try_resample成功、清临时曲线之后，140019595检查missingflag；1400208ad..20914使用原 `Distortion values missing for ` +segment formatter。true没有此panic。失败曲线路径应保原回退/skip顺序，不能把门提前。

## 当前边界
本合同只覆盖入口/partition/owner与所列范围门，不能当完整62KB main已可实现。待合上三种segment几何/rails/两requests records/compoundcolliders/actual drawables和GPUtopdown、postloop history/extraarch。原大body已经齐，不再以工具timeout说缺原证；剩下是逐路径源码恢复与shared canonical真正接口。
