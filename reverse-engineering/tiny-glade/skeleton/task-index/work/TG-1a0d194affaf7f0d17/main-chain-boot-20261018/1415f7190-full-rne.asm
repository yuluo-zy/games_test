1415f7190: push rbp
1415f7191: push r15
1415f7193: push r14
1415f7195: push r13
1415f7197: push r12
1415f7199: push rsi
1415f719a: push rdi
1415f719b: push rbx
1415f719c: sub rsp, 0x378
1415f71a3: lea rbp, [rsp + 0x80]
1415f71ab: movdqa xmmword ptr [rbp + 0x2e0], xmm8
1415f71b4: movdqa xmmword ptr [rbp + 0x2d0], xmm7
1415f71bc: movdqa xmmword ptr [rbp + 0x2c0], xmm6
1415f71c4: mov qword ptr [rbp + 0x2b8], 0xfffffffffffffffe
1415f71cf: mov rsi, rdx
1415f71d2: mov qword ptr [rbp + 0x88], rcx
1415f71d9: mov dword ptr [rbp + 0x220], 0
1415f71e3: pxor xmm0, xmm0
1415f71e7: movdqu xmmword ptr [rbp + 0x228], xmm0
1415f71ef: mov dword ptr [rbp + 0x238], 7
1415f71f9: mov qword ptr [rbp + 0x23c], 0
1415f7204: mov word ptr [rbp + 0x244], 0
1415f720d: mov byte ptr [rbp + 0x240], 1
1415f7214: lea rdx, [rip + 0x1686070]
1415f721b: lea rcx, [rbp + 0x220]
1415f7222: mov r8d, 0x16
1415f7228: call 0x140d59dc0 ; std::fs::OpenOptions::_open
1415f722d: mov rdi, rdx
1415f7230: test al, 1
1415f7232: jne 0x1415f805c
1415f7238: movzx eax, byte ptr [rip + 0x203d9ca]
1415f723f: mov ecx, 0x2000
1415f7244: mov edx, 1
1415f7249: call 0x140613c10
1415f724e: test rax, rax
1415f7251: je 0x1415f8093
1415f7257: mov qword ptr [rbp + 0x220], rax
1415f725e: mov qword ptr [rbp + 0x228], 0x2000
1415f7269: pxor xmm0, xmm0
1415f726d: movdqu xmmword ptr [rbp + 0x230], xmm0
1415f7275: mov qword ptr [rbp + 0x240], 0
1415f7280: mov qword ptr [rbp + 0x248], rdi
1415f7287: mov qword ptr [rbp + 0xb0], 0
1415f7292: mov qword ptr [rbp + 0xa0], 1
1415f729d: mov qword ptr [rbp + 0xa8], 0x80
1415f72a8: lea rcx, [rbp + 0x1a0]
1415f72af: lea rdx, [rbp + 0xa0]
1415f72b6: lea r8, [rbp + 0x220]
1415f72bd: call 0x14169c5b0 ; ron::options::Options::from_reader_seed<std::io::buffered::bufreader::BufReader<std::fs::File>,core::marker::PhantomData<alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >,alloc::vec::Vec<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >
1415f72c2: cmp byte ptr [rbp + 0x1a0], 0x32
1415f72c9: jne 0x1415f80b5
1415f72cf: mov rcx, qword ptr [rbp + 0x1a8]
1415f72d6: mov rax, qword ptr [rbp + 0x1b0]
1415f72dd: mov rdx, qword ptr [rbp + 0x1b8]
1415f72e4: lea rdx, [rdx + rdx*4]
1415f72e8: shl rdx, 4
1415f72ec: add rdx, rax
1415f72ef: mov qword ptr [rbp + 0x40], rax
1415f72f3: mov qword ptr [rbp + 0x48], rax
1415f72f7: mov qword ptr [rbp + 0x50], rcx
1415f72fb: mov qword ptr [rbp + 0x98], rdx
1415f7302: mov qword ptr [rbp + 0x58], rdx
1415f7306: lea r12, [rbp + 0x1f8]
1415f730d: lea r13, [rbp + 0x220]
1415f7314: pcmpeqd xmm6, xmm6
1415f7318: mov qword ptr [rbp + 0x80], rsi
1415f731f: jmp 0x1415f7343 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7321: nop word ptr cs:[rax + rax]
1415f7330: lea rcx, [rbp + 0xa0]
1415f7337: call 0x1408919b0 ; core::ptr::drop_in_place<country_core::startup::load_mesh_atlases::MeshAttribConfig>
1415f733c: mov rax, qword ptr [rbp + 0x90]
1415f7343: cmp rax, qword ptr [rbp + 0x98]
1415f734a: je 0x1415f7ea7
1415f7350: lea r8, [rax + 0x50]
1415f7354: mov qword ptr [rbp + 0x48], r8
1415f7358: mov rcx, qword ptr [rax]
1415f735b: mov rdx, qword ptr [rax + 0x48]
1415f735f: mov qword ptr [rbp], rdx
1415f7363: movups xmm0, xmmword ptr [rax + 0x38]
1415f7367: movaps xmmword ptr [rbp - 0x10], xmm0
1415f736b: movups xmm0, xmmword ptr [rax + 0x28]
1415f736f: movaps xmmword ptr [rbp - 0x20], xmm0
1415f7373: movups xmm0, xmmword ptr [rax + 0x18]
1415f7377: movaps xmmword ptr [rbp - 0x30], xmm0
1415f737b: movups xmm0, xmmword ptr [rax + 8]
1415f737f: movaps xmmword ptr [rbp - 0x40], xmm0
1415f7383: mov rax, rcx
1415f7386: neg rax
1415f7389: jo 0x1415f7ea7
1415f738f: mov qword ptr [rbp + 0x90], r8
1415f7396: mov qword ptr [rbp + 0xa0], rcx
1415f739d: mov rax, qword ptr [rbp]
1415f73a1: lea rcx, [rbp + 0xa8]
1415f73a8: mov qword ptr [rcx + 0x40], rax
1415f73ac: movdqa xmm0, xmmword ptr [rbp - 0x40]
1415f73b1: movaps xmm1, xmmword ptr [rbp - 0x30]
1415f73b5: movaps xmm2, xmmword ptr [rbp - 0x20]
1415f73b9: movaps xmm3, xmmword ptr [rbp - 0x10]
1415f73bd: movups xmmword ptr [rcx + 0x30], xmm3
1415f73c1: movups xmmword ptr [rcx + 0x20], xmm2
1415f73c5: movups xmmword ptr [rcx + 0x10], xmm1
1415f73c9: movdqu xmmword ptr [rcx], xmm0
1415f73cd: movzx eax, byte ptr [rbp + 0xe8]
1415f73d4: mov byte ptr [rbp + 0x2b4], al
1415f73da: cmp al, 9
1415f73dc: je 0x1415f8014
1415f73e2: mov rsi, qword ptr [rbp + 0xd0]
1415f73e9: mov rax, qword ptr [rbp + 0xd8]
1415f73f0: mov rdi, qword ptr [rbp + 0xe0]
1415f73f7: shl rdi, 5
1415f73fb: lea rcx, [rax + rdi]
1415f73ff: mov qword ptr [rbp + 0x218], rcx
1415f7406: add rdi, -0x20
1415f740a: lea rbx, [rax + 0x28]
1415f740e: mov qword ptr [rbp + 0x198], rax
1415f7415: mov qword ptr [rbp + 0x190], rsi
1415f741c: jmp 0x1415f742b ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f741e: nop 
1415f7420: add rdi, -0x20
1415f7424: add rbx, 0x20
1415f7428: mov rax, r14
1415f742b: cmp rax, qword ptr [rbp + 0x218]
1415f7432: je 0x1415f7e80
1415f7438: lea r14, [rax + 0x20]
1415f743c: mov rdx, qword ptr [rax]
1415f743f: mov rcx, qword ptr [rax + 0x18]
1415f7443: mov qword ptr [rbp + 0x30], rcx
1415f7447: movdqu xmm0, xmmword ptr [rax + 8]
1415f744c: movdqa xmmword ptr [rbp + 0x20], xmm0
1415f7451: mov rax, rdx
1415f7454: neg rax
1415f7457: jo 0x1415f7e40
1415f745d: mov qword ptr [rbp + 0x138], rdx
1415f7464: mov rax, qword ptr [rbp + 0x30]
1415f7468: lea rcx, [rbp + 0x140]
1415f746f: mov qword ptr [rcx + 0x10], rax
1415f7473: movdqa xmm0, xmmword ptr [rbp + 0x20]
1415f7478: movdqu xmmword ptr [rcx], xmm0
1415f747c: cmp byte ptr [rip + 0x203d90d], 0
1415f7483: je 0x1415f7492
1415f7485: cmp byte ptr [rbp + 0x150], 1
1415f748c: je 0x1415f7e11
1415f7492: mov qword ptr [rbp + 0x188], r14
1415f7499: mov qword ptr [rbp + 0x180], rbx
1415f74a0: mov qword ptr [rbp + 0x178], rdi
1415f74a7: lea rax, [rbp + 0x138]
1415f74ae: mov qword ptr [rbp + 0x1a0], rax
1415f74b5: lea rax, [rip - 0x15bb9ec]
1415f74bc: mov qword ptr [rbp + 0x1a8], rax
1415f74c3: lea rax, [rip + 0x1685eee]
1415f74ca: mov qword ptr [rbp + 0x220], rax
1415f74d1: mov qword ptr [rbp + 0x228], 2
1415f74dc: lea rax, [rbp + 0x1a0]
1415f74e3: mov qword ptr [rbp + 0x230], rax
1415f74ea: mov qword ptr [rbp + 0x238], 1
1415f74f5: mov qword ptr [rbp + 0x240], 0
1415f7500: lea rcx, [rbp - 0x58]
1415f7504: mov rdx, r13
1415f7507: call 0x140d79f90 ; alloc::fmt::format::format_inner
1415f750c: movdqu xmm0, xmmword ptr [rbp - 0x58]
1415f7511: movdqa xmmword ptr [rbp + 0x110], xmm0
1415f7519: mov rax, qword ptr [rbp - 0x48]
1415f751d: mov qword ptr [rbp + 0x120], rax
1415f7524: mov byte ptr [rbp + 0x128], 1
1415f752b: mov byte ptr [rbp + 0x2b6], 1
1415f7532: mov rcx, r13
1415f7535: lea rdx, [rbp + 0x110]
1415f753c: call 0x140b11f50 ; country_core::utils::load_json::load_json_as_mesh<ref$<std::path::PathBuf> >
1415f7541: xor eax, eax
1415f7543: cmp rax, qword ptr [rbp + 0x220]
1415f754a: jo 0x1415f7f8e
1415f7550: mov rax, qword ptr [rbp + 0x290]
1415f7557: mov qword ptr [rbp + 0x210], rax
1415f755e: movups xmm0, xmmword ptr [rbp + 0x280]
1415f7565: movaps xmmword ptr [rbp + 0x200], xmm0
1415f756c: movups xmm0, xmmword ptr [rbp + 0x270]
1415f7573: movaps xmmword ptr [rbp + 0x1f0], xmm0
1415f757a: movups xmm0, xmmword ptr [rbp + 0x260]
1415f7581: movaps xmmword ptr [rbp + 0x1e0], xmm0
1415f7588: movdqu xmm0, xmmword ptr [rbp + 0x220]
1415f7590: movups xmm1, xmmword ptr [rbp + 0x230]
1415f7597: movups xmm2, xmmword ptr [rbp + 0x240]
1415f759e: movups xmm3, xmmword ptr [rbp + 0x250]
1415f75a5: movaps xmmword ptr [rbp + 0x1d0], xmm3
1415f75ac: movaps xmmword ptr [rbp + 0x1c0], xmm2
1415f75b3: movaps xmmword ptr [rbp + 0x1b0], xmm1
1415f75ba: movdqa xmmword ptr [rbp + 0x1a0], xmm0
1415f75c2: mov rdx, qword ptr [rbp + 0x80]
1415f75c9: test rdx, rdx
1415f75cc: je 0x1415f76c0
1415f75d2: movdqa xmm0, xmmword ptr [rbp + 0x110]
1415f75da: movaps xmm1, xmmword ptr [rbp + 0x120]
1415f75e1: movaps xmmword ptr [rbp + 0x230], xmm1
1415f75e8: movdqa xmmword ptr [rbp + 0x220], xmm0
1415f75f0: cmp dword ptr [rdx], 3
1415f75f3: je 0x1415f762e
1415f75f5: movzx eax, byte ptr [rbp + 0xe8]
1415f75fc: mov r9, qword ptr [rdx + 0x38]
1415f7600: lock inc qword ptr [r9]
1415f7604: jle 0x1415f8136
1415f760a: mov byte ptr [rsp + 0x20], al
1415f760e: lea rcx, [rbp + 8]
1415f7612: mov r8, r13
1415f7615: call 0x14091d540 ; hotwatch::GenericHotwatch::watch<notify::windows::ReadDirectoryChangesWatcher,ref$<std::path::PathBuf>,country_core::startup::load_prefabs::impl$1::watch_atlas::closure_env$0>
1415f761a: movabs rax, 0x8000000000000004
1415f7624: cmp qword ptr [rbp + 8], rax
1415f7628: jne 0x1415f7fcc
1415f762e: mov rdx, qword ptr [rbp + 0x220]
1415f7635: test rdx, rdx
1415f7638: je 0x1415f764c
1415f763a: mov rcx, qword ptr [rbp + 0x228]
1415f7641: mov r8d, 1
1415f7647: call 0x140613c20
1415f764c: xor r15d, r15d
1415f764f: cmp qword ptr [rbp + 0x1f0], 0
1415f7657: je 0x1415f76cd
1415f7659: mov rax, qword ptr [rbp + 0x1d8]
1415f7660: movdqa xmm0, xmmword ptr [rax]
1415f7664: pmovmskb ecx, xmm0
1415f7668: cmp ecx, 0xffff
1415f766e: jne 0x1415f769a
1415f7670: lea rdx, [rax + 0x10]
1415f7674: nop word ptr cs:[rax + rax]
1415f7680: movdqa xmm0, xmmword ptr [rdx]
1415f7684: pmovmskb ecx, xmm0
1415f7688: add rax, -0x380
1415f768e: add rdx, 0x10
1415f7692: cmp ecx, 0xffff
1415f7698: je 0x1415f7680
1415f769a: not ecx
1415f769c: tzcnt ecx, ecx
1415f76a0: neg rcx
1415f76a3: imul rcx, rcx, 0x38
1415f76a7: mov rax, qword ptr [rax + rcx - 8]
1415f76ac: mov qword ptr [rbp + 0x2a8], rax
1415f76b3: jmp 0x1415f76d8 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f76b5: nop word ptr cs:[rax + rax]
1415f76c0: mov r15b, 1
1415f76c3: cmp qword ptr [rbp + 0x1f0], 0
1415f76cb: jne 0x1415f7659
1415f76cd: mov qword ptr [rbp + 0x2a8], 0
1415f76d8: mov rsi, qword ptr [rbp + 0xa8]
1415f76df: mov rax, qword ptr [rbp + 0xb0]
1415f76e6: lea rax, [rax + rax*2]
1415f76ea: lea r14, [rsi + rax*8]
1415f76ee: mov dword ptr [rbp + 0x2a0], r15d
1415f76f5: jmp 0x1415f7707 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f76f7: nop word ptr [rax + rax]
1415f7700: mov r15d, dword ptr [rbp + 0x2a0]
1415f7707: cmp rsi, r14
1415f770a: je 0x1415f7820
1415f7710: mov byte ptr [rbp + 0x2b7], r15b
1415f7717: mov rcx, r12
1415f771a: mov rdx, rsi
1415f771d: call 0x1405b18f0 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<alloc::string::String> >
1415f7722: mov byte ptr [rbp + 0x2b7], r15b
1415f7729: mov rcx, r13
1415f772c: lea rdx, [rbp + 0x1d8]
1415f7733: mov r8, rax
1415f7736: mov r9, rsi
1415f7739: call 0x140b77de0 ; hashbrown::raw::RawTable::remove_entry<tuple$<alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<alloc::string::String,alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> > >
1415f773e: add rsi, 0x18
1415f7742: mov r15, qword ptr [rbp + 0x238]
1415f7749: cmp r15, 5
1415f774d: je 0x1415f7700
1415f774f: mov rdx, qword ptr [rbp + 0x220]
1415f7756: mov rdi, qword ptr [rbp + 0x240]
1415f775d: mov rbx, qword ptr [rbp + 0x248]
1415f7764: test rdx, rdx
1415f7767: je 0x1415f777b
1415f7769: mov rcx, qword ptr [rbp + 0x228]
1415f7770: mov r8d, 1
1415f7776: call 0x140613c20
1415f777b: cmp r15, 3
1415f777f: ja 0x1415f77e8
1415f7781: lea rcx, [rip + 0x16860d8]
1415f7788: movsxd rax, dword ptr [rcx + r15*4]
1415f778c: add rax, rcx
1415f778f: jmp rax
1415f7791: test rdi, rdi
1415f7794: mov r15d, dword ptr [rbp + 0x2a0]
1415f779b: je 0x1415f7707
1415f77a1: shl rdi, 2
1415f77a5: jmp 0x1415f77fc ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f77a7: test rdi, rdi
1415f77aa: mov r15d, dword ptr [rbp + 0x2a0]
1415f77b1: je 0x1415f7707
1415f77b7: shl rdi, 3
1415f77bb: jmp 0x1415f77fc ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f77bd: test rdi, rdi
1415f77c0: mov r15d, dword ptr [rbp + 0x2a0]
1415f77c7: je 0x1415f7707
1415f77cd: shl rdi, 2
1415f77d1: lea rdx, [rdi + rdi*2]
1415f77d5: mov r8d, 4
1415f77db: mov rcx, rbx
1415f77de: call 0x140613c20
1415f77e3: jmp 0x1415f7707 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f77e8: test rdi, rdi
1415f77eb: mov r15d, dword ptr [rbp + 0x2a0]
1415f77f2: je 0x1415f7707
1415f77f8: shl rdi, 4
1415f77fc: mov r8d, 4
1415f7802: mov rcx, rbx
1415f7805: mov rdx, rdi
1415f7808: call 0x140613c20
1415f780d: jmp 0x1415f7707 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7812: nop word ptr cs:[rax + rax]
1415f7820: mov rdi, qword ptr [rbp + 0xc0]
1415f7827: mov rax, qword ptr [rbp + 0xc8]
1415f782e: shl rax, 5
1415f7832: add rax, rdi
1415f7835: mov qword ptr [rbp + 0x60], rax
1415f7839: mov r8, qword ptr [rbp + 0x2a8]
1415f7840: mov rdx, r8
1415f7843: shl rdx, 4
1415f7847: mov rax, r8
1415f784a: shr rax, 0x3c
1415f784e: setne al
1415f7851: movabs rcx, 0x7ffffffffffffffc
1415f785b: mov qword ptr [rbp + 0x160], rdx
1415f7862: cmp rdx, rcx
1415f7865: seta dl
1415f7868: or dl, al
1415f786a: mov byte ptr [rbp + 0x2b1], dl
1415f7870: mov rax, r8
1415f7873: mov edx, 0xc
1415f7878: mul rdx
1415f787b: seto dl
1415f787e: mov qword ptr [rbp + 0x168], rax
1415f7885: cmp rax, rcx
1415f7888: seta al
1415f788b: or al, dl
1415f788d: mov byte ptr [rbp + 0x2b2], al
1415f7893: lea rdx, [r8*8]
1415f789b: mov rax, r8
1415f789e: shr rax, 0x3d
1415f78a2: setne al
1415f78a5: mov qword ptr [rbp + 0x170], rdx
1415f78ac: cmp rdx, rcx
1415f78af: seta dl
1415f78b2: or dl, al
1415f78b4: mov byte ptr [rbp + 0x2b3], dl
1415f78ba: lea rdx, [r8*4]
1415f78c2: shr r8, 0x3e
1415f78c6: setne al
1415f78c9: mov qword ptr [rbp + 0x298], rdx
1415f78d0: cmp rdx, rcx
1415f78d3: seta cl
1415f78d6: or cl, al
1415f78d8: mov byte ptr [rbp + 0x2b5], cl
1415f78de: jmp 0x1415f7998 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f78e3: nop word ptr cs:[rax + rax]
1415f78f0: movdqu xmm8, xmmword ptr [rbx + r13]
1415f78f6: movdqa xmm0, xmm8
1415f78fb: pcmpeqb xmm0, xmm7
1415f78ff: pmovmskb r14d, xmm0
1415f7904: test r14d, r14d
1415f7907: je 0x1415f7950
1415f7909: mov qword ptr [rbp + 0x78], rcx
1415f790d: tzcnt eax, r14d
1415f7912: add rax, r13
1415f7915: and rax, r15
1415f7918: neg rax
1415f791b: imul rax, rax, 0x38
1415f791f: mov r8, qword ptr [rbp + 0x68]
1415f7923: cmp qword ptr [r12 + rax + 0x10], r8
1415f7928: jne 0x1415f793e
1415f792a: add rax, r12
1415f792d: mov rdx, qword ptr [rax + 8]
1415f7931: mov rcx, qword ptr [rbp + 0x70]
1415f7935: call 0x142923bf0
1415f793a: test eax, eax
1415f793c: je 0x1415f7980
1415f793e: lea eax, [r14 - 1]
1415f7942: and ax, r14w
1415f7946: mov r14d, eax
1415f7949: mov rcx, qword ptr [rbp + 0x78]
1415f794d: jne 0x1415f790d
1415f794f: nop 
1415f7950: pcmpeqb xmm8, xmm6
1415f7955: pmovmskb eax, xmm8
1415f795a: test eax, eax
1415f795c: jne 0x1415f7a20
1415f7962: add r13, rcx
1415f7965: add r13, 0x10
1415f7969: add rcx, 0x10
1415f796d: and r13, r15
1415f7970: jmp 0x1415f78f0 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7975: nop word ptr cs:[rax + rax]
1415f7980: mov rdi, qword ptr [rbp + 0x158]
1415f7987: mov r12, rsi
1415f798a: lea r13, [rbp + 0x220]
1415f7991: mov r15d, dword ptr [rbp + 0x2a0]
1415f7998: cmp rdi, qword ptr [rbp + 0x60]
1415f799c: je 0x1415f7d30
1415f79a2: lea rsi, [rdi + 0x20]
1415f79a6: mov r14, qword ptr [rdi + 8]
1415f79aa: mov rbx, qword ptr [rdi + 0x10]
1415f79ae: cmp qword ptr [rbp + 0x1f0], 0
1415f79b6: je 0x1415f7a40
1415f79bc: mov byte ptr [rbp + 0x2b7], r15b
1415f79c3: mov rcx, r12
1415f79c6: mov rdx, r14
1415f79c9: mov r8, rbx
1415f79cc: call 0x1405b1a70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<str$> >
1415f79d1: mov qword ptr [rbp + 0x70], r14
1415f79d5: mov qword ptr [rbp + 0x68], rbx
1415f79d9: mov qword ptr [rbp + 0x158], rsi
1415f79e0: mov rsi, r12
1415f79e3: mov rbx, qword ptr [rbp + 0x1d8]
1415f79ea: mov r15, qword ptr [rbp + 0x1e0]
1415f79f1: mov r13, r15
1415f79f4: and r13, rax
1415f79f7: shr rax, 0x39
1415f79fb: movd xmm0, eax
1415f79ff: punpcklbw xmm0, xmm0
1415f7a03: pshuflw xmm0, xmm0, 0
1415f7a08: pshufd xmm7, xmm0, 0
1415f7a0d: lea r12, [rbx - 0x38]
1415f7a11: xor ecx, ecx
1415f7a13: jmp 0x1415f78f0 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7a18: nop dword ptr [rax + rax]
1415f7a20: mov r14, qword ptr [rdi + 8]
1415f7a24: mov rbx, qword ptr [rdi + 0x10]
1415f7a28: mov r12, rsi
1415f7a2b: lea r13, [rbp + 0x220]
1415f7a32: mov r15d, dword ptr [rbp + 0x2a0]
1415f7a39: mov rsi, qword ptr [rbp + 0x158]
1415f7a40: movzx eax, byte ptr [rdi + 0x18]
1415f7a44: lea rcx, [rip + 0x1685e25]
1415f7a4b: movsxd rax, dword ptr [rcx + rax*4]
1415f7a4f: add rax, rcx
1415f7a52: jmp rax
1415f7a54: cmp byte ptr [rbp + 0x2b5], 0
1415f7a5b: mov rcx, qword ptr [rbp + 0x298]
1415f7a62: jne 0x1415f7f46
1415f7a68: test rcx, rcx
1415f7a6b: je 0x1415f7ca0
1415f7a71: movzx eax, byte ptr [rip + 0x203d191]
1415f7a78: mov edi, 4
1415f7a7d: mov edx, 4
1415f7a82: call 0x140613c40
1415f7a87: test rax, rax
1415f7a8a: je 0x1415f7f48
1415f7a90: mov rdx, qword ptr [rbp + 0x2a8]
1415f7a97: mov rcx, rdx
1415f7a9a: jmp 0x1415f7cae ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7a9f: cmp byte ptr [rbp + 0x2b1], 0
1415f7aa6: mov rcx, qword ptr [rbp + 0x160]
1415f7aad: jne 0x1415f7f6a
1415f7ab3: test rcx, rcx
1415f7ab6: je 0x1415f7bcb
1415f7abc: movzx eax, byte ptr [rip + 0x203d146]
1415f7ac3: mov edi, 4
1415f7ac8: mov edx, 4
1415f7acd: call 0x140613c40
1415f7ad2: test rax, rax
1415f7ad5: je 0x1415f7f6c
1415f7adb: mov rdx, qword ptr [rbp + 0x2a8]
1415f7ae2: mov rcx, rdx
1415f7ae5: jmp 0x1415f7bd9 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7aea: cmp byte ptr [rbp + 0x2b3], 0
1415f7af1: jne 0x1415f7f22
1415f7af7: mov rcx, qword ptr [rbp + 0x170]
1415f7afe: test rcx, rcx
1415f7b01: je 0x1415f7c12
1415f7b07: movzx eax, byte ptr [rip + 0x203d0fb]
1415f7b0e: mov edi, 4
1415f7b13: mov edx, 4
1415f7b18: call 0x140613c40
1415f7b1d: test rax, rax
1415f7b20: je 0x1415f7f24
1415f7b26: mov rdx, qword ptr [rbp + 0x2a8]
1415f7b2d: mov rcx, rdx
1415f7b30: jmp 0x1415f7c20 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7b35: cmp byte ptr [rbp + 0x2b2], 0
1415f7b3c: jne 0x1415f7eda
1415f7b42: mov rcx, qword ptr [rbp + 0x168]
1415f7b49: test rcx, rcx
1415f7b4c: je 0x1415f7c59
1415f7b52: movzx eax, byte ptr [rip + 0x203d0b0]
1415f7b59: mov edi, 4
1415f7b5e: mov edx, 4
1415f7b63: call 0x140613c40
1415f7b68: test rax, rax
1415f7b6b: je 0x1415f7edc
1415f7b71: mov rdx, qword ptr [rbp + 0x2a8]
1415f7b78: mov rcx, rdx
1415f7b7b: jmp 0x1415f7c67 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7b80: cmp byte ptr [rbp + 0x2b5], 0
1415f7b87: mov rcx, qword ptr [rbp + 0x298]
1415f7b8e: jne 0x1415f7efe
1415f7b94: test rcx, rcx
1415f7b97: je 0x1415f7ce7
1415f7b9d: movzx eax, byte ptr [rip + 0x203d065]
1415f7ba4: mov edi, 4
1415f7ba9: mov edx, 4
1415f7bae: call 0x140613c40
1415f7bb3: test rax, rax
1415f7bb6: je 0x1415f7f00
1415f7bbc: mov rdx, qword ptr [rbp + 0x2a8]
1415f7bc3: mov rcx, rdx
1415f7bc6: jmp 0x1415f7cf5 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7bcb: mov eax, 4
1415f7bd0: xor ecx, ecx
1415f7bd2: mov rdx, qword ptr [rbp + 0x2a8]
1415f7bd9: mov qword ptr [rbp + 0x220], rcx
1415f7be0: mov qword ptr [rbp + 0x228], rax
1415f7be7: mov qword ptr [rbp + 0x230], rdx
1415f7bee: mov byte ptr [rbp + 0x2b7], r15b
1415f7bf5: lea rcx, [rbp + 0x1a0]
1415f7bfc: mov rdx, r14
1415f7bff: mov r8, rbx
1415f7c02: mov r9, r13
1415f7c05: call 0x140a34170 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,4>,alloc::alloc::Global> >
1415f7c0a: mov rdi, rsi
1415f7c0d: jmp 0x1415f7998 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7c12: mov eax, 4
1415f7c17: xor ecx, ecx
1415f7c19: mov rdx, qword ptr [rbp + 0x2a8]
1415f7c20: mov qword ptr [rbp + 0x220], rcx
1415f7c27: mov qword ptr [rbp + 0x228], rax
1415f7c2e: mov qword ptr [rbp + 0x230], rdx
1415f7c35: mov byte ptr [rbp + 0x2b7], r15b
1415f7c3c: lea rcx, [rbp + 0x1a0]
1415f7c43: mov rdx, r14
1415f7c46: mov r8, rbx
1415f7c49: mov r9, r13
1415f7c4c: call 0x140a355f0 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global> >
1415f7c51: mov rdi, rsi
1415f7c54: jmp 0x1415f7998 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7c59: mov eax, 4
1415f7c5e: xor ecx, ecx
1415f7c60: mov rdx, qword ptr [rbp + 0x2a8]
1415f7c67: mov qword ptr [rbp + 0x220], rcx
1415f7c6e: mov qword ptr [rbp + 0x228], rax
1415f7c75: mov qword ptr [rbp + 0x230], rdx
1415f7c7c: mov byte ptr [rbp + 0x2b7], r15b
1415f7c83: lea rcx, [rbp + 0x1a0]
1415f7c8a: mov rdx, r14
1415f7c8d: mov r8, rbx
1415f7c90: mov r9, r13
1415f7c93: call 0x140a34c90 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> >
1415f7c98: mov rdi, rsi
1415f7c9b: jmp 0x1415f7998 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7ca0: mov eax, 4
1415f7ca5: xor ecx, ecx
1415f7ca7: mov rdx, qword ptr [rbp + 0x2a8]
1415f7cae: mov qword ptr [rbp + 0x220], rcx
1415f7cb5: mov qword ptr [rbp + 0x228], rax
1415f7cbc: mov qword ptr [rbp + 0x230], rdx
1415f7cc3: mov byte ptr [rbp + 0x2b7], r15b
1415f7cca: lea rcx, [rbp + 0x1a0]
1415f7cd1: mov rdx, r14
1415f7cd4: mov r8, rbx
1415f7cd7: mov r9, r13
1415f7cda: call 0x140a34700 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<i32,alloc::alloc::Global> >
1415f7cdf: mov rdi, rsi
1415f7ce2: jmp 0x1415f7998 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7ce7: mov eax, 4
1415f7cec: xor ecx, ecx
1415f7cee: mov rdx, qword ptr [rbp + 0x2a8]
1415f7cf5: mov qword ptr [rbp + 0x220], rcx
1415f7cfc: mov qword ptr [rbp + 0x228], rax
1415f7d03: mov qword ptr [rbp + 0x230], rdx
1415f7d0a: mov byte ptr [rbp + 0x2b7], r15b
1415f7d11: lea rcx, [rbp + 0x1a0]
1415f7d18: mov rdx, r14
1415f7d1b: mov r8, rbx
1415f7d1e: mov r9, r13
1415f7d21: call 0x140a35b80 ; country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<f32,alloc::alloc::Global> >
1415f7d26: mov rdi, rsi
1415f7d29: jmp 0x1415f7998 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7d2e: nop 
1415f7d30: movzx edx, byte ptr [rbp + 0xe8]
1415f7d37: mov r8, qword ptr [rbp + 0x140]
1415f7d3e: mov r9, qword ptr [rbp + 0x148]
1415f7d45: mov byte ptr [rbp + 0x2b7], r15b
1415f7d4c: lea rax, [rbp + 0x1a0]
1415f7d53: mov qword ptr [rsp + 0x20], rax
1415f7d58: mov rcx, qword ptr [rbp + 0x88]
1415f7d5f: call 0x140a371d0 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::add<ref$<str$> >
1415f7d64: mov r8d, 0x38
1415f7d6a: mov r9d, 0x10
1415f7d70: lea rcx, [rbp + 0x1d8]
1415f7d77: mov rdx, r12
1415f7d7a: call 0x140b75360 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<alloc::string::String,enum2$<country_core::render::mesh::VertexAttributeValues> >,alloc::alloc::Global>
1415f7d7f: mov rdx, qword ptr [rbp + 0x1a0]
1415f7d86: test rdx, rdx
1415f7d89: je 0x1415f7da1
1415f7d8b: mov rcx, qword ptr [rbp + 0x1a8]
1415f7d92: shl rdx, 2
1415f7d96: mov r8d, 4
1415f7d9c: call 0x140613c20
1415f7da1: mov rdx, qword ptr [rbp + 0x1b8]
1415f7da8: mov rax, rdx
1415f7dab: neg rax
1415f7dae: mov rsi, qword ptr [rbp + 0x190]
1415f7db5: mov rdi, qword ptr [rbp + 0x178]
1415f7dbc: mov rbx, qword ptr [rbp + 0x180]
1415f7dc3: mov r14, qword ptr [rbp + 0x188]
1415f7dca: mov r15d, dword ptr [rbp + 0x2a0]
1415f7dd1: jo 0x1415f7de7
1415f7dd3: jae 0x1415f7de7
1415f7dd5: mov rcx, qword ptr [rbp + 0x1c0]
1415f7ddc: mov r8d, 1
1415f7de2: call 0x140613c20
1415f7de7: test r15b, r15b
1415f7dea: je 0x1415f7e0a
1415f7dec: mov rdx, qword ptr [rbp + 0x110]
1415f7df3: test rdx, rdx
1415f7df6: je 0x1415f7e0a
1415f7df8: mov rcx, qword ptr [rbp + 0x118]
1415f7dff: mov r8d, 1
1415f7e05: call 0x140613c20
1415f7e0a: mov rdx, qword ptr [rbp + 0x138]
1415f7e11: test rdx, rdx
1415f7e14: je 0x1415f7420
1415f7e1a: mov rcx, qword ptr [rbp + 0x140]
1415f7e21: mov r8d, 1
1415f7e27: call 0x140613c20
1415f7e2c: jmp 0x1415f7420 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7e31: nop word ptr cs:[rax + rax]
1415f7e40: cmp qword ptr [rbp + 0x218], r14
1415f7e47: je 0x1415f7e80
1415f7e49: shr rdi, 5
1415f7e4d: jmp 0x1415f7e59 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7e4f: nop 
1415f7e50: add rbx, 0x20
1415f7e54: dec rdi
1415f7e57: je 0x1415f7e80
1415f7e59: mov rdx, qword ptr [rbx - 8]
1415f7e5d: test rdx, rdx
1415f7e60: je 0x1415f7e50
1415f7e62: mov rcx, qword ptr [rbx]
1415f7e65: mov r8d, 1
1415f7e6b: call 0x140613c20
1415f7e70: jmp 0x1415f7e50 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7e72: nop word ptr cs:[rax + rax]
1415f7e80: test rsi, rsi
1415f7e83: je 0x1415f7330
1415f7e89: shl rsi, 5
1415f7e8d: mov r8d, 8
1415f7e93: mov rcx, qword ptr [rbp + 0x198]
1415f7e9a: mov rdx, rsi
1415f7e9d: call 0x140613c20
1415f7ea2: jmp 0x1415f7330 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7ea7: lea rcx, [rbp + 0x40]
1415f7eab: call 0x14087bd60 ; core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >
1415f7eb0: movaps xmm6, xmmword ptr [rbp + 0x2c0]
1415f7eb7: movaps xmm7, xmmword ptr [rbp + 0x2d0]
1415f7ebe: movaps xmm8, xmmword ptr [rbp + 0x2e0]
1415f7ec6: add rsp, 0x378
1415f7ecd: pop rbx
1415f7ece: pop rdi
1415f7ecf: pop rsi
1415f7ed0: pop r12
1415f7ed2: pop r13
1415f7ed4: pop r14
1415f7ed6: pop r15
1415f7ed8: pop rbp
1415f7ed9: ret 
1415f7eda: xor edi, edi
1415f7edc: mov byte ptr [rbp + 0x2b7], r15b
1415f7ee3: lea r8, [rip + 0x168554e]
1415f7eea: mov rcx, rdi
1415f7eed: mov rdx, qword ptr [rbp + 0x168]
1415f7ef4: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1415f7ef9: jmp 0x1415f8136 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7efe: xor edi, edi
1415f7f00: mov byte ptr [rbp + 0x2b7], r15b
1415f7f07: lea r8, [rip + 0x16854fa]
1415f7f0e: mov rcx, rdi
1415f7f11: mov rdx, qword ptr [rbp + 0x298]
1415f7f18: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1415f7f1d: jmp 0x1415f8136 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7f22: xor edi, edi
1415f7f24: mov byte ptr [rbp + 0x2b7], r15b
1415f7f2b: lea r8, [rip + 0x16854ee]
1415f7f32: mov rcx, rdi
1415f7f35: mov rdx, qword ptr [rbp + 0x170]
1415f7f3c: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1415f7f41: jmp 0x1415f8136 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7f46: xor edi, edi
1415f7f48: mov byte ptr [rbp + 0x2b7], r15b
1415f7f4f: lea r8, [rip + 0x168549a]
1415f7f56: mov rcx, rdi
1415f7f59: mov rdx, qword ptr [rbp + 0x298]
1415f7f60: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1415f7f65: jmp 0x1415f8136 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7f6a: xor edi, edi
1415f7f6c: mov byte ptr [rbp + 0x2b7], r15b
1415f7f73: lea r8, [rip + 0x16854d6]
1415f7f7a: mov rcx, rdi
1415f7f7d: mov rdx, qword ptr [rbp + 0x160]
1415f7f84: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1415f7f89: jmp 0x1415f8136 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7f8e: mov rax, qword ptr [rbp + 0x228]
1415f7f95: mov qword ptr [rbp + 0xf0], rax
1415f7f9c: lea rax, [rip + 0x1685435]
1415f7fa3: mov qword ptr [rsp + 0x20], rax
1415f7fa8: lea rcx, [rip + 0x16849d9]
1415f7faf: lea r9, [rip + 0x16849b2]
1415f7fb6: lea r8, [rbp + 0xf0]
1415f7fbd: mov edx, 0x2b
1415f7fc2: call 0x1428d9760 ; core::result::unwrap_failed
1415f7fc7: jmp 0x1415f8136 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f7fcc: mov rax, qword ptr [rbp + 0x18]
1415f7fd0: mov qword ptr [rbp + 0x100], rax
1415f7fd7: movdqu xmm0, xmmword ptr [rbp + 8]
1415f7fdc: movdqa xmmword ptr [rbp + 0xf0], xmm0
1415f7fe4: lea rax, [rip + 0x1632add]
1415f7feb: mov qword ptr [rsp + 0x20], rax
1415f7ff0: lea rcx, [rip + 0x1632a99]
1415f7ff7: lea r9, [rip + 0x1631fda]
1415f7ffe: lea r8, [rbp + 0xf0]
1415f8005: mov edx, 0x1e
1415f800a: call 0x1428d9760 ; core::result::unwrap_failed
1415f800f: jmp 0x1415f8136 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f8014: lea rax, [rip + 0x168535d]
1415f801b: mov qword ptr [rbp + 0x220], rax
1415f8022: mov qword ptr [rbp + 0x228], 1
1415f802d: mov qword ptr [rbp + 0x230], 8
1415f8038: pxor xmm0, xmm0
1415f803c: movdqu xmmword ptr [rbp + 0x238], xmm0
1415f8044: lea rdx, [rip + 0x168533d]
1415f804b: lea rcx, [rbp + 0x220]
1415f8052: call 0x1428d9390 ; core::panicking::panic_fmt
1415f8057: jmp 0x1415f8136 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f805c: mov qword ptr [rbp + 0x220], rdi
1415f8063: lea rax, [rip + 0x1685286]
1415f806a: mov qword ptr [rsp + 0x20], rax
1415f806f: lea rcx, [rip + 0x168522b]
1415f8076: lea r9, [rip + 0x16848ab]
1415f807d: lea r8, [rbp + 0x220]
1415f8084: mov edx, 0x14
1415f8089: call 0x1428d9760 ; core::result::unwrap_failed
1415f808e: jmp 0x1415f8136 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f8093: mov qword ptr [rbp + 0x2a0], rdi
1415f809a: lea r8, [rip + 0x168ef27]
1415f80a1: mov ecx, 1
1415f80a6: mov edx, 0x2000
1415f80ab: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1415f80b0: jmp 0x1415f8136 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f80b5: mov rax, qword ptr [rbp + 0x1f0]
1415f80bc: mov qword ptr [rbp + 0x270], rax
1415f80c3: movups xmm0, xmmword ptr [rbp + 0x1e0]
1415f80ca: movaps xmmword ptr [rbp + 0x260], xmm0
1415f80d1: movdqu xmm0, xmmword ptr [rbp + 0x1a0]
1415f80d9: movups xmm1, xmmword ptr [rbp + 0x1b0]
1415f80e0: movups xmm2, xmmword ptr [rbp + 0x1c0]
1415f80e7: movups xmm3, xmmword ptr [rbp + 0x1d0]
1415f80ee: movaps xmmword ptr [rbp + 0x250], xmm3
1415f80f5: movaps xmmword ptr [rbp + 0x240], xmm2
1415f80fc: movaps xmmword ptr [rbp + 0x230], xmm1
1415f8103: movdqa xmmword ptr [rbp + 0x220], xmm0
1415f810b: lea rax, [rip + 0x1685216]
1415f8112: mov qword ptr [rsp + 0x20], rax
1415f8117: lea rcx, [rip + 0x16851ea]
1415f811e: lea r9, [rip + 0x1684823]
1415f8125: lea r8, [rbp + 0x220]
1415f812c: mov edx, 0x1b
1415f8131: call 0x1428d9760 ; core::result::unwrap_failed
1415f8136: ud2 
1415f8138: nop dword ptr [rax + rax]
1415f8140: mov qword ptr [rsp + 0x10], rdx
1415f8145: push rbp
1415f8146: push r15
1415f8148: push r14
1415f814a: push r13
1415f814c: push r12
1415f814e: push rsi
1415f814f: push rdi
1415f8150: push rbx
1415f8151: sub rsp, 0x58
1415f8155: lea rbp, [rdx + 0x80]
1415f815c: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f8163: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f8169: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f816f: mov rcx, qword ptr [rbp + 0x2a0]
1415f8176: call qword ptr [rip + 0x1b9a274]
1415f817c: movaps xmm6, xmmword ptr [rsp + 0x40]
1415f8181: movaps xmm7, xmmword ptr [rsp + 0x30]
1415f8186: movaps xmm8, xmmword ptr [rsp + 0x20]
1415f818c: add rsp, 0x58
1415f8190: pop rbx
1415f8191: pop rdi
1415f8192: pop rsi
1415f8193: pop r12
1415f8195: pop r13
1415f8197: pop r14
1415f8199: pop r15
1415f819b: pop rbp
1415f819c: ret 
1415f819d: nop dword ptr [rax]
1415f81a0: mov qword ptr [rsp + 0x10], rdx
1415f81a5: push rbp
1415f81a6: push r15
1415f81a8: push r14
1415f81aa: push r13
1415f81ac: push r12
1415f81ae: push rsi
1415f81af: push rdi
1415f81b0: push rbx
1415f81b1: sub rsp, 0x58
1415f81b5: lea rbp, [rdx + 0x80]
1415f81bc: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f81c3: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f81c9: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f81cf: mov rdx, qword ptr [rbp + 0x138]
1415f81d6: test rdx, rdx
1415f81d9: je 0x1415f81ed
1415f81db: mov rcx, qword ptr [rbp + 0x140]
1415f81e2: mov r8d, 1
1415f81e8: call 0x140613c20
1415f81ed: mov rax, qword ptr [rbp + 0x188]
1415f81f4: cmp qword ptr [rbp + 0x218], rax
1415f81fb: mov rsi, qword ptr [rbp + 0x190]
1415f8202: mov rdi, qword ptr [rbp + 0x178]
1415f8209: mov rbx, qword ptr [rbp + 0x180]
1415f8210: jne 0x1415f8251
1415f8212: test rsi, rsi
1415f8215: je 0x1415f8230
1415f8217: shl rsi, 5
1415f821b: mov r8d, 8
1415f8221: mov rcx, qword ptr [rbp + 0x198]
1415f8228: mov rdx, rsi
1415f822b: call 0x140613c20
1415f8230: movaps xmm6, xmmword ptr [rsp + 0x40]
1415f8235: movaps xmm7, xmmword ptr [rsp + 0x30]
1415f823a: movaps xmm8, xmmword ptr [rsp + 0x20]
1415f8240: add rsp, 0x58
1415f8244: pop rbx
1415f8245: pop rdi
1415f8246: pop rsi
1415f8247: pop r12
1415f8249: pop r13
1415f824b: pop r14
1415f824d: pop r15
1415f824f: pop rbp
1415f8250: ret 
1415f8251: shr rdi, 5
1415f8255: jmp 0x1415f8269 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f8257: nop word ptr [rax + rax]
1415f8260: add rbx, 0x20
1415f8264: dec rdi
1415f8267: je 0x1415f8212
1415f8269: mov rdx, qword ptr [rbx - 8]
1415f826d: test rdx, rdx
1415f8270: je 0x1415f8260
1415f8272: mov rcx, qword ptr [rbx]
1415f8275: mov r8d, 1
1415f827b: call 0x140613c20
1415f8280: jmp 0x1415f8260 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f8282: nop word ptr cs:[rax + rax]
1415f8290: mov qword ptr [rsp + 0x10], rdx
1415f8295: push rbp
1415f8296: push r15
1415f8298: push r14
1415f829a: push r13
1415f829c: push r12
1415f829e: push rsi
1415f829f: push rdi
1415f82a0: push rbx
1415f82a1: sub rsp, 0x58
1415f82a5: lea rbp, [rdx + 0x80]
1415f82ac: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f82b3: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f82b9: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f82bf: mov rdx, qword ptr [rbp + 0x220]
1415f82c6: test rdx, rdx
1415f82c9: je 0x1415f82dd
1415f82cb: mov rcx, qword ptr [rbp + 0x228]
1415f82d2: mov r8d, 1
1415f82d8: call 0x140613c20
1415f82dd: mov byte ptr [rbp + 0x2b7], 0
1415f82e4: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f82ea: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f82f0: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f82f7: add rsp, 0x58
1415f82fb: pop rbx
1415f82fc: pop rdi
1415f82fd: pop rsi
1415f82fe: pop r12
1415f8300: pop r13
1415f8302: pop r14
1415f8304: pop r15
1415f8306: pop rbp
1415f8307: ret 
1415f8308: nop dword ptr [rax + rax]
1415f8310: mov qword ptr [rsp + 0x10], rdx
1415f8315: push rbp
1415f8316: push r15
1415f8318: push r14
1415f831a: push r13
1415f831c: push r12
1415f831e: push rsi
1415f831f: push rdi
1415f8320: push rbx
1415f8321: sub rsp, 0x58
1415f8325: lea rbp, [rdx + 0x80]
1415f832c: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f8333: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f8339: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f833f: movzx ebx, byte ptr [rbp + 0x2b7]
1415f8346: lea rcx, [rbp + 0x1a0]
1415f834d: call 0x140888b30 ; core::ptr::drop_in_place<country_core::render::mesh::Mesh>
1415f8352: mov byte ptr [rbp + 0x2b6], bl
1415f8358: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f835e: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f8364: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f836b: add rsp, 0x58
1415f836f: pop rbx
1415f8370: pop rdi
1415f8371: pop rsi
1415f8372: pop r12
1415f8374: pop r13
1415f8376: pop r14
1415f8378: pop r15
1415f837a: pop rbp
1415f837b: ret 
1415f837c: nop dword ptr [rax]
1415f8380: mov qword ptr [rsp + 0x10], rdx
1415f8385: push rbp
1415f8386: push r15
1415f8388: push r14
1415f838a: push r13
1415f838c: push r12
1415f838e: push rsi
1415f838f: push rdi
1415f8390: push rbx
1415f8391: sub rsp, 0x58
1415f8395: lea rbp, [rdx + 0x80]
1415f839c: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f83a3: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f83a9: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f83af: mov rdx, qword ptr [rbp + 0x1a0]
1415f83b6: test rdx, rdx
1415f83b9: je 0x1415f83d1
1415f83bb: mov rcx, qword ptr [rbp + 0x1a8]
1415f83c2: shl rdx, 2
1415f83c6: mov r8d, 4
1415f83cc: call 0x140613c20
1415f83d1: mov rdx, qword ptr [rbp + 0x1b8]
1415f83d8: lea rax, [rdx*2]
1415f83e0: test rax, rax
1415f83e3: mov esi, dword ptr [rbp + 0x2a0]
1415f83e9: je 0x1415f83fd
1415f83eb: mov rcx, qword ptr [rbp + 0x1c0]
1415f83f2: mov r8d, 1
1415f83f8: call 0x140613c20
1415f83fd: mov byte ptr [rbp + 0x2b6], sil
1415f8404: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f840a: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f8410: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f8417: add rsp, 0x58
1415f841b: pop rbx
1415f841c: pop rdi
1415f841d: pop rsi
1415f841e: pop r12
1415f8420: pop r13
1415f8422: pop r14
1415f8424: pop r15
1415f8426: pop rbp
1415f8427: ret 
1415f8428: nop dword ptr [rax + rax]
1415f8430: mov qword ptr [rsp + 0x10], rdx
1415f8435: push rbp
1415f8436: push r15
1415f8438: push r14
1415f843a: push r13
1415f843c: push r12
1415f843e: push rsi
1415f843f: push rdi
1415f8440: push rbx
1415f8441: sub rsp, 0x58
1415f8445: lea rbp, [rdx + 0x80]
1415f844c: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f8453: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f8459: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f845f: test byte ptr [rbp + 0x2b6], 1
1415f8466: je 0x1415f84aa
1415f8468: mov rdx, qword ptr [rbp + 0x110]
1415f846f: test rdx, rdx
1415f8472: je 0x1415f8486
1415f8474: mov rcx, qword ptr [rbp + 0x118]
1415f847b: mov r8d, 1
1415f8481: call 0x140613c20
1415f8486: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f848c: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f8492: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f8499: add rsp, 0x58
1415f849d: pop rbx
1415f849e: pop rdi
1415f849f: pop rsi
1415f84a0: pop r12
1415f84a2: pop r13
1415f84a4: pop r14
1415f84a6: pop r15
1415f84a8: pop rbp
1415f84a9: ret 
1415f84aa: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f84b0: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f84b6: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f84bd: add rsp, 0x58
1415f84c1: pop rbx
1415f84c2: pop rdi
1415f84c3: pop rsi
1415f84c4: pop r12
1415f84c6: pop r13
1415f84c8: pop r14
1415f84ca: pop r15
1415f84cc: pop rbp
1415f84cd: ret 
1415f84ce: nop 
1415f84d0: mov qword ptr [rsp + 0x10], rdx
1415f84d5: push rbp
1415f84d6: push r15
1415f84d8: push r14
1415f84da: push r13
1415f84dc: push r12
1415f84de: push rsi
1415f84df: push rdi
1415f84e0: push rbx
1415f84e1: sub rsp, 0x58
1415f84e5: lea rbp, [rdx + 0x80]
1415f84ec: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f84f3: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f84f9: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f84ff: lea rcx, [rbp + 0xa0]
1415f8506: call 0x1408919b0 ; core::ptr::drop_in_place<country_core::startup::load_mesh_atlases::MeshAttribConfig>
1415f850b: cmp byte ptr [rbp + 0x2b4], 9
1415f8512: jne 0x1415f8569
1415f8514: mov rsi, qword ptr [rbp + 0xd8]
1415f851b: mov rdi, qword ptr [rbp + 0xe0]
1415f8522: test rdi, rdi
1415f8525: jne 0x1415f858d
1415f8527: mov rdx, qword ptr [rbp + 0xd0]
1415f852e: test rdx, rdx
1415f8531: je 0x1415f8545
1415f8533: shl rdx, 5
1415f8537: mov r8d, 8
1415f853d: mov rcx, rsi
1415f8540: call 0x140613c20
1415f8545: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f854b: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f8551: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f8558: add rsp, 0x58
1415f855c: pop rbx
1415f855d: pop rdi
1415f855e: pop rsi
1415f855f: pop r12
1415f8561: pop r13
1415f8563: pop r14
1415f8565: pop r15
1415f8567: pop rbp
1415f8568: ret 
1415f8569: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f856f: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f8575: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f857c: add rsp, 0x58
1415f8580: pop rbx
1415f8581: pop rdi
1415f8582: pop rsi
1415f8583: pop r12
1415f8585: pop r13
1415f8587: pop r14
1415f8589: pop r15
1415f858b: pop rbp
1415f858c: ret 
1415f858d: lea rbx, [rsi + 8]
1415f8591: jmp 0x1415f85ad ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f8593: nop word ptr cs:[rax + rax]
1415f85a0: add rbx, 0x20
1415f85a4: dec rdi
1415f85a7: je 0x1415f8527
1415f85ad: mov rdx, qword ptr [rbx - 8]
1415f85b1: test rdx, rdx
1415f85b4: je 0x1415f85a0
1415f85b6: mov rcx, qword ptr [rbx]
1415f85b9: mov r8d, 1
1415f85bf: call 0x140613c20
1415f85c4: jmp 0x1415f85a0 ; country_core::startup::load_mesh_atlases::load_nani_meshes<country_core::startup::load_mesh_atlases::load_mesh_atlases::closure_env$0>
1415f85c6: nop word ptr cs:[rax + rax]
1415f85d0: mov qword ptr [rsp + 0x10], rdx
1415f85d5: push rbp
1415f85d6: push r15
1415f85d8: push r14
1415f85da: push r13
1415f85dc: push r12
1415f85de: push rsi
1415f85df: push rdi
1415f85e0: push rbx
1415f85e1: sub rsp, 0x58
1415f85e5: lea rbp, [rdx + 0x80]
1415f85ec: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f85f3: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f85f9: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f85ff: lea rcx, [rbp + 0x220]
1415f8606: call 0x1405e7a00 ; core::ptr::drop_in_place<std::io::error::Error>
1415f860b: movaps xmm6, xmmword ptr [rsp + 0x40]
1415f8610: movaps xmm7, xmmword ptr [rsp + 0x30]
1415f8615: movaps xmm8, xmmword ptr [rsp + 0x20]
1415f861b: add rsp, 0x58
1415f861f: pop rbx
1415f8620: pop rdi
1415f8621: pop rsi
1415f8622: pop r12
1415f8624: pop r13
1415f8626: pop r14
1415f8628: pop r15
1415f862a: pop rbp
1415f862b: ret 
1415f862c: nop dword ptr [rax]
1415f8630: mov qword ptr [rsp + 0x10], rdx
1415f8635: push rbp
1415f8636: push r15
1415f8638: push r14
1415f863a: push r13
1415f863c: push r12
1415f863e: push rsi
1415f863f: push rdi
1415f8640: push rbx
1415f8641: sub rsp, 0x58
1415f8645: lea rbp, [rdx + 0x80]
1415f864c: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f8653: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f8659: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f865f: lea rcx, [rbp + 0x220]
1415f8666: call 0x140887a50 ; core::ptr::drop_in_place<ron::error::SpannedError>
1415f866b: movaps xmm6, xmmword ptr [rsp + 0x40]
1415f8670: movaps xmm7, xmmword ptr [rsp + 0x30]
1415f8675: movaps xmm8, xmmword ptr [rsp + 0x20]
1415f867b: add rsp, 0x58
1415f867f: pop rbx
1415f8680: pop rdi
1415f8681: pop rsi
1415f8682: pop r12
1415f8684: pop r13
1415f8686: pop r14
1415f8688: pop r15
1415f868a: pop rbp
1415f868b: ret 
1415f868c: nop dword ptr [rax]
1415f8690: mov qword ptr [rsp + 0x10], rdx
1415f8695: push rbp
1415f8696: push r15
1415f8698: push r14
1415f869a: push r13
1415f869c: push r12
1415f869e: push rsi
1415f869f: push rdi
1415f86a0: push rbx
1415f86a1: sub rsp, 0x58
1415f86a5: lea rbp, [rdx + 0x80]
1415f86ac: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f86b3: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f86b9: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f86bf: lea rcx, [rbp + 0x40]
1415f86c3: call 0x14087bd60 ; core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::load_mesh_atlases::NaniWorldSubsetConfig,alloc::alloc::Global> >
1415f86c8: movaps xmm6, xmmword ptr [rsp + 0x40]
1415f86cd: movaps xmm7, xmmword ptr [rsp + 0x30]
1415f86d2: movaps xmm8, xmmword ptr [rsp + 0x20]
1415f86d8: add rsp, 0x58
1415f86dc: pop rbx
1415f86dd: pop rdi
1415f86de: pop rsi
1415f86df: pop r12
1415f86e1: pop r13
1415f86e3: pop r14
1415f86e5: pop r15
1415f86e7: pop rbp
1415f86e8: ret 
1415f86e9: nop dword ptr [rax]
1415f86f0: mov qword ptr [rsp + 0x10], rdx
1415f86f5: push rbp
1415f86f6: push r15
1415f86f8: push r14
1415f86fa: push r13
1415f86fc: push r12
1415f86fe: push rsi
1415f86ff: push rdi
1415f8700: push rbx
1415f8701: sub rsp, 0x58
1415f8705: lea rbp, [rdx + 0x80]
1415f870c: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f8713: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f8719: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f871f: mov byte ptr [rbp + 0x2b6], 1
1415f8726: lea rcx, [rbp + 0xf0]
1415f872d: call 0x140d4cc30 ; anyhow::error::impl$6::drop
1415f8732: mov byte ptr [rbp + 0x2b6], 1
1415f8739: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f873f: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f8745: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f874c: add rsp, 0x58
1415f8750: pop rbx
1415f8751: pop rdi
1415f8752: pop rsi
1415f8753: pop r12
1415f8755: pop r13
1415f8757: pop r14
1415f8759: pop r15
1415f875b: pop rbp
1415f875c: ret 
1415f875d: nop dword ptr [rax]
1415f8760: mov qword ptr [rsp + 0x10], rdx
1415f8765: push rbp
1415f8766: push r15
1415f8768: push r14
1415f876a: push r13
1415f876c: push r12
1415f876e: push rsi
1415f876f: push rdi
1415f8770: push rbx
1415f8771: sub rsp, 0x58
1415f8775: lea rbp, [rdx + 0x80]
1415f877c: movdqa xmmword ptr [rsp + 0x20], xmm8
1415f8783: movdqa xmmword ptr [rsp + 0x30], xmm7
1415f8789: movdqa xmmword ptr [rsp + 0x40], xmm6
1415f878f: lea rcx, [rbp + 0xf0]
1415f8796: call 0x1408b9aa0 ; core::ptr::drop_in_place<enum2$<hotwatch::Error> >
1415f879b: movdqa xmm6, xmmword ptr [rsp + 0x40]
1415f87a1: movdqa xmm7, xmmword ptr [rsp + 0x30]
1415f87a7: movdqa xmm8, xmmword ptr [rsp + 0x20]
1415f87ae: add rsp, 0x58
1415f87b2: pop rbx
1415f87b3: pop rdi
1415f87b4: pop rsi
1415f87b5: pop r12
1415f87b7: pop r13
1415f87b9: pop r14
1415f87bb: pop r15
1415f87bd: pop rbp
1415f87be: ret 
