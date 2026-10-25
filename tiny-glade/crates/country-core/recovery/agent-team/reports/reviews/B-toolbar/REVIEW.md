# REVIEW — B-toolbar / tiny-glade：B 类重建函数独立复核（硬标准逐语句判等）

- reviewer：`reviewer-B-toolbar`
- 日期：2026-10-17（本轮 fresh 2026-10-18 00:06–00:14）
- 范围：`planning/2026-10-17-evidence-audit-interaction.md` B 清单中 **toolbar 12 项** 与
  **tiny-glade 重建函数**（tinyglade B 20 项；并按任务要求补读/补审 tinyglade D 的 4 项重建
  与 E 的 5 项重建，共 **41 项**）。
- 方法：每函数 **fresh 反编译**（Ghidra shared 只读，`get_functions`
  `signature,parameters,locals,decompiled_code,callees`；B patched EXE，`program=tiny-glade.exe`）
  → 独立复述控制流/调用链/字段偏移 → 对照当前 `src` body 逐语句 → verdict；
  微小函数另用 **radare2 对 B patched EXE 反汇编** 复核（Ghidra 的 `void()` 占位会吞掉小返回值）。
  对照 `planning/2026-10-16-logic-verify-protocol.md` 的 **8 条分歧检查表**。
- **未改 src、未改库**；Ghidra/radare2 均只读。
- 说明：复核期间 `crates/systems/toolbar/src/{mei/core.rs,mei/systems.rs,ui_interaction.rs}`
  在 00:08 被并行 restorer 更新，本报告以 **00:08 之后的当前源码** 为准（并注明该情况）。

## 0. 三态结论

| 范围 | 条目 | 判等（升 C） | 需修（仍 B） | 未实现 |
|---|---:|---:|---:|---:|
| toolbar（审计 B 清单） | 12 | **3** | **9** | 0 |
| tiny-glade B（审计 B 清单 20 重建） | 20 | **0** | **20** | 0 |
| tiny-glade D 重建（补审） | 4 | **1** | **3** | 0 |
| tiny-glade E 重建（补审） | 5 | **1** | **4** | 0 |
| **合计** | **41** | **5** | **36** | **0** |

- 判等升 C：`ComputedLayout::center`、`Node::hover`、`Node::has_focus`（toolbar）；
  `NvAftermathDelegate::default`（D，re-verify 既有 FIX-A3）；
  `gather_pending_video_setting_changes`（E，新增 C）。
- 其余 36 项均有可执行实现，但存在**逐语句可指认的分歧**（清单见 §5），按硬标准仍计 B。
- **无未实现**（没有空壳/todo 落在本清单内）。

---

## 1. toolbar 12 项

> fresh：`evidence/fresh/0x14068*.c`、`0x140fe1*.c`、`0x142344560_*`、`0x142342f30_*`、
> `0x14233b3f0_*`、`0x141e4d880_*`；小函数另见 `results/radare-tiny-B.result.json`。

### 1.1 判等 3 项（升 C）

| 函数 | VA | fresh 复述 | Rust | verdict |
|---|---|---|---|---|
| `ComputedLayout::center` | 0x14068ea00 | `xmm1=0.5; xmm0=[rcx]*0.5; xmm1=0.5*[rcx+4]; xmm0+=[rcx+8]; xmm1+=[rcx+0xc]`（B EXE radare 实测 32B） | `self.pos + self.size*0.5` | **判等**（公式逐指令；注意原生有效字段序为 `size@+0,pos@+8`，Rust 声明为 `{pos,size}`，repr(Rust) 重排后仅在消费面接线时需最终对偏移，登记 I） |
| `Node::hover` | 0x140683c90 | `movzx eax,[rcx+0xb6]; and al,1; ret` | `state_bits & 1` | **判等** |
| `Node::has_focus` | 0x140683ca0 | `movzx eax,[rcx+0xb6]; and al,0x40; shr al,6; ret` | `state_bits & 64` | **判等** |

> 这三项原审计只按“物理 `return;` + getter 内联”记 B；radare2 对 **B patched EXE**
> 直读机器码后，返回表达式可逐指令判等（不是语义推测）。

### 1.2 需修 9 项

#### (1) `MeiContext::reuse_or_create_node` 0x140683ec0 — 需修
native：命中 → 返回既有节点；未命中 → `+0x1b0==0` 时 `reserve_rehash`，然后
`hash_one<RandomState>` + **`RawTable::remove_entry`（身份回收）**：旧节点
`flags & 0xf3`、旧 id（16B）与 4×u32 载荷被带入新节点；再初始化
children(cap 8)/state `{0,0,4,0}`/opacity `1.0`/sounds `0x0d0d`/`+0x5d=0`，len+1。
Rust：`contains_key` → `insert(Node::new(EmptyContent))` → `get_mut`。
**FIX-BT-1**：复刻 remove_entry 的旧身份/载荷回收（`Node.recycled` 字段已有位但未写）。

#### (2) `process_input_action_handlers` 0x140687eb0 — 需修（当前源码 00:08 版）
fresh 复述：`vec=[focus_id]`；沿 `parents` 表（ctrl +0x1d0 / mask +0x1d8 / len +0x1e8）
上溯到缺失，逐个 push；`len>1` 时 **reverse**（root→focus）；正向遍历
**节点表**（ctrl +0x1a0 / mask +0x1a8 / len +0x1b8），仅当节点表非空才探表；调
`content.on_input(action, &state16)`（vtable +0x50，状态字初值 `{0,0}`），首个 handled
记 `local_80=索引`；随后 **从该索引反向** 到 0 再次调用（状态 `{1,handled}`），命中即返回。
Rust：reverse ✓；但两处具体分歧：
1. 循环门用 `self.parents.is_empty()`，原生门是 **节点表** `+0x1b8`（父母表为空而节点非空时，
   原生仍会调 focus_id 的 on_input，Rust 直接 break 不调用）；
