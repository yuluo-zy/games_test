use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorBinding {
    pub set: u32,
    pub binding: u32,
    pub descriptor_type: String,
    pub count: u32,
    pub stages: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PushConstantRange {
    pub offset: u32,
    pub size: u32,
    pub stages: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageLocation {
    pub location: u32,
    pub component: u32,
    pub type_signature: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InterfaceError {
    DescriptorConflict { set: u32, binding: u32 },
    ZeroPushConstantRange,
    InvalidPushConstantAlignment,
    PushConstantOverflow,
    MissingStageInput { location: u32, component: u32 },
    StageTypeMismatch { location: u32, component: u32 },
}

pub fn merge_bindings(
    bindings: &[DescriptorBinding],
) -> Result<Vec<DescriptorBinding>, InterfaceError> {
    let mut merged: BTreeMap<(u32, u32), DescriptorBinding> = BTreeMap::new();
    for binding in bindings {
        let key = (binding.set, binding.binding);
        if let Some(previous) = merged.get_mut(&key) {
            if previous.descriptor_type != binding.descriptor_type
                || previous.count != binding.count
            {
                return Err(InterfaceError::DescriptorConflict {
                    set: key.0,
                    binding: key.1,
                });
            }
            previous.stages.extend(binding.stages.clone());
            previous.stages.sort();
            previous.stages.dedup();
        } else {
            let mut binding = binding.clone();
            binding.stages.sort();
            binding.stages.dedup();
            merged.insert(key, binding);
        }
    }
    Ok(merged.into_values().collect())
}

pub fn validate_push_constants(ranges: &[PushConstantRange]) -> Result<(), InterfaceError> {
    for range in ranges {
        if range.size == 0 {
            return Err(InterfaceError::ZeroPushConstantRange);
        }
        if range.offset % 4 != 0 || range.size % 4 != 0 {
            return Err(InterfaceError::InvalidPushConstantAlignment);
        }
        range
            .offset
            .checked_add(range.size)
            .ok_or(InterfaceError::PushConstantOverflow)?;
    }
    Ok(())
}

pub fn validate_stage_link(
    outputs: &[StageLocation],
    inputs: &[StageLocation],
) -> Result<(), InterfaceError> {
    for input in inputs {
        let output = outputs
            .iter()
            .find(|output| output.location == input.location && output.component == input.component)
            .ok_or(InterfaceError::MissingStageInput {
                location: input.location,
                component: input.component,
            })?;
        if output.type_signature != input.type_signature {
            return Err(InterfaceError::StageTypeMismatch {
                location: input.location,
                component: input.component,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn binding(stage: &str) -> DescriptorBinding {
        DescriptorBinding {
            set: 1,
            binding: 2,
            descriptor_type: "StorageBuffer".into(),
            count: 0,
            stages: vec![stage.into()],
        }
    }
    #[test]
    fn dynamic_count_and_stage_union_remain_explicit() {
        let merged = merge_bindings(&[binding("Vertex"), binding("Fragment")]).unwrap();
        assert_eq!(merged[0].count, 0);
        assert_eq!(merged[0].stages, vec!["Fragment", "Vertex"]);
    }
    #[test]
    fn conflicting_binding_types_or_counts_are_rejected() {
        let mut changed = binding("Fragment");
        changed.count = 1;
        assert!(merge_bindings(&[binding("Vertex"), changed]).is_err());
    }
    #[test]
    fn push_ranges_validate_alignment_and_overflow() {
        assert!(validate_push_constants(&[PushConstantRange {
            offset: 0,
            size: 16,
            stages: vec![]
        }])
        .is_ok());
        assert!(validate_push_constants(&[PushConstantRange {
            offset: 2,
            size: 16,
            stages: vec![]
        }])
        .is_err());
        assert!(validate_push_constants(&[PushConstantRange {
            offset: u32::MAX - 3,
            size: 8,
            stages: vec![]
        }])
        .is_err());
    }
    #[test]
    fn stage_link_rejects_missing_or_incompatible_locations() {
        let input = StageLocation {
            location: 0,
            component: 0,
            type_signature: "vec2<f32>".into(),
        };
        assert!(
            validate_stage_link(std::slice::from_ref(&input), std::slice::from_ref(&input)).is_ok()
        );
        assert!(validate_stage_link(&[], &[input]).is_err());
    }
}
