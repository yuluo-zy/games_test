use country_core::asset_libraries::texture_library::{
    AssetTextureLibrary, DescriptorImageUpdate, ImageView, TextureLibraryDevice,
    TextureUpdate, format_bytes_per_texel,
};
use country_core::render::load_texture::boundary::TextureDevice;
use country_core::resources::texture_presets::TexturePreset;
use country_core::rhapsody_contract::DescriptorSet;
use country_core::rhapsody_types::{BackendError, BufferGraphBackend, ImportedBuffer, RenderGraph, Texture, TextureDesc};

struct Device;
impl TextureDevice for Device {
    fn create_texture(&self, _: &str, d: &TextureDesc, _: &[u8]) -> Result<Texture, BackendError> {
        Ok(Texture { id: 1, width: d.extent[0], height: d.extent[1], format: d.format })
    }
}
impl TextureLibraryDevice for Device {
    fn create_persistent_descriptor_set(&self, _: &str) -> DescriptorSet { DescriptorSet }
    fn update_persistent_descriptor_set(&self, _: &DescriptorSet, _: &[DescriptorImageUpdate]) {}
    fn get_vk_image_view(&self, _: &Texture, _: u32) -> Result<ImageView, BackendError> { Ok(ImageView(1)) }
    fn update_texture(&self, _: &Texture, _: &TextureUpdate) -> Result<(), BackendError> { Ok(()) }
}
#[test]
fn all_preset_format_sizes_match_recovered_format_table() {
    let differences: Vec<_> = (0..TexturePreset::COUNT).filter_map(|p| {
        let f = TexturePreset(p).texture().desc.format;
        let expected = rhapsody::raw::format::observed::try_vk_format_info(
            ash::vk::Format::from_raw(f as i32)).unwrap().word_at_04();
        let actual = format_bytes_per_texel(f);
        (actual != expected).then_some((p, f, actual, expected))
    }).collect();
    assert!(differences.is_empty(), "(preset, format, actual, expected): {differences:?}");
}
struct Graph;
impl BufferGraphBackend for Graph {
    fn import_unowned(&mut self, buffer: u64, access: u8) -> ImportedBuffer {
        ImportedBuffer { resource: buffer, generation: 1, access: access as u32 }
    }
    fn export(&mut self, _: ImportedBuffer, _: u8) {}
}
fn imported(p: u8) -> TextureDesc {
    let mut library = AssetTextureLibrary::new(&Device);
    library.create_preset_textures(&Device);
    library.import_texture_presets_to_rg(&mut RenderGraph::with_backend(Graph));
    library.get_rg_texture(TexturePreset(p)).desc
}
#[test]
fn import_preserves_mip_count() {
    assert_eq!(imported(8).mip_levels, TexturePreset(8).texture().desc.mip_levels);
}
#[test]
fn import_preserves_array_layers() {
    assert_eq!(imported(0x11).array_elements, TexturePreset(0x11).texture().desc.array_elements);
}
