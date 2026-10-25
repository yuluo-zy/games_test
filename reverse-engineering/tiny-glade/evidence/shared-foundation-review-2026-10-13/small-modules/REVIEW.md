# 小型共享模块代码 review

范围：B 版 `paths`、`lang-lib`、`tabloid`、`theme-defs`。检查恢复源码、原作 LLVM/PDB/指令证据及原锁依赖；不以候选测试通过宣称整个模块与原游戏调用链已验收。

## 已确认并修复

**SM-001，P2，LanguageLibrary::get 缓存命中额外分配。** 原实现先执行 `cache.entry(key.to_owned())`，即使缺失键已经缓存，每次查询仍复制一次 key。原入口 `0x141848140` 中，缺失键缓存命中分支 `0x1418483b8..0x1418483c9` 直接进入解锁/返回，跳过格式化与分配分支 `0x14184841c..0x14184859c`。现改为先使用 std HashMap 的借用查询，仅首次未命中才创建 key 与占位文本，保持全局缓存、首次日志和文本引用稳定的行为。

独立分配探针在缓存预热后执行 10,000 次查询：修复前 10,000 次分配，修复后 0 次；每次均检查返回地址一致。这里只证明该查询路径的分配变化，没有测量整帧耗时。探针属于验证代码，不是恢复的游戏接口。

## 功能完整性与保留项

- `paths`：十个已观察接口的 Windows 路径策略有恢复；真实启动链、其他平台、全部错误字符串和类型仍未完整验收。检查创建顺序、部分失败保留目录、Steam ID 路径、override、KnownFolder/USERPROFILE、current_dir 时序与 OnceLock 使用，未发现新的确定偏差。
- `lang-lib`：十一个已观察接口已有部分业务分析；库替换、 fallback 环境开关、缺失文本缓存、首个 YAML 文档和 schema 错误策略与已有原指令证据一致。`LanguageLibrary::new` 保持传入 locale/map 的逻辑未因本次 get 优化改变，构造测试重新通过；作者所有权/类型与整条原 UI 消费链仍未验收。
- `tabloid`：六个已观察接口中 publish/subscribe/take 已有业务分析；私有 Default/Drop/Deref 仍保留三项证据闭合缺口。两次 read-lock 的 take 顺序确实存在于原入口 `0x141247c90`，不将其当作恢复代码多余操作删除。持有 reader 阻塞 publish 为原锁语义。
- `theme-defs`：四个已观察 Clone/Eq 接口恢复了数据载荷子集；Error 的 Display 文案、完整生产/消费接线和协议形式尚未证实。不得将推断文案当作已恢复原协议。

## 性能与内存风险

**SM-002，P2，原作同样保留订阅者的生命周期风险，暂不修改。** `Publisher` 保持 subscriptions 的强 Arc，drop 最后一个外部 Subscriber 不从集合移除；每次 publish 仍访问全部 subscriptions，未消费的 publication 持续累积。原 subscribe `0x141247b10` 明确把强 Arc 加入 Vec，原 publish `0x141cf6500` 遍历整个 subscriptions，未发现用于清理该集合的分支。频繁创建和丢弃订阅者可能导致内存与发布工作量随总订阅数量增长。结论基于源码/原指令静态审查，没有整场景 benchmark；消费者实际订阅生命周期尚未恢复，不能断言原游戏必然泄漏。改成 Weak/自动退订会改变已证实行为，先登记风险。

**SM-003，P2，语言热替换的累计文本内存。** 返回静态文本要求重建 API 保留 leaked 文本；原指令也显示文本所有权被转移/泄漏。反复加载语言包仍会累计保留文本，不擅自回收返回中的引用。没有证实普通游戏会高频加载；原生命周期与实际运行内存仍是缺口。

## 依赖复用

std 路径/HashMap/Arc/OnceLock/时间、known-folders、yaml-rust2、arc-swap、once_cell、parking_lot 和 thiserror 均直接使用依赖 API，没有重写其实现。路径拼接、fallback 选择、publication 累积与主题区间判断属于游戏调用策略。示例里的 System allocator 包装仅用于统计分配，不替代运行时分配器。

原 tabloid 指令出现 parking_lot deadlock resource 跟踪，当前 manifest 只显式开启 arc_lock，原完整 feature 配置及泛型 bound 仍未核实；登记为依赖配置缺口，不能仅凭版本相同宣称全部同步诊断功能相同。

## 验证

四包共 9 个候选单元测试、严格 all-targets Clippy、分配回归探针均通过（locked/offline）。日志、当前源码 SHA 和原证据 SHA 见 `validation.json`/`review.json`。本轮未执行完整原语言/主题/publication 入口；历史报告原样保留，不替换旧源码 SHA 冒充本轮差分。
