# 测试适配独立 SOURCE 审查

结论：限定hunk通过，可与canonical visual262f和必要调用者成组交root实际CPU验证。审查者未编写该hunk；无候选/生产修改，无Cargo。

tiny-glade/crates/systems/decorator/tests/decorator_unload_lifecycle.rs

before `64eb7ca17e3ac0e143be378815c22862ce1781d0fa24915581c8649af500e5d4` → after `ebdfc7b8a1231c8f252cda87c6f9c3080a8b25dc4fb09818b1fb56606e129b56`；delivery `4c0e42784f297c5fcd7efefb3c77b49046f3f972e42e1bde9347242089cf145f`；patch `5d12c366b5c66462a2ec316c2c05782f19c28291830aac9199f1a2c23fd21e45`。精确字节和重放返回码见 SOURCE_REVIEW_WATER.json。

仅卸载测试的空视觉条目setup；保graph(edge,0)、真实registry初始化和全部断言。

已读实际测试上下文和断言目的，逐字验证只发生合同中的setup替换。canonical visual.edges_mut().entry(edge).or_default()建立真实默认空状态（空实体/空cache/无allocation），不会调用原生产reset；它是此测试判断表内存在/缺失的合法最小域，不是假geometry，不为了编译保留无Commands两参数API。空record没有GPU/实体物权；本测试不以其作为完整重建或资源释放证据。

原reset必须Commands+Brick/Sparkle/Wooden真buffer，Tree本源三个实际测试已另独审，覆盖四allocation/持锁/Commands/Buffer Arc最后release。不能把这里的空setup升格为那组生命周期验收。

独立隔离副本对每个交付patch执行正反 patch -p1 --fuzz=0 --batch，全部exact；Rustfmt2024/skip_children解析0。未跑实际测试，root仍需完整atomic包验证。

卸载test本身验证各canonical owner重置、外部Arc保留、Queued topdown removal、既有实体不由visual ID表销毁及无audio发送。改前无资源reset只为造一个表条目；替换后仍有同一实际空条目供resource replacement drop，未删任何断言。原graph(edge,0)与peer必要StairColliderRegistry初始化完整保留。

另独立重放combined-from-graph10bffbae patch（只叠peer registry初始化和本空entry setup）及post-registry64eb patch，均到同ebdfc7b8 exact；未把已含registry基线的patch重复应用给root。
