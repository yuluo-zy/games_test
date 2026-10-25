140a32770: push rbp
140a32771: push r15
140a32773: push r14
140a32775: push r13
140a32777: push r12
140a32779: push rsi
140a3277a: push rdi
140a3277b: push rbx
140a3277c: sub rsp, 0x478
140a32783: lea rbp, [rsp + 0x80]
140a3278b: mov qword ptr [rbp + 0x3f0], 0xfffffffffffffffe
140a32796: mov rdi, rdx
140a32799: mov rbx, rcx
140a3279c: lea rsi, [rbp + 0x210]
140a327a3: mov rcx, rsi
140a327a6: mov dl, 0x15
140a327a8: call 0x140a59890 ; country_core::resources::texture_presets::impl$1::from
140a327ad: mov rcx, rsi
140a327b0: call 0x140bd5790 ; rhapsody::raw::texture::TextureDesc::extent_2d
140a327b5: mov rsi, rax
140a327b8: lea r14, [rdi + 0x38]
140a327bc: movabs rdx, 0x9b126e795fb22adf
140a327c6: movabs r8, 0xcb7c6a0645e229de
140a327d0: mov rcx, r14
140a327d3: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
140a327d8: test rax, rax
140a327db: je 0x140a334b4
140a327e1: cmp qword ptr [rdi + 0x218], rdx
140a327e8: jbe 0x140a334b4
140a327ee: mov rax, qword ptr [rdi + 0x210]
140a327f5: mov rcx, qword ptr [rax + rdx*8]
140a327f9: test rcx, rcx
140a327fc: je 0x140a334b4
140a32802: mov rax, qword ptr [rdi + 0x1e0]
140a32809: not rcx
140a3280c: lea rcx, [rcx + rcx*2]
140a32810: shl rcx, 5
140a32814: cmp qword ptr [rax + rcx + 0x20], 0
140a3281a: je 0x140a334b4
140a32820: add rax, rcx
140a32823: mov rax, qword ptr [rax + 0x10]
140a32827: mov rcx, qword ptr [rax]
140a3282a: mov rdx, qword ptr [rax + 8]
140a3282e: add rdx, rcx
140a32831: inc rdx
140a32834: movdqa xmm0, xmmword ptr [rcx]
140a32838: pmovmskb r8d, xmm0
140a3283d: not r8d
140a32840: mov word ptr [rbp + 0x228], r8w
140a32848: mov qword ptr [rbp + 0x210], rcx
140a3284f: add rcx, 0x10
140a32853: mov qword ptr [rbp + 0x218], rcx
140a3285a: mov qword ptr [rbp + 0x220], rdx
140a32861: mov rax, qword ptr [rax + 0x18]
140a32865: mov qword ptr [rbp + 0x230], rax
140a3286c: lea r8, [rip + 0x2099065]
140a32873: lea rcx, [rbp + 0x1f8]
140a3287a: lea rdx, [rbp + 0x210]
140a32881: call 0x1412fc510 ; alloc::vec::spec_from_iter::impl$0::from_iter<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo,core::iter::adapters::filter_map::FilterMap<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings::closure_env$0> >
140a32886: movabs rdx, 0x8226cc786e679355
140a32890: movabs r8, 0x65cfda6a55a841ac
140a3289a: mov rcx, r14
140a3289d: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
140a328a2: test rax, rax
140a328a5: je 0x140a334c0
140a328ab: cmp qword ptr [rdi + 0x218], rdx
140a328b2: jbe 0x140a334c0
140a328b8: mov rax, qword ptr [rdi + 0x210]
140a328bf: mov rcx, qword ptr [rax + rdx*8]
140a328c3: test rcx, rcx
140a328c6: je 0x140a334c0
140a328cc: mov rax, qword ptr [rdi + 0x1e0]
140a328d3: not rcx
140a328d6: lea rcx, [rcx + rcx*2]
140a328da: shl rcx, 5
140a328de: cmp qword ptr [rax + rcx + 0x20], 0
140a328e4: je 0x140a334c0
140a328ea: add rax, rcx
140a328ed: mov rcx, qword ptr [rax + 0x10]
140a328f1: lea rdx, [rip + 0x21fbb08]
140a328f8: call 0x140969720 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
140a328fd: mov r15, rax
140a32900: test rax, rax
140a32903: je 0x140a32a58
140a32909: lea rax, [rip + 0x206b5ed]
140a32910: mov qword ptr [rbp + 0x60], rax
140a32914: mov qword ptr [rbp + 0x68], 0x5e
140a3291c: mov r12, r15
140a3291f: add r12, 0x10
140a32923: cmp qword ptr [r15 + 0x18], 0x5e
140a32928: jne 0x140a33483
140a3292e: mov rdx, qword ptr [r12]
140a32932: lea rcx, [rip + 0x206b5c4]
140a32939: mov r8d, 0x5e
140a3293f: call 0x142923bf0 ; ?
140a32944: test eax, eax
140a32946: jne 0x140a33483
140a3294c: mov rcx, qword ptr [r15]
140a3294f: mov rax, qword ptr [r15 + 8]
140a32953: call qword ptr [rax + 0x18]
140a32956: mov r15, rax
140a32959: test rax, rax
140a3295c: je 0x140a32a58
140a32962: mov r12, qword ptr [rbp + 0x208]
140a32969: cmp r12, 0x1387
140a32970: ja 0x140a334e2
140a32976: movabs rdx, 0x2ed189581b92decf
140a32980: movabs r8, 0x15d3d99396065f38
140a3298a: mov rcx, r14
140a3298d: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
140a32992: test rax, rax
140a32995: je 0x140a334d1
140a3299b: cmp qword ptr [rdi + 0x218], rdx
140a329a2: jbe 0x140a334d1
140a329a8: mov rax, qword ptr [rdi + 0x210]
140a329af: mov rcx, qword ptr [rax + rdx*8]
140a329b3: test rcx, rcx
140a329b6: je 0x140a334d1
140a329bc: mov rax, qword ptr [rdi + 0x1e0]
140a329c3: not rcx
140a329c6: lea rdx, [rcx + rcx*2]
140a329ca: shl rdx, 5
140a329ce: cmp qword ptr [rax + rdx + 0x20], 0
140a329d4: je 0x140a334d1
140a329da: test r12, r12
140a329dd: je 0x140a32a55
140a329df: mov rcx, r12
140a329e2: shl rcx, 5
140a329e6: cmp rcx, qword ptr [r15 + 0x20]
140a329ea: ja 0x140a3352a
140a329f0: add rax, rdx
140a329f3: mov rax, qword ptr [rax + 0x10]
140a329f7: mov rdx, qword ptr [rax]
140a329fa: add rdx, 0x10
140a329fe: mov r9, qword ptr [rbp + 0x200]
140a32a05: mov r8, r15
140a32a08: add r8, 8
140a32a0c: mov qword ptr [rbp + 0x60], 0
140a32a14: mov qword ptr [rbp + 0x68], rcx
140a32a18: lea rax, [rbp + 0x60]
140a32a1c: mov qword ptr [rsp + 0x28], rax
140a32a21: mov qword ptr [rsp + 0x20], rcx
140a32a26: mov qword ptr [rsp + 0x38], 0
140a32a2f: mov qword ptr [rsp + 0x30], 1
140a32a38: lea rcx, [rbp + 0x210]
140a32a3f: call 0x140beefa0 ; rhapsody::raw::device::GraphicsDevice::update_buffer_bytes
140a32a44: mov rax, qword ptr [rbp + 0x210]
140a32a4b: cmp rax, 0xb
140a32a4f: jne 0x140a33531
140a32a55: mov qword ptr [r15], r12
140a32a58: mov qword ptr [rsp + 0x30], rbx
140a32a5d: mov qword ptr [rsp + 0x28], rdi
140a32a62: mov qword ptr [rsp + 0x20], 0x1c
140a32a6b: lea rdx, [rip + 0x209999e]
140a32a72: lea r9, [rip + 0x20999aa]
140a32a79: lea rcx, [rbp + 0x210]
140a32a80: mov r8d, 0x13
140a32a86: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a32a8b: lea rcx, [rbp + 0x60]
140a32a8f: lea rdx, [rbp + 0x210]
140a32a96: movabs r8, 0x4a99c8969e0b36cd
140a32aa0: call 0x14160cba0 ; country_core::render::pass_builder::compute_pass_builder::impl$3::read_storage_buffer<country_core::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>
140a32aa5: cmp byte ptr [rbp + 0x196], 8
140a32aac: je 0x140a32ad7
140a32aae: mov r9, qword ptr [rbp + 0x208]
140a32ab5: lea rcx, [rbp + 0x170]
140a32abc: mov edx, 0xe8
140a32ac1: add rdx, qword ptr [rbp + 0x198]
140a32ac8: movabs r8, 0xce4cfd9dd902baae
140a32ad2: call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140a32ad7: lea r14, [rbp + 0x210]
140a32ade: lea r15, [rbp + 0x60]
140a32ae2: mov r8d, 0x190
140a32ae8: mov rcx, r14
140a32aeb: mov rdx, r15
140a32aee: call 0x142923c00 ; ?
140a32af3: mov byte ptr [rbp + 0x3a0], 0x15
140a32afa: mov dword ptr [rbp + 0x60], 2
140a32b01: mov qword ptr [rsp + 0x20], r15
140a32b06: lea rcx, [rbp + 0x3a0]
140a32b0d: movabs r8, 0xbdf6ed93e1e613b8
140a32b17: mov rdx, r14
140a32b1a: mov r9b, 0x32
140a32b1d: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a32b22: lea r14, [rbp + 0x60]
140a32b26: lea r15, [rbp + 0x210]
140a32b2d: mov r8d, 0x190
140a32b33: mov rcx, r14
140a32b36: mov rdx, r15
140a32b39: call 0x142923c00 ; ?
140a32b3e: mov byte ptr [rbp + 0x3a0], 0x16
140a32b45: mov dword ptr [rbp + 0x210], 2
140a32b4f: mov qword ptr [rsp + 0x20], r15
140a32b54: lea rcx, [rbp + 0x3a0]
140a32b5b: movabs r8, 0xb0a93c48d9f3337e
140a32b65: mov rdx, r14
140a32b68: mov r9b, 0x32
140a32b6b: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a32b70: lea rcx, [rbp + 0x210]
140a32b77: lea rdx, [rbp + 0x60]
140a32b7b: mov r8d, 0x190
140a32b81: call 0x142923c00 ; ?
140a32b86: mov r12, rsi
140a32b89: shr r12, 0x20
140a32b8d: movups xmm0, xmmword ptr [rbp + 0x2b0]
140a32b94: movaps xmmword ptr [rbp + 0x3e0], xmm0
140a32b9b: movups xmm0, xmmword ptr [rbp + 0x2a0]
140a32ba2: movaps xmmword ptr [rbp + 0x3d0], xmm0
140a32ba9: movups xmm0, xmmword ptr [rbp + 0x290]
140a32bb0: movaps xmmword ptr [rbp + 0x3c0], xmm0
140a32bb7: movups xmm0, xmmword ptr [rbp + 0x280]
140a32bbe: movaps xmmword ptr [rbp + 0x3b0], xmm0
140a32bc5: movups xmm0, xmmword ptr [rbp + 0x270]
140a32bcc: movaps xmmword ptr [rbp + 0x3a0], xmm0
140a32bd3: mov rax, qword ptr [rbp + 0x348]
140a32bda: mov rcx, qword ptr [rbp + 0x390]
140a32be1: mov rdx, qword ptr [rbp + 0x220]
140a32be8: mov qword ptr [rbp + 0x70], rdx
140a32bec: movups xmm0, xmmword ptr [rbp + 0x210]
140a32bf3: movaps xmmword ptr [rbp + 0x60], xmm0
140a32bf7: movups xmm0, xmmword ptr [rbp + 0x228]
140a32bfe: movaps xmmword ptr [rbp + 0x90], xmm0
140a32c05: mov rdx, qword ptr [rbp + 0x238]
140a32c0c: mov qword ptr [rbp + 0xa0], rdx
140a32c13: movups xmm0, xmmword ptr [rbp + 0x240]
140a32c1a: movups xmmword ptr [rbp + 0x78], xmm0
140a32c1e: mov rdx, qword ptr [rbp + 0x250]
140a32c25: mov qword ptr [rbp + 0x88], rdx
140a32c2c: movups xmm0, xmmword ptr [rbp + 0x2c0]
140a32c33: movups xmm1, xmmword ptr [rbp + 0x2d0]
140a32c3a: movups xmm2, xmmword ptr [rbp + 0x2e0]
140a32c41: movups xmm3, xmmword ptr [rbp + 0x2f0]
140a32c48: movups xmmword ptr [rbp + 0xd8], xmm0
140a32c4f: movups xmmword ptr [rbp + 0xe8], xmm1
140a32c56: movups xmmword ptr [rbp + 0xf8], xmm2
140a32c5d: movups xmmword ptr [rbp + 0x108], xmm3
140a32c64: movups xmm0, xmmword ptr [rbp + 0x300]
140a32c6b: movups xmmword ptr [rbp + 0x118], xmm0
140a32c72: movups xmm0, xmmword ptr [rbp + 0x310]
140a32c79: movups xmmword ptr [rbp + 0x128], xmm0
140a32c80: mov rdx, qword ptr [rbp + 0x268]
140a32c87: mov qword ptr [rbp + 0xb8], rdx
140a32c8e: movdqu xmm0, xmmword ptr [rbp + 0x258]
140a32c96: movdqu xmmword ptr [rbp + 0xa8], xmm0
140a32c9e: mov qword ptr [rbp + 0x140], rcx
140a32ca5: mov qword ptr [rbp + 0x138], rax
140a32cac: mov dword ptr [rbp + 0xc0], 1
140a32cb6: mov dword ptr [rbp + 0xc4], esi
140a32cbc: mov dword ptr [rbp + 0xc8], r12d
140a32cc3: mov dword ptr [rbp + 0xcc], 1
140a32ccd: lea rcx, [rbp + 0x3a0]
140a32cd4: lea rdx, [rbp + 0x60]
140a32cd8: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a32cdd: cmp byte ptr [rbp + 0x346], 8
140a32ce4: je 0x140a32cf2
140a32ce6: lea rcx, [rbp + 0x330]
140a32ced: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a32cf2: lea rcx, [rbp + 0x60]
140a32cf6: mov dl, 0x15
140a32cf8: call 0x140a59890 ; country_core::resources::texture_presets::impl$1::from
140a32cfd: mov r8d, dword ptr [rbp + 0x68]
140a32d01: lea rcx, [rbp + 0x40]
140a32d05: movabs rdx, 0x20000000200
140a32d0f: call 0x140bd52a0 ; rhapsody::raw::texture::TextureDesc::new_2d
140a32d14: movdqu xmm0, xmmword ptr [rbp + 0x40]
140a32d19: movups xmm1, xmmword ptr [rbp + 0x50]
140a32d1d: movaps xmmword ptr [rbp + 0x3b0], xmm1
140a32d24: movdqa xmmword ptr [rbp + 0x3a0], xmm0
140a32d2c: lea rcx, [rbp + 0x220]
140a32d33: lea rdx, [rbp + 0x3a0]
140a32d3a: call 0x140bd6ce0 ; rhapsody::rg::resource::impl$4::from
140a32d3f: lea rax, [rip + 0x20996f9]
140a32d46: mov qword ptr [rbp + 0x210], rax
140a32d4d: mov qword ptr [rbp + 0x218], 0x13
140a32d58: lea rdx, [rbp + 0x210]
140a32d5f: mov rcx, rbx
140a32d62: call 0x140be94d0 ; rhapsody::rg::graph::RenderGraph::create_raw_resource
140a32d67: movdqu xmm0, xmmword ptr [rbp + 0x40]
140a32d6c: movups xmm1, xmmword ptr [rbp + 0x50]
140a32d70: movdqu xmmword ptr [rbp - 0x30], xmm0
140a32d75: movups xmmword ptr [rbp - 0x20], xmm1
140a32d79: mov dword ptr [rbp - 0x34], eax
140a32d7c: mov qword ptr [rsp + 0x30], rbx
140a32d81: mov qword ptr [rsp + 0x28], rdi
140a32d86: mov qword ptr [rsp + 0x20], 0x21
140a32d8f: lea rdx, [rip + 0x20996bc]
140a32d96: lea r9, [rip + 0x20996c8]
140a32d9d: lea rcx, [rbp + 0x210]
140a32da4: mov r8d, 0x13
140a32daa: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a32daf: cmp byte ptr [rbp + 0x346], 8
140a32db6: je 0x140a32ddd
140a32db8: lea rcx, [rbp + 0x320]
140a32dbf: mov edx, 0xe8
140a32dc4: add rdx, qword ptr [rbp + 0x348]
140a32dcb: movabs r8, 0xd91ff8372094bc22
140a32dd5: mov r9b, 1
140a32dd8: call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
140a32ddd: mov r8d, 0x190
140a32de3: mov rcx, r14
140a32de6: mov rdx, r15
140a32de9: call 0x142923c00 ; ?
140a32dee: mov byte ptr [rbp + 0x3a0], 0x15
140a32df5: mov dword ptr [rbp + 0x210], 2
140a32dff: mov qword ptr [rsp + 0x20], r15
140a32e04: lea rcx, [rbp + 0x3a0]
140a32e0b: movabs r8, 0x2de87fdb66740ece
140a32e15: mov rdx, r14
140a32e18: mov r9b, 0x19
140a32e1b: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a32e20: lea r14, [rbp + 0x210]
140a32e27: lea r15, [rbp + 0x60]
140a32e2b: mov r8d, 0x190
140a32e31: mov rcx, r14
140a32e34: mov rdx, r15
140a32e37: call 0x142923c00 ; ?
140a32e3c: lea r13, [rbp - 0x34]
140a32e40: mov qword ptr [rbp + 0x3a0], r13
140a32e47: mov dword ptr [rbp + 0x60], 2
140a32e4e: mov qword ptr [rsp + 0x20], r15
140a32e53: lea rcx, [rbp + 0x3a0]
140a32e5a: movabs r8, 0x960d27b7b6f81d59
140a32e64: mov rdx, r14
140a32e67: mov r9b, 0x32
140a32e6a: call 0x1408edd00 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref_mut$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
140a32e6f: lea rcx, [rbp + 0x60]
140a32e73: lea rdx, [rbp + 0x210]
140a32e7a: mov r8d, 0x190
140a32e80: call 0x142923c00 ; ?
140a32e85: movups xmm0, xmmword ptr [rbp + 0x100]
140a32e8c: movaps xmmword ptr [rbp + 0x3e0], xmm0
140a32e93: movups xmm0, xmmword ptr [rbp + 0xf0]
140a32e9a: movaps xmmword ptr [rbp + 0x3d0], xmm0
140a32ea1: movups xmm0, xmmword ptr [rbp + 0xe0]
140a32ea8: movaps xmmword ptr [rbp + 0x3c0], xmm0
140a32eaf: movups xmm0, xmmword ptr [rbp + 0xd0]
140a32eb6: movaps xmmword ptr [rbp + 0x3b0], xmm0
140a32ebd: movups xmm0, xmmword ptr [rbp + 0xc0]
140a32ec4: movaps xmmword ptr [rbp + 0x3a0], xmm0
140a32ecb: mov rax, qword ptr [rbp + 0x198]
140a32ed2: mov rcx, qword ptr [rbp + 0x1e0]
140a32ed9: mov rdx, qword ptr [rbp + 0x70]
140a32edd: mov qword ptr [rbp + 0x220], rdx
140a32ee4: movups xmm0, xmmword ptr [rbp + 0x60]
140a32ee8: movaps xmmword ptr [rbp + 0x210], xmm0
140a32eef: movups xmm0, xmmword ptr [rbp + 0x78]
140a32ef3: movaps xmmword ptr [rbp + 0x240], xmm0
140a32efa: mov rdx, qword ptr [rbp + 0x88]
140a32f01: mov qword ptr [rbp + 0x250], rdx
140a32f08: movups xmm0, xmmword ptr [rbp + 0x90]
140a32f0f: movups xmmword ptr [rbp + 0x228], xmm0
140a32f16: mov rdx, qword ptr [rbp + 0xa0]
140a32f1d: mov qword ptr [rbp + 0x238], rdx
140a32f24: movups xmm0, xmmword ptr [rbp + 0x110]
140a32f2b: movups xmm1, xmmword ptr [rbp + 0x120]
140a32f32: movups xmm2, xmmword ptr [rbp + 0x130]
140a32f39: movups xmm3, xmmword ptr [rbp + 0x140]
140a32f40: movups xmmword ptr [rbp + 0x288], xmm0
140a32f47: movups xmmword ptr [rbp + 0x298], xmm1
140a32f4e: movups xmmword ptr [rbp + 0x2a8], xmm2
140a32f55: movups xmmword ptr [rbp + 0x2b8], xmm3
140a32f5c: movups xmm0, xmmword ptr [rbp + 0x150]
140a32f63: movups xmmword ptr [rbp + 0x2c8], xmm0
140a32f6a: movups xmm0, xmmword ptr [rbp + 0x160]
140a32f71: movups xmmword ptr [rbp + 0x2d8], xmm0
140a32f78: mov rdx, qword ptr [rbp + 0xb8]
140a32f7f: mov qword ptr [rbp + 0x268], rdx
140a32f86: movdqu xmm0, xmmword ptr [rbp + 0xa8]
140a32f8e: movdqu xmmword ptr [rbp + 0x258], xmm0
140a32f96: mov qword ptr [rbp + 0x2f0], rcx
140a32f9d: mov qword ptr [rbp + 0x2e8], rax
140a32fa4: mov dword ptr [rbp + 0x270], 1
140a32fae: mov dword ptr [rbp + 0x274], esi
140a32fb4: mov dword ptr [rbp + 0x278], r12d
140a32fbb: mov dword ptr [rbp + 0x27c], 1
140a32fc5: lea rcx, [rbp + 0x3a0]
140a32fcc: lea rdx, [rbp + 0x210]
140a32fd3: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a32fd8: cmp byte ptr [rbp + 0x196], 8
140a32fdf: je 0x140a32fed
140a32fe1: lea rcx, [rbp + 0x180]
140a32fe8: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a32fed: mov qword ptr [rsp + 0x30], rbx
140a32ff2: mov qword ptr [rsp + 0x28], rdi
140a32ff7: mov qword ptr [rsp + 0x20], 0x21
140a33000: lea rdx, [rip + 0x209944b]
140a33007: lea r9, [rip + 0x2099457]
140a3300e: lea rcx, [rbp + 0x210]
140a33015: mov r8d, 0x13
140a3301b: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a33020: cmp byte ptr [rbp + 0x346], 8
140a33027: je 0x140a3304e
140a33029: lea rcx, [rbp + 0x320]
140a33030: mov edx, 0xe8
140a33035: add rdx, qword ptr [rbp + 0x348]
140a3303c: movabs r8, 0xd91ff8372094bc22
140a33046: xor r9d, r9d
140a33049: call 0x14096b850 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>
140a3304e: lea r14, [rbp + 0x60]
140a33052: lea r15, [rbp + 0x210]
140a33059: mov r8d, 0x190
140a3305f: mov rcx, r14
140a33062: mov rdx, r15
140a33065: call 0x142923c00 ; ?
140a3306a: mov qword ptr [rbp + 0x3a0], r13
140a33071: mov dword ptr [rbp + 0x210], 2
140a3307b: mov qword ptr [rsp + 0x20], r15
140a33080: lea rcx, [rbp + 0x3a0]
140a33087: movabs r8, 0x2de87fdb66740ece
140a33091: mov rdx, r14
140a33094: mov r9b, 0x19
140a33097: call 0x1408ee240 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<ref$<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture> > >
140a3309c: lea r14, [rbp + 0x210]
140a330a3: lea r15, [rbp + 0x60]
140a330a7: mov r8d, 0x190
140a330ad: mov rcx, r14
140a330b0: mov rdx, r15
140a330b3: call 0x142923c00 ; ?
140a330b8: mov byte ptr [rbp + 0x3a0], 0x15
140a330bf: mov dword ptr [rbp + 0x60], 2
140a330c6: mov qword ptr [rsp + 0x20], r15
140a330cb: lea rcx, [rbp + 0x3a0]
140a330d2: movabs r8, 0x960d27b7b6f81d59
140a330dc: mov rdx, r14
140a330df: mov r9b, 0x32
140a330e2: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a330e7: lea r14, [rbp + 0x60]
140a330eb: lea rdx, [rbp + 0x210]
140a330f2: mov r8d, 0x190
140a330f8: mov rcx, r14
140a330fb: call 0x142923c00 ; ?
140a33100: movups xmm0, xmmword ptr [rbp + 0x100]
140a33107: movaps xmmword ptr [rbp + 0x3e0], xmm0
140a3310e: movups xmm0, xmmword ptr [rbp + 0xf0]
140a33115: movaps xmmword ptr [rbp + 0x3d0], xmm0
140a3311c: movups xmm0, xmmword ptr [rbp + 0xe0]
140a33123: movaps xmmword ptr [rbp + 0x3c0], xmm0
140a3312a: movups xmm0, xmmword ptr [rbp + 0xd0]
140a33131: movaps xmmword ptr [rbp + 0x3b0], xmm0
140a33138: movups xmm0, xmmword ptr [rbp + 0xc0]
140a3313f: movaps xmmword ptr [rbp + 0x3a0], xmm0
140a33146: mov rax, qword ptr [rbp + 0x198]
140a3314d: mov rcx, qword ptr [rbp + 0x1e0]
140a33154: mov rdx, qword ptr [rbp + 0x70]
140a33158: mov qword ptr [rbp + 0x220], rdx
140a3315f: movups xmm0, xmmword ptr [rbp + 0x60]
140a33163: movaps xmmword ptr [rbp + 0x210], xmm0
140a3316a: movups xmm0, xmmword ptr [rbp + 0x78]
140a3316e: movaps xmmword ptr [rbp + 0x240], xmm0
140a33175: mov rdx, qword ptr [rbp + 0x88]
140a3317c: mov qword ptr [rbp + 0x250], rdx
140a33183: movups xmm0, xmmword ptr [rbp + 0x90]
140a3318a: movups xmmword ptr [rbp + 0x228], xmm0
140a33191: mov rdx, qword ptr [rbp + 0xa0]
140a33198: mov qword ptr [rbp + 0x238], rdx
140a3319f: movups xmm0, xmmword ptr [rbp + 0x110]
140a331a6: movups xmm1, xmmword ptr [rbp + 0x120]
140a331ad: movups xmm2, xmmword ptr [rbp + 0x130]
140a331b4: movups xmm3, xmmword ptr [rbp + 0x140]
140a331bb: movups xmmword ptr [rbp + 0x288], xmm0
140a331c2: movups xmmword ptr [rbp + 0x298], xmm1
140a331c9: movups xmmword ptr [rbp + 0x2a8], xmm2
140a331d0: movups xmmword ptr [rbp + 0x2b8], xmm3
140a331d7: movups xmm0, xmmword ptr [rbp + 0x150]
140a331de: movups xmmword ptr [rbp + 0x2c8], xmm0
140a331e5: movups xmm0, xmmword ptr [rbp + 0x160]
140a331ec: movups xmmword ptr [rbp + 0x2d8], xmm0
140a331f3: mov rdx, qword ptr [rbp + 0xb8]
140a331fa: mov qword ptr [rbp + 0x268], rdx
140a33201: movdqu xmm0, xmmword ptr [rbp + 0xa8]
140a33209: movdqu xmmword ptr [rbp + 0x258], xmm0
140a33211: mov qword ptr [rbp + 0x2f0], rcx
140a33218: mov qword ptr [rbp + 0x2e8], rax
140a3321f: mov dword ptr [rbp + 0x270], 1
140a33229: mov dword ptr [rbp + 0x274], esi
140a3322f: mov dword ptr [rbp + 0x278], r12d
140a33236: mov dword ptr [rbp + 0x27c], 1
140a33240: lea rcx, [rbp + 0x3a0]
140a33247: lea rdx, [rbp + 0x210]
140a3324e: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a33253: cmp byte ptr [rbp + 0x196], 8
140a3325a: je 0x140a33268
140a3325c: lea rcx, [rbp + 0x180]
140a33263: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a33268: mov qword ptr [rsp + 0x30], rbx
140a3326d: mov qword ptr [rsp + 0x28], rdi
140a33272: mov qword ptr [rsp + 0x20], 0x24
140a3327b: lea rdx, [rip + 0x2099204]
140a33282: lea r9, [rip + 0x2099213]
140a33289: lea rcx, [rbp + 0x210]
140a33290: mov r8d, 0x16
140a33296: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140a3329b: mov byte ptr [rbp + 0x3a0], 0x15
140a332a2: mov dword ptr [rbp + 0x60], 2
140a332a9: mov qword ptr [rsp + 0x20], r14
140a332ae: lea rcx, [rbp + 0x3a0]
140a332b5: lea rdx, [rbp + 0x210]
140a332bc: movabs r8, 0xbdf6ed93e1e613b8
140a332c6: mov r9b, 0x32
140a332c9: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140a332ce: lea rcx, [rbp + 0x60]
140a332d2: lea rdx, [rbp + 0x210]
140a332d9: mov r8d, 0x190
140a332df: call 0x142923c00 ; ?
140a332e4: movups xmm0, xmmword ptr [rbp + 0x100]
140a332eb: movaps xmmword ptr [rbp + 0x3e0], xmm0
140a332f2: movups xmm0, xmmword ptr [rbp + 0xf0]
140a332f9: movaps xmmword ptr [rbp + 0x3d0], xmm0
140a33300: movups xmm0, xmmword ptr [rbp + 0xe0]
140a33307: movaps xmmword ptr [rbp + 0x3c0], xmm0
140a3330e: movups xmm0, xmmword ptr [rbp + 0xd0]
140a33315: movaps xmmword ptr [rbp + 0x3b0], xmm0
140a3331c: movups xmm0, xmmword ptr [rbp + 0xc0]
140a33323: movaps xmmword ptr [rbp + 0x3a0], xmm0
140a3332a: mov rax, qword ptr [rbp + 0x198]
140a33331: mov rcx, qword ptr [rbp + 0x1e0]
140a33338: mov rdx, qword ptr [rbp + 0x70]
140a3333c: mov qword ptr [rbp + 0x220], rdx
140a33343: movups xmm0, xmmword ptr [rbp + 0x60]
140a33347: movaps xmmword ptr [rbp + 0x210], xmm0
140a3334e: movups xmm0, xmmword ptr [rbp + 0x78]
140a33352: movaps xmmword ptr [rbp + 0x240], xmm0
140a33359: mov rdx, qword ptr [rbp + 0x88]
140a33360: mov qword ptr [rbp + 0x250], rdx
140a33367: movups xmm0, xmmword ptr [rbp + 0x90]
140a3336e: movups xmmword ptr [rbp + 0x228], xmm0
140a33375: mov rdx, qword ptr [rbp + 0xa0]
140a3337c: mov qword ptr [rbp + 0x238], rdx
140a33383: movups xmm0, xmmword ptr [rbp + 0x110]
140a3338a: movups xmm1, xmmword ptr [rbp + 0x120]
140a33391: movups xmm2, xmmword ptr [rbp + 0x130]
140a33398: movups xmm3, xmmword ptr [rbp + 0x140]
140a3339f: movups xmmword ptr [rbp + 0x288], xmm0
140a333a6: movups xmmword ptr [rbp + 0x298], xmm1
140a333ad: movups xmmword ptr [rbp + 0x2a8], xmm2
140a333b4: movups xmmword ptr [rbp + 0x2b8], xmm3
140a333bb: movups xmm0, xmmword ptr [rbp + 0x150]
140a333c2: movups xmmword ptr [rbp + 0x2c8], xmm0
140a333c9: movups xmm0, xmmword ptr [rbp + 0x160]
140a333d0: movups xmmword ptr [rbp + 0x2d8], xmm0
140a333d7: mov rdx, qword ptr [rbp + 0xb8]
140a333de: mov qword ptr [rbp + 0x268], rdx
140a333e5: movdqu xmm0, xmmword ptr [rbp + 0xa8]
140a333ed: movdqu xmmword ptr [rbp + 0x258], xmm0
140a333f5: mov qword ptr [rbp + 0x2f0], rcx
140a333fc: mov qword ptr [rbp + 0x2e8], rax
140a33403: mov dword ptr [rbp + 0x270], 1
140a3340d: mov dword ptr [rbp + 0x274], esi
140a33413: mov dword ptr [rbp + 0x278], r12d
140a3341a: mov dword ptr [rbp + 0x27c], 1
140a33424: lea rcx, [rbp + 0x3a0]
140a3342b: lea rdx, [rbp + 0x210]
140a33432: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140a33437: cmp byte ptr [rbp + 0x196], 8
140a3343e: je 0x140a3344c
140a33440: lea rcx, [rbp + 0x180]
140a33447: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a3344c: mov rdx, qword ptr [rbp + 0x1f8]
140a33453: test rdx, rdx
140a33456: je 0x140a3346e
140a33458: mov rcx, qword ptr [rbp + 0x200]
140a3345f: shl rdx, 5
140a33463: mov r8d, 4
140a33469: call 0x140613c20 ; ?
140a3346e: nop 
140a3346f: add rsp, 0x478
140a33476: pop rbx
140a33477: pop rdi
140a33478: pop rsi
140a33479: pop r12
140a3347b: pop r13
140a3347d: pop r14
140a3347f: pop r15
140a33481: pop rbp
140a33482: ret 
140a33483: mov qword ptr [rbp + 0x210], 0
140a3348e: lea rax, [rip + 0x21faceb]
140a33495: mov qword ptr [rsp + 0x20], rax
140a3349a: lea rdx, [rbp + 0x60]
140a3349e: lea r9, [rbp + 0x210]
140a334a5: xor ecx, ecx
140a334a7: mov r8, r12
140a334aa: call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
140a334af: jmp 0x140a335d5
140a334b4: lea rcx, [rip + 0x2098e9d]
140a334bb: call 0x1428d9310 ; core::option::unwrap_failed
140a334c0: lea rcx, [rip + 0x2098ea9]
140a334c7: call 0x1428d9310 ; core::option::unwrap_failed
140a334cc: jmp 0x140a335d5
140a334d1: lea rcx, [rip + 0x2098f08]
140a334d8: call 0x1428d9310 ; core::option::unwrap_failed
140a334dd: jmp 0x140a335d5
140a334e2: lea rax, [rip + 0x2098ecf]
140a334e9: mov qword ptr [rbp + 0x210], rax
140a334f0: mov qword ptr [rbp + 0x218], 1
140a334fb: mov qword ptr [rbp + 0x220], 8
140a33506: pxor xmm0, xmm0
140a3350a: movdqu xmmword ptr [rbp + 0x228], xmm0
140a33512: lea rdx, [rip + 0x2098eaf]
140a33519: lea rcx, [rbp + 0x210]
140a33520: call 0x1428d9390 ; core::panicking::panic_fmt
140a33525: jmp 0x140a335d5
140a3352a: mov eax, 0xb
140a3352f: jmp 0x140a3356a
140a33531: mov rcx, qword ptr [rbp + 0x258]
140a33538: mov qword ptr [rbp + 0x30], rcx
140a3353c: movdqu xmm0, xmmword ptr [rbp + 0x218]
140a33544: movups xmm1, xmmword ptr [rbp + 0x228]
140a3354b: movups xmm2, xmmword ptr [rbp + 0x238]
140a33552: movups xmm3, xmmword ptr [rbp + 0x248]
140a33559: movaps xmmword ptr [rbp + 0x20], xmm3
140a3355d: movaps xmmword ptr [rbp + 0x10], xmm2
140a33561: movaps xmmword ptr [rbp], xmm1
140a33565: movdqa xmmword ptr [rbp - 0x10], xmm0
140a3356a: mov qword ptr [rbp + 0x210], rax
140a33571: movdqa xmm0, xmmword ptr [rbp - 0x10]
140a33576: movaps xmm1, xmmword ptr [rbp]
140a3357a: movaps xmm2, xmmword ptr [rbp + 0x10]
140a3357e: movaps xmm3, xmmword ptr [rbp + 0x20]
140a33582: movdqu xmmword ptr [rbp + 0x218], xmm0
140a3358a: movups xmmword ptr [rbp + 0x228], xmm1
140a33591: movups xmmword ptr [rbp + 0x238], xmm2
140a33598: movups xmmword ptr [rbp + 0x248], xmm3
140a3359f: mov rax, qword ptr [rbp + 0x30]
140a335a3: mov qword ptr [rbp + 0x258], rax
140a335aa: lea rax, [rip + 0x2098e47]
140a335b1: mov qword ptr [rsp + 0x20], rax
140a335b6: lea rcx, [rip + 0x209837b]
140a335bd: lea r9, [rip + 0x2098354]
140a335c4: lea r8, [rbp + 0x210]
140a335cb: mov edx, 0x2b
140a335d0: call 0x1428d9760 ; core::result::unwrap_failed
140a335d5: ud2 
140a335d7: nop word ptr [rax + rax]
140a335e0: mov qword ptr [rsp + 0x10], rdx
140a335e5: push rbp
140a335e6: push r15
140a335e8: push r14
140a335ea: push r13
140a335ec: push r12
140a335ee: push rsi
140a335ef: push rdi
140a335f0: push rbx
140a335f1: sub rsp, 0x48
140a335f5: lea rbp, [rdx + 0x80]
140a335fc: mov rdx, qword ptr [rbp + 0x1f8]
140a33603: test rdx, rdx
140a33606: je 0x140a3361e
140a33608: mov rcx, qword ptr [rbp + 0x200]
140a3360f: shl rdx, 5
140a33613: mov r8d, 4
140a33619: call 0x140613c20 ; ?
140a3361e: nop 
140a3361f: add rsp, 0x48
140a33623: pop rbx
140a33624: pop rdi
140a33625: pop rsi
140a33626: pop r12
140a33628: pop r13
140a3362a: pop r14
140a3362c: pop r15
140a3362e: pop rbp
140a3362f: ret 
140a33630: mov qword ptr [rsp + 0x10], rdx
140a33635: push rbp
140a33636: push r15
140a33638: push r14
140a3363a: push r13
140a3363c: push r12
140a3363e: push rsi
140a3363f: push rdi
140a33640: push rbx
140a33641: sub rsp, 0x48
140a33645: lea rbp, [rdx + 0x80]
140a3364c: cmp byte ptr [rbp + 0x346], 8
140a33653: je 0x140a33661
140a33655: lea rcx, [rbp + 0x330]
140a3365c: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a33661: nop 
140a33662: add rsp, 0x48
140a33666: pop rbx
140a33667: pop rdi
140a33668: pop rsi
140a33669: pop r12
140a3366b: pop r13
140a3366d: pop r14
140a3366f: pop r15
140a33671: pop rbp
140a33672: ret 
140a33673: nop word ptr cs:[rax + rax]
140a33680: mov qword ptr [rsp + 0x10], rdx
140a33685: push rbp
140a33686: push r15
140a33688: push r14
140a3368a: push r13
140a3368c: push r12
140a3368e: push rsi
140a3368f: push rdi
140a33690: push rbx
140a33691: sub rsp, 0x48
140a33695: lea rbp, [rdx + 0x80]
140a3369c: lea rcx, [rbp + 0x210]
140a336a3: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a336a8: nop 
140a336a9: add rsp, 0x48
140a336ad: pop rbx
140a336ae: pop rdi
140a336af: pop rsi
140a336b0: pop r12
140a336b2: pop r13
140a336b4: pop r14
140a336b6: pop r15
140a336b8: pop rbp
140a336b9: ret 
140a336ba: nop word ptr [rax + rax]
140a336c0: mov qword ptr [rsp + 0x10], rdx
140a336c5: push rbp
140a336c6: push r15
140a336c8: push r14
140a336ca: push r13
140a336cc: push r12
140a336ce: push rsi
140a336cf: push rdi
140a336d0: push rbx
140a336d1: sub rsp, 0x48
140a336d5: lea rbp, [rdx + 0x80]
140a336dc: lea rcx, [rbp + 0x60]
140a336e0: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a336e5: nop 
140a336e6: add rsp, 0x48
140a336ea: pop rbx
140a336eb: pop rdi
140a336ec: pop rsi
140a336ed: pop r12
140a336ef: pop r13
140a336f1: pop r14
140a336f3: pop r15
140a336f5: pop rbp
140a336f6: ret 
140a336f7: nop word ptr [rax + rax]
140a33700: mov qword ptr [rsp + 0x10], rdx
140a33705: push rbp
140a33706: push r15
140a33708: push r14
140a3370a: push r13
140a3370c: push r12
140a3370e: push rsi
140a3370f: push rdi
140a33710: push rbx
140a33711: sub rsp, 0x48
140a33715: lea rbp, [rdx + 0x80]
140a3371c: lea rcx, [rbp + 0x210]
140a33723: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a33728: nop 
140a33729: add rsp, 0x48
140a3372d: pop rbx
140a3372e: pop rdi
140a3372f: pop rsi
140a33730: pop r12
140a33732: pop r13
140a33734: pop r14
140a33736: pop r15
140a33738: pop rbp
140a33739: ret 
140a3373a: nop word ptr [rax + rax]
140a33740: mov qword ptr [rsp + 0x10], rdx
140a33745: push rbp
140a33746: push r15
140a33748: push r14
140a3374a: push r13
140a3374c: push r12
140a3374e: push rsi
140a3374f: push rdi
140a33750: push rbx
140a33751: sub rsp, 0x48
140a33755: lea rbp, [rdx + 0x80]
140a3375c: cmp byte ptr [rbp + 0x196], 8
140a33763: je 0x140a33771
140a33765: lea rcx, [rbp + 0x180]
140a3376c: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a33771: nop 
140a33772: add rsp, 0x48
140a33776: pop rbx
140a33777: pop rdi
140a33778: pop rsi
140a33779: pop r12
140a3377b: pop r13
140a3377d: pop r14
140a3377f: pop r15
140a33781: pop rbp
140a33782: ret 
140a33783: nop word ptr cs:[rax + rax]
140a33790: mov qword ptr [rsp + 0x10], rdx
140a33795: push rbp
140a33796: push r15
140a33798: push r14
140a3379a: push r13
140a3379c: push r12
140a3379e: push rsi
140a3379f: push rdi
140a337a0: push rbx
140a337a1: sub rsp, 0x48
140a337a5: lea rbp, [rdx + 0x80]
140a337ac: lea rcx, [rbp + 0x210]
140a337b3: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a337b8: nop 
140a337b9: add rsp, 0x48
140a337bd: pop rbx
140a337be: pop rdi
140a337bf: pop rsi
140a337c0: pop r12
140a337c2: pop r13
140a337c4: pop r14
140a337c6: pop r15
140a337c8: pop rbp
140a337c9: ret 
140a337ca: nop word ptr [rax + rax]
140a337d0: mov qword ptr [rsp + 0x10], rdx
140a337d5: push rbp
140a337d6: push r15
140a337d8: push r14
140a337da: push r13
140a337dc: push r12
140a337de: push rsi
140a337df: push rdi
140a337e0: push rbx
140a337e1: sub rsp, 0x48
140a337e5: lea rbp, [rdx + 0x80]
140a337ec: lea rcx, [rbp + 0x60]
140a337f0: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a337f5: nop 
140a337f6: add rsp, 0x48
140a337fa: pop rbx
140a337fb: pop rdi
140a337fc: pop rsi
140a337fd: pop r12
140a337ff: pop r13
140a33801: pop r14
140a33803: pop r15
140a33805: pop rbp
140a33806: ret 
140a33807: nop word ptr [rax + rax]
140a33810: mov qword ptr [rsp + 0x10], rdx
140a33815: push rbp
140a33816: push r15
140a33818: push r14
140a3381a: push r13
140a3381c: push r12
140a3381e: push rsi
140a3381f: push rdi
140a33820: push rbx
140a33821: sub rsp, 0x48
140a33825: lea rbp, [rdx + 0x80]
140a3382c: lea rcx, [rbp + 0x210]
140a33833: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a33838: nop 
140a33839: add rsp, 0x48
140a3383d: pop rbx
140a3383e: pop rdi
140a3383f: pop rsi
140a33840: pop r12
140a33842: pop r13
140a33844: pop r14
140a33846: pop r15
140a33848: pop rbp
140a33849: ret 
140a3384a: nop word ptr [rax + rax]
140a33850: mov qword ptr [rsp + 0x10], rdx
140a33855: push rbp
140a33856: push r15
140a33858: push r14
140a3385a: push r13
140a3385c: push r12
140a3385e: push rsi
140a3385f: push rdi
140a33860: push rbx
140a33861: sub rsp, 0x48
140a33865: lea rbp, [rdx + 0x80]
140a3386c: cmp byte ptr [rbp + 0x196], 8
140a33873: je 0x140a33881
140a33875: lea rcx, [rbp + 0x180]
140a3387c: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a33881: nop 
140a33882: add rsp, 0x48
140a33886: pop rbx
140a33887: pop rdi
140a33888: pop rsi
140a33889: pop r12
140a3388b: pop r13
140a3388d: pop r14
140a3388f: pop r15
140a33891: pop rbp
140a33892: ret 
140a33893: nop word ptr cs:[rax + rax]
140a338a0: mov qword ptr [rsp + 0x10], rdx
140a338a5: push rbp
140a338a6: push r15
140a338a8: push r14
140a338aa: push r13
140a338ac: push r12
140a338ae: push rsi
140a338af: push rdi
140a338b0: push rbx
140a338b1: sub rsp, 0x48
140a338b5: lea rbp, [rdx + 0x80]
140a338bc: lea rcx, [rbp + 0x210]
140a338c3: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a338c8: nop 
140a338c9: add rsp, 0x48
140a338cd: pop rbx
140a338ce: pop rdi
140a338cf: pop rsi
140a338d0: pop r12
140a338d2: pop r13
140a338d4: pop r14
140a338d6: pop r15
140a338d8: pop rbp
140a338d9: ret 
140a338da: nop word ptr [rax + rax]
140a338e0: mov qword ptr [rsp + 0x10], rdx
140a338e5: push rbp
140a338e6: push r15
140a338e8: push r14
140a338ea: push r13
140a338ec: push r12
140a338ee: push rsi
140a338ef: push rdi
140a338f0: push rbx
140a338f1: sub rsp, 0x48
140a338f5: lea rbp, [rdx + 0x80]
140a338fc: cmp byte ptr [rbp + 0x196], 8
140a33903: je 0x140a33911
140a33905: lea rcx, [rbp + 0x180]
140a3390c: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140a33911: nop 
140a33912: add rsp, 0x48
140a33916: pop rbx
140a33917: pop rdi
140a33918: pop rsi
140a33919: pop r12
140a3391b: pop r13
140a3391d: pop r14
140a3391f: pop r15
140a33921: pop rbp
140a33922: ret 
140a33923: nop word ptr cs:[rax + rax]
140a33930: mov qword ptr [rsp + 0x10], rdx
140a33935: push rbp
140a33936: push r15
140a33938: push r14
140a3393a: push r13
140a3393c: push r12
140a3393e: push rsi
140a3393f: push rdi
140a33940: push rbx
140a33941: sub rsp, 0x48
140a33945: lea rbp, [rdx + 0x80]
140a3394c: lea rcx, [rbp + 0x210]
140a33953: call 0x1407b1810 ; core::ptr::drop_in_place<enum2$<country_core::render::ssbo::BufferWriteError> >
140a33958: nop 
140a33959: add rsp, 0x48
140a3395d: pop rbx
140a3395e: pop rdi
140a3395f: pop rsi
140a33960: pop r12
140a33962: pop r13
140a33964: pop r14
140a33966: pop r15
140a33968: pop rbp
140a33969: ret 
140a3396a: nop word ptr [rax + rax]
140a33970: mov qword ptr [rsp + 0x10], rdx
140a33975: push rbp
140a33976: push r15
140a33978: push r14
140a3397a: push r13
140a3397c: push r12
140a3397e: push rsi
140a3397f: push rdi
140a33980: push rbx
140a33981: sub rsp, 0x48
140a33985: lea rbp, [rdx + 0x80]
140a3398c: lea rcx, [rbp + 0x60]
140a33990: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a33995: nop 
140a33996: add rsp, 0x48
140a3399a: pop rbx
140a3399b: pop rdi
140a3399c: pop rsi
140a3399d: pop r12
140a3399f: pop r13
140a339a1: pop r14
140a339a3: pop r15
140a339a5: pop rbp
140a339a6: ret 
140a339a7: nop word ptr [rax + rax]
140a339b0: mov qword ptr [rsp + 0x10], rdx
140a339b5: push rbp
140a339b6: push r15
140a339b8: push r14
140a339ba: push r13
140a339bc: push r12
140a339be: push rsi
140a339bf: push rdi
140a339c0: push rbx
140a339c1: sub rsp, 0x48
140a339c5: lea rbp, [rdx + 0x80]
140a339cc: lea rcx, [rbp + 0x210]
140a339d3: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a339d8: nop 
140a339d9: add rsp, 0x48
140a339dd: pop rbx
140a339de: pop rdi
140a339df: pop rsi
140a339e0: pop r12
140a339e2: pop r13
140a339e4: pop r14
140a339e6: pop r15
140a339e8: pop rbp
140a339e9: ret 
140a339ea: nop word ptr [rax + rax]
140a339f0: mov qword ptr [rsp + 0x10], rdx
140a339f5: push rbp
140a339f6: push r15
140a339f8: push r14
140a339fa: push r13
140a339fc: push r12
140a339fe: push rsi
140a339ff: push rdi
140a33a00: push rbx
140a33a01: sub rsp, 0x48
140a33a05: lea rbp, [rdx + 0x80]
140a33a0c: lea rcx, [rbp + 0x60]
140a33a10: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140a33a15: nop 
140a33a16: add rsp, 0x48
140a33a1a: pop rbx
140a33a1b: pop rdi
140a33a1c: pop rsi
140a33a1d: pop r12
140a33a1f: pop r13
140a33a21: pop r14
140a33a23: pop r15
140a33a25: pop rbp
140a33a26: ret 
