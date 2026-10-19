//! toolbar 布局：`ToolbarItemKind`/`add_toolbar_item`（O 0x141e4dc40）与
//! `ToolbarLayout`/`PhotomodeToolbarLayout`（O 0x141e4ccb0 / 0x141e4d1e0）。
//!
//! wave2b fresh 证据：`ghidra-0x141e4dc40-*.c.txt`（完整）+ `add_toolbar_item-*kind*.disasm.txt`
//! ×6 + kind 跳表 0x142f8f774 解码（re-analysis evidence）。
//!
//! 第 11 轮 TB-D（2026-10-19）fresh 补充：`setup_screen_and_toolbar_nodes`
//! 0x141e4ccb0 与 `add_contextual_menu` 0x141e4d1e0 已 prepare + 反编译
//! （`evidence/round11/tbd-jobs/decomp/141e4ccb0.c`、`141e4d1e0.c`）：
//! - setup 只登记 **1** 条 anchored `{order 0, offset 0, node screen, side 4=Window}`，
//!   随后 `new_leaf` toolbar 并 `add_child(screen, toolbar)`；`root()` 由调用方完成。
//! - `add_contextual_menu` 在 `ToolbarLayout+0x08/+0x10` 的 stride 0x20 定义表中
//!   按 `+0x18` tag 查找（未命中 `unwrap_failed`）；行布局
//!   `relative_to@+0x00`、`menu node@+0x10`、`tag@+0x18`。新菜单登记 anchored
//!   `{order 1, offset 0, relative_to=行+0, gap 10.0, side 0=Above}`，递归添加项
//!   到 `item+0x38 == 2` 哨兵，最后把菜单 node 写回定义表 +0x10。

/// 工具栏项（O：`ToolbarItemKind`，判别子@+0，字节；0..7）。
///
/// **B 布局（stride 0x40 = 64B，add_toolbar_item 读面，fresh 反汇编）**：
/// - +0：判别子（`*in_R9`）。
/// - +8/+0x10/+0x18：**kind 4/5（容器）** 的子项容器（B Vec 内存序
///   `{cap@+0x08, ptr@+0x10, len@+0x18}`，O：迭代基址 = [item+0x10]、
///   长度 = [item+0x18]、dealloc `size = [item+0x08] << 6, align = 8`；
///   子项 stride 0x40；循环界 `byte[item+0x38] != 2`）。
/// - +0x20/+0x28：叶子 label（`Box<str>` 读面：ptr@+0x20、len@+0x28；O：
///   `memcpy(dst, [item+0x20], [item+0x28])`）。
/// - +0x30/+0x34：两个 `f32`（`glam::Vec2` 读面；O：spawn 前
///   `MOVSD XMM6,[item+0x30]` 后 `MULPS [0x142f8f600]`）。
/// - +0x38：叶子 ShowOnlyInFpvAndFree 标志字节（非容器插入条件）。
/// - +1：kind 2 的子判别（朝向；三张 f32 表 0x142f8f6d8/0x142f8f70c/0x142f8f740）。
///
/// 变体名（判别子 0..7 → 语义）U；本文件以标签值 + 字段读面承载，不做名称猜测。
#[derive(Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct ToolbarItemKind {
    /// +0 判别子（0..7）。
    pub tag: u8,
    /// +1 kind 2 子判别（朝向；I）。
    pub sub: u8,
    /// +2..+7 typed gap。
    pub _gap0: [u8; 6],
    /// +8 容器子项容量（dealloc `size = cap << 6` 来源；B Vec 内存序）。
    pub children_cap: usize,
    /// +0x10 容器子项指针（迭代基址 / dealloc ptr；B Vec 内存序）。
    pub children_ptr: *mut ToolbarItemKind,
    /// +0x18 容器子项长度（循环界来源）。
    pub children_len: usize,
    /// +0x20 叶子 label ptr（`Box<str>`；O）。
    pub label_ptr: *mut u8,
    /// +0x28 叶子 label len（`Box<str>`；O）。
    pub label_len: usize,
    /// +0x30 两 f32（Vec2 读面；O）。
    pub size: [f32; 2],
    /// +0x38 叶子 ShowOnlyInFpvAndFree 标志。
    pub show_only_fpv: u8,
    /// +0x39..+0x40 typed gap。
    pub _gap1: [u8; 7],
}

