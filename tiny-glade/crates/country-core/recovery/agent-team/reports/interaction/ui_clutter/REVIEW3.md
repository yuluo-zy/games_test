# REVIEW3 — systems/ui_clutter(三轮 FIX 复验)

审查员: reviewer-interaction | 日期: 2026-10-15
基线: restorer 已按 REVIEW2 修复旋转输入;`cargo check --lib` 0 error。

## 三轮 FIX 复核

### ui_convert 旋转输入 — **PASS(诚实待取证,不冒充)**

- 不再传 `acos(position.y)`(二轮指出的疑似错输入已移除)。
- `ui_convert_autoclutter_to_decoclutter` 现 `rotation = 0.0`,注释明确"输入向量(roof 空间
  朝向)依赖 DecoratorStorage 实体位图,re-analyst 补证前不臆造——旋转暂置 0 并登记待取证"。
  **诚实标记,未冒充完成** ✓。
- `get_clutter_rot_in_roof_space(dot)` 保留 `dot.clamp(-1,1).acos()` 合同(ac 链结构),dot
  输入来源待 re-analyst 按 decomp 0x14099ca60 定。
- 0x48B spawn 事件布局常量(tag=2/subtype=3@+0x29/type@+0x3c/entity@+0x40)登记不变 ✓。

## 结论与对 DISPATCH 建议

- 旋转输入 FIX **复验通过**(删除了无证据的 acos(position.y),改为显式待取证)。
- 维持 `in_progress`;待 re-analyst 补证据后闭合:
  1) roof 空间朝向向量/旋转公式(DecoratorStorage 实体机制);
  2) debug_spawn 的 `SPIRAL_ANGLE=_UNK_142abbcec` 精确值 + 类型字节表 `(&UNK_142aa0013)[n]`;
  3) decoclutter_show_pins 组件注册(现 register_required_components 空存根)与图钉实体切换;
  4) ui_focus_decoclutter 状态机主体(现门存根,不计数闭合)。
- delete_auto_clutter 无条件删除 + 原名映射维持前轮 OK。
