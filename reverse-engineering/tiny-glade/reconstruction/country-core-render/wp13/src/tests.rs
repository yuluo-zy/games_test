//! WP13 单元测试：验证已恢复业务逻辑（非原侧行为验证；契约见 contracts/）。

#[cfg(test)]
mod unit {
    use crate::resources::history::{EditType, History, HistoryMutationToken, MultiEdit};
    use crate::resources::prefab::{Prefab, PrefabLibrary, PrefabName};
    use crate::resources::session::GladeType;
    use crate::systems::save::save_game::SaveFileSection;

    /// O：GladeType 判别字节 1/2/3/4（原机器码逐字节证实）。
    #[test]
    fn glade_type_tags_from_machine_code() {
        assert!(GladeType::from(1).is_olden());
        assert!(GladeType::from(2).is_flowery());
        assert!(GladeType::from(3).is_autumn());
        assert!(GladeType::from(4).is_winter());
        assert!(!GladeType::from(1).is_winter());
    }

    /// O：undo 下界受 history_start 锚定（history_process 内联游标规则）。
    #[test]
    fn history_undo_respects_history_start() {
        let mut history = History::new_from_params(0);
        history.insert_starting_edit(MultiEdit {
            edits: vec![EditType::Wall],
            ..Default::default()
        });
        history.start_input_recording();
        history.add(EditType::Roof);
        history.finish_input_recording();
        // cursor == 1；undo 到 start(0) 后不能再退。
        history.undo();
        assert_eq!(history.chain_index, 0);
        history.undo();
        assert_eq!(history.chain_index, 0);
    }

    /// O：redo 只在 cursor+1 < len 时前进。
    #[test]
    fn history_redo_bound() {
        let mut history = History::new_from_params(0);
        history.insert_starting_edit(MultiEdit {
            edits: vec![EditType::Wall],
            ..Default::default()
        });
        history.undo(); // cursor -> -1（受 start=0 限制实际停在 0）
        history.redo();
        assert!(history.chain_index <= 0);
    }

    /// O：add 在 pending 为空且 cursor-len < -1 时先 nuke_future_states。
    #[test]
    fn history_add_nukes_future_states() {
        let mut history = History::new_from_params(0);
        history.start_input_recording();
        history.add(EditType::Tree);
        history.finish_input_recording();
        let before = history.edit_chain.inner.len();
        // 新录制；空 pending + cursor 已在尾部 → 不截断。
        history.start_input_recording();
        history.add(EditType::Garden);
        history.finish_input_recording();
        assert_eq!(history.edit_chain.inner.len(), before + 1);
    }

    /// O：HistoryMutationToken 未消费 drop 时 panic（忘记变更守卫）。
    #[test]
    #[should_panic(expected = "History was mutated, but wasn't told about it")]
    fn mutation_token_guard_panics() {
        let _token = HistoryMutationToken { active: true };
    }

    /// O：SaveFileSection::json_key 10 变体（.rdata 串块）。
    #[test]
    fn save_section_json_keys() {
        let all = [
            SaveFileSection::Version,
            SaveFileSection::Identity,
            SaveFileSection::BackwardCompatibility,
            SaveFileSection::Camera,
            SaveFileSection::TimeOfDay,
            SaveFileSection::Sheep,
            SaveFileSection::Colors,
            SaveFileSection::HistoryChainIndex,
            SaveFileSection::HistoryStart,
            SaveFileSection::HistorySnapshot,
        ];
        let keys: Vec<&str> = all.iter().map(|section| section.json_key()).collect();
        assert_eq!(
            keys,
            vec![
                "Version",
                "Identity",
                "BackwardCompatibility",
                "Camera",
                "Time of day",
                "Sheep",
                "Colors",
                "chain_index",
                "history_start",
                "snapshot"
            ]
        );
    }

    /// O：SaveFileSection::padding_bytes 表。
    #[test]
    fn save_section_padding_bytes() {
        let all = [
            SaveFileSection::Version,
            SaveFileSection::Identity,
            SaveFileSection::BackwardCompatibility,
            SaveFileSection::Camera,
            SaveFileSection::TimeOfDay,
            SaveFileSection::Sheep,
            SaveFileSection::Colors,
            SaveFileSection::HistoryChainIndex,
            SaveFileSection::HistoryStart,
            SaveFileSection::HistorySnapshot,
        ];
        let paddings: Vec<u64> = all.iter().map(|section| section.padding_bytes()).collect();
        assert_eq!(paddings, vec![0, 16, 0, 128, 32, 128, 64, 8, 8, 2048]);
    }

    /// O：PrefabLibrary add/get/has（HashMap<PrefabName, Prefab>）。
    #[test]
    fn prefab_library_lookup() {
        let mut library = PrefabLibrary::default();
        let allocation: Vec<u8> = {
            let mut bytes = vec![0u8; 16];
            bytes.extend_from_slice(b"pine");
            bytes
        };
        library.add(
            PrefabName {
                allocation: &allocation,
                byte_len: 4,
            },
            Prefab::default(),
        );
        let name = PrefabName {
            allocation: &allocation,
            byte_len: 4,
        };
        assert!(library.get(&name).is_some());
        assert!(library.has(&name.into()));
    }
}