const _TOOLBAR_ITEM_SIZE: () = assert!(std::mem::size_of::<ToolbarItemKind>() == 0x40);

impl ToolbarItemKind {
    /// kind 判别子是否为容器（4/5）。
    pub fn is_container(&self) -> bool {
        self.tag == 4 || self.tag == 5
    }

    /// 叶子 label（+0x20/+0x28 读面，`Box<str>`；O）。
    pub fn label(&self) -> &str {
        if self.label_len == 0 || self.label_ptr.is_null() {
            return "";
        }
        // SAFETY: 调用侧保证 label_ptr/len 有效（B 布局 Box<str>）。
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.label_ptr, self.label_len))
        }
    }

    /// 叶子尺寸（+0x30 两个 f32；O）。
    pub fn size(&self) -> glam::Vec2 {
        glam::Vec2::from_array(self.size)
    }
}

/// O 0x141e4dc40 `add_toolbar_item`：把一项工具栏树登记进 taffy 与 Bevy 命令队列。
///
/// **已重建控制流（逐语句，fresh decompile 0x141e4dc40 + disasm + kind 跳表
/// 0x142f8f774）**：
/// 1. `*item` 判别子：kind 4/5（容器）→ `Taffy::new_leaf`+`add_child`（各 `.unwrap()`，
///    Result 判别子 4 为 Ok）→ 遍历子项（stride 0x40，`byte[child+0x38] != 2` 为界）
///    递归 `add_toolbar_item` → 跳过已消费前缀，drop 停止位置之后的尾项
///    （decomp：drop 次数 `(end - next)/0x40`）→ 释放容器缓冲。
/// 2. 其余 kind：`Taffy::new_leaf`+`add_child` → 克隆 label（`Box<str>` ptr+len）→
///    `Commands::spawn<TaffyNode,UiScaleWithUserSetting,UiInteractionState,UiAnimationState,
///    UiMouseInteractable,Ui2dExtent,Ui2dImage,UiDrawOrder>` → 无条件插入
///    `UiShowIfParentIsInFocus` → 条件（`item[0x38] != 0`）插入 `ShowOnlyInFpvAndFree` →
///    kind 跳表分派。
/// 3. **spawn 组件取值（O，disasm 0x141e4e433..0x141e4e4e6）**：
///    - `TaffyNode` = 本次 `new_leaf` 的节点（索引@载荷+0x40、代@+0x44）；
///    - `UiScaleWithUserSetting` = ZST（fresh `register_required_components` size 0，
///      wave2b 的 16B `size*82.5` 归属改判，见 `taffy_update` 证据）；
///    - `UiInteractionState`/`UiMouseInteractable`/`Ui2dExtent`/`UiDrawOrder` = 全零；
///    - `UiAnimationState` = `{0.0, 0.0, 0, state: 2}`（状态字节 **2**，O）；
///    - `Ui2dImage` = `Ui2dImage::new(label_clone)`（PDB inline，O）。
/// 4. kind 跳表（8 项）：kind0→0x141e4e5f3（压 **0x60B** 记录
///    `{fn@0x140fdf9a0,0,0,0,0,0,rbx,r13}`，调 vtable[+0x18]）；kind7→0x141e4e7e0
///    （压 **0x20B** 记录 `{fn@0x140fdf500,r13,r12,0}`）；kind2→0x141e4e817（读 `[item+1]`
///    子判别，三张 f32 朝向表，调 0x140c904a0）；kind1/3/6→0x141e4e9fe/0x141e4e95b/
///    0x141e4e911。
///
/// **判等**（控制流 + 8 组件 spawn 取值/顺序逐条对齐）；**分歧（未实现）**：
/// kind 处理器的命令记录载荷（0x60B/0x20B、fn@0x140fdf9a0/0x140fdf500、
/// [rax+0x18] vtable、0x140c904a0 朝向）类型 U——本实现以 `kind_handler` 枚举占位，
/// 不伪造记录编码。
pub fn add_toolbar_item(
    taffy: &mut taffy::node::Taffy,
    parent: taffy::node::Node,
    item: &mut ToolbarItemKind,
    commands: &mut bevy_ecs::system::Commands,
) {
    // kind 4/5：容器——new_leaf+add_child、递归子项、逐子 drop、释放 Vec 缓冲。
    if item.is_container() {
        let leaf = crate::resouces_and_components::TaffyNode::spawn_leaf(taffy, parent);
        let base = item.children_ptr;
        let mut it = base;
        let count = item.children_len;
        let mut remaining = count;
        // 循环界：`byte[child+0x38] != 2`（B：`*(char*)(lVar5 + 0x38) != 2`）。
        while remaining != 0 && unsafe { (*it).show_only_fpv } != 2 {
            unsafe { add_toolbar_item(taffy, leaf.node, &mut *it, commands) };
            it = unsafe { it.add(1) };
            remaining -= 1;
        }
        // decomp：drop 次数 = `(end - lVar7) >> 6`，只 drop 停止位置之后的尾项。
        if remaining != 0 {
            let mut drop_it = unsafe { it.add(1) };
            let end = unsafe { base.add(count) };
            while (drop_it as usize) < (end as usize) {
                unsafe { std::ptr::drop_in_place(drop_it) };
                drop_it = unsafe { drop_it.add(1) };
            }
        }
        // 0x141e4e794 `SHL RDX(cap),0x6; MOV R8D,0x8; ... CALL __rust_dealloc`。
        if item.children_cap != 0 {
            // SAFETY: 容器分支的缓冲由 B Vec 分配（cap 个 0x40 B 元素，align 8）。
            unsafe {
                let layout =
                    std::alloc::Layout::from_size_align_unchecked(item.children_cap * 0x40, 8);
                std::alloc::dealloc(item.children_ptr as *mut u8, layout);
            }
        }
        return;
    }

    let leaf = crate::resouces_and_components::TaffyNode::spawn_leaf(taffy, parent);
    let label = item.label().to_owned();
    // O：+0x30 两 f32（ToolbarItemKind::size）。UiScaleWithUserSetting 为 ZST，
    // 该读数在 native 中供后续尺寸路径使用；此处无消费者，显式保留读取。
    let _size = item.size();
    let mut entity = commands.spawn((
        leaf,
        crate::resouces_and_components::UiScaleWithUserSetting,
        crate::resouces_and_components::UiInteractionState::default(),
        // O：状态字节初值 2（载荷 +0x2c）。
        crate::resouces_and_components::UiAnimationState::for_spawn(),
        crate::resouces_and_components::UiMouseInteractable::default(),
        country_core::components::Ui2dExtent::default(),
        country_core::components::Ui2dImage::new(label),
        country_core::components::UiDrawOrder::default(),
    ));
    entity.insert(crate::resouces_and_components::UiShowIfParentIsInFocus::default());
    if item.show_only_fpv != 0 {
        entity.insert(crate::resouces_and_components::ShowOnlyInFpvAndFree::default());
    }
    kind_handler(item.tag, item.sub);
}

