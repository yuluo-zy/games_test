use crate::edit::{Adjustment, EditCommand};
use crate::schema::TerrainEdit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplayDirection { Forward, Backward }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplayError { RequiresLifecycleConsumer }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReplayObservation {
    pub id: crate::schema::StrokeId,
    pub falloff_operation: u32,
    pub falloff_value: f32,
    pub height_operation: u32,
    pub height_value: f32,
}
impl ReplayObservation {
    pub fn command(self) -> EditCommand {
        EditCommand {
            id: self.id,
            falloff: if self.falloff_operation == 0 { Adjustment::Keep } else { Adjustment::Absolute(self.falloff_value) },
            height: if self.height_operation == 0 { Adjustment::Keep } else { Adjustment::Absolute(self.height_value) },
            position: [0.0; 3],
        }
    }
}

/// Original Edit branch, operation order preserved; does not clamp progress.
pub fn observe_edit(edit: &TerrainEdit, progress: f32, direction: ReplayDirection) -> Result<ReplayObservation, ReplayError> {
    let TerrainEdit::EditTerrainStroke { index, prev_height_ws, new_height_ws, prev_falloff, new_falloff } = *edit else {
        return Err(ReplayError::RequiresLifecycleConsumer);
    };
    let inverse = 1.0 - progress;
    let mix = |previous: f32, new: f32| match direction {
        ReplayDirection::Forward => new * progress + previous * inverse,
        ReplayDirection::Backward => previous * progress + new * inverse,
    };
    Ok(ReplayObservation {
        id: index,
        falloff_operation: u32::from(prev_falloff != new_falloff),
        falloff_value: mix(prev_falloff, new_falloff),
        height_operation: u32::from(prev_height_ws != new_height_ws),
        height_value: mix(prev_height_ws, new_height_ws),
    })
}
pub fn edit_command(edit: &TerrainEdit, progress: f32, direction: ReplayDirection) -> Result<EditCommand, ReplayError> {
    observe_edit(edit, progress, direction).map(ReplayObservation::command)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::StrokeId;
    #[test]
    fn flags_come_from_endpoints_not_interpolated_result() {
        let edit = TerrainEdit::EditTerrainStroke { index: StrokeId(5), prev_height_ws: 2.5, new_height_ws: 4.0, prev_falloff: 0.4, new_falloff: 0.4 };
        let event = edit_command(&edit, 0.0, ReplayDirection::Forward).unwrap();
        assert_eq!(event.height, Adjustment::Absolute(2.5));
        assert_eq!(event.falloff, Adjustment::Keep);
    }
}
