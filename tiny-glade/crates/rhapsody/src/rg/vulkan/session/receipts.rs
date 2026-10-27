//! Completed-resource receipts and their adoption.
use std::sync::Arc;
use super::CompletedNamedResources;
use super::VulkanGraphSession;
use super::super::named_resources::NamedResources;

impl VulkanGraphSession {
    /// Additive current-architecture handoff used by background App warmup.
    /// Ordinary sessions retain their existing API and resource/cache owners.
    pub fn take_completed_named_resources(
        &mut self,
        timeout_ns: u64,
    ) -> crate::Result<CompletedNamedResources> {
        if self.active_prepared.is_some()
            || self.last_execution_partial
            || !self.ledger.has_ended()
            || self.ledger.submissions() == 0
        {
            return Err(crate::Error::invalid_usage(
                "named handoff requires a completely submitted frame",
            ));
        }
        self.wait_submits_done(timeout_ns)?;
        if !self.pending.is_empty() || self.frame_import_count() != 0 || self.is_poisoned() {
            return Err(crate::Error::invalid_usage(
                "named handoff has pending or lost GPU work",
            ));
        }
        let named = self.named.as_ref().ok_or_else(|| {
            crate::Error::invalid_usage("named handoff while a graph is executing")
        })?;
        if named.state.requested_validity().next().is_some() {
            return Err(crate::Error::invalid_usage(
                "named handoff has an uncommitted frame declaration",
            ));
        }
        let families = named
            .resources
            .resources
            .keys()
            .map(|token| {
                (
                    *token,
                    self.backend
                        .import_families
                        .get(token)
                        .copied()
                        .unwrap_or(self.backend.device.queues.primary_family),
                )
            })
            .collect();
        let mut named = self
            .named
            .replace(NamedResources::new(self.backend.device.clone()))
            .expect("validated named owner");
        named.graph_id = None;
        Ok(CompletedNamedResources { named, families })
    }

    /// Restore the same actual named owners into the App's original idle
    /// session. It never merges two histories or overwrites live GPU work.
    pub fn validate_completed_named_resources_adoption(
        &self,
        completed: &CompletedNamedResources,
    ) -> crate::Result<()> {
        if self.active_prepared.is_some()
            || !self.pending.is_empty()
            || self.frame_import_count() != 0
            || self.is_poisoned()
        {
            return Err(crate::Error::invalid_usage(
                "named adoption requires an idle healthy session",
            ));
        }
        let current = self.named.as_ref().ok_or_else(|| {
            crate::Error::invalid_usage("named adoption while a graph is executing")
        })?;
        if (current.graph_id.is_some() && !self.ledger.has_ended()) || current.resources.len() != 0
        {
            return Err(crate::Error::invalid_usage(
                "named adoption cannot replace an existing graph/history owner",
            ));
        }
        if !Arc::ptr_eq(&completed.named.resources.device, &self.backend.device) {
            return Err(crate::Error::invalid_usage(
                "named handoff belongs to another device owner",
            ));
        }
        for resource in completed.named.resources.resources.values() {
            resource.check_device(&self.backend.device)?;
        }
        Ok(())
    }

    /// Consume a completed owner's cache only after all adoption checks succeed.
    /// Call the borrowed validation first when retaining the transfer on failure
    /// is required by an application-owned one-shot channel.
    pub fn adopt_completed_named_resources(
        &mut self,
        completed: CompletedNamedResources,
    ) -> crate::Result<()> {
        self.validate_completed_named_resources_adoption(&completed)?;
        for (token, family) in completed.families {
            self.backend.import_families.insert(token, family);
        }
        self.named = Some(completed.named);
        Ok(())
    }

    pub fn configure_receipt_storage(
        &mut self,
        config: crate::rg::receipt_storage::ReceiptStorageConfig,
    ) -> crate::Result<()> {
        if self.active_prepared.is_some() {
            return Err(crate::Error::invalid_usage(
                "a compiled frame owns an admitted receipt slot",
            ));
        }
        self.backend.receipt_storage = Some(crate::rg::receipt_storage::ReceiptStorage::new(
            config,
            self.backend.device.memory.stats.clone(),
        )?);
        Ok(())
    }

    pub fn receipt_storage_statistics(&mut self) -> crate::rg::receipt_storage::ReceiptStorageStats {
        self.backend.receipt_storage
            .as_mut()
            .map_or(Default::default(), |pool| pool.statistics())
    }
}
