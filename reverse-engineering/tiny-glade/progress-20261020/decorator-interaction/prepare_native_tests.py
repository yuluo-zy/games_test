from pathlib import Path
import hashlib,json
here=Path(__file__).parent
root=Path('D:/game')
target=root/'tiny-glade/crates/systems/decorator/src/ui_systems/native_interaction_goldens.rs'
moves=json.loads((here/'move-destination-goldens.json').read_text())
grabs=json.loads((here/'grab-offset-goldens.json').read_text())
offsets=json.loads((here/'offset-ray-goldens.json').read_text())
def arr(values):return '['+','.join(str(v) for v in values)+']'
source='''//! Generated native-output fixtures, not a second implementation of the business algorithm.
//! B F507A6...; complete original bodies + named dependency hooks are retained in
//! progress-20261020/decorator-interaction/collect_native.py and native-manifest.json.
use super::{move_ops::*, calculate_decorator_grab_offset::*};
use country_core::resources::walls::{WallId, WallState, InnerWalls};
use country_core::resources::walls::decorator::*;
use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::publication_source::TerrainHeightGrid512;
use country_core::resources::walls::freeform_height_cache::FreeformWallHeightCache;
use country_core::resources::roofs::geometry::surface::Curve2;
use country_core::systems::collision::world::RaycastWorldResult;
use country_core::utils::components::transform::Transform;
use glam::{Vec2,Vec3};
use utils::wall_space::WallSpaceCoord;

fn published_walls() -> PublicWalls {
    let mut inner=InnerWalls::default();
    let curve=Curve2::try_new(vec![[2.25,-4.0],[6.25,-4.0]],false).unwrap();
    inner.new_wall_with_id(WallId(0x9876),WallState::new(curve,4.0,0,0,false));
    let terrain=TerrainHeightGrid512 { encoded:vec![0u16;512*512].into(),minimum_y:0.0,maximum_y:0.0,sampling_fix:false };
    let mut walls=PublicWalls::default();
    walls.consume(&mut inner,&terrain);
    let wall=walls.get_mut(WallId(0x9876)).unwrap();
    // Post-initialization fixture supplies the independently owned freeform
    // cache. Production lookup/height/world-coordinate functions remain real.
    wall.freeform_heights=Some(FreeformWallHeightCache { maximum_y:2.25,
        arch_segment_heights:vec![2.25;wall.geometry.outline.len()],
        segment_heights:vec![0.0;wall.geometry.outline.len()] });
    walls
}
fn u32_at(bytes:&[u8],at:usize)->u32 { u32::from_le_bytes(bytes[at..at+4].try_into().unwrap()) }
fn u64_at(bytes:&[u8],at:usize)->u64 { u64::from_le_bytes(bytes[at..at+8].try_into().unwrap()) }
fn f32_at(bytes:&[u8],at:usize)->f32 { f32::from_bits(u32_at(bytes,at)) }
fn info_from_bytes(b:&[u8;72])->DecoratorInfo {
    DecoratorInfo { dst_wall:u64_at(b,0),dst_secondary:u64_at(b,8),dst_payload:u64_at(b,16),u18:u64_at(b,24),
        u20:u32_at(b,32),u24:u32_at(b,36),u28:u32_at(b,40),decorator_id:u32_at(b,44),u30:u64_at(b,48),
        u38:u32_at(b,56),u3c:u32_at(b,60),u40:u32_at(b,64),rank:b[68],u45:b[69],ty:b[70],state:b[71] }
}
fn info_bytes(i:&DecoratorInfo)->Vec<u8> {
    let mut b=Vec::new();
    for v in [i.dst_wall,i.dst_secondary,i.dst_payload,i.u18] {b.extend(v.to_le_bytes());}
    for v in [i.u20,i.u24,i.u28,i.decorator_id] {b.extend(v.to_le_bytes());}
    b.extend(i.u30.to_le_bytes());
    for v in [i.u38,i.u3c,i.u40] {b.extend(v.to_le_bytes());}
    b.extend([i.rank,i.u45,i.ty,i.state]);b
}

#[test]
fn original_move_destination_preserves_all_72_bytes_with_real_published_wall() {
    let walls=published_walls();
    for (before,intent,expected) in MOVES {
        let mut info=info_from_bytes(&before);
        let pair=|at|Vec2::new(f32_at(&intent,at),f32_at(&intent,at+4));
        let candidate=match u32_at(&intent,0x20) {
            0=>MoveDstCandidate::Roof(RoofDecoratorDst {coord:pair(0x28),yaw:DecoratorRotationXZ(f32_at(&intent,0x24))}),
            1=>MoveDstCandidate::Wall {storage:DecoratorStorageAddr::Wall(WallId(u64_at(&intent,0x18))),
                coordinate:WallSpaceCoord::new_unchecked(f32_at(&intent,0x28),f32_at(&intent,0x2c)),
                platform:WallId(u64_at(&intent,0x30)),corner_attachment:intent[0x38],
                bottom_as_door:intent[0x39]!=0,compound:intent[0x3a]!=0},
            2=>MoveDstCandidate::Terrain(TerrainDecoratorDst {position_xz:pair(0x28),yaw:DecoratorRotationXZ(f32_at(&intent,0x24))}),
            3=>MoveDstCandidate::Stair(StairDecoratorDst {first_id:u32_at(&intent,0x28),second_id:u32_at(&intent,0x2c),
                coordinate:pair(0x30),yaw:DecoratorRotationXZ(f32_at(&intent,0x24))}),
            _=>unreachable!(),
        };
        calculate_move_dst(&mut info,candidate,&walls);
        assert_eq!(info_bytes(&info),expected);
    }
}

#[test]
fn original_grab_offsets_and_same_surface_gates_400_cases() {
    let walls=published_walls();
    let transform=Transform::from_translation(Vec3::new(1.0,2.0,3.0));
    for (destination,ty,source,present,offset) in GRABS {
        let info=DecoratorInfo { dst_wall:0x0000000500000003,u20:u32::from(destination)<<16,ty,..Default::default() };
        let hit=RaycastWorldResult {handle:source,position:Vec3::new(1.25,2.5,3.75),..RaycastWorldResult::miss()};
        let got=try_get_grab_offset_ss_if_valid(&info,&transform,DecoratorStorageAddr::Wall(WallId(0x9876)),&walls,
            Vec2::new(4.0,5.0),|p|Vec2::new(p.x*2.0+p.z,p.y*3.0),|delta| {
                assert_eq!(delta,Vec2::ONE);hit
            },|_,_|panic!("freehand fixture must not access premade trigger"));
        assert_eq!(got.is_some(),present,"destination={destination} type={ty} source={source:?}");
        if let Some(got)=got { assert_eq!(got.to_array().map(f32::to_bits),offset); }
    }
}

#[test]
fn original_offset_raycast_priority_mask_fallback_24_cases() {
    use std::cell::Cell;
    for (ty,physical,terrain,dy,mask,tag,position,normal) in OFFSETS {
        let calls=Cell::new(0);
        let ray=utils::geometry::Ray { origin:Vec3::new(1.0,2.0,3.0),direction:Vec3::new(2.0,dy,4.0) };
        let got=select_offset_raycast(ray,ty,|_,got_mask|{
            assert_eq!(got_mask,mask);
            RaycastWorldResult {handle:[0,0x9876,0],position:Vec3::new(17.0,18.0,19.0),normal:Vec3::new(0.25,0.5,0.75),
                discriminant:if physical {0}else{2},..RaycastWorldResult::miss()}
        },|_|{calls.set(calls.get()+1);terrain.then_some(Vec3::new(7.0,8.0,9.0))});
        assert_eq!(calls.get(),1);
        assert_eq!(got.handle[0] as u8,tag);
        assert_eq!(got.position.to_array().map(f32::to_bits),position);
        assert_eq!(got.normal.to_array().map(f32::to_bits),normal);
    }
}

const MOVES:[([u8;72],[u8;64],[u8;72]);8]=[
MOVE_ROWS
];
const GRABS:[(u8,u8,[u64;3],bool,[u32;2]);400]=[
GRAB_ROWS
];
const OFFSETS:[(u8,bool,bool,f32,u32,u8,[u32;3],[u32;3]);24]=[
OFFSET_ROWS
];
'''
source=source.replace('MOVE_ROWS','\n'.join('('+arr(bytes.fromhex(c['before_hex']))+','+arr(bytes.fromhex(c['intent_hex']))+','+arr(bytes.fromhex(c['after_hex']))+'),' for c in moves))
source=source.replace('GRAB_ROWS','\n'.join(f'({c["destination_tag"]},{c["type"]},{arr([int.from_bytes(bytes.fromhex(c["hit_hex"])[i:i+8],"little") for i in (0,8,16)])},{str(c["present"]).lower()},{arr(c["offset_bits"] or [0,0])}),' for c in grabs))
source=source.replace('OFFSET_ROWS','\n'.join(f'({c["type"]},{str(c["physical"]).lower()},{str(c["terrain"]).lower()},{c["direction_y"]}f32,{c["mask"]},{c["source_tag"]},{arr(c["position_bits"])},{arr(c["normal_bits"])}),' for c in offsets))
target.write_text(source,encoding='utf-8',newline='\n')
(here/'native-tests-manifest.json').write_text(json.dumps({'source':str(target),'sha256':hashlib.sha256(target.read_bytes()).hexdigest(),'fixtures':{name:hashlib.sha256((here/name).read_bytes()).hexdigest() for name in ['move-destination-goldens.json','grab-offset-goldens.json','offset-ray-goldens.json']}},indent=2)+'\n')
print('authored production tests from original native receipts: 8 full-record move, 400 grab, 24 offset')
