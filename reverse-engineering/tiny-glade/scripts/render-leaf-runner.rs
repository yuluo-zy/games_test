#[path = "../reconstruction/render/resource_usage.rs"]
mod resource_usage;
use std::io::{self, BufRead, Write};
fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let mut out = io::BufWriter::new(io::stdout().lock());
    for line in io::stdin().lock().lines() {
        let text = line.unwrap();
        let mut words = text.split_whitespace();
        let kind = words.next().unwrap();
        let value = u32::from_str_radix(words.next().unwrap(), 16).unwrap();
        match kind {
            "b" => writeln!(out, "{:x}", resource_usage::buffer_access_mask_to_usage_flags(value).as_raw()).unwrap(),
            "i" => match std::panic::catch_unwind(|| resource_usage::image_access_mask_to_usage_flags(value)) {
                Ok(value) => writeln!(out, "{:x}", value.as_raw()).unwrap(),
                Err(_) => writeln!(out, "panic").unwrap(),
            },
            _ => panic!("bad mode"),
        }
    }
}
