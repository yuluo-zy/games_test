1415f5b60: push rbp
1415f5b61: push r15
1415f5b63: push r14
1415f5b65: push r13
1415f5b67: push r12
1415f5b69: push rsi
1415f5b6a: push rdi
1415f5b6b: push rbx
1415f5b6c: sub rsp, 0x398
1415f5b73: lea rbp, [rsp + 0x80]
1415f5b7b: movdqa xmmword ptr [rbp + 0x300], xmm8
1415f5b84: movdqa xmmword ptr [rbp + 0x2f0], xmm7
1415f5b8c: movdqa xmmword ptr [rbp + 0x2e0], xmm6
1415f5b94: mov qword ptr [rbp + 0x2d8], 0xfffffffffffffffe
1415f5b9f: mov rsi, r8
1415f5ba2: mov rbx, rdx
1415f5ba5: mov qword ptr [rbp + 0xa0], rcx
1415f5bac: mov dword ptr [rbp + 0x240], 0
1415f5bb6: pxor xmm0, xmm0
1415f5bba: movdqu xmmword ptr [rbp + 0x248], xmm0
1415f5bc2: mov dword ptr [rbp + 0x258], 7
1415f5bcc: mov qword ptr [rbp + 0x25c], 0
1415f5bd7: mov word ptr [rbp + 0x264], 0
1415f5be0: mov byte ptr [rbp + 0x260], 1
1415f5be7: lea rdx, [rip + 0x168769d]
1415f5bee: lea rcx, [rbp + 0x240]
1415f5bf5: mov r8d, 0x16
1415f5bfb: call 0x140d59dc0 ; std::fs::OpenOptions::_open
1415f5c00: mov rdi, rdx
1415f5c03: test al, 1
1415f5c05: jne 0x1415f6b79
1415f5c0b: movzx eax, byte ptr [rip + 0x203eff7]
1415f5c12: mov ecx, 0x2000
1415f5c17: mov edx, 1
1415f5c1c: call 0x140613c10
1415f5c21: test rax, rax
1415f5c24: je 0x1415f6bb0
1415f5c2a: mov qword ptr [rbp + 0x240], rax
1415f5c31: mov qword ptr [rbp + 0x248], 0x2000
1415f5c3c: pxor xmm0, xmm0
1415f5c40: movdqu xmmword ptr [rbp + 0x250], xmm0
1415f5c48: mov qword ptr [rbp + 0x260], 0
1415f5c53: mov qword ptr [rbp + 0x268], rdi
1415f5c5a: mov qword ptr [rbp + 0xe0], 0
1415f5c65: mov qword ptr [rbp + 0xd0], 1
1415f5c70: mov qword ptr [rbp + 0xd8], 0x80
1415f5c7b: lea rcx, [rbp + 0x1b0]
1415f5c82: lea rdx, [rbp + 0xd0]
1415f5c89: lea r8, [rbp + 0x240]
1415f5c90: call 0x14169c5b0 ; ron::options::Options::from_reader_seed<std::io::buffered::bufreader::BufReader<std::fs::File>,core::marker::PhantomData<alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >,alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >
1415f5c95: cmp byte ptr [rbp + 0x1b0], 0x32
1415f5c9c: jne 0x1415f6bd2
1415f5ca2: mov rcx, qword ptr [rbp + 0x1b8]
1415f5ca9: mov rax, qword ptr [rbp + 0x1c0]
1415f5cb0: mov rdx, qword ptr [rbp + 0x1c8]
1415f5cb7: lea rdx, [rdx + rdx*4]
1415f5cbb: shl rdx, 4
1415f5cbf: add rdx, rax
1415f5cc2: mov qword ptr [rbp + 0x50], rax
1415f5cc6: mov qword ptr [rbp + 0x58], rax
1415f5cca: mov qword ptr [rbp + 0x60], rcx
1415f5cce: mov qword ptr [rbp + 0xb0], rdx
1415f5cd5: mov qword ptr [rbp + 0x68], rdx
1415f5cd9: mov rcx, qword ptr [rsi + 0x18]
1415f5cdd: mov qword ptr [rbp + 0xb8], rcx
1415f5ce4: lea rcx, [rsi + 0x20]
1415f5ce8: mov qword ptr [rbp + 0xc0], rcx
1415f5cef: mov rcx, qword ptr [rsi]
1415f5cf2: mov qword ptr [rbp + 0x1a0], rcx
1415f5cf9: mov rcx, qword ptr [rsi + 8]
1415f5cfd: mov qword ptr [rbp + 0x1a8], rcx
1415f5d04: lea r13, [rbp + 0x208]
1415f5d0b: lea r14, [rbp + 0x240]
1415f5d12: pcmpeqd xmm6, xmm6
1415f5d16: mov qword ptr [rbp + 0xa8], rbx
1415f5d1d: cmp rax, qword ptr [rbp + 0xb0]
1415f5d24: jne 0x1415f5d44
1415f5d26: jmp 0x1415f69ee ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f5d2b: nop dword ptr [rax + rax]
1415f5d30: mov rax, qword ptr [rbp + 0x198]
1415f5d37: cmp rax, qword ptr [rbp + 0xb0]
1415f5d3e: je 0x1415f69ee
1415f5d44: lea rcx, [rax + 0x50]
1415f5d48: mov qword ptr [rbp + 0x198], rcx
1415f5d4f: mov qword ptr [rbp + 0x58], rcx
1415f5d53: mov rcx, qword ptr [rax]
1415f5d56: mov rdx, qword ptr [rax + 0x48]
1415f5d5a: mov qword ptr [rbp], rdx
1415f5d5e: movups xmm0, xmmword ptr [rax + 0x38]
1415f5d62: movaps xmmword ptr [rbp - 0x10], xmm0
1415f5d66: movups xmm0, xmmword ptr [rax + 0x28]
1415f5d6a: movaps xmmword ptr [rbp - 0x20], xmm0
1415f5d6e: movups xmm0, xmmword ptr [rax + 0x18]
1415f5d72: movaps xmmword ptr [rbp - 0x30], xmm0
1415f5d76: movups xmm0, xmmword ptr [rax + 8]
1415f5d7a: movaps xmmword ptr [rbp - 0x40], xmm0
1415f5d7e: mov rax, rcx
1415f5d81: neg rax
1415f5d84: jo 0x1415f69ee
1415f5d8a: mov qword ptr [rbp + 0xd0], rcx
1415f5d91: mov rax, qword ptr [rbp]
1415f5d95: lea rcx, [rbp + 0xd8]
1415f5d9c: mov qword ptr [rcx + 0x40], rax
1415f5da0: movdqa xmm0, xmmword ptr [rbp - 0x40]
1415f5da5: movdqa xmm1, xmmword ptr [rbp - 0x30]
1415f5daa: movdqa xmm2, xmmword ptr [rbp - 0x20]
1415f5daf: movaps xmm3, xmmword ptr [rbp - 0x10]
1415f5db3: movups xmmword ptr [rcx + 0x30], xmm3
1415f5db7: movdqu xmmword ptr [rcx + 0x20], xmm2
1415f5dbc: movdqu xmmword ptr [rcx + 0x10], xmm1
1415f5dc1: movdqu xmmword ptr [rcx], xmm0
1415f5dc5: movzx esi, byte ptr [rbp + 0x118]
1415f5dcc: mov byte ptr [rbp + 0x240], sil
1415f5dd3: cmp qword ptr [rbp + 0xb8], 0
1415f5ddb: je 0x1415f5e80
1415f5de1: mov rcx, qword ptr [rbp + 0xc0]
1415f5de8: mov rdx, r14
1415f5deb: call 0x1412e7650 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::render::nani::gpu_world::GpuWorldSubsetKind> >
1415f5df0: mov rcx, rax
1415f5df3: shr rcx, 0x39
1415f5df7: movd xmm0, ecx
1415f5dfb: punpcklbw xmm0, xmm0
1415f5dff: pshuflw xmm0, xmm0, 0
1415f5e04: pshufd xmm0, xmm0, 0
1415f5e09: xor ecx, ecx
1415f5e0b: and rax, qword ptr [rbp + 0x1a8]
1415f5e12: mov rdx, qword ptr [rbp + 0x1a0]
1415f5e19: movdqu xmm1, xmmword ptr [rdx + rax]
1415f5e1e: movdqa xmm2, xmm1
1415f5e22: pcmpeqb xmm2, xmm0
1415f5e26: pmovmskb edx, xmm2
1415f5e2a: test edx, edx
1415f5e2c: je 0x1415f5e60
1415f5e2e: tzcnt r8d, edx
1415f5e33: add r8, rax
1415f5e36: and r8, qword ptr [rbp + 0x1a8]
1415f5e3d: not r8
1415f5e40: mov r9, qword ptr [rbp + 0x1a0]
1415f5e47: cmp sil, byte ptr [r8 + r9]
1415f5e4b: je 0x1415f5f10
1415f5e51: lea r8d, [rdx - 1]
1415f5e55: and r8w, dx
1415f5e59: mov edx, r8d
1415f5e5c: jne 0x1415f5e2e
1415f5e5e: nop 
1415f5e60: pcmpeqb xmm1, xmm6
1415f5e64: pmovmskb edx, xmm1
1415f5e68: test edx, edx
1415f5e6a: jne 0x1415f5e80
1415f5e6c: add rax, rcx
1415f5e6f: add rax, 0x10
1415f5e73: add rcx, 0x10
1415f5e77: jmp 0x1415f5e0b ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f5e79: nop dword ptr [rax]
1415f5e80: lea rcx, [rbp + 0xd0]
1415f5e87: call 0x1408919b0 ; core::ptr::drop_in_place<country_core::startup::load_mesh_atlases::MeshAttribConfig>
1415f5e8c: mov rsi, qword ptr [rbp + 0x110]
1415f5e93: test rsi, rsi
1415f5e96: je 0x1415f5ee0
1415f5e98: mov rdi, qword ptr [rbp + 0x108]
1415f5e9f: add rdi, 8
1415f5ea3: jmp 0x1415f5eb9 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f5ea5: nop word ptr cs:[rax + rax]
1415f5eb0: add rdi, 0x20
1415f5eb4: dec rsi
1415f5eb7: je 0x1415f5ee0
1415f5eb9: mov rdx, qword ptr [rdi - 8]
1415f5ebd: test rdx, rdx
1415f5ec0: je 0x1415f5eb0
1415f5ec2: mov rcx, qword ptr [rdi]
1415f5ec5: mov r8d, 1
1415f5ecb: call 0x140613c20
1415f5ed0: jmp 0x1415f5eb0 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f5ed2: nop word ptr cs:[rax + rax]
1415f5ee0: mov rdx, qword ptr [rbp + 0x100]
1415f5ee7: test rdx, rdx
1415f5eea: je 0x1415f5d30
1415f5ef0: mov rcx, qword ptr [rbp + 0x108]
1415f5ef7: shl rdx, 5
1415f5efb: mov r8d, 8
1415f5f01: call 0x140613c20
1415f5f06: jmp 0x1415f5d30 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f5f0b: nop dword ptr [rax + rax]
1415f5f10: cmp sil, 9
1415f5f14: mov byte ptr [rbp + 0x2c7], sil
1415f5f1b: je 0x1415f6b31
1415f5f21: mov rax, qword ptr [rbp + 0x100]
1415f5f28: mov rcx, qword ptr [rbp + 0x108]
1415f5f2f: mov rsi, qword ptr [rbp + 0x110]
1415f5f36: shl rsi, 5
1415f5f3a: lea rdx, [rcx + rsi]
1415f5f3e: mov qword ptr [rbp + 0x10], rcx
1415f5f42: mov qword ptr [rbp + 0x230], rax
1415f5f49: mov qword ptr [rbp + 0x20], rax
1415f5f4d: mov qword ptr [rbp + 0x190], rdx
1415f5f54: mov qword ptr [rbp + 0x28], rdx
1415f5f58: add rsi, -0x20
1415f5f5c: lea rdi, [rcx + 0x28]
1415f5f60: mov qword ptr [rbp + 0xc8], rcx
1415f5f67: mov rax, rcx
1415f5f6a: jmp 0x1415f5f7f ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f5f6c: nop dword ptr [rax]
1415f5f70: add rsi, -0x20
1415f5f74: add rdi, 0x20
1415f5f78: mov rax, qword ptr [rbp + 0x188]
1415f5f7f: cmp rax, qword ptr [rbp + 0x190]
1415f5f86: je 0x1415f694c
1415f5f8c: lea rcx, [rax + 0x20]
1415f5f90: mov qword ptr [rbp + 0x188], rcx
1415f5f97: mov qword ptr [rbp + 0x18], rcx
1415f5f9b: mov rdx, qword ptr [rax]
1415f5f9e: mov rcx, qword ptr [rax + 0x18]
1415f5fa2: mov qword ptr [rbp + 0x40], rcx
1415f5fa6: movdqu xmm0, xmmword ptr [rax + 8]
1415f5fab: movdqa xmmword ptr [rbp + 0x30], xmm0
1415f5fb0: mov rax, rdx
1415f5fb3: neg rax
1415f5fb6: jo 0x1415f6955
1415f5fbc: mov qword ptr [rbp + 0x140], rdx
1415f5fc3: mov rax, qword ptr [rbp + 0x40]
1415f5fc7: lea rcx, [rbp + 0x148]
1415f5fce: mov qword ptr [rcx + 0x10], rax
1415f5fd2: movdqa xmm0, xmmword ptr [rbp + 0x30]
1415f5fd7: movdqu xmmword ptr [rcx], xmm0
1415f5fdb: cmp byte ptr [rip + 0x203edae], 0
1415f5fe2: je 0x1415f5ff1
1415f5fe4: cmp byte ptr [rbp + 0x158], 1
1415f5feb: je 0x1415f692c
1415f5ff1: lea rax, [rbp + 0x140]
1415f5ff8: mov qword ptr [rbp + 0x1b0], rax
1415f5fff: lea rax, [rip - 0x15ba536]
1415f6006: mov qword ptr [rbp + 0x1b8], rax
1415f600d: lea rax, [rip + 0x16873a4]
1415f6014: mov qword ptr [rbp + 0x240], rax
1415f601b: mov qword ptr [rbp + 0x248], 2
1415f6026: lea rax, [rbp + 0x1b0]
1415f602d: mov qword ptr [rbp + 0x250], rax
1415f6034: mov qword ptr [rbp + 0x258], 1
1415f603f: mov qword ptr [rbp + 0x260], 0
1415f604a: lea rcx, [rbp - 0x58]
1415f604e: mov rdx, r14
1415f6051: call 0x140d79f90 ; alloc::fmt::format::format_inner
1415f6056: movdqu xmm0, xmmword ptr [rbp - 0x58]
1415f605b: movdqa xmmword ptr [rbp + 0x120], xmm0
1415f6063: mov rax, qword ptr [rbp - 0x48]
1415f6067: mov qword ptr [rbp + 0x130], rax
1415f606e: mov byte ptr [rbp + 0x138], 1
1415f6075: mov byte ptr [rbp + 0x2d6], 1
1415f607c: mov rcx, r14
1415f607f: lea rdx, [rbp + 0x120]
1415f6086: call 0x140b11f50 ; country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> >
1415f608b: xor eax, eax
1415f608d: cmp rax, qword ptr [rbp + 0x240]
1415f6094: jo 0x1415f6af3
1415f609a: mov qword ptr [rbp + 0x98], rdi
1415f60a1: mov qword ptr [rbp + 0x90], rsi
1415f60a8: mov rax, qword ptr [rbp + 0x2b0]
1415f60af: mov qword ptr [rbp + 0x220], rax
1415f60b6: movups xmm0, xmmword ptr [rbp + 0x2a0]
1415f60bd: movaps xmmword ptr [rbp + 0x210], xmm0
1415f60c4: movups xmm0, xmmword ptr [rbp + 0x290]
1415f60cb: movaps xmmword ptr [rbp + 0x200], xmm0
1415f60d2: movups xmm0, xmmword ptr [rbp + 0x280]
1415f60d9: movaps xmmword ptr [rbp + 0x1f0], xmm0
1415f60e0: movdqu xmm0, xmmword ptr [rbp + 0x240]
1415f60e8: movdqu xmm1, xmmword ptr [rbp + 0x250]
1415f60f0: movdqu xmm2, xmmword ptr [rbp + 0x260]
1415f60f8: movups xmm3, xmmword ptr [rbp + 0x270]
1415f60ff: movaps xmmword ptr [rbp + 0x1e0], xmm3
1415f6106: movdqa xmmword ptr [rbp + 0x1d0], xmm2
1415f610e: movdqa xmmword ptr [rbp + 0x1c0], xmm1
1415f6116: movdqa xmmword ptr [rbp + 0x1b0], xmm0
1415f611e: test rbx, rbx
1415f6121: je 0x1415f61d0
1415f6127: movdqa xmm0, xmmword ptr [rbp + 0x120]
1415f612f: movdqa xmm1, xmmword ptr [rbp + 0x130]
1415f6137: movdqa xmmword ptr [rbp + 0x250], xmm1
1415f613f: movdqa xmmword ptr [rbp + 0x240], xmm0
1415f6147: movzx r8d, byte ptr [rbp + 0x118]
1415f614f: mov byte ptr [rbp + 0x2d7], 0
1415f6156: mov rcx, rbx
1415f6159: mov rdx, r14
1415f615c: call 0x1412a8fa0 ; country_core::startup::load_prefabs::PrefabWatcher::watch_atlas
1415f6161: mov dword ptr [rbp + 0x2c8], 0
1415f616b: cmp qword ptr [rbp + 0x200], 0
1415f6173: je 0x1415f61e2
1415f6175: mov rax, qword ptr [rbp + 0x1e8]
1415f617c: movdqa xmm0, xmmword ptr [rax]
1415f6180: pmovmskb ecx, xmm0
1415f6184: cmp ecx, 0xffff
1415f618a: jne 0x1415f61aa
1415f618c: lea rdx, [rax + 0x10]
1415f6190: movdqa xmm0, xmmword ptr [rdx]
1415f6194: pmovmskb ecx, xmm0
1415f6198: add rax, -0x380
1415f619e: add rdx, 0x10
1415f61a2: cmp ecx, 0xffff
1415f61a8: je 0x1415f6190
1415f61aa: not ecx
1415f61ac: tzcnt ecx, ecx
1415f61b0: neg rcx
1415f61b3: imul rcx, rcx, 0x38
1415f61b7: mov rax, qword ptr [rax + rcx - 8]
1415f61bc: mov qword ptr [rbp + 0x2b8], rax
1415f61c3: jmp 0x1415f61ed ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f61c5: nop word ptr cs:[rax + rax]
1415f61d0: mov al, 1
1415f61d2: mov dword ptr [rbp + 0x2c8], eax
1415f61d8: cmp qword ptr [rbp + 0x200], 0
1415f61e0: jne 0x1415f6175
1415f61e2: mov qword ptr [rbp + 0x2b8], 0
1415f61ed: mov rsi, qword ptr [rbp + 0xd8]
1415f61f4: mov rax, qword ptr [rbp + 0xe0]
1415f61fb: lea rax, [rax + rax*2]
1415f61ff: lea r15, [rsi + rax*8]
1415f6203: jmp 0x1415f6217 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6205: nop word ptr cs:[rax + rax]
1415f6210: lea r14, [rbp + 0x240]
1415f6217: cmp rsi, r15
1415f621a: je 0x1415f6330
1415f6220: mov eax, dword ptr [rbp + 0x2c8]
1415f6226: mov byte ptr [rbp + 0x2d7], al
1415f622c: mov rcx, r13
1415f622f: mov rdx, rsi
1415f6232: call 0x1405b18f0 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<alloc::string::String> >
1415f6237: mov ecx, dword ptr [rbp + 0x2c8]
1415f623d: mov byte ptr [rbp + 0x2d7], cl
1415f6243: mov rcx, r14
1415f6246: lea rdx, [rbp + 0x1e8]
1415f624d: mov r8, rax
1415f6250: mov r9, rsi
1415f6253: call 0x140b77de0 ; hashbrown::raw::RawTable::remove_entry<tuple$<alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<alloc::string::String,alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> > >
1415f6258: add rsi, 0x18
1415f625c: mov r14, qword ptr [rbp + 0x258]
1415f6263: cmp r14, 5
1415f6267: je 0x1415f6210
1415f6269: mov rdx, qword ptr [rbp + 0x240]
1415f6270: mov rdi, qword ptr [rbp + 0x260]
1415f6277: mov rbx, qword ptr [rbp + 0x268]
1415f627e: test rdx, rdx
1415f6281: je 0x1415f6295
1415f6283: mov rcx, qword ptr [rbp + 0x248]
1415f628a: mov r8d, 1
1415f6290: call 0x140613c20
1415f6295: cmp r14, 3
1415f6299: ja 0x1415f6302
1415f629b: lea rcx, [rip + 0x168759a]
1415f62a2: movsxd rax, dword ptr [rcx + r14*4]
1415f62a6: add rax, rcx
1415f62a9: jmp rax
1415f62ab: test rdi, rdi
1415f62ae: lea r14, [rbp + 0x240]
1415f62b5: je 0x1415f6217
1415f62bb: shl rdi, 2
1415f62bf: jmp 0x1415f6316 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f62c1: test rdi, rdi
1415f62c4: lea r14, [rbp + 0x240]
1415f62cb: je 0x1415f6217
1415f62d1: shl rdi, 3
1415f62d5: jmp 0x1415f6316 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f62d7: test rdi, rdi
1415f62da: lea r14, [rbp + 0x240]
1415f62e1: je 0x1415f6217
1415f62e7: shl rdi, 2
1415f62eb: lea rdx, [rdi + rdi*2]
1415f62ef: mov r8d, 4
1415f62f5: mov rcx, rbx
1415f62f8: call 0x140613c20
1415f62fd: jmp 0x1415f6217 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6302: test rdi, rdi
1415f6305: lea r14, [rbp + 0x240]
1415f630c: je 0x1415f6217
1415f6312: shl rdi, 4
1415f6316: mov r8d, 4
1415f631c: mov rcx, rbx
1415f631f: mov rdx, rdi
1415f6322: call 0x140613c20
1415f6327: jmp 0x1415f6217 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f632c: nop dword ptr [rax]
1415f6330: mov r12, qword ptr [rbp + 0xf0]
1415f6337: mov rax, qword ptr [rbp + 0xf8]
1415f633e: shl rax, 5
1415f6342: add rax, r12
1415f6345: mov qword ptr [rbp + 0x70], rax
1415f6349: mov r8, qword ptr [rbp + 0x2b8]
1415f6350: mov rdx, r8
1415f6353: shl rdx, 4
1415f6357: mov rax, r8
1415f635a: shr rax, 0x3c
1415f635e: setne al
1415f6361: movabs rcx, 0x7ffffffffffffffc
1415f636b: mov qword ptr [rbp + 0x170], rdx
1415f6372: cmp rdx, rcx
1415f6375: seta dl
1415f6378: or dl, al
1415f637a: mov byte ptr [rbp + 0x2c4], dl
1415f6380: mov rax, r8
1415f6383: mov edx, 0xc
1415f6388: mul rdx
1415f638b: seto dl
1415f638e: mov qword ptr [rbp + 0x178], rax
1415f6395: cmp rax, rcx
1415f6398: seta al
1415f639b: or al, dl
1415f639d: mov byte ptr [rbp + 0x2c5], al
1415f63a3: lea rdx, [r8*8]
1415f63ab: mov rax, r8
1415f63ae: shr rax, 0x3d
1415f63b2: setne al
1415f63b5: mov qword ptr [rbp + 0x180], rdx
1415f63bc: cmp rdx, rcx
1415f63bf: seta dl
1415f63c2: or dl, al
1415f63c4: mov byte ptr [rbp + 0x2c6], dl
1415f63ca: lea rdx, [r8*4]
1415f63d2: shr r8, 0x3e
1415f63d6: setne al
1415f63d9: mov qword ptr [rbp + 0x238], rdx
1415f63e0: cmp rdx, rcx
1415f63e3: seta cl
1415f63e6: or cl, al
1415f63e8: mov byte ptr [rbp + 0x2d5], cl
1415f63ee: jmp 0x1415f64b1 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f63f3: nop word ptr cs:[rax + rax]
1415f6400: movdqu xmm8, xmmword ptr [rbx + r14]
1415f6406: movdqa xmm0, xmm8
1415f640b: pcmpeqb xmm0, xmm7
1415f640f: pmovmskb r15d, xmm0
1415f6414: test r15d, r15d
1415f6417: je 0x1415f6470
1415f6419: mov qword ptr [rbp + 0x88], rcx
1415f6420: tzcnt eax, r15d
1415f6425: add rax, r14
1415f6428: and rax, rdi
1415f642b: neg rax
1415f642e: imul rax, rax, 0x38
1415f6432: mov r8, qword ptr [rbp + 0x78]
1415f6436: cmp qword ptr [r13 + rax + 0x10], r8
1415f643b: jne 0x1415f6454
1415f643d: add rax, r13
1415f6440: mov rdx, qword ptr [rax + 8]
1415f6444: mov rcx, qword ptr [rbp + 0x80]
1415f644b: call 0x142923bf0
1415f6450: test eax, eax
1415f6452: je 0x1415f64a0
1415f6454: lea eax, [r15 - 1]
1415f6458: and ax, r15w
1415f645c: mov r15d, eax
1415f645f: mov rcx, qword ptr [rbp + 0x88]
1415f6466: jne 0x1415f6420
1415f6468: nop dword ptr [rax + rax]
1415f6470: pcmpeqb xmm8, xmm6
1415f6475: pmovmskb eax, xmm8
1415f647a: test eax, eax
1415f647c: jne 0x1415f6540
1415f6482: add r14, rcx
1415f6485: add r14, 0x10
1415f6489: add rcx, 0x10
1415f648d: and r14, rdi
1415f6490: jmp 0x1415f6400 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6495: nop word ptr cs:[rax + rax]
1415f64a0: mov r12, qword ptr [rbp + 0x168]
1415f64a7: mov r13, rsi
1415f64aa: lea r14, [rbp + 0x240]
1415f64b1: cmp r12, qword ptr [rbp + 0x70]
1415f64b5: je 0x1415f6850
1415f64bb: lea rdi, [r12 + 0x20]
1415f64c0: mov r15, qword ptr [r12 + 8]
1415f64c5: mov rbx, qword ptr [r12 + 0x10]
1415f64ca: cmp qword ptr [rbp + 0x200], 0
1415f64d2: je 0x1415f655b
1415f64d8: mov eax, dword ptr [rbp + 0x2c8]
1415f64de: mov byte ptr [rbp + 0x2d7], al
1415f64e4: mov rcx, r13
1415f64e7: mov rdx, r15
1415f64ea: mov r8, rbx
1415f64ed: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1415f64f2: mov qword ptr [rbp + 0x80], r15
1415f64f9: mov qword ptr [rbp + 0x78], rbx
1415f64fd: mov qword ptr [rbp + 0x168], rdi
1415f6504: mov rsi, r13
1415f6507: mov rbx, qword ptr [rbp + 0x1e8]
1415f650e: mov rdi, qword ptr [rbp + 0x1f0]
1415f6515: mov r14, rdi
1415f6518: and r14, rax
1415f651b: shr rax, 0x39
1415f651f: movd xmm0, eax
1415f6523: punpcklbw xmm0, xmm0
1415f6527: pshuflw xmm0, xmm0, 0
1415f652c: pshufd xmm7, xmm0, 0
1415f6531: lea r13, [rbx - 0x38]
1415f6535: xor ecx, ecx
1415f6537: jmp 0x1415f6400 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f653c: nop dword ptr [rax]
1415f6540: mov r15, qword ptr [r12 + 8]
1415f6545: mov rbx, qword ptr [r12 + 0x10]
1415f654a: mov r13, rsi
1415f654d: lea r14, [rbp + 0x240]
1415f6554: mov rdi, qword ptr [rbp + 0x168]
1415f655b: movzx eax, byte ptr [r12 + 0x18]
1415f6561: lea rcx, [rip + 0x16872e4]
1415f6568: movsxd rax, dword ptr [rcx + rax*4]
1415f656c: add rax, rcx
1415f656f: jmp rax
1415f6571: cmp byte ptr [rbp + 0x2d5], 0
1415f6578: mov rsi, qword ptr [rbp + 0x2b8]
1415f657f: mov rcx, qword ptr [rbp + 0x238]
1415f6586: jne 0x1415f6a9f
1415f658c: test rcx, rcx
1415f658f: je 0x1415f67bc
1415f6595: movzx eax, byte ptr [rip + 0x203e66d]
1415f659c: mov r12d, 4
1415f65a2: mov edx, 4
1415f65a7: call 0x140613c40
1415f65ac: test rax, rax
1415f65af: je 0x1415f6aa2
1415f65b5: mov rcx, rsi
1415f65b8: jmp 0x1415f67c3 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f65bd: cmp byte ptr [rbp + 0x2c4], 0
1415f65c4: mov rsi, qword ptr [rbp + 0x2b8]
1415f65cb: mov rcx, qword ptr [rbp + 0x170]
1415f65d2: jne 0x1415f6ac9
1415f65d8: test rcx, rcx
1415f65db: je 0x1415f66ed
1415f65e1: movzx eax, byte ptr [rip + 0x203e621]
1415f65e8: mov r12d, 4
1415f65ee: mov edx, 4
1415f65f3: call 0x140613c40
1415f65f8: test rax, rax
1415f65fb: je 0x1415f6acc
1415f6601: mov rcx, rsi
1415f6604: jmp 0x1415f66f4 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6609: cmp byte ptr [rbp + 0x2c6], 0
1415f6610: mov rsi, qword ptr [rbp + 0x2b8]
1415f6617: jne 0x1415f6a75
1415f661d: mov rcx, qword ptr [rbp + 0x180]
1415f6624: test rcx, rcx
1415f6627: je 0x1415f6732
1415f662d: movzx eax, byte ptr [rip + 0x203e5d5]
1415f6634: mov r12d, 4
1415f663a: mov edx, 4
1415f663f: call 0x140613c40
1415f6644: test rax, rax
1415f6647: je 0x1415f6a78
1415f664d: mov rcx, rsi
1415f6650: jmp 0x1415f6739 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6655: cmp byte ptr [rbp + 0x2c5], 0
1415f665c: mov rsi, qword ptr [rbp + 0x2b8]
1415f6663: jne 0x1415f6a21
1415f6669: mov rcx, qword ptr [rbp + 0x178]
1415f6670: test rcx, rcx
1415f6673: je 0x1415f6777
1415f6679: movzx eax, byte ptr [rip + 0x203e589]
1415f6680: mov r12d, 4
1415f6686: mov edx, 4
1415f668b: call 0x140613c40
1415f6690: test rax, rax
1415f6693: je 0x1415f6a24
1415f6699: mov rcx, rsi
1415f669c: jmp 0x1415f677e ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f66a1: cmp byte ptr [rbp + 0x2d5], 0
1415f66a8: mov rsi, qword ptr [rbp + 0x2b8]
1415f66af: mov rcx, qword ptr [rbp + 0x238]
1415f66b6: jne 0x1415f6a4b
1415f66bc: test rcx, rcx
1415f66bf: je 0x1415f6801
1415f66c5: movzx eax, byte ptr [rip + 0x203e53d]
1415f66cc: mov r12d, 4
1415f66d2: mov edx, 4
1415f66d7: call 0x140613c40
1415f66dc: test rax, rax
1415f66df: je 0x1415f6a4e
1415f66e5: mov rcx, rsi
1415f66e8: jmp 0x1415f6808 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f66ed: mov eax, 4
1415f66f2: xor ecx, ecx
1415f66f4: mov qword ptr [rbp + 0x240], rcx
1415f66fb: mov qword ptr [rbp + 0x248], rax
1415f6702: mov qword ptr [rbp + 0x250], rsi
1415f6709: mov eax, dword ptr [rbp + 0x2c8]
1415f670f: mov byte ptr [rbp + 0x2d7], al
1415f6715: lea rcx, [rbp + 0x1b0]
1415f671c: mov rdx, r15
1415f671f: mov r8, rbx
1415f6722: mov r9, r14
1415f6725: call 0x140a34170 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,4>,alloc::alloc::Global> >
1415f672a: mov r12, rdi
1415f672d: jmp 0x1415f64b1 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6732: mov eax, 4
1415f6737: xor ecx, ecx
1415f6739: mov qword ptr [rbp + 0x240], rcx
1415f6740: mov qword ptr [rbp + 0x248], rax
1415f6747: mov qword ptr [rbp + 0x250], rsi
1415f674e: mov eax, dword ptr [rbp + 0x2c8]
1415f6754: mov byte ptr [rbp + 0x2d7], al
1415f675a: lea rcx, [rbp + 0x1b0]
1415f6761: mov rdx, r15
1415f6764: mov r8, rbx
1415f6767: mov r9, r14
1415f676a: call 0x140a355f0 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global> >
1415f676f: mov r12, rdi
1415f6772: jmp 0x1415f64b1 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6777: mov eax, 4
1415f677c: xor ecx, ecx
1415f677e: mov qword ptr [rbp + 0x240], rcx
1415f6785: mov qword ptr [rbp + 0x248], rax
1415f678c: mov qword ptr [rbp + 0x250], rsi
1415f6793: mov eax, dword ptr [rbp + 0x2c8]
1415f6799: mov byte ptr [rbp + 0x2d7], al
1415f679f: lea rcx, [rbp + 0x1b0]
1415f67a6: mov rdx, r15
1415f67a9: mov r8, rbx
1415f67ac: mov r9, r14
1415f67af: call 0x140a34c90 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> >
1415f67b4: mov r12, rdi
1415f67b7: jmp 0x1415f64b1 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f67bc: mov eax, 4
1415f67c1: xor ecx, ecx
1415f67c3: mov qword ptr [rbp + 0x240], rcx
1415f67ca: mov qword ptr [rbp + 0x248], rax
1415f67d1: mov qword ptr [rbp + 0x250], rsi
1415f67d8: mov eax, dword ptr [rbp + 0x2c8]
1415f67de: mov byte ptr [rbp + 0x2d7], al
1415f67e4: lea rcx, [rbp + 0x1b0]
1415f67eb: mov rdx, r15
1415f67ee: mov r8, rbx
1415f67f1: mov r9, r14
1415f67f4: call 0x140a34700 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<i32,alloc::alloc::Global> >
1415f67f9: mov r12, rdi
1415f67fc: jmp 0x1415f64b1 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6801: mov eax, 4
1415f6806: xor ecx, ecx
1415f6808: mov qword ptr [rbp + 0x240], rcx
1415f680f: mov qword ptr [rbp + 0x248], rax
1415f6816: mov qword ptr [rbp + 0x250], rsi
1415f681d: mov eax, dword ptr [rbp + 0x2c8]
1415f6823: mov byte ptr [rbp + 0x2d7], al
1415f6829: lea rcx, [rbp + 0x1b0]
1415f6830: mov rdx, r15
1415f6833: mov r8, rbx
1415f6836: mov r9, r14
1415f6839: call 0x140a35b80 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<f32,alloc::alloc::Global> >
1415f683e: mov r12, rdi
1415f6841: jmp 0x1415f64b1 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6846: nop word ptr cs:[rax + rax]
1415f6850: movzx edx, byte ptr [rbp + 0x118]
1415f6857: mov r8, qword ptr [rbp + 0x148]
1415f685e: mov r9, qword ptr [rbp + 0x150]
1415f6865: mov eax, dword ptr [rbp + 0x2c8]
1415f686b: mov byte ptr [rbp + 0x2d7], al
1415f6871: lea rax, [rbp + 0x1b0]
1415f6878: mov qword ptr [rsp + 0x20], rax
1415f687d: mov rcx, qword ptr [rbp + 0xa0]
1415f6884: call 0x140a371d0 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> >
1415f6889: mov r8d, 0x38
1415f688f: mov r9d, 0x10
1415f6895: lea rcx, [rbp + 0x1e8]
1415f689c: mov rdx, r13
1415f689f: call 0x140b75360 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> >,alloc::alloc::Global>
1415f68a4: mov rdx, qword ptr [rbp + 0x1b0]
1415f68ab: test rdx, rdx
1415f68ae: mov rsi, qword ptr [rbp + 0x90]
1415f68b5: mov rdi, qword ptr [rbp + 0x98]
1415f68bc: je 0x1415f68d4
1415f68be: mov rcx, qword ptr [rbp + 0x1b8]
1415f68c5: shl rdx, 2
1415f68c9: mov r8d, 4
1415f68cf: call 0x140613c20
1415f68d4: mov rdx, qword ptr [rbp + 0x1c8]
1415f68db: mov rax, rdx
1415f68de: neg rax
1415f68e1: jo 0x1415f68f7
1415f68e3: jae 0x1415f68f7
1415f68e5: mov rcx, qword ptr [rbp + 0x1d0]
1415f68ec: mov r8d, 1
1415f68f2: call 0x140613c20
1415f68f7: cmp byte ptr [rbp + 0x2c8], 0
1415f68fe: mov rbx, qword ptr [rbp + 0xa8]
1415f6905: je 0x1415f6925
1415f6907: mov rdx, qword ptr [rbp + 0x120]
1415f690e: test rdx, rdx
1415f6911: je 0x1415f6925
1415f6913: mov rcx, qword ptr [rbp + 0x128]
1415f691a: mov r8d, 1
1415f6920: call 0x140613c20
1415f6925: mov rdx, qword ptr [rbp + 0x140]
1415f692c: test rdx, rdx
1415f692f: je 0x1415f5f70
1415f6935: mov rcx, qword ptr [rbp + 0x148]
1415f693c: mov r8d, 1
1415f6942: call 0x140613c20
1415f6947: jmp 0x1415f5f70 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f694c: mov rax, qword ptr [rbp + 0x230]
1415f6953: jmp 0x1415f69b0 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6955: mov rax, qword ptr [rbp + 0x188]
1415f695c: cmp qword ptr [rbp + 0x190], rax
1415f6963: mov rax, qword ptr [rbp + 0x230]
1415f696a: je 0x1415f69b0
1415f696c: shr rsi, 5
1415f6970: jmp 0x1415f6989 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6972: nop word ptr cs:[rax + rax]
1415f6980: add rdi, 0x20
1415f6984: dec rsi
1415f6987: je 0x1415f69b0
1415f6989: mov rdx, qword ptr [rdi - 8]
1415f698d: test rdx, rdx
1415f6990: je 0x1415f6980
1415f6992: mov rcx, qword ptr [rdi]
1415f6995: mov r8d, 1
1415f699b: call 0x140613c20
1415f69a0: mov rax, qword ptr [rbp + 0x230]
1415f69a7: jmp 0x1415f6980 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f69a9: nop dword ptr [rax]
1415f69b0: test rax, rax
1415f69b3: je 0x1415f69ce
1415f69b5: shl rax, 5
1415f69b9: mov r8d, 8
1415f69bf: mov rcx, qword ptr [rbp + 0xc8]
1415f69c6: mov rdx, rax
1415f69c9: call 0x140613c20
1415f69ce: lea rcx, [rbp + 0xd0]
1415f69d5: call 0x1408919b0 ; core::ptr::drop_in_place<country_core::startup::load_mesh_atlases::MeshAttribConfig>
1415f69da: mov rax, qword ptr [rbp + 0x198]
1415f69e1: cmp rax, qword ptr [rbp + 0xb0]
1415f69e8: jne 0x1415f5d44
1415f69ee: lea rcx, [rbp + 0x50]
1415f69f2: call 0x14087bd60 ; core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >
1415f69f7: movaps xmm6, xmmword ptr [rbp + 0x2e0]
1415f69fe: movaps xmm7, xmmword ptr [rbp + 0x2f0]
1415f6a05: movaps xmm8, xmmword ptr [rbp + 0x300]
1415f6a0d: add rsp, 0x398
1415f6a14: pop rbx
1415f6a15: pop rdi
1415f6a16: pop rsi
1415f6a17: pop r12
1415f6a19: pop r13
1415f6a1b: pop r14
1415f6a1d: pop r15
1415f6a1f: pop rbp
1415f6a20: ret 
1415f6a21: xor r12d, r12d
1415f6a24: mov eax, dword ptr [rbp + 0x2c8]
1415f6a2a: mov byte ptr [rbp + 0x2d7], al
1415f6a30: lea r8, [rip + 0x1686a01]
1415f6a37: mov rcx, r12
1415f6a3a: mov rdx, qword ptr [rbp + 0x178]
1415f6a41: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1415f6a46: jmp 0x1415f6c57 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6a4b: xor r12d, r12d
1415f6a4e: mov eax, dword ptr [rbp + 0x2c8]
1415f6a54: mov byte ptr [rbp + 0x2d7], al
1415f6a5a: lea r8, [rip + 0x16869a7]
1415f6a61: mov rcx, r12
1415f6a64: mov rdx, qword ptr [rbp + 0x238]
1415f6a6b: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1415f6a70: jmp 0x1415f6c57 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6a75: xor r12d, r12d
1415f6a78: mov eax, dword ptr [rbp + 0x2c8]
1415f6a7e: mov byte ptr [rbp + 0x2d7], al
1415f6a84: lea r8, [rip + 0x1686995]
1415f6a8b: mov rcx, r12
1415f6a8e: mov rdx, qword ptr [rbp + 0x180]
1415f6a95: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1415f6a9a: jmp 0x1415f6c57 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6a9f: xor r12d, r12d
1415f6aa2: mov eax, dword ptr [rbp + 0x2c8]
1415f6aa8: mov byte ptr [rbp + 0x2d7], al
1415f6aae: lea r8, [rip + 0x168693b]
1415f6ab5: mov rcx, r12
1415f6ab8: mov rdx, qword ptr [rbp + 0x238]
1415f6abf: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1415f6ac4: jmp 0x1415f6c57 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6ac9: xor r12d, r12d
1415f6acc: mov eax, dword ptr [rbp + 0x2c8]
1415f6ad2: mov byte ptr [rbp + 0x2d7], al
1415f6ad8: lea r8, [rip + 0x1686971]
1415f6adf: mov rcx, r12
1415f6ae2: mov rdx, qword ptr [rbp + 0x170]
1415f6ae9: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1415f6aee: jmp 0x1415f6c57 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6af3: mov rax, qword ptr [rbp + 0x248]
1415f6afa: mov qword ptr [rbp + 0x160], rax
1415f6b01: lea rax, [rip + 0x16868d0]
1415f6b08: mov qword ptr [rsp + 0x20], rax
1415f6b0d: lea rcx, [rip + 0x1685e74]
1415f6b14: lea r9, [rip + 0x1685e4d]
1415f6b1b: lea r8, [rbp + 0x160]
1415f6b22: mov edx, 0x2b
1415f6b27: call 0x1428d9760 ; core::result::unwrap_failed
1415f6b2c: jmp 0x1415f6c57 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6b31: lea rax, [rip + 0x1686840]
1415f6b38: mov qword ptr [rbp + 0x240], rax
1415f6b3f: mov qword ptr [rbp + 0x248], 1
1415f6b4a: mov qword ptr [rbp + 0x250], 8
1415f6b55: pxor xmm0, xmm0
1415f6b59: movdqu xmmword ptr [rbp + 0x258], xmm0
1415f6b61: lea rdx, [rip + 0x1686820]
1415f6b68: lea rcx, [rbp + 0x240]
1415f6b6f: call 0x1428d9390 ; core::panicking::panic_fmt
1415f6b74: jmp 0x1415f6c57 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6b79: mov qword ptr [rbp + 0x240], rdi
1415f6b80: lea rax, [rip + 0x1686769]
1415f6b87: mov qword ptr [rsp + 0x20], rax
1415f6b8c: lea rcx, [rip + 0x168670e]
1415f6b93: lea r9, [rip + 0x1685d8e]
1415f6b9a: lea r8, [rbp + 0x240]
1415f6ba1: mov edx, 0x14
1415f6ba6: call 0x1428d9760 ; core::result::unwrap_failed
1415f6bab: jmp 0x1415f6c57 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6bb0: mov qword ptr [rbp + 0x2c8], rdi
1415f6bb7: lea r8, [rip + 0x169040a]
1415f6bbe: mov ecx, 1
1415f6bc3: mov edx, 0x2000
1415f6bc8: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1415f6bcd: jmp 0x1415f6c57 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::reload_mesh_atlases::closure_env$1>
1415f6bd2: mov rax, qword ptr [rbp + 0x200]
1415f6bd9: mov qword ptr [rbp + 0x290], rax
1415f6be0: movups xmm0, xmmword ptr [rbp + 0x1f0]
1415f6be7: movaps xmmword ptr [rbp + 0x280], xmm0
1415f6bee: movdqu xmm0, xmmword ptr [rbp + 0x1b0]
1415f6bf6: movdqu xmm1, xmmword ptr [rbp + 0x1c0]
1415f6bfe: movdqu xmm2, xmmword ptr [rbp + 0x1d0]
1415f6c06: movups xmm3, xmmword ptr [rbp + 0x1e0]
1415f6c0d: movaps xmmword ptr [rbp + 0x270], xmm3
1415f6c14: movdqa xmmword ptr [rbp + 0x260], xmm2
1415f6c1c: movdqa xmmword ptr [rbp + 0x250], xmm1
1415f6c24: movdqa xmmword ptr [rbp + 0x240], xmm0
1415f6c2c: lea rax, [rip + 0x16866f5]
1415f6c33: mov qword ptr [rsp + 0x20], rax
1415f6c38: lea rcx, [rip + 0x16866c9]
1415f6c3f: lea r9, [rip + 0x1685d02]
1415f6c46: lea r8, [rbp + 0x240]
1415f6c4d: mov edx, 0x1b
1415f6c52: call 0x1428d9760 ; core::result::unwrap_failed
1415f6c57: ud2 
1415f6c59: nop dword ptr [rax]
1415f6c60: mov qword ptr [rsp + 0x10], rdx
1415f6c65: push rbp
1415f6c66: push r15
1415f6c68: push r14
1415f6c6a: push r13
1415f6c6c: push r12
1415f6c6e: push rsi
1415f6c6f: push rdi
1415f6c70: push rbx
1415f6c71: sub rsp, 0x58
1415f6c75: lea rbp, [rdx + 0x80]
1415f6c7c: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f6c83: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f6c89: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f6c8f: mov rcx, qword ptr [rbp + 0x2c8]
1415f6c96: call qword ptr [rip + 0x1b9b754]
1415f6c9c: movaps xmm6, xmmword ptr [rsp + 0x40]
1415f6ca1: movaps xmm7, xmmword ptr [rsp + 0x30]
1415f6ca6: movaps xmm8, xmmword ptr [rsp + 0x20]
1415f6cac: add rsp, 0x58
1415f6cb0: pop rbx
1415f6cb1: pop rdi
1415f6cb2: pop rsi
1415f6cb3: pop r12
1415f6cb5: pop r13
1415f6cb7: pop r14
1415f6cb9: pop r15
1415f6cbb: pop rbp
1415f6cbc: ret 
1415f6cbd: nop dword ptr [rax]
1415f6cc0: mov qword ptr [rsp + 0x10], rdx
1415f6cc5: push rbp
1415f6cc6: push r15
1415f6cc8: push r14
1415f6cca: push r13
1415f6ccc: push r12
1415f6cce: push rsi
1415f6ccf: push rdi
1415f6cd0: push rbx
1415f6cd1: sub rsp, 0x58
1415f6cd5: lea rbp, [rdx + 0x80]
1415f6cdc: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f6ce3: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f6ce9: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f6cef: lea rcx, [rbp + 0x100]
1415f6cf6: call 0x1405d8fc0 ; core::ptr::drop_in_place<regex_syntax::hir::literal::Literals>
1415f6cfb: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f6d01: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f6d07: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f6d0e: add rsp, 0x58
1415f6d12: pop rbx
1415f6d13: pop rdi
1415f6d14: pop rsi
1415f6d15: pop r12
1415f6d17: pop r13
1415f6d19: pop r14
1415f6d1b: pop r15
1415f6d1d: pop rbp
1415f6d1e: ret 
1415f6d1f: nop 
1415f6d20: mov qword ptr [rsp + 0x10], rdx
1415f6d25: push rbp
1415f6d26: push r15
1415f6d28: push r14
1415f6d2a: push r13
1415f6d2c: push r12
1415f6d2e: push rsi
1415f6d2f: push rdi
1415f6d30: push rbx
1415f6d31: sub rsp, 0x58
1415f6d35: lea rbp, [rdx + 0x80]
1415f6d3c: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f6d43: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f6d49: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f6d4f: mov rdx, qword ptr [rbp + 0x140]
1415f6d56: test rdx, rdx
1415f6d59: je 0x1415f6d6d
1415f6d5b: mov rcx, qword ptr [rbp + 0x148]
1415f6d62: mov r8d, 1
1415f6d68: call 0x140613c20
1415f6d6d: lea rcx, [rbp + 0x10]
1415f6d71: call 0x1405dd0b0 ; core::ptr::drop_in_place<core::iter::adapters::map::Map<std::env::ArgsOs,clap_lex::impl$1::from::closure_env$0<std::env::ArgsOs,std::ffi::os_str::OsString> > >
1415f6d76: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f6d7c: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f6d82: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f6d89: add rsp, 0x58
1415f6d8d: pop rbx
1415f6d8e: pop rdi
1415f6d8f: pop rsi
1415f6d90: pop r12
1415f6d92: pop r13
1415f6d94: pop r14
1415f6d96: pop r15
1415f6d98: pop rbp
1415f6d99: ret 
1415f6d9a: nop word ptr [rax + rax]
1415f6da0: mov qword ptr [rsp + 0x10], rdx
1415f6da5: push rbp
1415f6da6: push r15
1415f6da8: push r14
1415f6daa: push r13
1415f6dac: push r12
1415f6dae: push rsi
1415f6daf: push rdi
1415f6db0: push rbx
1415f6db1: sub rsp, 0x58
1415f6db5: lea rbp, [rdx + 0x80]
1415f6dbc: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f6dc3: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f6dc9: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f6dcf: movzx ebx, byte ptr [rbp + 0x2d7]
1415f6dd6: lea rcx, [rbp + 0x1b0]
1415f6ddd: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
1415f6de2: mov byte ptr [rbp + 0x2d6], bl
1415f6de8: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f6dee: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f6df4: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f6dfb: add rsp, 0x58
1415f6dff: pop rbx
1415f6e00: pop rdi
1415f6e01: pop rsi
1415f6e02: pop r12
1415f6e04: pop r13
1415f6e06: pop r14
1415f6e08: pop r15
1415f6e0a: pop rbp
1415f6e0b: ret 
1415f6e0c: nop dword ptr [rax]
1415f6e10: mov qword ptr [rsp + 0x10], rdx
1415f6e15: push rbp
1415f6e16: push r15
1415f6e18: push r14
1415f6e1a: push r13
1415f6e1c: push r12
1415f6e1e: push rsi
1415f6e1f: push rdi
1415f6e20: push rbx
1415f6e21: sub rsp, 0x58
1415f6e25: lea rbp, [rdx + 0x80]
1415f6e2c: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f6e33: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f6e39: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f6e3f: mov rdx, qword ptr [rbp + 0x1b0]
1415f6e46: test rdx, rdx
1415f6e49: je 0x1415f6e61
1415f6e4b: mov rcx, qword ptr [rbp + 0x1b8]
1415f6e52: shl rdx, 2
1415f6e56: mov r8d, 4
1415f6e5c: call 0x140613c20
1415f6e61: mov rdx, qword ptr [rbp + 0x1c8]
1415f6e68: lea rax, [rdx*2]
1415f6e70: test rax, rax
1415f6e73: je 0x1415f6e87
1415f6e75: mov rcx, qword ptr [rbp + 0x1d0]
1415f6e7c: mov r8d, 1
1415f6e82: call 0x140613c20
1415f6e87: mov eax, dword ptr [rbp + 0x2c8]
1415f6e8d: mov byte ptr [rbp + 0x2d6], al
1415f6e93: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f6e99: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f6e9f: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f6ea6: add rsp, 0x58
1415f6eaa: pop rbx
1415f6eab: pop rdi
1415f6eac: pop rsi
1415f6ead: pop r12
1415f6eaf: pop r13
1415f6eb1: pop r14
1415f6eb3: pop r15
1415f6eb5: pop rbp
1415f6eb6: ret 
1415f6eb7: nop word ptr [rax + rax]
1415f6ec0: mov qword ptr [rsp + 0x10], rdx
1415f6ec5: push rbp
1415f6ec6: push r15
1415f6ec8: push r14
1415f6eca: push r13
1415f6ecc: push r12
1415f6ece: push rsi
1415f6ecf: push rdi
1415f6ed0: push rbx
1415f6ed1: sub rsp, 0x58
1415f6ed5: lea rbp, [rdx + 0x80]
1415f6edc: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f6ee3: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f6ee9: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f6eef: test byte ptr [rbp + 0x2d6], 1
1415f6ef6: je 0x1415f6f3a
1415f6ef8: mov rdx, qword ptr [rbp + 0x120]
1415f6eff: test rdx, rdx
1415f6f02: je 0x1415f6f16
1415f6f04: mov rcx, qword ptr [rbp + 0x128]
1415f6f0b: mov r8d, 1
1415f6f11: call 0x140613c20
1415f6f16: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f6f1c: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f6f22: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f6f29: add rsp, 0x58
1415f6f2d: pop rbx
1415f6f2e: pop rdi
1415f6f2f: pop rsi
1415f6f30: pop r12
1415f6f32: pop r13
1415f6f34: pop r14
1415f6f36: pop r15
1415f6f38: pop rbp
1415f6f39: ret 
1415f6f3a: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f6f40: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f6f46: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f6f4d: add rsp, 0x58
1415f6f51: pop rbx
1415f6f52: pop rdi
1415f6f53: pop rsi
1415f6f54: pop r12
1415f6f56: pop r13
1415f6f58: pop r14
1415f6f5a: pop r15
1415f6f5c: pop rbp
1415f6f5d: ret 
1415f6f5e: nop 
1415f6f60: mov qword ptr [rsp + 0x10], rdx
1415f6f65: push rbp
1415f6f66: push r15
1415f6f68: push r14
1415f6f6a: push r13
1415f6f6c: push r12
1415f6f6e: push rsi
1415f6f6f: push rdi
1415f6f70: push rbx
1415f6f71: sub rsp, 0x58
1415f6f75: lea rbp, [rdx + 0x80]
1415f6f7c: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f6f83: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f6f89: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f6f8f: lea rcx, [rbp + 0xd0]
1415f6f96: call 0x1408919b0 ; core::ptr::drop_in_place<country_core::startup::load_mesh_atlases::MeshAttribConfig>
1415f6f9b: cmp byte ptr [rbp + 0x2c7], 9
1415f6fa2: jne 0x1415f6fd4
1415f6fa4: lea rcx, [rbp + 0x100]
1415f6fab: call 0x1405d8fc0 ; core::ptr::drop_in_place<regex_syntax::hir::literal::Literals>
1415f6fb0: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f6fb6: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f6fbc: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f6fc3: add rsp, 0x58
1415f6fc7: pop rbx
1415f6fc8: pop rdi
1415f6fc9: pop rsi
1415f6fca: pop r12
1415f6fcc: pop r13
1415f6fce: pop r14
1415f6fd0: pop r15
1415f6fd2: pop rbp
1415f6fd3: ret 
1415f6fd4: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f6fda: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f6fe0: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f6fe7: add rsp, 0x58
1415f6feb: pop rbx
1415f6fec: pop rdi
1415f6fed: pop rsi
1415f6fee: pop r12
1415f6ff0: pop r13
1415f6ff2: pop r14
1415f6ff4: pop r15
1415f6ff6: pop rbp
1415f6ff7: ret 
1415f6ff8: nop dword ptr [rax + rax]
1415f7000: mov qword ptr [rsp + 0x10], rdx
1415f7005: push rbp
1415f7006: push r15
1415f7008: push r14
1415f700a: push r13
1415f700c: push r12
1415f700e: push rsi
1415f700f: push rdi
1415f7010: push rbx
1415f7011: sub rsp, 0x58
1415f7015: lea rbp, [rdx + 0x80]
1415f701c: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f7023: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f7029: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f702f: lea rcx, [rbp + 0x240]
1415f7036: call 0x1405e7a00 ; core::ptr::drop_in_place<std::io::error::Error>
1415f703b: movaps xmm6, xmmword ptr [rsp + 0x40]
1415f7040: movaps xmm7, xmmword ptr [rsp + 0x30]
1415f7045: movaps xmm8, xmmword ptr [rsp + 0x20]
1415f704b: add rsp, 0x58
1415f704f: pop rbx
1415f7050: pop rdi
1415f7051: pop rsi
1415f7052: pop r12
1415f7054: pop r13
1415f7056: pop r14
1415f7058: pop r15
1415f705a: pop rbp
1415f705b: ret 
1415f705c: nop dword ptr [rax]
1415f7060: mov qword ptr [rsp + 0x10], rdx
1415f7065: push rbp
1415f7066: push r15
1415f7068: push r14
1415f706a: push r13
1415f706c: push r12
1415f706e: push rsi
1415f706f: push rdi
1415f7070: push rbx
1415f7071: sub rsp, 0x58
1415f7075: lea rbp, [rdx + 0x80]
1415f707c: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f7083: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f7089: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f708f: lea rcx, [rbp + 0x240]
1415f7096: call 0x140887a50 ; core::ptr::drop_in_place<ron::error::SpannedError>
1415f709b: movaps xmm6, xmmword ptr [rsp + 0x40]
1415f70a0: movaps xmm7, xmmword ptr [rsp + 0x30]
1415f70a5: movaps xmm8, xmmword ptr [rsp + 0x20]
1415f70ab: add rsp, 0x58
1415f70af: pop rbx
1415f70b0: pop rdi
1415f70b1: pop rsi
1415f70b2: pop r12
1415f70b4: pop r13
1415f70b6: pop r14
1415f70b8: pop r15
1415f70ba: pop rbp
1415f70bb: ret 
1415f70bc: nop dword ptr [rax]
1415f70c0: mov qword ptr [rsp + 0x10], rdx
1415f70c5: push rbp
1415f70c6: push r15
1415f70c8: push r14
1415f70ca: push r13
1415f70cc: push r12
1415f70ce: push rsi
1415f70cf: push rdi
1415f70d0: push rbx
1415f70d1: sub rsp, 0x58
1415f70d5: lea rbp, [rdx + 0x80]
1415f70dc: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f70e3: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f70e9: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f70ef: lea rcx, [rbp + 0x50]
1415f70f3: call 0x14087bd60 ; core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >
1415f70f8: movaps xmm6, xmmword ptr [rsp + 0x40]
1415f70fd: movaps xmm7, xmmword ptr [rsp + 0x30]
1415f7102: movaps xmm8, xmmword ptr [rsp + 0x20]
1415f7108: add rsp, 0x58
1415f710c: pop rbx
1415f710d: pop rdi
1415f710e: pop rsi
1415f710f: pop r12
1415f7111: pop r13
1415f7113: pop r14
1415f7115: pop r15
1415f7117: pop rbp
1415f7118: ret 
1415f7119: nop dword ptr [rax]
1415f7120: mov qword ptr [rsp + 0x10], rdx
1415f7125: push rbp
1415f7126: push r15
1415f7128: push r14
1415f712a: push r13
1415f712c: push r12
1415f712e: push rsi
1415f712f: push rdi
1415f7130: push rbx
1415f7131: sub rsp, 0x58
1415f7135: lea rbp, [rdx + 0x80]
1415f713c: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f7143: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f7149: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f714f: mov byte ptr [rbp + 0x2d6], 1
1415f7156: lea rcx, [rbp + 0x160]
1415f715d: call 0x140d4cc30 ; anyhow::error::impl$6::drop
1415f7162: mov byte ptr [rbp + 0x2d6], 1
1415f7169: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f716f: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f7175: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f717c: add rsp, 0x58
1415f7180: pop rbx
1415f7181: pop rdi
1415f7182: pop rsi
1415f7183: pop r12
1415f7185: pop r13
1415f7187: pop r14
1415f7189: pop r15
1415f718b: pop rbp
1415f718c: ret 
