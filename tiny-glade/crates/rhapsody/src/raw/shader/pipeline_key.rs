//! Raster pipeline cache keys: every field that affects the Vulkan pipeline.
use super::*;

#[cfg(test)]
pub(super) fn append_dynamic_rendering_key(
    key: &mut Vec<u64>,
    formats: Option<&DynamicRenderingFormats>,
) {
    match formats {
        None => key.push(0),
        Some(formats) => {
            key.extend([1, u64::from(formats.view_mask), formats.colors.len() as u64]);
            key.extend(formats.colors.iter().map(|format| format.as_raw() as u64));
            key.extend([
                formats.depth.as_raw() as u64,
                formats.stencil.as_raw() as u64,
            ]);
        }
    }
}

/// The key covers every RasterPipelineDesc field that affects the Vulkan pipeline;不使用不稳定 Debug 字串或哈希碰撞替代相等性。
pub(crate) fn raster_pipeline_key_len(desc: &RasterPipelineDesc) -> usize {
    let fragment = desc.fragment_entry.as_deref().unwrap_or(&desc.entry);
    // Nineteen fixed state words and four following sequence lengths. Native
    // render-pass/subpass fields were removed with the dynamic-only contract.
    23usize
        .saturating_add(
            desc.dynamic_rendering
                .as_ref()
                .map_or(1, |f| 5usize.saturating_add(f.colors.len())),
        )
        .saturating_add(desc.entry.as_bytes().len())
        .saturating_add(fragment.to_bytes().len())
        .saturating_add(desc.vertex_bindings.len().saturating_mul(3))
        .saturating_add(desc.vertex_attributes.len().saturating_mul(4))
        .saturating_add(desc.blend_attachments.len().saturating_mul(8))
}
pub(crate) fn raster_pipeline_key_matches(desc: &RasterPipelineDesc, key: &[u64]) -> bool {
    if key.len() != raster_pipeline_key_len(desc) {
        return false;
    }
    let mut index = 0;
    let mut equal = true;
    visit_raster_pipeline_key(desc, &mut |word| {
        equal &= key[index] == word;
        index += 1;
    });
    equal
}
pub(crate) fn raster_pipeline_key(desc: &RasterPipelineDesc) -> Vec<u64> {
    let mut key = Vec::with_capacity(raster_pipeline_key_len(desc));
    visit_raster_pipeline_key(desc, &mut |word| key.push(word));
    key
}
pub(super) fn visit_raster_pipeline_key(desc: &RasterPipelineDesc, emit: &mut dyn FnMut(u64)) {
    struct Writer<'a>(&'a mut dyn FnMut(u64));
    impl Writer<'_> {
        fn push(&mut self, value: u64) {
            (self.0)(value);
        }
        fn extend(&mut self, values: impl IntoIterator<Item = u64>) {
            for value in values {
                self.push(value);
            }
        }
    }
    use ash::vk::Handle;
    let mut key = Writer(emit);
    key.extend([
        desc.vertex.handle.as_raw(),
        desc.fragment
            .as_ref()
            .map_or(0, |shader| shader.handle.as_raw()),
        desc.layout.handle.as_raw(),
        desc.topology.as_raw() as u64,
        desc.polygon_mode.as_raw() as u64,
        u64::from(desc.cull_mode.as_raw()),
        desc.front_face.as_raw() as u64,
        u64::from(desc.samples.as_raw()),
        u64::from(desc.depth_test),
        u64::from(desc.depth_write),
        desc.depth_compare.as_raw() as u64,
        u64::from(desc.stencil_test),
        desc.stencil.fail_op.as_raw() as u64,
        desc.stencil.pass_op.as_raw() as u64,
        desc.stencil.depth_fail_op.as_raw() as u64,
        desc.stencil.compare_op.as_raw() as u64,
        u64::from(desc.stencil.compare_mask),
        u64::from(desc.stencil.write_mask),
        desc.entry.as_bytes().len() as u64,
    ]);
    match desc.dynamic_rendering.as_ref() {
        None => key.push(0),
        Some(formats) => {
            key.extend([1, u64::from(formats.view_mask), formats.colors.len() as u64]);
            key.extend(formats.colors.iter().map(|format| format.as_raw() as u64));
            key.extend([
                formats.depth.as_raw() as u64,
                formats.stencil.as_raw() as u64,
            ]);
        }
    }
    key.extend(desc.entry.as_bytes().iter().map(|byte| u64::from(*byte)));
    let fragment_entry = desc.fragment_entry.as_deref().unwrap_or(&desc.entry);
    key.push(fragment_entry.to_bytes().len() as u64);
    key.extend(
        fragment_entry
            .to_bytes()
            .iter()
            .map(|byte| u64::from(*byte)),
    );
    key.push(desc.vertex_bindings.len() as u64);
    for binding in &desc.vertex_bindings {
        key.extend([
            u64::from(binding.binding),
            u64::from(binding.stride),
            binding.input_rate.as_raw() as u64,
        ]);
    }
    key.push(desc.vertex_attributes.len() as u64);
    for attribute in &desc.vertex_attributes {
        key.extend([
            u64::from(attribute.location),
            u64::from(attribute.binding),
            attribute.format.as_raw() as u64,
            u64::from(attribute.offset),
        ]);
    }
    key.push(desc.blend_attachments.len() as u64);
    for blend in &desc.blend_attachments {
        key.extend([
            u64::from(blend.blend_enable),
            blend.src_color_blend_factor.as_raw() as u64,
            blend.dst_color_blend_factor.as_raw() as u64,
            blend.color_blend_op.as_raw() as u64,
            blend.src_alpha_blend_factor.as_raw() as u64,
            blend.dst_alpha_blend_factor.as_raw() as u64,
            blend.alpha_blend_op.as_raw() as u64,
            u64::from(blend.color_write_mask.as_raw()),
        ]);
    }
}
