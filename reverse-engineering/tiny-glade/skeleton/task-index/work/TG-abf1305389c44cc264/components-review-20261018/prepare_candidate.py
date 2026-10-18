from pathlib import Path

ROOT = Path(__file__).resolve().parent
BASE = ROOT / 'before'
OUT = ROOT / 'candidate'


def edit(path, changes):
    text = (BASE / path).read_text()
    for old, new in changes:
        assert old in text, (path, old[:100])
        text = text.replace(old, new, 1)
    dest = OUT / path
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_text(text)


path = 'crates/country-core/src/components/drawable.rs'
text = (BASE / path).read_text()
start = text.index('use bevy_ecs::bundle::Bundle;')
text = '''//! 可绘制实体的网格描述与显式组件组合。
//!
//! NaniBundle 用于实例化渲染，DrawableMeshBundle 用于普通网格绘制。
//! 使用 Bevy Bundle 派生完成插入，不另实现原作的库转发胶水。
//! 原生字段复制范围与本机 Rust 分配大小分别记录在布局测试中。
//!
//! 当前仍未在本文件恢复 cache_prev_object_transforms 系统和完整的材质组件族；
//! 详见 recovery/components-2026-10-18.md。
'''+text[start:]
text = text.replace('use bevy_ecs::component::Component;\n', '')
text = text.replace('#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]', '#[derive(Clone, Copy, Debug, PartialEq, Eq)]')
# Replace recovery history in API comments with the relevant usage/contract.
a = text.index('/// 原始类型路径是')
b = text.index('pub use crate::render::nani::systems::NaniInstanceDesc;', a)
text = text[:a] + '/// 共享实例描述组件的原模块入口，保持同一 ECS 类型身份。\n' + text[b:]
a = text.index('/// nani 实例实体 bundle')
b = text.index('#[derive(Bundle)]', a)
text = text[:a] + '''/// 实例渲染实体：网格描述、世界变换和实例数据类型标记。
///
/// B component_ids 注册序为 desc → transform → marker；字段物理顺序由 Rust
/// 布局决定。本机 transform@0、desc@0x30、marker@0x48，尾部对齐后大小 0x50。
/// 原作写入器的有效复制范围至 0x48；不能将其当作本机分配大小。
''' + text[b:]
a = text.index('/// 普通网格可绘制 bundle')
b = text.index('#[derive(Bundle)]', a)
text = text[:a] + '''/// 普通网格实体：网格句柄、光栅着色器句柄和世界变换。
///
/// B component_ids 注册序为 mesh → shader → transform；本机物理偏移见测试。
/// Bundle 载荷不含 bool，旧证据中的 bool 属于外层命令闭包。
''' + text[b:]
(OUT / path).write_text(text)

path = 'crates/country-core/src/render/nani/systems.rs'
text = (BASE / path).read_text()
a = text.index('/// 网格选择器（O：')
b = text.index('/// 实例描述组件', a)
text = text[:a] + '''/// 与 components::drawable 共用网格选择器，保持已有导入路径。
pub use crate::components::drawable::NaniInstanceMeshDesc as NaniMeshSelector;

''' + text[b:]
text = text.replace('#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]\npub struct NaniInstanceDesc', '#[derive(Component, Debug, PartialEq, Eq)]\npub struct NaniInstanceDesc', 1)
dest = OUT / path
dest.parent.mkdir(parents=True, exist_ok=True)
dest.write_text(text)

edit('crates/systems/tree-tool/src/process_tree_cmds.rs', [
    ('commands.entity(entity).insert(TreeComponent);', '''// 当前 InsertTreeCmd 尚无原版 is_preplaced 字段；这条手放入口写 false。
        commands.entity(entity).insert(TreeComponent { id: cmd.id, is_preplaced: false });'''),
    ('commands.entity(entity).insert(TreeAnimation { alpha, _gap: 0 });', 'commands.entity(entity).insert(TreeAnimation::new(alpha));'),
    ('''        // O：TreeMoveBend = 8B {x, z}（`[R12+0x18]`→+0x78、`[R12+0x20]`→+0x7c；
        // 共享组件以 `_gap` 承载打包位）。
        commands.entity(entity).insert(TreeMoveBend {
            _gap: (pos.x.to_bits() as u64) | ((pos.z.to_bits() as u64) << 32),
        });''', '''        // B：初始缓动位置等于树的世界 XZ 位置。
        commands.entity(entity).insert(TreeMoveBend {
            position: glam::Vec2::new(pos.x, pos.z),
        });'''),
    ('''        // TreeAnimation.alpha 字段语义 I（共享型 α@+0 与真实 +4 差异登记分歧 5）。
        if animation.alpha < DESPAWN_ALPHA_THRESHOLD {''', '''        // 读取生长进度 +4；+0 是时长，不能用于删除判断。
        if animation.progress < DESPAWN_ALPHA_THRESHOLD {'''),
])

