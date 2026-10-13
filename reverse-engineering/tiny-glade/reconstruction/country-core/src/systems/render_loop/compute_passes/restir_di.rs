//! `restir_di.rs` — W41 ReSTIR direct-lighting reservoir sampling (0x140a6fec0).
//!
//! Exact shader U (`_rdi_temporal` candidate). `AssetSsboLibrary::get` +
//! `InstanceBuffer::instance_count` + read_texture + write_texture + dispatch_2d
//! + `RenderGraphRecorder::create` (O). Reservoir texture `rdi_reservoir_tex`.

pub fn restir_sample_direct_lights(world: &mut crate::typed_gap::Gap, _light_count: u32) {
    let _ = world;
    unimplemented!()
}
