# Windows ICC 原 dirty producer V2 合同

V1初始/identity子集包冻结保留，不作完整来源准入。V2基于已合长度修复的当时生产source，以完整B OS callback3283B独审实证纠正，保原quirk。

## 原调用/副作用顺序

原run_event_loop初始化真实window.current_monitor缓存且dirty=true (14026e528..540)。Moved callback140557da1取实际currentMonitor：若cached Some且HMONITOR相等则直接返回，无通知/dirty。否则先send tag5 (dd1..e0e)；成功后只有cached Some且旧/新refresh_rate_millihertz Optional不等，才更新cached=当前 (1405584af/4b7)，随后send tag6（SurfaceStale，140558562..590）；所有正常变更路径最后dirty=true(140558592/596)。cached None不自行填充；刷新率相等跨monitor也保旧cached，这是原观察行为，不擅自纠正。OS批处理尾dirty&&cachedSome才先清dirty再query成功发canonical ICC event，失败warn无event (14026f5ea..613/820..84f)。完整callback唯一dirty setter，无额外同monitor profile hotchange来源。

## 当前映射/可测试状态

在既有icc.rs增加最窄私有MovedMonitorState<M>（M仅Eq），生产M=真实MonitorHandle；状态存cached_monitor与dirty，初值真实window.current_monitor()/true。on_moved接本次实际monitor、按需读取Optional rate的闭包、按原顺序发送通知的闭包。只有真实Moved调用，不做逐帧monitorgetter/LCMS/文件I/O。纯状态泛型只允许无OS句柄的反例测试，不引入另一个窗口框架。

GameLoop新增字段/local初始化/原批次后非零App.update前publish三位置；不碰resize/settings/suspend/shutdown。event_loop原WindowEvent::Moved Windows分支调用tracker.on_moved，通知直接进入已有pending_os_events VecDeque（tag5→MonitorChanged，tag6→SurfaceStale）；这两个variant原queue_os_event没有额外副作用，保持batch owner和原排队顺序。非Windows保持既有Moved→MonitorChanged，Mac不伪造Windows ICC。

当前主线程适配无原mpsc断开；通知闭包仍Result并测试首/第二send失败时dirty与cached更新顺序，不能把失败变成功。Window.current_monitor Windows正常实现Some；若工程API返回None则不构造假monitor/不调用其refresh/getICM，只不产生本Moved通知（平台异常边界，不声称原null HMONITOR路径已执行）。

publish消费真实dirty OR当前App TextureDevice命名空间变化；后者是跨双App必要工程适配。Weak保身份不强留旧App/GPU，先检查已有Weak真实Arc对象地址（Weak保controlblock避免地址重用），仅变化时Arc::downgrade，不每帧分配/原子引用计数操作。无变化不query、不分配；消费dirty/更新owner stamp先于可能失败query。查询对象是原cached monitor，不能改成本次actual monitor，保持equal-rate原quirk。对cachedNone消dirty但无event。成功发送精确路径到当前World已有event owner；失败warn不清Active，送达坏path由核心先清旧。

## 必要反例/验证边界

纯测试覆盖sameMonitor无rate getter/通知/dirty；equal-rate跨monitor重复tag5保旧cache；rate不同tag5→cache替换→tag6→dirty；cachedNone不填/仅tag5；tag5失败不变state、tag6失败cache已替换但不置dirty；batch consume一次、查询失败不可每帧retry。保V1三个publisher测试与原长度四测试。后续root编译Mac与实际Windows验证，作者不执行Cargo/GPU。原Windows目标行为与MacColorSync策略不混称。