edit('crates/systems/tree-tool/src/tree_move_bend.rs', [
    ('''        // TreeMoveBend 是 8 B 组件 {x f32 @+0, y f32 @+4}（原生迭代步长 8）——
        // core 型以 `_gap: u64` 承载，按位解包读写。
        let mut bx = f32::from_bits(bend._gap as u32);
        let mut by = f32::from_bits((bend._gap >> 32) as u32);''', '''        // 组件保存 XZ 平面中的缓动位置。
        let mut bx = bend.position.x;
        let mut by = bend.position.y;'''),
    ('bend._gap = (bx.to_bits() as u64) | ((by.to_bits() as u64) << 32);', 'bend.position = glam::Vec2::new(bx, by);'),
])

path = 'crates/systems/forest/src/tree_animation.rs'
text = (BASE / path).read_text()
a = text.index('//! `TreeAnimation` 真实 stride')
b = text.index('//! （+0x8/+0xc/+0x10）', a)
text = text[:a] + '''//! TreeAnimation 已恢复为实体自身的时长/进度/树冠可见性/临时隐藏标志。
//! PrefabInstanceData 的放置记录写面仍缺失；InteractableForestTree 的
//! 动画字段仍借 position 的 x/y/z 偏移承载
''' + text[b:]
a = text.index('/// 手放树逐树进度跟踪')
b = text.index('/// 手放树 canopy 全局生长状态', a)
text = text[:a] + text[b:]
text = text.replace('    mut per_tree: Local<HandplacedTreeProgress>,\n', '', 1)
a = text.index('    // TreeAnimation 真实 stride')
b = text.index('\n}\n\n/// `forest_tree_animation`', a)
text = text[:a] + '''    // 进度属于实体；查询顺序改变、实体删除或切换 archetype 不会串用别的树状态。
    for (mut animation, _instance, marked) in trees.iter_mut() {
        let changed = animation.advance(delta, marked.is_some(), !(subtract && !audio_hit));
        // PrefabInstanceData 的 GPU 放置记录写面尚待接线；打包公式保持原式。
        if changed {
            let fade = animation.canopy_visibility;
            let _packed = pack_2x16f_uint(animation.progress, 1.0 - (1.0 - fade) * (1.0 - fade));
        }
    }''' + text[b:]
dest = OUT / path
dest.parent.mkdir(parents=True, exist_ok=True)
dest.write_text(text)

edit('crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs', [
    ('''    // `Ui2dImage::new(label)`。core 的 Ui2dImage 仍是 typed-gap（{_gap: u64}），
    // 本实现保留克隆并登记，组件暂以 default() 落库。
    let _label = item.label().to_owned();''', '''    // Ui2dImage 持有 label；构造时图层为零。
    let label = item.label().to_owned();'''),
    ('country_core::components::Ui2dImage::default(),', 'country_core::components::Ui2dImage::new(label),'),
    ('/// - `Ui2dImage::new(label)` 载荷面 typed-gap（core 类型字段未落）。', '/// - 图像名称/图层已接入 core Ui2dImage；其余未恢复面见下方 kind_handler。'),
])

edit('crates/country-core/src/systems/render_loop/draw_passes/ui_2d_pass.rs', [
    ('pub struct Ui2dImageRef {\n    pub name: String,\n    pub layer: u32,\n}', '''pub struct Ui2dImageRef {
    pub name: String,
    pub layer: u32,
}

impl From<&crate::components::Ui2dImage> for Ui2dImageRef {
    fn from(image: &crate::components::Ui2dImage) -> Self {
        Self { name: image.name.clone(), layer: image.layer }
    }
}'''),
    ('//! 组件名由 PDB 查询类型确定（O）；组件布局在 `components::Ui2d*` / `resources::color_icon` 里仍是 typed gap，下面的 `*Read` 记录只含被机器码读取的字段（字段真名 U）。', '//! Ui2dExtent/Ui2dImage/UiDrawOrder 已有共享组件定义；下面的 owned 绘制快照从组件转换。color_icon 等其余读取视图仍保留各自证据边界。'),
])

path = 'crates/country-core/src/components/registry.rs'
text = (BASE / path).read_text()
b = text.index('/// `ComponentCloneBehavior`')
text = '''//! 原作 B 的组件证据清单，供恢复源码和核对接口使用。
//!
//! 记录 57 个类型的名称、register_required_components 地址及 clone_behavior。
//! 表中全部 required 集合为空；这不是运行时注册器，也不表示 57 种载荷均已实现。
//! 注册/克隆胶水由锁定版本的 Bevy 依赖提供，不计独立业务恢复。
//!
//! Default 是 Bevy 的默认克隆策略，不能理解为调用组件的 Default::default；
//! Custom 表示原入口选择 component_clone_via_clone::<T>。
//! 证据入口：country-core-deep/components-geometry/ALGORITHMS.md §1–§2。

''' + text[b:]
text = text.replace('/// 组件没有 `Clone`。', '/// 原入口返回 Bevy Default（本清单中为未实现 Clone 的类型）。')
text = text.replace('/// 组件 derive `Clone`；`handler_va` 是该组件独立的 `clone_behavior` 函数。', '/// 原入口选择 Clone 专用处理；handler_va 记录该选择入口的地址。')
(OUT / path).write_text(text)
