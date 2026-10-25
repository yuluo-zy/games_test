140a3ee80: push rbp
140a3ee81: push r14
140a3ee83: push rsi
140a3ee84: push rdi
140a3ee85: push rbx
140a3ee86: sub rsp, 0x500
140a3ee8d: lea rbp, [rsp + 0x80]
140a3ee95: mov qword ptr [rbp + 0x478], 0xfffffffffffffffe
140a3eea0: mov rdi, rdx
140a3eea3: mov rbx, rcx
140a3eea6: lea rcx, [rdx + 0x38]
140a3eeaa: movabs rdx, 0xfa44538b95ae76c6
140a3eeb4: movabs r8, 0x12146c4ea814069b
140a3eebe: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
140a3eec3: test rax, rax
140a3eec6: je 0x140a3f2e0
140a3eecc: cmp qword ptr [rdi + 0x218], rdx
140a3eed3: jbe 0x140a3f2e0
140a3eed9: mov rax, qword ptr [rdi + 0x210]
140a3eee0: mov rcx, qword ptr [rax + rdx*8]
140a3eee4: test rcx, rcx
140a3eee7: je 0x140a3f2e0
140a3eeed: mov rax, qword ptr [rdi + 0x1e0]
140a3eef4: not rcx
140a3eef7: lea rcx, [rcx + rcx*2]
140a3eefb: shl rcx, 5
140a3eeff: cmp qword ptr [rax + rcx + 0x20], 0
140a3ef05: je 0x140a3f2e0
140a3ef0b: add rax, rcx
140a3ef0e: mov rcx, qword ptr [rax + 0x10]
140a3ef12: movabs rax, 0xaaaaaaaaaaaaaaab
140a3ef1c: mul qword ptr [rcx]
140a3ef1f: mov rsi, rdx
140a3ef22: mov qword ptr [rsp + 0x30], rbx
140a3ef27: mov qword ptr [rsp + 0x28], rdi
140a3ef2c: mov qword ptr [rsp + 0x20], 0x2d
140a3ef35: lea rdx, [rip + 0x208ef7c]
140a3ef3c: lea r9, [rip + 0x208ef8b]
140a3ef43: lea rdi, [rbp + 0x2e0]
140a3ef4a: mov r8d, 0x16
140a3ef50: mov rcx, rdi
140a3ef53: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a3ef58: lea rbx, [rbp + 0x150]
140a3ef5f: movabs r8, 0x12ef39a9447f1f0e
140a3ef69: mov rcx, rbx
140a3ef6c: mov rdx, rdi
140a3ef6f: call 0x141605c50 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::resources::rocky_terrain::RockyTerrainVertexSSBO>
140a3ef74: lea r14, [rbp - 0x40]
140a3ef78: movabs r8, 0x480a6146ae76b082
140a3ef82: mov rcx, r14
140a3ef85: mov rdx, rbx
140a3ef88: call 0x141604c00 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::resources::rocky_terrain::RockyTerrainTriangleSSBO>
140a3ef8d: mov byte ptr [rbp + 0x150], 9
140a3ef94: mov dword ptr [rbp + 0x2e0], 2
140a3ef9e: mov qword ptr [rsp + 0x20], rdi
140a3efa3: lea rdi, [rbp + 0x150]
140a3efaa: movabs r8, 0x61b08b5e0470c5f2
140a3efb4: mov rcx, rdi
140a3efb7: mov rdx, r14
140a3efba: mov r9b, 0x19
140a3efbd: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a3efc2: lea rbx, [rbp + 0x2e0]
140a3efc9: lea rdx, [rbp - 0x40]
140a3efcd: mov r8d, 0x190
140a3efd3: mov rcx, rbx
140a3efd6: call 0x142923c00 ; ?
140a3efdb: mov byte ptr [rbp + 0x477], 0xa
140a3efe2: mov dword ptr [rbp + 0x150], 2
140a3efec: mov qword ptr [rsp + 0x20], rdi
140a3eff1: lea rcx, [rbp + 0x477]
140a3eff8: movabs r8, 0x297d1e4cdae8f657
140a3f002: mov rdx, rbx
140a3f005: mov r9b, 0x19
140a3f008: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a3f00d: mov r8d, 0x190
140a3f013: mov rcx, rdi
140a3f016: mov rdx, rbx
140a3f019: call 0x142923c00 ; ?
140a3f01e: mov byte ptr [rbp - 0x40], 0xe
140a3f022: mov dword ptr [rbp + 0x2e0], 2
140a3f02c: mov qword ptr [rsp + 0x20], rbx
140a3f031: lea rcx, [rbp - 0x40]
140a3f035: movabs r8, 0x4ee098b38612d2b9
140a3f03f: mov rdx, rdi
140a3f042: mov r9b, 0x19
140a3f045: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a3f04a: lea rdi, [rbp + 0x2e0]
140a3f051: lea rbx, [rbp + 0x150]
140a3f058: mov r8d, 0x190
140a3f05e: mov rcx, rdi
140a3f061: mov rdx, rbx
140a3f064: call 0x142923c00 ; ?
140a3f069: mov byte ptr [rbp - 0x40], 5
140a3f06d: mov dword ptr [rbp + 0x150], 2
140a3f077: mov qword ptr [rsp + 0x20], rbx
140a3f07c: lea rcx, [rbp - 0x40]
140a3f080: movabs r8, 0xed502fff6dc4021f
140a3f08a: mov rdx, rdi
140a3f08d: mov r9b, 0x19
140a3f090: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a3f095: lea rdi, [rbp + 0x150]
140a3f09c: lea rbx, [rbp + 0x2e0]
140a3f0a3: mov r8d, 0x190
140a3f0a9: mov rcx, rdi
140a3f0ac: mov rdx, rbx
140a3f0af: call 0x142923c00 ; ?
140a3f0b4: mov byte ptr [rbp - 0x40], 0x13
140a3f0b8: mov dword ptr [rbp + 0x2e0], 2
140a3f0c2: mov qword ptr [rsp + 0x20], rbx
140a3f0c7: lea rcx, [rbp - 0x40]
140a3f0cb: movabs r8, 0x6ad63ccd18c8f041
140a3f0d5: mov rdx, rdi
140a3f0d8: mov r9b, 0x19
140a3f0db: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a3f0e0: lea rcx, [rbp + 0x2e0]
140a3f0e7: lea rdx, [rbp + 0x150]
140a3f0ee: mov r8d, 0x190
140a3f0f4: call 0x142923c00 ; ?
140a3f0f9: movzx eax, byte ptr [rip + 0x2bf5b09]
140a3f100: mov ecx, 4
140a3f105: mov edx, 1
140a3f10a: call 0x140613c10 ; ?
140a3f10f: test rax, rax
140a3f112: je 0x140a3f2ec
140a3f118: mov rdi, rax
140a3f11b: shr rsi, 1
140a3f11e: mov dword ptr [rax], esi
140a3f120: mov rdx, qword ptr [rbp + 0x328]
140a3f127: test rdx, rdx
140a3f12a: je 0x140a3f13e
140a3f12c: mov rcx, qword ptr [rbp + 0x330]
140a3f133: mov r8d, 1
140a3f139: call 0x140613c20 ; ?
140a3f13e: mov qword ptr [rbp + 0x328], 4
140a3f149: mov qword ptr [rbp + 0x330], rdi
140a3f150: mov qword ptr [rbp + 0x338], 4
140a3f15b: lea rcx, [rbp + 0x150]
140a3f162: lea rdx, [rbp + 0x2e0]
140a3f169: mov r8d, 0x190
140a3f16f: call 0x142923c00 ; ?
140a3f174: movups xmm0, xmmword ptr [rbp + 0x1f0]
140a3f17b: movaps xmmword ptr [rbp], xmm0
140a3f17f: movups xmm0, xmmword ptr [rbp + 0x1e0]
140a3f186: movaps xmmword ptr [rbp - 0x10], xmm0
140a3f18a: movups xmm0, xmmword ptr [rbp + 0x1d0]
140a3f191: movaps xmmword ptr [rbp - 0x20], xmm0
140a3f195: movups xmm0, xmmword ptr [rbp + 0x1c0]
140a3f19c: movaps xmmword ptr [rbp - 0x30], xmm0
140a3f1a0: movups xmm0, xmmword ptr [rbp + 0x1b0]
140a3f1a7: movaps xmmword ptr [rbp - 0x40], xmm0
140a3f1ab: mov rax, qword ptr [rbp + 0x288]
140a3f1b2: mov rcx, qword ptr [rbp + 0x2d0]
140a3f1b9: mov rdx, qword ptr [rbp + 0x160]
140a3f1c0: mov qword ptr [rbp + 0x2f0], rdx
140a3f1c7: movups xmm0, xmmword ptr [rbp + 0x150]
140a3f1ce: movaps xmmword ptr [rbp + 0x2e0], xmm0
140a3f1d5: movups xmm0, xmmword ptr [rbp + 0x168]
140a3f1dc: movaps xmmword ptr [rbp + 0x310], xmm0
140a3f1e3: mov rdx, qword ptr [rbp + 0x178]
140a3f1ea: mov qword ptr [rbp + 0x320], rdx
140a3f1f1: movups xmm0, xmmword ptr [rbp + 0x180]
140a3f1f8: movups xmmword ptr [rbp + 0x2f8], xmm0
140a3f1ff: mov rdx, qword ptr [rbp + 0x190]
140a3f206: mov qword ptr [rbp + 0x308], rdx
140a3f20d: movups xmm0, xmmword ptr [rbp + 0x200]
140a3f214: movups xmm1, xmmword ptr [rbp + 0x210]
140a3f21b: movups xmm2, xmmword ptr [rbp + 0x220]
140a3f222: movups xmm3, xmmword ptr [rbp + 0x230]
140a3f229: movups xmmword ptr [rbp + 0x358], xmm0
140a3f230: movups xmmword ptr [rbp + 0x368], xmm1
140a3f237: movups xmmword ptr [rbp + 0x378], xmm2
140a3f23e: movups xmmword ptr [rbp + 0x388], xmm3
140a3f245: movups xmm0, xmmword ptr [rbp + 0x240]
140a3f24c: movups xmmword ptr [rbp + 0x398], xmm0
140a3f253: movups xmm0, xmmword ptr [rbp + 0x250]
140a3f25a: movups xmmword ptr [rbp + 0x3a8], xmm0
140a3f261: mov rdx, qword ptr [rbp + 0x1a8]
140a3f268: mov qword ptr [rbp + 0x338], rdx
140a3f26f: movups xmm0, xmmword ptr [rbp + 0x198]
140a3f276: movups xmmword ptr [rbp + 0x328], xmm0
140a3f27d: mov qword ptr [rbp + 0x3c0], rcx
140a3f284: mov qword ptr [rbp + 0x3b8], rax
140a3f28b: mov dword ptr [rbp + 0x340], 1
140a3f295: mov dword ptr [rbp + 0x344], esi
140a3f29b: movabs rax, 0x100000001
140a3f2a5: mov qword ptr [rbp + 0x348], rax
140a3f2ac: lea rcx, [rbp - 0x40]
140a3f2b0: lea rdx, [rbp + 0x2e0]
140a3f2b7: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a3f2bc: cmp byte ptr [rbp + 0x286], 8
140a3f2c3: je 0x140a3f2d1
140a3f2c5: lea rcx, [rbp + 0x270]
140a3f2cc: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a3f2d1: nop 
140a3f2d2: add rsp, 0x500
140a3f2d9: pop rbx
140a3f2da: pop rdi
140a3f2db: pop rsi
140a3f2dc: pop r14
140a3f2de: pop rbp
140a3f2df: ret 
140a3f2e0: lea rcx, [rip + 0x208ebb9]
140a3f2e7: call 0x1428d9310 ; core::option::unwrap_failed
140a3f2ec: lea r8, [rip + 0x223e5dd]
140a3f2f3: mov ecx, 1
140a3f2f8: mov edx, 4
140a3f2fd: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a3f302: ud2 
140a3f304: nop word ptr cs:[rax + rax]
140a3f310: mov qword ptr [rsp + 0x10], rdx
140a3f315: push rbp
140a3f316: push r14
140a3f318: push rsi
140a3f319: push rdi
140a3f31a: push rbx
140a3f31b: sub rsp, 0x40
140a3f31f: lea rbp, [rdx + 0x80]
140a3f326: cmp byte ptr [rbp + 0x286], 8
140a3f32d: je 0x140a3f33b
140a3f32f: lea rcx, [rbp + 0x270]
140a3f336: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a3f33b: nop 
140a3f33c: add rsp, 0x40
140a3f340: pop rbx
140a3f341: pop rdi
140a3f342: pop rsi
140a3f343: pop r14
140a3f345: pop rbp
140a3f346: ret 
140a3f347: nop word ptr [rax + rax]
140a3f350: mov qword ptr [rsp + 0x10], rdx
140a3f355: push rbp
140a3f356: push r14
140a3f358: push rsi
140a3f359: push rdi
140a3f35a: push rbx
140a3f35b: sub rsp, 0x40
140a3f35f: lea rbp, [rdx + 0x80]
140a3f366: lea rcx, [rbp - 0x40]
140a3f36a: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a3f36f: nop 
140a3f370: add rsp, 0x40
140a3f374: pop rbx
140a3f375: pop rdi
140a3f376: pop rsi
140a3f377: pop r14
140a3f379: pop rbp
140a3f37a: ret 
140a3f37b: nop dword ptr [rax + rax]
140a3f380: mov qword ptr [rsp + 0x10], rdx
140a3f385: push rbp
140a3f386: push r14
140a3f388: push rsi
140a3f389: push rdi
140a3f38a: push rbx
140a3f38b: sub rsp, 0x40
140a3f38f: lea rbp, [rdx + 0x80]
140a3f396: lea rcx, [rbp + 0x2e0]
140a3f39d: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a3f3a2: nop 
140a3f3a3: add rsp, 0x40
140a3f3a7: pop rbx
140a3f3a8: pop rdi
140a3f3a9: pop rsi
140a3f3aa: pop r14
140a3f3ac: pop rbp
140a3f3ad: ret 
140a3f3ae: nop 
140a3f3b0: mov qword ptr [rsp + 0x10], rdx
140a3f3b5: push rbp
140a3f3b6: push r14
140a3f3b8: push rsi
140a3f3b9: push rdi
140a3f3ba: push rbx
140a3f3bb: sub rsp, 0x40
140a3f3bf: lea rbp, [rdx + 0x80]
140a3f3c6: lea rcx, [rbp + 0x150]
140a3f3cd: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a3f3d2: nop 
140a3f3d3: add rsp, 0x40
140a3f3d7: pop rbx
140a3f3d8: pop rdi
140a3f3d9: pop rsi
140a3f3da: pop r14
140a3f3dc: pop rbp
140a3f3dd: ret 
140a3f3de: nop 
140a3f3e0: mov qword ptr [rsp + 0x10], rdx
140a3f3e5: push rbp
140a3f3e6: push r14
140a3f3e8: push rsi
140a3f3e9: push rdi
140a3f3ea: push rbx
140a3f3eb: sub rsp, 0x40
140a3f3ef: lea rbp, [rdx + 0x80]
140a3f3f6: lea rcx, [rbp + 0x2e0]
140a3f3fd: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a3f402: nop 
140a3f403: add rsp, 0x40
140a3f407: pop rbx
140a3f408: pop rdi
140a3f409: pop rsi
140a3f40a: pop r14
140a3f40c: pop rbp
140a3f40d: ret 
140a3f40e: nop 
140a3f410: mov qword ptr [rsp + 0x10], rdx
140a3f415: push rbp
140a3f416: push r14
140a3f418: push rsi
140a3f419: push rdi
140a3f41a: push rbx
140a3f41b: sub rsp, 0x40
140a3f41f: lea rbp, [rdx + 0x80]
140a3f426: lea rcx, [rbp + 0x150]
140a3f42d: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a3f432: nop 
140a3f433: add rsp, 0x40
140a3f437: pop rbx
140a3f438: pop rdi
140a3f439: pop rsi
140a3f43a: pop r14
140a3f43c: pop rbp
140a3f43d: ret 
140a3f43e: nop 
140a3f440: mov qword ptr [rsp + 0x10], rdx
140a3f445: push rbp
140a3f446: push r14
140a3f448: push rsi
140a3f449: push rdi
140a3f44a: push rbx
140a3f44b: sub rsp, 0x40
140a3f44f: lea rbp, [rdx + 0x80]
140a3f456: lea rcx, [rbp + 0x2e0]
140a3f45d: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a3f462: nop 
140a3f463: add rsp, 0x40
140a3f467: pop rbx
140a3f468: pop rdi
140a3f469: pop rsi
140a3f46a: pop r14
140a3f46c: pop rbp
140a3f46d: ret 
