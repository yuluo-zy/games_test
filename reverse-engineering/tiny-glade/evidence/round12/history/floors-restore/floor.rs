//! Canonical floor component, B construct_floors1407661e0.
//! Native40B: WallId8 + complete ShapeParameters28 + alignment padding4.
use bevy_ecs::component::Component;
use crate::resources::walls::{WallId, publication_fields::WallShapeSnapshot};

#[repr(C)]
#[derive(Component, Clone, Debug)]
pub struct Floor {
    pub wall: WallId,
    pub shape: WallShapeSnapshot,
}
impl Floor {
    /// Semantic view for the existing StoneFloor GPU writer. The final word is
    /// unused alignment padding; no native game field is invented there.
    pub fn words(&self) -> crate::render::nani::instance_updates::FloorWords {
        let mut words=[0u32;10];words[0]=self.wall.0 as u32;words[1]=(self.wall.0>>32) as u32;
        for (target,source) in words[2..9].iter_mut().zip(self.shape.bytes().chunks_exact(4)) {
            *target=u32::from_le_bytes(source.try_into().unwrap());
        }
        words
    }
}

/// Original native MAXSS radius,1; multiply3; log2f; MAXSS1,value;
/// MINSS5,value; saturating unsigned cast. Rectangle uses atlas part0.
pub fn floor_atlas_part(shape:&WallShapeSnapshot)->u32 {
    if shape.tag()&1!=0 {return 0;}
    let radius=f32::from_le_bytes(shape.bytes()[12..16].try_into().unwrap());
    let radius=if radius>1.0 {radius}else{1.0};
    let value=(radius*3.0).log2();
    let value=if 1.0>value {1.0}else{value};
    let value=if 5.0<value {5.0}else{value};
    value as u32
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn canonical_floor_is_40_bytes_and_retains_the_full_wall_id_and_shape() {
        use core::mem::{size_of,offset_of};assert_eq!(size_of::<Floor>(),40);assert_eq!(offset_of!(Floor,shape),8);
        let source=core::array::from_fn(|i|(i as u8).wrapping_mul(13));
        let floor=Floor{wall:WallId(0xfedcba98_76543210),shape:WallShapeSnapshot::from_observed(source)};
        let words=floor.words();assert_eq!(words[0],0x76543210);assert_eq!(words[1],0xfedcba98);
        let bytes:Vec<_>=words[2..9].iter().flat_map(|w|w.to_le_bytes()).collect();assert_eq!(bytes,source);
    }
}
