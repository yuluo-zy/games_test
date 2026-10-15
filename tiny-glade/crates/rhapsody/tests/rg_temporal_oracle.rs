//! 原版 oracle 重放：compile 前段 take / post-export restore 的 Option 所有权合同
//! （`parallel-batch-06/temporal-lifecycle/native-oracle.json`，原入口 0x140166510..0x140166617 / 0x140166d2a..0x140166e19）。
//! 数据 = 1101 例：mode0 take 前缀 48、mode1 restore 48、mode2 `should_reset_temporal` 叶 1005。
//! mode2 的实现在 country-core（`FrameConstants::should_reset_temporal`），本 crate 不重放，只核对计数。
use rhapsody::rg::graph::RenderGraph;
use rhapsody::rg::state::{PersistentCompileState, TakeFailure};
use rhapsody::rg::temporal::TemporalRenderGraphState;
use serde_json::Value;
use std::path::PathBuf;

fn load() -> Option<Vec<(Vec<u64>, String)>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../../reverse-engineering/tiny-glade/evidence/render-recovery/parallel-batch-06/temporal-lifecycle/native-oracle.json",
    );
    let text = std::fs::read_to_string(path).ok()?;
    let rows: Vec<Value> = serde_json::from_str(&text).ok()?;
    Some(
        rows.iter()
            .map(|r| {
                (
                    r["input"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap()).collect(),
                    r["native"].as_str().unwrap().to_owned(),
                )
            })
            .collect(),
    )
}

fn q0(hex: &str) -> u64 {
    let b: Vec<u8> = (0..8).map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap()).collect();
    u64::from_le_bytes(b.try_into().unwrap())
}

#[test]
fn compile_take_and_restore_ownership_matches_original() {
    let Some(rows) = load() else {
        eprintln!("skip temporal-lifecycle: evidence missing");
        return;
    };
    assert_eq!(rows.len(), 1101);
    let (mut take, mut restore, mut leaf) = (0, 0, 0);
    for (input, native) in &rows {
        let mode = input[0];
        if mode == 2 {
            // should_reset_temporal：word == 0；rhapsody 内无该函数，仅核对期望值格式。
            assert_eq!(native, if input[1] == 0 { "1" } else { "0" });
            leaf += 1;
            continue;
        }
        let (graph_present, temporal_present) = (input[1] != 0, input[2] != 0);
        let mut parts = native.split('|');
        let events = parts.next().unwrap();
        let graph_hex = parts.next().unwrap();
        let temporal_hex = parts.next().unwrap();
        let mut state = PersistentCompileState {
            graph: graph_present.then(RenderGraph::new),
            temporal: temporal_present.then(TemporalRenderGraphState::default),
        };
        if mode == 0 {
            let taken = state.take_for_compile();
            let expect_events = match &taken {
                Err(TakeFailure::GraphMissing) => "graph-missing",
                Err(TakeFailure::TemporalMissing { .. }) => "temporal-missing",
                Ok(_) => "taken",
            };
            assert_eq!(expect_events, events, "take {input:?}");
            // 取走后：graph 首 qword = None 哨兵 (1<<63)；temporal 首 qword = 0 表示已取走，
            // 图缺失时 temporal 不动（保持输入的 present 值）。
            assert_eq!(state.graph.is_none(), q0(graph_hex) == 1 << 63, "graph {input:?}");
            assert_eq!(state.temporal.is_some(), q0(temporal_hex) != 0, "temporal {input:?}");
            take += 1;
        } else {
            // 注入 post-export：新 temporal 总是写入（首 qword=1）；旧值存在才触发 drop。
            let had_old = state.temporal.is_some();
            state.restore_temporal(TemporalRenderGraphState::default());
            assert_eq!(had_old, events == "drop-old-records", "restore {input:?}");
            assert!(state.temporal.is_some());
            assert_eq!(q0(temporal_hex), 1);
            assert_eq!(state.graph.is_some(), q0(graph_hex) != 1 << 63, "graph kept {input:?}");
            restore += 1;
        }
    }
    assert_eq!((take, restore, leaf), (48, 48, 1005));
}
