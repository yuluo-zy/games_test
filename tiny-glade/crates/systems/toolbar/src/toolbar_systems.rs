//! toolbar 系统：`toolbar_alignment`（O 0x142344560）与 `toolbar_auto_hide`
//! （O 0x142343a10）。wave2b fresh 反汇编 + 跳表 0x14304e6d8 解码；
//! wave3（restorer-wave3-toolbar-layout）fresh 复核 + caller SystemParam 签名。
//!
//! **toolbar_alignment**（0x142344560）签名（caller mangled 名）：
//! `Res<country_core::game_config::GameConfig>, Res<ToolbarLayout>, Res<PhotomodeToolbarLayout>,
//! ResMut<TaffyLayout>`。前置清位循环已重建；分派 case 调 `taffy_mut_style` 单态族
//! （0x142375b40..0x142376300，`Taffy::style→clone→closure→set_style` 模式已证），
//! 但各 closure_env 的 **setter 目标（taffy Style 字段）U** —— 见下。
//!
//! **toolbar_auto_hide**（0x142343a10）与 undo_redo_alignment/undo_redo_auto_hide/
//! toolbar_steal_mouse_input 共享尾区（LLVM 合并）；auto-hide 阈值/数学已取证，
//! 系统参数接线与 taffy setter 目标 U（未实现，见模块尾）。

use taffy::node::{Node, Taffy};

/// `toolbar_alignment::closure_env_*` setter 目标（O：fresh 8 个
/// `taffy_mut_style` 单态反汇编的 inline 写面 + taffy 0.3.13 `Style` 精确偏移）。
///
/// 偏移依据（`/tmp` 探针按 workspace taffy 0.3.13 实测）：`Style` 344B/align 8；
/// `flex_direction@0x14b`、`align_items@0x14d`、`justify_content@0x152`。
/// `Option<AlignItems>` 使用 niche 编码：`Some(FlexStart)=2`、`Some(FlexEnd)=3`、
/// `Some(Center)=4`；`Option<AlignContent>` 同：`Some(FlexStart)=2`、
/// `Some(FlexEnd)=3`、`Some(Center)=4`。
///
/// 各单态写面（逐指令，地址/`byte [RSP+off]`）：
/// - toolbar_alignment closure_env$0  (0x142375c10)：`[0x1c2]=2` → justify_content = Some(FlexStart)
/// - toolbar_alignment closure_env$2  (0x142375eb0)：`[0x1bb]=1` → flex_direction = Column
/// - toolbar_alignment closure_env$10 (0x142375b40)：`[0x1bb]=0` → flex_direction = Row
/// - toolbar_alignment closure_env$11 (0x142375ce0)：`[0x1bd]=3` → align_items = Some(FlexEnd)
/// - toolbar_alignment closure_env$12 (0x142376050)：`[0x1c2]=4` → justify_content = Some(Center)
/// - toolbar_alignment closure_env$13 (0x142376300)：`[0x1bd]=2` → align_items = Some(FlexStart)
/// - undo_redo_alignment closure_env$0  (0x142376230)：`[0x1c2]=3` → justify_content = Some(FlexEnd)
/// - undo_redo_alignment closure_env$1  (0x142375f80)：`[0x1bd]=4` → align_items = Some(Center)
///
/// **未实现（U）**：`toolbar_alignment::closure_env_1/3..9` 在 B 中**没有**
/// `taffy_mut_style` 单态。2026-10-17 restorer-mei-body 重新核对
/// `evidence/toolbar-origin-2026-10-16/procs.tsv` 全表：`taffy_mut_style` 单态
/// 恰为 10 个 —— toolbar_alignment 的 closure_env 0/2/10/11/12/13、undo_redo
/// 的 closure_env 0/1、`toolbar_button_size_and_padding_update::closure_env$1`、
/// `taffy_layout::impl$1::perform_layout::closure_env$1`。closure_env 1/3..9
/// **无独立符号**（未实例化或已内联），无法取得 setter 写面，**不伪造**。
///
/// **case 4/6 二跳（2026-10-17 fresh 补充证据）**：
/// 主跳表 0x14304e6d8 的条目 4/6 都指向 `0x14234485d`。该地址是**二级跳表**：
/// `lea rcx,[0x14304e6e8]; movsxd rax,[rcx+rax*4]; add rax,rcx; jmp rax`。
/// 该二级表与主跳表物理重叠（0x14304e6e8 = 主表 + 4 项），是 LLVM 的
/// 跳表后缀共享；条目按主表偏移解释时得到 4→`0x1423448d6`（三级跳表
/// 0x14304e6f8 的派发）、6→`0x142344915`（尾段中段地址，非合法指令起点）；
/// 按后缀表解释时 4/6→`0x14234486d`（= `taffy_mut_style<closure_env_13>` 尾段）。
/// 两级解释互斥且都只部分自洽（RAX 在 `jmp rax` 后是目标地址而非索引，
/// 与二级表的 `[rcx+rax*4]` 用法冲突），因此 **case 4/6 未实现（分歧：
/// 二级表基址/索引寄存器语义）**，不落近似 setter 序列。
/// 另：0x1423447af 起的 per-item 循环（stride 0x20，`+0x10 != 0` 项）对
/// alignment ∈ {1,2} 先 `closure_env_2` 再 `closure_env_13`；alignment ∈ {0,3}
/// 先 `closure_env_10` 再 `closure_env_13`（0x1423447c0/0x1423447d0 段已逐指令）；
/// 其余 alignment 走二级/三级派发，语义同上未闭合。
pub mod alignment_closures {
    use taffy::style::{AlignContent, AlignItems, FlexDirection, Style};

