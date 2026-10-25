//! toolbar 布局：`ToolbarItemKind` 与 `add_toolbar_item`（O 0x141e4dc40）。
//!
//! wave2b fresh 证据：`ghidra-0x141e4dc40-*.c.txt`（完整）+ `add_toolbar_item-*kind*.disasm.txt`
//! ×6 + kind 跳表 0x142f8f774 解码（re-analysis evidence）。

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
/// 3. **spawn 8 组件取值（O，disasm 0x141e4e433..0x141e4e4e6；见
///    evidence/disasm-1.txt）**：
///    - `TaffyNode` = 本次 `new_leaf` 的节点（索引@载荷+0x40、代@+0x44）；
///    - `UiScaleWithUserSetting` = `item.size * (82.5,82.5)` + `(1.0,0.011)`
///      （`MULPS [0x142f8f600]` / `UNPCKLPD [0x142f8f610]`，载荷 +0x48..+0x58）；
///    - `UiInteractionState`/`UiMouseInteractable`/`Ui2dExtent`/`UiDrawOrder` = 全零
///      （载荷 +0x18/+0x30..+0x40/+0x58/+0x60；`Ui2dExtent` 另有 PDB inline
///      `components::impl$45::default`，O）；
///    - `UiAnimationState` = `{0.0, 0.0, 0, state: 2}`（载荷 +0x20..+0x30；
///      状态字节 **2**，O）；
///    - `Ui2dImage` = `Ui2dImage::new(label_clone)`（PDB inline，components/mod.rs:229，
///      O；core 现类型为 `{_gap: u64}` typed-gap，无法承载 `String`——本实现保留
///      label 克隆并登记，组件暂以 `default()` 落库）。
/// 4. kind 跳表（8 项）：kind0→0x141e4e5f3（压 **0x60B** 记录
///    `{fn@0x140fdf9a0,0,0,0,0,0,rbx,r13}`，调 vtable[+0x18]）；kind7→0x141e4e7e0
///    （压 **0x20B** 记录 `{fn@0x140fdf500,r13,r12,0}`）；kind2→0x141e4e817（读 `[item+1]`
///    子判别，三张 f32 朝向表，调 0x140c904a0）；kind1/3/6→0x141e4e9fe/0x141e4e95b/
///    0x141e4e911。
///
/// **判等**（控制流 + 8 组件 spawn 取值/顺序逐条对齐）；**分歧（未实现）**：
/// - kind 处理器的命令记录载荷（0x60B/0x20B、fn@0x140fdf9a0/0x140fdf500、
///   [rax+0x18] vtable、0x140c904a0 朝向）类型 U——本实现以 `kind_handler` 枚举占位，
///   不伪造记录编码。
/// - `Ui2dImage::new(label)` 载荷面 typed-gap（core 类型字段未落）。
/// - **签名/节点接线为重建（I）**：B 函数 6 参（rcx、edx、r8d、r9、栈×2），
///   `0x140fe7600`/`0x140fe7790`（new_leaf/add_child 包装）返回两个 NodeId
///   （`[rbp+0x1b4]/[rbp+0x1b8]`），递归沿用该结果——本实现简化为
///   `(taffy, parent, item, commands)` 4 参、容器节点为父。
pub fn add_toolbar_item(
    taffy: &mut taffy::node::Taffy,
    parent: taffy::node::Node,
    item: &mut ToolbarItemKind,
    commands: &mut bevy_ecs::system::Commands,
) {
    // kind 4/5：容器——new_leaf+add_child、递归子项、逐子 drop、释放 Vec 缓冲。
    if item.is_container() {
        let _leaf = crate::resouces_and_components::TaffyNode::spawn_leaf(taffy, parent);
        let base = item.children_ptr;
        let mut it = base;
        let count = item.children_len;
        let mut remaining = count;
        // 循环界：`byte[child+0x38] != 2`（B：`*(char*)(lVar5 + 0x38) != 2`）。
        while remaining != 0 && unsafe { (*it).show_only_fpv } != 2 {
            unsafe { add_toolbar_item(taffy, _leaf.node, &mut *it, commands) };
            it = unsafe { it.add(1) };
            remaining -= 1;
        }
        // decomp：drop 次数 = `(end - lVar7) >> 6`，其中 lVar7 是循环停止后的
        // 下一项（`lVar5+0x40`）；即只 drop **停止位置之后**未消费的尾项
        // （前缀已被递归消费；len==0 结束时 lVar7==end，不 drop）。
        if remaining != 0 {
            let mut drop_it = unsafe { it.add(1) };
            let end = unsafe { base.add(count) };
            while (drop_it as usize) < (end as usize) {
                unsafe { std::ptr::drop_in_place(drop_it) };
                drop_it = unsafe { drop_it.add(1) };
            }
        }
        // **FIX-1（wave4）**：B 在容器分支末尾 `if (cap != 0) __rust_dealloc(...)`：
        // 0x141e4e794 `SHL RDX(cap),0x6; MOV R8D,0x8; MOV RCX,ptr; CALL __rust_dealloc`
        // ⇒ 释放 `cap * 0x40`、align 8 的子项缓冲（原实现遗漏）。
        if item.children_cap != 0 {
            // SAFETY: 容器分支的缓冲由 B Vec 分配（cap 个 0x40 B 元素，align 8）；
            // 指针与容量来自 ToolbarItemKind 的原始布局读面。
            unsafe {
                let layout =
                    std::alloc::Layout::from_size_align_unchecked(item.children_cap * 0x40, 8);
                std::alloc::dealloc(item.children_ptr as *mut u8, layout);
            }
        }
        return;
    }

    // 其余 kind：new_leaf + add_child → spawn 8 组件 → 插入 UiShowIfParentIsInFocus →
    // 条件插入 ShowOnlyInFpvAndFree → kind 处理器。
    let leaf = crate::resouces_and_components::TaffyNode::spawn_leaf(taffy, parent);
    // O：label 以 `Box<str>` 读面（ptr@+0x20、len@+0x28）克隆；B 把它移入
    // `Ui2dImage::new(label)`。core 的 Ui2dImage 仍是 typed-gap（{_gap: u64}），
    // 本实现保留克隆并登记，组件暂以 default() 落库。
    let _label = item.label().to_owned();
    // O：+0x30 两 f32（ToolbarItemKind::size）。
    let size = item.size();
    // **FIX（2026-10-17 unlock-mei-toolbar）**：fresh `register_component` 证明
    // `UiScaleWithUserSetting` 为 ZST（size 0），wave2b 的 16B `size*82.5` 归属
    // 改判：该 16B 观测值对应 16B 的 `Ui2dExtent`（core typed-gap 未承载，登记）。
    // 这里保留 `size` 读数并显式丢弃，避免伪造字段。
    let _ = size;
    let mut entity = commands.spawn((
        // O：TaffyNode = 本次 new_leaf 的节点（不是 default）。
        leaf,
        // O：UiScaleWithUserSetting 为 ZST（fresh impl$3 size 0x0）。
        crate::resouces_and_components::UiScaleWithUserSetting,
        crate::resouces_and_components::UiInteractionState::default(),
        // O：状态字节初值 2（载荷 +0x2c）。
        crate::resouces_and_components::UiAnimationState::for_spawn(),
        crate::resouces_and_components::UiMouseInteractable::default(),
        country_core::components::Ui2dExtent::default(),
        country_core::components::Ui2dImage::default(),
        country_core::components::UiDrawOrder::default(),
    ));
    entity.insert(crate::resouces_and_components::UiShowIfParentIsInFocus::default());
    if item.show_only_fpv != 0 {
        entity.insert(crate::resouces_and_components::ShowOnlyInFpvAndFree::default());
    }
    // kind 处理器（未实现：命令记录载荷类型 U，见模块注释与 RESTORE.md）。
    crate::resouces_and_components::toolbar_layout::kind_handler(item.tag, item.sub);
}

/// kind 处理器占位（O kind 跳表 0x142f8f774 各 case）。
///
/// **未实现（重建阻塞）**：kind0/7 的 0x60B/0x20B 命令记录（fn@0x140fdf9a0/
/// 0x140fdf500）、[rax+0x18] vtable 回调、kind2 三张 f32 朝向表（0x142f8f6d8/
/// 0x142f8f70c/0x142f8f740）+ 0x140c904a0 载荷类型 U；禁止近似体。
pub fn kind_handler(_tag: u8, _sub: u8) {
    // 未实现（分歧见 RESTORE.md §add_toolbar_item）。
}
