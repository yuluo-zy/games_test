14129f400: push rbp
14129f401: push r15
14129f403: push r14
14129f405: push r13
14129f407: push r12
14129f409: push rsi
14129f40a: push rdi
14129f40b: push rbx
14129f40c: sub rsp, 0x298
14129f413: lea rbp, [rsp + 0x80]
14129f41b: movaps xmmword ptr [rbp + 0x200], xmm12
14129f423: movaps xmmword ptr [rbp + 0x1f0], xmm11
14129f42b: movaps xmmword ptr [rbp + 0x1e0], xmm10
14129f433: movaps xmmword ptr [rbp + 0x1d0], xmm9
14129f43b: movaps xmmword ptr [rbp + 0x1c0], xmm8
14129f443: movaps xmmword ptr [rbp + 0x1b0], xmm7
14129f44a: movaps xmmword ptr [rbp + 0x1a0], xmm6
14129f451: mov qword ptr [rbp + 0x198], 0xfffffffffffffffe
14129f45c: mov qword ptr [rbp + 0x180], r9
14129f463: mov qword ptr [rbp - 0x58], r8
14129f467: mov rax, qword ptr [rcx]
14129f46a: mov r8, qword ptr [rax + 0x18]
14129f46e: mov ecx, dword ptr [rdx + 0x1c]
14129f471: mov dword ptr [rbp + 0x194], ecx
14129f477: mov rcx, qword ptr [rdx]
14129f47a: mov qword ptr [rbp + 0x168], rcx
14129f481: mov rcx, qword ptr [rdx + 0x10]
14129f485: mov qword ptr [rbp + 0x170], rcx
14129f48c: test r8, r8
14129f48f: je 0x14129f5a4
14129f495: mov r12, qword ptr [rax]
14129f498: lea r13, [r12 + 0x10]
14129f49d: movdqa xmm0, xmmword ptr [r12]
14129f4a3: pmovmskb r14d, xmm0
14129f4a8: not r14d
14129f4ab: jmp 0x14129f4c0 ; country_core::startup::load_mesh_atlases::load_mesh_atlases
14129f4ad: nop dword ptr [rax]
14129f4b0: mov r8, qword ptr [rbp + 0x188]
14129f4b7: dec r8
14129f4ba: je 0x14129f5a4
14129f4c0: test r14w, r14w
14129f4c4: mov qword ptr [rbp + 0x188], r8
14129f4cb: je 0x14129f4e0
14129f4cd: lea eax, [r14 - 1]
14129f4d1: and eax, r14d
14129f4d4: jmp 0x14129f50f ; country_core::startup::load_mesh_atlases::load_mesh_atlases
14129f4d6: nop word ptr cs:[rax + rax]
14129f4e0: movdqa xmm0, xmmword ptr [r13]
14129f4e6: pmovmskb r14d, xmm0
14129f4eb: add r12, -0x280
14129f4f2: add r13, 0x10
14129f4f6: cmp r14d, 0xffff
14129f4fd: je 0x14129f4e0
14129f4ff: mov ecx, 0xfffffffe
14129f504: sub ecx, r14d
14129f507: not r14d
14129f50a: mov eax, r14d
14129f50d: and eax, ecx
14129f50f: mov ecx, r14d
14129f512: tzcnt ecx, ecx
14129f516: mov r14d, eax
14129f519: neg rcx
14129f51c: lea rcx, [rcx + rcx*4]
14129f520: mov rax, qword ptr [r12 + rcx*8 - 8]
14129f525: test rax, rax
14129f528: je 0x14129f4b0
14129f52a: lea rsi, [r12 + rcx*8]
14129f52e: mov rdi, qword ptr [r12 + rcx*8 - 0x10]
14129f533: imul rbx, rax, 0x98
14129f53a: xor r15d, r15d
14129f53d: jmp 0x14129f56d ; country_core::startup::load_mesh_atlases::load_mesh_atlases
14129f53f: nop 
14129f540: mov rax, qword ptr [rdi + r15 + 0x88]
14129f548: add rax, 0x10
14129f54c: mov qword ptr [rsp + 0x20], rax
14129f551: mov rcx, qword ptr [rbp + 0x168]
14129f558: call 0x140a37760 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<asset::prefab::PrefabName>
14129f55d: add r15, 0x98
14129f564: cmp rbx, r15
14129f567: je 0x14129f4b0
14129f56d: mov rax, qword ptr [rdi + r15 + 0x88]
14129f575: cmp qword ptr [rax + 0x20], 0
14129f57a: je 0x14129f55d
14129f57c: mov eax, dword ptr [rbp + 0x194]
14129f582: mov rcx, qword ptr [rbp + 0x170]
14129f589: mov dword ptr [rcx], eax
14129f58b: movzx edx, byte ptr [rdi + r15 + 0x90]
14129f594: mov r8, qword ptr [rsi - 0x28]
14129f598: mov r9, qword ptr [rsi - 0x20]
14129f59c: lock inc qword ptr [r8]
14129f5a0: jg 0x14129f540
14129f5a2: ud2 
14129f5a4: mov eax, dword ptr [rbp + 0x194]
14129f5aa: mov rcx, qword ptr [rbp + 0x170]
14129f5b1: mov dword ptr [rcx], eax
14129f5b3: mov rcx, qword ptr [rbp + 0x180]
14129f5ba: mov rdx, qword ptr [rcx]
14129f5bd: test rdx, rdx
14129f5c0: je 0x14129f5cb
14129f5c2: mov eax, dword ptr [rcx + 0x1c]
14129f5c5: mov rcx, qword ptr [rcx + 0x10]
14129f5c9: mov dword ptr [rcx], eax
14129f5cb: mov rcx, qword ptr [rbp + 0x168]
14129f5d2: call 0x1415f7190 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
14129f5d7: mov eax, dword ptr [rbp + 0x194]
14129f5dd: mov rcx, qword ptr [rbp + 0x170]
14129f5e4: mov dword ptr [rcx], eax
14129f5e6: xor ecx, ecx
14129f5e8: lea r12, [rbp + 0x60]
14129f5ec: lea rsi, [rbp + 0x178]
14129f5f3: lea rdi, [rbp - 0x50]
14129f5f7: lea rbx, [rbp + 0x17c]
14129f5fe: movss xmm6, dword ptr [rip + 0x174e59e]
14129f606: movss xmm7, dword ptr [rip + 0x1686262]
14129f60e: movaps xmm8, xmmword ptr [rip + 0x16862da]
14129f616: movaps xmm9, xmmword ptr [rip + 0x1686312]
14129f61e: movss xmm10, dword ptr [rip + 0x17edf9d]
14129f627: movss xmm11, dword ptr [rip + 0x16862d4]
14129f630: xor r13d, r13d
14129f633: jmp 0x14129f64b ; country_core::startup::load_mesh_atlases::load_mesh_atlases
14129f635: nop word ptr cs:[rax + rax]
14129f640: mov rcx, rdi
14129f643: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
14129f648: mov ecx, r14d
14129f64b: test r13b, 1
14129f64f: jne 0x14129f93c
14129f655: cmp ecx, 5
14129f658: ja 0x14129f93c
14129f65e: lea r14d, [rcx + 1]
14129f662: cmp ecx, 5
14129f665: mov eax, 5
14129f66a: cmovae r14d, eax
14129f66e: setae r13b
14129f672: mov dword ptr [rbp + 0x17c], ecx
14129f678: mov r15d, 1
14129f67e: shl r15d, cl
14129f681: mov dword ptr [rbp + 0x178], r15d
14129f688: xor ecx, ecx
14129f68a: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
14129f68f: test rax, rax
14129f692: je 0x1412a00fd
14129f698: movups xmm0, xmmword ptr [rax]
14129f69b: inc qword ptr [rax]
14129f69e: movups xmm1, xmmword ptr [rip + 0x1826e2b]
14129f6a5: lea rax, [rbp + 0x98]
14129f6ac: movups xmmword ptr [rax + 0x10], xmm1
14129f6b0: movups xmm1, xmmword ptr [rip + 0x1826e09]
14129f6b7: movups xmmword ptr [rax], xmm1
14129f6ba: movups xmmword ptr [rbp + 0xb8], xmm0
14129f6c1: mov qword ptr [rbp + 0x60], 0
14129f6c9: mov qword ptr [rbp + 0x68], 4
14129f6d1: mov qword ptr [rbp + 0x70], 0
14129f6d9: movabs rax, 0x8000000000000000
14129f6e3: mov qword ptr [rbp + 0x78], rax
14129f6e7: lea rax, [rip + 0x180fbfc]
14129f6ee: mov qword ptr [rbp + 0xc8], rax
14129f6f5: mov qword ptr [rbp + 0xd0], 7
14129f700: mov dword ptr [rsp + 0x20], 1
14129f708: mov rcx, r12
14129f70b: xor edx, edx
14129f70d: mov r8d, 1
14129f713: mov r9d, r15d
14129f716: call 0x140928100 ; country_core::startup::startup_terrain::grid_gen_cw
14129f71b: mov rax, qword ptr [rbp + 0xd0]
14129f722: mov qword ptr [rbp + 0x20], rax
14129f726: movups xmm0, xmmword ptr [rbp + 0xc0]
14129f72d: movaps xmmword ptr [rbp + 0x10], xmm0
14129f731: movups xmm0, xmmword ptr [rbp + 0xb0]
14129f738: movaps xmmword ptr [rbp], xmm0
14129f73c: movups xmm0, xmmword ptr [rbp + 0xa0]
14129f743: movaps xmmword ptr [rbp - 0x10], xmm0
14129f747: movups xmm0, xmmword ptr [rbp + 0x60]
14129f74b: movups xmm1, xmmword ptr [rbp + 0x70]
14129f74f: movups xmm2, xmmword ptr [rbp + 0x80]
14129f756: movups xmm3, xmmword ptr [rbp + 0x90]
14129f75d: movaps xmmword ptr [rbp - 0x20], xmm3
14129f761: movaps xmmword ptr [rbp - 0x30], xmm2
14129f765: movaps xmmword ptr [rbp - 0x40], xmm1
14129f769: movaps xmmword ptr [rbp - 0x50], xmm0
14129f76d: mov eax, dword ptr [rbp + 0x178]
14129f773: inc eax
14129f775: mov qword ptr [rbp + 0x60], 0
14129f77d: mov qword ptr [rbp + 0x90], 0
14129f788: mov r15, rsi
14129f78b: mov qword ptr [rbp + 0xc0], rsi
14129f792: mov dword ptr [rbp + 0xc8], 0
14129f79c: mov dword ptr [rbp + 0xcc], eax
14129f7a2: lea rcx, [rbp + 0x30]
14129f7a6: mov rdx, r12
14129f7a9: lea r8, [rip + 0x198a810]
14129f7b0: call 0x141302d60 ; alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<u32>,array$<array$<f32,3>,2>,country_core::startup::load_mesh_atlases::generate_roof_floor_meshes::closure_env$0> >
14129f7b5: mov r12, rdi
14129f7b8: cmp dword ptr [rbp + 0x17c], 0
14129f7bf: je 0x14129f85d
14129f7c5: mov rax, qword ptr [rbp + 0x40]
14129f7c9: test rax, rax
14129f7cc: je 0x14129f85d
14129f7d2: mov rsi, qword ptr [rbp + 0x38]
14129f7d6: shl rax, 2
14129f7da: lea rdi, [rax + rax*2]
14129f7de: xor ebx, ebx
14129f7e0: movss xmm12, dword ptr [rsi + rbx]
14129f7e6: movss xmm0, dword ptr [rsi + rbx + 8]
14129f7ec: mulss xmm0, xmm6
14129f7f0: call 0x142923fd0
14129f7f5: mulss xmm0, xmm7
14129f7f9: movss dword ptr [rsi + rbx + 8], xmm0
14129f7ff: movaps xmm1, xmm12
14129f803: andps xmm1, xmm8
14129f807: orps xmm1, xmm9
14129f80b: cmpunordss xmm12, xmm12
14129f811: movaps xmm2, xmm12
14129f815: andnps xmm2, xmm1
14129f818: andps xmm12, xmm10
14129f81c: orps xmm12, xmm2
14129f820: addss xmm0, xmm0
14129f824: mulss xmm0, xmm0
14129f828: movaps xmm1, xmm11
14129f82c: subss xmm1, xmm0
14129f830: xorps xmm0, xmm0
14129f833: maxss xmm0, xmm1
14129f837: movaps xmm1, xmm11
14129f83b: minss xmm1, xmm0
14129f83f: xorps xmm0, xmm0
14129f842: sqrtss xmm0, xmm1
14129f846: mulss xmm0, xmm12
14129f84b: mulss xmm0, xmm7
14129f84f: movss dword ptr [rsi + rbx], xmm0
14129f854: add rbx, 0xc
14129f858: cmp rdi, rbx
14129f85b: jne 0x14129f7e0
14129f85d: mov rax, qword ptr [rbp + 0x40]
14129f861: mov qword ptr [rbp + 0x70], rax
14129f865: movups xmm0, xmmword ptr [rbp + 0x30]
14129f869: movaps xmmword ptr [rbp + 0x60], xmm0
14129f86d: mov r8d, 0xf
14129f873: mov rdi, r12
14129f876: mov rcx, r12
14129f879: lea rdx, [rip + 0x198aae1]
14129f880: lea r12, [rbp + 0x60]
14129f884: mov r9, r12
14129f887: call 0x140a34c90 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> >
14129f88c: lea rbx, [rbp + 0x17c]
14129f893: mov qword ptr [rbp + 0x158], rbx
14129f89a: lea rax, [rip - 0x512341]
14129f8a1: mov qword ptr [rbp + 0x160], rax
14129f8a8: lea rax, [rip + 0x198aad1]
14129f8af: mov qword ptr [rbp + 0x60], rax
14129f8b3: mov qword ptr [rbp + 0x68], 1
14129f8bb: lea rax, [rbp + 0x158]
14129f8c2: mov qword ptr [rbp + 0x70], rax
14129f8c6: mov qword ptr [rbp + 0x78], 1
14129f8ce: mov qword ptr [rbp + 0x80], 0
14129f8d9: lea rcx, [rbp + 0x48]
14129f8dd: mov rdx, r12
14129f8e0: call 0x140d79f90 ; alloc::fmt::format::format_inner
14129f8e5: mov rax, qword ptr [rbp + 0x48]
14129f8e9: mov qword ptr [rbp + 0x188], rax
14129f8f0: mov r8, qword ptr [rbp + 0x50]
14129f8f4: mov r9, qword ptr [rbp + 0x58]
14129f8f8: mov qword ptr [rsp + 0x20], rdi
14129f8fd: mov rcx, qword ptr [rbp + 0x168]
14129f904: mov dl, 9
14129f906: mov qword ptr [rbp + 0x180], r8
14129f90d: call 0x140a371d0 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> >
14129f912: mov rsi, r15
14129f915: mov rdx, qword ptr [rbp + 0x188]
14129f91c: test rdx, rdx
14129f91f: je 0x14129f640
14129f925: mov r8d, 1
14129f92b: mov rcx, qword ptr [rbp + 0x180]
14129f932: call 0x140613c20
14129f937: jmp 0x14129f640 ; country_core::startup::load_mesh_atlases::load_mesh_atlases
14129f93c: mov eax, dword ptr [rbp + 0x194]
14129f942: mov rcx, qword ptr [rbp + 0x170]
14129f949: mov dword ptr [rcx], eax
14129f94b: xor ecx, ecx
14129f94d: lea r12, [rbp + 0x60]
14129f951: xor r14d, r14d
14129f954: jmp 0x14129f96b ; country_core::startup::load_mesh_atlases::load_mesh_atlases
14129f956: nop word ptr cs:[rax + rax]
14129f960: mov rcx, rdi
14129f963: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
14129f968: mov ecx, r13d
14129f96b: test r14b, 1
14129f96f: jne 0x14129fb93
14129f975: cmp ecx, 5
14129f978: ja 0x14129fb93
14129f97e: lea r13d, [rcx + 1]
14129f982: cmp ecx, 5
14129f985: mov eax, 5
14129f98a: cmovae r13d, eax
14129f98e: setae r14b
14129f992: mov dword ptr [rbp + 0x17c], ecx
14129f998: mov r15d, 1
14129f99e: shl r15d, cl
14129f9a1: mov dword ptr [rbp + 0x178], r15d
14129f9a8: xor ecx, ecx
14129f9aa: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
14129f9af: test rax, rax
14129f9b2: je 0x1412a00fd
14129f9b8: movups xmm0, xmmword ptr [rax]
14129f9bb: inc qword ptr [rax]
14129f9be: movups xmm1, xmmword ptr [rip + 0x1826b0b]
14129f9c5: lea rax, [rbp + 0x98]
14129f9cc: movups xmmword ptr [rax + 0x10], xmm1
14129f9d0: movups xmm1, xmmword ptr [rip + 0x1826ae9]
14129f9d7: movups xmmword ptr [rax], xmm1
14129f9da: movups xmmword ptr [rbp + 0xb8], xmm0
14129f9e1: mov qword ptr [rbp + 0x60], 0
14129f9e9: mov qword ptr [rbp + 0x68], 4
14129f9f1: mov qword ptr [rbp + 0x70], 0
14129f9f9: movabs rax, 0x8000000000000000
14129fa03: mov qword ptr [rbp + 0x78], rax
14129fa07: lea rax, [rip + 0x180f992]
14129fa0e: mov qword ptr [rbp + 0xc8], rax
14129fa15: mov qword ptr [rbp + 0xd0], 8
14129fa20: mov dword ptr [rsp + 0x20], 6
14129fa28: mov rcx, r12
14129fa2b: xor edx, edx
14129fa2d: mov r8d, 6
14129fa33: mov r9d, r15d
14129fa36: call 0x140928350 ; country_core::startup::startup_terrain::grid_gen_ccw
14129fa3b: mov rax, qword ptr [rbp + 0xd0]
14129fa42: mov qword ptr [rbp + 0x20], rax
14129fa46: movups xmm0, xmmword ptr [rbp + 0xc0]
14129fa4d: movaps xmmword ptr [rbp + 0x10], xmm0
14129fa51: movups xmm0, xmmword ptr [rbp + 0xb0]
14129fa58: movaps xmmword ptr [rbp], xmm0
14129fa5c: movups xmm0, xmmword ptr [rbp + 0xa0]
14129fa63: movaps xmmword ptr [rbp - 0x10], xmm0
14129fa67: movdqu xmm0, xmmword ptr [rbp + 0x60]
14129fa6c: movups xmm1, xmmword ptr [rbp + 0x70]
14129fa70: movups xmm2, xmmword ptr [rbp + 0x80]
14129fa77: movups xmm3, xmmword ptr [rbp + 0x90]
14129fa7e: movaps xmmword ptr [rbp - 0x20], xmm3
14129fa82: movaps xmmword ptr [rbp - 0x30], xmm2
14129fa86: movaps xmmword ptr [rbp - 0x40], xmm1
14129fa8a: movdqa xmmword ptr [rbp - 0x50], xmm0
14129fa8f: mov eax, dword ptr [rbp + 0x178]
14129fa95: inc eax
14129fa97: mov qword ptr [rbp + 0x60], 0
14129fa9f: mov qword ptr [rbp + 0xd0], 0
14129faaa: mov qword ptr [rbp + 0x140], rsi
14129fab1: mov dword ptr [rbp + 0x148], 0
14129fabb: mov dword ptr [rbp + 0x14c], eax
14129fac1: lea rcx, [rbp + 0x30]
14129fac5: mov rdx, r12
14129fac8: lea r8, [rip + 0x198a4f1]
14129facf: call 0x1413018b0 ; alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::flatten::FlatMap<core::ops::range::Range<u32>,array$<array$<f32,3>,7>,country_core::startup::load_mesh_atlases::generate_curved_halftimber_meshes::closure_env$0> >
14129fad4: mov r8d, 0xf
14129fada: mov rcx, rdi
14129fadd: lea rdx, [rip + 0x198a87d]
14129fae4: lea r9, [rbp + 0x30]
14129fae8: call 0x140a34c90 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> >
14129faed: mov qword ptr [rbp + 0x158], rbx
14129faf4: lea rax, [rip - 0x51259b]
14129fafb: mov qword ptr [rbp + 0x160], rax
14129fb02: lea rax, [rip + 0x198a88f]
14129fb09: mov qword ptr [rbp + 0x60], rax
14129fb0d: mov qword ptr [rbp + 0x68], 1
14129fb15: lea rax, [rbp + 0x158]
14129fb1c: mov qword ptr [rbp + 0x70], rax
14129fb20: mov qword ptr [rbp + 0x78], 1
14129fb28: mov qword ptr [rbp + 0x80], 0
14129fb33: lea rcx, [rbp + 0x48]
14129fb37: mov rdx, r12
14129fb3a: call 0x140d79f90 ; alloc::fmt::format::format_inner
14129fb3f: mov rax, qword ptr [rbp + 0x48]
14129fb43: mov qword ptr [rbp + 0x188], rax
14129fb4a: mov r8, qword ptr [rbp + 0x50]
14129fb4e: mov r9, qword ptr [rbp + 0x58]
14129fb52: mov qword ptr [rsp + 0x20], rdi
14129fb57: mov rcx, qword ptr [rbp + 0x168]
14129fb5e: mov dl, 0x18
14129fb60: mov qword ptr [rbp + 0x180], r8
14129fb67: call 0x140a371d0 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> >
14129fb6c: mov rdx, qword ptr [rbp + 0x188]
14129fb73: test rdx, rdx
14129fb76: je 0x14129f960
14129fb7c: mov r8d, 1
14129fb82: mov rcx, qword ptr [rbp + 0x180]
14129fb89: call 0x140613c20
14129fb8e: jmp 0x14129f960 ; country_core::startup::load_mesh_atlases::load_mesh_atlases
14129fb93: mov eax, dword ptr [rbp + 0x194]
14129fb99: mov rcx, qword ptr [rbp + 0x170]
14129fba0: mov dword ptr [rcx], eax
14129fba2: xor r13d, r13d
14129fba5: xor ecx, ecx
14129fba7: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
14129fbac: test rax, rax
14129fbaf: je 0x1412a00fd
14129fbb5: movdqu xmm0, xmmword ptr [rax]
14129fbb9: inc qword ptr [rax]
14129fbbc: movups xmm6, xmmword ptr [rip + 0x182690d]
14129fbc3: movups xmmword ptr [rbp + 0xa8], xmm6
14129fbca: movups xmm7, xmmword ptr [rip + 0x18268ef]
14129fbd1: movups xmmword ptr [rbp + 0x98], xmm7
14129fbd8: movdqu xmmword ptr [rbp + 0xb8], xmm0
14129fbe0: mov qword ptr [rbp + 0x60], 0
14129fbe8: mov qword ptr [rbp + 0x68], 4
14129fbf0: mov qword ptr [rbp + 0x70], 0
14129fbf8: movabs rax, 0x8000000000000000
14129fc02: mov qword ptr [rbp + 0x78], rax
14129fc06: lea rax, [rip + 0x198a79b]
14129fc0d: mov qword ptr [rbp + 0xc8], rax
14129fc14: mov qword ptr [rbp + 0xd0], 7
14129fc1f: lea rdi, [rip + 0x198a79a]
14129fc26: lea r14, [rbp + 0x60]
14129fc2a: lea rbx, [rip + 0x198a7a7]
14129fc31: xor r12d, r12d
14129fc34: jmp 0x14129fc54 ; country_core::startup::load_mesh_atlases::load_mesh_atlases
14129fc36: nop word ptr cs:[rax + rax]
14129fc40: mov rax, qword ptr [rbp + 0x68]
14129fc44: mov dword ptr [rax + r12*4 + 0x14], r15d
14129fc49: add r12, 6
14129fc4d: mov qword ptr [rbp + 0x70], r12
14129fc51: mov r13d, esi
14129fc54: cmp r13d, 0x20
14129fc58: jae 0x14129fd4c
14129fc5e: cmp r12, qword ptr [rbp + 0x60]
14129fc62: jne 0x14129fc6f
14129fc64: mov rcx, r14
14129fc67: mov rdx, rdi
14129fc6a: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
14129fc6f: mov r15d, r13d
14129fc72: or r15d, 0x10000
14129fc79: mov rax, qword ptr [rbp + 0x68]
14129fc7d: mov dword ptr [rax + r12*4], r15d
14129fc81: mov rsi, r12
14129fc84: or rsi, 1
14129fc88: mov qword ptr [rbp + 0x70], rsi
14129fc8c: cmp rsi, qword ptr [rbp + 0x60]
14129fc90: jne 0x14129fc9d
14129fc92: mov rcx, r14
14129fc95: mov rdx, rbx
14129fc98: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
14129fc9d: mov rax, qword ptr [rbp + 0x68]
14129fca1: mov dword ptr [rax + rsi*4], r13d
14129fca5: lea rax, [r12 + 2]
14129fcaa: mov qword ptr [rbp + 0x70], rax
14129fcae: cmp rax, qword ptr [rbp + 0x60]
14129fcb2: jne 0x14129fcc3
14129fcb4: mov rcx, r14
14129fcb7: lea rdx, [rip + 0x198a732]
14129fcbe: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
14129fcc3: lea esi, [r13 + 1]
14129fcc7: mov rax, qword ptr [rbp + 0x68]
14129fccb: mov dword ptr [rax + r12*4 + 8], esi
14129fcd0: lea rax, [r12 + 3]
14129fcd5: mov qword ptr [rbp + 0x70], rax
14129fcd9: cmp rax, qword ptr [rbp + 0x60]
14129fcdd: jne 0x14129fcee
14129fcdf: mov rcx, r14
14129fce2: lea rdx, [rip + 0x198a71f]
14129fce9: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
14129fcee: mov rax, qword ptr [rbp + 0x68]
14129fcf2: mov dword ptr [rax + r12*4 + 0xc], esi
14129fcf7: lea rax, [r12 + 4]
14129fcfc: mov qword ptr [rbp + 0x70], rax
14129fd00: cmp rax, qword ptr [rbp + 0x60]
14129fd04: jne 0x14129fd15
14129fd06: mov rcx, r14
14129fd09: lea rdx, [rip + 0x198a710]
14129fd10: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
14129fd15: add r13d, 0x10001
14129fd1c: mov rax, qword ptr [rbp + 0x68]
14129fd20: mov dword ptr [rax + r12*4 + 0x10], r13d
14129fd25: lea rax, [r12 + 5]
14129fd2a: mov qword ptr [rbp + 0x70], rax
14129fd2e: cmp rax, qword ptr [rbp + 0x60]
14129fd32: jne 0x14129fc40
14129fd38: mov rcx, r14
14129fd3b: lea rdx, [rip + 0x198a6f6]
14129fd42: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
14129fd47: jmp 0x14129fc40 ; country_core::startup::load_mesh_atlases::load_mesh_atlases
14129fd4c: mov qword ptr [rsp + 0x20], r14
14129fd51: lea r8, [rip + 0x198a657]
14129fd58: mov r9d, 0xd
14129fd5e: mov rcx, qword ptr [rbp + 0x168]
14129fd65: mov dl, 0x13
14129fd67: call 0x140a371d0 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> >
14129fd6c: mov rcx, r14
14129fd6f: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
14129fd74: mov eax, dword ptr [rbp + 0x194]
14129fd7a: mov rcx, qword ptr [rbp + 0x170]
14129fd81: mov dword ptr [rcx], eax
14129fd83: mov dword ptr [rbp + 0x158], 4
14129fd8d: mov eax, 4
14129fd92: lea r12, [rip + 0x198a6ff]
14129fd99: lea r13, [rip + 0x198a710]
14129fda0: jmp 0x14129fdc4 ; country_core::startup::load_mesh_atlases::load_mesh_atlases
14129fda2: nop word ptr cs:[rax + rax]
14129fdb0: shl dword ptr [rbp + 0x158], 1
14129fdb6: mov rcx, r14
14129fdb9: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
14129fdbe: mov eax, dword ptr [rbp + 0x158]
14129fdc4: cmp eax, 0x41
14129fdc7: jae 0x14129ff72
14129fdcd: xor ecx, ecx
14129fdcf: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
14129fdd4: test rax, rax
14129fdd7: je 0x1412a00fd
14129fddd: movdqu xmm0, xmmword ptr [rax]
14129fde1: inc qword ptr [rax]
14129fde4: lea rax, [rbp + 0x98]
14129fdeb: movups xmmword ptr [rax + 0x10], xmm6
14129fdef: movups xmmword ptr [rax], xmm7
14129fdf2: movdqu xmmword ptr [rbp + 0xb8], xmm0
14129fdfa: mov qword ptr [rbp + 0x60], 0
14129fe02: mov qword ptr [rbp + 0x68], 4
14129fe0a: mov qword ptr [rbp + 0x70], 0
14129fe12: movabs rax, 0x8000000000000000
14129fe1c: mov qword ptr [rbp + 0x78], rax
14129fe20: lea rax, [rip + 0x198a629]
14129fe27: mov qword ptr [rbp + 0xc8], rax
14129fe2e: mov qword ptr [rbp + 0xd0], 0xc
14129fe39: xor ebx, ebx
14129fe3b: mov esi, dword ptr [rbp + 0x158]
14129fe41: xor edi, edi
14129fe43: jmp 0x14129fe67 ; country_core::startup::load_mesh_atlases::load_mesh_atlases
14129fe45: nop word ptr cs:[rax + rax]
14129fe50: inc edi
14129fe52: add r15d, 2
14129fe56: mov rax, qword ptr [rbp + 0x68]
14129fe5a: mov dword ptr [rax + rbx*4 + 8], r15d
14129fe5f: add rbx, 3
14129fe63: mov qword ptr [rbp + 0x70], rbx
14129fe67: cmp edi, esi
14129fe69: jae 0x14129fed0
14129fe6b: cmp rbx, qword ptr [rbp + 0x60]
14129fe6f: jne 0x14129fe80
14129fe71: mov rcx, r14
14129fe74: lea rdx, [rip + 0x198a605]
14129fe7b: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
14129fe80: lea r15d, [rdi + rdi*2]
14129fe84: mov rax, qword ptr [rbp + 0x68]
14129fe88: mov dword ptr [rax + rbx*4], r15d
14129fe8c: lea rax, [rbx + 1]
14129fe90: mov qword ptr [rbp + 0x70], rax
14129fe94: cmp rax, qword ptr [rbp + 0x60]
14129fe98: jne 0x14129fea5
14129fe9a: mov rcx, r14
14129fe9d: mov rdx, r12
14129fea0: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
14129fea5: lea eax, [r15 + 1]
14129fea9: mov rcx, qword ptr [rbp + 0x68]
14129fead: mov dword ptr [rcx + rbx*4 + 4], eax
14129feb1: lea rax, [rbx + 2]
14129feb5: mov qword ptr [rbp + 0x70], rax
14129feb9: cmp rax, qword ptr [rbp + 0x60]
14129febd: jne 0x14129fe50
14129febf: mov rcx, r14
14129fec2: mov rdx, r13
14129fec5: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
14129feca: jmp 0x14129fe50 ; country_core::startup::load_mesh_atlases::load_mesh_atlases
14129fecc: nop dword ptr [rax]
14129fed0: lea rax, [rbp + 0x158]
14129fed7: mov qword ptr [rbp + 0x30], rax
14129fedb: lea rax, [rip - 0x512982]
14129fee2: mov qword ptr [rbp + 0x38], rax
14129fee6: lea rax, [rip + 0x198a583]
14129feed: mov qword ptr [rbp - 0x50], rax
14129fef1: mov qword ptr [rbp - 0x48], 1
14129fef9: lea rax, [rbp + 0x30]
14129fefd: mov qword ptr [rbp - 0x40], rax
14129ff01: mov qword ptr [rbp - 0x38], 1
14129ff09: mov qword ptr [rbp - 0x30], 0
14129ff11: lea rcx, [rbp + 0x48]
14129ff15: lea rdx, [rbp - 0x50]
14129ff19: call 0x140d79f90 ; alloc::fmt::format::format_inner
14129ff1e: mov rax, qword ptr [rbp + 0x48]
14129ff22: mov qword ptr [rbp + 0x188], rax
14129ff29: mov r8, qword ptr [rbp + 0x50]
14129ff2d: mov r9, qword ptr [rbp + 0x58]
14129ff31: mov qword ptr [rsp + 0x20], r14
14129ff36: mov rcx, qword ptr [rbp + 0x168]
14129ff3d: mov dl, 0x1a
14129ff3f: mov qword ptr [rbp + 0x180], r8
14129ff46: call 0x140a371d0 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> >
14129ff4b: mov rdx, qword ptr [rbp + 0x188]
14129ff52: test rdx, rdx
14129ff55: je 0x14129fdb0
14129ff5b: mov r8d, 1
14129ff61: mov rcx, qword ptr [rbp + 0x180]
14129ff68: call 0x140613c20
14129ff6d: jmp 0x14129fdb0 ; country_core::startup::load_mesh_atlases::load_mesh_atlases
14129ff72: mov eax, dword ptr [rbp + 0x194]
14129ff78: mov rcx, qword ptr [rbp + 0x170]
14129ff7f: mov dword ptr [rcx], eax
14129ff81: xor ecx, ecx
14129ff83: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
14129ff88: test rax, rax
14129ff8b: je 0x1412a00fd
14129ff91: movups xmm0, xmmword ptr [rax]
14129ff94: inc qword ptr [rax]
14129ff97: movups xmmword ptr [rbp + 0xa8], xmm6
14129ff9e: movups xmmword ptr [rbp + 0x98], xmm7
14129ffa5: movups xmmword ptr [rbp + 0xb8], xmm0
14129ffac: mov qword ptr [rbp + 0x60], 0
14129ffb4: mov qword ptr [rbp + 0x68], 4
14129ffbc: mov qword ptr [rbp + 0x70], 0
14129ffc4: movabs rax, 0x8000000000000000
14129ffce: mov qword ptr [rbp + 0x78], rax
14129ffd2: lea rax, [rip + 0x180f311]
14129ffd9: mov qword ptr [rbp + 0xc8], rax
14129ffe0: mov qword ptr [rbp + 0xd0], 7
14129ffeb: mov dword ptr [rsp + 0x20], 0xc
14129fff3: lea rcx, [rbp + 0x60]
14129fff7: xor edx, edx
14129fff9: mov r8d, 0xc
14129ffff: mov r9d, 0x32
1412a0005: call 0x140928100 ; country_core::startup::startup_terrain::grid_gen_cw
1412a000a: mov rsi, qword ptr [rbp + 0x168]
1412a0011: mov rax, qword ptr [rbp + 0xd0]
1412a0018: mov qword ptr [rbp + 0x20], rax
1412a001c: movups xmm0, xmmword ptr [rbp + 0xc0]
1412a0023: movaps xmmword ptr [rbp + 0x10], xmm0
1412a0027: movups xmm0, xmmword ptr [rbp + 0xb0]
1412a002e: movaps xmmword ptr [rbp], xmm0
1412a0032: movups xmm0, xmmword ptr [rbp + 0xa0]
1412a0039: movaps xmmword ptr [rbp - 0x10], xmm0
1412a003d: movups xmm0, xmmword ptr [rbp + 0x60]
1412a0041: movups xmm1, xmmword ptr [rbp + 0x70]
1412a0045: movups xmm2, xmmword ptr [rbp + 0x80]
1412a004c: movups xmm3, xmmword ptr [rbp + 0x90]
1412a0053: movaps xmmword ptr [rbp - 0x20], xmm3
1412a0057: movaps xmmword ptr [rbp - 0x30], xmm2
1412a005b: movaps xmmword ptr [rbp - 0x40], xmm1
1412a005f: movaps xmmword ptr [rbp - 0x50], xmm0
1412a0063: lea rax, [rbp - 0x50]
1412a0067: mov qword ptr [rsp + 0x20], rax
1412a006c: lea r8, [rip + 0x198a2e9]
1412a0073: mov r9d, 5
1412a0079: mov rcx, rsi
1412a007c: mov dl, 0x15
1412a007e: call 0x140a371d0 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> >
1412a0083: lea rcx, [rbp - 0x50]
1412a0087: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
1412a008c: mov eax, dword ptr [rbp + 0x194]
1412a0092: mov rcx, qword ptr [rbp + 0x170]
1412a0099: mov dword ptr [rcx], eax
1412a009b: mov rdx, qword ptr [rbp - 0x58]
1412a009f: mov eax, dword ptr [rdx + 0x1c]
1412a00a2: mov rcx, qword ptr [rdx + 0x10]
1412a00a6: mov dword ptr [rcx], eax
1412a00a8: mov rdx, qword ptr [rdx]
1412a00ab: mov rcx, rsi
1412a00ae: call 0x140ae6b90 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::register_meshes
1412a00b3: movaps xmm6, xmmword ptr [rbp + 0x1a0]
1412a00ba: movaps xmm7, xmmword ptr [rbp + 0x1b0]
1412a00c1: movaps xmm8, xmmword ptr [rbp + 0x1c0]
1412a00c9: movaps xmm9, xmmword ptr [rbp + 0x1d0]
1412a00d1: movaps xmm10, xmmword ptr [rbp + 0x1e0]
1412a00d9: movaps xmm11, xmmword ptr [rbp + 0x1f0]
1412a00e1: movaps xmm12, xmmword ptr [rbp + 0x200]
1412a00e9: add rsp, 0x298
1412a00f0: pop rbx
1412a00f1: pop rdi
1412a00f2: pop rsi
1412a00f3: pop r12
1412a00f5: pop r13
1412a00f7: pop r14
1412a00f9: pop r15
1412a00fb: pop rbp
1412a00fc: ret 
1412a00fd: lea rcx, [rip + 0x199a99c]
1412a0104: call 0x1428d80d0 ; std::thread::local::panic_access_error
1412a0109: int3 
1412a010a: nop word ptr [rax + rax]
1412a0110: mov qword ptr [rsp + 0x10], rdx
1412a0115: push rbp
1412a0116: push r15
1412a0118: push r14
1412a011a: push r13
1412a011c: push r12
1412a011e: push rsi
1412a011f: push rdi
1412a0120: push rbx
1412a0121: sub rsp, 0x98
1412a0128: lea rbp, [rdx + 0x80]
1412a012f: movaps xmmword ptr [rsp + 0x20], xmm12
1412a0135: movaps xmmword ptr [rsp + 0x30], xmm11
1412a013b: movaps xmmword ptr [rsp + 0x40], xmm10
1412a0141: movaps xmmword ptr [rsp + 0x50], xmm9
1412a0147: movaps xmmword ptr [rsp + 0x60], xmm8
1412a014d: movaps xmmword ptr [rsp + 0x70], xmm7
1412a0152: movaps xmmword ptr [rsp + 0x80], xmm6
1412a015a: cmp qword ptr [rbp + 0x188], 0
1412a0162: je 0x1412a017d
1412a0164: mov r8d, 1
1412a016a: mov rcx, qword ptr [rbp + 0x180]
1412a0171: mov rdx, qword ptr [rbp + 0x188]
1412a0178: call 0x140613c20
1412a017d: movaps xmm6, xmmword ptr [rsp + 0x80]
1412a0185: movaps xmm7, xmmword ptr [rsp + 0x70]
1412a018a: movaps xmm8, xmmword ptr [rsp + 0x60]
1412a0190: movaps xmm9, xmmword ptr [rsp + 0x50]
1412a0196: movaps xmm10, xmmword ptr [rsp + 0x40]
1412a019c: movaps xmm11, xmmword ptr [rsp + 0x30]
1412a01a2: movaps xmm12, xmmword ptr [rsp + 0x20]
1412a01a8: add rsp, 0x98
1412a01af: pop rbx
1412a01b0: pop rdi
1412a01b1: pop rsi
1412a01b2: pop r12
1412a01b4: pop r13
1412a01b6: pop r14
1412a01b8: pop r15
1412a01ba: pop rbp
1412a01bb: ret 
1412a01bc: nop dword ptr [rax]
1412a01c0: mov qword ptr [rsp + 0x10], rdx
1412a01c5: push rbp
1412a01c6: push r15
1412a01c8: push r14
1412a01ca: push r13
1412a01cc: push r12
1412a01ce: push rsi
1412a01cf: push rdi
1412a01d0: push rbx
1412a01d1: sub rsp, 0x98
1412a01d8: lea rbp, [rdx + 0x80]
1412a01df: movaps xmmword ptr [rsp + 0x20], xmm12
1412a01e5: movaps xmmword ptr [rsp + 0x30], xmm11
1412a01eb: movaps xmmword ptr [rsp + 0x40], xmm10
1412a01f1: movaps xmmword ptr [rsp + 0x50], xmm9
1412a01f7: movaps xmmword ptr [rsp + 0x60], xmm8
1412a01fd: movaps xmmword ptr [rsp + 0x70], xmm7
1412a0202: movaps xmmword ptr [rsp + 0x80], xmm6
1412a020a: cmp qword ptr [rbp + 0x188], 0
1412a0212: je 0x1412a022d
1412a0214: mov r8d, 1
1412a021a: mov rcx, qword ptr [rbp + 0x180]
1412a0221: mov rdx, qword ptr [rbp + 0x188]
1412a0228: call 0x140613c20
1412a022d: movaps xmm6, xmmword ptr [rsp + 0x80]
1412a0235: movaps xmm7, xmmword ptr [rsp + 0x70]
1412a023a: movaps xmm8, xmmword ptr [rsp + 0x60]
1412a0240: movaps xmm9, xmmword ptr [rsp + 0x50]
1412a0246: movaps xmm10, xmmword ptr [rsp + 0x40]
1412a024c: movaps xmm11, xmmword ptr [rsp + 0x30]
1412a0252: movaps xmm12, xmmword ptr [rsp + 0x20]
1412a0258: add rsp, 0x98
1412a025f: pop rbx
1412a0260: pop rdi
1412a0261: pop rsi
1412a0262: pop r12
1412a0264: pop r13
1412a0266: pop r14
1412a0268: pop r15
1412a026a: pop rbp
1412a026b: ret 
1412a026c: nop dword ptr [rax]
1412a0270: mov qword ptr [rsp + 0x10], rdx
1412a0275: push rbp
1412a0276: push r15
1412a0278: push r14
1412a027a: push r13
1412a027c: push r12
1412a027e: push rsi
1412a027f: push rdi
1412a0280: push rbx
1412a0281: sub rsp, 0x98
1412a0288: lea rbp, [rdx + 0x80]
1412a028f: movaps xmmword ptr [rsp + 0x20], xmm12
1412a0295: movaps xmmword ptr [rsp + 0x30], xmm11
1412a029b: movaps xmmword ptr [rsp + 0x40], xmm10
1412a02a1: movaps xmmword ptr [rsp + 0x50], xmm9
1412a02a7: movaps xmmword ptr [rsp + 0x60], xmm8
1412a02ad: movaps xmmword ptr [rsp + 0x70], xmm7
1412a02b2: movaps xmmword ptr [rsp + 0x80], xmm6
1412a02ba: lea rcx, [rbp + 0x60]
1412a02be: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
1412a02c3: movaps xmm6, xmmword ptr [rsp + 0x80]
1412a02cb: movaps xmm7, xmmword ptr [rsp + 0x70]
1412a02d0: movaps xmm8, xmmword ptr [rsp + 0x60]
1412a02d6: movaps xmm9, xmmword ptr [rsp + 0x50]
1412a02dc: movaps xmm10, xmmword ptr [rsp + 0x40]
1412a02e2: movaps xmm11, xmmword ptr [rsp + 0x30]
1412a02e8: movaps xmm12, xmmword ptr [rsp + 0x20]
1412a02ee: add rsp, 0x98
1412a02f5: pop rbx
1412a02f6: pop rdi
1412a02f7: pop rsi
1412a02f8: pop r12
1412a02fa: pop r13
1412a02fc: pop r14
1412a02fe: pop r15
1412a0300: pop rbp
1412a0301: ret 
1412a0302: nop word ptr cs:[rax + rax]
1412a0310: mov qword ptr [rsp + 0x10], rdx
1412a0315: push rbp
1412a0316: push r15
1412a0318: push r14
1412a031a: push r13
1412a031c: push r12
1412a031e: push rsi
1412a031f: push rdi
1412a0320: push rbx
1412a0321: sub rsp, 0x98
1412a0328: lea rbp, [rdx + 0x80]
1412a032f: movaps xmmword ptr [rsp + 0x20], xmm12
1412a0335: movaps xmmword ptr [rsp + 0x30], xmm11
1412a033b: movaps xmmword ptr [rsp + 0x40], xmm10
1412a0341: movaps xmmword ptr [rsp + 0x50], xmm9
1412a0347: movaps xmmword ptr [rsp + 0x60], xmm8
1412a034d: movaps xmmword ptr [rsp + 0x70], xmm7
1412a0352: movaps xmmword ptr [rsp + 0x80], xmm6
1412a035a: cmp qword ptr [rbp + 0x188], 0
1412a0362: je 0x1412a037d
1412a0364: mov r8d, 1
1412a036a: mov rcx, qword ptr [rbp + 0x180]
1412a0371: mov rdx, qword ptr [rbp + 0x188]
1412a0378: call 0x140613c20
1412a037d: movaps xmm6, xmmword ptr [rsp + 0x80]
1412a0385: movaps xmm7, xmmword ptr [rsp + 0x70]
1412a038a: movaps xmm8, xmmword ptr [rsp + 0x60]
1412a0390: movaps xmm9, xmmword ptr [rsp + 0x50]
1412a0396: movaps xmm10, xmmword ptr [rsp + 0x40]
1412a039c: movaps xmm11, xmmword ptr [rsp + 0x30]
1412a03a2: movaps xmm12, xmmword ptr [rsp + 0x20]
1412a03a8: add rsp, 0x98
1412a03af: pop rbx
1412a03b0: pop rdi
1412a03b1: pop rsi
1412a03b2: pop r12
1412a03b4: pop r13
1412a03b6: pop r14
1412a03b8: pop r15
1412a03ba: pop rbp
1412a03bb: ret 
1412a03bc: nop dword ptr [rax]
1412a03c0: mov qword ptr [rsp + 0x10], rdx
1412a03c5: push rbp
1412a03c6: push r15
1412a03c8: push r14
1412a03ca: push r13
1412a03cc: push r12
1412a03ce: push rsi
1412a03cf: push rdi
1412a03d0: push rbx
1412a03d1: sub rsp, 0x98
1412a03d8: lea rbp, [rdx + 0x80]
1412a03df: movaps xmmword ptr [rsp + 0x20], xmm12
1412a03e5: movaps xmmword ptr [rsp + 0x30], xmm11
1412a03eb: movaps xmmword ptr [rsp + 0x40], xmm10
1412a03f1: movaps xmmword ptr [rsp + 0x50], xmm9
1412a03f7: movaps xmmword ptr [rsp + 0x60], xmm8
1412a03fd: movaps xmmword ptr [rsp + 0x70], xmm7
1412a0402: movaps xmmword ptr [rsp + 0x80], xmm6
1412a040a: lea rcx, [rbp - 0x50]
1412a040e: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
1412a0413: movaps xmm6, xmmword ptr [rsp + 0x80]
1412a041b: movaps xmm7, xmmword ptr [rsp + 0x70]
1412a0420: movaps xmm8, xmmword ptr [rsp + 0x60]
1412a0426: movaps xmm9, xmmword ptr [rsp + 0x50]
1412a042c: movaps xmm10, xmmword ptr [rsp + 0x40]
1412a0432: movaps xmm11, xmmword ptr [rsp + 0x30]
1412a0438: movaps xmm12, xmmword ptr [rsp + 0x20]
1412a043e: add rsp, 0x98
1412a0445: pop rbx
1412a0446: pop rdi
1412a0447: pop rsi
1412a0448: pop r12
1412a044a: pop r13
1412a044c: pop r14
1412a044e: pop r15
1412a0450: pop rbp
1412a0451: ret 
1412a0452: nop word ptr cs:[rax + rax]
1412a0460: mov qword ptr [rsp + 0x10], rdx
1412a0465: push rbp
1412a0466: push r15
1412a0468: push r14
1412a046a: push r13
1412a046c: push r12
1412a046e: push rsi
1412a046f: push rdi
1412a0470: push rbx
1412a0471: sub rsp, 0x98
1412a0478: lea rbp, [rdx + 0x80]
1412a047f: movaps xmmword ptr [rsp + 0x20], xmm12
1412a0485: movaps xmmword ptr [rsp + 0x30], xmm11
1412a048b: movaps xmmword ptr [rsp + 0x40], xmm10
1412a0491: movaps xmmword ptr [rsp + 0x50], xmm9
1412a0497: movaps xmmword ptr [rsp + 0x60], xmm8
1412a049d: movaps xmmword ptr [rsp + 0x70], xmm7
1412a04a2: movaps xmmword ptr [rsp + 0x80], xmm6
1412a04aa: lea rcx, [rbp + 0x60]
1412a04ae: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
1412a04b3: movaps xmm6, xmmword ptr [rsp + 0x80]
1412a04bb: movaps xmm7, xmmword ptr [rsp + 0x70]
1412a04c0: movaps xmm8, xmmword ptr [rsp + 0x60]
1412a04c6: movaps xmm9, xmmword ptr [rsp + 0x50]
1412a04cc: movaps xmm10, xmmword ptr [rsp + 0x40]
1412a04d2: movaps xmm11, xmmword ptr [rsp + 0x30]
1412a04d8: movaps xmm12, xmmword ptr [rsp + 0x20]
1412a04de: add rsp, 0x98
1412a04e5: pop rbx
1412a04e6: pop rdi
1412a04e7: pop rsi
1412a04e8: pop r12
1412a04ea: pop r13
1412a04ec: pop r14
1412a04ee: pop r15
1412a04f0: pop rbp
1412a04f1: ret 
1412a04f2: nop word ptr cs:[rax + rax]
1412a0500: mov qword ptr [rsp + 0x10], rdx
1412a0505: push rbp
1412a0506: push r15
1412a0508: push r14
1412a050a: push r13
1412a050c: push r12
1412a050e: push rsi
1412a050f: push rdi
1412a0510: push rbx
1412a0511: sub rsp, 0x98
1412a0518: lea rbp, [rdx + 0x80]
1412a051f: movaps xmmword ptr [rsp + 0x20], xmm12
1412a0525: movaps xmmword ptr [rsp + 0x30], xmm11
1412a052b: movaps xmmword ptr [rsp + 0x40], xmm10
1412a0531: movaps xmmword ptr [rsp + 0x50], xmm9
1412a0537: movaps xmmword ptr [rsp + 0x60], xmm8
1412a053d: movaps xmmword ptr [rsp + 0x70], xmm7
1412a0542: movaps xmmword ptr [rsp + 0x80], xmm6
1412a054a: lea rcx, [rbp - 0x50]
1412a054e: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
1412a0553: movaps xmm6, xmmword ptr [rsp + 0x80]
1412a055b: movaps xmm7, xmmword ptr [rsp + 0x70]
1412a0560: movaps xmm8, xmmword ptr [rsp + 0x60]
1412a0566: movaps xmm9, xmmword ptr [rsp + 0x50]
1412a056c: movaps xmm10, xmmword ptr [rsp + 0x40]
1412a0572: movaps xmm11, xmmword ptr [rsp + 0x30]
1412a0578: movaps xmm12, xmmword ptr [rsp + 0x20]
1412a057e: add rsp, 0x98
1412a0585: pop rbx
1412a0586: pop rdi
1412a0587: pop rsi
1412a0588: pop r12
1412a058a: pop r13
1412a058c: pop r14
1412a058e: pop r15
1412a0590: pop rbp
1412a0591: ret 
1412a0592: nop word ptr cs:[rax + rax]
1412a05a0: mov qword ptr [rsp + 0x10], rdx
1412a05a5: push rbp
1412a05a6: push r15
1412a05a8: push r14
1412a05aa: push r13
1412a05ac: push r12
1412a05ae: push rsi
1412a05af: push rdi
1412a05b0: push rbx
1412a05b1: sub rsp, 0x98
1412a05b8: lea rbp, [rdx + 0x80]
1412a05bf: movaps xmmword ptr [rsp + 0x20], xmm12
1412a05c5: movaps xmmword ptr [rsp + 0x30], xmm11
1412a05cb: movaps xmmword ptr [rsp + 0x40], xmm10
1412a05d1: movaps xmmword ptr [rsp + 0x50], xmm9
1412a05d7: movaps xmmword ptr [rsp + 0x60], xmm8
1412a05dd: movaps xmmword ptr [rsp + 0x70], xmm7
1412a05e2: movaps xmmword ptr [rsp + 0x80], xmm6
1412a05ea: lea rcx, [rbp + 0x60]
1412a05ee: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
1412a05f3: movaps xmm6, xmmword ptr [rsp + 0x80]
1412a05fb: movaps xmm7, xmmword ptr [rsp + 0x70]
1412a0600: movaps xmm8, xmmword ptr [rsp + 0x60]
1412a0606: movaps xmm9, xmmword ptr [rsp + 0x50]
1412a060c: movaps xmm10, xmmword ptr [rsp + 0x40]
1412a0612: movaps xmm11, xmmword ptr [rsp + 0x30]
1412a0618: movaps xmm12, xmmword ptr [rsp + 0x20]
1412a061e: add rsp, 0x98
1412a0625: pop rbx
1412a0626: pop rdi
1412a0627: pop rsi
1412a0628: pop r12
1412a062a: pop r13
1412a062c: pop r14
1412a062e: pop r15
1412a0630: pop rbp
1412a0631: ret 
1412a0632: nop word ptr cs:[rax + rax]
1412a0640: mov qword ptr [rsp + 0x10], rdx
1412a0645: push rbp
1412a0646: push r15
1412a0648: push r14
1412a064a: push r13
1412a064c: push r12
1412a064e: push rsi
1412a064f: push rdi
1412a0650: push rbx
1412a0651: sub rsp, 0x98
1412a0658: lea rbp, [rdx + 0x80]
1412a065f: movaps xmmword ptr [rsp + 0x20], xmm12
1412a0665: movaps xmmword ptr [rsp + 0x30], xmm11
1412a066b: movaps xmmword ptr [rsp + 0x40], xmm10
1412a0671: movaps xmmword ptr [rsp + 0x50], xmm9
1412a0677: movaps xmmword ptr [rsp + 0x60], xmm8
1412a067d: movaps xmmword ptr [rsp + 0x70], xmm7
1412a0682: movaps xmmword ptr [rsp + 0x80], xmm6
1412a068a: lea rcx, [rbp - 0x50]
1412a068e: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
1412a0693: movaps xmm6, xmmword ptr [rsp + 0x80]
1412a069b: movaps xmm7, xmmword ptr [rsp + 0x70]
1412a06a0: movaps xmm8, xmmword ptr [rsp + 0x60]
1412a06a6: movaps xmm9, xmmword ptr [rsp + 0x50]
1412a06ac: movaps xmm10, xmmword ptr [rsp + 0x40]
1412a06b2: movaps xmm11, xmmword ptr [rsp + 0x30]
1412a06b8: movaps xmm12, xmmword ptr [rsp + 0x20]
1412a06be: add rsp, 0x98
1412a06c5: pop rbx
1412a06c6: pop rdi
1412a06c7: pop rsi
1412a06c8: pop r12
1412a06ca: pop r13
1412a06cc: pop r14
1412a06ce: pop r15
1412a06d0: pop rbp
1412a06d1: ret 
1412a06d2: nop word ptr cs:[rax + rax]
1412a06e0: mov qword ptr [rsp + 0x10], rdx
1412a06e5: push rbp
1412a06e6: push r15
1412a06e8: push r14
1412a06ea: push r13
1412a06ec: push r12
1412a06ee: push rsi
1412a06ef: push rdi
1412a06f0: push rbx
1412a06f1: sub rsp, 0x98
1412a06f8: lea rbp, [rdx + 0x80]
1412a06ff: movaps xmmword ptr [rsp + 0x20], xmm12
1412a0705: movaps xmmword ptr [rsp + 0x30], xmm11
1412a070b: movaps xmmword ptr [rsp + 0x40], xmm10
1412a0711: movaps xmmword ptr [rsp + 0x50], xmm9
1412a0717: movaps xmmword ptr [rsp + 0x60], xmm8
1412a071d: movaps xmmword ptr [rsp + 0x70], xmm7
1412a0722: movaps xmmword ptr [rsp + 0x80], xmm6
1412a072a: lea rcx, [rbp + 0x60]
1412a072e: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
1412a0733: movaps xmm6, xmmword ptr [rsp + 0x80]
1412a073b: movaps xmm7, xmmword ptr [rsp + 0x70]
1412a0740: movaps xmm8, xmmword ptr [rsp + 0x60]
1412a0746: movaps xmm9, xmmword ptr [rsp + 0x50]
1412a074c: movaps xmm10, xmmword ptr [rsp + 0x40]
1412a0752: movaps xmm11, xmmword ptr [rsp + 0x30]
1412a0758: movaps xmm12, xmmword ptr [rsp + 0x20]
1412a075e: add rsp, 0x98
1412a0765: pop rbx
1412a0766: pop rdi
1412a0767: pop rsi
1412a0768: pop r12
1412a076a: pop r13
1412a076c: pop r14
1412a076e: pop r15
1412a0770: pop rbp
1412a0771: ret 
1412a0772: nop word ptr cs:[rax + rax]
1412a0780: mov qword ptr [rsp + 0x10], rdx
1412a0785: push rbp
1412a0786: push r15
1412a0788: push r14
1412a078a: push r13
1412a078c: push r12
1412a078e: push rsi
1412a078f: push rdi
1412a0790: push rbx
1412a0791: sub rsp, 0x98
1412a0798: lea rbp, [rdx + 0x80]
1412a079f: movaps xmmword ptr [rsp + 0x20], xmm12
1412a07a5: movaps xmmword ptr [rsp + 0x30], xmm11
1412a07ab: movaps xmmword ptr [rsp + 0x40], xmm10
1412a07b1: movaps xmmword ptr [rsp + 0x50], xmm9
1412a07b7: movaps xmmword ptr [rsp + 0x60], xmm8
1412a07bd: movaps xmmword ptr [rsp + 0x70], xmm7
1412a07c2: movaps xmmword ptr [rsp + 0x80], xmm6
1412a07ca: lea rcx, [rbp + 0x60]
1412a07ce: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
1412a07d3: movaps xmm6, xmmword ptr [rsp + 0x80]
1412a07db: movaps xmm7, xmmword ptr [rsp + 0x70]
1412a07e0: movaps xmm8, xmmword ptr [rsp + 0x60]
1412a07e6: movaps xmm9, xmmword ptr [rsp + 0x50]
1412a07ec: movaps xmm10, xmmword ptr [rsp + 0x40]
1412a07f2: movaps xmm11, xmmword ptr [rsp + 0x30]
1412a07f8: movaps xmm12, xmmword ptr [rsp + 0x20]
1412a07fe: add rsp, 0x98
1412a0805: pop rbx
1412a0806: pop rdi
1412a0807: pop rsi
1412a0808: pop r12
1412a080a: pop r13
1412a080c: pop r14
1412a080e: pop r15
1412a0810: pop rbp
1412a0811: ret 
