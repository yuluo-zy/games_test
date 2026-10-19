//! Original material statistics, B140a88280..140a8899f. Owning ECS systems
//! borrow this record exclusively; ECS ownership replaces the lock/CAS ABI.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MaterialStats {
    /// Native28..40 and90..a8, in storage order (stairs precedes sewer).
    pub previous_lengths: [f32; 7],
    pub lengths: [f32; 7],
    pub previous_pillar_placements: Vec<[f32; 2]>,
    pub pillar_placements: Vec<[f32; 2]>,
    pub previous_fences_segments_seed: u64,
    pub fences_segments_seed: u64,
    /// Nativeac..c8: path, lintel segments/pillars, arch segments/pillars,
    /// gate segments, sewer segments, fence segments.
    pub previous_counts: [u32; 8],
    pub counts: [u32; 8],
}
impl MaterialStats {
    /// Original publisher140a8b530 moves old current half into new previous;
    /// the new current half starts empty. This happens only on row publication.
    pub fn for_new_publication(old: &mut Self) -> Self {
        Self {
            previous_lengths: old.lengths,
            previous_pillar_placements: std::mem::take(&mut old.pillar_placements),
            previous_fences_segments_seed: old.fences_segments_seed,
            previous_counts: old.counts,
            ..Self::default()
        }
    }