2. 第二遍是 `chain.iter().rev()` **全链**，原生只覆盖 `[0, stop]`（从停点回到 root，
   `puVar14 = buf + local_80` 向下）；Rust 会对 stop 之后的 focus 侧节点重复调用。
3. `on_input` 缺第 2 个状态字实参（原生 `&{1,handled}`/`{0,0}`；已登记 U）。
**FIX-BT-2**：门改为 `self.nodes.is_empty()`（且不是 break 整个链，语义按原生“跳过探表”），
第二遍改 `chain[..=stop].iter().rev()`；扩展 `NodeContent::on_input` 的 2B 状态参数。

#### (3) `MeiContext::load_configs` 0x140682e70 — 需修
fresh：`WalkDir::new(dir)` + closure filter（`config::impl$2::load_configs::closure_env$0`）→
`DirEntry::path().extension()`（`try_from` + len 3 + bytes `ron`）→
`relative_to(dir)`（**结果 unwrap，失败 `unwrap_failed`**）→ `to_relative_path_buf` →
`file_stem` → `read_node_config_from_file`（**解析失败 `panic_fmt`，实参含路径 OsStr + ron 错误**）→
`log`（target 字面量 `"toolbar::mei::config: "`，`Loaded UI config from {stem}`）→
`HashMap<String, Arc<NodeConfig>>::insert` **入 self**，旧 Arc 递减/drop_slow。
Rust：`entry.path().file_stem()`（**没有 relative_to**）、解析失败 `panic!("{e}")`（**缺路径实参**）、
`log::info!` 的 target 为模块路径 `toolbar::mei::core`。核心键/插入一致。
**FIX-BT-3**：补 `relative_to(dir)`（保留 unwrap）；panic 文案补路径；log 用
`target:"toolbar::mei::config"`。

#### (4) `config::load_font_table` 0x14068fc60 — 需修
fresh：`std::fs::read_to_string::inner()`；IO 失败 → `panic_fmt`（路径 OsStr + io::Error）；
成功 → `ron::Options{local_118=1, local_110=0x80, local_108=0}::from_str_seed<
PhantomData<FontTable>, FontTable>`，**直接返回 Result**（无 validate 调用）。
Rust：`FontTable::from_ron(&read_to_string(path)?)`。
- **panic-vs-return 反**：IO 失败 `?` 返回 `Err(TextError)`，原生 panic。
- **callee 转发错**：原生直接 `Options::from_str_seed`；`FontTable::from_ron` 另走
  `ron::from_str` + `table.validate()`（多出校验，可能拒绝原生可接受的表）。
**FIX-BT-4**：IO 失败改为 panic（打印 path+io error）；解析按原生 Options 直接
`from_str_seed`，或证明 `from_ron` 与 `Options{1,0x80,0}` 及无 validate 完全等价。

#### (5) `LayoutStyle::place_childen_impl<XY/YX>` 0x140fe12a0 / 0x140fe19a0 — 需修
fresh（去混淆后可直读的段落）：
- XY：主 pad = padding[0]/[2]（父 main 基准），交叉 pad = padding[1]/[3]（父 cross）；
  YX：主 pad = padding[1]/[3]（父 cross），交叉 pad = padding[0]/[2]（父 main）✓ Rust。
- gap 百分号基准 = `main_without_pads` ✓；`available = main_without_pads-(n-1)*gap` ✓。
- 子求和：XY 读 `child.padding[0]` + `style.size`(+0x2c) + `prev_x`(+0x10)；
  **YX 读 `child.padding[1]` + `style.min`(+0x34) + `prev_y`(+0x14)**。
  Rust 的 `sum` 对 XY/YX **都读 `style.size`** → YX 少读 `min`。
- `weight = clamp01(1-sum_expand)*remaining`：原生 NaN 时 `max/min` 序列得 0；Rust
  `f32::clamp` 保留 NaN（低危 I）。
- justify 分派：原生跳表 `0x142bc223c`(XY)/`0x142bc2260`(YX)；Rust 只实现 0..=4 与
  1/2/3 的 start/gap 写入，**case 6/7/8 及 >8 的共享尾区未实现**（Rust `_=>{}`，
  注释写了 6→center 但代码没有）。
- 装配写面：原生每个输出元素写
  `[main_size@-8, cross_pos@-4, u64{pos.x,pos.y}@+0]`（即元素 = `{f32,f32,Vec2}`）；
  Rust 写 `ComputedLayout { pos, size }`（`{Vec2,Vec2}`）→ **字段顺序/语义不符**。
- 装配主轴项原生为 `(expand/sum_expand)*fStack_fc + max(...)`，其中 `fStack_fc`
  在共享段为 `max(remaining,0)`；Rust 用 `(expand/sum_expand)*parent.main`（基数不同）。
**FIX-BT-5**：YX 求和改 `style.min`；补 justify 6/7/8 与 >8 尾区；
`ComputedLayout` 输出改原生 `{main_size,cross_pos,pos}` 序（或由消费面 `mei_post_update`
取证后统一）；expand 基数据原生段取证改为 `remaining`。

