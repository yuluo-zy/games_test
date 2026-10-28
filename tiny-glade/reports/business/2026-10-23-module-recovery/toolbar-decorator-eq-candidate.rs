pub struct DecoratorHistoryState {
    /// +0x00：目标墙 id（DecoratorInfo.dst_wall）。
    pub dst_wall: u64,
    /// +0x08：目标次级 id。
    pub dst_secondary: u64,
    /// +0x10：目标载荷。
    pub dst_payload: u64,
    /// +0x18：墙空间数据。
    pub u18: u64,
    /// +0x20：u32（`DecoratorInfo.u20`）。
    pub u20: u32,
    /// +0x24：u32（`DecoratorInfo.u24`）。
    pub u24: u32,
    /// +0x28：u64（`DecoratorInfo.u28`）。
    pub u28: u64,
    /// +0x30：style 派生（style==4/8+ 或 <2 → `u30` low，否则 0）。
    pub u30: u32,
    /// +0x34：style==6（Window）→ `u30` low（u32），否则 0。
    pub u34: u32,
    /// +0x38：style==7（Lantern）→ `u30` low（f32），否则 1.0。
    pub u38: u32,
    /// +0x3c：style 8+ 或 <2 → `u30` high（f32），否则 1.0。
    pub u3c: u32,
    /// +0x40：style 8+ 或 <2 → `u38`，否则 0。
    pub u40: u32,
    /// +0x44：style 8+ 或 <2 → `u3c`，否则 0。
    pub u44: u32,
    /// +0x48：rank。
    pub rank: u8,
    /// +0x49：u45。
    pub u49: u8,
    /// +0x4a：DecoratorType。
    pub ty: u8,
    /// +0x4b：style 8+ 或 <2 → `u40` byte0，否则 0。
    pub u4b: u8,
    /// +0x4c：style==5（Flag）→ `u30` byte0（u8），否则 0。
    pub u4c: u8,
    /// +0x4d：style==4（Chimney）→ `u30` byte4（u8），否则 1。
    pub u4d: u8,
    /// +0x4e：style 字节（= `DecoratorInfo.extra_style()`，style 2..7 时为 0）。
    pub style: u8,
}
fn history_states_equal(a: &DecoratorHistoryState, b: &DecoratorHistoryState) -> bool {
    let float_eq = |x: u32, y: u32| f32::from_bits(x) == f32::from_bits(y);
    let low = |x: u64| x as u32;
    let high = |x: u64| (x >> 32) as u32;
    let a_tag = ((a.u20 >> 16) & 0xff) as u8;
    let b_tag = ((b.u20 >> 16) & 0xff) as u8;
    let variant = |tag: u8| if (2..=4).contains(&tag) { tag - 1 } else { 0 };
    if variant(a_tag) != variant(b_tag) {
        return false;
    }
    let same_destination = match a_tag {
        2 | 3 => {
            float_eq(low(a.dst_wall), low(b.dst_wall))
                && float_eq(high(a.dst_wall), high(b.dst_wall))
                && float_eq(low(a.dst_secondary), low(b.dst_secondary))
        }
        4 => {
            a.dst_wall == b.dst_wall
                && float_eq(low(a.dst_secondary), low(b.dst_secondary))
                && float_eq(high(a.dst_secondary), high(b.dst_secondary))
                && float_eq(low(a.dst_payload), low(b.dst_payload))
        }
        _ => {
            a_tag == b_tag
                && (a.u20 as u16) == (b.u20 as u16)
                && a.dst_secondary == b.dst_secondary
                && float_eq(low(a.dst_wall), low(b.dst_wall))
                && float_eq(high(a.dst_wall), high(b.dst_wall))
                && float_eq(low(a.dst_payload), low(b.dst_payload))
                && float_eq(high(a.dst_payload), high(b.dst_payload))
                && float_eq(low(a.u18), low(b.u18))
                && float_eq(high(a.u18), high(b.u18))
        }
    };
    same_destination
        && a.ty == b.ty
        && a.rank == b.rank
        && a.u49 == b.u49
        && a.u28 == b.u28
        && float_eq(a.u30, b.u30)
        && a.u34 == b.u34
        && float_eq(a.u38, b.u38)
        && float_eq(a.u3c, b.u3c)
        && a.u40 == b.u40
        && a.u44 == b.u44
        && a.u4b == b.u4b
        && a.u4c == b.u4c
        && a.u4d == b.u4d
        && a.style == b.style
}

use std::io::{self,BufRead};
fn decode(s:&str)->DecoratorHistoryState {
 let bytes:Vec<u8>=(0..s.len()).step_by(2).map(|i|u8::from_str_radix(&s[i..i+2],16).unwrap()).collect();
 let u64_at=|i|u64::from_le_bytes(bytes[i..i+8].try_into().unwrap());
 let u32_at=|i|u32::from_le_bytes(bytes[i..i+4].try_into().unwrap());
 DecoratorHistoryState {dst_wall:u64_at(0),dst_secondary:u64_at(8),dst_payload:u64_at(16),u18:u64_at(24),u20:u32_at(32),u24:u32_at(36),u28:u64_at(40),u30:u32_at(48),u34:u32_at(52),u38:u32_at(56),u3c:u32_at(60),u40:u32_at(64),u44:u32_at(68),rank:bytes[72],u49:bytes[73],ty:bytes[74],u4b:bytes[75],u4c:bytes[76],u4d:bytes[77],style:bytes[78]}
}
fn main(){for l in io::stdin().lock().lines(){let l=l.unwrap();let parts:Vec<_>=l.split_whitespace().collect();println!("{}",u8::from(history_states_equal(&decode(parts[0]),&decode(parts[1]))));}}
