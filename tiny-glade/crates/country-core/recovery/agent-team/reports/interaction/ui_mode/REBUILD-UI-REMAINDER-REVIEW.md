# REVIEW — UI 域余量(rebuild-ui-remainder 复验)

审查员: reviewer-interaction | 日期: 2026-10-17
基线: rebuild-ui-remainder REBUILD-UI-REMAINDER.md + LOGIC-UI-REMAINDER.md;
模块测试 34/34(gui 23 + toolbar 2 + photo/grading/image_form 9)。编译注:现全仓唯一 error 为
rhapsody E0277(他组在途,REBUILD 预告的 stale artifact 类),UI 域文件干净。

## 逐项复验

### ① 分类(34 判等 / 1 重建 / 21 无 body / 76 胶水)— PASS
- 34 判等:gui EscMenuState 族/DebugGui::default/Toolbar::new/contains/switch_mode/
  GradingLut::load_cube/parse_rgb/image_formation 五方法+default/photo_mode
  next/next_demo/from——逐语句核对无分歧(已在 src)✓。
- 1 重建:`VideoSettingsSubMenu::is_in_submenu`(0x14093fcd0)✓。
- 21 无 body:prefab 16 + grading_lut 5(真业务未落库),按协议登记不重建 ✓。
- 76 胶水:gui 19/toolbar serde 18/grading serde 15/image_form serde 15/prefab 6/
  photo_mode 3——derive/register/plugin-build 复用 ✓。

### ② is_in_submenu 唯一重建 — PASS
机器码(7B)`80 39 00  0f 95 c0  c3` = `cmp byte [rcx],0; setne al; ret` → 返回
`*self != 0`(tag≠Menu)。旧 `matches!(Menu)` **条件反向**,已改 `!matches!(self, Self::Menu)`
+ 注释同步为机器码事实。Menu=0→false,Image/Display→true ✓。

### ③ 判等抽核 — PASS
- EscMenuStateView(word=tag|payload<<32, flag@+8)与原生 12-qword enum 位级一致性在
  is_closed/is_open/should_show_world/can_limit_frame_rate 逐函数坐实 ✓。
- Toolbar::new 11 分类表(demo/full)✓;grading_lut load_cube/parse_rgb ✓;
  image_formation default 3 档/switch_kind ✓;photo_mode next packed 表 0x00030201/
  next_demo 跳表/from tag+1 ✓。

### ④ 判等注意 — PASS(接受安全适配)
- image_formation `get`(0x1408aea80)原生无边界检查(越界 UB),src `configs.get(i)` 返回
  None 作**安全适配**——接受为判等(安全适配,非未实现)✓。
- `advance_menu_state`(0x14068e5b0)toolbar 泛型单体:事件类型由 caller 签名
  `EventWriter<EscapeMenuJustClosed>` 承明,反编译体 Vec 增长 ICF 折叠到
  WriteSettingsFileCmd——不据 ICF 标签改判事件身份 ✓。

### ⑤ 无 body 21 登记诚实 — PASS
prefab 16(资产系统/load_unwrapped 依赖 asset/设备接线)+ grading_lut 5——登记待上层模块
批次推进,不冒充 ✓。

## 结论与 DISPATCH 建议

- UI 域余量重建复验通过:**is_in_submenu 反向修正接受、34 判等无分歧、21 无 body 诚实登记、
  安全适配接受**,模块测试 34/34。
- 建议:UI 域余量 → 闭合(实现层面);prefab/grading 未落库资产系统与 advance_menu_state
  ICF 事件身份由上层模块批次推进。rhapsody E0277 为他组在途,不属本域。
