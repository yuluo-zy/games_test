//! 检查此前缺失的多平面、压缩、深度格式语义和未知格式边界。
use ash::vk;
use rhapsody::raw::format::*;
#[test]
fn extended_formats_and_plane_aspects() {
    assert_eq!(vk_format_block_bytes(vk::Format::R8G8B8_UNORM), Some(3));
    assert_eq!(
        vk_format_block_bytes(vk::Format::A2B10G10R10_UNORM_PACK32),
        Some(4)
    );
    let astc = vk_format_info(vk::Format::ASTC_10X8_UNORM_BLOCK).unwrap();
    assert_eq!(astc.block_extent, [10, 8, 1]);
    assert_eq!(astc.block_bytes, 16);
    assert_eq!(
        vk_format_valid_aspect_flags(vk::Format::D16_UNORM_S8_UINT),
        vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL
    );
    let planes = vk_format_planes(vk::Format::G8_B8R8_2PLANE_420_UNORM);
    assert_eq!(planes.len(), 2);
    assert_eq!((planes[1].width_divisor, planes[1].height_divisor), (2, 2));
    assert_eq!(planes[1].compatible, vk::Format::R8G8_UNORM);
    assert_eq!(
        vk_format_all_aspects(vk::Format::G8_B8R8_2PLANE_420_UNORM),
        vk::ImageAspectFlags::COLOR | vk::ImageAspectFlags::PLANE_0 | vk::ImageAspectFlags::PLANE_1
    );
    assert_eq!(vk_format_info(vk::Format::from_raw(i32::MAX)), None);
    assert!(vk_format_valid_aspect_flags(vk::Format::UNDEFINED).is_empty());
}

#[test]
fn original_info_writes_ten_bytes_and_keeps_tail() {
    use rhapsody::raw::format::observed;
    assert_eq!(std::mem::size_of::<observed::FormatInfo>(), 10);
    let undefined = observed::vk_format_info(vk::Format::UNDEFINED);
    assert_eq!(undefined.bytes, [0, 0, 0, 0, 0, 0, 0, 0, 1, 1]);
    let info = observed::vk_format_info(vk::Format::BC1_RGB_UNORM_BLOCK);
    assert_eq!(
        (
            info.word_at_00(),
            info.word_at_04(),
            info.byte_at_08(),
            info.byte_at_09()
        ),
        (4, 8, 4, 4)
    );
    let mut output = [0x5a; 16];
    info.write_into(&mut output).unwrap();
    assert_eq!(&output[..10], &info.bytes);
    assert_eq!(&output[10..], &[0x5a; 6]);
    assert!(observed::try_vk_format_info(vk::Format::ETC2_R8G8B8_UNORM_BLOCK).is_err());
    assert!(observed::try_vk_format_valid_aspect_flags(vk::Format::UNDEFINED).is_err());
}

#[test]
fn original_block_return_is_not_public_storage_bytes() {
    use rhapsody::raw::format::{khronos, observed};
    assert_eq!(
        observed::vk_format_block_bytes(vk::Format::R32G32B32A32_SFLOAT),
        1
    );
    assert_eq!(
        khronos::block_bytes(vk::Format::R32G32B32A32_SFLOAT),
        Some(16)
    );
    assert_eq!(
        observed::vk_format_block_bytes(vk::Format::BC1_RGB_UNORM_BLOCK),
        8
    );
    assert_eq!(
        observed::vk_format_block_bytes(vk::Format::BC3_SRGB_BLOCK),
        16
    );
    assert!(observed::try_vk_format_block_bytes(vk::Format::BC1_RGBA_UNORM_BLOCK).is_err());
    assert!(observed::try_vk_format_block_bytes(vk::Format::BC2_UNORM_BLOCK).is_err());
    assert!(observed::try_vk_format_block_bytes(vk::Format::BC6H_UFLOAT_BLOCK).is_err());
    assert!(observed::try_vk_format_block_bytes(vk::Format::BC4_SNORM_BLOCK).is_err());
    assert_eq!(
        khronos::block_bytes(vk::Format::BC6H_UFLOAT_BLOCK),
        Some(16)
    );
    assert!(
        std::panic::catch_unwind(|| observed::vk_format_block_bytes(vk::Format::BC2_UNORM_BLOCK))
            .is_err()
    );
}

#[test]
fn public_metadata_matches_all_original_version_xml_formats() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../evidence/format-registry.json")).unwrap();
    for format in data["formats"].as_array().unwrap() {
        let vk_format = vk::Format::from_raw(format["value"].as_i64().unwrap() as i32);
        let info = khronos::info(vk_format).unwrap();
        assert_eq!(info.block_bytes, format["bytes"].as_u64().unwrap() as u32);
        assert_eq!(
            info.aspect.as_raw(),
            format["aspect"].as_u64().unwrap() as u32
        );
        assert_eq!(
            info.block_extent,
            [
                format["extent"][0].as_u64().unwrap() as u32,
                format["extent"][1].as_u64().unwrap() as u32,
                format["extent"][2].as_u64().unwrap() as u32
            ]
        );
        assert_eq!(
            khronos::planes(vk_format).len(),
            format["planes"].as_array().unwrap().len()
        );
    }
    assert_eq!(data["formats"].as_array().unwrap().len(), 249);
}

#[test]
fn emit_original_comparison_fixture() {
    let data: serde_json::Value =
        serde_json::from_str(include_str!("../evidence/format-registry.json")).unwrap();
    let mut values = (-3i32..=200).collect::<std::collections::BTreeSet<_>>();
    values.extend([i32::MIN, i32::MAX, 0x40000000]);
    values.extend(
        data["formats"]
            .as_array()
            .unwrap()
            .iter()
            .map(|format| format["value"].as_i64().unwrap() as i32),
    );
    let cases = values.into_iter().map(|raw| {
        let format = vk::Format::from_raw(raw);
        let mut output = [0x5a;16];
        let info = observed::try_vk_format_info(format).ok().map(|info| {info.write_into(&mut output).unwrap();info.bytes.to_vec()});
        serde_json::json!({"format":raw,"aspects":observed::try_vk_format_valid_aspect_flags(format).ok().map(|flags|flags.as_raw()),"info":info,"output_bytes":output,"block_bytes":observed::try_vk_format_block_bytes(format).ok()})
    }).collect::<Vec<_>>();
    println!(
        "FORMAT_CANDIDATE_JSON={}",
        serde_json::json!({"cases":cases})
    );
}
