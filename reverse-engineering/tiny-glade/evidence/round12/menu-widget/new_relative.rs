    /// Original relative-frame source adapter. All five native payload fields
    /// are retained; new_root supplies the same per-frame reuse/event lifecycle
    /// as fullscreen, safe_frame and new_at_pos.
    #[allow(clippy::too_many_arguments)]
    pub fn new_relative(
        &mut self,
        name: &str,
        relative_to: GlobalId,
        requested_size: Option<Vec2>,
        offset: Vec2,
        reference_anchor: Vec2,
        anchor: Vec2,
        opaque: bool,
    ) -> NodeContext {
        self.new_root(
            FrameKind::Relative { requested_size, relative_to, offset, reference_anchor, anchor },
            name,
            opaque,
        )
    }