/// kind 处理器占位（O kind 跳表 0x142f8f774 各 case）。
///
/// **未实现（重建阻塞，U）**：kind0/7 的 0x60B/0x20B 命令记录
/// （fn@0x140fdf9a0/0x140fdf500）、[rax+0x18] vtable 回调、kind2 三张 f32
/// 朝向表（0x142f8f6d8/0x142f8f70c/0x142f8f740）+ 0x140c904a0 载荷类型 U。
pub fn kind_handler(_tag: u8, _sub: u8) {
    // 未实现（见 delivery §U）。
}

/// 一条 contextual menu 定义/实例记录。
///
/// Native stride 0x20（`add_contextual_menu` 0x141e4d1e0 / `toolbar_alignment`
/// 0x142344560）：`relative_to Node@+0x00`、`menu Node@+0x10`（0 = 尚未创建）、
/// `tag@+0x18`。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ContextualMenu {
    /// 定义表 +0x00：菜单锚定目标节点。
    pub relative_to: taffy::node::Node,
    /// 定义表 +0x10：菜单根节点（创建前为 `Node::default()`）。
    pub node: taffy::node::Node,
    /// 定义表 +0x18：菜单 tag。
    pub tag: u8,
}

/// 工具栏布局资源（native `ToolbarLayout`）。
///
/// **B 字段读面（O，0x141e4ccb0 / 0x141e4d1e0 / 0x142344560）**：
/// - `+0x08/+0x10`：contextual menu 定义表 `Vec`（ptr/len，stride 0x20）；
/// - `+0x18`：Node A（对齐批改目标，U 名称）；
/// - `+0x20`：Node B（对齐/间距批改目标，U 名称）；
/// - `+0x28`：`f32`（`toolbar_button_size_and_padding_update` 的 BackOut 输入，
///   U 名称）；
/// - `+0x2c`：`f32` 自动隐藏进度（`toolbar_auto_hide` clamp01 写面，O）。
#[derive(bevy_ecs::prelude::Resource, Default)]
pub struct ToolbarLayout {
    /// native +0x08/+0x10 定义表。
    pub menus: Vec<ContextualMenu>,
    /// native +0x18。
    pub node_a: taffy::node::Node,
    /// native +0x20。
    pub node_b: taffy::node::Node,
    /// native +0x28（U 语义；BackOut easing 输入）。
    pub scale_progress: f32,
    /// native +0x2c（自动隐藏进度，clamp01）。
    pub hide_progress: f32,
}