#### (6) `toolbar_alignment` 0x142344560 — 需修
fresh：先按 `GameConfig+0x1e`（`&3`）算 `0x1020300 >> ((a&3)<<3)`，遍历工具栏项
（stride 0x28）`+0x24 != 4` 时写该字节（Rust ✓，写 changed tick 由 `&mut` 自动）。
随后跳表 `0x14304e6d8`（20 项）分派：
- case 0/1/2/3：`taffy_mut_style<closure_env_*>` 序列（Rust 的 `apply_alignment_setters` ✓）；
- 5/7：共享段 `closure_env_13`（✓）；
- 9/10/0xb：`Taffy::style→clone→set_style`（空 setter，Rust ✓）；
- case 4/6：二跳共享段 `0x14234485d`（**Rust `_=>{}` 未实现**）；
- case 8、0xc..0x13：非 taffy 路径（资源写、缓动、`PreferredNodeAlignment` push、
  `Aabb2::from_center_dims/contains`、`DigitalActionState::pressed` 等），**Rust 未实现**。
另外 Rust 的信号是 helper `(alignment, items, _taffy)`，`_taffy` 完全未用，且不等价于
原生 SystemParam 签名；`apply_alignment_setters` 未被 `toolbar_alignment` 调用。
**FIX-BT-6**：落 4/6 二跳与 8/0xc..0x13 分支；恢复 SystemParam 签名并把分派接回本函数。

#### (7) `tod_v2_button` 0x142342f30 — 需修
fresh：① 逐条处理 **UiClickEvent**：命中 query → `visible ^= 1`；insert
`UiPlaySelectAnimation{0.1,0.2}`；若 toggle 后 `visible==1` → 写 UiNavState changed tick 且
**`*(u64*)(UiNavState+0x10)=0`**（清 focus）。
② click 读尽后处理 **ModeJustChanged**：首个未读事件判别 `!=0xd` 且 `visible==1` →
写 changed tick、`visible=0`。
③ 逐项写 `interaction+2 = visible`。
Rust（`toolbar_button.rs` 当前版）顺序/门/动画载荷/per-widget 写均 ✓；
**缺 ① 的 `UiNavState+0x10=0` 写面**（`_nav_state` 参数未用）。
**FIX-BT-7**：补 `nav_state.focus = Default`（按 UiNavState 布局 +0x10 的 tag/id 槽；与
FIX-T1 一并处理）。

#### (8) `ui_animation_state_update` 0x14233b3f0 — 需修
fresh 逐语句：`keep(+1)==0` → `state==0 → 2`，否则 `select += dt/-0.2`、`state=1`；
`keep!=0` → `state!=1 → select += dt/0.2、state=0`，否则 `state=2`；随后
**无条件** `select = clamp01(select)`；`appear += 4·dt·sign`（sign = `state(+0)==0 ? -1 : +1`）
且 **无条件** `appear = clamp01(appear)`；changed tick 每项写。
Rust `update_progress` 的 4 分支 ✓；**两处“state 只置 2”的路径没有 clamp select**
（原生无论如何都 clamp），NaN 语义也不同（原生 max/min → 0，Rust clamp 保留 NaN）。
**FIX-BT-8**：把 `select.clamp(0,1)` 提到分支外；NaN 用原生 max/min 序列。
（FIX-T1 组件承载与 schedule 仍未闭合，见 A-toolbar。）

#### (9) `ToolbarLayout::link_up_nav` 0x141e4d880 — 需修（限定）
fresh：`ids[i] = ToolbarLayout.element(+stride 0x20).+8`（4 个一组 SIMD 拷贝）；
对每 i：`prev=ids[(i+n-1)%n]`、`next=ids[(i+1)%n]`；横向写
slot2={3,next}、slot3={3,prev}、slot0/1 **只写 tag=0（value 不写，栈初值 U）**；
纵向写 slot0={3,prev}、slot1={3,next}、slot2/3 只写 tag=0；每项
**push 一条 `EntityCommand::insert<UiNav>` 到 Commands 队列**（0x60 stride 元素，
+0x50=entity id，+0x58=panic handler）。
Rust helper：prev/next 与槽位 ✓（未连接 value 取 0，已登记 I），但
**没有 command-queue 插入**（“由调用侧负责”），且当前无调用点 → 函数面不等于原生。
**FIX-BT-9**：在函数内或唯一调用侧落 `Commands::entity(id).insert(UiNav{...})`；
并注意 ids 源元素 stride 0x20（`+8`）而非 ToolbarItem 的 0x28。
（既有 reviewer-A-unlock 的 C 只覆盖 payload 写面，未覆盖 command 面。）

---

## 2. tiny-glade B（审计 20 项重建）

> fresh：`evidence/fresh/0x1404f8240_*`、`0x140244790_*`、`0x14015aeb0_*`、`0x14048a170_*`、
> `0x1402aee80_*`、`0x1402ad880_*`、`0x1402adae0_*`、`0x1404b17d0_*`、`0x1400db5b0_*`、
> `0x1400db6b0_*`、`0x1400dbc30_*`、`0x1404b4d50_*`、`0x140365e20_*`、`0x1402552b0_*`、
> `0x14031af10_*`、`0x14031d140_*`、`0x14031ea20_*`、`0x14031eed0_*`、`0x1403e74b0_*`、
> `0x140148b40_*`；callees 逐项落 `results/tinygladeB-fresh.result.json`。

