# REVIEW — systems/toolbar（restorer-interaction-toolbar 还原复核）

> reviewer-interaction-toolbar，2026-10-17。方法：全部**fresh 反编译**（Ghidra shared worker，
> `get_functions fields=signature,parameters,locals,decompiled_code,callees`），独立复述原函数
> 控制流后再与 `crates/systems/toolbar/src` Rust body 逐语句对照，不依赖 restorer 描述。
> 证据：本目录 `review-evidence/batch{1..5}-*.txt`（batch1-gating、batch2-node、
> batch3-nodecontext、batch4-layout-config、batch5-hash；jobs/*.json 可重放）。
> 身份：B patched EXE（metadata 核对通过）。
> 编译复核：`cargo check -p toolbar` exit 0（0 错误/0 警告，与 restorer 声明一致）。

## 0. 结论速览

- restorer 判等 29 项：**25 判等成立；2 项高估（child_with_global_id/into_child_with_global_id，
  父节点缺失时原 panic、Rust 静默创建）；1 项状态不符（new_at_pos<String> 无 src 落盘）；
  1 项微分歧（NodeContext::node 原 expect_failed、Rust 用 unwrap）**。
- 判等核心主张全部**坐实**：`safe_frame`/`new_at_pos` 的 `(id&1)==0` push 门控（超出
  ANALYSIS-A 的既有分析，restorer 发现属实）；`new_at_pos<String>` 0x142289f80 物理函数
  存在（268B，修正 ANALYSIS-A「无物理函数」）；`on_key_event` tag@+0x68、
  `on_clipboard_paste` tag 0xc5@+0x58（修正 ANALYSIS-A「tag@+0x58」笼统说法）。
- 重建 7 项：`reuse_or_create_node` 重建诚实；**`process_input_action_handlers` 降级为
  未实现-需重做**（多调 vtable+0x48、正向无早停、retain 压实原无此副作用，均分歧）；
  `load_configs` 重建-需接线（原存 self.configs 字段，Rust 返回 HashMap）。
- 类型级分歧：`NodeContext.mei` 应为 `&MeiContext`（指针），Rust 用 `GlobalId`(u64)。
- 哈希级风险：`utils::calculate_hash` 原为 `Sip13Rounds`（小体积、无 RandomState 机制，
  疑零键 DefaultHasher）；Rust 镜像用 `RandomState`（随机键）→ id 跨进程不稳且值不等。
  `WidgetId::from_name`（0x14092df30）= `calculate_hash<tuple<&str,&()>>` 同一函数 →
  restorer 的 `<String>`/`<&str>` 同值疑虑**已消解**（同源同值）。
- 胶水分类（build/register_required_components/GlobalId Hash/NodeContent 默认）核实合理，
  GlobalId `as_id`(0x140683c10)=sip write×2 确认是 derive 胶水。
- FIX 清单：8 条（见 §5）。

## 1. 判等函数逐函数复核（fresh 反编译独立复述 → Rust 对照）

### 1.1 Node 方法（batch2-node.txt，0x140683c20..0x140683cd0）

| 符号 | VA | 原逻辑（fresh 复述） | Rust body | 判定 |
|---|---|---|---|---|
| `set_sounds` | 0x140683c20 | `node+0xb4=hover、+0xb5=click`，14B | `self.sound_hover=hover; self.sound_click=click` | **判等** |
| `ignore_input` | 0x140683c30 | `+0xb7 \|= 2` | `self.flags \|= IGNORE_INPUT(2)` | **判等** |
| `force_interactive` | 0x140683c40 | `+0xb7 \|= 1` | `self.flags \|= FORCE_INTERACTIVE(1)` | **判等** |
| `default_focus` | 0x140683c50 | `+0xb7 \|= 4` | `self.flags \|= DEFAULT_FOCUS(4)` | **判等** |
| `with_nav` | 0x140683c60 | `+0xb7 = b<<5 \| (+0xb7 & 0xdf)` | `(nav<<5)\|(flags&!0x20)` | **判等** |
| `used` | 0x140683c80 | 物理 `return;` | 空函数 | **判等**（物理空壳） |
| `block_input` | 0x140683cd0 | **赋值** `+0xb7 = 1`（非 OR） | `self.flags = FORCE_INTERACTIVE` | **判等**（赋值 vs OR 区分正确） |
| `hover` | 0x140683c90 | 物理 `return;`（getter 内联） | `state_bits&1` | **重建(I)**：位语义未在原函数验证 |
| `has_focus` | 0x140683ca0 | 物理 `return;` | `state_bits&64` | **重建(I)** |
| `prev_computed_layout` | 0x140683cb0 | 物理 `return;` | 返回零值 | **未实现**（诚实，不伪造） |

### 1.2 NodeContext 系列（batch2-node.txt / batch3-nodecontext.txt）

| 符号 | VA | 原逻辑（fresh 复述） | Rust body | 判定 |
|---|---|---|---|---|
| `NodeContext::hover` | 0x140683ce0 | `mei=*self`；表非空则探 `+0x1a0/+0x1a8` 找 `self.id`，命中 `return;`（不写返回值），探空→`unwrap_failed` | `let _=self.node(ctx)`（get+unwrap） | **判等** |
| `NodeContext::interaction` | 0x140683dd0 | 同 hover | 同上 | **判等** |
| `node`(deref) | 0x140681d50 | 同探表；缺失→**`expect_failed`** | `ctx.nodes.get(...).unwrap()` | **判等-微分歧**：原为 `Option::expect`，Rust 用 `.unwrap()`（panic 串/类型不同）→ FIX |
| `node_mut`(deref_mut) | 0x140681e50 | 同探表；缺失→`unwrap_failed` | `get_mut(...).unwrap()` | **判等** |
| `child_with_global_id` | 0x1406861b0 | `reuse_or_create_node(child)`；**探父节点表，缺失→unwrap_failed**；命中则 children push child；返回 `{mei_ptr, child, 0}` | `reuse_or_create_node(child)`；`reuse_or_create_node(self.id).children.push(child)`（**父缺失静默创建**）；返回 `NodeContext::new(self.mei, child)` | **重建-分歧（高估）**：原父缺失 panic，Rust 创建父节点；返回 mei 应为指针 |
| `into_child_with_global_id` | 0x140686300 | 与 child_with 字节同构（332B） | 同 Rust 实现 | **重建-分歧（高估）**：同上 |
| `has_global_id` | 0x140686450 | 探表命中→`return;`(bool 于 AL)；探空→`return;`（false） | `contains_key(&id)` | **判等** |
| `expect_child_with_global_id` | 0x140686530 | 构造 `{mei, id, 0}`（22B，无探表） | `NodeContext::new(self.mei, id)` | **判等**（构造；mei 类型除外） |
| `layout` | 0x140686550 | 探表命中→写 Node style 区 72B（in_R8[0..8] 至 node+0x20..+0x68）；缺失→unwrap_failed | `self.node_mut(ctx).style = style` | **判等** |
| `NodeContext::ignore_input` | 0x140686690 | 探表命中→flags 字节 `\|=2`；缺失→unwrap_failed | `node_mut(ctx).ignore_input()` | **判等** |
| `NodeContext::with_nav` | 0x140686790 | 探表命中→flags `=b<<5\|(&0xdf)`；缺失→unwrap_failed | `node_mut(ctx).with_nav(nav)` | **判等** |

### 1.3 MeiContext 核心（batch1-gating.txt）

| 符号 | VA | 原逻辑（fresh 复述） | Rust body | 判定 |
|---|---|---|---|---|
| `safe_frame<&str>` | 0x140f99230 | `id=calculate_hash((name,()))^0xb1695320004fb443`；`reuse_or_create_node`→`+0x5d=0`；opaque→`+0xb7=1`；**`if((id&1)==0)`** push 事件(stride 0x38：+0=1、+0x30=id，满 cap→grow_one)；返 `{events,id,0}` | `GlobalId::new`；`node.used_frame=0`；opaque→flags\|=1；`if (id&1)==0` push `FrameEvent::safe`；返 `SafeFrame{events,id,0}` | **判等**（门控逐句一致） |
| `new_at_pos<&str>` | 0x140f992e0 | 同 safe_frame；`(id&1)==0` 时事件写 +0=4、+4=x、+8=y、+0xc=w、+0x10=h、+0x14=position(8B)、+0x1c=extra、+0x30=id | `FrameEvent::at_pos` 各字段 | **判等** |
| `new_at_pos<String>` | 0x142289f80 | **存在 268B 物理函数**：`WidgetId::from_name(name)`（=calculate_hash((name,()))）^mask；结构逐字段同 <&str>；末尾 `if(*in_R8) dealloc`（String 释放） | **无 String 重载 src** | **物理复核成立；判等状态名不符实（src 未落）** → FIX |
| `on_key_event` | 0x140685fc0 | 读 13×u64 载荷（in_RDX[0..10]+in_R9[0..3]）；`len==cap`→grow_one；写 0x70 载荷，**tag 字节@+0x68**（`CONCAT71(…,in_R8B)`）；len+1 | `input_events.push(keyboard(payload, tag))`，`data[13]=(…&!0xff)\|tag`（0x68=13×8） | **判等**（tag 偏移坐实） |
| `on_clipboard_paste` | 0x1406860d0 | 同 push；**tag 0xc5@+0x58**（`CONCAT44(uStack_4c,0xc5)`） | `data[11]=(…&!0xff)\|0xc5`（0x58=11×8） | **判等** |
| `load_font_table` | 0x140683a20 | `config::load_font_table().unwrap()`（判别子≠2→unwrap_failed）；`Arc::new`（strong=1）；旧 Arc@+0x260 递减，归零 drop_slow | `config::load_font_table(path).unwrap()`；`self.font_table=Arc::new(table)` | **判等** |
| `reuse_or_create_node` | 0x140683ec0 | hash_one 探表命中返 &Node；未命中：+0x1b0==0→reserve_rehash；**remove_entry 清陈旧 + 身份回收**（旧 flags&0xf3、旧 id、4×u32 载荷→新节点）；初始化（children cap=8/len=0、state[2]=4、opacity=1.0、声音 0x0d0d）；len+1 | `contains_key`+`insert(Node::new(EmptyContent))`+`get_mut` | **重建（诚实）**：身份回收未复刻，restorer 已登记分歧 |
| `load_configs` | 0x140682e70 | walkdir 迭代→extension=="ron"（0x6f72/0x6e）→relative_to+unwrap→file_stem→read_node_config_from_file→**解析失败 panic_fmt(ron::error)**→log "Loaded UI config from {}"→`HashMap::insert<String,Arc<NodeConfig>>` **入 self** | `WalkDir`+filter "ron"+file_stem+`read_node_config_from_file(...).unwrap_or_else(panic)`+`log::info!`+**返回 HashMap** | **重建-需接线**：原存 self.configs 字段，Rust 返回 map 未落 self → FIX |
| `process_input_action_handlers` | 0x140687eb0 | 收集 handler 表 id（去重）→**排序**→正向遍历：探节点表→`vtable[+0x50]`(on_input) →**首个 handled 即停**(记位置)→反向从停点遍历重新调用 on_input → dealloc 缓冲返回；**不修改 handler 表** | `filter contains_key`+`sort`+全遍历调用 `process_input_action‖on_input`+`handlers.retain` | **未实现-需重做**（分歧见 §4） |

### 1.4 Layout / Config / LogSlider（batch4-layout-config.txt）

| 符号 | VA | 原逻辑（fresh 复述） | Rust body | 判定 |
|---|---|---|---|---|
| `LayoutStyle::default` | 0x14068e850 | 全零 + `+0x44 u16=0`(swizzle) + `+0x46 u8=4`(justify) | 全零+swizzle 0+justify 4 | **判等** |
| `horizontal` | 0x14068e880 | 同字节（+0x46=4，其余零） | `LayoutStyle::default()` | **判等** |
| `LayoutStyle::place_childen` | 0x14068ea20 | `+0x44 低字节==0 → impl<SwizzleXY>` 否则 `impl<SwizzleYX>` | `style.swizzle==0` 分派 | **判等**（分派） |
| `ComputedLayout::center` | 0x14068ea00 | 物理 `return;` | `pos+size*0.5` | **重建(I)**（getter 内联，语义未在原验证） |
| `read_node_config_from_file` | 0x14068fe40 | read_to_string→**Ok：Options{1,0x80}::from_str_seed<NodeConfig>→返 Result，dealloc 串**；**Err(io)→panic_fmt(io::Error)** | `unwrap_or_else(panic)`+`Options::default().from_str` | **判等**（控制流；ron Options 标志 I 级微分歧 → FIX 记录） |
| `config::load_font_table` | 0x14068fc60 | 与 read_node_config 同构，解析 FontTable | 同构 | **重建(I)**（Options 标志 U） |
| `LogSlider::param_to_value` | 0x142383730 | `min<=max` 检查；`fVar1=max(min,param)`、`fVar2=min(fVar1,max)`=clamp；**min>max→do_panic::runtime**；`exp2f(clamp+[3])` | `param.clamp(min,max)`+`(p+base).exp2()` | **判等**（含 min>max panic） |

### 1.5 Hash 链（batch5-hash.txt）

| 符号 | VA | 原逻辑 | Rust | 判定 |
|---|---|---|---|---|
| `utils::calculate_hash<tuple<&str,&()>>` | 0x1405bdda0 | `sip::impl$5::write<Sip13Rounds>`×2（375B，**无 RandomState/thread-local 机制 → 疑零键 DefaultHasher**） | `RandomState::new().build_hasher()`（**随机键**） | **风险**：随机键→id 跨进程不稳且与原值不等 → FIX 核实 |
| `WidgetId::from_name` | 0x14092df30 | = `calculate_hash<tuple<&str,&()>>`（同一函数） | — | **消解 restorer I 疑虑**：<String> 与 <&str> 同源同值 |
| `GlobalId` Hash(impl$0::as_id) | 0x140683c10 | `write<Sip13Rounds>`×2（5B，derive） | derive(Hash) | **glue-legitimate** |

## 2. 重建 7 项复核

- `reuse_or_create_node`（0x140683ec0）：**重建** 成立，身份回收分歧已登记（§1.3）。
- `process_input_action_handlers`（0x140687eb0）：**降级未实现-需重做**（§4）。
- `load_configs`（0x140682e70）：**重建-需接线**（存 self.configs）。
- `config::load_font_table`（0x14068fc60）：**重建(I)** 成立。
- `ComputedLayout::center`（0x14068ea00）：**重建(I)** 成立（getter 内联）。
- `Node::hover`/`Node::has_focus`（0x140683c90/0x140683ca0）：**重建(I)** 成立（物理空壳）。

## 3. 胶水 / 未实现抽查

- 5 插件 `build`、26× `register_required_components`、`NodeContent` 默认、`GlobalId` derive：
  分类合理；`GlobalId::as_id`（0x140683c10）已 fresh 核实为 sip write 胶水。系统/事件未落库
  而 build 未重建属诚实登记。
- 未实现清单（mei_pre/post_update、widgets/tweaker/decorate/audio、menu/*、photomode/*、
  text、taffy 族、toolbar_systems/button/ui_interaction、demo、许可服务）逐函数登记分歧点，
  共同根因（taffy/tiny_http/image 未入 workspace、country-core GlobalId/TaffyNode/UiNav
  未落库、utils::calculate_hash 未落库）与 ANALYSIS/骨架一致，**无高估**。
  `taffy_finalize_element_size`（0x142340890）标注「复核确认真实业务」合理（1152B 非胶水）。

## 4. process_input_action_handlers 分歧点（未实现-需重做）

fresh 反编译 0x140687eb0（1279B）逐分支：
1. **调用面**：原正向/反向遍历都只调 `(***(node+0x30)+0x50)`（on_input）；Rust 额外调
   `process_input_action`（vtable+0x48）→ 对 process_input_action 返回 true 的内容类型，
   Rust 会标记 handled 而原不会。
2. **正向早停**：原正向遍历在首个 handled 处 `local_80=uVar8` 并 `goto LAB_140688271`
   （停止继续处理）；Rust 全量遍历。
3. **压实语义**：原压实是收集缓冲的**反向重新调用**（从停点向前 re-apply on_input，
   命中 handled 即 dealloc 返回），**不修改 handler 表（+0x1d0/+0x1d8/+0x1e8 无写入）**；
   Rust 以 `handlers.retain(...)` 删除未处理项 → 新增原不存在的表副作用。
4. 收集/去重/排序为 hashbrown 探表细节，原与 Rust 均可接受，但 1–3 必须按原分支重做。

## 5. FIX 清单（8 条）

1. `child_with_global_id`/`into_child_with_global_id`（0x1406861b0/0x140686300）：父节点缺失
   改为探表+panic（unwrap_failed），删除 `reuse_or_create_node(self.id)` 静默创建。
2. `NodeContext::node`（0x140681d50）：`.unwrap()` 改 `.expect(...)`（原 expect_failed）。
3. `NodeContext.mei` 类型：`GlobalId` → `&MeiContext`（指针）；`new`/`expect_child_with_global_id`/
   `child_with_*` 返回一并修正。
4. `calculate_hash` 镜像：核实原 `utils::calculate_hash` 键源（疑零键 DefaultHasher）；
   若零键则以 `DefaultHasher::new()`（零键 SipHasher13）替换 `RandomState`，否则 id 跨进程
   不稳且与原值不等。`<String>`/`<&str>` 同值已坐实（同源 from_name）。
5. `new_at_pos<String>`（0x142289f80）：补 String 重载 src（WidgetId::from_name 路径 + 末尾
   dealloc），判等状态改为「物理复核+待落 src」。
6. `load_configs`（0x140682e70）：MeiContext 补 configs 字段，结果插入 self 而非返回 map。
7. `Node::new`：`used_frame` 初值改 0（原新节点 +0x5d 区零）。
8. `read_node_config_from_file`（0x14068fe40）：核 ron Options{1,0x80} 与 `Options::default()`
   同值（I 级，控制流已判等）。

## 6. 高估判定汇总

- **判等高估 2**：`child_with_global_id`、`into_child_with_global_id`（应为重建）。
- **状态不符 1**：`new_at_pos<String>`（无 src，非「判等」）。
- **微分歧 1**：`NodeContext::node`（expect vs unwrap）。
- **重建降级 1**：`process_input_action_handlers` → 未实现-需重做。
- 其余 25 判等与 6 重建/胶水主张 fresh 复核成立；safe_frame/new_at_pos 门控、on_*_event
  tag 偏移、new_at_pos<String> 物理存在等关键主张均超既有分析被坐实。

## 7. FIX 落实复核（restorer 已实现，2026-10-17 第二轮 fresh 对照）

| FIX | 落实 | 对照原反汇编 |
|---|---|---|
| 1 child_with/into_child 父缺失 panic | `get_mut(&self.id).unwrap()`（不再 reuse_or_create_node 创建父） | 原探父表缺失→unwrap_failed，命中→children push；吻合 |
| 2 node() 用 expect | `.expect("NodeContext::node: node missing")` | 原 deref 0x140681d50 缺失→expect_failed；吻合（串 U） |
| 3 NodeContext.mei 裸指针 | `*mut MeiContext` + `mei_mut/mei_ref` 重借 | 原 NodeContext+0x00=mei 指针、+0x08=id；吻合 |
| 4 calculate_hash 零键 | `DefaultHasher::new()`（SipHasher13 零键） | 原 0x1405bdda0 `write<Sip13Rounds>`×2、375B 无 RandomState 机制；吻合（零键判定，确定性恢复） |
| 5 new_at_pos<String> src | `new_at_pos_string(name: String,…)`，id 经 `GlobalId::new(&name)`（=from_name 同源），String 帧末 drop | 原 0x142289f80（268B）结构逐字段同 <&str>、末尾 `if(*in_R8) dealloc`；吻合 |
| 6 load_configs 存 self | MeiContext 补 `configs` 字段，`self.configs.insert(...)` | 原 `HashMap::insert<String,Arc<NodeConfig>>` 入 self；吻合 |
| 7 Node::new used_frame=0 | `used_frame: 0` | 原插入路径 +0x5d 区零；吻合 |
| 8 read_node_config Options | 保留 `Options::default()`，登记 ron 0.8.1 核对（recursion_limit 0x80=128 吻合；default_extensions 1 vs 0 疑 IMPLICIT_SOME，I 级残留） | 原 Options{1,0x80}；0x80 已对，1 差异 I 级保留 |

**process_input_action_handlers 重写对照（原 0x140687eb0，1279B）**：原分歧 4 点全部落实——
(1) 仅调 `on_input`（vtable+0x50），删除 process_input_action(+0x48) 调用 ✓；
(2) 正向遍历首个 handled 即 `break`（`stop=i`），与原 `local_80=uVar8; goto LAB_140688271` 早停一致 ✓；
(3) 反向自停点 `ids[..end].rev()` re-apply `on_input`，命中 handled 即返回，**不修改 handler 表**（原对 +0x1d0/+0x1d8/+0x1e8 无写入）✓；
(4) 边界核对：正向命中 index stop → 原 reverse 覆盖 [0, stop+1)（含 handled 元素二次调用），Rust `end=stop+1` 覆盖 [0..=stop]，同构；无命中 → 全覆盖反向。剩余 U：on_input 第二/三参状态线程（local_70/local_64）、初始缓冲元素与收集去重细节——已诚实登记。
重写后状态：**重建（I）** 成立（不再降级未实现）。

**无新过度断言**：`new_at_pos_string`/`child_with_global_id`/`into_child_with_global_id`/`node()` 恢复判等注释均与反汇编吻合（child_with/into_child 依赖 reuse_or_create_node 重建，同 safe_frame 依赖级）；`cargo check -p toolbar` 0 错、`cargo test -p toolbar` 1/1 通过（复核坐实）。

## 8. 复核结论（fix_verified）

8 条 FIX 全部落实且与原反汇编逐条对照吻合；process_input_action_handlers 重写修正了原 4 处控制流分歧，降为重建(I) 合理；无新过度断言。残留 I 级项（ron Options default_extensions=1、on_input 载荷线程、reuse_or_create_node 身份回收、new_at_pos_string 的 String 入参与原 ABI 的寄存器/栈传参差异）均已登记，不影响判等/重建判定。