/// 拍照模式工具栏布局资源（native `PhotomodeToolbarLayout`）。
///
/// PDB 为独立类型（`toolbar_alignment` / `toolbar_button_size_and_padding_update`
/// 依次读取二者），字段与 [`ToolbarLayout`] 同构。
#[derive(bevy_ecs::prelude::Resource, Default)]
pub struct PhotomodeToolbarLayout {
    /// native +0x08/+0x10 定义表。
    pub menus: Vec<ContextualMenu>,
    /// native +0x18。
    pub node_a: taffy::node::Node,
    /// native +0x20。
    pub node_b: taffy::node::Node,
    /// native +0x28（U 语义）。
    pub scale_progress: f32,
    /// native +0x2c（自动隐藏进度）。
    pub hide_progress: f32,
}

/// Shared field view of the two context resources. Native `toolbar_alignment`
/// and `toolbar_button_size_and_padding_update` run the same body once per
/// resource (two-iteration loop in 0x142344560 / 0x141eaec50); this trait keeps
/// that single body without duplicating it.
pub trait ToolbarContext {
    fn node_a(&self) -> taffy::node::Node;
    fn node_b(&self) -> taffy::node::Node;
    fn contextual_menus(&self) -> &[ContextualMenu];
    fn scale_progress(&self) -> f32;
    fn hide_progress(&self) -> f32;
    fn set_hide_progress(&mut self, value: f32);
}

impl ToolbarContext for ToolbarLayout {
    fn node_a(&self) -> taffy::node::Node {
        self.node_a
    }
    fn node_b(&self) -> taffy::node::Node {
        self.node_b
    }
    fn contextual_menus(&self) -> &[ContextualMenu] {
        &self.menus
    }
    fn scale_progress(&self) -> f32 {
        self.scale_progress
    }
    fn hide_progress(&self) -> f32 {
        self.hide_progress
    }
    fn set_hide_progress(&mut self, value: f32) {
        self.hide_progress = value;
    }
}

