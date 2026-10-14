# tg-text 内部字体合同

2026-10-14，B Windows x86_64。只读研究，未修改生产 Rust、调用方、GPU 或共享跟踪。源证据复用前轮原 PDB module624/625/1185/1782；本轮重新扫描完整 PDB IPI/PROC，重新读取 RNE 字节，并受控执行原字体 CPU 入口。

## 确定的原模型

`TextFragment` 持有一个拥有所有权的 `Arc<FontTable>`，原 Arc allocation pointer 保存在 +0x230，table 数据由 allocation+0x10 访问。原 `FontTable` 是 `HashMap<FontFamily, FontTableEntry, RandomState>`；entry 持有 family String 和 scale f32。`FontFamily` 的真实 tag 经原 enum visitor 运行确认：Label=0、MenuButton=1、TextEditor=2、Hand=3。没有 Custom/None 字体角色；visitor 只有这四个成功名字分支，未知名字走 serde error。align None 的 tag5 属于另一字段，不是字体角色。

原 metrics 状态有两个 f32：+0x240 字体大小、+0x244 行高，表示未缩放基准；当前 cosmic Buffer 的 metrics 是实际缩放结果。PDB 采用 line-tables-only/void()，没有恢复 metrics 目标类型的作者名字。IPI 记录 `tg_text::impl$5::from`(0x86aa) 与 `impl$6::from`(0x86ac)，前者没有 emitted PROC/INLINESITE host；后者的唯一发射 `0x140b906c0` 4字节将 f32复制到两个返回寄存器，明确 `From<f32>` = `(v,v)`。完整PROC库存只有 `<f32>` new 和三份 `<f32>` set_metrics，没有 tuple 实例。二f32类型的恢复命名可以保留来源说明，不能宣称PDB证实作者类型叫 `FontMetrics`，也不能由 impl$5 编号推断 tuple `From` 合同。

`TextFragment::new<f32>` 的实际受控输入为 `(out, scalar:f32, role:u8, Arc<FontTable>)`，并不接受 TextRenderer/FontSystem。构造先查 role，调用 entry.scaled、创建 `Buffer::new_empty(scaled_metrics)`、`Editor::new`，创建 `AttrsOwned::new(Attrs::new().family(Family::Name(entry.family)))`；将传入 Arc 直接 move 到片段，无 clone。角色保存 +0x255；未缩放基准保存 +0x240/+0x244。初值：buffer.lines为空，bounds=(0,0)，max_width=FLT_MAX，align=None(tag5)，cursor-visible=false，attrs.weight=400，其他 attrs 使用原 cosmic defaults。actions/primitives/glyph refs为空，上一帧/上一失效stamp均0。

应直接复用 Arc/HashMap/String/AttrsOwned/Buffer/Editor API，原库内 refcount、HashMap hash 和内存布局不应成为生产手写实现。

## 状态转移：按原作保留

