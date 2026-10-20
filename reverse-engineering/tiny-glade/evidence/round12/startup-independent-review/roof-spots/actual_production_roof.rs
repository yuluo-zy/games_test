use country_core::resources::roofs::roof_shape::Roof;
#[path="D:/game/tiny-glade/crates/systems/critters/src/bird_spots.rs"] mod bird_spots;
#[path="D:/game/tiny-glade/crates/systems/critters/src/wren/roof_spots.rs"] mod roof_spots;
use bird_spots::BirdSpotManager;
use roof_spots::sample_roof_bird_spots;
fn fixtures() -> serde_json::Value { serde_json::from_str(include_str!("D:/game/tiny-glade/crates/tiny-glade/tests/fixtures/roof_birdspot_original.json")).unwrap() }
fn roof(case: &serde_json::Value) -> Roof {
    let s=case["roof_hex"].as_str().unwrap(); let mut bytes=[0;88];
    for (i,b) in bytes.iter_mut().enumerate() { *b=u8::from_str_radix(&s[i*2..i*2+2],16).unwrap(); }
    Roof::from_observed(bytes)
}
#[test]
fn complete_roof_perches_match_92_original_instruction_cases() {
    let fixture=fixtures(); let cases=fixture["cases"].as_array().unwrap(); assert_eq!(cases.len(),92);
    let mut nonempty=0;
    for case in cases {
        let mut manager=BirdSpotManager::default();
        let points=sample_roof_bird_spots(&roof(case), case["frame_count"].as_u64().unwrap() as u32, &mut manager);
        let expected=case["native_points"].as_array().unwrap(); assert_eq!(points.len(),expected.len(),"{}",case["name"]);
        nonempty+=usize::from(!points.is_empty());
        for (point, expected) in points.iter().zip(expected) {
            assert!(point.occupant.is_none()); assert_eq!(point.id.0,expected["id"].as_u64().unwrap() as u32);
            for (axis, actual) in [point.pos.x,point.pos.y,point.pos.z].into_iter().enumerate() {
                assert_eq!(actual.to_bits(),expected["position_bits"][axis].as_u64().unwrap() as u32,"case {} axis {axis}",case["name"]);
            }
        }
    }
    assert_eq!(nonempty,55);
}
