# caller min3 原门与格式字面补充

原身份同 TGf768/native-pack：B RNE f507a6ce、PDB2d9686b8，未运行target。

true280875加载control points Vec len，28087c cmpq$2，280880 jbe2829bc；后者282a23 panic_fmt，PDB本stairs.rs line817。28089b才调用周期ctor。false279255/27925c/279260→27b38f..b3f6同门，27927b才调同ctor。精确两份slice在本目录，完整原包仍governing。

原 fmt fragments table1429a9a08=(ptr1429a99e0,len36)，真实36B UTF8：`A loop must have at least 3 points; `，hex41206c6f6f70206d7573742068617665206174206c65617374203320706f696e74733b20。原格式参数 data指向path VecDeque，formatter140511ce0由PDB209:11220确认为 VecDeque<DecoratorId>::fmt。原Location1429a9a18是本source line817 col17。故caller并非多余的现代断言，不可只留ctor门；v2用真实literal与原typed DecoratorId队列Debug输出。只是scratch队列从等价u32 adapter转回typed ID，不改变图key/排序/遍历算法或增加分配。

3处fixture跨bucket迁移已先用真实apply_decorator_remove旧addr/id，再place新bucket；业务断言与合法重复控制点失败回退测试全部保留。原v1依赖旧d2s.place bug的setup过期，不能作为通过结果；本轮未Cargo。