    pub fn toolbar_env_0(style: &mut Style) {
        style.justify_content = Some(AlignContent::FlexStart);
    }
    pub fn toolbar_env_2(style: &mut Style) {
        style.flex_direction = FlexDirection::Column;
    }
    pub fn toolbar_env_10(style: &mut Style) {
        style.flex_direction = FlexDirection::Row;
    }
    pub fn toolbar_env_11(style: &mut Style) {
        style.align_items = Some(AlignItems::FlexEnd);
    }
    pub fn toolbar_env_12(style: &mut Style) {
        style.justify_content = Some(AlignContent::Center);
    }
    pub fn toolbar_env_13(style: &mut Style) {
        style.align_items = Some(AlignItems::FlexStart);
    }
    pub fn undo_redo_env_0(style: &mut Style) {
        style.justify_content = Some(AlignContent::FlexEnd);
    }
    pub fn undo_redo_env_1(style: &mut Style) {
        style.align_items = Some(AlignItems::Center);
    }
}

/// O 0x142344560 `toolbar_alignment` 的 taffy setter 分派序列（跳表
/// 0x14304e6d8 20 项，条目已离线解码）。
///
/// 逐 case（fresh 反编译 case 0..3 + 5/7；`nodes` 为按序遍历的 taffy 节点）：
/// - 0：env12, env11, env10, env11
/// - 1：env0, undo1, env2, env13
/// - 2：undo0, undo1, env2, env11
/// - 3：env12, env13, env10, env13
/// - 5/7：env13（共享段 0x1423447d0）
/// - 9/10/0xb：`Taffy::style`→clone→`set_style`（反编译无字段 inline 写面 ⇒ 空 setter）
/// - 4/6：跳表指向共享二跳 0x14234485d（未恢复）；**未实现**
/// - 8/0xc..0x13：非 taffy setter 路径（写资源/缓动/算术），不在本函数范围
///
/// **判等**：setter 序列与跳表目标逐项；**分歧**：case 4/6 未实现。
pub fn apply_alignment_setters(alignment: u8, nodes: &[Node], taffy: &mut Taffy) {
    use alignment_closures::*;
    let apply = |taffy: &mut Taffy, node: Node, f: &dyn Fn(&mut taffy::style::Style)| {
        let mut style = taffy.style(node).unwrap().clone();
        f(&mut style);
        taffy.set_style(node, style).unwrap();
    };
    match alignment {
        0 => {
            if nodes.len() >= 4 {
                apply(taffy, nodes[0], &toolbar_env_12);
                apply(taffy, nodes[1], &toolbar_env_11);
                apply(taffy, nodes[2], &toolbar_env_10);
                apply(taffy, nodes[3], &toolbar_env_11);
            }
        }
        1 => {
            if nodes.len() >= 4 {
                apply(taffy, nodes[0], &toolbar_env_0);
                apply(taffy, nodes[1], &undo_redo_env_1);
                apply(taffy, nodes[2], &toolbar_env_2);
                apply(taffy, nodes[3], &toolbar_env_13);
            }
        }
        2 => {
            if nodes.len() >= 4 {
                apply(taffy, nodes[0], &undo_redo_env_0);
                apply(taffy, nodes[1], &undo_redo_env_1);
                apply(taffy, nodes[2], &toolbar_env_2);
                apply(taffy, nodes[3], &toolbar_env_11);
            }
        }
        3 => {
            if nodes.len() >= 4 {
                apply(taffy, nodes[0], &toolbar_env_12);
                apply(taffy, nodes[1], &toolbar_env_13);
                apply(taffy, nodes[2], &toolbar_env_10);
                apply(taffy, nodes[3], &toolbar_env_13);
            }
        }
        5 | 7 => {
            if let Some(node) = nodes.first() {
                apply(taffy, *node, &toolbar_env_13);
            }
        }
        // 9/10/0xb：直设空 setter（clone+set_style，字段不变）。
        9 | 10 | 11 => {
            for &node in nodes {
                apply(taffy, node, &|_| {});
            }
        }
        // 4/6 二跳未恢复；8/0xc..0x13 非 taffy 路径。未实现——不伪造。
        _ => {}
    }
}

