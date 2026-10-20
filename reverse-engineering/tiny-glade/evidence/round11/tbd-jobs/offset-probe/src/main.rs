use taffy::style::*;
use std::mem::{size_of, MaybeUninit};

fn off<T, F>(base: *const T, field: *const F) -> usize { field as usize - base as usize }

fn main() {
    let s = Style::default();
    let p = &s as *const Style;
    println!("size_of Style = {}", size_of::<Style>());
    println!("display {}", off(p, &s.display));
    println!("position {}", off(p, &s.position));
    println!("flex_direction {}", off(p, &s.flex_direction));
    println!("align_items {}", off(p, &s.align_items));
    println!("justify_content {}", off(p, &s.justify_content));
    println!("inset {}", off(p, &s.inset));
    println!("size {}", off(p, &s.size));
    println!("min_size {}", off(p, &s.min_size));
    println!("max_size {}", off(p, &s.max_size));
    println!("margin {}", off(p, &s.margin));
    println!("padding {}", off(p, &s.padding));
    println!("border {}", off(p, &s.border));
    println!("gap {}", off(p, &s.gap));
    println!("size_of Rect<LengthPercentage> {}", size_of::<taffy::geometry::Rect<LengthPercentage>>());
    println!("size_of Size<Dimension> {}", size_of::<taffy::geometry::Size<Dimension>>());
    println!("size_of Rect<LengthPercentageAuto> {}", size_of::<taffy::geometry::Rect<LengthPercentageAuto>>());
}
