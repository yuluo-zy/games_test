//! system-critters raw locomotion predicates from B collision consumers.
//! Species names and original enum layout remain unknown; this is a CPU input
//! projection, not a replacement Bevy component or an eight-byte ABI claim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CritterLocomotion(pub u32);

impl CritterLocomotion {
    pub fn samples_rock_mask(self) -> bool {
        self.0 == 2
    }
    pub fn skips_water_sample(self) -> bool {
        self.0 == 4
    }
    pub fn skips_public_walls(self) -> bool {
        self.0 == 3
    }
    pub fn wall_hash_only(self) -> bool {
        (99u32 >> (self.0 & 31)) & 1 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_locomotion_mask_selects_observed_values() {
        let selected: Vec<u32> = (0..32)
            .filter(|&value| CritterLocomotion(value).wall_hash_only())
            .collect();
        assert_eq!(selected, [0, 1, 5, 6]);
    }
    #[test]
    fn original_other_branch_predicates() {
        assert!(CritterLocomotion(2).samples_rock_mask());
        assert!(CritterLocomotion(4).skips_water_sample());
        assert!(CritterLocomotion(3).skips_public_walls());
        assert!(!CritterLocomotion(7).wall_hash_only());
    }
}
