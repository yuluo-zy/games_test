//! Bounded maintenance ticks. Shutdown and explicit draining use a separate,
//! complete collection operation rather than silently ignoring these limits.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReclamationBudget {
    /// Maximum retirement entries inspected during one maintenance tick.
    pub max_scans: usize,
    /// Maximum native objects destroyed during one maintenance tick.
    pub max_destroys: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReclamationConfig {
    pub normal: ReclamationBudget,
    pub pressure: ReclamationBudget,
}
impl Default for ReclamationConfig {
    fn default() -> Self {
        Self {
            normal: ReclamationBudget {
                max_scans: 256,
                max_destroys: 64,
            },
            pressure: ReclamationBudget {
                max_scans: 4096,
                max_destroys: 1024,
            },
        }
    }
}
impl ReclamationConfig {
    pub(crate) fn budget(self, action: crate::accounting::BudgetAction) -> ReclamationBudget {
        match action {
            crate::accounting::BudgetAction::None => self.normal,
            crate::accounting::BudgetAction::EvictCache | crate::accounting::BudgetAction::Backpressure => self.pressure,
        }
    }
    pub fn validate(self) -> crate::Result<()> {
        if self.normal.max_scans == 0
            || self.normal.max_destroys == 0
            || self.normal.max_destroys > self.normal.max_scans
            || self.pressure.max_scans < self.normal.max_scans
            || self.pressure.max_destroys < self.normal.max_destroys
            || self.pressure.max_destroys > self.pressure.max_scans
        {
            return Err(crate::Error::invalid_usage(
                "invalid normal/pressure reclamation budgets",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
