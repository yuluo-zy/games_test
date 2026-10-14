//! system-tree::despawn_trees, B 0x1422686d0: the resolved deletion predicate.
//! The caller supplies the original marker/query result and animation field.
pub const DESPAWN_THRESHOLD: f32 = 0.01;

pub fn should_despawn_tree(marked_for_deletion: bool, animation_value: f32) -> bool {
    marked_for_deletion && animation_value < DESPAWN_THRESHOLD
}

#[cfg(test)]
mod tests {
    use super::*;
    // Original source test `sends_only_below_threshold` and unmarked query case.
    #[test]
    fn original_deletion_boundary_and_marker_contract() {
        for (value, expected) in [
            (0.0, true),
            (0.0099, true),
            (0.01, false),
            (0.5, false),
            (f32::NAN, false),
            (-1.0, true),
        ] {
            assert_eq!(should_despawn_tree(true, value), expected);
            assert!(!should_despawn_tree(false, value));
        }
    }
}
