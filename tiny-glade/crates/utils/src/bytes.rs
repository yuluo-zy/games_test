//! Explicit-endian scalar byte access.

pub fn read_f32_le(bytes: &[u8], offset: usize) -> f32 {
    f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

pub fn read_f32_ne(bytes: &[u8], offset: usize) -> f32 {
    f32::from_ne_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

pub fn write_f32_le(bytes: &mut [u8], offset: usize, value: f32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

pub fn write_f32_ne(bytes: &mut [u8], offset: usize, value: f32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
}

pub fn write_u64_le(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn little_endian_f32_round_trip() {
        let mut bytes = [0u8; 8];
        write_f32_le(&mut bytes, 2, 1.5);
        assert_eq!(read_f32_le(&bytes, 2), 1.5);
        assert_eq!(&bytes[2..6], &1.5f32.to_le_bytes());
    }

    #[test]
    fn native_endian_f32_round_trip() {
        let mut bytes = [0u8; 4];
        write_f32_ne(&mut bytes, 0, -2.25);
        assert_eq!(read_f32_ne(&bytes, 0), -2.25);
    }

    #[test]
    fn little_endian_u64_write() {
        let mut bytes = [0u8; 8];
        write_u64_le(&mut bytes, 0, 0x0123_4567_89ab_cdef);
        assert_eq!(bytes, 0x0123_4567_89ab_cdefu64.to_le_bytes());
    }

    #[test]
    #[should_panic]
    fn out_of_range_read_panics() {
        let bytes = [0u8; 3];
        let _ = read_f32_le(&bytes, 0);
    }
}