1. **new(metrics, role, table)**：metrics 转为二f32；table.get(role).unwrap；按 `scale * input` 得到实际metrics；空buffer/editor；拥有当前字体名称 attrs；move Arc；保存初始role和未缩放metrics；没有 renderer 参数、没有提前shape/load fonts，也没有自动 Buffer::new 的空行。
2. **set_family(requested_role)**：若 requested_role 等于**构造时保存的 role字段**，直接早退。否则临时 clone当前Arc，查requested_role并unwrap，替换attrs.family_owned，apply_buffer_attrs，再以基准metrics与requested entry.scale重建editor；最后临时Arc被drop。**全函数没有写 +0x255，永久stored_role不变化**。这不是根据反编译猜测：原入口运行已确认切换到1后stored_role仍0、切回0早退、后续set_metrics查0。恢复代码必须保留此观察，不擅自修正这个原行为。
3. **set_metrics(metrics)**：与未缩放基准两分量有序相等则早退；NaN不相等。否则按stored_role查当前共享table、entry.scaled、set_internal_scaled_metrics，最后写基准两分量。不会修改attrs.family。由此即使某次set_family使用了另一个角色，set_metrics仍可能使用初始角色的scale。
4. **set_internal_scaled_metrics(font_size,line_height)**：与当前buffermetrics有序相等则早退。否则移走旧buffer.lines，new_empty(newmetrics)，把lines装入newbuffer并Editor::new替换旧editor。旧cursor/selection/scroll/wrap/size/indent/tabwidth/change丢弃；fragment外层Arc、role、基准metrics、attrs、actions不变。依赖 `Buffer::new_empty` 的 line_height!=0 panic 保留；不能替换为 `Buffer::set_metrics` 的不同初始化/校验。
5. **set_weight(weight)**：相同u16早退；不同更新attrs.weight并apply_buffer_attrs，不修改metrics/role。
6. **apply_buffer_attrs**：对每条 line执行 `AttrsList::new(current_attrs)` 后 `line.set_attrs_list`，即新列表没有spans；随后buffer.redraw=true，即使lines为空也置dirty。原closure通过三种 cosmic BufferRef形式取buffer，不能将这些分支误称字体role分支。
7. **prepare**：排空actions到editor.action；设置align=None、Wrap::Word、size(max_width,FLT_MAX)、shape(false)；width取max(layout line_w)，height取**当前buffer scaled line_height +0.0**。原+0x38的Owned/Borrowed/Arc BufferRef分支与font role无关，没有table条目line_height字段。prepare不重新查table、也不再应用scale。
8. **take_editor**：返回旧Editor，片段内改为以**当前buffer scaled metrics**创建的空Editor。Arc、stored_role、基准metrics、attrs和pending actions保持；不把基准metrics再次缩放，也不设置max_width/预先shape。

## 数值表达式和失败边界

- new/set_metrics 调用 `FontTableEntry::scaled`：两分量均 `entry.scale * input`；返回 family的借用str和两f32，无 clamp。
- set_family 的机器码具有不同操作数顺序：`font_size = base_font_size * scale`，`line_height = scale * base_line_height`。NaN payload传播可能因SSE/LLVM产生差异，不能统一表达式后宣称每位等价。
- lookup缺失全部走原 `Option::unwrap` panic标准消息 `called Option::unwrap() on a None value`。原 new位置lib.rs545:48，set_metrics622:18，set_family661:47；详见panic-locations.json。不能使用自定义 `font role missing from font table` 模板来代替。
- set_family会临时clone/drop Arc，fragment持续持有同一table；new是所有权move。只读取该Arc中的table，不建立每次传入的另一个字体表、不缓存候选scale字段来替代原lookup。

## 原指令验证

`native_font.py` / `native-font.json` 共17组通过：四个字符串visitor tag、四个FontTable::get成功和一个缺失、四个new角色、switch→switch-back→set_metrics三状态、take_editor一状态。原 set_family 后role不变、Arc临时clone/drop后strong回到1、基准与实际metrics分离、动作队列保留都由实际执行确认。

fixture明确使用原hash函数填预分配HashMap桶；通用allocator/memcpy/memset/memcmp/free调用跳到CRT边界，布局请求不超过8byte alignment；dummy Arc header用于局部refcount观察，没有执行last-drop。没有启动游戏或GPU，没有运行原missing-role panic/unwind，真实原allocator、完整Arc last-drop、prepare整形/attrs spans入口本轮未动态验收。原Shared Arc布局仅作为oracle输入，不写入生产Rust。

机器证据：new-setmetrics.asm.txt、get-scaled.asm.txt、role-visitor.asm.txt、apply-attrs.asm.txt、family-owned.asm.txt、attrs-list-new.asm.txt；set_family的完整544B原汇编沿用 `evidence/text-followup-2026-10-14/set-family.asm.txt`。字节/PDB身份和函数范围见contract.json。原发行SHA `f507a6ce…`，本轮没有改持久数据库，无新backend需要停止。调用方/GPU按用户明确要求不处理。