/// ToolbarItem 最小读面（O toolbar_alignment 遍历：stride 0x28，`+0x24` 对齐位；
/// 字段真名/完整布局 U）。wave3 fresh 复核：`+0x18`=尺寸 x、`+0x1c`=尺寸 y、
/// `+0x20`=附加尺寸、`+0x24`=对齐位（`!= 4` 时按 alignment 掩码重写）。
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct ToolbarItem {
    /// +0..+0x18 前区（taffy 节点/元数据，U）。
    pub _head: [u8; 0x18],
    /// +0x18 尺寸 x（I）。
    pub size_x: f32,
    /// +0x1c 尺寸 y（I）。
    pub size_y: f32,
    /// +0x20 附加尺寸（I）。
    pub _extra: f32,
    /// +0x24 对齐位（`!= 4` 时按 alignment 掩码重写）。
    pub align: u8,
    /// typed gap（+0x25..+0x28）。
    pub _gap: [u8; 3],
}

const _TOOLBAR_ITEM_SIZE: () = assert!(std::mem::size_of::<ToolbarItem>() == 0x28);

/// `taffy_mut_style<T>` 通用模式（O 0x142375b40/0x142375ce0/0x142376050/0x142376300
/// 四单态 fresh 反编译逐语句）：`let mut s = taffy.style(node).unwrap(); f(&mut s);
/// taffy.set_style(node, s);`。
///
/// **判等（骨架）**：`Taffy::style`→`impl$16::clone`→closure→`Taffy::set_style` 逐语句。
/// **分歧（未实现）**：closure `f`（alignment closure_env_0..13）的 setter 目标
/// （taffy Style 字段）U——需定点反编译各 closure 调用体。
pub fn taffy_mut_style<T: FnOnce(&mut taffy::style::Style)>(
    taffy: &mut Taffy,
    node: Node,
    f: T,
) {
    let mut style = taffy.style(node).unwrap().clone();
    f(&mut style);
    // O：`Taffy::set_style` 后按 Result 判别子 4（Ok）返，否则 unwrap_failed。
    taffy.set_style(node, style).unwrap();
}

