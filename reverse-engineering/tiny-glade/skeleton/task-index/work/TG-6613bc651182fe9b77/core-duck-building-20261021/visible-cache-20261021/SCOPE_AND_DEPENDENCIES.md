# 08ecee 新阶段：可见cache闭包合同与源范围

root明确委派本worker唯一候选writer；TG6613保持core-duck-building-20261021，TG1d8保持cloth-visual-state-20261020。两源均已重新live show并从当前真实生产冻结before，碰巧仍分别f67cb1d3、8011a64c，不能据旧候选hash推定未变。禁止生产/Cargo/GPU/Git；不新生产文件/agent。

## 本次生产文件/函数范围

1. utils/src/lib.rs：新增原 `pub fn xz_rotation_from_forward(Vec2)->Quat`（140c913e0/607B），其必要失败域与方向测试；不改bad、timefactor、四参lerp、modules或其它公共函数。
2. systems/decorator/src/cache_decorator_transforms.rs：完整9331B producer（1408153d0）接同9资源；补roof63/terrain63/dst4全路径，修每项compare→insert→modified、顶层immutable门与删除清册。仅该源私有拆分数值/存储小函数以及真实World测试，不新文件、不修改外部owner。
3. 原canonical Info.ty63真正直接producer继续只读定位；raw InstaCreateStairPoint事件transport不当成Info producer，不默认ty/extra。如果发现缺失外部source/API，先列确切task/owner后中央协调，不暗改第三源。

## 主合同与本次必要精化

完整已冻结业务合同：../readonly-water-contract/CACHE_FULL_BUSINESS_CONTRACT.md SHA d19e369a；必须配MODE3_AND_WEIGHTED_ORDER_ADDENDUM.md SHA40d804cd。原B9331B和peer全包逐byte匹配f507 RNE；PDB458:2156。真正9资源typed f9ea3bd7ad9711ee362baef4 + rununsafe1400c552b直call，共同绑定。物理TerrainHeightSsbo，不是Visual；原系统Local previous，current唯一owner不换。

依赖现已整合：Params/Requests2a046da8、storage98494a43；既有Curve2 get_coord、Curve3 points/u/length、StairSegment::nodes、节点锚点、梯598/603B真实helper、DecoratorRotationXZ、Transform::abs_diff_eq、Roof/volume与ClutterMeshes。只借同owner，无镜像/每帧whole-entry collect。

主合同编写时有两项本次源码审额外精化（直接由同9331B字节可读，不新增业务族）：

- 原wall/roof桶的raw3/4应unreachable；不能把这些错误条目送terrain/stair helper。当前旧source未防。主循环将按原wall domain guard；terrain/stair桶分别required3/4。
- 当前旧source在每个wall桶开头 eager `walls.get(wall)`，原wall branch才required查墙，roof branch独立查roof association并可fallback；恢复为wall branch内懒取，避免未访问分支多读/多panic。已有geometry算法保持。

wall/roof direct insert，无compare/dirty；terrain/stair先top storage.get_mut（原不可变门），后每项computed→oldabsdiff→insert→changed立即modified；原29B item只标id。不新增“changed时才assertimmutable”或bucketimmutable字段。current.swap清册既保buffer分配也自然清删除id；失败保已发生副作用。

拟在同源拆出完整stair transform私有函数，与wall/terrain helper各借实际owner；Option空实现消除，但保原no-request(-0,-100,-0)+identity+ZERO真实哨兵和roof缺UV/关联previous或(-100全轴)+ONE两个不同分支。需要None的真实资源/Curve3仍required，不吞panic。

## 验证边界

必须先607合同独审，再源候选；cache按本完整合同独审实现。root唯一运行真实CPU/GPU。既有上轮core2831/decorator340/utils153全通过只证明08ecee基线，不覆盖本次未写功能。原程序行为oracle未执行，不能声称所有NaN payload/目标浮点控制字位等价。新测试使用actual World/Query/Params/双cache owner，并覆盖完整失败顺序、两量化与删除，不以比例或普通非零方向代替验收。
