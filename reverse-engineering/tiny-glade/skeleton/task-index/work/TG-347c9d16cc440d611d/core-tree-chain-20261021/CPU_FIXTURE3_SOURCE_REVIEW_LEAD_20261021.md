# 当前CPU三份窄测试修复独审

通过源码/原证独审，无生产算法变化。

Chimney三fail共同在fixture构造期间调用required max_y尚无heights，原producer根本未执行。原1408133ca required max_y、第二pass requiredmesh/缓存/strictdistance/self后置已由water及duck分别核原，生产门不应放宽。局部对真实PublicWall prepare_paths后再读height，保公用helper与所有阈值/断言。

凸包负例原三点合法平面，锁定Parry0.17 dimension2会构双面凸体；不可新增体积过滤。只改第三点共线，使真实from_convex_mesh重复索引失败，原expect/18已提交键断言都保留。仅用current0ce一行hunk，root新增2真实资产测试未变。

新stairs失败为ARM/x86算术NaN符号差异，root已明确按跨ISA窄测试处理。两assert只屏蔽signbit仍精确要求quiet payload7fc00000，其余raw metadata位比较、-signum生产体、阈值/finite/Infinity/零断言未改。

3源按actual current before隔离F0/hash过。特别NaN patch header是a/stairs.rs，root须在decorator_visual目录patch -p1，不能按repo根错误定位。manifest列cwd。未Cargo/runtime；修复后是否通过由root真实运行，不把F0当通过。
