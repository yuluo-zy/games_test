// Candidate test module to append inside startup/load_glade_grading.rs::tests.
// No production source or Cargo changes were made by the audit owner.
#[test]
fn consecutive_sessions_match_original_grading_lifecycle_fixtures() {
    let data: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../reverse-engineering/tiny-glade/evidence/round12/color-startup/grading-original-lifecycle-fixtures.json"
    )))
    .unwrap();
    let mut configs = ImageFormationConfigs::default();
    let saved = ImageFormationConfig {
        name: "saved-full-config".to_owned(),
        params_18: [0.25, -1.5, 1.75, 0.875],
        flag_28: 1,
        params_30: [0.125, 0.0625],
        word_38: (-0.25_f32).to_bits(),
        flag_3c: 1,
        flag_3d: 1,
        ..ImageFormationConfig::default()
    };
    for fixture in data["fixtures"].as_array().unwrap() {
        configs.kind = fixture["kind"].as_u64().unwrap() as u8;
        let mut meta = SessionMeta::default();
        meta.mutators.push(SessionMutator::Halloween2024);
        meta.image = fixture["saved_override"]
            .as_bool()
            .unwrap()
            .then(|| saved.clone());
        load_session_grading(&mut configs, &meta, "nonexistent-root-unused-by-halloween");
        assert_eq!(u64::from(configs.kind), fixture["kind"].as_u64().unwrap());
        for (config, expected) in configs
            .configs
            .iter()
            .zip(fixture["slots"].as_array().unwrap())
        {
            assert_eq!(config.name, expected["name"].as_str().unwrap());
            for (actual, expected) in config
                .params_18
                .iter()
                .zip(expected["grading"].as_array().unwrap())
            {
                assert_eq!(
                    actual.to_bits(),
                    (expected.as_f64().unwrap() as f32).to_bits()
                );
            }
            let lens = [
                config.params_30[0].to_bits(),
                config.params_30[1].to_bits(),
                config.word_38,
            ];
            for (actual, expected) in lens.iter().zip(expected["lens_bits"].as_array().unwrap()) {
                assert_eq!(u64::from(*actual), expected.as_u64().unwrap());
            }
            assert_eq!(
                u64::from(config.flag_28),
                expected["bypass"].as_u64().unwrap()
            );
            assert_eq!(
                u64::from(config.flag_3c),
                expected["lighting_only"].as_u64().unwrap()
            );
            assert_eq!(
                u64::from(config.flag_3d),
                expected["tree_see_thru"].as_u64().unwrap()
            );
        }
    }
}
