use super::*;
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
    let expected = [
r###"category:0:toolbar/building
0:5::3f800000:3f800000:0:0:0:0
1:4::3f800000:3f800000:0:0:0:0
2:1:toolbar/wall:3f800000:3f800000:0:0:6:0
2:1:toolbar/fence:3f800000:3f800000:0:0:18:0
1:4::3f800000:3f800000:0:0:0:0
2:1:toolbar/tower:3f800000:3f800000:0:0:6:1
2:1:toolbar/building_2:3f800000:3f800000:0:0:6:2
2:1:toolbar/stairs:3f800000:3f800000:0:0:11:63
category:1:toolbar/path
0:5::3f800000:3f800000:0:0:0:0
1:4::3f800000:3f800000:0:0:0:0
2:2:toolbar/slider_bg_path:40400000:3f800000:0:10:0:0
2:3:toolbar/reset_path:3f800000:3f800000:0:10:0:0
1:4::3f800000:3f800000:0:0:0:0
2:1:toolbar/path_2_large:3f800000:3f800000:0:0:7:0
2:1:toolbar/cobblestone_1:3f800000:3f800000:0:0:8:1
2:1:toolbar/cobblestone_2:3f800000:3f800000:0:0:8:2
2:1:toolbar/cobblestone_3:3f800000:3f800000:0:0:8:3
category:2:toolbar/decorators
0:5::3f800000:3f800000:0:0:0:0
1:4::3f800000:3f800000:0:0:0:0
2:1:toolbar/cottage_window:3f800000:3f800000:0:0:11:56
2:1:toolbar/gothic_window:3f800000:3f800000:0:0:11:57
2:1:toolbar/arrow_slit:3f800000:3f800000:0:0:11:58
1:4::3f800000:3f800000:0:0:0:0
2:1:toolbar/lantern:3f800000:3f800000:0:0:11:60
2:1:toolbar/flag:3f800000:3f800000:0:0:11:61
2:1:toolbar/chimney:3f800000:3f800000:0:0:11:59
category:3:toolbar/customization
category:4:toolbar/eraser
0:5::3f800000:3f800000:0:0:0:0
1:4::3f800000:3f800000:0:0:0:0
2:2:toolbar/slider_bg_eraser:40400000:3f800000:0:9:0:0
1:4::3f800000:3f800000:0:0:0:0
2:1:toolbar/garden:3f800000:3f800000:0:0:9:5
2:1:toolbar/tree:3f800000:3f800000:0:0:9:6
2:1:toolbar/water:3f800000:3f800000:0:0:9:4
1:4::3f800000:3f800000:0:0:0:0
2:1:toolbar/wall:3f800000:3f800000:0:0:9:1
2:1:toolbar/path_2:3f800000:3f800000:0:0:9:2
2:1:toolbar/hammer:3f800000:3f800000:0:0:9:3
category:6:toolbar/terrain
category:9:toolbar/garden
0:4::3f800000:3f800000:0:0:0:0
1:2:toolbar/slider_bg_garden:40400000:3f800000:0:12:0:0
1:3:toolbar/reset_garden:3f800000:3f800000:0:12:0:0
category:5:toolbar/tree
category:8:toolbar/water
0:4::3f800000:3f800000:0:0:0:0
1:2:toolbar/slider_bg_water:40400000:3f800000:0:11:0:0
1:3:toolbar/reset_water:3f800000:3f800000:0:11:0:0
category:10:toolbar/photomode
category:11:toolbar/time_of_day"###,
r###"category:0:toolbar/building
0:5::3f800000:3f800000:0:0:0:0
1:4::3f800000:3f800000:0:0:0:0
2:1:toolbar/wall:3f800000:3f800000:0:0:6:0
2:7:toolbar/decorators_lock_demo_fences:3f800000:3f800000:0:0:0:0
1:4::3f800000:3f800000:0:0:0:0
2:1:toolbar/tower:3f800000:3f800000:0:0:6:1
2:1:toolbar/building_2:3f800000:3f800000:0:0:6:2
2:7:toolbar/decorators_lock_demo_stairs:3f800000:3f800000:0:0:0:0
category:1:toolbar/path
0:5::3f800000:3f800000:0:0:0:0
1:4::3f800000:3f800000:0:0:0:0
2:2:toolbar/slider_bg_path:40400000:3f800000:0:10:0:0
2:3:toolbar/reset_path:3f800000:3f800000:0:10:0:0
1:4::3f800000:3f800000:0:0:0:0
2:7:toolbar/decorators_lock_demo_gothic_window:3f800000:3f800000:0:0:0:0
2:7:toolbar/decorators_lock_demo_gothic_window:3f800000:3f800000:0:0:0:0
2:7:toolbar/decorators_lock_demo_gothic_window:3f800000:3f800000:0:0:0:0
category:2:toolbar/decorators
0:5::3f800000:3f800000:0:0:0:0
1:4::3f800000:3f800000:0:0:0:0
2:1:toolbar/cottage_window:3f800000:3f800000:0:0:11:56
2:7:toolbar/decorators_lock_demo_gothic_window:3f800000:3f800000:0:0:0:0
2:7:toolbar/decorators_lock_demo_arrow_slit:3f800000:3f800000:0:0:0:0
1:4::3f800000:3f800000:0:0:0:0
2:1:toolbar/lantern:3f800000:3f800000:0:0:11:60
2:7:toolbar/decorators_lock_demo_flag:3f800000:3f800000:0:0:0:0
2:7:toolbar/decorators_lock_demo_chimney:3f800000:3f800000:0:0:0:0
category:3:toolbar/customization
category:4:toolbar/eraser
0:1:toolbar/wall:3f800000:3f800000:0:0:9:1
0:1:toolbar/path_2:3f800000:3f800000:0:0:9:2
0:1:toolbar/garden:3f800000:3f800000:0:0:9:5
0:1:toolbar/hammer:3f800000:3f800000:0:0:9:3
category:6:toolbar/terrain
category:9:toolbar/garden
0:4::3f800000:3f800000:0:0:0:0
1:2:toolbar/slider_bg_garden:40400000:3f800000:0:12:0:0
1:3:toolbar/reset_garden:3f800000:3f800000:0:12:0:0
category:5:toolbar/decorators_lock_demo_tree
category:8:toolbar/decorators_lock_demo_water
category:10:toolbar/photomode
category:11:toolbar/time_of_day"###];
    for (demo, expected) in [(false,expected[0]),(true,expected[1])] {
        let mut actual=Vec::new();
        for (label,category,items) in toolbar_category_items(demo) {
            actual.push(format!("category:{}:{label}",category as u8));
            for item in items { flatten(&item,0,&mut actual); }
        }
        assert_eq!(actual.join("\n"),expected,"original edition demo={demo}");
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
