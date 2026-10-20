1412fd550 55 push rbp
1412fd551 4157 push r15
1412fd553 4156 push r14
1412fd555 4155 push r13
1412fd557 4154 push r12
1412fd559 56 push rsi
1412fd55a 57 push rdi
1412fd55b 53 push rbx
1412fd55c 4883ec68 sub rsp, 0x68
1412fd560 488d6c2460 lea rbp, [rsp + 0x60]
1412fd565 0f297df0 movaps xmmword ptr [rbp - 0x10], xmm7
1412fd569 0f2975e0 movaps xmmword ptr [rbp - 0x20], xmm6
1412fd56d 48c745d8feffffff mov qword ptr [rbp - 0x28], 0xfffffffffffffffe
1412fd575 4c8945d0 mov qword ptr [rbp - 0x30], r8
1412fd579 4889d3 mov rbx, rdx
1412fd57c 4889ce mov rsi, rcx
1412fd57f 4c8b2a mov r13, qword ptr [rdx]
1412fd582 4c8b7208 mov r14, qword ptr [rdx + 8]
1412fd586 4c89f7 mov rdi, r14
1412fd589 4c29ef sub rdi, r13
1412fd58c 48c1ef02 shr rdi, 2
1412fd590 4531e4 xor r12d, r12d
1412fd593 b90c000000 mov ecx, 0xc
1412fd598 4889f8 mov rax, rdi
1412fd59b 48f7e1 mul rcx
1412fd59e 4989c7 mov r15, rax
1412fd5a1 0f801e010000 jo 0x1412fd6c5
1412fd5a7 48b8fcffffffffffff7f movabs rax, 0x7ffffffffffffffc
1412fd5b1 4939c7 cmp r15, rax
1412fd5b4 0f870b010000 ja 0x1412fd6c5
1412fd5ba 4d85ff test r15, r15
1412fd5bd 0f84a1000000 je 0x1412fd664
1412fd5c3 0fb6053f763302 movzx eax, byte ptr [rip + 0x233763f]
1412fd5ca 41bc04000000 mov r12d, 4
1412fd5d0 ba04000000 mov edx, 4
1412fd5d5 4c89f9 mov rcx, r15
1412fd5d8 e8336631ff call 0x140613c10
1412fd5dd 488945c8 mov qword ptr [rbp - 0x38], rax
1412fd5e1 4885c0 test rax, rax
1412fd5e4 0f84db000000 je 0x1412fd6c5
1412fd5ea 48897dd0 mov qword ptr [rbp - 0x30], rdi
1412fd5ee 488975c0 mov qword ptr [rbp - 0x40], rsi
1412fd5f2 4d39f5 cmp r13, r14
1412fd5f5 0f8487000000 je 0x1412fd682
1412fd5fb 4c8b7b10 mov r15, qword ptr [rbx + 0x10]
1412fd5ff 4c8b7318 mov r14, qword ptr [rbx + 0x18]
1412fd603 4c8b6320 mov r12, qword ptr [rbx + 0x20]
1412fd607 31db xor ebx, ebx
1412fd609 31f6 xor esi, esi
1412fd60b 0f1f440000 nop dword ptr [rax + rax]
1412fd610 418b4cb500 mov ecx, dword ptr [r13 + rsi*4]
1412fd615 4939ce cmp r14, rcx
1412fd618 0f8696000000 jbe 0x1412fd6b4
1412fd61e 488d0449 lea rax, [rcx + rcx*2]
1412fd622 f3410f103c87 movss xmm7, dword ptr [r15 + rax*4]
1412fd628 f3410f10748708 movss xmm6, dword ptr [r15 + rax*4 + 8]
1412fd62f 498b0c24 mov rcx, qword ptr [r12]
1412fd633 0f28cf movaps xmm1, xmm7
1412fd636 0f28d6 movaps xmm2, xmm6
1412fd639 e802c06eff call 0x1409e9640
1412fd63e 488d045b lea rax, [rbx + rbx*2]
1412fd642 488b4dc8 mov rcx, qword ptr [rbp - 0x38]
1412fd646 f30f113c81 movss dword ptr [rcx + rax*4], xmm7
1412fd64b f30f11448104 movss dword ptr [rcx + rax*4 + 4], xmm0
1412fd651 f30f11748108 movss dword ptr [rcx + rax*4 + 8], xmm6
1412fd657 48ffc3 inc rbx
1412fd65a 48ffc6 inc rsi
1412fd65d 4839fe cmp rsi, rdi
1412fd660 75ae jne 0x1412fd610
1412fd662 eb20 jmp 0x1412fd684
1412fd664 b804000000 mov eax, 4
1412fd669 488945c8 mov qword ptr [rbp - 0x38], rax
1412fd66d 48c745d000000000 mov qword ptr [rbp - 0x30], 0
1412fd675 488975c0 mov qword ptr [rbp - 0x40], rsi
1412fd679 4d39f5 cmp r13, r14
1412fd67c 0f8579ffffff jne 0x1412fd5fb
1412fd682 31db xor ebx, ebx
1412fd684 488b45c0 mov rax, qword ptr [rbp - 0x40]
1412fd688 488b4dd0 mov rcx, qword ptr [rbp - 0x30]
1412fd68c 488908 mov qword ptr [rax], rcx
1412fd68f 488b4dc8 mov rcx, qword ptr [rbp - 0x38]
1412fd693 48894808 mov qword ptr [rax + 8], rcx
1412fd697 48895810 mov qword ptr [rax + 0x10], rbx
1412fd69b 0f2875e0 movaps xmm6, xmmword ptr [rbp - 0x20]
1412fd69f 0f287df0 movaps xmm7, xmmword ptr [rbp - 0x10]
1412fd6a3 4883c468 add rsp, 0x68
1412fd6a7 5b pop rbx
1412fd6a8 5f pop rdi
1412fd6a9 5e pop rsi
1412fd6aa 415c pop r12
1412fd6ac 415d pop r13
1412fd6ae 415e pop r14
1412fd6b0 415f pop r15
1412fd6b2 5d pop rbp
1412fd6b3 c3 ret 
1412fd6b4 4c8d05c5129701 lea r8, [rip + 0x19712c5]
1412fd6bb 4c89f2 mov rdx, r14
1412fd6be e855be5d01 call 0x1428d9518
1412fd6c3 0f0b ud2 
1412fd6c5 4c89e1 mov rcx, r12
1412fd6c8 4c89fa mov rdx, r15
1412fd6cb 4c8b45d0 mov r8, qword ptr [rbp - 0x30]
1412fd6cf e80fb95d01 call 0x1428d8fe3
1412fd6d4 cc int3 
1412fd6d5 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
1412fd6e0 4889542410 mov qword ptr [rsp + 0x10], rdx
1412fd6e5 55 push rbp
1412fd6e6 4157 push r15
1412fd6e8 4156 push r14
1412fd6ea 4155 push r13
1412fd6ec 4154 push r12
1412fd6ee 56 push rsi
1412fd6ef 57 push rdi
1412fd6f0 53 push rbx
1412fd6f1 4883ec48 sub rsp, 0x48
1412fd6f5 488d6a60 lea rbp, [rdx + 0x60]
1412fd6f9 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
1412fd6fe 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
1412fd703 48837dd000 cmp qword ptr [rbp - 0x30], 0
1412fd708 741b je 0x1412fd725
1412fd70a 488b45d0 mov rax, qword ptr [rbp - 0x30]
1412fd70e 48c1e002 shl rax, 2
1412fd712 488d1440 lea rdx, [rax + rax*2]
1412fd716 41b804000000 mov r8d, 4
1412fd71c 488b4dc8 mov rcx, qword ptr [rbp - 0x38]
1412fd720 e8fb6431ff call 0x140613c20
1412fd725 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
1412fd72a 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
1412fd72f 4883c448 add rsp, 0x48
1412fd733 5b pop rbx
1412fd734 5f pop rdi
1412fd735 5e pop rsi
1412fd736 415c pop r12
1412fd738 415d pop r13
1412fd73a 415e pop r14
1412fd73c 415f pop r15
1412fd73e 5d pop rbp
1412fd73f c3 ret 
