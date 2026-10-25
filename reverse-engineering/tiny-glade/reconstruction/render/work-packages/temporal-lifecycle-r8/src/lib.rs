//! Private compile ownership control flow, not a reconstruction of HashMap/drop glue.

#[derive(Debug, PartialEq, Eq)]
pub struct PersistentState<G, T> {
    pub graph: Option<G>,
    pub temporal: Option<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DetachedState<G, T> {
    pub graph: G,
    pub temporal: T,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TakeFailure<G> {
    GraphMissing,
    /// The graph has already been taken when the temporal-presence check fails.
    TemporalMissing {
        detached_graph: G,
    },
}

impl<G, T> PersistentState<G, T> {
    /// Observed at 0x140166510..0x140166617. Failure is exposed before unwind.
    pub fn take_for_compile(&mut self) -> Result<DetachedState<G, T>, TakeFailure<G>> {
        let graph = self.graph.take().ok_or(TakeFailure::GraphMissing)?;
        let Some(temporal) = self.temporal.take() else {
            return Err(TakeFailure::TemporalMissing {
                detached_graph: graph,
            });
        };
        Ok(DetachedState { graph, temporal })
    }

    /// Exported state is installed before dropping any previous temporal value.
    /// Normal serialized compilation has None here after take_for_compile.
    pub fn restore_temporal(&mut self, exported: T) {
        let previous = self.temporal.replace(exported);
        drop(previous);
    }
}

/// Game consumer leaf 0x140976880 reads the unlabelled frame word at +0x80.
/// This resets temporal history selection; it does not itself clear the rg cache.
pub fn should_reset_temporal(frame_word: u32) -> bool {
    frame_word == 0
}

pub const GRAPH_SIZE: usize = 144;
pub const TEMPORAL_SIZE: usize = 96;

/// Opaque snapshots are confined to the oracle adapter; the runtime API above is typed.
pub fn snapshot_fixture(
    mode: u32,
    graph_present: bool,
    temporal_present: bool,
    marker: u8,
    new_marker: u8,
) -> String {
    let mut graph = [marker; GRAPH_SIZE];
    graph[..8].copy_from_slice(&(if graph_present { 3u64 } else { 1u64 << 63 }).to_le_bytes());
    let mut temporal = [marker; TEMPORAL_SIZE];
    temporal[..8].copy_from_slice(&u64::from(temporal_present).to_le_bytes());
    let mut state = PersistentState {
        graph: graph_present.then_some(graph),
        temporal: temporal_present.then_some(temporal),
    };
    let mut events = vec![];
    let mut detached = Vec::new();
    if mode == 0 {
        match state.take_for_compile() {
            Ok(taken) => {
                graph[..8].copy_from_slice(&(1u64 << 63).to_le_bytes());
                temporal[..8].fill(0);
                detached.extend_from_slice(&taken.graph);
                detached.extend_from_slice(&taken.temporal);
                events.push("taken".to_string());
            }
            Err(TakeFailure::GraphMissing) => events.push("graph-missing".to_string()),
            Err(TakeFailure::TemporalMissing { .. }) => {
                graph[..8].copy_from_slice(&(1u64 << 63).to_le_bytes());
                events.push("temporal-missing".to_string());
            }
        }
    } else {
        let mut replacement = [new_marker; TEMPORAL_SIZE];
        replacement[..8].copy_from_slice(&1u64.to_le_bytes());
        // The old parity map is empty in these owned-map cleanup fixtures.
        temporal[56..64].fill(0);
        if temporal_present {
            events.push("drop-old-records".to_string());
        }
        state.restore_temporal(replacement);
        temporal = state.temporal.unwrap();
    }
    let hex = |bytes: &[u8]| bytes.iter().map(|v| format!("{v:02x}")).collect::<String>();
    format!(
        "{}|{}|{}|{}",
        events.join(","),
        hex(&graph),
        hex(&temporal),
        hex(&detached)
    )
}