impl ToolbarContext for PhotomodeToolbarLayout {
    fn node_a(&self) -> taffy::node::Node {
        self.node_a
    }
    fn node_b(&self) -> taffy::node::Node {
        self.node_b
    }
    fn contextual_menus(&self) -> &[ContextualMenu] {
        &self.menus
    }
    fn scale_progress(&self) -> f32 {
        self.scale_progress
    }
    fn hide_progress(&self) -> f32 {
        self.hide_progress
    }
    fn set_hide_progress(&mut self, value: f32) {
        self.hide_progress = value;
    }
}

impl ToolbarLayout {
    /// O 0x141e4ccb0 `ToolbarLayout::setup_screen_and_toolbar_nodes`.
    ///
    /// Native takes only the `TaffyLayout` owner (param_2) and returns two
    /// builders; Rust cannot return two builders borrowing the same `&mut`, so
    /// this port returns the two nodes. `root()`/further anchoring is the
    /// caller's (`toolbar_setup`) responsibility.
    pub fn setup_screen_and_toolbar_nodes(
        taffy: &mut crate::resouces_and_components::taffy_layout::TaffyLayout,
    ) -> (taffy::node::Node, taffy::node::Node) {
        use crate::resouces_and_components::taffy_layout::{AnchorSide, AnchoredNode};
        let screen = taffy
            .taffy
            .new_leaf(taffy::style::Style::default())
            .expect("native screen root new_leaf");
        // Native writes order=0, offset=(0,0), node=screen, side=4; the
        // Window branch never reads relative_to (filled with screen itself, I).
        taffy.anchored.push(AnchoredNode {
            order: 0,
            offset: glam::Vec2::ZERO,
            node: screen,
            relative_to: screen,
            gap: 0.,
            side: AnchorSide::Window,
        });
        let toolbar = taffy
            .taffy
            .new_leaf(taffy::style::Style::default())
            .expect("native toolbar new_leaf");
        taffy
            .taffy
            .add_child(screen, toolbar)
            .expect("native toolbar add_child");
        (screen, toolbar)
    }

    /// O 0x141e4d1e0 `ToolbarLayout::add_contextual_menu`.
    ///
    /// Looks up the definition row by `tag` (native unwrap on failure), creates
    /// the menu leaf, records the native anchor, recursively adds items until
    /// the sentinel, and writes the menu node back into the definition row.
    pub fn add_contextual_menu(
        &mut self,
        taffy: &mut crate::resouces_and_components::taffy_layout::TaffyLayout,
        tag: u8,
        items: &mut [ToolbarItemKind],
        commands: &mut bevy_ecs::system::Commands,
    ) -> taffy::node::Node {
        use crate::resouces_and_components::taffy_layout::{AnchorSide, AnchoredNode};
        let row = self
            .menus
            .iter_mut()
            .find(|row| row.tag == tag)
            .expect("contextual menu definition tag missing (native unwrap_failed)");
        let menu = taffy
            .taffy
            .new_leaf(taffy::style::Style::default())
            .expect("contextual menu new_leaf");
        taffy.anchored.push(AnchoredNode {
            order: 1,
            offset: glam::Vec2::ZERO,
            node: menu,
            relative_to: row.relative_to,
            // `MOV dword [entry+0x20],0x41200000` = 10.0.
            gap: 10.0,
            // `MOV byte [entry+0x24],0` = Above.
            side: AnchorSide::Above,
        });
        for item in items.iter_mut() {
            if item.show_only_fpv == 2 {
                break;
            }
            add_toolbar_item(&mut taffy.taffy, menu, item, commands);
        }
        row.node = menu;
        menu
    }

    /// O `contextual_menus` inline (5 instances): the registered menu slice
    /// consumed by alignment and the size/padding update.
    pub fn contextual_menus(&self) -> &[ContextualMenu] {
        &self.menus
    }

