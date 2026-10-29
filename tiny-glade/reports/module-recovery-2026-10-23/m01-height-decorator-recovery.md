# M01 高度编辑装饰物消费者复原

2026-10-26。原decorator_on_wall_height_changed14081b5e0（818B）、closure14081b920（401B）、iterator/get_mut/attachment三个helper均已新鲜只读取证，见本目录m01-height-decorator*-native.json/asm。替换同原路径假事件，不另建装饰物功能。

## 六参四流

原类型来自DecoratorPlugin→五成员普通元组→into_configs1401691d0：PublicWalls、AdjustWallHeightCmd Reader、AdjustWallHeightUi Reader、FinishAdjWallHeightMode Reader、ReanchorDecorators Reader、DecoratorStorage mut。

1. UI流排空，只对最后一条wall执行closure（14081b6b0）。
2. Finish也排空，只对最后一条wall执行closure（14081b841）。
3. 两者都空才遍历Command全流；有UI/Finish时不消费Command，留待后帧（14081b729..734）。
4. Reanchor全流始终逐条处理（14081b850..909）。

新增原resources/events.rs的ReanchorDecorators(pub WallId)，由中央App.add_event并按原set25挂六参消费者。删除无任何生产者的WallHeightChangedEvent{wall,extra}，不加兼容转发掩盖未接线。

## 修复旧错误

原impl$3::next从Decorators+0x30完整map取条目，+0x60是MaybeMut写时跟踪的modified集合。旧版本误判为仅遍历modified，会遗漏高度变化后原本未修改的装饰物。现遍历全部map，只有side0标modified并重算，side1不写，其余原不可达panic。拆借两个字段，不分配整桶ID数组，保持原逐项副作用顺序。

14081ba55读取Info+0的u供attachment查询，+4是绝对y；14081ba67..70写 `relative_y = coordinate.y - attachment_height(coordinate.u)`。旧实现错把y当u。真实World fixture使用非均匀arch_segment_heights检出此错。原140b2a890实际只用u，callee会覆盖caller传来的XMM2；保留已恢复的get_wall_attachment_height(u)完整曲线投影与插值算法。

缺桶、空桶、仅side1记录不会强行查PublicWalls；side0先标modified再查询，错误仍按原时序失败。

## 原执行与World验证

m01-height-decorator-reader-native.py隔离执行原完整818B，唯一业务边界是closure记录wall。32例×2帧=64帧覆盖old/new双缓冲、UI/Finish最后一条、Commands抑制及后帧延迟、Reanchor全量。JSON保留原调用序列及游标，TXT为不增加Rust依赖的测试fixture；脚本同时重建两者。

World以持久Schedule对照每帧真实modified墙集合，同时验证坐标不变、relative_y正确、terrain不标modified和缺桶不查缺墙。没有OS输入，不冒充真实窗口验收。

2026-10-26实跑 `CARGO_INCREMENTAL=0 cargo test -p system-decorator --lib --offline decorator_on_wall_height_changed -- --test-threads=1`：**5 passed，0 failed**（完整日志`/tmp/m01-height-decorator-test.log`）。上轮target曾写proc-macro2遭ENOSPC，中央清理已停止编译的incremental后恢复；现有通过结果来自真实重跑。

经验：区分iterator遍历map与MaybeMut附带modified指针；区分last、read及完全跳过reader分支，跨帧未读游标本身就是业务。
