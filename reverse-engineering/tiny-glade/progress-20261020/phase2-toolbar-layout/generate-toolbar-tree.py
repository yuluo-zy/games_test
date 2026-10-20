from pathlib import Path
import json,struct
r=Path('D:/game');ev=r/'reverse-engineering/tiny-glade/progress-20261020/phase2-toolbar-layout';ds=json.loads((ev/'native-toolbar-setup-trees.json').read_text());cats=['Building','Path','Decorators','Customization','Eraser','Tree','Terrain','Editing','Water','Garden','Photomode','TimeOfDay']
def mode(raw):
 b=bytes.fromhex(raw);k=struct.unpack_from('<I',b)[0]
 if k==6:return 'AppMode::Wall(WallShape::'+['Freeform','Circle','Rectangle'][b[4]]+')'
 if k==7:return 'AppMode::Path'
 if k==8:return 'AppMode::Cobblestone('+str(struct.unpack_from('<Q',b,8)[0])+')'
 if k==9:return 'AppMode::Eraser(EraserLayer::'+['All','Wall','Path','Bulldoze','Water','Garden','Tree'][b[4]]+')'
 if k==11:return 'AppMode::Decorator(DecoratorType::'+{56:'CottageWindow',57:'GothicWindow',58:'ArrowSlit',59:'Chimney',60:'Lantern',61:'Flag',63:'Stairs'}[b[4]]+')'
 if k==18:return 'AppMode::Fences'
 raise ValueError(raw)
def expr(i):
 k=i['kind'];kind={6:'K::Spacer',7:'K::Locked'}.get(k)
 if k in [4,5]:kind='K::'+{4:'Row',5:'Column'}[k]+'(vec!['+', '.join(map(expr,i['children']))+'])'
 if k==1:kind='K::AppMode('+mode(i['mode_raw'])+')'
 if k in [2,3]:kind='K::'+{2:'Slider',3:'SliderReset'}[k]+'('+str(i['sub'])+')'
 assert kind is not None
 x,y=i['size'];size='Vec2::new('+str(x)+', '+str(y)+')'
 e='ToolbarItem::new('+kind+', '+json.dumps(i['label'])+', '+size+')'
 if i['show_only_fpv']:e='{ let mut item = '+e+'; item.show_only_fpv=true; item }'
 return e
s='''//! Original toolbar_setup category tree (142345890) and its complete two
//! edition branches. Data emitted from the original machine code, not labels.
//! Native capture: phase2-toolbar-layout/native-toolbar-setup-trees.json.
use bevy_ecs::system::Commands;
use glam::Vec2;
use country_core::{resources::toolbar::ToolbarCategory, resources::ui_settings::ToolbarAlignment,
    systems::mode_manager::{AppMode, WallShape, EraserLayer, DecoratorType}};
use crate::resouces_and_components::{toolbar_layout::{ToolbarItem, ToolbarItemKind as K, ToolbarLayout},
    taffy_layout::TaffyLayout, SwitchToolbarCategoryButton, UiLocked, TimeOfDayButtonV2};

pub fn toolbar_category_items(demo: bool) -> Vec<(&'static str, ToolbarCategory, Vec<ToolbarItem>)> {
    if demo {
        vec![
'''
for cat in ds[1]['categories']:s+='            ('+json.dumps(cat['label'])+', ToolbarCategory::'+cats[cat['tag']]+', vec!['+', '.join(map(expr,cat['items']))+']),\n'
s+='''        ]
    } else {
        vec![
'''
for cat in ds[0]['categories']:s+='            ('+json.dumps(cat['label'])+', ToolbarCategory::'+cats[cat['tag']]+', vec!['+', '.join(map(expr,cat['items']))+']),\n'
s+='''        ]
    }
}

/// Normal toolbar portion of the original OnEnterGame setup. The outer owner
/// resets Taffy once, then constructs this, the photomode toolbar and undo/redo.
pub fn setup_toolbar(
    taffy: &mut TaffyLayout,
    commands: &mut Commands,
    demo: bool,
    alignment: ToolbarAlignment,
) -> ToolbarLayout {
    let (screen, toolbar) = ToolbarLayout::setup_screen_and_toolbar_nodes(taffy);
    let mut layout = ToolbarLayout { node_a: screen, node_b: toolbar, ..Default::default() };
    for (label, category, items) in toolbar_category_items(demo) {
        layout.add_category_button(taffy, category, label, commands, |entity| {
            if category == ToolbarCategory::TimeOfDay {
                entity.insert(TimeOfDayButtonV2);
            } else if demo && matches!(category, ToolbarCategory::Tree | ToolbarCategory::Water) {
                entity.insert(UiLocked);
            } else {
                entity.insert(SwitchToolbarCategoryButton { category });
            }
        }, None);
        layout.add_contextual_menu(taffy, category as u8, items, commands);
    }
    let ids: Vec<_> = layout.menus.iter().map(|row| (row.parent_entity, row.parent_entity.to_bits())).collect();
    layout.link_up_cyclic_list_nav(&ids, alignment.is_horizontal(), commands);
    layout
}
''';(r/'tiny-glade/crates/systems/toolbar/src/toolbar_startup.rs').write_text(s,encoding='utf-8');print(len(s))
