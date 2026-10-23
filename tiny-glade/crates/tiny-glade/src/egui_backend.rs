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
//!   called on it with the freshly produced delta; it remains owned by the backend;
//! * the `ViewportId → ViewportOutput` table is explicitly dropped
//!   (`drop_inner_table<ViewportId, ViewportOutput>`), and the 15-qword
//!   `PlatformOutput` is copied to the sret slot (not `FullOutput`).
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
/// Fields correspond to shapes `+0x00`, textures delta `+0x18`, context
/// `+0x60` in B. This semantic adapter does not promise the native Rust ABI.
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

    /// Move this frame's shapes and texture changes to a backend for the render
    /// thread (same context; the game thread starts the next egui frame).
    pub fn hand_over(&mut self) -> Self {
        Self {
            shapes: std::mem::take(&mut self.shapes),
            textures_delta: std::mem::take(&mut self.textures_delta),
            context: self.context.clone(),
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
    /// 5. return only the platform output to the OS thread. The graph consumes
    ///    the shapes and texture deltas from this backend, not from that channel.
    pub fn end_frame(&mut self) -> egui::PlatformOutput {
        let mut output = self.context.end_frame();

        // Step 2: the original drops the viewport-output hash table.
        output.viewport_output.clear();

        // Step 3: drop old shapes, then move the new shapes into the backend.
        // `clear` runs drop_in_place<Shape> for each element; `swap` performs the
        // recovered header replacement without aliasing the returned Vec.
        self.shapes.clear();
        std::mem::swap(&mut self.shapes, &mut output.shapes);

        // Step 4/5: retain GPU work and return the OS-side output. B copies
        // exactly 15 qwords to sret; FullOutput also owns shapes/deltas/viewports.
        self.textures_delta
            .append(std::mem::take(&mut output.textures_delta));
        output.platform_output
    }

    /// B `EguiBackend::draw` (`0x1400ddc80`).
    ///
    /// Uploads retained texture sets, tessellates cached shapes using the
    /// context's pixels_per_point, draws, then releases the pending texture IDs.
    /// B's texture remove_entry loop follows the per-primitive draw loop.
    pub fn draw(
        &mut self,
        screen_size_pixels: [u32; 2],
        renderer: &mut dyn EguiRenderer,
    ) -> Result<()> {
        for (id, image_delta) in &self.textures_delta.set {
            renderer.update_texture(*id, image_delta)?;
        }
        let shapes = std::mem::take(&mut self.shapes);
        let primitives = self.context.tessellate(shapes, self.context.pixels_per_point());
        renderer.draw_primitives(&primitives, screen_size_pixels)?;
        self.textures_delta.set.clear();
        for id in self.textures_delta.free.drain(..) {
            renderer.free_texture(id)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum Call {
        Upload(egui::TextureId),
        Draw,
        Free(egui::TextureId),
    }

    #[derive(Default)]
    struct Renderer {
        calls: Vec<Call>,
        textures_used: Vec<egui::TextureId>,
    }
    impl EguiRenderer for Renderer {
        fn update_texture(&mut self, id: egui::TextureId, _: &egui::epaint::ImageDelta) -> Result<()> {
            self.calls.push(Call::Upload(id));
            Ok(())
        }
        fn free_texture(&mut self, id: egui::TextureId) -> Result<()> {
            self.calls.push(Call::Free(id));
            Ok(())
        }
        fn draw_primitives(&mut self, primitives: &[egui::ClippedPrimitive], _: [u32; 2]) -> Result<()> {
            for primitive in primitives {
                if let egui::epaint::Primitive::Mesh(mesh) = &primitive.primitive {
                    self.textures_used.push(mesh.texture_id);
                }
            }
            self.calls.push(Call::Draw);
            Ok(())
        }
    }

    #[test]
    fn platform_output_does_not_steal_gpu_updates_or_free_a_texture_before_its_draw() {
        let context = egui::Context::default();
        let mut backend = EguiBackend::new(context.clone());
        context.begin_frame(egui::RawInput::default());
        let texture = context.load_texture(
            "frame-texture",
            egui::ColorImage::new([2, 2], egui::Color32::WHITE),
            egui::TextureOptions::LINEAR,
        );
        let id = texture.id();
        context.layer_painter(egui::LayerId::background()).image(
            id,
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(16.0, 16.0)),
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
        context.output_mut(|output| output.copied_text = "OS output".into());
        drop(texture);
        assert_eq!(backend.end_frame().copied_text, "OS output");
        assert!(backend.textures_delta.set.iter().any(|(key, _)| *key == id));
        let mut renderer = Renderer::default();
        backend.draw([64, 64], &mut renderer).unwrap();
        let upload = renderer.calls.iter().position(|call| *call == Call::Upload(id)).unwrap();
        let draw = renderer.calls.iter().position(|call| *call == Call::Draw).unwrap();
        let free = renderer.calls.iter().position(|call| *call == Call::Free(id)).unwrap();
        assert!(upload < draw && draw < free);
        assert!(renderer.textures_used.contains(&id));
        assert!(backend.textures_delta.is_empty());
    }
}
