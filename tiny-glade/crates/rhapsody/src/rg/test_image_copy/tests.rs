use super::*;
#[test]
fn raw_aspects_exclude_depth_stencil_padding() {
    let f = vk::Format::D32_SFLOAT_S8_UINT;
    assert_eq!(
        raw_byte_count(f, [8, 4], vk::ImageAspectFlags::DEPTH).unwrap(),
        128
    );
    assert!(raw_byte_count(f, [8, 4], vk::ImageAspectFlags::STENCIL).is_err());
    assert!(
        raw_byte_count(
            f,
            [8, 4],
            vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL
        )
        .is_err()
    );
}
#[test]
fn raw_bounds_and_unsupported_formats_fail_closed() {
    assert_eq!(
        raw_byte_count(
            vk::Format::B10G11R11_UFLOAT_PACK32,
            [1280, 720],
            vk::ImageAspectFlags::COLOR
        )
        .unwrap(),
        1280 * 720 * 4
    );
    assert_eq!(
        raw_byte_count(
            vk::Format::R16G16B16A16_SFLOAT,
            [8, 4],
            vk::ImageAspectFlags::COLOR
        )
        .unwrap(),
        8 * 4 * 8
    );
    assert_eq!(
        raw_byte_count(
            vk::Format::R8G8B8A8_UNORM,
            [8, 4],
            vk::ImageAspectFlags::COLOR
        )
        .unwrap(),
        8 * 4 * 4
    );
    assert!(raw_byte_count(vk::Format::R8_UNORM, [8, 4], vk::ImageAspectFlags::COLOR).is_err());
    for (format, texel_bytes) in [(vk::Format::R32G32_UINT, 8), (vk::Format::R16G16_SFLOAT, 4)] {
        assert_eq!(raw_byte_count(format, [8, 4], vk::ImageAspectFlags::COLOR).unwrap(),
            8 * 4 * texel_bytes);
        assert!(image_shape(&TextureDesc::new_2d(8, 4, format).sampled()).is_ok());
    }
    assert!(
        raw_byte_count(
            vk::Format::B10G11R11_UFLOAT_PACK32,
            [u32::MAX, u32::MAX],
            vk::ImageAspectFlags::COLOR
        )
        .is_err()
    );
    assert!(
        raw_byte_count(
            vk::Format::B10G11R11_UFLOAT_PACK32,
            [0, 4],
            vk::ImageAspectFlags::COLOR
        )
        .is_err()
    );
}
#[test]
fn shape_keeps_original_six_mips_but_rejects_multisample() {
    let mut d = TextureDesc::new_2d(8, 8, vk::Format::B10G11R11_UFLOAT_PACK32).sampled();
    assert!(image_shape(&d).is_ok());
    d.sample_count = 4;
    assert!(image_shape(&d).is_err());
    d.sample_count = 1;
    d.width = 1280;
    d.height = 720;
    d.mip_levels = 6;
    assert!(image_shape(&d).is_ok());
    d.mip_levels = 7;
    assert!(image_shape(&d).is_err());
}
