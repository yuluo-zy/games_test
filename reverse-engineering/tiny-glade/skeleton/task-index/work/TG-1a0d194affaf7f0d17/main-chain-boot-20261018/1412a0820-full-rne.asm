1412a0820: push rbp
1412a0821: push r15
1412a0823: push r14
1412a0825: push r13
1412a0827: push r12
1412a0829: push rsi
1412a082a: push rdi
1412a082b: push rbx
1412a082c: sub rsp, 0x238
1412a0833: lea rbp, [rsp + 0x80]
1412a083b: movdqa xmmword ptr [rbp + 0x1a0], xmm11
1412a0844: movdqa xmmword ptr [rbp + 0x190], xmm10
1412a084d: movdqa xmmword ptr [rbp + 0x180], xmm9
1412a0856: movaps xmmword ptr [rbp + 0x170], xmm8
1412a085e: movaps xmmword ptr [rbp + 0x160], xmm7
1412a0865: movaps xmmword ptr [rbp + 0x150], xmm6
1412a086c: mov qword ptr [rbp + 0x148], 0xfffffffffffffffe
1412a0877: mov rdi, r9
1412a087a: mov rsi, r8
1412a087d: mov rbx, rdx
1412a0880: mov r14, rcx
1412a0883: xor ecx, ecx
1412a0885: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
1412a088a: test rax, rax
1412a088d: je 0x1412a16e1
1412a0893: movdqu xmm0, xmmword ptr [rax]
1412a0897: inc qword ptr [rax]
1412a089a: movdqa xmmword ptr [rbp + 0x20], xmm0
1412a089f: movdqu xmm6, xmmword ptr [rip + 0x19cb659]
1412a08a7: movdqa xmmword ptr [rbp], xmm6
1412a08ac: movdqu xmm7, xmmword ptr [rip + 0x19cb65c]
1412a08b4: movdqa xmmword ptr [rbp + 0x10], xmm7
1412a08b9: xor ecx, ecx
1412a08bb: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
1412a08c0: mov qword ptr [rbp + 0x70], rbx
1412a08c4: mov qword ptr [rbp - 0x48], rsi
1412a08c8: test rax, rax
1412a08cb: je 0x1412a16ed
1412a08d1: movdqu xmm0, xmmword ptr [rax]
1412a08d5: inc qword ptr [rax]
1412a08d8: movdqa xmmword ptr [rbp + 0x50], xmm0
1412a08dd: movdqa xmmword ptr [rbp + 0x30], xmm6
1412a08e2: movdqa xmmword ptr [rbp + 0x40], xmm7
1412a08e7: mov qword ptr [rbp - 0x40], rdi
1412a08eb: mov rax, qword ptr [rdi]
1412a08ee: mov qword ptr [rbp - 0x38], rax
1412a08f2: mov rsi, qword ptr [rax + 0x38]
1412a08f6: lea rbx, [rsi + 0x10]
1412a08fa: mov cl, 1
1412a08fc: xor eax, eax
1412a08fe: lock cmpxchg byte ptr [rsi + 0x10], cl
1412a0903: jne 0x1412a162e
1412a0909: mov rdi, qword ptr [rip + 0x238bb10]
1412a0910: mov rax, qword ptr [rdi]
1412a0913: shl rax, 1
1412a0916: test rax, rax
1412a0919: jne 0x1412a1642
1412a091f: mov dword ptr [rbp + 0x130], 0
1412a0929: mov qword ptr [rbp + 0x138], rbx
1412a0930: movzx eax, byte ptr [rsi + 0x11]
1412a0934: test al, al
1412a0936: jne 0x1412a1669
1412a093c: lea rax, [rsi + 0x18]
1412a0940: mov rcx, qword ptr [rsi + 0x18]
1412a0944: mov rdx, qword ptr [rsi + 0x20]
1412a0948: add rdx, rcx
1412a094b: inc rdx
1412a094e: movdqa xmm0, xmmword ptr [rcx]
1412a0952: pmovmskb r8d, xmm0
1412a0957: not r8d
1412a095a: mov r9, qword ptr [rsi + 0x30]
1412a095e: movups xmm0, xmmword ptr [rsi + 0x18]
1412a0962: movdqu xmm1, xmmword ptr [rsi + 0x28]
1412a0967: movdqu xmmword ptr [rbp + 0xb8], xmm1
1412a096f: movups xmmword ptr [rbp + 0xa8], xmm0
1412a0976: movups xmm0, xmmword ptr [rip + 0x1854233]
1412a097d: movups xmmword ptr [rsi + 0x18], xmm0
1412a0981: movdqu xmm0, xmmword ptr [rip + 0x1854237]
1412a0989: movdqu xmmword ptr [rsi + 0x28], xmm0
1412a098e: mov qword ptr [rbp + 0x80], rcx
1412a0995: add rcx, 0x10
1412a0999: mov qword ptr [rbp + 0x88], rcx
1412a09a0: mov qword ptr [rbp + 0x90], rdx
1412a09a7: mov word ptr [rbp + 0x98], r8w
1412a09af: mov ecx, dword ptr [rbp + 0x6a]
1412a09b2: mov dword ptr [rbp + 0x9a], ecx
1412a09b8: movzx ecx, word ptr [rbp + 0x6e]
1412a09bc: mov word ptr [rbp + 0x9e], cx
1412a09c3: mov qword ptr [rbp + 0xa0], r9
1412a09ca: mov qword ptr [rbp + 0xc8], rax
1412a09d1: lea r8, [rip + 0x19895e8]
1412a09d8: lea rcx, [rbp - 0x30]
1412a09dc: lea rdx, [rbp + 0x80]
1412a09e3: call 0x14130b3d0 ; alloc::vec::spec_from_iter::impl$0::from_iter<country_core::render::nani::gpu_world::GpuWorldSubsetKind,std::collections::hash::set::Drain<country_core::render::nani::gpu_world::GpuWorldSubsetKind> >
1412a09e8: cmp byte ptr [rbp + 0x130], 0
1412a09ef: mov rbx, qword ptr [rbp + 0x138]
1412a09f6: jne 0x1412a0a07
1412a09f8: mov rax, qword ptr [rdi]
1412a09fb: shl rax, 1
1412a09fe: test rax, rax
1412a0a01: jne 0x1412a16c4
1412a0a07: xor eax, eax
1412a0a09: xchg byte ptr [rbx], al
1412a0a0b: cmp al, 2
1412a0a0d: je 0x1412a16b0
1412a0a13: mov rax, qword ptr [rbp - 0x30]
1412a0a17: mov rcx, qword ptr [rbp - 0x28]
1412a0a1b: mov rdx, qword ptr [rbp - 0x20]
1412a0a1f: add rdx, rcx
1412a0a22: mov qword ptr [rbp + 0x80], rcx
1412a0a29: mov qword ptr [rbp + 0x88], rcx
1412a0a30: mov qword ptr [rbp + 0x90], rax
1412a0a37: mov qword ptr [rbp + 0x98], rdx
1412a0a3e: mov byte ptr [rbp + 0x147], 1
1412a0a45: lea rcx, [rbp + 0x30]
1412a0a49: lea rdx, [rbp + 0x80]
1412a0a50: call 0x1415232f0 ; hashbrown::map::impl$82::extend<country_core::render::nani::gpu_world::GpuWorldSubsetKind,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::nani::gpu_world::GpuWorldSubsetKind,alloc::alloc::Global>,hashbrown::set::impl$10::extend::closure_env$0<country_core::render::nani::gpu_world::GpuWorldSubsetKind,std::hash::random::RandomState,alloc::alloc::Global,alloc::vec::Vec<country_core::render::nani::gpu_world::GpuWorldSubsetKind,alloc::alloc::Global> > > >
1412a0a55: mov rax, qword ptr [rbp + 0x220]
1412a0a5c: mov r10, qword ptr [rax]
1412a0a5f: mov rax, qword ptr [rax + 8]
1412a0a63: mov rcx, qword ptr [r10]
1412a0a66: xor ebx, ebx
1412a0a68: mov rdx, rcx
1412a0a6b: sub rdx, qword ptr [rax + 0x18]
1412a0a6f: cmovb rdx, rbx
1412a0a73: sub rcx, qword ptr [rax + 0x38]
1412a0a77: cmovb rcx, rbx
1412a0a7b: imul r15, rdx, 0x38
1412a0a7f: add r15, qword ptr [rax + 8]
1412a0a83: mov r8, qword ptr [rax + 0x10]
1412a0a87: sub r8, rdx
1412a0a8a: cmovb r8, rbx
1412a0a8e: mov rdx, qword ptr [rax + 0x30]
1412a0a92: mov r9d, 8
1412a0a98: cmovb r15, r9
1412a0a9c: imul rdi, rcx, 0x38
1412a0aa0: add rdi, qword ptr [rax + 0x28]
1412a0aa4: sub rdx, rcx
1412a0aa7: cmovb rdx, rbx
1412a0aab: cmovb rdi, r9
1412a0aaf: mov rax, qword ptr [rax + 0x40]
1412a0ab3: sub rax, rdx
1412a0ab6: sub rax, r8
1412a0ab9: mov qword ptr [rbp + 0x130], r10
1412a0ac0: mov qword ptr [r10], rax
1412a0ac3: imul rax, r8, 0x38
1412a0ac7: add rax, r15
1412a0aca: mov qword ptr [rbp + 0x120], rax
1412a0ad1: imul rax, rdx, 0x38
1412a0ad5: add rax, rdi
1412a0ad8: mov qword ptr [rbp + 0x128], rax
1412a0adf: mov rax, qword ptr [r14]
1412a0ae2: mov qword ptr [rbp + 0x108], rax
1412a0ae9: jmp 0x1412a0af7 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a0aeb: nop dword ptr [rax + rax]
1412a0af0: mov rdx, qword ptr [rbp + 0x138]
1412a0af7: test rbx, rbx
1412a0afa: je 0x1412a0be0
1412a0b00: test rdx, rdx
1412a0b03: je 0x1412a0be0
1412a0b09: mov qword ptr [rbp + 0x138], rdx
1412a0b10: test si, si
1412a0b13: jne 0x1412a0b3f
1412a0b15: nop word ptr cs:[rax + rax]
1412a0b20: movdqa xmm0, xmmword ptr [r12]
1412a0b26: pmovmskb esi, xmm0
1412a0b2a: add rbx, -0x100
1412a0b31: add r12, 0x10
1412a0b35: cmp esi, 0xffff
1412a0b3b: je 0x1412a0b20
1412a0b3d: not esi
1412a0b3f: tzcnt eax, esi
1412a0b43: shl eax, 4
1412a0b46: mov r14, rbx
1412a0b49: sub r14, rax
1412a0b4c: mov rdx, qword ptr [r14 - 0x10]
1412a0b50: mov r8, qword ptr [r14 - 8]
1412a0b54: lock inc qword ptr [rdx]
1412a0b58: jle 0x1412a16f9
1412a0b5e: mov byte ptr [rbp + 0x147], 1
1412a0b65: mov rcx, rbp
1412a0b68: call 0x14152b320 ; hashbrown::map::HashMap::insert<asset::prefab::PrefabName,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global>
1412a0b6d: add r14, -0x10
1412a0b71: mov byte ptr [rbp + 0x147], 1
1412a0b78: mov rcx, qword ptr [rbp + 0x108]
1412a0b7f: mov rdx, r14
1412a0b82: call 0x141552d70 ; country_core::resources::prefab::PrefabLibrary::get<ref$<asset::prefab::PrefabName> >
1412a0b87: lea ecx, [rsi - 1]
1412a0b8a: and ecx, esi
1412a0b8c: mov esi, ecx
1412a0b8e: dec qword ptr [rbp + 0x138]
1412a0b95: mov r14, qword ptr [rax + 8]
1412a0b99: imul r13, qword ptr [rax + 0x10], 0x98
1412a0ba1: add r13, r14
1412a0ba4: nop word ptr cs:[rax + rax]
1412a0bb0: cmp r14, r13
1412a0bb3: je 0x1412a0af0
1412a0bb9: movzx edx, byte ptr [r14 + 0x90]
1412a0bc1: add r14, 0x98
1412a0bc8: mov byte ptr [rbp + 0x147], 1
1412a0bcf: lea rcx, [rbp + 0x30]
1412a0bd3: call 0x1415265f0 ; hashbrown::map::HashMap::insert<country_core::render::nani::gpu_world::GpuWorldSubsetKind,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global>
1412a0bd8: jmp 0x1412a0bb0 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a0bda: nop word ptr [rax + rax]
1412a0be0: test r15, r15
1412a0be3: je 0x1412a0c00
1412a0be5: lea rax, [r15 + 0x38]
1412a0be9: cmp r15, qword ptr [rbp + 0x120]
1412a0bf0: je 0x1412a0c00
1412a0bf2: mov rcx, rdi
1412a0bf5: mov rdi, r15
1412a0bf8: jmp 0x1412a0c14 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a0bfa: nop word ptr [rax + rax]
1412a0c00: test rdi, rdi
1412a0c03: je 0x1412a0c44
1412a0c05: cmp rdi, qword ptr [rbp + 0x128]
1412a0c0c: je 0x1412a0c44
1412a0c0e: lea rcx, [rdi + 0x38]
1412a0c12: xor eax, eax
1412a0c14: mov rdx, qword ptr [rbp + 0x130]
1412a0c1b: inc qword ptr [rdx]
1412a0c1e: mov rbx, qword ptr [rdi]
1412a0c21: mov rdx, qword ptr [rdi + 0x18]
1412a0c25: movdqa xmm0, xmmword ptr [rbx]
1412a0c29: pmovmskb esi, xmm0
1412a0c2d: not esi
1412a0c2f: lea r12, [rbx + 0x10]
1412a0c33: mov rdi, rcx
1412a0c36: mov r15, rax
1412a0c39: test rdx, rdx
1412a0c3c: jne 0x1412a0b09
1412a0c42: jmp 0x1412a0be0 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a0c44: mov rsi, qword ptr [rbp + 0x48]
1412a0c48: mov rax, qword ptr [rbp + 0x18]
1412a0c4c: or rax, rsi
1412a0c4f: jne 0x1412a0c8d
1412a0c51: mov rdx, qword ptr [rbp + 0x38]
1412a0c55: test rdx, rdx
1412a0c58: je 0x1412a15d4
1412a0c5e: mov rax, rdx
1412a0c61: and rax, 0xfffffffffffffff0
1412a0c65: add rdx, rax
1412a0c68: add rdx, 0x21
1412a0c6c: je 0x1412a15d4
1412a0c72: mov rcx, qword ptr [rbp + 0x30]
1412a0c76: sub rcx, rax
1412a0c79: add rcx, -0x10
1412a0c7d: mov r8d, 0x10
1412a0c83: call 0x140613c20
1412a0c88: jmp 0x1412a15d4 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a0c8d: mov rdi, qword ptr [rbp + 0x30]
1412a0c91: movdqa xmm0, xmmword ptr [rdi]
1412a0c95: pmovmskb eax, xmm0
1412a0c99: not eax
1412a0c9b: lea rbx, [rdi + 0x10]
1412a0c9f: mov rdx, qword ptr [rbp + 0x70]
1412a0ca3: mov ecx, dword ptr [rdx + 0x1c]
1412a0ca6: mov dword ptr [rbp + 0x120], ecx
1412a0cac: mov rcx, qword ptr [rdx]
1412a0caf: mov qword ptr [rbp + 0x118], rcx
1412a0cb6: mov rcx, qword ptr [rdx + 0x10]
1412a0cba: mov qword ptr [rbp + 0x128], rcx
1412a0cc1: jmp 0x1412a0d07 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a0cc3: nop word ptr cs:[rax + rax]
1412a0cd0: tzcnt ecx, eax
1412a0cd4: not rcx
1412a0cd7: lea r14d, [rax - 1]
1412a0cdb: and r14d, eax
1412a0cde: mov eax, dword ptr [rbp + 0x120]
1412a0ce4: mov rdx, qword ptr [rbp + 0x128]
1412a0ceb: mov dword ptr [rdx], eax
1412a0ced: movzx edx, byte ptr [rcx + rdi]
1412a0cf1: mov byte ptr [rbp + 0x147], 1
1412a0cf8: mov rcx, qword ptr [rbp + 0x118]
1412a0cff: call 0x140ae6880 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::clear_world_subset_atlas
1412a0d04: mov eax, r14d
1412a0d07: sub rsi, 1
1412a0d0b: jb 0x1412a0d3b
1412a0d0d: test ax, ax
1412a0d10: jne 0x1412a0cd0
1412a0d12: nop word ptr cs:[rax + rax]
1412a0d20: movdqa xmm0, xmmword ptr [rbx]
1412a0d24: pmovmskb eax, xmm0
1412a0d28: add rdi, -0x10
1412a0d2c: add rbx, 0x10
1412a0d30: cmp eax, 0xffff
1412a0d35: je 0x1412a0d20
1412a0d37: not eax
1412a0d39: jmp 0x1412a0cd0 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a0d3b: mov rsi, qword ptr [rbp]
1412a0d3f: mov rdi, qword ptr [rbp + 0x18]
1412a0d43: movdqa xmm0, xmmword ptr [rsi]
1412a0d47: pmovmskb eax, xmm0
1412a0d4b: not eax
1412a0d4d: lea rbx, [rsi + 0x10]
1412a0d51: jmp 0x1412a0d9d ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a0d53: nop word ptr cs:[rax + rax]
1412a0d60: tzcnt ecx, eax
1412a0d64: shl ecx, 4
1412a0d67: mov rdx, rsi
1412a0d6a: sub rdx, rcx
1412a0d6d: add rdx, -0x10
1412a0d71: lea r14d, [rax - 1]
1412a0d75: and r14d, eax
1412a0d78: mov eax, dword ptr [rbp + 0x120]
1412a0d7e: mov rcx, qword ptr [rbp + 0x128]
1412a0d85: mov dword ptr [rcx], eax
1412a0d87: mov byte ptr [rbp + 0x147], 1
1412a0d8e: mov rcx, qword ptr [rbp + 0x118]
1412a0d95: call 0x140a37e50 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::clear_atlas_by_name<ref$<asset::prefab::PrefabName> >
1412a0d9a: mov eax, r14d
1412a0d9d: sub rdi, 1
1412a0da1: jb 0x1412a0dce
1412a0da3: test ax, ax
1412a0da6: jne 0x1412a0d60
1412a0da8: nop dword ptr [rax + rax]
1412a0db0: movdqa xmm0, xmmword ptr [rbx]
1412a0db4: pmovmskb eax, xmm0
1412a0db8: add rsi, -0x100
1412a0dbf: add rbx, 0x10
1412a0dc3: cmp eax, 0xffff
1412a0dc8: je 0x1412a0db0
1412a0dca: not eax
1412a0dcc: jmp 0x1412a0d60 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a0dce: mov rax, qword ptr [rbp + 0x108]
1412a0dd5: mov rdx, qword ptr [rax]
1412a0dd8: mov rcx, qword ptr [rax + 0x18]
1412a0ddc: movdqa xmm0, xmmword ptr [rdx]
1412a0de0: pmovmskb eax, xmm0
1412a0de4: not eax
1412a0de6: lea r8, [rdx + 0x10]
1412a0dea: movdqa xmm6, xmmword ptr [rip + 0x172c28e]
1412a0df2: movdqa xmm7, xmmword ptr [rip + 0x172c296]
1412a0dfa: xorps xmm8, xmm8
1412a0dfe: pcmpeqd xmm9, xmm9
1412a0e03: jmp 0x1412a0e22 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a0e05: nop word ptr cs:[rax + rax]
1412a0e10: mov eax, dword ptr [rbp + 0x114]
1412a0e16: mov rdx, qword ptr [rbp - 0x58]
1412a0e1a: mov rcx, qword ptr [rbp - 0x50]
1412a0e1e: mov r8, qword ptr [rbp + 0x78]
1412a0e22: mov qword ptr [rbp + 0x78], r8
1412a0e26: sub rcx, 1
1412a0e2a: jb 0x1412a13ad
1412a0e30: test ax, ax
1412a0e33: mov qword ptr [rbp - 0x50], rcx
1412a0e37: je 0x1412a0e50
1412a0e39: lea ecx, [rax - 1]
1412a0e3c: and ecx, eax
1412a0e3e: mov dword ptr [rbp + 0x114], ecx
1412a0e44: jmp 0x1412a0e94 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a0e46: nop word ptr cs:[rax + rax]
1412a0e50: mov rcx, qword ptr [rbp + 0x78]
1412a0e54: nop word ptr cs:[rax + rax]
1412a0e60: movdqa xmm0, xmmword ptr [rcx]
1412a0e64: pmovmskb eax, xmm0
1412a0e68: add rdx, -0x280
1412a0e6f: add rcx, 0x10
1412a0e73: cmp eax, 0xffff
1412a0e78: je 0x1412a0e60
1412a0e7a: mov qword ptr [rbp + 0x78], rcx
1412a0e7e: mov ecx, 0xfffffffe
1412a0e83: sub ecx, eax
1412a0e85: not eax
1412a0e87: mov r8d, eax
1412a0e8a: and r8d, ecx
1412a0e8d: mov dword ptr [rbp + 0x114], r8d
1412a0e94: tzcnt eax, eax
1412a0e98: neg rax
1412a0e9b: lea rax, [rax + rax*4]
1412a0e9f: lea rcx, [rdx + rax*8]
1412a0ea3: mov qword ptr [rbp + 0x138], rcx
1412a0eaa: mov rdi, qword ptr [rdx + rax*8 - 0x10]
1412a0eaf: mov qword ptr [rbp - 0x58], rdx
1412a0eb3: imul rax, qword ptr [rdx + rax*8 - 8], 0x98
1412a0ebc: add rax, rdi
1412a0ebf: mov qword ptr [rbp + 0x130], rax
1412a0ec6: jmp 0x1412a0f80 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a0ecb: nop dword ptr [rax + rax]
1412a0ed0: and r13, rsi
1412a0ed3: movdqu xmm11, xmmword ptr [r12 + r13]
1412a0ed9: movdqa xmm0, xmm11
1412a0ede: pcmpeqb xmm0, xmm10
1412a0ee3: pmovmskb r15d, xmm0
1412a0ee8: test r15d, r15d
1412a0eeb: je 0x1412a0f60
1412a0eed: mov rax, qword ptr [rbp + 0x138]
1412a0ef4: mov rbx, qword ptr [rax - 0x20]
1412a0ef8: mov qword ptr [rbp + 0x108], rdx
1412a0eff: mov qword ptr [rbp + 0x70], rcx
1412a0f03: tzcnt ecx, r15d
1412a0f08: add rcx, r13
1412a0f0b: and rcx, rsi
1412a0f0e: shl rcx, 4
1412a0f12: mov rax, rdx
1412a0f15: sub rax, rcx
1412a0f18: cmp rbx, qword ptr [rax + 8]
1412a0f1c: jne 0x1412a0f44
1412a0f1e: mov rdx, qword ptr [rax]
1412a0f21: add rdx, 0x10
1412a0f25: mov rax, qword ptr [rbp + 0x138]
1412a0f2c: mov rcx, qword ptr [rax - 0x28]
1412a0f30: add rcx, 0x10
1412a0f34: mov r8, rbx
1412a0f37: call 0x142923bf0
1412a0f3c: test eax, eax
1412a0f3e: je 0x1412a1190
1412a0f44: lea eax, [r15 - 1]
1412a0f48: and ax, r15w
1412a0f4c: mov r15d, eax
1412a0f4f: mov rdx, qword ptr [rbp + 0x108]
1412a0f56: mov rcx, qword ptr [rbp + 0x70]
1412a0f5a: jne 0x1412a0f03
1412a0f5c: nop dword ptr [rax]
1412a0f60: pcmpeqb xmm11, xmm9
1412a0f65: pmovmskb eax, xmm11
1412a0f6a: test eax, eax
1412a0f6c: jne 0x1412a0f80
1412a0f6e: add r13, rcx
1412a0f71: add r13, 0x10
1412a0f75: add rcx, 0x10
1412a0f79: jmp 0x1412a0ed0 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a0f7e: nop 
1412a0f80: cmp rdi, qword ptr [rbp + 0x130]
1412a0f87: je 0x1412a0e10
1412a0f8d: mov r14, rdi
1412a0f90: add rdi, 0x98
1412a0f97: cmp qword ptr [rbp + 0x48], 0
1412a0f9c: je 0x1412a1200
1412a0fa2: movdqa xmm0, xmmword ptr [rbp + 0x50]
1412a0fa7: pshufd xmm1, xmm0, 0x44
1412a0fac: pxor xmm1, xmm6
1412a0fb0: movdqa xmmword ptr [rbp + 0x80], xmm1
1412a0fb8: pshufd xmm1, xmm0, 0xee
1412a0fbd: pxor xmm1, xmm7
1412a0fc1: movdqa xmmword ptr [rbp + 0x90], xmm1
1412a0fc9: movdqa xmmword ptr [rbp + 0xa0], xmm0
1412a0fd1: lea rax, [rbp + 0xb0]
1412a0fd8: movaps xmmword ptr [rax], xmm8
1412a0fdc: mov qword ptr [rax + 0x10], 0
1412a0fe4: movzx eax, byte ptr [r14 + 0x90]
1412a0fec: mov byte ptr [rbp - 0x30], al
1412a0fef: mov r8d, 1
1412a0ff5: lea rcx, [rbp + 0x80]
1412a0ffc: lea rdx, [rbp - 0x30]
1412a1000: call 0x140106160 ; std::hash::random::impl$4::write
1412a1005: mov r9, qword ptr [rbp + 0xb0]
1412a100c: shl r9, 0x38
1412a1010: or r9, qword ptr [rbp + 0xb8]
1412a1017: mov rcx, qword ptr [rbp + 0x90]
1412a101e: mov rax, qword ptr [rbp + 0x98]
1412a1025: xor rax, r9
1412a1028: mov r8, qword ptr [rbp + 0x80]
1412a102f: add r8, rcx
1412a1032: mov rdx, qword ptr [rbp + 0x88]
1412a1039: rol rcx, 0xd
1412a103d: add rdx, rax
1412a1040: xor rcx, r8
1412a1043: rol rax, 0x10
1412a1047: xor rax, rdx
1412a104a: rol r8, 0x20
1412a104e: add rdx, rcx
1412a1051: add r8, rax
1412a1054: rol rcx, 0x11
1412a1058: xor rcx, rdx
1412a105b: rol rax, 0x15
1412a105f: rol rdx, 0x20
1412a1063: xor rax, r8
1412a1066: xor r8, r9
1412a1069: xor rdx, 0xff
1412a1070: add r8, rcx
1412a1073: add rdx, rax
1412a1076: rol rcx, 0xd
1412a107a: xor rcx, r8
1412a107d: rol rax, 0x10
1412a1081: rol r8, 0x20
1412a1085: xor rax, rdx
1412a1088: add rdx, rcx
1412a108b: rol rcx, 0x11
1412a108f: add r8, rax
1412a1092: xor rcx, rdx
1412a1095: rol rax, 0x15
1412a1099: xor rax, r8
1412a109c: rol rdx, 0x20
1412a10a0: add r8, rcx
1412a10a3: add rdx, rax
1412a10a6: rol rcx, 0xd
1412a10aa: xor rcx, r8
1412a10ad: rol rax, 0x10
1412a10b1: rol r8, 0x20
1412a10b5: xor rax, rdx
1412a10b8: add rdx, rcx
1412a10bb: rol rcx, 0x11
1412a10bf: add r8, rax
1412a10c2: xor rcx, rdx
1412a10c5: rol rax, 0x15
1412a10c9: xor rax, r8
1412a10cc: rol rdx, 0x20
1412a10d0: add r8, rcx
1412a10d3: add rdx, rax
1412a10d6: rol rcx, 0xd
1412a10da: xor rcx, r8
1412a10dd: rol rax, 0x10
1412a10e1: xor rax, rdx
1412a10e4: add rdx, rcx
1412a10e7: rol rcx, 0x11
1412a10eb: rol rax, 0x15
1412a10ef: mov r8, rdx
1412a10f2: rol r8, 0x20
1412a10f6: xor r8, rcx
1412a10f9: xor r8, rax
1412a10fc: xor r8, rdx
1412a10ff: mov rdx, r8
1412a1102: shr rdx, 0x39
1412a1106: mov rax, qword ptr [rbp + 0x30]
1412a110a: mov rcx, qword ptr [rbp + 0x38]
1412a110e: movd xmm0, edx
1412a1112: punpcklbw xmm0, xmm0
1412a1116: pshuflw xmm0, xmm0, 0
1412a111b: pshufd xmm0, xmm0, 0
1412a1120: xor edx, edx
1412a1122: and r8, rcx
1412a1125: movdqu xmm1, xmmword ptr [rax + r8]
1412a112b: movdqa xmm2, xmm1
1412a112f: pcmpeqb xmm2, xmm0
1412a1133: pmovmskb r9d, xmm2
1412a1138: test r9d, r9d
1412a113b: je 0x1412a1170
1412a113d: movzx r10d, byte ptr [r14 + 0x90]
1412a1145: tzcnt r11d, r9d
1412a114a: add r11, r8
1412a114d: and r11, rcx
1412a1150: not r11
1412a1153: cmp r10b, byte ptr [r11 + rax]
1412a1157: je 0x1412a1190
1412a1159: lea r11d, [r9 - 1]
1412a115d: and r11w, r9w
1412a1161: mov r9d, r11d
1412a1164: jne 0x1412a1145
1412a1166: nop word ptr cs:[rax + rax]
1412a1170: pcmpeqb xmm1, xmm9
1412a1175: pmovmskb r9d, xmm1
1412a117a: test r9d, r9d
1412a117d: jne 0x1412a1200
1412a1183: add r8, rdx
1412a1186: add r8, 0x10
1412a118a: add rdx, 0x10
1412a118e: jmp 0x1412a1122 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a1190: mov rax, qword ptr [r14 + 0x88]
1412a1197: cmp qword ptr [rax + 0x20], 0
1412a119c: je 0x1412a0f80
1412a11a2: mov eax, dword ptr [rbp + 0x120]
1412a11a8: mov rcx, qword ptr [rbp + 0x128]
1412a11af: mov dword ptr [rcx], eax
1412a11b1: movzx edx, byte ptr [r14 + 0x90]
1412a11b9: mov rax, qword ptr [rbp + 0x138]
1412a11c0: mov r8, qword ptr [rax - 0x28]
1412a11c4: mov r9, qword ptr [rax - 0x20]
1412a11c8: lock inc qword ptr [r8]
1412a11cc: jle 0x1412a16f9
1412a11d2: mov rax, qword ptr [r14 + 0x88]
1412a11d9: add rax, 0x10
1412a11dd: mov byte ptr [rbp + 0x147], 1
1412a11e4: mov qword ptr [rsp + 0x20], rax
1412a11e9: mov rcx, qword ptr [rbp + 0x118]
1412a11f0: call 0x140a37760 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<asset::prefab::PrefabName>
1412a11f5: jmp 0x1412a0f80 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a11fa: nop word ptr [rax + rax]
1412a1200: cmp qword ptr [rbp + 0x18], 0
1412a1205: je 0x1412a0f80
1412a120b: movdqa xmm0, xmmword ptr [rbp + 0x20]
1412a1210: pshufd xmm1, xmm0, 0x44
1412a1215: pxor xmm1, xmm6
1412a1219: movdqa xmmword ptr [rbp + 0x80], xmm1
1412a1221: pshufd xmm1, xmm0, 0xee
1412a1226: pxor xmm1, xmm7
1412a122a: movdqa xmmword ptr [rbp + 0x90], xmm1
1412a1232: movdqa xmmword ptr [rbp + 0xa0], xmm0
1412a123a: lea rax, [rbp + 0xb0]
1412a1241: movaps xmmword ptr [rax], xmm8
1412a1245: mov qword ptr [rax + 0x10], 0
1412a124d: mov rax, qword ptr [rbp + 0x138]
1412a1254: mov rdx, qword ptr [rax - 0x28]
1412a1258: mov r8, qword ptr [rax - 0x20]
1412a125c: add rdx, 0x10
1412a1260: lea rsi, [rbp + 0x80]
1412a1267: mov rcx, rsi
1412a126a: call 0x140106160 ; std::hash::random::impl$4::write
1412a126f: mov byte ptr [rbp - 0x30], 0xff
1412a1273: mov r8d, 1
1412a1279: mov rcx, rsi
1412a127c: lea rdx, [rbp - 0x30]
1412a1280: call 0x140106160 ; std::hash::random::impl$4::write
1412a1285: mov r9, qword ptr [rbp + 0xb0]
1412a128c: shl r9, 0x38
1412a1290: or r9, qword ptr [rbp + 0xb8]
1412a1297: mov rcx, qword ptr [rbp + 0x90]
1412a129e: mov rax, qword ptr [rbp + 0x98]
1412a12a5: xor rax, r9
1412a12a8: mov r8, qword ptr [rbp + 0x80]
1412a12af: add r8, rcx
1412a12b2: mov rdx, qword ptr [rbp + 0x88]
1412a12b9: rol rcx, 0xd
1412a12bd: add rdx, rax
1412a12c0: xor rcx, r8
1412a12c3: rol rax, 0x10
1412a12c7: xor rax, rdx
1412a12ca: rol r8, 0x20
1412a12ce: add rdx, rcx
1412a12d1: add r8, rax
1412a12d4: rol rcx, 0x11
1412a12d8: xor rcx, rdx
1412a12db: rol rax, 0x15
1412a12df: rol rdx, 0x20
1412a12e3: xor rax, r8
1412a12e6: xor r8, r9
1412a12e9: xor rdx, 0xff
1412a12f0: add r8, rcx
1412a12f3: add rdx, rax
1412a12f6: rol rcx, 0xd
1412a12fa: xor rcx, r8
1412a12fd: rol rax, 0x10
1412a1301: rol r8, 0x20
1412a1305: xor rax, rdx
1412a1308: add rdx, rcx
1412a130b: rol rcx, 0x11
1412a130f: add r8, rax
1412a1312: xor rcx, rdx
1412a1315: rol rax, 0x15
1412a1319: xor rax, r8
1412a131c: rol rdx, 0x20
1412a1320: add r8, rcx
1412a1323: add rdx, rax
1412a1326: rol rcx, 0xd
1412a132a: xor rcx, r8
1412a132d: rol rax, 0x10
1412a1331: rol r8, 0x20
1412a1335: xor rax, rdx
1412a1338: add rdx, rcx
1412a133b: rol rcx, 0x11
1412a133f: add r8, rax
1412a1342: xor rcx, rdx
1412a1345: rol rax, 0x15
1412a1349: xor rax, r8
1412a134c: rol rdx, 0x20
1412a1350: add r8, rcx
1412a1353: add rdx, rax
1412a1356: rol rcx, 0xd
1412a135a: xor rcx, r8
1412a135d: rol rax, 0x10
1412a1361: xor rax, rdx
1412a1364: add rdx, rcx
1412a1367: rol rcx, 0x11
1412a136b: rol rax, 0x15
1412a136f: mov r13, rdx
1412a1372: rol r13, 0x20
1412a1376: xor r13, rcx
1412a1379: xor r13, rax
1412a137c: xor r13, rdx
1412a137f: mov rax, r13
1412a1382: shr rax, 0x39
1412a1386: mov r12, qword ptr [rbp]
1412a138a: mov rsi, qword ptr [rbp + 8]
1412a138e: movd xmm0, eax
1412a1392: punpcklbw xmm0, xmm0
1412a1396: pshuflw xmm0, xmm0, 0
1412a139b: pshufd xmm10, xmm0, 0
1412a13a1: lea rdx, [r12 - 0x10]
1412a13a6: xor ecx, ecx
1412a13a8: jmp 0x1412a0ed0 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a13ad: cmp qword ptr [rbp + 0x48], 0
1412a13b2: je 0x1412a14e5
1412a13b8: mov rax, qword ptr [rip + 0x238afe9]
1412a13bf: mov rax, qword ptr [rax]
1412a13c2: cmp rax, 3
1412a13c6: jb 0x1412a14ae
1412a13cc: lea rax, [rbp + 0x30]
1412a13d0: mov qword ptr [rbp - 0x30], rax
1412a13d4: lea rax, [rip - 0x9eec7b]
1412a13db: mov qword ptr [rbp - 0x28], rax
1412a13df: mov byte ptr [rbp + 0x147], 1
1412a13e6: lea rcx, [rip + 0x198911b]
1412a13ed: call 0x14067dd80 ; utils::swizzlers::impl$5::as_vec2
1412a13f2: movdqu xmm0, xmmword ptr [rax]
1412a13f6: mov eax, dword ptr [rax + 0x10]
1412a13f9: mov qword ptr [rbp + 0xb0], 3
1412a1404: lea rcx, [rip + 0x1989115]
1412a140b: mov qword ptr [rbp + 0xb8], rcx
1412a1412: mov qword ptr [rbp + 0xc0], 0x28
1412a141d: lea rdx, [rip + 0x19890d4]
1412a1424: mov qword ptr [rbp + 0xd0], rdx
1412a142b: mov qword ptr [rbp + 0xd8], 1
1412a1436: lea rdx, [rbp - 0x30]
1412a143a: mov qword ptr [rbp + 0xe0], rdx
1412a1441: mov qword ptr [rbp + 0xe8], 1
1412a144c: mov qword ptr [rbp + 0xf0], 0
1412a1457: mov qword ptr [rbp + 0x80], 0
1412a1462: mov qword ptr [rbp + 0x88], rcx
1412a1469: mov qword ptr [rbp + 0x90], 0x28
1412a1474: mov qword ptr [rbp + 0x98], 0
1412a147f: movdqu xmmword ptr [rbp + 0xa0], xmm0
1412a1487: mov dword ptr [rbp + 0xc8], 1
1412a1491: mov dword ptr [rbp + 0xcc], eax
1412a1497: mov byte ptr [rbp + 0x147], 1
1412a149e: lea rcx, [rbp + 0x6a]
1412a14a2: lea rdx, [rbp + 0x80]
1412a14a9: call 0x140d4a330 ; log::__private_api::impl$2::log
1412a14ae: mov eax, dword ptr [rbp + 0x120]
1412a14b4: mov rcx, qword ptr [rbp + 0x128]
1412a14bb: mov dword ptr [rcx], eax
1412a14bd: mov rcx, qword ptr [rbp - 0x40]
1412a14c1: mov eax, dword ptr [rcx + 0x1c]
1412a14c4: mov rcx, qword ptr [rcx + 0x10]
1412a14c8: mov dword ptr [rcx], eax
1412a14ca: mov byte ptr [rbp + 0x147], 1
1412a14d1: lea r8, [rbp + 0x30]
1412a14d5: mov rcx, qword ptr [rbp + 0x118]
1412a14dc: mov rdx, qword ptr [rbp - 0x38]
1412a14e0: call 0x1415f5b60 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1412a14e5: mov eax, dword ptr [rbp + 0x120]
1412a14eb: mov rcx, qword ptr [rbp + 0x128]
1412a14f2: mov dword ptr [rcx], eax
1412a14f4: mov rdx, qword ptr [rbp - 0x48]
1412a14f8: mov eax, dword ptr [rdx + 0x1c]
1412a14fb: mov rcx, qword ptr [rdx + 0x10]
1412a14ff: mov dword ptr [rcx], eax
1412a1501: mov rdx, qword ptr [rdx]
1412a1504: mov byte ptr [rbp + 0x147], 1
1412a150b: mov rcx, qword ptr [rbp + 0x118]
1412a1512: call 0x140ae6b90 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::register_meshes
1412a1517: movaps xmm0, xmmword ptr [rbp + 0x30]
1412a151b: movaps xmm1, xmmword ptr [rbp + 0x40]
1412a151f: movaps xmm2, xmmword ptr [rbp + 0x50]
1412a1523: movaps xmmword ptr [rbp - 0x10], xmm2
1412a1527: movaps xmmword ptr [rbp - 0x20], xmm1
1412a152b: movaps xmmword ptr [rbp - 0x30], xmm0
1412a152f: mov rdx, qword ptr [rbp + 0x228]
1412a1536: mov eax, dword ptr [rdx + 0x1c]
1412a1539: mov rcx, qword ptr [rdx + 0x10]
1412a153d: mov dword ptr [rcx], eax
1412a153f: mov rsi, qword ptr [rdx]
1412a1542: mov rax, qword ptr [rsi + 0x40]
1412a1546: mov qword ptr [rbp + 0xb0], rax
1412a154d: movdqa xmm0, xmmword ptr [rbp - 0x30]
1412a1552: movdqa xmm1, xmmword ptr [rbp - 0x20]
1412a1557: movdqa xmm2, xmmword ptr [rbp - 0x10]
1412a155c: movdqa xmmword ptr [rbp + 0xa0], xmm2
1412a1564: movdqa xmmword ptr [rbp + 0x90], xmm1
1412a156c: movdqa xmmword ptr [rbp + 0x80], xmm0
1412a1574: mov rdi, qword ptr [rsi + 0x30]
1412a1578: cmp rdi, qword ptr [rsi + 0x20]
1412a157c: jne 0x1412a158e
1412a157e: lea rcx, [rsi + 0x20]
1412a1582: lea rdx, [rip + 0x1998bbf]
1412a1589: call 0x1406319d0 ; alloc::raw_vec::RawVec::grow_one<utils::curve::Curve<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>,alloc::alloc::Global>
1412a158e: mov rax, qword ptr [rsi + 0x28]
1412a1592: imul rcx, rdi, 0x38
1412a1596: movdqa xmm0, xmmword ptr [rbp + 0x80]
1412a159e: movdqa xmm1, xmmword ptr [rbp + 0x90]
1412a15a6: movaps xmm2, xmmword ptr [rbp + 0xa0]
1412a15ad: mov rdx, qword ptr [rbp + 0xb0]
1412a15b4: mov qword ptr [rax + rcx + 0x30], rdx
1412a15b9: movups xmmword ptr [rax + rcx + 0x20], xmm2
1412a15be: movdqu xmmword ptr [rax + rcx + 0x10], xmm1
1412a15c4: movdqu xmmword ptr [rax + rcx], xmm0
1412a15c9: inc rdi
1412a15cc: mov qword ptr [rsi + 0x30], rdi
1412a15d0: inc qword ptr [rsi + 0x40]
1412a15d4: mov rcx, rbp
1412a15d7: mov r8d, 0x10
1412a15dd: mov r9d, 0x10
1412a15e3: lea rdx, [rbp + 0x20]
1412a15e7: call 0x140b73f00 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,tuple$<> >,alloc::alloc::Global>
1412a15ec: movaps xmm6, xmmword ptr [rbp + 0x150]
1412a15f3: movaps xmm7, xmmword ptr [rbp + 0x160]
1412a15fa: movaps xmm8, xmmword ptr [rbp + 0x170]
1412a1602: movaps xmm9, xmmword ptr [rbp + 0x180]
1412a160a: movaps xmm10, xmmword ptr [rbp + 0x190]
1412a1612: movaps xmm11, xmmword ptr [rbp + 0x1a0]
1412a161a: add rsp, 0x238
1412a1621: pop rbx
1412a1622: pop rdi
1412a1623: pop rsi
1412a1624: pop r12
1412a1626: pop r13
1412a1628: pop r14
1412a162a: pop r15
1412a162c: pop rbp
1412a162d: ret 
1412a162e: mov byte ptr [rbp + 0x147], 1
1412a1635: mov rcx, rbx
1412a1638: call 0x1428d8750 ; std::sys::sync::mutex::futex::Mutex::lock_contended
1412a163d: jmp 0x1412a0909 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a1642: mov byte ptr [rbp + 0x147], 1
1412a1649: call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
1412a164e: xor al, 1
1412a1650: mov dword ptr [rbp + 0x130], eax
1412a1656: mov qword ptr [rbp + 0x138], rbx
1412a165d: movzx eax, byte ptr [rsi + 0x11]
1412a1661: test al, al
1412a1663: je 0x1412a093c
1412a1669: mov rax, qword ptr [rbp + 0x138]
1412a1670: mov qword ptr [rbp + 0x80], rax
1412a1677: mov eax, dword ptr [rbp + 0x130]
1412a167d: mov byte ptr [rbp + 0x88], al
1412a1683: lea rax, [rip + 0x1989456]
1412a168a: mov qword ptr [rsp + 0x20], rax
1412a168f: lea rcx, [rip + 0x1988982]
1412a1696: lea r9, [rip + 0x19889ab]
1412a169d: lea r8, [rbp + 0x80]
1412a16a4: mov edx, 0x2b
1412a16a9: call 0x1428d9760 ; core::result::unwrap_failed
1412a16ae: jmp 0x1412a16f9 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a16b0: mov byte ptr [rbp + 0x147], 1
1412a16b7: mov rcx, rbx
1412a16ba: call 0x1428d8810 ; std::sys::sync::mutex::futex::Mutex::wake
1412a16bf: jmp 0x1412a0a13 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a16c4: mov byte ptr [rbp + 0x147], 1
1412a16cb: call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
1412a16d0: test al, al
1412a16d2: jne 0x1412a0a07
1412a16d8: mov byte ptr [rsi + 0x11], 1
1412a16dc: jmp 0x1412a0a07 ; country_core::startup::load_mesh_atlases::reload_mesh_atlases
1412a16e1: lea rcx, [rip + 0x19993b8]
1412a16e8: call 0x1428d80d0 ; std::thread::local::panic_access_error
1412a16ed: lea rcx, [rip + 0x19993ac]
1412a16f4: call 0x1428d80d0 ; std::thread::local::panic_access_error
1412a16f9: ud2 
1412a16fb: nop dword ptr [rax + rax]
1412a1700: mov qword ptr [rsp + 0x10], rdx
1412a1705: push rbp
1412a1706: push r15
1412a1708: push r14
1412a170a: push r13
1412a170c: push r12
1412a170e: push rsi
1412a170f: push rdi
1412a1710: push rbx
1412a1711: sub rsp, 0x88
1412a1718: lea rbp, [rdx + 0x80]
1412a171f: movdqa xmmword ptr [rsp + 0x20], xmm11
1412a1726: movdqa xmmword ptr [rsp + 0x30], xmm10
1412a172d: movdqa xmmword ptr [rsp + 0x40], xmm9
1412a1734: movaps xmmword ptr [rsp + 0x50], xmm8
1412a173a: movdqa xmmword ptr [rsp + 0x60], xmm7
1412a1740: movdqa xmmword ptr [rsp + 0x70], xmm6
1412a1746: mov rcx, rbp
1412a1749: mov r8d, 0x10
1412a174f: mov r9d, 0x10
1412a1755: lea rdx, [rbp + 0x20]
1412a1759: call 0x140b73f00 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<asset::prefab::PrefabName,tuple$<> >,alloc::alloc::Global>
1412a175e: movaps xmm6, xmmword ptr [rsp + 0x70]
1412a1763: movaps xmm7, xmmword ptr [rsp + 0x60]
1412a1768: movaps xmm8, xmmword ptr [rsp + 0x50]
1412a176e: movaps xmm9, xmmword ptr [rsp + 0x40]
1412a1774: movaps xmm10, xmmword ptr [rsp + 0x30]
1412a177a: movaps xmm11, xmmword ptr [rsp + 0x20]
1412a1780: add rsp, 0x88
1412a1787: pop rbx
1412a1788: pop rdi
1412a1789: pop rsi
1412a178a: pop r12
1412a178c: pop r13
1412a178e: pop r14
1412a1790: pop r15
1412a1792: pop rbp
1412a1793: ret 
1412a1794: nop word ptr cs:[rax + rax]
1412a17a0: mov qword ptr [rsp + 0x10], rdx
1412a17a5: push rbp
1412a17a6: push r15
1412a17a8: push r14
1412a17aa: push r13
1412a17ac: push r12
1412a17ae: push rsi
1412a17af: push rdi
1412a17b0: push rbx
1412a17b1: sub rsp, 0x88
1412a17b8: lea rbp, [rdx + 0x80]
1412a17bf: movdqa xmmword ptr [rsp + 0x20], xmm11
1412a17c6: movdqa xmmword ptr [rsp + 0x30], xmm10
1412a17cd: movdqa xmmword ptr [rsp + 0x40], xmm9
1412a17d4: movaps xmmword ptr [rsp + 0x50], xmm8
1412a17da: movdqa xmmword ptr [rsp + 0x60], xmm7
1412a17e0: movdqa xmmword ptr [rsp + 0x70], xmm6
1412a17e6: mov byte ptr [rbp + 0x147], 1
1412a17ed: mov rcx, qword ptr [rbp + 0x138]
1412a17f4: mov edx, dword ptr [rbp + 0x130]
1412a17fa: call 0x140043000 ; core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<winit::platform_impl::platform::window_state::WindowState> >
1412a17ff: mov byte ptr [rbp + 0x147], 1
1412a1806: movdqa xmm6, xmmword ptr [rsp + 0x70]
1412a180c: movdqa xmm7, xmmword ptr [rsp + 0x60]
1412a1812: movaps xmm8, xmmword ptr [rsp + 0x50]
1412a1818: movdqa xmm9, xmmword ptr [rsp + 0x40]
1412a181f: movdqa xmm10, xmmword ptr [rsp + 0x30]
1412a1826: movdqa xmm11, xmmword ptr [rsp + 0x20]
1412a182d: add rsp, 0x88
1412a1834: pop rbx
1412a1835: pop rdi
1412a1836: pop rsi
1412a1837: pop r12
1412a1839: pop r13
1412a183b: pop r14
1412a183d: pop r15
1412a183f: pop rbp
1412a1840: ret 
1412a1841: nop word ptr cs:[rax + rax]
1412a1850: mov qword ptr [rsp + 0x10], rdx
1412a1855: push rbp
1412a1856: push r15
1412a1858: push r14
1412a185a: push r13
1412a185c: push r12
1412a185e: push rsi
1412a185f: push rdi
1412a1860: push rbx
1412a1861: sub rsp, 0x88
1412a1868: lea rbp, [rdx + 0x80]
1412a186f: movdqa xmmword ptr [rsp + 0x20], xmm11
1412a1876: movdqa xmmword ptr [rsp + 0x30], xmm10
1412a187d: movdqa xmmword ptr [rsp + 0x40], xmm9
1412a1884: movaps xmmword ptr [rsp + 0x50], xmm8
1412a188a: movdqa xmmword ptr [rsp + 0x60], xmm7
1412a1890: movdqa xmmword ptr [rsp + 0x70], xmm6
1412a1896: cmp byte ptr [rbp + 0x147], 0
1412a189d: je 0x1412a18e3
1412a189f: lea rcx, [rbp + 0x30]
1412a18a3: call 0x14087bb00 ; hashbrown::raw::impl$18::drop<tuple$<winit::keyboard::KeyCode,tuple$<> >,alloc::alloc::Global>
1412a18a8: movdqa xmm6, xmmword ptr [rsp + 0x70]
1412a18ae: movdqa xmm7, xmmword ptr [rsp + 0x60]
1412a18b4: movaps xmm8, xmmword ptr [rsp + 0x50]
1412a18ba: movdqa xmm9, xmmword ptr [rsp + 0x40]
1412a18c1: movdqa xmm10, xmmword ptr [rsp + 0x30]
1412a18c8: movdqa xmm11, xmmword ptr [rsp + 0x20]
1412a18cf: add rsp, 0x88
1412a18d6: pop rbx
1412a18d7: pop rdi
1412a18d8: pop rsi
1412a18d9: pop r12
1412a18db: pop r13
1412a18dd: pop r14
1412a18df: pop r15
1412a18e1: pop rbp
1412a18e2: ret 
1412a18e3: movdqa xmm6, xmmword ptr [rsp + 0x70]
1412a18e9: movdqa xmm7, xmmword ptr [rsp + 0x60]
1412a18ef: movaps xmm8, xmmword ptr [rsp + 0x50]
1412a18f5: movdqa xmm9, xmmword ptr [rsp + 0x40]
1412a18fc: movdqa xmm10, xmmword ptr [rsp + 0x30]
1412a1903: movdqa xmm11, xmmword ptr [rsp + 0x20]
1412a190a: add rsp, 0x88
1412a1911: pop rbx
1412a1912: pop rdi
1412a1913: pop rsi
1412a1914: pop r12
1412a1916: pop r13
1412a1918: pop r14
1412a191a: pop r15
1412a191c: pop rbp
1412a191d: ret 
1412a191e: nop 
1412a1920: mov qword ptr [rsp + 0x10], rdx
1412a1925: push rbp
1412a1926: push r15
1412a1928: push r14
1412a192a: push r13
1412a192c: push r12
1412a192e: push rsi
1412a192f: push rdi
1412a1930: push rbx
1412a1931: sub rsp, 0x88
1412a1938: lea rbp, [rdx + 0x80]
1412a193f: movdqa xmmword ptr [rsp + 0x20], xmm11
1412a1946: movdqa xmmword ptr [rsp + 0x30], xmm10
1412a194d: movdqa xmmword ptr [rsp + 0x40], xmm9
1412a1954: movaps xmmword ptr [rsp + 0x50], xmm8
1412a195a: movdqa xmmword ptr [rsp + 0x60], xmm7
1412a1960: movdqa xmmword ptr [rsp + 0x70], xmm6
1412a1966: mov byte ptr [rbp + 0x147], 1
1412a196d: lea rcx, [rbp + 0x80]
1412a1974: call 0x1405d2870 ; core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<winit::platform_impl::platform::window_state::WindowState> >
1412a1979: mov byte ptr [rbp + 0x147], 1
1412a1980: movdqa xmm6, xmmword ptr [rsp + 0x70]
1412a1986: movdqa xmm7, xmmword ptr [rsp + 0x60]
1412a198c: movaps xmm8, xmmword ptr [rsp + 0x50]
1412a1992: movdqa xmm9, xmmword ptr [rsp + 0x40]
1412a1999: movdqa xmm10, xmmword ptr [rsp + 0x30]
1412a19a0: movdqa xmm11, xmmword ptr [rsp + 0x20]
1412a19a7: add rsp, 0x88
1412a19ae: pop rbx
1412a19af: pop rdi
1412a19b0: pop rsi
1412a19b1: pop r12
1412a19b3: pop r13
1412a19b5: pop r14
1412a19b7: pop r15
1412a19b9: pop rbp
1412a19ba: ret 
1412a19bb: nop dword ptr [rax + rax]
1412a19c0: mov qword ptr [rsp + 0x10], rdx
1412a19c5: push rbp
1412a19c6: push r15
1412a19c8: push r14
1412a19ca: push r13
1412a19cc: push r12
1412a19ce: push rsi
1412a19cf: push rdi
1412a19d0: push rbx
1412a19d1: sub rsp, 0x88
1412a19d8: lea rbp, [rdx + 0x80]
1412a19df: movdqa xmmword ptr [rsp + 0x20], xmm11
1412a19e6: movdqa xmmword ptr [rsp + 0x30], xmm10
1412a19ed: movdqa xmmword ptr [rsp + 0x40], xmm9
1412a19f4: movaps xmmword ptr [rsp + 0x50], xmm8
1412a19fa: movdqa xmmword ptr [rsp + 0x60], xmm7
1412a1a00: movdqa xmmword ptr [rsp + 0x70], xmm6
1412a1a06: mov byte ptr [rbp + 0x147], 0
1412a1a0d: lea rcx, [rbp + 0x80]
1412a1a14: call 0x14087bb00 ; hashbrown::raw::impl$18::drop<tuple$<winit::keyboard::KeyCode,tuple$<> >,alloc::alloc::Global>
1412a1a19: mov byte ptr [rbp + 0x147], 0
1412a1a20: movdqa xmm6, xmmword ptr [rsp + 0x70]
1412a1a26: movdqa xmm7, xmmword ptr [rsp + 0x60]
1412a1a2c: movaps xmm8, xmmword ptr [rsp + 0x50]
1412a1a32: movdqa xmm9, xmmword ptr [rsp + 0x40]
1412a1a39: movdqa xmm10, xmmword ptr [rsp + 0x30]
1412a1a40: movdqa xmm11, xmmword ptr [rsp + 0x20]
1412a1a47: add rsp, 0x88
1412a1a4e: pop rbx
1412a1a4f: pop rdi
1412a1a50: pop rsi
1412a1a51: pop r12
1412a1a53: pop r13
1412a1a55: pop r14
1412a1a57: pop r15
1412a1a59: pop rbp
1412a1a5a: ret 
