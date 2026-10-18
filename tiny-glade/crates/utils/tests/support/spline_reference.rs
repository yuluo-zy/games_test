//! Independent f64 reference: solve the full second-derivative system by pivoted elimination.
use glam::Vec3;

pub struct ReferenceSpline {
    pub lengths: Vec<f64>,
    coefficients: Vec<[[f64; 3]; 4]>,
}

fn solve(mut rows: Vec<Vec<f64>>) -> Vec<f64> {
    let n = rows.len();
    for column in 0..n {
        let pivot = (column..n)
            .max_by(|&a, &b| rows[a][column].abs().total_cmp(&rows[b][column].abs()))
            .unwrap();
        rows.swap(column, pivot);
        let diagonal = rows[column][column];
        assert!(diagonal.abs() > 1e-12);
        for entry in &mut rows[column][column..=n] {
            *entry /= diagonal;
        }
        let (before, pivot_and_after) = rows.split_at_mut(column);
        let (pivot, after) = pivot_and_after.split_first_mut().unwrap();
        for row in before.iter_mut().chain(after) {
            let factor = row[column];
            for (entry, pivot_entry) in row[column..=n].iter_mut().zip(&pivot[column..=n]) {
                *entry -= factor * pivot_entry;
            }
        }
    }
    rows.into_iter().map(|row| row[n]).collect()
}

impl ReferenceSpline {
    pub fn new(points: &[Vec3], periodic: bool, start: Option<Vec3>, end: Option<Vec3>) -> Self {
        let points: Vec<_> = points.iter().map(|p| p.to_array().map(f64::from)).collect();
        let n = points.len();
        let segments = if periodic { n } else { n - 1 };
        let lengths: Vec<_> = (0..segments)
            .map(|i| {
                let next = (i + 1) % n;
                (0..3)
                    .map(|axis| (points[next][axis] - points[i][axis]).powi(2))
                    .sum::<f64>()
                    .sqrt()
            })
            .collect();
        let second_derivatives: [Vec<f64>; 3] = std::array::from_fn(|axis| {
            let mut rows = vec![vec![0.0; n + 1]; n];
            let slope = |i: usize| (points[(i + 1) % n][axis] - points[i][axis]) / lengths[i];
            for (i, row) in rows.iter_mut().enumerate() {
                if !periodic && i == 0 {
                    if let Some(tangent) = start {
                        row[0] = 2.0 * lengths[0];
                        row[1] = lengths[0];
                        row[n] = 6.0 * (slope(0) - f64::from(tangent[axis]));
                    } else {
                        row[0] = 1.0;
                    }
                } else if !periodic && i + 1 == n {
                    if let Some(tangent) = end {
                        row[n - 2] = lengths[n - 2];
                        row[n - 1] = 2.0 * lengths[n - 2];
                        row[n] = 6.0 * (f64::from(tangent[axis]) - slope(n - 2));
                    } else {
                        row[n - 1] = 1.0;
                    }
                } else {
                    let previous = (i + n - 1) % n;
                    row[previous] += lengths[previous];
                    row[i] += 2.0 * (lengths[previous] + lengths[i]);
                    row[(i + 1) % n] += lengths[i];
                    row[n] = 6.0 * (slope(i) - slope(previous));
                }
            }
            solve(rows)
        });
        let coefficients = (0..segments)
            .map(|i| {
                let next = (i + 1) % n;
                let h = lengths[i];
                [
                    points[i],
                    std::array::from_fn(|axis| {
                        (points[next][axis] - points[i][axis]) / h
                            - h * (2.0 * second_derivatives[axis][i]
                                + second_derivatives[axis][next])
                                / 6.0
                    }),
                    std::array::from_fn(|axis| second_derivatives[axis][i] / 2.0),
                    std::array::from_fn(|axis| {
                        (second_derivatives[axis][next] - second_derivatives[axis][i]) / (6.0 * h)
                    }),
                ]
            })
            .collect();
        Self {
            lengths,
            coefficients,
        }
    }

    pub fn position(&self, segment: usize, local_t: f64) -> Vec3 {
        let c = self.coefficients[segment];
        Vec3::from_array(std::array::from_fn(|axis| {
            (((c[3][axis] * local_t + c[2][axis]) * local_t + c[1][axis]) * local_t + c[0][axis])
                as f32
        }))
    }

    pub fn tangent(&self, segment: usize, local_t: f64) -> Vec3 {
        let c = self.coefficients[segment];
        Vec3::from_array(std::array::from_fn(|axis| {
            ((3.0 * c[3][axis] * local_t + 2.0 * c[2][axis]) * local_t + c[1][axis]) as f32
        }))
    }
}
