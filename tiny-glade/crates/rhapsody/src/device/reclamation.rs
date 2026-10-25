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
    pub(crate) fn budget(self, action: super::BudgetAction) -> ReclamationBudget {
        match action {
            super::BudgetAction::None => self.normal,
            super::BudgetAction::EvictCache | super::BudgetAction::Backpressure => self.pressure,
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
mod tests {
    use super::*;
    #[test]
    fn reclamation_limits_must_progress_and_pressure_cannot_slow_collection() {
        let config = ReclamationConfig::default();
        config.validate().unwrap();
        assert_eq!(
            config.budget(super::super::BudgetAction::None),
            config.normal
        );
        assert_eq!(
            config.budget(super::super::BudgetAction::EvictCache),
            config.pressure
        );
        assert_eq!(
            config.budget(super::super::BudgetAction::Backpressure),
            config.pressure
        );
        for changed in [
            ReclamationConfig {
                normal: ReclamationBudget {
                    max_scans: 0,
                    ..config.normal
                },
                ..config
            },
            ReclamationConfig {
                normal: ReclamationBudget {
                    max_destroys: 0,
                    ..config.normal
                },
                ..config
            },
            ReclamationConfig {
                normal: ReclamationBudget {
                    max_destroys: 257,
                    ..config.normal
                },
                ..config
            },
            ReclamationConfig {
                pressure: ReclamationBudget {
                    max_scans: 255,
                    ..config.pressure
                },
                ..config
            },
            ReclamationConfig {
                pressure: ReclamationBudget {
                    max_destroys: 63,
                    ..config.pressure
                },
                ..config
            },
            ReclamationConfig {
                pressure: ReclamationBudget {
                    max_destroys: 4097,
                    ..config.pressure
                },
                ..config
            },
        ] {
            assert_eq!(
                changed.validate().unwrap_err().kind(),
                crate::ErrorKind::InvalidUsage
            );
        }
    }
}
