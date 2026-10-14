use ash::vk;
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
