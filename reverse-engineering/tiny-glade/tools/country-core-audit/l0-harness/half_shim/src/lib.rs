//! 测试台替身：只含 f16 <-> f32 位转换（IEEE binary16，round-to-nearest-even）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[allow(non_camel_case_types)]
pub struct f16(u16);
impl f16 {
    pub fn from_bits(b: u16) -> Self { f16(b) }
    pub fn to_bits(self) -> u16 { self.0 }
    pub fn from_f32(v: f32) -> Self {
        let x = v.to_bits();
        let sign = ((x >> 16) & 0x8000) as u16;
        let exp = ((x >> 23) & 0xff) as i32;
        let man = x & 0x7f_ffff;
        if exp == 0xff { return f16(sign | 0x7c00 | if man != 0 { 0x200 } else { 0 }); }
        let e = exp - 127 + 15;
        if e >= 0x1f { return f16(sign | 0x7c00); }
        if e <= 0 {
            if e < -10 { return f16(sign); }
            let m = man | 0x80_0000;
            let shift = (14 - e) as u32;
            let mut h = (m >> shift) as u16;
            let rem = m & ((1 << shift) - 1);
            let half = 1 << (shift - 1);
            if rem > half || (rem == half && (h & 1) == 1) { h += 1; }
            return f16(sign | h);
        }
        let mut h = sign | ((e as u16) << 10) | (man >> 13) as u16;
        let rem = man & 0x1fff;
        if rem > 0x1000 || (rem == 0x1000 && (h & 1) == 1) { h += 1; }
        f16(h)
    }
    pub fn to_f32(self) -> f32 {
        let h = self.0 as u32;
        let sign = (h & 0x8000) << 16;
        let exp = (h >> 10) & 0x1f;
        let man = h & 0x3ff;
        let bits = if exp == 0 {
            if man == 0 { sign } else {
                let mut e = 127 - 15 + 1; let mut m = man;
                while m & 0x400 == 0 { m <<= 1; e -= 1; }
                sign | ((e as u32) << 23) | ((m & 0x3ff) << 13)
            }
        } else if exp == 0x1f { sign | 0x7f80_0000 | (man << 13) } else { sign | ((exp + 112) << 23) | (man << 13) };
        f32::from_bits(bits)
    }
}
