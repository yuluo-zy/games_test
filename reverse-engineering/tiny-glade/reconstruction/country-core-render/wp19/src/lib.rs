//! WP19 — `country_core` render_loop misc + stylized domains reconstruction (B).
//!
//! Version B (patched `tiny-glade.exe` cf048e27…, PDB `tiny_glade.pdb` 2d9686b8…).
//! Coverage (11 skeleton units, WP19):
//!   - `systems::render_loop::{painterly, vermeer}` — stylized stroke draw chains
//!     (generate_strokes → raster_strokes → combine) used by reveal/session flow.
//!   - `systems::render_loop::{menu_background, session_selection, lut_gallery,
//!     indirect_params, resources::critters}` — boot/menu fullscreen UI passes,
//!     LUT gallery, indirect-args SSBO carriers, critter SSBO markers.
//!   - `resources::{gpu_lut, grading_lut, loading_screen, logo_animation}` — LUT
//!     texture resources + loading/logo boot resources.
//!
//! Business-logic contracts: `contracts/{painterly-vermeer,menu-session,lut,
//! indirect-params}.md`. Machine-code diff is only used to adjudicate key numeric
//! ambiguity (per project workflow); the Rust here is skeleton-aligned recovery.
//!
//! The sibling packages own the shared types (wp1 pass_builder, wp4 asset
//! libraries + frame constants, wp15 ui_state, wp13 session); this package
//! CONSUMES them and does NOT redefine them — see `consume`.

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod typed_gap;

/// Contract surface for the sibling-owned shared types this package consumes
/// (wp1/wp2/wp4/wp15/wp13). Local Gap stubs only; the integrated crate uses the
/// real types.
pub mod consume {
    use crate::typed_gap::Gap;

    /// wp1 `country_core::render::pass_builder` surface.
    pub mod pass_builder {
        use crate::typed_gap::Gap;

        /// 400-byte compute-pass builder (wp1).
        #[derive(Clone, Default)]
        pub struct ComputePass(pub Gap);
        /// 0xf8-byte raster draw builder (wp1).
        #[derive(Clone, Default)]
        pub struct DrawBuilder(pub Gap);
        /// Raster pass wrapper over ≤5 color attachments + optional depth (wp1).
        #[derive(Clone, Default)]
        pub struct RasterPass(pub Gap);
        /// Raster draw packet (default shader + maps + blend) (wp1).
        #[derive(Clone, Default)]
        pub struct DrawPacket(pub Gap);
        /// 48-byte ShaderUri (whole/name/entry) (wp1).
        #[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
        pub struct ShaderUri(pub Gap);
        /// Uniform writer over a bindings table (wp1).
        #[derive(Clone, Default)]
        pub struct UniformBinder(pub Gap);

        /// `AsComputeShaderSource` trait (wp1): `GenerateStrokes` etc. implement it.
        pub trait AsComputeShaderSource {
            fn source_hash(&self) -> u64;
            fn as_compute_pipeline(&self) -> ShaderUri;
        }

        impl ComputePass {
            pub fn new<A: AsComputeShaderSource>() -> Self { unimplemented!() }
            pub fn write_rg_storage_buffer<T>(self, _name: &str) -> Self { unimplemented!() }
            pub fn dispatch_2d(self, _x: u32, _y: u32) -> Self { unimplemented!() }
        }
        impl DrawBuilder {
            pub fn new(_world: &Gap, _shader: Gap, _variant: u8) -> Self { unimplemented!() }
            pub fn read_rg_storage_buffer<T>(self, _name: &str) -> Self { unimplemented!() }
            pub fn optional_uniform<T>(self, _name: &str, _value: &T) -> Self { unimplemented!() }
            pub fn draw_no_vao(self) -> Self { unimplemented!() }
        }
    }

    /// wp4 `country_core::render` / `asset_libraries` core.
    pub mod core {
        use crate::typed_gap::Gap;

        /// ECS resource holding the graphics device (wp4).
        pub struct GraphicsDeviceResource(pub Gap);
        pub struct Mesh(pub Gap);
        pub struct Texture(pub Gap);
        /// 3D LUT texture handle carrier (wp4 texture_library).
        pub struct NamedTexture(pub Gap);
        pub struct TextureFromDisk(pub Gap);

        pub struct AssetShaderLibrary(pub Gap);
        pub struct AssetMeshLibrary(pub Gap);
        pub struct AssetTextureLibrary(pub Gap);
        pub struct AssetSsboLibrary(pub Gap);
        pub struct MeshWatch(pub Gap);
        pub struct TextureWatch(pub Gap);

        /// wp15 `systems::ui::resources::ui_state::UiState`.
        pub struct UiState(pub Gap);

        /// wp13 `resources::session::SavedSession`.
        pub struct SavedSession(pub Gap);

        /// wp14 `resources::audio_resources::AudioCore`.
        pub struct AudioCore(pub Gap);

        /// utils::timer::FilteredWallTime.
        pub struct FilteredWallTime(pub Gap);

        /// WP19 `resources::logo_animation::LogoAnimation`.
        pub struct LogoAnimation(pub Gap);

        /// WP19 `resources::grading_lut::GradingLuts`.
        pub struct GradingLuts(pub Gap);

        /// WP19 `resources::grading_lut::LutGalleryState`.
        pub struct LutGalleryState(pub Gap);
    }

    /// wp4 `render::shaderwatch::inner::ShaderWatch`.
    pub struct ShaderWatch(pub crate::typed_gap::Gap);
}

pub mod systems;
pub mod resources;