    /// O 0x141e4d880 `link_up_nav`: for each button write
    /// `UiNav{prev=ids[(i+n-1)%n], next=ids[(i+1)%n]}` into the real entity.
    /// The boolean selects the horizontal/vertical slot layout.
    pub fn link_up_cyclic_list_nav(
        &self,
        ids: &[(bevy_ecs::entity::Entity, u64)],
        horizontal: bool,
        commands: &mut bevy_ecs::system::Commands,
    ) {
        if ids.is_empty() {
            return;
        }
        for (i, (entity, _)) in ids.iter().enumerate() {
            let prev = ids[(i + ids.len() - 1) % ids.len()].1;
            let next = ids[(i + 1) % ids.len()].1;
            let nav = if horizontal {
                crate::resouces_and_components::UiNav::horizontal(prev, next)
            } else {
                crate::resouces_and_components::UiNav::vertical(prev, next)
            };
            commands.entity(*entity).insert(nav);
        }
    }
}

impl PhotomodeToolbarLayout {
    /// Same inline `contextual_menus` (native reads both layouts).
    pub fn contextual_menus(&self) -> &[ContextualMenu] {
        &self.menus
    }
}

#[cfg(test)]
mod layout_owner_tests {
    use super::*;
    use crate::resouces_and_components::taffy_layout::{AnchorSide, TaffyLayout};

    #[test]
    fn setup_registers_single_window_anchor_and_screen_child() {
        let mut taffy = TaffyLayout::default();
        let (screen, toolbar) = ToolbarLayout::setup_screen_and_toolbar_nodes(&mut taffy);
        assert_eq!(taffy.anchored.len(), 1);
        assert_eq!(taffy.anchored[0].order, 0);
        assert_eq!(taffy.anchored[0].node, screen);
        assert_eq!(taffy.anchored[0].side, AnchorSide::Window);
        assert_eq!(taffy.children(screen), vec![toolbar]);
    }

    #[test]
    fn contextual_menu_uses_definition_row_anchor_and_sentinel() {
        let mut taffy = TaffyLayout::default();
        let (screen, toolbar) = ToolbarLayout::setup_screen_and_toolbar_nodes(&mut taffy);
        let mut layout = ToolbarLayout {
            menus: vec![ContextualMenu {
                relative_to: toolbar,
                node: taffy::node::Node::default(),
                tag: 5,
            }],
            node_a: screen,
            node_b: toolbar,
            ..Default::default()
        };
        let mut world = bevy_ecs::world::World::new();
        let mut queue = bevy_ecs::world::CommandQueue::default();
        let mut commands = bevy_ecs::system::Commands::new(&mut queue, &world);
        let mut items = [
            ToolbarItemKind { tag: 0, ..Default::default() },
            ToolbarItemKind {
                tag: 7,
                show_only_fpv: 2,
                ..Default::default()
            },
            ToolbarItemKind { tag: 1, ..Default::default() },
        ];
        let menu = layout.add_contextual_menu(&mut taffy, 5, &mut items, &mut commands);
        queue.apply(&mut world);
        assert_eq!(layout.contextual_menus().len(), 1);
        assert_eq!(layout.contextual_menus()[0].tag, 5);
        assert_eq!(layout.contextual_menus()[0].node, menu);
        assert_eq!(layout.contextual_menus()[0].relative_to, toolbar);
        assert_eq!(taffy.anchored.len(), 2);
        assert_eq!(taffy.anchored[1].relative_to, toolbar);
        assert_eq!(taffy.anchored[1].gap, 10.0);
        assert_eq!(taffy.anchored[1].side, AnchorSide::Above);
        // Only the item before the sentinel was linked.
        assert_eq!(taffy.children(menu).len(), 1);
    }

    #[test]
    #[should_panic(expected = "contextual menu definition tag missing")]
    fn contextual_menu_without_definition_row_panics_like_unwrap_failed() {
        let mut taffy = TaffyLayout::default();
        let mut layout = ToolbarLayout::default();
        let mut world = bevy_ecs::world::World::new();
        let mut queue = bevy_ecs::world::CommandQueue::default();
        let mut commands = bevy_ecs::system::Commands::new(&mut queue, &world);
        let mut items: [ToolbarItemKind; 0] = [];
        layout.add_contextual_menu(&mut taffy, 1, &mut items, &mut commands);
    }
}
