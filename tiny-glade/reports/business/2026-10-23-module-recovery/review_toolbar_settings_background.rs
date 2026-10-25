mod country_core {pub mod resources {pub mod session {#[derive(PartialEq)] pub enum GladeType {Summer=0,Olden=1,Flowery=2,Autumn=3,Winter=4}}}}
pub fn settings_background(glade: country_core::resources::session::GladeType, t: f32) -> &'static str {
    use country_core::resources::session::GladeType;
    if glade == GladeType::Olden {
        if (0.35 <= t && t <= 0.5) || (0.94 <= t && t <= 0.98) { "olden_sunset" }
        else if 0.5 <= t && t <= 0.84 { "olden_night" }
        else if 0.83 <= t && t <= 0.94 { "olden_sunrise" }
        else { "olden_day" }
    } else if (0.345 <= t && t <= 0.46) || (0.913 <= t && t <= 0.926) { "orange" }
    else if (0.46 <= t && t <= 0.48) || (0.84 <= t && t <= 0.913) { "pink" }
    else if 0.48 <= t && t <= 0.84 { "blue" }
    else { match glade { GladeType::Autumn => "orange", GladeType::Winter => "winter_day", _ => "green" } }
}

fn main() { let rows=include_str!("/Users/yuluo/Desktop/game/tiny-glade/tests/fixtures/native/toolbar-settings-background.ron");let mut n=0;for row in rows.lines().filter(|r|r.starts_with('(')){ let x:Vec<_>=row.trim().trim_start_matches('(').trim_end_matches(',').trim_end_matches(')').split(',').collect();let g=match x[0].parse::<u8>().unwrap(){0=>country_core::resources::session::GladeType::Summer,1=>country_core::resources::session::GladeType::Olden,2=>country_core::resources::session::GladeType::Flowery,3=>country_core::resources::session::GladeType::Autumn,4=>country_core::resources::session::GladeType::Winter,_=>unreachable!()};let b=x[1].parse::<u32>().unwrap();let expected=x[2].trim_matches('"');assert_eq!(settings_background(g,f32::from_bits(b)),expected,"glade={}, bits={b:08x}",x[0]);n+=1;}println!("{n} native background selectors passed");}
