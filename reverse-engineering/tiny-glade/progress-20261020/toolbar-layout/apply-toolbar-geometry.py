from pathlib import Path
p=Path('D:/game/tiny-glade/crates/systems/toolbar/src/toolbar_systems.rs')
s=p.read_text(encoding='utf-8'); ev=Path('D:/game/reverse-engineering/tiny-glade/progress-20261020/toolbar-layout');(ev/'toolbar_systems-before.rs').write_text(s,encoding='utf-8')
start=s.index('/// Activation-zone rectangle used by the auto-hide cursor test.')
end=s.index('/// `toolbar_auto_hide` 的可见进度更新',start)
s=s[:start]+'''/// Original activation-zone center and unpadded dimensions (0x142343f59..0x142343fd8).
/// Alignment tags are bottom, left, right, top. Both vertical branches use
/// the thickness for their distance from the edge, not the toolbar length.
pub fn activation_rect(alignment: u8, window: Vec2, thin: f32, total: f32) -> (Vec2, Vec2) {
    match alignment {
        0 => (
            Vec2::new(window.x * 0.5, window.y + thin * -0.5),
            Vec2::new(total, thin),
        ),
        1 => (
            Vec2::new(thin * 0.5, window.y * 0.5),
            Vec2::new(thin, total),
        ),
        2 => (
            Vec2::new(window.x + thin * -0.5, window.y * 0.5),
            Vec2::new(thin, total),
        ),
        3 => (
            Vec2::new(window.x * 0.5, thin * 0.5),
            Vec2::new(total, thin),
        ),
        _ => unreachable!("invalid toolbar alignment"),
    }
}

/// Full original geometry (0x142343f2f..0x142343ff9), shared by the real
/// cursor hit test and DebugToolbarActivationZone drawable producer.
/// Preserve the original f32 evaluation order, including count == 0.
pub fn toolbar_activation_geometry(
    alignment: u8,
    window: Vec2,
    scale: f32,
    count: i32,
) -> (Vec2, Vec2) {
    let count = count as f32;
    let button = scale * 110.0;
    let total = (count + -1.0) * 10.0 * scale + button * count;
    let thin = button * 0.3;
    let (center, dimensions) = activation_rect(alignment, window, thin, total);
    let margin = if alignment == 1 || alignment == 2 {
        Vec2::new(scale * 80.0, scale * 40.0)
    } else {
        Vec2::new(scale * 40.0, scale * 80.0)
    };
    (center, dimensions + (margin + margin))
}

'''+s[end:]
start=s.index('pub fn auto_hide_sound_kind(');end=s.index('\n/// I 0x142343a10',start)
s=s[:start]+'''pub fn auto_hide_sound_kind(old: f32, new: f32) -> Option<u32> {
    if old == 0.0 && new > 0.0 {
        Some(0x10)
    } else if old == 1.0 && new < 1.0 {
        Some(0x11)
    } else {
        None
    }
}
'''+s[end:]
s=s.replace('/// I 0x142343a10 `toolbar::toolbar_systems::toolbar_auto_hide`.','/// O 0x142343a10 `toolbar::toolbar_systems::toolbar_auto_hide`.')
s=s.replace('/// See the module docs for the recovered/output surface and the explicit U\n/// items (activation-zone `Ui2dExtent` geometry, action slot, sound payload).','/// Geometry is independently checked against the original SSE/jump-table range;\n/// the first digital action is click. Sound variants 0x10/0x11 have no payload.')
s=s.replace('    _zones: Query<(&mut country_core::components::Ui2dExtent, &DebugToolbarActivationZone)>,','    mut zones: Query<(&mut country_core::components::Ui2dExtent, &DebugToolbarActivationZone)>,')
s=s.replace('        let mut count = 0usize;','        let mut count = 0i32;').replace('                count += 1;','                count = count.wrapping_add(1);')
a=s.index('        let count_f = count as f32;');b=s.index('        let inside_or_keep =',a)
s=s[:a]+'''        let (center, dims) = toolbar_activation_geometry(
            config.ui.toolbar_alignment as u8,
            window.as_vec2(),
            scale,
            count,
        );
        // Original 0x142344103..0x14234419b writes every matching debug extent
        // in either Bevy table or sparse-set storage before the hit test.
        for (mut extent, _) in &mut zones {
            extent.pos = center;
            extent.size = dims;
        }
'''+s[b:]
s=s.replace('        old + dt / (-0.3)\n','        old + dt / (-0.3)\n')
s=s.replace('// Toolbar hotkey (first digital slot) held → show.','// Cursor inside, menu closed and click not held → show.')
s=s.replace('Vec2::new(500.0, 5.0)','Vec2::new(500.0, 595.0)').replace('Vec2::new(900.0, 300.0)','Vec2::new(5.0, 300.0)').replace('Vec2::new(100.0, 300.0)','Vec2::new(995.0, 300.0)').replace('Vec2::new(500.0, 595.0));\n        assert_eq!(d3','Vec2::new(500.0, 5.0));\n        assert_eq!(d3')
old='''//!  **U（未落）**：`DebugToolbarActivationZone` 的 Ui2dExtent 尺寸/位置公式
//!  （热块 0x142343f2f..0x1423440xx 的 4 个 alignment 分支使用寄存器打包，
//!  Ghidra 未给出可复核的逐轴语义）；`ActionSetStates` 具体动作槽（native 读
//!  首 u32）与 PlayUiSound `id` 载荷；激活区 AABB 的窗口轴序（本实现按
//!  undo_redo_auto_hide 0x1423449f0 的同构公式构造，I）。'''
new='''//!  2026-10-20 原版 RNE 指令差分补闭：四向中心、缩放后双轴边距、真实 debug
//!  extent 写入，以及第一数字动作 click。0x10/0x11 是无载荷声音变体，native
//!  记录 +0x18 是事件队列自增 EventId，不是声音业务载荷；安全表示的未使用载荷清零。
//!  证据：progress-20261020/toolbar-layout。'''
s=s.replace(old,new)
p.write_text(s,encoding='utf-8')