| # | 函数 | VA | verdict | 逐语句分歧（fresh 依据） |
|---|---|---|---|---|
| 1 | `calculate_resolution_scale_mem_estimate_text` | 0x1404f8240 | **需修** | 数值链（w*h*47.7399、scaled+146.109、reserve、f64→u64 饱和、×2^-30、u64→f64）逐项 ✓；**格式串空格错**：原生 pieces 为 `["GPU: ~","GiB / ","GiB\nSYS: ~","GiB / ","GiB"]` 与 `["iGPU: ~","GiB / ","GiB"]`（实读 0x1429ed2b0/0x1429ed270），输出无“数字 与 GiB”之间的空格；Rust `"… {:.2} GiB …"` 每处多一个空格。FIX-BT-10 |
| 2 | `NvAftermathDelegate::dumped` | 0x140244790 | **需修** | 结构 ✓：Info 日志→parking_lot 锁→逐条 `start_file("shader-nvdbg/{:x}-{:x}.nvdbg")`（name pieces 实读 `shader-nvdbg/`+`-`+`.nvdbg`）+`write_all`（context `writing debug info data`）→解锁→主 dump 条目（context `writing dump data`）→finish→tmp/join→`fs::write`→成功日志/失败 anyhow 日志。**未锚定**：`start_file` 的 `FileOptions` 立即数（0x1ed00000001）与 Rust 显式 `Deflated` 是否一致；crc32fast Hasher 由 zip 内部使用与否。FIX-BT-11（反汇编 options 后定 compression） |
| 3 | `CrashReporter::spawn_server` | 0x14015aeb0 | **需修** | temp_dir+Uuid+join、current_exe、4×arg、spawn、`Child::id`、`start` ✓；**panic-vs-return**：原生 `socket.to_str()` 走 `try_from`+`unwrap_failed`（panic），Rust `ok_or_else(Error::Connect)` 返回 Err；**Child 保存缺口**：原生保存 Child（供后续 kill/清理），Rust 只取 id 后 drop。FIX-BT-12 |
| 4 | `client::start` | 0x14048a170 | **需修** | 50ms 重试次序（先加 waited 再判超时再 sleep）逐语句 ✓；**缺 callee**：原生成功后 `thread::spawn(reader closure)` + `CrashHandler::attach`，Rust 两者都没有；默认 `start` transport 恒返回错误（缝）。FIX-BT-13 |
| 5 | `server::run` | 0x1402aee80 | **需修** | 原生 = `minidumper::Server::with_name(...)` + `Server::run` + 两个 `Arc<Mutex<Option<ResolvedCrash>>>`/`Arc<Mutex<CrashMiscInfo>>` + `Option::expect` + Info `"Shutting down the crash server."`；Rust 以 `Receiver<ServerEvent>` 自造事件循环 + `ServerOutput` trait 取代（传输缝，**签名/结构不符**）。FIX-BT-14（SDK 类型落库后改回 Server::run 形态） |
| 6 | `create_minidump_file` | 0x1402ad880 | **需修** | DirBuilder/格式串 `["",".dmp"]`+Uuid/join/OpenOptions(write/create/truncate)/Result ✓；**自造副作用**：Rust 额外 `output.open_minidump(&path)` 与 `self.next_minidump += 1`（原生 callees 无此项）。FIX-BT-15 |
| 7 | `on_minidump_created` | 0x1402adae0 | **需修（主体缺失）** | 原生：处理 `Result<MinidumpBinary,Error>`；Ok 且带路径时 `OpenOptions::open(path).unwrap()`+`read_to_end`；加锁 `+0x20` 的 `Mutex<Option<ResolvedCrash>>`；空槽用 `crash_payload::default()` 初始化；把 MinidumpBinary/文件 handle 写入 `+0x200`（handle@+0x238）；释放锁。Rust 只 `minidumps.push(path)`+`output.send_path` → **整段读文件/存储未实现**（D 报告早已判未实现）。FIX-BT-16 |
| 8 | `crash_trace_to_sentry_event` | 0x1404b17d0 | **需修** | 原生构造 `sentry_types::protocol::v7::Event`：`Event::default`、Exception{ty:`"crash"`, value: crash_reason}、Mechanism、Stacktrace（`drop_in_place<Exception/Mechanism/Stacktrace>` 实证）、modules→breadcrumbs；Rust 扁平化成 `message/level/platform/frames/extra` 的 `SentryEvent` → **返回结构不符**（检查表 #6）。FIX-BT-17 |
| 9 | `setup_panic_hook` | 0x1400db5b0 | **需修** | take_hook→堆分配闭包（4 字：旧 hook fn/data + 捕获的 client 指针）→set_hook ✓；Rust 用三个 `AtomicBool` + 自旋锁，并以静态 `CRASH_CLIENT` 取代闭包捕获（原生 env 第 3 字为 client，`if lVar1!=0`）。FIX-BT-18 |
| 10 | `setup_panic_hook::closure$0` | 0x1400db6b0 | **需修** | 原生：std futex `Mutex`（guard 为 `MutexGuard<HashSet<String>>`，`drop_in_place` 实证）+ `panic_count::is_zero` 双 panic 判定（`DAT_143634b79`）+ `DAT_143634b7a` 已报告标志 + `_eprint`+`int 0x29` + Backtrace/两日志/`recover_game_state_after_crash`/client 消息/abort；已报告路径 `panic_count` 判定后置双 panic 标志、解锁并尾调旧 hook。Rust：原子自旋、`thread::panicking()` 近似、双 panic 标志位置不同（RESTORE 已登记“近似”）。FIX-BT-18 |
| 11 | `recover_game_state_after_crash` | 0x1400dbc30 | **需修** | 目录 `tmp/panics/panic_{ts}`、8 采集器顺序（save/screenshot/rhapsody/inputs/log/settings/profile/gpu）、`save_folder_path`/`backup_folder_path` ✓；原生目录失败用 `ContextError<&str, io::Error>`（context `Creating the bug report directory`），错误经 `logging_map_error`→breadcrumb（`fold<Breadcrumb>`）；Rust 用 `format!("...: {error}")` 字符串、自造 breadcrumb `category:"panic_reporter"`/level Error。FIX-BT-19 |
| 12 | `create_sentry_attachment` | 0x1404b4d50 | **需修** | 5 条错误串、扩展名映射、`stat`→limit→read→累计 ✓；**off-by-one**：原生通过条件 `total+len < 0x1400001`（即允许恰好 0x1400000），Rust `>= 0x1400000` 拒绝 → 少 1 字节；`stat` 原生 `with_context`，Rust `metadata(path)?` 无 context；read context 文案需按 closure 实读核对。FIX-BT-20 |
| 13 | `hang_watchdog` | 0x140365e20 | **需修** | 20s/1s/`check_deadlock` Vec<Vec>/日志/abort 链 ✓；**缺 callee**：原生 `thread_priority::set_for_current` + 成功/失败两条日志（callees `set_for_current` 实证），Rust `set_thread_priority()` 空（依赖未落库，登记 blocker）。FIX-BT-21 |
| 14 | `verify_package_integrity_and_show_report` | 0x1402552b0 | **需修** | 原生 `Instant::now`→`verify_package_manifest`→按变体日志→**无条件** `EventLoop::new/init`+`run_integrity_report_gui`+`RedrawWindow`；Rust 只有 `Option<&dyn IntegrityReportGui>`，缺 EventLoop/RedrawWindow 且 Ok 时不显示 GUI（trait seam）。FIX-BT-22 |
| 15 | `gather_screenshot_requests` | 0x14031af10 | **需修（主体缺失）** | 原生 callees 显示：`MetaCommander::peek/consume`、`DigitalActionState::just_pressed`（热键）、`EscMenuState::is_closed`、`paths::config`、`date_time_string_for_filename`、`_join`/`format_inner`（文件命名）、`add_captured_screenshots_to_steam_library`（`retain_mut<SteamScreenshotRequest>`）、`fold` 到 `ActiveScreenshotRequest`、`WriteSettingsFileCmd` 事件；Rust 仅分配纹理并 push `state=0`。FIX-BT-23 |
| 16 | `handle_screenshot_opportunity` | 0x14031d140 | **需修（主体缺失）** | 原生匹配条件为 `(opp.purpose==3 || req.purpose==opp.purpose) && (req.purpose-2==opp.purpose) && tag(+0x96)==6`，并有纹理创建/`dispatch_2d`/mapped buffer/readback/state 写面；Rust 用 `purpose==purpose && tag==6` 简化并直接 `state=2`。FIX-BT-24 |
| 17 | `finish_screenshot_capture` | 0x14031ea20 | **需修** | 原生逐项：`tag(+0x96)!=6 && state(+0x50)!=2` → `state==1`：`wait_idle()`+**`read_back_screenshot()`**+state=2；`else if frames(+0x54)==0`：**`read_back_screenshot()`**+state=2；否则 frames-=1；最后 `retain`。Rust **两处 read_back_screenshot 均缺**；且 native `state` 为 u32@+0x50，Rust 为 u8（字段宽度/偏移不符）。FIX-BT-25 |
| 18 | `hurry_active_screenshots_on_shutdown` | 0x14031eed0 | **需修** | 原生：`once_cell` 的 `Mutex<Vec<JoinHandle<()>>>` → 逐个 `join`（callees `join<()>`），并做预算/`Instant` 差值；Rust 只置 state=1 + 一条 debug 日志，**不 join、不计算预算**（且日志是自造）。FIX-BT-26 |
| 19 | `run_vulkan_configuration_app` | 0x1403e74b0 | **需修** | 原生：`GraphicsInstanceLoader::new`、`implicit_layers`、ash `LayerProperties`→`GraphicsInstanceLayer`（`from_iter`）、`EventLoop::new/init`、`run_vulkan_configuration_gui`、**`write_game_config_to_disk`**、`RedrawWindow`；Rust 层选择是自造 name/description 映射，缺 EventLoop、缺写配置（`Ok(())` 代替）。FIX-BT-27 |
| 20 | `date_spinner` | 0x140148b40 | **需修** | 原生 352B 仅见：分配→`*alloc=in_RDX`→`egui::widgets::drag_value::impl$1::ui`→Arc drop_slow；**speed 0.1（.rdata 0x1429d3768）与 range 描述符 0x1429d3710 在本函数体内未直接锚定**（可能在 inlined DragValue 构造/调用侧）。Rust `DragValue::new(value).speed(0.1)` 属未证实等价。FIX-BT-28（反汇编确认 `.speed/.range` 后再定） |

