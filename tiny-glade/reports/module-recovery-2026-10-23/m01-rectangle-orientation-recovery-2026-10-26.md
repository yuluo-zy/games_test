# M01 矩形旋转控件补链，2026-10-26

本轮接手前实际读取 B 骨架的 `crates/country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_orientation.rs` 及相邻现有目录；只替换该原路径下的两个空壳，不增加业务目录。producer/DO/EditRectangle/历史由rectangle owner恢复，host调度由root集中整合。

## 身份、函数和原分支

B PDB `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`，骨架source SHA `51988613f34f08d3dec8f9ce5ffb90c863426119e0021047de1c98d718f848a3`；实际执行B RNE SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。

- widget `14083d3f0` /1561B：公开Prepared Curve3按XZ最近segment/t插值地面Y+1；两项乘加顺序原样保留，不改成a+(b-a)t。缺路径/墙顶缓存保留失败。
- signifier `14083da10` /1335B：未聚焦只在真实交点存在时显示，Y来自56B原交点载荷+40；聚焦始终显示，Y=前述height+0.1。未聚焦scale0.6；聚焦普通/活动为原bits `3ef5c290` / `3f1374bd`。活动门原hover/click字节，不替换成intersection存在。
- 工厂closure `14083df50` /690B：两个原named mesh `corner_arrow` / `corner_arrow_outline`，vertex_color_shader，真实位置/rotation_from_normal/scale，duration0与原pass选择。
- 控件ID由原tuple `(WallId, usize corner)` 的Hash异或 `a8f64de710aa4d92`。声音按camera_moved为真全关闭，否则原四槽kind1/2/3/3。遮挡为!selected，历史标志保留。

## 纠正selected几何误读

旧反编译遗漏了外部单态化FlatMap闭包，容易误以为selected可以用一个盒，或把不相关skewed_box函数套进来。实际链是：

`widget → from_iter1413047b0 → Flatten.next1412f1b70 → 原inline closure1412f1c0f..1412f1cb1 → box_trigger_volume140ada0e0`。

原get_edge_dirs_from_corner_id返回两条沿边方向；分别配向外垂线 `(dirA.y,-dirA.x)` / `(-dirB.y,dirB.x)`。每个方向生成独立盒（12三角形），最后flat_map拼成24三角形，保留顺序。中心为 `outward*0.7 + (corner + dir*f32bits(bdccccc8))`，高度height+0.1，rotation使用完整原xz_rotation_bad，尺寸0.7/0.5/`f32bits(3ff33334)`。这两个系数不是四舍五入的-0.1/1.9。

未聚焦分支为另一完整原盒：平面中心corner-0.3*corner_normal，宽/深1.5，Y尺寸=墙顶插值-height，中心Y=(墙顶-height)*0.5+height；保留原四元数Mat3转Quat算法，没有简化为轴对齐盒。

全部复用utils已有借用最近点内核与原几何/旋转方法；不为每个角控件复制整条曲线。两个pass故意共用原名字 `rectangle_{WallId:?}_rotate_{Vec3:?}{scale:?}`。现signifier_create以帧前实体快照去重，允许同帧两次wake；不能误加mesh名或只留一个工厂。

## 证明范围与验收

`m01-rectangle-orientation-native.py/.json` 在Unicorn执行原FlatMap闭包及原xvy/xz_rotation_bad函数，5组输入（轴向、旋转、大坐标、负高、退化方向）记录完整Quaternion/translation/scale与实际box尺寸。执行不替换旋转算法，也不宣称执行整个原游戏。

`CARGO_INCREMENTAL=0 cargo test -p country-core ui_modes::edit_orientation --offline -- --nocapture`：3项全部通过。

1. 原5组指令输出与Rust完整Transform/box尺寸逐位相等。
2. selected保留两个独立盒的24个三角形。
3. 真实PublicWallState/PreparedWallPath/墙顶缓存→WidgetParams→UiState真实Widget→两个owned工厂→资源库查找与实体唤醒；两分支拾取数量、遮挡/声音、原同名工厂、真实Y/scale/duration验证通过。

日志 `/tmp/m01-rectangle-orientation-tests.log`。CPU控件与工厂契约已通过，不代表rectangle主业务、历史或整游戏完成；producer/DO/root仍需联合接线验证。用户暂不OS真实输入，未绕过该约束。

## 复用经验

遇到复杂迭代器生成的几何，必须顺着物理call继续到next/flat_map实际closure。业务闭包可能只以原源码inline行记录在标准库单态化实例；from_iter本身不是业务算法。原常量以字节为准，保留表达式求值次序。工厂名字和同帧多次wake也是生命周期协议，不能为了看起来唯一自行加后缀。
