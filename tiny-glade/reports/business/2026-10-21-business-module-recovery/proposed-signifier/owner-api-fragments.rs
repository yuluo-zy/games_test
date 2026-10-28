// Merge these narrow additions into the central candidate. Not a standalone module.

// WidgetParams field (retain Clone/Debug/PartialEq via factory identity below):
// pub signifiers: Vec<super::super::signifiers::SignifierFactory>,
// WidgetParams::new initializer:
// signifiers: Vec::new(),

// Add inside impl WidgetParams:
pub fn with_signifiers(mut self, signifiers: Vec<super::super::signifiers::SignifierFactory>) -> Self {
    self.signifiers = signifiers;
    self
}

// Add inside impl UiState. Never clone the captured Arc to mutate it.
pub fn active_params_ref(&self) -> Option<&WidgetParams> {
    self.active_params.as_deref()
}
pub fn active_params_mut(&mut self) -> Option<&mut WidgetParams> {
    self.active_params.as_mut().map(|params| {
        std::sync::Arc::get_mut(params)
            .expect("original signifier append requires uniquely owned active WidgetParams")
    })
}
pub fn intersection_element(&self) -> usize {
    u64::from_le_bytes(self.intersection_payload[24..32].try_into().unwrap()) as usize
}

// Add in signifier_factory.rs to preserve the existing WidgetParams PartialEq API.
// Closure equality means shared callable identity, never inferred behavioral equality.
impl PartialEq for SignifierFactory {
    fn eq(&self, other: &Self) -> bool {
        self.payload() == other.payload()
            && self.inner == other.inner
            && self.marker == other.marker
            && match (&self.closure, &other.closure) {
                (Some(a), Some(b)) => std::sync::Arc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
    }
}

// Existing external SignifierId(0) placeholders in wall_splitter.rs and
// structure_outline.rs become SignifierId(0, WidgetId(0)); genuine create
// always overwrites both fields on returned entities before they are queried.
