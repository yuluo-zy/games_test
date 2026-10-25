/// Native140b2d4f9..514 uses std Arc::get_mut's strong/weak uniqueness test.
/// It must fail rather than copying params when another owner remains alive.
/// The creation caller unwraps this result at its original panic boundary.
pub fn active_params_mut(&mut self) -> Option<&mut WidgetParams> {
    self.active_params.as_mut().and_then(Arc::get_mut)
}
