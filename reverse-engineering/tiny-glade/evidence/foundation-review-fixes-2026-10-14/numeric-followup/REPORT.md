# 三项数值失败的后续修复

本轮只审查源码、既有测试/记录和独立数学参考，没有新增原作逆向查询。

## 原因与处理

- 零长度切线：允许重复点，但 normalize(0) 在 ARM64 与 x86 上生成的 NaN 符号不同。Vec2/Vec3 的 CurvePoint::normalized 对精确零向量直接返回所有分量为 0xffc00000 的 quiet NaN；非零次正规向量仍进入原 normalize，避免平方下溢误判。没有修改 tangent 原样例或放宽其断言。
- 圆形：既有样例由 Windows UCRT atan2f/sinf/cosf 生成。atan2f 中间值 1 ULP 的误差经 1-(angle-rotation)/TAU 放大为 u 的 4 ULP；此前首个失败也遮住了后续两项 sin/cos 输出 1 ULP 差异。优化构建还观察到单位向量 1 ULP 经半径缩放与圆心加法相消放大为位置 4 ULP；因此位置采用误差传播界，不用固定输出 ULP。保留原数学公式和样例值。Windows 仍逐位比对；其他平台仅圆形含超越函数的样例允许单位法线/切线 1 ULP、位置绝对误差 EPSILON*(abs(radius)+max(abs(actual),abs(expected)))、u 绝对误差 2*f32::EPSILON。矩形及其他纯算术断言保持逐位。
- 弹性缓动：现有乘法顺序与记录一致。独立 mpmath 80 位十进制精度计算 sin/2^x，再按原 f32 步骤舍入：wave=0x3f44c1da、power=0x3c56d8cf，最终=3148161199；Windows 样例=3148161200。这不是已证实的算法缺陷。Windows 保持逐位断言；其他平台仅 ElasticInOut 的既有样例允许 1 ULP。没有修改其原预期值，其他缓动分支仍逐位。

此前“所有容差不变”的承诺被这次合同纠正取代：跨平台测试通过不等于复制 Windows CRT 全部位模式。没有重写或移植数学库，没有增加生产依赖，也没有跳过任何测试。Windows 的严格分支本轮未执行；不宣称 Windows 原版逐位验收。

## 新增验证

numeric_contract.rs 新增 3 项测试：Vec2/Vec3 固定 NaN 与非零次正规向量、5 个旋转下共 1280 个圆形参数的独立 f64 参考、缓动问题输入的高精度参考及端点。高精度参考值独立保留，不替换旧原作样例。样例生成器同步相同平台合同，防止再次生成旧测试合同。

## 结果

macOS ARM64：utils debug/release all-targets 各 93 项通过（90 单元+3 集成），0 失败/0 忽略；三库严格 all-targets Clippy 通过；修改 Rust 文件格式与 git diff 空白检查通过。asset/asset-pipe 本轮没有生产代码变化，上轮各 15 项通过记录保留。原业务覆盖库存与完整原版等价状态不升级。
