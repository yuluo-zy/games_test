# REVIEW — systems/ui_clutter

审查员: reviewer-interaction (独立核对,以原证据为准)
日期: 2026-10-15
审查范围: 当前 src 已有实现 + 原函数清单逐项核对。restorer 尚未提交新增(拾取/生成可见消费),本次审已有现状。

## 核对方法

- inventory.json 枚举原函数 + 章节 + 既有 decomp 全文核对:
  - `history_record_auto_clutter` @0x1409a9160(既有 decomp FUN_1409a9160 全文)。
  - `rmb_autoclutter` @0x140976ca0(既有 decomp FUN_140976ca0,用于对照 delete_auto_clutter 适配器缺口)。
  - `mod.rs` 资源(章节:mod.rs 无独立业务函数,仅 default 胶水)。

## 原函数清单(no_formal_mapping ≠ 零实现)

| 原函数 | VA | 当前 src | verdict |
|---|---|---|---|
| `mod::DeletedAutoClutter`(impl$1::default 胶水) | — | mod.rs HashSet<u64> | **OK** |
| `defered_autoclutter_history::history_record_auto_clutter` | 0x1409a9160 | defered_autoclutter_history.rs(改名) | **OK**(改名待登记) |
| `rmb_autoclutter::handle_rmb_autoclutter` | 0x140976ca0 | 缺失(有 delete_auto_clutter 适配器) | GAP |
| `ui_autoclutter::ui_convert_autoclutter_to_decoclutter` | 0x14099ca60 | 缺失 | GAP |
| `debug_spawn_clutter::debug_spawn_all_clutter` | 0x1409a1f50 | 缺失 | GAP |
| `decoclutter_show_pins::decoclutter_show_pins` | 0x1412357c0 | 缺失 | GAP |
| `decoclutter_show_pins::register_required_components` | 0x141237200 | 缺失 | GAP |
| `decoclutter_show_pins::clone_behavior` | 0x140002370 | 缺失 | GAP |
| `ui_focus_decoclutter::ui_focus_decoclutter` | 0x1409a92c0 | 缺失 | GAP |

## 逐函数核对

### 1. DeletedAutoClutter(mod.rs) — **OK**

章节:mod.rs 原版仅 `default` 胶水(资源),B 0x140aa4a60 消费一张 8B id 表;PDB default = HashSet::default。当前 `DeletedAutoClutter(pub HashSet<u64>)` + `is_deleted`(访问器,非原函数,无害)一致。✓

### 2. defered_autoclutter_history / history_record_auto_clutter @0x1409a9160 — **OK**

既有 decomp(FUN_1409a9160)核对:
- 排空 `RecordAutoClutterHistory` 事件(stride 0x10,载荷 u64@+8)。
- 每条:alloc ArcInner 0x18 `{strong=1, weak=1, value=载荷}` → `FUN_140a84ec0(writer, tag 0xd, UNK_142abd990)` 入历史。
- 当前 Rust:逐事件 `EditType::AutoClutter(Arc::new(AutoClutterEdit::Delete{id}))` + `history.add(...)`,失败写 HistoryRecordingFailed。**语义一致**(tag 0xd=AutoClutter;当前 History::add 对应 FUN_140a84ec0 的返回 Result 适配;失败路径为当前历史合同适配,原版无显式失败分支)。✓
- **注意**:当前函数名 `defered_autoclutter_history` 与原函数名 `history_record_auto_clutter` 不同。contracts/台账应按**原函数名**登记映射,不能以改名代替闭合。建议在函数 doc 或 RESTORE.md 里显式登记原名映射。

### 3. delete_auto_clutter(新适配器,非原函数) — PARTIAL(适配器缺口)

头注释自认"not the still-unrestored native RMB picking/conversion function"。对照原 rmb_autoclutter @0x140976ca0 的 decomp:
- 原版:**消费右键命中**(hashbrown SIMD probe + 实体位集 bitmap),计算朝向/旋转(FUN_140923e20 开方 + FUN_1408b83d0),push `EvRightClickedAutoClutter`(0x48B,pos/rot,@+0x29=3),末尾 alloc Arc → FUN_140a84ec0(tag 0xd) 入历史。
- 当前适配器:消费 `DeleteAutoClutterCmd(u64)`(ID 由调用方解析),`!history.is_recording()` → HistoryRecordingFailed(跳过);否则 insert DeletedAutoClutter + 写 RecordAutoClutterHistory。
- **缺口**:
  1. 右键命中→ID 解析、旋转/变换、EvRightClickedAutoClutter 事件、deco-clutter 转换(ui_autoclutter)全部未实现 —— 即 DISPATCH 验收"生成/拾取"链缺。
  2. 原版删除/记录**无条件**(直接 push 事件+历史),当前适配器在未录音时**跳过删除** —— 与原生行为不一致(原生此时仍会发出删除事件)。若适配器要保留录音门,应只影响"是否写历史",不应阻止删除本身。

### 4. registration.rs — PARTIAL

只挂 delete→record 链(HistoryCpuStage::Record),原 add_ui 挂载 6 个 clutter 系统(debug_spawn/decoclutter_show_pins/defered/rmb/ui_autoclutter/ui_focus_decoclutter)全缺。

## 对新增工作(restorer 批次:拾取/生成可见消费)的核对基线

- `rmb_autoclutter`@0x140976ca0、`ui_autoclutter`@0x14099ca60、`debug_spawn_clutter`@0x1409a1f50、`decoclutter_show_pins`@0x1412357c0(+@0x141237200/@0x140002370)、`ui_focus_decoclutter`@0x1409a92c0:按章节既有 decomp 闭合。生成链(debug_spawn 螺旋 42 个 + FUN_1409e9640)与转换链(ui_convert 旋转)逐项 fresh 反编译。

## 结论

当前已有实现仅覆盖"删除集合 + 延迟历史"一小段,与原生命名有映射差异(函数改名未登记),其余 7 个原函数(含右键拾取/转换/图钉/焦点)全部缺失。delete_auto_clutter 的录音门导致与原生"无条件删除"行为分歧。ui_clutter 维持 queued,新增工作按基线核对,并需补函数名→原函数名映射登记。
