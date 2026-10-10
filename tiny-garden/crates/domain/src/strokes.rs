//! Authoritative wall/path strokes. Geometry and engine handles are derived.
use crate::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrokeId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StrokeKind {
    Wall,
    Path,
    Fence,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_nonfinite_short_outside_and_unbounded_strokes() {
        let valid = Stroke {
            id: StrokeId(1),
            kind: StrokeKind::Wall,
            points: vec![Point { x: 0., z: 0. }, Point { x: 4., z: 0. }],
            width: 0.4,
            height: 2.4,
        };
        valid.validate().unwrap();
        let mut s = valid.clone();
        s.points[0].x = f64::NAN;
        assert!(s.validate().is_err());
        let mut s = valid.clone();
        s.width = f64::INFINITY;
        assert!(s.validate().is_err());
        let mut s = valid.clone();
        s.points[1].x = 0.01;
        assert!(s.validate().is_err());
        let mut s = valid.clone();
        s.points[1].x = 25.;
        s.width = 3.;
        assert!(s.validate().is_err());
        let mut s = valid.clone();
        s.points = vec![Point { x: 0., z: 0. }; 129];
        assert!(s.validate().is_err());
        let mut s = valid;
        s.id = StrokeId(0);
        assert!(s.validate().is_err());
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub z: f64,
}
impl Point {
    pub fn distance(self, other: Self) -> f64 {
        (self.x - other.x).hypot(self.z - other.z)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    pub id: StrokeId,
    pub kind: StrokeKind,
    pub points: Vec<Point>,
    pub width: f64,
    pub height: f64,
}
impl Stroke {
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.id.0 == 0
            || self.id.0 >= (1 << 60)
            || !(2..=128).contains(&self.points.len())
            || !(0.25..=3.).contains(&self.width)
            || !(0.0..=5.).contains(&self.height)
            || self.points.iter().any(|p| {
                !p.x.is_finite()
                    || !p.z.is_finite()
                    || !(-25.0..=25.0).contains(&p.x)
                    || !(-15.5..=15.5).contains(&p.z)
            })
        {
            return Err(DomainError("invalid stroke"));
        }
        let lengths = self
            .points
            .windows(2)
            .map(|p| p[0].distance(p[1]))
            .collect::<Vec<_>>();
        if lengths.iter().any(|l| *l < 0.04) || lengths.iter().sum::<f64>() > 60. {
            return Err(DomainError("invalid stroke length"));
        }
        let b = self.bounds();
        if b[0] < -25.5 || b[2] > 25.5 || b[1] < -16. || b[3] > 16. {
            return Err(DomainError("invalid stroke"));
        }
        Ok(())
    }
    pub fn bounds(&self) -> [f64; 4] {
        let mut b = [
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ];
        for p in &self.points {
            b[0] = b[0].min(p.x);
            b[1] = b[1].min(p.z);
            b[2] = b[2].max(p.x);
            b[3] = b[3].max(p.z);
        }
        let margin = self.width * 0.5 + 0.3;
        [b[0] - margin, b[1] - margin, b[2] + margin, b[3] + margin]
    }
    pub fn affects(&self, other: &Self) -> bool {
        let a = self.bounds();
        let b = other.bounds();
        a[0] <= b[2] && b[0] <= a[2] && a[1] <= b[3] && b[1] <= a[3]
    }
}
