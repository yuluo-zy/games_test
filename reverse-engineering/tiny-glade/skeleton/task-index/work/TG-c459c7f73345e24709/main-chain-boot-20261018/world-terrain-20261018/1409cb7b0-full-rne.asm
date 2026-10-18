1409cb7b0: push rbp
1409cb7b1: push r15
1409cb7b3: push r14
1409cb7b5: push r13
1409cb7b7: push r12
1409cb7b9: push rsi
1409cb7ba: push rdi
1409cb7bb: push rbx
1409cb7bc: sub rsp, 0xf58
1409cb7c3: lea rbp, [rsp + 0x80]
1409cb7cb: mov qword ptr [rbp + 0xed0], 0xfffffffffffffffe
1409cb7d6: mov rdi, rdx
1409cb7d9: mov rbx, rcx
1409cb7dc: lea r14, [rdx + 0x38]
1409cb7e0: movabs rdx, 0x1e9ac9b6648cf0a6
1409cb7ea: movabs r8, 0xe6222c4af0fcfd90
1409cb7f4: mov rcx, r14
1409cb7f7: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1409cb7fc: test rax, rax
1409cb7ff: je 0x1409cb8c4
1409cb805: cmp qword ptr [rdi + 0x218], rdx
1409cb80c: jbe 0x1409cb8c4
1409cb812: mov rax, qword ptr [rdi + 0x210]
1409cb819: mov rcx, qword ptr [rax + rdx*8]
1409cb81d: test rcx, rcx
1409cb820: je 0x1409cb8c4
1409cb826: mov rax, qword ptr [rdi + 0x1e0]
1409cb82d: not rcx
1409cb830: lea rcx, [rcx + rcx*2]
1409cb834: shl rcx, 5
1409cb838: cmp qword ptr [rax + rcx + 0x20], 0
1409cb83e: je 0x1409cb8c4
1409cb844: add rax, rcx
1409cb847: mov rsi, qword ptr [rax + 0x10]
1409cb84b: call 0x140c16a00 ; puffin::are_scopes_on
1409cb850: test al, al
1409cb852: je 0x1409cb9f6
1409cb858: lea r12, [rip + 0x20f4f8c]
1409cb85f: mov r15d, 0x69
1409cb865: mov edx, 0x69
1409cb86a: mov rcx, r12
1409cb86d: call 0x1409c8970 ; core::str::impl$0::rfind<ref$<str$> >
1409cb872: cmp rax, 1
1409cb876: jne 0x1409cba70
1409cb87c: mov r9, rdx
1409cb87f: test rdx, rdx
1409cb882: je 0x1409cba03
1409cb888: cmp r9, 0x69
1409cb88c: jae 0x1409cb9fd
1409cb892: lea rax, [rip + 0x20f4f52]
1409cb899: cmp byte ptr [r9 + rax], 0xbf
1409cb89e: jg 0x1409cba03
1409cb8a4: lea rax, [rip + 0x20f5075]
1409cb8ab: mov qword ptr [rsp + 0x20], rax
1409cb8b0: lea rcx, [rip + 0x20f4f34]
1409cb8b7: mov edx, 0x69
1409cb8bc: xor r8d, r8d
1409cb8bf: call 0x1428d9940 ; core::str::slice_error_fail
1409cb8c4: cmp byte ptr [rip + 0x2c6942d], 0
1409cb8cb: je 0x1409cb9e2
1409cb8d1: movabs rdx, 0xa68561b396784eaa
1409cb8db: movabs r8, 0xf5fd74e6faf4ed14
1409cb8e5: mov rcx, r14
1409cb8e8: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1409cb8ed: test rax, rax
1409cb8f0: je 0x1409cc027
1409cb8f6: cmp qword ptr [rdi + 0x218], rdx
1409cb8fd: jbe 0x1409cc027
1409cb903: mov rax, qword ptr [rdi + 0x210]
1409cb90a: mov rcx, qword ptr [rax + rdx*8]
1409cb90e: test rcx, rcx
1409cb911: je 0x1409cc027
1409cb917: mov rax, qword ptr [rdi + 0x1e0]
1409cb91e: not rcx
1409cb921: lea rcx, [rcx + rcx*2]
1409cb925: shl rcx, 5
1409cb929: cmp qword ptr [rax + rcx + 0x20], 0
1409cb92f: je 0x1409cc027
1409cb935: add rax, rcx
1409cb938: mov rdx, qword ptr [rax + 0x10]
1409cb93c: mov eax, dword ptr [rdi + 0x5b0]
1409cb942: lea rsi, [rbp + 0x990]
1409cb949: mov rcx, rsi
1409cb94c: mov r8b, 0x10
1409cb94f: call 0x14093cf30 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_texture
1409cb954: lea rcx, [rbp + 0x994]
1409cb95b: call 0x140bd5790 ; rhapsody::raw::texture::TextureDesc::extent_2d
1409cb960: mov dword ptr [rbp + 0x3b8], 1
1409cb96a: xorps xmm0, xmm0
1409cb96d: movups xmmword ptr [rbp + 0x3bc], xmm0
1409cb974: mov dword ptr [rbp + 0x3cc], 0
1409cb97e: mov qword ptr [rbp + 0x3d0], rsi
1409cb985: mov dword ptr [rbp + 0xd78], 2
1409cb98f: mov qword ptr [rsp + 0x40], rbx
1409cb994: mov qword ptr [rsp + 0x38], rdi
1409cb999: mov qword ptr [rsp + 0x30], rax
1409cb99e: lea rax, [rbp + 0xd78]
1409cb9a5: mov qword ptr [rsp + 0x28], rax
1409cb9aa: mov qword ptr [rsp + 0x20], 1
1409cb9b3: lea rdx, [rip + 0x20f5296]
1409cb9ba: lea rsi, [rbp - 0x30]
1409cb9be: lea r9, [rbp + 0x3b8]
1409cb9c5: mov r8d, 0x11
1409cb9cb: mov rcx, rsi
1409cb9ce: call 0x140867970 ; country_core::render::pass_builder::draw_stream::RasterPass::new_impl
1409cb9d3: mov rcx, rsi
1409cb9d6: call 0x140868370 ; country_core::render::pass_builder::draw_stream::RasterPass::render
1409cb9db: mov byte ptr [rip + 0x2c69316], 0
1409cb9e2: add rsp, 0xf58
1409cb9e9: pop rbx
1409cb9ea: pop rdi
1409cb9eb: pop rsi
1409cb9ec: pop r12
1409cb9ee: pop r13
1409cb9f0: pop r14
1409cb9f2: pop r15
1409cb9f4: pop rbp
1409cb9f5: ret 
1409cb9f6: xor eax, eax
1409cb9f8: jmp 0x1409cbbe4
1409cb9fd: jne 0x1409cb8a4
1409cba03: lea r12, [rip + 0x20f4de1]
1409cba0a: mov rcx, r12
1409cba0d: mov rdx, r9
1409cba10: call 0x1409c8970 ; core::str::impl$0::rfind<ref$<str$> >
1409cba15: cmp rax, 1
1409cba19: jne 0x1409cba70
1409cba1b: mov r8, rdx
1409cba1e: add r8, 2
1409cba22: je 0x1409cba5d
1409cba24: cmp r8, 0x69
1409cba28: jae 0x1409cba5b
1409cba2a: lea rax, [rip + 0x20f4dba]
1409cba31: cmp byte ptr [r8 + rax], 0xbf
1409cba36: jg 0x1409cba5d
1409cba38: lea rax, [rip + 0x20f4ef9]
1409cba3f: mov qword ptr [rsp + 0x20], rax
1409cba44: lea rcx, [rip + 0x20f4da0]
1409cba4b: mov edx, 0x69
1409cba50: mov r9d, 0x69
1409cba56: call 0x1428d9940 ; core::str::slice_error_fail
1409cba5b: jne 0x1409cba38
1409cba5d: mov r15d, 0x67
1409cba63: sub r15, rdx
1409cba66: lea r12, [rip + 0x20f4d7e]
1409cba6d: add r12, r8
1409cba70: lea rax, [rip + 0x20f5172]
1409cba77: lea r13, [rip + 0x20f511a]
1409cba7e: mov qword ptr [rbp + 0xec0], rsi
1409cba85: nop word ptr cs:[rax + rax]
1409cba90: movsx ecx, byte ptr [rax - 1]
1409cba94: test ecx, ecx
1409cba96: js 0x1409cbab0
1409cba98: dec rax
1409cba9b: cmp ecx, 0x5c
1409cba9e: jne 0x1409cbb09
1409cbaa0: jmp 0x1409cbb1e
1409cbaa2: nop word ptr cs:[rax + rax]
1409cbab0: movzx edx, byte ptr [rax - 2]
1409cbab4: cmp dl, 0xc0
1409cbab7: jge 0x1409cbade
1409cbab9: movzx r8d, byte ptr [rax - 3]
1409cbabe: cmp r8b, 0xc0
1409cbac2: jge 0x1409cbae7
1409cbac4: movzx r9d, byte ptr [rax - 4]
1409cbac9: add rax, -4
1409cbacd: and r9d, 7
1409cbad1: shl r9d, 6
1409cbad5: and r8d, 0x3f
1409cbad9: or r8d, r9d
1409cbadc: jmp 0x1409cbaef
1409cbade: add rax, -2
1409cbae2: and edx, 0x1f
1409cbae5: jmp 0x1409cbaf9
1409cbae7: add rax, -3
1409cbaeb: and r8d, 0xf
1409cbaef: shl r8d, 6
1409cbaf3: and edx, 0x3f
1409cbaf6: or edx, r8d
1409cbaf9: shl edx, 6
1409cbafc: and cl, 0x3f
1409cbaff: movzx ecx, cl
1409cbb02: or ecx, edx
1409cbb04: cmp ecx, 0x5c
1409cbb07: je 0x1409cbb1e
1409cbb09: cmp ecx, 0x2f
1409cbb0c: je 0x1409cbb1e
1409cbb0e: cmp rax, r13
1409cbb11: jne 0x1409cba90
1409cbb17: mov esi, 0x51
1409cbb1c: jmp 0x1409cbb6d
1409cbb1e: lea r13, [rip + 0x20f5073]
1409cbb25: sub rax, r13
1409cbb28: mov r8, rax
1409cbb2b: inc r8
1409cbb2e: je 0x1409cbb62
1409cbb30: cmp r8, 0x51
1409cbb34: jae 0x1409cbb60
1409cbb36: cmp byte ptr [r8 + r13], 0xbf
1409cbb3b: jg 0x1409cbb62
1409cbb3d: lea rax, [rip + 0x20f4dbc]
1409cbb44: mov qword ptr [rsp + 0x20], rax
1409cbb49: lea rcx, [rip + 0x20f5048]
1409cbb50: mov edx, 0x51
1409cbb55: mov r9d, 0x51
1409cbb5b: call 0x1428d9940 ; core::str::slice_error_fail
1409cbb60: jne 0x1409cbb3d
1409cbb62: mov esi, 0x50
1409cbb67: sub rsi, rax
1409cbb6a: add r13, r8
1409cbb6d: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1409cbb72: mov rcx, rax
1409cbb75: mov rax, qword ptr [rax]
1409cbb78: cmp rax, 1
1409cbb7c: jne 0x1409cbff7
1409cbb82: add rcx, 8
1409cbb86: cmp qword ptr [rcx], 0
1409cbb8a: jne 0x1409cc01b
1409cbb90: mov qword ptr [rcx], 0xffffffffffffffff
1409cbb97: mov qword ptr [rbp + 0xeb8], rcx
1409cbb9e: add rcx, 8
1409cbba2: mov qword ptr [rsp + 0x20], rsi
1409cbba7: mov qword ptr [rsp + 0x30], 0
1409cbbb0: mov qword ptr [rsp + 0x28], 1
1409cbbb9: mov rdx, r12
1409cbbbc: mov r8, r15
1409cbbbf: mov r9, r13
1409cbbc2: call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
1409cbbc7: mov rcx, qword ptr [rbp + 0xeb8]
1409cbbce: inc qword ptr [rcx]
1409cbbd1: mov qword ptr [rbp + 0xeb0], rax
1409cbbd8: mov eax, 1
1409cbbdd: mov rsi, qword ptr [rbp + 0xec0]
1409cbbe4: mov qword ptr [rbp + 0xea8], rax
1409cbbeb: movabs rdx, 0x8597ee0727b461b3
1409cbbf5: movabs r8, 0x8b297ec5ae15f15c
1409cbbff: mov rcx, r14
1409cbc02: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1409cbc07: test rax, rax
1409cbc0a: je 0x1409cc033
1409cbc10: cmp qword ptr [rdi + 0x218], rdx
1409cbc17: jbe 0x1409cc033
1409cbc1d: mov rax, qword ptr [rdi + 0x210]
1409cbc24: mov rcx, qword ptr [rax + rdx*8]
1409cbc28: test rcx, rcx
1409cbc2b: je 0x1409cc033
1409cbc31: mov rax, qword ptr [rdi + 0x1e0]
1409cbc38: not rcx
1409cbc3b: lea rcx, [rcx + rcx*2]
1409cbc3f: shl rcx, 5
1409cbc43: cmp qword ptr [rax + rcx + 0x20], 0
1409cbc49: je 0x1409cc033
1409cbc4f: add rax, rcx
1409cbc52: mov rcx, qword ptr [rax + 0x10]
1409cbc56: lea rdx, [rip + 0x20f4fab]
1409cbc5d: mov r8d, 0x18
1409cbc63: call 0x140937aa0 ; country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
1409cbc68: test al, 1
1409cbc6a: je 0x1409cc04f
1409cbc70: mov r15, rdx
1409cbc73: movabs rdx, 0xa68561b396784eaa
1409cbc7d: movabs r8, 0xf5fd74e6faf4ed14
1409cbc87: mov rcx, r14
1409cbc8a: call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id
1409cbc8f: test rax, rax
1409cbc92: je 0x1409cc041
1409cbc98: cmp qword ptr [rdi + 0x218], rdx
1409cbc9f: jbe 0x1409cc041
1409cbca5: mov rax, qword ptr [rdi + 0x210]
1409cbcac: mov rcx, qword ptr [rax + rdx*8]
1409cbcb0: test rcx, rcx
1409cbcb3: je 0x1409cc041
1409cbcb9: mov rax, qword ptr [rdi + 0x1e0]
1409cbcc0: not rcx
1409cbcc3: lea rcx, [rcx + rcx*2]
1409cbcc7: shl rcx, 5
1409cbccb: cmp qword ptr [rax + rcx + 0x20], 0
1409cbcd1: je 0x1409cc041
1409cbcd7: add rax, rcx
1409cbcda: mov rdx, qword ptr [rax + 0x10]
1409cbcde: mov eax, dword ptr [rdi + 0x5b0]
1409cbce4: lea r14, [rbp + 0xe74]
1409cbceb: mov rcx, r14
1409cbcee: mov r8b, 0x10
1409cbcf1: call 0x14093cf30 ; country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_texture
1409cbcf6: lea rcx, [rbp + 0xe78]
1409cbcfd: call 0x140bd5790 ; rhapsody::raw::texture::TextureDesc::extent_2d
1409cbd02: mov dword ptr [rbp + 0x990], 1
1409cbd0c: xorps xmm0, xmm0
1409cbd0f: movups xmmword ptr [rbp + 0x994], xmm0
1409cbd16: mov dword ptr [rbp + 0x9a4], 0
1409cbd20: mov qword ptr [rbp + 0x9a8], r14
1409cbd27: mov dword ptr [rbp + 0xd78], 2
1409cbd31: mov qword ptr [rsp + 0x40], rbx
1409cbd36: mov qword ptr [rsp + 0x38], rdi
1409cbd3b: mov qword ptr [rsp + 0x30], rax
1409cbd40: lea rax, [rbp + 0xd78]
1409cbd47: mov qword ptr [rsp + 0x28], rax
1409cbd4c: mov qword ptr [rsp + 0x20], 1
1409cbd55: lea rdx, [rip + 0x20f4ef4]
1409cbd5c: lea rcx, [rbp + 0x3b8]
1409cbd63: lea r9, [rbp + 0x990]
1409cbd6a: mov r8d, 0x11
1409cbd70: call 0x140867970 ; country_core::render::pass_builder::draw_stream::RasterPass::new_impl
1409cbd75: mov byte ptr [rbp + 0xecf], 1
1409cbd7c: call 0x140be6750 ; rhapsody::raw::shader::impl$7::default
1409cbd81: mov edi, eax
1409cbd83: mov byte ptr [rbp + 0xecf], 1
1409cbd8a: xor ecx, ecx
1409cbd8c: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
1409cbd91: test rax, rax
1409cbd94: je 0x1409cc05d
1409cbd9a: mov dword ptr [rbp + 0xa38], edi
1409cbda0: mov word ptr [rbp + 0xa40], 0
1409cbda9: mov dword ptr [rbp + 0xa3c], 0x3f800000
1409cbdb3: mov qword ptr [rbp + 0x990], 0
1409cbdbe: mov qword ptr [rbp + 0xa30], r15
1409cbdc5: mov qword ptr [rbp + 0x9a0], 0
1409cbdd0: mov qword ptr [rbp + 0x9a8], 4
1409cbddb: mov qword ptr [rbp + 0x9b0], 0
1409cbde6: movups xmm0, xmmword ptr [rax]
1409cbde9: inc qword ptr [rax]
1409cbdec: movups xmm1, xmmword ptr [rip + 0x20ca93d]
1409cbdf3: movups xmmword ptr [rbp + 0xa10], xmm1
1409cbdfa: movups xmm1, xmmword ptr [rip + 0x20ca91f]
1409cbe01: movups xmmword ptr [rbp + 0xa00], xmm1
1409cbe08: movups xmmword ptr [rbp + 0xa20], xmm0
1409cbe0f: movabs rax, 0x100000003
1409cbe19: mov qword ptr [rbp + 0x9d0], rax
1409cbe20: mov qword ptr [rbp + 0x9b8], 0
1409cbe2b: mov qword ptr [rbp + 0x9c0], 1
1409cbe36: mov qword ptr [rbp + 0x9c8], 0
1409cbe41: mov byte ptr [rbp + 0xecf], 1
1409cbe48: lea rcx, [rbp + 0x7a0]
1409cbe4f: lea rdx, [rbp + 0x990]
1409cbe56: lea r8, [rbp + 0x3b8]
1409cbe5d: xor r9d, r9d
1409cbe60: call 0x140905be0 ; country_core::render::pass_builder::draw_builder::DrawBuilder::new
1409cbe65: and byte ptr [rbp + 0x848], 0x73
1409cbe6c: lea rdi, [rbp + 0x990]
1409cbe73: lea rdx, [rbp + 0x7a0]
1409cbe7a: mov r8d, 0xf8
1409cbe80: mov rcx, rdi
1409cbe83: call 0x142923c00 ; ?
1409cbe88: mov byte ptr [rbp + 0xecf], 1
1409cbe8f: lea rcx, [rbp + 0xd78]
1409cbe96: movabs r8, 0x92930b0d61fef16e
1409cbea0: mov rdx, rdi
1409cbea3: mov r9, rsi
1409cbea6: call 0x140906400 ; country_core::render::pass_builder::draw_builder::DrawBuilder::bind_external_storage_buffer
1409cbeab: mov byte ptr [rbp + 0x898], 9
1409cbeb2: mov dword ptr [rbp + 0x990], 2
1409cbebc: mov qword ptr [rsp + 0x20], rdi
1409cbec1: lea rdi, [rbp + 0x898]
1409cbec8: lea rdx, [rbp + 0xd78]
1409cbecf: movabs r8, 0x61b08b5e0470c5f2
1409cbed9: mov rcx, rdi
1409cbedc: mov r9b, 0x1d
1409cbedf: call 0x140908c50 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>
1409cbee4: lea rbx, [rbp + 0x990]
1409cbeeb: lea rdx, [rbp + 0xd78]
1409cbef2: mov r8d, 0xf8
1409cbef8: mov rcx, rbx
1409cbefb: call 0x142923c00 ; ?
1409cbf00: mov byte ptr [rbp + 0xece], 0xa
1409cbf07: mov dword ptr [rbp + 0x898], 2
1409cbf11: mov qword ptr [rsp + 0x20], rdi
1409cbf16: lea rcx, [rbp + 0xece]
1409cbf1d: movabs r8, 0x297d1e4cdae8f657
1409cbf27: mov rdx, rbx
1409cbf2a: mov r9b, 0x1d
1409cbf2d: call 0x140908c50 ; country_core::render::pass_builder::draw_builder::impl$3::bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>
1409cbf32: lea rdx, [rbp + 0x990]
1409cbf39: mov r8d, 0xf8
1409cbf3f: mov rcx, rdi
1409cbf42: call 0x142923c00 ; ?
1409cbf47: mov edx, dword ptr [rsi + 0x28]
1409cbf4a: mov byte ptr [rbp + 0xecf], 1
1409cbf51: mov rcx, rdi
1409cbf54: call 0x140906a90 ; country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
1409cbf59: lea rsi, [rbp + 0x990]
1409cbf60: lea rdx, [rbp + 0x3b8]
1409cbf67: mov r8d, 0x3e8
1409cbf6d: mov rcx, rsi
1409cbf70: call 0x142923c00 ; ?
1409cbf75: mov byte ptr [rbp + 0xecf], 0
1409cbf7c: mov rcx, rsi
1409cbf7f: call 0x140868370 ; country_core::render::pass_builder::draw_stream::RasterPass::render
1409cbf84: mov byte ptr [rip + 0x2c68d6d], 1
1409cbf8b: cmp qword ptr [rbp + 0xea8], 0
1409cbf93: je 0x1409cb9e2
1409cbf99: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1409cbf9e: mov rcx, rax
1409cbfa1: mov rax, qword ptr [rax]
1409cbfa4: cmp rax, 1
1409cbfa8: jne 0x1409cbfe1
1409cbfaa: add rcx, 8
1409cbfae: cmp qword ptr [rcx], 0
1409cbfb2: jne 0x1409cc01b
1409cbfb4: mov qword ptr [rcx], 0xffffffffffffffff
1409cbfbb: mov qword ptr [rbp + 0xec0], rcx
1409cbfc2: add rcx, 8
1409cbfc6: mov rdx, qword ptr [rbp + 0xeb0]
1409cbfcd: call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
1409cbfd2: mov rax, qword ptr [rbp + 0xec0]
1409cbfd9: inc qword ptr [rax]
1409cbfdc: jmp 0x1409cb9e2
1409cbfe1: test rax, rax
1409cbfe4: jne 0x1409cc00f
1409cbfe6: xor edx, edx
1409cbfe8: call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1409cbfed: mov rcx, rax
1409cbff0: test rax, rax
1409cbff3: jne 0x1409cbfae
1409cbff5: jmp 0x1409cc00f
1409cbff7: test rax, rax
1409cbffa: jne 0x1409cc00f
1409cbffc: xor edx, edx
1409cbffe: call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1409cc003: mov rcx, rax
1409cc006: test rax, rax
1409cc009: jne 0x1409cbb86
1409cc00f: lea rcx, [rip + 0x226ea8a]
1409cc016: call 0x1428d80d0 ; std::thread::local::panic_access_error
1409cc01b: lea rcx, [rip + 0x226eb4e]
1409cc022: call 0x1428d9250 ; core::cell::panic_already_borrowed
1409cc027: lea rcx, [rip + 0x20f4c3a]
1409cc02e: call 0x1428d9310 ; core::option::unwrap_failed
1409cc033: lea rcx, [rip + 0x20f4bb6]
1409cc03a: call 0x1428d9310 ; core::option::unwrap_failed
1409cc03f: jmp 0x1409cc070
1409cc041: lea rcx, [rip + 0x20f4bf0]
1409cc048: call 0x1428d9310 ; core::option::unwrap_failed
1409cc04d: jmp 0x1409cc070
1409cc04f: lea rcx, [rip + 0x20f4bca]
1409cc056: call 0x1428d9310 ; core::option::unwrap_failed
1409cc05b: jmp 0x1409cc070
1409cc05d: mov byte ptr [rbp + 0xecf], 1
1409cc064: lea rcx, [rip + 0x226ea35]
1409cc06b: call 0x1428d80d0 ; std::thread::local::panic_access_error
1409cc070: ud2 
1409cc072: nop word ptr cs:[rax + rax]
1409cc080: mov qword ptr [rsp + 0x10], rdx
1409cc085: push rbp
1409cc086: push r15
1409cc088: push r14
1409cc08a: push r13
1409cc08c: push r12
1409cc08e: push rsi
1409cc08f: push rdi
1409cc090: push rbx
1409cc091: sub rsp, 0x48
1409cc095: lea rbp, [rdx + 0x80]
1409cc09c: mov rax, qword ptr [rbp + 0xeb8]
1409cc0a3: inc qword ptr [rax]
1409cc0a6: add rsp, 0x48
1409cc0aa: pop rbx
1409cc0ab: pop rdi
1409cc0ac: pop rsi
1409cc0ad: pop r12
1409cc0af: pop r13
1409cc0b1: pop r14
1409cc0b3: pop r15
1409cc0b5: pop rbp
1409cc0b6: ret 
1409cc0b7: nop word ptr [rax + rax]
1409cc0c0: mov qword ptr [rsp + 0x10], rdx
1409cc0c5: push rbp
1409cc0c6: push r15
1409cc0c8: push r14
1409cc0ca: push r13
1409cc0cc: push r12
1409cc0ce: push rsi
1409cc0cf: push rdi
1409cc0d0: push rbx
1409cc0d1: sub rsp, 0x48
1409cc0d5: lea rbp, [rdx + 0x80]
1409cc0dc: cmp qword ptr [rbp + 0xea8], 0
1409cc0e4: je 0x1409cc115
1409cc0e6: lea rax, [rbp + 0xeb0]
1409cc0ed: mov qword ptr [rbp + 0xea0], rax
1409cc0f4: lea rax, [rbp + 0xea0]
1409cc0fb: mov qword ptr [rbp + 0xe98], rax
1409cc102: lea rcx, [rip + 0x20f4797]
1409cc109: lea rdx, [rbp + 0xe98]
1409cc110: call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
1409cc115: nop 
1409cc116: add rsp, 0x48
1409cc11a: pop rbx
1409cc11b: pop rdi
1409cc11c: pop rsi
1409cc11d: pop r12
1409cc11f: pop r13
1409cc121: pop r14
1409cc123: pop r15
1409cc125: pop rbp
1409cc126: ret 
1409cc127: nop word ptr [rax + rax]
1409cc130: mov qword ptr [rsp + 0x10], rdx
1409cc135: push rbp
1409cc136: push r15
1409cc138: push r14
1409cc13a: push r13
1409cc13c: push r12
1409cc13e: push rsi
1409cc13f: push rdi
1409cc140: push rbx
1409cc141: sub rsp, 0x48
1409cc145: lea rbp, [rdx + 0x80]
1409cc14c: mov rax, qword ptr [rbp + 0xec0]
1409cc153: inc qword ptr [rax]
1409cc156: add rsp, 0x48
1409cc15a: pop rbx
1409cc15b: pop rdi
1409cc15c: pop rsi
1409cc15d: pop r12
1409cc15f: pop r13
1409cc161: pop r14
1409cc163: pop r15
1409cc165: pop rbp
1409cc166: ret 
1409cc167: nop word ptr [rax + rax]
1409cc170: mov qword ptr [rsp + 0x10], rdx
1409cc175: push rbp
1409cc176: push r15
1409cc178: push r14
1409cc17a: push r13
1409cc17c: push r12
1409cc17e: push rsi
1409cc17f: push rdi
1409cc180: push rbx
1409cc181: sub rsp, 0x48
1409cc185: lea rbp, [rdx + 0x80]
1409cc18c: cmp byte ptr [rbp + 0xecf], 0
1409cc193: je 0x1409cc1b3
1409cc195: lea rcx, [rbp + 0x3b8]
1409cc19c: call 0x1408923c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_stream::RasterPass>
1409cc1a1: nop 
1409cc1a2: add rsp, 0x48
1409cc1a6: pop rbx
1409cc1a7: pop rdi
1409cc1a8: pop rsi
1409cc1a9: pop r12
1409cc1ab: pop r13
1409cc1ad: pop r14
1409cc1af: pop r15
1409cc1b1: pop rbp
1409cc1b2: ret 
1409cc1b3: add rsp, 0x48
1409cc1b7: pop rbx
1409cc1b8: pop rdi
1409cc1b9: pop rsi
1409cc1ba: pop r12
1409cc1bc: pop r13
1409cc1be: pop r14
1409cc1c0: pop r15
1409cc1c2: pop rbp
1409cc1c3: ret 
1409cc1c4: nop word ptr cs:[rax + rax]
1409cc1d0: mov qword ptr [rsp + 0x10], rdx
1409cc1d5: push rbp
1409cc1d6: push r15
1409cc1d8: push r14
1409cc1da: push r13
1409cc1dc: push r12
1409cc1de: push rsi
1409cc1df: push rdi
1409cc1e0: push rbx
1409cc1e1: sub rsp, 0x48
1409cc1e5: lea rbp, [rdx + 0x80]
1409cc1ec: mov byte ptr [rbp + 0xecf], 1
1409cc1f3: lea rcx, [rbp + 0xd78]
1409cc1fa: call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409cc1ff: mov byte ptr [rbp + 0xecf], 1
1409cc206: add rsp, 0x48
1409cc20a: pop rbx
1409cc20b: pop rdi
1409cc20c: pop rsi
1409cc20d: pop r12
1409cc20f: pop r13
1409cc211: pop r14
1409cc213: pop r15
1409cc215: pop rbp
1409cc216: ret 
1409cc217: nop word ptr [rax + rax]
1409cc220: mov qword ptr [rsp + 0x10], rdx
1409cc225: push rbp
1409cc226: push r15
1409cc228: push r14
1409cc22a: push r13
1409cc22c: push r12
1409cc22e: push rsi
1409cc22f: push rdi
1409cc230: push rbx
1409cc231: sub rsp, 0x48
1409cc235: lea rbp, [rdx + 0x80]
1409cc23c: mov byte ptr [rbp + 0xecf], 1
1409cc243: lea rcx, [rbp + 0x990]
1409cc24a: call 0x1408935c0 ; core::ptr::drop_in_place<country_core::render::pass_builder::draw_builder::DrawBuilder>
1409cc24f: mov byte ptr [rbp + 0xecf], 1
1409cc256: add rsp, 0x48
1409cc25a: pop rbx
1409cc25b: pop rdi
1409cc25c: pop rsi
1409cc25d: pop r12
1409cc25f: pop r13
1409cc261: pop r14
1409cc263: pop r15
1409cc265: pop rbp
1409cc266: ret 
