use taffy::style::Style;
fn main(){ macro_rules! field{($($f:ident),*)=>{$(println!("{} {:x}",stringify!($f),std::mem::offset_of!(Style,$f));)*}} field!(display,position,flex_direction,flex_wrap,align_items,align_self,align_content,justify_content,inset,margin,padding,border,gap,flex_grow,flex_shrink,flex_basis,size,min_size,max_size,aspect_ratio);}
