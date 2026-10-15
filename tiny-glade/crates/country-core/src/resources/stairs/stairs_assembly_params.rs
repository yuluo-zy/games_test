//! B stairs assembly geometry leaves. The full assembly-request owner and its
//! graph-to-geometry producer are separate; no missing request fields default.
use glam::Vec2;

/// B 0x140959e80 returns the supplied packed Vec2 and angle unchanged.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StairWallAnchor {
    pub position: Vec2,
    pub angle: f32,
}
impl StairWallAnchor {
    pub fn new(position: Vec2, angle: f32) -> Self {
        Self { position, angle }
    }
    /// B 0x140959e90 calls sinf then cosf and XORs both results with 0x80000000.
    /// The old chapter omitted those flips. These point into the wall.
    pub fn wall_normal(self) -> Vec2 {
        let x = f32::from_bits(self.angle.sin().to_bits() ^ 0x8000_0000);
        let y = f32::from_bits(self.angle.cos().to_bits() ^ 0x8000_0000);
        Vec2::new(x, y)
    }
}

/// Reads only the confirmed StairNodeAnchor::Wall payload. Retaining a slice
/// keeps the other variants and their still-unresolved payload layout intact.
pub struct ObservedStairNodeAnchor<'a>(&'a [u8]);
impl<'a> ObservedStairNodeAnchor<'a> {
    pub fn from_observed(bytes: &'a [u8]) -> Self {
        assert!(
            bytes.len() >= 4,
            "StairNodeAnchor requires its discriminant"
        );
        Self(bytes)
    }
    pub fn try_as_wall(&self) -> Option<StairWallAnchor> {
        // B 0x140959ef0 compares the four-byte discriminant to 2 and returns +8.
        if u32::from_le_bytes(self.0[..4].try_into().unwrap()) != 2 {
            return None;
        }
        assert!(
            self.0.len() >= 20,
            "StairNodeAnchor::Wall requires its payload"
        );
        Some(StairWallAnchor::new(
            Vec2::new(read(self.0, 8), read(self.0, 12)),
            read(self.0, 16),
        ))
    }
}

/// The confirmed 20 bytes read by AlignKind's two geometry methods. Variant
/// names are unproven, so the original tag and boolean bytes remain explicit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObservedAlignKind([u8; 20]);
impl ObservedAlignKind {
    pub fn from_observed(bytes: [u8; 20]) -> Self {
        Self(bytes)
    }
    pub fn observed(&self) -> &[u8; 20] {
        &self.0
    }
    /// B 0x140959f00. The f32 scale argument is in XMM1; it was omitted by the
    /// old failed decompilation. Unsupported variants return the original zero.
    pub fn position_offset(&self, scale: f32) -> Vec2 {
        match self.0[0] {
            0 if self.0[1] != 0 || self.0[2] == 1 => self.vec2(4) * scale,
            1 => {
                let mut direction = self.vec2(4);
                if self.0[1] != 0 {
                    direction += self.vec2(12);
                }
                direction * (scale * 0.5)
            }
            _ => Vec2::ZERO,
        }
    }
    /// B 0x140959f50 returns both scalar result registers, not one u32.
    pub fn outgoing_dir(&self) -> Vec2 {
        match self.0[0] {
            0 => self.vec2(4),
            1 => self.vec2(12),
            _ => panic!("AlignKind has no outgoing direction"),
        }
    }
    fn vec2(&self, offset: usize) -> Vec2 {
        Vec2::new(read(&self.0, offset), read(&self.0, offset + 4))
    }
}

/// B PlayermadeStairsAssemblyParams::length_ws (0x140959ca0). Its actual path
/// points and per-point heights are zipped; differing lengths stop at the
/// shorter source, and each neighboring pair contributes one 3D segment.
pub fn length_ws(points: &[Vec2], heights: &[f32]) -> f32 {
    let mut path = points.iter().zip(heights);
    let Some((first, &height)) = path.next() else {
        return 0.0;
    };
    let mut previous = [first.x, height, first.y];
    let mut total = 0.0;
    for (point, &height) in path {
        let next = [point.x, height, point.y];
        let x = previous[0] - next[0];
        let y = previous[1] - next[1];
        let z = previous[2] - next[2];
        total += (z * z + x * x + y * y).sqrt();
        previous = next;
    }
    total
}
fn read(bytes: &[u8], offset: usize) -> f32 {
    f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}
