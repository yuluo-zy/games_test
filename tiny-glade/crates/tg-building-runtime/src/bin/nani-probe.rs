//! Line-oriented roof projection probe: 88-byte roof hex and animation bits.
use std::io::{self, BufRead};
use tg_building_runtime::nani::{StoneFloorCpuProjection, project_roof_stone_floor};

fn project_line(line: &str) -> Result<StoneFloorCpuProjection, String> {
    let mut fields = line.split_whitespace();
    let roof_hex = fields.next().ok_or("Missing roof hex")?;
    let animation_hex = fields.next().ok_or("Missing animation bits")?;
    if fields.next().is_some() {
        return Err("Expected exactly roof hex and animation bits".into());
    }
    if roof_hex.len() != 176 || !roof_hex.is_ascii() {
        return Err("Roof must contain exactly 176 ASCII hex digits".into());
    }
    if animation_hex.len() != 8 || !animation_hex.is_ascii() {
        return Err("Animation must contain exactly 8 ASCII hex digits".into());
    }
    let mut roof = [0; 88];
    for (target, pair) in roof.iter_mut().zip(roof_hex.as_bytes().as_chunks::<2>().0) {
        let pair = std::str::from_utf8(pair).map_err(|error| error.to_string())?;
        *target = u8::from_str_radix(pair, 16).map_err(|error| error.to_string())?;
    }
    let animation =
        f32::from_bits(u32::from_str_radix(animation_hex, 16).map_err(|error| error.to_string())?);
    project_roof_stone_floor(&roof, animation)
        .map_err(|error| format!("Roof projection: {error:?}"))
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    for (index, line) in io::stdin().lock().lines().enumerate() {
        let result =
            project_line(&line?).map_err(|error| format!("Line {}: {error}", index + 1))?;
        let record: String = result.record.iter().map(|x| format!("{x:02x}")).collect();
        let bounds: String = result
            .bounds_center
            .into_iter()
            .chain([result.bounds_radius])
            .flat_map(|x| x.to_le_bytes())
            .map(|x| format!("{x:02x}"))
            .collect();
        println!("{record} {bounds}");
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::project_line;

    #[test]
    fn malformed_probe_lines_return_errors() {
        let valid = format!("{} 00000000", "00".repeat(88));
        assert!(project_line(&valid).is_ok());
        for invalid in [
            String::new(),
            "00 00000000".into(),
            format!("{} 00000000", "é".repeat(88)),
            format!("{} 00000000", "gg".repeat(88)),
            format!("{} xyz", "00".repeat(88)),
            format!("{valid} extra"),
        ] {
            assert!(project_line(&invalid).is_err(), "{invalid}");
        }
    }
}
