//! Scalar presentation primitive. Retarget from current display, never old start.
//! This is not a topology/mesh morphing implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidMotion;

#[derive(Debug, Clone, Copy)]
pub struct SmoothValue {
    current: f64,
    target: f64,
    half_life: f64,
}

impl SmoothValue {
    pub fn new(initial: f64, half_life: f64) -> Result<Self, InvalidMotion> {
        if !initial.is_finite() || !half_life.is_finite() || half_life <= 0.0 {
            return Err(InvalidMotion);
        }
        Ok(Self {
            current: initial,
            target: initial,
            half_life,
        })
    }
    pub fn current(self) -> f64 {
        self.current
    }
    pub fn target(self) -> f64 {
        self.target
    }
    pub fn retarget(&mut self, target: f64) -> Result<(), InvalidMotion> {
        if !target.is_finite() {
            return Err(InvalidMotion);
        }
        self.target = target;
        Ok(())
    }
    /// Exponential ease-out with frame-rate independent decay. Value-continuous,
    /// not velocity-continuous: springs are a future option if art requires C1.
    pub fn advance(&mut self, seconds: f64) -> Result<f64, InvalidMotion> {
        if !seconds.is_finite() || seconds < 0.0 {
            return Err(InvalidMotion);
        }
        let remaining = (-seconds / self.half_life).exp2();
        self.current = self.current * remaining + self.target * (1.0 - remaining);
        Ok(self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn retarget_keeps_display_value_and_updates_toward_new_target() {
        let mut value = SmoothValue::new(0.0, 0.1).unwrap();
        value.retarget(10.0).unwrap();
        value.advance(0.1).unwrap();
        let displayed = value.current();
        assert_eq!(displayed, 5.0);
        value.retarget(2.0).unwrap();
        assert_eq!(value.current(), displayed);
        assert_eq!(value.advance(0.1).unwrap(), 3.5);
    }
    #[test]
    fn equivalent_elapsed_time_matches_across_frame_rates() {
        let mut a = SmoothValue::new(0.0, 0.1).unwrap();
        a.retarget(10.0).unwrap();
        let mut b = a;
        a.advance(0.3).unwrap();
        for _ in 0..30 {
            b.advance(0.01).unwrap();
        }
        assert!((a.current() - b.current()).abs() < 1e-12);
        assert!(b.advance(f64::NAN).is_err());
        assert!(b.retarget(f64::INFINITY).is_err());
    }
}
