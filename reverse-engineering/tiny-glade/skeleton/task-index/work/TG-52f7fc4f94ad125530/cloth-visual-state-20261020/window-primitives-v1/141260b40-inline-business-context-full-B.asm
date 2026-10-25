141260b40: 55 push rbp
141260b41: 4157 push r15
141260b43: 4156 push r14
141260b45: 4155 push r13
141260b47: 4154 push r12
141260b49: 56 push rsi
141260b4a: 57 push rdi
141260b4b: 53 push rbx
141260b4c: 4881eca8000000 sub rsp, 0xa8
141260b53: 488dac2480000000 lea rbp, [rsp + 0x80]
141260b5b: 48c74520feffffff mov qword ptr [rbp + 0x20], 0xfffffffffffffffe
141260b63: 4c894500 mov qword ptr [rbp], r8
141260b67: 4889d7 mov rdi, rdx
141260b6a: 4889ce mov rsi, rcx
141260b6d: 4c8b6a20 mov r13, qword ptr [rdx + 0x20]
141260b71: 4c8b7228 mov r14, qword ptr [rdx + 0x28]
141260b75: 4c89f3 mov rbx, r14
141260b78: 4c29eb sub rbx, r13
141260b7b: 4531e4 xor r12d, r12d
141260b7e: b930000000 mov ecx, 0x30
141260b83: 4889d8 mov rax, rbx
141260b86: 48f7e1 mul rcx
141260b89: 4989c7 mov r15, rax
141260b8c: 7033 jo 0x141260bc1
141260b8e: 48b8f0ffffffffffff7f movabs rax, 0x7ffffffffffffff0
141260b98: 4939c7 cmp r15, rax
141260b9b: 7724 ja 0x141260bc1
141260b9d: 4d85ff test r15, r15
141260ba0: 742e je 0x141260bd0
141260ba2: 0fb60560403d02 movzx eax, byte ptr [rip + 0x23d4060]
141260ba9: 41bc10000000 mov r12d, 0x10
141260baf: ba10000000 mov edx, 0x10
141260bb4: 4c89f9 mov rcx, r15
141260bb7: e854303bff call 0x140613c10
141260bbc: 4885c0 test rax, rax
141260bbf: 7516 jne 0x141260bd7
141260bc1: 4c89e1 mov rcx, r12
141260bc4: 4c89fa mov rdx, r15
141260bc7: 4c8b4500 mov r8, qword ptr [rbp]
141260bcb: e813846701 call 0x1428d8fe3
141260bd0: b810000000 mov eax, 0x10
141260bd5: 31db xor ebx, ebx
141260bd7: 48895d08 mov qword ptr [rbp + 8], rbx
141260bdb: 48894510 mov qword ptr [rbp + 0x10], rax
141260bdf: 488d4d18 lea rcx, [rbp + 0x18]
141260be3: 48c7451800000000 mov qword ptr [rbp + 0x18], 0
141260beb: 0f104730 movups xmm0, xmmword ptr [rdi + 0x30]
141260bef: 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
141260bf3: 0f1007 movups xmm0, xmmword ptr [rdi]
141260bf6: 0f104f10 movups xmm1, xmmword ptr [rdi + 0x10]
141260bfa: 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
141260bfe: 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
141260c02: 4c896dc0 mov qword ptr [rbp - 0x40], r13
141260c06: 4c8975c8 mov qword ptr [rbp - 0x38], r14
141260c0a: 48894de8 mov qword ptr [rbp - 0x18], rcx
141260c0e: 48c745f000000000 mov qword ptr [rbp - 0x10], 0
141260c16: 488945f8 mov qword ptr [rbp - 8], rax
141260c1a: 488d4da0 lea rcx, [rbp - 0x60]
141260c1e: 488d55e8 lea rdx, [rbp - 0x18]
141260c22: e819310300 call 0x141293d40
141260c27: 488b4518 mov rax, qword ptr [rbp + 0x18]
141260c2b: 48894610 mov qword ptr [rsi + 0x10], rax
141260c2f: 0f104508 movups xmm0, xmmword ptr [rbp + 8]
141260c33: 0f1106 movups xmmword ptr [rsi], xmm0
141260c36: 4889f0 mov rax, rsi
141260c39: 4881c4a8000000 add rsp, 0xa8
141260c40: 5b pop rbx
141260c41: 5f pop rdi
141260c42: 5e pop rsi
141260c43: 415c pop r12
141260c45: 415d pop r13
141260c47: 415e pop r14
141260c49: 415f pop r15
141260c4b: 5d pop rbp
141260c4c: c3 ret 
141260c4d: 0f1f00 nop dword ptr [rax]
141260c50: 4889542410 mov qword ptr [rsp + 0x10], rdx
141260c55: 55 push rbp
141260c56: 4157 push r15
141260c58: 4156 push r14
141260c5a: 4155 push r13
141260c5c: 4154 push r12
141260c5e: 56 push rsi
141260c5f: 57 push rdi
141260c60: 53 push rbx
141260c61: 4883ec28 sub rsp, 0x28
141260c65: 488daa80000000 lea rbp, [rdx + 0x80]
141260c6c: 488b4508 mov rax, qword ptr [rbp + 8]
141260c70: 4885c0 test rax, rax
141260c73: 7417 je 0x141260c8c
141260c75: 488b4d10 mov rcx, qword ptr [rbp + 0x10]
141260c79: 48c1e004 shl rax, 4
141260c7d: 488d1440 lea rdx, [rax + rax*2]
141260c81: 41b810000000 mov r8d, 0x10
141260c87: e8942f3bff call 0x140613c20
141260c8c: 90 nop 
141260c8d: 4883c428 add rsp, 0x28
141260c91: 5b pop rbx
141260c92: 5f pop rdi
141260c93: 5e pop rsi
141260c94: 415c pop r12
141260c96: 415d pop r13
141260c98: 415e pop r14
141260c9a: 415f pop r15
141260c9c: 5d pop rbp
141260c9d: c3 ret 
