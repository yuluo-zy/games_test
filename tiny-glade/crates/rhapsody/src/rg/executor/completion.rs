//! Cache statistics and command completion bookkeeping.
use crate::command::recording;
use super::ExecutionChain;

impl ExecutionChain {
    pub fn set_plan_cache_enabled(&mut self, enabled: bool) {
        self.plan_cache_enabled = enabled;
        if !enabled {
            self.plan_cache
                .clear_with_reason(crate::collections::cache::CacheReason::Disabled);
        }
    }

    pub fn set_subgraph_cache_enabled(&mut self, enabled: bool) {
        self.subgraph_cache.set_enabled(enabled);
    }

    pub fn subgraph_cache_stats(&self) -> super::super::subgraph::SubgraphCacheStats {
        self.subgraph_cache.stats()
    }

    pub fn plan_cache_stats(&self) -> crate::collections::cache::CacheStats {
        self.plan_cache.stats()
    }

    pub fn plan_cache_bytes(&self) -> usize {
        self.plan_cache.bytes()
    }

    /// Called only after the backend has observed this queue completion.
    fn complete(&mut self, point: crate::sync::SyncPoint) {
        let mut index = 0;
        while index < self.pending_commands.len() {
            let (required, _) = &self.pending_commands[index];
            if required.queue == point.queue && required.value <= point.value {
                let (_, command) = self.pending_commands.swap_remove(index);
                self.completed_commands.push(command);
            } else {
                index += 1;
            }
        }
    }

    pub(crate) fn observe_completion(&mut self, point: crate::sync::SyncPoint) {
        self.complete(point);
    }

    pub(crate) fn recycle(&mut self) -> Option<recording::Command> {
        self.completed_commands.pop()
    }

    pub fn pending_command_count(&self) -> usize {
        self.pending_commands.len()
    }

    pub fn recording_command_count(&self) -> usize {
        self.command_records.len()
    }

    pub fn completed_command_count(&self) -> usize {
        self.completed_commands.len()
    }
}
