# 测试适配独立 SOURCE 审查

结论：限定hunk通过，可与canonical visual262f和必要调用者成组交root实际CPU验证。审查者未编写该hunk；无候选/生产修改，无Cargo。

tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs

before `e36c2d847e15665a660c0b102164cd57206ecefd78c7159b802a062031beee4f` → after `4010988159a545bd1223a47e5186584591c13b281e133b06720e127fc1aad470`；delivery `4859a7b0c90aa05af640cd93e52bf577b02e99b30a96284699be2ce786d6372c`；patch `1e1f879b322f75c558e0651aa54905c4ae2b2f4c0fa2da92370869f443621941`。精确字节和重放返回码见 SOURCE_REVIEW_WATER.json。

仅interpolate测试两个setup；真实空条目存在/缺失门，保真实请求producer/全部断言/force与global policy。

已读实际测试上下文和断言目的，逐字验证只发生合同中的setup替换。canonical visual.edges_mut().entry(edge).or_default()建立真实默认空状态（空实体/空cache/无allocation），不会调用原生产reset；它是此测试判断表内存在/缺失的合法最小域，不是假geometry，不为了编译保留无Commands两参数API。空record没有GPU/实体物权；本测试不以其作为完整重建或资源释放证据。

原reset必须Commands+Brick/Sparkle/Wooden真buffer，Tree本源三个实际测试已另独审，覆盖四allocation/持锁/Commands/Buffer Arc最后release。不能把这里的空setup升格为那组生命周期验收。

独立隔离副本对每个交付patch执行正反 patch -p1 --fuzz=0 --batch，全部exact；Rustfmt2024/skip_children解析0。未跑实际测试，root仍需完整atomic包验证。

两个精确替换都在cfg(test)模块。generated_world仍先调用真实请求producer得到canonical Params，随后仅种空可视缓存条目；seed_previous仍由真实interpolate首帧发布历史。第二hunk只从同一map移除此无实体/无allocation的条目，以触发visual-missing复用门；不是在测试remove_edge的析构副作用，所以不伪造Commands或三buffer。force/global laziness另两分支、全部断言和全局policy guard未变，生产prefix逐字相同。
