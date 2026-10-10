use tiny_glade_reconstruction::{roof_state, window_aspect, surface, ridge, roof_pipeline, SOURCE_SHA256};

fn roof_bytes(s: &str) -> [u8; 88] {
    assert_eq!(s.len(), 176, "Roof input must contain 88 bytes as hex");
    std::array::from_fn(|i| u8::from_str_radix(&s[i*2..i*2+2],16).expect("invalid hex"))
}
fn point_bits(p: [f32; 3]) -> String {
    format!("[\"{:08x}\",\"{:08x}\",\"{:08x}\"]",p[0].to_bits(),p[1].to_bits(),p[2].to_bits())
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("aspect") if args.len() == 3 => {
            let a = args[1].parse::<u32>().expect("first value must be u32");
            let b = args[2].parse::<u32>().expect("second value must be u32");
            let result = window_aspect::aspect_ratio_observed(a, b);
            println!("{{\"value\":\"{result}\",\"f32_bits\":\"{:08x}\"}}", result.to_bits());
        }
        Some("roof-state") if args.len() == 3 => {
            let a = args[1].parse::<f32>().expect("value at +0x30 must be f32");
            let b = args[2].parse::<f32>().expect("value at +0x34 must be f32");
            println!("{{\"ty_byte\":{},\"is_gable\":{}}}", roof_state::ty_observed(a) as u8,
                     roof_state::is_gable_observed(a, b));
        }
        Some("identity") => println!("{{\"source_sha256\":\"{SOURCE_SHA256}\"}}"),
        Some("roof-section") if args.len() == 3 => {
            let roof=roof_bytes(&args[1]);
            let t=args[2].parse::<f32>().expect("section parameter must be f32");
            let mut out=[0xa5;28];
            surface::get_roof_shape(&roof,t,&mut out);
            println!("{{\"shape_bytes\":\"{}\"}}",out.iter().map(|b|format!("{b:02x}")).collect::<String>());
        }
        Some("roof-ridge") if args.len() == 2 => {
            let roof=roof_bytes(&args[1]);
            let dims=ridge::ridge_dims_from_observed(&roof);
            match ridge::ridge_to_world_space_observed(dims,&roof) {
                Ok(ridge::RidgeGeometry::Point(p)) => println!("{{\"type\":\"point\",\"f32_bits\":{}}}",point_bits(p)),
                Ok(ridge::RidgeGeometry::Segment(p)) => println!("{{\"type\":\"segment\",\"f32_bits\":[{},{}]}}",point_bits(p[0]),point_bits(p[1])),
                Err(e) => println!("{{\"error\":\"{e:?}\"}}"),
            }
        }
        Some("roof-profile") if args.len() == 5 => {
            let values: Vec<f32>=args[1..].iter().map(|s|s.parse().expect("profile arguments must be f32")).collect();
            let points=surface::profile_curve_ws_points(&surface::profile_curve_normalized(values[0]),values[1],values[2],values[3]);
            let bits:Vec<_>=points.iter().map(|p|format!("[\"{:08x}\",\"{:08x}\"]",p[0].to_bits(),p[1].to_bits())).collect();
            println!("{{\"point_f32_bits\":[{}]}}",bits.join(","));
        }
        Some("roof-tiles") if (4..=5).contains(&args.len()) => {
            let roof=roof_bytes(&args[1]);
            let rect=if args[2]=="-" {None} else {
                let s=&args[2];assert_eq!(s.len(),48,"rectangle must contain 24 bytes as hex");
                Some(std::array::from_fn::<_,24,_>(|i|u8::from_str_radix(&s[2*i..2*i+2],16).unwrap()))
            };
            let batch=roof_pipeline::assemble_roof_tiles_observed(&roof,rect.as_ref(),args[3]=="1").expect("invalid roof input");
            if let Some(path)=args.get(4) {
                let bytes:Vec<u8>=batch.records.iter().flat_map(|r|r.0).collect();
                std::fs::write(path,bytes).expect("unable to save packed tile records");
            }
            println!("{{\"records\":{},\"record_size\":64,\"edges\":{},\"ridge_caps\":{},\"faces_and_fillers\":{}}}",
              batch.records.len(),batch.edges,batch.ridge_caps,batch.faces_and_fillers);
        }
        _ => {
            println!("Tiny Glade reconstruction: current verified algorithm runner");
            println!("Commands: identity | aspect <u32> <u32> | roof-state <f32> <f32>");
            println!("roof-section <88-byte-hex> <t> | roof-ridge <88-byte-hex> | roof-profile <profile> <bottom> <top> <height>");
            println!("roof-tiles <88-byte-hex> <24-byte-rectangle-hex or -> <mode 0/1> [packed-record-output]");
        }
    }
}