**tiny-glade B 三态：判等 0 / 需修 20 / 未实现 0。**

---

## 3. 补审：tiny-glade D 重建 4 项 + E 重建 5 项

> 任务要求同时读/覆盖 tinyglade D/E。这两批不在审计 B 表内，但同为“重建未判等”。

| 函数 | VA | verdict | 分歧 |
|---|---|---|---|
| `NvAftermathDelegate::default` | 0x140244460 | **判等（re-verify）** | `tmp()/gpu-crash-dump.zip` `is_file`→unlink；失败 Error 级 `Error deleting stale {path}: {error}`（fresh 两格式化实参）；随后构造 `HashMap`（RandomState 随机键，`hashmap_random_keys` 实证）。FIX-A3 落库后与 fresh 一致；布局差异（Mutex<HashMap> vs 原生表+keys）为 I。 |
| `capture_image` | 0x140589c90 | **需修** | 尺寸算法逐句 ✓（`sqrt(230400/(w/h))`、f32 截断、饱和 u32::MAX、0→1、`230400/tw & 0x3fffe`、非零才 dispatch）。分歧：原生 `h==0` 时 `sqrt(inf)=inf→u32::MAX→230400/MAX=0→&mask=0` 跳过，`w==0` 也跳过；**Rust 提前 `if w==0||h==0 {(0,0)}`**，`h==0` 行为不同；GPU compute pass 为缝（登记）。FIX-BT-29 |
| `report_crash_event_to_sentry` | 0x1404b3e30 | **需修** | 原生数组为 **9 项**（`array::IntoIter<_,9>`），顺序 Minidump/GPU/Rhapsody/Inputs/Settings/Profile/Screenshot/Log/Save；Rust 建 8 项再**最后**追加 Minidump → 顺序错（影响 20MiB 累计与 attachment 顺序）。原生还 `from_str_seed<GameProfile>` 读取 profile（Rust 未做）。FIX-BT-30 |
| `ReverseBacktrace::fmt` | 0x1402cfc30 | **需修** | 原生是 `call_once` 包装 `impl$0::fmt::closure_0`（逐 frame 逆序遍历并格式化 `BytesOrWideString`）；Rust 用 `format!("{:?}", bt)` 后**按文本行反转**（近似体；多行 frame/Symbol 输出时不等）。FIX-BT-31 |
| `find_best_fullscreen_video_mode` | 0x1400453c0 | **需修** | 评分键 `area_diff + refresh_diff/10 + depth_diff*1000`（fresh 0x…1008 逐指令）✓；`dmFields & 0x5c0000` 全置位否则 panic ✓；`dmDisplayFrequency*1000` ✓；空表 warn/current 匹配/首个模式 ✓。**D1** 平台内部 `get_monitor_info` 失败分支（warn+空表）未建模；**D2** 第二次 `stable driftsort` 的比较器是 `bool(&VideoMode,&VideoMode)`，方向未锚定，Rust `b.cmp(a)`；**D3** "does not support" 第二实参原生 `fmt<i32>` 取值未锚定。FIX-BT-32 |
| `setup_window` | 0x1400436b0 | **需修** | 主链 ✓（expect 文案、available_monitors find、`Attempting…`、primary fallback、borderless/exclusive、find_best、`Icon::from_rgba`、`RedrawWindow(...,2)`）。**D4/D5/D6**：参数 ABI、WindowAttributes 逐字段布尔面、FullscreenConfig 载荷未锚定；**新分歧**：原生 `display.monitor == None` 分支也先打 `"Desired monitor not found; using primary"`（0x140043c98 段），Rust None 分支直接 primary、无日志。FIX-BT-33 |
| `gather_pending_video_setting_changes` | 0x1404e3380 | **判等** | 逐语句：`off1/2=consumed.saturating_sub(start)`；`end<off → (empty,0)`；`*cursor=(total-len2)-len1`；先段 1 后段 2，逐项 `*cursor+=1`、std `Mutex<u8>` lock/poison unwrap/`flags|=item+8`/unlock（futex wake）。Rust 逐句一致；仅 Element/Queue 字段名为重建（E1，I）。 |
| `GameLoop::extract_available_monitors_info` | 0x1404e2470 | **需修** | `ResMut<AvailableMonitors>`（resource id `0xe5e047cb9d6d98d7` 实证）+changed tick+`clear`+`available_monitors`+逐 monitor `MonitorMatch::from_winit`+`get_monitor_info` 失败 warn+模式枚举（dmSize 0xdc、`dmFields&0x5c0000`）+BTreeSet→Vec→sort→`VideoMode::from_winit`→push。Rust 用公开 `video_modes` 等价面，**get_monitor_info 失败 warn 分支与第二次 sort 方向（D1/E2）未建模**。FIX-BT-34 |
| `EguiEventIo::new` | 0x14026de50 | **需修** | 原生：`memcpy 0x120`（State）+写 `{1,1,0}`@+0x120/+0x128/+0x130+`memcpy 0x126`→`alloc 0x138`→memcpy→返回 24B 外壳 `{rdx, ptr, r9}`；Rust `Mutex<egui_winit::State>` 外壳，**三个包装标志与 0x138 载荷未建模**（F1）。FIX-BT-35 |
| `EguiEventIo::take_egui_input` | 0x14026dfd0 | **判等（胶水）** | std futex Mutex lock/poison `unwrap_failed`→puffin scope→`State::take_egui_input`→end_scope→unlock（旧 state==2→wake）逐句对应 Rust `Mutex::lock().unwrap()`+`puffin::profile_function!()`+上游转发。 |

