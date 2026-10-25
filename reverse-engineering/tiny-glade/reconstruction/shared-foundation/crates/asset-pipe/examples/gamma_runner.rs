use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let bits = u32::from_str_radix(&line.unwrap(), 16).unwrap();
        println!(
            "{:08x}",
            asset_pipe::texture::srgb_oetf(f32::from_bits(bits)).to_bits()
        );
    }
}