/// O 0x142344560 `toolbar::toolbar_systems::toolbar_alignment`。
///
/// **已重建（逐语句，反汇编头 0x142344560..0x142344630 + wave3 fresh）**：
/// - 遍历工具栏项（stride 0x28）：项 `+0x24` 对齐位 `!= 4` 时按
///   `0x1020300 >> (alignment&3 << 3)` 的字节重写。
/// - 按 `alignment`（0..19）经跳表 0x14304e6d8 分派。
///
/// **分派结构（跳表 0x14304e6d8，20 项；wave3 fresh 复核）**：
/// - case 0：`taffy_mut_style<closure_env_12/11/10/11>`；
/// - case 1：`<closure_env_0/undo_redo::closure_env_1/closure_env_2/closure_env_13>`；
/// - case 2：`<undo_redo::closure_env_0/closure_env_1/closure_env_2/closure_env_11>`；
/// - case 3：`<closure_env_12/13/10/13>`；
/// - case 5/7：`<closure_env_13>`（经 0x1423447d0 共享段）；case 9/10/0xb：
///   `taffy::style+set_style` 直设（经 0x142375b40 共享段）；
/// - case 0xc/0xe：`EasingFunction::apply` + 写偏移；case 0xd：
///   `EscMenuState::is_closed` + `Aabb2` + `DigitalActionState::pressed` +
///   `EasingFunction::apply` + 写进度；case 0xf/0x10/0x11/0x12/0x13：算术/跳表转发。
///
/// **分歧（未实现）**：各 case 的 `taffy_mut_style` closure_env_0..13 **setter 目标
/// 语义 U**（taffy Style 字段）；alignment 枚举（0..19）→ 锚点名 U；`r13+0xf0`
/// 资源（对齐设置）类型 U；case 0xd 的 Aabb2 中心/尺寸源 U。本实现落前置清位循环，
/// 分派结构登记，setter 目标不伪造。**签名分歧登记**：Rust 为 helper
/// `(alignment, items, taffy)`，非二进制 SystemParam（Res<GameConfig>/Res<ToolbarLayout>/
/// Res<PhotomodeToolbarLayout>/ResMut<TaffyLayout>）——分派 setter 目标落库后需改回
/// SystemParam 签名（reviewer-wave3 FIX-A）。
pub fn toolbar_alignment(
    alignment: u8,
    items: &mut [ToolbarItem],
    _taffy: &mut Taffy,
) {
    // 前置：清非默认项对齐位（O 0x142344600..0x14234462a）。
    // FIX-A（reviewer-wave3-toolbar-decorator）：二进制显式 `& 3`——
    // `uVar25 = 0x1020300 >> ((*(byte*)(GameConfig+0x1e) & 3) << 3)`；本实现补显式
    // `& 3`（release 与 `(a<<3)&31` 等价，debug 下防 `u32>>32` panic）。
    let mask = 0x1020300u32 >> ((alignment & 3) << 3);
    for item in items.iter_mut() {
        if item.align != 4 {
            item.align = mask as u8;
        }
    }
    // alignment 分派（O 跳表 0x14304e6d8，20 项）：case 0..19 各调
    // taffy_mut_style 设样式。**未实现**（setter 目标语义 U，见模块注释）。
    // 本函数不重建分派 case 体；完整实现需 closure_env_0..13 的 taffy 字段语义。
}