**D/E 三态：判等 3（default、gather_pending、take_egui_input）/ 需修 6 / 未实现 0。**
（按条目计：D 4 项 = 1 判等 + 3 需修；E 5 项 = 1 判等 + 4 需修。）

---

## 4. 8 条分歧检查表命中

| 检查表 | 命中项 |
|---|---|
| 1. panic-vs-return 反 | `load_font_table`（IO panic→Err）、`spawn_server`（to_str panic→Err）、`server::on_message`/`send_message` 已由 A 复核修；`on_minidump_created`（缺读文件路径） |
| 2. 门控极性反 | `process_input_action_handlers`（父母表 vs 节点表、反向区间起点）、`tod_v2`（缺 focus 清位）、`verify_package_integrity`（GUI 非无条件） |
| 3. 载荷平面近似 | `place_childen`（输出 `{f32,f32,Vec2}` vs `{Vec2,Vec2}`）、`crash_trace_to_sentry_event`（扁平化）、`recover…`（breadcrumb category）、`report_crash_event`（9 项序） |
| 4. 事件/命令自造 | `create_minidump_file`（`open_minidump`/`next_minidump`）、`gather_screenshot_requests`（自造 Active 记录）、`server::run`（自造 channel loop） |
| 5. callee 转发错 | `load_font_table`（`FontTable::from_ron`+validate）、`finish_screenshot_capture`（缺 `read_back_screenshot`）、`hurry…`（缺 `join`）、`client::start`（缺 `spawn`/`attach`） |
| 6. 返回结构不符 | `crash_trace_to_sentry_event`、`link_up_nav`（缺 command）、`on_minidump_created`（缺 ResolvedCrash 写面） |
| 7. 排序取最近 vs 首个 | `place_childen`（justify 分派/装配）、`find_best`（sort 方向） |
| 8. 偏移表/字段错位 | `place_childen`（YX `min` vs `size`、ComputedLayout 序）、`finish_screenshot_capture`（state u32@+0x50 vs u8）、`create_sentry_attachment`（limit 边界） |