    /// Original resets only the current half. Advancing the previous half is a
    /// separate owner operation and must never happen implicitly here.
    pub fn init_this_update(&mut self, length: f32, special: bool) {
        self.pillar_placements = Vec::new();
        self.fences_segments_seed = 0;
        self.lengths = [0.0; 7];
        self.counts = [0; 8];
        self.lengths[if special { 3 } else { 0 }] = length;
    }
    pub fn set_pillar_placements(&mut self, placements: Vec<[f32; 2]>) {
        self.pillar_placements = placements;
    }
    pub fn set_path_segmentation_count(&mut self, n: u32) {
        self.counts[0] = n;
    }
    pub fn set_lintels_segments_count(&mut self, n: u32) {
        self.counts[1] = n;
    }
    pub fn set_lintel_pillars_count(&mut self, n: u32) {
        self.counts[2] = n;
    }
    pub fn set_arches_segments_count(&mut self, n: u32) {
        self.counts[3] = n;
    }
    pub fn set_arch_pillars_count(&mut self, n: u32) {
        self.counts[4] = n;
    }
    pub fn set_gates_segments_count(&mut self, n: u32) {
        self.counts[5] = n;
    }
    pub fn set_sewer_segments_count(&mut self, n: u32) {
        self.counts[6] = n;
    }
    pub fn set_fences_segments_count(&mut self, n: u32) {
        self.counts[7] = n;
    }
    pub fn set_fences_segments_seed(&mut self, seed: u64) {
        self.fences_segments_seed = seed;
    }
    pub fn add_lintels_length(&mut self, x: f32) {
        self.lengths[1] += x;
        self.lengths[3] -= x;
    }
    pub fn add_arches_length(&mut self, x: f32) {
        self.lengths[2] += x;
        self.lengths[0] -= x;
    }
    pub fn add_gate_length(&mut self, x: f32) {
        self.lengths[4] += x;
        self.lengths[0] -= x;
    }
    pub fn add_stairs_length(&mut self, x: f32) {
        self.lengths[5] += x;
        self.lengths[0] -= x;
    }
    pub fn add_sewer_length(&mut self, x: f32) {
        self.lengths[6] += x;
        self.lengths[0] -= x;
    }
    /// Original zips the two arrays to their shorter length. Incoming NaN is
    /// suppressed by MAXSS/maxnum, not selected with a total-order maximum.
    pub fn max_pillar_movement(&self) -> f32 {
        let mut maximum = 0.0f32;
        for (a, b) in self
            .previous_pillar_placements
            .iter()
            .zip(&self.pillar_placements)
        {
            let x = a[0] - b[0];
            let y = a[1] - b[1];
            let distance = (y * y + x * x).sqrt();
            if maximum.is_nan() || distance > maximum {
                maximum = distance;
            }
        }
        maximum
    }
    /// Original total_cmp argmax: last-wins ties, plain bias, first-five gate
    /// and sewer-before-stairs comparison. Seven is the no-material sentinel.
    pub fn which_material_changed_the_most(&self) -> u8 {
        let d: [f32; 7] = std::array::from_fn(|i| {
            f32::from_bits((self.lengths[i] - self.previous_lengths[i]).to_bits() & 0x7fff_ffff)
        });
        let plain = d[0] + f32::from_bits(0xbc23_d70a);
        // UCOMISS zero,sum / JAE skips only ordered sum<=0. NaN proceeds.
        if ((((d[3] + d[2]) + d[1]) + plain) + d[4]) <= 0.0 {
            return 7;
        }
        let mut best = plain;
        let mut kind = 0;
        for (candidate, value) in [
            (3, d[3]),
            (1, d[1]),
            (2, d[2]),
            (4, d[4]),
            (5, d[6]),
            (6, d[5]),
        ] {
            if !value.total_cmp(&best).is_lt() {
                best = value;
                kind = candidate;
            }
        }
        kind
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_native_material_stats_calls_match_all_bits_and_selectors() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reverse-engineering/tiny-glade/evidence/round12/wall-gpu/material-stats-native-cases.json"
        ))).unwrap();
        for (i, case) in fixture["cases"].as_array().unwrap().iter().enumerate() {
            let source = |key: &str| {
                std::array::from_fn(|n| f32::from_bits(case[key][n].as_u64().unwrap() as u32))
            };
            let stats = MaterialStats {
                previous_lengths: source("previous"),
                lengths: source("current"),
                ..Default::default()
            };
            assert_eq!(
                stats.which_material_changed_the_most(),
                case["material"].as_u64().unwrap() as u8,
                "native argmax {i}"
            );
        }
        for (i, case) in fixture["init_cases"].as_array().unwrap().iter().enumerate() {
            let mut stats = MaterialStats {
                previous_lengths: std::array::from_fn(|i| i as f32),
                ..Default::default()
            };
            stats.init_this_update(
                f32::from_bits(case["length"].as_u64().unwrap() as u32),
                case["special"].as_u64().unwrap() != 0,
            );
            for (key, values) in [
                ("current", stats.lengths),
                ("previous", stats.previous_lengths),
            ] {
                for (j, value) in values.iter().enumerate() {
                    assert_eq!(
                        value.to_bits(),
                        case[key][j].as_u64().unwrap() as u32,
                        "native init {i} {key} {j}"
                    );
                }
            }
            assert_eq!(stats.counts, [0; 8]);
        }
    }
    #[test]
    fn init_preserves_previous_and_transfers_each_budget() {
        let mut s = MaterialStats {
            previous_lengths: [3.; 7],
            ..Default::default()
        };
        s.init_this_update(10., false);
        s.add_arches_length(2.);
        s.add_gate_length(1.);
        s.add_stairs_length(1.5);
        s.add_sewer_length(0.5);
        assert_eq!(s.lengths, [5., 0., 2., 0., 1., 1.5, 0.5]);
        s.init_this_update(7., true);
        s.add_lintels_length(2.);
        assert_eq!(s.lengths, [0., 2., 0., 5., 0., 0., 0.]);
        assert_eq!(s.previous_lengths, [3.; 7]);
    }
    #[test]
    fn native_ties_gate_and_pillar_zip() {
        let mut s = MaterialStats::default();
        s.lengths[5] = 10.;
        assert_eq!(s.which_material_changed_the_most(), 7);
        s.lengths[1] = 10.;
        assert_eq!(s.which_material_changed_the_most(), 6);
        s.lengths[5] = 0.;
        s.lengths[6] = 10.;
        assert_eq!(s.which_material_changed_the_most(), 5);
        s.previous_pillar_placements = vec![[0., 0.], [0., 0.], [90., 90.]];
        s.pillar_placements = vec![[3., 4.], [f32::NAN, 0.]];
        assert_eq!(s.max_pillar_movement(), 5.);
    }
}
