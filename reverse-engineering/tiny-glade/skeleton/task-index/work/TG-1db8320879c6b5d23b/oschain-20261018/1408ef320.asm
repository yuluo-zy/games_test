00000001408ef320: 55                               push rbp
00000001408ef321: 4157                             push r15
00000001408ef323: 4156                             push r14
00000001408ef325: 4155                             push r13
00000001408ef327: 4154                             push r12
00000001408ef329: 56                               push rsi
00000001408ef32a: 57                               push rdi
00000001408ef32b: 53                               push rbx
00000001408ef32c: 4881ecd8000000                   sub rsp, 0xd8
00000001408ef333: 488dac2480000000                 lea rbp, [rsp + 0x80]
00000001408ef33b: 0f297540                         movaps xmmword ptr [rbp + 0x40], xmm6
00000001408ef33f: 48c74538feffffff                 mov qword ptr [rbp + 0x38], 0xfffffffffffffffe
00000001408ef347: 4c89cb                           mov rbx, r9
00000001408ef34a: 4d89c7                           mov r15, r8
00000001408ef34d: 4889d7                           mov rdi, rdx
00000001408ef350: 4989ce                           mov r14, rcx
00000001408ef353: e8a8763200                       call 0x140c16a00 ; puffin::are_scopes_on
00000001408ef358: 89c6                             mov esi, eax
00000001408ef35a: 84c0                             test al, al
00000001408ef35c: 48895d30                         mov qword ptr [rbp + 0x30], rbx
00000001408ef360: 7460                             je 0x1408ef3c2
00000001408ef362: 4c8d2557771b02                   lea r12, [rip + 0x21b7757]
00000001408ef369: 41bd45000000                     mov r13d, 0x45
00000001408ef36f: ba45000000                       mov edx, 0x45
00000001408ef374: 4c89e1                           mov rcx, r12
00000001408ef377: e8a4e0ffff                       call 0x1408ed420 ; core::str::impl$0::rfind<ref$<str$> >
00000001408ef37c: 4883f801                         cmp rax, 1
00000001408ef380: 0f85b2000000                     jne 0x1408ef438
00000001408ef386: 4989d1                           mov r9, rdx
00000001408ef389: 4885d2                           test rdx, rdx
00000001408ef38c: 743d                             je 0x1408ef3cb
00000001408ef38e: 4983f945                         cmp r9, 0x45
00000001408ef392: 7335                             jae 0x1408ef3c9
00000001408ef394: 488d0525771b02                   lea rax, [rip + 0x21b7725]
00000001408ef39b: 41803c01bf                       cmp byte ptr [r9 + rax], 0xbf
00000001408ef3a0: 7f29                             jg 0x1408ef3cb
00000001408ef3a2: 488d053f781b02                   lea rax, [rip + 0x21b783f]
00000001408ef3a9: 4889442420                       mov qword ptr [rsp + 0x20], rax
00000001408ef3ae: 488d0d0b771b02                   lea rcx, [rip + 0x21b770b]
00000001408ef3b5: ba45000000                       mov edx, 0x45
00000001408ef3ba: 4531c0                           xor r8d, r8d
00000001408ef3bd: e87ea5fe01                       call 0x1428d9940 ; core::str::slice_error_fail
00000001408ef3c2: 31c0                             xor eax, eax
00000001408ef3c4: e9f3010000                       jmp 0x1408ef5bc
00000001408ef3c9: 75d7                             jne 0x1408ef3a2
00000001408ef3cb: 4c8d25ee761b02                   lea r12, [rip + 0x21b76ee]
00000001408ef3d2: 4c89e1                           mov rcx, r12
00000001408ef3d5: 4c89ca                           mov rdx, r9
00000001408ef3d8: e843e0ffff                       call 0x1408ed420 ; core::str::impl$0::rfind<ref$<str$> >
00000001408ef3dd: 4883f801                         cmp rax, 1
00000001408ef3e1: 7555                             jne 0x1408ef438
00000001408ef3e3: 4989d0                           mov r8, rdx
00000001408ef3e6: 4983c002                         add r8, 2
00000001408ef3ea: 7439                             je 0x1408ef425
00000001408ef3ec: 4983f845                         cmp r8, 0x45
00000001408ef3f0: 7331                             jae 0x1408ef423
00000001408ef3f2: 488d05c7761b02                   lea rax, [rip + 0x21b76c7]
00000001408ef3f9: 41803c00bf                       cmp byte ptr [r8 + rax], 0xbf
00000001408ef3fe: 7f25                             jg 0x1408ef425
00000001408ef400: 488d05f9771b02                   lea rax, [rip + 0x21b77f9]
00000001408ef407: 4889442420                       mov qword ptr [rsp + 0x20], rax
00000001408ef40c: 488d0dad761b02                   lea rcx, [rip + 0x21b76ad]
00000001408ef413: ba45000000                       mov edx, 0x45
00000001408ef418: 41b945000000                     mov r9d, 0x45
00000001408ef41e: e81da5fe01                       call 0x1428d9940 ; core::str::slice_error_fail
00000001408ef423: 75db                             jne 0x1408ef400
00000001408ef425: 41bd43000000                     mov r13d, 0x43
00000001408ef42b: 4929d5                           sub r13, rdx
00000001408ef42e: 4c8d258b761b02                   lea r12, [rip + 0x21b768b]
00000001408ef435: 4d01c4                           add r12, r8
00000001408ef438: 4c896df8                         mov qword ptr [rbp - 8], r13
00000001408ef43c: 488d054f801b02                   lea rax, [rip + 0x21b804f]
00000001408ef443: 4c8d2d0e801b02                   lea r13, [rip + 0x21b800e]
00000001408ef44a: 660f1f440000                     nop word ptr [rax + rax]
00000001408ef450: 0fbe48ff                         movsx ecx, byte ptr [rax - 1]
00000001408ef454: 85c9                             test ecx, ecx
00000001408ef456: 7818                             js 0x1408ef470
00000001408ef458: 48ffc8                           dec rax
00000001408ef45b: 83f95c                           cmp ecx, 0x5c
00000001408ef45e: 7569                             jne 0x1408ef4c9
00000001408ef460: e985000000                       jmp 0x1408ef4ea
00000001408ef465: 66662e0f1f840000000000           nop word ptr cs:[rax + rax]
00000001408ef470: 0fb650fe                         movzx edx, byte ptr [rax - 2]
00000001408ef474: 80fac0                           cmp dl, 0xc0
00000001408ef477: 7d25                             jge 0x1408ef49e
00000001408ef479: 440fb640fd                       movzx r8d, byte ptr [rax - 3]
00000001408ef47e: 4180f8c0                         cmp r8b, 0xc0
00000001408ef482: 7d23                             jge 0x1408ef4a7
00000001408ef484: 440fb648fc                       movzx r9d, byte ptr [rax - 4]
00000001408ef489: 4883c0fc                         add rax, -4
00000001408ef48d: 4183e107                         and r9d, 7
00000001408ef491: 41c1e106                         shl r9d, 6
00000001408ef495: 4183e03f                         and r8d, 0x3f
00000001408ef499: 4509c8                           or r8d, r9d
00000001408ef49c: eb11                             jmp 0x1408ef4af
00000001408ef49e: 4883c0fe                         add rax, -2
00000001408ef4a2: 83e21f                           and edx, 0x1f
00000001408ef4a5: eb12                             jmp 0x1408ef4b9
00000001408ef4a7: 4883c0fd                         add rax, -3
00000001408ef4ab: 4183e00f                         and r8d, 0xf
00000001408ef4af: 41c1e006                         shl r8d, 6
00000001408ef4b3: 83e23f                           and edx, 0x3f
00000001408ef4b6: 4409c2                           or edx, r8d
00000001408ef4b9: c1e206                           shl edx, 6
00000001408ef4bc: 80e13f                           and cl, 0x3f
00000001408ef4bf: 0fb6c9                           movzx ecx, cl
00000001408ef4c2: 09d1                             or ecx, edx
00000001408ef4c4: 83f95c                           cmp ecx, 0x5c
00000001408ef4c7: 7421                             je 0x1408ef4ea
00000001408ef4c9: 83f92f                           cmp ecx, 0x2f
00000001408ef4cc: 741c                             je 0x1408ef4ea
00000001408ef4ce: 4c39e8                           cmp rax, r13
00000001408ef4d1: 0f8579ffffff                     jne 0x1408ef450
00000001408ef4d7: 4889fb                           mov rbx, rdi
00000001408ef4da: 4c89ff                           mov rdi, r15
00000001408ef4dd: 4d89f7                           mov r15, r14
00000001408ef4e0: 4189f6                           mov r14d, esi
00000001408ef4e3: be3a000000                       mov esi, 0x3a
00000001408ef4e8: eb5b                             jmp 0x1408ef545
00000001408ef4ea: 4c8d2d677f1b02                   lea r13, [rip + 0x21b7f67]
00000001408ef4f1: 4c29e8                           sub rax, r13
00000001408ef4f4: 4989c0                           mov r8, rax
00000001408ef4f7: 49ffc0                           inc r8
00000001408ef4fa: 7432                             je 0x1408ef52e
00000001408ef4fc: 4983f83a                         cmp r8, 0x3a
00000001408ef500: 732a                             jae 0x1408ef52c
00000001408ef502: 43803c28bf                       cmp byte ptr [r8 + r13], 0xbf
00000001408ef507: 7f25                             jg 0x1408ef52e
00000001408ef509: 488d05b8761b02                   lea rax, [rip + 0x21b76b8]
00000001408ef510: 4889442420                       mov qword ptr [rsp + 0x20], rax
00000001408ef515: 488d0d3c7f1b02                   lea rcx, [rip + 0x21b7f3c]
00000001408ef51c: ba3a000000                       mov edx, 0x3a
00000001408ef521: 41b93a000000                     mov r9d, 0x3a
00000001408ef527: e814a4fe01                       call 0x1428d9940 ; core::str::slice_error_fail
00000001408ef52c: 75db                             jne 0x1408ef509
00000001408ef52e: 4889fb                           mov rbx, rdi
00000001408ef531: 4c89ff                           mov rdi, r15
00000001408ef534: 4d89f7                           mov r15, r14
00000001408ef537: 4189f6                           mov r14d, esi
00000001408ef53a: be39000000                       mov esi, 0x39
00000001408ef53f: 4829c6                           sub rsi, rax
00000001408ef542: 4d01c5                           add r13, r8
00000001408ef545: e8e68b3200                       call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
00000001408ef54a: 4889c1                           mov rcx, rax
00000001408ef54d: 488b00                           mov rax, qword ptr [rax]
00000001408ef550: 4883f801                         cmp rax, 1
00000001408ef554: 0f858d030000                     jne 0x1408ef8e7
00000001408ef55a: 4883c108                         add rcx, 8
00000001408ef55e: 48833900                         cmp qword ptr [rcx], 0
00000001408ef562: 0f85d7030000                     jne 0x1408ef93f
00000001408ef568: 48c701ffffffff                   mov qword ptr [rcx], 0xffffffffffffffff
00000001408ef56f: 48894d10                         mov qword ptr [rbp + 0x10], rcx
00000001408ef573: 4883c108                         add rcx, 8
00000001408ef577: 4889742420                       mov qword ptr [rsp + 0x20], rsi
00000001408ef57c: 48c744243000000000               mov qword ptr [rsp + 0x30], 0
00000001408ef585: 48c744242801000000               mov qword ptr [rsp + 0x28], 1
00000001408ef58e: 4c89e2                           mov rdx, r12
00000001408ef591: 4c8b45f8                         mov r8, qword ptr [rbp - 8]
00000001408ef595: 4d89e9                           mov r9, r13
00000001408ef598: e8a3773200                       call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
00000001408ef59d: 4989c5                           mov r13, rax
00000001408ef5a0: 488b4510                         mov rax, qword ptr [rbp + 0x10]
00000001408ef5a4: 48ff00                           inc qword ptr [rax]
00000001408ef5a7: 4c896de0                         mov qword ptr [rbp - 0x20], r13
00000001408ef5ab: b801000000                       mov eax, 1
00000001408ef5b0: 4489f6                           mov esi, r14d
00000001408ef5b3: 4d89fe                           mov r14, r15
00000001408ef5b6: 4989ff                           mov r15, rdi
00000001408ef5b9: 4889df                           mov rdi, rbx
00000001408ef5bc: 488b8dc0000000                   mov rcx, qword ptr [rbp + 0xc0]
00000001408ef5c3: 488945d8                         mov qword ptr [rbp - 0x28], rax
00000001408ef5c7: 488b01                           mov rax, qword ptr [rcx]
00000001408ef5ca: 833800                           cmp dword ptr [rax], 0
00000001408ef5cd: 745e                             je 0x1408ef62d
00000001408ef5cf: 4084f6                           test sil, sil
00000001408ef5d2: 7441                             je 0x1408ef615
00000001408ef5d4: e8578b3200                       call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
00000001408ef5d9: 4889c1                           mov rcx, rax
00000001408ef5dc: 488b00                           mov rax, qword ptr [rax]
00000001408ef5df: 4883f801                         cmp rax, 1
00000001408ef5e3: 0f8518030000                     jne 0x1408ef901
00000001408ef5e9: 4883c108                         add rcx, 8
00000001408ef5ed: 48833900                         cmp qword ptr [rcx], 0
00000001408ef5f1: 0f8548030000                     jne 0x1408ef93f
00000001408ef5f7: 48c701ffffffff                   mov qword ptr [rcx], 0xffffffffffffffff
00000001408ef5fe: 48894d30                         mov qword ptr [rbp + 0x30], rcx
00000001408ef602: 4883c108                         add rcx, 8
00000001408ef606: 4c89ea                           mov rdx, r13
00000001408ef609: e8e27a3200                       call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
00000001408ef60e: 488b4530                         mov rax, qword ptr [rbp + 0x30]
00000001408ef612: 48ff00                           inc qword ptr [rax]
00000001408ef615: 0f287540                         movaps xmm6, xmmword ptr [rbp + 0x40]
00000001408ef619: 4881c4d8000000                   add rsp, 0xd8
00000001408ef620: 5b                               pop rbx
00000001408ef621: 5f                               pop rdi
00000001408ef622: 5e                               pop rsi
00000001408ef623: 415c                             pop r12
00000001408ef625: 415d                             pop r13
00000001408ef627: 415e                             pop r14
00000001408ef629: 415f                             pop r15
00000001408ef62b: 5d                               pop rbp
00000001408ef62c: c3                               ret 
00000001408ef62d: 488b9dd0000000                   mov rbx, qword ptr [rbp + 0xd0]
00000001408ef634: 4c8ba5c8000000                   mov r12, qword ptr [rbp + 0xc8]
00000001408ef63b: 498b0424                         mov rax, qword ptr [r12]
00000001408ef63f: 488b4810                         mov rcx, qword ptr [rax + 0x10]
00000001408ef643: 48894dd0                         mov qword ptr [rbp - 0x30], rcx
00000001408ef647: 0f1000                           movups xmm0, xmmword ptr [rax]
00000001408ef64a: 0f2945c0                         movaps xmmword ptr [rbp - 0x40], xmm0
00000001408ef64e: 488b4530                         mov rax, qword ptr [rbp + 0x30]
00000001408ef652: 488b08                           mov rcx, qword ptr [rax]
00000001408ef655: 833901                           cmp dword ptr [rcx], 1
00000001408ef658: 0f850e010000                     jne 0x1408ef76c
00000001408ef65e: 8b471c                           mov eax, dword ptr [rdi + 0x1c]
00000001408ef661: 488b5710                         mov rdx, qword ptr [rdi + 0x10]
00000001408ef665: 8902                             mov dword ptr [rdx], eax
00000001408ef667: 488b07                           mov rax, qword ptr [rdi]
00000001408ef66a: 498b1424                         mov rdx, qword ptr [r12]
00000001408ef66e: f30f104104                       movss xmm0, dword ptr [rcx + 4]
00000001408ef673: f20f1012                         movsd xmm2, qword ptr [rdx]
00000001408ef677: f20f105a0c                       movsd xmm3, qword ptr [rdx + 0xc]
00000001408ef67c: f30f16ca                         movshdup xmm1, xmm2
00000001408ef680: f30f5cc1                         subss xmm0, xmm1
00000001408ef684: f30f16cb                         movshdup xmm1, xmm3
00000001408ef688: f30f5ec1                         divss xmm0, xmm1
00000001408ef68c: f30f12c8                         movsldup xmm1, xmm0
00000001408ef690: 0f59cb                           mulps xmm1, xmm3
00000001408ef693: f30f594214                       mulss xmm0, dword ptr [rdx + 0x14]
00000001408ef698: 0f58ca                           addps xmm1, xmm2
00000001408ef69b: f30f584208                       addss xmm0, dword ptr [rdx + 8]
00000001408ef6a0: 833801                           cmp dword ptr [rax], 1
00000001408ef6a3: 754c                             jne 0x1408ef6f1
00000001408ef6a5: 488b0b                           mov rcx, qword ptr [rbx]
00000001408ef6a8: f30f105158                       movss xmm2, dword ptr [rcx + 0x58]
00000001408ef6ad: f20f105804                       movsd xmm3, qword ptr [rax + 4]
00000001408ef6b2: 0f28e1                           movaps xmm4, xmm1
00000001408ef6b5: 0f5ce3                           subps xmm4, xmm3
00000001408ef6b8: 0f28d8                           movaps xmm3, xmm0
00000001408ef6bb: f30f5c580c                       subss xmm3, dword ptr [rax + 0xc]
00000001408ef6c0: 0f136010                         movlps qword ptr [rax + 0x10], xmm4
00000001408ef6c4: f30f115818                       movss dword ptr [rax + 0x18], xmm3
00000001408ef6c9: 0f57ed                           xorps xmm5, xmm5
00000001408ef6cc: 0f2ed5                           ucomiss xmm2, xmm5
00000001408ef6cf: 7420                             je 0x1408ef6f1
00000001408ef6d1: 0f59e4                           mulps xmm4, xmm4
00000001408ef6d4: f30f16ec                         movshdup xmm5, xmm4
00000001408ef6d8: f30f58ec                         addss xmm5, xmm4
00000001408ef6dc: f30f59db                         mulss xmm3, xmm3
00000001408ef6e0: f30f58dd                         addss xmm3, xmm5
00000001408ef6e4: f30f51db                         sqrtss xmm3, xmm3
00000001408ef6e8: f30f5eda                         divss xmm3, xmm2
00000001408ef6ec: f30f11581c                       movss dword ptr [rax + 0x1c], xmm3
00000001408ef6f1: c70001000000                     mov dword ptr [rax], 1
00000001408ef6f7: 0f134804                         movlps qword ptr [rax + 4], xmm1
00000001408ef6fb: f30f11400c                       movss dword ptr [rax + 0xc], xmm0
00000001408ef700: 8b480c                           mov ecx, dword ptr [rax + 0xc]
00000001408ef703: 894d28                           mov dword ptr [rbp + 0x28], ecx
00000001408ef706: 488b4004                         mov rax, qword ptr [rax + 4]
00000001408ef70a: 48894520                         mov qword ptr [rbp + 0x20], rax
00000001408ef70e: c7451c01000000                   mov dword ptr [rbp + 0x1c], 1
00000001408ef715: 4c8d451c                         lea r8, [rbp + 0x1c]
00000001408ef719: 4c89f1                           mov rcx, r14
00000001408ef71c: 4c89e2                           mov rdx, r12
00000001408ef71f: e84c030000                       call 0x1408efa70 ; country_core::resources::mouse_terrain_raycast::mouse_terrain_raycast::closure$0
00000001408ef724: 4084f6                           test sil, sil
00000001408ef727: 0f84e8feffff                     je 0x1408ef615
00000001408ef72d: e8fe893200                       call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
00000001408ef732: 4889c1                           mov rcx, rax
00000001408ef735: 488b00                           mov rax, qword ptr [rax]
00000001408ef738: 4883f801                         cmp rax, 1
00000001408ef73c: 0f85d9010000                     jne 0x1408ef91b
00000001408ef742: 4883c108                         add rcx, 8
00000001408ef746: 48833900                         cmp qword ptr [rcx], 0
00000001408ef74a: 0f85ef010000                     jne 0x1408ef93f
00000001408ef750: 48c701ffffffff                   mov qword ptr [rcx], 0xffffffffffffffff
00000001408ef757: 48894d30                         mov qword ptr [rbp + 0x30], rcx
00000001408ef75b: 4883c108                         add rcx, 8
00000001408ef75f: 4c89ea                           mov rdx, r13
00000001408ef762: e889793200                       call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
00000001408ef767: e9a2feffff                       jmp 0x1408ef60e
00000001408ef76c: 40887530                         mov byte ptr [rbp + 0x30], sil
00000001408ef770: 498b17                           mov rdx, qword ptr [r15]
00000001408ef773: 488d4d1c                         lea rcx, [rbp + 0x1c]
00000001408ef777: 4c8d45c0                         lea r8, [rbp - 0x40]
00000001408ef77b: f30f101dfd610302                 movss xmm3, dword ptr [rip + 0x20361fd]
00000001408ef783: e898a20f00                       call 0x1409e9a20 ; country_core::resources::terrain_height::TerrainHeightsData::intersect_ray<country_core::resources::terrain_height::TerrainHeightSsbo>
00000001408ef788: f6451c01                         test byte ptr [rbp + 0x1c], 1
00000001408ef78c: 0f8421010000                     je 0x1408ef8b3
00000001408ef792: f30f10552c                       movss xmm2, dword ptr [rbp + 0x2c]
00000001408ef797: 498b0424                         mov rax, qword ptr [r12]
00000001408ef79b: f30f106008                       movss xmm4, dword ptr [rax + 8]
00000001408ef7a0: f20f105d24                       movsd xmm3, qword ptr [rbp + 0x24]
00000001408ef7a5: f20f1028                         movsd xmm5, qword ptr [rax]
00000001408ef7a9: f20f10700c                       movsd xmm6, qword ptr [rax + 0xc]
00000001408ef7ae: f30f16cd                         movshdup xmm1, xmm5
00000001408ef7b2: 0f57c0                           xorps xmm0, xmm0
00000001408ef7b5: f30f5cc1                         subss xmm0, xmm1
00000001408ef7b9: f30f16ce                         movshdup xmm1, xmm6
00000001408ef7bd: f30f5ec1                         divss xmm0, xmm1
00000001408ef7c1: f30f12c8                         movsldup xmm1, xmm0
00000001408ef7c5: 0f59ce                           mulps xmm1, xmm6
00000001408ef7c8: f30f594014                       mulss xmm0, dword ptr [rax + 0x14]
00000001408ef7cd: 0f58cd                           addps xmm1, xmm5
00000001408ef7d0: f30f58c4                         addss xmm0, xmm4
00000001408ef7d4: 0f28f5                           movaps xmm6, xmm5
00000001408ef7d7: 0f5cf1                           subps xmm6, xmm1
00000001408ef7da: 0f59f6                           mulps xmm6, xmm6
00000001408ef7dd: 0f5ceb                           subps xmm5, xmm3
00000001408ef7e0: 0f59ed                           mulps xmm5, xmm5
00000001408ef7e3: f20f7cf5                         haddps xmm6, xmm5
00000001408ef7e7: f30f12e4                         movsldup xmm4, xmm4
00000001408ef7eb: 0f28e8                           movaps xmm5, xmm0
00000001408ef7ee: 0f14ea                           unpcklps xmm5, xmm2
00000001408ef7f1: 0f5ce5                           subps xmm4, xmm5
00000001408ef7f4: 0fc6f6e8                         shufps xmm6, xmm6, 0xe8
00000001408ef7f8: 0f59e4                           mulps xmm4, xmm4
00000001408ef7fb: 0f58e6                           addps xmm4, xmm6
00000001408ef7fe: 0f51e4                           sqrtps xmm4, xmm4
00000001408ef801: f30f16ec                         movshdup xmm5, xmm4
00000001408ef805: 31c0                             xor eax, eax
00000001408ef807: 0f2eec                           ucomiss xmm5, xmm4
00000001408ef80a: 0f97c0                           seta al
00000001408ef80d: 660f6ee0                         movd xmm4, eax
00000001408ef811: 660f70e450                       pshufd xmm4, xmm4, 0x50
00000001408ef816: 660f72f41f                       pslld xmm4, 0x1f
00000001408ef81b: 660f72e41f                       psrad xmm4, 0x1f
00000001408ef820: 0f54cc                           andps xmm1, xmm4
00000001408ef823: 0f55e3                           andnps xmm4, xmm3
00000001408ef826: 7703                             ja 0x1408ef82b
00000001408ef828: 0f28c2                           movaps xmm0, xmm2
00000001408ef82b: 0f56cc                           orps xmm1, xmm4
00000001408ef82e: 8b471c                           mov eax, dword ptr [rdi + 0x1c]
00000001408ef831: 488b4f10                         mov rcx, qword ptr [rdi + 0x10]
00000001408ef835: 8901                             mov dword ptr [rcx], eax
00000001408ef837: 488b07                           mov rax, qword ptr [rdi]
00000001408ef83a: 833801                           cmp dword ptr [rax], 1
00000001408ef83d: 0fb67530                         movzx esi, byte ptr [rbp + 0x30]
00000001408ef841: 754c                             jne 0x1408ef88f
00000001408ef843: 488b0b                           mov rcx, qword ptr [rbx]
00000001408ef846: f30f105158                       movss xmm2, dword ptr [rcx + 0x58]
00000001408ef84b: f20f105804                       movsd xmm3, qword ptr [rax + 4]
00000001408ef850: 0f28e1                           movaps xmm4, xmm1
00000001408ef853: 0f5ce3                           subps xmm4, xmm3
00000001408ef856: 0f28d8                           movaps xmm3, xmm0
00000001408ef859: f30f5c580c                       subss xmm3, dword ptr [rax + 0xc]
00000001408ef85e: 0f136010                         movlps qword ptr [rax + 0x10], xmm4
00000001408ef862: f30f115818                       movss dword ptr [rax + 0x18], xmm3
00000001408ef867: 0f57ed                           xorps xmm5, xmm5
00000001408ef86a: 0f2ed5                           ucomiss xmm2, xmm5
00000001408ef86d: 7420                             je 0x1408ef88f
00000001408ef86f: 0f59e4                           mulps xmm4, xmm4
00000001408ef872: f30f16ec                         movshdup xmm5, xmm4
00000001408ef876: f30f58ec                         addss xmm5, xmm4
00000001408ef87a: f30f59db                         mulss xmm3, xmm3
00000001408ef87e: f30f58dd                         addss xmm3, xmm5
00000001408ef882: f30f51db                         sqrtss xmm3, xmm3
00000001408ef886: f30f5eda                         divss xmm3, xmm2
00000001408ef88a: f30f11581c                       movss dword ptr [rax + 0x1c], xmm3
00000001408ef88f: c70001000000                     mov dword ptr [rax], 1
00000001408ef895: 0f134804                         movlps qword ptr [rax + 4], xmm1
00000001408ef899: f30f11400c                       movss dword ptr [rax + 0xc], xmm0
00000001408ef89e: 8b480c                           mov ecx, dword ptr [rax + 0xc]
00000001408ef8a1: 894d08                           mov dword ptr [rbp + 8], ecx
00000001408ef8a4: 488b4004                         mov rax, qword ptr [rax + 4]
00000001408ef8a8: 48894500                         mov qword ptr [rbp], rax
00000001408ef8ac: b801000000                       mov eax, 1
00000001408ef8b1: eb06                             jmp 0x1408ef8b9
00000001408ef8b3: 31c0                             xor eax, eax
00000001408ef8b5: 0fb67530                         movzx esi, byte ptr [rbp + 0x30]
00000001408ef8b9: 89451c                           mov dword ptr [rbp + 0x1c], eax
00000001408ef8bc: 488b4500                         mov rax, qword ptr [rbp]
00000001408ef8c0: 48894520                         mov qword ptr [rbp + 0x20], rax
00000001408ef8c4: 8b4508                           mov eax, dword ptr [rbp + 8]
00000001408ef8c7: 894528                           mov dword ptr [rbp + 0x28], eax
00000001408ef8ca: 4c8d451c                         lea r8, [rbp + 0x1c]
00000001408ef8ce: 4c89f1                           mov rcx, r14
00000001408ef8d1: 4c89e2                           mov rdx, r12
00000001408ef8d4: e897010000                       call 0x1408efa70 ; country_core::resources::mouse_terrain_raycast::mouse_terrain_raycast::closure$0
00000001408ef8d9: 4084f6                           test sil, sil
00000001408ef8dc: 0f85f2fcffff                     jne 0x1408ef5d4
00000001408ef8e2: e92efdffff                       jmp 0x1408ef615
00000001408ef8e7: 4885c0                           test rax, rax
00000001408ef8ea: 7547                             jne 0x1408ef933
00000001408ef8ec: 31d2                             xor edx, edx
00000001408ef8ee: e8dd8e0002                       call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
00000001408ef8f3: 4889c1                           mov rcx, rax
00000001408ef8f6: 4885c0                           test rax, rax
00000001408ef8f9: 0f855ffcffff                     jne 0x1408ef55e
00000001408ef8ff: eb32                             jmp 0x1408ef933
00000001408ef901: 4885c0                           test rax, rax
00000001408ef904: 752d                             jne 0x1408ef933
00000001408ef906: 31d2                             xor edx, edx
00000001408ef908: e8c38e0002                       call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
00000001408ef90d: 4889c1                           mov rcx, rax
00000001408ef910: 4885c0                           test rax, rax
00000001408ef913: 0f85d4fcffff                     jne 0x1408ef5ed
00000001408ef919: eb18                             jmp 0x1408ef933
00000001408ef91b: 4885c0                           test rax, rax
00000001408ef91e: 7513                             jne 0x1408ef933
00000001408ef920: 31d2                             xor edx, edx
00000001408ef922: e8a98e0002                       call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
00000001408ef927: 4889c1                           mov rcx, rax
00000001408ef92a: 4885c0                           test rax, rax
00000001408ef92d: 0f8513feffff                     jne 0x1408ef746
00000001408ef933: 488d0d66b13402                   lea rcx, [rip + 0x234b166]
00000001408ef93a: e89187fe01                       call 0x1428d80d0 ; std::thread::local::panic_access_error
00000001408ef93f: 488d0d2ab23402                   lea rcx, [rip + 0x234b22a]
00000001408ef946: e80599fe01                       call 0x1428d9250 ; core::cell::panic_already_borrowed
00000001408ef94b: cc                               int3 
00000001408ef94c: 0f1f4000                         nop dword ptr [rax]
00000001408ef950: 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001408ef955: 55                               push rbp
00000001408ef956: 4157                             push r15
00000001408ef958: 4156                             push r14
00000001408ef95a: 4155                             push r13
00000001408ef95c: 4154                             push r12
00000001408ef95e: 56                               push rsi
00000001408ef95f: 57                               push rdi
00000001408ef960: 53                               push rbx
00000001408ef961: 4883ec48                         sub rsp, 0x48
00000001408ef965: 488daa80000000                   lea rbp, [rdx + 0x80]
00000001408ef96c: 0f29742430                       movaps xmmword ptr [rsp + 0x30], xmm6
00000001408ef971: 488b4510                         mov rax, qword ptr [rbp + 0x10]
00000001408ef975: 48ff00                           inc qword ptr [rax]
00000001408ef978: 0f28742430                       movaps xmm6, xmmword ptr [rsp + 0x30]
00000001408ef97d: 4883c448                         add rsp, 0x48
00000001408ef981: 5b                               pop rbx
00000001408ef982: 5f                               pop rdi
00000001408ef983: 5e                               pop rsi
00000001408ef984: 415c                             pop r12
00000001408ef986: 415d                             pop r13
00000001408ef988: 415e                             pop r14
00000001408ef98a: 415f                             pop r15
00000001408ef98c: 5d                               pop rbp
00000001408ef98d: c3                               ret 
00000001408ef98e: 6690                             nop 
00000001408ef990: 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001408ef995: 55                               push rbp
00000001408ef996: 4157                             push r15
00000001408ef998: 4156                             push r14
00000001408ef99a: 4155                             push r13
00000001408ef99c: 4154                             push r12
00000001408ef99e: 56                               push rsi
00000001408ef99f: 57                               push rdi
00000001408ef9a0: 53                               push rbx
00000001408ef9a1: 4883ec48                         sub rsp, 0x48
00000001408ef9a5: 488daa80000000                   lea rbp, [rdx + 0x80]
00000001408ef9ac: 0f29742430                       movaps xmmword ptr [rsp + 0x30], xmm6
00000001408ef9b1: 807d3000                         cmp byte ptr [rbp + 0x30], 0
00000001408ef9b5: 7420                             je 0x1408ef9d7
00000001408ef9b7: 488d45e0                         lea rax, [rbp - 0x20]
00000001408ef9bb: 488945f0                         mov qword ptr [rbp - 0x10], rax
00000001408ef9bf: 488d45f0                         lea rax, [rbp - 0x10]
00000001408ef9c3: 488945e8                         mov qword ptr [rbp - 0x18], rax
00000001408ef9c7: 488d0d9a711b02                   lea rcx, [rip + 0x21b719a]
00000001408ef9ce: 488d55e8                         lea rdx, [rbp - 0x18]
00000001408ef9d2: e8f9f8a300                       call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
00000001408ef9d7: 0f28742430                       movaps xmm6, xmmword ptr [rsp + 0x30]
00000001408ef9dc: 4883c448                         add rsp, 0x48
00000001408ef9e0: 5b                               pop rbx
00000001408ef9e1: 5f                               pop rdi
00000001408ef9e2: 5e                               pop rsi
00000001408ef9e3: 415c                             pop r12
00000001408ef9e5: 415d                             pop r13
00000001408ef9e7: 415e                             pop r14
00000001408ef9e9: 415f                             pop r15
00000001408ef9eb: 5d                               pop rbp
00000001408ef9ec: c3                               ret 
00000001408ef9ed: 0f1f00                           nop dword ptr [rax]
00000001408ef9f0: 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001408ef9f5: 55                               push rbp
00000001408ef9f6: 4157                             push r15
00000001408ef9f8: 4156                             push r14
00000001408ef9fa: 4155                             push r13
00000001408ef9fc: 4154                             push r12
00000001408ef9fe: 56                               push rsi
00000001408ef9ff: 57                               push rdi
00000001408efa00: 53                               push rbx
00000001408efa01: 4883ec48                         sub rsp, 0x48
00000001408efa05: 488daa80000000                   lea rbp, [rdx + 0x80]
00000001408efa0c: 0f29742430                       movaps xmmword ptr [rsp + 0x30], xmm6
00000001408efa11: 488b4530                         mov rax, qword ptr [rbp + 0x30]
00000001408efa15: 48ff00                           inc qword ptr [rax]
00000001408efa18: 0f28742430                       movaps xmm6, xmmword ptr [rsp + 0x30]
00000001408efa1d: 4883c448                         add rsp, 0x48
00000001408efa21: 5b                               pop rbx
00000001408efa22: 5f                               pop rdi
00000001408efa23: 5e                               pop rsi
00000001408efa24: 415c                             pop r12
00000001408efa26: 415d                             pop r13
00000001408efa28: 415e                             pop r14
00000001408efa2a: 415f                             pop r15
00000001408efa2c: 5d                               pop rbp
00000001408efa2d: c3                               ret 
00000001408efa2e: 6690                             nop 
00000001408efa30: 4889542410                       mov qword ptr [rsp + 0x10], rdx
00000001408efa35: 55                               push rbp
00000001408efa36: 4157                             push r15
00000001408efa38: 4156                             push r14
00000001408efa3a: 4155                             push r13
00000001408efa3c: 4154                             push r12
00000001408efa3e: 56                               push rsi
00000001408efa3f: 57                               push rdi
00000001408efa40: 53                               push rbx
00000001408efa41: 4883ec48                         sub rsp, 0x48
00000001408efa45: 488daa80000000                   lea rbp, [rdx + 0x80]
00000001408efa4c: 0f29742430                       movaps xmmword ptr [rsp + 0x30], xmm6
00000001408efa51: 488b4530                         mov rax, qword ptr [rbp + 0x30]
00000001408efa55: 48ff00                           inc qword ptr [rax]
00000001408efa58: 0f28742430                       movaps xmm6, xmmword ptr [rsp + 0x30]
00000001408efa5d: 4883c448                         add rsp, 0x48
00000001408efa61: 5b                               pop rbx
00000001408efa62: 5f                               pop rdi
00000001408efa63: 5e                               pop rsi
00000001408efa64: 415c                             pop r12
00000001408efa66: 415d                             pop r13
00000001408efa68: 415e                             pop r14
00000001408efa6a: 415f                             pop r15
00000001408efa6c: 5d                               pop rbp
00000001408efa6d: c3                               ret 
