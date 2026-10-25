140a22b80: push rbp
140a22b81: push r15
140a22b83: push r14
140a22b85: push rsi
140a22b86: push rdi
140a22b87: push rbx
140a22b88: sub rsp, 0x508
140a22b8f: lea rbp, [rsp + 0x80]
140a22b97: mov qword ptr [rbp + 0x480], 0xfffffffffffffffe
140a22ba2: mov rsi, rdx
140a22ba5: mov rdi, rcx
140a22ba8: mov qword ptr [rsp + 0x30], rcx
140a22bad: mov qword ptr [rsp + 0x28], rdx
140a22bb2: mov qword ptr [rsp + 0x20], 0x23
140a22bbb: lea rdx, [rip + 0x20a6d9e]
140a22bc2: lea r9, [rip + 0x20a6da3]
140a22bc9: lea rbx, [rbp + 0x2e0]
140a22bd0: mov r8d, 0xc
140a22bd6: mov rcx, rbx
140a22bd9: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a22bde: lea r14, [rbp + 0x150]
140a22be5: movabs r8, 0x5baefac4a34b3dee
140a22bef: mov rcx, r14
140a22bf2: mov rdx, rbx
140a22bf5: mov r9d, 1
140a22bfb: call 0x141608830 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo>
140a22c00: lea rcx, [rbp + 0x2e0]
140a22c07: movabs r8, 0xf3aa86819743c1d8
140a22c11: mov rdx, r14
140a22c14: mov r9d, 1
140a22c1a: call 0x141608640 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>
140a22c1f: cmp byte ptr [rbp + 0x416], 8
140a22c26: je 0x140a22c54
140a22c28: lea rcx, [rbp + 0x3f0]
140a22c2f: mov edx, 0xe8
140a22c34: add rdx, qword ptr [rbp + 0x418]
140a22c3b: movabs r8, 0xfb539f7243dbb832
140a22c45: movabs r9, 0x5a0000005a
140a22c4f: call 0x14096b760 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<array$<u32,2> >,array$<u32,2> >
140a22c54: lea rbx, [rbp + 0x150]
140a22c5b: lea r14, [rbp + 0x2e0]
140a22c62: mov r8d, 0x190
140a22c68: mov rcx, rbx
140a22c6b: mov rdx, r14
140a22c6e: call 0x142923c00
140a22c73: mov byte ptr [rbp - 0x40], 9
140a22c77: mov dword ptr [rbp + 0x2e0], 2
140a22c81: mov qword ptr [rsp + 0x20], r14
140a22c86: lea r14, [rbp - 0x40]
140a22c8a: movabs r8, 0x61b08b5e0470c5f2
140a22c94: mov rcx, r14
140a22c97: mov rdx, rbx
140a22c9a: mov r9b, 0x19
140a22c9d: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a22ca2: lea r15, [rbp + 0x2e0]
140a22ca9: mov r8d, 0x190
140a22caf: mov rcx, r15
140a22cb2: mov rdx, rbx
140a22cb5: call 0x142923c00
140a22cba: mov byte ptr [rbp + 0x47f], 0xa
140a22cc1: mov dword ptr [rbp - 0x40], 2
140a22cc8: mov qword ptr [rsp + 0x20], r14
140a22ccd: lea rcx, [rbp + 0x47f]
140a22cd4: movabs r8, 0x297d1e4cdae8f657
140a22cde: mov rdx, r15
140a22ce1: mov r9b, 0x19
140a22ce4: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a22ce9: lea rdx, [rbp + 0x2e0]
140a22cf0: mov r8d, 0x190
140a22cf6: mov rcx, r14
140a22cf9: call 0x142923c00
140a22cfe: mov rcx, r15
140a22d01: mov rdx, r14
140a22d04: call 0x14160e630 ; country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$3::bind_garden_exclusion<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a22d09: mov byte ptr [rbp - 0x40], 0x14
140a22d0d: mov dword ptr [rbp + 0x150], 2
140a22d17: mov qword ptr [rsp + 0x20], rbx
140a22d1c: lea rcx, [rbp - 0x40]
140a22d20: movabs r8, 0x49f43a487883738d
140a22d2a: mov rdx, r15
140a22d2d: mov r9b, 0x19
140a22d30: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a22d35: lea r14, [rbp + 0x2e0]
140a22d3c: mov r8d, 0x190
140a22d42: mov rcx, rbx
140a22d45: mov rdx, r14
140a22d48: call 0x142923c00
140a22d4d: mov byte ptr [rbp + 0x47f], 0x16
140a22d54: mov dword ptr [rbp + 0x2e0], 2
140a22d5e: mov qword ptr [rsp + 0x20], r14
140a22d63: lea rcx, [rbp + 0x47f]
140a22d6a: movabs r8, 0x6570181649ac6420
140a22d74: mov rdx, rbx
140a22d77: mov r9b, 0x19
140a22d7a: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a22d7f: lea rbx, [rbp - 0x40]
140a22d83: lea rdx, [rbp + 0x150]
140a22d8a: mov r8d, 0x190
140a22d90: mov rcx, rbx
140a22d93: call 0x142923c00
140a22d98: lea rcx, [rbp + 0x2e0]
140a22d9f: movabs r8, 0xf1c07a2b8012765a
140a22da9: mov rdx, rbx
140a22dac: mov r9d, 1
140a22db2: call 0x141608c10 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>
140a22db7: cmp byte ptr [rbp + 0x416], 8
140a22dbe: je 0x140a22de8
140a22dc0: lea rcx, [rbp + 0x3f0]
140a22dc7: mov edx, 0xe8
140a22dcc: add rdx, qword ptr [rbp + 0x418]
140a22dd3: movabs r8, 0xe9511a616b75789
140a22ddd: mov r9d, 0xa
140a22de3: call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a22de8: lea rcx, [rbp + 0x150]
140a22def: lea rdx, [rbp + 0x2e0]
140a22df6: mov r8d, 0x190
140a22dfc: call 0x142923c00
140a22e01: movups xmm0, xmmword ptr [rbp + 0x1f0]
140a22e08: movaps xmmword ptr [rbp], xmm0
140a22e0c: movups xmm0, xmmword ptr [rbp + 0x1e0]
140a22e13: movaps xmmword ptr [rbp - 0x10], xmm0
140a22e17: movups xmm0, xmmword ptr [rbp + 0x1d0]
140a22e1e: movaps xmmword ptr [rbp - 0x20], xmm0
140a22e22: movups xmm0, xmmword ptr [rbp + 0x1c0]
140a22e29: movaps xmmword ptr [rbp - 0x30], xmm0
140a22e2d: movups xmm0, xmmword ptr [rbp + 0x1b0]
140a22e34: movaps xmmword ptr [rbp - 0x40], xmm0
140a22e38: mov rax, qword ptr [rbp + 0x288]
140a22e3f: mov rcx, qword ptr [rbp + 0x2d0]
140a22e46: mov rdx, qword ptr [rbp + 0x160]
140a22e4d: mov qword ptr [rbp + 0x2f0], rdx
140a22e54: movups xmm0, xmmword ptr [rbp + 0x150]
140a22e5b: movaps xmmword ptr [rbp + 0x2e0], xmm0
140a22e62: movups xmm0, xmmword ptr [rbp + 0x168]
140a22e69: movaps xmmword ptr [rbp + 0x310], xmm0
140a22e70: mov rdx, qword ptr [rbp + 0x178]
140a22e77: mov qword ptr [rbp + 0x320], rdx
140a22e7e: movups xmm0, xmmword ptr [rbp + 0x180]
140a22e85: movups xmmword ptr [rbp + 0x2f8], xmm0
140a22e8c: mov rdx, qword ptr [rbp + 0x190]
140a22e93: mov qword ptr [rbp + 0x308], rdx
140a22e9a: movups xmm0, xmmword ptr [rbp + 0x200]
140a22ea1: movups xmm1, xmmword ptr [rbp + 0x210]
140a22ea8: movups xmm2, xmmword ptr [rbp + 0x220]
140a22eaf: movups xmm3, xmmword ptr [rbp + 0x230]
140a22eb6: movups xmmword ptr [rbp + 0x358], xmm0
140a22ebd: movups xmmword ptr [rbp + 0x368], xmm1
140a22ec4: movups xmmword ptr [rbp + 0x378], xmm2
140a22ecb: movups xmmword ptr [rbp + 0x388], xmm3
140a22ed2: movups xmm0, xmmword ptr [rbp + 0x240]
140a22ed9: movups xmmword ptr [rbp + 0x398], xmm0
140a22ee0: movups xmm0, xmmword ptr [rbp + 0x250]
140a22ee7: movups xmmword ptr [rbp + 0x3a8], xmm0
140a22eee: mov rdx, qword ptr [rbp + 0x1a8]
140a22ef5: mov qword ptr [rbp + 0x338], rdx
140a22efc: movups xmm0, xmmword ptr [rbp + 0x198]
140a22f03: movups xmmword ptr [rbp + 0x328], xmm0
140a22f0a: mov qword ptr [rbp + 0x3c0], rcx
140a22f11: mov qword ptr [rbp + 0x3b8], rax
140a22f18: movaps xmm0, xmmword ptr [rip + 0x2082201]
140a22f1f: movaps xmmword ptr [rbp + 0x340], xmm0
140a22f26: lea rcx, [rbp - 0x40]
140a22f2a: lea rdx, [rbp + 0x2e0]
140a22f31: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a22f36: cmp byte ptr [rbp + 0x286], 8
140a22f3d: je 0x140a22f4b
140a22f3f: lea rcx, [rbp + 0x270]
140a22f46: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a22f4b: mov qword ptr [rsp + 0x30], rdi
140a22f50: mov qword ptr [rsp + 0x28], rsi
140a22f55: mov qword ptr [rsp + 0x20], 0x24
140a22f5e: lea rdx, [rip + 0x20a6a2a]
140a22f65: lea r9, [rip + 0x20a6a30]
140a22f6c: lea rbx, [rbp + 0x2e0]
140a22f73: mov r8d, 0xd
140a22f79: mov rcx, rbx
140a22f7c: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a22f81: lea rcx, [rbp + 0x150]
140a22f88: movabs r8, 0x633d7e34d2dd23f1
140a22f92: mov rdx, rbx
140a22f95: mov r9d, 1
140a22f9b: call 0x141608640 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>
140a22fa0: cmp byte ptr [rbp + 0x286], 8
140a22fa7: je 0x140a22fd5
140a22fa9: lea rcx, [rbp + 0x260]
140a22fb0: mov edx, 0xe8
140a22fb5: add rdx, qword ptr [rbp + 0x288]
140a22fbc: movabs r8, 0xfb539f7243dbb832
140a22fc6: movabs r9, 0x9600000096
140a22fd0: call 0x14096b760 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<array$<u32,2> >,array$<u32,2> >
140a22fd5: lea r14, [rbp - 0x40]
140a22fd9: lea rdx, [rbp + 0x150]
140a22fe0: mov r8d, 0x190
140a22fe6: mov rcx, r14
140a22fe9: call 0x142923c00
140a22fee: mov byte ptr [rbp + 0x150], 9
140a22ff5: mov dword ptr [rbp + 0x2e0], 2
140a22fff: mov qword ptr [rsp + 0x20], rbx
140a23004: lea rbx, [rbp + 0x150]
140a2300b: movabs r8, 0x61b08b5e0470c5f2
140a23015: mov rcx, rbx
140a23018: mov rdx, r14
140a2301b: mov r9b, 0x19
140a2301e: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a23023: lea r14, [rbp + 0x2e0]
140a2302a: lea rdx, [rbp - 0x40]
140a2302e: mov r8d, 0x190
140a23034: mov rcx, r14
140a23037: call 0x142923c00
140a2303c: mov byte ptr [rbp + 0x47f], 0xa
140a23043: mov dword ptr [rbp + 0x150], 2
140a2304d: mov qword ptr [rsp + 0x20], rbx
140a23052: lea rcx, [rbp + 0x47f]
140a23059: movabs r8, 0x297d1e4cdae8f657
140a23063: mov rdx, r14
140a23066: mov r9b, 0x19
140a23069: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a2306e: lea rdx, [rbp + 0x2e0]
140a23075: mov r8d, 0x190
140a2307b: mov rcx, rbx
140a2307e: call 0x142923c00
140a23083: mov rcx, r14
140a23086: mov rdx, rbx
140a23089: call 0x14160e630 ; country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$3::bind_garden_exclusion<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a2308e: mov byte ptr [rbp - 0x40], 0x14
140a23092: mov dword ptr [rbp + 0x150], 2
140a2309c: mov qword ptr [rsp + 0x20], rbx
140a230a1: lea rcx, [rbp - 0x40]
140a230a5: movabs r8, 0x49f43a487883738d
140a230af: mov rdx, r14
140a230b2: mov r9b, 0x19
140a230b5: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a230ba: mov r8d, 0x190
140a230c0: mov rcx, rbx
140a230c3: mov rdx, r14
140a230c6: call 0x142923c00
140a230cb: mov byte ptr [rbp + 0x47f], 0x16
140a230d2: mov dword ptr [rbp + 0x2e0], 2
140a230dc: mov qword ptr [rsp + 0x20], r14
140a230e1: lea rcx, [rbp + 0x47f]
140a230e8: movabs r8, 0x6570181649ac6420
140a230f2: mov rdx, rbx
140a230f5: mov r9b, 0x19
140a230f8: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a230fd: lea rbx, [rbp - 0x40]
140a23101: lea rdx, [rbp + 0x150]
140a23108: mov r8d, 0x190
140a2310e: mov rcx, rbx
140a23111: call 0x142923c00
140a23116: lea rcx, [rbp + 0x2e0]
140a2311d: movabs r8, 0xf1c07a2b8012765a
140a23127: mov rdx, rbx
140a2312a: mov r9d, 1
140a23130: call 0x141608c10 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>
140a23135: cmp byte ptr [rbp + 0x416], 8
140a2313c: je 0x140a23166
140a2313e: lea rcx, [rbp + 0x3f0]
140a23145: mov edx, 0xe8
140a2314a: add rdx, qword ptr [rbp + 0x418]
140a23151: movabs r8, 0xe9511a616b75789
140a2315b: mov r9d, 0xb
140a23161: call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a23166: lea rcx, [rbp + 0x150]
140a2316d: lea rdx, [rbp + 0x2e0]
140a23174: mov r8d, 0x190
140a2317a: call 0x142923c00
140a2317f: movups xmm0, xmmword ptr [rbp + 0x1f0]
140a23186: movaps xmmword ptr [rbp], xmm0
140a2318a: movups xmm0, xmmword ptr [rbp + 0x1e0]
140a23191: movaps xmmword ptr [rbp - 0x10], xmm0
140a23195: movups xmm0, xmmword ptr [rbp + 0x1d0]
140a2319c: movaps xmmword ptr [rbp - 0x20], xmm0
140a231a0: movups xmm0, xmmword ptr [rbp + 0x1c0]
140a231a7: movaps xmmword ptr [rbp - 0x30], xmm0
140a231ab: movups xmm0, xmmword ptr [rbp + 0x1b0]
140a231b2: movaps xmmword ptr [rbp - 0x40], xmm0
140a231b6: mov rax, qword ptr [rbp + 0x288]
140a231bd: mov rcx, qword ptr [rbp + 0x2d0]
140a231c4: mov rdx, qword ptr [rbp + 0x160]
140a231cb: mov qword ptr [rbp + 0x2f0], rdx
140a231d2: movups xmm0, xmmword ptr [rbp + 0x150]
140a231d9: movaps xmmword ptr [rbp + 0x2e0], xmm0
140a231e0: movups xmm0, xmmword ptr [rbp + 0x168]
140a231e7: movaps xmmword ptr [rbp + 0x310], xmm0
140a231ee: mov rdx, qword ptr [rbp + 0x178]
140a231f5: mov qword ptr [rbp + 0x320], rdx
140a231fc: movups xmm0, xmmword ptr [rbp + 0x180]
140a23203: movups xmmword ptr [rbp + 0x2f8], xmm0
140a2320a: mov rdx, qword ptr [rbp + 0x190]
140a23211: mov qword ptr [rbp + 0x308], rdx
140a23218: movups xmm0, xmmword ptr [rbp + 0x200]
140a2321f: movups xmm1, xmmword ptr [rbp + 0x210]
140a23226: movups xmm2, xmmword ptr [rbp + 0x220]
140a2322d: movups xmm3, xmmword ptr [rbp + 0x230]
140a23234: movups xmmword ptr [rbp + 0x358], xmm0
140a2323b: movups xmmword ptr [rbp + 0x368], xmm1
140a23242: movups xmmword ptr [rbp + 0x378], xmm2
140a23249: movups xmmword ptr [rbp + 0x388], xmm3
140a23250: movups xmm0, xmmword ptr [rbp + 0x240]
140a23257: movups xmmword ptr [rbp + 0x398], xmm0
140a2325e: movups xmm0, xmmword ptr [rbp + 0x250]
140a23265: movups xmmword ptr [rbp + 0x3a8], xmm0
140a2326c: mov rdx, qword ptr [rbp + 0x1a8]
140a23273: mov qword ptr [rbp + 0x338], rdx
140a2327a: movups xmm0, xmmword ptr [rbp + 0x198]
140a23281: movups xmmword ptr [rbp + 0x328], xmm0
140a23288: mov qword ptr [rbp + 0x3c0], rcx
140a2328f: mov qword ptr [rbp + 0x3b8], rax
140a23296: movaps xmm0, xmmword ptr [rip + 0x2081e93]
140a2329d: movaps xmmword ptr [rbp + 0x340], xmm0
140a232a4: lea rcx, [rbp - 0x40]
140a232a8: lea rdx, [rbp + 0x2e0]
140a232af: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a232b4: cmp byte ptr [rbp + 0x286], 8
140a232bb: je 0x140a232c9
140a232bd: lea rcx, [rbp + 0x270]
140a232c4: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a232c9: mov qword ptr [rsp + 0x30], rdi
140a232ce: mov qword ptr [rsp + 0x28], rsi
140a232d3: mov qword ptr [rsp + 0x20], 0x20
140a232dc: lea rdx, [rip + 0x20a66dd]
140a232e3: lea r9, [rip + 0x20a66ed]
140a232ea: lea rbx, [rbp + 0x2e0]
140a232f1: mov r8d, 0x17
140a232f7: mov rcx, rbx
140a232fa: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a232ff: lea r14, [rbp + 0x150]
140a23306: mov rcx, r14
140a23309: mov rdx, rbx
140a2330c: call 0x14160e630 ; country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$3::bind_garden_exclusion<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23311: mov byte ptr [rbp - 0x40], 0x14
140a23315: mov dword ptr [rbp + 0x2e0], 2
140a2331f: mov qword ptr [rsp + 0x20], rbx
140a23324: lea rcx, [rbp - 0x40]
140a23328: movabs r8, 0x49f43a487883738d
140a23332: mov rdx, r14
140a23335: mov r9b, 0x19
140a23338: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a2333d: lea rdx, [rbp + 0x150]
140a23344: mov r8d, 0x190
140a2334a: mov rcx, rbx
140a2334d: call 0x142923c00
140a23352: lea rcx, [rbp + 0x150]
140a23359: movabs r8, 0x6d9256d4ac7887ad
140a23363: mov rdx, rbx
140a23366: call 0x141604fa0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>
140a2336b: cmp byte ptr [rbp + 0x286], 8
140a23372: je 0x140a233a0
140a23374: lea rcx, [rbp + 0x260]
140a2337b: mov edx, 0xe8
140a23380: add rdx, qword ptr [rbp + 0x288]
140a23387: movabs r8, 0xf32096f91a47fc71
140a23391: movabs r9, 0xb4000000b4
140a2339b: call 0x14096b760 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<array$<u32,2> >,array$<u32,2> >
140a233a0: lea r14, [rbp - 0x40]
140a233a4: lea rdx, [rbp + 0x150]
140a233ab: mov r8d, 0x190
140a233b1: mov rcx, r14
140a233b4: call 0x142923c00
140a233b9: mov byte ptr [rbp + 0x150], 9
140a233c0: mov dword ptr [rbp + 0x2e0], 2
140a233ca: mov qword ptr [rsp + 0x20], rbx
140a233cf: lea rbx, [rbp + 0x150]
140a233d6: movabs r8, 0x61b08b5e0470c5f2
140a233e0: mov rcx, rbx
140a233e3: mov rdx, r14
140a233e6: mov r9b, 0x19
140a233e9: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a233ee: lea r14, [rbp + 0x2e0]
140a233f5: lea rdx, [rbp - 0x40]
140a233f9: mov r8d, 0x190
140a233ff: mov rcx, r14
140a23402: call 0x142923c00
140a23407: mov byte ptr [rbp + 0x47f], 0xa
140a2340e: mov dword ptr [rbp + 0x150], 2
140a23418: mov qword ptr [rsp + 0x20], rbx
140a2341d: lea rcx, [rbp + 0x47f]
140a23424: movabs r8, 0x297d1e4cdae8f657
140a2342e: mov rdx, r14
140a23431: mov r9b, 0x19
140a23434: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a23439: lea rdx, [rbp + 0x2e0]
140a23440: mov r8d, 0x190
140a23446: mov rcx, rbx
140a23449: call 0x142923c00
140a2344e: lea rcx, [rbp + 0x2e0]
140a23455: movabs r8, 0xf1c07a2b8012765a
140a2345f: mov rdx, rbx
140a23462: mov r9d, 1
140a23468: call 0x141608c10 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>
140a2346d: cmp byte ptr [rbp + 0x416], 8
140a23474: je 0x140a2349e
140a23476: lea rcx, [rbp + 0x3f0]
140a2347d: mov edx, 0xe8
140a23482: add rdx, qword ptr [rbp + 0x418]
140a23489: movabs r8, 0xe9511a616b75789
140a23493: mov r9d, 0xe
140a23499: call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a2349e: lea rcx, [rbp - 0x40]
140a234a2: lea rdx, [rbp + 0x2e0]
140a234a9: mov r8d, 0x190
140a234af: call 0x142923c00
140a234b4: movups xmm0, xmmword ptr [rbp + 0x60]
140a234b8: movaps xmmword ptr [rbp + 0x190], xmm0
140a234bf: movups xmm0, xmmword ptr [rbp + 0x50]
140a234c3: movaps xmmword ptr [rbp + 0x180], xmm0
140a234ca: movups xmm0, xmmword ptr [rbp + 0x40]
140a234ce: movaps xmmword ptr [rbp + 0x170], xmm0
140a234d5: movups xmm0, xmmword ptr [rbp + 0x30]
140a234d9: movaps xmmword ptr [rbp + 0x160], xmm0
140a234e0: movups xmm0, xmmword ptr [rbp + 0x20]
140a234e4: movaps xmmword ptr [rbp + 0x150], xmm0
140a234eb: mov rax, qword ptr [rbp + 0xf8]
140a234f2: mov rcx, qword ptr [rbp + 0x140]
140a234f9: mov rdx, qword ptr [rbp - 0x30]
140a234fd: mov qword ptr [rbp + 0x2f0], rdx
140a23504: movups xmm0, xmmword ptr [rbp - 0x40]
140a23508: movaps xmmword ptr [rbp + 0x2e0], xmm0
140a2350f: movups xmm0, xmmword ptr [rbp - 0x28]
140a23513: movaps xmmword ptr [rbp + 0x310], xmm0
140a2351a: mov rdx, qword ptr [rbp - 0x18]
140a2351e: mov qword ptr [rbp + 0x320], rdx
140a23525: movups xmm0, xmmword ptr [rbp - 0x10]
140a23529: movups xmmword ptr [rbp + 0x2f8], xmm0
140a23530: mov rdx, qword ptr [rbp]
140a23534: mov qword ptr [rbp + 0x308], rdx
140a2353b: movups xmm0, xmmword ptr [rbp + 0x70]
140a2353f: movups xmm1, xmmword ptr [rbp + 0x80]
140a23546: movups xmm2, xmmword ptr [rbp + 0x90]
140a2354d: movups xmm3, xmmword ptr [rbp + 0xa0]
140a23554: movups xmmword ptr [rbp + 0x358], xmm0
140a2355b: movups xmmword ptr [rbp + 0x368], xmm1
140a23562: movups xmmword ptr [rbp + 0x378], xmm2
140a23569: movups xmmword ptr [rbp + 0x388], xmm3
140a23570: movups xmm0, xmmword ptr [rbp + 0xb0]
140a23577: movups xmmword ptr [rbp + 0x398], xmm0
140a2357e: movups xmm0, xmmword ptr [rbp + 0xc0]
140a23585: movups xmmword ptr [rbp + 0x3a8], xmm0
140a2358c: mov rdx, qword ptr [rbp + 0x18]
140a23590: mov qword ptr [rbp + 0x338], rdx
140a23597: movups xmm0, xmmword ptr [rbp + 8]
140a2359b: movups xmmword ptr [rbp + 0x328], xmm0
140a235a2: mov qword ptr [rbp + 0x3c0], rcx
140a235a9: mov qword ptr [rbp + 0x3b8], rax
140a235b0: movaps xmm0, xmmword ptr [rip + 0x20a5429]
140a235b7: movaps xmmword ptr [rbp + 0x340], xmm0
140a235be: lea rcx, [rbp + 0x150]
140a235c5: lea rdx, [rbp + 0x2e0]
140a235cc: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a235d1: cmp byte ptr [rbp + 0xf6], 8
140a235d8: je 0x140a235e6
140a235da: lea rcx, [rbp + 0xe0]
140a235e1: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a235e6: mov qword ptr [rsp + 0x30], rdi
140a235eb: mov qword ptr [rsp + 0x28], rsi
140a235f0: mov qword ptr [rsp + 0x20], 0x2a
140a235f9: lea rdx, [rip + 0x20a63f7]
140a23600: lea r9, [rip + 0x20a6403]
140a23607: lea rsi, [rbp + 0x2e0]
140a2360e: mov r8d, 0x13
140a23614: mov rcx, rsi
140a23617: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a2361c: lea rcx, [rbp + 0x150]
140a23623: movabs r8, 0x7369bcad63f3af6e
140a2362d: mov rdx, rsi
140a23630: call 0x141605e20 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>
140a23635: cmp byte ptr [rbp + 0x286], 8
140a2363c: je 0x140a2366a
140a2363e: lea rcx, [rbp + 0x260]
140a23645: mov edx, 0xe8
140a2364a: add rdx, qword ptr [rbp + 0x288]
140a23651: movabs r8, 0xfb539f7243dbb832
140a2365b: movabs r9, 0x19000000190
140a23665: call 0x14096b760 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<array$<u32,2> >,array$<u32,2> >
140a2366a: lea rdi, [rbp - 0x40]
140a2366e: lea rdx, [rbp + 0x150]
140a23675: mov r8d, 0x190
140a2367b: mov rcx, rdi
140a2367e: call 0x142923c00
140a23683: mov byte ptr [rbp + 0x150], 9
140a2368a: mov dword ptr [rbp + 0x2e0], 2
140a23694: mov qword ptr [rsp + 0x20], rsi
140a23699: lea rsi, [rbp + 0x150]
140a236a0: movabs r8, 0x61b08b5e0470c5f2
140a236aa: mov rcx, rsi
140a236ad: mov rdx, rdi
140a236b0: mov r9b, 0x19
140a236b3: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a236b8: lea rdi, [rbp + 0x2e0]
140a236bf: lea rdx, [rbp - 0x40]
140a236c3: mov r8d, 0x190
140a236c9: mov rcx, rdi
140a236cc: call 0x142923c00
140a236d1: mov byte ptr [rbp + 0x47f], 0xa
140a236d8: mov dword ptr [rbp + 0x150], 2
140a236e2: mov qword ptr [rsp + 0x20], rsi
140a236e7: lea rcx, [rbp + 0x47f]
140a236ee: movabs r8, 0x297d1e4cdae8f657
140a236f8: mov rdx, rdi
140a236fb: mov r9b, 0x19
140a236fe: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a23703: lea rdx, [rbp + 0x2e0]
140a2370a: mov r8d, 0x190
140a23710: mov rcx, rsi
140a23713: call 0x142923c00
140a23718: mov rcx, rdi
140a2371b: mov rdx, rsi
140a2371e: call 0x14160e630 ; country_core::systems::render_loop::draw_passes::draw_shaders::common_includes::impl$3::bind_garden_exclusion<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23723: mov byte ptr [rbp - 0x40], 0x14
140a23727: mov dword ptr [rbp + 0x150], 2
140a23731: mov qword ptr [rsp + 0x20], rsi
140a23736: lea rcx, [rbp - 0x40]
140a2373a: movabs r8, 0x49f43a487883738d
140a23744: mov rdx, rdi
140a23747: mov r9b, 0x19
140a2374a: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a2374f: mov r8d, 0x190
140a23755: mov rcx, rsi
140a23758: mov rdx, rdi
140a2375b: call 0x142923c00
140a23760: mov byte ptr [rbp + 0x47f], 0x15
140a23767: mov dword ptr [rbp + 0x2e0], 2
140a23771: mov qword ptr [rsp + 0x20], rdi
140a23776: lea rcx, [rbp + 0x47f]
140a2377d: movabs r8, 0x4a9aecb3eff0f013
140a23787: mov rdx, rsi
140a2378a: mov r9b, 0x19
140a2378d: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a23792: lea rsi, [rbp - 0x40]
140a23796: lea rdx, [rbp + 0x150]
140a2379d: mov r8d, 0x190
140a237a3: mov rcx, rsi
140a237a6: call 0x142923c00
140a237ab: lea rcx, [rbp + 0x2e0]
140a237b2: movabs r8, 0xf1c07a2b8012765a
140a237bc: mov rdx, rsi
140a237bf: mov r9d, 1
140a237c5: call 0x141608c10 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>
140a237ca: cmp byte ptr [rbp + 0x416], 8
140a237d1: je 0x140a237fb
140a237d3: lea rcx, [rbp + 0x3f0]
140a237da: mov edx, 0xe8
140a237df: add rdx, qword ptr [rbp + 0x418]
140a237e6: movabs r8, 0xe9511a616b75789
140a237f0: mov r9d, 0x10
140a237f6: call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a237fb: lea rcx, [rbp + 0x150]
140a23802: lea rdx, [rbp + 0x2e0]
140a23809: mov r8d, 0x190
140a2380f: call 0x142923c00
140a23814: movups xmm0, xmmword ptr [rbp + 0x1f0]
140a2381b: movaps xmmword ptr [rbp], xmm0
140a2381f: movups xmm0, xmmword ptr [rbp + 0x1e0]
140a23826: movaps xmmword ptr [rbp - 0x10], xmm0
140a2382a: movups xmm0, xmmword ptr [rbp + 0x1d0]
140a23831: movaps xmmword ptr [rbp - 0x20], xmm0
140a23835: movups xmm0, xmmword ptr [rbp + 0x1c0]
140a2383c: movaps xmmword ptr [rbp - 0x30], xmm0
140a23840: movups xmm0, xmmword ptr [rbp + 0x1b0]
140a23847: movaps xmmword ptr [rbp - 0x40], xmm0
140a2384b: mov rax, qword ptr [rbp + 0x288]
140a23852: mov rcx, qword ptr [rbp + 0x2d0]
140a23859: mov rdx, qword ptr [rbp + 0x160]
140a23860: mov qword ptr [rbp + 0x2f0], rdx
140a23867: movups xmm0, xmmword ptr [rbp + 0x150]
140a2386e: movaps xmmword ptr [rbp + 0x2e0], xmm0
140a23875: movups xmm0, xmmword ptr [rbp + 0x168]
140a2387c: movaps xmmword ptr [rbp + 0x310], xmm0
140a23883: mov rdx, qword ptr [rbp + 0x178]
140a2388a: mov qword ptr [rbp + 0x320], rdx
140a23891: movups xmm0, xmmword ptr [rbp + 0x180]
140a23898: movups xmmword ptr [rbp + 0x2f8], xmm0
140a2389f: mov rdx, qword ptr [rbp + 0x190]
140a238a6: mov qword ptr [rbp + 0x308], rdx
140a238ad: movups xmm0, xmmword ptr [rbp + 0x200]
140a238b4: movups xmm1, xmmword ptr [rbp + 0x210]
140a238bb: movups xmm2, xmmword ptr [rbp + 0x220]
140a238c2: movups xmm3, xmmword ptr [rbp + 0x230]
140a238c9: movups xmmword ptr [rbp + 0x358], xmm0
140a238d0: movups xmmword ptr [rbp + 0x368], xmm1
140a238d7: movups xmmword ptr [rbp + 0x378], xmm2
140a238de: movups xmmword ptr [rbp + 0x388], xmm3
140a238e5: movups xmm0, xmmword ptr [rbp + 0x240]
140a238ec: movups xmmword ptr [rbp + 0x398], xmm0
140a238f3: movups xmm0, xmmword ptr [rbp + 0x250]
140a238fa: movups xmmword ptr [rbp + 0x3a8], xmm0
140a23901: mov rdx, qword ptr [rbp + 0x1a8]
140a23908: mov qword ptr [rbp + 0x338], rdx
140a2390f: movups xmm0, xmmword ptr [rbp + 0x198]
140a23916: movups xmmword ptr [rbp + 0x328], xmm0
140a2391d: mov qword ptr [rbp + 0x3c0], rcx
140a23924: mov qword ptr [rbp + 0x3b8], rax
140a2392b: movaps xmm0, xmmword ptr [rip + 0x20a50be]
140a23932: movaps xmmword ptr [rbp + 0x340], xmm0
140a23939: lea rcx, [rbp - 0x40]
140a2393d: lea rdx, [rbp + 0x2e0]
140a23944: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a23949: cmp byte ptr [rbp + 0x286], 8
140a23950: je 0x140a2395e
140a23952: lea rcx, [rbp + 0x270]
140a23959: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a2395e: nop 
140a2395f: add rsp, 0x508
140a23966: pop rbx
140a23967: pop rdi
140a23968: pop rsi
140a23969: pop r14
140a2396b: pop r15
140a2396d: pop rbp
140a2396e: ret 
140a2396f: nop 
140a23970: mov qword ptr [rsp + 0x10], rdx
140a23975: push rbp
140a23976: push r15
140a23978: push r14
140a2397a: push rsi
140a2397b: push rdi
140a2397c: push rbx
140a2397d: sub rsp, 0x38
140a23981: lea rbp, [rdx + 0x80]
140a23988: cmp byte ptr [rbp + 0x286], 8
140a2398f: je 0x140a2399d
140a23991: lea rcx, [rbp + 0x270]
140a23998: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a2399d: nop 
140a2399e: add rsp, 0x38
140a239a2: pop rbx
140a239a3: pop rdi
140a239a4: pop rsi
140a239a5: pop r14
140a239a7: pop r15
140a239a9: pop rbp
140a239aa: ret 
140a239ab: nop dword ptr [rax + rax]
140a239b0: mov qword ptr [rsp + 0x10], rdx
140a239b5: push rbp
140a239b6: push r15
140a239b8: push r14
140a239ba: push rsi
140a239bb: push rdi
140a239bc: push rbx
140a239bd: sub rsp, 0x38
140a239c1: lea rbp, [rdx + 0x80]
140a239c8: lea rcx, [rbp + 0x150]
140a239cf: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a239d4: nop 
140a239d5: add rsp, 0x38
140a239d9: pop rbx
140a239da: pop rdi
140a239db: pop rsi
140a239dc: pop r14
140a239de: pop r15
140a239e0: pop rbp
140a239e1: ret 
140a239e2: nop word ptr cs:[rax + rax]
140a239f0: mov qword ptr [rsp + 0x10], rdx
140a239f5: push rbp
140a239f6: push r15
140a239f8: push r14
140a239fa: push rsi
140a239fb: push rdi
140a239fc: push rbx
140a239fd: sub rsp, 0x38
140a23a01: lea rbp, [rdx + 0x80]
140a23a08: lea rcx, [rbp - 0x40]
140a23a0c: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23a11: nop 
140a23a12: add rsp, 0x38
140a23a16: pop rbx
140a23a17: pop rdi
140a23a18: pop rsi
140a23a19: pop r14
140a23a1b: pop r15
140a23a1d: pop rbp
140a23a1e: ret 
140a23a1f: nop 
140a23a20: mov qword ptr [rsp + 0x10], rdx
140a23a25: push rbp
140a23a26: push r15
140a23a28: push r14
140a23a2a: push rsi
140a23a2b: push rdi
140a23a2c: push rbx
140a23a2d: sub rsp, 0x38
140a23a31: lea rbp, [rdx + 0x80]
140a23a38: lea rcx, [rbp + 0x2e0]
140a23a3f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23a44: nop 
140a23a45: add rsp, 0x38
140a23a49: pop rbx
140a23a4a: pop rdi
140a23a4b: pop rsi
140a23a4c: pop r14
140a23a4e: pop r15
140a23a50: pop rbp
140a23a51: ret 
140a23a52: nop word ptr cs:[rax + rax]
140a23a60: mov qword ptr [rsp + 0x10], rdx
140a23a65: push rbp
140a23a66: push r15
140a23a68: push r14
140a23a6a: push rsi
140a23a6b: push rdi
140a23a6c: push rbx
140a23a6d: sub rsp, 0x38
140a23a71: lea rbp, [rdx + 0x80]
140a23a78: lea rcx, [rbp + 0x2e0]
140a23a7f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23a84: nop 
140a23a85: add rsp, 0x38
140a23a89: pop rbx
140a23a8a: pop rdi
140a23a8b: pop rsi
140a23a8c: pop r14
140a23a8e: pop r15
140a23a90: pop rbp
140a23a91: ret 
140a23a92: nop word ptr cs:[rax + rax]
140a23aa0: mov qword ptr [rsp + 0x10], rdx
140a23aa5: push rbp
140a23aa6: push r15
140a23aa8: push r14
140a23aaa: push rsi
140a23aab: push rdi
140a23aac: push rbx
140a23aad: sub rsp, 0x38
140a23ab1: lea rbp, [rdx + 0x80]
140a23ab8: lea rcx, [rbp + 0x150]
140a23abf: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23ac4: nop 
140a23ac5: add rsp, 0x38
140a23ac9: pop rbx
140a23aca: pop rdi
140a23acb: pop rsi
140a23acc: pop r14
140a23ace: pop r15
140a23ad0: pop rbp
140a23ad1: ret 
140a23ad2: nop word ptr cs:[rax + rax]
140a23ae0: mov qword ptr [rsp + 0x10], rdx
140a23ae5: push rbp
140a23ae6: push r15
140a23ae8: push r14
140a23aea: push rsi
140a23aeb: push rdi
140a23aec: push rbx
140a23aed: sub rsp, 0x38
140a23af1: lea rbp, [rdx + 0x80]
140a23af8: lea rcx, [rbp + 0x2e0]
140a23aff: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23b04: nop 
140a23b05: add rsp, 0x38
140a23b09: pop rbx
140a23b0a: pop rdi
140a23b0b: pop rsi
140a23b0c: pop r14
140a23b0e: pop r15
140a23b10: pop rbp
140a23b11: ret 
140a23b12: nop word ptr cs:[rax + rax]
140a23b20: mov qword ptr [rsp + 0x10], rdx
140a23b25: push rbp
140a23b26: push r15
140a23b28: push r14
140a23b2a: push rsi
140a23b2b: push rdi
140a23b2c: push rbx
140a23b2d: sub rsp, 0x38
140a23b31: lea rbp, [rdx + 0x80]
140a23b38: cmp byte ptr [rbp + 0x286], 8
140a23b3f: je 0x140a23b4d
140a23b41: lea rcx, [rbp + 0x270]
140a23b48: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a23b4d: nop 
140a23b4e: add rsp, 0x38
140a23b52: pop rbx
140a23b53: pop rdi
140a23b54: pop rsi
140a23b55: pop r14
140a23b57: pop r15
140a23b59: pop rbp
140a23b5a: ret 
140a23b5b: nop dword ptr [rax + rax]
140a23b60: mov qword ptr [rsp + 0x10], rdx
140a23b65: push rbp
140a23b66: push r15
140a23b68: push r14
140a23b6a: push rsi
140a23b6b: push rdi
140a23b6c: push rbx
140a23b6d: sub rsp, 0x38
140a23b71: lea rbp, [rdx + 0x80]
140a23b78: lea rcx, [rbp + 0x150]
140a23b7f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23b84: nop 
140a23b85: add rsp, 0x38
140a23b89: pop rbx
140a23b8a: pop rdi
140a23b8b: pop rsi
140a23b8c: pop r14
140a23b8e: pop r15
140a23b90: pop rbp
140a23b91: ret 
140a23b92: nop word ptr cs:[rax + rax]
140a23ba0: mov qword ptr [rsp + 0x10], rdx
140a23ba5: push rbp
140a23ba6: push r15
140a23ba8: push r14
140a23baa: push rsi
140a23bab: push rdi
140a23bac: push rbx
140a23bad: sub rsp, 0x38
140a23bb1: lea rbp, [rdx + 0x80]
140a23bb8: lea rcx, [rbp + 0x150]
140a23bbf: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23bc4: nop 
140a23bc5: add rsp, 0x38
140a23bc9: pop rbx
140a23bca: pop rdi
140a23bcb: pop rsi
140a23bcc: pop r14
140a23bce: pop r15
140a23bd0: pop rbp
140a23bd1: ret 
140a23bd2: nop word ptr cs:[rax + rax]
140a23be0: mov qword ptr [rsp + 0x10], rdx
140a23be5: push rbp
140a23be6: push r15
140a23be8: push r14
140a23bea: push rsi
140a23beb: push rdi
140a23bec: push rbx
140a23bed: sub rsp, 0x38
140a23bf1: lea rbp, [rdx + 0x80]
140a23bf8: lea rcx, [rbp - 0x40]
140a23bfc: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23c01: nop 
140a23c02: add rsp, 0x38
140a23c06: pop rbx
140a23c07: pop rdi
140a23c08: pop rsi
140a23c09: pop r14
140a23c0b: pop r15
140a23c0d: pop rbp
140a23c0e: ret 
140a23c0f: nop 
140a23c10: mov qword ptr [rsp + 0x10], rdx
140a23c15: push rbp
140a23c16: push r15
140a23c18: push r14
140a23c1a: push rsi
140a23c1b: push rdi
140a23c1c: push rbx
140a23c1d: sub rsp, 0x38
140a23c21: lea rbp, [rdx + 0x80]
140a23c28: lea rcx, [rbp + 0x2e0]
140a23c2f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23c34: nop 
140a23c35: add rsp, 0x38
140a23c39: pop rbx
140a23c3a: pop rdi
140a23c3b: pop rsi
140a23c3c: pop r14
140a23c3e: pop r15
140a23c40: pop rbp
140a23c41: ret 
140a23c42: nop word ptr cs:[rax + rax]
140a23c50: mov qword ptr [rsp + 0x10], rdx
140a23c55: push rbp
140a23c56: push r15
140a23c58: push r14
140a23c5a: push rsi
140a23c5b: push rdi
140a23c5c: push rbx
140a23c5d: sub rsp, 0x38
140a23c61: lea rbp, [rdx + 0x80]
140a23c68: lea rcx, [rbp + 0x2e0]
140a23c6f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23c74: nop 
140a23c75: add rsp, 0x38
140a23c79: pop rbx
140a23c7a: pop rdi
140a23c7b: pop rsi
140a23c7c: pop r14
140a23c7e: pop r15
140a23c80: pop rbp
140a23c81: ret 
140a23c82: nop word ptr cs:[rax + rax]
140a23c90: mov qword ptr [rsp + 0x10], rdx
140a23c95: push rbp
140a23c96: push r15
140a23c98: push r14
140a23c9a: push rsi
140a23c9b: push rdi
140a23c9c: push rbx
140a23c9d: sub rsp, 0x38
140a23ca1: lea rbp, [rdx + 0x80]
140a23ca8: cmp byte ptr [rbp + 0xf6], 8
140a23caf: je 0x140a23cbd
140a23cb1: lea rcx, [rbp + 0xe0]
140a23cb8: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a23cbd: nop 
140a23cbe: add rsp, 0x38
140a23cc2: pop rbx
140a23cc3: pop rdi
140a23cc4: pop rsi
140a23cc5: pop r14
140a23cc7: pop r15
140a23cc9: pop rbp
140a23cca: ret 
140a23ccb: nop dword ptr [rax + rax]
140a23cd0: mov qword ptr [rsp + 0x10], rdx
140a23cd5: push rbp
140a23cd6: push r15
140a23cd8: push r14
140a23cda: push rsi
140a23cdb: push rdi
140a23cdc: push rbx
140a23cdd: sub rsp, 0x38
140a23ce1: lea rbp, [rdx + 0x80]
140a23ce8: lea rcx, [rbp + 0x150]
140a23cef: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23cf4: nop 
140a23cf5: add rsp, 0x38
140a23cf9: pop rbx
140a23cfa: pop rdi
140a23cfb: pop rsi
140a23cfc: pop r14
140a23cfe: pop r15
140a23d00: pop rbp
140a23d01: ret 
140a23d02: nop word ptr cs:[rax + rax]
140a23d10: mov qword ptr [rsp + 0x10], rdx
140a23d15: push rbp
140a23d16: push r15
140a23d18: push r14
140a23d1a: push rsi
140a23d1b: push rdi
140a23d1c: push rbx
140a23d1d: sub rsp, 0x38
140a23d21: lea rbp, [rdx + 0x80]
140a23d28: lea rcx, [rbp - 0x40]
140a23d2c: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23d31: nop 
140a23d32: add rsp, 0x38
140a23d36: pop rbx
140a23d37: pop rdi
140a23d38: pop rsi
140a23d39: pop r14
140a23d3b: pop r15
140a23d3d: pop rbp
140a23d3e: ret 
140a23d3f: nop 
140a23d40: mov qword ptr [rsp + 0x10], rdx
140a23d45: push rbp
140a23d46: push r15
140a23d48: push r14
140a23d4a: push rsi
140a23d4b: push rdi
140a23d4c: push rbx
140a23d4d: sub rsp, 0x38
140a23d51: lea rbp, [rdx + 0x80]
140a23d58: lea rcx, [rbp + 0x2e0]
140a23d5f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23d64: nop 
140a23d65: add rsp, 0x38
140a23d69: pop rbx
140a23d6a: pop rdi
140a23d6b: pop rsi
140a23d6c: pop r14
140a23d6e: pop r15
140a23d70: pop rbp
140a23d71: ret 
140a23d72: nop word ptr cs:[rax + rax]
140a23d80: mov qword ptr [rsp + 0x10], rdx
140a23d85: push rbp
140a23d86: push r15
140a23d88: push r14
140a23d8a: push rsi
140a23d8b: push rdi
140a23d8c: push rbx
140a23d8d: sub rsp, 0x38
140a23d91: lea rbp, [rdx + 0x80]
140a23d98: lea rcx, [rbp + 0x2e0]
140a23d9f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23da4: nop 
140a23da5: add rsp, 0x38
140a23da9: pop rbx
140a23daa: pop rdi
140a23dab: pop rsi
140a23dac: pop r14
140a23dae: pop r15
140a23db0: pop rbp
140a23db1: ret 
140a23db2: nop word ptr cs:[rax + rax]
140a23dc0: mov qword ptr [rsp + 0x10], rdx
140a23dc5: push rbp
140a23dc6: push r15
140a23dc8: push r14
140a23dca: push rsi
140a23dcb: push rdi
140a23dcc: push rbx
140a23dcd: sub rsp, 0x38
140a23dd1: lea rbp, [rdx + 0x80]
140a23dd8: lea rcx, [rbp + 0x150]
140a23ddf: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23de4: nop 
140a23de5: add rsp, 0x38
140a23de9: pop rbx
140a23dea: pop rdi
140a23deb: pop rsi
140a23dec: pop r14
140a23dee: pop r15
140a23df0: pop rbp
140a23df1: ret 
140a23df2: nop word ptr cs:[rax + rax]
140a23e00: mov qword ptr [rsp + 0x10], rdx
140a23e05: push rbp
140a23e06: push r15
140a23e08: push r14
140a23e0a: push rsi
140a23e0b: push rdi
140a23e0c: push rbx
140a23e0d: sub rsp, 0x38
140a23e11: lea rbp, [rdx + 0x80]
140a23e18: lea rcx, [rbp + 0x2e0]
140a23e1f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23e24: nop 
140a23e25: add rsp, 0x38
140a23e29: pop rbx
140a23e2a: pop rdi
140a23e2b: pop rsi
140a23e2c: pop r14
140a23e2e: pop r15
140a23e30: pop rbp
140a23e31: ret 
140a23e32: nop word ptr cs:[rax + rax]
140a23e40: mov qword ptr [rsp + 0x10], rdx
140a23e45: push rbp
140a23e46: push r15
140a23e48: push r14
140a23e4a: push rsi
140a23e4b: push rdi
140a23e4c: push rbx
140a23e4d: sub rsp, 0x38
140a23e51: lea rbp, [rdx + 0x80]
140a23e58: cmp byte ptr [rbp + 0x286], 8
140a23e5f: je 0x140a23e6d
140a23e61: lea rcx, [rbp + 0x270]
140a23e68: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a23e6d: nop 
140a23e6e: add rsp, 0x38
140a23e72: pop rbx
140a23e73: pop rdi
140a23e74: pop rsi
140a23e75: pop r14
140a23e77: pop r15
140a23e79: pop rbp
140a23e7a: ret 
140a23e7b: nop dword ptr [rax + rax]
140a23e80: mov qword ptr [rsp + 0x10], rdx
140a23e85: push rbp
140a23e86: push r15
140a23e88: push r14
140a23e8a: push rsi
140a23e8b: push rdi
140a23e8c: push rbx
140a23e8d: sub rsp, 0x38
140a23e91: lea rbp, [rdx + 0x80]
140a23e98: lea rcx, [rbp + 0x2e0]
140a23e9f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23ea4: nop 
140a23ea5: add rsp, 0x38
140a23ea9: pop rbx
140a23eaa: pop rdi
140a23eab: pop rsi
140a23eac: pop r14
140a23eae: pop r15
140a23eb0: pop rbp
140a23eb1: ret 
140a23eb2: nop word ptr cs:[rax + rax]
140a23ec0: mov qword ptr [rsp + 0x10], rdx
140a23ec5: push rbp
140a23ec6: push r15
140a23ec8: push r14
140a23eca: push rsi
140a23ecb: push rdi
140a23ecc: push rbx
140a23ecd: sub rsp, 0x38
140a23ed1: lea rbp, [rdx + 0x80]
140a23ed8: lea rcx, [rbp + 0x150]
140a23edf: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23ee4: nop 
140a23ee5: add rsp, 0x38
140a23ee9: pop rbx
140a23eea: pop rdi
140a23eeb: pop rsi
140a23eec: pop r14
140a23eee: pop r15
140a23ef0: pop rbp
140a23ef1: ret 
140a23ef2: nop word ptr cs:[rax + rax]
140a23f00: mov qword ptr [rsp + 0x10], rdx
140a23f05: push rbp
140a23f06: push r15
140a23f08: push r14
140a23f0a: push rsi
140a23f0b: push rdi
140a23f0c: push rbx
140a23f0d: sub rsp, 0x38
140a23f11: lea rbp, [rdx + 0x80]
140a23f18: lea rcx, [rbp + 0x2e0]
140a23f1f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23f24: nop 
140a23f25: add rsp, 0x38
140a23f29: pop rbx
140a23f2a: pop rdi
140a23f2b: pop rsi
140a23f2c: pop r14
140a23f2e: pop r15
140a23f30: pop rbp
140a23f31: ret 
140a23f32: nop word ptr cs:[rax + rax]
140a23f40: mov qword ptr [rsp + 0x10], rdx
140a23f45: push rbp
140a23f46: push r15
140a23f48: push r14
140a23f4a: push rsi
140a23f4b: push rdi
140a23f4c: push rbx
140a23f4d: sub rsp, 0x38
140a23f51: lea rbp, [rdx + 0x80]
140a23f58: lea rcx, [rbp + 0x2e0]
140a23f5f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23f64: nop 
140a23f65: add rsp, 0x38
140a23f69: pop rbx
140a23f6a: pop rdi
140a23f6b: pop rsi
140a23f6c: pop r14
140a23f6e: pop r15
140a23f70: pop rbp
140a23f71: ret 
140a23f72: nop word ptr cs:[rax + rax]
140a23f80: mov qword ptr [rsp + 0x10], rdx
140a23f85: push rbp
140a23f86: push r15
140a23f88: push r14
140a23f8a: push rsi
140a23f8b: push rdi
140a23f8c: push rbx
140a23f8d: sub rsp, 0x38
140a23f91: lea rbp, [rdx + 0x80]
140a23f98: lea rcx, [rbp + 0x150]
140a23f9f: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23fa4: nop 
140a23fa5: add rsp, 0x38
140a23fa9: pop rbx
140a23faa: pop rdi
140a23fab: pop rsi
140a23fac: pop r14
140a23fae: pop r15
140a23fb0: pop rbp
140a23fb1: ret 
140a23fb2: nop word ptr cs:[rax + rax]
140a23fc0: mov qword ptr [rsp + 0x10], rdx
140a23fc5: push rbp
140a23fc6: push r15
140a23fc8: push r14
140a23fca: push rsi
140a23fcb: push rdi
140a23fcc: push rbx
140a23fcd: sub rsp, 0x38
140a23fd1: lea rbp, [rdx + 0x80]
140a23fd8: lea rcx, [rbp + 0x2e0]
140a23fdf: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a23fe4: nop 
140a23fe5: add rsp, 0x38
140a23fe9: pop rbx
140a23fea: pop rdi
140a23feb: pop rsi
140a23fec: pop r14
140a23fee: pop r15
140a23ff0: pop rbp
140a23ff1: ret 
