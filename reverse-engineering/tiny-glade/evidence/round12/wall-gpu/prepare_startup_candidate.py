from pathlib import Path
root=Path('D:/game');base=root/'reverse-engineering/tiny-glade/evidence/round12/wall-gpu/candidates-startup'
p=base/'crates/country-core/src/startup/startup_walls.rs'
s=p.read_text(encoding='utf-8-sig').replace('draw_callbacks_a_views::WallBrickInstancedDrawProxyView','draw_callbacks_a::WallBrickInstancedDrawProxyView').replace('No vertex attributes are present: LOD1 derives its corners from vertex ID.','No vertex attributes are present; do not generate a generic cube mesh.')
p.write_text(s,encoding='utf-8')
rel=Path('crates/country-core/src/geometry/instanced_wall.rs');s=(root/'tiny-glade'/rel).read_text(encoding="utf-8");s += '''\n/// Original three-byte global wall draw policy component, written by\n/// glade_startup_walls and consumed by the LOD0/LOD1 indirect callbacks.\n/// Field names remain observed byte offsets: semantic enum names are unproved.\n#[repr(C)]\n#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default, PartialEq, Eq)]\npub struct WallBrickInstancedDrawProxy {\n    pub b0: u8,\n    pub b1: u8,\n    pub b2: u8,\n}\n''';p=base/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(s,encoding="utf-8")
rel=Path('crates/country-core/src/systems/render_loop/draw_passes/draw_callbacks_a_views.rs');s=(root/'tiny-glade'/rel).read_text(encoding="utf-8");old='''#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WallBrickInstancedDrawProxyView {
    pub b0: u8,
    pub b1: u8,
    pub b2: u8,
}''';assert old in s;s=s.replace(old,'''pub use crate::geometry::instanced_wall::WallBrickInstancedDrawProxy as WallBrickInstancedDrawProxyView;''');p=base/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(s,encoding="utf-8")

