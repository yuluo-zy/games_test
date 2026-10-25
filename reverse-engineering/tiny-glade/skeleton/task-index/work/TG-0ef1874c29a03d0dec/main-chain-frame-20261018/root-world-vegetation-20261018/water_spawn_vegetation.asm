140a13970: push rbp
140a13971: push r14
140a13973: push rsi
140a13974: push rdi
140a13975: push rbx
140a13976: sub rsp, 0x500
140a1397d: lea rbp, [rsp + 0x80]
140a13985: mov qword ptr [rbp + 0x478], 0xfffffffffffffffe
140a13990: cmp byte ptr [rip + 0x2c213f9], 0
140a13997: jne 0x140a1422b
140a1399d: mov rsi, rdx
140a139a0: mov rdi, rcx
140a139a3: mov qword ptr [rsp + 0x30], rcx
140a139a8: mov qword ptr [rsp + 0x28], rdx
140a139ad: mov qword ptr [rsp + 0x20], 0x1d
140a139b6: lea rdx, [rip + 0x20b4341]
140a139bd: lea r9, [rip + 0x20b4348]
140a139c4: lea rbx, [rbp + 0x2e0]
140a139cb: mov r8d, 0xe
140a139d1: mov rcx, rbx
140a139d4: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a139d9: lea r14, [rbp + 0x150]
140a139e0: movabs r8, 0x1de1282bcdffb385
140a139ea: mov rcx, r14
140a139ed: mov rdx, rbx
140a139f0: call 0x141606560 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadSsbo>
140a139f5: lea rcx, [rbp + 0x2e0]
140a139fc: movabs r8, 0xd0bb0d2aa1cf854b
140a13a06: mov rdx, r14
140a13a09: call 0x141605ff0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>
140a13a0e: lea rcx, [rsi + 0x38]
140a13a12: mov byte ptr [rbp + 0x476], 1
140a13a19: movabs rdx, 0x74430dfcc6d8719b
140a13a23: movabs r8, 0xca5591f7e464509c
140a13a2d: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
140a13a32: test rax, rax
140a13a35: je 0x140a1423a
140a13a3b: cmp qword ptr [rsi + 0x218], rdx
140a13a42: jbe 0x140a1423a
140a13a48: mov rax, qword ptr [rsi + 0x210]
140a13a4f: mov rcx, qword ptr [rax + rdx*8]
140a13a53: test rcx, rcx
140a13a56: je 0x140a1423a
140a13a5c: mov rax, qword ptr [rsi + 0x1e0]
140a13a63: not rcx
140a13a66: lea rcx, [rcx + rcx*2]
140a13a6a: shl rcx, 5
140a13a6e: cmp qword ptr [rax + rcx + 0x20], 0
140a13a74: je 0x140a1423a
140a13a7a: cmp byte ptr [rbp + 0x416], 8
140a13a81: je 0x140a13ab4
140a13a83: add rax, rcx
140a13a86: mov rax, qword ptr [rax + 0x10]
140a13a8a: cmp byte ptr [rax + 0x58], 3
140a13a8e: sete r9b
140a13a92: lea rcx, [rbp + 0x3f0]
140a13a99: mov edx, 0xe8
140a13a9e: add rdx, qword ptr [rbp + 0x418]
140a13aa5: movabs r8, 0xeaa3e0c70b923afc
140a13aaf: call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
140a13ab4: lea rcx, [rbp + 0x150]
140a13abb: mov r8d, 0x190
140a13ac1: mov rdx, rbx
140a13ac4: call 0x142923c00
140a13ac9: cmp byte ptr [rbp + 0x286], 8
140a13ad0: je 0x140a13afe
140a13ad2: lea rcx, [rbp + 0x260]
140a13ad9: mov edx, 0xe8
140a13ade: add rdx, qword ptr [rbp + 0x288]
140a13ae5: movabs r8, 0xfb539f7243dbb832
140a13aef: movabs r9, 0xe1000000e1
140a13af9: call 0x14096b760 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<array$<u32,2> >,array$<u32,2> >
140a13afe: lea r14, [rbp - 0x40]
140a13b02: lea rdx, [rbp + 0x150]
140a13b09: mov r8d, 0x190
140a13b0f: mov rcx, r14
140a13b12: call 0x142923c00
140a13b17: mov byte ptr [rbp + 0x150], 9
140a13b1e: mov dword ptr [rbp + 0x2e0], 2
140a13b28: mov qword ptr [rsp + 0x20], rbx
140a13b2d: lea rbx, [rbp + 0x150]
140a13b34: movabs r8, 0x61b08b5e0470c5f2
140a13b3e: mov rcx, rbx
140a13b41: mov rdx, r14
140a13b44: mov r9b, 0x19
140a13b47: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a13b4c: lea r14, [rbp + 0x2e0]
140a13b53: lea rdx, [rbp - 0x40]
140a13b57: mov r8d, 0x190
140a13b5d: mov rcx, r14
140a13b60: call 0x142923c00
140a13b65: mov byte ptr [rbp + 0x477], 0xa
140a13b6c: mov dword ptr [rbp + 0x150], 2
140a13b76: mov qword ptr [rsp + 0x20], rbx
140a13b7b: lea rcx, [rbp + 0x477]
140a13b82: movabs r8, 0x297d1e4cdae8f657
140a13b8c: mov rdx, r14
140a13b8f: mov r9b, 0x19
140a13b92: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a13b97: mov r8d, 0x190
140a13b9d: mov rcx, rbx
140a13ba0: mov rdx, r14
140a13ba3: call 0x142923c00
140a13ba8: mov byte ptr [rbp - 0x40], 0x13
140a13bac: mov dword ptr [rbp + 0x2e0], 2
140a13bb6: mov qword ptr [rsp + 0x20], r14
140a13bbb: lea rcx, [rbp - 0x40]
140a13bbf: movabs r8, 0x6ad63ccd18c8f041
140a13bc9: mov rdx, rbx
140a13bcc: mov r9b, 0x19
140a13bcf: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a13bd4: lea rbx, [rbp + 0x2e0]
140a13bdb: lea r14, [rbp + 0x150]
140a13be2: mov r8d, 0x190
140a13be8: mov rcx, rbx
140a13beb: mov rdx, r14
140a13bee: call 0x142923c00
140a13bf3: mov byte ptr [rbp - 0x40], 5
140a13bf7: mov dword ptr [rbp + 0x150], 2
140a13c01: mov qword ptr [rsp + 0x20], r14
140a13c06: lea rcx, [rbp - 0x40]
140a13c0a: movabs r8, 0xed502fff6dc4021f
140a13c14: mov rdx, rbx
140a13c17: mov r9b, 0x19
140a13c1a: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a13c1f: lea rbx, [rbp + 0x150]
140a13c26: lea r14, [rbp + 0x2e0]
140a13c2d: mov r8d, 0x190
140a13c33: mov rcx, rbx
140a13c36: mov rdx, r14
140a13c39: call 0x142923c00
140a13c3e: mov byte ptr [rbp - 0x40], 0x14
140a13c42: mov dword ptr [rbp + 0x2e0], 2
140a13c4c: mov qword ptr [rsp + 0x20], r14
140a13c51: lea rcx, [rbp - 0x40]
140a13c55: movabs r8, 0x49f43a487883738d
140a13c5f: mov rdx, rbx
140a13c62: mov r9b, 0x19
140a13c65: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a13c6a: lea rbx, [rbp + 0x2e0]
140a13c71: lea r14, [rbp + 0x150]
140a13c78: mov r8d, 0x190
140a13c7e: mov rcx, rbx
140a13c81: mov rdx, r14
140a13c84: call 0x142923c00
140a13c89: mov byte ptr [rbp - 0x40], 0x17
140a13c8d: mov dword ptr [rbp + 0x150], 2
140a13c97: mov qword ptr [rsp + 0x20], r14
140a13c9c: lea rcx, [rbp - 0x40]
140a13ca0: movabs r8, 0x9140004c9a5b7b9d
140a13caa: mov rdx, rbx
140a13cad: mov r9b, 0x32
140a13cb0: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a13cb5: lea rbx, [rbp + 0x150]
140a13cbc: lea rdx, [rbp + 0x2e0]
140a13cc3: mov r8d, 0x190
140a13cc9: mov rcx, rbx
140a13ccc: call 0x142923c00
140a13cd1: lea rcx, [rbp + 0x2e0]
140a13cd8: movabs r8, 0xf1c07a2b8012765a
140a13ce2: mov rdx, rbx
140a13ce5: mov r9d, 1
140a13ceb: call 0x141608c10 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>
140a13cf0: cmp byte ptr [rbp + 0x416], 8
140a13cf7: je 0x140a13d21
140a13cf9: lea rcx, [rbp + 0x3f0]
140a13d00: mov edx, 0xe8
140a13d05: add rdx, qword ptr [rbp + 0x418]
140a13d0c: movabs r8, 0xe9511a616b75789
140a13d16: mov r9d, 8
140a13d1c: call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a13d21: lea rcx, [rbp - 0x40]
140a13d25: lea rdx, [rbp + 0x2e0]
140a13d2c: mov r8d, 0x190
140a13d32: call 0x142923c00
140a13d37: movups xmm0, xmmword ptr [rbp + 0x60]
140a13d3b: movaps xmmword ptr [rbp + 0x190], xmm0
140a13d42: movups xmm0, xmmword ptr [rbp + 0x50]
140a13d46: movaps xmmword ptr [rbp + 0x180], xmm0
140a13d4d: movups xmm0, xmmword ptr [rbp + 0x40]
140a13d51: movaps xmmword ptr [rbp + 0x170], xmm0
140a13d58: movups xmm0, xmmword ptr [rbp + 0x30]
140a13d5c: movaps xmmword ptr [rbp + 0x160], xmm0
140a13d63: movups xmm0, xmmword ptr [rbp + 0x20]
140a13d67: movaps xmmword ptr [rbp + 0x150], xmm0
140a13d6e: mov rax, qword ptr [rbp + 0xf8]
140a13d75: mov rcx, qword ptr [rbp + 0x140]
140a13d7c: mov rdx, qword ptr [rbp - 0x30]
140a13d80: mov qword ptr [rbp + 0x2f0], rdx
140a13d87: movups xmm0, xmmword ptr [rbp - 0x40]
140a13d8b: movaps xmmword ptr [rbp + 0x2e0], xmm0
140a13d92: movups xmm0, xmmword ptr [rbp - 0x28]
140a13d96: movaps xmmword ptr [rbp + 0x310], xmm0
140a13d9d: mov rdx, qword ptr [rbp - 0x18]
140a13da1: mov qword ptr [rbp + 0x320], rdx
140a13da8: movups xmm0, xmmword ptr [rbp - 0x10]
140a13dac: movups xmmword ptr [rbp + 0x2f8], xmm0
140a13db3: mov rdx, qword ptr [rbp]
140a13db7: mov qword ptr [rbp + 0x308], rdx
140a13dbe: movups xmm0, xmmword ptr [rbp + 0x70]
140a13dc2: movups xmm1, xmmword ptr [rbp + 0x80]
140a13dc9: movups xmm2, xmmword ptr [rbp + 0x90]
140a13dd0: movups xmm3, xmmword ptr [rbp + 0xa0]
140a13dd7: movups xmmword ptr [rbp + 0x358], xmm0
140a13dde: movups xmmword ptr [rbp + 0x368], xmm1
140a13de5: movups xmmword ptr [rbp + 0x378], xmm2
140a13dec: movups xmmword ptr [rbp + 0x388], xmm3
140a13df3: movups xmm0, xmmword ptr [rbp + 0xb0]
140a13dfa: movups xmmword ptr [rbp + 0x398], xmm0
140a13e01: movups xmm0, xmmword ptr [rbp + 0xc0]
140a13e08: movups xmmword ptr [rbp + 0x3a8], xmm0
140a13e0f: mov rdx, qword ptr [rbp + 0x18]
140a13e13: mov qword ptr [rbp + 0x338], rdx
140a13e1a: movups xmm0, xmmword ptr [rbp + 8]
140a13e1e: movups xmmword ptr [rbp + 0x328], xmm0
140a13e25: mov qword ptr [rbp + 0x3c0], rcx
140a13e2c: mov qword ptr [rbp + 0x3b8], rax
140a13e33: movaps xmm0, xmmword ptr [rip + 0x20b3266]
140a13e3a: movaps xmmword ptr [rbp + 0x340], xmm0
140a13e41: lea rcx, [rbp + 0x150]
140a13e48: lea rdx, [rbp + 0x2e0]
140a13e4f: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a13e54: cmp byte ptr [rbp + 0xf6], 8
140a13e5b: je 0x140a13e69
140a13e5d: lea rcx, [rbp + 0xe0]
140a13e64: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a13e69: mov qword ptr [rsp + 0x30], rdi
140a13e6e: mov qword ptr [rsp + 0x28], rsi
140a13e73: mov qword ptr [rsp + 0x20], 0x1a
140a13e7c: lea rdx, [rip + 0x20b3ec5]
140a13e83: lea r9, [rip + 0x20b3ec9]
140a13e8a: lea rsi, [rbp + 0x2e0]
140a13e91: mov r8d, 0xb
140a13e97: mov rcx, rsi
140a13e9a: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a13e9f: lea rcx, [rbp + 0x150]
140a13ea6: movabs r8, 0x6d9256d4ac7887ad
140a13eb0: mov rdx, rsi
140a13eb3: call 0x141606ad0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::compute_water::ReedSsbo>
140a13eb8: cmp byte ptr [rbp + 0x286], 8
140a13ebf: je 0x140a13eed
140a13ec1: lea rcx, [rbp + 0x260]
140a13ec8: mov edx, 0xe8
140a13ecd: add rdx, qword ptr [rbp + 0x288]
140a13ed4: movabs r8, 0xfb539f7243dbb832
140a13ede: movabs r9, 0x9600000096
140a13ee8: call 0x14096b760 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<array$<u32,2> >,array$<u32,2> >
140a13eed: lea rdi, [rbp - 0x40]
140a13ef1: lea rdx, [rbp + 0x150]
140a13ef8: mov r8d, 0x190
140a13efe: mov rcx, rdi
140a13f01: call 0x142923c00
140a13f06: mov byte ptr [rbp + 0x150], 9
140a13f0d: mov dword ptr [rbp + 0x2e0], 2
140a13f17: mov qword ptr [rsp + 0x20], rsi
140a13f1c: lea rsi, [rbp + 0x150]
140a13f23: movabs r8, 0x61b08b5e0470c5f2
140a13f2d: mov rcx, rsi
140a13f30: mov rdx, rdi
140a13f33: mov r9b, 0x19
140a13f36: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a13f3b: lea rdi, [rbp + 0x2e0]
140a13f42: lea rdx, [rbp - 0x40]
140a13f46: mov r8d, 0x190
140a13f4c: mov rcx, rdi
140a13f4f: call 0x142923c00
140a13f54: mov byte ptr [rbp + 0x477], 0xa
140a13f5b: mov dword ptr [rbp + 0x150], 2
140a13f65: mov qword ptr [rsp + 0x20], rsi
140a13f6a: lea rcx, [rbp + 0x477]
140a13f71: movabs r8, 0x297d1e4cdae8f657
140a13f7b: mov rdx, rdi
140a13f7e: mov r9b, 0x19
140a13f81: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a13f86: mov r8d, 0x190
140a13f8c: mov rcx, rsi
140a13f8f: mov rdx, rdi
140a13f92: call 0x142923c00
140a13f97: mov byte ptr [rbp - 0x40], 0x13
140a13f9b: mov dword ptr [rbp + 0x2e0], 2
140a13fa5: mov qword ptr [rsp + 0x20], rdi
140a13faa: lea rcx, [rbp - 0x40]
140a13fae: movabs r8, 0x6ad63ccd18c8f041
140a13fb8: mov rdx, rsi
140a13fbb: mov r9b, 0x19
140a13fbe: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a13fc3: lea rsi, [rbp + 0x2e0]
140a13fca: lea rdi, [rbp + 0x150]
140a13fd1: mov r8d, 0x190
140a13fd7: mov rcx, rsi
140a13fda: mov rdx, rdi
140a13fdd: call 0x142923c00
140a13fe2: mov byte ptr [rbp - 0x40], 5
140a13fe6: mov dword ptr [rbp + 0x150], 2
140a13ff0: mov qword ptr [rsp + 0x20], rdi
140a13ff5: lea rcx, [rbp - 0x40]
140a13ff9: movabs r8, 0xed502fff6dc4021f
140a14003: mov rdx, rsi
140a14006: mov r9b, 0x19
140a14009: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a1400e: lea rsi, [rbp + 0x150]
140a14015: lea rdi, [rbp + 0x2e0]
140a1401c: mov r8d, 0x190
140a14022: mov rcx, rsi
140a14025: mov rdx, rdi
140a14028: call 0x142923c00
140a1402d: mov byte ptr [rbp + 0x477], 0x16
140a14034: mov dword ptr [rbp + 0x2e0], 2
140a1403e: mov qword ptr [rsp + 0x20], rdi
140a14043: lea rcx, [rbp + 0x477]
140a1404a: movabs r8, 0x6570181649ac6420
140a14054: mov rdx, rsi
140a14057: mov r9b, 0x19
140a1405a: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a1405f: lea rsi, [rbp - 0x40]
140a14063: lea rdx, [rbp + 0x150]
140a1406a: mov r8d, 0x190
140a14070: mov rcx, rsi
140a14073: call 0x142923c00
140a14078: lea rcx, [rbp + 0x2e0]
140a1407f: movabs r8, 0xf1c07a2b8012765a
140a14089: mov rdx, rsi
140a1408c: mov r9d, 1
140a14092: call 0x141608c10 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>
140a14097: cmp byte ptr [rbp + 0x416], 8
140a1409e: je 0x140a140c8
140a140a0: lea rcx, [rbp + 0x3f0]
140a140a7: mov edx, 0xe8
140a140ac: add rdx, qword ptr [rbp + 0x418]
140a140b3: movabs r8, 0xe9511a616b75789
140a140bd: mov r9d, 9
140a140c3: call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a140c8: lea rcx, [rbp + 0x150]
140a140cf: lea rdx, [rbp + 0x2e0]
140a140d6: mov r8d, 0x190
140a140dc: call 0x142923c00
140a140e1: movups xmm0, xmmword ptr [rbp + 0x1f0]
140a140e8: movaps xmmword ptr [rbp], xmm0
140a140ec: movups xmm0, xmmword ptr [rbp + 0x1e0]
140a140f3: movaps xmmword ptr [rbp - 0x10], xmm0
140a140f7: movups xmm0, xmmword ptr [rbp + 0x1d0]
140a140fe: movaps xmmword ptr [rbp - 0x20], xmm0
140a14102: movups xmm0, xmmword ptr [rbp + 0x1c0]
140a14109: movaps xmmword ptr [rbp - 0x30], xmm0
140a1410d: movups xmm0, xmmword ptr [rbp + 0x1b0]
140a14114: movaps xmmword ptr [rbp - 0x40], xmm0
140a14118: mov rax, qword ptr [rbp + 0x288]
140a1411f: mov rcx, qword ptr [rbp + 0x2d0]
140a14126: mov rdx, qword ptr [rbp + 0x160]
140a1412d: mov qword ptr [rbp + 0x2f0], rdx
140a14134: movups xmm0, xmmword ptr [rbp + 0x150]
140a1413b: movaps xmmword ptr [rbp + 0x2e0], xmm0
140a14142: movups xmm0, xmmword ptr [rbp + 0x168]
140a14149: movaps xmmword ptr [rbp + 0x310], xmm0
140a14150: mov rdx, qword ptr [rbp + 0x178]
140a14157: mov qword ptr [rbp + 0x320], rdx
140a1415e: movups xmm0, xmmword ptr [rbp + 0x180]
140a14165: movups xmmword ptr [rbp + 0x2f8], xmm0
140a1416c: mov rdx, qword ptr [rbp + 0x190]
140a14173: mov qword ptr [rbp + 0x308], rdx
140a1417a: movups xmm0, xmmword ptr [rbp + 0x200]
140a14181: movups xmm1, xmmword ptr [rbp + 0x210]
140a14188: movups xmm2, xmmword ptr [rbp + 0x220]
140a1418f: movups xmm3, xmmword ptr [rbp + 0x230]
140a14196: movups xmmword ptr [rbp + 0x358], xmm0
140a1419d: movups xmmword ptr [rbp + 0x368], xmm1
140a141a4: movups xmmword ptr [rbp + 0x378], xmm2
140a141ab: movups xmmword ptr [rbp + 0x388], xmm3
140a141b2: movups xmm0, xmmword ptr [rbp + 0x240]
140a141b9: movups xmmword ptr [rbp + 0x398], xmm0
140a141c0: movups xmm0, xmmword ptr [rbp + 0x250]
140a141c7: movups xmmword ptr [rbp + 0x3a8], xmm0
140a141ce: mov rdx, qword ptr [rbp + 0x1a8]
140a141d5: mov qword ptr [rbp + 0x338], rdx
140a141dc: movups xmm0, xmmword ptr [rbp + 0x198]
140a141e3: movups xmmword ptr [rbp + 0x328], xmm0
140a141ea: mov qword ptr [rbp + 0x3c0], rcx
140a141f1: mov qword ptr [rbp + 0x3b8], rax
140a141f8: movaps xmm0, xmmword ptr [rip + 0x2090f31]
140a141ff: movaps xmmword ptr [rbp + 0x340], xmm0
140a14206: lea rcx, [rbp - 0x40]
140a1420a: lea rdx, [rbp + 0x2e0]
140a14211: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a14216: cmp byte ptr [rbp + 0x286], 8
140a1421d: je 0x140a1422b
140a1421f: lea rcx, [rbp + 0x270]
140a14226: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a1422b: nop 
140a1422c: add rsp, 0x500
140a14233: pop rbx
140a14234: pop rdi
140a14235: pop rsi
140a14236: pop r14
140a14238: pop rbp
140a14239: ret 
140a1423a: mov byte ptr [rbp + 0x476], 1
140a14241: lea rcx, [rip + 0x20b3ae8]
140a14248: call 0x1428d9310 ; core::option::unwrap_failed
140a1424d: ud2 
140a1424f: nop 
140a14250: mov qword ptr [rsp + 0x10], rdx
140a14255: push rbp
140a14256: push r14
140a14258: push rsi
140a14259: push rdi
140a1425a: push rbx
140a1425b: sub rsp, 0x40
140a1425f: lea rbp, [rdx + 0x80]
140a14266: mov byte ptr [rbp + 0x476], 0
140a1426d: lea rcx, [rbp + 0x2e0]
140a14274: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a14279: mov byte ptr [rbp + 0x476], 0
140a14280: add rsp, 0x40
140a14284: pop rbx
140a14285: pop rdi
140a14286: pop rsi
140a14287: pop r14
140a14289: pop rbp
140a1428a: ret 
140a1428b: nop dword ptr [rax + rax]
140a14290: mov qword ptr [rsp + 0x10], rdx
140a14295: push rbp
140a14296: push r14
140a14298: push rsi
140a14299: push rdi
140a1429a: push rbx
140a1429b: sub rsp, 0x40
140a1429f: lea rbp, [rdx + 0x80]
140a142a6: cmp byte ptr [rbp + 0xf6], 8
140a142ad: je 0x140a142bb
140a142af: lea rcx, [rbp + 0xe0]
140a142b6: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a142bb: nop 
140a142bc: add rsp, 0x40
140a142c0: pop rbx
140a142c1: pop rdi
140a142c2: pop rsi
140a142c3: pop r14
140a142c5: pop rbp
140a142c6: ret 
140a142c7: nop word ptr [rax + rax]
140a142d0: mov qword ptr [rsp + 0x10], rdx
140a142d5: push rbp
140a142d6: push r14
140a142d8: push rsi
140a142d9: push rdi
140a142da: push rbx
140a142db: sub rsp, 0x40
140a142df: lea rbp, [rdx + 0x80]
140a142e6: lea rcx, [rbp + 0x150]
140a142ed: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a142f2: nop 
140a142f3: add rsp, 0x40
140a142f7: pop rbx
140a142f8: pop rdi
140a142f9: pop rsi
140a142fa: pop r14
140a142fc: pop rbp
140a142fd: ret 
140a142fe: nop 
140a14300: mov qword ptr [rsp + 0x10], rdx
140a14305: push rbp
140a14306: push r14
140a14308: push rsi
140a14309: push rdi
140a1430a: push rbx
140a1430b: sub rsp, 0x40
140a1430f: lea rbp, [rdx + 0x80]
140a14316: lea rcx, [rbp - 0x40]
140a1431a: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a1431f: nop 
140a14320: add rsp, 0x40
140a14324: pop rbx
140a14325: pop rdi
140a14326: pop rsi
140a14327: pop r14
140a14329: pop rbp
140a1432a: ret 
140a1432b: nop dword ptr [rax + rax]
140a14330: mov qword ptr [rsp + 0x10], rdx
140a14335: push rbp
140a14336: push r14
140a14338: push rsi
140a14339: push rdi
140a1433a: push rbx
140a1433b: sub rsp, 0x40
140a1433f: lea rbp, [rdx + 0x80]
140a14346: lea rcx, [rbp + 0x2e0]
140a1434d: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a14352: nop 
140a14353: add rsp, 0x40
140a14357: pop rbx
140a14358: pop rdi
140a14359: pop rsi
140a1435a: pop r14
140a1435c: pop rbp
140a1435d: ret 
140a1435e: nop 
140a14360: mov qword ptr [rsp + 0x10], rdx
140a14365: push rbp
140a14366: push r14
140a14368: push rsi
140a14369: push rdi
140a1436a: push rbx
140a1436b: sub rsp, 0x40
140a1436f: lea rbp, [rdx + 0x80]
140a14376: lea rcx, [rbp + 0x150]
140a1437d: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a14382: nop 
140a14383: add rsp, 0x40
140a14387: pop rbx
140a14388: pop rdi
140a14389: pop rsi
140a1438a: pop r14
140a1438c: pop rbp
140a1438d: ret 
140a1438e: nop 
140a14390: mov qword ptr [rsp + 0x10], rdx
140a14395: push rbp
140a14396: push r14
140a14398: push rsi
140a14399: push rdi
140a1439a: push rbx
140a1439b: sub rsp, 0x40
140a1439f: lea rbp, [rdx + 0x80]
140a143a6: lea rcx, [rbp + 0x2e0]
140a143ad: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a143b2: nop 
140a143b3: add rsp, 0x40
140a143b7: pop rbx
140a143b8: pop rdi
140a143b9: pop rsi
140a143ba: pop r14
140a143bc: pop rbp
140a143bd: ret 
140a143be: nop 
140a143c0: mov qword ptr [rsp + 0x10], rdx
140a143c5: push rbp
140a143c6: push r14
140a143c8: push rsi
140a143c9: push rdi
140a143ca: push rbx
140a143cb: sub rsp, 0x40
140a143cf: lea rbp, [rdx + 0x80]
140a143d6: lea rcx, [rbp + 0x150]
140a143dd: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a143e2: nop 
140a143e3: add rsp, 0x40
140a143e7: pop rbx
140a143e8: pop rdi
140a143e9: pop rsi
140a143ea: pop r14
140a143ec: pop rbp
140a143ed: ret 
140a143ee: nop 
140a143f0: mov qword ptr [rsp + 0x10], rdx
140a143f5: push rbp
140a143f6: push r14
140a143f8: push rsi
140a143f9: push rdi
140a143fa: push rbx
140a143fb: sub rsp, 0x40
140a143ff: lea rbp, [rdx + 0x80]
140a14406: lea rcx, [rbp + 0x2e0]
140a1440d: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a14412: nop 
140a14413: add rsp, 0x40
140a14417: pop rbx
140a14418: pop rdi
140a14419: pop rsi
140a1441a: pop r14
140a1441c: pop rbp
140a1441d: ret 
140a1441e: nop 
140a14420: mov qword ptr [rsp + 0x10], rdx
140a14425: push rbp
140a14426: push r14
140a14428: push rsi
140a14429: push rdi
140a1442a: push rbx
140a1442b: sub rsp, 0x40
140a1442f: lea rbp, [rdx + 0x80]
140a14436: cmp byte ptr [rbp + 0x286], 8
140a1443d: je 0x140a1444b
140a1443f: lea rcx, [rbp + 0x270]
140a14446: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a1444b: nop 
140a1444c: add rsp, 0x40
140a14450: pop rbx
140a14451: pop rdi
140a14452: pop rsi
140a14453: pop r14
140a14455: pop rbp
140a14456: ret 
140a14457: nop word ptr [rax + rax]
140a14460: mov qword ptr [rsp + 0x10], rdx
140a14465: push rbp
140a14466: push r14
140a14468: push rsi
140a14469: push rdi
140a1446a: push rbx
140a1446b: sub rsp, 0x40
140a1446f: lea rbp, [rdx + 0x80]
140a14476: cmp byte ptr [rbp + 0x476], 0
140a1447d: jne 0x140a1448a
140a1447f: add rsp, 0x40
140a14483: pop rbx
140a14484: pop rdi
140a14485: pop rsi
140a14486: pop r14
140a14488: pop rbp
140a14489: ret 
140a1448a: lea rcx, [rbp + 0x2e0]
140a14491: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a14496: jmp 0x140a1447f
140a14498: nop dword ptr [rax + rax]
140a144a0: mov qword ptr [rsp + 0x10], rdx
140a144a5: push rbp
140a144a6: push r14
140a144a8: push rsi
140a144a9: push rdi
140a144aa: push rbx
140a144ab: sub rsp, 0x40
140a144af: lea rbp, [rdx + 0x80]
140a144b6: lea rcx, [rbp + 0x150]
140a144bd: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a144c2: nop 
140a144c3: add rsp, 0x40
140a144c7: pop rbx
140a144c8: pop rdi
140a144c9: pop rsi
140a144ca: pop r14
140a144cc: pop rbp
140a144cd: ret 
140a144ce: nop 
140a144d0: mov qword ptr [rsp + 0x10], rdx
140a144d5: push rbp
140a144d6: push r14
140a144d8: push rsi
140a144d9: push rdi
140a144da: push rbx
140a144db: sub rsp, 0x40
140a144df: lea rbp, [rdx + 0x80]
140a144e6: lea rcx, [rbp - 0x40]
140a144ea: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a144ef: nop 
140a144f0: add rsp, 0x40
140a144f4: pop rbx
140a144f5: pop rdi
140a144f6: pop rsi
140a144f7: pop r14
140a144f9: pop rbp
140a144fa: ret 
140a144fb: nop dword ptr [rax + rax]
140a14500: mov qword ptr [rsp + 0x10], rdx
140a14505: push rbp
140a14506: push r14
140a14508: push rsi
140a14509: push rdi
140a1450a: push rbx
140a1450b: sub rsp, 0x40
140a1450f: lea rbp, [rdx + 0x80]
140a14516: lea rcx, [rbp + 0x2e0]
140a1451d: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a14522: nop 
140a14523: add rsp, 0x40
140a14527: pop rbx
140a14528: pop rdi
140a14529: pop rsi
140a1452a: pop r14
140a1452c: pop rbp
140a1452d: ret 
140a1452e: nop 
140a14530: mov qword ptr [rsp + 0x10], rdx
140a14535: push rbp
140a14536: push r14
140a14538: push rsi
140a14539: push rdi
140a1453a: push rbx
140a1453b: sub rsp, 0x40
140a1453f: lea rbp, [rdx + 0x80]
140a14546: lea rcx, [rbp + 0x150]
140a1454d: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a14552: nop 
140a14553: add rsp, 0x40
140a14557: pop rbx
140a14558: pop rdi
140a14559: pop rsi
140a1455a: pop r14
140a1455c: pop rbp
140a1455d: ret 
140a1455e: nop 
140a14560: mov qword ptr [rsp + 0x10], rdx
140a14565: push rbp
140a14566: push r14
140a14568: push rsi
140a14569: push rdi
140a1456a: push rbx
140a1456b: sub rsp, 0x40
140a1456f: lea rbp, [rdx + 0x80]
140a14576: lea rcx, [rbp + 0x2e0]
140a1457d: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a14582: nop 
140a14583: add rsp, 0x40
140a14587: pop rbx
140a14588: pop rdi
140a14589: pop rsi
140a1458a: pop r14
140a1458c: pop rbp
140a1458d: ret 
140a1458e: nop 
140a14590: mov qword ptr [rsp + 0x10], rdx
140a14595: push rbp
140a14596: push r14
140a14598: push rsi
140a14599: push rdi
140a1459a: push rbx
140a1459b: sub rsp, 0x40
140a1459f: lea rbp, [rdx + 0x80]
140a145a6: lea rcx, [rbp + 0x150]
140a145ad: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a145b2: nop 
140a145b3: add rsp, 0x40
140a145b7: pop rbx
140a145b8: pop rdi
140a145b9: pop rsi
140a145ba: pop r14
140a145bc: pop rbp
140a145bd: ret 
140a145be: nop 
140a145c0: mov qword ptr [rsp + 0x10], rdx
140a145c5: push rbp
140a145c6: push r14
140a145c8: push rsi
140a145c9: push rdi
140a145ca: push rbx
140a145cb: sub rsp, 0x40
140a145cf: lea rbp, [rdx + 0x80]
140a145d6: lea rcx, [rbp + 0x2e0]
140a145dd: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a145e2: nop 
140a145e3: add rsp, 0x40
140a145e7: pop rbx
140a145e8: pop rdi
140a145e9: pop rsi
140a145ea: pop r14
140a145ec: pop rbp
140a145ed: ret 
140a145ee: nop 
140a145f0: mov qword ptr [rsp + 0x10], rdx
140a145f5: push rbp
140a145f6: push r14
140a145f8: push rsi
140a145f9: push rdi
140a145fa: push rbx
140a145fb: sub rsp, 0x40
140a145ff: lea rbp, [rdx + 0x80]
140a14606: lea rcx, [rbp + 0x2e0]
140a1460d: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a14612: nop 
140a14613: add rsp, 0x40
140a14617: pop rbx
140a14618: pop rdi
140a14619: pop rsi
140a1461a: pop r14
140a1461c: pop rbp
140a1461d: ret 
