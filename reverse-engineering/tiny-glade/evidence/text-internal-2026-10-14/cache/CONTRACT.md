# B tg-text 内部 frame/cache 合同

本 owner 只写此 evidence 子树，生产 Rust 与共享台账由父 owner 修改。静态来源为原 RNE、原 PDB module624/1782 与 `tg-text-full-2026-10-13` 反编译；原依赖实现继续复用。复核不涉及调用方/GPU。

## 字段与写入图

| 字段 | 原 offset / 类型 | 初始值与唯一相关写入 |
|---|---|---|
| renderer current_frame | +0x4c8 `u32` | new=1；consume 成功、upload 返回后 `incl`，wrapping32 |
| renderer invalidation_stamp | +0x4cc `u32` | new=1；scale 有序不相等/NaN时复制 current_frame；overflow upload 成功末尾复制 current_frame；普通 upload 不写 |
| fragment primitive_stamp | +0x24c `u32` | 构造0；validate失败0；generate完成复制 renderer stamp；不会自增 |
| fragment validated_at | +0x250 `u32` | 构造0；只有validate全通过/空glyph IDs时写current_frame；generate完成不写 |
| usage Vec | renderer +0x3f8 cap / +0x400 ptr / +0x408 len | 每slot8 bytes；+0 last_used `u32`、+4 generation `u16`、+6 pad2 |
| allocation ID | etagere `u32` | low16 slot index，high16 generation；原库产生，不能用独立自增counter替代 |

new 原 `0x140b8ec5b` 的常数 `0x100000001` 一次写两u32。scale `0x140b8f415/41b` 复制，不推进frame，也不推进单独generation；同帧两次scale写相同stamp。setter清glyph map/pending/etagere，**usage保留Vec长度、容量和地址，每slot仅last/generation置零**（`0x140b8f4f0/4f6`，pad不写）。overflow则重建atlas和allocator，释放旧usage Vec并置cap0/ptr4/len0。

overflow upload **先**clear overflow flag，资源装配/替换成功后清glyph map/usage/pending，**最后** `0x140b8fc21/27` 复制current_frame到stamp。失败/panic若发生在装配前段，不能擅自提前写stamp。consume `0x140fac946/951` 清两流len → `0x140fac963` upload → `0x140fac968` current_frame wrapping_add(1)。overflow_stamp是消费前frame，绝非frame+1。上传回调失败前count snapshot已写，但流len/frame未清/推进；upload失败在流len清后、frame推进前。

## render / generate 决策原序

原 `TextFragment::render 0x140b91120` 内 inline `generate_primitives`：

```text
if editor.redraw():
    generate
else if primitive_stamp < renderer.invalidation_stamp:  // unsigned u32
    generate
else:
    if validated_at < renderer.current_frame:           // unsigned u32
        for raw_id in fragment.ids, in original order:
            slot = usage[raw_id & 0xffff]               // bounds panic if index >= len
            if slot.generation != raw_id >> 16 or slot.last_used == 0:
                primitive_stamp = 0
                break
            slot.last_used = renderer.current_frame    // immediate; retain prefix effects on later failure
        else:                                          // includes empty IDs
            validated_at = renderer.current_frame
    if primitive_stamp < renderer.invalidation_stamp:
        generate
    else:
        reuse
```

stamp比较是 `<`，不是 `!=`。renderer stamp==0时validation失败写0后仍可reuse；renderer frame0时 `validated_at < 0` 恒false。回绕后旧MAX stamp/frame不会自动视为过期，不能以合理化cache行为为由补强比较。没有always-validate、two-pass先检查全组、单独u64epoch或same-frame强制失效。

generate先清primitive len与IDs len，设置buffer align/size，再调用原cosmic Editor::shape_as_needed(prune=true)，随后生成primitive/IDs。成功只更新primitive_stamp=renderer invalidation_stamp；validated_at保持原值。**原generate/render不调用set_redraw(false)**：原1028字节函数无此写，PDB只有redraw getter；依赖shape_as_needed只清cursor_moved，buffer redraw可能保持true或被依赖置true。不得添加生成后人为clear redraw。

原generate bounds.y=`scaled_line_height + max(0, layout_run.line_top)`，按原f32顺序先max line_top、最后一次加line_height（`0x140b913e6/ec/f8/3ff`），不要改成逐行max(line_top+line_height)。prepare的y为scaled_line_height+0；多行render之后可扩展y。bound/closure证据在原 `0x141801980.c`，其入口写max(line_top)；这里没有GPU行为。

## get_glyph / 回收

`0x140b8fea0` 命中glyph map后仅拷贝cached payload并return（`0x140b90065`）；**不更新usage.last_used**。usage更新发生于原allocation成功helper `0x141800040` 与上述fragment validate。

- raster无image：直接None返回，不新增glyph map entry。
- raster width==0或height==0：插入placeholder allocation=`0xffffffff`，不分配/写usage。原generate遇此allocation sentinel不emit glyph、不写fragment allocation IDs。
- allocation失败后扫描所有usage slot，只有 `last_used != 0 && last_used.wrapping_add(1) < current_frame` 回收。原 `0x140b90313/15/19/1b/21` 是test/incl/unsigned cmp；不是saturating，也不把last0视为expired。
- 每回收一项：构造完整AllocId（generation<<16 | slot低16），etagere.deallocate，last_used置0，generation保留，并把完整ID加进本次removed HashSet。
- `0x1418008d0` 仅删除cache中allocation完整ID属于**本次removed集合**的条目；不是按usage存在/last非零筛选。没有raster的缓存缺项、placeholder与非本次失效ID不被该操作清除。
- 重试分配成功：usage对应slot写current_frame与AllocId高16、入pending/cache；失败：overflowflag=1，返回None，未把失败glyph插入cache。

## 原指令 oracle

[native_cache_oracle.py](native_cache_oracle.py) 可重跑，解释器为 `D:/tools/reverse-engineering/venvs/reverse-engineering-assistant/Scripts/python.exe`。原RNE SHA固定 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，相关函数range SHA按原PDB快照复核。单次结果 [native-cache-oracle.json](native-cache-oracle.json)：**791项、0差异**。

675 render/validate矩阵覆盖frame0/1/2/MAX-1/MAX、stamp0/1/MAX、validated0/1/MAX及空IDs/正常/后项gen mismatch/后项last0/OOB；40项执行完整未改render warm leaf路径；25 setter compare/store slices；5 same-frame两次scale；25 expiry回绕/0哨兵；15完整usage helper无分配路径；5 overflow stamp copy→consume incl；1 new双字段store。

prefix测试只在原指令决策的三个后继点换成return marker，setter slice止于清理前；它们是受控原指令切片，**不是完整GPU/生成路径或原发行进程执行**。bounds panic仅观察原分支抵达，不执行进程终止。没有保留生产adapter，没有启动MCP重型库，也没有逆向/重写HashSet/hash/etagere/cosmic-text库实现。

本轮closed：frame/stamp写入、无符号比较与回绕、partial validate副作用、slot/gen/sentinel、cache命中usage不更新与本次removed集合retain策略。生成完整字体/多行/失败生命周期与GPU不是这791项oracle的覆盖声明；对应依赖调用与几何细节沿原已有证据恢复。
