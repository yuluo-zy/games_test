# -*- coding: utf-8 -*-
import io, os

CH = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters"

PATCHES = [
    # (rel, old_substring, new_substring)
    # decorator impl$97::from — machine code: 16B copy, no disc write
    ("resources/walls/decorator.rs.md",
     "| `country_core::resources::walls::decorator::impl$97::from` | 0x140839560 | 403 | DecoratorExtraInfo From 变体 8（写判别字节 +0x11=8） | O |  |",
     "| `country_core::resources::walls::decorator::impl$97::from` | 0x140839560 | 403 | DecoratorExtraInfo From 变体 8 | O（round3 round2 复查修正：原\u201c写判别字节 +0x11=8\u201d与机器码不符）：机器码 16B 仅拷贝 payload {16B@+0, u32@+0x10}，无判别写入（对照 impl$96 0x140839550 确有 c6 41 11 07） | rne-read 0x140839560/0x140839550 |"),

    # ui_state into_drag_state + dragged — real 26B function, not folded no-op
    ("systems/ui/resources/ui_state.rs.md",
     "| `country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged` | 0x14092df10 | 254 | UiWidgetContext::dragged（与 Slider3d validate_param 折叠） | O：折叠单行（NotFound）；语义=拖拽中（DragState::is_started） | ui-input\\decomp\\decomp-0x14092df10.txt |",
     "| `country_core::systems::ui::resources::ui_state::UiWidgetContext::dragged` | 0x14092df10 | 254 | UiWidgetContext::dragged（与 into_drag_state 同址折叠） | O（round3 round2 复查修正：原\u201c折叠单行（NotFound）\u201d证伪）：真实 26B 条件函数（默认返 2；[+0x18]>=2 且 [+0x38]==[self+8] 时返 byte@+0x19）；S_PUB32 该址含 dragged 与 IntoHandleDragState 两符号 | rne-read 0x14092df10；S_PUB32 |"),
    ("systems/ui/resources/ui_state.rs.md",
     "| `country_core::systems::ui::resources::slider3d::impl$1::into_drag_state` | 0x14092df10 | 255 | UiWidgetContext::dragged（与 Slider3d validate_param 折叠） | O：折叠单行（NotFound）；语义=拖拽中（DragState::is_started） | ui-input\\decomp\\decomp-0x14092df10.txt |",
     "| `country_core::systems::ui::resources::slider3d::impl$1::into_drag_state` | 0x14092df10 | 255 | into_drag_state（与 dragged 同址折叠） | O（round3 round2 复查修正：原\u201c折叠（NotFound）\u201d证伪）：真实 26B 条件函数（默认返 2；状态 tag@+0x18>=2 且 @+0x38==self+8 时返 byte@+0x19）；\u201c语义=DragState::is_started\u201d未证实（判别值映射需枚举证据） | rne-read 0x14092df10；PDB size 26 |"),

    # widget_params on_drag_release — real 70B struct copy
    ("systems/ui/resources/widget_params.rs.md",
     "| `country_core::systems::ui::resources::widget_params::UiSounds::on_drag_release` | 0x1409a90c0 | 199 | UiSounds::on_drag_release | O：折叠（NotFound；槽 6-8） | ui-input\\decomp\\decomp-0x1409a90c0.txt |",
     "| `country_core::systems::ui::resources::widget_params::UiSounds::on_drag_release` | 0x1409a90c0 | 199 | UiSounds::on_drag_release | O（round3 round2 复查修正：原\u201c折叠（NotFound）\u201d证伪）：真实 70B 结构复制/重排（槽 0..0x50 复制；[+0x30]=[src+0]、[+0x40]=[r8+0x10] 替换、0x48/0x50 直拷；\u201c槽 6-8\u201d与触及 0x30..0x48 部分吻合） | rne-read 0x1409a90c0；PDB size 70 |"),

    # ui_move_shapes — evidence file exists under reconstruction/country-core-render/wp20
    ("systems/ui_move_shapes.rs.md",
     "| `country_core::systems::ui_move_shapes::move_gizmo_and_trigger_vol` | 0x14097d280 | 594 | 施加 delta + volume 触发 | O：move_gizmo_and_trigger_vol（wp20 既有） | wp20/analysis/0x14097d280_move_gizmo_and_trigger_vol.txt |",
     "| `country_core::systems::ui_move_shapes::move_gizmo_and_trigger_vol` | 0x14097d280 | 594 | 施加 delta + volume 触发 | O：move_gizmo_and_trigger_vol（round3 round2 补证：证据文件在 reconstruction/country-core-render/wp20/analysis/，11KB 反编译与该址 PDB 吻合，非缺失） | wp20/analysis/0x14097d280_move_gizmo_and_trigger_vol.txt（reconstruction/country-core-render/ 下） |"),

    # toolbar Toolbar::new — 0x140af9000 inside same function
    ("resources/toolbar.rs.md",
     "| `country_core::resources::toolbar::Toolbar::new` | 0x140af8c50 | 0 | 构造（Toolbar::new） | O(部分)：SOURCE_MAP 地址；此变体 alloc 0x30×8B 元素表（与 0x140af9000 的 11 类 0x10B 表不同——两地址为 new 的不同单态/内联，关系待核 I） |  |",
     "| `country_core::resources::toolbar::Toolbar::new` | 0x140af8c50 | 0 | 构造（Toolbar::new） | O(部分，round3 round2 复查修正)：PDB/S_PUB32 身份成立（Toolbar::new）；但行为断言未核——0x140af9000 实为该函数体内 +0x1b0（非独立函数），\u201c不同单态/内联\u201d说法可疑；alloc 0x30×8B 元素表待反汇编核验，未核前不升完整 O | PDB/S_PUB32 0x140af8c50 |"),

    # signifiers new — 35 callers unsupported
    ("systems/ui/signifiers/mod.rs.md",
     "| `country_core::systems::ui::signifiers::SignifierEntityBuilder::new` | 0x14089d9b0 | 153 | 构造（SignifierEntityBuilder::new） | O（wp15 b6）：35 callers，默认值序列见文件作用 | wp15/analysis/b6_widgets/new_0x14089d9b0.c |",
     "| `country_core::systems::ui::signifiers::SignifierEntityBuilder::new` | 0x14089d9b0 | 153 | 构造（SignifierEntityBuilder::new） | O（round3 round2 复查修正）：默认值序列实证（wp15 b6 反编译：vertex_color_shader@0x60..0x70、0x3e4ccccd f32@+0x78、0x100 u16@+0x40、0x13f800000、13B 串长）；\u201c35 callers\u201d未证实（与 round1 rotation 同规则，数量断言缺 xrefs 证据，删除） | wp15/analysis/b6_widgets/new_0x14089d9b0.c（reconstruction/country-core-render/ 下） |"),
]

for rel, old, new in PATCHES:
    p = os.path.join(CH, rel)
    txt = io.open(p, encoding="utf-8").read()
    if old not in txt:
        print("NOT FOUND:", rel, "|", old[:70])
        continue
    io.open(p, "w", encoding="utf-8", newline="").write(txt.replace(old, new, 1))
    print("patched:", rel)
