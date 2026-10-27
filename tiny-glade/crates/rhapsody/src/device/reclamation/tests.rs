use super::*;
#[test]
fn reclamation_limits_must_progress_and_pressure_cannot_slow_collection() {
    let config = ReclamationConfig::default();
    config.validate().unwrap();
    assert_eq!(
        config.budget(crate::accounting::BudgetAction::None),
        config.normal
    );
    assert_eq!(
        config.budget(crate::accounting::BudgetAction::EvictCache),
        config.pressure
    );
    assert_eq!(
        config.budget(crate::accounting::BudgetAction::Backpressure),
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
