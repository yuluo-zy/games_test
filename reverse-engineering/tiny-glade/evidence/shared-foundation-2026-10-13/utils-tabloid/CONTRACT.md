# Tabloid 与 utils 本轮交接

## 本轮交付范围

原作 B；行为恢复候选而非作者原始 Rust 文本。tabloid 原目录仅 `crates/tabloid/src/lib.rs`，当前交付同路径完整可编译候选 crate。utils 顶层源码保存后按父 agent 的新排程交给后继；没有编辑 utils/lib.rs、根 Cargo 或全局台账。

tabloid 编译与三个合同测试通过，严格 `cargo clippy -p tabloid --all-targets -- -D warnings` 通过。原指令已读取并据此恢复策略，但尚没有整包原生差分执行、真实游戏 DirtyTiles/ColliderDiff 接线或原版本依赖锁完整验收。

## Tabloid 原作证据与算法

- `Publisher::publish` 的 B DirtyTiles 入口 `0x141cf6500` 和另一份同逻辑 `0x1425db920`：sequence 从 +0x30 读出后加 1；先对已有 publications 运行 retain 闭包。共享批次中仍有 pending 订阅或 reader 的 publication 接受每一后续 delta；publisher 独占的 publication 被移除。
- retain 容器 `0x141bb0250` 属于 std，**没有重写该库算法**；其游戏闭包 `0x141bb03f0` 才是业务证据：strong_count==1 时丢弃，否则写锁→`StateDelta::apply_to`→publication sequence 更新。DirtyTiles 的 `State::empty` 是原函数 `0x140960ac0`，`apply_to` 是 `0x140960ef0`。这些是实际游戏类型消费者边界。
- `publish` 对各 subscription 只在 pending 为 None 且 last_sequence<current 时分配一个本次共享批次；State::empty 之后应用本次 delta。已有 pending 不替换，所以不是 latest-value channel。
- `Publisher::subscribe` `0x141247b10`：新 subscription 的 last_sequence=0、pending=None，添加强 Arc 到 publisher 的 subscriptions。不会补历史状态。
- `Subscriber::take` `0x141247c90`、`0x1412929e0`：在 subscription Mutex 下取出 pending 并清空；读 publication sequence 更新 subscriber last_sequence，再获取长期 read lock 返回 reader。持有 reader 时原 publish 写锁会等待。
- ColliderDiff publish `0x141453350` / `0x141453c00`、subscribe `0x141454290` / `0x141454410`、take `0x141454590` 是同机制的第二个游戏消费者族。

原字段类型/布局不是从 PDB void() 占位推导。当前公共安全 Rust API 以 owning read guard 表达生命周期；ABI、作者字段名字、trait 定义的全部 bound 尚未证明。`new`、`Default`、公开 sequence getter、Clone API 是重建的安全使用表面，不能从存在这些 API 宣称作者同名源文本已恢复。

## Tabloid 限定验证

1. independent consumption：fast 消费一次、slow 保持旧 pending，第二次发布得到 fast=[9] 与 slow=[7,9]；新 late 订阅不获取历史。
2. clone：克隆共享 subscription 的 take 状态；一次 take 后另一个 clone 无 pending。
3. concurrency：活 reader 阻塞 concurrent publish；drop reader 后写线程完成，后续订阅看到第二批。

