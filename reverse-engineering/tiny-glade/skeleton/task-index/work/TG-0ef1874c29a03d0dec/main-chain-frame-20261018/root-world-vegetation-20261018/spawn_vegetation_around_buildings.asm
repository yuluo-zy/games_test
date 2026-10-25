1408e4cc0: push rbp
1408e4cc1: push r15
1408e4cc3: push r14
1408e4cc5: push rsi
1408e4cc6: push rdi
1408e4cc7: push rbx
1408e4cc8: sub rsp, 0x508
1408e4ccf: lea rbp, [rsp + 0x80]
1408e4cd7: mov qword ptr [rbp + 0x480], 0xfffffffffffffffe
1408e4ce2: mov rsi, rdx
1408e4ce5: mov rdi, rcx
1408e4ce8: lea rcx, [rdx + 0x38]
1408e4cec: movabs rdx, 0x3dce36237fb50dff
1408e4cf6: movabs r8, 0x93a27cf4512d885c
1408e4d00: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1408e4d05: test rax, rax
1408e4d08: je 0x1408e5589
1408e4d0e: cmp qword ptr [rsi + 0x218], rdx
1408e4d15: jbe 0x1408e5589
1408e4d1b: mov rax, qword ptr [rsi + 0x210]
1408e4d22: mov rcx, qword ptr [rax + rdx*8]
1408e4d26: test rcx, rcx
1408e4d29: je 0x1408e5589
1408e4d2f: mov rax, qword ptr [rsi + 0x1e0]
1408e4d36: not rcx
1408e4d39: lea rcx, [rcx + rcx*2]
1408e4d3d: shl rcx, 5
1408e4d41: cmp qword ptr [rax + rcx + 0x20], 0
1408e4d47: je 0x1408e5589
1408e4d4d: add rax, rcx
1408e4d50: mov rax, qword ptr [rax + 0x10]
1408e4d54: cmp byte ptr [rax + 0x20], 1
1408e4d58: jne 0x1408e5578
1408e4d5e: mov qword ptr [rsp + 0x30], rdi
1408e4d63: mov qword ptr [rsp + 0x28], rsi
1408e4d68: mov qword ptr [rsp + 0x20], 0x26
1408e4d71: lea rdx, [rip + 0x21c0d88]
1408e4d78: lea r9, [rip + 0x21c0d9e]
1408e4d7f: lea rbx, [rbp + 0x2e0]
1408e4d86: mov r8d, 0x1d
1408e4d8c: mov rcx, rbx
1408e4d8f: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
1408e4d94: lea r14, [rbp + 0x150]
1408e4d9b: movabs r8, 0x5baefac4a34b3dee
1408e4da5: mov rcx, r14
1408e4da8: mov rdx, rbx
1408e4dab: mov r9d, 1
1408e4db1: call 0x141608830 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>
1408e4db6: lea rcx, [rbp + 0x2e0]
1408e4dbd: movabs r8, 0xf3aa86819743c1d8
1408e4dc7: mov rdx, r14
1408e4dca: mov r9d, 1
1408e4dd0: call 0x141608640 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>
1408e4dd5: cmp byte ptr [rbp + 0x416], 8
1408e4ddc: je 0x1408e4e0a
1408e4dde: lea rcx, [rbp + 0x3f0]
1408e4de5: mov edx, 0xe8
1408e4dea: add rdx, qword ptr [rbp + 0x418]
1408e4df1: movabs r8, 0xfb539f7243dbb832
1408e4dfb: movabs r9, 0x5a0000005a
1408e4e05: call 0x14096b760 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<array$<u32,2> >,array$<u32,2> >
1408e4e0a: lea rbx, [rbp + 0x150]
1408e4e11: lea r14, [rbp + 0x2e0]
1408e4e18: mov r8d, 0x190
1408e4e1e: mov rcx, rbx
1408e4e21: mov rdx, r14
1408e4e24: call 0x142923c00
1408e4e29: mov byte ptr [rbp - 0x40], 9
1408e4e2d: mov dword ptr [rbp + 0x2e0], 2
1408e4e37: mov qword ptr [rsp + 0x20], r14
1408e4e3c: lea r14, [rbp - 0x40]
1408e4e40: movabs r8, 0x61b08b5e0470c5f2
1408e4e4a: mov rcx, r14
1408e4e4d: mov rdx, rbx
1408e4e50: mov r9b, 0x19
1408e4e53: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408e4e58: lea r15, [rbp + 0x2e0]
1408e4e5f: mov r8d, 0x190
1408e4e65: mov rcx, r15
1408e4e68: mov rdx, rbx
1408e4e6b: call 0x142923c00
1408e4e70: mov byte ptr [rbp + 0x47f], 0xa
1408e4e77: mov dword ptr [rbp - 0x40], 2
1408e4e7e: mov qword ptr [rsp + 0x20], r14
1408e4e83: lea rcx, [rbp + 0x47f]
1408e4e8a: movabs r8, 0x297d1e4cdae8f657
1408e4e94: mov rdx, r15
1408e4e97: mov r9b, 0x19
1408e4e9a: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408e4e9f: lea rdx, [rbp + 0x2e0]
1408e4ea6: mov r8d, 0x190
1408e4eac: mov rcx, r14
1408e4eaf: call 0x142923c00
1408e4eb4: mov rcx, r15
1408e4eb7: mov rdx, r14
1408e4eba: call 0x14160e630 ; country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$3::bind_garden_exclusion<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e4ebf: mov byte ptr [rbp - 0x40], 0x14
1408e4ec3: mov dword ptr [rbp + 0x150], 2
1408e4ecd: mov qword ptr [rsp + 0x20], rbx
1408e4ed2: lea rcx, [rbp - 0x40]
1408e4ed6: movabs r8, 0x49f43a487883738d
1408e4ee0: mov rdx, r15
1408e4ee3: mov r9b, 0x19
1408e4ee6: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408e4eeb: lea r14, [rbp + 0x2e0]
1408e4ef2: mov r8d, 0x190
1408e4ef8: mov rcx, rbx
1408e4efb: mov rdx, r14
1408e4efe: call 0x142923c00
1408e4f03: mov byte ptr [rbp - 0x40], 0x15
1408e4f07: mov dword ptr [rbp + 0x2e0], 2
1408e4f11: mov qword ptr [rsp + 0x20], r14
1408e4f16: lea rcx, [rbp - 0x40]
1408e4f1a: movabs r8, 0x4a9aecb3eff0f013
1408e4f24: mov rdx, rbx
1408e4f27: mov r9b, 0x19
1408e4f2a: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408e4f2f: lea rbx, [rbp + 0x2e0]
1408e4f36: lea r14, [rbp + 0x150]
1408e4f3d: mov r8d, 0x190
1408e4f43: mov rcx, rbx
1408e4f46: mov rdx, r14
1408e4f49: call 0x142923c00
1408e4f4e: mov byte ptr [rbp - 0x40], 0x16
1408e4f52: mov dword ptr [rbp + 0x150], 2
1408e4f5c: mov qword ptr [rsp + 0x20], r14
1408e4f61: lea rcx, [rbp - 0x40]
1408e4f65: movabs r8, 0x6570181649ac6420
1408e4f6f: mov rdx, rbx
1408e4f72: mov r9b, 0x19
1408e4f75: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408e4f7a: lea rbx, [rbp + 0x150]
1408e4f81: lea rdx, [rbp + 0x2e0]
1408e4f88: mov r8d, 0x190
1408e4f8e: mov rcx, rbx
1408e4f91: call 0x142923c00
1408e4f96: lea rcx, [rbp + 0x2e0]
1408e4f9d: movabs r8, 0xf1c07a2b8012765a
1408e4fa7: mov rdx, rbx
1408e4faa: mov r9d, 1
1408e4fb0: call 0x141608c10 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>
1408e4fb5: cmp byte ptr [rbp + 0x416], 8
1408e4fbc: je 0x1408e4fe6
1408e4fbe: lea rcx, [rbp + 0x3f0]
1408e4fc5: mov edx, 0xe8
1408e4fca: add rdx, qword ptr [rbp + 0x418]
1408e4fd1: movabs r8, 0xe9511a616b75789
1408e4fdb: mov r9d, 0xa
1408e4fe1: call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
1408e4fe6: lea rcx, [rbp - 0x40]
1408e4fea: lea rdx, [rbp + 0x2e0]
1408e4ff1: mov r8d, 0x190
1408e4ff7: call 0x142923c00
1408e4ffc: movups xmm0, xmmword ptr [rbp + 0x60]
1408e5000: movaps xmmword ptr [rbp + 0x190], xmm0
1408e5007: movups xmm0, xmmword ptr [rbp + 0x50]
1408e500b: movaps xmmword ptr [rbp + 0x180], xmm0
1408e5012: movups xmm0, xmmword ptr [rbp + 0x40]
1408e5016: movaps xmmword ptr [rbp + 0x170], xmm0
1408e501d: movups xmm0, xmmword ptr [rbp + 0x30]
1408e5021: movaps xmmword ptr [rbp + 0x160], xmm0
1408e5028: movups xmm0, xmmword ptr [rbp + 0x20]
1408e502c: movaps xmmword ptr [rbp + 0x150], xmm0
1408e5033: mov rax, qword ptr [rbp + 0xf8]
1408e503a: mov rcx, qword ptr [rbp + 0x140]
1408e5041: mov rdx, qword ptr [rbp - 0x30]
1408e5045: mov qword ptr [rbp + 0x2f0], rdx
1408e504c: movups xmm0, xmmword ptr [rbp - 0x40]
1408e5050: movaps xmmword ptr [rbp + 0x2e0], xmm0
1408e5057: movups xmm0, xmmword ptr [rbp - 0x28]
1408e505b: movaps xmmword ptr [rbp + 0x310], xmm0
1408e5062: mov rdx, qword ptr [rbp - 0x18]
1408e5066: mov qword ptr [rbp + 0x320], rdx
1408e506d: movups xmm0, xmmword ptr [rbp - 0x10]
1408e5071: movups xmmword ptr [rbp + 0x2f8], xmm0
1408e5078: mov rdx, qword ptr [rbp]
1408e507c: mov qword ptr [rbp + 0x308], rdx
1408e5083: movups xmm0, xmmword ptr [rbp + 0x70]
1408e5087: movups xmm1, xmmword ptr [rbp + 0x80]
1408e508e: movups xmm2, xmmword ptr [rbp + 0x90]
1408e5095: movups xmm3, xmmword ptr [rbp + 0xa0]
1408e509c: movups xmmword ptr [rbp + 0x358], xmm0
1408e50a3: movups xmmword ptr [rbp + 0x368], xmm1
1408e50aa: movups xmmword ptr [rbp + 0x378], xmm2
1408e50b1: movups xmmword ptr [rbp + 0x388], xmm3
1408e50b8: movups xmm0, xmmword ptr [rbp + 0xb0]
1408e50bf: movups xmmword ptr [rbp + 0x398], xmm0
1408e50c6: movups xmm0, xmmword ptr [rbp + 0xc0]
1408e50cd: movups xmmword ptr [rbp + 0x3a8], xmm0
1408e50d4: mov rdx, qword ptr [rbp + 0x18]
1408e50d8: mov qword ptr [rbp + 0x338], rdx
1408e50df: movups xmm0, xmmword ptr [rbp + 8]
1408e50e3: movups xmmword ptr [rbp + 0x328], xmm0
1408e50ea: mov qword ptr [rbp + 0x3c0], rcx
1408e50f1: mov qword ptr [rbp + 0x3b8], rax
1408e50f8: movaps xmm0, xmmword ptr [rip + 0x21c0021]
1408e50ff: movaps xmmword ptr [rbp + 0x340], xmm0
1408e5106: lea rcx, [rbp + 0x150]
1408e510d: lea rdx, [rbp + 0x2e0]
1408e5114: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
1408e5119: cmp byte ptr [rbp + 0xf6], 8
1408e5120: je 0x1408e512e
1408e5122: lea rcx, [rbp + 0xe0]
1408e5129: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408e512e: mov qword ptr [rsp + 0x30], rdi
1408e5133: mov qword ptr [rsp + 0x28], rsi
1408e5138: mov qword ptr [rsp + 0x20], 0x27
1408e5141: lea rdx, [rip + 0x21c09fb]
1408e5148: lea r9, [rip + 0x21c0a12]
1408e514f: lea rsi, [rbp + 0x2e0]
1408e5156: mov r8d, 0x1e
1408e515c: mov rcx, rsi
1408e515f: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
1408e5164: lea rcx, [rbp + 0x150]
1408e516b: movabs r8, 0x633d7e34d2dd23f1
1408e5175: mov rdx, rsi
1408e5178: mov r9d, 1
1408e517e: call 0x141608640 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>
1408e5183: cmp byte ptr [rbp + 0x286], 8
1408e518a: je 0x1408e51b8
1408e518c: lea rcx, [rbp + 0x260]
1408e5193: mov edx, 0xe8
1408e5198: add rdx, qword ptr [rbp + 0x288]
1408e519f: movabs r8, 0xfb539f7243dbb832
1408e51a9: movabs r9, 0x9600000096
1408e51b3: call 0x14096b760 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<array$<u32,2> >,array$<u32,2> >
1408e51b8: lea rdi, [rbp - 0x40]
1408e51bc: lea rdx, [rbp + 0x150]
1408e51c3: mov r8d, 0x190
1408e51c9: mov rcx, rdi
1408e51cc: call 0x142923c00
1408e51d1: mov byte ptr [rbp + 0x150], 9
1408e51d8: mov dword ptr [rbp + 0x2e0], 2
1408e51e2: mov qword ptr [rsp + 0x20], rsi
1408e51e7: lea rsi, [rbp + 0x150]
1408e51ee: movabs r8, 0x61b08b5e0470c5f2
1408e51f8: mov rcx, rsi
1408e51fb: mov rdx, rdi
1408e51fe: mov r9b, 0x19
1408e5201: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408e5206: lea rdi, [rbp + 0x2e0]
1408e520d: lea rdx, [rbp - 0x40]
1408e5211: mov r8d, 0x190
1408e5217: mov rcx, rdi
1408e521a: call 0x142923c00
1408e521f: mov byte ptr [rbp + 0x47f], 0xa
1408e5226: mov dword ptr [rbp + 0x150], 2
1408e5230: mov qword ptr [rsp + 0x20], rsi
1408e5235: lea rcx, [rbp + 0x47f]
1408e523c: movabs r8, 0x297d1e4cdae8f657
1408e5246: mov rdx, rdi
1408e5249: mov r9b, 0x19
1408e524c: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408e5251: lea rdx, [rbp + 0x2e0]
1408e5258: mov r8d, 0x190
1408e525e: mov rcx, rsi
1408e5261: call 0x142923c00
1408e5266: mov rcx, rdi
1408e5269: mov rdx, rsi
1408e526c: call 0x14160e630 ; country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$3::bind_garden_exclusion<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e5271: mov byte ptr [rbp - 0x40], 0x14
1408e5275: mov dword ptr [rbp + 0x150], 2
1408e527f: mov qword ptr [rsp + 0x20], rsi
1408e5284: lea rcx, [rbp - 0x40]
1408e5288: movabs r8, 0x49f43a487883738d
1408e5292: mov rdx, rdi
1408e5295: mov r9b, 0x19
1408e5298: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408e529d: mov r8d, 0x190
1408e52a3: mov rcx, rsi
1408e52a6: mov rdx, rdi
1408e52a9: call 0x142923c00
1408e52ae: mov byte ptr [rbp - 0x40], 0x15
1408e52b2: mov dword ptr [rbp + 0x2e0], 2
1408e52bc: mov qword ptr [rsp + 0x20], rdi
1408e52c1: lea rcx, [rbp - 0x40]
1408e52c5: movabs r8, 0x4a9aecb3eff0f013
1408e52cf: mov rdx, rsi
1408e52d2: mov r9b, 0x19
1408e52d5: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408e52da: lea rsi, [rbp + 0x2e0]
1408e52e1: lea rdi, [rbp + 0x150]
1408e52e8: mov r8d, 0x190
1408e52ee: mov rcx, rsi
1408e52f1: mov rdx, rdi
1408e52f4: call 0x142923c00
1408e52f9: mov byte ptr [rbp - 0x40], 0x16
1408e52fd: mov dword ptr [rbp + 0x150], 2
1408e5307: mov qword ptr [rsp + 0x20], rdi
1408e530c: lea rcx, [rbp - 0x40]
1408e5310: movabs r8, 0x6570181649ac6420
1408e531a: mov rdx, rsi
1408e531d: mov r9b, 0x19
1408e5320: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408e5325: mov r8d, 0x190
1408e532b: mov rcx, rdi
1408e532e: mov rdx, rsi
1408e5331: call 0x142923c00
1408e5336: mov byte ptr [rbp - 0x40], 9
1408e533a: mov dword ptr [rbp + 0x2e0], 2
1408e5344: mov qword ptr [rsp + 0x20], rsi
1408e5349: lea rsi, [rbp - 0x40]
1408e534d: movabs r8, 0x61b08b5e0470c5f2
1408e5357: mov rcx, rsi
1408e535a: mov rdx, rdi
1408e535d: mov r9b, 0x19
1408e5360: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408e5365: lea rdi, [rbp + 0x2e0]
1408e536c: lea rdx, [rbp + 0x150]
1408e5373: mov r8d, 0x190
1408e5379: mov rcx, rdi
1408e537c: call 0x142923c00
1408e5381: mov byte ptr [rbp + 0x47f], 0xa
1408e5388: mov dword ptr [rbp - 0x40], 2
1408e538f: mov qword ptr [rsp + 0x20], rsi
1408e5394: lea rcx, [rbp + 0x47f]
1408e539b: movabs r8, 0x297d1e4cdae8f657
1408e53a5: mov rdx, rdi
1408e53a8: mov r9b, 0x19
1408e53ab: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1408e53b0: lea rdx, [rbp + 0x2e0]
1408e53b7: mov r8d, 0x190
1408e53bd: mov rcx, rsi
1408e53c0: call 0x142923c00
1408e53c5: lea rcx, [rbp + 0x2e0]
1408e53cc: movabs r8, 0xf1c07a2b8012765a
1408e53d6: mov rdx, rsi
1408e53d9: mov r9d, 1
1408e53df: call 0x141608c10 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>
1408e53e4: cmp byte ptr [rbp + 0x416], 8
1408e53eb: je 0x1408e5415
1408e53ed: lea rcx, [rbp + 0x3f0]
1408e53f4: mov edx, 0xe8
1408e53f9: add rdx, qword ptr [rbp + 0x418]
1408e5400: movabs r8, 0xe9511a616b75789
1408e540a: mov r9d, 0xb
1408e5410: call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
1408e5415: lea rcx, [rbp + 0x150]
1408e541c: lea rdx, [rbp + 0x2e0]
1408e5423: mov r8d, 0x190
1408e5429: call 0x142923c00
1408e542e: movups xmm0, xmmword ptr [rbp + 0x1f0]
1408e5435: movaps xmmword ptr [rbp], xmm0
1408e5439: movups xmm0, xmmword ptr [rbp + 0x1e0]
1408e5440: movaps xmmword ptr [rbp - 0x10], xmm0
1408e5444: movups xmm0, xmmword ptr [rbp + 0x1d0]
1408e544b: movaps xmmword ptr [rbp - 0x20], xmm0
1408e544f: movups xmm0, xmmword ptr [rbp + 0x1c0]
1408e5456: movaps xmmword ptr [rbp - 0x30], xmm0
1408e545a: movups xmm0, xmmword ptr [rbp + 0x1b0]
1408e5461: movaps xmmword ptr [rbp - 0x40], xmm0
1408e5465: mov rax, qword ptr [rbp + 0x288]
1408e546c: mov rcx, qword ptr [rbp + 0x2d0]
1408e5473: mov rdx, qword ptr [rbp + 0x160]
1408e547a: mov qword ptr [rbp + 0x2f0], rdx
1408e5481: movups xmm0, xmmword ptr [rbp + 0x150]
1408e5488: movaps xmmword ptr [rbp + 0x2e0], xmm0
1408e548f: movups xmm0, xmmword ptr [rbp + 0x168]
1408e5496: movaps xmmword ptr [rbp + 0x310], xmm0
1408e549d: mov rdx, qword ptr [rbp + 0x178]
1408e54a4: mov qword ptr [rbp + 0x320], rdx
1408e54ab: movups xmm0, xmmword ptr [rbp + 0x180]
1408e54b2: movups xmmword ptr [rbp + 0x2f8], xmm0
1408e54b9: mov rdx, qword ptr [rbp + 0x190]
1408e54c0: mov qword ptr [rbp + 0x308], rdx
1408e54c7: movups xmm0, xmmword ptr [rbp + 0x200]
1408e54ce: movups xmm1, xmmword ptr [rbp + 0x210]
1408e54d5: movups xmm2, xmmword ptr [rbp + 0x220]
1408e54dc: movups xmm3, xmmword ptr [rbp + 0x230]
1408e54e3: movups xmmword ptr [rbp + 0x358], xmm0
1408e54ea: movups xmmword ptr [rbp + 0x368], xmm1
1408e54f1: movups xmmword ptr [rbp + 0x378], xmm2
1408e54f8: movups xmmword ptr [rbp + 0x388], xmm3
1408e54ff: movups xmm0, xmmword ptr [rbp + 0x240]
1408e5506: movups xmmword ptr [rbp + 0x398], xmm0
1408e550d: movups xmm0, xmmword ptr [rbp + 0x250]
1408e5514: movups xmmword ptr [rbp + 0x3a8], xmm0
1408e551b: mov rdx, qword ptr [rbp + 0x1a8]
1408e5522: mov qword ptr [rbp + 0x338], rdx
1408e5529: movups xmm0, xmmword ptr [rbp + 0x198]
1408e5530: movups xmmword ptr [rbp + 0x328], xmm0
1408e5537: mov qword ptr [rbp + 0x3c0], rcx
1408e553e: mov qword ptr [rbp + 0x3b8], rax
1408e5545: movaps xmm0, xmmword ptr [rip + 0x21bfbe4]
1408e554c: movaps xmmword ptr [rbp + 0x340], xmm0
1408e5553: lea rcx, [rbp - 0x40]
1408e5557: lea rdx, [rbp + 0x2e0]
1408e555e: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
1408e5563: cmp byte ptr [rbp + 0x286], 8
1408e556a: je 0x1408e5578
1408e556c: lea rcx, [rbp + 0x270]
1408e5573: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408e5578: nop 
1408e5579: add rsp, 0x508
1408e5580: pop rbx
1408e5581: pop rdi
1408e5582: pop rsi
1408e5583: pop r14
1408e5585: pop r15
1408e5587: pop rbp
1408e5588: ret 
1408e5589: lea rcx, [rip + 0x21c0558]
1408e5590: call 0x1428d9310 ; core::option::unwrap_failed
1408e5595: int3 
1408e5596: nop word ptr cs:[rax + rax]
1408e55a0: mov qword ptr [rsp + 0x10], rdx
1408e55a5: push rbp
1408e55a6: push r15
1408e55a8: push r14
1408e55aa: push rsi
1408e55ab: push rdi
1408e55ac: push rbx
1408e55ad: sub rsp, 0x38
1408e55b1: lea rbp, [rdx + 0x80]
1408e55b8: cmp byte ptr [rbp + 0xf6], 8
1408e55bf: je 0x1408e55cd
1408e55c1: lea rcx, [rbp + 0xe0]
1408e55c8: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408e55cd: nop 
1408e55ce: add rsp, 0x38
1408e55d2: pop rbx
1408e55d3: pop rdi
1408e55d4: pop rsi
1408e55d5: pop r14
1408e55d7: pop r15
1408e55d9: pop rbp
1408e55da: ret 
1408e55db: nop dword ptr [rax + rax]
1408e55e0: mov qword ptr [rsp + 0x10], rdx
1408e55e5: push rbp
1408e55e6: push r15
1408e55e8: push r14
1408e55ea: push rsi
1408e55eb: push rdi
1408e55ec: push rbx
1408e55ed: sub rsp, 0x38
1408e55f1: lea rbp, [rdx + 0x80]
1408e55f8: lea rcx, [rbp + 0x150]
1408e55ff: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e5604: nop 
1408e5605: add rsp, 0x38
1408e5609: pop rbx
1408e560a: pop rdi
1408e560b: pop rsi
1408e560c: pop r14
1408e560e: pop r15
1408e5610: pop rbp
1408e5611: ret 
1408e5612: nop word ptr cs:[rax + rax]
1408e5620: mov qword ptr [rsp + 0x10], rdx
1408e5625: push rbp
1408e5626: push r15
1408e5628: push r14
1408e562a: push rsi
1408e562b: push rdi
1408e562c: push rbx
1408e562d: sub rsp, 0x38
1408e5631: lea rbp, [rdx + 0x80]
1408e5638: lea rcx, [rbp - 0x40]
1408e563c: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e5641: nop 
1408e5642: add rsp, 0x38
1408e5646: pop rbx
1408e5647: pop rdi
1408e5648: pop rsi
1408e5649: pop r14
1408e564b: pop r15
1408e564d: pop rbp
1408e564e: ret 
1408e564f: nop 
1408e5650: mov qword ptr [rsp + 0x10], rdx
1408e5655: push rbp
1408e5656: push r15
1408e5658: push r14
1408e565a: push rsi
1408e565b: push rdi
1408e565c: push rbx
1408e565d: sub rsp, 0x38
1408e5661: lea rbp, [rdx + 0x80]
1408e5668: lea rcx, [rbp + 0x2e0]
1408e566f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e5674: nop 
1408e5675: add rsp, 0x38
1408e5679: pop rbx
1408e567a: pop rdi
1408e567b: pop rsi
1408e567c: pop r14
1408e567e: pop r15
1408e5680: pop rbp
1408e5681: ret 
1408e5682: nop word ptr cs:[rax + rax]
1408e5690: mov qword ptr [rsp + 0x10], rdx
1408e5695: push rbp
1408e5696: push r15
1408e5698: push r14
1408e569a: push rsi
1408e569b: push rdi
1408e569c: push rbx
1408e569d: sub rsp, 0x38
1408e56a1: lea rbp, [rdx + 0x80]
1408e56a8: lea rcx, [rbp + 0x2e0]
1408e56af: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e56b4: nop 
1408e56b5: add rsp, 0x38
1408e56b9: pop rbx
1408e56ba: pop rdi
1408e56bb: pop rsi
1408e56bc: pop r14
1408e56be: pop r15
1408e56c0: pop rbp
1408e56c1: ret 
1408e56c2: nop word ptr cs:[rax + rax]
1408e56d0: mov qword ptr [rsp + 0x10], rdx
1408e56d5: push rbp
1408e56d6: push r15
1408e56d8: push r14
1408e56da: push rsi
1408e56db: push rdi
1408e56dc: push rbx
1408e56dd: sub rsp, 0x38
1408e56e1: lea rbp, [rdx + 0x80]
1408e56e8: lea rcx, [rbp + 0x150]
1408e56ef: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e56f4: nop 
1408e56f5: add rsp, 0x38
1408e56f9: pop rbx
1408e56fa: pop rdi
1408e56fb: pop rsi
1408e56fc: pop r14
1408e56fe: pop r15
1408e5700: pop rbp
1408e5701: ret 
1408e5702: nop word ptr cs:[rax + rax]
1408e5710: mov qword ptr [rsp + 0x10], rdx
1408e5715: push rbp
1408e5716: push r15
1408e5718: push r14
1408e571a: push rsi
1408e571b: push rdi
1408e571c: push rbx
1408e571d: sub rsp, 0x38
1408e5721: lea rbp, [rdx + 0x80]
1408e5728: lea rcx, [rbp + 0x2e0]
1408e572f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e5734: nop 
1408e5735: add rsp, 0x38
1408e5739: pop rbx
1408e573a: pop rdi
1408e573b: pop rsi
1408e573c: pop r14
1408e573e: pop r15
1408e5740: pop rbp
1408e5741: ret 
1408e5742: nop word ptr cs:[rax + rax]
1408e5750: mov qword ptr [rsp + 0x10], rdx
1408e5755: push rbp
1408e5756: push r15
1408e5758: push r14
1408e575a: push rsi
1408e575b: push rdi
1408e575c: push rbx
1408e575d: sub rsp, 0x38
1408e5761: lea rbp, [rdx + 0x80]
1408e5768: lea rcx, [rbp + 0x150]
1408e576f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e5774: nop 
1408e5775: add rsp, 0x38
1408e5779: pop rbx
1408e577a: pop rdi
1408e577b: pop rsi
1408e577c: pop r14
1408e577e: pop r15
1408e5780: pop rbp
1408e5781: ret 
1408e5782: nop word ptr cs:[rax + rax]
1408e5790: mov qword ptr [rsp + 0x10], rdx
1408e5795: push rbp
1408e5796: push r15
1408e5798: push r14
1408e579a: push rsi
1408e579b: push rdi
1408e579c: push rbx
1408e579d: sub rsp, 0x38
1408e57a1: lea rbp, [rdx + 0x80]
1408e57a8: lea rcx, [rbp + 0x2e0]
1408e57af: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e57b4: nop 
1408e57b5: add rsp, 0x38
1408e57b9: pop rbx
1408e57ba: pop rdi
1408e57bb: pop rsi
1408e57bc: pop r14
1408e57be: pop r15
1408e57c0: pop rbp
1408e57c1: ret 
1408e57c2: nop word ptr cs:[rax + rax]
1408e57d0: mov qword ptr [rsp + 0x10], rdx
1408e57d5: push rbp
1408e57d6: push r15
1408e57d8: push r14
1408e57da: push rsi
1408e57db: push rdi
1408e57dc: push rbx
1408e57dd: sub rsp, 0x38
1408e57e1: lea rbp, [rdx + 0x80]
1408e57e8: lea rcx, [rbp + 0x2e0]
1408e57ef: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e57f4: nop 
1408e57f5: add rsp, 0x38
1408e57f9: pop rbx
1408e57fa: pop rdi
1408e57fb: pop rsi
1408e57fc: pop r14
1408e57fe: pop r15
1408e5800: pop rbp
1408e5801: ret 
1408e5802: nop word ptr cs:[rax + rax]
1408e5810: mov qword ptr [rsp + 0x10], rdx
1408e5815: push rbp
1408e5816: push r15
1408e5818: push r14
1408e581a: push rsi
1408e581b: push rdi
1408e581c: push rbx
1408e581d: sub rsp, 0x38
1408e5821: lea rbp, [rdx + 0x80]
1408e5828: cmp byte ptr [rbp + 0x286], 8
1408e582f: je 0x1408e583d
1408e5831: lea rcx, [rbp + 0x270]
1408e5838: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1408e583d: nop 
1408e583e: add rsp, 0x38
1408e5842: pop rbx
1408e5843: pop rdi
1408e5844: pop rsi
1408e5845: pop r14
1408e5847: pop r15
1408e5849: pop rbp
1408e584a: ret 
1408e584b: nop dword ptr [rax + rax]
1408e5850: mov qword ptr [rsp + 0x10], rdx
1408e5855: push rbp
1408e5856: push r15
1408e5858: push r14
1408e585a: push rsi
1408e585b: push rdi
1408e585c: push rbx
1408e585d: sub rsp, 0x38
1408e5861: lea rbp, [rdx + 0x80]
1408e5868: lea rcx, [rbp + 0x2e0]
1408e586f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e5874: nop 
1408e5875: add rsp, 0x38
1408e5879: pop rbx
1408e587a: pop rdi
1408e587b: pop rsi
1408e587c: pop r14
1408e587e: pop r15
1408e5880: pop rbp
1408e5881: ret 
1408e5882: nop word ptr cs:[rax + rax]
1408e5890: mov qword ptr [rsp + 0x10], rdx
1408e5895: push rbp
1408e5896: push r15
1408e5898: push r14
1408e589a: push rsi
1408e589b: push rdi
1408e589c: push rbx
1408e589d: sub rsp, 0x38
1408e58a1: lea rbp, [rdx + 0x80]
1408e58a8: lea rcx, [rbp + 0x150]
1408e58af: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e58b4: nop 
1408e58b5: add rsp, 0x38
1408e58b9: pop rbx
1408e58ba: pop rdi
1408e58bb: pop rsi
1408e58bc: pop r14
1408e58be: pop r15
1408e58c0: pop rbp
1408e58c1: ret 
1408e58c2: nop word ptr cs:[rax + rax]
1408e58d0: mov qword ptr [rsp + 0x10], rdx
1408e58d5: push rbp
1408e58d6: push r15
1408e58d8: push r14
1408e58da: push rsi
1408e58db: push rdi
1408e58dc: push rbx
1408e58dd: sub rsp, 0x38
1408e58e1: lea rbp, [rdx + 0x80]
1408e58e8: lea rcx, [rbp + 0x2e0]
1408e58ef: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e58f4: nop 
1408e58f5: add rsp, 0x38
1408e58f9: pop rbx
1408e58fa: pop rdi
1408e58fb: pop rsi
1408e58fc: pop r14
1408e58fe: pop r15
1408e5900: pop rbp
1408e5901: ret 
1408e5902: nop word ptr cs:[rax + rax]
1408e5910: mov qword ptr [rsp + 0x10], rdx
1408e5915: push rbp
1408e5916: push r15
1408e5918: push r14
1408e591a: push rsi
1408e591b: push rdi
1408e591c: push rbx
1408e591d: sub rsp, 0x38
1408e5921: lea rbp, [rdx + 0x80]
1408e5928: lea rcx, [rbp + 0x2e0]
1408e592f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e5934: nop 
1408e5935: add rsp, 0x38
1408e5939: pop rbx
1408e593a: pop rdi
1408e593b: pop rsi
1408e593c: pop r14
1408e593e: pop r15
1408e5940: pop rbp
1408e5941: ret 
1408e5942: nop word ptr cs:[rax + rax]
1408e5950: mov qword ptr [rsp + 0x10], rdx
1408e5955: push rbp
1408e5956: push r15
1408e5958: push r14
1408e595a: push rsi
1408e595b: push rdi
1408e595c: push rbx
1408e595d: sub rsp, 0x38
1408e5961: lea rbp, [rdx + 0x80]
1408e5968: lea rcx, [rbp + 0x150]
1408e596f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e5974: nop 
1408e5975: add rsp, 0x38
1408e5979: pop rbx
1408e597a: pop rdi
1408e597b: pop rsi
1408e597c: pop r14
1408e597e: pop r15
1408e5980: pop rbp
1408e5981: ret 
1408e5982: nop word ptr cs:[rax + rax]
1408e5990: mov qword ptr [rsp + 0x10], rdx
1408e5995: push rbp
1408e5996: push r15
1408e5998: push r14
1408e599a: push rsi
1408e599b: push rdi
1408e599c: push rbx
1408e599d: sub rsp, 0x38
1408e59a1: lea rbp, [rdx + 0x80]
1408e59a8: lea rcx, [rbp + 0x2e0]
1408e59af: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e59b4: nop 
1408e59b5: add rsp, 0x38
1408e59b9: pop rbx
1408e59ba: pop rdi
1408e59bb: pop rsi
1408e59bc: pop r14
1408e59be: pop r15
1408e59c0: pop rbp
1408e59c1: ret 
1408e59c2: nop word ptr cs:[rax + rax]
1408e59d0: mov qword ptr [rsp + 0x10], rdx
1408e59d5: push rbp
1408e59d6: push r15
1408e59d8: push r14
1408e59da: push rsi
1408e59db: push rdi
1408e59dc: push rbx
1408e59dd: sub rsp, 0x38
1408e59e1: lea rbp, [rdx + 0x80]
1408e59e8: lea rcx, [rbp + 0x2e0]
1408e59ef: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1408e59f4: nop 
1408e59f5: add rsp, 0x38
1408e59f9: pop rbx
1408e59fa: pop rdi
1408e59fb: pop rsi
1408e59fc: pop r14
1408e59fe: pop r15
1408e5a00: pop rbp
1408e5a01: ret 