---

## 5. FIX 清单（按优先级）

> 本轮只报告；修后需二次独立复核方可升 C。P1 = 行为可见/跨模块；P2 = 字段/文案/顺序；P3 = 低危/限定。

| ID | 级别 | 函数 | 修正 |
|---|---|---|---|
| FIX-BT-1 | P1 | `reuse_or_create_node` | 复刻 remove_entry 身份回收（旧 flags&0xf3、旧 id、4×u32） |
| FIX-BT-2 | P1 | `process_input_action_handlers` | 门改节点表 `nodes.is_empty()`；第二遍仅 `[0,stop]`；补 on_input 2B 状态实参 |
| FIX-BT-4 | P1 | `load_font_table` | IO 失败 panic（path+io error）；直接 `Options{1,0x80,0}::from_str_seed`，勿额外 validate |
| FIX-BT-5 | P1 | `place_childen_impl` | YX 求和改 `style.min`；补 justify 6/7/8 与 >8；输出改 `{main_size,cross_pos,pos}`；expand 基数按原生段取证 |
| FIX-BT-6 | P1 | `toolbar_alignment` | 补 case 4/6 二跳与 8/0xc..0x13；恢复 SystemParam 并接回分派 |
| FIX-BT-16 | P1 | `on_minidump_created` | 打开并 read_to_end minidump，写入 `+0x200` ResolvedCrash（含 handle@+0x238），Mutex 保护 |
| FIX-BT-23 | P1 | `gather_screenshot_requests` | 补热键门（just_pressed/is_closed）、文件命名（date_time/_join/config）、Steam retain_mut、MetaCommander peek/consume、fold 记录 |
| FIX-BT-24 | P1 | `handle_screenshot_opportunity` | 按原生三条件匹配（purpose/opp==3/tag==6）与 GPU dispatch/readback/state 写面 |
| FIX-BT-25 | P1 | `finish_screenshot_capture` | 两处补 `read_back_screenshot()`；state 改 u32@+0x50 |
| FIX-BT-26 | P1 | `hurry_active_screenshots_on_shutdown` | 补 writer JoinHandle join 与预算/Instant 计算；删自造日志 |
| FIX-BT-30 | P1 | `report_crash_event_to_sentry` | 改 9 项数组（Minidump 首项）；补 GameProfile 读取 |
| FIX-BT-32 | P1 | `find_best_fullscreen_video_mode` | 锚定 sort 比较器方向；建模 get_monitor_info 失败分支；D3 实参 |
| FIX-BT-33 | P1 | `setup_window` | None 分支补 `Desired monitor not found; using primary`；D4–D6 取证 |
| FIX-BT-3 | P2 | `load_configs` | 补 `relative_to(dir)`；panic 带路径；log target `toolbar::mei::config` |
| FIX-BT-7 | P2 | `tod_v2_button` | 补 `UiNavState+0x10=0`（focus 清位） |
| FIX-BT-8 | P2 | `ui_animation_state_update` | select clamp 提到分支外；NaN 用 max/min 序列 |
| FIX-BT-11 | P2 | `dumped` | 反汇编 `start_file` FileOptions，确认 compression method/level |
| FIX-BT-12 | P2 | `spawn_server` | to_str 失败改 `unwrap`（panic）；保存 Child |
| FIX-BT-13 | P2 | `client::start` | 成功后 `spawn(reader)` + `CrashHandler::attach`；真实 connect transport |
| FIX-BT-14 | P2 | `server::run` | SDK 类型落库后恢复 `Server::with_name/run` + Arc<Mutex> 形态 |
| FIX-BT-15 | P2 | `create_minidump_file` | 删除自造 `open_minidump`/`next_minidump` |
| FIX-BT-17 | P2 | `crash_trace_to_sentry_event` | 按 Exception{ty:"crash"}/Mechanism/Stacktrace 结构返回 |
| FIX-BT-18 | P2 | `setup_panic_hook`(+closure) | std Mutex 形态、panic_count 双 panic 判定、client 闭包捕获 |
| FIX-BT-19 | P2 | `recover_game_state_after_crash` | 目录错误用 Context；breadcrumb 经 logging_map_error（category 取证） |
| FIX-BT-20 | P2 | `create_sentry_attachment` | limit 改 `total+len < 0x1400001`；stat/read context 对齐 |
| FIX-BT-21 | P2 | `hang_watchdog` | `thread_priority::set_for_current` + 两条日志（依赖落库后） |
| FIX-BT-22 | P2 | `verify_package_integrity` | EventLoop::new/init + 无条件 run GUI + RedrawWindow |
| FIX-BT-27 | P2 | `run_vulkan_configuration_app` | LayerProperties→GraphicsInstanceLayer 映射、EventLoop、`write_game_config_to_disk` |
| FIX-BT-28 | P2 | `date_spinner` | 反汇编确认 speed/range 后再定实现 |
| FIX-BT-29 | P2 | `capture_image` | 去掉 w/h==0 提前返回（按原生 f32 计算） |
| FIX-BT-31 | P2 | `ReverseBacktrace::fmt` | 逐 frame 逆序格式化，替代文本行反转 |
| FIX-BT-34 | P2 | `extract_available_monitors_info` | get_monitor_info 失败 warn 分支；sort 方向 |
| FIX-BT-35 | P2 | `EguiEventIo::new` | 锚定 24B 外壳与 +0x120/+0x128/+0x130 三标志 |
| FIX-BT-9 | P3 | `link_up_nav` | 补 `Commands::entity(id).insert(UiNav)`；ids 源 stride 0x20@+8 |
| FIX-BT-10 | P3 | `vram text` | 格式串去掉“数字 与 GiB”之间的空格（4 处/2 处） |

