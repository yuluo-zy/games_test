//! Runtime resolution publication, recovered from main_loop B
//! 1404dd1f0..1404dd6ac. Startup deliberately copies the three raw extents;
//! each frame then publishes the scaled output and aligned inner extent.
use bevy_ecs::prelude::*;
use country_core::{game_config::GameConfig, resources::window::WindowSize,
    render::render_config::{FrameRenderConfig, VideoSettings, UpscalingMode}};

pub(crate) fn dimensions(window: WindowSize, video: &VideoSettings, mut frame: FrameRenderConfig) -> FrameRenderConfig {
    let raw = window.as_dims();
    frame.extent_0 = raw;
    // The effective scale belongs to adaptive/OOM control; video retains the requested target.
    frame.tgsr_resolution_scale = if video.upscaling_mode == UpscalingMode::Tgsr {
        video.upscaling_preset.tgsr_resolution_scale()
    } else { 1.0 };
    if raw.contains(&0) {
        // The native zero-extent gate precedes this frame's resolution work.
        frame.extent_8 = raw;
        frame.extent_10 = raw;
        return frame;
    }
    // Preserve the f32 multiplication order and saturating conversion: the
    // inner extent is computed from the untruncated scaled dimensions.
    let scaled = raw.map(|axis| axis as f32 * frame.world_resolution_scale);
    frame.extent_8 = scaled.map(|axis| (axis as u32).clamp(32, 10_912));
    frame.extent_10 = scaled.map(|axis|
        (((axis * frame.tgsr_resolution_scale + 31.0) as u32) & !31).clamp(32, 10_912));
    frame
}

pub(crate) fn update(window: Option<Res<WindowSize>>, config: Option<Res<GameConfig>>,
    frame: Option<ResMut<FrameRenderConfig>>,
    adaptive: Option<ResMut<country_core::render::adaptive_render_state::AdaptiveRenderState>>) {
    let (Some(window), Some(config), Some(mut frame)) = (window, config, frame) else { return; };
    if let Some(mut adaptive) = adaptive {
        let mut next = *adaptive;
        if next.request_scale(config.video.world_resolution_scale) {
            frame.world_resolution_scale = config.video.world_resolution_scale.unwrap_or(1.0);
        }
        if next != *adaptive { *adaptive = next; }
    } else if let Some(scale) = config.video.world_resolution_scale {
        frame.world_resolution_scale = scale;
    }
    let next = dimensions(*window, &config.video, *frame);
    if *frame != next { *frame = next; }
}

/// Resize and an OOM retry need the same publication before another Update.
pub(crate) fn refresh(world: &mut World) {
    let Some(frame) = world.get_resource::<FrameRenderConfig>().copied() else { return; };
    let (Some(window), Some(config)) = (world.get_resource::<WindowSize>(), world.get_resource::<GameConfig>()) else { return; };
    let next = dimensions(*window, &config.video, frame);
    if frame != next { *world.resource_mut::<FrameRenderConfig>() = next; }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::render::render_config::UpscalingPreset;
    #[test]
    fn scaled_output_and_aligned_inner_use_the_same_untruncated_dimensions() {
        let mut video = VideoSettings::default();
        video.world_resolution_scale = Some(0.75);
        video.upscaling_mode = UpscalingMode::Tgsr;
        video.upscaling_preset = UpscalingPreset::Quality;
        let window = WindowSize::new(514, 354);
        let frame = dimensions(window, &video, FrameRenderConfig::at_startup(window, &video));
        assert_eq!(frame.extent_0, [514, 354]);
        assert_eq!(frame.extent_8, [385, 265]);
        assert_eq!(frame.extent_10, [288, 192]);
        video.upscaling_mode = UpscalingMode::Off;
        assert_eq!(dimensions(window, &video, frame).extent_10, [416, 288]);
    }
    #[test]
    fn automatic_mode_retains_the_existing_scale_without_resetting_each_frame() {
        let video = VideoSettings::default();
        let window = WindowSize::new(640, 360);
        let mut frame = FrameRenderConfig::at_startup(window, &video);
        frame.world_resolution_scale = 0.5;
        let next = dimensions(window, &video, frame);
        assert_eq!(next.world_resolution_scale, 0.5);
        assert_eq!(next.extent_8, [320, 180]);
        assert_eq!(dimensions(window, &video, next), next);
    }
    #[test]
    fn zero_extent_stays_suspended_and_conversion_limits_are_bounded() {
        let mut video = VideoSettings::default();
        let frame = FrameRenderConfig::at_startup(WindowSize::new(0, 720), &video);
        assert_eq!(dimensions(WindowSize::new(0, 720), &video, frame).extent_10, [0, 720]);
        for (scale, expected) in [(f32::NAN, 32), (-1.0, 32), (f32::INFINITY, 10_912)] {
            video.world_resolution_scale = Some(scale);
            let actual = dimensions(WindowSize::new(100, 100), &video, FrameRenderConfig { world_resolution_scale: scale, ..frame });
            assert_eq!(actual.extent_8, [expected; 2]);
            assert_eq!(actual.extent_10, [expected; 2]);
        }
    }
}

/// Run once on the resident render owner before declaring resources and camera
/// constants. The game only publishes requests and consumes field-wise feedback.
pub(crate) fn adapt(world: &mut World, device: &rhapsody::device::Device) {
    use country_core::render::adaptive_render_state::AdaptiveRenderState;
    let (Some(frame), Some(config), Some(state)) = (
        world.get_resource::<FrameRenderConfig>().copied(),
        world.get_resource::<GameConfig>(), world.get_resource::<AdaptiveRenderState>().copied()
    ) else { return; };
    if frame.extent_0.contains(&0) { return; }
    let requested = config.video.world_resolution_scale;
    // Platform adaptation: unavailable telemetry is unknown, not zero bytes.
    // Never substitute allocator heap capacity for a driver memory budget.
    let maximum = if state.enabled && device.memory_budget_enabled() {
        crate::vram::calculate_max_resolution_scale(device, frame.extent_0)
    } else { f32::MAX };
    let mut next = state;
    let current = if next.request_scale(requested) { requested.unwrap_or(1.0) } else { frame.world_resolution_scale };
    let scale = next.update(std::time::Instant::now(), current, requested.unwrap_or(1.0), maximum);
    if next != state { *world.resource_mut::<AdaptiveRenderState>() = next; }
    if scale.to_bits() != frame.world_resolution_scale.to_bits() {
        world.resource_mut::<FrameRenderConfig>().world_resolution_scale = scale;
        refresh(world);
    }
}
