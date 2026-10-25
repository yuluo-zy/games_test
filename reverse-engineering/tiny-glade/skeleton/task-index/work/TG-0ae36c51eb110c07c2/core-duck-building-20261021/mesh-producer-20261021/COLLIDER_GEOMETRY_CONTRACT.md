# gen_stairs_collider_and_rt_geo 完整必要闭包合同

## 原身份与实际ABI
- 外层679B `1407de8b0` / PDB437:157148：完整原证`helpers-native-pack/fn-7de8b0`。
- `spec_extend`570B `14125f8e0` /1503:10412；tuple_windows next684B `14129cfc0` /1552:264；其once+remaining-step迭代296B `141275490` /1510:264；真实游戏几何回调2763B `141fe5f70` /3052:264，完整原包`collider-geometry-native-pack`。不是把依赖wrapper数量当业务恢复；其中游戏回调几何/输出必须逐项实现。
- 外层原10 ABI值：positions ptr/len RCX/RDX，tangents ptr/len R8/R9，widths ptr/len callerstack20/28，stride stack30，collider Vec<MeshData> stack38，RT Vec<Vec3> stack40，floor Vec<Vec4> stack48。见main14000d04f..d0b1（真实3输出分别fromstatewriter/RT同owner，没有新registry）。
- 外层在校验stride!=0之前没有任何输出写；0按原panic文本 `assertion failed: step != 0`（原literal长度27，VA142a879d0；需PE literal逐字绑定）。

## 完整迭代与输出
- `positions.iter().zip(tangents).zip(widths)`以最短长度为界；其map只平展tuple，随后step_by(stride)、tuple_windows二元相邻窗口。首采样index0，然后stride，**不会额外补最后点**；内部Once是windows保存上次元素，非尾端补点。
- 每窗口构造8点凸壳；先附加24个RT点，再附加6个Vec4 floor点，最后调用canonical `RaycastMeshData::new_convex(&points)`。Some仅追加到colliders；None不追加，但**保留已写RT/floor几何**。不用structural叶的tiny-sphere回退，不造默认凸包。
- 原`spec_extend`仅在返回Some时使目标Vec长+1，不重置任何目标Vec。原输出已有前缀保留，失败panic时已发生的前缀写不能被事务式替换。

## 数值与顺序
每个端点p、t、w：
`dx = (-t.y * w) * .5; dy=(0.0*w)*.5; dz=(t.x*w)*.5`；无normalize/finite门。
`left=(p.x-dx, (p.y+(-.375))-dy, p.z-dz)`；
`right=(p.x+dx, (dy+p.y)+(-.375), p.z+dz)`；注意左右Y加法顺序不同。
4点依次：left.y+(-.5)、right.y+(-.5)、left+(0,.5,0)、right+(0,.5,0)。后两点XZ显式加+0，保-0变化。两个端点依次给points[0..4]/[4..8]。
RT索引准确24项（外层局部stack58开始）：`[0,4,1,1,4,5,2,3,6,3,7,6,0,2,4,2,6,4,1,5,3,3,5,7]`。
floor索引6项（stack28开始）：`[2,3,6,3,7,6]`，追加每点`.extend(1.0)`。
- 几何回调原141fe5fef..6082：读取tangent，再XOR其y符号位构造normal；swizzler x0y。原常量-.375=VA142ed1838，-.5/+ .5由14295a0e8/142925870，须PE记录绑定。
- NaN/Inf/负width不提前校验；0*Inf与0*NaN必须传播到Y，不能省略dy。反转w不手工abs。凸包的依赖错误行为由canonical方法承载。

## 拟现代API与测试
`gen_stairs_collider_and_rt_geo(positions:&[Vec3], tangents:&[Vec2], widths:&[f32], stride:usize, colliders:&mut Vec<RaycastMeshData>, rt_points:&mut Vec<Vec3>, floor_points:&mut Vec<Vec4>)`。
只改本源错误无操作壳，公开签名无现caller；未来main必须传同一真实owner。
测试：actual convex成功与24+6有序输出/前缀不清；zip最短、stride不补尾；stride0 panic输出不变；退化凸包None仍有RT/floor；变宽转弯+左右Y顺序；负零/NaN/Inf边界；nativephysics几何实际shape。不运行Cargo，由root执行。
本leaf恢复不代表完整62KB main/construct_ladder/steps/floors/rails及宿主完成。