---

## 6. 账目影响

- toolbar（审计 B 12）：**B 12 → C 3 / B 9**。升 C：`center`、`hover`、`has_focus`。
- tiny-glade B（审计 B 20）：**B 20 → C 0 / B 20**（全部可执行但逐语句仍有分歧）。
- tiny-glade D 重建（4）：`default` C（复核 FIX-A3）、`capture_image`/`report…`/`ReverseBacktrace` B。
- tiny-glade E 重建（5）：`gather_pending` C、`EguiEventIo::take_egui_input` C（item 因 new 未闭合仍 B）、
  `find_best`/`setup_window`/`extract` B。
- 合计：**判等 5 / 需修 36 / 未实现 0**。

## 7. 附带的 C 清单纠正（不属本任务，但证据确凿）

- `Node::used`（0x140683c80，审计 C 清单）：Ghidra `void()` 反编译显示 `return;`，但
  **B patched EXE radare2 实测**为
  `test byte [rcx+0xb6],0x30; setne al; ret` → 真语义是
  `used(&self) -> bool { self.state_bits & 0x30 != 0 }`；
  当前 Rust `pub fn used(&mut self) {}` 为**空体**（无调用点）。建议在 C 账目中改判
  “需修/待重判”，并由 toolbar owner 修签名与返回。证据：
  `evidence/results/radare-tiny-B.result.json`。
- `Node::prev_computed_layout`（0x140683cb0）也非纯空：`xor eax,eax; cmp byte [rcx+0x90],0;
  lea rcx,[rcx+0x94]` → 依赖 `+0x90` Option 判别与 `+0x94` 载荷；Rust 返回全零，
  维持“未实现”更准确（同样无调用点）。

## 8. 证据索引（`reports/reviews/B-toolbar/evidence/`）

- `jobs/`：`toolbar-fresh.json`、`tinygladeB-fresh.json`、`tinygladeDE-fresh.json`、
  `xrefs.json`、`mem-config.json`、`mem-formats.json`、`mem-strings2.json`、
  `radare-tiny.json`、`radare-tiny-B.json`（全部可重放）。
- `results/`：对应 `*.result.json`（Ghidra/radare2 原始返回）。
- `fresh/`：43 个函数（41 清单 + default/Radare 辅助）的 fresh 反编译 `.c`。
- 关键字节实读：格式 pieces `0x1429ed260..0x1429ed2e0`、aftermath name
  `0x1429a06f0..0x1429a0718`、attachment 文案 `0x1429e8fc0..0x1429e9020`。
- 身份：B patched `tiny-glade.exe`（命令参数 `program="tiny-glade.exe"`），
  radare2 另对同文件直读；未使用 `--write`。

## 9. 复现

```sh
cd /Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core/recovery/agent-team/reports/reviews/B-toolbar/evidence
/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python \
  /Users/yuluo/.codex/mcp-tools/reverse-operator.py ghidra jobs/toolbar-fresh.json \
  results/toolbar-fresh.result.json --worker shared
```

（tinygladeB/tinygladeDE、xrefs、mem、radare jobs 同理。）
