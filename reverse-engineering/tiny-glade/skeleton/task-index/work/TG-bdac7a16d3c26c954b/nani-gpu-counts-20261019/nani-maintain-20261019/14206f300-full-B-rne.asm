14206f300: 55 push rbp
14206f301: 4157 push r15
14206f303: 4156 push r14
14206f305: 4155 push r13
14206f307: 4154 push r12
14206f309: 56 push rsi
14206f30a: 57 push rdi
14206f30b: 53 push rbx
14206f30c: 4881ec28020000 sub rsp, 0x228
14206f313: 488dac2480000000 lea rbp, [rsp + 0x80]
14206f31b: 440f298590010000 movaps xmmword ptr [rbp + 0x190], xmm8
14206f323: 0f29bd80010000 movaps xmmword ptr [rbp + 0x180], xmm7
14206f32a: 0f29b570010000 movaps xmmword ptr [rbp + 0x170], xmm6
14206f331: 48c78568010000feffffff mov qword ptr [rbp + 0x168], 0xfffffffffffffffe
14206f33c: 4c89cb mov rbx, r9
14206f33f: 4c89c6 mov rsi, r8
14206f342: 488955f8 mov qword ptr [rbp - 8], rdx
14206f346: 4989cd mov r13, rcx
14206f349: e8b276bafe call 0x140c16a00 ; puffin::are_scopes_on
14206f34e: 84c0 test al, al
14206f350: 7460 je 0x14206f3b2
14206f352: 4c8d3d6e3bf600 lea r15, [rip + 0xf63b6e]
14206f359: 41be3c000000 mov r14d, 0x3c
14206f35f: ba3c000000 mov edx, 0x3c
14206f364: 4c89f9 mov rcx, r15
14206f367: e804fbffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
14206f36c: 4883f801 cmp rax, 1
14206f370: 0f85b2000000 jne 0x14206f428
14206f376: 4989d1 mov r9, rdx
14206f379: 4885d2 test rdx, rdx
14206f37c: 743d je 0x14206f3bb
14206f37e: 4983f93c cmp r9, 0x3c
14206f382: 7335 jae 0x14206f3b9
14206f384: 488d053c3bf600 lea rax, [rip + 0xf63b3c]
14206f38b: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
14206f390: 7f29 jg 0x14206f3bb
14206f392: 488d053f40f600 lea rax, [rip + 0xf6403f]
14206f399: 4889442420 mov qword ptr [rsp + 0x20], rax
14206f39e: 488d0d223bf600 lea rcx, [rip + 0xf63b22]
14206f3a5: ba3c000000 mov edx, 0x3c
14206f3aa: 4531c0 xor r8d, r8d
14206f3ad: e88ea58600 call 0x1428d9940 ; core::str::slice_error_fail
14206f3b2: 31c9 xor ecx, ecx
14206f3b4: e9c5010000 jmp 0x14206f57e ; unknown
14206f3b9: 75d7 jne 0x14206f392
14206f3bb: 4c8d3d053bf600 lea r15, [rip + 0xf63b05]
14206f3c2: 4c89f9 mov rcx, r15
14206f3c5: 4c89ca mov rdx, r9
14206f3c8: e8a3faffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
14206f3cd: 4883f801 cmp rax, 1
14206f3d1: 7555 jne 0x14206f428
14206f3d3: 4989d0 mov r8, rdx
14206f3d6: 4983c002 add r8, 2
14206f3da: 7439 je 0x14206f415
14206f3dc: 4983f83c cmp r8, 0x3c
14206f3e0: 7331 jae 0x14206f413
14206f3e2: 488d05de3af600 lea rax, [rip + 0xf63ade]
14206f3e9: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
14206f3ee: 7f25 jg 0x14206f415
14206f3f0: 488d05f93ff600 lea rax, [rip + 0xf63ff9]
14206f3f7: 4889442420 mov qword ptr [rsp + 0x20], rax
14206f3fc: 488d0dc43af600 lea rcx, [rip + 0xf63ac4]
14206f403: ba3c000000 mov edx, 0x3c
14206f408: 41b93c000000 mov r9d, 0x3c
14206f40e: e82da58600 call 0x1428d9940 ; core::str::slice_error_fail
14206f413: 75db jne 0x14206f3f0
14206f415: 41be3a000000 mov r14d, 0x3a
14206f41b: 4929d6 sub r14, rdx
14206f41e: 4c8d3da23af600 lea r15, [rip + 0xf63aa2]
14206f425: 4d01c7 add r15, r8
14206f428: 4c8d052f40f600 lea r8, [rip + 0xf6402f]
14206f42f: 4c8d25fa3ff600 lea r12, [rip + 0xf63ffa]
14206f436: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
14206f440: 410fbe40ff movsx eax, byte ptr [r8 - 1]
14206f445: 85c0 test eax, eax
14206f447: 7817 js 0x14206f460
14206f449: 49ffc8 dec r8
14206f44c: 83f85c cmp eax, 0x5c
14206f44f: 7563 jne 0x14206f4b4
14206f451: eb72 jmp 0x14206f4c5 ; unknown
14206f453: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14206f460: 410fb648fe movzx ecx, byte ptr [r8 - 2]
14206f465: 80f9c0 cmp cl, 0xc0
14206f468: 7d23 jge 0x14206f48d
14206f46a: 410fb650fd movzx edx, byte ptr [r8 - 3]
14206f46f: 80fac0 cmp dl, 0xc0
14206f472: 7d22 jge 0x14206f496
14206f474: 450fb648fc movzx r9d, byte ptr [r8 - 4]
14206f479: 4983c0fc add r8, -4
14206f47d: 4183e107 and r9d, 7
14206f481: 41c1e106 shl r9d, 6
14206f485: 83e23f and edx, 0x3f
14206f488: 4409ca or edx, r9d
14206f48b: eb10 jmp 0x14206f49d ; unknown
14206f48d: 4983c0fe add r8, -2
14206f491: 83e11f and ecx, 0x1f
14206f494: eb0f jmp 0x14206f4a5 ; unknown
14206f496: 4983c0fd add r8, -3
14206f49a: 83e20f and edx, 0xf
14206f49d: c1e206 shl edx, 6
14206f4a0: 83e13f and ecx, 0x3f
14206f4a3: 09d1 or ecx, edx
14206f4a5: c1e106 shl ecx, 6
14206f4a8: 243f and al, 0x3f
14206f4aa: 0fb6c0 movzx eax, al
14206f4ad: 09c8 or eax, ecx
14206f4af: 83f85c cmp eax, 0x5c
14206f4b2: 7411 je 0x14206f4c5
14206f4b4: 83f82f cmp eax, 0x2f
14206f4b7: 740c je 0x14206f4c5
14206f4b9: 4d39e0 cmp r8, r12
14206f4bc: 7582 jne 0x14206f440
14206f4be: bf2e000000 mov edi, 0x2e
14206f4c3: eb4c jmp 0x14206f511 ; unknown
14206f4c5: 4c8d25643ff600 lea r12, [rip + 0xf63f64]
14206f4cc: 4d29e0 sub r8, r12
14206f4cf: 49ffc0 inc r8
14206f4d2: 7432 je 0x14206f506
14206f4d4: 4983f82e cmp r8, 0x2e
14206f4d8: 732a jae 0x14206f504
14206f4da: 43803c20bf cmp byte ptr [r8 + r12], 0xbf
14206f4df: 7f25 jg 0x14206f506
14206f4e1: 488d05d03ef600 lea rax, [rip + 0xf63ed0]
14206f4e8: 4889442420 mov qword ptr [rsp + 0x20], rax
14206f4ed: 488d0d3c3ff600 lea rcx, [rip + 0xf63f3c]
14206f4f4: ba2e000000 mov edx, 0x2e
14206f4f9: 41b92e000000 mov r9d, 0x2e
14206f4ff: e83ca48600 call 0x1428d9940 ; core::str::slice_error_fail
14206f504: 75db jne 0x14206f4e1
14206f506: bf2e000000 mov edi, 0x2e
14206f50b: 4c29c7 sub rdi, r8
14206f50e: 4d01c4 add r12, r8
14206f511: e81a8cbafe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14206f516: 4889c1 mov rcx, rax
14206f519: 488b00 mov rax, qword ptr [rax]
14206f51c: 4883f801 cmp rax, 1
14206f520: 0f8566100000 jne 0x14207058c
14206f526: 4883c108 add rcx, 8
14206f52a: 48833900 cmp qword ptr [rcx], 0
14206f52e: 0f857c100000 jne 0x1420705b0
14206f534: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14206f53b: 48898d60010000 mov qword ptr [rbp + 0x160], rcx
14206f542: 4883c108 add rcx, 8
14206f546: 48897c2420 mov qword ptr [rsp + 0x20], rdi
14206f54b: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
14206f554: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
14206f55d: 4c89fa mov rdx, r15
14206f560: 4d89f0 mov r8, r14
14206f563: 4d89e1 mov r9, r12
14206f566: e8d577bafe call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
14206f56b: 488b8d60010000 mov rcx, qword ptr [rbp + 0x160]
14206f572: 48ff01 inc qword ptr [rcx]
14206f575: 48894508 mov qword ptr [rbp + 8], rax
14206f579: b901000000 mov ecx, 1
14206f57e: 488b8510020000 mov rax, qword ptr [rbp + 0x210]
14206f585: 48894d00 mov qword ptr [rbp], rcx
14206f589: 488b16 mov rdx, qword ptr [rsi]
14206f58c: 488b4e10 mov rcx, qword ptr [rsi + 0x10]
14206f590: 488b12 mov rdx, qword ptr [rdx]
14206f593: 48395140 cmp qword ptr [rcx + 0x40], rdx
14206f597: 48899d50010000 mov qword ptr [rbp + 0x150], rbx
14206f59e: 4c89ad00010000 mov qword ptr [rbp + 0x100], r13
14206f5a5: 0f86a7000000 jbe 0x14206f652
14206f5ab: 4c8b4138 mov r8, qword ptr [rcx + 0x38]
14206f5af: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
14206f5b3: 4885d2 test rdx, rdx
14206f5b6: 0f8496000000 je 0x14206f652
14206f5bc: 488b4908 mov rcx, qword ptr [rcx + 8]
14206f5c0: 48f7d2 not rdx
14206f5c3: 488d14d2 lea rdx, [rdx + rdx*8]
14206f5c7: 4c8b6608 mov r12, qword ptr [rsi + 8]
14206f5cb: 4d8b0c24 mov r9, qword ptr [r12]
14206f5cf: 4531c0 xor r8d, r8d
14206f5d2: 4d89ca mov r10, r9
14206f5d5: 4c2b54d118 sub r10, qword ptr [rcx + rdx*8 + 0x18]
14206f5da: 4d0f42d0 cmovb r10, r8
14206f5de: 4c2b4cd138 sub r9, qword ptr [rcx + rdx*8 + 0x38]
14206f5e3: 4d0f42c8 cmovb r9, r8
14206f5e7: 4d89d6 mov r14, r10
14206f5ea: 49c1e604 shl r14, 4
14206f5ee: 4c0374d108 add r14, qword ptr [rcx + rdx*8 + 8]
14206f5f3: 4c8b5cd110 mov r11, qword ptr [rcx + rdx*8 + 0x10]
14206f5f8: 488b74d130 mov rsi, qword ptr [rcx + rdx*8 + 0x30]
14206f5fd: 4d29d3 sub r11, r10
14206f600: 4d0f42d8 cmovb r11, r8
14206f604: 41ba08000000 mov r10d, 8
14206f60a: 4d0f42f2 cmovb r14, r10
14206f60e: 4c89cf mov rdi, r9
14206f611: 48c1e704 shl rdi, 4
14206f615: 48037cd128 add rdi, qword ptr [rcx + rdx*8 + 0x28]
14206f61a: 4c29ce sub rsi, r9
14206f61d: 490f42f0 cmovb rsi, r8
14206f621: 490f42fa cmovb rdi, r10
14206f625: 488b4cd140 mov rcx, qword ptr [rcx + rdx*8 + 0x40]
14206f62a: 4829f1 sub rcx, rsi
14206f62d: 4c29d9 sub rcx, r11
14206f630: 49890c24 mov qword ptr [r12], rcx
14206f634: 49c1e304 shl r11, 4
14206f638: 4d01f3 add r11, r14
14206f63b: 4c899d60010000 mov qword ptr [rbp + 0x160], r11
14206f642: 48c1e604 shl rsi, 4
14206f646: 4801fe add rsi, rdi
14206f649: 4889b558010000 mov qword ptr [rbp + 0x158], rsi
14206f650: eb03 jmp 0x14206f655 ; unknown
14206f652: 4531e4 xor r12d, r12d
14206f655: 8b481c mov ecx, dword ptr [rax + 0x1c]
14206f658: 898d34010000 mov dword ptr [rbp + 0x134], ecx
14206f65e: 488b08 mov rcx, qword ptr [rax]
14206f661: 48898de0000000 mov qword ptr [rbp + 0xe0], rcx
14206f668: 488b4010 mov rax, qword ptr [rax + 0x10]
14206f66c: 488985e8000000 mov qword ptr [rbp + 0xe8], rax
14206f673: 488b8518020000 mov rax, qword ptr [rbp + 0x218]
14206f67a: 4883c020 add rax, 0x20
14206f67e: 48898548010000 mov qword ptr [rbp + 0x148], rax
14206f685: 488d5d71 lea rbx, [rbp + 0x71]
14206f689: 31f6 xor esi, esi
14206f68b: 0f1f440000 nop dword ptr [rax + rax]
14206f690: 4c89f8 mov rax, r15
14206f693: 4c89e9 mov rcx, r13
14206f696: 4885f6 test rsi, rsi
14206f699: 751d jne 0x14206f6b8
14206f69b: 0f1f440000 nop dword ptr [rax + rax]
14206f6a0: 4c89e6 mov rsi, r12
14206f6a3: 41bc00000000 mov r12d, 0
14206f6a9: 4889f8 mov rax, rdi
14206f6ac: 4c89f1 mov rcx, r14
14206f6af: 4885f6 test rsi, rsi
14206f6b2: 0f84e6000000 je 0x14206f79e
14206f6b8: 4885c9 test rcx, rcx
14206f6bb: 7423 je 0x14206f6e0
14206f6bd: 4c8d6910 lea r13, [rcx + 0x10]
14206f6c1: 483b8d60010000 cmp rcx, qword ptr [rbp + 0x160]
14206f6c8: 7416 je 0x14206f6e0
14206f6ca: 4989c7 mov r15, rax
14206f6cd: 4889c8 mov rax, rcx
14206f6d0: eb23 jmp 0x14206f6f5 ; unknown
14206f6d2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14206f6e0: 4885c0 test rax, rax
14206f6e3: 74bb je 0x14206f6a0
14206f6e5: 483b8558010000 cmp rax, qword ptr [rbp + 0x158]
14206f6ec: 74b2 je 0x14206f6a0
14206f6ee: 4c8d7810 lea r15, [rax + 0x10]
14206f6f2: 4531ed xor r13d, r13d
14206f6f5: 48ff06 inc qword ptr [rsi]
14206f6f8: 488b4008 mov rax, qword ptr [rax + 8]
14206f6fc: 4889c1 mov rcx, rax
14206f6ff: 48c1e920 shr rcx, 0x20
14206f703: 749b je 0x14206f6a0
14206f705: 48898510010000 mov qword ptr [rbp + 0x110], rax
14206f70c: 488b8d48010000 mov rcx, qword ptr [rbp + 0x148]
14206f713: 488d9510010000 lea rdx, [rbp + 0x110]
14206f71a: e8511654fe call 0x1405b0d70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >
14206f71f: 488d4d60 lea rcx, [rbp + 0x60]
14206f723: 488b9518020000 mov rdx, qword ptr [rbp + 0x218]
14206f72a: 4989c0 mov r8, rax
14206f72d: 4c8d8d10010000 lea r9, [rbp + 0x110]
14206f734: e8376db0fe call 0x140b76470 ; hashbrown::raw::RawTable::remove_entry<tuple$<bevy_ecs::entity::Entity,country_core::render::nani::InstanceHandle>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$
14206f739: 0fb64570 movzx eax, byte ptr [rbp + 0x70]
14206f73d: 3c1b cmp al, 0x1b
14206f73f: 0f84490f0000 je 0x14207068e
14206f745: 488b4d68 mov rcx, qword ptr [rbp + 0x68]
14206f749: 0fb713 movzx edx, word ptr [rbx]
14206f74c: 6689953c010000 mov word ptr [rbp + 0x13c], dx
14206f753: 0fb65302 movzx edx, byte ptr [rbx + 2]
14206f757: 88953e010000 mov byte ptr [rbp + 0x13e], dl
14206f75d: 8b9534010000 mov edx, dword ptr [rbp + 0x134]
14206f763: 4c8b85e8000000 mov r8, qword ptr [rbp + 0xe8]
14206f76a: 418910 mov dword ptr [r8], edx
14206f76d: 48894d60 mov qword ptr [rbp + 0x60], rcx
14206f771: 884568 mov byte ptr [rbp + 0x68], al
14206f774: 0fb6853e010000 movzx eax, byte ptr [rbp + 0x13e]
14206f77b: 8843fa mov byte ptr [rbx - 6], al
14206f77e: 0fb7853c010000 movzx eax, word ptr [rbp + 0x13c]
14206f785: 668943f8 mov word ptr [rbx - 8], ax
14206f789: 488b8de0000000 mov rcx, qword ptr [rbp + 0xe0]
14206f790: 488d5560 lea rdx, [rbp + 0x60]
14206f794: e8c72792fe call 0x140991f60 ; country_core::render::nani::gpu_world::GpuWorld::remove_instance
14206f799: e9f2feffff jmp 0x14206f690 ; unknown
14206f79e: 488b9500010000 mov rdx, qword ptr [rbp + 0x100]
14206f7a5: 488b02 mov rax, qword ptr [rdx]
14206f7a8: 4c8ba0f8000000 mov r12, qword ptr [rax + 0xf8]
14206f7af: 48898508010000 mov qword ptr [rbp + 0x108], rax
14206f7b6: 488b8000010000 mov rax, qword ptr [rax + 0x100]
14206f7bd: 4c8b8520020000 mov r8, qword ptr [rbp + 0x220]
14206f7c4: 498b4808 mov rcx, qword ptr [r8 + 8]
14206f7c8: 48898df0000000 mov qword ptr [rbp + 0xf0], rcx
14206f7cf: 4885c9 test rcx, rcx
14206f7d2: 750b jne 0x14206f7df
14206f7d4: 498b4810 mov rcx, qword ptr [r8 + 0x10]
14206f7d8: 48898df0000000 mov qword ptr [rbp + 0xf0], rcx
14206f7df: 488b5208 mov rdx, qword ptr [rdx + 8]
14206f7e3: 498d3484 lea rsi, [r12 + rax*4]
14206f7e7: 488b8508010000 mov rax, qword ptr [rbp + 0x108]
14206f7ee: 0fb68008010000 movzx eax, byte ptr [rax + 0x108]
14206f7f5: 888500010000 mov byte ptr [rbp + 0x100], al
14206f7fb: 488b8550010000 mov rax, qword ptr [rbp + 0x150]
14206f802: 488b00 mov rax, qword ptr [rax]
14206f805: 48894528 mov qword ptr [rbp + 0x28], rax
14206f809: b808000000 mov eax, 8
14206f80e: 48894518 mov qword ptr [rbp + 0x18], rax
14206f812: 0f283567f18b00 movaps xmm6, xmmword ptr [rip + 0x8bf167]
14206f819: 48894520 mov qword ptr [rbp + 0x20], rax
14206f81d: 48c7855001000000000000 mov qword ptr [rbp + 0x150], 0
14206f828: 48c7854001000000000000 mov qword ptr [rbp + 0x140], 0
14206f833: 4531ff xor r15d, r15d
14206f836: 48c7856001000000000000 mov qword ptr [rbp + 0x160], 0
14206f841: 48895510 mov qword ptr [rbp + 0x10], rdx
14206f845: 80bd0001000000 cmp byte ptr [rbp + 0x100], 0
14206f84c: 0f84ae000000 je 0x14206f900
14206f852: 488b8d60010000 mov rcx, qword ptr [rbp + 0x160]
14206f859: 4c39f9 cmp rcx, r15
14206f85c: 0f85f1010000 jne 0x14206fa53
14206f862: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14206f870: 4939f4 cmp r12, rsi
14206f873: 0f8457080000 je 0x1420700d0
14206f879: 418b0c24 mov ecx, dword ptr [r12]
14206f87d: 4983c404 add r12, 4
14206f881: 488b82a8010000 mov rax, qword ptr [rdx + 0x1a8]
14206f888: 488d0cc9 lea rcx, [rcx + rcx*8]
14206f88c: 4c8b7cc810 mov r15, qword ptr [rax + rcx*8 + 0x10]
14206f891: 4d85ff test r15, r15
14206f894: 74da je 0x14206f870
14206f896: 488d04c8 lea rax, [rax + rcx*8]
14206f89a: 488b8d08010000 mov rcx, qword ptr [rbp + 0x108]
14206f8a1: 488b9110010000 mov rdx, qword ptr [rcx + 0x110]
14206f8a8: 488b4840 mov rcx, qword ptr [rax + 0x40]
14206f8ac: 4839d1 cmp rcx, rdx
14206f8af: 0f86f9000000 jbe 0x14206f9ae
14206f8b5: 4c8b4038 mov r8, qword ptr [rax + 0x38]
14206f8b9: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
14206f8bd: 4885d2 test rdx, rdx
14206f8c0: 0f84e8000000 je 0x14206f9ae
14206f8c6: 4c8b4018 mov r8, qword ptr [rax + 0x18]
14206f8ca: 48f7d2 not rdx
14206f8cd: 488d1452 lea rdx, [rdx + rdx*2]
14206f8d1: 48c1e204 shl rdx, 4
14206f8d5: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
14206f8da: 48899550010000 mov qword ptr [rbp + 0x150], rdx
14206f8e1: 488b9508010000 mov rdx, qword ptr [rbp + 0x108]
14206f8e8: 488b9218010000 mov rdx, qword ptr [rdx + 0x118]
14206f8ef: 4839d1 cmp rcx, rdx
14206f8f2: 0f87d4000000 ja 0x14206f9cc
14206f8f8: e941010000 jmp 0x14206fa3e ; unknown
14206f8fd: 0f1f00 nop dword ptr [rax]
14206f900: 488b8d60010000 mov rcx, qword ptr [rbp + 0x160]
14206f907: 4c39f9 cmp rcx, r15
14206f90a: 0f856b010000 jne 0x14206fa7b
14206f910: 4939f4 cmp r12, rsi
14206f913: 0f84b7070000 je 0x1420700d0
14206f919: 418b0c24 mov ecx, dword ptr [r12]
14206f91d: 4983c404 add r12, 4
14206f921: 488b8200010000 mov rax, qword ptr [rdx + 0x100]
14206f928: 488d0c89 lea rcx, [rcx + rcx*4]
14206f92c: 48c1e105 shl rcx, 5
14206f930: 4c8b7c0858 mov r15, qword ptr [rax + rcx + 0x58]
14206f935: 4d85ff test r15, r15
14206f938: 74d6 je 0x14206f910
14206f93a: 4801c8 add rax, rcx
14206f93d: 8b8894000000 mov ecx, dword ptr [rax + 0x94]
14206f943: 488b92a8010000 mov rdx, qword ptr [rdx + 0x1a8]
14206f94a: 4c8d0cc9 lea r9, [rcx + rcx*8]
14206f94e: 4a8d0cca lea rcx, [rdx + r9*8]
14206f952: 4c8b8508010000 mov r8, qword ptr [rbp + 0x108]
14206f959: 4d8b8010010000 mov r8, qword ptr [r8 + 0x110]
14206f960: 4a8b54ca40 mov rdx, qword ptr [rdx + r9*8 + 0x40]
14206f965: 4c39c2 cmp rdx, r8
14206f968: 0f8688000000 jbe 0x14206f9f6
14206f96e: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
14206f972: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
14206f976: 4d85c0 test r8, r8
14206f979: 747b je 0x14206f9f6
14206f97b: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
14206f97f: 49f7d0 not r8
14206f982: 4f8d0440 lea r8, [r8 + r8*2]
14206f986: 49c1e004 shl r8, 4
14206f98a: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
14206f98f: 4c898550010000 mov qword ptr [rbp + 0x150], r8
14206f996: 4c8b8508010000 mov r8, qword ptr [rbp + 0x108]
14206f99d: 4d8b8018010000 mov r8, qword ptr [r8 + 0x118]
14206f9a4: 4c39c2 cmp rdx, r8
14206f9a7: 776b ja 0x14206fa14
14206f9a9: e9b8000000 jmp 0x14206fa66 ; unknown
14206f9ae: 48c7855001000000000000 mov qword ptr [rbp + 0x150], 0
14206f9b9: 488b9508010000 mov rdx, qword ptr [rbp + 0x108]
14206f9c0: 488b9218010000 mov rdx, qword ptr [rdx + 0x118]
14206f9c7: 4839d1 cmp rcx, rdx
14206f9ca: 7672 jbe 0x14206fa3e
14206f9cc: 488b4838 mov rcx, qword ptr [rax + 0x38]
14206f9d0: 488b0cd1 mov rcx, qword ptr [rcx + rdx*8]
14206f9d4: 4885c9 test rcx, rcx
14206f9d7: 7465 je 0x14206fa3e
14206f9d9: 488b5018 mov rdx, qword ptr [rax + 0x18]
14206f9dd: 48f7d1 not rcx
14206f9e0: 488d0c49 lea rcx, [rcx + rcx*2]
14206f9e4: 48c1e104 shl rcx, 4
14206f9e8: 488b4c0a10 mov rcx, qword ptr [rdx + rcx + 0x10]
14206f9ed: 48898d40010000 mov qword ptr [rbp + 0x140], rcx
14206f9f4: eb53 jmp 0x14206fa49 ; unknown
14206f9f6: 48c7855001000000000000 mov qword ptr [rbp + 0x150], 0
14206fa01: 4c8b8508010000 mov r8, qword ptr [rbp + 0x108]
14206fa08: 4d8b8018010000 mov r8, qword ptr [r8 + 0x118]
14206fa0f: 4c39c2 cmp rdx, r8
14206fa12: 7652 jbe 0x14206fa66
14206fa14: 488b5138 mov rdx, qword ptr [rcx + 0x38]
14206fa18: 4a8b14c2 mov rdx, qword ptr [rdx + r8*8]
14206fa1c: 4885d2 test rdx, rdx
14206fa1f: 7445 je 0x14206fa66
14206fa21: 488b4918 mov rcx, qword ptr [rcx + 0x18]
14206fa25: 48f7d2 not rdx
14206fa28: 488d1452 lea rdx, [rdx + rdx*2]
14206fa2c: 48c1e204 shl rdx, 4
14206fa30: 488b4c1110 mov rcx, qword ptr [rcx + rdx + 0x10]
14206fa35: 48898d40010000 mov qword ptr [rbp + 0x140], rcx
14206fa3c: eb33 jmp 0x14206fa71 ; unknown
14206fa3e: 48c7854001000000000000 mov qword ptr [rbp + 0x140], 0
14206fa49: 488b4008 mov rax, qword ptr [rax + 8]
14206fa4d: 48894518 mov qword ptr [rbp + 0x18], rax
14206fa51: 31c9 xor ecx, ecx
14206fa53: 488b4518 mov rax, qword ptr [rbp + 0x18]
14206fa57: 488d04c8 lea rax, [rax + rcx*8]
14206fa5b: 48898d60010000 mov qword ptr [rbp + 0x160], rcx
14206fa62: 89c9 mov ecx, ecx
14206fa64: eb2c jmp 0x14206fa92 ; unknown
14206fa66: 48c7854001000000000000 mov qword ptr [rbp + 0x140], 0
14206fa71: 488b4050 mov rax, qword ptr [rax + 0x50]
14206fa75: 48894520 mov qword ptr [rbp + 0x20], rax
14206fa79: 31c9 xor ecx, ecx
14206fa7b: 48898d60010000 mov qword ptr [rbp + 0x160], rcx
14206fa82: 48c1e104 shl rcx, 4
14206fa86: 488b5520 mov rdx, qword ptr [rbp + 0x20]
14206fa8a: 488d040a lea rax, [rdx + rcx]
14206fa8e: 8b4c0a08 mov ecx, dword ptr [rdx + rcx + 8]
14206fa92: 4c8d3449 lea r14, [rcx + rcx*2]
14206fa96: 488b8d50010000 mov rcx, qword ptr [rbp + 0x150]
14206fa9d: 4a8d0cf1 lea rcx, [rcx + r14*8]
14206faa1: 49c1e604 shl r14, 4
14206faa5: 4c03b540010000 add r14, qword ptr [rbp + 0x140]
14206faac: 488b00 mov rax, qword ptr [rax]
14206faaf: 48898558010000 mov qword ptr [rbp + 0x158], rax
14206fab6: 48894d58 mov qword ptr [rbp + 0x58], rcx
14206faba: f60101 test byte ptr [rcx], 1
14206fabd: 7411 je 0x14206fad0
14206fabf: 4883c104 add rcx, 4
14206fac3: eb55 jmp 0x14206fb1a ; unknown
14206fac5: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
14206fad0: 488b5108 mov rdx, qword ptr [rcx + 8]
14206fad4: 488955f0 mov qword ptr [rbp - 0x10], rdx
14206fad8: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
14206fadc: e86f869cfe call 0x140a38150 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name<utils::name_hash::NameHash>
14206fae1: 4c8b4558 mov r8, qword ptr [rbp + 0x58]
14206fae5: 450fb64810 movzx r9d, byte ptr [r8 + 0x10]
14206faea: 4983c010 add r8, 0x10
14206faee: 4883c0f8 add rax, -8
14206faf2: 48c1e203 shl rdx, 3
14206faf6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
14206fb00: 4885d2 test rdx, rdx
14206fb03: 0f84140b0000 je 0x14207061d
14206fb09: 488d4808 lea rcx, [rax + 8]
14206fb0d: 4883c2f8 add rdx, -8
14206fb11: 4438480c cmp byte ptr [rax + 0xc], r9b
14206fb15: 4889c8 mov rax, rcx
14206fb18: 75e6 jne 0x14206fb00
14206fb1a: 8b19 mov ebx, dword ptr [rcx]
14206fb1c: c645541b mov byte ptr [rbp + 0x54], 0x1b
14206fb20: 8b8534010000 mov eax, dword ptr [rbp + 0x134]
14206fb26: 488b8de8000000 mov rcx, qword ptr [rbp + 0xe8]
14206fb2d: 8901 mov dword ptr [rcx], eax
14206fb2f: 488b4558 mov rax, qword ptr [rbp + 0x58]
14206fb33: 408a7810 mov dil, byte ptr [rax + 0x10]
14206fb37: f2410f107e10 movsd xmm7, qword ptr [r14 + 0x10]
14206fb3d: f3450f104618 movss xmm8, dword ptr [r14 + 0x18]
14206fb43: f3410f10461c movss xmm0, dword ptr [r14 + 0x1c]
14206fb49: 0f54c6 andps xmm0, xmm6
14206fb4c: f3410f104e20 movss xmm1, dword ptr [r14 + 0x20]
14206fb52: 0f54ce andps xmm1, xmm6
14206fb55: f3410f105624 movss xmm2, dword ptr [r14 + 0x24]
14206fb5b: 0f54d6 andps xmm2, xmm6
14206fb5e: 0f28d9 movaps xmm3, xmm1
14206fb61: f30fc2d903 cmpunordss xmm3, xmm1
14206fb66: 0f28e3 movaps xmm4, xmm3
14206fb69: 0f54e2 andps xmm4, xmm2
14206fb6c: f30f5fd1 maxss xmm2, xmm1
14206fb70: 0f55da andnps xmm3, xmm2
14206fb73: 0f56dc orps xmm3, xmm4
14206fb76: 0f28cb movaps xmm1, xmm3
14206fb79: f30f5fc8 maxss xmm1, xmm0
14206fb7d: f30fc2c003 cmpunordss xmm0, xmm0
14206fb82: 0f28d0 movaps xmm2, xmm0
14206fb85: 0f54d3 andps xmm2, xmm3
14206fb88: 0f55c1 andnps xmm0, xmm1
14206fb8b: 0f56c2 orps xmm0, xmm2
14206fb8e: e88da384fe call 0x1408b9f20 ; half::binary16::f16::from_f32
14206fb93: 0f137d60 movlps qword ptr [rbp + 0x60], xmm7
14206fb97: f3440f114568 movss dword ptr [rbp + 0x68], xmm8
14206fb9d: 6689456c mov word ptr [rbp + 0x6c], ax
14206fba1: 66895d6e mov word ptr [rbp + 0x6e], bx
14206fba5: 488d454c lea rax, [rbp + 0x4c]
14206fba9: 48898510010000 mov qword ptr [rbp + 0x110], rax
14206fbb0: 488b8de0000000 mov rcx, qword ptr [rbp + 0xe0]
14206fbb7: 89fa mov edx, edi
14206fbb9: e8922092fe call 0x140991c50 ; country_core::render::nani::gpu_world::GpuWorld::owned_subset_mut
14206fbbe: 48898548010000 mov qword ptr [rbp + 0x148], rax
14206fbc5: 488d05c464c000 lea rax, [rip + 0xc064c4]
14206fbcc: 4889442430 mov qword ptr [rsp + 0x30], rax
14206fbd1: 488d059064c000 lea rax, [rip + 0xc06490]
14206fbd8: 4889442428 mov qword ptr [rsp + 0x28], rax
14206fbdd: 488d8510010000 lea rax, [rbp + 0x110]
14206fbe4: 4889442420 mov qword ptr [rsp + 0x20], rax
14206fbe9: 41b901000000 mov r9d, 1
14206fbef: 4889d1 mov rcx, rdx
14206fbf2: 89fa mov edx, edi
14206fbf4: 4c8d4560 lea r8, [rbp + 0x60]
14206fbf8: e8832692fe call 0x140992280 ; country_core::render::nani::gpu_world::OwnedGpuWorldSubset::extend_no_data
14206fbfd: 488bbd48010000 mov rdi, qword ptr [rbp + 0x148]
14206fc04: 4889f9 mov rcx, rdi
14206fc07: e80422c1fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14206fc0c: 4889f9 mov rcx, rdi
14206fc0f: 4883c901 or rcx, 1
14206fc13: e8f821c1fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14206fc18: b808000000 mov eax, 8
14206fc1d: 31c9 xor ecx, ecx
14206fc1f: f0480fb10f lock cmpxchg qword ptr [rdi], rcx
14206fc24: 0f85e2030000 jne 0x14207000c
14206fc2a: 488b85e0000000 mov rax, qword ptr [rbp + 0xe0]
14206fc31: 48ff8040140000 inc qword ptr [rax + 0x1440]
14206fc38: 48ff8560010000 inc qword ptr [rbp + 0x160]
14206fc3f: 440fb67554 movzx r14d, byte ptr [rbp + 0x54]
14206fc44: 4180fe1b cmp r14b, 0x1b
14206fc48: 0f85f2020000 jne 0x14206ff40
14206fc4e: 488d4558 lea rax, [rbp + 0x58]
14206fc52: 48898510010000 mov qword ptr [rbp + 0x110], rax
14206fc59: 488d05801baefe lea rax, [rip - 0x151e480]
14206fc60: 48898518010000 mov qword ptr [rbp + 0x118], rax
14206fc67: 488d051239f600 lea rax, [rip + 0xf63912]
14206fc6e: 48894560 mov qword ptr [rbp + 0x60], rax
14206fc72: 48c7456801000000 mov qword ptr [rbp + 0x68], 1
14206fc7a: 488d8510010000 lea rax, [rbp + 0x110]
14206fc81: 48894570 mov qword ptr [rbp + 0x70], rax
14206fc85: 48c7457801000000 mov qword ptr [rbp + 0x78], 1
14206fc8d: 48c7858000000000000000 mov qword ptr [rbp + 0x80], 0
14206fc98: 488d4dc0 lea rcx, [rbp - 0x40]
14206fc9c: 488d5560 lea rdx, [rbp + 0x60]
14206fca0: e8eba2d0fe call 0x140d79f90 ; alloc::fmt::format::format_inner
14206fca5: 0f1045c0 movups xmm0, xmmword ptr [rbp - 0x40]
14206fca9: 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
14206fcad: 488b45d0 mov rax, qword ptr [rbp - 0x30]
14206fcb1: 48894540 mov qword ptr [rbp + 0x40], rax
14206fcb5: 8b056dae5c01 mov eax, dword ptr [rip + 0x15cae6d]
14206fcbb: 83f803 cmp eax, 3
14206fcbe: 0f8557030000 jne 0x14207001b
14206fcc4: 488b1555ae5c01 mov rdx, qword ptr [rip + 0x15cae55]
14206fccb: 488d4d60 lea rcx, [rbp + 0x60]
14206fccf: e8bc2b4dfe call 0x140542890 ; std::sync::poison::mutex::Mutex::lock<winit::dpi::PhysicalSize<u32> >
14206fcd4: 837d6001 cmp dword ptr [rbp + 0x60], 1
14206fcd8: 0f84e5090000 je 0x1420706c3
14206fcde: 488b4568 mov rax, qword ptr [rbp + 0x68]
14206fce2: 48898558010000 mov qword ptr [rbp + 0x158], rax
14206fce9: 0fb64570 movzx eax, byte ptr [rbp + 0x70]
14206fced: 888548010000 mov byte ptr [rbp + 0x148], al
14206fcf3: 0fb6050f4f5c01 movzx eax, byte ptr [rip + 0x15c4f0f]
14206fcfa: b90f000000 mov ecx, 0xf
14206fcff: ba01000000 mov edx, 1
14206fd04: e8073f5afe call 0x140613c10 ; unknown
14206fd09: 4885c0 test rax, rax
14206fd0c: 0f8499090000 je 0x1420706ab
14206fd12: 48b97061746821202829 movabs rcx, 0x2928202168746170
14206fd1c: 48894807 mov qword ptr [rax + 7], rcx
14206fd20: 48b96d6f64756c655f70 movabs rcx, 0x705f656c75646f6d
14206fd2a: 488908 mov qword ptr [rax], rcx
14206fd2d: 48c745600f000000 mov qword ptr [rbp + 0x60], 0xf
14206fd35: 48894568 mov qword ptr [rbp + 0x68], rax
14206fd39: 48c745700f000000 mov qword ptr [rbp + 0x70], 0xf
14206fd41: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
14206fd4a: ba0f000000 mov edx, 0xf
14206fd4f: 41b817000000 mov r8d, 0x17
14206fd55: 41b901000000 mov r9d, 1
14206fd5b: 488d4d60 lea rcx, [rbp + 0x60]
14206fd5f: e8fc9c8300 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
14206fd64: 488b4568 mov rax, qword ptr [rbp + 0x68]
14206fd68: 488b7d70 mov rdi, qword ptr [rbp + 0x70]
14206fd6c: 0f10054838f600 movups xmm0, xmmword ptr [rip + 0xf63848]
14206fd73: 0f110438 movups xmmword ptr [rax + rdi], xmm0
14206fd77: 48b9203a3a205761726e movabs rcx, 0x6e726157203a3a20
14206fd81: 48894c380f mov qword ptr [rax + rdi + 0xf], rcx
14206fd86: 4883c717 add rdi, 0x17
14206fd8a: 4889bd20010000 mov qword ptr [rbp + 0x120], rdi
14206fd91: 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
14206fd95: 0f298510010000 movaps xmmword ptr [rbp + 0x110], xmm0
14206fd9c: 4c8b7538 mov r14, qword ptr [rbp + 0x38]
14206fda0: 488b5d40 mov rbx, qword ptr [rbp + 0x40]
14206fda4: 488b8510010000 mov rax, qword ptr [rbp + 0x110]
14206fdab: 4829f8 sub rax, rdi
14206fdae: 4839d8 cmp rax, rbx
14206fdb1: 0f82c9020000 jb 0x142070080
14206fdb7: 488b8d18010000 mov rcx, qword ptr [rbp + 0x118]
14206fdbe: 4801f9 add rcx, rdi
14206fdc1: 4c89f2 mov rdx, r14
14206fdc4: 4989d8 mov r8, rbx
14206fdc7: e8343e8b00 call 0x142923c00 ; unknown
14206fdcc: 4801df add rdi, rbx
14206fdcf: 4889bd20010000 mov qword ptr [rbp + 0x120], rdi
14206fdd6: 48897d70 mov qword ptr [rbp + 0x70], rdi
14206fdda: 0f288510010000 movaps xmm0, xmmword ptr [rbp + 0x110]
14206fde1: 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
14206fde5: 488b8558010000 mov rax, qword ptr [rbp + 0x158]
14206fdec: 488d4808 lea rcx, [rax + 8]
14206fdf0: 488d5560 lea rdx, [rbp + 0x60]
14206fdf4: e8b7ba3cff call 0x14143b8b0 ; alloc::collections::btree::map::BTreeMap::insert<alloc::string::String,alloc::collections::btree::set_val::SetValZST,alloc::alloc::Global>
14206fdf9: 84c0 test al, al
14206fdfb: 0f85e3000000 jne 0x14206fee4
14206fe01: 488b05a0c55b01 mov rax, qword ptr [rip + 0x15bc5a0]
14206fe08: 488b00 mov rax, qword ptr [rax]
14206fe0b: 4883f802 cmp rax, 2
14206fe0f: 0f82cf000000 jb 0x14206fee4
14206fe15: 488d4530 lea rax, [rbp + 0x30]
14206fe19: 48898510010000 mov qword ptr [rbp + 0x110], rax
14206fe20: 488d05a9bcfcfd lea rax, [rip - 0x2034357]
14206fe27: 48898518010000 mov qword ptr [rbp + 0x118], rax
14206fe2e: 488d0d5b37f600 lea rcx, [rip + 0xf6375b]
14206fe35: e846df60fe call 0x14067dd80 ; toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut
14206fe3a: 0f1000 movups xmm0, xmmword ptr [rax]
14206fe3d: 8b4010 mov eax, dword ptr [rax + 0x10]
14206fe40: 48c7859000000002000000 mov qword ptr [rbp + 0x90], 2
14206fe4b: 488d159637f600 lea rdx, [rip + 0xf63796]
14206fe52: 48899598000000 mov qword ptr [rbp + 0x98], rdx
14206fe59: 48c785a000000023000000 mov qword ptr [rbp + 0xa0], 0x23
14206fe64: 488d0d6d37f600 lea rcx, [rip + 0xf6376d]
14206fe6b: 48898db0000000 mov qword ptr [rbp + 0xb0], rcx
14206fe72: 48c785b800000001000000 mov qword ptr [rbp + 0xb8], 1
14206fe7d: 488d8d10010000 lea rcx, [rbp + 0x110]
14206fe84: 48898dc0000000 mov qword ptr [rbp + 0xc0], rcx
14206fe8b: 48c785c800000001000000 mov qword ptr [rbp + 0xc8], 1
14206fe96: 48c785d000000000000000 mov qword ptr [rbp + 0xd0], 0
14206fea1: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
14206fea9: 48895568 mov qword ptr [rbp + 0x68], rdx
14206fead: 48c7457023000000 mov qword ptr [rbp + 0x70], 0x23
14206feb5: 48c7457800000000 mov qword ptr [rbp + 0x78], 0
14206febd: 0f118580000000 movups xmmword ptr [rbp + 0x80], xmm0
14206fec4: c785a800000001000000 mov dword ptr [rbp + 0xa8], 1
14206fece: 8985ac000000 mov dword ptr [rbp + 0xac], eax
14206fed4: 488d8dff000000 lea rcx, [rbp + 0xff]
14206fedb: 488d5560 lea rdx, [rbp + 0x60]
14206fedf: e84ca4cdfe call 0x140d4a330 ; log::__private_api::impl$2::log
14206fee4: 80bd4801000000 cmp byte ptr [rbp + 0x148], 0
14206feeb: 488bbd58010000 mov rdi, qword ptr [rbp + 0x158]
14206fef2: 7516 jne 0x14206ff0a
14206fef4: 488b0525c55b01 mov rax, qword ptr [rip + 0x15bc525]
14206fefb: 488b00 mov rax, qword ptr [rax]
14206fefe: 48d1e0 shl rax, 1
14206ff01: 4885c0 test rax, rax
14206ff04: 0f85b0010000 jne 0x1420700ba
14206ff0a: 31c0 xor eax, eax
14206ff0c: 8607 xchg byte ptr [rdi], al
14206ff0e: 3c02 cmp al, 2
14206ff10: 0f8497010000 je 0x1420700ad
14206ff16: 488b5530 mov rdx, qword ptr [rbp + 0x30]
14206ff1a: 4885d2 test rdx, rdx
14206ff1d: 740f je 0x14206ff2e
14206ff1f: 488b4d38 mov rcx, qword ptr [rbp + 0x38]
14206ff23: 41b801000000 mov r8d, 1
14206ff29: e8f23c5afe call 0x140613c20 ; unknown
14206ff2e: 488b5510 mov rdx, qword ptr [rbp + 0x10]
14206ff32: e90ef9ffff jmp 0x14206f845 ; unknown
14206ff37: 660f1f840000000000 nop word ptr [rax + rax]
14206ff40: 8b5d4c mov ebx, dword ptr [rbp + 0x4c]
14206ff43: 448b6d50 mov r13d, dword ptr [rbp + 0x50]
14206ff47: 488d4d55 lea rcx, [rbp + 0x55]
14206ff4b: 0fb701 movzx eax, word ptr [rcx]
14206ff4e: 66898538010000 mov word ptr [rbp + 0x138], ax
14206ff55: 0fb64102 movzx eax, byte ptr [rcx + 2]
14206ff59: 88853a010000 mov byte ptr [rbp + 0x13a], al
14206ff5f: 44887568 mov byte ptr [rbp + 0x68], r14b
14206ff63: 895d60 mov dword ptr [rbp + 0x60], ebx
14206ff66: 44896d64 mov dword ptr [rbp + 0x64], r13d
14206ff6a: 488d8d10010000 lea rcx, [rbp + 0x110]
14206ff71: 488b9518020000 mov rdx, qword ptr [rbp + 0x218]
14206ff78: 4c8b8558010000 mov r8, qword ptr [rbp + 0x158]
14206ff7f: 4c8d4d60 lea r9, [rbp + 0x60]
14206ff83: e8d8674bff call 0x141526760 ; hashbrown::map::HashMap::insert<bevy_ecs::entity::Entity,country_core::render::nani::InstanceHandle,std::hash::random::RandomState,alloc::alloc::Global>
14206ff88: 488b8df0000000 mov rcx, qword ptr [rbp + 0xf0]
14206ff8f: 488b01 mov rax, qword ptr [rcx]
14206ff92: 488b7910 mov rdi, qword ptr [rcx + 0x10]
14206ff96: 4829f8 sub rax, rdi
14206ff99: 4883f827 cmp rax, 0x27
14206ff9d: 0f86b4000000 jbe 0x142070057
14206ffa3: 488b95f0000000 mov rdx, qword ptr [rbp + 0xf0]
14206ffaa: 488b4208 mov rax, qword ptr [rdx + 8]
14206ffae: 488d0d5bba26ff lea rcx, [rip - 0xd945a5]
14206ffb5: 48890c38 mov qword ptr [rax + rdi], rcx
14206ffb9: 488d0d704d4fff lea rcx, [rip - 0xb0b290]
14206ffc0: 48894c3808 mov qword ptr [rax + rdi + 8], rcx
14206ffc5: 488b8d58010000 mov rcx, qword ptr [rbp + 0x158]
14206ffcc: 48894c3810 mov qword ptr [rax + rdi + 0x10], rcx
14206ffd1: 895c3818 mov dword ptr [rax + rdi + 0x18], ebx
14206ffd5: 44896c381c mov dword ptr [rax + rdi + 0x1c], r13d
14206ffda: 4488743820 mov byte ptr [rax + rdi + 0x20], r14b
14206ffdf: 0fb78d38010000 movzx ecx, word ptr [rbp + 0x138]
14206ffe6: 66894c3821 mov word ptr [rax + rdi + 0x21], cx
14206ffeb: 0fb68d3a010000 movzx ecx, byte ptr [rbp + 0x13a]
14206fff2: 884c3823 mov byte ptr [rax + rdi + 0x23], cl
14206fff6: c644382400 mov byte ptr [rax + rdi + 0x24], 0
14206fffb: 4883c728 add rdi, 0x28
14206ffff: 48897a10 mov qword ptr [rdx + 0x10], rdi
142070003: 488b5510 mov rdx, qword ptr [rbp + 0x10]
142070007: e939f8ffff jmp 0x14206f845 ; unknown
14207000c: 4889f9 mov rcx, rdi
14207000f: 31d2 xor edx, edx
142070011: e84abf8500 call 0x1428cbf60 ; parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
142070016: e90ffcffff jmp 0x14206fc2a ; unknown
14207001b: c6851001000001 mov byte ptr [rbp + 0x110], 1
142070022: 488d8510010000 lea rax, [rbp + 0x110]
142070029: 48894560 mov qword ptr [rbp + 0x60], rax
14207002d: 488d055c35f600 lea rax, [rip + 0xf6355c]
142070034: 4889442420 mov qword ptr [rsp + 0x20], rax
142070039: 488d0de8aa5c01 lea rcx, [rip + 0x15caae8]
142070040: 31d2 xor edx, edx
142070042: 4c8d4560 lea r8, [rbp + 0x60]
142070046: 4c8d0d3b2bf600 lea r9, [rip + 0xf62b3b]
14207004d: e8ce878600 call 0x1428d8820 ; std::sys::sync::once::futex::Once::call
142070052: e96dfcffff jmp 0x14206fcc4 ; unknown
142070057: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
142070060: 41b828000000 mov r8d, 0x28
142070066: 41b901000000 mov r9d, 1
14207006c: 488b8df0000000 mov rcx, qword ptr [rbp + 0xf0]
142070073: 4889fa mov rdx, rdi
142070076: e8e5998300 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
14207007b: e923ffffff jmp 0x14206ffa3 ; unknown
142070080: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
142070089: 41b901000000 mov r9d, 1
14207008f: 488d8d10010000 lea rcx, [rbp + 0x110]
142070096: 4889fa mov rdx, rdi
142070099: 4989d8 mov r8, rbx
14207009c: e8bf998300 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1420700a1: 488bbd20010000 mov rdi, qword ptr [rbp + 0x120]
1420700a8: e90afdffff jmp 0x14206fdb7 ; unknown
1420700ad: 4889f9 mov rcx, rdi
1420700b0: e85b878600 call 0x1428d8810 ; std::sys::sync::mutex::futex::Mutex::wake
1420700b5: e95cfeffff jmp 0x14206ff16 ; unknown
1420700ba: e8f1858600 call 0x1428d86b0 ; std::panicking::panic_count::is_zero_slow_path
1420700bf: 84c0 test al, al
1420700c1: 0f8543feffff jne 0x14206ff0a
1420700c7: c6470101 mov byte ptr [rdi + 1], 1
1420700cb: e93afeffff jmp 0x14206ff0a ; unknown
1420700d0: 488b4df8 mov rcx, qword ptr [rbp - 8]
1420700d4: 488b31 mov rsi, qword ptr [rcx]
1420700d7: 4c8b7908 mov r15, qword ptr [rcx + 8]
1420700db: 448b4114 mov r8d, dword ptr [rcx + 0x14]
1420700df: 4c8baef8000000 mov r13, qword ptr [rsi + 0xf8]
1420700e6: 488b8600010000 mov rax, qword ptr [rsi + 0x100]
1420700ed: 4d8d748500 lea r14, [r13 + rax*4]
1420700f2: 0fb68608010000 movzx eax, byte ptr [rsi + 0x108]
1420700f9: 888550010000 mov byte ptr [rbp + 0x150], al
1420700ff: 4489c0 mov eax, r8d
142070102: 2b4110 sub eax, dword ptr [rcx + 0x10]
142070105: 3d00b033c2 cmp eax, 0xc233b000
14207010a: bb00b033c2 mov ebx, 0xc233b000
14207010f: 0f42d8 cmovb ebx, eax
142070112: 41b908000000 mov r9d, 8
142070118: 48c7856001000000000000 mov qword ptr [rbp + 0x160], 0
142070123: 48c7855801000000000000 mov qword ptr [rbp + 0x158], 0
14207012e: 31c0 xor eax, eax
142070130: 31ff xor edi, edi
142070132: 4531e4 xor r12d, r12d
142070135: 44898548010000 mov dword ptr [rbp + 0x148], r8d
14207013c: 80bd5001000000 cmp byte ptr [rbp + 0x150], 0
142070143: 0f8487010000 je 0x1420702d0
142070149: 4c898d00010000 mov qword ptr [rbp + 0x100], r9
142070150: 4939fc cmp r12, rdi
142070153: 0f8552010000 jne 0x1420702ab
142070159: 0f1f8000000000 nop dword ptr [rax]
142070160: 4d39f5 cmp r13, r14
142070163: 0f8498030000 je 0x142070501
142070169: 418b4d00 mov ecx, dword ptr [r13]
14207016d: 4983c504 add r13, 4
142070171: 498b87a8010000 mov rax, qword ptr [r15 + 0x1a8]
142070178: 488d14c9 lea rdx, [rcx + rcx*8]
14207017c: 48837cd01000 cmp qword ptr [rax + rdx*8 + 0x10], 0
142070182: 742c je 0x1420701b0
142070184: 488d0cd0 lea rcx, [rax + rdx*8]
142070188: 488d3cd0 lea rdi, [rax + rdx*8]
14207018c: 4883c710 add rdi, 0x10
142070190: 488b9610010000 mov rdx, qword ptr [rsi + 0x110]
142070197: 488b4140 mov rax, qword ptr [rcx + 0x40]
14207019b: 4839d0 cmp rax, rdx
14207019e: 0f868c000000 jbe 0x142070230
1420701a4: eb46 jmp 0x1420701ec ; unknown
1420701a6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1420701b0: 4d39f5 cmp r13, r14
1420701b3: 0f8448030000 je 0x142070501
1420701b9: 418b4d00 mov ecx, dword ptr [r13]
1420701bd: 4983c504 add r13, 4
1420701c1: 498b87a8010000 mov rax, qword ptr [r15 + 0x1a8]
1420701c8: 488d0cc9 lea rcx, [rcx + rcx*8]
1420701cc: 48837cc81000 cmp qword ptr [rax + rcx*8 + 0x10], 0
1420701d2: 74dc je 0x1420701b0
1420701d4: 488d0cc8 lea rcx, [rax + rcx*8]
1420701d8: 488d7910 lea rdi, [rcx + 0x10]
1420701dc: 488b9610010000 mov rdx, qword ptr [rsi + 0x110]
1420701e3: 488b4140 mov rax, qword ptr [rcx + 0x40]
1420701e7: 4839d0 cmp rax, rdx
1420701ea: 7644 jbe 0x142070230
1420701ec: 4c8b4138 mov r8, qword ptr [rcx + 0x38]
1420701f0: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
1420701f4: 4885d2 test rdx, rdx
1420701f7: 7437 je 0x142070230
1420701f9: 4c8b4118 mov r8, qword ptr [rcx + 0x18]
1420701fd: 48f7d2 not rdx
142070200: 488d1452 lea rdx, [rdx + rdx*2]
142070204: 48c1e204 shl rdx, 4
142070208: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
14207020d: 48899560010000 mov qword ptr [rbp + 0x160], rdx
142070214: 488b9618010000 mov rdx, qword ptr [rsi + 0x118]
14207021b: 4839d0 cmp rax, rdx
14207021e: 7727 ja 0x142070247
142070220: eb5e jmp 0x142070280 ; unknown
142070222: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142070230: 48c7856001000000000000 mov qword ptr [rbp + 0x160], 0
14207023b: 488b9618010000 mov rdx, qword ptr [rsi + 0x118]
142070242: 4839d0 cmp rax, rdx
142070245: 7639 jbe 0x142070280
142070247: 488b4138 mov rax, qword ptr [rcx + 0x38]
14207024b: 488b04d0 mov rax, qword ptr [rax + rdx*8]
14207024f: 4885c0 test rax, rax
142070252: 742c je 0x142070280
142070254: 488b5118 mov rdx, qword ptr [rcx + 0x18]
142070258: 48f7d0 not rax
14207025b: 488d0440 lea rax, [rax + rax*2]
14207025f: 48c1e004 shl rax, 4
142070263: 488b440210 mov rax, qword ptr [rdx + rax + 0x10]
142070268: 48898558010000 mov qword ptr [rbp + 0x158], rax
14207026f: eb1a jmp 0x14207028b ; unknown
142070271: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142070280: 48c7855801000000000000 mov qword ptr [rbp + 0x158], 0
14207028b: 488b9620010000 mov rdx, qword ptr [rsi + 0x120]
142070292: e8d904c9fe call 0x140d00770 ; bevy_ecs::storage::table::Table::get_changed_ticks_slice_for
142070297: 448b8548010000 mov r8d, dword ptr [rbp + 0x148]
14207029e: 4c8b8d00010000 mov r9, qword ptr [rbp + 0x100]
1420702a5: 488b3f mov rdi, qword ptr [rdi]
1420702a8: 4531e4 xor r12d, r12d
1420702ab: 4489e1 mov ecx, r12d
1420702ae: 4489c2 mov edx, r8d
1420702b1: 2b1488 sub edx, dword ptr [rax + rcx*4]
1420702b4: 39da cmp edx, ebx
1420702b6: 0f8284010000 jb 0x142070440
1420702bc: 49ffc4 inc r12
1420702bf: 4939fc cmp r12, rdi
1420702c2: 0f8498feffff je 0x142070160
1420702c8: ebe1 jmp 0x1420702ab ; unknown
1420702ca: 660f1f440000 nop word ptr [rax + rax]
1420702d0: 4939fc cmp r12, rdi
1420702d3: 0f8540010000 jne 0x142070419
1420702d9: 4d39f5 cmp r13, r14
1420702dc: 0f841f020000 je 0x142070501
1420702e2: 418b4d00 mov ecx, dword ptr [r13]
1420702e6: 4983c504 add r13, 4
1420702ea: 498b8700010000 mov rax, qword ptr [r15 + 0x100]
1420702f1: 488d0c89 lea rcx, [rcx + rcx*4]
1420702f5: 48c1e105 shl rcx, 5
1420702f9: 48837c085800 cmp qword ptr [rax + rcx + 0x58], 0
1420702ff: 740f je 0x142070310
142070301: 488d3c08 lea rdi, [rax + rcx]
142070305: 4c8d2408 lea r12, [rax + rcx]
142070309: 4983c458 add r12, 0x58
14207030d: eb30 jmp 0x14207033f ; unknown
14207030f: 90 nop 
142070310: 4d39f5 cmp r13, r14
142070313: 0f84e8010000 je 0x142070501
142070319: 418b4500 mov eax, dword ptr [r13]
14207031d: 4983c504 add r13, 4
142070321: 498bbf00010000 mov rdi, qword ptr [r15 + 0x100]
142070328: 488d0480 lea rax, [rax + rax*4]
14207032c: 48c1e005 shl rax, 5
142070330: 48837c075800 cmp qword ptr [rdi + rax + 0x58], 0
142070336: 74d8 je 0x142070310
142070338: 4801c7 add rdi, rax
14207033b: 4c8d6758 lea r12, [rdi + 0x58]
14207033f: 8b8794000000 mov eax, dword ptr [rdi + 0x94]
142070345: 4d8b87a8010000 mov r8, qword ptr [r15 + 0x1a8]
14207034c: 488d04c0 lea rax, [rax + rax*8]
142070350: 498d0cc0 lea rcx, [r8 + rax*8]
142070354: 488b9610010000 mov rdx, qword ptr [rsi + 0x110]
14207035b: 498b44c040 mov rax, qword ptr [r8 + rax*8 + 0x40]
142070360: 4839d0 cmp rax, rdx
142070363: 763b jbe 0x1420703a0
142070365: 4c8b4138 mov r8, qword ptr [rcx + 0x38]
142070369: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
14207036d: 4885d2 test rdx, rdx
142070370: 742e je 0x1420703a0
142070372: 4c8b4118 mov r8, qword ptr [rcx + 0x18]
142070376: 48f7d2 not rdx
142070379: 488d1452 lea rdx, [rdx + rdx*2]
14207037d: 48c1e204 shl rdx, 4
142070381: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
142070386: 48899560010000 mov qword ptr [rbp + 0x160], rdx
14207038d: 488b9618010000 mov rdx, qword ptr [rsi + 0x118]
142070394: 4839d0 cmp rax, rdx
142070397: 771e ja 0x1420703b7
142070399: eb55 jmp 0x1420703f0 ; unknown
14207039b: 0f1f440000 nop dword ptr [rax + rax]
1420703a0: 48c7856001000000000000 mov qword ptr [rbp + 0x160], 0
1420703ab: 488b9618010000 mov rdx, qword ptr [rsi + 0x118]
1420703b2: 4839d0 cmp rax, rdx
1420703b5: 7639 jbe 0x1420703f0
1420703b7: 488b4138 mov rax, qword ptr [rcx + 0x38]
1420703bb: 488b04d0 mov rax, qword ptr [rax + rdx*8]
1420703bf: 4885c0 test rax, rax
1420703c2: 742c je 0x1420703f0
1420703c4: 488b5118 mov rdx, qword ptr [rcx + 0x18]
1420703c8: 48f7d0 not rax
1420703cb: 488d0440 lea rax, [rax + rax*2]
1420703cf: 48c1e004 shl rax, 4
1420703d3: 488b440210 mov rax, qword ptr [rdx + rax + 0x10]
1420703d8: 48898558010000 mov qword ptr [rbp + 0x158], rax
1420703df: eb1a jmp 0x1420703fb ; unknown
1420703e1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1420703f0: 48c7855801000000000000 mov qword ptr [rbp + 0x158], 0
1420703fb: 488b9620010000 mov rdx, qword ptr [rsi + 0x120]
142070402: e86903c9fe call 0x140d00770 ; bevy_ecs::storage::table::Table::get_changed_ticks_slice_for
142070407: 448b8548010000 mov r8d, dword ptr [rbp + 0x148]
14207040e: 4c8b4f50 mov r9, qword ptr [rdi + 0x50]
142070412: 498b3c24 mov rdi, qword ptr [r12]
142070416: 4531e4 xor r12d, r12d
142070419: 4c89e1 mov rcx, r12
14207041c: 48c1e104 shl rcx, 4
142070420: 418b4c0908 mov ecx, dword ptr [r9 + rcx + 8]
142070425: 4489c2 mov edx, r8d
142070428: 2b1488 sub edx, dword ptr [rax + rcx*4]
14207042b: 39da cmp edx, ebx
14207042d: 7211 jb 0x142070440
14207042f: 49ffc4 inc r12
142070432: e999feffff jmp 0x1420702d0 ; unknown
142070437: 660f1f840000000000 nop word ptr [rax + rax]
142070440: 488d0c49 lea rcx, [rcx + rcx*2]
142070444: 488b9560010000 mov rdx, qword ptr [rbp + 0x160]
14207044b: 4c8d04ca lea r8, [rdx + rcx*8]
14207044f: 488b9558010000 mov rdx, qword ptr [rbp + 0x158]
142070456: 488d148a lea rdx, [rdx + rcx*4]
14207045a: 4c898d00010000 mov qword ptr [rbp + 0x100], r9
142070461: 48898540010000 mov qword ptr [rbp + 0x140], rax
142070468: 41833801 cmp dword ptr [r8], 1
14207046c: 7506 jne 0x142070474
14207046e: 458b4004 mov r8d, dword ptr [r8 + 4]
142070472: eb55 jmp 0x1420704c9 ; unknown
142070474: 48899508010000 mov qword ptr [rbp + 0x108], rdx
14207047b: 4c8985f0000000 mov qword ptr [rbp + 0xf0], r8
142070482: 498b5008 mov rdx, qword ptr [r8 + 8]
142070486: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
14207048a: e8c17c9cfe call 0x140a38150 ; country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::get_by_name<utils::name_hash::NameHash>
14207048f: 488955e8 mov qword ptr [rbp - 0x18], rdx
142070493: 4883fa01 cmp rdx, 1
142070497: 0f851f010000 jne 0x1420705bc
14207049d: 448b00 mov r8d, dword ptr [rax]
1420704a0: 0fb64004 movzx eax, byte ptr [rax + 4]
1420704a4: 44898510010000 mov dword ptr [rbp + 0x110], r8d
1420704ab: 888514010000 mov byte ptr [rbp + 0x114], al
1420704b1: 4c8b95f0000000 mov r10, qword ptr [rbp + 0xf0]
1420704b8: 413a4210 cmp al, byte ptr [r10 + 0x10]
1420704bc: 0f8529010000 jne 0x1420705eb
1420704c2: 488b9508010000 mov rdx, qword ptr [rbp + 0x108]
1420704c9: 49ffc4 inc r12
1420704cc: 8b8534010000 mov eax, dword ptr [rbp + 0x134]
1420704d2: 488b8de8000000 mov rcx, qword ptr [rbp + 0xe8]
1420704d9: 8901 mov dword ptr [rcx], eax
1420704db: 488b8de0000000 mov rcx, qword ptr [rbp + 0xe0]
1420704e2: e8791892fe call 0x140991d60 ; country_core::render::nani::gpu_world::GpuWorld::update_instance_mesh
1420704e7: 488b8540010000 mov rax, qword ptr [rbp + 0x140]
1420704ee: 448b8548010000 mov r8d, dword ptr [rbp + 0x148]
1420704f5: 4c8b8d00010000 mov r9, qword ptr [rbp + 0x100]
1420704fc: e93bfcffff jmp 0x14207013c ; unknown
142070501: 48837d0000 cmp qword ptr [rbp], 0
142070506: 7444 je 0x14207054c
142070508: e8237cbafe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14207050d: 4889c1 mov rcx, rax
142070510: 488b00 mov rax, qword ptr [rax]
142070513: 4883f801 cmp rax, 1
142070517: 755d jne 0x142070576
142070519: 4883c108 add rcx, 8
14207051d: 48833900 cmp qword ptr [rcx], 0
142070521: 0f8589000000 jne 0x1420705b0
142070527: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14207052e: 48898d60010000 mov qword ptr [rbp + 0x160], rcx
142070535: 4883c108 add rcx, 8
142070539: 488b5508 mov rdx, qword ptr [rbp + 8]
14207053d: e8ae6bbafe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
142070542: 488b8560010000 mov rax, qword ptr [rbp + 0x160]
142070549: 48ff00 inc qword ptr [rax]
14207054c: 0f28b570010000 movaps xmm6, xmmword ptr [rbp + 0x170]
142070553: 0f28bd80010000 movaps xmm7, xmmword ptr [rbp + 0x180]
14207055a: 440f288590010000 movaps xmm8, xmmword ptr [rbp + 0x190]
142070562: 4881c428020000 add rsp, 0x228
142070569: 5b pop rbx
14207056a: 5f pop rdi
14207056b: 5e pop rsi
14207056c: 415c pop r12
14207056e: 415d pop r13
142070570: 415e pop r14
142070572: 415f pop r15
142070574: 5d pop rbp
142070575: c3 ret 
142070576: 4885c0 test rax, rax
142070579: 7529 jne 0x1420705a4
14207057b: 31d2 xor edx, edx
14207057d: e84e828800 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
142070582: 4889c1 mov rcx, rax
142070585: 4885c0 test rax, rax
142070588: 7593 jne 0x14207051d
14207058a: eb18 jmp 0x1420705a4 ; unknown
14207058c: 4885c0 test rax, rax
14207058f: 7513 jne 0x1420705a4
142070591: 31d2 xor edx, edx
142070593: e838828800 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
142070598: 4889c1 mov rcx, rax
14207059b: 4885c0 test rax, rax
14207059e: 0f8586efffff jne 0x14206f52a
1420705a4: 488d0df5a4bc00 lea rcx, [rip + 0xbca4f5]
1420705ab: e8207b8600 call 0x1428d80d0 ; std::thread::local::panic_access_error
1420705b0: 488d0db9a5bc00 lea rcx, [rip + 0xbca5b9]
1420705b7: e8948c8600 call 0x1428d9250 ; core::cell::panic_already_borrowed
1420705bc: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
1420705c4: 488d059d2ef600 lea rax, [rip + 0xf62e9d]
1420705cb: 4889442420 mov qword ptr [rsp + 0x20], rax
1420705d0: 4c8d05892ef600 lea r8, [rip + 0xf62e89]
1420705d7: 488d55e8 lea rdx, [rbp - 0x18]
1420705db: 4c8d4d60 lea r9, [rbp + 0x60]
1420705df: 31c9 xor ecx, ecx
1420705e1: e8b08f8600 call 0x1428d9596 ; core::panicking::assert_failed<usize,usize>
1420705e6: e918010000 jmp 0x142070703 ; unknown
1420705eb: 4983c210 add r10, 0x10
1420705ef: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
1420705f7: 488d05822ef600 lea rax, [rip + 0xf62e82]
1420705fe: 4889442420 mov qword ptr [rsp + 0x20], rax
142070603: 4c8d4d60 lea r9, [rbp + 0x60]
142070607: 31c9 xor ecx, ecx
142070609: 488d9514010000 lea rdx, [rbp + 0x114]
142070610: 4d89d0 mov r8, r10
142070613: e8c1928300 call 0x1428a98d9 ; core::panicking::assert_failed<country_core::render::nani::gpu_world::GpuWorldSubsetKind,country_core::render::nani::gpu_world::GpuWorldSubsetKind>
142070618: e9e6000000 jmp 0x142070703 ; unknown
14207061d: 488d45f0 lea rax, [rbp - 0x10]
142070621: 48898510010000 mov qword ptr [rbp + 0x110], rax
142070628: 488d0561ecffff lea rax, [rip - 0x139f]
14207062f: 48898518010000 mov qword ptr [rbp + 0x118], rax
142070636: 4c898520010000 mov qword ptr [rbp + 0x120], r8
14207063d: 488d051c040100 lea rax, [rip + 0x1041c]
142070644: 48898528010000 mov qword ptr [rbp + 0x128], rax
14207064b: 488d05b62ef600 lea rax, [rip + 0xf62eb6]
142070652: 48894560 mov qword ptr [rbp + 0x60], rax
142070656: 48c7456803000000 mov qword ptr [rbp + 0x68], 3
14207065e: 48c7858000000000000000 mov qword ptr [rbp + 0x80], 0
142070669: 488d8510010000 lea rax, [rbp + 0x110]
142070670: 48894570 mov qword ptr [rbp + 0x70], rax
142070674: 48c7457802000000 mov qword ptr [rbp + 0x78], 2
14207067c: 488d15b52ef600 lea rdx, [rip + 0xf62eb5]
142070683: 488d4d60 lea rcx, [rbp + 0x60]
142070687: e8048d8600 call 0x1428d9390 ; core::panicking::panic_fmt
14207068c: eb75 jmp 0x142070703 ; unknown
14207068e: 8b8534010000 mov eax, dword ptr [rbp + 0x134]
142070694: 488b8de8000000 mov rcx, qword ptr [rbp + 0xe8]
14207069b: 8901 mov dword ptr [rcx], eax
14207069d: 488d0d6c2ff600 lea rcx, [rip + 0xf62f6c]
1420706a4: e8678c8600 call 0x1428d9310 ; core::option::unwrap_failed
1420706a9: eb58 jmp 0x142070703 ; unknown
1420706ab: 4c8d05ae26f600 lea r8, [rip + 0xf626ae]
1420706b2: b901000000 mov ecx, 1
1420706b7: ba0f000000 mov edx, 0xf
1420706bc: e822898600 call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1420706c1: eb40 jmp 0x142070703 ; unknown
1420706c3: 488b4568 mov rax, qword ptr [rbp + 0x68]
1420706c7: 0fb64d70 movzx ecx, byte ptr [rbp + 0x70]
1420706cb: 48898510010000 mov qword ptr [rbp + 0x110], rax
1420706d2: 888d18010000 mov byte ptr [rbp + 0x118], cl
1420706d8: 488d05b12ef600 lea rax, [rip + 0xf62eb1]
1420706df: 4889442420 mov qword ptr [rsp + 0x20], rax
1420706e4: 488d0dbd2ef600 lea rcx, [rip + 0xf62ebd]
1420706eb: 4c8d0dfe25f600 lea r9, [rip + 0xf625fe]
1420706f2: 4c8d8510010000 lea r8, [rbp + 0x110]
1420706f9: ba13000000 mov edx, 0x13
1420706fe: e85d908600 call 0x1428d9760 ; core::result::unwrap_failed
142070703: 0f0b ud2 
142070705: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
142070710: 4889542410 mov qword ptr [rsp + 0x10], rdx
142070715: 55 push rbp
142070716: 4157 push r15
142070718: 4156 push r14
14207071a: 4155 push r13
14207071c: 4154 push r12
14207071e: 56 push rsi
14207071f: 57 push rdi
142070720: 53 push rbx
142070721: 4883ec68 sub rsp, 0x68
142070725: 488daa80000000 lea rbp, [rdx + 0x80]
14207072c: 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
142070732: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
142070737: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
14207073c: 488b9510010000 mov rdx, qword ptr [rbp + 0x110]
142070743: 4885d2 test rdx, rdx
142070746: 7412 je 0x14207075a
142070748: 488b8d18010000 mov rcx, qword ptr [rbp + 0x118]
14207074f: 41b801000000 mov r8d, 1
142070755: e8c6345afe call 0x140613c20 ; unknown
14207075a: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
14207075f: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
142070764: 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
14207076a: 4883c468 add rsp, 0x68
14207076e: 5b pop rbx
14207076f: 5f pop rdi
142070770: 5e pop rsi
142070771: 415c pop r12
142070773: 415d pop r13
142070775: 415e pop r14
142070777: 415f pop r15
142070779: 5d pop rbp
14207077a: c3 ret 
14207077b: 0f1f440000 nop dword ptr [rax + rax]
142070780: 4889542410 mov qword ptr [rsp + 0x10], rdx
142070785: 55 push rbp
142070786: 4157 push r15
142070788: 4156 push r14
14207078a: 4155 push r13
14207078c: 4154 push r12
14207078e: 56 push rsi
14207078f: 57 push rdi
142070790: 53 push rbx
142070791: 4883ec68 sub rsp, 0x68
142070795: 488daa80000000 lea rbp, [rdx + 0x80]
14207079c: 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
1420707a2: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1420707a7: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1420707ac: 488b8560010000 mov rax, qword ptr [rbp + 0x160]
1420707b3: 48ff00 inc qword ptr [rax]
1420707b6: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1420707bb: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1420707c0: 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
1420707c6: 4883c468 add rsp, 0x68
1420707ca: 5b pop rbx
1420707cb: 5f pop rdi
1420707cc: 5e pop rsi
1420707cd: 415c pop r12
1420707cf: 415d pop r13
1420707d1: 415e pop r14
1420707d3: 415f pop r15
1420707d5: 5d pop rbp
1420707d6: c3 ret 
1420707d7: 660f1f840000000000 nop word ptr [rax + rax]
1420707e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1420707e5: 55 push rbp
1420707e6: 4157 push r15
1420707e8: 4156 push r14
1420707ea: 4155 push r13
1420707ec: 4154 push r12
1420707ee: 56 push rsi
1420707ef: 57 push rdi
1420707f0: 53 push rbx
1420707f1: 4883ec68 sub rsp, 0x68
1420707f5: 488daa80000000 lea rbp, [rdx + 0x80]
1420707fc: 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
142070802: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
142070807: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
14207080c: 488b8560010000 mov rax, qword ptr [rbp + 0x160]
142070813: 48ff00 inc qword ptr [rax]
142070816: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
14207081b: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
142070820: 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
142070826: 4883c468 add rsp, 0x68
14207082a: 5b pop rbx
14207082b: 5f pop rdi
14207082c: 5e pop rsi
14207082d: 415c pop r12
14207082f: 415d pop r13
142070831: 415e pop r14
142070833: 415f pop r15
142070835: 5d pop rbp
142070836: c3 ret 
142070837: 660f1f840000000000 nop word ptr [rax + rax]
142070840: 4889542410 mov qword ptr [rsp + 0x10], rdx
142070845: 55 push rbp
142070846: 4157 push r15
142070848: 4156 push r14
14207084a: 4155 push r13
14207084c: 4154 push r12
14207084e: 56 push rsi
14207084f: 57 push rdi
142070850: 53 push rbx
142070851: 4883ec68 sub rsp, 0x68
142070855: 488daa80000000 lea rbp, [rdx + 0x80]
14207085c: 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
142070862: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
142070867: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
14207086c: 488d8d10010000 lea rcx, [rbp + 0x110]
142070873: e8f81f56fe call 0x1405d2870 ; core::ptr::drop_in_place<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<bool> > >
142070878: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
14207087d: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
142070882: 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
142070888: 4883c468 add rsp, 0x68
14207088c: 5b pop rbx
14207088d: 5f pop rdi
14207088e: 5e pop rsi
14207088f: 415c pop r12
142070891: 415d pop r13
142070893: 415e pop r14
142070895: 415f pop r15
142070897: 5d pop rbp
142070898: c3 ret 
142070899: 0f1f8000000000 nop dword ptr [rax]
1420708a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1420708a5: 55 push rbp
1420708a6: 4157 push r15
1420708a8: 4156 push r14
1420708aa: 4155 push r13
1420708ac: 4154 push r12
1420708ae: 56 push rsi
1420708af: 57 push rdi
1420708b0: 53 push rbx
1420708b1: 4883ec68 sub rsp, 0x68
1420708b5: 488daa80000000 lea rbp, [rdx + 0x80]
1420708bc: 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
1420708c2: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1420708c7: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1420708cc: 488b5560 mov rdx, qword ptr [rbp + 0x60]
1420708d0: 4885d2 test rdx, rdx
1420708d3: 740f je 0x1420708e4
1420708d5: 488b4d68 mov rcx, qword ptr [rbp + 0x68]
1420708d9: 41b801000000 mov r8d, 1
1420708df: e83c335afe call 0x140613c20 ; unknown
1420708e4: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1420708e9: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1420708ee: 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
1420708f4: 4883c468 add rsp, 0x68
1420708f8: 5b pop rbx
1420708f9: 5f pop rdi
1420708fa: 5e pop rsi
1420708fb: 415c pop r12
1420708fd: 415d pop r13
1420708ff: 415e pop r14
142070901: 415f pop r15
142070903: 5d pop rbp
142070904: c3 ret 
142070905: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
142070910: 4889542410 mov qword ptr [rsp + 0x10], rdx
142070915: 55 push rbp
142070916: 4157 push r15
142070918: 4156 push r14
14207091a: 4155 push r13
14207091c: 4154 push r12
14207091e: 56 push rsi
14207091f: 57 push rdi
142070920: 53 push rbx
142070921: 4883ec68 sub rsp, 0x68
142070925: 488daa80000000 lea rbp, [rdx + 0x80]
14207092c: 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
142070932: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
142070937: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
14207093c: 488b8d48010000 mov rcx, qword ptr [rbp + 0x148]
142070943: e88809fdfd call 0x1400412d0 ; core::ptr::drop_in_place<lock_api::rwlock::MappedRwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,egui::context::ContextImpl> >
142070948: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
14207094d: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
142070952: 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
142070958: 4883c468 add rsp, 0x68
14207095c: 5b pop rbx
14207095d: 5f pop rdi
14207095e: 5e pop rsi
14207095f: 415c pop r12
142070961: 415d pop r13
142070963: 415e pop r14
142070965: 415f pop r15
142070967: 5d pop rbp
142070968: c3 ret 
142070969: 0f1f8000000000 nop dword ptr [rax]
142070970: 4889542410 mov qword ptr [rsp + 0x10], rdx
142070975: 55 push rbp
142070976: 4157 push r15
142070978: 4156 push r14
14207097a: 4155 push r13
14207097c: 4154 push r12
14207097e: 56 push rsi
14207097f: 57 push rdi
142070980: 53 push rbx
142070981: 4883ec68 sub rsp, 0x68
142070985: 488daa80000000 lea rbp, [rdx + 0x80]
14207098c: 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
142070992: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
142070997: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
14207099c: 488b8d58010000 mov rcx, qword ptr [rbp + 0x158]
1420709a3: 0fb69548010000 movzx edx, byte ptr [rbp + 0x148]
1420709aa: e85126fdfd call 0x140043000 ; core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<std::collections::hash::set::HashSet<alloc::string::String,std::hash::random::RandomState> > >
1420709af: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
1420709b4: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
1420709b9: 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
1420709bf: 4883c468 add rsp, 0x68
1420709c3: 5b pop rbx
1420709c4: 5f pop rdi
1420709c5: 5e pop rsi
1420709c6: 415c pop r12
1420709c8: 415d pop r13
1420709ca: 415e pop r14
1420709cc: 415f pop r15
1420709ce: 5d pop rbp
1420709cf: c3 ret 
1420709d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1420709d5: 55 push rbp
1420709d6: 4157 push r15
1420709d8: 4156 push r14
1420709da: 4155 push r13
1420709dc: 4154 push r12
1420709de: 56 push rsi
1420709df: 57 push rdi
1420709e0: 53 push rbx
1420709e1: 4883ec68 sub rsp, 0x68
1420709e5: 488daa80000000 lea rbp, [rdx + 0x80]
1420709ec: 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
1420709f2: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
1420709f7: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
1420709fc: 488b5530 mov rdx, qword ptr [rbp + 0x30]
142070a00: 4885d2 test rdx, rdx
142070a03: 740f je 0x142070a14
142070a05: 488b4d38 mov rcx, qword ptr [rbp + 0x38]
142070a09: 41b801000000 mov r8d, 1
142070a0f: e80c325afe call 0x140613c20 ; unknown
142070a14: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
142070a19: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
142070a1e: 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
142070a24: 4883c468 add rsp, 0x68
142070a28: 5b pop rbx
142070a29: 5f pop rdi
142070a2a: 5e pop rsi
142070a2b: 415c pop r12
142070a2d: 415d pop r13
142070a2f: 415e pop r14
142070a31: 415f pop r15
142070a33: 5d pop rbp
142070a34: c3 ret 
142070a35: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
142070a40: 4889542410 mov qword ptr [rsp + 0x10], rdx
142070a45: 55 push rbp
142070a46: 4157 push r15
142070a48: 4156 push r14
142070a4a: 4155 push r13
142070a4c: 4154 push r12
142070a4e: 56 push rsi
142070a4f: 57 push rdi
142070a50: 53 push rbx
142070a51: 4883ec68 sub rsp, 0x68
142070a55: 488daa80000000 lea rbp, [rdx + 0x80]
142070a5c: 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
142070a62: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
142070a67: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
142070a6c: 48837d0000 cmp qword ptr [rbp], 0
142070a71: 7420 je 0x142070a93
142070a73: 488d4508 lea rax, [rbp + 8]
142070a77: 488945e0 mov qword ptr [rbp - 0x20], rax
142070a7b: 488d45e0 lea rax, [rbp - 0x20]
142070a7f: 488945d8 mov qword ptr [rbp - 0x28], rax
142070a83: 488d0dce28f600 lea rcx, [rip + 0xf628ce]
142070a8a: 488d55d8 lea rdx, [rbp - 0x28]
142070a8e: e83de82bff call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
142070a93: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
142070a98: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
142070a9d: 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
142070aa3: 4883c468 add rsp, 0x68
142070aa7: 5b pop rbx
142070aa8: 5f pop rdi
142070aa9: 5e pop rsi
142070aaa: 415c pop r12
142070aac: 415d pop r13
142070aae: 415e pop r14
142070ab0: 415f pop r15
142070ab2: 5d pop rbp
142070ab3: c3 ret 
