/// Original PlayUiSound business payload is 24B. UiSounds stores four such
/// values. Native Events<EventInstance<PlayUiSound>> appends its own 8B EventId;
/// that counter is not part of the game event and must not be stored here.
#[repr(C)]
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlayUiSound {
    pub kind: u32,
    /// Native +4..24, including nested enum tags at +4. Never drop this word.
    pub _payload: [u8; 20],
}
impl PlayUiSound {
    pub fn from_native_slot(slot: [u8; 24]) -> Self {
        Self { kind: u32::from_le_bytes(slot[..4].try_into().unwrap()),
            _payload: slot[4..].try_into().unwrap() }
    }
    pub fn native_slot(self) -> [u8; 24] {
        let mut slot = [0; 24];
        slot[..4].copy_from_slice(&self.kind.to_le_bytes());
        slot[4..].copy_from_slice(&self._payload);
        slot
    }
    pub fn variant4() -> Self { Self { kind: 4, ..Default::default() } }
}

