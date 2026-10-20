from pathlib import Path
import json,struct
r=Path('D:/game');ds=json.loads((r/'reverse-engineering/tiny-glade/progress-20261020/phase2-toolbar-layout/native-toolbar-setup-trees.json').read_text());expected=[]
def flatten(i,depth,out):
 b=bytes.fromhex(i.get('mode_raw','00'*16));tag=struct.unpack_from('<I',b)[0] if i['kind']==1 else 0;payload=struct.unpack_from('<Q',b,8)[0] if tag==8 else b[4] if tag in [6,9,11] else 0
 x,y=i['size'];bits=lambda f:struct.unpack('<I',struct.pack('<f',f))[0]
 out.append(f"{depth}:{i['kind']}:{i['label']}:{bits(x):08x}:{bits(y):08x}:{i['show_only_fpv']}:{i.get('sub',0)}:{tag}:{payload}")
 for c in i.get('children',[]):flatten(c,depth+1,out)
for d in ds:
 out=[]
 for c in d['categories']:
  out.append('category:'+str(c['tag'])+':'+c['label'])
  for i in c['items']:flatten(i,0,out)
 expected.append('\n'.join(out))
s='''use super::*;
fn flatten(item: &ToolbarItem, depth:usize, out:&mut Vec<String>) {
    use country_core::systems::mode_manager::AppMode as A;
    let (kind,sub,mode,children) = match &item.kind {
        K::Custom(_) => (0,0,None,None), K::AppMode(mode) => (1,0,Some(*mode),None),
        K::Slider(kind) => (2,*kind,None,None), K::SliderReset(kind) => (3,*kind,None,None),
        K::Row(children) => (4,0,None,Some(children)), K::Column(children) => (5,0,None,Some(children)),
        K::Spacer => (6,0,None,None), K::Locked => (7,0,None,None),
    };
    let (tag,payload) = mode.map_or((0,0), |mode| (mode.native_tag(), match mode {
        A::Wall(shape) => shape as u64, A::Eraser(layer) => layer as u64,
        A::Decorator(deco) => deco.native_tag() as u64, A::Cobblestone(kind) =>kind, _=>0,
    }));
    out.push(format!("{depth}:{kind}:{}:{:08x}:{:08x}:{}:{sub}:{tag}:{payload}",item.label,item.size.x.to_bits(),item.size.y.to_bits(),u8::from(item.show_only_fpv)));
    if let Some(children)=children { for child in children {flatten(child,depth+1,out);} }
}
#[test]
fn both_complete_category_trees_match_original_machine_capture() {
    let expected = [\n'''+',\n'.join('r###"'+x+'"###' for x in expected)+'''];
    for (demo, expected) in [(false,expected[0]),(true,expected[1])] {
        let mut actual=Vec::new();
        for (label,category,items) in toolbar_category_items(demo) {
            actual.push(format!("category:{}:{label}",category as u8));
            for item in items { flatten(&item,0,&mut actual); }
        }
        assert_eq!(actual.join("\\n"),expected,"original edition demo={demo}");
    }
}
#[test]
fn real_setup_builds_eleven_categories_full_menus_parent_links_and_cyclic_nav() {
    use crate::resouces_and_components::*;
    use bevy_ecs::{prelude::*, world::CommandQueue};
    for demo in [false,true] {
        let mut taffy=TaffyLayout::default(); let mut world=World::new(); let mut queue=CommandQueue::default();
        let layout=setup_toolbar(&mut taffy,&mut Commands::new(&mut queue,&world),demo,ToolbarAlignment::Bottom);
        queue.apply(&mut world);
        assert_eq!(layout.menus.len(),11);
        assert_eq!(taffy.children(layout.node_b).len(),11);
        let mut leaf_count=0;
        for (_,_,items) in toolbar_category_items(demo) {
            fn count(i:&ToolbarItem)->usize{ match &i.kind {K::Row(c)|K::Column(c)=>c.iter().map(count).sum(),_=>1} }
            leaf_count+=items.iter().map(count).sum::<usize>();
        }
        assert_eq!(world.entities().len() as usize,11+leaf_count);
        for (index,row) in layout.menus.iter().enumerate() {
            let entity=row.parent_entity;
            let node=world.get::<TaffyNode>(entity).unwrap();
            assert_eq!(crate::taffy_update::taffy_node_base_size(node),Vec2::splat(110.));
            assert_eq!(world.get::<UiAnimationState>(entity).unwrap().appear,1.);
            assert_eq!(world.get::<UiInteractionState>(entity).unwrap().state,1);
            assert!(world.get::<HideIfMenuIsOpen>(entity).is_some());
            let nav=world.get::<UiNav>(entity).unwrap();
            assert_eq!(nav.links[2].value,layout.menus[(index+1)%11].parent_entity.to_bits());
            assert_eq!(nav.links[3].value,layout.menus[(index+10)%11].parent_entity.to_bits());
            if row.tag==11 { assert!(world.get::<TimeOfDayButtonV2>(entity).is_some()); }
            else if demo && [5,8].contains(&row.tag) {assert!(world.get::<UiLocked>(entity).is_some());}
            else {assert_eq!(world.get::<SwitchToolbarCategoryButton>(entity).unwrap().category as u8,row.tag);}
        }
        let mut query=world.query::<&UiShowIfParentIsInFocus>();
        assert!(query.iter(&world).all(|component|layout.menus.iter().any(|row|row.parent_entity.to_bits()==component.parent)));
    }
}
''';(r/'tiny-glade/crates/systems/toolbar/src/toolbar_startup_tests.rs').write_text(s,encoding='utf-8');p=r/'tiny-glade/crates/systems/toolbar/src/toolbar_startup.rs';s=p.read_text(encoding='utf-8')+'\n#[cfg(test)]\n#[path = "toolbar_startup_tests.rs"]\nmod startup_tests;\n';p.write_text(s,encoding='utf-8');print('native fixture lines',*[len(x.splitlines()) for x in expected])