/// O 0x142343a10 `toolbar::toolbar_systems::toolbar_auto_hide`。
///
/// **已重建（常量/前段）**：多 SystemParam 系统（栈参密集 0x28..0x48）。常量
/// `110.0`(0x14304e2fc)、`-1.0`(0x14295a0e4)、`10.0`(0x1429345a8)、`0.3`
/// (0x1429258d8)、`-0.5`(0x14295a0e8)、`0.5`(0x142925870)、`{80,40}`(0x14304e300)/
/// `{40,80}`(0x14304e310)、`200.0`(0x142a13848)。前段把多 SystemParam 资源
/// （窗口尺寸、工具栏尺寸、焦点项等）压栈并逐项写 `+0x1c` 尺寸。
///
/// **wave3 fresh 复核（公式）**：`fVar55 = ((count-1)·10 + count·110)·fVar56`；
/// `fVar49 = fVar56·110·0.3`；按 `byte+0x1e`（对齐枚举）switch 设 `{80,40}`/
/// `{40,80}` 边距；`fVar55` 经 `EscMenuState::is_closed`/`Aabb2::contains`/
/// `DigitalActionState::pressed` 三路增量（`±dt/0.3`）后 `clamp01` 写 `+0x2c`；
/// 经 `EasingFunction::apply` 缩放 + `movmskpd` 项查找写 `{dx,dy}`。
///
/// **未实现（重建阻塞）**：Ghidra "Control flow encountered bad instruction data"
/// 截断区（switch 尾 case）未取证；系统参数接线（taffy setter 目标、工具栏尺寸
/// 资源 `+0xf1`/`+0x1e` 类型）U。不落近似体。
pub fn toolbar_auto_hide() {
    // 未实现（分歧见 RESTORE.md §toolbar_auto_hide）。
}

#[cfg(test)]
mod tests {
    use super::*;
    use taffy::style::{AlignContent, AlignItems, FlexDirection};

    /// 8 个已取证 setter 的字段写面（O：taffy_mut_style 单态 inline 写）。
    #[test]
    fn alignment_closures_write_expected_style_fields() {
        use alignment_closures::*;
        let mut s = taffy::style::Style::default();
        toolbar_env_0(&mut s);
        assert_eq!(s.justify_content, Some(AlignContent::FlexStart));
        let mut s = taffy::style::Style::default();
        toolbar_env_2(&mut s);
        assert_eq!(s.flex_direction, FlexDirection::Column);
        let mut s = taffy::style::Style::default();
        toolbar_env_10(&mut s);
        assert_eq!(s.flex_direction, FlexDirection::Row);
        let mut s = taffy::style::Style::default();
        toolbar_env_11(&mut s);
        assert_eq!(s.align_items, Some(AlignItems::FlexEnd));
        let mut s = taffy::style::Style::default();
        toolbar_env_12(&mut s);
        assert_eq!(s.justify_content, Some(AlignContent::Center));
        let mut s = taffy::style::Style::default();
        toolbar_env_13(&mut s);
        assert_eq!(s.align_items, Some(AlignItems::FlexStart));
        let mut s = taffy::style::Style::default();
        undo_redo_env_0(&mut s);
        assert_eq!(s.justify_content, Some(AlignContent::FlexEnd));
        let mut s = taffy::style::Style::default();
        undo_redo_env_1(&mut s);
        assert_eq!(s.align_items, Some(AlignItems::Center));
    }

    /// alignment case 0 在 4 个节点上的 setter 序列（O 跳表 case 0）。
    #[test]
    fn alignment_case_zero_applies_four_setters() {
        let mut taffy = Taffy::new();
        let n0 = taffy.new_leaf(taffy::style::Style::default()).unwrap();
        let n1 = taffy.new_leaf(taffy::style::Style::default()).unwrap();
        let n2 = taffy.new_leaf(taffy::style::Style::default()).unwrap();
        let n3 = taffy.new_leaf(taffy::style::Style::default()).unwrap();
        apply_alignment_setters(0, &[n0, n1, n2, n3], &mut taffy);
        assert_eq!(
            taffy.style(n0).unwrap().justify_content,
            Some(AlignContent::Center)
        );
        assert_eq!(
            taffy.style(n1).unwrap().align_items,
            Some(AlignItems::FlexEnd)
        );
        assert_eq!(
            taffy.style(n2).unwrap().flex_direction,
            FlexDirection::Row
        );
        assert_eq!(
            taffy.style(n3).unwrap().align_items,
            Some(AlignItems::FlexEnd)
        );
    }
}
