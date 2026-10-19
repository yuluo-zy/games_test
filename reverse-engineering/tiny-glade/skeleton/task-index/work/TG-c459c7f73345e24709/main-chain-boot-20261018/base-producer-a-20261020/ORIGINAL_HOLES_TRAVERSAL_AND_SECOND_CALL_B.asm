0000000140757d34 488b9518040000                   mov rdx, qword ptr [rbp + 0x418]
0000000140757d3b 8b421c                           mov eax, dword ptr [rdx + 0x1c]
0000000140757d3e 488b4a10                         mov rcx, qword ptr [rdx + 0x10]
0000000140757d42 898594030000                     mov dword ptr [rbp + 0x394], eax
0000000140757d48 48898d10020000                   mov qword ptr [rbp + 0x210], rcx
0000000140757d4f 8901                             mov dword ptr [rcx], eax
0000000140757d51 488b02                           mov rax, qword ptr [rdx]
0000000140757d54 488b38                           mov rdi, qword ptr [rax]
0000000140757d57 48898510040000                   mov qword ptr [rbp + 0x410], rax
0000000140757d5e 488b5818                         mov rbx, qword ptr [rax + 0x18]
0000000140757d62 660f6f07                         movdqa xmm0, xmmword ptr [rdi]
0000000140757d66 660fd7c8                         pmovmskb ecx, xmm0
0000000140757d6a f7d1                             not ecx
0000000140757d6c 4c8d7f10                         lea r15, [rdi + 0x10]
0000000140757d70 4c8d35e14e3202                   lea r14, [rip + 0x2324ee1]
0000000140757d77 eb1e                             jmp 0x140757d97
0000000140757d79 0f1f8000000000                   nop dword ptr [rax]
0000000140757d80 488b85b8030000                   mov rax, qword ptr [rbp + 0x3b8]
0000000140757d87 4e892ce0                         mov qword ptr [rax + r12*8], r13
0000000140757d8b 49ffc4                           inc r12
0000000140757d8e 4c89a5c0030000                   mov qword ptr [rbp + 0x3c0], r12
0000000140757d95 89f1                             mov ecx, esi
0000000140757d97 4885db                           test rbx, rbx
0000000140757d9a 0f84d0000000                     je 0x140757e70
0000000140757da0 6685c9                           test cx, cx
0000000140757da3 740b                             je 0x140757db0
0000000140757da5 8d71ff                           lea esi, [rcx - 1]
0000000140757da8 21ce                             and esi, ecx
0000000140757daa eb2d                             jmp 0x140757dd9
0000000140757dac 0f1f4000                         nop dword ptr [rax]
0000000140757db0 66410f6f07                       movdqa xmm0, xmmword ptr [r15]
0000000140757db5 660fd7c8                         pmovmskb ecx, xmm0
0000000140757db9 4881c780f9ffff                   add rdi, -0x680
0000000140757dc0 4983c710                         add r15, 0x10
0000000140757dc4 81f9ffff0000                     cmp ecx, 0xffff
0000000140757dca 74e4                             je 0x140757db0
0000000140757dcc b8feffffff                       mov eax, 0xfffffffe
0000000140757dd1 29c8                             sub eax, ecx
0000000140757dd3 f7d1                             not ecx
0000000140757dd5 89ce                             mov esi, ecx
0000000140757dd7 21c6                             and esi, eax
0000000140757dd9 f30fbcc1                         tzcnt eax, ecx
0000000140757ddd 48f7d8                           neg rax
0000000140757de0 4c6be068                         imul r12, rax, 0x68
0000000140757de4 4e8b6c2798                       mov r13, qword ptr [rdi + r12 - 0x68]
0000000140757de9 c6853304000001                   mov byte ptr [rbp + 0x433], 1
0000000140757df0 488b8df8030000                   mov rcx, qword ptr [rbp + 0x3f8]
0000000140757df7 4c89ea                           mov rdx, r13
0000000140757dfa 4d89f0                           mov r8, r14
0000000140757dfd e8eea03300                       call 0x140a91ef0 ; country_core::resources::walls::public_walls::PublicWalls::get
0000000140757e02 48ffcb                           dec rbx
0000000140757e05 89f1                             mov ecx, esi
0000000140757e07 4885c0                           test rax, rax
0000000140757e0a 748b                             je 0x140757d97
0000000140757e0c 4a8d0c27                         lea rcx, [rdi + r12]
0000000140757e10 4883c1a0                         add rcx, -0x60
0000000140757e14 660f6e888c020000                 movd xmm1, dword ptr [rax + 0x28c]
0000000140757e1c 83b85802000002                   cmp dword ptr [rax + 0x258], 2
0000000140757e23 410f95c0                         setne r8b
0000000140757e27 c6853304000001                   mov byte ptr [rbp + 0x433], 1
0000000140757e2e e8cdf48200                       call 0x140f87300 ; country_core::resources::walls::wall_holes::WallHoleStorage::finalize
0000000140757e33 89f1                             mov ecx, esi
0000000140757e35 84c0                             test al, al
0000000140757e37 0f845affffff                     je 0x140757d97
0000000140757e3d 4c8ba5c0030000                   mov r12, qword ptr [rbp + 0x3c0]
0000000140757e44 4c3ba5b0030000                   cmp r12, qword ptr [rbp + 0x3b0]
0000000140757e4b 0f852fffffff                     jne 0x140757d80
0000000140757e51 c6853304000001                   mov byte ptr [rbp + 0x433], 1
0000000140757e58 488d8db0030000                   lea rcx, [rbp + 0x3b0]
0000000140757e5f 488d150a4e3202                   lea rdx, [rip + 0x2324e0a]
0000000140757e66 e805aae0ff                       call 0x140562870 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::glade_border::ShowDemoBorderOverlay>,alloc::alloc::Global>
0000000140757e6b e910ffffff                       jmp 0x140757d80
0000000140758bc7 488d8df0020000                   lea rcx, [rbp + 0x2f0]
0000000140758bce e87de38200                       call 0x140f86f50 ; country_core::resources::walls::wall_holes::WallHoleStorage::clone_and_reopen
0000000140758bd3 0f108540030000                   movups xmm0, xmmword ptr [rbp + 0x340]
0000000140758bda 0f298580020000                   movaps xmmword ptr [rbp + 0x280], xmm0
0000000140758be1 0f108530030000                   movups xmm0, xmmword ptr [rbp + 0x330]
0000000140758be8 0f298570020000                   movaps xmmword ptr [rbp + 0x270], xmm0
0000000140758bef f30f6f85f0020000                 movdqu xmm0, xmmword ptr [rbp + 0x2f0]
0000000140758bf7 f30f6f8d00030000                 movdqu xmm1, xmmword ptr [rbp + 0x300]
0000000140758bff f30f6f9510030000                 movdqu xmm2, xmmword ptr [rbp + 0x310]
0000000140758c07 f30f6f9d20030000                 movdqu xmm3, xmmword ptr [rbp + 0x320]
0000000140758c0f 660f7f9d60020000                 movdqa xmmword ptr [rbp + 0x260], xmm3
0000000140758c17 660f7f9550020000                 movdqa xmmword ptr [rbp + 0x250], xmm2
0000000140758c1f 660f7f8d40020000                 movdqa xmmword ptr [rbp + 0x240], xmm1
0000000140758c27 660f7f8530020000                 movdqa xmmword ptr [rbp + 0x230], xmm0
0000000140758c2f c6853404000001                   mov byte ptr [rbp + 0x434], 1
0000000140758c36 488d8d30020000                   lea rcx, [rbp + 0x230]
0000000140758c3d e8bee28200                       call 0x140f86f00 ; country_core::resources::walls::wall_holes::WallHoleStorage::as_wip_mut
0000000140758c42 f30f108598030000                 movss xmm0, dword ptr [rbp + 0x398]
0000000140758c4a f30f1185f0020000                 movss dword ptr [rbp + 0x2f0], xmm0
0000000140758c52 66440f7e9df4020000               movd dword ptr [rbp + 0x2f4], xmm11
0000000140758c5b 66440f7ea5f8020000               movd dword ptr [rbp + 0x2f8], xmm12
0000000140758c64 66440f7eadfc020000               movd dword ptr [rbp + 0x2fc], xmm13
0000000140758c6d 660f6e859c030000                 movd xmm0, dword ptr [rbp + 0x39c]
0000000140758c75 660f7e8500030000                 movd dword ptr [rbp + 0x300], xmm0
0000000140758c7d 4488a504030000                   mov byte ptr [rbp + 0x304], r12b
0000000140758c84 488b9528040000                   mov rdx, qword ptr [rbp + 0x428]
0000000140758c8b c6853404000001                   mov byte ptr [rbp + 0x434], 1
0000000140758c92 488b8dd0020000                   mov rcx, qword ptr [rbp + 0x2d0]
0000000140758c99 48894c2450                       mov qword ptr [rsp + 0x50], rcx
0000000140758c9e 488d8df0020000                   lea rcx, [rbp + 0x2f0]
0000000140758ca5 48894c2448                       mov qword ptr [rsp + 0x48], rcx
0000000140758caa 488b8df0030000                   mov rcx, qword ptr [rbp + 0x3f0]
0000000140758cb1 884c2440                         mov byte ptr [rsp + 0x40], cl
0000000140758cb5 4889442438                       mov qword ptr [rsp + 0x38], rax
0000000140758cba 488d85b0020000                   lea rax, [rbp + 0x2b0]
0000000140758cc1 4889442430                       mov qword ptr [rsp + 0x30], rax
0000000140758cc6 488b85a0030000                   mov rax, qword ptr [rbp + 0x3a0]
0000000140758ccd 4889442428                       mov qword ptr [rsp + 0x28], rax
0000000140758cd2 488b85e0030000                   mov rax, qword ptr [rbp + 0x3e0]
0000000140758cd9 4889442420                       mov qword ptr [rsp + 0x20], rax
0000000140758cde 488b8d20040000                   mov rcx, qword ptr [rbp + 0x420]
0000000140758ce5 4c8b8500040000                   mov r8, qword ptr [rbp + 0x400]
0000000140758cec 4c8b8dd8030000                   mov r9, qword ptr [rbp + 0x3d8]
0000000140758cf3 e848c8ab00                       call 0x141215540 ; system_wall_constructor::utils::wall_corners::add_wall_corners
0000000140758cf8 488b8518040000                   mov rax, qword ptr [rbp + 0x418]
0000000140758cff 660f6e48ec                       movd xmm1, dword ptr [rax - 0x14]
0000000140758d04 8378b802                         cmp dword ptr [rax - 0x48], 2
0000000140758d08 410f95c0                         setne r8b
0000000140758d0c c6853404000001                   mov byte ptr [rbp + 0x434], 1
0000000140758d13 488d8d30020000                   lea rcx, [rbp + 0x230]
0000000140758d1a e8e1e58200                       call 0x140f87300 ; country_core::resources::walls::wall_holes::WallHoleStorage::finalize
0000000140758d1f c6853404000001                   mov byte ptr [rbp + 0x434], 1
