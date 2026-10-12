use crate::schema::StrokeId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Adjustment {
    Keep,
    Absolute(f32),
    Delta(f32),
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EditCommand {
    pub id: StrokeId,
    pub falloff: Adjustment,
    pub height: Adjustment,
    /// Actual game point payload; coordinate meaning remains an input contract.
    pub position: [f32; 3],
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrokeParameters {
    pub falloff: f32,
    pub height: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioObservation {
    /// Numeric PlayTerrainSound discriminant, not a guessed sound name.
    pub tag: u32,
    pub falloff: f32,
    pub height: Option<f32>,
    pub magnitude: f32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EditOutcome {
    pub state: StrokeParameters,
    pub audio: Vec<AudioObservation>,
    /// Each resolved original edit emits recalculation even if both fields Keep.
    pub requests_recalculation: bool,
}

// SSE MAXSS/MINSS choose their source operand on an unordered/equal comparison.
fn source_max(destination: f32, source: f32) -> f32 {
    if destination > source { destination } else { source }
}
fn source_min(destination: f32, source: f32) -> f32 {
    if destination < source { destination } else { source }
}

/// B 14094eb76..14094ef1b: business writes and ordered audio observations.
/// Does not allocate/resolve native hash buckets or enqueue ECS events.
pub fn apply_resolved(state: StrokeParameters, command: EditCommand) -> EditOutcome {
    let mut state = state;
    let mut audio = Vec::new();
    match command.falloff {
        Adjustment::Keep => {}
        Adjustment::Absolute(value) => {
            let previous = state.falloff;
            state.falloff = value;
            audio.push(AudioObservation { tag: 1, falloff: value, height: None,
                magnitude: (value - previous).abs() });
        }
        Adjustment::Delta(delta) => {
            let previous = state.falloff;
            let value = source_min(1.0, source_max(0.0, previous + delta));
            state.falloff = value;
            if previous != value {
                audio.push(AudioObservation { tag: 1, falloff: value, height: None,
                    magnitude: (previous - value).abs() });
            }
        }
    }
    match command.height {
        Adjustment::Keep => {}
        Adjustment::Absolute(value) => {
            let previous = state.height;
            let magnitude = (value - previous).abs();
            if value == 0.0 && previous > 0.0 {
                audio.push(AudioObservation { tag: 4, falloff: state.falloff, height: None, magnitude });
            }
            state.height = value;
            audio.push(AudioObservation { tag: 2, falloff: state.falloff, height: Some(value), magnitude });
        }
        Adjustment::Delta(delta) => {
            let mut previous = state.height;
            if delta > 0.0 && previous == 0.0 {
                previous = 1.0;
                state.height = 1.0;
            }
            let clamped = source_min(20.0, source_max(0.0, previous + delta));
            let value = if clamped < 1.0 { 0.0 } else { clamped };
            state.height = value;
            let magnitude = (previous - value).abs();
            if previous != value {
                audio.push(AudioObservation { tag: 2, falloff: state.falloff, height: Some(value), magnitude });
            }
            if previous > 0.0 && value == 0.0 {
                audio.push(AudioObservation { tag: 4, falloff: state.falloff, height: None, magnitude });
            }
        }
    }
    EditOutcome { state, audio, requests_recalculation: true }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn command(height: Adjustment) -> EditCommand {
        EditCommand { id: StrokeId(0), falloff: Adjustment::Keep, height, position: [0.0; 3] }
    }
    #[test]
    fn activation_uses_one_as_delta_baseline() {
        let out = apply_resolved(StrokeParameters { falloff: 0.4, height: 0.0 }, command(Adjustment::Delta(0.1)));
        assert_eq!(out.state.height.to_bits(), 1.1_f32.to_bits());
        assert_eq!(out.audio.len(), 1);
        assert_eq!(out.audio[0].magnitude.to_bits(), (1.0_f32 - 1.1).abs().to_bits());
    }
    #[test]
    fn delta_and_absolute_to_zero_have_distinct_order() {
        let state = StrokeParameters { falloff: 0.4, height: 2.5 };
        assert_eq!(apply_resolved(state, command(Adjustment::Delta(-3.0))).audio.iter().map(|a| a.tag).collect::<Vec<_>>(), [2, 4]);
        assert_eq!(apply_resolved(state, command(Adjustment::Absolute(0.0))).audio.iter().map(|a| a.tag).collect::<Vec<_>>(), [4, 2]);
    }
    #[test]
    fn absolute_keeps_raw_unclamped_value() {
        let state = StrokeParameters { falloff: 0.4, height: 2.5 };
        assert_eq!(apply_resolved(state, command(Adjustment::Absolute(-3.0))).state.height, -3.0);
    }
}
