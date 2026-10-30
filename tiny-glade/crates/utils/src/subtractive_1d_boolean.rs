//! 原 B 141fdc2d0/141fdc350 区间减法；保留无序浮点和端点分支。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval {
    pub start: f32,
    pub end: f32,
    pub upper_exclusive: bool,
}
#[derive(Clone, Debug)]
pub struct SubtractiveBoolean1D {
    ranges: Vec<Interval>,
}
impl SubtractiveBoolean1D {
    /// 原入口直接保存边界，不额外验证顺序。
    pub fn new_range(start: f32, end: f32) -> Self {
        Self {
            ranges: vec![Interval {
                start,
                end,
                upper_exclusive: false,
            }],
        }
    }
    pub fn result(&self) -> &[Interval] {
        &self.ranges
    }
    /// 按原 UCOMISS carry/zero 分支逐段扣除；无序比较也保留原去向。
    pub fn subtract(&mut self, cut: Interval) {
        // x86 UCOMISS 的 CF 在小于或无序时置位；JNC 排除 NaN。
        let below = |a: f32, b: f32| a < b || a.is_nan() || b.is_nan();
        let below_equal = |a: f32, b: f32| a <= b || a.is_nan() || b.is_nan();
        let mut result = Vec::new();
        for &range in &self.ranges {
            let a = range.start;
            let b = range.end;
            let s = cut.start;
            let e = cut.end;
            let proceed = if below(a, s) { true } else if cut.upper_exclusive {
                below_equal(e, a) || below(b, s) || !(e > b)
            } else if e >= a {
                below(b, s) || !(e >= b)
            } else { true };
            if !proceed { continue; }
            let mut use_right_test = !(s >= a);
            if !use_right_test {
                let split = if range.upper_exclusive {
                    if below_equal(b, s) { use_right_test = true; false }
                    else { !below(e, a) && b > e }
                } else {
                    let b_before_s = below(b, s);
                    if b_before_s { use_right_test = true; false }
                    else { !below(e, a) && !below(b, e) }
                };
                if split {
                    result.push(Interval { start: a, end: s, upper_exclusive: false });
                    result.push(Interval { start: e, end: b, upper_exclusive: false });
                    continue;
                }
                if !use_right_test {
                    result.push(Interval { start: a, end: s, upper_exclusive: false });
                    continue;
                }
            }
            if e >= a && if range.upper_exclusive { b > e } else { b >= e } {
                result.push(Interval { start: e, end: b, upper_exclusive: false });
            } else {
                result.push(range);
            }
        }
        self.ranges = result;
    }

}
#[cfg(test)]
#[path = "oracle/subtractive_1d_boolean_tests.rs"]
mod tests;

#[cfg(test)]
// Fixture tuples retain the captured native input/output record shape.
#[allow(clippy::type_complexity)]
#[path = "oracle/subtractive_1d_boolean_native_oracle.rs"]
mod native_oracle;
