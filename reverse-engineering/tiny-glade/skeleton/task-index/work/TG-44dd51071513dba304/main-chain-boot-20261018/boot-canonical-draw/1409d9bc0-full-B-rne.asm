1409d9bc0: 55 push rbp
1409d9bc1: 4157 push r15
1409d9bc3: 4156 push r14
1409d9bc5: 4155 push r13
1409d9bc7: 4154 push r12
1409d9bc9: 56 push rsi
1409d9bca: 57 push rdi
1409d9bcb: 53 push rbx
1409d9bcc: 4881ec38030000 sub rsp, 0x338
1409d9bd3: 488dac2480000000 lea rbp, [rsp + 0x80]
1409d9bdb: 48c785b0020000feffffff mov qword ptr [rbp + 0x2b0], 0xfffffffffffffffe
1409d9be6: 803da3b1c50200 cmp byte ptr [rip + 0x2c5b1a3], 0
1409d9bed: 48898d98020000 mov qword ptr [rbp + 0x298], rcx
1409d9bf4: 742f je 0x1409d9c25
1409d9bf6: 4889c8 mov rax, rcx
1409d9bf9: 488db5a0010000 lea rsi, [rbp + 0x1a0]
1409d9c00: 41b8f8000000 mov r8d, 0xf8
1409d9c06: 4889f1 mov rcx, rsi
1409d9c09: 4889c2 mov rdx, rax
1409d9c0c: e8ef9ff401 call 0x142923c00 ; 
1409d9c11: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409d9c18: 4889f1 mov rcx, rsi
1409d9c1b: e8a0e6f2ff call 0x1409082c0 ; country_core::render::pass_builder::draw_builder::DrawBuilder::discard
1409d9c20: e952050000 jmp 0x1409da177
1409d9c25: 4c89c6 mov rsi, r8
1409d9c28: 498d7838 lea rdi, [r8 + 0x38]
1409d9c2c: c685af02000001 mov byte ptr [rbp + 0x2af], 1
1409d9c33: 48ba9b71d8c6fc0d4374 movabs rdx, 0x74430dfcc6d8719b
1409d9c3d: 49b89c5064e4f79155ca movabs r8, 0xca5591f7e464509c
1409d9c47: 4889f9 mov rcx, rdi
1409d9c4a: e8a13a70ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
1409d9c4f: 4885c0 test rax, rax
1409d9c52: 0f8434050000 je 0x1409da18c
1409d9c58: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1409d9c5f: 0f8627050000 jbe 0x1409da18c
1409d9c65: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1409d9c6c: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1409d9c70: 4885c9 test rcx, rcx
1409d9c73: 0f8413050000 je 0x1409da18c
1409d9c79: 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
1409d9c80: 48f7d1 not rcx
1409d9c83: 488d0c49 lea rcx, [rcx + rcx*2]
1409d9c87: 48c1e105 shl rcx, 5
1409d9c8b: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1409d9c91: 0f84f5040000 je 0x1409da18c
1409d9c97: 4801c8 add rax, rcx
1409d9c9a: 488b4010 mov rax, qword ptr [rax + 0x10]
1409d9c9e: 448a7058 mov r14b, byte ptr [rax + 0x58]
1409d9ca2: c685af02000001 mov byte ptr [rbp + 0x2af], 1
1409d9ca9: 48ba9b71d8c6fc0d4374 movabs rdx, 0x74430dfcc6d8719b
1409d9cb3: 49b89c5064e4f79155ca movabs r8, 0xca5591f7e464509c
1409d9cbd: 4889f9 mov rcx, rdi
1409d9cc0: e82b3a70ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id
1409d9cc5: 4885c0 test rax, rax
1409d9cc8: 0f84d3040000 je 0x1409da1a1
1409d9cce: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1409d9cd5: 0f86c6040000 jbe 0x1409da1a1
1409d9cdb: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1409d9ce2: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
1409d9ce6: 4885c9 test rcx, rcx
1409d9ce9: 0f84b2040000 je 0x1409da1a1
1409d9cef: 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
1409d9cf6: 48f7d1 not rcx
1409d9cf9: 488d0c49 lea rcx, [rcx + rcx*2]
1409d9cfd: 48c1e105 shl rcx, 5
1409d9d01: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
1409d9d07: 0f8494040000 je 0x1409da1a1
1409d9d0d: 4801c8 add rax, rcx
1409d9d10: 488b4810 mov rcx, qword ptr [rax + 0x10]
1409d9d14: 488b4108 mov rax, qword ptr [rcx + 8]
1409d9d18: 4c8b7910 mov r15, qword ptr [rcx + 0x10]
1409d9d1c: 31c9 xor ecx, ecx
1409d9d1e: 6690 nop 
1409d9d20: 4989cc mov r12, rcx
1409d9d23: 4939cf cmp r15, rcx
1409d9d26: 740c je 0x1409d9d34
1409d9d28: 498d4c2401 lea rcx, [r12 + 1]
1409d9d2d: 42803c2000 cmp byte ptr [rax + r12], 0
1409d9d32: 75ec jne 0x1409d9d20
1409d9d34: 488b9598020000 mov rdx, qword ptr [rbp + 0x298]
1409d9d3b: 80a2ab0000009f and byte ptr [rdx + 0xab], 0x9f
1409d9d42: 488d5db0 lea rbx, [rbp - 0x50]
1409d9d46: 41b8f8000000 mov r8d, 0xf8
1409d9d4c: 4889d9 mov rcx, rbx
1409d9d4f: e8ac9ef401 call 0x142923c00 ; 
1409d9d54: c685a800000009 mov byte ptr [rbp + 0xa8], 9
1409d9d5b: c785a001000002000000 mov dword ptr [rbp + 0x1a0], 2
1409d9d65: 488d85a0010000 lea rax, [rbp + 0x1a0]
1409d9d6c: 4889442420 mov qword ptr [rsp + 0x20], rax
1409d9d71: 488dbda8000000 lea rdi, [rbp + 0xa8]
1409d9d78: 49b8f2c570045e8bb061 movabs r8, 0x61b08b5e0470c5f2
1409d9d82: 4889f9 mov rcx, rdi
1409d9d85: 4889da mov rdx, rbx
1409d9d88: 41b11d mov r9b, 0x1d
1409d9d8b: e8c0eef2ff call 0x140908c50 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>
1409d9d90: 488d9da0010000 lea rbx, [rbp + 0x1a0]
1409d9d97: 488d55b0 lea rdx, [rbp - 0x50]
1409d9d9b: 41b8f8000000 mov r8d, 0xf8
1409d9da1: 4889d9 mov rcx, rbx
1409d9da4: e8579ef401 call 0x142923c00 ; 
1409d9da9: c685a00200000a mov byte ptr [rbp + 0x2a0], 0xa
1409d9db0: c785a800000002000000 mov dword ptr [rbp + 0xa8], 2
1409d9dba: 48897c2420 mov qword ptr [rsp + 0x20], rdi
1409d9dbf: 488d8da0020000 lea rcx, [rbp + 0x2a0]
1409d9dc6: 49b857f6e8da4c1e7d29 movabs r8, 0x297d1e4cdae8f657
1409d9dd0: 4889da mov rdx, rbx
1409d9dd3: 41b11d mov r9b, 0x1d
1409d9dd6: e875eef2ff call 0x140908c50 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>
1409d9ddb: 41b8f8000000 mov r8d, 0xf8
1409d9de1: 4889f9 mov rcx, rdi
1409d9de4: 4889da mov rdx, rbx
1409d9de7: e8149ef401 call 0x142923c00 ; 
1409d9dec: c645b013 mov byte ptr [rbp - 0x50], 0x13
1409d9df0: c785a001000002000000 mov dword ptr [rbp + 0x1a0], 2
1409d9dfa: 48895c2420 mov qword ptr [rsp + 0x20], rbx
1409d9dff: 488d4db0 lea rcx, [rbp - 0x50]
1409d9e03: 49b841f0c818cd3cd66a movabs r8, 0x6ad63ccd18c8f041
1409d9e0d: 4889fa mov rdx, rdi
1409d9e10: 41b11d mov r9b, 0x1d
1409d9e13: e838eef2ff call 0x140908c50 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>
1409d9e18: 488dbda0010000 lea rdi, [rbp + 0x1a0]
1409d9e1f: 488d95a8000000 lea rdx, [rbp + 0xa8]
1409d9e26: 41b8f8000000 mov r8d, 0xf8
1409d9e2c: 4889f9 mov rcx, rdi
1409d9e2f: e8cc9df401 call 0x142923c00 ; 
1409d9e34: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409d9e3b: 488d4db0 lea rcx, [rbp - 0x50]
1409d9e3f: 49b8c052ba66497c1968 movabs r8, 0x68197c4966ba52c0
1409d9e49: 4889fa mov rdx, rdi
1409d9e4c: e83f03c500 call 0x14162a190 ; country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::resources::water::RippleSourceSsbo>
1409d9e51: c685a800000018 mov byte ptr [rbp + 0xa8], 0x18
1409d9e58: c785a001000002000000 mov dword ptr [rbp + 0x1a0], 2
1409d9e62: 48897c2420 mov qword ptr [rsp + 0x20], rdi
1409d9e67: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409d9e6e: 488d55b0 lea rdx, [rbp - 0x50]
1409d9e72: 49b8159635a6f2af32fb movabs r8, 0xfb32aff2a6359615
1409d9e7c: 41b11d mov r9b, 0x1d
1409d9e7f: e8ccedf2ff call 0x140908c50 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>
1409d9e84: 488d5db0 lea rbx, [rbp - 0x50]
1409d9e88: 41b8f8000000 mov r8d, 0xf8
1409d9e8e: 4889f9 mov rcx, rdi
1409d9e91: 4889da mov rdx, rbx
1409d9e94: e8679df401 call 0x142923c00 ; 
1409d9e99: 49bd6518a091ab399f57 movabs r13, 0x579f39ab91a01865
1409d9ea3: 4c89ada8000000 mov qword ptr [rbp + 0xa8], r13
1409d9eaa: c745b002000000 mov dword ptr [rbp - 0x50], 2
1409d9eb1: 48895c2420 mov qword ptr [rsp + 0x20], rbx
1409d9eb6: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409d9ebd: 49b88c97666903f55b81 movabs r8, 0x815bf5036966978c
1409d9ec7: 4889fa mov rdx, rdi
1409d9eca: 41b11d mov r9b, 0x1d
1409d9ecd: e83ee7f2ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409d9ed2: 41b8f8000000 mov r8d, 0xf8
1409d9ed8: 4889d9 mov rcx, rbx
1409d9edb: 4889fa mov rdx, rdi
1409d9ede: e81d9df401 call 0x142923c00 ; 
1409d9ee3: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409d9eea: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409d9ef1: 49b8b0084bdec65465c2 movabs r8, 0xc26554c6de4b08b0
1409d9efb: 4889da mov rdx, rbx
1409d9efe: e88d38c500 call 0x14162d790 ; country_core::render::pass_builder::draw_builder::impl$2::read_storage_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>
1409d9f03: c645b009 mov byte ptr [rbp - 0x50], 9
1409d9f07: c785a001000002000000 mov dword ptr [rbp + 0x1a0], 2
1409d9f11: 48897c2420 mov qword ptr [rsp + 0x20], rdi
1409d9f16: 488d7db0 lea rdi, [rbp - 0x50]
1409d9f1a: 488d95a8000000 lea rdx, [rbp + 0xa8]
1409d9f21: 49b8f2c570045e8bb061 movabs r8, 0x61b08b5e0470c5f2
1409d9f2b: 4889f9 mov rcx, rdi
1409d9f2e: 41b11d mov r9b, 0x1d
1409d9f31: e81aedf2ff call 0x140908c50 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>
1409d9f36: 488d9da0010000 lea rbx, [rbp + 0x1a0]
1409d9f3d: 488d95a8000000 lea rdx, [rbp + 0xa8]
1409d9f44: 41b8f8000000 mov r8d, 0xf8
1409d9f4a: 4889d9 mov rcx, rbx
1409d9f4d: e8ae9cf401 call 0x142923c00 ; 
1409d9f52: c685a00200000a mov byte ptr [rbp + 0x2a0], 0xa
1409d9f59: c745b002000000 mov dword ptr [rbp - 0x50], 2
1409d9f60: 48897c2420 mov qword ptr [rsp + 0x20], rdi
1409d9f65: 488d8da0020000 lea rcx, [rbp + 0x2a0]
1409d9f6c: 49b857f6e8da4c1e7d29 movabs r8, 0x297d1e4cdae8f657
1409d9f76: 4889da mov rdx, rbx
1409d9f79: 41b11d mov r9b, 0x1d
1409d9f7c: e8cfecf2ff call 0x140908c50 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>
1409d9f81: 488d5db0 lea rbx, [rbp - 0x50]
1409d9f85: 488dbda0010000 lea rdi, [rbp + 0x1a0]
1409d9f8c: 41b8f8000000 mov r8d, 0xf8
1409d9f92: 4889d9 mov rcx, rbx
1409d9f95: 4889fa mov rdx, rdi
1409d9f98: e8639cf401 call 0x142923c00 ; 
1409d9f9d: c685a800000008 mov byte ptr [rbp + 0xa8], 8
1409d9fa4: c785a001000002000000 mov dword ptr [rbp + 0x1a0], 2
1409d9fae: 48897c2420 mov qword ptr [rsp + 0x20], rdi
1409d9fb3: 488dbda8000000 lea rdi, [rbp + 0xa8]
1409d9fba: 49b83fca8dac13011677 movabs r8, 0x77160113ac8dca3f
1409d9fc4: 4889f9 mov rcx, rdi
1409d9fc7: 4889da mov rdx, rbx
1409d9fca: 41b11d mov r9b, 0x1d
1409d9fcd: e87eecf2ff call 0x140908c50 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>
1409d9fd2: 488d9da0010000 lea rbx, [rbp + 0x1a0]
1409d9fd9: 488d55b0 lea rdx, [rbp - 0x50]
1409d9fdd: 41b8f8000000 mov r8d, 0xf8
1409d9fe3: 4889d9 mov rcx, rbx
1409d9fe6: e8159cf401 call 0x142923c00 ; 
1409d9feb: 4c89ada0020000 mov qword ptr [rbp + 0x2a0], r13
1409d9ff2: c785a800000002000000 mov dword ptr [rbp + 0xa8], 2
1409d9ffc: 48897c2420 mov qword ptr [rsp + 0x20], rdi
1409da001: 488d8da0020000 lea rcx, [rbp + 0x2a0]
1409da008: 49b88c97666903f55b81 movabs r8, 0x815bf5036966978c
1409da012: 4889da mov rdx, rbx
1409da015: 41b11d mov r9b, 0x1d
1409da018: e8f3e5f2ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409da01d: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409da024: 488d95a0010000 lea rdx, [rbp + 0x1a0]
1409da02b: 41b8f8000000 mov r8d, 0xf8
1409da031: e8ca9bf401 call 0x142923c00 ; 
1409da036: 80bd8601000008 cmp byte ptr [rbp + 0x186], 8
1409da03d: 7478 je 0x1409da0b7
1409da03f: 31c0 xor eax, eax
1409da041: 4d39e7 cmp r15, r12
1409da044: 0f94c0 sete al
1409da047: 488d0df2780e02 lea rcx, [rip + 0x20e78f2]
1409da04e: 660f6ec0 movd xmm0, eax
1409da052: 660f70c050 pshufd xmm0, xmm0, 0x50
1409da057: 660f72f01f pslld xmm0, 0x1f
1409da05c: 660f72e01f psrad xmm0, 0x1f
1409da061: 660f6fc8 movdqa xmm1, xmm0
1409da065: 660fdf0de3780e02 pandn xmm1, xmmword ptr [rip + 0x20e78e3]
1409da06d: 660fdb05eb780e02 pand xmm0, xmmword ptr [rip + 0x20e78eb]
1409da075: f30f101481 movss xmm2, dword ptr [rcx + rax*4]
1409da07a: 660febc1 por xmm0, xmm1
1409da07e: 488d8d60010000 lea rcx, [rbp + 0x160]
1409da085: bae8000000 mov edx, 0xe8
1409da08a: 48039590010000 add rdx, qword ptr [rbp + 0x190]
1409da091: 660fd685a0010000 movq qword ptr [rbp + 0x1a0], xmm0
1409da099: f30f1195a8010000 movss dword ptr [rbp + 0x1a8], xmm2
1409da0a1: 49b87029905d7608716f movabs r8, 0x6f7108765d902970
1409da0ab: 4c8d8da0010000 lea r9, [rbp + 0x1a0]
1409da0b2: e8a915f9ff call 0x14096b660 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<glam::f32::vec3::Vec3>,glam::f32::vec3::Vec3>
1409da0b7: 41b8f8000000 mov r8d, 0xf8
1409da0bd: 4889d9 mov rcx, rbx
1409da0c0: 4889fa mov rdx, rdi
1409da0c3: e8389bf401 call 0x142923c00 ; 
1409da0c8: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da0cf: 488d4db0 lea rcx, [rbp - 0x50]
1409da0d3: 4889da mov rdx, rbx
1409da0d6: 4989f0 mov r8, rsi
1409da0d9: e8f2e8c400 call 0x1416289d0 ; country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409da0de: 4180fe04 cmp r14b, 4
1409da0e2: 756b jne 0x1409da14f
1409da0e4: 488db5a0010000 lea rsi, [rbp + 0x1a0]
1409da0eb: 488d55b0 lea rdx, [rbp - 0x50]
1409da0ef: 41b8f8000000 mov r8d, 0xf8
1409da0f5: 4889f1 mov rcx, rsi
1409da0f8: e8039bf401 call 0x142923c00 ; 
1409da0fd: 48b8ac47d26b3f8555c5 movabs rax, 0xc555853f6bd247ac
1409da107: 488985a0020000 mov qword ptr [rbp + 0x2a0], rax
1409da10e: c785a800000002000000 mov dword ptr [rbp + 0xa8], 2
1409da118: 48897c2420 mov qword ptr [rsp + 0x20], rdi
1409da11d: 488d8da0020000 lea rcx, [rbp + 0x2a0]
1409da124: 49b877775559c8676070 movabs r8, 0x706067c859557777
1409da12e: 4889f2 mov rdx, rsi
1409da131: 41b11d mov r9b, 0x1d
1409da134: e8d7e4f2ff call 0x140908610 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>
1409da139: 488d4db0 lea rcx, [rbp - 0x50]
1409da13d: 488d95a0010000 lea rdx, [rbp + 0x1a0]
1409da144: 41b8f8000000 mov r8d, 0xf8
1409da14a: e8b19af401 call 0x142923c00 ; 
1409da14f: 488db5a0010000 lea rsi, [rbp + 0x1a0]
1409da156: 488d55b0 lea rdx, [rbp - 0x50]
1409da15a: 41b8f8000000 mov r8d, 0xf8
1409da160: 4889f1 mov rcx, rsi
1409da163: e8989af401 call 0x142923c00 ; 
1409da168: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da16f: 4889f1 mov rcx, rsi
1409da172: e8c9cdf2ff call 0x140906f40 ; country_core::render::pass_builder::draw_builder::DrawBuilder::draw
1409da177: 90 nop 
1409da178: 4881c438030000 add rsp, 0x338
1409da17f: 5b pop rbx
1409da180: 5f pop rdi
1409da181: 5e pop rsi
1409da182: 415c pop r12
1409da184: 415d pop r13
1409da186: 415e pop r14
1409da188: 415f pop r15
1409da18a: 5d pop rbp
1409da18b: c3 ret 
1409da18c: c685af02000001 mov byte ptr [rbp + 0x2af], 1
1409da193: 488d0d06890e02 lea rcx, [rip + 0x20e8906]
1409da19a: e871f1ef01 call 0x1428d9310 ; core::option::unwrap_failed
1409da19f: eb13 jmp 0x1409da1b4
1409da1a1: c685af02000001 mov byte ptr [rbp + 0x2af], 1
1409da1a8: 488d0d09890e02 lea rcx, [rip + 0x20e8909]
1409da1af: e85cf1ef01 call 0x1428d9310 ; core::option::unwrap_failed
1409da1b4: 0f0b ud2 
1409da1b6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409da1c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409da1c5: 55 push rbp
1409da1c6: 4157 push r15
1409da1c8: 4156 push r14
1409da1ca: 4155 push r13
1409da1cc: 4154 push r12
1409da1ce: 56 push rsi
1409da1cf: 57 push rdi
1409da1d0: 53 push rbx
1409da1d1: 4883ec28 sub rsp, 0x28
1409da1d5: 488daa80000000 lea rbp, [rdx + 0x80]
1409da1dc: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da1e3: 488d8da0010000 lea rcx, [rbp + 0x1a0]
1409da1ea: e8d193ebff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409da1ef: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da1f6: 4883c428 add rsp, 0x28
1409da1fa: 5b pop rbx
1409da1fb: 5f pop rdi
1409da1fc: 5e pop rsi
1409da1fd: 415c pop r12
1409da1ff: 415d pop r13
1409da201: 415e pop r14
1409da203: 415f pop r15
1409da205: 5d pop rbp
1409da206: c3 ret 
1409da207: 660f1f840000000000 nop word ptr [rax + rax]
1409da210: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409da215: 55 push rbp
1409da216: 4157 push r15
1409da218: 4156 push r14
1409da21a: 4155 push r13
1409da21c: 4154 push r12
1409da21e: 56 push rsi
1409da21f: 57 push rdi
1409da220: 53 push rbx
1409da221: 4883ec28 sub rsp, 0x28
1409da225: 488daa80000000 lea rbp, [rdx + 0x80]
1409da22c: 80bdaf02000000 cmp byte ptr [rbp + 0x2af], 0
1409da233: 7511 jne 0x1409da246
1409da235: 4883c428 add rsp, 0x28
1409da239: 5b pop rbx
1409da23a: 5f pop rdi
1409da23b: 5e pop rsi
1409da23c: 415c pop r12
1409da23e: 415d pop r13
1409da240: 415e pop r14
1409da242: 415f pop r15
1409da244: 5d pop rbp
1409da245: c3 ret 
1409da246: 488b8d98020000 mov rcx, qword ptr [rbp + 0x298]
1409da24d: e86e93ebff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409da252: ebe1 jmp 0x1409da235
1409da254: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409da260: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409da265: 55 push rbp
1409da266: 4157 push r15
1409da268: 4156 push r14
1409da26a: 4155 push r13
1409da26c: 4154 push r12
1409da26e: 56 push rsi
1409da26f: 57 push rdi
1409da270: 53 push rbx
1409da271: 4883ec28 sub rsp, 0x28
1409da275: 488daa80000000 lea rbp, [rdx + 0x80]
1409da27c: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da283: 488d4db0 lea rcx, [rbp - 0x50]
1409da287: e83493ebff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409da28c: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da293: 4883c428 add rsp, 0x28
1409da297: 5b pop rbx
1409da298: 5f pop rdi
1409da299: 5e pop rsi
1409da29a: 415c pop r12
1409da29c: 415d pop r13
1409da29e: 415e pop r14
1409da2a0: 415f pop r15
1409da2a2: 5d pop rbp
1409da2a3: c3 ret 
1409da2a4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409da2b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409da2b5: 55 push rbp
1409da2b6: 4157 push r15
1409da2b8: 4156 push r14
1409da2ba: 4155 push r13
1409da2bc: 4154 push r12
1409da2be: 56 push rsi
1409da2bf: 57 push rdi
1409da2c0: 53 push rbx
1409da2c1: 4883ec28 sub rsp, 0x28
1409da2c5: 488daa80000000 lea rbp, [rdx + 0x80]
1409da2cc: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da2d3: 488d8da0010000 lea rcx, [rbp + 0x1a0]
1409da2da: e8e192ebff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409da2df: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da2e6: 4883c428 add rsp, 0x28
1409da2ea: 5b pop rbx
1409da2eb: 5f pop rdi
1409da2ec: 5e pop rsi
1409da2ed: 415c pop r12
1409da2ef: 415d pop r13
1409da2f1: 415e pop r14
1409da2f3: 415f pop r15
1409da2f5: 5d pop rbp
1409da2f6: c3 ret 
1409da2f7: 660f1f840000000000 nop word ptr [rax + rax]
1409da300: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409da305: 55 push rbp
1409da306: 4157 push r15
1409da308: 4156 push r14
1409da30a: 4155 push r13
1409da30c: 4154 push r12
1409da30e: 56 push rsi
1409da30f: 57 push rdi
1409da310: 53 push rbx
1409da311: 4883ec28 sub rsp, 0x28
1409da315: 488daa80000000 lea rbp, [rdx + 0x80]
1409da31c: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da323: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409da32a: e89192ebff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409da32f: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da336: 4883c428 add rsp, 0x28
1409da33a: 5b pop rbx
1409da33b: 5f pop rdi
1409da33c: 5e pop rsi
1409da33d: 415c pop r12
1409da33f: 415d pop r13
1409da341: 415e pop r14
1409da343: 415f pop r15
1409da345: 5d pop rbp
1409da346: c3 ret 
1409da347: 660f1f840000000000 nop word ptr [rax + rax]
1409da350: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409da355: 55 push rbp
1409da356: 4157 push r15
1409da358: 4156 push r14
1409da35a: 4155 push r13
1409da35c: 4154 push r12
1409da35e: 56 push rsi
1409da35f: 57 push rdi
1409da360: 53 push rbx
1409da361: 4883ec28 sub rsp, 0x28
1409da365: 488daa80000000 lea rbp, [rdx + 0x80]
1409da36c: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da373: 488d4db0 lea rcx, [rbp - 0x50]
1409da377: e84492ebff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409da37c: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da383: 4883c428 add rsp, 0x28
1409da387: 5b pop rbx
1409da388: 5f pop rdi
1409da389: 5e pop rsi
1409da38a: 415c pop r12
1409da38c: 415d pop r13
1409da38e: 415e pop r14
1409da390: 415f pop r15
1409da392: 5d pop rbp
1409da393: c3 ret 
1409da394: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409da3a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409da3a5: 55 push rbp
1409da3a6: 4157 push r15
1409da3a8: 4156 push r14
1409da3aa: 4155 push r13
1409da3ac: 4154 push r12
1409da3ae: 56 push rsi
1409da3af: 57 push rdi
1409da3b0: 53 push rbx
1409da3b1: 4883ec28 sub rsp, 0x28
1409da3b5: 488daa80000000 lea rbp, [rdx + 0x80]
1409da3bc: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da3c3: 488d8da0010000 lea rcx, [rbp + 0x1a0]
1409da3ca: e8f191ebff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409da3cf: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da3d6: 4883c428 add rsp, 0x28
1409da3da: 5b pop rbx
1409da3db: 5f pop rdi
1409da3dc: 5e pop rsi
1409da3dd: 415c pop r12
1409da3df: 415d pop r13
1409da3e1: 415e pop r14
1409da3e3: 415f pop r15
1409da3e5: 5d pop rbp
1409da3e6: c3 ret 
1409da3e7: 660f1f840000000000 nop word ptr [rax + rax]
1409da3f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409da3f5: 55 push rbp
1409da3f6: 4157 push r15
1409da3f8: 4156 push r14
1409da3fa: 4155 push r13
1409da3fc: 4154 push r12
1409da3fe: 56 push rsi
1409da3ff: 57 push rdi
1409da400: 53 push rbx
1409da401: 4883ec28 sub rsp, 0x28
1409da405: 488daa80000000 lea rbp, [rdx + 0x80]
1409da40c: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da413: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409da41a: e8a191ebff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409da41f: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da426: 4883c428 add rsp, 0x28
1409da42a: 5b pop rbx
1409da42b: 5f pop rdi
1409da42c: 5e pop rsi
1409da42d: 415c pop r12
1409da42f: 415d pop r13
1409da431: 415e pop r14
1409da433: 415f pop r15
1409da435: 5d pop rbp
1409da436: c3 ret 
1409da437: 660f1f840000000000 nop word ptr [rax + rax]
1409da440: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409da445: 55 push rbp
1409da446: 4157 push r15
1409da448: 4156 push r14
1409da44a: 4155 push r13
1409da44c: 4154 push r12
1409da44e: 56 push rsi
1409da44f: 57 push rdi
1409da450: 53 push rbx
1409da451: 4883ec28 sub rsp, 0x28
1409da455: 488daa80000000 lea rbp, [rdx + 0x80]
1409da45c: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da463: 488d8da0010000 lea rcx, [rbp + 0x1a0]
1409da46a: e85191ebff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409da46f: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da476: 4883c428 add rsp, 0x28
1409da47a: 5b pop rbx
1409da47b: 5f pop rdi
1409da47c: 5e pop rsi
1409da47d: 415c pop r12
1409da47f: 415d pop r13
1409da481: 415e pop r14
1409da483: 415f pop r15
1409da485: 5d pop rbp
1409da486: c3 ret 
1409da487: 660f1f840000000000 nop word ptr [rax + rax]
1409da490: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409da495: 55 push rbp
1409da496: 4157 push r15
1409da498: 4156 push r14
1409da49a: 4155 push r13
1409da49c: 4154 push r12
1409da49e: 56 push rsi
1409da49f: 57 push rdi
1409da4a0: 53 push rbx
1409da4a1: 4883ec28 sub rsp, 0x28
1409da4a5: 488daa80000000 lea rbp, [rdx + 0x80]
1409da4ac: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da4b3: 488d4db0 lea rcx, [rbp - 0x50]
1409da4b7: e80491ebff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409da4bc: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da4c3: 4883c428 add rsp, 0x28
1409da4c7: 5b pop rbx
1409da4c8: 5f pop rdi
1409da4c9: 5e pop rsi
1409da4ca: 415c pop r12
1409da4cc: 415d pop r13
1409da4ce: 415e pop r14
1409da4d0: 415f pop r15
1409da4d2: 5d pop rbp
1409da4d3: c3 ret 
1409da4d4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1409da4e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409da4e5: 55 push rbp
1409da4e6: 4157 push r15
1409da4e8: 4156 push r14
1409da4ea: 4155 push r13
1409da4ec: 4154 push r12
1409da4ee: 56 push rsi
1409da4ef: 57 push rdi
1409da4f0: 53 push rbx
1409da4f1: 4883ec28 sub rsp, 0x28
1409da4f5: 488daa80000000 lea rbp, [rdx + 0x80]
1409da4fc: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da503: 488d8da0010000 lea rcx, [rbp + 0x1a0]
1409da50a: e8b190ebff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409da50f: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da516: 4883c428 add rsp, 0x28
1409da51a: 5b pop rbx
1409da51b: 5f pop rdi
1409da51c: 5e pop rsi
1409da51d: 415c pop r12
1409da51f: 415d pop r13
1409da521: 415e pop r14
1409da523: 415f pop r15
1409da525: 5d pop rbp
1409da526: c3 ret 
1409da527: 660f1f840000000000 nop word ptr [rax + rax]
1409da530: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409da535: 55 push rbp
1409da536: 4157 push r15
1409da538: 4156 push r14
1409da53a: 4155 push r13
1409da53c: 4154 push r12
1409da53e: 56 push rsi
1409da53f: 57 push rdi
1409da540: 53 push rbx
1409da541: 4883ec28 sub rsp, 0x28
1409da545: 488daa80000000 lea rbp, [rdx + 0x80]
1409da54c: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da553: 488d8da8000000 lea rcx, [rbp + 0xa8]
1409da55a: e86190ebff call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409da55f: c685af02000000 mov byte ptr [rbp + 0x2af], 0
1409da566: 4883c428 add rsp, 0x28
1409da56a: 5b pop rbx
1409da56b: 5f pop rdi
1409da56c: 5e pop rsi
1409da56d: 415c pop r12
1409da56f: 415d pop r13
1409da571: 415e pop r14
1409da573: 415f pop r15
1409da575: 5d pop rbp
1409da576: c3 ret 
