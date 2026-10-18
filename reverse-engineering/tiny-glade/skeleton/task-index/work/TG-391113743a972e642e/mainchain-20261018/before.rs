//! `tiny_glade::egui_backend` — egui frame extraction and draw submission.
//!
//! Recovered VAs: `EguiBackend::end_frame` `0x1400dd910`,
//! `EguiBackend::draw` `0x1400ddc80`.
//!
//! `end_frame` layout recovered from the fresh disassembly:
//! * `self.context` is at `+0x60` (`ADD RDX,0x60` before
//!   `egui::Context::end_frame`);
//! * `self.shapes` is the Vec at `+0x00` (capacity/ptr/len at `+0/+8/+0x10`,
//!   element stride `0x60`), dropped one `Shape` at a time and then replaced by
//!   the output's Vec header;
//! * `self.textures_delta` is at `+0x18`; `epaint::TexturesDelta::append` is
//!   called on it with the freshly produced delta, then the combined delta is
//!   handed back in the `FullOutput`;
//! * the `ViewportId → ViewportOutput` table is explicitly dropped
//!   (`drop_inner_table<ViewportId, ViewportOutput>`), and the 15-qword
//!   `FullOutput` is copied to the sret slot.
//!
//! `draw` iterates the tessellated `ClippedPrimitive`s and uploads
//! textures/buffers through rhapsody; that device plumbing stays behind
//! [`EguiRenderer`]. The recovered shapes are consumed from `self.shapes`.

use anyhow::Result;

/// GPU submission interface (rhapsody render graph in the original).
pub trait EguiRenderer {
    fn update_texture(
        &mut self,
        id: egui::TextureId,
        delta: &egui::epaint::ImageDelta,
    ) -> Result<()>;
    fn free_texture(&mut self, id: egui::TextureId) -> Result<()>;
    fn draw_primitives(
        &mut self,
        primitives: &[egui::ClippedPrimitive],
        screen_size_pixels: [u32; 2],
    ) -> Result<()>;
}

/// B `tiny_glade::egui_backend::EguiBackend`.
///
/// Field order mirrors the recovered offsets: shapes `+0x00`, textures delta
/// `+0x18`, context `+0x60`.
pub struct EguiBackend {
    shapes: Vec<egui::epaint::ClippedShape>,
    textures_delta: egui::TexturesDelta,
    context: egui::Context,
}

impl EguiBackend {
    pub fn new(context: egui::Context) -> Self {
        Self {
            shapes: Vec::new(),
            textures_delta: egui::TexturesDelta::default(),
            context,
        }
    }

    pub fn context(&self) -> &egui::Context {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut egui::Context {
        &mut self.context
    }

    /// B `EguiBackend::end_frame` (`0x1400dd910`).
    ///
    /// Steps (in machine-code order):
    /// 1. `Context::end_frame()` into a stack `FullOutput`;
    /// 2. drop the `ViewportId → ViewportOutput` table;
    /// 3. drop every old `Shape` in `self.shapes` (stride `0x60`), deallocate
    ///    its buffer, then overwrite the Vec header with the output's shapes;
    /// 4. `self.textures_delta.append(output.textures_delta)`;
    /// 5. return the `FullOutput` with the combined delta.
    pub fn end_frame(&mut self) -> egui::FullOutput {
        let mut output = self.context.end_frame();

        // Step 2: the original drops the viewport-output hash table.
        output.viewport_output.clear();

        // Step 3: drop old shapes, then move the new shapes into the backend.
        // `clear` runs drop_in_place<Shape> for each element; `swap` performs the
        // recovered header replacement without aliasing the returned Vec.
        self.shapes.clear();
        std::mem::swap(&mut self.shapes, &mut output.shapes);

        // Step 4/5: accumulate then hand the combined delta back.
        self.textures_delta
            .append(std::mem::take(&mut output.textures_delta));
        output.textures_delta = std::mem::take(&mut self.textures_delta);
        output
    }

    /// B `EguiBackend::draw` (`0x1400ddc80`).
    ///
    /// Applies the frame's texture deltas, tessellates the shapes cached by
    /// `end_frame` with the frame's `pixels_per_point`, then hands the
    /// primitives to the renderer.
    pub fn draw(
        &mut self,
        output: egui::FullOutput,
        screen_size_pixels: [u32; 2],
        renderer: &mut dyn EguiRenderer,
    ) -> Result<()> {
        for (id, image_delta) in &output.textures_delta.set {
            renderer.update_texture(*id, image_delta)?;
        }
        for id in &output.textures_delta.free {
            renderer.free_texture(*id)?;
        }

        let shapes = std::mem::take(&mut self.shapes);
        let primitives = self.context.tessellate(shapes, output.pixels_per_point);
        renderer.draw_primitives(&primitives, screen_size_pixels)
    }
}
