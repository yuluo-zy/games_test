140a8b530: push rbp
140a8b531: push r15
140a8b533: push r14
140a8b535: push r13
140a8b537: push r12
140a8b539: push rsi
140a8b53a: push rdi
140a8b53b: push rbx
140a8b53c: sub rsp, 0xe88
140a8b543: lea rbp, [rsp + 0x80]
140a8b54b: movaps xmmword ptr [rbp + 0xdf0], xmm15
140a8b553: movaps xmmword ptr [rbp + 0xde0], xmm14
140a8b55b: movaps xmmword ptr [rbp + 0xdd0], xmm13
140a8b563: movaps xmmword ptr [rbp + 0xdc0], xmm12
140a8b56b: movaps xmmword ptr [rbp + 0xdb0], xmm11
140a8b573: movaps xmmword ptr [rbp + 0xda0], xmm10
140a8b57b: movaps xmmword ptr [rbp + 0xd90], xmm9
140a8b583: movaps xmmword ptr [rbp + 0xd80], xmm8
140a8b58b: movaps xmmword ptr [rbp + 0xd70], xmm7
140a8b592: movdqa xmmword ptr [rbp + 0xd60], xmm6
140a8b59a: mov qword ptr [rbp + 0xd58], 0xfffffffffffffffe
140a8b5a5: mov r14, r9
140a8b5a8: mov rbx, r8
140a8b5ab: mov rsi, rdx
140a8b5ae: mov rdi, rcx
140a8b5b1: call 0x140c16a00 ; puffin::are_scopes_on
140a8b5b6: test al, al
140a8b5b8: mov qword ptr [rbp + 0xd10], rbx
140a8b5bf: je 0x140a8b621
140a8b5c1: lea r12, [rip + 0x204d609]
140a8b5c8: mov r15d, 0x5d
140a8b5ce: mov edx, 0x5d
140a8b5d3: mov rcx, r12
140a8b5d6: call 0x140a87c80 ; core::str::impl$0::rfind<ref$<str$> >
140a8b5db: cmp rax, 1
140a8b5df: jne 0x140a8b697
140a8b5e5: mov r9, rdx
140a8b5e8: test rdx, rdx
140a8b5eb: je 0x140a8b62a
140a8b5ed: cmp r9, 0x5d
140a8b5f1: jae 0x140a8b628
140a8b5f3: lea rax, [rip + 0x204d5d7]
140a8b5fa: cmp byte ptr [r9 + rax], 0xbf
140a8b5ff: jg 0x140a8b62a
140a8b601: lea rax, [rip + 0x204d740]
140a8b608: mov qword ptr [rsp + 0x20], rax
140a8b60d: lea rcx, [rip + 0x204d5bd]
140a8b614: mov edx, 0x5d
140a8b619: xor r8d, r8d
140a8b61c: call 0x1428d9940 ; core::str::slice_error_fail
140a8b621: xor eax, eax
140a8b623: jmp 0x140a8b7fd
140a8b628: jne 0x140a8b601
140a8b62a: lea r12, [rip + 0x204d5a0]
140a8b631: mov rcx, r12
140a8b634: mov rdx, r9
140a8b637: call 0x140a87c80 ; core::str::impl$0::rfind<ref$<str$> >
140a8b63c: cmp rax, 1
140a8b640: jne 0x140a8b697
140a8b642: mov r8, rdx
140a8b645: add r8, 2
140a8b649: je 0x140a8b684
140a8b64b: cmp r8, 0x5d
140a8b64f: jae 0x140a8b682
140a8b651: lea rax, [rip + 0x204d579]
140a8b658: cmp byte ptr [r8 + rax], 0xbf
140a8b65d: jg 0x140a8b684
140a8b65f: lea rax, [rip + 0x204d6fa]
140a8b666: mov qword ptr [rsp + 0x20], rax
140a8b66b: lea rcx, [rip + 0x204d55f]
140a8b672: mov edx, 0x5d
140a8b677: mov r9d, 0x5d
140a8b67d: call 0x1428d9940 ; core::str::slice_error_fail
140a8b682: jne 0x140a8b65f
140a8b684: mov r15d, 0x5b
140a8b68a: sub r15, rdx
140a8b68d: lea r12, [rip + 0x204d53d]
140a8b694: add r12, r8
140a8b697: lea rax, [rip + 0x204dc16]
140a8b69e: lea r13, [rip + 0x204dbc3]
140a8b6a5: nop word ptr cs:[rax + rax]
140a8b6b0: movsx ecx, byte ptr [rax - 1]
140a8b6b4: test ecx, ecx
140a8b6b6: js 0x140a8b6d0
140a8b6b8: dec rax
140a8b6bb: cmp ecx, 0x5c
140a8b6be: jne 0x140a8b729
140a8b6c0: jmp 0x140a8b73e
140a8b6c2: nop word ptr cs:[rax + rax]
140a8b6d0: movzx edx, byte ptr [rax - 2]
140a8b6d4: cmp dl, 0xc0
140a8b6d7: jge 0x140a8b6fe
140a8b6d9: movzx r8d, byte ptr [rax - 3]
140a8b6de: cmp r8b, 0xc0
140a8b6e2: jge 0x140a8b707
140a8b6e4: movzx r9d, byte ptr [rax - 4]
140a8b6e9: add rax, -4
140a8b6ed: and r9d, 7
140a8b6f1: shl r9d, 6
140a8b6f5: and r8d, 0x3f
140a8b6f9: or r8d, r9d
140a8b6fc: jmp 0x140a8b70f
140a8b6fe: add rax, -2
140a8b702: and edx, 0x1f
140a8b705: jmp 0x140a8b719
140a8b707: add rax, -3
140a8b70b: and r8d, 0xf
140a8b70f: shl r8d, 6
140a8b713: and edx, 0x3f
140a8b716: or edx, r8d
140a8b719: shl edx, 6
140a8b71c: and cl, 0x3f
140a8b71f: movzx ecx, cl
140a8b722: or ecx, edx
140a8b724: cmp ecx, 0x5c
140a8b727: je 0x140a8b73e
140a8b729: cmp ecx, 0x2f
140a8b72c: je 0x140a8b73e
140a8b72e: cmp rax, r13
140a8b731: jne 0x140a8b6b0
140a8b737: mov ebx, 0x4c
140a8b73c: jmp 0x140a8b78d
140a8b73e: lea r13, [rip + 0x204db23]
140a8b745: sub rax, r13
140a8b748: mov r8, rax
140a8b74b: inc r8
140a8b74e: je 0x140a8b782
140a8b750: cmp r8, 0x4c
140a8b754: jae 0x140a8b780
140a8b756: cmp byte ptr [r8 + r13], 0xbf
140a8b75b: jg 0x140a8b782
140a8b75d: lea rax, [rip + 0x204d5c4]
140a8b764: mov qword ptr [rsp + 0x20], rax
140a8b769: lea rcx, [rip + 0x204daf8]
140a8b770: mov edx, 0x4c
140a8b775: mov r9d, 0x4c
140a8b77b: call 0x1428d9940 ; core::str::slice_error_fail
140a8b780: jne 0x140a8b75d
140a8b782: mov ebx, 0x4b
140a8b787: sub rbx, rax
140a8b78a: add r13, r8
140a8b78d: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
140a8b792: mov rcx, rax
140a8b795: mov rax, qword ptr [rax]
140a8b798: cmp rax, 1
140a8b79c: jne 0x140a8dd97
140a8b7a2: add rcx, 8
140a8b7a6: cmp qword ptr [rcx], 0
140a8b7aa: jne 0x140a8df97
140a8b7b0: mov qword ptr [rcx], 0xffffffffffffffff
140a8b7b7: mov qword ptr [rbp + 0xd48], rcx
140a8b7be: add rcx, 8
140a8b7c2: mov qword ptr [rsp + 0x20], rbx
140a8b7c7: mov qword ptr [rsp + 0x30], 0
140a8b7d0: mov qword ptr [rsp + 0x28], 1
140a8b7d9: mov rdx, r12
140a8b7dc: mov r8, r15
140a8b7df: mov r9, r13
140a8b7e2: call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
140a8b7e7: mov rcx, qword ptr [rbp + 0xd48]
140a8b7ee: inc qword ptr [rcx]
140a8b7f1: mov qword ptr [rbp + 0xc08], rax
140a8b7f8: mov eax, 1
140a8b7fd: mov qword ptr [rbp + 0xd00], r14
140a8b804: mov rcx, qword ptr [rbp + 0xe90]
140a8b80b: mov r15, qword ptr [rbp + 0xe78]
140a8b812: mov qword ptr [rbp + 0xc00], rax
140a8b819: mov eax, dword ptr [rsi + 0x1c]
140a8b81c: mov rdx, qword ptr [rsi + 0x10]
140a8b820: mov dword ptr [rdx], eax
140a8b822: mov rbx, qword ptr [rsi]
140a8b825: mov eax, dword ptr [rdi + 0x1c]
140a8b828: mov rdx, qword ptr [rdi + 0x10]
140a8b82c: mov dword ptr [rdx], eax
140a8b82e: mov rax, qword ptr [rdi]
140a8b831: mov qword ptr [rbp + 0xd48], rax
140a8b838: mov rax, qword ptr [rcx]
140a8b83b: mov rcx, qword ptr [rcx + 8]
140a8b83f: mov rdx, qword ptr [rax]
140a8b842: xor r8d, r8d
140a8b845: mov r9, rdx
140a8b848: sub r9, qword ptr [rcx + 0x18]
140a8b84c: cmovb r9, r8
140a8b850: sub rdx, qword ptr [rcx + 0x38]
140a8b854: cmovb rdx, r8
140a8b858: mov r10, qword ptr [rcx + 0x30]
140a8b85c: sub r10, rdx
140a8b85f: cmovb r10, r8
140a8b863: seta dl
140a8b866: mov r11, qword ptr [rcx + 0x10]
140a8b86a: sub r11, r9
140a8b86d: cmovb r11, r8
140a8b871: mov rcx, qword ptr [rcx + 0x40]
140a8b875: seta r8b
140a8b879: add r11, r10
140a8b87c: mov r9, rcx
140a8b87f: sub r9, r11
140a8b882: mov qword ptr [rax], r9
140a8b885: or r8b, dl
140a8b888: mov qword ptr [rbp + 0xcf8], rbx
140a8b88f: jne 0x140a8be09
140a8b895: mov eax, dword ptr [r15 + 0x1c]
140a8b899: mov dword ptr [rbp + 0xce0], eax
140a8b89f: mov r14, qword ptr [r15]
140a8b8a2: mov rax, qword ptr [r15 + 0x10]
140a8b8a6: mov qword ptr [rbp + 0xce8], rax
140a8b8ad: mov rax, qword ptr [rbp + 0xe80]
140a8b8b4: mov r13, qword ptr [rax]
140a8b8b7: mov rcx, qword ptr [rax + 8]
140a8b8bb: mov r8, qword ptr [r13]
140a8b8bf: xor edx, edx
140a8b8c1: mov rax, r8
140a8b8c4: sub rax, qword ptr [rcx + 0x18]
140a8b8c8: cmovb rax, rdx
140a8b8cc: sub r8, qword ptr [rcx + 0x38]
140a8b8d0: cmovb r8, rdx
140a8b8d4: mov rbx, rax
140a8b8d7: shl rbx, 4
140a8b8db: add rbx, qword ptr [rcx + 8]
140a8b8df: mov r10, qword ptr [rcx + 0x10]
140a8b8e3: mov r11, qword ptr [rcx + 0x30]
140a8b8e7: sub r10, rax
140a8b8ea: cmovb r10, rdx
140a8b8ee: mov r9d, 8
140a8b8f4: cmovb rbx, r9
140a8b8f8: mov rax, r8
140a8b8fb: shl rax, 4
140a8b8ff: add rax, qword ptr [rcx + 0x28]
140a8b903: sub r11, r8
140a8b906: cmovb r11, rdx
140a8b90a: cmovb rax, r9
140a8b90e: mov rcx, qword ptr [rcx + 0x40]
140a8b912: sub rcx, r11
140a8b915: sub rcx, r10
140a8b918: mov qword ptr [r13], rcx
140a8b91c: shl r10, 4
140a8b920: add r10, rbx
140a8b923: mov qword ptr [rbp + 0xd30], r10
140a8b92a: shl r11, 4
140a8b92e: add r11, rax
140a8b931: mov qword ptr [rbp + 0xcf0], r11
140a8b938: mov rcx, qword ptr [rbp + 0xd48]
140a8b93f: add rcx, 0x20
140a8b943: mov qword ptr [rbp + 0xcc0], rcx
140a8b94a: pcmpeqd xmm6, xmm6
140a8b94e: lea rcx, [r14 + 0x20]
140a8b952: mov qword ptr [rbp + 0xd20], rcx
140a8b959: jmp 0x140a8b985
140a8b95b: nop dword ptr [rax + rax]
140a8b960: mov rax, qword ptr [r14 + 0x28]
140a8b964: mov rcx, rsi
140a8b967: shl rcx, 4
140a8b96b: mov qword ptr [rax + rcx], r15
140a8b96f: mov qword ptr [rax + rcx + 8], rbx
140a8b974: inc rsi
140a8b977: mov qword ptr [r14 + 0x30], rsi
140a8b97b: inc qword ptr [r14 + 0x40]
140a8b97f: mov rax, rdi
140a8b982: mov rbx, r12
140a8b985: test rbx, rbx
140a8b988: je 0x140a8b9a0
140a8b98a: cmp rbx, qword ptr [rbp + 0xd30]
140a8b991: je 0x140a8b9a0
140a8b993: lea r12, [rbx + 0x10]
140a8b997: mov rdi, rax
140a8b99a: jmp 0x140a8b9c0
140a8b99c: nop dword ptr [rax]
140a8b9a0: test rax, rax
140a8b9a3: je 0x140a8bacd
140a8b9a9: cmp rax, qword ptr [rbp + 0xcf0]
140a8b9b0: je 0x140a8bacd
140a8b9b6: lea rdi, [rax + 0x10]
140a8b9ba: xor r12d, r12d
140a8b9bd: mov rbx, rax
140a8b9c0: inc qword ptr [r13]
140a8b9c4: mov rsi, qword ptr [rbx + 8]
140a8b9c8: mov qword ptr [rbp + 0x840], rsi
140a8b9cf: mov rax, qword ptr [rbp + 0xd48]
140a8b9d6: cmp qword ptr [rax + 0x18], 0
140a8b9db: je 0x140a8b97f
140a8b9dd: mov rcx, qword ptr [rbp + 0xcc0]
140a8b9e4: lea rdx, [rbp + 0x840]
140a8b9eb: call 0x1405b0d70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >
140a8b9f0: mov rdx, qword ptr [rbp + 0xd48]
140a8b9f7: mov rcx, qword ptr [rdx]
140a8b9fa: mov rdx, qword ptr [rdx + 8]
140a8b9fe: mov r8, rdx
140a8ba01: and r8, rax
140a8ba04: shr rax, 0x39
140a8ba08: movd xmm0, eax
140a8ba0c: punpcklbw xmm0, xmm0
140a8ba10: pshuflw xmm0, xmm0, 0
140a8ba15: pshufd xmm0, xmm0, 0
140a8ba1a: lea rax, [rcx - 0x78]
140a8ba1e: xor r9d, r9d
140a8ba21: movdqu xmm1, xmmword ptr [rcx + r8]
140a8ba27: movdqa xmm2, xmm1
140a8ba2b: pcmpeqb xmm2, xmm0
140a8ba2f: pmovmskb r10d, xmm2
140a8ba34: test r10d, r10d
140a8ba37: je 0x140a8ba60
140a8ba39: tzcnt r11d, r10d
140a8ba3e: add r11, r8
140a8ba41: and r11, rdx
140a8ba44: neg r11
140a8ba47: imul r11, r11, 0x78
140a8ba4b: cmp qword ptr [rax + r11], rsi
140a8ba4f: je 0x140a8ba90
140a8ba51: lea r11d, [r10 - 1]
140a8ba55: and r11w, r10w
140a8ba59: mov r10d, r11d
140a8ba5c: jne 0x140a8ba39
140a8ba5e: nop 
140a8ba60: pcmpeqb xmm1, xmm6
140a8ba64: pmovmskb r10d, xmm1
140a8ba69: test r10d, r10d
140a8ba6c: jne 0x140a8b97f
140a8ba72: add r8, r9
140a8ba75: add r8, 0x10
140a8ba79: add r9, 0x10
140a8ba7d: and r8, rdx
140a8ba80: jmp 0x140a8ba21
140a8ba82: nop word ptr cs:[rax + rax]
140a8ba90: mov rbx, qword ptr [rbx + 8]
140a8ba94: mov eax, dword ptr [rbp + 0xce0]
140a8ba9a: mov rcx, qword ptr [rbp + 0xce8]
140a8baa1: mov dword ptr [rcx], eax
140a8baa3: mov rsi, qword ptr [r14 + 0x30]
140a8baa7: mov r15, qword ptr [r14 + 0x40]
140a8baab: cmp rsi, qword ptr [r14 + 0x20]
140a8baaf: jne 0x140a8b960
140a8bab5: mov rcx, qword ptr [rbp + 0xd20]
140a8babc: lea rdx, [rip + 0x21ae685]
140a8bac3: call 0x140562630 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
140a8bac8: jmp 0x140a8b960
140a8bacd: mov rax, qword ptr [rbp + 0xe88]
140a8bad4: mov r13, qword ptr [rax]
140a8bad7: mov rcx, qword ptr [rax + 8]
140a8badb: mov r8, qword ptr [r13]
140a8badf: xor edx, edx
140a8bae1: mov rax, r8
140a8bae4: sub rax, qword ptr [rcx + 0x18]
140a8bae8: cmovb rax, rdx
140a8baec: sub r8, qword ptr [rcx + 0x38]
140a8baf0: cmovb r8, rdx
140a8baf4: mov rbx, rax
140a8baf7: shl rbx, 4
140a8bafb: add rbx, qword ptr [rcx + 8]
140a8baff: mov r10, qword ptr [rcx + 0x10]
140a8bb03: mov r11, qword ptr [rcx + 0x30]
140a8bb07: sub r10, rax
140a8bb0a: cmovb r10, rdx
140a8bb0e: mov r9d, 8
140a8bb14: cmovb rbx, r9
140a8bb18: mov rax, r8
140a8bb1b: shl rax, 4
140a8bb1f: add rax, qword ptr [rcx + 0x28]
140a8bb23: sub r11, r8
140a8bb26: cmovb r11, rdx
140a8bb2a: cmovb rax, r9
140a8bb2e: mov rcx, qword ptr [rcx + 0x40]
140a8bb32: sub rcx, r11
140a8bb35: sub rcx, r10
140a8bb38: mov qword ptr [r13], rcx
140a8bb3c: shl r10, 4
140a8bb40: add r10, rbx
140a8bb43: mov qword ptr [rbp + 0xd30], r10
140a8bb4a: shl r11, 4
140a8bb4e: add r11, rax
140a8bb51: mov qword ptr [rbp + 0xcf0], r11
140a8bb58: pcmpeqd xmm6, xmm6
140a8bb5c: jmp 0x140a8bb85
140a8bb5e: nop 
140a8bb60: mov rax, qword ptr [r14 + 0x28]
140a8bb64: mov rcx, rbx
140a8bb67: shl rcx, 4
140a8bb6b: mov qword ptr [rax + rcx], rsi
140a8bb6f: mov qword ptr [rax + rcx + 8], r15
140a8bb74: inc rbx
140a8bb77: mov qword ptr [r14 + 0x30], rbx
140a8bb7b: inc qword ptr [r14 + 0x40]
140a8bb7f: mov rbx, r12
140a8bb82: mov rax, rdi
140a8bb85: test rbx, rbx
140a8bb88: je 0x140a8bba0
140a8bb8a: cmp rbx, qword ptr [rbp + 0xd30]
140a8bb91: je 0x140a8bba0
140a8bb93: lea r12, [rbx + 0x10]
140a8bb97: mov rdi, rax
140a8bb9a: jmp 0x140a8bbc0
140a8bb9c: nop dword ptr [rax]
140a8bba0: test rax, rax
140a8bba3: je 0x140a8bccd
140a8bba9: cmp rax, qword ptr [rbp + 0xcf0]
140a8bbb0: je 0x140a8bccd
140a8bbb6: lea rdi, [rax + 0x10]
140a8bbba: xor r12d, r12d
140a8bbbd: mov rbx, rax
140a8bbc0: inc qword ptr [r13]
140a8bbc4: mov r15, qword ptr [rbx + 8]
140a8bbc8: mov qword ptr [rbp + 0x840], r15
140a8bbcf: mov rax, qword ptr [rbp + 0xd48]
140a8bbd6: cmp qword ptr [rax + 0x18], 0
140a8bbdb: je 0x140a8bb7f
140a8bbdd: mov rcx, qword ptr [rbp + 0xcc0]
140a8bbe4: lea rdx, [rbp + 0x840]
140a8bbeb: call 0x1405b0d70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >
140a8bbf0: mov rdx, qword ptr [rbp + 0xd48]
140a8bbf7: mov rcx, qword ptr [rdx]
140a8bbfa: mov rdx, qword ptr [rdx + 8]
140a8bbfe: mov r8, rdx
140a8bc01: and r8, rax
140a8bc04: shr rax, 0x39
140a8bc08: movd xmm0, eax
140a8bc0c: punpcklbw xmm0, xmm0
140a8bc10: pshuflw xmm0, xmm0, 0
140a8bc15: pshufd xmm0, xmm0, 0
140a8bc1a: lea rax, [rcx - 0x78]
140a8bc1e: xor r9d, r9d
140a8bc21: movdqu xmm1, xmmword ptr [rcx + r8]
140a8bc27: movdqa xmm2, xmm1
140a8bc2b: pcmpeqb xmm2, xmm0
140a8bc2f: pmovmskb r10d, xmm2
140a8bc34: test r10d, r10d
140a8bc37: je 0x140a8bc60
140a8bc39: tzcnt r11d, r10d
140a8bc3e: add r11, r8
140a8bc41: and r11, rdx
140a8bc44: neg r11
140a8bc47: imul r11, r11, 0x78
140a8bc4b: cmp qword ptr [rax + r11], r15
140a8bc4f: je 0x140a8bc90
140a8bc51: lea r11d, [r10 - 1]
140a8bc55: and r11w, r10w
140a8bc59: mov r10d, r11d
140a8bc5c: jne 0x140a8bc39
140a8bc5e: nop 
140a8bc60: pcmpeqb xmm1, xmm6
140a8bc64: pmovmskb r10d, xmm1
140a8bc69: test r10d, r10d
140a8bc6c: jne 0x140a8bb7f
140a8bc72: add r8, r9
140a8bc75: add r8, 0x10
140a8bc79: add r9, 0x10
140a8bc7d: and r8, rdx
140a8bc80: jmp 0x140a8bc21
140a8bc82: nop word ptr cs:[rax + rax]
140a8bc90: mov r15, qword ptr [rbx + 8]
140a8bc94: mov eax, dword ptr [rbp + 0xce0]
140a8bc9a: mov rcx, qword ptr [rbp + 0xce8]
140a8bca1: mov dword ptr [rcx], eax
140a8bca3: mov rbx, qword ptr [r14 + 0x30]
140a8bca7: mov rsi, qword ptr [r14 + 0x40]
140a8bcab: cmp rbx, qword ptr [r14 + 0x20]
140a8bcaf: jne 0x140a8bb60
140a8bcb5: mov rcx, qword ptr [rbp + 0xd20]
140a8bcbc: lea rdx, [rip + 0x21ae485]
140a8bcc3: call 0x140562630 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
140a8bcc8: jmp 0x140a8bb60
140a8bccd: mov rax, qword ptr [rbp + 0xd10]
140a8bcd4: mov rsi, qword ptr [rax]
140a8bcd7: mov rax, qword ptr [rbp + 0xd00]
140a8bcde: mov rax, qword ptr [rax]
140a8bce1: mov qword ptr [rbp + 0xce0], rax
140a8bce8: mov rdx, qword ptr [rbp + 0xe98]
140a8bcef: mov eax, dword ptr [rdx + 0x1c]
140a8bcf2: mov rcx, qword ptr [rdx + 0x10]
140a8bcf6: mov dword ptr [rcx], eax
140a8bcf8: mov r14, qword ptr [rdx]
140a8bcfb: mov qword ptr [rbp + 0x840], rsi
140a8bd02: mov rax, qword ptr [rbp + 0xd48]
140a8bd09: mov qword ptr [rbp + 0x848], rax
140a8bd10: mov rax, qword ptr [rbp + 0xcf8]
140a8bd17: mov qword ptr [rbp + 0x850], rax
140a8bd1e: lea r15, [rbp + 0x840]
140a8bd25: mov rcx, r14
140a8bd28: mov rdx, r15
140a8bd2b: call 0x141458b20 ; country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0>
140a8bd30: xor ecx, ecx
140a8bd32: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
140a8bd37: test rax, rax
140a8bd3a: je 0x140a8e0cd
140a8bd40: movups xmm0, xmmword ptr [rax]
140a8bd43: inc qword ptr [rax]
140a8bd46: movaps xmmword ptr [rbp + 0x860], xmm0
140a8bd4d: movups xmm0, xmmword ptr [rip + 0x21e01ac]
140a8bd54: movaps xmmword ptr [rbp + 0x840], xmm0
140a8bd5b: movdqu xmm0, xmmword ptr [rip + 0x21e01ad]
140a8bd63: movdqa xmmword ptr [rbp + 0x850], xmm0
140a8bd6b: add r14, 8
140a8bd6f: mov qword ptr [rbp + 0x260], rsi
140a8bd76: mov rsi, qword ptr [rbp + 0xd48]
140a8bd7d: mov qword ptr [rbp + 0x268], rsi
140a8bd84: mov rbx, qword ptr [rbp + 0xcf8]
140a8bd8b: mov qword ptr [rbp + 0x270], rbx
140a8bd92: mov qword ptr [rbp + 0x278], r15
140a8bd99: mov byte ptr [rbp + 0xd55], 1
140a8bda0: lea rdx, [rbp + 0x260]
140a8bda7: mov rcx, r14
140a8bdaa: call 0x141458a50 ; country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1>
140a8bdaf: mov rdi, qword ptr [rbp + 0x840]
140a8bdb6: mov rcx, qword ptr [rbp + 0x848]
140a8bdbd: movdqa xmm0, xmmword ptr [rdi]
140a8bdc1: test rcx, rcx
140a8bdc4: mov qword ptr [rbp + 0xd30], rcx
140a8bdcb: je 0x140a8bf1e
140a8bdd1: lea rax, [rcx*8 + 0x17]
140a8bdd9: and rax, 0xfffffffffffffff0
140a8bddd: add rcx, rax
140a8bde0: add rcx, 0x11
140a8bde4: mov qword ptr [rbp + 0xd20], rcx
140a8bdeb: mov rcx, rdi
140a8bdee: sub rcx, rax
140a8bdf1: mov qword ptr [rbp + 0xd00], rcx
140a8bdf8: mov eax, 0x10
140a8bdfd: mov qword ptr [rbp + 0xd10], rax
140a8be04: jmp 0x140a8bf29
140a8be09: mov qword ptr [rax], rcx
140a8be0c: mov rsi, qword ptr [rbx]
140a8be0f: mov rdx, qword ptr [rbx + 0x18]
140a8be13: movdqa xmm0, xmmword ptr [rsi]
140a8be17: pmovmskb eax, xmm0
140a8be1b: not eax
140a8be1d: lea rbx, [rsi + 0x10]
140a8be21: mov ecx, dword ptr [r15 + 0x1c]
140a8be25: mov dword ptr [rbp + 0xce0], ecx
140a8be2b: mov r14, qword ptr [r15]
140a8be2e: mov rcx, qword ptr [r15 + 0x10]
140a8be32: mov qword ptr [rbp + 0xce8], rcx
140a8be39: lea rcx, [r14 + 0x20]
140a8be3d: mov qword ptr [rbp + 0xd30], rcx
140a8be44: jmp 0x140a8be79
140a8be46: nop word ptr cs:[rax + rax]
140a8be50: mov rax, qword ptr [r14 + 0x28]
140a8be54: mov rcx, r15
140a8be57: shl rcx, 4
140a8be5b: mov qword ptr [rax + rcx], rdi
140a8be5f: mov qword ptr [rax + rcx + 8], r12
140a8be64: inc r15
140a8be67: mov qword ptr [r14 + 0x30], r15
140a8be6b: inc qword ptr [r14 + 0x40]
140a8be6f: mov eax, r13d
140a8be72: mov rdx, qword ptr [rbp + 0xcc0]
140a8be79: test rdx, rdx
140a8be7c: je 0x140a8b8ad
140a8be82: test ax, ax
140a8be85: je 0x140a8be90
140a8be87: lea r13d, [rax - 1]
140a8be8b: and r13d, eax
140a8be8e: jmp 0x140a8beb9
140a8be90: movdqa xmm0, xmmword ptr [rbx]
140a8be94: pmovmskb eax, xmm0
140a8be98: add rsi, -0x2a80
140a8be9f: add rbx, 0x10
140a8bea3: cmp eax, 0xffff
140a8bea8: je 0x140a8be90
140a8beaa: mov ecx, 0xfffffffe
140a8beaf: sub ecx, eax
140a8beb1: not eax
140a8beb3: mov r13d, eax
140a8beb6: and r13d, ecx
140a8beb9: dec rdx
140a8bebc: tzcnt eax, eax
140a8bec0: neg rax
140a8bec3: imul rcx, rax, 0x2a8
140a8beca: cmp byte ptr [rsi + rcx - 7], 1
140a8becf: mov eax, r13d
140a8bed2: jbe 0x140a8be79
140a8bed4: mov qword ptr [rbp + 0xcc0], rdx
140a8bedb: add rcx, rsi
140a8bede: mov r12, qword ptr [rcx - 0x2a8]
140a8bee5: mov eax, dword ptr [rbp + 0xce0]
140a8beeb: mov rcx, qword ptr [rbp + 0xce8]
140a8bef2: mov dword ptr [rcx], eax
140a8bef4: mov r15, qword ptr [r14 + 0x30]
140a8bef8: mov rdi, qword ptr [r14 + 0x40]
140a8befc: cmp r15, qword ptr [r14 + 0x20]
140a8bf00: jne 0x140a8be50
140a8bf06: mov rcx, qword ptr [rbp + 0xd30]
140a8bf0d: lea rdx, [rip + 0x21ae234]
140a8bf14: call 0x140562630 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
140a8bf19: jmp 0x140a8be50
140a8bf1e: mov qword ptr [rbp + 0xd10], 0
140a8bf29: mov r14, qword ptr [rbp + 0x858]
140a8bf30: lea r15, [rdi + 0x10]
140a8bf34: pmovmskb eax, xmm0
140a8bf38: not eax
140a8bf3a: lea rcx, [rsi + 0x60]
140a8bf3e: mov qword ptr [rbp + 0xcf0], rcx
140a8bf45: lea rcx, [rbx + 0x20]
140a8bf49: mov qword ptr [rbp + 0xce8], rcx
140a8bf50: pcmpeqd xmm6, xmm6
140a8bf54: xorps xmm7, xmm7
140a8bf57: jmp 0x140a8c150
140a8bf5c: nop dword ptr [rax]
140a8bf60: movdqu xmm1, xmmword ptr [rsi + rdx]
140a8bf65: movdqa xmm2, xmm1
140a8bf69: pcmpeqb xmm2, xmm0
140a8bf6d: pmovmskb r9d, xmm2
140a8bf72: test r9d, r9d
140a8bf75: je 0x140a8bfa0
140a8bf77: tzcnt r10d, r9d
140a8bf7c: add r10, rdx
140a8bf7f: and r10, rcx
140a8bf82: neg r10
140a8bf85: imul r12, r10, 0x2a8
140a8bf8c: cmp qword ptr [rax + r12], r13
140a8bf90: je 0x140a8bfd0
140a8bf92: lea r10d, [r9 - 1]
140a8bf96: and r10w, r9w
140a8bf9a: mov r9d, r10d
140a8bf9d: jne 0x140a8bf77
140a8bf9f: nop 
140a8bfa0: pcmpeqb xmm1, xmm6
140a8bfa4: pmovmskb r9d, xmm1
140a8bfa9: test r9d, r9d
140a8bfac: jne 0x140a8c210
140a8bfb2: add rdx, r8
140a8bfb5: add rdx, 0x10
140a8bfb9: add r8, 0x10
140a8bfbd: and rdx, rcx
140a8bfc0: jmp 0x140a8bf60
140a8bfc2: nop word ptr cs:[rax + rax]
140a8bfd0: cmp qword ptr [rsi + r12 - 0x290], 2
140a8bfd9: jb 0x140a8df59
140a8bfdf: movss xmm0, dword ptr [rsi + r12 - 0x270]
140a8bfe9: ucomiss xmm0, xmm7
140a8bfec: jbe 0x140a8df59
140a8bff2: mov qword ptr [rbp + 0x260], r13
140a8bff9: mov rax, qword ptr [rbp + 0xd48]
140a8c000: cmp qword ptr [rax + 0x18], 0
140a8c005: je 0x140a8c217
140a8c00b: mov rcx, qword ptr [rbp + 0xcc0]
140a8c012: lea rdx, [rbp + 0x260]
140a8c019: call 0x1405b0d70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >
140a8c01e: add rsi, r12
140a8c021: mov rdx, qword ptr [rbp + 0xd48]
140a8c028: mov rcx, qword ptr [rdx]
140a8c02b: mov rdx, qword ptr [rdx + 8]
140a8c02f: mov r8, rdx
140a8c032: and r8, rax
140a8c035: shr rax, 0x39
140a8c039: movd xmm0, eax
140a8c03d: punpcklbw xmm0, xmm0
140a8c041: pshuflw xmm0, xmm0, 0
140a8c046: pshufd xmm0, xmm0, 0
140a8c04b: lea rax, [rcx - 0x78]
140a8c04f: xor r9d, r9d
140a8c052: movdqu xmm1, xmmword ptr [rcx + r8]
140a8c058: movdqa xmm2, xmm1
140a8c05c: pcmpeqb xmm2, xmm0
140a8c060: pmovmskb r10d, xmm2
140a8c065: test r10d, r10d
140a8c068: je 0x140a8c090
140a8c06a: tzcnt r11d, r10d
140a8c06f: add r11, r8
140a8c072: and r11, rdx
140a8c075: neg r11
140a8c078: imul r11, r11, 0x78
140a8c07c: cmp qword ptr [rax + r11], r13
140a8c080: je 0x140a8c0c0
140a8c082: lea r11d, [r10 - 1]
140a8c086: and r11w, r10w
140a8c08a: mov r10d, r11d
140a8c08d: jne 0x140a8c06a
140a8c08f: nop 
140a8c090: pcmpeqb xmm1, xmm6
140a8c094: pmovmskb r10d, xmm1
140a8c099: test r10d, r10d
140a8c09c: jne 0x140a8c217
140a8c0a2: add r8, r9
140a8c0a5: add r8, 0x10
140a8c0a9: add r9, 0x10
140a8c0ad: and r8, rdx
140a8c0b0: jmp 0x140a8c052
140a8c0b2: nop word ptr cs:[rax + rax]
140a8c0c0: movss xmm8, dword ptr [rcx + r11 - 0x18]
140a8c0c7: mov r12, qword ptr [rsi - 0x260]
140a8c0ce: mov rax, qword ptr [rsi - 0x258]
140a8c0d5: lea rax, [rax + rax*2]
140a8c0d9: lea rsi, [r12 + rax*4]
140a8c0dd: jmp 0x140a8c0e5
140a8c0df: nop 
140a8c0e0: ucomiss xmm0, xmm1
140a8c0e3: ja 0x140a8c132
140a8c0e5: cmp r12, rsi
140a8c0e8: je 0x140a8c12e
140a8c0ea: movd xmm1, dword ptr [r12]
140a8c0f0: movd xmm2, dword ptr [r12 + 8]
140a8c0f7: mov rcx, qword ptr [rbp + 0xce0]
140a8c0fe: call 0x1409e9640 ; country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>
140a8c103: movss xmm1, dword ptr [r12 + 4]
140a8c10a: add r12, 0xc
140a8c10e: ucomiss xmm1, xmm8
140a8c112: jbe 0x140a8c0e0
140a8c114: ucomiss xmm0, xmm1
140a8c117: jne 0x140a8c11b
140a8c119: jnp 0x140a8c0e5
140a8c11b: mov rcx, qword ptr [rbp + 0xcf0]
140a8c122: mov rdx, r13
140a8c125: call 0x1402ca2f0 ; hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global>
140a8c12a: mov eax, ebx
140a8c12c: jmp 0x140a8c150
140a8c12e: mov eax, ebx
140a8c130: jmp 0x140a8c150
140a8c132: mov rcx, qword ptr [rbp + 0xcf0]
140a8c139: mov rdx, r13
140a8c13c: call 0x1402ca2f0 ; hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global>
140a8c141: mov eax, ebx
140a8c143: nop word ptr cs:[rax + rax]
140a8c150: test r14, r14
140a8c153: je 0x140a8c228
140a8c159: test ax, ax
140a8c15c: je 0x140a8c170
140a8c15e: lea ebx, [rax - 1]
140a8c161: and ebx, eax
140a8c163: jmp 0x140a8c195
140a8c165: nop word ptr cs:[rax + rax]
140a8c170: movdqa xmm0, xmmword ptr [r15]
140a8c175: pmovmskb eax, xmm0
140a8c179: add rdi, -0x80
140a8c17d: add r15, 0x10
140a8c181: cmp eax, 0xffff
140a8c186: je 0x140a8c170
140a8c188: mov ecx, 0xfffffffe
140a8c18d: sub ecx, eax
140a8c18f: not eax
140a8c191: mov ebx, eax
140a8c193: and ebx, ecx
140a8c195: dec r14
140a8c198: tzcnt eax, eax
140a8c19c: shl eax, 3
140a8c19f: mov rcx, rdi
140a8c1a2: sub rcx, rax
140a8c1a5: mov r13, qword ptr [rcx - 8]
140a8c1a9: mov qword ptr [rbp + 0x260], r13
140a8c1b0: mov rax, qword ptr [rbp + 0xcf8]
140a8c1b7: cmp qword ptr [rax + 0x18], 0
140a8c1bc: je 0x140a8c210
140a8c1be: mov rcx, qword ptr [rbp + 0xce8]
140a8c1c5: lea rdx, [rbp + 0x260]
140a8c1cc: call 0x1405b0d70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >
140a8c1d1: mov rcx, qword ptr [rbp + 0xcf8]
140a8c1d8: mov rsi, qword ptr [rcx]
140a8c1db: mov rcx, qword ptr [rcx + 8]
140a8c1df: mov rdx, rcx
140a8c1e2: and rdx, rax
140a8c1e5: shr rax, 0x39
140a8c1e9: movd xmm0, eax
140a8c1ed: punpcklbw xmm0, xmm0
140a8c1f1: pshuflw xmm0, xmm0, 0
140a8c1f6: pshufd xmm0, xmm0, 0
140a8c1fb: lea rax, [rsi - 0x2a8]
140a8c202: xor r8d, r8d
140a8c205: jmp 0x140a8bf60
140a8c20a: nop word ptr [rax + rax]
140a8c210: mov eax, ebx
140a8c212: jmp 0x140a8c150
140a8c217: lea rcx, [rip + 0x204d102]
140a8c21e: call 0x1428d9310 ; core::option::unwrap_failed
140a8c223: jmp 0x140a8e207
140a8c228: cmp qword ptr [rbp + 0xd30], 0
140a8c230: mov rsi, qword ptr [rbp + 0xd48]
140a8c237: mov rdx, qword ptr [rbp + 0xd20]
140a8c23e: mov rcx, qword ptr [rbp + 0xd00]
140a8c245: je 0x140a8c258
140a8c247: test rdx, rdx
140a8c24a: je 0x140a8c258
140a8c24c: mov r8, qword ptr [rbp + 0xd10]
140a8c253: call 0x140613c20
140a8c258: lea rax, [rsi + 0x90]
140a8c25f: mov qword ptr [rbp + 0xb80], rax
140a8c266: mov qword ptr [rbp + 0x840], rax
140a8c26d: lea rdx, [rbp + 0x840]
140a8c274: mov rcx, qword ptr [rbp + 0xcf0]
140a8c27b: call 0x14152d580 ; hashbrown::map::HashMap::retain<country_core::resources::walls::WallId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global,hashbrown::set::impl$1::retain::closure_env$0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state::closure_env$0> >
140a8c280: mov rax, qword ptr [rbp + 0xd48]
140a8c287: mov rsi, qword ptr [rax + 0x90]
140a8c28e: mov rdi, qword ptr [rax + 0xa8]
140a8c295: movdqa xmm0, xmmword ptr [rsi]
140a8c299: pmovmskb r12d, xmm0
140a8c29e: not r12d
140a8c2a1: lea rbx, [rsi + 0x10]
140a8c2a5: mov rax, qword ptr [rbp + 0xe70]
140a8c2ac: mov ecx, dword ptr [rax + 0x1c]
140a8c2af: mov dword ptr [rbp + 0xd20], ecx
140a8c2b5: mov rcx, qword ptr [rax]
140a8c2b8: mov rax, qword ptr [rax + 0x10]
140a8c2bc: mov qword ptr [rbp + 0xd10], rax
140a8c2c3: mov qword ptr [rbp + 0xd30], rcx
140a8c2ca: lea rax, [rcx + 0x20]
140a8c2ce: mov qword ptr [rbp + 0xd00], rax
140a8c2d5: jmp 0x140a8c306
140a8c2d7: nop word ptr [rax + rax]
140a8c2e0: mov rdx, qword ptr [rbp + 0xd30]
140a8c2e7: mov rax, qword ptr [rdx + 0x28]
140a8c2eb: mov rcx, r15
140a8c2ee: shl rcx, 4
140a8c2f2: mov qword ptr [rax + rcx], r13
140a8c2f6: mov qword ptr [rax + rcx + 8], r14
140a8c2fb: inc r15
140a8c2fe: mov qword ptr [rdx + 0x30], r15
140a8c302: inc qword ptr [rdx + 0x40]
140a8c306: test rdi, rdi
140a8c309: je 0x140a8c41d
140a8c30f: test r12w, r12w
140a8c313: jne 0x140a8c33d
140a8c315: nop word ptr cs:[rax + rax]
140a8c320: movdqa xmm0, xmmword ptr [rbx]
140a8c324: pmovmskb r12d, xmm0
140a8c329: add rsi, -0x80
140a8c32d: add rbx, 0x10
140a8c331: cmp r12d, 0xffff
140a8c338: je 0x140a8c320
140a8c33a: not r12d
140a8c33d: tzcnt eax, r12d
140a8c342: shl eax, 3
140a8c345: mov rcx, rsi
140a8c348: sub rcx, rax
140a8c34b: mov rax, qword ptr [rcx - 8]
140a8c34f: mov qword ptr [rbp + 0xc70], rax
140a8c356: mov rcx, qword ptr [rbp + 0xce8]
140a8c35d: lea rdx, [rbp + 0xc70]
140a8c364: call 0x1405b0d70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >
140a8c369: lea rcx, [rbp + 0x840]
140a8c370: mov rdx, qword ptr [rbp + 0xcf8]
140a8c377: mov r8, rax
140a8c37a: lea r9, [rbp + 0xc70]
140a8c381: call 0x140b772b0 ; hashbrown::raw::RawTable::remove_entry<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState> >
140a8c386: lea eax, [r12 - 1]
140a8c38b: and eax, r12d
140a8c38e: mov r12d, eax
140a8c391: dec rdi
140a8c394: xor r14d, r14d
140a8c397: cmp r14, qword ptr [rbp + 0x848]
140a8c39e: jo 0x140a8c306
140a8c3a4: mov r8d, 0x2a0
140a8c3aa: lea rcx, [rbp + 0x260]
140a8c3b1: lea rdx, [rbp + 0x848]
140a8c3b8: call 0x142923c00
140a8c3bd: cmp r14, qword ptr [rbp + 0x260]
140a8c3c4: jo 0x140a8c306
140a8c3ca: lea rcx, [rbp + 0x260]
140a8c3d1: call 0x140895600 ; core::ptr::drop_in_place<country_core::resources::walls::public_wall_state::PublicWallState>
140a8c3d6: mov r14, qword ptr [rbp + 0xc70]
140a8c3dd: mov eax, dword ptr [rbp + 0xd20]
140a8c3e3: mov rcx, qword ptr [rbp + 0xd10]
140a8c3ea: mov dword ptr [rcx], eax
140a8c3ec: mov rax, qword ptr [rbp + 0xd30]
140a8c3f3: mov r15, qword ptr [rax + 0x30]
140a8c3f7: mov r13, qword ptr [rax + 0x40]
140a8c3fb: cmp r15, qword ptr [rax + 0x20]
140a8c3ff: jne 0x140a8c2e0
140a8c405: mov rcx, qword ptr [rbp + 0xd00]
140a8c40c: lea rdx, [rip + 0x21add35]
140a8c413: call 0x140562630 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
140a8c418: jmp 0x140a8c2e0
140a8c41d: mov rax, qword ptr [rbp + 0xd48]
140a8c424: mov r8, qword ptr [rax + 0x30]
140a8c428: mov rcx, qword ptr [rax + 0x48]
140a8c42c: mov qword ptr [rbp + 0xd30], rcx
140a8c433: movdqa xmm0, xmmword ptr [r8]
140a8c438: pmovmskb ecx, xmm0
140a8c43c: not ecx
140a8c43e: mov qword ptr [rbp + 0xd00], rcx
140a8c445: lea r13, [r8 + 0x10]
140a8c449: mov r12, qword ptr [rax + 0x60]
140a8c44d: movdqa xmm0, xmmword ptr [r12]
140a8c453: pmovmskb r15d, xmm0
140a8c458: not r15d
140a8c45b: mov rcx, qword ptr [rbp + 0xea0]
140a8c462: mov rcx, qword ptr [rcx]
140a8c465: mov qword ptr [rbp + 0xc18], rcx
140a8c46c: mov rcx, qword ptr [rbp + 0xeb8]
140a8c473: mov rcx, qword ptr [rcx]
140a8c476: mov qword ptr [rbp + 0xb60], rcx
140a8c47d: mov rcx, qword ptr [rbp + 0xeb0]
140a8c484: mov rcx, qword ptr [rcx]
140a8c487: mov qword ptr [rbp + 0xb68], rcx
140a8c48e: mov rcx, qword ptr [rbp + 0xea8]
140a8c495: mov rcx, qword ptr [rcx]
140a8c498: mov qword ptr [rbp + 0xb78], rcx
140a8c49f: mov rdx, qword ptr [rbp + 0xe78]
140a8c4a6: mov ecx, dword ptr [rdx + 0x1c]
140a8c4a9: mov dword ptr [rbp + 0xc60], ecx
140a8c4af: mov rcx, qword ptr [rdx]
140a8c4b2: mov rdx, qword ptr [rdx + 0x10]
140a8c4b6: mov qword ptr [rbp + 0xb58], rdx
140a8c4bd: pcmpeqd xmm7, xmm7
140a8c4c1: xorps xmm8, xmm8
140a8c4c5: movss xmm6, dword ptr [rip + 0x1e993a3]
140a8c4cd: movss xmm10, dword ptr [rip + 0x1ea812a]
140a8c4d6: movss xmm11, dword ptr [rip + 0x1ea028d]
140a8c4df: movss xmm15, dword ptr [rip + 0x1e993f0]
140a8c4e8: movss xmm14, dword ptr [rip + 0x1f40b73]
140a8c4f1: lea r14, [r12 + 0x10]
140a8c4f6: mov rax, qword ptr [rax + 0x78]
140a8c4fa: mov qword ptr [rbp + 0xd10], rax
140a8c501: mov qword ptr [rbp + 0xc10], rcx
140a8c508: lea rax, [rcx + 0x20]
140a8c50c: mov qword ptr [rbp + 0xb70], rax
140a8c513: test r8, r8
140a8c516: je 0x140a8c598
140a8c51c: cmp qword ptr [rbp + 0xd30], 0
140a8c524: je 0x140a8c58d
140a8c526: mov rcx, qword ptr [rbp + 0xd00]
140a8c52d: test cx, cx
140a8c530: je 0x140a8c540
140a8c532: lea edx, [rcx - 1]
140a8c535: and edx, ecx
140a8c537: jmp 0x140a8c567
140a8c539: nop dword ptr [rax]
140a8c540: movdqa xmm0, xmmword ptr [r13]
140a8c546: pmovmskb ecx, xmm0
140a8c54a: add r8, -0x80
140a8c54e: add r13, 0x10
140a8c552: cmp ecx, 0xffff
140a8c558: je 0x140a8c540
140a8c55a: mov eax, 0xfffffffe
140a8c55f: sub eax, ecx
140a8c561: not ecx
140a8c563: mov edx, ecx
140a8c565: and edx, eax
140a8c567: dec qword ptr [rbp + 0xd30]
140a8c56e: mov al, 1
140a8c570: mov qword ptr [rbp + 0xd38], rax
140a8c577: mov rax, r8
140a8c57a: mov qword ptr [rbp + 0xd20], r8
140a8c581: mov qword ptr [rbp + 0xd00], rdx
140a8c588: jmp 0x140a8c621
140a8c58d: mov qword ptr [rbp + 0xd30], 0
140a8c598: mov rdi, qword ptr [rbp + 0xd48]
140a8c59f: test r12, r12
140a8c5a2: je 0x140a8ddbb
140a8c5a8: cmp qword ptr [rbp + 0xd10], 0
140a8c5b0: je 0x140a8ddbb
140a8c5b6: test r15w, r15w
140a8c5ba: je 0x140a8c5d0
140a8c5bc: lea edx, [r15 - 1]
140a8c5c0: and edx, r15d
140a8c5c3: jmp 0x140a8c5fb
140a8c5c5: nop word ptr cs:[rax + rax]
140a8c5d0: movdqa xmm0, xmmword ptr [r14]
140a8c5d5: pmovmskb r15d, xmm0
140a8c5da: add r12, -0x80
140a8c5de: add r14, 0x10
140a8c5e2: cmp r15d, 0xffff
140a8c5e9: je 0x140a8c5d0
140a8c5eb: mov eax, 0xfffffffe
140a8c5f0: sub eax, r15d
140a8c5f3: not r15d
140a8c5f6: mov edx, r15d
140a8c5f9: and edx, eax
140a8c5fb: dec qword ptr [rbp + 0xd10]
140a8c602: mov qword ptr [rbp + 0xd20], 0
140a8c60d: mov ecx, r15d
140a8c610: mov rax, r12
140a8c613: mov r15d, edx
140a8c616: mov qword ptr [rbp + 0xd38], 0
140a8c621: tzcnt ecx, ecx
140a8c625: shl ecx, 3
140a8c628: sub rax, rcx
140a8c62b: mov rsi, qword ptr [rax - 8]
140a8c62f: mov qword ptr [rbp + 0xcb8], rsi
140a8c636: mov qword ptr [rbp + 0x840], rsi
140a8c63d: mov rax, qword ptr [rbp + 0xd48]
140a8c644: cmp qword ptr [rax + 0x18], 0
140a8c649: je 0x140a8dd86
140a8c64f: mov rcx, qword ptr [rbp + 0xcc0]
140a8c656: lea rdx, [rbp + 0x840]
140a8c65d: call 0x1405b0d70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >
140a8c662: mov rcx, qword ptr [rbp + 0xd48]
140a8c669: mov rbx, qword ptr [rcx]
140a8c66c: mov rcx, qword ptr [rcx + 8]
140a8c670: mov rdx, rcx
140a8c673: and rdx, rax
140a8c676: shr rax, 0x39
140a8c67a: movd xmm0, eax
140a8c67e: punpcklbw xmm0, xmm0
140a8c682: pshuflw xmm0, xmm0, 0
140a8c687: pshufd xmm0, xmm0, 0
140a8c68c: lea rax, [rbx - 0x78]
140a8c690: xor r8d, r8d
140a8c693: movdqu xmm1, xmmword ptr [rbx + rdx]
140a8c698: movdqa xmm2, xmm1
140a8c69c: pcmpeqb xmm2, xmm0
140a8c6a0: pmovmskb r9d, xmm2
140a8c6a5: test r9d, r9d
140a8c6a8: je 0x140a8c6cf
140a8c6aa: tzcnt r10d, r9d
140a8c6af: add r10, rdx
140a8c6b2: and r10, rcx
140a8c6b5: neg r10
140a8c6b8: imul rdi, r10, 0x78
140a8c6bc: cmp qword ptr [rax + rdi], rsi
140a8c6c0: je 0x140a8c700
140a8c6c2: lea r10d, [r9 - 1]
140a8c6c6: and r10w, r9w
140a8c6ca: mov r9d, r10d
140a8c6cd: jne 0x140a8c6aa
140a8c6cf: pcmpeqb xmm1, xmm7
140a8c6d3: pmovmskb r9d, xmm1
140a8c6d8: test r9d, r9d
140a8c6db: jne 0x140a8dd86
140a8c6e1: add rdx, r8
140a8c6e4: add rdx, 0x10
140a8c6e8: add r8, 0x10
140a8c6ec: and rdx, rcx
140a8c6ef: jmp 0x140a8c693
140a8c6f1: nop word ptr cs:[rax + rax]
140a8c700: cmp qword ptr [rbx + rdi - 0x60], 2
140a8c706: mov r8, qword ptr [rbp + 0xd20]
140a8c70d: jb 0x140a8c513
140a8c713: movss xmm0, dword ptr [rbx + rdi - 0x40]
140a8c719: ucomiss xmm0, xmm8
140a8c71d: jbe 0x140a8c513
140a8c723: mov rax, qword ptr [rbp + 0xcb8]
140a8c72a: mov qword ptr [rbp + 0xd40], rax
140a8c731: mov eax, dword ptr [rbp + 0xc60]
140a8c737: mov rcx, qword ptr [rbp + 0xb58]
140a8c73e: mov dword ptr [rcx], eax
140a8c740: mov rax, qword ptr [rbp + 0xc10]
140a8c747: mov rsi, qword ptr [rax + 0x30]
140a8c74b: mov rcx, qword ptr [rax + 0x40]
140a8c74f: mov qword ptr [rbp + 0xd18], rcx
140a8c756: cmp rsi, qword ptr [rax + 0x20]
140a8c75a: jne 0x140a8c76f
140a8c75c: mov rcx, qword ptr [rbp + 0xb70]
140a8c763: lea rdx, [rip + 0x21ad9de]
140a8c76a: call 0x140562630 ; alloc::raw_vec::RawVec::grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
140a8c76f: mov rdx, qword ptr [rbp + 0xc10]
140a8c776: mov rax, qword ptr [rdx + 0x28]
140a8c77a: mov rcx, rsi
140a8c77d: shl rcx, 4
140a8c781: mov r8, qword ptr [rbp + 0xd18]
140a8c788: mov qword ptr [rax + rcx], r8
140a8c78c: mov r8, qword ptr [rbp + 0xd40]
140a8c793: mov qword ptr [rax + rcx + 8], r8
140a8c798: inc rsi
140a8c79b: mov qword ptr [rdx + 0x30], rsi
140a8c79f: inc qword ptr [rdx + 0x40]
140a8c7a3: mov rcx, qword ptr [rbp + 0xce8]
140a8c7aa: lea rdx, [rbp + 0xcb8]
140a8c7b1: call 0x1405b0d70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >
140a8c7b6: lea rcx, [rbp + 0x840]
140a8c7bd: mov rdx, qword ptr [rbp + 0xcf8]
140a8c7c4: mov r8, rax
140a8c7c7: lea r9, [rbp + 0xcb8]
140a8c7ce: call 0x140b772b0 ; hashbrown::raw::RawTable::remove_entry<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState> >
140a8c7d3: xor eax, eax
140a8c7d5: cmp rax, qword ptr [rbp + 0x848]
140a8c7dc: jno 0x140a8c7fc
140a8c7de: movabs rax, 0x8000000000000000
140a8c7e8: mov qword ptr [rbp - 0x40], rax
140a8c7ec: mov r10b, 1
140a8c7ef: mov qword ptr [rbp + 0xcc8], 0
140a8c7fa: jmp 0x140a8c82b
140a8c7fc: mov r8d, 0x2a0
140a8c802: lea rsi, [rbp - 0x40]
140a8c806: mov rcx, rsi
140a8c809: lea rdx, [rbp + 0x848]
140a8c810: call 0x142923c00
140a8c815: xor eax, eax
140a8c817: cmp rax, qword ptr [rbp - 0x40]
140a8c81b: jno 0x140a8c838
140a8c81d: mov qword ptr [rbp + 0xcc8], 0
140a8c828: mov r10b, 1
140a8c82b: mov al, 1
140a8c82d: mov dword ptr [rbp + 0xd28], eax
140a8c833: jmp 0x140a8cad6
140a8c838: mov r8d, 0x2a0
140a8c83e: lea rcx, [rbp + 0x840]
140a8c845: mov rdx, rsi
140a8c848: call 0x142923c00
140a8c84d: cmp byte ptr [rbp + 0xd38], 0
140a8c854: jne 0x140a8e0b0
140a8c85a: movzx eax, byte ptr [rbp + 0x1b0]
140a8c861: mov byte ptr [rbp + 0xd2f], al
140a8c867: lea rdx, [rbp + 0x1b1]
140a8c86e: mov eax, dword ptr [rdx]
140a8c870: mov ecx, dword ptr [rdx + 3]
140a8c873: mov dword ptr [rbp + 0xc93], ecx
140a8c879: mov dword ptr [rbp + 0xc90], eax
140a8c87f: mov rax, qword ptr [rbp + 0x1b8]
140a8c886: mov qword ptr [rbp + 0xc30], rax
140a8c88d: mov rax, qword ptr [rbp + 0x1c0]
140a8c894: mov qword ptr [rbp + 0xc38], rax
140a8c89b: movups xmm0, xmmword ptr [rdx + 0x17]
140a8c89f: movaps xmmword ptr [rbp + 0x7b0], xmm0
140a8c8a6: mov rax, qword ptr [rdx + 0x57]
140a8c8aa: mov qword ptr [rbp + 0x830], rax
140a8c8b1: movups xmm0, xmmword ptr [rdx + 0x27]
140a8c8b5: movdqu xmm1, xmmword ptr [rdx + 0x37]
140a8c8ba: movdqu xmm2, xmmword ptr [rdx + 0x47]
140a8c8bf: movdqa xmmword ptr [rbp + 0x820], xmm2
140a8c8c7: movdqa xmmword ptr [rbp + 0x810], xmm1
140a8c8cf: movaps xmmword ptr [rbp + 0x800], xmm0
140a8c8d6: mov eax, dword ptr [rbp + 0x210]
140a8c8dc: mov dword ptr [rbp + 0xc6c], eax
140a8c8e2: mov eax, dword ptr [rbp + 0x214]
140a8c8e8: mov dword ptr [rbp + 0xc64], eax
140a8c8ee: mov rdx, qword ptr [rbp + 0x840]
140a8c8f5: test rdx, rdx
140a8c8f8: je 0x140a8c910
140a8c8fa: mov rcx, qword ptr [rbp + 0x848]
140a8c901: shl rdx, 3
140a8c905: mov r8d, 4
140a8c90b: call 0x140613c20
140a8c910: mov rdx, qword ptr [rbp + 0x858]
140a8c917: test rdx, rdx
140a8c91a: je 0x140a8c932
140a8c91c: mov rcx, qword ptr [rbp + 0x860]
140a8c923: shl rdx, 2
140a8c927: mov r8d, 4
140a8c92d: call 0x140613c20
140a8c932: mov rax, qword ptr [rbp + 0x878]
140a8c939: test rax, rax
140a8c93c: je 0x140a8c958
140a8c93e: mov rcx, qword ptr [rbp + 0x880]
140a8c945: shl rax, 2
140a8c949: lea rdx, [rax + rax*2]
140a8c94d: mov r8d, 4
140a8c953: call 0x140613c20
140a8c958: mov rdx, qword ptr [rbp + 0x890]
140a8c95f: test rdx, rdx
140a8c962: je 0x140a8c97a
140a8c964: mov rcx, qword ptr [rbp + 0x898]
140a8c96b: shl rdx, 2
140a8c96f: mov r8d, 4
140a8c975: call 0x140613c20
140a8c97a: mov rdx, qword ptr [rbp + 0x8b0]
140a8c981: test rdx, rdx
140a8c984: je 0x140a8c99c
140a8c986: mov rcx, qword ptr [rbp + 0x8b8]
140a8c98d: shl rdx, 2
140a8c991: mov r8d, 4
140a8c997: call 0x140613c20
140a8c99c: mov rax, qword ptr [rbp + 0x8c8]
140a8c9a3: test rax, rax
140a8c9a6: je 0x140a8c9c2
140a8c9a8: mov rcx, qword ptr [rbp + 0x8d0]
140a8c9af: shl rax, 2
140a8c9b3: lea rdx, [rax + rax*2]
140a8c9b7: mov r8d, 4
140a8c9bd: call 0x140613c20
140a8c9c2: mov rdx, qword ptr [rbp + 0x8e0]
140a8c9c9: test rdx, rdx
140a8c9cc: je 0x140a8c9e4
140a8c9ce: mov rcx, qword ptr [rbp + 0x8e8]
140a8c9d5: shl rdx, 2
140a8c9d9: mov r8d, 4
140a8c9df: call 0x140613c20
140a8c9e4: mov rdx, qword ptr [rbp + 0x900]
140a8c9eb: test rdx, rdx
140a8c9ee: je 0x140a8ca06
140a8c9f0: mov rcx, qword ptr [rbp + 0x908]
140a8c9f7: shl rdx, 2
140a8c9fb: mov r8d, 4
140a8ca01: call 0x140613c20
140a8ca06: lea rcx, [rbp + 0x958]
140a8ca0d: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8ca12: lea rcx, [rbp + 0x968]
140a8ca19: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8ca1e: lea rcx, [rbp + 0x978]
140a8ca25: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8ca2a: lea rcx, [rbp + 0x988]
140a8ca31: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8ca36: lea rcx, [rbp + 0x998]
140a8ca3d: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8ca42: lea rcx, [rbp + 0x9a8]
140a8ca49: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8ca4e: lea rcx, [rbp + 0x9b8]
140a8ca55: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8ca5a: mov rdx, qword ptr [rbp + 0x918]
140a8ca61: test rdx, rdx
140a8ca64: je 0x140a8ca7c
140a8ca66: mov rcx, qword ptr [rbp + 0x920]
140a8ca6d: shl rdx, 2
140a8ca71: mov r8d, 4
140a8ca77: call 0x140613c20
140a8ca7c: mov rdx, qword ptr [rbp + 0x930]
140a8ca83: test rdx, rdx
140a8ca86: je 0x140a8ca9e
140a8ca88: mov rcx, qword ptr [rbp + 0x938]
140a8ca8f: shl rdx, 2
140a8ca93: mov r8d, 4
140a8ca99: call 0x140613c20
140a8ca9e: mov rdx, qword ptr [rbp + 0x9d0]
140a8caa5: test rdx, rdx
140a8caa8: je 0x140a8cac0
140a8caaa: mov rcx, qword ptr [rbp + 0x9d8]
140a8cab1: shl rdx, 3
140a8cab5: mov r8d, 4
140a8cabb: call 0x140613c20
140a8cac0: mov al, 1
140a8cac2: mov qword ptr [rbp + 0xcc8], rax
140a8cac9: xor r10d, r10d
140a8cacc: mov dword ptr [rbp + 0xd28], 0
140a8cad6: lea rax, [rbx + rdi]
140a8cada: mov qword ptr [rbp + 0xd40], rax
140a8cae1: lea r11, [rbx + rdi]
140a8cae5: add r11, -0x70
140a8cae9: mov eax, dword ptr [rbp + 0xc90]
140a8caef: mov ecx, dword ptr [rbp + 0xc93]
140a8caf5: mov dword ptr [rbp + 0xc88], eax
140a8cafb: mov dword ptr [rbp + 0xc8b], ecx
140a8cb01: movaps xmm0, xmmword ptr [rbp + 0x7b0]
140a8cb08: movaps xmmword ptr [rbp + 0x7a0], xmm0
140a8cb0f: mov rax, qword ptr [rbp + 0x830]
140a8cb16: mov qword ptr [rbp + 0x7f0], rax
140a8cb1d: movaps xmm0, xmmword ptr [rbp + 0x800]
140a8cb24: movaps xmm1, xmmword ptr [rbp + 0x810]
140a8cb2b: movaps xmm2, xmmword ptr [rbp + 0x820]
140a8cb32: movaps xmmword ptr [rbp + 0x7e0], xmm2
140a8cb39: movaps xmmword ptr [rbp + 0x7d0], xmm1
140a8cb40: movaps xmmword ptr [rbp + 0x7c0], xmm0
140a8cb47: mov rax, qword ptr [rbp + 0xcb8]
140a8cb4e: mov rdx, qword ptr [rbp + 0xb60]
140a8cb55: mov rcx, qword ptr [rdx + 8]
140a8cb59: imul rdx, qword ptr [rdx + 0x10], 0x98
140a8cb61: mov r9, rdx
140a8cb64: test rdx, rdx
140a8cb67: je 0x140a8cb80
140a8cb69: lea r8, [rcx + 0x98]
140a8cb70: lea rdx, [r9 - 0x98]
140a8cb77: cmp qword ptr [rcx + 0x70], rax
140a8cb7b: mov rcx, r8
140a8cb7e: jne 0x140a8cb61
140a8cb80: mov qword ptr [rbp + 0xb88], r9
140a8cb87: mov rax, qword ptr [rbp + 0xd40]
140a8cb8e: mov rdi, qword ptr [rax - 0x60]
140a8cb92: lea r9, [rdi*8]
140a8cb9a: mov rax, rdi
140a8cb9d: shr rax, 0x3d
140a8cba1: mov dword ptr [rbp + 0xca8], r10d
140a8cba8: jne 0x140a8e1a3
140a8cbae: mov ecx, 0
140a8cbb3: movabs rax, 0x7ffffffffffffffc
140a8cbbd: cmp r9, rax
140a8cbc0: ja 0x140a8e1a5
140a8cbc6: mov rax, qword ptr [rbp + 0xd40]
140a8cbcd: mov rsi, qword ptr [rax - 0x68]
140a8cbd1: test r9, r9
140a8cbd4: mov qword ptr [rbp + 0xc98], r11
140a8cbdb: je 0x140a8cc16
140a8cbdd: movzx eax, byte ptr [rip + 0x2ba8025]
140a8cbe4: mov edx, 4
140a8cbe9: mov rcx, r9
140a8cbec: mov qword ptr [rbp + 0xd18], r9
140a8cbf3: call 0x140613c10
140a8cbf8: mov ecx, 4
140a8cbfd: mov r9, qword ptr [rbp + 0xd18]
140a8cc04: test rax, rax
140a8cc07: je 0x140a8e1a5
140a8cc0d: mov qword ptr [rbp + 0xd38], rdi
140a8cc14: jmp 0x140a8cc26
140a8cc16: mov eax, 4
140a8cc1b: mov qword ptr [rbp + 0xd38], 0
140a8cc26: mov qword ptr [rbp + 0xd18], rax
140a8cc2d: mov rcx, rax
140a8cc30: mov rdx, rsi
140a8cc33: mov r8, r9
140a8cc36: call 0x142923c00
140a8cc3b: mov rax, qword ptr [rbp + 0xd40]
140a8cc42: mov rbx, qword ptr [rax - 0x48]
140a8cc46: lea r9, [rbx*4]
140a8cc4e: mov rax, rbx
140a8cc51: shr rax, 0x3e
140a8cc55: jne 0x140a8e1b6
140a8cc5b: mov ecx, 0
140a8cc60: movabs rax, 0x7ffffffffffffffc
140a8cc6a: cmp r9, rax
140a8cc6d: ja 0x140a8e1b8
140a8cc73: mov rax, qword ptr [rbp + 0xd40]
140a8cc7a: mov rsi, qword ptr [rax - 0x50]
140a8cc7e: test r9, r9
140a8cc81: je 0x140a8ccbc
140a8cc83: movzx eax, byte ptr [rip + 0x2ba7f7f]
140a8cc8a: mov edx, 4
140a8cc8f: mov rcx, r9
140a8cc92: mov qword ptr [rbp + 0xcd0], r9
140a8cc99: call 0x140613c10
140a8cc9e: mov ecx, 4
140a8cca3: mov r9, qword ptr [rbp + 0xcd0]
140a8ccaa: test rax, rax
140a8ccad: je 0x140a8e1b8
140a8ccb3: mov qword ptr [rbp + 0xd08], rbx
140a8ccba: jmp 0x140a8cccc
140a8ccbc: mov eax, 4
140a8ccc1: mov qword ptr [rbp + 0xd08], 0
140a8cccc: mov rcx, rax
140a8cccf: mov rdx, rsi
140a8ccd2: mov r8, r9
140a8ccd5: mov rsi, rax
140a8ccd8: call 0x142923c00
140a8ccdd: mov rax, qword ptr [rbp + 0xd40]
140a8cce4: movd xmm7, dword ptr [rax - 0x40]
140a8cce9: mov rcx, qword ptr [rbp + 0xd38]
140a8ccf0: mov qword ptr [rbp + 0x840], rcx
140a8ccf7: mov rcx, qword ptr [rbp + 0xd18]
140a8ccfe: mov qword ptr [rbp + 0x848], rcx
140a8cd05: mov qword ptr [rbp + 0x850], rdi
140a8cd0c: mov rcx, qword ptr [rbp + 0xd08]
140a8cd13: mov qword ptr [rbp + 0x858], rcx
140a8cd1a: mov qword ptr [rbp + 0x860], rsi
140a8cd21: mov qword ptr [rbp + 0x868], rbx
140a8cd28: movd dword ptr [rbp + 0x870], xmm7
140a8cd30: mov eax, dword ptr [rax - 0x38]
140a8cd33: mov qword ptr [rbp + 0xcd0], rax
140a8cd3a: cmp eax, 1
140a8cd3d: sete r8b
140a8cd41: mov rax, qword ptr [rbp + 0xc98]
140a8cd48: mov qword ptr [rsp + 0x28], rax
140a8cd4d: mov rax, qword ptr [rbp + 0xce0]
140a8cd54: mov qword ptr [rsp + 0x20], rax
140a8cd59: lea rcx, [rbp + 0x260]
140a8cd60: lea rdx, [rbp + 0x840]
140a8cd67: movaps xmm3, xmm6
140a8cd6a: call 0x14146bb70 ; utils::curve::Curve::crashy_resample_remap_per_segment_2<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,country_core::systems::wall::wall_state::publish_public_wall_state::process::closure_env$0>
140a8cd6f: mov rdx, qword ptr [rbp + 0x840]
140a8cd76: test rdx, rdx
140a8cd79: je 0x140a8cd91
140a8cd7b: mov rcx, qword ptr [rbp + 0x848]
140a8cd82: shl rdx, 3
140a8cd86: mov r8d, 4
140a8cd8c: call 0x140613c20
140a8cd91: mov rdx, qword ptr [rbp + 0x858]
140a8cd98: test rdx, rdx
140a8cd9b: je 0x140a8cdb3
140a8cd9d: mov rcx, qword ptr [rbp + 0x860]
140a8cda4: shl rdx, 2
140a8cda8: mov r8d, 4
140a8cdae: call 0x140613c20
140a8cdb3: mov rax, qword ptr [rbp + 0x268]
140a8cdba: mov qword ptr [rbp + 0xd08], rax
140a8cdc1: mov rbx, qword ptr [rbp + 0x270]
140a8cdc8: lea rcx, [rbx*4]
140a8cdd0: test rbx, rbx
140a8cdd3: mov qword ptr [rbp + 0xd18], rbx
140a8cdda: mov qword ptr [rbp + 0xcd8], rcx
140a8cde1: je 0x140a8ce4c
140a8cde3: movzx eax, byte ptr [rip + 0x2ba7e1f]
140a8cdea: mov edx, 4
140a8cdef: call 0x140613c10
140a8cdf4: mov qword ptr [rbp + 0xd38], rax
140a8cdfb: test rax, rax
140a8cdfe: je 0x140a8e189
140a8ce04: mov rax, qword ptr [rbp + 0xd08]
140a8ce0b: lea rsi, [rax + 8]
140a8ce0f: xor edi, edi
140a8ce11: movss xmm1, dword ptr [rsi - 8]
140a8ce16: movss xmm9, dword ptr [rsi - 4]
140a8ce1c: movss xmm2, dword ptr [rsi]
140a8ce20: mov rcx, qword ptr [rbp + 0xce0]
140a8ce27: call 0x1409e9640 ; country_core::resources::terrain_height::TerrainHeightsData::sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>
140a8ce2c: subss xmm9, xmm0
140a8ce31: mov rax, qword ptr [rbp + 0xd38]
140a8ce38: movss dword ptr [rax + rdi*4], xmm9
140a8ce3e: inc rdi
140a8ce41: add rsi, 0xc
140a8ce45: cmp rbx, rdi
140a8ce48: jne 0x140a8ce11
140a8ce4a: jmp 0x140a8ce58
140a8ce4c: mov eax, 4
140a8ce51: mov qword ptr [rbp + 0xd38], rax
140a8ce58: mov rax, qword ptr [rbp + 0xd40]
140a8ce5f: movss xmm12, dword ptr [rax - 0x1c]
140a8ce65: movzx eax, byte ptr [rax - 0xb]
140a8ce69: mov byte ptr [rbp + 0xc98], al
140a8ce6f: mov rcx, qword ptr [rbp + 0xc18]
140a8ce76: lea rdx, [rip + 0x21a143b]
140a8ce7d: call 0x140969720 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
140a8ce82: mov rsi, rax
140a8ce85: test rax, rax
140a8ce88: je 0x140a8e0de
140a8ce8e: lea rcx, [rip + 0x201096a]
140a8ce95: mov qword ptr [rbp + 0xc40], rcx
140a8ce9c: mov qword ptr [rbp + 0xc48], 0x3a
140a8cea7: mov rdi, rsi
140a8ceaa: add rdi, 0x10
140a8ceae: cmp qword ptr [rsi + 0x18], 0x3a
140a8ceb3: jne 0x140a8e04b
140a8ceb9: mov rdx, qword ptr [rdi]
140a8cebc: mov r8d, 0x3a
140a8cec2: call 0x142923bf0
140a8cec7: test eax, eax
140a8cec9: jne 0x140a8e04b
140a8cecf: mov rcx, qword ptr [rsi]
140a8ced2: mov rax, qword ptr [rsi + 8]
140a8ced6: call qword ptr [rax + 0x18]
140a8ced9: mov rdi, rax
140a8cedc: test rax, rax
140a8cedf: je 0x140a8e0de
140a8cee5: movzx eax, byte ptr [rip + 0x2ba7d1d]
140a8ceec: mov ecx, 0x20
140a8cef1: mov edx, 8
140a8cef6: call 0x140613c10
140a8cefb: test rax, rax
140a8cefe: je 0x140a8e100
140a8cf04: mov rdx, rax
140a8cf07: mov qword ptr [rax], 0
140a8cf0e: mov qword ptr [rax + 8], 4
140a8cf16: mov qword ptr [rax + 0x10], 0
140a8cf1e: mov byte ptr [rax + 0x18], 0
140a8cf22: mov rsi, qword ptr [rdi + 0x60]
140a8cf26: jmp 0x140a8cf2a
140a8cf28: pause 
140a8cf2a: mov rax, qword ptr [rsi + 8]
140a8cf2e: nop 
140a8cf30: cmp rax, -1
140a8cf34: je 0x140a8cf28
140a8cf36: test rax, rax
140a8cf39: js 0x140a8dfa3
140a8cf3f: lea rcx, [rax + 1]
140a8cf43: lock cmpxchg qword ptr [rsi + 8], rcx
140a8cf49: jne 0x140a8cf30
140a8cf4b: mov qword ptr [rbp + 0xb48], rsi
140a8cf52: mov qword ptr [rbp + 0xb90], rdx
140a8cf59: mov qword ptr [rbp + 0xb50], rdx
140a8cf60: movzx eax, byte ptr [rip + 0x2ba7ca2]
140a8cf67: mov ecx, 0x20
140a8cf6c: mov edx, 8
140a8cf71: call 0x140613c10
140a8cf76: test rax, rax
140a8cf79: je 0x140a8e114
140a8cf7f: mov rbx, rax
140a8cf82: mov qword ptr [rax], 0
140a8cf89: mov qword ptr [rax + 8], 4
140a8cf91: mov qword ptr [rax + 0x10], 0
140a8cf99: mov byte ptr [rax + 0x18], 0
140a8cf9d: mov rdx, qword ptr [rdi + 0x60]
140a8cfa1: jmp 0x140a8cfa5
140a8cfa3: pause 
140a8cfa5: mov rax, qword ptr [rdx + 8]
140a8cfa9: nop dword ptr [rax]
140a8cfb0: cmp rax, -1
140a8cfb4: je 0x140a8cfa3
140a8cfb6: test rax, rax
140a8cfb9: js 0x140a8dfbb
140a8cfbf: lea rcx, [rax + 1]
140a8cfc3: lock cmpxchg qword ptr [rdx + 8], rcx
140a8cfc9: jne 0x140a8cfb0
140a8cfcb: mov qword ptr [rbp + 0xb98], rdx
140a8cfd2: mov qword ptr [rbp + 0xb38], rdx
140a8cfd9: mov qword ptr [rbp + 0xb40], rbx
140a8cfe0: movzx eax, byte ptr [rip + 0x2ba7c22]
140a8cfe7: mov ecx, 0x20
140a8cfec: mov edx, 8
140a8cff1: call 0x140613c10
140a8cff6: test rax, rax
140a8cff9: je 0x140a8e128
140a8cfff: mov rdx, rax
140a8d002: mov qword ptr [rax], 0
140a8d009: mov qword ptr [rax + 8], 4
140a8d011: mov qword ptr [rax + 0x10], 0
140a8d019: mov byte ptr [rax + 0x18], 0
140a8d01d: mov r8, qword ptr [rdi + 0x60]
140a8d021: jmp 0x140a8d025
140a8d023: pause 
140a8d025: mov rax, qword ptr [r8 + 8]
140a8d029: nop dword ptr [rax]
140a8d030: cmp rax, -1
140a8d034: je 0x140a8d023
140a8d036: test rax, rax
140a8d039: js 0x140a8dfd3
140a8d03f: lea rcx, [rax + 1]
140a8d043: lock cmpxchg qword ptr [r8 + 8], rcx
140a8d049: jne 0x140a8d030
140a8d04b: mov qword ptr [rbp + 0xba8], r8
140a8d052: mov qword ptr [rbp + 0xb28], r8
140a8d059: mov qword ptr [rbp + 0xba0], rdx
140a8d060: mov qword ptr [rbp + 0xb30], rdx
140a8d067: movzx eax, byte ptr [rip + 0x2ba7b9b]
140a8d06e: mov ecx, 0x20
140a8d073: mov edx, 8
140a8d078: call 0x140613c10
140a8d07d: test rax, rax
140a8d080: je 0x140a8e13c
140a8d086: mov rdx, rax
140a8d089: mov qword ptr [rax], 0
140a8d090: mov qword ptr [rax + 8], 4
140a8d098: mov qword ptr [rax + 0x10], 0
140a8d0a0: mov byte ptr [rax + 0x18], 0
140a8d0a4: mov r8, qword ptr [rdi + 0x60]
140a8d0a8: jmp 0x140a8d0ac
140a8d0aa: pause 
140a8d0ac: mov rax, qword ptr [r8 + 8]
140a8d0b0: cmp rax, -1
140a8d0b4: je 0x140a8d0aa
140a8d0b6: test rax, rax
140a8d0b9: js 0x140a8dfeb
140a8d0bf: lea rcx, [rax + 1]
140a8d0c3: lock cmpxchg qword ptr [r8 + 8], rcx
140a8d0c9: jne 0x140a8d0b0
140a8d0cb: mov qword ptr [rbp + 0xbb8], r8
140a8d0d2: mov qword ptr [rbp + 0xb18], r8
140a8d0d9: mov qword ptr [rbp + 0xbb0], rdx
140a8d0e0: mov qword ptr [rbp + 0xb20], rdx
140a8d0e7: mov rcx, qword ptr [rbp + 0xc18]
140a8d0ee: lea rdx, [rip + 0x21a126b]
140a8d0f5: call 0x140969720 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
140a8d0fa: mov rdi, rax
140a8d0fd: test rax, rax
140a8d100: je 0x140a8e0ef
140a8d106: lea rcx, [rip + 0x2010d2c]
140a8d10d: mov qword ptr [rbp + 0xc40], rcx
140a8d114: mov qword ptr [rbp + 0xc48], 0x3d
140a8d11f: mov r8, rdi
140a8d122: add r8, 0x10
140a8d126: cmp qword ptr [rdi + 0x18], 0x3d
140a8d12b: jne 0x140a8e07f
140a8d131: mov rdx, qword ptr [r8]
140a8d134: mov qword ptr [rbp + 0xc78], r8
140a8d13b: mov r8d, 0x3d
140a8d141: call 0x142923bf0
140a8d146: mov r8, qword ptr [rbp + 0xc78]
140a8d14d: test eax, eax
140a8d14f: jne 0x140a8e07f
140a8d155: mov rcx, qword ptr [rdi]
140a8d158: mov rax, qword ptr [rdi + 8]
140a8d15c: call qword ptr [rax + 0x18]
140a8d15f: mov rdi, rax
140a8d162: test rax, rax
140a8d165: je 0x140a8e0ef
140a8d16b: mov qword ptr [rbp + 0xbc0], rbx
140a8d172: mov qword ptr [rbp + 0xc78], rsi
140a8d179: movzx eax, byte ptr [rip + 0x2ba7a89]
140a8d180: mov ecx, 0x20
140a8d185: mov edx, 8
140a8d18a: call 0x140613c10
140a8d18f: test rax, rax
140a8d192: mov rsi, qword ptr [rbp + 0xd18]
140a8d199: mov rbx, qword ptr [rbp + 0xd38]
140a8d1a0: je 0x140a8e150
140a8d1a6: mov rdx, rax
140a8d1a9: mov qword ptr [rax], 0
140a8d1b0: mov qword ptr [rax + 8], 4
140a8d1b8: mov qword ptr [rax + 0x10], 0
140a8d1c0: mov byte ptr [rax + 0x18], 0
140a8d1c4: mov r8, qword ptr [rdi + 0x60]
140a8d1c8: jmp 0x140a8d1cc
140a8d1ca: pause 
140a8d1cc: mov rax, qword ptr [r8 + 8]
140a8d1d0: cmp rax, -1
140a8d1d4: je 0x140a8d1ca
140a8d1d6: test rax, rax
140a8d1d9: js 0x140a8e003
140a8d1df: lea rcx, [rax + 1]
140a8d1e3: lock cmpxchg qword ptr [r8 + 8], rcx
140a8d1e9: jne 0x140a8d1d0
140a8d1eb: mov qword ptr [rbp + 0xbd0], r8
140a8d1f2: mov qword ptr [rbp + 0xb08], r8
140a8d1f9: mov qword ptr [rbp + 0xbc8], rdx
140a8d200: mov qword ptr [rbp + 0xb10], rdx
140a8d207: movzx eax, byte ptr [rip + 0x2ba79fb]
140a8d20e: mov ecx, 0x20
140a8d213: mov edx, 8
140a8d218: call 0x140613c10
140a8d21d: test rax, rax
140a8d220: je 0x140a8e164
140a8d226: mov rdx, rax
140a8d229: mov qword ptr [rax], 0
140a8d230: mov qword ptr [rax + 8], 4
140a8d238: mov qword ptr [rax + 0x10], 0
140a8d240: mov byte ptr [rax + 0x18], 0
140a8d244: mov r8, qword ptr [rdi + 0x60]
140a8d248: jmp 0x140a8d24c
140a8d24a: pause 
140a8d24c: mov rax, qword ptr [r8 + 8]
140a8d250: cmp rax, -1
140a8d254: je 0x140a8d24a
140a8d256: test rax, rax
140a8d259: js 0x140a8e01b
140a8d25f: lea rcx, [rax + 1]
140a8d263: lock cmpxchg qword ptr [r8 + 8], rcx
140a8d269: jne 0x140a8d250
140a8d26b: mov qword ptr [rbp + 0xbe0], r8
140a8d272: mov qword ptr [rbp + 0xaf8], r8
140a8d279: mov qword ptr [rbp + 0xbd8], rdx
140a8d280: mov qword ptr [rbp + 0xb00], rdx
140a8d287: movzx eax, byte ptr [rip + 0x2ba797b]
140a8d28e: mov ecx, 0x20
140a8d293: mov edx, 8
140a8d298: call 0x140613c10
140a8d29d: test rax, rax
140a8d2a0: je 0x140a8e178
140a8d2a6: mov r10, rax
140a8d2a9: mov qword ptr [rax], 0
140a8d2b0: mov qword ptr [rax + 8], 4
140a8d2b8: mov qword ptr [rax + 0x10], 0
140a8d2c0: mov byte ptr [rax + 0x18], 0
140a8d2c4: mov rdi, qword ptr [rdi + 0x60]
140a8d2c8: jmp 0x140a8d2cc
140a8d2ca: pause 
140a8d2cc: mov rax, qword ptr [rdi + 8]
140a8d2d0: cmp rax, -1
140a8d2d4: je 0x140a8d2ca
140a8d2d6: test rax, rax
140a8d2d9: js 0x140a8e033
140a8d2df: lea rcx, [rax + 1]
140a8d2e3: lock cmpxchg qword ptr [rdi + 8], rcx
140a8d2e9: jne 0x140a8d2d0
140a8d2eb: mov qword ptr [rbp + 0xae8], rdi
140a8d2f2: mov qword ptr [rbp + 0xaf0], r10
140a8d2f9: movaps xmm9, xmm10
140a8d2fd: test rsi, rsi
140a8d300: je 0x140a8d39d
140a8d306: cmp rsi, 1
140a8d30a: jne 0x140a8d314
140a8d30c: movaps xmm9, xmm10
140a8d310: xor eax, eax
140a8d312: jmp 0x140a8d373
140a8d314: mov rcx, rsi
140a8d317: and rcx, 0xfffffffffffffffe
140a8d31b: movaps xmm0, xmm10
140a8d31f: xor eax, eax
140a8d321: movaps xmm9, xmm10
140a8d325: movss xmm1, dword ptr [rbx + rax*4]
140a8d32a: movss xmm2, dword ptr [rbx + rax*4 + 4]
140a8d330: cmpunordss xmm9, xmm9
140a8d336: movaps xmm3, xmm9
140a8d33a: andps xmm3, xmm1
140a8d33d: minss xmm1, xmm0
140a8d341: andnps xmm9, xmm1
140a8d345: orps xmm9, xmm3
140a8d349: movaps xmm0, xmm2
140a8d34c: minss xmm0, xmm9
140a8d351: cmpunordss xmm9, xmm9
140a8d357: movaps xmm1, xmm9
140a8d35b: andnps xmm1, xmm0
140a8d35e: andps xmm9, xmm2
140a8d362: orps xmm9, xmm1
140a8d366: add rax, 2
140a8d36a: movaps xmm0, xmm9
140a8d36e: cmp rcx, rax
140a8d371: jne 0x140a8d325
140a8d373: test sil, 1
140a8d377: je 0x140a8d39d
140a8d379: movss xmm0, dword ptr [rbx + rax*4]
140a8d37e: movaps xmm1, xmm9
140a8d382: cmpunordss xmm1, xmm9
140a8d388: movaps xmm2, xmm1
140a8d38b: andps xmm2, xmm0
140a8d38e: minss xmm0, xmm9
140a8d393: andnps xmm1, xmm0
140a8d396: orps xmm1, xmm2
140a8d399: movaps xmm9, xmm1
140a8d39d: test rsi, rsi
140a8d3a0: je 0x140a8d41a
140a8d3a2: mov rax, rsi
140a8d3a5: dec rax
140a8d3a8: je 0x140a8d427
140a8d3aa: mov rdx, rsi
140a8d3ad: and rdx, 0xfffffffffffffffe
140a8d3b1: mov r9, qword ptr [rbp + 0xd08]
140a8d3b8: lea r8, [r9 + 0x10]
140a8d3bc: movaps xmm0, xmm11
140a8d3c0: xor ecx, ecx
140a8d3c2: movaps xmm10, xmm11
140a8d3c6: movss xmm1, dword ptr [r8 - 0xc]
140a8d3cc: movss xmm2, dword ptr [r8]
140a8d3d1: cmpunordss xmm10, xmm10
140a8d3d7: movaps xmm3, xmm10
140a8d3db: andps xmm3, xmm1
140a8d3de: minss xmm1, xmm0
140a8d3e2: andnps xmm10, xmm1
140a8d3e6: orps xmm10, xmm3
140a8d3ea: movaps xmm0, xmm2
140a8d3ed: minss xmm0, xmm10
140a8d3f2: cmpunordss xmm10, xmm10
140a8d3f8: movaps xmm1, xmm10
140a8d3fc: andnps xmm1, xmm0
140a8d3ff: andps xmm10, xmm2
140a8d403: orps xmm10, xmm1
140a8d407: add rcx, 2
140a8d40b: add r8, 0x18
140a8d40f: movaps xmm0, xmm10
140a8d413: cmp rdx, rcx
140a8d416: jne 0x140a8d3c6
140a8d418: jmp 0x140a8d434
140a8d41a: xorps xmm13, xmm13
140a8d41e: movaps xmm10, xmm11
140a8d422: jmp 0x140a8d505
140a8d427: movaps xmm10, xmm11
140a8d42b: xor ecx, ecx
140a8d42d: mov r9, qword ptr [rbp + 0xd08]
140a8d434: test sil, 1
140a8d438: je 0x140a8d464
140a8d43a: lea rcx, [rcx + rcx*2]
140a8d43e: movss xmm0, dword ptr [r9 + rcx*4 + 4]
140a8d445: movaps xmm1, xmm10
140a8d449: cmpunordss xmm1, xmm10
140a8d44f: movaps xmm2, xmm1
140a8d452: andps xmm2, xmm0
140a8d455: minss xmm0, xmm10
140a8d45a: andnps xmm1, xmm0
140a8d45d: orps xmm1, xmm2
140a8d460: movaps xmm10, xmm1
140a8d464: test rax, rax
140a8d467: je 0x140a8d4cf
140a8d469: mov rcx, rsi
140a8d46c: and rcx, 0xfffffffffffffffe
140a8d470: lea rdx, [r9 + 0x10]
140a8d474: xorps xmm0, xmm0
140a8d477: xor eax, eax
140a8d479: movaps xmm13, xmm0
140a8d47d: movss xmm1, dword ptr [rdx - 0xc]
140a8d482: movss xmm2, dword ptr [rdx]
140a8d486: cmpunordss xmm13, xmm13
140a8d48c: movaps xmm3, xmm13
140a8d490: andps xmm3, xmm1
140a8d493: maxss xmm1, xmm0
140a8d497: andnps xmm13, xmm1
140a8d49b: orps xmm13, xmm3
140a8d49f: movaps xmm0, xmm2
140a8d4a2: maxss xmm0, xmm13
140a8d4a7: cmpunordss xmm13, xmm13
140a8d4ad: movaps xmm1, xmm13
140a8d4b1: andnps xmm1, xmm0
140a8d4b4: andps xmm13, xmm2
140a8d4b8: orps xmm13, xmm1
140a8d4bc: add rax, 2
140a8d4c0: add rdx, 0x18
140a8d4c4: movaps xmm0, xmm13
140a8d4c8: cmp rcx, rax
140a8d4cb: jne 0x140a8d47d
140a8d4cd: jmp 0x140a8d4d5
140a8d4cf: xorps xmm13, xmm13
140a8d4d3: xor eax, eax
140a8d4d5: test sil, 1
140a8d4d9: je 0x140a8d505
140a8d4db: lea rax, [rax + rax*2]
140a8d4df: movss xmm0, dword ptr [r9 + rax*4 + 4]
140a8d4e6: movaps xmm1, xmm13
140a8d4ea: cmpunordss xmm1, xmm13
140a8d4f0: movaps xmm2, xmm1
140a8d4f3: andps xmm2, xmm0
140a8d4f6: maxss xmm0, xmm13
140a8d4fb: andnps xmm1, xmm0
140a8d4fe: orps xmm1, xmm2
140a8d501: movaps xmm13, xmm1
140a8d505: mov rax, qword ptr [rbp + 0xd40]
140a8d50c: mov rax, qword ptr [rax - 0x60]
140a8d510: lea rsi, [rax*8]
140a8d518: mov qword ptr [rbp + 0xd08], rax
140a8d51f: shr rax, 0x3d
140a8d523: jne 0x140a8e1c9
140a8d529: mov ecx, 0
140a8d52e: movabs rax, 0x7ffffffffffffffc
140a8d538: cmp rsi, rax
140a8d53b: ja 0x140a8e1cb
140a8d541: mov rax, qword ptr [rbp + 0xd40]
140a8d548: mov rbx, qword ptr [rax - 0x68]
140a8d54c: test rsi, rsi
140a8d54f: mov qword ptr [rbp + 0xbe8], r10
140a8d556: je 0x140a8d58a
140a8d558: movzx eax, byte ptr [rip + 0x2ba76aa]
140a8d55f: mov edx, 4
140a8d564: mov rcx, rsi
140a8d567: call 0x140613c10
140a8d56c: mov ecx, 4
140a8d571: test rax, rax
140a8d574: je 0x140a8e1cb
140a8d57a: mov rcx, qword ptr [rbp + 0xd08]
140a8d581: mov qword ptr [rbp + 0xca0], rcx
140a8d588: jmp 0x140a8d59a
140a8d58a: mov eax, 4
140a8d58f: mov qword ptr [rbp + 0xca0], 0
140a8d59a: mov qword ptr [rbp + 0xc20], rax
140a8d5a1: mov rcx, rax
140a8d5a4: mov rdx, rbx
140a8d5a7: mov r8, rsi
140a8d5aa: call 0x142923c00
140a8d5af: mov rax, qword ptr [rbp + 0xd40]
140a8d5b6: mov rsi, qword ptr [rax - 0x48]
140a8d5ba: lea r9, [rsi*4]
140a8d5c2: mov rax, rsi
140a8d5c5: shr rax, 0x3e
140a8d5c9: jne 0x140a8e1dc
140a8d5cf: mov ecx, 0
140a8d5d4: movabs rax, 0x7ffffffffffffffc
140a8d5de: cmp r9, rax
140a8d5e1: ja 0x140a8e1de
140a8d5e7: mov rax, qword ptr [rbp + 0xd40]
140a8d5ee: mov rbx, qword ptr [rax - 0x50]
140a8d5f2: test r9, r9
140a8d5f5: je 0x140a8d636
140a8d5f7: movzx eax, byte ptr [rip + 0x2ba760b]
140a8d5fe: mov edx, 4
140a8d603: mov rcx, r9
140a8d606: mov qword ptr [rbp + 0xc80], rsi
140a8d60d: mov rsi, r9
140a8d610: call 0x140613c10
140a8d615: mov ecx, 4
140a8d61a: mov r9, rsi
140a8d61d: mov rsi, qword ptr [rbp + 0xc80]
140a8d624: test rax, rax
140a8d627: je 0x140a8e1de
140a8d62d: mov qword ptr [rbp + 0xca8], rsi
140a8d634: jmp 0x140a8d646
140a8d636: mov eax, 4
140a8d63b: mov qword ptr [rbp + 0xca8], 0
140a8d646: mov qword ptr [rbp + 0xc80], rax
140a8d64d: mov rcx, rax
140a8d650: mov rdx, rbx
140a8d653: mov r8, r9
140a8d656: call 0x142923c00
140a8d65b: mov rax, qword ptr [rbp + 0xd40]
140a8d662: movss xmm0, dword ptr [rax - 0x40]
140a8d667: mov rdx, qword ptr [rbp + 0xcd0]
140a8d66e: lea ecx, [rdx - 2]
140a8d671: cmp ecx, 2
140a8d674: mov eax, 2
140a8d679: cmovae ecx, eax
140a8d67c: test ecx, ecx
140a8d67e: je 0x140a8d6ab
140a8d680: cmp ecx, 2
140a8d683: jne 0x140a8d6a6
140a8d685: mov rcx, qword ptr [rbp + 0xd40]
140a8d68c: mov rax, qword ptr [rcx - 0x24]
140a8d690: mov qword ptr [rbp + 0xc50], rax
140a8d697: movups xmm1, xmmword ptr [rcx - 0x34]
140a8d69b: movaps xmmword ptr [rbp + 0xc40], xmm1
140a8d6a2: mov eax, edx
140a8d6a4: jmp 0x140a8d6ab
140a8d6a6: mov eax, 3
140a8d6ab: cmp byte ptr [rbp + 0xcc8], 0
140a8d6b2: je 0x140a8d74e
140a8d6b8: mov ecx, dword ptr [rbp + 0xc88]
140a8d6be: mov edx, dword ptr [rbp + 0xc8b]
140a8d6c4: mov dword ptr [rbp + 0xcb3], edx
140a8d6ca: mov dword ptr [rbp + 0xcb0], ecx
140a8d6d0: movaps xmm1, xmmword ptr [rbp + 0x7a0]
140a8d6d7: movaps xmmword ptr [rbp + 0xbf0], xmm1
140a8d6de: movaps xmm1, xmmword ptr [rbp + 0x7c0]
140a8d6e5: movaps xmm2, xmmword ptr [rbp + 0x7d0]
140a8d6ec: movaps xmm3, xmmword ptr [rbp + 0x7e0]
140a8d6f3: movaps xmmword ptr [rbp + 0x840], xmm1
140a8d6fa: movaps xmmword ptr [rbp + 0x850], xmm2
140a8d701: movaps xmmword ptr [rbp + 0x860], xmm3
140a8d708: mov rcx, qword ptr [rbp + 0x7f0]
140a8d70f: mov qword ptr [rbp + 0x870], rcx
140a8d716: movzx ecx, byte ptr [rbp + 0xd2f]
140a8d71d: mov qword ptr [rbp + 0xcd8], rcx
140a8d724: mov rcx, qword ptr [rbp + 0xc30]
140a8d72b: mov qword ptr [rbp + 0xc28], rcx
140a8d732: mov rcx, qword ptr [rbp + 0xc38]
140a8d739: mov qword ptr [rbp + 0xcd0], rcx
140a8d740: mov ecx, dword ptr [rbp + 0xc6c]
140a8d746: mov dword ptr [rbp + 0xcc8], ecx
140a8d74c: jmp 0x140a8d7a4
140a8d74e: xorps xmm1, xmm1
140a8d751: movaps xmmword ptr [rbp + 0x860], xmm1
140a8d758: movaps xmmword ptr [rbp + 0x850], xmm1
140a8d75f: movaps xmmword ptr [rbp + 0x840], xmm1
140a8d766: mov qword ptr [rbp + 0x870], 0
140a8d771: movaps xmmword ptr [rbp + 0xbf0], xmm1
140a8d778: mov qword ptr [rbp + 0xcd8], 0
140a8d783: mov ecx, 4
140a8d788: mov qword ptr [rbp + 0xcd0], rcx
140a8d78f: mov qword ptr [rbp + 0xc28], 0
140a8d79a: mov dword ptr [rbp + 0xcc8], 0
140a8d7a4: movaps xmm8, xmm15
140a8d7a8: movaps xmm3, xmm15
140a8d7ac: cmp byte ptr [rbp + 0xc98], 0
140a8d7b3: jne 0x140a8d7b8
140a8d7b5: movaps xmm3, xmm6
140a8d7b8: movaps xmm15, xmm6
140a8d7bc: movaps xmm6, xmm11
140a8d7c0: movaps xmm2, xmm14
140a8d7c4: jne 0x140a8d7ce
140a8d7c6: movss xmm2, dword ptr [rip + 0x1ff0632]
140a8d7ce: cmp qword ptr [rbp + 0xb88], 0
140a8d7d6: xorps xmm1, xmm1
140a8d7d9: lea r8, [rbp + 0x708]
140a8d7e0: movups xmmword ptr [r8 + 0x30], xmm1
140a8d7e5: movups xmmword ptr [r8 + 0x20], xmm1
140a8d7ea: movups xmmword ptr [r8 + 0x10], xmm1
140a8d7ef: movups xmmword ptr [r8], xmm1
140a8d7f3: mov qword ptr [r8 + 0x40], 0
140a8d7fb: mov rcx, qword ptr [rbp + 0xd18]
140a8d802: mov qword ptr [rbp + 0x570], rcx
140a8d809: mov rdx, qword ptr [rbp + 0xd38]
140a8d810: mov qword ptr [rbp + 0x578], rdx
140a8d817: mov qword ptr [rbp + 0x580], rcx
140a8d81e: mov r11, qword ptr [rbp + 0xd40]
140a8d825: movss xmm4, dword ptr [r11 - 0x18]
140a8d82b: movss xmm5, dword ptr [r11 - 0x14]
140a8d831: movd xmm1, dword ptr [r11 - 0x10]
140a8d837: movzx ecx, byte ptr [r11 - 0xc]
140a8d83c: movzx edx, byte ptr [r11 - 0xa]
140a8d841: movzx r10d, byte ptr [r11 - 9]
140a8d846: movzx r11d, word ptr [r11 - 8]
140a8d84b: mov r9, qword ptr [rbp + 0xca0]
140a8d852: mov qword ptr [rbp + 0x500], r9
140a8d859: mov r9, qword ptr [rbp + 0xc20]
140a8d860: mov qword ptr [rbp + 0x508], r9
140a8d867: mov r9, qword ptr [rbp + 0xd08]
140a8d86e: mov qword ptr [rbp + 0x510], r9
140a8d875: mov r9, qword ptr [rbp + 0xca8]
140a8d87c: mov qword ptr [rbp + 0x518], r9
140a8d883: mov r9, qword ptr [rbp + 0xc80]
140a8d88a: mov qword ptr [rbp + 0x520], r9
140a8d891: mov qword ptr [rbp + 0x528], rsi
140a8d898: movd dword ptr [rbp + 0x530], xmm7
140a8d8a0: mov rsi, qword ptr [rbp + 0x290]
140a8d8a7: mov qword ptr [r8 - 0x1a0], rsi
140a8d8ae: maxss xmm3, xmm12
140a8d8b3: minss xmm2, xmm3
140a8d8b7: movups xmm3, xmmword ptr [rbp + 0x260]
140a8d8be: movups xmm11, xmmword ptr [rbp + 0x270]
140a8d8c6: movups xmm12, xmmword ptr [rbp + 0x280]
140a8d8ce: movups xmmword ptr [r8 - 0x1b0], xmm12
140a8d8d6: movups xmmword ptr [r8 - 0x1c0], xmm11
140a8d8de: movups xmmword ptr [r8 - 0x1d0], xmm3
140a8d8e6: movss dword ptr [rbp + 0x774], xmm10
140a8d8ef: xorps xmm3, xmm3
140a8d8f2: movss dword ptr [rbp + 0x778], xmm13
140a8d8fb: movss dword ptr [rbp + 0x77c], xmm9
140a8d904: movss dword ptr [rbp + 0x780], xmm4
140a8d90c: movss dword ptr [rbp + 0x784], xmm2
140a8d914: movss dword ptr [rbp + 0x788], xmm5
140a8d91c: mov byte ptr [rbp + 0x794], cl
140a8d922: mov byte ptr [rbp + 0x795], dl
140a8d928: mov qword ptr [rbp + 0x588], 0
140a8d933: mov qword ptr [rbp + 0x590], 4
140a8d93e: movups xmmword ptr [r8 - 0x170], xmm3
140a8d946: mov qword ptr [rbp + 0x5a8], 4
140a8d951: mov qword ptr [rbp + 0x5b0], 0
140a8d95c: mov dword ptr [rbp + 0x5b8], 0
140a8d966: mov qword ptr [rbp + 0x5c0], 0
140a8d971: mov qword ptr [rbp + 0x5c8], 4
140a8d97c: mov qword ptr [rbp + 0x5d0], 0
140a8d987: movss dword ptr [rbp + 0x78c], xmm0
140a8d98f: mov dword ptr [rbp + 0x758], eax
140a8d995: mov rax, qword ptr [rbp + 0xc50]
140a8d99c: mov qword ptr [r8 + 0x64], rax
140a8d9a0: movaps xmm0, xmmword ptr [rbp + 0xc40]
140a8d9a7: movups xmmword ptr [r8 + 0x54], xmm0
140a8d9ac: mov rax, qword ptr [rbp + 0xc78]
140a8d9b3: mov qword ptr [rbp + 0x618], rax
140a8d9ba: mov rax, qword ptr [rbp + 0xb90]
140a8d9c1: mov qword ptr [rbp + 0x620], rax
140a8d9c8: mov rax, qword ptr [rbp + 0xbd0]
140a8d9cf: mov qword ptr [rbp + 0x628], rax
140a8d9d6: mov rax, qword ptr [rbp + 0xbc8]
140a8d9dd: mov qword ptr [rbp + 0x630], rax
140a8d9e4: mov rax, qword ptr [rbp + 0xbe0]
140a8d9eb: mov qword ptr [rbp + 0x638], rax
140a8d9f2: mov rax, qword ptr [rbp + 0xbd8]
140a8d9f9: mov qword ptr [rbp + 0x640], rax
140a8da00: mov rax, qword ptr [rbp + 0xb98]
140a8da07: mov qword ptr [rbp + 0x648], rax
140a8da0e: mov rax, qword ptr [rbp + 0xbc0]
140a8da15: mov qword ptr [rbp + 0x650], rax
140a8da1c: mov rax, qword ptr [rbp + 0xba8]
140a8da23: mov qword ptr [rbp + 0x658], rax
140a8da2a: mov rax, qword ptr [rbp + 0xba0]
140a8da31: mov qword ptr [rbp + 0x660], rax
140a8da38: mov rax, qword ptr [rbp + 0xbb8]
140a8da3f: mov qword ptr [rbp + 0x668], rax
140a8da46: mov rax, qword ptr [rbp + 0xbb0]
140a8da4d: mov qword ptr [rbp + 0x670], rax
140a8da54: mov qword ptr [rbp + 0x678], rdi
140a8da5b: mov rax, qword ptr [rbp + 0xbe8]
140a8da62: mov qword ptr [rbp + 0x680], rax
140a8da69: mov qword ptr [rbp + 0x5d8], 0
140a8da74: mov qword ptr [rbp + 0x5e0], 4
140a8da7f: movups xmmword ptr [r8 - 0x120], xmm3
140a8da87: mov qword ptr [rbp + 0x5f8], 4
140a8da92: mov qword ptr [rbp + 0x600], 0
140a8da9d: mov rax, qword ptr [rbp + 0xcd8]
140a8daa4: mov byte ptr [rbp + 0x688], al
140a8daaa: mov eax, dword ptr [rbp + 0xcb3]
140a8dab0: mov dword ptr [r8 - 0x7c], eax
140a8dab4: mov eax, dword ptr [rbp + 0xcb0]
140a8daba: mov dword ptr [r8 - 0x7f], eax
140a8dabe: mov rax, qword ptr [rbp + 0xc28]
140a8dac5: mov qword ptr [rbp + 0x690], rax
140a8dacc: mov rax, qword ptr [rbp + 0xcd0]
140a8dad3: mov qword ptr [rbp + 0x698], rax
140a8dada: movaps xmm0, xmmword ptr [rbp + 0xbf0]
140a8dae1: movups xmmword ptr [r8 - 0x68], xmm0
140a8dae6: mov rax, qword ptr [rbp + 0x870]
140a8daed: mov qword ptr [r8 - 0x28], rax
140a8daf1: movdqa xmm0, xmmword ptr [rbp + 0x840]
140a8daf9: movdqa xmm2, xmmword ptr [rbp + 0x850]
140a8db01: movaps xmm3, xmmword ptr [rbp + 0x860]
140a8db08: movups xmmword ptr [r8 - 0x38], xmm3
140a8db0d: movdqu xmmword ptr [r8 - 0x48], xmm2
140a8db13: movdqu xmmword ptr [r8 - 0x58], xmm0
140a8db19: mov eax, dword ptr [rbp + 0xcc8]
140a8db1f: mov dword ptr [rbp + 0x6e8], eax
140a8db25: mov eax, dword ptr [rbp + 0xc64]
140a8db2b: mov dword ptr [rbp + 0x6ec], eax
140a8db31: mov byte ptr [rbp + 0x6f0], 0
140a8db38: mov qword ptr [rbp + 0x6f8], 0
140a8db43: mov qword ptr [rbp + 0x700], 4
140a8db4e: mov dword ptr [rbp + 0x750], 0
140a8db58: mov dword ptr [rbp + 0x608], 0
140a8db62: mov dword ptr [rbp + 0x610], 0
140a8db6c: mov word ptr [rbp + 0x799], r11w
140a8db74: movzx eax, byte ptr [rbp + 0xc98]
140a8db7b: mov byte ptr [rbp + 0x796], al
140a8db81: setne byte ptr [rbp + 0x797]
140a8db88: movd dword ptr [rbp + 0x790], xmm1
140a8db90: mov byte ptr [rbp + 0x798], r10b
140a8db97: mov rax, qword ptr [rbp + 0xb68]
140a8db9e: cmp byte ptr [rax + 0xf4], 1
140a8dba5: jne 0x140a8dc47
140a8dbab: mov rbx, qword ptr [rbp + 0x548]
140a8dbb2: mov rax, rbx
140a8dbb5: mov ecx, 0xc
140a8dbba: mul rcx
140a8dbbd: mov r9, rax
140a8dbc0: mov rdi, qword ptr [rbp + 0xcf8]
140a8dbc7: jo 0x140a8e1ef
140a8dbcd: movaps xmm11, xmm6
140a8dbd1: mov ecx, 0
140a8dbd6: movabs rax, 0x7ffffffffffffffc
140a8dbe0: cmp r9, rax
140a8dbe3: movaps xmm6, xmm15
140a8dbe7: movss xmm10, dword ptr [rip + 0x1ea6a10]
140a8dbf0: movaps xmm15, xmm8
140a8dbf4: xorps xmm8, xmm8
140a8dbf8: pcmpeqd xmm7, xmm7
140a8dbfc: ja 0x140a8e1f1
140a8dc02: mov rsi, qword ptr [rbp + 0x540]
140a8dc09: test r9, r9
140a8dc0c: je 0x140a8dc70
140a8dc0e: movzx eax, byte ptr [rip + 0x2ba6ff4]
140a8dc15: mov edx, 4
140a8dc1a: mov rcx, r9
140a8dc1d: mov qword ptr [rbp + 0xd40], r9
140a8dc24: call 0x140613c10
140a8dc29: mov ecx, 4
140a8dc2e: mov r9, qword ptr [rbp + 0xd40]
140a8dc35: test rax, rax
140a8dc38: je 0x140a8e1f1
140a8dc3e: mov qword ptr [rbp + 0xd38], rbx
140a8dc45: jmp 0x140a8dc80
140a8dc47: mov rdi, qword ptr [rbp + 0xcf8]
140a8dc4e: movaps xmm11, xmm6
140a8dc52: movaps xmm6, xmm15
140a8dc56: movss xmm10, dword ptr [rip + 0x1ea69a1]
140a8dc5f: movaps xmm15, xmm8
140a8dc63: xorps xmm8, xmm8
140a8dc67: pcmpeqd xmm7, xmm7
140a8dc6b: jmp 0x140a8dd06
140a8dc70: mov eax, 4
140a8dc75: mov qword ptr [rbp + 0xd38], 0
140a8dc80: mov rcx, rax
140a8dc83: mov rdx, rsi
140a8dc86: mov r8, r9
140a8dc89: mov rsi, rax
140a8dc8c: call 0x142923c00
140a8dc91: mov ecx, dword ptr [rbp + 0xc68]
140a8dc97: and ecx, 0xffff0000
140a8dc9d: inc ecx
140a8dc9f: mov rax, qword ptr [rbp + 0xd38]
140a8dca6: mov qword ptr [rbp + 0x840], rax
140a8dcad: mov qword ptr [rbp + 0x848], rsi
140a8dcb4: mov qword ptr [rbp + 0x850], rbx
140a8dcbb: movabs rax, 0x8000000000000000
140a8dcc5: mov qword ptr [rbp + 0x858], rax
140a8dccc: mov dword ptr [rbp + 0x870], 1
140a8dcd6: mov dword ptr [rbp + 0x880], 0x40400000
140a8dce0: mov dword ptr [rbp + 0xc68], ecx
140a8dce6: mov dword ptr [rbp + 0x884], ecx
140a8dcec: mov byte ptr [rbp + 0xd56], 1
140a8dcf3: lea rcx, [rbp + 0x840]
140a8dcfa: mov rdx, qword ptr [rbp + 0xb78]
140a8dd01: call 0x140b04f70 ; country_core::utils::debug_viz::DebugLine::show
140a8dd06: mov r8d, 0x2a0
140a8dd0c: lea rsi, [rbp + 0x840]
140a8dd13: mov rcx, rsi
140a8dd16: lea rdx, [rbp + 0x500]
140a8dd1d: call 0x142923c00
140a8dd22: mov r8, qword ptr [rbp + 0xcb8]
140a8dd29: mov byte ptr [rbp + 0xd56], 0
140a8dd30: lea rcx, [rbp + 0x260]
140a8dd37: mov rdx, rdi
140a8dd3a: mov r9, rsi
140a8dd3d: call 0x14152a360 ; hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState,std::hash::random::RandomState,alloc::alloc::Global>
140a8dd42: xor eax, eax
140a8dd44: cmp rax, qword ptr [rbp + 0x260]
140a8dd4b: jo 0x140a8dd60
140a8dd4d: mov byte ptr [rbp + 0xd56], 0
140a8dd54: lea rcx, [rbp + 0x260]
140a8dd5b: call 0x140895600 ; core::ptr::drop_in_place<country_core::resources::walls::public_wall_state::PublicWallState>
140a8dd60: xor eax, eax
140a8dd62: cmp rax, qword ptr [rbp - 0x40]
140a8dd66: jo 0x140a8dd7a
140a8dd68: cmp byte ptr [rbp + 0xd28], 0
140a8dd6f: je 0x140a8dd7a
140a8dd71: lea rcx, [rbp - 0x40]
140a8dd75: call 0x140895600 ; core::ptr::drop_in_place<country_core::resources::walls::public_wall_state::PublicWallState>
140a8dd7a: mov r8, qword ptr [rbp + 0xd20]
140a8dd81: jmp 0x140a8c513
140a8dd86: lea rcx, [rip + 0x204b52b]
140a8dd8d: call 0x1428d9310 ; core::option::unwrap_failed
140a8dd92: jmp 0x140a8e207
140a8dd97: test rax, rax
140a8dd9a: jne 0x140a8ddaf
140a8dd9c: xor edx, edx
140a8dd9e: call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
140a8dda3: mov rcx, rax
140a8dda6: test rax, rax
140a8dda9: jne 0x140a8b7a6
140a8ddaf: lea rcx, [rip + 0x21accea]
140a8ddb6: call 0x1428d80d0 ; std::thread::local::panic_access_error
140a8ddbb: cmp qword ptr [rdi + 0x48], 0
140a8ddc0: je 0x140a8de00
140a8ddc2: mov rsi, qword ptr [rdi + 0x38]
140a8ddc6: test rsi, rsi
140a8ddc9: je 0x140a8ddda
140a8ddcb: mov rcx, qword ptr [rdi + 0x30]
140a8ddcf: lea r8, [rsi + 0x11]
140a8ddd3: mov dl, 0xff
140a8ddd5: call 0x142923c20
140a8ddda: mov qword ptr [rdi + 0x48], 0
140a8dde2: lea rax, [rsi + 1]
140a8dde6: mov rcx, rax
140a8dde9: shr rcx, 3
140a8dded: and rax, 0xfffffffffffffff8
140a8ddf1: sub rax, rcx
140a8ddf4: cmp rsi, 8
140a8ddf8: cmovb rax, rsi
140a8ddfc: mov qword ptr [rdi + 0x40], rax
140a8de00: cmp qword ptr [rdi + 0x78], 0
140a8de05: je 0x140a8de4b
140a8de07: mov rsi, qword ptr [rdi + 0x68]
140a8de0b: test rsi, rsi
140a8de0e: je 0x140a8de25
140a8de10: mov rax, qword ptr [rbp + 0xcf0]
140a8de17: mov rcx, qword ptr [rax]
140a8de1a: lea r8, [rsi + 0x11]
140a8de1e: mov dl, 0xff
140a8de20: call 0x142923c20
140a8de25: mov qword ptr [rdi + 0x78], 0
140a8de2d: lea rax, [rsi + 1]
140a8de31: mov rcx, rax
140a8de34: shr rcx, 3
140a8de38: and rax, 0xfffffffffffffff8
140a8de3c: sub rax, rcx
140a8de3f: cmp rsi, 8
140a8de43: cmovb rax, rsi
140a8de47: mov qword ptr [rdi + 0x70], rax
140a8de4b: cmp qword ptr [rdi + 0xa8], 0
140a8de53: je 0x140a8dea2
140a8de55: mov rsi, qword ptr [rdi + 0x98]
140a8de5c: test rsi, rsi
140a8de5f: je 0x140a8de76
140a8de61: mov rax, qword ptr [rbp + 0xb80]
140a8de68: mov rcx, qword ptr [rax]
140a8de6b: lea r8, [rsi + 0x11]
140a8de6f: mov dl, 0xff
140a8de71: call 0x142923c20
140a8de76: mov qword ptr [rdi + 0xa8], 0
140a8de81: lea rax, [rsi + 1]
140a8de85: mov rcx, rax
140a8de88: shr rcx, 3
140a8de8c: and rax, 0xfffffffffffffff8
140a8de90: sub rax, rcx
140a8de93: cmp rsi, 8
140a8de97: cmovb rax, rsi
140a8de9b: mov qword ptr [rdi + 0xa0], rax
140a8dea2: cmp qword ptr [rbp + 0xc00], 0
140a8deaa: je 0x140a8def7
140a8deac: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
140a8deb1: mov rcx, rax
140a8deb4: mov rax, qword ptr [rax]
140a8deb7: cmp rax, 1
140a8debb: jne 0x140a8df76
140a8dec1: add rcx, 8
140a8dec5: cmp qword ptr [rcx], 0
140a8dec9: jne 0x140a8df97
140a8decf: mov qword ptr [rcx], 0xffffffffffffffff
140a8ded6: mov qword ptr [rbp + 0xd48], rcx
140a8dedd: add rcx, 8
140a8dee1: mov rdx, qword ptr [rbp + 0xc08]
140a8dee8: call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
140a8deed: mov rax, qword ptr [rbp + 0xd48]
140a8def4: inc qword ptr [rax]
140a8def7: movaps xmm6, xmmword ptr [rbp + 0xd60]
140a8defe: movaps xmm7, xmmword ptr [rbp + 0xd70]
140a8df05: movaps xmm8, xmmword ptr [rbp + 0xd80]
140a8df0d: movaps xmm9, xmmword ptr [rbp + 0xd90]
140a8df15: movaps xmm10, xmmword ptr [rbp + 0xda0]
140a8df1d: movaps xmm11, xmmword ptr [rbp + 0xdb0]
140a8df25: movaps xmm12, xmmword ptr [rbp + 0xdc0]
140a8df2d: movaps xmm13, xmmword ptr [rbp + 0xdd0]
140a8df35: movaps xmm14, xmmword ptr [rbp + 0xde0]
140a8df3d: movaps xmm15, xmmword ptr [rbp + 0xdf0]
140a8df45: add rsp, 0xe88
140a8df4c: pop rbx
140a8df4d: pop rdi
140a8df4e: pop rsi
140a8df4f: pop r12
140a8df51: pop r13
140a8df53: pop r14
140a8df55: pop r15
140a8df57: pop rbp
140a8df58: ret 
140a8df59: lea rcx, [rip + 0x204c233]
140a8df60: lea r8, [rip + 0x204b3a1]
140a8df67: mov edx, 0x32
140a8df6c: call 0x1428d9430 ; core::panicking::panic
140a8df71: jmp 0x140a8e207
140a8df76: test rax, rax
140a8df79: jne 0x140a8ddaf
140a8df7f: xor edx, edx
140a8df81: call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
140a8df86: mov rcx, rax
140a8df89: test rax, rax
140a8df8c: jne 0x140a8dec5
140a8df92: jmp 0x140a8ddaf
140a8df97: lea rcx, [rip + 0x21acbd2]
140a8df9e: call 0x1428d9250 ; core::cell::panic_already_borrowed
140a8dfa3: lea rcx, [rip + 0x2063396]
140a8dfaa: lea rdx, [rip + 0x20633ef]
140a8dfb1: call 0x1428a9a00 ; alloc::sync::impl$21::downgrade::panic_cold_display<ref$<str$> >
140a8dfb6: jmp 0x140a8e207
140a8dfbb: lea rcx, [rip + 0x206337e]
140a8dfc2: lea rdx, [rip + 0x20633d7]
140a8dfc9: call 0x1428a9a00 ; alloc::sync::impl$21::downgrade::panic_cold_display<ref$<str$> >
140a8dfce: jmp 0x140a8e207
140a8dfd3: lea rcx, [rip + 0x2063366]
140a8dfda: lea rdx, [rip + 0x20633bf]
140a8dfe1: call 0x1428a9a00 ; alloc::sync::impl$21::downgrade::panic_cold_display<ref$<str$> >
140a8dfe6: jmp 0x140a8e207
140a8dfeb: lea rcx, [rip + 0x206334e]
140a8dff2: lea rdx, [rip + 0x20633a7]
140a8dff9: call 0x1428a9a00 ; alloc::sync::impl$21::downgrade::panic_cold_display<ref$<str$> >
140a8dffe: jmp 0x140a8e207
140a8e003: lea rcx, [rip + 0x2063336]
140a8e00a: lea rdx, [rip + 0x206338f]
140a8e011: call 0x1428a9a00 ; alloc::sync::impl$21::downgrade::panic_cold_display<ref$<str$> >
140a8e016: jmp 0x140a8e207
140a8e01b: lea rcx, [rip + 0x206331e]
140a8e022: lea rdx, [rip + 0x2063377]
140a8e029: call 0x1428a9a00 ; alloc::sync::impl$21::downgrade::panic_cold_display<ref$<str$> >
140a8e02e: jmp 0x140a8e207
140a8e033: lea rcx, [rip + 0x2063306]
140a8e03a: lea rdx, [rip + 0x206335f]
140a8e041: call 0x1428a9a00 ; alloc::sync::impl$21::downgrade::panic_cold_display<ref$<str$> >
140a8e046: jmp 0x140a8e207
140a8e04b: mov qword ptr [rbp + 0x840], 0
140a8e056: lea rax, [rip + 0x21a0123]
140a8e05d: mov qword ptr [rsp + 0x20], rax
140a8e062: lea rdx, [rbp + 0xc40]
140a8e069: lea r9, [rbp + 0x840]
140a8e070: xor ecx, ecx
140a8e072: mov r8, rdi
140a8e075: call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
140a8e07a: jmp 0x140a8e207
140a8e07f: mov qword ptr [rbp + 0x840], 0
140a8e08a: lea rax, [rip + 0x21a00ef]
140a8e091: mov qword ptr [rsp + 0x20], rax
140a8e096: lea rdx, [rbp + 0xc40]
140a8e09d: lea r9, [rbp + 0x840]
140a8e0a4: xor ecx, ecx
140a8e0a6: call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
140a8e0ab: jmp 0x140a8e207
140a8e0b0: lea rcx, [rip + 0x204b219]
140a8e0b7: lea r8, [rip + 0x204b232]
140a8e0be: mov edx, 0x1c
140a8e0c3: call 0x1428d9430 ; core::panicking::panic
140a8e0c8: jmp 0x140a8e207
140a8e0cd: lea rcx, [rip + 0x21ac9cc]
140a8e0d4: call 0x1428d80d0 ; std::thread::local::panic_access_error
140a8e0d9: jmp 0x140a8e207
140a8e0de: lea rcx, [rip + 0x204b253]
140a8e0e5: call 0x1428d9310 ; core::option::unwrap_failed
140a8e0ea: jmp 0x140a8e207
140a8e0ef: lea rcx, [rip + 0x204b25a]
140a8e0f6: call 0x1428d9310 ; core::option::unwrap_failed
140a8e0fb: jmp 0x140a8e207
140a8e100: mov ecx, 8
140a8e105: mov edx, 0x20
140a8e10a: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
140a8e10f: jmp 0x140a8e207
140a8e114: mov ecx, 8
140a8e119: mov edx, 0x20
140a8e11e: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
140a8e123: jmp 0x140a8e207
140a8e128: mov ecx, 8
140a8e12d: mov edx, 0x20
140a8e132: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
140a8e137: jmp 0x140a8e207
140a8e13c: mov ecx, 8
140a8e141: mov edx, 0x20
140a8e146: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
140a8e14b: jmp 0x140a8e207
140a8e150: mov ecx, 8
140a8e155: mov edx, 0x20
140a8e15a: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
140a8e15f: jmp 0x140a8e207
140a8e164: mov ecx, 8
140a8e169: mov edx, 0x20
140a8e16e: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
140a8e173: jmp 0x140a8e207
140a8e178: mov ecx, 8
140a8e17d: mov edx, 0x20
140a8e182: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
140a8e187: jmp 0x140a8e207
140a8e189: lea r8, [rip + 0x204a9a0]
140a8e190: mov ecx, 4
140a8e195: mov rdx, qword ptr [rbp + 0xcd8]
140a8e19c: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a8e1a1: jmp 0x140a8e207
140a8e1a3: xor ecx, ecx
140a8e1a5: lea r8, [rip + 0x21aa9cc]
140a8e1ac: mov rdx, r9
140a8e1af: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a8e1b4: jmp 0x140a8e207
140a8e1b6: xor ecx, ecx
140a8e1b8: lea r8, [rip + 0x21aa9b9]
140a8e1bf: mov rdx, r9
140a8e1c2: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a8e1c7: jmp 0x140a8e207
140a8e1c9: xor ecx, ecx
140a8e1cb: lea r8, [rip + 0x21aa9a6]
140a8e1d2: mov rdx, rsi
140a8e1d5: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a8e1da: jmp 0x140a8e207
140a8e1dc: xor ecx, ecx
140a8e1de: lea r8, [rip + 0x21aa993]
140a8e1e5: mov rdx, r9
140a8e1e8: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a8e1ed: jmp 0x140a8e207
140a8e1ef: xor ecx, ecx
140a8e1f1: mov byte ptr [rbp + 0xd56], 1
140a8e1f8: lea r8, [rip + 0x21aa979]
140a8e1ff: mov rdx, r9
140a8e202: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a8e207: ud2 
140a8e209: nop dword ptr [rax]
140a8e210: mov qword ptr [rsp + 0x10], rdx
140a8e215: push rbp
140a8e216: push r15
140a8e218: push r14
140a8e21a: push r13
140a8e21c: push r12
140a8e21e: push rsi
140a8e21f: push rdi
140a8e220: push rbx
140a8e221: sub rsp, 0xd8
140a8e228: lea rbp, [rdx + 0x80]
140a8e22f: movaps xmmword ptr [rsp + 0x30], xmm15
140a8e235: movaps xmmword ptr [rsp + 0x40], xmm14
140a8e23b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8e241: movaps xmmword ptr [rsp + 0x60], xmm12
140a8e247: movaps xmmword ptr [rsp + 0x70], xmm11
140a8e24d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8e256: movaps xmmword ptr [rsp + 0x90], xmm9
140a8e25f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8e268: movaps xmmword ptr [rsp + 0xb0], xmm7
140a8e270: movdqa xmmword ptr [rsp + 0xc0], xmm6
140a8e279: mov rax, qword ptr [rbp + 0xd48]
140a8e280: inc qword ptr [rax]
140a8e283: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8e28b: movaps xmm7, xmmword ptr [rsp + 0xb0]
140a8e293: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8e29c: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8e2a5: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8e2ae: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8e2b4: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8e2ba: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8e2c0: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8e2c6: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8e2cc: add rsp, 0xd8
140a8e2d3: pop rbx
140a8e2d4: pop rdi
140a8e2d5: pop rsi
140a8e2d6: pop r12
140a8e2d8: pop r13
140a8e2da: pop r14
140a8e2dc: pop r15
140a8e2de: pop rbp
140a8e2df: ret 
140a8e2e0: mov qword ptr [rsp + 0x10], rdx
140a8e2e5: push rbp
140a8e2e6: push r15
140a8e2e8: push r14
140a8e2ea: push r13
140a8e2ec: push r12
140a8e2ee: push rsi
140a8e2ef: push rdi
140a8e2f0: push rbx
140a8e2f1: sub rsp, 0xd8
140a8e2f8: lea rbp, [rdx + 0x80]
140a8e2ff: movaps xmmword ptr [rsp + 0x30], xmm15
140a8e305: movaps xmmword ptr [rsp + 0x40], xmm14
140a8e30b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8e311: movaps xmmword ptr [rsp + 0x60], xmm12
140a8e317: movaps xmmword ptr [rsp + 0x70], xmm11
140a8e31d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8e326: movaps xmmword ptr [rsp + 0x90], xmm9
140a8e32f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8e338: movaps xmmword ptr [rsp + 0xb0], xmm7
140a8e340: movdqa xmmword ptr [rsp + 0xc0], xmm6
140a8e349: cmp qword ptr [rbp + 0xc00], 0
140a8e351: je 0x140a8e382
140a8e353: lea rax, [rbp + 0xc08]
140a8e35a: mov qword ptr [rbp + 0xcb0], rax
140a8e361: lea rax, [rbp + 0xcb0]
140a8e368: mov qword ptr [rbp + 0xbf0], rax
140a8e36f: lea rcx, [rip + 0x204a952]
140a8e376: lea rdx, [rbp + 0xbf0]
140a8e37d: call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
140a8e382: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8e38a: movaps xmm7, xmmword ptr [rsp + 0xb0]
140a8e392: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8e39b: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8e3a4: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8e3ad: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8e3b3: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8e3b9: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8e3bf: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8e3c5: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8e3cb: add rsp, 0xd8
140a8e3d2: pop rbx
140a8e3d3: pop rdi
140a8e3d4: pop rsi
140a8e3d5: pop r12
140a8e3d7: pop r13
140a8e3d9: pop r14
140a8e3db: pop r15
140a8e3dd: pop rbp
140a8e3de: ret 
140a8e3df: nop 
140a8e3e0: mov qword ptr [rsp + 0x10], rdx
140a8e3e5: push rbp
140a8e3e6: push r15
140a8e3e8: push r14
140a8e3ea: push r13
140a8e3ec: push r12
140a8e3ee: push rsi
140a8e3ef: push rdi
140a8e3f0: push rbx
140a8e3f1: sub rsp, 0xd8
140a8e3f8: lea rbp, [rdx + 0x80]
140a8e3ff: movaps xmmword ptr [rsp + 0x30], xmm15
140a8e405: movaps xmmword ptr [rsp + 0x40], xmm14
140a8e40b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8e411: movaps xmmword ptr [rsp + 0x60], xmm12
140a8e417: movaps xmmword ptr [rsp + 0x70], xmm11
140a8e41d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8e426: movaps xmmword ptr [rsp + 0x90], xmm9
140a8e42f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8e438: movaps xmmword ptr [rsp + 0xb0], xmm7
140a8e440: movdqa xmmword ptr [rsp + 0xc0], xmm6
140a8e449: cmp qword ptr [rbp + 0xd30], 0
140a8e451: mov rdx, qword ptr [rbp + 0xd20]
140a8e458: je 0x140a8e472
140a8e45a: test rdx, rdx
140a8e45d: je 0x140a8e472
140a8e45f: mov rcx, qword ptr [rbp + 0xd00]
140a8e466: mov r8, qword ptr [rbp + 0xd10]
140a8e46d: call 0x140613c20
140a8e472: mov byte ptr [rbp + 0xd55], 0
140a8e479: movdqa xmm6, xmmword ptr [rsp + 0xc0]
140a8e482: movaps xmm7, xmmword ptr [rsp + 0xb0]
140a8e48a: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8e493: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8e49c: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8e4a5: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8e4ab: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8e4b1: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8e4b7: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8e4bd: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8e4c3: add rsp, 0xd8
140a8e4ca: pop rbx
140a8e4cb: pop rdi
140a8e4cc: pop rsi
140a8e4cd: pop r12
140a8e4cf: pop r13
140a8e4d1: pop r14
140a8e4d3: pop r15
140a8e4d5: pop rbp
140a8e4d6: ret 
140a8e4d7: nop word ptr [rax + rax]
140a8e4e0: mov qword ptr [rsp + 0x10], rdx
140a8e4e5: push rbp
140a8e4e6: push r15
140a8e4e8: push r14
140a8e4ea: push r13
140a8e4ec: push r12
140a8e4ee: push rsi
140a8e4ef: push rdi
140a8e4f0: push rbx
140a8e4f1: sub rsp, 0xd8
140a8e4f8: lea rbp, [rdx + 0x80]
140a8e4ff: movaps xmmword ptr [rsp + 0x30], xmm15
140a8e505: movaps xmmword ptr [rsp + 0x40], xmm14
140a8e50b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8e511: movaps xmmword ptr [rsp + 0x60], xmm12
140a8e517: movaps xmmword ptr [rsp + 0x70], xmm11
140a8e51d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8e526: movaps xmmword ptr [rsp + 0x90], xmm9
140a8e52f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8e538: movaps xmmword ptr [rsp + 0xb0], xmm7
140a8e540: movdqa xmmword ptr [rsp + 0xc0], xmm6
140a8e549: cmp byte ptr [rbp + 0xd55], 0
140a8e550: je 0x140a8e55e
140a8e552: lea rcx, [rbp + 0x840]
140a8e559: call 0x140149f50 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
140a8e55e: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8e566: movaps xmm7, xmmword ptr [rsp + 0xb0]
140a8e56e: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8e577: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8e580: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8e589: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8e58f: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8e595: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8e59b: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8e5a1: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8e5a7: add rsp, 0xd8
140a8e5ae: pop rbx
140a8e5af: pop rdi
140a8e5b0: pop rsi
140a8e5b1: pop r12
140a8e5b3: pop r13
140a8e5b5: pop r14
140a8e5b7: pop r15
140a8e5b9: pop rbp
140a8e5ba: ret 
140a8e5bb: nop dword ptr [rax + rax]
140a8e5c0: mov qword ptr [rsp + 0x10], rdx
140a8e5c5: push rbp
140a8e5c6: push r15
140a8e5c8: push r14
140a8e5ca: push r13
140a8e5cc: push r12
140a8e5ce: push rsi
140a8e5cf: push rdi
140a8e5d0: push rbx
140a8e5d1: sub rsp, 0xd8
140a8e5d8: lea rbp, [rdx + 0x80]
140a8e5df: movaps xmmword ptr [rsp + 0x30], xmm15
140a8e5e5: movaps xmmword ptr [rsp + 0x40], xmm14
140a8e5eb: movaps xmmword ptr [rsp + 0x50], xmm13
140a8e5f1: movaps xmmword ptr [rsp + 0x60], xmm12
140a8e5f7: movaps xmmword ptr [rsp + 0x70], xmm11
140a8e5fd: movaps xmmword ptr [rsp + 0x80], xmm10
140a8e606: movaps xmmword ptr [rsp + 0x90], xmm9
140a8e60f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8e618: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8e621: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8e629: mov rax, qword ptr [rbp + 0xd48]
140a8e630: inc qword ptr [rax]
140a8e633: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8e63b: movaps xmm7, xmmword ptr [rsp + 0xb0]
140a8e643: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8e64c: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8e655: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8e65e: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8e664: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8e66a: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8e670: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8e676: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8e67c: add rsp, 0xd8
140a8e683: pop rbx
140a8e684: pop rdi
140a8e685: pop rsi
140a8e686: pop r12
140a8e688: pop r13
140a8e68a: pop r14
140a8e68c: pop r15
140a8e68e: pop rbp
140a8e68f: ret 
140a8e690: mov qword ptr [rsp + 0x10], rdx
140a8e695: push rbp
140a8e696: push r15
140a8e698: push r14
140a8e69a: push r13
140a8e69c: push r12
140a8e69e: push rsi
140a8e69f: push rdi
140a8e6a0: push rbx
140a8e6a1: sub rsp, 0xd8
140a8e6a8: lea rbp, [rdx + 0x80]
140a8e6af: movaps xmmword ptr [rsp + 0x30], xmm15
140a8e6b5: movaps xmmword ptr [rsp + 0x40], xmm14
140a8e6bb: movaps xmmword ptr [rsp + 0x50], xmm13
140a8e6c1: movaps xmmword ptr [rsp + 0x60], xmm12
140a8e6c7: movaps xmmword ptr [rsp + 0x70], xmm11
140a8e6cd: movaps xmmword ptr [rsp + 0x80], xmm10
140a8e6d6: movaps xmmword ptr [rsp + 0x90], xmm9
140a8e6df: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8e6e8: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8e6f1: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8e6f9: mov rdx, qword ptr [rbp + 0x918]
140a8e700: test rdx, rdx
140a8e703: jne 0x140a8e79f
140a8e709: mov rdx, qword ptr [rbp + 0x930]
140a8e710: test rdx, rdx
140a8e713: jne 0x140a8e7c5
140a8e719: mov rdx, qword ptr [rbp + 0x9d0]
140a8e720: test rdx, rdx
140a8e723: je 0x140a8e73b
140a8e725: mov rcx, qword ptr [rbp + 0x9d8]
140a8e72c: shl rdx, 3
140a8e730: mov r8d, 4
140a8e736: call 0x140613c20
140a8e73b: mov byte ptr [rbp + 0xd57], 0
140a8e742: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8e74a: movaps xmm7, xmmword ptr [rsp + 0xb0]
140a8e752: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8e75b: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8e764: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8e76d: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8e773: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8e779: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8e77f: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8e785: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8e78b: add rsp, 0xd8
140a8e792: pop rbx
140a8e793: pop rdi
140a8e794: pop rsi
140a8e795: pop r12
140a8e797: pop r13
140a8e799: pop r14
140a8e79b: pop r15
140a8e79d: pop rbp
140a8e79e: ret 
140a8e79f: mov rcx, qword ptr [rbp + 0x920]
140a8e7a6: shl rdx, 2
140a8e7aa: mov r8d, 4
140a8e7b0: call 0x140613c20
140a8e7b5: mov rdx, qword ptr [rbp + 0x930]
140a8e7bc: test rdx, rdx
140a8e7bf: je 0x140a8e719
140a8e7c5: mov rcx, qword ptr [rbp + 0x938]
140a8e7cc: shl rdx, 2
140a8e7d0: mov r8d, 4
140a8e7d6: call 0x140613c20
140a8e7db: mov rdx, qword ptr [rbp + 0x9d0]
140a8e7e2: test rdx, rdx
140a8e7e5: jne 0x140a8e725
140a8e7eb: jmp 0x140a8e73b
140a8e7f0: mov qword ptr [rsp + 0x10], rdx
140a8e7f5: push rbp
140a8e7f6: push r15
140a8e7f8: push r14
140a8e7fa: push r13
140a8e7fc: push r12
140a8e7fe: push rsi
140a8e7ff: push rdi
140a8e800: push rbx
140a8e801: sub rsp, 0xd8
140a8e808: lea rbp, [rdx + 0x80]
140a8e80f: movaps xmmword ptr [rsp + 0x30], xmm15
140a8e815: movaps xmmword ptr [rsp + 0x40], xmm14
140a8e81b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8e821: movaps xmmword ptr [rsp + 0x60], xmm12
140a8e827: movaps xmmword ptr [rsp + 0x70], xmm11
140a8e82d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8e836: movaps xmmword ptr [rsp + 0x90], xmm9
140a8e83f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8e848: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8e851: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8e859: lea rcx, [rbp + 0x840]
140a8e860: call 0x140895600 ; core::ptr::drop_in_place<country_core::resources::walls::public_wall_state::PublicWallState>
140a8e865: mov byte ptr [rbp + 0xd57], 0
140a8e86c: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8e874: movaps xmm7, xmmword ptr [rsp + 0xb0]
140a8e87c: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8e885: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8e88e: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8e897: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8e89d: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8e8a3: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8e8a9: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8e8af: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8e8b5: add rsp, 0xd8
140a8e8bc: pop rbx
140a8e8bd: pop rdi
140a8e8be: pop rsi
140a8e8bf: pop r12
140a8e8c1: pop r13
140a8e8c3: pop r14
140a8e8c5: pop r15
140a8e8c7: pop rbp
140a8e8c8: ret 
140a8e8c9: nop dword ptr [rax]
140a8e8d0: mov qword ptr [rsp + 0x10], rdx
140a8e8d5: push rbp
140a8e8d6: push r15
140a8e8d8: push r14
140a8e8da: push r13
140a8e8dc: push r12
140a8e8de: push rsi
140a8e8df: push rdi
140a8e8e0: push rbx
140a8e8e1: sub rsp, 0xd8
140a8e8e8: lea rbp, [rdx + 0x80]
140a8e8ef: movaps xmmword ptr [rsp + 0x30], xmm15
140a8e8f5: movaps xmmword ptr [rsp + 0x40], xmm14
140a8e8fb: movaps xmmword ptr [rsp + 0x50], xmm13
140a8e901: movaps xmmword ptr [rsp + 0x60], xmm12
140a8e907: movaps xmmword ptr [rsp + 0x70], xmm11
140a8e90d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8e916: movaps xmmword ptr [rsp + 0x90], xmm9
140a8e91f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8e928: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8e931: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8e939: cmp qword ptr [rbp + 0xd38], 0
140a8e941: je 0x140a8e960
140a8e943: mov rdx, qword ptr [rbp + 0xd38]
140a8e94a: shl rdx, 3
140a8e94e: mov r8d, 4
140a8e954: mov rcx, qword ptr [rbp + 0xd18]
140a8e95b: call 0x140613c20
140a8e960: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8e968: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8e971: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8e97a: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8e983: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8e98c: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8e992: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8e998: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8e99e: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8e9a4: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8e9aa: add rsp, 0xd8
140a8e9b1: pop rbx
140a8e9b2: pop rdi
140a8e9b3: pop rsi
140a8e9b4: pop r12
140a8e9b6: pop r13
140a8e9b8: pop r14
140a8e9ba: pop r15
140a8e9bc: pop rbp
140a8e9bd: ret 
140a8e9be: nop 
140a8e9c0: mov qword ptr [rsp + 0x10], rdx
140a8e9c5: push rbp
140a8e9c6: push r15
140a8e9c8: push r14
140a8e9ca: push r13
140a8e9cc: push r12
140a8e9ce: push rsi
140a8e9cf: push rdi
140a8e9d0: push rbx
140a8e9d1: sub rsp, 0xd8
140a8e9d8: lea rbp, [rdx + 0x80]
140a8e9df: movaps xmmword ptr [rsp + 0x30], xmm15
140a8e9e5: movaps xmmword ptr [rsp + 0x40], xmm14
140a8e9eb: movaps xmmword ptr [rsp + 0x50], xmm13
140a8e9f1: movaps xmmword ptr [rsp + 0x60], xmm12
140a8e9f7: movaps xmmword ptr [rsp + 0x70], xmm11
140a8e9fd: movaps xmmword ptr [rsp + 0x80], xmm10
140a8ea06: movaps xmmword ptr [rsp + 0x90], xmm9
140a8ea0f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8ea18: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8ea21: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8ea29: lea rcx, [rbp + 0x840]
140a8ea30: call 0x1405d24f0 ; core::ptr::drop_in_place<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D> >
140a8ea35: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8ea3d: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8ea46: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8ea4f: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8ea58: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8ea61: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8ea67: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8ea6d: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8ea73: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8ea79: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8ea7f: add rsp, 0xd8
140a8ea86: pop rbx
140a8ea87: pop rdi
140a8ea88: pop rsi
140a8ea89: pop r12
140a8ea8b: pop r13
140a8ea8d: pop r14
140a8ea8f: pop r15
140a8ea91: pop rbp
140a8ea92: ret 
140a8ea93: nop word ptr cs:[rax + rax]
140a8eaa0: mov qword ptr [rsp + 0x10], rdx
140a8eaa5: push rbp
140a8eaa6: push r15
140a8eaa8: push r14
140a8eaaa: push r13
140a8eaac: push r12
140a8eaae: push rsi
140a8eaaf: push rdi
140a8eab0: push rbx
140a8eab1: sub rsp, 0xd8
140a8eab8: lea rbp, [rdx + 0x80]
140a8eabf: movaps xmmword ptr [rsp + 0x30], xmm15
140a8eac5: movaps xmmword ptr [rsp + 0x40], xmm14
140a8eacb: movaps xmmword ptr [rsp + 0x50], xmm13
140a8ead1: movaps xmmword ptr [rsp + 0x60], xmm12
140a8ead7: movaps xmmword ptr [rsp + 0x70], xmm11
140a8eadd: movaps xmmword ptr [rsp + 0x80], xmm10
140a8eae6: movaps xmmword ptr [rsp + 0x90], xmm9
140a8eaef: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8eaf8: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8eb01: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8eb09: lea rcx, [rbp + 0x260]
140a8eb10: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140a8eb15: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8eb1d: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8eb26: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8eb2f: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8eb38: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8eb41: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8eb47: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8eb4d: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8eb53: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8eb59: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8eb5f: add rsp, 0xd8
140a8eb66: pop rbx
140a8eb67: pop rdi
140a8eb68: pop rsi
140a8eb69: pop r12
140a8eb6b: pop r13
140a8eb6d: pop r14
140a8eb6f: pop r15
140a8eb71: pop rbp
140a8eb72: ret 
140a8eb73: nop word ptr cs:[rax + rax]
140a8eb80: mov qword ptr [rsp + 0x10], rdx
140a8eb85: push rbp
140a8eb86: push r15
140a8eb88: push r14
140a8eb8a: push r13
140a8eb8c: push r12
140a8eb8e: push rsi
140a8eb8f: push rdi
140a8eb90: push rbx
140a8eb91: sub rsp, 0xd8
140a8eb98: lea rbp, [rdx + 0x80]
140a8eb9f: movaps xmmword ptr [rsp + 0x30], xmm15
140a8eba5: movaps xmmword ptr [rsp + 0x40], xmm14
140a8ebab: movaps xmmword ptr [rsp + 0x50], xmm13
140a8ebb1: movaps xmmword ptr [rsp + 0x60], xmm12
140a8ebb7: movaps xmmword ptr [rsp + 0x70], xmm11
140a8ebbd: movaps xmmword ptr [rsp + 0x80], xmm10
140a8ebc6: movaps xmmword ptr [rsp + 0x90], xmm9
140a8ebcf: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8ebd8: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8ebe1: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8ebe9: mov r8d, 4
140a8ebef: mov rcx, qword ptr [rbp + 0xd38]
140a8ebf6: mov rdx, qword ptr [rbp + 0xcd8]
140a8ebfd: call 0x140613c20
140a8ec02: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8ec0a: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8ec13: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8ec1c: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8ec25: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8ec2e: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8ec34: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8ec3a: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8ec40: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8ec46: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8ec4c: add rsp, 0xd8
140a8ec53: pop rbx
140a8ec54: pop rdi
140a8ec55: pop rsi
140a8ec56: pop r12
140a8ec58: pop r13
140a8ec5a: pop r14
140a8ec5c: pop r15
140a8ec5e: pop rbp
140a8ec5f: ret 
140a8ec60: mov qword ptr [rsp + 0x10], rdx
140a8ec65: push rbp
140a8ec66: push r15
140a8ec68: push r14
140a8ec6a: push r13
140a8ec6c: push r12
140a8ec6e: push rsi
140a8ec6f: push rdi
140a8ec70: push rbx
140a8ec71: sub rsp, 0xd8
140a8ec78: lea rbp, [rdx + 0x80]
140a8ec7f: movaps xmmword ptr [rsp + 0x30], xmm15
140a8ec85: movaps xmmword ptr [rsp + 0x40], xmm14
140a8ec8b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8ec91: movaps xmmword ptr [rsp + 0x60], xmm12
140a8ec97: movaps xmmword ptr [rsp + 0x70], xmm11
140a8ec9d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8eca6: movaps xmmword ptr [rsp + 0x90], xmm9
140a8ecaf: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8ecb8: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8ecc1: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8ecc9: cmp qword ptr [rbp + 0xd18], 0
140a8ecd1: je 0x140a8ecec
140a8ecd3: mov r8d, 4
140a8ecd9: mov rcx, qword ptr [rbp + 0xd38]
140a8ece0: mov rdx, qword ptr [rbp + 0xcd8]
140a8ece7: call 0x140613c20
140a8ecec: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8ecf4: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8ecfd: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8ed06: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8ed0f: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8ed18: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8ed1e: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8ed24: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8ed2a: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8ed30: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8ed36: add rsp, 0xd8
140a8ed3d: pop rbx
140a8ed3e: pop rdi
140a8ed3f: pop rsi
140a8ed40: pop r12
140a8ed42: pop r13
140a8ed44: pop r14
140a8ed46: pop r15
140a8ed48: pop rbp
140a8ed49: ret 
140a8ed4a: nop word ptr [rax + rax]
140a8ed50: mov qword ptr [rsp + 0x10], rdx
140a8ed55: push rbp
140a8ed56: push r15
140a8ed58: push r14
140a8ed5a: push r13
140a8ed5c: push r12
140a8ed5e: push rsi
140a8ed5f: push rdi
140a8ed60: push rbx
140a8ed61: sub rsp, 0xd8
140a8ed68: lea rbp, [rdx + 0x80]
140a8ed6f: movaps xmmword ptr [rsp + 0x30], xmm15
140a8ed75: movaps xmmword ptr [rsp + 0x40], xmm14
140a8ed7b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8ed81: movaps xmmword ptr [rsp + 0x60], xmm12
140a8ed87: movaps xmmword ptr [rsp + 0x70], xmm11
140a8ed8d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8ed96: movaps xmmword ptr [rsp + 0x90], xmm9
140a8ed9f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8eda8: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8edb1: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8edb9: lea rcx, [rbp + 0xb48]
140a8edc0: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8edc5: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8edcd: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8edd6: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8eddf: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8ede8: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8edf1: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8edf7: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8edfd: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8ee03: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8ee09: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8ee0f: add rsp, 0xd8
140a8ee16: pop rbx
140a8ee17: pop rdi
140a8ee18: pop rsi
140a8ee19: pop r12
140a8ee1b: pop r13
140a8ee1d: pop r14
140a8ee1f: pop r15
140a8ee21: pop rbp
140a8ee22: ret 
140a8ee23: nop word ptr cs:[rax + rax]
140a8ee30: mov qword ptr [rsp + 0x10], rdx
140a8ee35: push rbp
140a8ee36: push r15
140a8ee38: push r14
140a8ee3a: push r13
140a8ee3c: push r12
140a8ee3e: push rsi
140a8ee3f: push rdi
140a8ee40: push rbx
140a8ee41: sub rsp, 0xd8
140a8ee48: lea rbp, [rdx + 0x80]
140a8ee4f: movaps xmmword ptr [rsp + 0x30], xmm15
140a8ee55: movaps xmmword ptr [rsp + 0x40], xmm14
140a8ee5b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8ee61: movaps xmmword ptr [rsp + 0x60], xmm12
140a8ee67: movaps xmmword ptr [rsp + 0x70], xmm11
140a8ee6d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8ee76: movaps xmmword ptr [rsp + 0x90], xmm9
140a8ee7f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8ee88: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8ee91: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8ee99: lea rcx, [rbp + 0xb38]
140a8eea0: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8eea5: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8eead: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8eeb6: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8eebf: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8eec8: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8eed1: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8eed7: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8eedd: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8eee3: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8eee9: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8eeef: add rsp, 0xd8
140a8eef6: pop rbx
140a8eef7: pop rdi
140a8eef8: pop rsi
140a8eef9: pop r12
140a8eefb: pop r13
140a8eefd: pop r14
140a8eeff: pop r15
140a8ef01: pop rbp
140a8ef02: ret 
140a8ef03: nop word ptr cs:[rax + rax]
140a8ef10: mov qword ptr [rsp + 0x10], rdx
140a8ef15: push rbp
140a8ef16: push r15
140a8ef18: push r14
140a8ef1a: push r13
140a8ef1c: push r12
140a8ef1e: push rsi
140a8ef1f: push rdi
140a8ef20: push rbx
140a8ef21: sub rsp, 0xd8
140a8ef28: lea rbp, [rdx + 0x80]
140a8ef2f: movaps xmmword ptr [rsp + 0x30], xmm15
140a8ef35: movaps xmmword ptr [rsp + 0x40], xmm14
140a8ef3b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8ef41: movaps xmmword ptr [rsp + 0x60], xmm12
140a8ef47: movaps xmmword ptr [rsp + 0x70], xmm11
140a8ef4d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8ef56: movaps xmmword ptr [rsp + 0x90], xmm9
140a8ef5f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8ef68: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8ef71: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8ef79: lea rcx, [rbp + 0xb28]
140a8ef80: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8ef85: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8ef8d: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8ef96: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8ef9f: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8efa8: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8efb1: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8efb7: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8efbd: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8efc3: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8efc9: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8efcf: add rsp, 0xd8
140a8efd6: pop rbx
140a8efd7: pop rdi
140a8efd8: pop rsi
140a8efd9: pop r12
140a8efdb: pop r13
140a8efdd: pop r14
140a8efdf: pop r15
140a8efe1: pop rbp
140a8efe2: ret 
140a8efe3: nop word ptr cs:[rax + rax]
140a8eff0: mov qword ptr [rsp + 0x10], rdx
140a8eff5: push rbp
140a8eff6: push r15
140a8eff8: push r14
140a8effa: push r13
140a8effc: push r12
140a8effe: push rsi
140a8efff: push rdi
140a8f000: push rbx
140a8f001: sub rsp, 0xd8
140a8f008: lea rbp, [rdx + 0x80]
140a8f00f: movaps xmmword ptr [rsp + 0x30], xmm15
140a8f015: movaps xmmword ptr [rsp + 0x40], xmm14
140a8f01b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8f021: movaps xmmword ptr [rsp + 0x60], xmm12
140a8f027: movaps xmmword ptr [rsp + 0x70], xmm11
140a8f02d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8f036: movaps xmmword ptr [rsp + 0x90], xmm9
140a8f03f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8f048: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8f051: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8f059: lea rcx, [rbp + 0xb18]
140a8f060: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8f065: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8f06d: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8f076: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8f07f: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8f088: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8f091: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8f097: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8f09d: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8f0a3: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8f0a9: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8f0af: add rsp, 0xd8
140a8f0b6: pop rbx
140a8f0b7: pop rdi
140a8f0b8: pop rsi
140a8f0b9: pop r12
140a8f0bb: pop r13
140a8f0bd: pop r14
140a8f0bf: pop r15
140a8f0c1: pop rbp
140a8f0c2: ret 
140a8f0c3: nop word ptr cs:[rax + rax]
140a8f0d0: mov qword ptr [rsp + 0x10], rdx
140a8f0d5: push rbp
140a8f0d6: push r15
140a8f0d8: push r14
140a8f0da: push r13
140a8f0dc: push r12
140a8f0de: push rsi
140a8f0df: push rdi
140a8f0e0: push rbx
140a8f0e1: sub rsp, 0xd8
140a8f0e8: lea rbp, [rdx + 0x80]
140a8f0ef: movaps xmmword ptr [rsp + 0x30], xmm15
140a8f0f5: movaps xmmword ptr [rsp + 0x40], xmm14
140a8f0fb: movaps xmmword ptr [rsp + 0x50], xmm13
140a8f101: movaps xmmword ptr [rsp + 0x60], xmm12
140a8f107: movaps xmmword ptr [rsp + 0x70], xmm11
140a8f10d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8f116: movaps xmmword ptr [rsp + 0x90], xmm9
140a8f11f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8f128: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8f131: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8f139: lea rcx, [rbp + 0xb08]
140a8f140: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8f145: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8f14d: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8f156: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8f15f: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8f168: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8f171: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8f177: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8f17d: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8f183: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8f189: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8f18f: add rsp, 0xd8
140a8f196: pop rbx
140a8f197: pop rdi
140a8f198: pop rsi
140a8f199: pop r12
140a8f19b: pop r13
140a8f19d: pop r14
140a8f19f: pop r15
140a8f1a1: pop rbp
140a8f1a2: ret 
140a8f1a3: nop word ptr cs:[rax + rax]
140a8f1b0: mov qword ptr [rsp + 0x10], rdx
140a8f1b5: push rbp
140a8f1b6: push r15
140a8f1b8: push r14
140a8f1ba: push r13
140a8f1bc: push r12
140a8f1be: push rsi
140a8f1bf: push rdi
140a8f1c0: push rbx
140a8f1c1: sub rsp, 0xd8
140a8f1c8: lea rbp, [rdx + 0x80]
140a8f1cf: movaps xmmword ptr [rsp + 0x30], xmm15
140a8f1d5: movaps xmmword ptr [rsp + 0x40], xmm14
140a8f1db: movaps xmmword ptr [rsp + 0x50], xmm13
140a8f1e1: movaps xmmword ptr [rsp + 0x60], xmm12
140a8f1e7: movaps xmmword ptr [rsp + 0x70], xmm11
140a8f1ed: movaps xmmword ptr [rsp + 0x80], xmm10
140a8f1f6: movaps xmmword ptr [rsp + 0x90], xmm9
140a8f1ff: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8f208: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8f211: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8f219: lea rcx, [rbp + 0xaf8]
140a8f220: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8f225: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8f22d: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8f236: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8f23f: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8f248: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8f251: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8f257: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8f25d: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8f263: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8f269: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8f26f: add rsp, 0xd8
140a8f276: pop rbx
140a8f277: pop rdi
140a8f278: pop rsi
140a8f279: pop r12
140a8f27b: pop r13
140a8f27d: pop r14
140a8f27f: pop r15
140a8f281: pop rbp
140a8f282: ret 
140a8f283: nop word ptr cs:[rax + rax]
140a8f290: mov qword ptr [rsp + 0x10], rdx
140a8f295: push rbp
140a8f296: push r15
140a8f298: push r14
140a8f29a: push r13
140a8f29c: push r12
140a8f29e: push rsi
140a8f29f: push rdi
140a8f2a0: push rbx
140a8f2a1: sub rsp, 0xd8
140a8f2a8: lea rbp, [rdx + 0x80]
140a8f2af: movaps xmmword ptr [rsp + 0x30], xmm15
140a8f2b5: movaps xmmword ptr [rsp + 0x40], xmm14
140a8f2bb: movaps xmmword ptr [rsp + 0x50], xmm13
140a8f2c1: movaps xmmword ptr [rsp + 0x60], xmm12
140a8f2c7: movaps xmmword ptr [rsp + 0x70], xmm11
140a8f2cd: movaps xmmword ptr [rsp + 0x80], xmm10
140a8f2d6: movaps xmmword ptr [rsp + 0x90], xmm9
140a8f2df: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8f2e8: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8f2f1: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8f2f9: lea rcx, [rbp + 0xae8]
140a8f300: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8f305: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8f30d: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8f316: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8f31f: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8f328: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8f331: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8f337: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8f33d: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8f343: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8f349: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8f34f: add rsp, 0xd8
140a8f356: pop rbx
140a8f357: pop rdi
140a8f358: pop rsi
140a8f359: pop r12
140a8f35b: pop r13
140a8f35d: pop r14
140a8f35f: pop r15
140a8f361: pop rbp
140a8f362: ret 
140a8f363: nop word ptr cs:[rax + rax]
140a8f370: mov qword ptr [rsp + 0x10], rdx
140a8f375: push rbp
140a8f376: push r15
140a8f378: push r14
140a8f37a: push r13
140a8f37c: push r12
140a8f37e: push rsi
140a8f37f: push rdi
140a8f380: push rbx
140a8f381: sub rsp, 0xd8
140a8f388: lea rbp, [rdx + 0x80]
140a8f38f: movaps xmmword ptr [rsp + 0x30], xmm15
140a8f395: movaps xmmword ptr [rsp + 0x40], xmm14
140a8f39b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8f3a1: movaps xmmword ptr [rsp + 0x60], xmm12
140a8f3a7: movaps xmmword ptr [rsp + 0x70], xmm11
140a8f3ad: movaps xmmword ptr [rsp + 0x80], xmm10
140a8f3b6: movaps xmmword ptr [rsp + 0x90], xmm9
140a8f3bf: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8f3c8: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8f3d1: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8f3d9: cmp qword ptr [rbp + 0xca0], 0
140a8f3e1: je 0x140a8f400
140a8f3e3: mov rdx, qword ptr [rbp + 0xca0]
140a8f3ea: shl rdx, 3
140a8f3ee: mov r8d, 4
140a8f3f4: mov rcx, qword ptr [rbp + 0xc20]
140a8f3fb: call 0x140613c20
140a8f400: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8f408: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8f411: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8f41a: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8f423: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8f42c: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8f432: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8f438: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8f43e: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8f444: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8f44a: add rsp, 0xd8
140a8f451: pop rbx
140a8f452: pop rdi
140a8f453: pop rsi
140a8f454: pop r12
140a8f456: pop r13
140a8f458: pop r14
140a8f45a: pop r15
140a8f45c: pop rbp
140a8f45d: ret 
140a8f45e: nop 
140a8f460: mov qword ptr [rsp + 0x10], rdx
140a8f465: push rbp
140a8f466: push r15
140a8f468: push r14
140a8f46a: push r13
140a8f46c: push r12
140a8f46e: push rsi
140a8f46f: push rdi
140a8f470: push rbx
140a8f471: sub rsp, 0xd8
140a8f478: lea rbp, [rdx + 0x80]
140a8f47f: movaps xmmword ptr [rsp + 0x30], xmm15
140a8f485: movaps xmmword ptr [rsp + 0x40], xmm14
140a8f48b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8f491: movaps xmmword ptr [rsp + 0x60], xmm12
140a8f497: movaps xmmword ptr [rsp + 0x70], xmm11
140a8f49d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8f4a6: movaps xmmword ptr [rsp + 0x90], xmm9
140a8f4af: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8f4b8: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8f4c1: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8f4c9: mov rdx, qword ptr [rbp + 0xc30]
140a8f4d0: test rdx, rdx
140a8f4d3: sete al
140a8f4d6: mov ecx, dword ptr [rbp + 0xca8]
140a8f4dc: or cl, al
140a8f4de: jne 0x140a8f4f6
140a8f4e0: shl rdx, 3
140a8f4e4: mov r8d, 4
140a8f4ea: mov rcx, qword ptr [rbp + 0xc38]
140a8f4f1: call 0x140613c20
140a8f4f6: mov eax, dword ptr [rbp + 0xd28]
140a8f4fc: mov byte ptr [rbp + 0xd57], al
140a8f502: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8f50a: movaps xmm7, xmmword ptr [rsp + 0xb0]
140a8f512: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8f51b: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8f524: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8f52d: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8f533: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8f539: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8f53f: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8f545: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8f54b: add rsp, 0xd8
140a8f552: pop rbx
140a8f553: pop rdi
140a8f554: pop rsi
140a8f555: pop r12
140a8f557: pop r13
140a8f559: pop r14
140a8f55b: pop r15
140a8f55d: pop rbp
140a8f55e: ret 
140a8f55f: nop 
140a8f560: mov qword ptr [rsp + 0x10], rdx
140a8f565: push rbp
140a8f566: push r15
140a8f568: push r14
140a8f56a: push r13
140a8f56c: push r12
140a8f56e: push rsi
140a8f56f: push rdi
140a8f570: push rbx
140a8f571: sub rsp, 0xd8
140a8f578: lea rbp, [rdx + 0x80]
140a8f57f: movaps xmmword ptr [rsp + 0x30], xmm15
140a8f585: movaps xmmword ptr [rsp + 0x40], xmm14
140a8f58b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8f591: movaps xmmword ptr [rsp + 0x60], xmm12
140a8f597: movaps xmmword ptr [rsp + 0x70], xmm11
140a8f59d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8f5a6: movaps xmmword ptr [rsp + 0x90], xmm9
140a8f5af: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8f5b8: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8f5c1: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8f5c9: cmp byte ptr [rbp + 0xd56], 0
140a8f5d0: je 0x140a8f647
140a8f5d2: lea rcx, [rbp + 0x500]
140a8f5d9: call 0x140895600 ; core::ptr::drop_in_place<country_core::resources::walls::public_wall_state::PublicWallState>
140a8f5de: mov eax, dword ptr [rbp + 0xd28]
140a8f5e4: mov byte ptr [rbp + 0xd57], al
140a8f5ea: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8f5f2: movaps xmm7, xmmword ptr [rsp + 0xb0]
140a8f5fa: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8f603: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8f60c: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8f615: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8f61b: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8f621: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8f627: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8f62d: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8f633: add rsp, 0xd8
140a8f63a: pop rbx
140a8f63b: pop rdi
140a8f63c: pop rsi
140a8f63d: pop r12
140a8f63f: pop r13
140a8f641: pop r14
140a8f643: pop r15
140a8f645: pop rbp
140a8f646: ret 
140a8f647: mov eax, dword ptr [rbp + 0xd28]
140a8f64d: mov byte ptr [rbp + 0xd57], al
140a8f653: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8f65b: movaps xmm7, xmmword ptr [rsp + 0xb0]
140a8f663: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8f66c: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8f675: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8f67e: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8f684: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8f68a: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8f690: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8f696: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8f69c: add rsp, 0xd8
140a8f6a3: pop rbx
140a8f6a4: pop rdi
140a8f6a5: pop rsi
140a8f6a6: pop r12
140a8f6a8: pop r13
140a8f6aa: pop r14
140a8f6ac: pop r15
140a8f6ae: pop rbp
140a8f6af: ret 
140a8f6b0: mov qword ptr [rsp + 0x10], rdx
140a8f6b5: push rbp
140a8f6b6: push r15
140a8f6b8: push r14
140a8f6ba: push r13
140a8f6bc: push r12
140a8f6be: push rsi
140a8f6bf: push rdi
140a8f6c0: push rbx
140a8f6c1: sub rsp, 0xd8
140a8f6c8: lea rbp, [rdx + 0x80]
140a8f6cf: movaps xmmword ptr [rsp + 0x30], xmm15
140a8f6d5: movaps xmmword ptr [rsp + 0x40], xmm14
140a8f6db: movaps xmmword ptr [rsp + 0x50], xmm13
140a8f6e1: movaps xmmword ptr [rsp + 0x60], xmm12
140a8f6e7: movaps xmmword ptr [rsp + 0x70], xmm11
140a8f6ed: movaps xmmword ptr [rsp + 0x80], xmm10
140a8f6f6: movaps xmmword ptr [rsp + 0x90], xmm9
140a8f6ff: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8f708: movaps xmmword ptr [rsp + 0xb0], xmm7
140a8f710: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8f718: xor eax, eax
140a8f71a: cmp rax, qword ptr [rbp - 0x40]
140a8f71e: jo 0x140a8f793
140a8f720: test byte ptr [rbp + 0xd57], 1
140a8f727: je 0x140a8f7f0
140a8f72d: lea rcx, [rbp - 0x40]
140a8f731: call 0x140895600 ; core::ptr::drop_in_place<country_core::resources::walls::public_wall_state::PublicWallState>
140a8f736: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8f73e: movaps xmm7, xmmword ptr [rsp + 0xb0]
140a8f746: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8f74f: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8f758: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8f761: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8f767: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8f76d: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8f773: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8f779: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8f77f: add rsp, 0xd8
140a8f786: pop rbx
140a8f787: pop rdi
140a8f788: pop rsi
140a8f789: pop r12
140a8f78b: pop r13
140a8f78d: pop r14
140a8f78f: pop r15
140a8f791: pop rbp
140a8f792: ret 
140a8f793: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8f79b: movaps xmm7, xmmword ptr [rsp + 0xb0]
140a8f7a3: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8f7ac: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8f7b5: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8f7be: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8f7c4: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8f7ca: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8f7d0: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8f7d6: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8f7dc: add rsp, 0xd8
140a8f7e3: pop rbx
140a8f7e4: pop rdi
140a8f7e5: pop rsi
140a8f7e6: pop r12
140a8f7e8: pop r13
140a8f7ea: pop r14
140a8f7ec: pop r15
140a8f7ee: pop rbp
140a8f7ef: ret 
140a8f7f0: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8f7f8: movaps xmm7, xmmword ptr [rsp + 0xb0]
140a8f800: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8f809: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8f812: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8f81b: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8f821: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8f827: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8f82d: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8f833: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8f839: add rsp, 0xd8
140a8f840: pop rbx
140a8f841: pop rdi
140a8f842: pop rsi
140a8f843: pop r12
140a8f845: pop r13
140a8f847: pop r14
140a8f849: pop r15
140a8f84b: pop rbp
140a8f84c: ret 
140a8f84d: nop dword ptr [rax]
140a8f850: mov qword ptr [rsp + 0x10], rdx
140a8f855: push rbp
140a8f856: push r15
140a8f858: push r14
140a8f85a: push r13
140a8f85c: push r12
140a8f85e: push rsi
140a8f85f: push rdi
140a8f860: push rbx
140a8f861: sub rsp, 0xd8
140a8f868: lea rbp, [rdx + 0x80]
140a8f86f: movaps xmmword ptr [rsp + 0x30], xmm15
140a8f875: movaps xmmword ptr [rsp + 0x40], xmm14
140a8f87b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8f881: movaps xmmword ptr [rsp + 0x60], xmm12
140a8f887: movaps xmmword ptr [rsp + 0x70], xmm11
140a8f88d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8f896: movaps xmmword ptr [rsp + 0x90], xmm9
140a8f89f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8f8a8: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8f8b1: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8f8b9: lea rcx, [rbp + 0x968]
140a8f8c0: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8f8c5: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8f8cd: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8f8d6: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8f8df: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8f8e8: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8f8f1: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8f8f7: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8f8fd: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8f903: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8f909: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8f90f: add rsp, 0xd8
140a8f916: pop rbx
140a8f917: pop rdi
140a8f918: pop rsi
140a8f919: pop r12
140a8f91b: pop r13
140a8f91d: pop r14
140a8f91f: pop r15
140a8f921: pop rbp
140a8f922: ret 
140a8f923: nop word ptr cs:[rax + rax]
140a8f930: mov qword ptr [rsp + 0x10], rdx
140a8f935: push rbp
140a8f936: push r15
140a8f938: push r14
140a8f93a: push r13
140a8f93c: push r12
140a8f93e: push rsi
140a8f93f: push rdi
140a8f940: push rbx
140a8f941: sub rsp, 0xd8
140a8f948: lea rbp, [rdx + 0x80]
140a8f94f: movaps xmmword ptr [rsp + 0x30], xmm15
140a8f955: movaps xmmword ptr [rsp + 0x40], xmm14
140a8f95b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8f961: movaps xmmword ptr [rsp + 0x60], xmm12
140a8f967: movaps xmmword ptr [rsp + 0x70], xmm11
140a8f96d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8f976: movaps xmmword ptr [rsp + 0x90], xmm9
140a8f97f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8f988: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8f991: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8f999: lea rcx, [rbp + 0x978]
140a8f9a0: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8f9a5: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8f9ad: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8f9b6: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8f9bf: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8f9c8: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8f9d1: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8f9d7: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8f9dd: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8f9e3: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8f9e9: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8f9ef: add rsp, 0xd8
140a8f9f6: pop rbx
140a8f9f7: pop rdi
140a8f9f8: pop rsi
140a8f9f9: pop r12
140a8f9fb: pop r13
140a8f9fd: pop r14
140a8f9ff: pop r15
140a8fa01: pop rbp
140a8fa02: ret 
140a8fa03: nop word ptr cs:[rax + rax]
140a8fa10: mov qword ptr [rsp + 0x10], rdx
140a8fa15: push rbp
140a8fa16: push r15
140a8fa18: push r14
140a8fa1a: push r13
140a8fa1c: push r12
140a8fa1e: push rsi
140a8fa1f: push rdi
140a8fa20: push rbx
140a8fa21: sub rsp, 0xd8
140a8fa28: lea rbp, [rdx + 0x80]
140a8fa2f: movaps xmmword ptr [rsp + 0x30], xmm15
140a8fa35: movaps xmmword ptr [rsp + 0x40], xmm14
140a8fa3b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8fa41: movaps xmmword ptr [rsp + 0x60], xmm12
140a8fa47: movaps xmmword ptr [rsp + 0x70], xmm11
140a8fa4d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8fa56: movaps xmmword ptr [rsp + 0x90], xmm9
140a8fa5f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8fa68: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8fa71: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8fa79: lea rcx, [rbp + 0x988]
140a8fa80: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8fa85: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8fa8d: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8fa96: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8fa9f: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8faa8: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8fab1: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8fab7: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8fabd: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8fac3: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8fac9: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8facf: add rsp, 0xd8
140a8fad6: pop rbx
140a8fad7: pop rdi
140a8fad8: pop rsi
140a8fad9: pop r12
140a8fadb: pop r13
140a8fadd: pop r14
140a8fadf: pop r15
140a8fae1: pop rbp
140a8fae2: ret 
140a8fae3: nop word ptr cs:[rax + rax]
140a8faf0: mov qword ptr [rsp + 0x10], rdx
140a8faf5: push rbp
140a8faf6: push r15
140a8faf8: push r14
140a8fafa: push r13
140a8fafc: push r12
140a8fafe: push rsi
140a8faff: push rdi
140a8fb00: push rbx
140a8fb01: sub rsp, 0xd8
140a8fb08: lea rbp, [rdx + 0x80]
140a8fb0f: movaps xmmword ptr [rsp + 0x30], xmm15
140a8fb15: movaps xmmword ptr [rsp + 0x40], xmm14
140a8fb1b: movaps xmmword ptr [rsp + 0x50], xmm13
140a8fb21: movaps xmmword ptr [rsp + 0x60], xmm12
140a8fb27: movaps xmmword ptr [rsp + 0x70], xmm11
140a8fb2d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8fb36: movaps xmmword ptr [rsp + 0x90], xmm9
140a8fb3f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8fb48: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8fb51: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8fb59: lea rcx, [rbp + 0x998]
140a8fb60: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8fb65: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8fb6d: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8fb76: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8fb7f: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8fb88: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8fb91: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8fb97: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8fb9d: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8fba3: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8fba9: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8fbaf: add rsp, 0xd8
140a8fbb6: pop rbx
140a8fbb7: pop rdi
140a8fbb8: pop rsi
140a8fbb9: pop r12
140a8fbbb: pop r13
140a8fbbd: pop r14
140a8fbbf: pop r15
140a8fbc1: pop rbp
140a8fbc2: ret 
140a8fbc3: nop word ptr cs:[rax + rax]
140a8fbd0: mov qword ptr [rsp + 0x10], rdx
140a8fbd5: push rbp
140a8fbd6: push r15
140a8fbd8: push r14
140a8fbda: push r13
140a8fbdc: push r12
140a8fbde: push rsi
140a8fbdf: push rdi
140a8fbe0: push rbx
140a8fbe1: sub rsp, 0xd8
140a8fbe8: lea rbp, [rdx + 0x80]
140a8fbef: movaps xmmword ptr [rsp + 0x30], xmm15
140a8fbf5: movaps xmmword ptr [rsp + 0x40], xmm14
140a8fbfb: movaps xmmword ptr [rsp + 0x50], xmm13
140a8fc01: movaps xmmword ptr [rsp + 0x60], xmm12
140a8fc07: movaps xmmword ptr [rsp + 0x70], xmm11
140a8fc0d: movaps xmmword ptr [rsp + 0x80], xmm10
140a8fc16: movaps xmmword ptr [rsp + 0x90], xmm9
140a8fc1f: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8fc28: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8fc31: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8fc39: lea rcx, [rbp + 0x9a8]
140a8fc40: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8fc45: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8fc4d: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8fc56: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8fc5f: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8fc68: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8fc71: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8fc77: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8fc7d: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8fc83: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8fc89: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8fc8f: add rsp, 0xd8
140a8fc96: pop rbx
140a8fc97: pop rdi
140a8fc98: pop rsi
140a8fc99: pop r12
140a8fc9b: pop r13
140a8fc9d: pop r14
140a8fc9f: pop r15
140a8fca1: pop rbp
140a8fca2: ret 
140a8fca3: nop word ptr cs:[rax + rax]
140a8fcb0: mov qword ptr [rsp + 0x10], rdx
140a8fcb5: push rbp
140a8fcb6: push r15
140a8fcb8: push r14
140a8fcba: push r13
140a8fcbc: push r12
140a8fcbe: push rsi
140a8fcbf: push rdi
140a8fcc0: push rbx
140a8fcc1: sub rsp, 0xd8
140a8fcc8: lea rbp, [rdx + 0x80]
140a8fccf: movaps xmmword ptr [rsp + 0x30], xmm15
140a8fcd5: movaps xmmword ptr [rsp + 0x40], xmm14
140a8fcdb: movaps xmmword ptr [rsp + 0x50], xmm13
140a8fce1: movaps xmmword ptr [rsp + 0x60], xmm12
140a8fce7: movaps xmmword ptr [rsp + 0x70], xmm11
140a8fced: movaps xmmword ptr [rsp + 0x80], xmm10
140a8fcf6: movaps xmmword ptr [rsp + 0x90], xmm9
140a8fcff: movaps xmmword ptr [rsp + 0xa0], xmm8
140a8fd08: movdqa xmmword ptr [rsp + 0xb0], xmm7
140a8fd11: movaps xmmword ptr [rsp + 0xc0], xmm6
140a8fd19: lea rcx, [rbp + 0x9b8]
140a8fd20: call 0x14087f390 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo> >
140a8fd25: movaps xmm6, xmmword ptr [rsp + 0xc0]
140a8fd2d: movdqa xmm7, xmmword ptr [rsp + 0xb0]
140a8fd36: movaps xmm8, xmmword ptr [rsp + 0xa0]
140a8fd3f: movaps xmm9, xmmword ptr [rsp + 0x90]
140a8fd48: movaps xmm10, xmmword ptr [rsp + 0x80]
140a8fd51: movaps xmm11, xmmword ptr [rsp + 0x70]
140a8fd57: movaps xmm12, xmmword ptr [rsp + 0x60]
140a8fd5d: movaps xmm13, xmmword ptr [rsp + 0x50]
140a8fd63: movaps xmm14, xmmword ptr [rsp + 0x40]
140a8fd69: movaps xmm15, xmmword ptr [rsp + 0x30]
140a8fd6f: add rsp, 0xd8
140a8fd76: pop rbx
140a8fd77: pop rdi
140a8fd78: pop rsi
140a8fd79: pop r12
140a8fd7b: pop r13
140a8fd7d: pop r14
140a8fd7f: pop r15
140a8fd81: pop rbp
140a8fd82: ret 
