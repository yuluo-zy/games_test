14098c680: push rbp
14098c681: push r15
14098c683: push r14
14098c685: push r13
14098c687: push r12
14098c689: push rsi
14098c68a: push rdi
14098c68b: push rbx
14098c68c: sub rsp, 0xcb8
14098c693: lea rbp, [rsp + 0x80]
14098c69b: movaps xmmword ptr [rbp + 0xc20], xmm7
14098c6a2: movaps xmmword ptr [rbp + 0xc10], xmm6
14098c6a9: mov qword ptr [rbp + 0xc08], 0xfffffffffffffffe
14098c6b4: mov rsi, rdx
14098c6b7: mov rdi, rcx
14098c6ba: lea rcx, [rdx + 0x38]
14098c6be: movabs rdx, 0xa68561b396784eaa
14098c6c8: movabs r8, 0xf5fd74e6faf4ed14
14098c6d2: mov qword ptr [rbp + 0xbf8], rcx
14098c6d9: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14098c6de: test rax, rax
14098c6e1: je 0x14098d8d1
14098c6e7: cmp qword ptr [rsi + 0x218], rdx
14098c6ee: jbe 0x14098d8d1
14098c6f4: mov rax, qword ptr [rsi + 0x210]
14098c6fb: mov rcx, qword ptr [rax + rdx*8]
14098c6ff: test rcx, rcx
14098c702: je 0x14098d8d1
14098c708: mov rax, qword ptr [rsi + 0x1e0]
14098c70f: not rcx
14098c712: lea rcx, [rcx + rcx*2]
14098c716: shl rcx, 5
14098c71a: cmp qword ptr [rax + rcx + 0x20], 0
14098c720: je 0x14098d8d1
14098c726: add rax, rcx
14098c729: mov rdx, qword ptr [rax + 0x10]
14098c72d: lea rbx, [rbp + 0x60c]
14098c734: mov rcx, rbx
14098c737: mov r8b, 0xa
14098c73a: call 0x14093cf30 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_texture
14098c73f: lea rcx, [rbp + 0x610]
14098c746: call 0x140bd5790 ; rhapsody::raw::texture::TextureDesc::extent_2d
14098c74b: mov qword ptr [rbp + 0x630], 0
14098c756: mov qword ptr [rbp + 0x638], rbx
14098c75d: mov qword ptr [rbp + 0x640], 1
14098c768: mov qword ptr [rbp + 0xbc0], rdi
14098c76f: mov qword ptr [rsp + 0x40], rdi
14098c774: mov qword ptr [rsp + 0x38], rsi
14098c779: mov qword ptr [rsp + 0x30], rax
14098c77e: lea rax, [rbp + 0x630]
14098c785: mov qword ptr [rsp + 0x28], rax
14098c78a: mov qword ptr [rsp + 0x20], 0
14098c793: lea rdx, [rip + 0x212c9fe]
14098c79a: lea rcx, [rbp + 0x220]
14098c7a1: lea r9, [rbp + 0xa20]
14098c7a8: mov r8d, 0x15
14098c7ae: call 0x140867970 ; country_core::render::pass_builder::draw_stream::RasterPass::new_impl
14098c7b3: mov byte ptr [rbp + 0xc07], 1
14098c7ba: movabs rdx, 0xd3bda3878f7a9d01
14098c7c4: movabs r8, 0x5dda49fe35d11774
14098c7ce: mov rcx, qword ptr [rbp + 0xbf8]
14098c7d5: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14098c7da: test rax, rax
14098c7dd: je 0x14098d8dd
14098c7e3: cmp qword ptr [rsi + 0x218], rdx
14098c7ea: jbe 0x14098d8dd
14098c7f0: mov rax, qword ptr [rsi + 0x210]
14098c7f7: mov rcx, qword ptr [rax + rdx*8]
14098c7fb: test rcx, rcx
14098c7fe: je 0x14098d8dd
14098c804: mov rax, qword ptr [rsi + 0x1e0]
14098c80b: not rcx
14098c80e: lea rcx, [rcx + rcx*2]
14098c812: shl rcx, 5
14098c816: cmp qword ptr [rax + rcx + 0x20], 0
14098c81c: je 0x14098d8dd
14098c822: add rax, rcx
14098c825: mov rax, qword ptr [rax + 0x10]
14098c829: mov qword ptr [rbp + 0xbb8], rax
14098c830: mov r8, qword ptr [rax]
14098c833: mov eax, r8d
14098c836: cmp rax, qword ptr [rsi + 0x10]
14098c83a: jae 0x14098d901
14098c840: mov rcx, qword ptr [rsi + 8]
14098c844: mov rdx, r8
14098c847: shr rdx, 0x20
14098c84b: lea rax, [rax + rax*4]
14098c84f: cmp dword ptr [rcx + rax*4], edx
14098c852: jne 0x14098d901
14098c858: lea rax, [rcx + rax*4]
14098c85c: cmp dword ptr [rax + 4], -1
14098c860: je 0x14098d901
14098c866: mov qword ptr [rbp + 0xbf0], r8
14098c86d: mov r14d, dword ptr [rax + 0xc]
14098c871: mov r12d, dword ptr [rax + 0x10]
14098c875: mov byte ptr [rbp + 0xc07], 1
14098c87c: movabs rdx, 0x93c2ea0b5dbce370
14098c886: movabs r8, 0xc3521cd69f6dc3c8
14098c890: mov rcx, qword ptr [rbp + 0xbf8]
14098c897: call 0x1409871a0 ; bevy_ecs::component::Components::get_id
14098c89c: test rax, rax
14098c89f: je 0x14098d93f
14098c8a5: cmp qword ptr [rsi + 0x1b0], r14
14098c8ac: jbe 0x14098d93f
14098c8b2: lea rbx, [r14 + r14*8]
14098c8b6: shl rbx, 3
14098c8ba: mov rcx, qword ptr [rsi + 0x1a8]
14098c8c1: add rcx, rbx
14098c8c4: mov byte ptr [rbp + 0xc07], 1
14098c8cb: mov r8d, r12d
14098c8ce: call 0x140d00b50 ; bevy_ecs::storage::table::Table::get_component
14098c8d3: mov qword ptr [rbp + 0xbe8], rax
14098c8da: cmp qword ptr [rbp + 0xbe8], 0
14098c8e2: je 0x14098d93f
14098c8e8: mov byte ptr [rbp + 0xc07], 1
14098c8ef: movabs rdx, 0x6deafeaf70315369
14098c8f9: movabs r8, 0x7ccce21f8e4423ee
14098c903: mov rcx, qword ptr [rbp + 0xbf8]
14098c90a: call 0x1409871a0 ; bevy_ecs::component::Components::get_id
14098c90f: test rax, rax
14098c912: je 0x14098d957
14098c918: cmp qword ptr [rsi + 0x1b0], r14
14098c91f: jbe 0x14098d957
14098c925: mov rcx, qword ptr [rsi + 0x1a8]
14098c92c: add rcx, rbx
14098c92f: mov byte ptr [rbp + 0xc07], 1
14098c936: mov r8d, r12d
14098c939: call 0x140d00b50 ; bevy_ecs::storage::table::Table::get_component
14098c93e: mov r15, rax
14098c941: test rax, rax
14098c944: je 0x14098d957
14098c94a: mov byte ptr [rbp + 0xc07], 1
14098c951: movabs rdx, 0x8672044888361604
14098c95b: movabs r8, 0x9340ff6b57fed7e4
14098c965: mov rcx, qword ptr [rbp + 0xbf8]
14098c96c: call 0x1409871a0 ; bevy_ecs::component::Components::get_id
14098c971: test rax, rax
14098c974: je 0x14098d96f
14098c97a: cmp qword ptr [rsi + 0x1b0], r14
14098c981: jbe 0x14098d96f
14098c987: mov rcx, qword ptr [rsi + 0x1a8]
14098c98e: add rcx, rbx
14098c991: mov byte ptr [rbp + 0xc07], 1
14098c998: mov r8d, r12d
14098c99b: call 0x140d00b50 ; bevy_ecs::storage::table::Table::get_component
14098c9a0: mov r13, rax
14098c9a3: test rax, rax
14098c9a6: je 0x14098d96f
14098c9ac: mov byte ptr [rbp + 0xc07], 1
14098c9b3: movabs rdx, 0xb04edb2d6216372f
14098c9bd: movabs r8, 0x541721773055ec85
14098c9c7: mov rcx, qword ptr [rbp + 0xbf8]
14098c9ce: call 0x1409871a0 ; bevy_ecs::component::Components::get_id
14098c9d3: mov edi, 2
14098c9d8: test rax, rax
14098c9db: je 0x14098ca0d
14098c9dd: cmp qword ptr [rsi + 0x1b0], r14
14098c9e4: jbe 0x14098ca0d
14098c9e6: add rbx, qword ptr [rsi + 0x1a8]
14098c9ed: mov byte ptr [rbp + 0xc07], 1
14098c9f4: mov rcx, rbx
14098c9f7: mov r8d, r12d
14098c9fa: call 0x140d00b50 ; bevy_ecs::storage::table::Table::get_component
14098c9ff: test rax, rax
14098ca02: je 0x14098ca0d
14098ca04: mov edi, dword ptr [rax]
14098ca06: movss xmm0, dword ptr [rax + 4]
14098ca0b: jmp 0x14098ca0d
14098ca0d: mov rax, qword ptr [rbp + 0xbf0]
14098ca14: mov qword ptr [rbp + 0xbe0], rax
14098ca1b: mov dword ptr [rbp + 0xbd8], edi
14098ca21: movss dword ptr [rbp + 0xbdc], xmm0
14098ca29: mov rdi, qword ptr [r15]
14098ca2c: mov rax, qword ptr [rbp + 0xbe8]
14098ca33: mov r14, qword ptr [rax]
14098ca36: mov byte ptr [rbp + 0xc07], 1
14098ca3d: call 0x140be6750 ; rhapsody::raw::shader::impl$7::default
14098ca42: mov ebx, eax
14098ca44: mov byte ptr [rbp + 0xc07], 1
14098ca4b: xor ecx, ecx
14098ca4d: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
14098ca52: test rax, rax
14098ca55: je 0x14098da01
14098ca5b: mov dword ptr [rbp + 0xac8], ebx
14098ca61: mov word ptr [rbp + 0xad0], 0
14098ca6a: mov dword ptr [rbp + 0xacc], 0x3f800000
14098ca74: mov qword ptr [rbp + 0xa20], 1
14098ca7f: mov qword ptr [rbp + 0xa28], r14
14098ca86: mov qword ptr [rbp + 0xac0], rdi
14098ca8d: mov qword ptr [rbp + 0xa30], 0
14098ca98: mov qword ptr [rbp + 0xa38], 4
14098caa3: mov qword ptr [rbp + 0xa40], 0
14098caae: movups xmm0, xmmword ptr [rax]
14098cab1: inc qword ptr [rax]
14098cab4: movups xmm6, xmmword ptr [rip + 0x2109c75]
14098cabb: movups xmmword ptr [rbp + 0xaa0], xmm6
14098cac2: movups xmm7, xmmword ptr [rip + 0x2109c57]
14098cac9: movups xmmword ptr [rbp + 0xa90], xmm7
14098cad0: movups xmmword ptr [rbp + 0xab0], xmm0
14098cad7: movabs rax, 0x100000003
14098cae1: mov qword ptr [rbp + 0xa60], rax
14098cae8: mov qword ptr [rbp + 0xa48], 0
14098caf3: mov qword ptr [rbp + 0xa50], 1
14098cafe: mov qword ptr [rbp + 0xa58], 0
14098cb09: mov byte ptr [rbp + 0xc07], 1
14098cb10: lea rcx, [rbp + 0x630]
14098cb17: lea rdx, [rbp + 0xa20]
14098cb1e: lea r8, [rbp + 0x220]
14098cb25: mov r9b, 1
14098cb28: call 0x140905be0 ; country_core::render::pass_builder::draw_builder::DrawBuilder::new
14098cb2d: mov rax, qword ptr [rbp + 0xbe8]
14098cb34: mov rax, qword ptr [rax]
14098cb37: mov rcx, qword ptr [r15]
14098cb3a: movaps xmm0, xmmword ptr [r13]
14098cb3f: movaps xmm1, xmmword ptr [r13 + 0x10]
14098cb44: movaps xmm2, xmmword ptr [r13 + 0x20]
14098cb49: movups xmm3, xmmword ptr [rbp + 0xbd8]
14098cb50: movaps xmmword ptr [rbp + 0xa20], xmm3
14098cb57: mov qword ptr [rbp + 0xa70], rax
14098cb5e: mov qword ptr [rbp + 0xa78], rcx
14098cb65: mov byte ptr [rbp + 0xa84], 0
14098cb6c: lea rax, [rip + 0x212c6b5]
14098cb73: mov qword ptr [rbp + 0xa30], rax
14098cb7a: mov qword ptr [rbp + 0xa38], 0xe
14098cb85: mov dword ptr [rbp + 0xa80], 0
14098cb8f: movaps xmmword ptr [rbp + 0xa40], xmm0
14098cb96: movaps xmmword ptr [rbp + 0xa50], xmm1
14098cb9d: movaps xmmword ptr [rbp + 0xa60], xmm2
14098cba4: mov byte ptr [rbp + 0xc07], 1
14098cbab: lea r8, [rip + 0x212c686]
14098cbb2: lea rcx, [rbp + 0x128]
14098cbb9: lea rdx, [rbp + 0x630]
14098cbc0: lea r9, [rbp + 0xa20]
14098cbc7: call 0x14090b460 ; country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common
14098cbcc: mov byte ptr [rbp + 0xc07], 1
14098cbd3: lea rcx, [rbp + 0x128]
14098cbda: lea rdx, [rbp + 0xbd8]
14098cbe1: mov r8, rsi
14098cbe4: mov r9b, 1
14098cbe7: call 0x1409cacb0 ; country_core::systems::render_loop::draw_passes::draw_shaders::floor_shader::draw_floor_shader
14098cbec: mov rax, qword ptr [rbp + 0xbb8]
14098cbf3: mov r8, qword ptr [rax + 8]
14098cbf7: mov eax, r8d
14098cbfa: cmp rax, qword ptr [rsi + 0x10]
14098cbfe: jae 0x14098d987
14098cc04: mov rcx, qword ptr [rsi + 8]
14098cc08: mov rdx, r8
14098cc0b: shr rdx, 0x20
14098cc0f: lea rax, [rax + rax*4]
14098cc13: cmp dword ptr [rcx + rax*4], edx
14098cc16: jne 0x14098d987
14098cc1c: lea rax, [rcx + rax*4]
14098cc20: cmp dword ptr [rax + 4], -1
14098cc24: je 0x14098d987
14098cc2a: mov qword ptr [rbp + 0xbf0], r8
14098cc31: mov r14d, dword ptr [rax + 0xc]
14098cc35: mov r12d, dword ptr [rax + 0x10]
14098cc39: mov byte ptr [rbp + 0xc07], 1
14098cc40: movabs rdx, 0x93c2ea0b5dbce370
14098cc4a: movabs r8, 0xc3521cd69f6dc3c8
14098cc54: mov rcx, qword ptr [rbp + 0xbf8]
14098cc5b: call 0x1409871a0 ; bevy_ecs::component::Components::get_id
14098cc60: test rax, rax
14098cc63: je 0x14098d9c2
14098cc69: cmp qword ptr [rsi + 0x1b0], r14
14098cc70: jbe 0x14098d9c2
14098cc76: lea rbx, [r14 + r14*8]
14098cc7a: shl rbx, 3
14098cc7e: mov rcx, qword ptr [rsi + 0x1a8]
14098cc85: add rcx, rbx
14098cc88: mov byte ptr [rbp + 0xc07], 1
14098cc8f: mov r8d, r12d
14098cc92: call 0x140d00b50 ; bevy_ecs::storage::table::Table::get_component
14098cc97: mov qword ptr [rbp + 0xbe8], rax
14098cc9e: cmp qword ptr [rbp + 0xbe8], 0
14098cca6: je 0x14098d9c2
14098ccac: mov byte ptr [rbp + 0xc07], 1
14098ccb3: movabs rdx, 0x6deafeaf70315369
14098ccbd: movabs r8, 0x7ccce21f8e4423ee
14098ccc7: mov rcx, qword ptr [rbp + 0xbf8]
14098ccce: call 0x1409871a0 ; bevy_ecs::component::Components::get_id
14098ccd3: test rax, rax
14098ccd6: je 0x14098d9d7
14098ccdc: cmp qword ptr [rsi + 0x1b0], r14
14098cce3: jbe 0x14098d9d7
14098cce9: mov rcx, qword ptr [rsi + 0x1a8]
14098ccf0: add rcx, rbx
14098ccf3: mov byte ptr [rbp + 0xc07], 1
14098ccfa: mov r8d, r12d
14098ccfd: call 0x140d00b50 ; bevy_ecs::storage::table::Table::get_component
14098cd02: mov r15, rax
14098cd05: test rax, rax
14098cd08: je 0x14098d9d7
14098cd0e: mov byte ptr [rbp + 0xc07], 1
14098cd15: movabs rdx, 0x8672044888361604
14098cd1f: movabs r8, 0x9340ff6b57fed7e4
14098cd29: mov rcx, qword ptr [rbp + 0xbf8]
14098cd30: call 0x1409871a0 ; bevy_ecs::component::Components::get_id
14098cd35: test rax, rax
14098cd38: je 0x14098d9ec
14098cd3e: cmp qword ptr [rsi + 0x1b0], r14
14098cd45: jbe 0x14098d9ec
14098cd4b: mov rcx, qword ptr [rsi + 0x1a8]
14098cd52: add rcx, rbx
14098cd55: mov byte ptr [rbp + 0xc07], 1
14098cd5c: mov r8d, r12d
14098cd5f: call 0x140d00b50 ; bevy_ecs::storage::table::Table::get_component
14098cd64: mov r13, rax
14098cd67: test rax, rax
14098cd6a: je 0x14098d9ec
14098cd70: mov byte ptr [rbp + 0xc07], 1
14098cd77: movabs rdx, 0xb04edb2d6216372f
14098cd81: movabs r8, 0x541721773055ec85
14098cd8b: mov rcx, qword ptr [rbp + 0xbf8]
14098cd92: call 0x1409871a0 ; bevy_ecs::component::Components::get_id
14098cd97: mov edi, 2
14098cd9c: test rax, rax
14098cd9f: je 0x14098cdd1
14098cda1: cmp qword ptr [rsi + 0x1b0], r14
14098cda8: jbe 0x14098cdd1
14098cdaa: add rbx, qword ptr [rsi + 0x1a8]
14098cdb1: mov byte ptr [rbp + 0xc07], 1
14098cdb8: mov rcx, rbx
14098cdbb: mov r8d, r12d
14098cdbe: call 0x140d00b50 ; bevy_ecs::storage::table::Table::get_component
14098cdc3: test rax, rax
14098cdc6: je 0x14098cdd1
14098cdc8: mov edi, dword ptr [rax]
14098cdca: movss xmm0, dword ptr [rax + 4]
14098cdcf: jmp 0x14098cdd1
14098cdd1: mov rax, qword ptr [rbp + 0xbf0]
14098cdd8: mov qword ptr [rbp + 0xbd0], rax
14098cddf: mov dword ptr [rbp + 0xbc8], edi
14098cde5: movss dword ptr [rbp + 0xbcc], xmm0
14098cded: mov rdi, qword ptr [r15]
14098cdf0: mov rax, qword ptr [rbp + 0xbe8]
14098cdf7: mov r14, qword ptr [rax]
14098cdfa: mov byte ptr [rbp + 0xc07], 1
14098ce01: call 0x140be6750 ; rhapsody::raw::shader::impl$7::default
14098ce06: mov ebx, eax
14098ce08: mov byte ptr [rbp + 0xc07], 1
14098ce0f: xor ecx, ecx
14098ce11: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
14098ce16: test rax, rax
14098ce19: je 0x14098da16
14098ce1f: mov dword ptr [rbp + 0xac8], ebx
14098ce25: mov word ptr [rbp + 0xad0], 0
14098ce2e: mov dword ptr [rbp + 0xacc], 0x3f800000
14098ce38: mov qword ptr [rbp + 0xa20], 1
14098ce43: mov qword ptr [rbp + 0xa28], r14
14098ce4a: mov qword ptr [rbp + 0xac0], rdi
14098ce51: mov qword ptr [rbp + 0xa30], 0
14098ce5c: mov qword ptr [rbp + 0xa38], 4
14098ce67: mov qword ptr [rbp + 0xa40], 0
14098ce72: movups xmm0, xmmword ptr [rax]
14098ce75: inc qword ptr [rax]
14098ce78: movups xmmword ptr [rbp + 0xaa0], xmm6
14098ce7f: movups xmmword ptr [rbp + 0xa90], xmm7
14098ce86: movups xmmword ptr [rbp + 0xab0], xmm0
14098ce8d: movabs rax, 0x100000003
14098ce97: mov qword ptr [rbp + 0xa60], rax
14098ce9e: mov qword ptr [rbp + 0xa48], 0
14098cea9: mov qword ptr [rbp + 0xa50], 1
14098ceb4: mov qword ptr [rbp + 0xa58], 0
14098cebf: mov byte ptr [rbp + 0xc07], 1
14098cec6: lea rcx, [rbp + 0x630]
14098cecd: lea rdx, [rbp + 0xa20]
14098ced4: lea r8, [rbp + 0x220]
14098cedb: mov r9b, 1
14098cede: call 0x140905be0 ; country_core::render::pass_builder::draw_builder::DrawBuilder::new
14098cee3: mov rax, qword ptr [rbp + 0xbe8]
14098ceea: mov rax, qword ptr [rax]
14098ceed: mov rcx, qword ptr [r15]
14098cef0: movaps xmm0, xmmword ptr [r13]
14098cef5: movaps xmm1, xmmword ptr [r13 + 0x10]
14098cefa: movaps xmm2, xmmword ptr [r13 + 0x20]
14098ceff: movups xmm3, xmmword ptr [rbp + 0xbc8]
14098cf06: movaps xmmword ptr [rbp + 0xa20], xmm3
14098cf0d: mov qword ptr [rbp + 0xa70], rax
14098cf14: mov qword ptr [rbp + 0xa78], rcx
14098cf1b: mov byte ptr [rbp + 0xa84], 0
14098cf22: lea rax, [rip + 0x212c377]
14098cf29: mov qword ptr [rbp + 0xa30], rax
14098cf30: mov qword ptr [rbp + 0xa38], 0xd
14098cf3b: mov dword ptr [rbp + 0xa80], 0
14098cf45: movaps xmmword ptr [rbp + 0xa40], xmm0
14098cf4c: movaps xmmword ptr [rbp + 0xa50], xmm1
14098cf53: movaps xmmword ptr [rbp + 0xa60], xmm2
14098cf5a: mov byte ptr [rbp + 0xc07], 1
14098cf61: lea r8, [rip + 0x212c2d0]
14098cf68: lea rcx, [rbp + 0x30]
14098cf6c: lea rdx, [rbp + 0x630]
14098cf73: lea r9, [rbp + 0xa20]
14098cf7a: call 0x14090b460 ; country_core::render::pass_builder::draw_builder::DrawBuilder::bind_common
14098cf7f: mov byte ptr [rbp + 0xc07], 1
14098cf86: lea rcx, [rbp + 0x30]
14098cf8a: lea rdx, [rbp + 0xbc8]
14098cf91: mov r8, rsi
14098cf94: mov r9b, 1
14098cf97: call 0x1409cb4e0 ; country_core::systems::render_loop::draw_passes::draw_shaders::rocky_terrain_draw::rocky_terrain_shader
14098cf9c: lea rbx, [rbp + 0x630]
14098cfa3: lea rdx, [rbp + 0x220]
14098cfaa: mov r8d, 0x3e8
14098cfb0: mov rcx, rbx
14098cfb3: call 0x142923c00 ; ?
14098cfb8: mov byte ptr [rbp + 0xc07], 0
14098cfbf: mov rcx, rbx
14098cfc2: call 0x140868370 ; country_core::render::pass_builder::draw_stream::RasterPass::render
14098cfc7: mov rdi, qword ptr [rbp + 0xbc0]
14098cfce: mov qword ptr [rsp + 0x30], rdi
14098cfd3: mov qword ptr [rsp + 0x28], rsi
14098cfd8: mov qword ptr [rsp + 0x20], 0x2c
14098cfe1: lea rdx, [rip + 0x212c2e0]
14098cfe8: lea r9, [rip + 0x212c2ec]
14098cfef: lea r14, [rbp + 0x220]
14098cff6: mov r8d, 0x13
14098cffc: mov rcx, r14
14098cfff: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
14098d004: mov byte ptr [rbp + 0xa20], 9
14098d00b: mov dword ptr [rbp + 0x630], 2
14098d015: mov qword ptr [rsp + 0x20], rbx
14098d01a: lea rbx, [rbp + 0xa20]
14098d021: movabs r8, 0x61b08b5e0470c5f2
14098d02b: mov rcx, rbx
14098d02e: mov rdx, r14
14098d031: mov r9b, 0x19
14098d034: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
14098d039: lea r14, [rbp + 0x630]
14098d040: lea rdx, [rbp + 0x220]
14098d047: mov r8d, 0x190
14098d04d: mov rcx, r14
14098d050: call 0x142923c00 ; ?
14098d055: mov byte ptr [rbp + 0x60c], 0xa
14098d05c: mov dword ptr [rbp + 0xa20], 2
14098d066: mov qword ptr [rsp + 0x20], rbx
14098d06b: lea rcx, [rbp + 0x60c]
14098d072: movabs r8, 0x297d1e4cdae8f657
14098d07c: mov rdx, r14
14098d07f: mov r9b, 0x19
14098d082: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
14098d087: lea rdx, [rbp + 0x630]
14098d08e: mov r8d, 0x190
14098d094: mov rcx, rbx
14098d097: call 0x142923c00 ; ?
14098d09c: movabs r8, 0xb25729c74f87a459
14098d0a6: mov rcx, r14
14098d0a9: mov rdx, rbx
14098d0ac: call 0x141604690 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::resources::terrain_height::TerrainHeightSsbo>
14098d0b1: lea rbx, [rbp + 0x220]
14098d0b8: movabs r8, 0x7703bd797f8b276e
14098d0c2: mov rcx, rbx
14098d0c5: mov rdx, r14
14098d0c8: call 0x141607950 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::resources::terrain_height::VisualTerrainHeightSsbo>
14098d0cd: movups xmm0, xmmword ptr [rbp + 0x2c0]
14098d0d4: movaps xmmword ptr [rbp + 0xa60], xmm0
14098d0db: movups xmm0, xmmword ptr [rbp + 0x2b0]
14098d0e2: movaps xmmword ptr [rbp + 0xa50], xmm0
14098d0e9: movups xmm0, xmmword ptr [rbp + 0x2a0]
14098d0f0: movaps xmmword ptr [rbp + 0xa40], xmm0
14098d0f7: movups xmm0, xmmword ptr [rbp + 0x290]
14098d0fe: movaps xmmword ptr [rbp + 0xa30], xmm0
14098d105: movups xmm0, xmmword ptr [rbp + 0x280]
14098d10c: movaps xmmword ptr [rbp + 0xa20], xmm0
14098d113: mov rax, qword ptr [rbp + 0x358]
14098d11a: mov rcx, qword ptr [rbp + 0x3a0]
14098d121: mov rdx, qword ptr [rbp + 0x230]
14098d128: mov qword ptr [rbp + 0x640], rdx
14098d12f: movups xmm0, xmmword ptr [rbp + 0x220]
14098d136: movaps xmmword ptr [rbp + 0x630], xmm0
14098d13d: movups xmm0, xmmword ptr [rbp + 0x238]
14098d144: movaps xmmword ptr [rbp + 0x660], xmm0
14098d14b: mov rdx, qword ptr [rbp + 0x248]
14098d152: mov qword ptr [rbp + 0x670], rdx
14098d159: movups xmm0, xmmword ptr [rbp + 0x250]
14098d160: movups xmmword ptr [rbp + 0x648], xmm0
14098d167: mov rdx, qword ptr [rbp + 0x260]
14098d16e: mov qword ptr [rbp + 0x658], rdx
14098d175: movups xmm0, xmmword ptr [rbp + 0x2d0]
14098d17c: movups xmm1, xmmword ptr [rbp + 0x2e0]
14098d183: movups xmm2, xmmword ptr [rbp + 0x2f0]
14098d18a: movups xmm3, xmmword ptr [rbp + 0x300]
14098d191: movups xmmword ptr [rbp + 0x6a8], xmm0
14098d198: movups xmmword ptr [rbp + 0x6b8], xmm1
14098d19f: movups xmmword ptr [rbp + 0x6c8], xmm2
14098d1a6: movups xmmword ptr [rbp + 0x6d8], xmm3
14098d1ad: movups xmm0, xmmword ptr [rbp + 0x310]
14098d1b4: movups xmmword ptr [rbp + 0x6e8], xmm0
14098d1bb: movups xmm0, xmmword ptr [rbp + 0x320]
14098d1c2: movups xmmword ptr [rbp + 0x6f8], xmm0
14098d1c9: mov rdx, qword ptr [rbp + 0x278]
14098d1d0: mov qword ptr [rbp + 0x688], rdx
14098d1d7: movups xmm0, xmmword ptr [rbp + 0x268]
14098d1de: movups xmmword ptr [rbp + 0x678], xmm0
14098d1e5: mov qword ptr [rbp + 0x710], rcx
14098d1ec: mov qword ptr [rbp + 0x708], rax
14098d1f3: movaps xmm0, xmmword ptr [rip + 0x212b216]
14098d1fa: movaps xmmword ptr [rbp + 0x690], xmm0
14098d201: lea rcx, [rbp + 0xa20]
14098d208: lea rdx, [rbp + 0x630]
14098d20f: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
14098d214: cmp byte ptr [rbp + 0x356], 8
14098d21b: je 0x14098d229
14098d21d: lea rcx, [rbp + 0x340]
14098d224: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
14098d229: mov qword ptr [rsp + 0x30], rdi
14098d22e: mov qword ptr [rsp + 0x28], rsi
14098d233: mov qword ptr [rsp + 0x20], 0x2f
14098d23c: lea rdx, [rip + 0x212c0c4]
14098d243: lea r9, [rip + 0x212c0d3]
14098d24a: mov r8d, 0x16
14098d250: mov rcx, r14
14098d253: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
14098d258: mov byte ptr [rbp + 0xa20], 0xe
14098d25f: mov dword ptr [rbp + 0x220], 2
14098d269: mov qword ptr [rsp + 0x20], rbx
14098d26e: lea rcx, [rbp + 0xa20]
14098d275: movabs r8, 0x4ee098b38612d2b9
14098d27f: mov rdx, r14
14098d282: mov r9b, 0x19
14098d285: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
14098d28a: lea rdx, [rbp + 0x630]
14098d291: mov r8d, 0x190
14098d297: mov rcx, rbx
14098d29a: call 0x142923c00 ; ?
14098d29f: lea rcx, [rbp + 0x630]
14098d2a6: movabs r8, 0x4ee7d41b19c88236
14098d2b0: mov rdx, rbx
14098d2b3: call 0x141605510 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo>
14098d2b8: movups xmm0, xmmword ptr [rbp + 0x6d0]
14098d2bf: movaps xmmword ptr [rbp + 0xa60], xmm0
14098d2c6: movups xmm0, xmmword ptr [rbp + 0x6c0]
14098d2cd: movaps xmmword ptr [rbp + 0xa50], xmm0
14098d2d4: movups xmm0, xmmword ptr [rbp + 0x6b0]
14098d2db: movaps xmmword ptr [rbp + 0xa40], xmm0
14098d2e2: movups xmm0, xmmword ptr [rbp + 0x6a0]
14098d2e9: movaps xmmword ptr [rbp + 0xa30], xmm0
14098d2f0: movups xmm0, xmmword ptr [rbp + 0x690]
14098d2f7: movaps xmmword ptr [rbp + 0xa20], xmm0
14098d2fe: mov rax, qword ptr [rbp + 0x768]
14098d305: mov rcx, qword ptr [rbp + 0x7b0]
14098d30c: mov rdx, qword ptr [rbp + 0x640]
14098d313: mov qword ptr [rbp + 0x230], rdx
14098d31a: movups xmm0, xmmword ptr [rbp + 0x630]
14098d321: movaps xmmword ptr [rbp + 0x220], xmm0
14098d328: movups xmm0, xmmword ptr [rbp + 0x648]
14098d32f: movaps xmmword ptr [rbp + 0x250], xmm0
14098d336: mov rdx, qword ptr [rbp + 0x658]
14098d33d: mov qword ptr [rbp + 0x260], rdx
14098d344: movups xmm0, xmmword ptr [rbp + 0x660]
14098d34b: movups xmmword ptr [rbp + 0x238], xmm0
14098d352: mov rdx, qword ptr [rbp + 0x670]
14098d359: mov qword ptr [rbp + 0x248], rdx
14098d360: movups xmm0, xmmword ptr [rbp + 0x6e0]
14098d367: movups xmm1, xmmword ptr [rbp + 0x6f0]
14098d36e: movups xmm2, xmmword ptr [rbp + 0x700]
14098d375: movups xmm3, xmmword ptr [rbp + 0x710]
14098d37c: movups xmmword ptr [rbp + 0x298], xmm0
14098d383: movups xmmword ptr [rbp + 0x2a8], xmm1
14098d38a: movups xmmword ptr [rbp + 0x2b8], xmm2
14098d391: movups xmmword ptr [rbp + 0x2c8], xmm3
14098d398: movups xmm0, xmmword ptr [rbp + 0x720]
14098d39f: movups xmmword ptr [rbp + 0x2d8], xmm0
14098d3a6: movups xmm0, xmmword ptr [rbp + 0x730]
14098d3ad: movups xmmword ptr [rbp + 0x2e8], xmm0
14098d3b4: mov rdx, qword ptr [rbp + 0x688]
14098d3bb: mov qword ptr [rbp + 0x278], rdx
14098d3c2: movups xmm0, xmmword ptr [rbp + 0x678]
14098d3c9: movups xmmword ptr [rbp + 0x268], xmm0
14098d3d0: mov qword ptr [rbp + 0x300], rcx
14098d3d7: mov qword ptr [rbp + 0x2f8], rax
14098d3de: movaps xmm6, xmmword ptr [rip + 0x212b03b]
14098d3e5: movaps xmmword ptr [rbp + 0x280], xmm6
14098d3ec: lea rcx, [rbp + 0xa20]
14098d3f3: lea r14, [rbp + 0x220]
14098d3fa: mov rdx, r14
14098d3fd: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
14098d402: cmp byte ptr [rbp + 0x766], 8
14098d409: je 0x14098d417
14098d40b: lea rcx, [rbp + 0x750]
14098d412: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
14098d417: cmp byte ptr [rip + 0x2ca7972], 0
14098d41e: jne 0x14098d632
14098d424: mov qword ptr [rsp + 0x30], rdi
14098d429: mov qword ptr [rsp + 0x28], rsi
14098d42e: mov qword ptr [rsp + 0x20], 0x1e
14098d437: lea rdx, [rip + 0x212bf0e]
14098d43e: lea r9, [rip + 0x212bf11]
14098d445: lea rbx, [rbp + 0x630]
14098d44c: mov r8d, 0xa
14098d452: mov rcx, rbx
14098d455: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
14098d45a: mov byte ptr [rbp + 0xa20], 0x13
14098d461: mov dword ptr [rbp + 0x220], 2
14098d46b: mov qword ptr [rsp + 0x20], r14
14098d470: lea rcx, [rbp + 0xa20]
14098d477: movabs r8, 0x6ad63ccd18c8f041
14098d481: mov rdx, rbx
14098d484: mov r9b, 0x19
14098d487: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
14098d48c: lea rbx, [rbp + 0x220]
14098d493: lea rdx, [rbp + 0x630]
14098d49a: mov r8d, 0x190
14098d4a0: mov rcx, rbx
14098d4a3: call 0x142923c00 ; ?
14098d4a8: lea r15, [rbp + 0x630]
14098d4af: movabs r8, 0x4ee7d41b19c88236
14098d4b9: mov rcx, r15
14098d4bc: mov rdx, rbx
14098d4bf: call 0x141607ec0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::resources::water_mask::WaterMaskSsbo>
14098d4c4: lea rcx, [rbp + 0x220]
14098d4cb: movabs r8, 0x7864ea0bf1becbd8
14098d4d5: mov rdx, r15
14098d4d8: call 0x1416044c0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::resources::water_exclude_clutter_mask::WaterExcludeClutterMaskSsbo>
14098d4dd: movups xmm0, xmmword ptr [rbp + 0x2c0]
14098d4e4: movaps xmmword ptr [rbp + 0xa60], xmm0
14098d4eb: movups xmm0, xmmword ptr [rbp + 0x2b0]
14098d4f2: movaps xmmword ptr [rbp + 0xa50], xmm0
14098d4f9: movups xmm0, xmmword ptr [rbp + 0x2a0]
14098d500: movaps xmmword ptr [rbp + 0xa40], xmm0
14098d507: movups xmm0, xmmword ptr [rbp + 0x290]
14098d50e: movaps xmmword ptr [rbp + 0xa30], xmm0
14098d515: movups xmm0, xmmword ptr [rbp + 0x280]
14098d51c: movaps xmmword ptr [rbp + 0xa20], xmm0
14098d523: mov rax, qword ptr [rbp + 0x358]
14098d52a: mov rcx, qword ptr [rbp + 0x3a0]
14098d531: mov rdx, qword ptr [rbp + 0x230]
14098d538: mov qword ptr [rbp + 0x640], rdx
14098d53f: movups xmm0, xmmword ptr [rbp + 0x220]
14098d546: movaps xmmword ptr [rbp + 0x630], xmm0
14098d54d: movups xmm0, xmmword ptr [rbp + 0x238]
14098d554: movaps xmmword ptr [rbp + 0x660], xmm0
14098d55b: mov rdx, qword ptr [rbp + 0x248]
14098d562: mov qword ptr [rbp + 0x670], rdx
14098d569: movups xmm0, xmmword ptr [rbp + 0x250]
14098d570: movups xmmword ptr [rbp + 0x648], xmm0
14098d577: mov rdx, qword ptr [rbp + 0x260]
14098d57e: mov qword ptr [rbp + 0x658], rdx
14098d585: movups xmm0, xmmword ptr [rbp + 0x2d0]
14098d58c: movups xmm1, xmmword ptr [rbp + 0x2e0]
14098d593: movups xmm2, xmmword ptr [rbp + 0x2f0]
14098d59a: movups xmm3, xmmword ptr [rbp + 0x300]
14098d5a1: movups xmmword ptr [rbp + 0x6a8], xmm0
14098d5a8: movups xmmword ptr [rbp + 0x6b8], xmm1
14098d5af: movups xmmword ptr [rbp + 0x6c8], xmm2
14098d5b6: movups xmmword ptr [rbp + 0x6d8], xmm3
14098d5bd: movups xmm0, xmmword ptr [rbp + 0x310]
14098d5c4: movups xmmword ptr [rbp + 0x6e8], xmm0
14098d5cb: movups xmm0, xmmword ptr [rbp + 0x320]
14098d5d2: movups xmmword ptr [rbp + 0x6f8], xmm0
14098d5d9: mov rdx, qword ptr [rbp + 0x278]
14098d5e0: mov qword ptr [rbp + 0x688], rdx
14098d5e7: movups xmm0, xmmword ptr [rbp + 0x268]
14098d5ee: movups xmmword ptr [rbp + 0x678], xmm0
14098d5f5: mov qword ptr [rbp + 0x710], rcx
14098d5fc: mov qword ptr [rbp + 0x708], rax
14098d603: movaps xmmword ptr [rbp + 0x690], xmm6
14098d60a: lea rcx, [rbp + 0xa20]
14098d611: lea rdx, [rbp + 0x630]
14098d618: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
14098d61d: cmp byte ptr [rbp + 0x356], 8
14098d624: je 0x14098d632
14098d626: lea rcx, [rbp + 0x340]
14098d62d: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
14098d632: mov qword ptr [rsp + 0x30], rdi
14098d637: mov qword ptr [rsp + 0x28], rsi
14098d63c: mov qword ptr [rsp + 0x20], 0x20
14098d645: lea rdx, [rip + 0x212bd28]
14098d64c: lea r9, [rip + 0x212bd2c]
14098d653: lea rbx, [rbp + 0x630]
14098d65a: mov r8d, 0xb
14098d660: mov rcx, rbx
14098d663: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
14098d668: mov byte ptr [rbp + 0xa20], 0x14
14098d66f: mov dword ptr [rbp + 0x220], 2
14098d679: mov qword ptr [rsp + 0x20], r14
14098d67e: lea rcx, [rbp + 0xa20]
14098d685: movabs r8, 0x49f43a487883738d
14098d68f: mov rdx, rbx
14098d692: mov r9b, 0x19
14098d695: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
14098d69a: lea rbx, [rbp + 0x220]
14098d6a1: lea rdx, [rbp + 0x630]
14098d6a8: mov r8d, 0x190
14098d6ae: mov rcx, rbx
14098d6b1: call 0x142923c00 ; ?
14098d6b6: lea rcx, [rbp + 0x630]
14098d6bd: movabs r8, 0x4ee7d41b19c88236
14098d6c7: mov rdx, rbx
14098d6ca: call 0x141605340 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::resources::garden_mask::GardenMaskSsbo>
14098d6cf: movups xmm0, xmmword ptr [rbp + 0x6d0]
14098d6d6: movaps xmmword ptr [rbp + 0xa60], xmm0
14098d6dd: movups xmm0, xmmword ptr [rbp + 0x6c0]
14098d6e4: movaps xmmword ptr [rbp + 0xa50], xmm0
14098d6eb: movups xmm0, xmmword ptr [rbp + 0x6b0]
14098d6f2: movaps xmmword ptr [rbp + 0xa40], xmm0
14098d6f9: movups xmm0, xmmword ptr [rbp + 0x6a0]
14098d700: movaps xmmword ptr [rbp + 0xa30], xmm0
14098d707: movups xmm0, xmmword ptr [rbp + 0x690]
14098d70e: movaps xmmword ptr [rbp + 0xa20], xmm0
14098d715: mov rax, qword ptr [rbp + 0x768]
14098d71c: mov rcx, qword ptr [rbp + 0x7b0]
14098d723: mov rdx, qword ptr [rbp + 0x640]
14098d72a: mov qword ptr [rbp + 0x230], rdx
14098d731: movups xmm0, xmmword ptr [rbp + 0x630]
14098d738: movaps xmmword ptr [rbp + 0x220], xmm0
14098d73f: movups xmm0, xmmword ptr [rbp + 0x648]
14098d746: movaps xmmword ptr [rbp + 0x250], xmm0
14098d74d: mov rdx, qword ptr [rbp + 0x658]
14098d754: mov qword ptr [rbp + 0x260], rdx
14098d75b: movups xmm0, xmmword ptr [rbp + 0x660]
14098d762: movups xmmword ptr [rbp + 0x238], xmm0
14098d769: mov rdx, qword ptr [rbp + 0x670]
14098d770: mov qword ptr [rbp + 0x248], rdx
14098d777: movups xmm0, xmmword ptr [rbp + 0x6e0]
14098d77e: movups xmm1, xmmword ptr [rbp + 0x6f0]
14098d785: movups xmm2, xmmword ptr [rbp + 0x700]
14098d78c: movups xmm3, xmmword ptr [rbp + 0x710]
14098d793: movups xmmword ptr [rbp + 0x298], xmm0
14098d79a: movups xmmword ptr [rbp + 0x2a8], xmm1
14098d7a1: movups xmmword ptr [rbp + 0x2b8], xmm2
14098d7a8: movups xmmword ptr [rbp + 0x2c8], xmm3
14098d7af: movups xmm0, xmmword ptr [rbp + 0x720]
14098d7b6: movups xmmword ptr [rbp + 0x2d8], xmm0
14098d7bd: movups xmm0, xmmword ptr [rbp + 0x730]
14098d7c4: movups xmmword ptr [rbp + 0x2e8], xmm0
14098d7cb: mov rdx, qword ptr [rbp + 0x688]
14098d7d2: mov qword ptr [rbp + 0x278], rdx
14098d7d9: movups xmm0, xmmword ptr [rbp + 0x678]
14098d7e0: movups xmmword ptr [rbp + 0x268], xmm0
14098d7e7: mov qword ptr [rbp + 0x300], rcx
14098d7ee: mov qword ptr [rbp + 0x2f8], rax
14098d7f5: movaps xmmword ptr [rbp + 0x280], xmm6
14098d7fc: lea rcx, [rbp + 0xa20]
14098d803: lea rdx, [rbp + 0x220]
14098d80a: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
14098d80f: cmp byte ptr [rbp + 0x766], 8
14098d816: je 0x14098d824
14098d818: lea rcx, [rbp + 0x750]
14098d81f: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
14098d824: movabs rdx, 0xfc8691076ab4d826
14098d82e: movabs r8, 0x24ccb35ae3644351
14098d838: mov rcx, qword ptr [rbp + 0xbf8]
14098d83f: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
14098d844: test rax, rax
14098d847: je 0x14098d8f5
14098d84d: cmp qword ptr [rsi + 0x218], rdx
14098d854: jbe 0x14098d8f5
14098d85a: mov rax, qword ptr [rsi + 0x210]
14098d861: mov rcx, qword ptr [rax + rdx*8]
14098d865: test rcx, rcx
14098d868: je 0x14098d8f5
14098d86e: mov rax, qword ptr [rsi + 0x1e0]
14098d875: not rcx
14098d878: lea rcx, [rcx + rcx*2]
14098d87c: shl rcx, 5
14098d880: cmp qword ptr [rax + rcx + 0x20], 0
14098d886: je 0x14098d8f5
14098d888: add rax, rcx
14098d88b: mov rdx, qword ptr [rax + 0x10]
14098d88f: mov ecx, dword ptr [rsi + 0x5b0]
14098d895: mov dword ptr [rax + 0x5c], ecx
14098d898: mov rcx, qword ptr [rdx]
14098d89b: xor r8d, r8d
14098d89e: mov rax, rcx
14098d8a1: lock cmpxchg qword ptr [rdx], r8
14098d8a6: test rcx, rcx
14098d8a9: setne al
14098d8ac: or byte ptr [rdx + 8], al
14098d8af: movaps xmm6, xmmword ptr [rbp + 0xc10]
14098d8b6: movaps xmm7, xmmword ptr [rbp + 0xc20]
14098d8bd: add rsp, 0xcb8
14098d8c4: pop rbx
14098d8c5: pop rdi
14098d8c6: pop rsi
14098d8c7: pop r12
14098d8c9: pop r13
14098d8cb: pop r14
14098d8cd: pop r15
14098d8cf: pop rbp
14098d8d0: ret 
14098d8d1: lea rcx, [rip + 0x212b8a8]
14098d8d8: call 0x1428d9310 ; core::option::unwrap_failed
14098d8dd: mov byte ptr [rbp + 0xc07], 1
14098d8e4: lea rcx, [rip + 0x212b8c5]
14098d8eb: call 0x1428d9310 ; core::option::unwrap_failed
14098d8f0: jmp 0x14098da29
14098d8f5: lea rcx, [rip + 0x212b9b4]
14098d8fc: call 0x1428d9310 ; core::option::unwrap_failed
14098d901: mov qword ptr [rbp + 0x630], r8
14098d908: mov byte ptr [rbp + 0xc07], 1
14098d90f: lea rax, [rip + 0x212b8b2]
14098d916: mov qword ptr [rsp + 0x20], rax
14098d91b: lea rcx, [rip + 0x212b056]
14098d922: lea r9, [rip + 0x212b02f]
14098d929: lea r8, [rbp + 0x630]
14098d930: mov edx, 0x2b
14098d935: call 0x1428d9760 ; core::result::unwrap_failed
14098d93a: jmp 0x14098da29
14098d93f: mov byte ptr [rbp + 0xc07], 1
14098d946: lea rcx, [rip + 0x212b893]
14098d94d: call 0x1428d9310 ; core::option::unwrap_failed
14098d952: jmp 0x14098da29
14098d957: mov byte ptr [rbp + 0xc07], 1
14098d95e: lea rcx, [rip + 0x212b893]
14098d965: call 0x1428d9310 ; core::option::unwrap_failed
14098d96a: jmp 0x14098da29
14098d96f: mov byte ptr [rbp + 0xc07], 1
14098d976: lea rcx, [rip + 0x212b893]
14098d97d: call 0x1428d9310 ; core::option::unwrap_failed
14098d982: jmp 0x14098da29
14098d987: mov qword ptr [rbp + 0x630], r8
14098d98e: mov byte ptr [rbp + 0xc07], 1
14098d995: lea rax, [rip + 0x212b8a4]
14098d99c: mov qword ptr [rsp + 0x20], rax
14098d9a1: lea rcx, [rip + 0x212afd0]
14098d9a8: lea r9, [rip + 0x212afa9]
14098d9af: lea r8, [rbp + 0x630]
14098d9b6: mov edx, 0x2b
14098d9bb: call 0x1428d9760 ; core::result::unwrap_failed
14098d9c0: jmp 0x14098da29
14098d9c2: mov byte ptr [rbp + 0xc07], 1
14098d9c9: lea rcx, [rip + 0x212b888]
14098d9d0: call 0x1428d9310 ; core::option::unwrap_failed
14098d9d5: jmp 0x14098da29
14098d9d7: mov byte ptr [rbp + 0xc07], 1
14098d9de: lea rcx, [rip + 0x212b88b]
14098d9e5: call 0x1428d9310 ; core::option::unwrap_failed
14098d9ea: jmp 0x14098da29
14098d9ec: mov byte ptr [rbp + 0xc07], 1
14098d9f3: lea rcx, [rip + 0x212b88e]
14098d9fa: call 0x1428d9310 ; core::option::unwrap_failed
14098d9ff: jmp 0x14098da29
14098da01: mov byte ptr [rbp + 0xc07], 1
14098da08: lea rcx, [rip + 0x22ad091]
14098da0f: call 0x1428d80d0 ; std::thread::local::panic_access_error
14098da14: jmp 0x14098da29
14098da16: mov byte ptr [rbp + 0xc07], 1
14098da1d: lea rcx, [rip + 0x22ad07c]
14098da24: call 0x1428d80d0 ; std::thread::local::panic_access_error
14098da29: ud2 
14098da2b: nop dword ptr [rax + rax]
14098da30: mov qword ptr [rsp + 0x10], rdx
14098da35: push rbp
14098da36: push r15
14098da38: push r14
14098da3a: push r13
14098da3c: push r12
14098da3e: push rsi
14098da3f: push rdi
14098da40: push rbx
14098da41: sub rsp, 0x68
14098da45: lea rbp, [rdx + 0x80]
14098da4c: movaps xmmword ptr [rsp + 0x40], xmm7
14098da51: movaps xmmword ptr [rsp + 0x50], xmm6
14098da56: cmp byte ptr [rbp + 0x356], 8
14098da5d: je 0x14098da6b
14098da5f: lea rcx, [rbp + 0x340]
14098da66: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
14098da6b: movaps xmm6, xmmword ptr [rsp + 0x50]
14098da70: movaps xmm7, xmmword ptr [rsp + 0x40]
14098da75: add rsp, 0x68
14098da79: pop rbx
14098da7a: pop rdi
14098da7b: pop rsi
14098da7c: pop r12
14098da7e: pop r13
14098da80: pop r14
14098da82: pop r15
14098da84: pop rbp
14098da85: ret 
14098da86: nop word ptr cs:[rax + rax]
14098da90: mov qword ptr [rsp + 0x10], rdx
14098da95: push rbp
14098da96: push r15
14098da98: push r14
14098da9a: push r13
14098da9c: push r12
14098da9e: push rsi
14098da9f: push rdi
14098daa0: push rbx
14098daa1: sub rsp, 0x68
14098daa5: lea rbp, [rdx + 0x80]
14098daac: movaps xmmword ptr [rsp + 0x40], xmm7
14098dab1: movaps xmmword ptr [rsp + 0x50], xmm6
14098dab6: lea rcx, [rbp + 0x630]
14098dabd: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14098dac2: movaps xmm6, xmmword ptr [rsp + 0x50]
14098dac7: movaps xmm7, xmmword ptr [rsp + 0x40]
14098dacc: add rsp, 0x68
14098dad0: pop rbx
14098dad1: pop rdi
14098dad2: pop rsi
14098dad3: pop r12
14098dad5: pop r13
14098dad7: pop r14
14098dad9: pop r15
14098dadb: pop rbp
14098dadc: ret 
14098dadd: nop dword ptr [rax]
14098dae0: mov qword ptr [rsp + 0x10], rdx
14098dae5: push rbp
14098dae6: push r15
14098dae8: push r14
14098daea: push r13
14098daec: push r12
14098daee: push rsi
14098daef: push rdi
14098daf0: push rbx
14098daf1: sub rsp, 0x68
14098daf5: lea rbp, [rdx + 0x80]
14098dafc: movaps xmmword ptr [rsp + 0x40], xmm7
14098db01: movaps xmmword ptr [rsp + 0x50], xmm6
14098db06: cmp byte ptr [rbp + 0x766], 8
14098db0d: je 0x14098db1b
14098db0f: lea rcx, [rbp + 0x750]
14098db16: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
14098db1b: movaps xmm6, xmmword ptr [rsp + 0x50]
14098db20: movaps xmm7, xmmword ptr [rsp + 0x40]
14098db25: add rsp, 0x68
14098db29: pop rbx
14098db2a: pop rdi
14098db2b: pop rsi
14098db2c: pop r12
14098db2e: pop r13
14098db30: pop r14
14098db32: pop r15
14098db34: pop rbp
14098db35: ret 
14098db36: nop word ptr cs:[rax + rax]
14098db40: mov qword ptr [rsp + 0x10], rdx
14098db45: push rbp
14098db46: push r15
14098db48: push r14
14098db4a: push r13
14098db4c: push r12
14098db4e: push rsi
14098db4f: push rdi
14098db50: push rbx
14098db51: sub rsp, 0x68
14098db55: lea rbp, [rdx + 0x80]
14098db5c: movaps xmmword ptr [rsp + 0x40], xmm7
14098db61: movaps xmmword ptr [rsp + 0x50], xmm6
14098db66: lea rcx, [rbp + 0x630]
14098db6d: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14098db72: movaps xmm6, xmmword ptr [rsp + 0x50]
14098db77: movaps xmm7, xmmword ptr [rsp + 0x40]
14098db7c: add rsp, 0x68
14098db80: pop rbx
14098db81: pop rdi
14098db82: pop rsi
14098db83: pop r12
14098db85: pop r13
14098db87: pop r14
14098db89: pop r15
14098db8b: pop rbp
14098db8c: ret 
14098db8d: nop dword ptr [rax]
14098db90: mov qword ptr [rsp + 0x10], rdx
14098db95: push rbp
14098db96: push r15
14098db98: push r14
14098db9a: push r13
14098db9c: push r12
14098db9e: push rsi
14098db9f: push rdi
14098dba0: push rbx
14098dba1: sub rsp, 0x68
14098dba5: lea rbp, [rdx + 0x80]
14098dbac: movaps xmmword ptr [rsp + 0x40], xmm7
14098dbb1: movaps xmmword ptr [rsp + 0x50], xmm6
14098dbb6: cmp byte ptr [rbp + 0x356], 8
14098dbbd: je 0x14098dbcb
14098dbbf: lea rcx, [rbp + 0x340]
14098dbc6: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
14098dbcb: movaps xmm6, xmmword ptr [rsp + 0x50]
14098dbd0: movaps xmm7, xmmword ptr [rsp + 0x40]
14098dbd5: add rsp, 0x68
14098dbd9: pop rbx
14098dbda: pop rdi
14098dbdb: pop rsi
14098dbdc: pop r12
14098dbde: pop r13
14098dbe0: pop r14
14098dbe2: pop r15
14098dbe4: pop rbp
14098dbe5: ret 
14098dbe6: nop word ptr cs:[rax + rax]
14098dbf0: mov qword ptr [rsp + 0x10], rdx
14098dbf5: push rbp
14098dbf6: push r15
14098dbf8: push r14
14098dbfa: push r13
14098dbfc: push r12
14098dbfe: push rsi
14098dbff: push rdi
14098dc00: push rbx
14098dc01: sub rsp, 0x68
14098dc05: lea rbp, [rdx + 0x80]
14098dc0c: movaps xmmword ptr [rsp + 0x40], xmm7
14098dc11: movaps xmmword ptr [rsp + 0x50], xmm6
14098dc16: lea rcx, [rbp + 0x630]
14098dc1d: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14098dc22: movaps xmm6, xmmword ptr [rsp + 0x50]
14098dc27: movaps xmm7, xmmword ptr [rsp + 0x40]
14098dc2c: add rsp, 0x68
14098dc30: pop rbx
14098dc31: pop rdi
14098dc32: pop rsi
14098dc33: pop r12
14098dc35: pop r13
14098dc37: pop r14
14098dc39: pop r15
14098dc3b: pop rbp
14098dc3c: ret 
14098dc3d: nop dword ptr [rax]
14098dc40: mov qword ptr [rsp + 0x10], rdx
14098dc45: push rbp
14098dc46: push r15
14098dc48: push r14
14098dc4a: push r13
14098dc4c: push r12
14098dc4e: push rsi
14098dc4f: push rdi
14098dc50: push rbx
14098dc51: sub rsp, 0x68
14098dc55: lea rbp, [rdx + 0x80]
14098dc5c: movaps xmmword ptr [rsp + 0x40], xmm7
14098dc61: movaps xmmword ptr [rsp + 0x50], xmm6
14098dc66: cmp byte ptr [rbp + 0x766], 8
14098dc6d: je 0x14098dc7b
14098dc6f: lea rcx, [rbp + 0x750]
14098dc76: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
14098dc7b: movaps xmm6, xmmword ptr [rsp + 0x50]
14098dc80: movaps xmm7, xmmword ptr [rsp + 0x40]
14098dc85: add rsp, 0x68
14098dc89: pop rbx
14098dc8a: pop rdi
14098dc8b: pop rsi
14098dc8c: pop r12
14098dc8e: pop r13
14098dc90: pop r14
14098dc92: pop r15
14098dc94: pop rbp
14098dc95: ret 
14098dc96: nop word ptr cs:[rax + rax]
14098dca0: mov qword ptr [rsp + 0x10], rdx
14098dca5: push rbp
14098dca6: push r15
14098dca8: push r14
14098dcaa: push r13
14098dcac: push r12
14098dcae: push rsi
14098dcaf: push rdi
14098dcb0: push rbx
14098dcb1: sub rsp, 0x68
14098dcb5: lea rbp, [rdx + 0x80]
14098dcbc: movaps xmmword ptr [rsp + 0x40], xmm7
14098dcc1: movaps xmmword ptr [rsp + 0x50], xmm6
14098dcc6: cmp byte ptr [rbp + 0xc07], 0
14098dccd: jne 0x14098dcea
14098dccf: movaps xmm6, xmmword ptr [rsp + 0x50]
14098dcd4: movaps xmm7, xmmword ptr [rsp + 0x40]
14098dcd9: add rsp, 0x68
14098dcdd: pop rbx
14098dcde: pop rdi
14098dcdf: pop rsi
14098dce0: pop r12
14098dce2: pop r13
14098dce4: pop r14
14098dce6: pop r15
14098dce8: pop rbp
14098dce9: ret 
14098dcea: lea rcx, [rbp + 0x220]
14098dcf1: call 0x1408923c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::RasterPass>
14098dcf6: jmp 0x14098dccf
14098dcf8: nop dword ptr [rax + rax]
14098dd00: mov qword ptr [rsp + 0x10], rdx
14098dd05: push rbp
14098dd06: push r15
14098dd08: push r14
14098dd0a: push r13
14098dd0c: push r12
14098dd0e: push rsi
14098dd0f: push rdi
14098dd10: push rbx
14098dd11: sub rsp, 0x68
14098dd15: lea rbp, [rdx + 0x80]
14098dd1c: movaps xmmword ptr [rsp + 0x40], xmm7
14098dd21: movaps xmmword ptr [rsp + 0x50], xmm6
14098dd26: lea rcx, [rbp + 0x220]
14098dd2d: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14098dd32: movaps xmm6, xmmword ptr [rsp + 0x50]
14098dd37: movaps xmm7, xmmword ptr [rsp + 0x40]
14098dd3c: add rsp, 0x68
14098dd40: pop rbx
14098dd41: pop rdi
14098dd42: pop rsi
14098dd43: pop r12
14098dd45: pop r13
14098dd47: pop r14
14098dd49: pop r15
14098dd4b: pop rbp
14098dd4c: ret 
14098dd4d: nop dword ptr [rax]
14098dd50: mov qword ptr [rsp + 0x10], rdx
14098dd55: push rbp
14098dd56: push r15
14098dd58: push r14
14098dd5a: push r13
14098dd5c: push r12
14098dd5e: push rsi
14098dd5f: push rdi
14098dd60: push rbx
14098dd61: sub rsp, 0x68
14098dd65: lea rbp, [rdx + 0x80]
14098dd6c: movaps xmmword ptr [rsp + 0x40], xmm7
14098dd71: movaps xmmword ptr [rsp + 0x50], xmm6
14098dd76: lea rcx, [rbp + 0x630]
14098dd7d: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14098dd82: movaps xmm6, xmmword ptr [rsp + 0x50]
14098dd87: movaps xmm7, xmmword ptr [rsp + 0x40]
14098dd8c: add rsp, 0x68
14098dd90: pop rbx
14098dd91: pop rdi
14098dd92: pop rsi
14098dd93: pop r12
14098dd95: pop r13
14098dd97: pop r14
14098dd99: pop r15
14098dd9b: pop rbp
14098dd9c: ret 
