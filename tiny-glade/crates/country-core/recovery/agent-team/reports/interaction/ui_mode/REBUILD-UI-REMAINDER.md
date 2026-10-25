# REBUILD — UI 域未审计模块真实逻辑重建(rebuild-ui-remainder)

日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md(§重建前必做分类 + 逐语句判等自检)
LOGIC.md: reports/interaction/ui_mode/LOGIC-UI-REMAINDER.md(1 个 `未实现-需重做`)
证据: ghidra `--worker shared` fresh 反编译 + read_memory 机器码(gui-b1/…、serde-*、photomode-prefab-b1 等,见 LOGIC.md)。

## 分类结论(协议 §2026-10-17 用户补充)

需重做候选共 1 个:`VideoSettingsSubMenu::is_in_submenu`(gui.rs)。其余 34 判等函数经逐语句核对无分歧;
21 个无 body 原函数全部为真业务(prefab 资产系统/ grading 未恢复族),按协议登记,不重建。

| 原函数 | VA | 分类 | 结论 |
|---|---|---|---|
| `VideoSettingsSubMenu::is_in_submenu` | 0x14093fcd0 | **真实业务(单字节条件)** | 重建 |
| `DebugGui::default`(impl$9::default) | 0x14093fce0 | 真实业务(315B 字段写,非 derive 空壳) | 判等(已在 src) |
| `Toolbar::new` / `contains` / `switch_mode` | 0x140af8c50/89d0/9850 | 真实业务 | 判等(已在 src) |
| `GradingLut::load_cube` / `parse_rgb` | 0x140b01050/0x140b01cc0 | 真实业务 | 判等(已在 src) |
| image_formation 五方法+default | 0x1408ae7f0/aea50/aea60/aea80/aea90 | 真实业务 | 判等(已在 src) |
| photo_mode next/next_demo/from | 0x140b59720/0x140b596b0/0x14090e500 | 真实业务(next 非 derive,是 packed 表算法) | 判等(已在 src) |
| prefab 16 业务函数 | 0x1408e2540 等 | **真实业务(未落库)** | 登记,不重建 |
| grading_lut 5 业务函数 | 0x140b00b20/bc0/df0 等 | **真实业务(未落库)** | 登记,不重建 |

胶水-非业务(不重建):gui 19、toolbar serde/derive 18、grading serde 15、image_form serde 15、
prefab 6、photo_mode build/eq/derive 3——derive/register/plugin-build 胶水,直接复用 derive 或
保持 serde 字段策略(ImageFormationConfig 的 GradingSave/LensSave/ArtisticSave 已与原生 3 单体一致)。

## 逐函数:分类 → 重建 → 逐语句判等自检

### VideoSettingsSubMenu::is_in_submenu(0x14093fcd0)— 真实业务,重建

- fresh 机器码(7B):`80 39 00  0f 95 c0  c3` = `cmp byte [rcx], 0; setne al; ret`
  → 返回 `*self != 0`(tag ≠ Menu)。
- 分歧:Rust 旧 body `matches!(self, Self::Menu)` 返回 `*self == 0` —— **条件反向**(与注释
  "B is exactly tag == 0" 均误)。
- 重建:`pub fn is_in_submenu(self) -> bool { !matches!(self, Self::Menu) }`,
  注释改为机器码事实。
- 判等自检:`cmp byte[rcx],0` 目标字节即判别子;`setne` = 不等置位,与 `!matches!(Menu)` 逐位一致。
  Menu=0 → false;Image=1 / Display=2 → true(原生语义:处于视频设置子菜单)。

## 重建后验证

- `cargo check -p country-core`:**通过**(lib 零错误;10 warnings 均为既有 snake_case/未用 import,
  与本次改动无关)。
- `cargo test -p country-core --lib -- gui_ toolbar_ photo_ grading_ image_form`:23/23 通过。
  `-- toolbar`:2/2 通过。重建的 `is_in_submenu` 经编译;无独立单测,由 gui 现有测试集承载。
- rhapsody 历史 E0624(ResourceSpec::ConcurrentBuffer/Image 缺分支)为 repo 既有独立错误,
  复检为 stale artifact,重跑后消失,与本模块无关。

## 留给 reviewer 的核对点

1. `is_in_submenu` 反向修正是否接受(机器码 setne 为铁证;注释同步改)。
2. `EscMenuStateView`/`view()` 结构重建(word=tag|payload<<32, flag@+8)与原生 12-qword
   enum 的位级一致性已在 is_closed/is_open/should_show_world/can_limit_frame_rate 逐函数坐实。
3. image_formation `get`(0x1408aea80)原生无边界检查(越界 UB),src 以 `configs.get(i)` 返回
   None 作安全适配——是否接受为判等(安全适配),而非未实现。
4. `advance_menu_state`(0x14068e5b0)为 toolbar::menu 泛型单体:事件类型由 caller 签名
   `EventWriter<EscapeMenuJustClosed>` 承明,反编译体 Vec 增长 ICF 折叠到 WriteSettingsFileCmd——
   不据 ICF 标签改判事件身份。
5. prefab/grading 未落库函数登记后,资产系统/load_unwrapped 等依赖 asset/设备接线,
   由上层模块批次推进。
