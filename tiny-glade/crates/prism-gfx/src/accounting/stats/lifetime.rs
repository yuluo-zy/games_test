//! GPU-use and retirement tracking of registered allocations.
use crate::sync::CompletedValues;
use crate::sync::MAX_QUEUES;
use crate::sync::QueueId;
use crate::sync::SyncPoint;
use std::sync::atomic::Ordering;
use super::AllocatorStats;
use super::state::StatisticsState;
use super::state::encode_action;
use super::state::transition_phase;
use super::super::budget::BudgetCategory;

impl AllocatorStats {
    /// Stage a GPU use without the global statistics lock.
    #[doc(hidden)]
    pub fn record_gpu_use(&self, id: u64, point: SyncPoint) {
        {
            let mut pending = self.pending_uses[point.queue.index()].lock();
            if pending.try_reserve(1).is_ok() {
                pending.push((point.value, id));
                self.pending_count.fetch_add(1, Ordering::AcqRel);
                return;
            }
        }
        // No staging capacity: apply directly under the state lock.
        self.with_state(|state| state.apply_use(point.queue, point.value, id));
    }

    #[doc(hidden)]
    pub fn mark_retiring(&self, id: u64) {
        // Drop owns this allocation exclusively. Do not flush unrelated staged
        // uses (which can grow in-flight statistics) while marking it terminal.
        let mut state = self.state.lock();
        transition_phase(&mut state, id, BudgetCategory::Retiring);
        self.action
            .store(encode_action(state.action()), Ordering::Release);
    }

    #[doc(hidden)]
    pub fn mark_cached(&self, id: u64, completed: &CompletedValues) {
        self.with_state(|state| {
            if state
                .phases
                .get(&id)
                .is_some_and(|phase| phase.last_use.is_reached(completed))
            {
                transition_phase(state, id, BudgetCategory::Cache);
            }
        });
    }

    /// Reusing completed cached storage changes its category, not its real GPU
    /// history. A retiring allocation stays terminal in transition_phase.
    #[doc(hidden)]
    pub fn mark_live(&self, id: u64, completed: &CompletedValues) {
        self.with_state(|state| {
            if state
                .phases
                .get(&id)
                .is_some_and(|phase| phase.last_use.is_reached(completed))
            {
                transition_phase(state, id, BudgetCategory::Live);
            }
        });
    }

    /// Move in-flight allocations whose uses completed back to `Live`. Work is
    /// proportional to the reached entries; an unchanged snapshot returns
    /// without taking the statistics lock.
    #[doc(hidden)]
    pub fn observe_completed(&self, completed: &CompletedValues) {
        let advanced = self.observed.iter().enumerate().any(|(index, seen)| {
            QueueId::new(index)
                .is_some_and(|queue| completed.get(queue) > seen.load(Ordering::Acquire))
        });
        if !advanced {
            return;
        }
        self.with_state(|state| {
            for index in 0..MAX_QUEUES {
                let Some(queue) = QueueId::new(index) else {
                    continue;
                };
                let done = completed.get(queue);
                if done <= state.observed.get(queue) {
                    continue;
                }
                state.observed.raise(queue, done);
                while let Some(&(value, id)) = state.in_flight[index].front() {
                    if value > done {
                        break;
                    }
                    state.in_flight[index].pop_front();
                    let StatisticsState {
                        budget,
                        phases,
                        records,
                        observed,
                        ..
                    } = &mut *state;
                    if let Some(phase) = phases.get_mut(&id)
                        && phase.category == BudgetCategory::InFlight
                        && phase.last_use.is_reached(observed)
                        && let Some(record) = records.get(&id)
                    {
                        budget.reclassify(
                            BudgetCategory::InFlight,
                            BudgetCategory::Live,
                            record.size,
                        );
                        phase.category = BudgetCategory::Live;
                    }
                }
                self.observed[index].fetch_max(done, Ordering::AcqRel);
            }
        });
    }
}
