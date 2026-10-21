# 矩形角窗边方向必要原语

只在现有 Rectangle2d 上恢复 get_edge_dirs_from_corner_id，角窗后续调用同一 canonical 类型。root 已明确窄委派；task live show 为 TG-6e76d82e8a953a9a45，owner 保持 boot-leaf-20261018，无 claim 修改。

原 B 140c9a210 完整598B/原 PDB 768:21952。所有指令字节逐条与绑定RNE对照，原跳表四项和literal均直接取文件；角点名称和分支对应保存在 NATIVE_RANGE_PROOF.json。先检查 area>0，再检查0..4角序，分别计算前一角/后一角减当前角、y²+x²、sqrt倒数和乘法。原版对世界角点消减到零的情况产生NaN，不引入零长回退。

复用现有 as_points2 的同一世界角点，不复制一套轴/方向算法。原机型任意NaN payload精确位模式不主张跨目标一致；返回 [Vec2;2] 是当前 Rust 安全接口，调试占位本身未证明原 Rust 名义返回类型。计算世界点在index检查前发生，而原版在检查后；此纯计算不引入资源写入或额外panic，原 area/index 失败优先级保持。

新增6项真实边界测试：四角顺序、旋转、双负尺寸正面积、大坐标消减NaN、面积先于下标的失败顺序、越界。作者未运行Cargo/测试/GPU；root 唯一整合及验证。此方法不等同角窗 producer/编辑/卸载完成。
