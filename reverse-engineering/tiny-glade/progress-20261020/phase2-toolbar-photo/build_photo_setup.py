from pathlib import Path
import json,sys
H=Path(__file__).resolve().parent;R=H.parents[3];src=R/'tiny-glade/crates/systems/toolbar/src/photomode/setup_photomode_toolbar.rs'
trees=json.loads((H/'photomode-original-trees.json').read_text())
names={11:'TimeOfDay',12:'PhotomodeFolder',13:'PhotomodeCamera',14:'PhotomodeFov',15:'PhotomodeFocus',16:'PhotomodeDof',17:'PhotomodeTakePic',18:'PhotomodeEvShift',19:'PhotomodeLuts',20:'PhotomodeArtistic',21:'PhotomodeBack'}
def item(i):
 k=i['kind'];label=json.dumps(i['label']);x,y=i['size']
 if k==0:v='K::Custom(Box::new(|entity| { entity.insert(PhotomodeButton { tag: '+str(int(i['custom_payload'],16))+' }); }))'
 elif k in (2,3):v=f'K::{"Slider" if k==2 else "SliderReset"}({i["sub"]})'
 elif k in (4,5):v='K::'+('Row' if k==4 else 'Column')+'(vec!['+', '.join(map(item,i['children']))+'])'
 elif k in (6,7):v='K::'+('Spacer' if k==6 else 'Locked')
 else:raise ValueError(k)
 s=f'ToolbarItem::new({v}, {label}, Vec2::new({x}, {y}))'
 if i['show_only_fpv']:s='{ let mut item = '+s+'; item.show_only_fpv = true; item }'
 return s
p='''//! Original photo toolbar setup (B 0x1426246e0, PDB size 9053).
//! Both edition trees and nominal custom payload types were executed/captured
//! from the original binary. Root callback dispatch: 142623ff4..1426242ee.
//! Evidence: progress-20261020/phase2-toolbar-photo/photomode-original-trees.json
//! and photo-category-callbacks.json. Shared factory owns nodes and closures.
use bevy_ecs::system::Commands;
use glam::Vec2;
use country_core::resources::{toolbar::ToolbarCategory, ui_settings::ToolbarAlignment};
use crate::resouces_and_components::{
    toolbar_layout::{ToolbarItem, ToolbarItemKind as K, ToolbarLayout, PhotomodeToolbarLayout},
    taffy_layout::TaffyLayout, PhotomodeButton, ShowOnlyInPhotomode, TimeOfDayButtonV2, UiLocked,
};

pub fn photomode_category_items(demo: bool) -> Vec<(&'static str, ToolbarCategory, Vec<ToolbarItem>)> {
'''
for j,demo in enumerate((True,False)):
 tree=next(t for t in trees if t['demo']==demo)
 p+=('    if demo {\n' if j==0 else '    } else {\n')+'        vec![\n'
 for c in tree['categories']:p+='            ('+json.dumps(c['label'])+', ToolbarCategory::'+names[c['tag']]+', vec!['+', '.join(map(item,c['items']))+']),\n'
 p+='        ]\n'
p+='''    }
}

/// Construct every category root and contextual tree, then link the actual
/// category entities into the original cyclic keyboard/controller navigation.
pub fn setup_photomode_toolbar(
    taffy: &mut TaffyLayout,
    commands: &mut Commands,
    demo: bool,
    alignment: ToolbarAlignment,
) -> PhotomodeToolbarLayout {
    let (screen, toolbar) = ToolbarLayout::setup_screen_and_toolbar_nodes(taffy);
    let mut layout = ToolbarLayout { node_a: screen, node_b: toolbar, ..Default::default() };
    for (label, category, items) in photomode_category_items(demo) {
        layout.add_category_button_ordered(taffy, category, label, commands, |entity| {
            use ToolbarCategory::*;
            match category {
                TimeOfDay => { entity.insert(TimeOfDayButtonV2); }
                PhotomodeFolder => { entity.insert(PhotomodeButton { tag: 4 }); }
                PhotomodeCamera => { entity.insert(PhotomodeButton { tag: 5 }); }
                PhotomodeFocus => { entity.insert(PhotomodeButton { tag: 15 }); }
                PhotomodeTakePic => { entity.insert(PhotomodeButton { tag: 9 }); }
                PhotomodeLuts => {
                    // Original demo branch intentionally queues no action.
                    if !demo { entity.insert(PhotomodeButton { tag: 11 }); }
                }
                PhotomodeBack => { entity.insert(PhotomodeButton { tag: 17 }); }
                _ => { entity.insert(UiLocked); }
            }
            entity.insert(ShowOnlyInPhotomode);
        }, (category == ToolbarCategory::PhotomodeTakePic).then_some(1.25), true);
        layout.add_contextual_menu(taffy, category as u8, items, commands);
    }
    let ids: Vec<_> = layout.menus.iter().map(|row| (row.parent_entity, row.parent_entity.to_bits())).collect();
    layout.link_up_cyclic_list_nav(&ids, alignment.is_horizontal(), commands);
    PhotomodeToolbarLayout { menus: layout.menus, node_a: layout.node_a, node_b: layout.node_b,
        scale_progress: layout.scale_progress, hide_progress: layout.hide_progress }
}

#[cfg(test)]
#[path = "setup_photomode_toolbar_tests.rs"]
mod setup_tests;
'''
if '--write-production' in sys.argv:
 src.write_text(p,encoding='utf-8')
else:
 (H/'generated-photo-setup.rs').write_text(p,encoding='utf-8')
# Stable independently captured original-tree fixture, retaining native kind/sub,
# all labels, float bits, conditional visibility and callback enum payloads.
def flatten(i,depth,out):
 import struct
 bits=lambda x:format(struct.unpack('<I',struct.pack('<f',x))[0],'08x')
 sub=int(i['custom_payload'],16) if i['kind']==0 else i.get('sub',0)
 out.append(f'{depth}:{i["kind"]}:{i["label"]}:{bits(i["size"][0])}:{bits(i["size"][1])}:{i["show_only_fpv"]}:{sub}')
 for c in i.get('children',[]):flatten(c,depth+1,out)
for t in trees:
 out=[]
 for c in t['categories']:
  out.append(f'category:{c["tag"]}:{c["label"]}')
  for i in c['items']:flatten(i,0,out)
 (H/('photo-tree-demo.txt' if t['demo'] else 'photo-tree-full.txt')).write_text('\n'.join(out))
print(src)
