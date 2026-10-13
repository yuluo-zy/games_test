use super::{Curve, CurveError, CurveMetric, CurvePoint};

fn count_from_spacing(length: f32, spacing: f32) -> usize {
    assert!(spacing != 0.0, "zero curve spacing");
    let step = (spacing / length.max(1e-8)).min(1.0);
    ((1.0 / step).round() as usize)
        .checked_add(1)
        .expect("curve sample count overflow")
        .max(2)
}
impl<T: CurvePoint, S: CurveMetric<T>> Curve<T, S> {
    /// Global normalized spacing, with the original furthest-vertex policy for
    /// returning paths. A corner can replace a sample to avoid losing a fold.
    pub fn try_resample(&self, spacing: f32, reject_duplicates: bool) -> Result<Self, CurveError> {
        assert!(self.points.len() >= 2 && self.length != 0.0);
        self.try_resample_approx_pt_count(
            count_from_spacing(self.length, spacing),
            reject_duplicates,
        )
    }
    pub fn crashy_resample(&self, spacing: f32) -> Self {
        self.try_resample(spacing, false).unwrap()
    }
    pub fn try_resample_approx_pt_count(
        &self,
        count: usize,
        reject_duplicates: bool,
    ) -> Result<Self, CurveError> {
        assert!(count > 1 && self.points.len() >= 2);
        let step = 1.0 / (count as f32 - 1.0);
        let limit = -0.5 * step + 1.0;
        let mut output = Vec::with_capacity(count);
        output.push(self.points[0]);
        let mut candidate = self.points[0];
        let mut furthest = 0.0;
        let mut available = 0.0;
        let mut progress = 0.0;
        for i in 1..self.points.len() {
            let a = self.points[i - 1];
            let b = self.points[i];
            let ua = self.points_u[i - 1];
            let ub = self.points_u[i];
            available = (available + (ub - ua)).min(limit);
            while step <= available - progress {
                progress += step;
                let distance = (b - *output.last().unwrap()).metric_squared().sqrt();
                if distance <= furthest {
                    if furthest > 0.0 {
                        output.push(candidate);
                    }
                } else {
                    let t = super::clamp01((progress - ua) / (ub - ua));
                    candidate = a * (1.0 - t) + b * t;
                    output.push(candidate);
                }
                furthest = 0.0;
            }
            let distance = (b - *output.last().unwrap()).metric_squared().sqrt();
            if distance > furthest {
                candidate = b;
                furthest = distance;
            }
        }
        let end = *self.points.last().unwrap();
        let distance = (end - *output.last().unwrap()).metric_squared().sqrt();
        if distance <= 0.0 {
            if furthest > distance {
                output.push(candidate);
                assert!((candidate - end).metric_squared().sqrt() > 0.0);
                output.push(end);
            }
        } else {
            output.push(end);
        }
        Self::try_new_from_points(output, reject_duplicates)
    }
    pub fn crashy_resample_approx_pt_count(&self, count: usize) -> Self {
        self.try_resample_approx_pt_count(count, false).unwrap()
    }
    /// B per-segment sampling measures the geometric donor delta, even when S
    /// stores projected arc lengths, and includes each segment's last point.
    pub fn try_resample_per_segment(&self, spacing: f32) -> Result<Self, CurveError> {
        assert!(self.points.len() >= 2 && spacing != 0.0 && self.length != 0.0);
        let mut points = Vec::with_capacity(self.points.len());
        points.push(self.points[0]);
        for pair in self.points.windows(2) {
            let count = count_from_spacing((pair[1] - pair[0]).metric_squared().sqrt(), spacing);
            for i in 1..count {
                let t = i as f32 / (count as f32 - 1.0);
                points.push(pair[1] * t + pair[0] * (1.0 - t));
            }
        }
        Self::try_new_from_points(points, false)
    }
    pub fn crashy_resample_per_segment(&self, spacing: f32) -> Self {
        self.try_resample_per_segment(spacing).unwrap()
    }
    pub fn try_resample_remap<U: CurvePoint, M: CurveMetric<U>>(
        &self,
        spacing: f32,
        mut remap: impl FnMut(T) -> U,
    ) -> Result<Curve<U, M>, CurveError> {
        let sampled = self.try_resample(spacing, false)?;
        Curve::try_new_from_points(sampled.points.into_iter().map(&mut remap).collect(), false)
    }
    pub fn try_resample_approx_pt_count_remap<U: CurvePoint, M: CurveMetric<U>>(
        &self,
        count: usize,
        mut remap: impl FnMut(T) -> U,
    ) -> Result<Curve<U, M>, CurveError> {
        let sampled = self.try_resample_approx_pt_count(count, false)?;
        Curve::try_new_from_points(sampled.points.into_iter().map(&mut remap).collect(), false)
    }
    pub fn try_resample_remap_per_segment<U: CurvePoint, M: CurveMetric<U>>(
        &self,
        spacing: f32,
        mut remap: impl FnMut(usize, f32, T) -> U,
    ) -> Result<Curve<U, M>, CurveError> {
        self.try_resample_remap_per_segment_2(spacing, false, &mut remap)
    }
    /// B `_2` specialization optionally forces an odd point count per segment,
    /// ensuring a midpoint is emitted. Caller remaps provide height policies.
    pub fn try_resample_remap_per_segment_2<U: CurvePoint, M: CurveMetric<U>>(
        &self,
        spacing: f32,
        ensure_midpoint: bool,
        mut remap: impl FnMut(usize, f32, T) -> U,
    ) -> Result<Curve<U, M>, CurveError> {
        assert!(self.points.len() >= 2 && spacing != 0.0 && self.length != 0.0);
        let mut points = Vec::with_capacity(self.points.len());
        points.push(remap(0, 0.0, self.points[0]));
        for (segment, pair) in self.points.windows(2).enumerate() {
            let count = count_from_spacing((pair[1] - pair[0]).metric_squared().sqrt(), spacing);
            let count = if ensure_midpoint { count | 1 } else { count };
            for i in 1..count {
                let t = i as f32 / (count as f32 - 1.0);
                points.push(remap(segment, t, pair[1] * t + pair[0] * (1.0 - t)));
            }
        }
        Curve::try_new_from_points(points, false)
    }
    pub fn crashy_resample_remap<U: CurvePoint, M: CurveMetric<U>>(
        &self,
        spacing: f32,
        remap: impl FnMut(T) -> U,
    ) -> Curve<U, M> {
        self.try_resample_remap(spacing, remap).unwrap()
    }
    pub fn crashy_resample_approx_pt_count_remap<U: CurvePoint, M: CurveMetric<U>>(
        &self,
        count: usize,
        remap: impl FnMut(T) -> U,
    ) -> Curve<U, M> {
        self.try_resample_approx_pt_count_remap(count, remap)
            .unwrap()
    }
    pub fn crashy_resample_remap_per_segment<U: CurvePoint, M: CurveMetric<U>>(
        &self,
        spacing: f32,
        remap: impl FnMut(usize, f32, T) -> U,
    ) -> Curve<U, M> {
        self.try_resample_remap_per_segment(spacing, remap).unwrap()
    }
    pub fn crashy_resample_remap_per_segment_2<U: CurvePoint, M: CurveMetric<U>>(
        &self,
        spacing: f32,
        ensure_midpoint: bool,
        remap: impl FnMut(usize, f32, T) -> U,
    ) -> Curve<U, M> {
        self.try_resample_remap_per_segment_2(spacing, ensure_midpoint, remap)
            .unwrap()
    }
}
