use bevy_app::App;
use bevy_ecs::event::{Event, EventRegistry, Events};


struct Probe; impl Event for Probe { type Traversal = (); }

fn main() {
    let mut bad = App::new();
    bad.world_mut().init_resource::<Events<Probe>>();
    bad.add_event::<Probe>();
    bad.world_mut().send_event(Probe);
    for _ in 0..4 { bad.update(); }
    assert_eq!(bad.world().resource::<Events<Probe>>().len(), 1);

    let mut good = App::new();
    good.add_event::<Probe>();
    // The real candidate registers before resources-only helpers. Their init
    // and all later add_event calls preserve the container and register once.
    good.world_mut().init_resource::<Events<Probe>>();
    good.add_event::<Probe>();
    good.add_event::<Probe>();
    good.world_mut().send_event(Probe);
    good.update();
    assert_eq!(good.world().resource::<Events<Probe>>().len(), 1);
    for _ in 0..3 { good.update(); }
    assert_eq!(good.world().resource::<Events<Probe>>().len(), 0);

    EventRegistry::register_event::<Probe>(bad.world_mut());
    for _ in 0..4 { bad.update(); }
    // A newly registered entry does not mark the old container changed. This
    // late patch cannot retroactively repair maintenance of preexisting data.
    assert_eq!(bad.world().resource::<Events<Probe>>().len(), 1);
    bad.world_mut().send_event(Probe);
    for _ in 0..4 { bad.update(); }
    assert_eq!(bad.world().resource::<Events<Probe>>().len(), 0);
    println!("PASS: Bevy0.16 actual App; bad order retains1 after4frames; early add_event plus repeated registration preserves1 after1frame and expires0 after4frames; late registry alone retains old data until a new write");
}

