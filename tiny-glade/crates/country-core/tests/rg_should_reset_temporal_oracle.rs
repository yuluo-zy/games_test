//! `FrameConstants::should_reset_temporal`（原叶 0x140976880：`[+0x80] u32 == 0`）对原 EXE 的回放。
//!
//! 数据：`parallel-batch-06/temporal-lifecycle/native-oracle.json` 里 mode2 的 1005 例
//! （输入 `[2, word]`，原机器码输出 `"1"`/`"0"`）。`+0x80` 在 country-core 里命名为 `frame_counter`。
use country_core::render::frame_constants::FrameConstants;
use serde_json::Value;
use std::path::PathBuf;

#[test]
fn should_reset_temporal_matches_the_original_leaf_on_1005_cases() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../../reverse-engineering/tiny-glade/evidence/render-recovery/parallel-batch-06/temporal-lifecycle/native-oracle.json",
    );
    let Ok(text) = std::fs::read_to_string(&path) else {
        eprintln!("skip should_reset_temporal oracle: evidence missing at {}", path.display());
        return;
    };
    let rows: Vec<Value> = serde_json::from_str(&text).expect("oracle json");
    let mut replayed = 0;
    for row in &rows {
        let input = row["input"].as_array().unwrap();
        if input[0].as_u64() != Some(2) {
            continue;
        }
        let word = u32::try_from(input[1].as_u64().unwrap()).expect("word fits u32");
        let mut constants = FrameConstants::default();
        constants.frame_counter = word;
        let expected = row["native"].as_str().unwrap();
        let actual = if constants.should_reset_temporal() { "1" } else { "0" };
        assert_eq!(actual, expected, "word {word:#x}");
        replayed += 1;
    }
    assert_eq!(replayed, 1005);
}
