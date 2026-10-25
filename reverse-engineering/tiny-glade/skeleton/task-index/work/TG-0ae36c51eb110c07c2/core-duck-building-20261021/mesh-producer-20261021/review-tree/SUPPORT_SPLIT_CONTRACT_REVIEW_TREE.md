# 支撑分段完整合同独立复核

结论：批准按本合同恢复原3075B方法；这是源码准入，不是实现、Cargo或GPU验收。

独立检查：读取完整660条指令（含展开清理）、真实RNE逐条连续字节对照、PDB437:119232的原3075B边界，均匹配。原caller五参归属与canonical wall_normal、native-physics短射线API相容。没有修改作者合同或其源码。

逐项确认：
- sret三Vec依次为unsupported、supported、width-runs；初始仅首width/2，循环仅内部点，单点合法且首末width各追加一次；空width在任何几何前失败。points空不被新增长度门吞掉。
- 原anchor来自第5参。每点wall_normal调用先于tangent bounds，随后是point与width bounds；符号比较为严格有序，unordered取+1。归一化仅finite正reciprocal有效，失败清零；保乘加关联、signed-zero和非有限值传播。
- origin高度偏移原bits bf95c28f，TOI原bits3fe3d70a，mask=1；同实际world slot0查询，无wall-id排除，无假命中替身。
- 命中侧距先z平方再加x平方开方，最后加width*(-.5)，无新增clamp/finite门。hit/miss四转移、首尾不测、共享boundary、单点range、最后取实际widths.last而非points[end]均与原控制流一致。
- 首次miss在i写supported(start,i-1)，恢复hit在i写unsupported(last-boundary,i)并新建宽度run；末尾unsupported不加端宽，supported才补末宽。当前合同没有删除原单点或重叠边界。
- 原方法只查询world。输出分配和失败清理属于标准Vec/依赖复用；合同未把现代OOM路径宣称原机器码等价。

源码验收仍需：真实RaycastWorld+flush覆盖全hit、全miss、hit-miss-hit、mask排除、短TOI，精确验证三个输出；长度1/2、尾部额外width及失败顺序。须显式核raw.y=-0等浮点关联，不以Vec3通用normalize替代；不允许旧VecRange单输出壳。未编写/执行这些测试，不宣称它们通过。
