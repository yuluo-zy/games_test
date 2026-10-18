141ce5900: 55 push rbp
141ce5901: 4157 push r15
141ce5903: 4156 push r14
141ce5905: 4155 push r13
141ce5907: 4154 push r12
141ce5909: 56 push rsi
141ce590a: 57 push rdi
141ce590b: 53 push rbx
141ce590c: 4881ec58010000 sub rsp, 0x158
141ce5913: 488dac2480000000 lea rbp, [rsp + 0x80]
141ce591b: 440f299dc0000000 movaps xmmword ptr [rbp + 0xc0], xmm11
141ce5923: 440f2995b0000000 movaps xmmword ptr [rbp + 0xb0], xmm10
141ce592b: 440f298da0000000 movaps xmmword ptr [rbp + 0xa0], xmm9
141ce5933: 440f298590000000 movaps xmmword ptr [rbp + 0x90], xmm8
141ce593b: 0f29bd80000000 movaps xmmword ptr [rbp + 0x80], xmm7
141ce5942: 0f297570 movaps xmmword ptr [rbp + 0x70], xmm6
141ce5946: 48c74568feffffff mov qword ptr [rbp + 0x68], 0xfffffffffffffffe
141ce594e: 4c89cb mov rbx, r9
141ce5951: 4c89c6 mov rsi, r8
141ce5954: 4889d7 mov rdi, rdx
141ce5957: e8a410f3fe call 0x140c16a00 ; puffin::are_scopes_on
141ce595c: 4189c5 mov r13d, eax
141ce595f: 84c0 test al, al
141ce5961: 745d je 0x141ce59c0
141ce5963: 41bf51000000 mov r15d, 0x51
141ce5969: b951000000 mov ecx, 0x51
141ce596e: e85dfbffff call 0x141ce54d0 ; core::str::impl$0::rfind<ref$<str$> >
141ce5973: 4c8d3506052501 lea r14, [rip + 0x1250506] ; @142f35e80 'system_clutter::autoclutter::add_preplaced_autoclutter::add_preplaced_autoclutter::f'
141ce597a: 4883f801 cmp rax, 1
141ce597e: 0f85a8000000 jne 0x141ce5a2c
141ce5984: 4989d1 mov r9, rdx
141ce5987: 4885d2 test rdx, rdx
141ce598a: 743d je 0x141ce59c9
141ce598c: 4983f951 cmp r9, 0x51
141ce5990: 7335 jae 0x141ce59c7
141ce5992: 488d05e7042501 lea rax, [rip + 0x12504e7] ; @142f35e80 'system_clutter::autoclutter::add_preplaced_autoclutter::add_preplaced_autoclutter::f'
141ce5999: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
141ce599e: 7f29 jg 0x141ce59c9
141ce59a0: 488d05b1052501 lea rax, [rip + 0x12505b1]
141ce59a7: 4889442420 mov qword ptr [rsp + 0x20], rax
141ce59ac: 488d0dcd042501 lea rcx, [rip + 0x12504cd] ; @142f35e80 'system_clutter::autoclutter::add_preplaced_autoclutter::add_preplaced_autoclutter::f'
141ce59b3: ba51000000 mov edx, 0x51
141ce59b8: 4531c0 xor r8d, r8d
141ce59bb: e8803fbf00 call 0x1428d9940 ; core::str::slice_error_fail
141ce59c0: 31c9 xor ecx, ecx
141ce59c2: e9d5010000 jmp 0x141ce5b9c
141ce59c7: 75d7 jne 0x141ce59a0
141ce59c9: 4c89c9 mov rcx, r9
141ce59cc: e8fffaffff call 0x141ce54d0 ; core::str::impl$0::rfind<ref$<str$> >
141ce59d1: 4883f801 cmp rax, 1
141ce59d5: 7555 jne 0x141ce5a2c
141ce59d7: 4989d0 mov r8, rdx
141ce59da: 4983c002 add r8, 2
141ce59de: 7439 je 0x141ce5a19
141ce59e0: 4983f851 cmp r8, 0x51
141ce59e4: 7331 jae 0x141ce5a17
141ce59e6: 488d0593042501 lea rax, [rip + 0x1250493] ; @142f35e80 'system_clutter::autoclutter::add_preplaced_autoclutter::add_preplaced_autoclutter::f'
141ce59ed: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
141ce59f2: 7f25 jg 0x141ce5a19
141ce59f4: 488d0575052501 lea rax, [rip + 0x1250575]
141ce59fb: 4889442420 mov qword ptr [rsp + 0x20], rax
141ce5a00: 488d0d79042501 lea rcx, [rip + 0x1250479] ; @142f35e80 'system_clutter::autoclutter::add_preplaced_autoclutter::add_preplaced_autoclutter::f'
141ce5a07: ba51000000 mov edx, 0x51
141ce5a0c: 41b951000000 mov r9d, 0x51
141ce5a12: e8293fbf00 call 0x1428d9940 ; core::str::slice_error_fail
141ce5a17: 75db jne 0x141ce59f4
141ce5a19: 41bf4f000000 mov r15d, 0x4f
141ce5a1f: 4929d7 sub r15, rdx
141ce5a22: 4c8d3557042501 lea r14, [rip + 0x1250457] ; @142f35e80 'system_clutter::autoclutter::add_preplaced_autoclutter::add_preplaced_autoclutter::f'
141ce5a29: 4d01c6 add r14, r8
141ce5a2c: 4c8d05b0052501 lea r8, [rip + 0x12505b0] ; @142f35fe3 'preplaced_clutter/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\\library\\core\\src\\str\\pattern.rs'
141ce5a33: 4c8d2566052501 lea r12, [rip + 0x1250566] ; @142f35fa0 'crates/systems/clutter/src/autoclutter/add_preplaced_autoclutter.rspreplaced_clutter/rustc/05f98'
141ce5a3a: 48895d58 mov qword ptr [rbp + 0x58], rbx
141ce5a3e: 6690 nop 
141ce5a40: 410fbe40ff movsx eax, byte ptr [r8 - 1]
141ce5a45: 85c0 test eax, eax
141ce5a47: 7817 js 0x141ce5a60
141ce5a49: 49ffc8 dec r8
141ce5a4c: 83f85c cmp eax, 0x5c
141ce5a4f: 7563 jne 0x141ce5ab4
141ce5a51: eb7c jmp 0x141ce5acf
141ce5a53: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141ce5a60: 410fb648fe movzx ecx, byte ptr [r8 - 2]
141ce5a65: 80f9c0 cmp cl, 0xc0
141ce5a68: 7d23 jge 0x141ce5a8d
141ce5a6a: 410fb650fd movzx edx, byte ptr [r8 - 3]
141ce5a6f: 80fac0 cmp dl, 0xc0
141ce5a72: 7d22 jge 0x141ce5a96
141ce5a74: 450fb648fc movzx r9d, byte ptr [r8 - 4]
141ce5a79: 4983c0fc add r8, -4
141ce5a7d: 4183e107 and r9d, 7
141ce5a81: 41c1e106 shl r9d, 6
141ce5a85: 83e23f and edx, 0x3f
141ce5a88: 4409ca or edx, r9d
141ce5a8b: eb10 jmp 0x141ce5a9d
141ce5a8d: 4983c0fe add r8, -2
141ce5a91: 83e11f and ecx, 0x1f
141ce5a94: eb0f jmp 0x141ce5aa5
141ce5a96: 4983c0fd add r8, -3
141ce5a9a: 83e20f and edx, 0xf
141ce5a9d: c1e206 shl edx, 6
141ce5aa0: 83e13f and ecx, 0x3f
141ce5aa3: 09d1 or ecx, edx
141ce5aa5: c1e106 shl ecx, 6
141ce5aa8: 243f and al, 0x3f
141ce5aaa: 0fb6c0 movzx eax, al
141ce5aad: 09c8 or eax, ecx
141ce5aaf: 83f85c cmp eax, 0x5c
141ce5ab2: 741b je 0x141ce5acf
141ce5ab4: 83f82f cmp eax, 0x2f
141ce5ab7: 7416 je 0x141ce5acf
141ce5ab9: 4d39e0 cmp r8, r12
141ce5abc: 7582 jne 0x141ce5a40
141ce5abe: 4889f3 mov rbx, rsi
141ce5ac1: 4889fe mov rsi, rdi
141ce5ac4: 4489ef mov edi, r13d
141ce5ac7: 41bd43000000 mov r13d, 0x43
141ce5acd: eb56 jmp 0x141ce5b25
141ce5acf: 4c8d25ca042501 lea r12, [rip + 0x12504ca] ; @142f35fa0 'crates/systems/clutter/src/autoclutter/add_preplaced_autoclutter.rspreplaced_clutter/rustc/05f98'
141ce5ad6: 4d29e0 sub r8, r12
141ce5ad9: 49ffc0 inc r8
141ce5adc: 7432 je 0x141ce5b10
141ce5ade: 4983f843 cmp r8, 0x43
141ce5ae2: 732a jae 0x141ce5b0e
141ce5ae4: 43803c20bf cmp byte ptr [r8 + r12], 0xbf
141ce5ae9: 7f25 jg 0x141ce5b10
141ce5aeb: 488d0546042501 lea rax, [rip + 0x1250446]
141ce5af2: 4889442420 mov qword ptr [rsp + 0x20], rax
141ce5af7: 488d0da2042501 lea rcx, [rip + 0x12504a2] ; @142f35fa0 'crates/systems/clutter/src/autoclutter/add_preplaced_autoclutter.rspreplaced_clutter/rustc/05f98'
141ce5afe: ba43000000 mov edx, 0x43
141ce5b03: 41b943000000 mov r9d, 0x43
141ce5b09: e8323ebf00 call 0x1428d9940 ; core::str::slice_error_fail
141ce5b0e: 75db jne 0x141ce5aeb
141ce5b10: 4889f3 mov rbx, rsi
141ce5b13: 4889fe mov rsi, rdi
141ce5b16: 4489ef mov edi, r13d
141ce5b19: 41bd43000000 mov r13d, 0x43
141ce5b1f: 4d29c5 sub r13, r8
141ce5b22: 4d01c4 add r12, r8
141ce5b25: e80626f3fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
141ce5b2a: 4889c1 mov rcx, rax
141ce5b2d: 488b00 mov rax, qword ptr [rax]
141ce5b30: 4883f801 cmp rax, 1
141ce5b34: 0f8583030000 jne 0x141ce5ebd
141ce5b3a: 4883c108 add rcx, 8
141ce5b3e: 48833900 cmp qword ptr [rcx], 0
141ce5b42: 0f85d7030000 jne 0x141ce5f1f
141ce5b48: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
141ce5b4f: 48894d50 mov qword ptr [rbp + 0x50], rcx
141ce5b53: 4883c108 add rcx, 8
141ce5b57: 4c896c2420 mov qword ptr [rsp + 0x20], r13
141ce5b5c: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
141ce5b65: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
141ce5b6e: 4c89f2 mov rdx, r14
141ce5b71: 4d89f8 mov r8, r15
141ce5b74: 4d89e1 mov r9, r12
141ce5b77: e8c411f3fe call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
141ce5b7c: 4989c7 mov r15, rax
141ce5b7f: 488b4550 mov rax, qword ptr [rbp + 0x50]
141ce5b83: 48ff00 inc qword ptr [rax]
141ce5b86: 4c897d00 mov qword ptr [rbp], r15
141ce5b8a: b901000000 mov ecx, 1
141ce5b8f: 4189fd mov r13d, edi
141ce5b92: 4889f7 mov rdi, rsi
141ce5b95: 4889de mov rsi, rbx
141ce5b98: 488b5d58 mov rbx, qword ptr [rbp + 0x58]
141ce5b9c: 488b8548010000 mov rax, qword ptr [rbp + 0x148]
141ce5ba3: 48894df8 mov qword ptr [rbp - 8], rcx
141ce5ba7: 488b0b mov rcx, qword ptr [rbx]
141ce5baa: 488b5308 mov rdx, qword ptr [rbx + 8]
141ce5bae: 4c8b01 mov r8, qword ptr [rcx]
141ce5bb1: 4531c9 xor r9d, r9d
141ce5bb4: 4d89c2 mov r10, r8
141ce5bb7: 4c2b5218 sub r10, qword ptr [rdx + 0x18]
141ce5bbb: 4d0f42d1 cmovb r10, r9
141ce5bbf: 4c2b4238 sub r8, qword ptr [rdx + 0x38]
141ce5bc3: 4d0f42c1 cmovb r8, r9
141ce5bc7: 4c8b5a30 mov r11, qword ptr [rdx + 0x30]
141ce5bcb: 4d29c3 sub r11, r8
141ce5bce: 4d0f42d9 cmovb r11, r9
141ce5bd2: 410f96c0 setbe r8b
141ce5bd6: 488b5a10 mov rbx, qword ptr [rdx + 0x10]
141ce5bda: 4c29d3 sub rbx, r10
141ce5bdd: 490f42d9 cmovb rbx, r9
141ce5be1: 488b5240 mov rdx, qword ptr [rdx + 0x40]
141ce5be5: 410f96c1 setbe r9b
141ce5be9: 4c01db add rbx, r11
141ce5bec: 4989d2 mov r10, rdx
141ce5bef: 4929da sub r10, rbx
141ce5bf2: 4c8911 mov qword ptr [rcx], r10
141ce5bf5: 4584c1 test r9b, r8b
141ce5bf8: 0f8424020000 je 0x141ce5e22
141ce5bfe: 488b9540010000 mov rdx, qword ptr [rbp + 0x140]
141ce5c05: 488b0a mov rcx, qword ptr [rdx]
141ce5c08: 488b5208 mov rdx, qword ptr [rdx + 8]
141ce5c0c: 4c8b01 mov r8, qword ptr [rcx]
141ce5c0f: 4531c9 xor r9d, r9d
141ce5c12: 4d89c2 mov r10, r8
141ce5c15: 4c2b5218 sub r10, qword ptr [rdx + 0x18]
141ce5c19: 4d0f42d1 cmovb r10, r9
141ce5c1d: 4c2b4238 sub r8, qword ptr [rdx + 0x38]
141ce5c21: 4d0f42c1 cmovb r8, r9
141ce5c25: 4c8b5a30 mov r11, qword ptr [rdx + 0x30]
141ce5c29: 4d29c3 sub r11, r8
141ce5c2c: 4d0f42d9 cmovb r11, r9
141ce5c30: 410f96c0 setbe r8b
141ce5c34: 488b5a10 mov rbx, qword ptr [rdx + 0x10]
141ce5c38: 4c29d3 sub rbx, r10
141ce5c3b: 490f42d9 cmovb rbx, r9
141ce5c3f: 488b5240 mov rdx, qword ptr [rdx + 0x40]
141ce5c43: 410f96c1 setbe r9b
141ce5c47: 4c01db add rbx, r11
141ce5c4a: 4989d2 mov r10, rdx
141ce5c4d: 4929da sub r10, rbx
141ce5c50: 4c8911 mov qword ptr [rcx], r10
141ce5c53: 4584c1 test r9b, r8b
141ce5c56: 0f84c6010000 je 0x141ce5e22
141ce5c5c: 803800 cmp byte ptr [rax], 0
141ce5c5f: 0f8449020000 je 0x141ce5eae
141ce5c65: 4c897d48 mov qword ptr [rbp + 0x48], r15
141ce5c69: 44886d67 mov byte ptr [rbp + 0x67], r13b
141ce5c6d: 488b0f mov rcx, qword ptr [rdi]
141ce5c70: 488b4110 mov rax, qword ptr [rcx + 0x10]
141ce5c74: 4885c0 test rax, rax
141ce5c77: 0f845e010000 je 0x141ce5ddb
141ce5c7d: 4c8b6108 mov r12, qword ptr [rcx + 8]
141ce5c81: 8b4e1c mov ecx, dword ptr [rsi + 0x1c]
141ce5c84: 894d50 mov dword ptr [rbp + 0x50], ecx
141ce5c87: 488b0e mov rcx, qword ptr [rsi]
141ce5c8a: 48894d20 mov qword ptr [rbp + 0x20], rcx
141ce5c8e: 488b4e10 mov rcx, qword ptr [rsi + 0x10]
141ce5c92: 48894d28 mov qword ptr [rbp + 0x28], rcx
141ce5c96: 498d4c2408 lea rcx, [r12 + 8]
141ce5c9b: 48894d58 mov qword ptr [rbp + 0x58], rcx
141ce5c9f: 488d0cc0 lea rcx, [rax + rax*8]
141ce5ca3: 4c8d2c49 lea r13, [rcx + rcx*2]
141ce5ca7: 4901c5 add r13, rax
141ce5caa: 31db xor ebx, ebx
141ce5cac: f30f103550fcc300 movss xmm6, dword ptr [rip + 0xc3fc50]
141ce5cb4: 4c8d75b8 lea r14, [rbp - 0x48]
141ce5cb8: f30f103d2844c700 movss xmm7, dword ptr [rip + 0xc74428]
141ce5cc0: 440f28052815d300 movaps xmm8, xmmword ptr [rip + 0xd31528]
141ce5cc8: 31ff xor edi, edi
141ce5cca: 660f1f440000 nop word ptr [rax + rax]
141ce5cd0: 488b4558 mov rax, qword ptr [rbp + 0x58]
141ce5cd4: 488d1418 lea rdx, [rax + rbx]
141ce5cd8: 488d4d30 lea rcx, [rbp + 0x30]
141ce5cdc: 0f28d6 movaps xmm2, xmm6
141ce5cdf: e8fc70fbfe call 0x140c9cde0 ; utils::swizzlers::impl$1::xvy
141ce5ce4: f2450f100c1c movsd xmm9, qword ptr [r12 + rbx]
141ce5cea: f2450f10541c10 movsd xmm10, qword ptr [r12 + rbx + 0x10]
141ce5cf1: 488b05a0639401 mov rax, qword ptr [rip + 0x19463a0]
141ce5cf8: 448a38 mov r15b, byte ptr [rax]
141ce5cfb: f3410f12c2 movsldup xmm0, xmm10
141ce5d00: 410f59c1 mulps xmm0, xmm9
141ce5d04: f3410f16ca movshdup xmm1, xmm10
141ce5d09: 410f28d1 movaps xmm2, xmm9
141ce5d0d: 410fc6d1e1 shufps xmm2, xmm9, 0xe1
141ce5d12: 0f59d1 mulps xmm2, xmm1
141ce5d15: f20fd0c2 addsubps xmm0, xmm2
141ce5d19: 0f134508 movlps qword ptr [rbp + 8], xmm0
141ce5d1d: 4c89f1 mov rcx, r14
141ce5d20: 488d5508 lea rdx, [rbp + 8]
141ce5d24: e89770fbfe call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
141ce5d29: 31c0 xor eax, eax
141ce5d2b: 4584ff test r15b, r15b
141ce5d2e: 0f94c0 sete al
141ce5d31: 448d3c80 lea r15d, [rax + rax*4]
141ce5d35: f3440f105dc0 movss xmm11, dword ptr [rbp - 0x40]
141ce5d3b: f3440f59df mulss xmm11, xmm7
141ce5d40: 450f16ca movlhps xmm9, xmm10
141ce5d44: f20f1045b8 movsd xmm0, qword ptr [rbp - 0x48]
141ce5d49: 410f59c0 mulps xmm0, xmm8
141ce5d4d: f2440f105530 movsd xmm10, qword ptr [rbp + 0x30]
141ce5d53: 440f58d0 addps xmm10, xmm0
141ce5d57: f3440f585d38 addss xmm11, dword ptr [rbp + 0x38]
141ce5d5d: 418a741c18 mov sil, byte ptr [r12 + rbx + 0x18]
141ce5d62: 4080c602 add sil, 2
141ce5d66: 8b4550 mov eax, dword ptr [rbp + 0x50]
141ce5d69: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
141ce5d6d: 8901 mov dword ptr [rcx], eax
141ce5d6f: 488d056d022501 lea rax, [rip + 0x125026d] ; @142f35fe3 'preplaced_clutter/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\\library\\core\\src\\str\\pattern.rs'
141ce5d76: 48894530 mov qword ptr [rbp + 0x30], rax
141ce5d7a: 48c7453811000000 mov qword ptr [rbp + 0x38], 0x11
141ce5d82: 48897d40 mov qword ptr [rbp + 0x40], rdi
141ce5d86: 488d4d30 lea rcx, [rbp + 0x30]
141ce5d8a: e841130100 call 0x141cf70d0 ; utils::calculate_hash<tuple$<ref$<str$>,usize> >
141ce5d8f: 48c745b805000000 mov qword ptr [rbp - 0x48], 5
141ce5d97: 440f114dd0 movups xmmword ptr [rbp - 0x30], xmm9
141ce5d9c: 440f1355e0 movlps qword ptr [rbp - 0x20], xmm10
141ce5da1: f3440f115de8 movss dword ptr [rbp - 0x18], xmm11
141ce5da7: 897dec mov dword ptr [rbp - 0x14], edi
141ce5daa: 408875f0 mov byte ptr [rbp - 0x10], sil
141ce5dae: 44887df1 mov byte ptr [rbp - 0xf], r15b
141ce5db2: 66c745f2ffff mov word ptr [rbp - 0xe], 0xffff
141ce5db8: c645f400 mov byte ptr [rbp - 0xc], 0
141ce5dbc: 488b4d20 mov rcx, qword ptr [rbp + 0x20]
141ce5dc0: 4889c2 mov rdx, rax
141ce5dc3: 4d89f0 mov r8, r14
141ce5dc6: e805e21fff call 0x140ee3fd0 ; system_clutter::autoclutter::AutoClutterState::add_region
141ce5dcb: 4883c31c add rbx, 0x1c
141ce5dcf: 48ffc7 inc rdi
141ce5dd2: 4939dd cmp r13, rbx
141ce5dd5: 0f85f5feffff jne 0x141ce5cd0
141ce5ddb: 807d6700 cmp byte ptr [rbp + 0x67], 0
141ce5ddf: 0f848a000000 je 0x141ce5e6f
141ce5de5: e84623f3fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
141ce5dea: 4889c1 mov rcx, rax
141ce5ded: 488b00 mov rax, qword ptr [rax]
141ce5df0: 4883f801 cmp rax, 1
141ce5df4: 0f85f7000000 jne 0x141ce5ef1
141ce5dfa: 4883c108 add rcx, 8
141ce5dfe: 488b5548 mov rdx, qword ptr [rbp + 0x48]
141ce5e02: 48833900 cmp qword ptr [rcx], 0
141ce5e06: 0f8513010000 jne 0x141ce5f1f
141ce5e0c: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
141ce5e13: 48894d58 mov qword ptr [rbp + 0x58], rcx
141ce5e17: 4883c108 add rcx, 8
141ce5e1b: e8d012f3fe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
141ce5e20: eb46 jmp 0x141ce5e68
141ce5e22: 488911 mov qword ptr [rcx], rdx
141ce5e25: 31c9 xor ecx, ecx
141ce5e27: 8808 mov byte ptr [rax], cl
141ce5e29: 4584ed test r13b, r13b
141ce5e2c: 7441 je 0x141ce5e6f
141ce5e2e: e8fd22f3fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
141ce5e33: 4889c1 mov rcx, rax
141ce5e36: 488b00 mov rax, qword ptr [rax]
141ce5e39: 4883f801 cmp rax, 1
141ce5e3d: 0f8594000000 jne 0x141ce5ed7
141ce5e43: 4883c108 add rcx, 8
141ce5e47: 48833900 cmp qword ptr [rcx], 0
141ce5e4b: 0f85ce000000 jne 0x141ce5f1f
141ce5e51: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
141ce5e58: 48894d58 mov qword ptr [rbp + 0x58], rcx
141ce5e5c: 4883c108 add rcx, 8
141ce5e60: 4c89fa mov rdx, r15
141ce5e63: e88812f3fe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
141ce5e68: 488b4558 mov rax, qword ptr [rbp + 0x58]
141ce5e6c: 48ff00 inc qword ptr [rax]
141ce5e6f: 0f287570 movaps xmm6, xmmword ptr [rbp + 0x70]
141ce5e73: 0f28bd80000000 movaps xmm7, xmmword ptr [rbp + 0x80]
141ce5e7a: 440f288590000000 movaps xmm8, xmmword ptr [rbp + 0x90]
141ce5e82: 440f288da0000000 movaps xmm9, xmmword ptr [rbp + 0xa0]
141ce5e8a: 440f2895b0000000 movaps xmm10, xmmword ptr [rbp + 0xb0]
141ce5e92: 440f289dc0000000 movaps xmm11, xmmword ptr [rbp + 0xc0]
141ce5e9a: 4881c458010000 add rsp, 0x158
141ce5ea1: 5b pop rbx
141ce5ea2: 5f pop rdi
141ce5ea3: 5e pop rsi
141ce5ea4: 415c pop r12
141ce5ea6: 415d pop r13
141ce5ea8: 415e pop r14
141ce5eaa: 415f pop r15
141ce5eac: 5d pop rbp
141ce5ead: c3 ret 
141ce5eae: b101 mov cl, 1
141ce5eb0: 8808 mov byte ptr [rax], cl
141ce5eb2: 4584ed test r13b, r13b
141ce5eb5: 0f8573ffffff jne 0x141ce5e2e
141ce5ebb: ebb2 jmp 0x141ce5e6f
141ce5ebd: 4885c0 test rax, rax
141ce5ec0: 7551 jne 0x141ce5f13
141ce5ec2: 31d2 xor edx, edx
141ce5ec4: e8b748c300 call 0x14291a780 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
141ce5ec9: 4889c1 mov rcx, rax
141ce5ecc: 4885c0 test rax, rax
141ce5ecf: 0f8569fcffff jne 0x141ce5b3e
141ce5ed5: eb3c jmp 0x141ce5f13
141ce5ed7: 4885c0 test rax, rax
141ce5eda: 7537 jne 0x141ce5f13
141ce5edc: 31d2 xor edx, edx
141ce5ede: e89d48c300 call 0x14291a780 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
141ce5ee3: 4889c1 mov rcx, rax
141ce5ee6: 4885c0 test rax, rax
141ce5ee9: 0f8558ffffff jne 0x141ce5e47
141ce5eef: eb22 jmp 0x141ce5f13
141ce5ef1: 4885c0 test rax, rax
141ce5ef4: 488b4548 mov rax, qword ptr [rbp + 0x48]
141ce5ef8: 7519 jne 0x141ce5f13
141ce5efa: 4889c6 mov rsi, rax
141ce5efd: 31d2 xor edx, edx
141ce5eff: e87c48c300 call 0x14291a780 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
141ce5f04: 4889f2 mov rdx, rsi
141ce5f07: 4889c1 mov rcx, rax
141ce5f0a: 4885c0 test rax, rax
141ce5f0d: 0f85effeffff jne 0x141ce5e02
141ce5f13: 488d0d962ceb00 lea rcx, [rip + 0xeb2c96]
141ce5f1a: e8b121bf00 call 0x1428d80d0 ; std::thread::local::panic_access_error
141ce5f1f: 488d0dfa2ceb00 lea rcx, [rip + 0xeb2cfa]
141ce5f26: e82533bf00 call 0x1428d9250 ; core::cell::panic_already_borrowed
141ce5f2b: cc int3 
141ce5f2c: 0f1f4000 nop dword ptr [rax]
141ce5f30: 4889542410 mov qword ptr [rsp + 0x10], rdx
141ce5f35: 55 push rbp
141ce5f36: 4157 push r15
141ce5f38: 4156 push r14
141ce5f3a: 4155 push r13
141ce5f3c: 4154 push r12
141ce5f3e: 56 push rsi
141ce5f3f: 57 push rdi
141ce5f40: 53 push rbx
141ce5f41: 4881ec98000000 sub rsp, 0x98
141ce5f48: 488daa80000000 lea rbp, [rdx + 0x80]
141ce5f4f: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
141ce5f55: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
141ce5f5b: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
141ce5f61: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
141ce5f67: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
141ce5f6c: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
141ce5f74: 488b4550 mov rax, qword ptr [rbp + 0x50]
141ce5f78: 48ff00 inc qword ptr [rax]
141ce5f7b: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
141ce5f83: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
141ce5f88: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
141ce5f8e: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
141ce5f94: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
141ce5f9a: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
141ce5fa0: 4881c498000000 add rsp, 0x98
141ce5fa7: 5b pop rbx
141ce5fa8: 5f pop rdi
141ce5fa9: 5e pop rsi
141ce5faa: 415c pop r12
141ce5fac: 415d pop r13
141ce5fae: 415e pop r14
141ce5fb0: 415f pop r15
141ce5fb2: 5d pop rbp
141ce5fb3: c3 ret 
141ce5fb4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141ce5fc0: 4889542410 mov qword ptr [rsp + 0x10], rdx
141ce5fc5: 55 push rbp
141ce5fc6: 4157 push r15
141ce5fc8: 4156 push r14
141ce5fca: 4155 push r13
141ce5fcc: 4154 push r12
141ce5fce: 56 push rsi
141ce5fcf: 57 push rdi
141ce5fd0: 53 push rbx
141ce5fd1: 4881ec98000000 sub rsp, 0x98
141ce5fd8: 488daa80000000 lea rbp, [rdx + 0x80]
141ce5fdf: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
141ce5fe5: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
141ce5feb: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
141ce5ff1: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
141ce5ff7: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
141ce5ffc: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
141ce6004: 807d6700 cmp byte ptr [rbp + 0x67], 0
141ce6008: 7420 je 0x141ce602a
141ce600a: 488d4500 lea rax, [rbp]
141ce600e: 48894518 mov qword ptr [rbp + 0x18], rax
141ce6012: 488d4518 lea rax, [rbp + 0x18]
141ce6016: 48894510 mov qword ptr [rbp + 0x10], rax
141ce601a: 488d0db7fe2401 lea rcx, [rip + 0x124feb7]
141ce6021: 488d5510 lea rdx, [rbp + 0x10]
141ce6025: e8c6ae20ff call 0x140ef0ef0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
141ce602a: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
141ce6032: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
141ce6037: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
141ce603d: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
141ce6043: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
141ce6049: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
141ce604f: 4881c498000000 add rsp, 0x98
141ce6056: 5b pop rbx
141ce6057: 5f pop rdi
141ce6058: 5e pop rsi
141ce6059: 415c pop r12
141ce605b: 415d pop r13
141ce605d: 415e pop r14
141ce605f: 415f pop r15
141ce6061: 5d pop rbp
141ce6062: c3 ret 
141ce6063: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141ce6070: 4889542410 mov qword ptr [rsp + 0x10], rdx
141ce6075: 55 push rbp
141ce6076: 4157 push r15
141ce6078: 4156 push r14
141ce607a: 4155 push r13
141ce607c: 4154 push r12
141ce607e: 56 push rsi
141ce607f: 57 push rdi
141ce6080: 53 push rbx
141ce6081: 4881ec98000000 sub rsp, 0x98
141ce6088: 488daa80000000 lea rbp, [rdx + 0x80]
141ce608f: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
141ce6095: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
141ce609b: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
141ce60a1: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
141ce60a7: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
141ce60ac: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
141ce60b4: 488b4558 mov rax, qword ptr [rbp + 0x58]
141ce60b8: 48ff00 inc qword ptr [rax]
141ce60bb: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
141ce60c3: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
141ce60c8: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
141ce60ce: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
141ce60d4: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
141ce60da: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
141ce60e0: 4881c498000000 add rsp, 0x98
141ce60e7: 5b pop rbx
141ce60e8: 5f pop rdi
141ce60e9: 5e pop rsi
141ce60ea: 415c pop r12
141ce60ec: 415d pop r13
141ce60ee: 415e pop r14
141ce60f0: 415f pop r15
141ce60f2: 5d pop rbp
141ce60f3: c3 ret 
141ce60f4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141ce6100: 4889542410 mov qword ptr [rsp + 0x10], rdx
141ce6105: 55 push rbp
141ce6106: 4157 push r15
141ce6108: 4156 push r14
141ce610a: 4155 push r13
141ce610c: 4154 push r12
141ce610e: 56 push rsi
141ce610f: 57 push rdi
141ce6110: 53 push rbx
141ce6111: 4881ec98000000 sub rsp, 0x98
141ce6118: 488daa80000000 lea rbp, [rdx + 0x80]
141ce611f: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
141ce6125: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
141ce612b: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
141ce6131: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
141ce6137: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
141ce613c: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
141ce6144: 488b4558 mov rax, qword ptr [rbp + 0x58]
141ce6148: 48ff00 inc qword ptr [rax]
141ce614b: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
141ce6153: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
141ce6158: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
141ce615e: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
141ce6164: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
141ce616a: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
141ce6170: 4881c498000000 add rsp, 0x98
141ce6177: 5b pop rbx
141ce6178: 5f pop rdi
141ce6179: 5e pop rsi
141ce617a: 415c pop r12
141ce617c: 415d pop r13
141ce617e: 415e pop r14
141ce6180: 415f pop r15
141ce6182: 5d pop rbp
141ce6183: c3 ret 
