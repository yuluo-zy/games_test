# M01 DecoratorOutline 主描边工厂恢复

2026-10-27。仅修改原 B 同路径 `crates/country-core/src/resources/walls/decorator_outline.rs`，未新建业务目录、未改 rectangle owner 的 ui_autoclutter。现有 DecoratorCollisionOutline::display 是独立碰撞轮廓，不能代替原主描边。

## 原文件和完整指令

实际读取原B skeleton同目录文件，并核PDB585原文件SHA `08404d80f8a320f42f6dee48e901b156c6b314b42a48449ce5f797e22913f434`。

| 原函数 | VA | PDB完整bytes |
| --- | --- | ---: |
| DecoratorOutline::display | 140b33b00 | 2876 |
| Mesh closure0 | 140b34640 | 665 |
| Lines closure1 | 140b348e0 | 855 |
| Sphere closure2 | 140b34c40 | 639 |

本目录 `m01-main-outline-evidence-manifest.json` / report 与四组 `{json,asm.txt}` 独立绑定发行RNE/PDB完整SHA、GUID/Age和source/inline metadata，均status ok。旧共享Ghidra主函数仅760B、停在坏跳表；三个closure的551/709/521B也不含完整异常清理，不能当PDB长度。

只在本agent独立worker修已证明的4项switch：`140b33df6`、原relative table`142aed9a8`，原目标分别`140b33df8/33ec3/33f2c/34144`。SHA门先核2876B原范围，原代码不变；独立worker导出完整4分支后正常停止，未修改共享/其它worker。本次初始repair使用到下一函数入口的2880B容器范围，随后原PDB复核更正实际procedure为2876B；原pack按2876B绑定，无跨下一函数取证。本次不宣称额外db reopen持久化验收，保留脚本/返回和修后C供重现。

## 真实调用合同与字段纠错

恢复的安全接口：

```rust
outline.display(&affine, &mut stream, id, rank, offset_forward, animate)
```

依次为self、Affine3A、UiSignifierStream独占借用、DecoratorId、DecoratorRank、forward偏移bool、动画bool。保持现安全ResMut流，不引入UnsafeCell，不重复建立描边事件资源。

ui_convert调用`14099dd4f..dd74`读取source+10为独立ID，info+44为rank，info+22<2作为forward门，最后true作为球动画门。第五参数不是subtype。原formatter140b35930内的字符串确为DecoratorRank，字段名x；现canonical Rank的全局derive Debug为tuple格式，不宜擅改所有调用。只在该工厂身份名用私有Debug投影恢复原 `DecoratorRank { x: N }`。

原格式常量`142aed708`三段分别 `move_decorator_`、`_`、`_`，三参数为十进制ID、偏移后的Vec3 Debug、原Rank Debug。显式None仍先进行pose/name计算，最后不追加工厂，保原分支次序。

## 四分支与几何

入口从完整Affine3A读取矩阵和平移，沿原四项比较恢复矩阵转Quat，保原加法顺序/有序比较，不归一化basis、不把scale从矩阵中剥离、不用缓存rotation替代原转换。`offset_forward`为真时平移加原Affine Z列乘`0x3e3851ec`（.18），不是由已归一化Quat重建forward。非单位输入也保原结果。

- Mesh：display深拷贝名字到owned Arc闭包；wake再次克隆为原Named mesh，使用原rotation/position，animation_duration固定0，真实builder后插DrawTopology::Alt(3.0)。
- Lines：display克隆本地点Vec进owned闭包；wake再克隆给原Points mesh，使用同rotation/position和线宽3。点不预先套Affine，不会被重复变换。真实builder创建TransientMesh，回收沿已有唯一registry。
- Sphere：使用严格命名资产`temp_sphere`，position与入口一致，rotation保持builder原identity，radius经原scale保存。`animate`只控制duration .1/0，不改变Mesh/Lines，也不与forward门耦合。没有额外line topology和TransientMesh。
- None：无factory/实体输出。

三分支均复用现真实SignifierFactory、SignifierEntityBuilder、UiSignifierStream和shader查找，未用CollisionOutline/DebugLine冒充主轮廓，未添加一套近似渲染算法。捕获String/Vec只按原两阶段拥有语义clone，没有每点堆分配或动态geometry转换；内核无分配。

## 原指令与真实World验证

`probe_m01_outline_pose_native.py`在Unicorn执行发行`140b33b56..140b33d6c`，没有业务callee替代。128行覆盖四项矩阵分支、两forward门、非均匀scale/shear、确定性随机有限矩阵；测试逐位比较四元数4word和位置3word，128全部相同。原代码页和读取的常量页均来自同一RNE。

`main_outline_display_tests`最终6 passed、0 failed/ignored、3020 filtered、.01s。真实测试包括：

1. 原128向量逐位对照。
2. 三factory原名字与rank、显式None不提交、owned closure而非raw载荷。
3. Mesh/Lines真实builder输出、局部geometry原样、非单位原Quat与forward、线宽3、duration0、TransientMesh回收。
4. Sphere严格名字/负radius保留、identity rotation、独立动画开关、无line topology。
5. 实际Stream→signifier_create→animation_t/animation→再次create去重→active params释放→mark→反向动画→despawn→transient library/VAO清理；weak closure最终无owner，mesh不存在。
6. 缺mesh、缺shader、空Lines在真实消费阶段失败；display不提前查资产或伪造成功。空点原mesh_from_pts入口`140a6c164`分支进入原失败路径，现严格mesh owner保持该失败。

命令使用CARGO_INCREMENTAL=0、既有target、country-core --features native-physics --lib --offline。CPU shader描述符用TestDevice，不是GPU程序编译/绘制；没有OS输入，没有本轮整游戏画面验收。首次测试夹具用了不存在的get_attribute和UiState::default，按实际现API修正后通过，保原失败日志，未更改业务来掩盖问题。

碰撞描边与主描边组合回归结果另记validation.json；中央/rectangle负责实际producer接线与联合帧验收，不将本轮独立叶测试等同宿主已完成。

## 下轮复用经验

- 从caller寄存器/栈实参和callee格式化函数共同裁定字段，info+44是rank；相邻类型名不能靠猜测。
- 原工厂身份字符串属于去重与生命周期合同，格式差异能使旧实体残留/重复，不能用新的Debug格式无声替换。
- 矩阵有scale时，从Affine直接提Quat和先拆scale再用cached rotation不同；用原四分支执行向量裁定，不以“更标准”数学替代。
- 事件producer成功追加不等于画面完成。必须跑真实factory/asset lookup、动画、标记删除和TransientMesh释放，并测资源缺失发生在哪个阶段。
- 首次反编译只760B是坏CFG，原2876B及PDB范围不能截断；独立worker修跳表，shared保持只读。
