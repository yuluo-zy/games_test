# 依赖未定，当前不是可编译冻结交付

本source已证入口/门/entry/pose与3真实World测试候选已备。唯一缺失canonical API为 `RaycastMeshData: Clone`，当前production shape仍Option<Box<dyn Shape>>；原要求同inner SharedShape Arc。Root必须恢复真实owner/Clone后才可编译成对验收。worker未私造Clone、未创建第二primitive、未将同outerArc充作原克隆。

配对circle叶函数after2a14ff8248f7ef0133dcf60192336b4cafecfdb4df83a18aa180da2f6067e20d（独审通过）；其SomeNaN保tag行为是trap实际同圆门必要输入。旧后续hole/visual仍是显式已知欠账，不能计完整视觉恢复。
