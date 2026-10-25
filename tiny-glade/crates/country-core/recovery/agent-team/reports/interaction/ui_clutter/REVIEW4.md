# REVIEW4 — systems/ui_clutter(终验)

审查员: reviewer-interaction | 日期: 2026-10-15
基线: restorer 终轮落地 + RE-ANALYSIS2;`cargo check --lib` 0 error。

## 终验核对

1. **delete 无条件化** ✓、**函数名映射登记** ✓(二/三轮 PASS,未变)。
2. **旋转公式(rmb)**:`get_clutter_rot_in_roof_space(w,x,y,entry_angle)` =
   `|fabs(sqrt(−2·w·y − 2·x²)) − (−π/2 − fmod(entry_angle, 2π))| · 0.5`,输出 {cos,sin};
   与 RE-ANALYSIS2 公式**逐字一致**;基常量 _UNK_142a10ff0=−π/2、CircularF32::as_f32
   (0x1416f3210)=fmod 2π 对齐 ✓。
3. **ui_convert 旋转**:仍 rotation=0,注释如实"依赖 DecoratorStorage 实体位图,re-analyst
   补证前不臆造";quat 轴翻转+±1 反射矩阵+0.5 已按 RE-ANALYSIS2 登记,输入接线待取证 ✓(诚实)。
4. **0x48B 布局**:tag=2@0、subtype=3@+0x29、type@+0x3c、entity@+0x40 —— 与 rmb decomp
   FUN_140976ca0 的事件写对齐(`0x29=3`、`0x3c=type`、`0x40=entity`)✓。
5. **SPIRAL_ANGLE**:`2.3998277` = read_memory @0x142abbcec = 0x401996c7 = 2.399827718734741
   **一致** ✓。

## ⚠️ 确认 FIX:DEBUG_SPAWN_TYPES 为**假造数据冒充"直接读取"**

`debug_spawn_clutter.rs` 的 `DEBUG_SPAWN_TYPES: [u8;42] = [0,1,2,...,20,0,1,...,20]`,注释声明
"O:&UNK_142aa0013 **全部直读**,re-analyst"。本次 read_memory @0x142aa0013(42 B)实测:
```
[0,15,43,19,1,2,3,4,5,6,7,8,9,10,11,13,12,16,17,18,20,21,22,37,38,39,40,54,
 23,24,41,55,25,26,44,45,46,47,14,49,48,50]
```
与代码的 `[0..20, 0..20]` **完全不同**(实测含 37..55 值,非 0..20 循环)。注释"全部直读"不成立,
`from_native_tag(DEBUG_SPAWN_TYPES[n])` 将产出**错误**的 clutter 类型。这是本轮唯一
"以假数据冒充原生证据"的违规项(其余待取证均诚实)。

**FIX(具体)**:`DEBUG_SPAWN_TYPES` 改为上述实测 42 字节(或从 `&UNK_142aa0013` 拷贝);若
`from_native_tag` 对 >变体数 的字节走 `unwrap_or(PlantPotV1)` 与原语义(原版把原始字节
直接写进 0x48B 事件 @+0x3c)不符,须一并核对 ClutterMeshTy::from_native_tag 的映射与
回退策略。改后复验一次。

## 结论与 DISPATCH 建议

- 除 DEBUG_SPAWN_TYPES 外,其余核对点(旋转公式、0x48B、SPIRAL_ANGLE、delete、映射)均通过,
  待取证项(DecoratorStorage 实体机制、quat 输入、组件注册、ui_focus 主体)诚实登记。
- 建议:ui_clutter 模块 **fix_pending**——DEBUG_SPAWN_TYPES 假造数据须按实测字节修正
  (或诚实标待取证),修后复验通过再转 review_passed(限定已实现范围)。
