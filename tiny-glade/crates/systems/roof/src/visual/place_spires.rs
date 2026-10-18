//! B `place_roof_spires@142196e70` (1943B), including its actual
//! `filter_map` closure@1419ee9e0 (337B). Canonical 32B records feed the
//! existing SSBO/device APIs; no intermediate ECS record resource is needed.
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    render::{device::GraphicsDeviceResource, ssbo::rhapsody_boundary::GraphicsDevice},
    resources::{
        roof_tile_ssbo::RoofSpireSsbo,
        roofs::{Roof, RoofAnimation, RoofRidgeDims},
    },
};

/// Original ECS inputs are device, filtered query, SSBO library. ResMut is
/// the current library's safe `get_mut` adapter, rather than its old interior
/// mutable storage representation.
pub fn place_roof_spires(
    device: Res<GraphicsDeviceResource>,
    spires: Query<(&Roof, &RoofAnimation), With<crate::utils::components::RoofSpire>>,
    mut ssbo: ResMut<AssetSsboLibrary>,
) {
    place_with_device(device.device(), &spires, &mut ssbo);
}

fn place_with_device<'a>(
    device: &dyn GraphicsDevice,
    spires: impl IntoIterator<Item = (&'a Roof, &'a RoofAnimation)>,
    ssbo: &mut AssetSsboLibrary,
) {
    // @1421970d5..148: lookup precedes query collection. Absence logs and
    // returns; it does not create a replacement buffer or change any count.
    let Some(storage) = ssbo.get_mut::<RoofSpireSsbo>() else {
        log::warn!("ssbo_library doesn't have RoofSpireSsbo ready");
        return;
    };
    let records: Vec<_> = spires
        .into_iter()
        .filter_map(|(roof, animation)| spire_record(roof, animation))
        .collect();
    // @1421971f6..260: an empty result skips upload; a failed nonempty
    // upload panics before the separate instance_count write.
    if !records.is_empty() {
        storage
            .buffer
            .gpu_write(device, &records)
            .expect("called `Result::unwrap()` on an `Err` value: update_buffer_bytes");
    }
    storage.instance_count = records.len();
}

