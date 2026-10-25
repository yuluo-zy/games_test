# 哥特窗实际CPU失败后的测试夹具基向量纠正

实际失败日志：`/private/tmp/tg-gothic-graph-cpu-v2-20261021.log`。生产算法不改，所有原坐标期望、容差、测试保留。原修复范围是测试输入构造，并不是原业务算法bug。

Rectangle2d::as_points2 的现有原证恢复顺序是 `(-4,-4) → (-4,+4) → (+4,+4) → (+4,-4)`，闭曲线总长32。原夹具在u=4处位于第一条左竖边的中点`(-4,0)`，gethat两端均在该边，其主法线为-X；原有z=-4、glass z=-3.86、collision z=-3.36、dormer z=-3.61的oracle明确假定底横边沿+X、法线+Z。原样测试输入与oracle不一致。

最窄候选仅在producer_world中显式按角点[0,3,2,1]创建同一个真实闭矩形，并加前置raw曲线角点与u4世界位置断言，确保所有原oracle测试的正是所描述的基。旋转与shader输入没有改，所有rank/subtype/style覆盖维持。现有circle/noncircle/bent-wall专用测试继续使用原定义；并没有靠改变期望或放宽阈值过门。

原B 1407eb500/1182B先按u±half转换两端，原B1407e87b0/10658B中姿态来自端点平均和法线，玻璃+0.14、碰撞+depth/2、dormer+0.25；完整原证复用父目录两个BYTE_PROOF与原包，未新增猜测算法。输入bottomedge时，端点z均-4，法线+Z，三个后继z的原期望与公式一致。

只准备候选；未写生产、未Cargo/运行测试。独审后由root执行实际CPU。原V1与root0_u8机械fixture修保留冻结。
