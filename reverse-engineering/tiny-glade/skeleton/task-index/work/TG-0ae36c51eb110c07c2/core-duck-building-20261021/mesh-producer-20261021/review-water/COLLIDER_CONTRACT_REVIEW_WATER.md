# 楼梯碰撞/RT几何必要闭包独立合同审查

**按Y算序addendum合读后准入通过，无剩余静态阻断。** 旧COLLIDER_GEOMETRY_CONTRACT.md ed54e99e的右侧Y关联表达式不能单独作为源码依据；已由作者接受纠正并冻结COLLIDER_GEOMETRY_Y_ORDER_ADDENDUM.md 15040f54。审者未写该source/候选，原外679B、游戏callback2763B及570/684/296迭代完整ASM均重新读取，全部指令再对f507原PE比字节；既有/新增全部literal和两index表均独立直读PE一致，记录另存。工具复核不是业务完成计数。

## 10 ABI降为7个真实Rust借用

外层R10取caller stack30 stride（prolog后240），callerstack20/28分别widths ptr/len，38/40/48三输出；RCX/RDX为positions ptr/len、R8/R9为tangents ptr/len。main真调用14000d04f..d0b1准备三输出同owner指针、width两点及step1和两slice对齐，支持拟API slices+usize+三Vec mut borrow。不是借推测参数数取代实际类型。当前public旧无操作壳没有真实caller，作者仅同源恢复可行；未来fullmain须把真实输出owner传入，不能造临时registry清零吸收结果。

外层ea0f在第一次输出写前检查stride!=0，失败原literal27B已直接PE核；局部index常量初始化没有输出副作用。Rust应保先门再迭代，不能把stride0在空输入上默认为成功。

## 原迭代/输出与失败域

- outer ea40..50先取positions/tangents/widths最短len；各stride均为12/8/4字节只借现slice元素。step_by首项0、之后原stride；tuple_windows始次调用chain两次建立首pair，之后只移旧元素/取下一采样。Once保存上一元素，不是末端补最后点；不能为曲线完整性私加尾点或先collect新Vec。复用依赖zip/step_by/tuple_windows即可，不复刻泛型迭代内核。
- callback按给定八点依次写24个RTVec3，长度在6613增加24；随后写6个floorVec4并6789加6，最后679a调用唯一canonicalnew_convex。原确先RT、再floor、再convex；None时spec_extend fa1b..1e跳回next，不增colliderVec，已写RT/floor保留。Some才移动完整MeshData并增长colliderVec；不清已有前缀，不TinySphere fallback，不空凸包/假成功。依赖new_convex保实际None/错误语义。
- RT顺序逐outer局部表解出[0,4,1,1,4,5,2,3,6,3,7,6,0,2,4,2,6,4,1,5,3,3,5,7]，floor[2,3,6,3,7,6]；floorW由原14293e0c0 lane3=1，不用任意0。索引是本helper固定合法值，不把原分配/边界失败改成忽略错误。

## 几何数值必须按勘误实施

先读tangent，再XOR其y符号位、x0y形成normal，分别乘width再乘.5，dy=(0*width)*.5保持，负width不abs、不normalize、不加finite门。原两端共享公式但base顺序为端0 p.y+(-.375)、端1 (-.375)+p.y；left_y=base-dy、right_y=dy+base。**不能沿旧主合同写(dy+p.y)+(-.375)。** 原60ad/60e2/6111与点store证明此点。

每端lowerleft y=left_y+(-.5)，lowerright y=(-.5)+right_y；upperleft y=left_y+.5、upperright y=right_y+.5。上部XZ显式加+0，不能省略影响-0的加法。原两常量142fcbcc0=[0,-.5,0,0]和142fcbcd0=[.5,0,0,.5]逐byte核过；floorW亦真实。NaN/Inf导致0*w传播，不能把dy省成常数零；单独有限几何样例不足以验这些原失败/传播域。跨ISA算术NaN payload未有动态oracle，不能声称全位等价。

## 源码验收界线

拟七参API适配准确，可源码化；使用真实Vec<RaycastMeshData>/RTVec3/floorVec4，不增加Arc/另一owner。应测试实际convex成功、None仍保24+6、前缀、最短zip、stride不补尾及0门、转弯变宽和原浮点边界，root运行nativephysics；我未Cargo或改production。完整main/ladder/steps/floors/rails及宿主未由本leaf判为完成，B证据不泛化C。
