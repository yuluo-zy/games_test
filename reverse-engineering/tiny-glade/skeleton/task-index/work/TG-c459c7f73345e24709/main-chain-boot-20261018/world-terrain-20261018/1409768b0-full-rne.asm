1409768b0: push rbp
1409768b1: push rsi
1409768b2: push rdi
1409768b3: sub rsp, 0x3e0
1409768ba: lea rbp, [rsp + 0x80]
1409768c2: movaps xmmword ptr [rbp + 0x350], xmm7
1409768c9: movaps xmmword ptr [rbp + 0x340], xmm6
1409768d0: mov qword ptr [rbp + 0x338], 0xfffffffffffffffe
1409768db: mov rsi, rdx
1409768de: mov rdi, rcx
1409768e1: lea rcx, [rdx + 0x38]
1409768e5: movabs rdx, 0x62dfe8f62b6fcc5
1409768ef: movabs r8, 0xf65008e548632ba7
1409768f9: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1409768fe: test rax, rax
140976901: je 0x140976bab
140976907: cmp qword ptr [rsi + 0x218], rdx
14097690e: jbe 0x140976bab
140976914: mov rax, qword ptr [rsi + 0x210]
14097691b: mov rcx, qword ptr [rax + rdx*8]
14097691f: test rcx, rcx
140976922: je 0x140976bab
140976928: mov rax, qword ptr [rsi + 0x1e0]
14097692f: not rcx
140976932: lea rcx, [rcx + rcx*2]
140976936: shl rcx, 5
14097693a: cmp qword ptr [rax + rcx + 0x20], 0
140976940: je 0x140976bab
140976946: add rax, rcx
140976949: mov rax, qword ptr [rax + 0x10]
14097694d: xorps xmm6, xmm6
140976950: test byte ptr [rax], 1
140976953: xorps xmm7, xmm7
140976956: je 0x140976962
140976958: movss xmm6, dword ptr [rax + 4]
14097695d: movss xmm7, dword ptr [rax + 0xc]
140976962: mov qword ptr [rsp + 0x30], rdi
140976967: mov qword ptr [rsp + 0x28], rsi
14097696c: mov qword ptr [rsp + 0x20], 0x1c
140976975: lea rdx, [rip + 0x213ff64]
14097697c: lea r9, [rip + 0x213ff70]
140976983: lea rsi, [rbp - 0x40]
140976987: mov r8d, 0x13
14097698d: mov rcx, rsi
140976990: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140976995: lea rdi, [rbp + 0x150]
14097699c: movabs r8, 0xa868b69644bd36ba
1409769a6: mov rcx, rdi
1409769a9: mov rdx, rsi
1409769ac: call 0x1416093d0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_to_cpu_mapped_storage_buffer<country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo>
1409769b1: mov byte ptr [rbp + 0x2e0], 0xb
1409769b8: mov dword ptr [rbp - 0x40], 2
1409769bf: mov qword ptr [rsp + 0x20], rsi
1409769c4: lea rcx, [rbp + 0x2e0]
1409769cb: movabs r8, 0xe3659c0fbde7bf03
1409769d5: mov rdx, rdi
1409769d8: mov r9b, 0x19
1409769db: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1409769e0: lea rcx, [rbp - 0x40]
1409769e4: lea rdx, [rbp + 0x150]
1409769eb: mov r8d, 0x190
1409769f1: call 0x142923c00 ; ?
1409769f6: movzx eax, byte ptr [rip + 0x2cbe20c]
1409769fd: mov ecx, 8
140976a02: mov edx, 1
140976a07: call 0x140613c10 ; ?
140976a0c: test rax, rax
140976a0f: je 0x140976bb7
140976a15: mov rsi, rax
140976a18: movss dword ptr [rax], xmm6
140976a1c: movss dword ptr [rax + 4], xmm7
140976a21: mov rdx, qword ptr [rbp + 8]
140976a25: test rdx, rdx
140976a28: je 0x140976a39
140976a2a: mov rcx, qword ptr [rbp + 0x10]
140976a2e: mov r8d, 1
140976a34: call 0x140613c20 ; ?
140976a39: mov qword ptr [rbp + 8], 8
140976a41: mov qword ptr [rbp + 0x10], rsi
140976a45: mov qword ptr [rbp + 0x18], 8
140976a4d: lea rcx, [rbp + 0x150]
140976a54: lea rdx, [rbp - 0x40]
140976a58: mov r8d, 0x190
140976a5e: call 0x142923c00 ; ?
140976a63: movups xmm0, xmmword ptr [rbp + 0x1f0]
140976a6a: movaps xmmword ptr [rbp + 0x320], xmm0
140976a71: movups xmm0, xmmword ptr [rbp + 0x1e0]
140976a78: movaps xmmword ptr [rbp + 0x310], xmm0
140976a7f: movups xmm0, xmmword ptr [rbp + 0x1d0]
140976a86: movaps xmmword ptr [rbp + 0x300], xmm0
140976a8d: movups xmm0, xmmword ptr [rbp + 0x1c0]
140976a94: movaps xmmword ptr [rbp + 0x2f0], xmm0
140976a9b: movups xmm0, xmmword ptr [rbp + 0x1b0]
140976aa2: movaps xmmword ptr [rbp + 0x2e0], xmm0
140976aa9: mov rax, qword ptr [rbp + 0x288]
140976ab0: mov rcx, qword ptr [rbp + 0x2d0]
140976ab7: mov rdx, qword ptr [rbp + 0x160]
140976abe: mov qword ptr [rbp - 0x30], rdx
140976ac2: movups xmm0, xmmword ptr [rbp + 0x150]
140976ac9: movaps xmmword ptr [rbp - 0x40], xmm0
140976acd: movups xmm0, xmmword ptr [rbp + 0x168]
140976ad4: movaps xmmword ptr [rbp - 0x10], xmm0
140976ad8: mov rdx, qword ptr [rbp + 0x178]
140976adf: mov qword ptr [rbp], rdx
140976ae3: movups xmm0, xmmword ptr [rbp + 0x180]
140976aea: movups xmmword ptr [rbp - 0x28], xmm0
140976aee: mov rdx, qword ptr [rbp + 0x190]
140976af5: mov qword ptr [rbp - 0x18], rdx
140976af9: movups xmm0, xmmword ptr [rbp + 0x200]
140976b00: movups xmm1, xmmword ptr [rbp + 0x210]
140976b07: movups xmm2, xmmword ptr [rbp + 0x220]
140976b0e: movups xmm3, xmmword ptr [rbp + 0x230]
140976b15: movups xmmword ptr [rbp + 0x38], xmm0
140976b19: movups xmmword ptr [rbp + 0x48], xmm1
140976b1d: movups xmmword ptr [rbp + 0x58], xmm2
140976b21: movups xmmword ptr [rbp + 0x68], xmm3
140976b25: movups xmm0, xmmword ptr [rbp + 0x240]
140976b2c: movups xmmword ptr [rbp + 0x78], xmm0
140976b30: movups xmm0, xmmword ptr [rbp + 0x250]
140976b37: movups xmmword ptr [rbp + 0x88], xmm0
140976b3e: mov rdx, qword ptr [rbp + 0x1a8]
140976b45: mov qword ptr [rbp + 0x18], rdx
140976b49: movups xmm0, xmmword ptr [rbp + 0x198]
140976b50: movups xmmword ptr [rbp + 8], xmm0
140976b54: mov qword ptr [rbp + 0xa0], rcx
140976b5b: mov qword ptr [rbp + 0x98], rax
140976b62: movaps xmm0, xmmword ptr [rip + 0x1fe2477]
140976b69: movaps xmmword ptr [rbp + 0x20], xmm0
140976b6d: lea rcx, [rbp + 0x2e0]
140976b74: lea rdx, [rbp - 0x40]
140976b78: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
140976b7d: cmp byte ptr [rbp + 0x286], 8
140976b84: je 0x140976b92
140976b86: lea rcx, [rbp + 0x270]
140976b8d: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140976b92: movaps xmm6, xmmword ptr [rbp + 0x340]
140976b99: movaps xmm7, xmmword ptr [rbp + 0x350]
140976ba0: add rsp, 0x3e0
140976ba7: pop rdi
140976ba8: pop rsi
140976ba9: pop rbp
140976baa: ret 
140976bab: lea rcx, [rip + 0x213fd16]
140976bb2: call 0x1428d9310 ; core::option::unwrap_failed
140976bb7: lea r8, [rip + 0x2306d12]
140976bbe: mov ecx, 1
140976bc3: mov edx, 8
140976bc8: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140976bcd: ud2 
140976bcf: nop 
140976bd0: mov qword ptr [rsp + 0x10], rdx
140976bd5: push rbp
140976bd6: push rsi
140976bd7: push rdi
140976bd8: sub rsp, 0x60
140976bdc: lea rbp, [rdx + 0x80]
140976be3: movaps xmmword ptr [rsp + 0x30], xmm7
140976be8: movaps xmmword ptr [rsp + 0x40], xmm6
140976bed: cmp byte ptr [rbp + 0x286], 8
140976bf4: je 0x140976c02
140976bf6: lea rcx, [rbp + 0x270]
140976bfd: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140976c02: movaps xmm6, xmmword ptr [rsp + 0x40]
140976c07: movaps xmm7, xmmword ptr [rsp + 0x30]
140976c0c: add rsp, 0x60
140976c10: pop rdi
140976c11: pop rsi
140976c12: pop rbp
140976c13: ret 
140976c14: nop word ptr cs:[rax + rax]
140976c20: mov qword ptr [rsp + 0x10], rdx
140976c25: push rbp
140976c26: push rsi
140976c27: push rdi
140976c28: sub rsp, 0x60
140976c2c: lea rbp, [rdx + 0x80]
140976c33: movaps xmmword ptr [rsp + 0x30], xmm7
140976c38: movaps xmmword ptr [rsp + 0x40], xmm6
140976c3d: lea rcx, [rbp + 0x150]
140976c44: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140976c49: movaps xmm6, xmmword ptr [rsp + 0x40]
140976c4e: movaps xmm7, xmmword ptr [rsp + 0x30]
140976c53: add rsp, 0x60
140976c57: pop rdi
140976c58: pop rsi
140976c59: pop rbp
140976c5a: ret 
140976c5b: nop dword ptr [rax + rax]
140976c60: mov qword ptr [rsp + 0x10], rdx
140976c65: push rbp
140976c66: push rsi
140976c67: push rdi
140976c68: sub rsp, 0x60
140976c6c: lea rbp, [rdx + 0x80]
140976c73: movaps xmmword ptr [rsp + 0x30], xmm7
140976c78: movaps xmmword ptr [rsp + 0x40], xmm6
140976c7d: lea rcx, [rbp - 0x40]
140976c81: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
140976c86: movaps xmm6, xmmword ptr [rsp + 0x40]
140976c8b: movaps xmm7, xmmword ptr [rsp + 0x30]
140976c90: add rsp, 0x60
140976c94: pop rdi
140976c95: pop rsi
140976c96: pop rbp
140976c97: ret 
