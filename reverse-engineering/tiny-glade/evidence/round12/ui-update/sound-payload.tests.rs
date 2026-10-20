#[cfg(test)]
mod native_payload_tests {
    use super::*;
    #[test]
    fn original_24_byte_ui_slot_preserves_nested_word_and_has_no_event_id() {
        let mut slot = [0u8; 24];
        for (i, b) in slot.iter_mut().enumerate() { *b = (i * 7 + 1) as u8; }
        let sound = PlayUiSound::from_native_slot(slot);
        assert_eq!(sound.native_slot(), slot);
        assert_eq!(std::mem::size_of::<PlayUiSound>(), 24);
        assert_eq!(sound._payload[..4], slot[4..8]);
    }
}
