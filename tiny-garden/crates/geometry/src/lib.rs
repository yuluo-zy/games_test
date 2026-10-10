//! Engine-independent geometry. Units are meters in building-local X/Z space.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub z: f64,
    pub width: f64,
    pub depth: f64,
}

impl Rect {
    pub fn valid(self) -> bool {
        [
            self.x,
            self.z,
            self.width,
            self.depth,
            self.right(),
            self.back(),
        ]
        .into_iter()
        .all(f64::is_finite)
            && self.width > 0.0
            && self.depth > 0.0
    }

    pub fn right(self) -> f64 {
        self.x + self.width
    }
    pub fn back(self) -> f64 {
        self.z + self.depth
    }
    pub fn area(self) -> f64 {
        self.width * self.depth
    }

    pub fn contains(self, other: Self, margin: f64) -> bool {
        other.x >= self.x + margin
            && other.z >= self.z + margin
            && other.right() <= self.right() - margin
            && other.back() <= self.back() - margin
    }

    pub fn intersects(self, other: Self) -> bool {
        self.x < other.right()
            && other.x < self.right()
            && self.z < other.back()
            && other.z < self.back()
    }

    /// Subtract one rectangle. Pieces are disjoint; touching edges have no area.
    pub fn subtract(self, other: Self) -> Vec<Self> {
        if !self.intersects(other) {
            return vec![self];
        }
        let left = self.x.max(other.x);
        let right = self.right().min(other.right());
        let front = self.z.max(other.z);
        let back = self.back().min(other.back());
        [
            Self {
                x: self.x,
                z: self.z,
                width: left - self.x,
                depth: self.depth,
            },
            Self {
                x: right,
                z: self.z,
                width: self.right() - right,
                depth: self.depth,
            },
            Self {
                x: left,
                z: self.z,
                width: right - left,
                depth: front - self.z,
            },
            Self {
                x: left,
                z: back,
                width: right - left,
                depth: self.back() - back,
            },
        ]
        .into_iter()
        .filter(|r| r.width > 0.0 && r.depth > 0.0)
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clipped_subtraction_handles_outside_and_touching_cutters() {
        let outer = Rect {
            x: 0.0,
            z: 0.0,
            width: 10.0,
            depth: 8.0,
        };
        for x in [-5.0, 0.0, 2.0, 9.0, 10.0] {
            for z in [-5.0, 0.0, 2.0, 7.0, 8.0] {
                let cutter = Rect {
                    x,
                    z,
                    width: 4.0,
                    depth: 3.0,
                };
                let overlap = (outer.right().min(cutter.right()) - outer.x.max(cutter.x)).max(0.0)
                    * (outer.back().min(cutter.back()) - outer.z.max(cutter.z)).max(0.0);
                let pieces = outer.subtract(cutter);
                assert!(
                    (pieces.iter().map(|r| r.area()).sum::<f64>() + overlap - outer.area()).abs()
                        < 1e-9
                );
                assert!(pieces.len() <= 4);
                for (i, a) in pieces.iter().enumerate() {
                    assert!(a.valid() && outer.contains(*a, 0.0));
                    assert!(!a.intersects(cutter));
                    for b in &pieces[i + 1..] {
                        assert!(!a.intersects(*b));
                    }
                }
            }
        }
    }
    #[test]
    fn subtraction_preserves_area_without_overlapping() {
        let outer = Rect {
            x: 0.0,
            z: 0.0,
            width: 10.0,
            depth: 8.0,
        };
        let inner = Rect {
            x: 2.0,
            z: 2.0,
            width: 4.0,
            depth: 3.0,
        };
        let parts = outer.subtract(inner);
        assert_eq!(parts.iter().map(|r| r.area()).sum::<f64>(), 68.0);
        for (i, a) in parts.iter().enumerate() {
            assert!(!a.intersects(inner));
            for b in &parts[i + 1..] {
                assert!(!a.intersects(*b));
            }
        }
        assert!(outer.subtract(outer).is_empty());
    }
}
pub mod linear;