这些是候选合同测试，并非原游戏完整 runtime 证明。原发行 RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；原 patched MCP B SHA256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`。raw instruction 证据见本目录 `*.asm.txt`、`native-manifest.json` 和 `B-symbols.json`。

## 依赖版本核对

tabloid 直接使用原 parking_lot=0.12.1；源码只调用 dependency API，未重写Mutex、RwLock、Arc、Vec。首次测试曾使用较新transitives，父owner已收到通知。收尾时共享锁已改变，最新 `tabloid-test.log` 明确编译原lock_api0.4.9、parking_lot_core0.9.3、smallvec1.13.2、scopeguard1.1.0、cfg-if1.0.0；同一三个合同测试再次通过。严格Clippy也已在该锁下复验通过，owning read guard表面确实兼容原lock_api0.4.9。机器报告逐dependency比较原版本/source/checksum，而不从direct pins推定完整workspace锁验收。

## Utils 所有权交接与未完成

后继可接管 `reconstruction/shared-foundation/crates/utils/src/` 顶层所有 .rs（除lib.rs由父管理）；本agent结束后不再写。源文件均已落盘但**整个utils尚未编译**，函数证据程度不均匀。

- 数学原码 fixture 六族（rng44、LoopIdx50、CircularF32 lerp50、AngleRange clamp50、EasingFunction119、rgb9e554）已真实执行B原RNE，用CRT函数hook仅复用fmodf/cosf/sinf/exp2f。`native-fixtures.json`保存输入和原结果，六文件中 `native_oracle` tests 已嵌入；**这些Rust测试还没运行**，不得计candidate/native diff通过。
- `loop_range.rs`：新发现原context其实是period，LoopIdx::index用signed rem_euclid；旧52候选opaque字段已纠正。next()和ExactSizeIterator是安全适配，仍须限定ABI/异常行为复核。
- `angle_range/circular_f32/easing/rgb9e5/rng/threshold_range`：原算法候选已落。transcendental rounding、NaN、signed zero、angle ties、serde wrapper normalization尚需fixture裁决。
- `timer.rs`：delta filter真实规则已恢复（tiny delta不更新、1.2上限、warmup包括0、后期4样本平均）；原初始resource状态与Bevy SystemParam注册/producer仍未知。现在不是原generated apply/queue no-op替身。
- `index_condenser.rs`：原remove已取证；stale cached_index查map并写回；swap_remove后修正moved key index。insert/resolve 是明确producer adapter；作者原通用键/handle定义和inline生产者还需查。
- `float_grid.rs`：固定512/其他width的sampling-fix与legacy策略已读原码，grid load使用trait消费者；legacy边缘有extrapolation、fixed texel-center及NaN边界需差分。原全grid trait/interface/flag producer尚未闭合。
- `comparative_space.rs/wall_space.rs`：project/project_clamp/project_unbounded、单次环形subtract、distance候选；WallSpaceCoord curve interface以 `CurveSampling` 显式桥接，父Curve2未接。nearest-u及tangent/extension边界仍需原码复核。
- `camera.rs`：reverse-Z infinite projection、matrix eye helpers、lens字段初始化已读原码；xz rotation目前是atan近似，**必须改为原normalize/basis→Quat::from_mat3表面或父roothelper**；不得宣称该函数行为验收。body matrix layout、乘法rounding、Lens smoothing/initProducer未知。
- `natural_cubic.rs`：只有2D/3D polynomial evaluator、3D clamped t/segment选择与clone；control-point/tangent constructor和periodic scalar solver未恢复。spline2d/spline-scalar旧临时ReVa反编译保留，但其推断types很差，不能冒充自然样条完整实现。
- `rdp.rs`：基于原3D constructor candidate写入，含unbounded-line distance、spacing、root force、left-span<=2 shortcut；尚未编译/差分；Vec2泛型适配等价未证明。
- `name_hash.rs`：原xxh3 seed0/default192byte secret，精确复用xxhash-rust0.8.10；debug_name字符串为 `(NameHash)`。需实际producer类型接线。
- `file.rs/local_time.rs`：已核原日期格式 `[year]-[month]-[day] [hour]-[minute]-[second]` 和backup保留原filename再加 `-date.json`；初次offset缓存失败UTC回退。filesystem异常、原日志与copy result类型仍需原码复核。
- `halver.rs`：只调用half2.4.1；原有Vec3/Vec4/Quat/array实例已定位，Vec2和泛型array扩展是适配，不能计原all types证明。
- `selective_deserialize.rs`：只恢复wrapper吞错/default策略，并写了field-selector适配。原两实例还有嵌套 `History`→`snapshots` visitor路径：需解明其field marker/schema及partial-success行为，当前不能直接记完整存档GC selector恢复。

## 工具状态

已弃用当前Codex临时ReVa项目。新Windows operator使用独立worker `foundation-utils-tabloid`，limited prepare_functions 原字节SHA约束后查询成功，见 `selective-persistent.json`（ok=true，完整stdout及checkpoint保存）。旧临时库的1byte body、错误粗略size和OverlappingFunctionException报告保留为历史失败，不作为当前源证据增强。父工具修复已有独立验收，本agent不再开展工具修复或重复全程序分析。
