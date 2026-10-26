# TB-R7 前置：聚焦墙／屋顶高度控件

本轮恢复原 B owner `country-core/src/systems/ui_focus_mode_shape_height.rs`，覆盖原主函数 `140a623e0` 的正常业务 CFG、两个 607B 工厂 `140a647e0`／`140a66a30`。PDB 主体为 9215B；Ghidra 返回 4083B 且跳表之后串入其它代码，故按完整有界指令核验。B 骨架仅提供来源信息，原作者源码 SHA 为 `cfaeef81368a106da04ab7d21c7224cec7b1650fecb798cd728168804a6f7be7`，不把骨架当原实现。

## 恢复内容

- 仅聚焦 Wall／Roof 时工作；无其它模式时清空原 Local，换墙、Wall→Roof 不强制跳变。
- 使用发布轮廓原数组，按已维护的 view／projection 计算屏幕最右轮廓点；保留屏幕初值 0、严格比较、首点兜底、最低高度归约、`max_y-0.75`、相机水平偏移 1.5、20/s 时间连续平滑，以及 NaN／有符号零行为。没有过滤、裁剪或备用轴。
- 高度球和抬升球真实登记到 UiState；拾取半径 0.5、关闭世界遮挡、保留历史标志。高度箭头门槛严格 `>0.7/<14`；抬升箭头严格 `>0/<60`。球视觉缩放 active=0.5、inactive=0.35。
- sphere 使用 `temp_sphere`，arrow 使用 `brush_arrow_flat_single_up`，通过真实 SignifierEntityBuilder／Factory／stream，时长仅改为 0，保留默认动画与渲染通道。工厂名字的分隔符逐条核对，不以 raw qword 或空闭包替代。
- Started 发 Adjust(wall) 再发 Init；Ongoing 发 Adjust(wall)；其它结束字节发 Finish(wall)，None 不发。原事件槽 Finish／Adjust 为 16B，Init 为 8B；本轮暴露出旧 Finish 缺 WallId，中央已修公共类型与原消费者。
- 每帧以实际 elevation widget ID 发 HandleRaise，后续拖拽、权威墙、历史、音频及提示交由原 impl_raise_shape。未直接伪造历史事件。
- 公共墙 +280 的来源与 daily_schema 交叉核验：`140a8d825` 读取 source+58，`d904` 写 stack+780，`dd16` 以 stack+500 为 PublicWallState 基址拷贝，所以读取 `authority.elevation()`；height 箭头使用公共 +284 约束高度，两者未混用。

## 原侧与 CPU 验证

所有原指令绑定 RNE SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；匹配 PDB SHA `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。详见三个 native JSON 与脚本。

1. 原 `140a626c8..140a6296a`，含完整原 eye_right／xvy，无桥、无跳过指令：512 例目标／偏移，有限浮点逐 bits，NaN 比较分类。输入使用原 eye_right 的实际输出；不宣称整个当前相机系统全域等价。
2. 原旋转两个独立片段与完整 xz_rotation_bad／eye_right，无依赖桥：512 例、511 个不同相机方向，up／down 四元数有限值逐 bits。
3. 原 `140a63294..140a63405`，2048 个 hover／id 匹配／phase 组合；正常已预留容量，不走分配分支。原 EventId 与 WallId 载荷都实读核对，再与真实 UiWidgetContext→ECS 事件比较。
4. 定点 7 个测试全部通过：真实登记、所有拖拽分派、原阈值、同 Schedule 持久 Local 退出／重开、真实工厂 mesh／变换／动画、实际 signifier_create→mark→反向动画→despawn→动态 DebugLine mesh 回收。捕获 WidgetParams 的 Weak 已归零，静态 sphere mesh 保留。
5. Demo owner 已把 Focus producer 加入真实 impl_raise_shape 联合测试：聚焦球 Started／Held／Finished 驱动权威 elevation 0→60、原最大提示、sheep 与待提交 ChangeElevation 历史。其 2 个测试通过，日志另存。
6. 既有 utils 相关 xz 过滤测试 8 个通过。

固定源码／测试二进制 SHA 及稳定性检查在 `toolbar-focus-height-bound-test.json`；最终定点运行前后 binary SHA 相同。编译通过不是整模块完成的替代证据。

## 发现并修正的公共算法错误

最初 512 行旋转样本只有 4 个不同方向，不能拿行数当覆盖。扩成 511 个不同方向后，case80 up.y 发现 1ULP 差异：原 xz_rotation_bad 第二次 right 归一化是 DIVPS；当前 glam `.normalize()` 使用倒长度再乘，产生第二次舍入。按原平方求和次序改为 `right / length`，保留第一次 try_normalize，重新通过全部逐位比较与现有 utils 测试。首失败日志保留，未放宽误差。影响所有复用该公共 helper 的消费者；本轮没有另造 Focus 专用近似四元数。

## 集成与验收边界

中央已导出原 core 模块；生产宿主调度、真实 Loading::Reload、窗口／GPU 画面与真实鼠标键盘验收由中央继续，本文不报通过。原 Local 只在非 Wall／Roof 模式实际运行时清空；本地同 Schedule 测试验证该合同，宿主不得在 Reload 禁跑后直接恢复聚焦墙而带入旧平滑位置。真实 OS 输入仍按用户要求暂停。

## 本轮经验

- 原函数 declared size、PDB physical size、跳表界限必须分别记录；同名类型候选不能取第一条就套。
- 字段名只作线索；跨 producer→consumer 的数据流与原拷贝基址才决定公共字段含义。
- 样本数量同时记录独立输入数量；重复相机方向会掩盖公共数学依赖的舍入错误。
- 不能把 normalize、除法、乘倒数互换，也不能把复杂错误消成 epsilon 测试。
- Unicorn 复用翻译块时，until 端点可能不在块入口；以只观察 PC 的 code hook 精确停止，并断言终点，避免越界执行。边界钩子不改数据或指令，不等于业务桥。
- 联合测试使用真实资源发布、真实控件和消费者，实体卸载要检查捕获 owner、动态 mesh 与静态资产各自的实际生命周期。
