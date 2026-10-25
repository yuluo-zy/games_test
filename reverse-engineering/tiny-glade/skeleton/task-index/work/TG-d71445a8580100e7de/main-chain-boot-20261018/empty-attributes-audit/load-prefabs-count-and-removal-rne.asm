1412a2c4b: add r12, 0x10
1412a2c4f: mov byte ptr [rbp + 0x4e2], 1
1412a2c56: mov r9d, 0xe
1412a2c5c: lea rcx, [rbp + 0x430]
1412a2c63: mov rdx, r12
1412a2c66: call 0x140a08680 ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>
1412a2c6b: cmp qword ptr [rbp + 0x480], 0
1412a2c73: je 0x1412a2f3c
1412a2c79: mov rsi, qword ptr [rbp + 0x468]
1412a2c80: movdqa xmm0, xmmword ptr [rsi]
1412a2c84: pmovmskb ecx, xmm0
1412a2c88: mov rax, rsi
1412a2c8b: cmp ecx, 0xffff
1412a2c91: jne 0x1412a2cba
1412a2c93: lea rdx, [rsi + 0x10]
1412a2c97: mov rax, rsi
1412a2c9a: nop word ptr [rax + rax]
1412a2ca0: movdqa xmm0, xmmword ptr [rdx]
1412a2ca4: pmovmskb ecx, xmm0
1412a2ca8: add rax, -0x380
1412a2cae: add rdx, 0x10
1412a2cb2: cmp ecx, 0xffff
1412a2cb8: je 0x1412a2ca0
1412a2cba: not ecx
1412a2cbc: tzcnt ecx, ecx
1412a2cc0: neg rcx
1412a2cc3: imul rcx, rcx, 0x38
1412a2cc7: mov rax, qword ptr [rax + rcx - 8]
1412a2ccc: mov qword ptr [rbp + 0x308], rax
1412a2cd3: mov r8d, 0xb
1412a2cd9: lea rcx, [rbp + 0x488]
1412a2ce0: lea rdx, [rip + 0x1987c69]
1412a2ce7: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a2cec: mov rdi, qword ptr [rbp + 0x470]
1412a2cf3: mov rcx, rdi
1412a2cf6: and rcx, rax
1412a2cf9: shr rax, 0x39
1412a2cfd: movd xmm0, eax
1412a2d01: punpcklbw xmm0, xmm0
1412a2d05: pshuflw xmm0, xmm0, 0
1412a2d0a: pshufd xmm0, xmm0, 0
1412a2d0f: lea rbx, [rsi - 0x38]
1412a2d13: xor eax, eax
1412a2d15: movdqu xmm1, xmmword ptr [rsi + rcx]
1412a2d1a: movdqa xmm2, xmm1
1412a2d1e: pcmpeqb xmm2, xmm0
1412a2d22: pmovmskb edx, xmm2
1412a2d26: test edx, edx
1412a2d28: je 0x1412a2d83
1412a2d2a: tzcnt r8d, edx
1412a2d2f: add r8, rcx
1412a2d32: and r8, rdi
1412a2d35: neg r8
1412a2d38: imul r8, r8, 0x38
1412a2d3c: cmp qword ptr [rbx + r8 + 0x10], 0xb
1412a2d42: jne 0x1412a2d76
1412a2d44: lea r9, [rbx + r8]
1412a2d48: mov r9, qword ptr [r9 + 8]
1412a2d4c: mov r10, qword ptr [r9]
1412a2d4f: movabs r11, 0x6e65635f6d697270
1412a2d59: xor r10, r11
1412a2d5c: mov r9, qword ptr [r9 + 3]
1412a2d60: movabs r11, 0x7265746e65635f6d
1412a2d6a: xor r9, r11
1412a2d6d: or r9, r10
1412a2d70: je 0x1412a2e51
1412a2d76: lea r8d, [rdx - 1]
1412a2d7a: and r8w, dx
1412a2d7e: mov edx, r8d
1412a2d81: jne 0x1412a2d2a
1412a2d83: pcmpeqb xmm1, xmm9
1412a2d88: pmovmskb edx, xmm1
1412a2d8c: test edx, edx
1412a2d8e: jne 0x1412a39ad
1412a2d94: add rcx, rax
1412a2d97: add rcx, 0x10
1412a2d9b: add rax, 0x10
1412a2d9f: and rcx, rdi
1412a2da2: jmp 0x1412a2d15 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a2da7: mov rax, qword ptr [r8]
1412a2daa: movabs rcx, 0x5f74636172747865
1412a2db4: xor rax, rcx
1412a2db7: mov rcx, qword ptr [r8 + 5]
1412a2dbb: movabs rdx, 0x6b6e7572745f7463
1412a2dc5: xor rcx, rdx
1412a2dc8: or rcx, rax
1412a2dcb: jne 0x1412a6734
1412a2dd1: add r12, 0x10
1412a2dd5: mov byte ptr [rbp + 0x4e2], 1
1412a2ddc: mov r9d, 0xd
1412a2de2: lea rcx, [rbp + 0x430]
1412a2de9: mov rdx, r12
1412a2dec: call 0x140a08d30 ; country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>
1412a2df1: cmp qword ptr [rbp + 0x480], 0
1412a2df9: je 0x1412a2f4c
1412a2dff: mov rax, qword ptr [rbp + 0x468]
1412a2e06: movdqa xmm0, xmmword ptr [rax]
1412a2e0a: pmovmskb ecx, xmm0
1412a2e0e: cmp ecx, 0xffff
1412a2e14: jne 0x1412a2e3a
1412a2e16: lea rdx, [rax + 0x10]
1412a2e1a: nop word ptr [rax + rax]
1412a2e20: movdqa xmm0, xmmword ptr [rdx]
1412a2e24: pmovmskb ecx, xmm0
1412a2e28: add rax, -0x380
1412a2e2e: add rdx, 0x10
1412a2e32: cmp ecx, 0xffff
1412a2e38: je 0x1412a2e20
1412a2e3a: not ecx
1412a2e3c: tzcnt ecx, ecx
1412a2e40: neg rcx
1412a2e43: imul rcx, rcx, 0x38
1412a2e47: mov rbx, qword ptr [rax + rcx - 8]
1412a2e4c: jmp 0x1412a2f4e ; country_core::startup::load_prefabs::load_prefabs_impl
1412a2e51: cmp dword ptr [rsi + r8 - 0x20], 3
1412a2f3c: mov qword ptr [rbp + 0x308], 0
1412a2f47: jmp 0x1412a39ad ; country_core::startup::load_prefabs::load_prefabs_impl
1412a2f4c: xor ebx, ebx
1412a2f4e: mov r8d, 2
1412a2f54: lea rcx, [rbp + 0x488]
1412a2f5b: lea rsi, [rip + 0x19878ee]
1412a2f62: mov rdx, rsi
1412a2f65: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a2f6a: mov qword ptr [rsp + 0x20], 2
1412a2f73: lea rcx, [rbp + 0x3b0]
1412a2f7a: lea rdx, [rbp + 0x468]
1412a2f81: mov r8, rax
1412a2f84: mov r9, rsi
1412a2f87: call 0x140b78d80 ; hashbrown::raw::RawTable::remove_entry<tuple$<alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<str$,alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> > >
1412a2f8c: mov r12, qword ptr [rbp + 0x3c8]
1412a2f93: cmp r12, 5
1412a2f97: je 0x1412a3a91
1412a2f9d: mov rdx, qword ptr [rbp + 0x3b0]
1412a2fa4: mov rsi, qword ptr [rbp + 0x3d0]
1412a2fab: mov rdi, qword ptr [rbp + 0x3d8]
1412a2fb2: test rdx, rdx
1412a2fb5: je 0x1412a2fc9
1412a2fb7: mov rcx, qword ptr [rbp + 0x3b8]
1412a2fbe: mov r8d, 1
1412a2fc4: call 0x140613c20
1412a2fc9: cmp r12, 3
1412a2fcd: ja 0x1412a3a77
1412a2fd3: lea rcx, [rip + 0x1987b1e]
1412a2fda: movsxd rax, dword ptr [rcx + r12*4]
1412a2fde: add rax, rcx
1412a2fe1: jmp rax
1412a2fe3: test rsi, rsi
1412a2fe6: je 0x1412a3a91
1412a2fec: shl rsi, 2
1412a2ff0: jmp 0x1412a3a80 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a39ad: mov r8d, 2
1412a39b3: lea rcx, [rbp + 0x488]
1412a39ba: lea rsi, [rip + 0x1986e8f]
1412a39c1: mov rdx, rsi
1412a39c4: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a39c9: mov qword ptr [rsp + 0x20], 2
1412a39d2: lea rcx, [rbp + 0x3b0]
1412a39d9: lea rdx, [rbp + 0x468]
1412a39e0: mov r8, rax
1412a39e3: mov r9, rsi
1412a39e6: call 0x140b78d80 ; hashbrown::raw::RawTable::remove_entry<tuple$<alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<str$,alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> > >
1412a39eb: mov rbx, qword ptr [rbp + 0x3c8]
1412a39f2: cmp rbx, 5
1412a39f6: je 0x1412a3ddc
1412a39fc: mov rdx, qword ptr [rbp + 0x3b0]
1412a3a03: mov rsi, qword ptr [rbp + 0x3d0]
1412a3a0a: mov rdi, qword ptr [rbp + 0x3d8]
1412a3a11: test rdx, rdx
1412a3a14: je 0x1412a3a28
1412a3a16: mov rcx, qword ptr [rbp + 0x3b8]
1412a3a1d: mov r8d, 1
1412a3a23: call 0x140613c20
1412a3a28: cmp rbx, 3
1412a3a2c: ja 0x1412a3dc2
1412a3a32: lea rcx, [rip + 0x19870cf]
1412a3a39: movsxd rax, dword ptr [rcx + rbx*4]
1412a3a3d: add rax, rcx
1412a3a40: jmp rax
1412a3a42: test rsi, rsi
1412a3a45: je 0x1412a3ddc
1412a3a4b: shl rsi, 2
1412a3a4f: jmp 0x1412a3dcb ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3a54: test rsi, rsi
1412a3a57: je 0x1412a3a91
1412a3a59: shl rsi, 3
1412a3a5d: jmp 0x1412a3a80 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3a5f: test rsi, rsi
1412a3a62: je 0x1412a3a91
1412a3a64: shl rsi, 2
1412a3a68: lea rdx, [rsi + rsi*2]
1412a3a6c: mov r8d, 4
1412a3a72: mov rcx, rdi
1412a3a75: jmp 0x1412a3a8c ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3a77: test rsi, rsi
1412a3a7a: je 0x1412a3a91
1412a3a7c: shl rsi, 4
1412a3a80: mov r8d, 4
1412a3a86: mov rcx, rdi
1412a3a89: mov rdx, rsi
1412a3a8c: call 0x140613c20
1412a3a91: test rbx, rbx
1412a3a94: je 0x1412a4121
1412a3a9a: cmp qword ptr [rbp + 0x480], 0
1412a3aa2: je 0x1412a3b7f
1412a3aa8: mov r8d, 0xa
1412a3aae: lea rcx, [rbp + 0x488]
1412a3ab5: lea rdx, [rip + 0x1986d96]
1412a3abc: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a3ac1: mov rcx, qword ptr [rbp + 0x468]
1412a3ac8: mov rdx, qword ptr [rbp + 0x470]
1412a3acf: mov r8, rdx
1412a3ad2: and r8, rax
1412a3ad5: shr rax, 0x39
1412a3ad9: movd xmm0, eax
1412a3add: punpcklbw xmm0, xmm0
1412a3ae1: pshuflw xmm0, xmm0, 0
1412a3ae6: pshufd xmm0, xmm0, 0
1412a3aeb: lea rax, [rcx - 0x38]
1412a3aef: xor r9d, r9d
1412a3af2: movdqu xmm1, xmmword ptr [rcx + r8]
1412a3af8: movdqa xmm2, xmm1
1412a3afc: pcmpeqb xmm2, xmm0
1412a3b00: pmovmskb r10d, xmm2
1412a3b05: test r10d, r10d
1412a3b08: je 0x1412a3b5d
1412a3b0a: tzcnt r11d, r10d
1412a3b0f: add r11, r8
1412a3b12: and r11, rdx
1412a3b15: neg r11
1412a3b18: imul r11, r11, 0x38
1412a3b1c: cmp qword ptr [rax + r11 + 0x10], 0xa
1412a3b22: jne 0x1412a3b50
1412a3b24: add r11, rax
1412a3b27: mov r11, qword ptr [r11 + 8]
1412a3b2b: mov rsi, qword ptr [r11]
1412a3b2e: movabs rdi, 0x705f726165707061
1412a3b38: xor rsi, rdi
1412a3b3b: movzx r11d, word ptr [r11 + 8]
1412a3b40: xor r11, 0x736f
1412a3b47: or r11, rsi
1412a3b4a: je 0x1412a3bdf
1412a3b50: lea r11d, [r10 - 1]
1412a3b54: and r11w, r10w
1412a3b58: mov r10d, r11d
1412a3b5b: jne 0x1412a3b0a
1412a3b5d: pcmpeqb xmm1, xmm9
1412a3b62: pmovmskb r10d, xmm1
1412a3b67: test r10d, r10d
1412a3b6a: jne 0x1412a3b7f
1412a3b6c: add r8, r9
1412a3b6f: add r8, 0x10
1412a3b73: add r9, 0x10
1412a3b77: and r8, rdx
1412a3b7a: jmp 0x1412a3af2 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3b7f: mov dword ptr [rbp + 0x368], 0
1412a3b89: mov qword ptr [rbp + 0x360], 0
1412a3b94: lea rcx, [rbp + 0x3b0]
1412a3b9b: lea rdx, [rbp + 0x360]
1412a3ba2: mov r8, rbx
1412a3ba5: lea r9, [rip + 0x1986cb4]
1412a3bac: call 0x1412f7260 ; alloc::vec::from_elem<array$<f32,3> >
1412a3bb1: mov r8d, 0xa
1412a3bb7: lea rcx, [rbp + 0x430]
1412a3bbe: lea rdx, [rip + 0x1986c8d]
1412a3bc5: lea r9, [rbp + 0x3b0]
1412a3bcc: call 0x140a34c90 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> >
1412a3bd1: cmp qword ptr [rbp + 0x480], 0
1412a3bd9: je 0x1412a3cae
1412a3bdf: mov r8d, 3
1412a3be5: lea rcx, [rbp + 0x488]
1412a3bec: lea rdx, [rip + 0x1986c85]
1412a3bf3: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a3bf8: mov rcx, qword ptr [rbp + 0x468]
1412a3bff: mov rdx, qword ptr [rbp + 0x470]
1412a3c06: mov r8, rdx
1412a3c09: and r8, rax
1412a3c0c: shr rax, 0x39
1412a3c10: movd xmm0, eax
1412a3c14: punpcklbw xmm0, xmm0
1412a3c18: pshuflw xmm0, xmm0, 0
1412a3c1d: pshufd xmm0, xmm0, 0
1412a3c22: lea rax, [rcx - 0x38]
1412a3c26: xor r9d, r9d
1412a3c29: movdqu xmm1, xmmword ptr [rcx + r8]
1412a3c2f: movdqa xmm2, xmm1
1412a3c33: pcmpeqb xmm2, xmm0
1412a3c37: pmovmskb r10d, xmm2
1412a3c3c: test r10d, r10d
1412a3c3f: je 0x1412a3c8c
1412a3c41: tzcnt r11d, r10d
1412a3c46: add r11, r8
1412a3c49: and r11, rdx
1412a3c4c: neg r11
1412a3c4f: imul r11, r11, 0x38
1412a3c53: cmp qword ptr [rax + r11 + 0x10], 3
1412a3c59: jne 0x1412a3c7f
1412a3c5b: add r11, rax
1412a3c5e: mov r11, qword ptr [r11 + 8]
1412a3c62: movzx esi, word ptr [r11]
1412a3c66: xor esi, 0x6761
1412a3c6c: movzx r11d, byte ptr [r11 + 2]
1412a3c71: xor r11d, 0x65
1412a3c75: or r11w, si
1412a3c79: je 0x1412a4121
1412a3c7f: lea r11d, [r10 - 1]
1412a3c83: and r11w, r10w
1412a3c87: mov r10d, r11d
1412a3c8a: jne 0x1412a3c41
1412a3c8c: pcmpeqb xmm1, xmm9
1412a3c91: pmovmskb r10d, xmm1
1412a3c96: test r10d, r10d
1412a3c99: jne 0x1412a3cae
1412a3c9b: add r8, r9
1412a3c9e: add r8, 0x10
1412a3ca2: add r9, 0x10
1412a3ca6: and r8, rdx
1412a3ca9: jmp 0x1412a3c29 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3cae: lea rsi, [rbx*4]
1412a3cb6: mov rax, rbx
1412a3cb9: shr rax, 0x3e
1412a3cbd: jne 0x1412a6889
1412a3cc3: mov edi, 0
1412a3cc8: movabs rax, 0x7ffffffffffffffc
1412a3cd2: cmp rsi, rax
1412a3cd5: ja 0x1412a688b
1412a3cdb: movzx eax, byte ptr [rip + 0x2390f27]
1412a3ce2: mov edi, 4
1412a3ce7: mov edx, 4
1412a3cec: mov rcx, rsi
1412a3cef: call 0x140613c10
1412a3cf4: test rax, rax
1412a3cf7: je 0x1412a688b
1412a3cfd: mov rcx, rax
1412a3d00: cmp rbx, 1
1412a3d04: je 0x1412a3d5f
1412a3d06: mov edx, 1
1412a3d0b: mov rcx, rax
1412a3d0e: cmp rbx, 9
1412a3d12: jb 0x1412a3d49
1412a3d14: lea r8, [rbx - 1]
1412a3d18: mov r9, r8
1412a3d1b: and r9, 0xfffffffffffffff8
1412a3d1f: lea rcx, [rax + r9*4]
1412a3d23: lea rdx, [r9 + 1]
1412a3d27: xor r10d, r10d
1412a3d2a: nop word ptr [rax + rax]
1412a3d30: movups xmmword ptr [rax + r10*4], xmm8
1412a3d35: movups xmmword ptr [rax + r10*4 + 0x10], xmm8
1412a3d3b: add r10, 8
1412a3d3f: cmp r9, r10
1412a3d42: jne 0x1412a3d30
1412a3d44: cmp r8, r9
1412a3d47: je 0x1412a3d5f
1412a3d49: mov r8, rbx
1412a3d4c: sub r8, rdx
1412a3d4f: nop 
1412a3d50: mov dword ptr [rcx], 0x3f800000
1412a3d56: add rcx, 4
1412a3d5a: dec r8
1412a3d5d: jne 0x1412a3d50
1412a3d5f: mov dword ptr [rcx], 0x3f800000
1412a3d65: mov qword ptr [rbp + 0x3b0], rbx
1412a3d6c: mov qword ptr [rbp + 0x3b8], rax
1412a3d73: mov qword ptr [rbp + 0x3c0], rbx
1412a3d7a: mov r8d, 3
1412a3d80: lea rcx, [rbp + 0x430]
1412a3d87: lea rdx, [rip + 0x1986aea]
1412a3d8e: lea r9, [rbp + 0x3b0]
1412a3d95: call 0x140a35b80 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<f32,alloc::alloc::Global> >
1412a3d9a: jmp 0x1412a4121 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3d9f: test rsi, rsi
1412a3da2: je 0x1412a3ddc
1412a3da4: shl rsi, 3
1412a3da8: jmp 0x1412a3dcb ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3daa: test rsi, rsi
1412a3dad: je 0x1412a3ddc
1412a3daf: shl rsi, 2
1412a3db3: lea rdx, [rsi + rsi*2]
1412a3db7: mov r8d, 4
1412a3dbd: mov rcx, rdi
1412a3dc0: jmp 0x1412a3dd7 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3dc2: test rsi, rsi
1412a3dc5: je 0x1412a3ddc
1412a3dc7: shl rsi, 4
1412a3dcb: mov r8d, 4
1412a3dd1: mov rcx, rdi
1412a3dd4: mov rdx, rsi
1412a3dd7: call 0x140613c20
1412a3ddc: cmp qword ptr [rbp + 0x480], 0
1412a3de4: je 0x1412a3ec1
1412a3dea: mov r8d, 0xa
1412a3df0: lea rcx, [rbp + 0x488]
1412a3df7: lea rdx, [rip + 0x1986a54]
1412a3dfe: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a3e03: mov rcx, qword ptr [rbp + 0x468]
1412a3e0a: mov rdx, qword ptr [rbp + 0x470]
1412a3e11: mov r8, rdx
1412a3e14: and r8, rax
1412a3e17: shr rax, 0x39
1412a3e1b: movd xmm0, eax
1412a3e1f: punpcklbw xmm0, xmm0
1412a3e23: pshuflw xmm0, xmm0, 0
1412a3e28: pshufd xmm0, xmm0, 0
1412a3e2d: lea rax, [rcx - 0x38]
1412a3e31: xor r9d, r9d
1412a3e34: movdqu xmm1, xmmword ptr [rcx + r8]
1412a3e3a: movdqa xmm2, xmm1
1412a3e3e: pcmpeqb xmm2, xmm0
1412a3e42: pmovmskb r10d, xmm2
1412a3e47: test r10d, r10d
1412a3e4a: je 0x1412a3e9f
1412a3e4c: tzcnt r11d, r10d
1412a3e51: add r11, r8
1412a3e54: and r11, rdx
1412a3e57: neg r11
1412a3e5a: imul r11, r11, 0x38
1412a3e5e: cmp qword ptr [rax + r11 + 0x10], 0xa
1412a3e64: jne 0x1412a3e92
1412a3e66: add r11, rax
1412a3e69: mov r11, qword ptr [r11 + 8]
1412a3e6d: mov rsi, qword ptr [r11]
1412a3e70: movabs rdi, 0x705f726165707061
1412a3e7a: xor rsi, rdi
1412a3e7d: movzx r11d, word ptr [r11 + 8]
1412a3e82: xor r11, 0x736f
1412a3e89: or r11, rsi
1412a3e8c: je 0x1412a3f25
1412a3e92: lea r11d, [r10 - 1]
1412a3e96: and r11w, r10w
1412a3e9a: mov r10d, r11d
1412a3e9d: jne 0x1412a3e4c
1412a3e9f: pcmpeqb xmm1, xmm9
1412a3ea4: pmovmskb r10d, xmm1
1412a3ea9: test r10d, r10d
1412a3eac: jne 0x1412a3ec1
1412a3eae: add r8, r9
1412a3eb1: add r8, 0x10
1412a3eb5: add r9, 0x10
1412a3eb9: and r8, rdx
1412a3ebc: jmp 0x1412a3e34 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3ec1: mov dword ptr [rbp + 0x368], 0
1412a3ecb: mov qword ptr [rbp + 0x360], 0
1412a3ed6: lea rcx, [rbp + 0x3b0]
1412a3edd: lea rdx, [rbp + 0x360]
1412a3ee4: mov r8, qword ptr [rbp + 0x308]
1412a3eeb: lea r9, [rip + 0x19869a6]
1412a3ef2: call 0x1412f7260 ; alloc::vec::from_elem<array$<f32,3> >
1412a3ef7: mov r8d, 0xa
1412a3efd: lea rcx, [rbp + 0x430]
1412a3f04: lea rdx, [rip + 0x1986947]
1412a3f0b: lea r9, [rbp + 0x3b0]
1412a3f12: call 0x140a34c90 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> >
1412a3f17: cmp qword ptr [rbp + 0x480], 0
1412a3f1f: je 0x1412a3ff4
1412a3f25: mov r8d, 3
1412a3f2b: lea rcx, [rbp + 0x488]
1412a3f32: lea rdx, [rip + 0x198693f]
1412a3f39: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1412a3f3e: mov rcx, qword ptr [rbp + 0x468]
1412a3f45: mov rdx, qword ptr [rbp + 0x470]
1412a3f4c: mov r8, rdx
1412a3f4f: and r8, rax
1412a3f52: shr rax, 0x39
1412a3f56: movd xmm0, eax
1412a3f5a: punpcklbw xmm0, xmm0
1412a3f5e: pshuflw xmm0, xmm0, 0
1412a3f63: pshufd xmm0, xmm0, 0
1412a3f68: lea rax, [rcx - 0x38]
1412a3f6c: xor r9d, r9d
1412a3f6f: movdqu xmm1, xmmword ptr [rcx + r8]
1412a3f75: movdqa xmm2, xmm1
1412a3f79: pcmpeqb xmm2, xmm0
1412a3f7d: pmovmskb r10d, xmm2
1412a3f82: test r10d, r10d
1412a3f85: je 0x1412a3fd2
1412a3f87: tzcnt r11d, r10d
1412a3f8c: add r11, r8
1412a3f8f: and r11, rdx
1412a3f92: neg r11
1412a3f95: imul r11, r11, 0x38
1412a3f99: cmp qword ptr [rax + r11 + 0x10], 3
1412a3f9f: jne 0x1412a3fc5
1412a3fa1: add r11, rax
1412a3fa4: mov r11, qword ptr [r11 + 8]
1412a3fa8: movzx esi, word ptr [r11]
1412a3fac: xor esi, 0x6761
1412a3fb2: movzx r11d, byte ptr [r11 + 2]
1412a3fb7: xor r11d, 0x65
1412a3fbb: or r11w, si
1412a3fbf: je 0x1412a4121
1412a3fc5: lea r11d, [r10 - 1]
1412a3fc9: and r11w, r10w
1412a3fcd: mov r10d, r11d
1412a3fd0: jne 0x1412a3f87
1412a3fd2: pcmpeqb xmm1, xmm9
1412a3fd7: pmovmskb r10d, xmm1
1412a3fdc: test r10d, r10d
1412a3fdf: jne 0x1412a3ff4
1412a3fe1: add r8, r9
1412a3fe4: add r8, 0x10
1412a3fe8: add r9, 0x10
1412a3fec: and r8, rdx
1412a3fef: jmp 0x1412a3f6f ; country_core::startup::load_prefabs::load_prefabs_impl
1412a3ff4: mov r8, qword ptr [rbp + 0x308]
1412a3ffb: lea rsi, [r8*4]
1412a4003: mov rax, r8
1412a4006: shr rax, 0x3e
1412a400a: jne 0x1412a6873
1412a4010: mov edi, 0
1412a4015: movabs rax, 0x7ffffffffffffffc
1412a401f: cmp rsi, rax
1412a4022: ja 0x1412a6875
1412a4028: test rsi, rsi
1412a402b: je 0x1412a4070
1412a402d: movzx eax, byte ptr [rip + 0x2390bd5]
1412a4034: mov edi, 4
1412a4039: mov edx, 4
1412a403e: mov rcx, rsi
1412a4041: call 0x140613c10
1412a4046: mov r8, qword ptr [rbp + 0x308]
1412a404d: test rax, rax
1412a4050: je 0x1412a6875
1412a4056: mov rcx, r8
1412a4059: cmp r8, 2
1412a405d: jb 0x1412a407d
1412a405f: cmp r8, 8
1412a4063: ja 0x1412a4087
1412a4065: mov r8d, 1
1412a406b: mov rdx, rax
1412a406e: jmp 0x1412a40b9 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4070: mov eax, 4
1412a4075: xor ecx, ecx
1412a4077: cmp r8, 2
1412a407b: jae 0x1412a405f
1412a407d: mov rdx, rax
1412a4080: test r8, r8
1412a4083: jne 0x1412a40df
1412a4085: jmp 0x1412a40e5 ; country_core::startup::load_prefabs::load_prefabs_impl
1412a4087: lea r9, [r8 - 1]
1412a408b: mov r10, r9
1412a408e: and r10, 0xfffffffffffffff8
1412a4092: lea rdx, [rax + r10*4]
1412a4096: lea r8, [r10 + 1]
1412a409a: xor r11d, r11d
1412a409d: nop dword ptr [rax]
1412a40a0: movups xmmword ptr [rax + r11*4], xmm8
1412a40a5: movups xmmword ptr [rax + r11*4 + 0x10], xmm8
1412a40ab: add r11, 8
1412a40af: cmp r10, r11
1412a40b2: jne 0x1412a40a0
1412a40b4: cmp r9, r10
1412a40b7: je 0x1412a40df
1412a40b9: mov r9, qword ptr [rbp + 0x308]
1412a40c0: sub r9, r8
1412a40c3: nop word ptr cs:[rax + rax]
1412a40d0: mov dword ptr [rdx], 0x3f800000
1412a40d6: add rdx, 4
1412a40da: dec r9
1412a40dd: jne 0x1412a40d0
1412a40df: mov dword ptr [rdx], 0x3f800000
1412a40e5: mov qword ptr [rbp + 0x3b0], rcx
1412a40ec: mov qword ptr [rbp + 0x3b8], rax
1412a40f3: mov rax, qword ptr [rbp + 0x308]
1412a40fa: mov qword ptr [rbp + 0x3c0], rax
1412a4101: mov r8d, 3
1412a4107: lea rcx, [rbp + 0x430]
1412a410e: lea rdx, [rip + 0x1986763]
1412a4115: lea r9, [rbp + 0x3b0]
1412a411c: call 0x140a35b80 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<f32,alloc::alloc::Global> >
1412a4121: mov rax, qword ptr [rbp + 0x4a0]
