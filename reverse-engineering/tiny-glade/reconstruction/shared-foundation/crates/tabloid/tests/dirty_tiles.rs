//! Fixed-size 512-byte dirty-grid accumulation with independent subscribers.
//! Taken batches are immutable snapshots; pending batches share accumulated ORs.
use tabloid::{Publisher, State, StateDelta};

/// 64x64 bit grid (512 bytes), mirroring `country_core::utils::dirty_tiles::DirtyTiles`.
#[derive(Debug, PartialEq, Clone)]
struct DirtyGrid([u64; 64]);

impl State for DirtyGrid {
    fn empty() -> Self {
        DirtyGrid([0; 64])
    }
}

impl StateDelta<DirtyGrid> for DirtyGrid {
    fn apply_to(&self, state: &mut DirtyGrid) {
        for (state, delta) in state.0.iter_mut().zip(self.0.iter()) {
            *state |= *delta;
        }
    }
}

fn grid_with_word(index: usize, bits: u64) -> DirtyGrid {
    let mut grid = DirtyGrid::empty();
    grid.0[index] = bits;
    grid
}

#[test]
fn dirty_grid_batches_or_accumulate_per_subscriber() {
    let mut publisher = Publisher::<DirtyGrid>::new();
    let mut fast = publisher.subscribe();
    let mut slow = publisher.subscribe();

    publisher
        .publish(grid_with_word(0, 0x0000_0000_0000_0001))
        .unwrap();
    // Fast consumes the first batch immediately; the shared publication keeps
    // OR-ing for subscribers that still hold it (slow).
    drop(fast.take().unwrap());
    publisher
        .publish(grid_with_word(1, 0x0000_0000_0000_0002))
        .unwrap();
    publisher
        .publish(grid_with_word(63, 0x8000_0000_0000_0000))
        .unwrap();

    // Fast took after the first publish, so its reader only shows the later
    // OR-accumulated batch (words 1 and 63), not the first.
    let fast_read = fast.take().unwrap();
    assert_eq!(fast_read.0[0], 0);
    assert_eq!(fast_read.0[1], 0x0000_0000_0000_0002);
    assert_eq!(fast_read.0[63], 0x8000_0000_0000_0000);
    assert_eq!(fast_read.sequence(), 3);

    // Slow never took, so it accumulated the OR of all three deltas.
    let slow_read = slow.take().unwrap();
    assert_eq!(slow_read.0[0], 0x0000_0000_0000_0001);
    assert_eq!(slow_read.0[1], 0x0000_0000_0000_0002);
    assert_eq!(slow_read.0[63], 0x8000_0000_0000_0000);
    assert_eq!(slow_read.sequence(), 3);
    assert!(slow.take().is_none());
}

#[test]
fn late_subscriber_only_receives_subsequent_batches() {
    let mut publisher = Publisher::<DirtyGrid>::new();
    let mut early = publisher.subscribe();
    publisher.publish(grid_with_word(0, 1)).unwrap();
    drop(early.take().unwrap());

    let mut late = publisher.subscribe();
    assert!(late.take().is_none());
    publisher.publish(grid_with_word(2, 4)).unwrap();
    let late_read = late.take().unwrap();
    assert_eq!(late_read.0[2], 4);
    assert_eq!(late_read.sequence(), 2);
}

#[test]
fn cloned_subscription_shares_take_state() {
    let mut publisher = Publisher::<DirtyGrid>::new();
    let mut a = publisher.subscribe();
    let mut b = a.clone();
    publisher.publish(grid_with_word(0, 1)).unwrap();
    drop(a.take().unwrap());
    assert!(b.take().is_none());
    publisher.publish(grid_with_word(1, 2)).unwrap();
    assert_eq!(b.take().unwrap().0[1], 2);
}

#[test]
fn held_grid_reader_is_a_nonblocking_immutable_snapshot() {
    let mut publisher = Publisher::<DirtyGrid>::new();
    let mut subscriber = publisher.subscribe();
    publisher.publish(grid_with_word(0, 1)).unwrap();
    let reader = subscriber.take().unwrap();
    publisher.publish(grid_with_word(5, 8)).unwrap();
    assert_eq!(reader.0[0], 1);
    assert_eq!(reader.0[5], 0);
    assert_eq!(reader.sequence(), 1);
    assert_eq!(subscriber.take().unwrap().0[5], 8);
}