/// `place_roof_spires::closure$0@1419ee9e0`. The two reciprocal ratios use
/// the original numeric max semantics, and the comparison rejects unordered
/// values just as `ucomiss threshold, ratio; jbe` does.
fn spire_record(roof: &Roof, animation: &RoofAnimation) -> Option<RoofSpireSsbo> {
    if !roof.is_pitched() {
        return None;
    }
    let [x, y] = RoofRidgeDims::from(roof).get();
    let a = x / y;
    let b = y / x;
    let ratio = if a.is_nan() || b > a { b } else { a };
    if !(ratio < f32::from_bits(0x3f8147ae)) {
        return None;
    }
    let pivot = roof.pivot_3d();
    let tip = roof.roof_tip_offset_xz();
    let height = roof.height();
    let zero_height = 0.0 * height;
    Some(RoofSpireSsbo {
        position: glam::Vec4::new(
            (pivot.x + tip[0]) + zero_height,
            height + (0.0 + pivot.y),
            (tip[1] + pivot.z) + zero_height,
            0.0,
        ),
        roof_animation_t: animation.values()[1],
        roof_profile: f32::from_le_bytes(roof.observed()[0x2c..0x30].try_into().unwrap()),
        radius: 0.25,
        pad1: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ops::Range;
    use country_core::render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc};
    use std::cell::RefCell;

    #[derive(Default)]
    struct Recorder {
        uploads: RefCell<Vec<(Vec<u8>, Vec<Range<usize>>, usize)>>,
        fail: bool,
    }
    impl GraphicsDevice for Recorder {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            _: &str,
            initial: Option<&[u8]>,
        ) -> Result<Buffer, BackendError> {
            assert!(
                initial.is_none(),
                "native roof constructor must not upload zeroes"
            );
            Ok(Buffer {
                byte_size: desc.size as u64,
                ..Default::default()
            })
        }
        fn update_buffer_bytes(
            &self,
            _: &Buffer,
            bytes: &[u8],
            ranges: &[Range<usize>],
            offset: usize,
        ) -> Result<(), BackendError> {
            if self.fail {
                return Err(BackendError::unsupported(
                    "explicit CPU upload failure fixture",
                ));
            }
            self.uploads
                .borrow_mut()
                .push((bytes.to_vec(), ranges.to_vec(), offset));
            Ok(())
        }
    }

    // Explicit CPU fixtures for the existing observed Roof API. They do not
    // stand in for a roof/session startup producer or a real GPU allocation.
    fn roof(rect: bool, height: f32, ridge: f32) -> Roof {
        let mut b = [0; 88];
        b[8..12].copy_from_slice(&(rect as u32).to_le_bytes());
        let floats = if rect {
            vec![
                (12, 1.0),
                (16, 0.0),
                (20, 3.0),
                (24, -4.0),
                (28, 2.0),
                (32, 8.0),
            ]
        } else {
            vec![(12, 3.0), (16, -4.0), (20, 1.0)]
        };
        for (offset, value) in floats.into_iter().chain([
            (0x24, 0.25),
            (0x28, -0.5),
            (0x2c, 0.375),
            (0x30, height),
            (0x34, ridge),
            (0x4c, 2.0),
        ]) {
            b[offset..offset + 4].copy_from_slice(&f32::to_le_bytes(value));
        }
        Roof::from_observed(b)
    }
    fn animation() -> RoofAnimation {
        let mut bytes = [0; 8];
        bytes[..4].copy_from_slice(&0.25f32.to_le_bytes());
        bytes[4..].copy_from_slice(&0.75f32.to_le_bytes());
        RoofAnimation::from_inner(
            country_core::resources::roofs::components::RoofAnimation::from_observed(bytes),
        )
    }

    #[test]
    fn native_spire_producer_filters_roofs_and_uploads_real_records() {
        let device = Recorder::default();
        let mut library = AssetSsboLibrary::new();
        library.add_new::<RoofSpireSsbo>(&device);
        let anim = animation();
        let circle = roof(false, 0.5, 0.0);
        let flat = roof(false, 0.1, 0.0);
        let long_ridge = roof(true, 0.5, 0.5);
        let unordered = roof(true, 0.5, f32::NAN);
        place_with_device(
            &device,
            [&circle, &flat, &long_ridge, &unordered].map(|r| (r, &anim)),
            &mut library,
        );
        assert_eq!(library.instance_count::<RoofSpireSsbo>(), 1);
        let uploads = device.uploads.borrow();
        assert_eq!(uploads.len(), 1);
        assert_eq!(
            (uploads[0].0.len(), uploads[0].1.as_slice(), uploads[0].2),
            (32, &[0..32][..], 0)
        );
        let words: Vec<u32> = uploads[0]
            .0
            .chunks_exact(4)
            .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
            .collect();
        assert_eq!(
            words,
            [
                3.25f32.to_bits(),
                8.2f32.to_bits(),
                (-4.5f32).to_bits(),
                0,
                0.75f32.to_bits(),
                0.375f32.to_bits(),
                0.25f32.to_bits(),
                0
            ]
        );
        drop(uploads);
        place_with_device(&device, core::iter::empty(), &mut library);
        assert_eq!(library.instance_count::<RoofSpireSsbo>(), 0);
        assert_eq!(
            device.uploads.borrow().len(),
            1,
            "empty result skips GPU upload"
        );
    }

    #[test]
    fn native_spire_count_is_written_only_after_successful_upload() {
        let device = Recorder::default();
        let mut library = AssetSsboLibrary::new();
        library.add_new::<RoofSpireSsbo>(&device);
        library.get_mut::<RoofSpireSsbo>().unwrap().instance_count = 9;
        let anim = animation();
        let roof = roof(false, 0.5, 0.0);
        let failure = Recorder {
            fail: true,
            ..Default::default()
        };
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            place_with_device(&failure, [(&roof, &anim)], &mut library)
        }));
        assert!(result.is_err());
        assert_eq!(library.instance_count::<RoofSpireSsbo>(), 9);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            place_with_device(
                &device,
                core::iter::repeat_n((&roof, &anim), 1001),
                &mut library,
            )
        }));
        assert!(result.is_err());
        assert_eq!(library.instance_count::<RoofSpireSsbo>(), 9);
        assert!(
            device.uploads.borrow().is_empty(),
            "over capacity must fail before upload"
        );
        let mut absent = AssetSsboLibrary::new();
        place_with_device(&device, [(&roof, &anim)], &mut absent);
        assert!(!absent.contains::<RoofSpireSsbo>());
    }
}
