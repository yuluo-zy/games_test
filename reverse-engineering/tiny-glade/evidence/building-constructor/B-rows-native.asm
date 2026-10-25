14123e110 push rbp
14123e111 push r15
14123e113 push r14
14123e115 push r13
14123e117 push r12
14123e119 push rsi
14123e11a push rdi
14123e11b push rbx
14123e11c sub rsp, 0x158
14123e123 lea rbp, [rsp + 0x80]
14123e12b movaps xmmword ptr [rbp + 0xc0], xmm15
14123e133 movaps xmmword ptr [rbp + 0xb0], xmm14
14123e13b movaps xmmword ptr [rbp + 0xa0], xmm13
14123e143 movaps xmmword ptr [rbp + 0x90], xmm12
14123e14b movaps xmmword ptr [rbp + 0x80], xmm11
14123e153 movaps xmmword ptr [rbp + 0x70], xmm10
14123e158 movaps xmmword ptr [rbp + 0x60], xmm9
14123e15d movaps xmmword ptr [rbp + 0x50], xmm8
14123e162 movaps xmmword ptr [rbp + 0x40], xmm7
14123e166 movaps xmmword ptr [rbp + 0x30], xmm6
14123e16a mov qword ptr [rbp + 0x28], 0xfffffffffffffffe
14123e172 movaps xmm6, xmm3
14123e175 mov rsi, r8
14123e178 mov r14, rdx
14123e17b mov qword ptr [rbp - 8], rcx
14123e17f call 0x140c16a00
14123e184 test al, al
14123e186 mov byte ptr [rbp + 0x20], al
14123e189 je 0x14123e1e7
14123e18b mov ebx, 0x4e
14123e190 mov ecx, 0x4e
14123e195 call 0x14123dce0
14123e19a lea r15, [rip + 0x19e0edf]
14123e1a1 cmp rax, 1
14123e1a5 jne 0x14123e252
14123e1ab mov r9, rdx
14123e1ae test rdx, rdx
14123e1b1 je 0x14123e1f0
14123e1b3 cmp r9, 0x4e
14123e1b7 jae 0x14123e1ee
14123e1b9 lea rax, [rip + 0x19e0ec0]
14123e1c0 cmp byte ptr [r9 + rax], 0xbf
14123e1c5 jg 0x14123e1f0
14123e1c7 lea rax, [rip + 0x19e0f8a]
14123e1ce mov qword ptr [rsp + 0x20], rax
14123e1d3 lea rcx, [rip + 0x19e0ea6]
14123e1da mov edx, 0x4e
14123e1df xor r8d, r8d
14123e1e2 call 0x1428d9940
14123e1e7 xor eax, eax
14123e1e9 jmp 0x14123e39c
14123e1ee jne 0x14123e1c7
14123e1f0 mov rcx, r9
14123e1f3 call 0x14123dce0
14123e1f8 cmp rax, 1
14123e1fc jne 0x14123e252
14123e1fe mov r8, rdx
14123e201 add r8, 2
14123e205 je 0x14123e240
14123e207 cmp r8, 0x4e
14123e20b jae 0x14123e23e
14123e20d lea rax, [rip + 0x19e0e6c]
14123e214 cmp byte ptr [r8 + rax], 0xbf
14123e219 jg 0x14123e240
14123e21b lea rax, [rip + 0x19e0f4e]
14123e222 mov qword ptr [rsp + 0x20], rax
14123e227 lea rcx, [rip + 0x19e0e52]
14123e22e mov edx, 0x4e
14123e233 mov r9d, 0x4e
14123e239 call 0x1428d9940
14123e23e jne 0x14123e21b
14123e240 mov ebx, 0x4c
14123e245 sub rbx, rdx
14123e248 lea r15, [rip + 0x19e0e31]
14123e24f add r15, r8
14123e252 lea r8, [rip + 0x19e0f8a]
14123e259 lea r12, [rip + 0x19e0f40]
14123e260 movsx eax, byte ptr [r8 - 1]
14123e265 test eax, eax
14123e267 js 0x14123e280
14123e269 dec r8
14123e26c cmp eax, 0x5c
14123e26f jne 0x14123e2d4
14123e271 jmp 0x14123e2e5
14123e273 nop word ptr cs:[rax + rax]
14123e280 movzx ecx, byte ptr [r8 - 2]
14123e285 cmp cl, 0xc0
14123e288 jge 0x14123e2ad
14123e28a movzx edx, byte ptr [r8 - 3]
14123e28f cmp dl, 0xc0
14123e292 jge 0x14123e2b6
14123e294 movzx r9d, byte ptr [r8 - 4]
14123e299 add r8, -4
14123e29d and r9d, 7
14123e2a1 shl r9d, 6
14123e2a5 and edx, 0x3f
14123e2a8 or edx, r9d
14123e2ab jmp 0x14123e2bd
14123e2ad add r8, -2
14123e2b1 and ecx, 0x1f
14123e2b4 jmp 0x14123e2c5
14123e2b6 add r8, -3
14123e2ba and edx, 0xf
14123e2bd shl edx, 6
14123e2c0 and ecx, 0x3f
14123e2c3 or ecx, edx
14123e2c5 shl ecx, 6
14123e2c8 and al, 0x3f
14123e2ca movzx eax, al
14123e2cd or eax, ecx
14123e2cf cmp eax, 0x5c
14123e2d2 je 0x14123e2e5
14123e2d4 cmp eax, 0x2f
14123e2d7 je 0x14123e2e5
14123e2d9 cmp r8, r12
14123e2dc jne 0x14123e260
14123e2de mov edi, 0x43
14123e2e3 jmp 0x14123e331
14123e2e5 lea r12, [rip + 0x19e0eb4]
14123e2ec sub r8, r12
14123e2ef inc r8
14123e2f2 je 0x14123e326
14123e2f4 cmp r8, 0x43
14123e2f8 jae 0x14123e324
14123e2fa cmp byte ptr [r8 + r12], 0xbf
14123e2ff jg 0x14123e326
14123e301 lea rax, [rip + 0x19e0e30]
14123e308 mov qword ptr [rsp + 0x20], rax
14123e30d lea rcx, [rip + 0x19e0e8c]
14123e314 mov edx, 0x43
14123e319 mov r9d, 0x43
14123e31f call 0x1428d9940
14123e324 jne 0x14123e301
14123e326 mov edi, 0x43
14123e32b sub rdi, r8
14123e32e add r12, r8
14123e331 call 0x140c18130
14123e336 mov rcx, rax
14123e339 mov rax, qword ptr [rax]
14123e33c cmp rax, 1
14123e340 jne 0x14123e6d4
14123e346 add rcx, 8
14123e34a cmp qword ptr [rcx], 0
14123e34e jne 0x14123e712
14123e354 mov qword ptr [rcx], 0xffffffffffffffff
14123e35b mov qword ptr [rbp + 0x18], rcx
14123e35f add rcx, 8
14123e363 mov qword ptr [rsp + 0x20], rdi
14123e368 mov qword ptr [rsp + 0x30], 0
14123e371 mov qword ptr [rsp + 0x28], 1
14123e37a mov rdx, r15
14123e37d mov r8, rbx
14123e380 mov r9, r12
14123e383 call 0x140c16d40
14123e388 mov rcx, qword ptr [rbp + 0x18]
14123e38c inc qword ptr [rcx]
14123e38f mov qword ptr [rbp + 0x18], rax
14123e393 mov qword ptr [rbp - 0x40], rax
14123e397 mov eax, 1
14123e39c mov rcx, qword ptr [rbp + 0x140]
14123e3a3 mov qword ptr [rbp - 0x48], rax
14123e3a7 mov qword ptr [rbp - 0x10], rcx
14123e3ab mov qword ptr [rbp], 0
14123e3b3 mov qword ptr [rbp + 8], 4
14123e3bb mov qword ptr [rbp + 0x10], 0
14123e3c3 lea rsi, [r14 + rsi*4]
14123e3c7 movss xmm7, dword ptr [rip + 0x183e4a5]
14123e3cf divss xmm7, xmm6
14123e3d3 movsldup xmm6, xmm6
14123e3d7 mov al, 1
14123e3d9 xorps xmm10, xmm10
14123e3dd movss xmm12, dword ptr [rip + 0x184b282]
14123e3e6 movss xmm13, dword ptr [rip + 0x176ff51]
14123e3ef movss xmm14, dword ptr [rip + 0x16e750c]
14123e3f8 movaps xmm15, xmmword ptr [rip + 0x16f0580]
14123e400 jmp 0x14123e415
14123e402 nop word ptr cs:[rax + rax]
14123e410 xor eax, eax
14123e412 mov rcx, rdi
14123e415 cmp r14, rsi
14123e418 je 0x14123e623
14123e41e lea rdx, [r14 + 4]
14123e422 test al, 1
14123e424 je 0x14123e440
14123e426 cmp rdx, rsi
14123e429 je 0x14123e623
14123e42f mov rcx, r14
14123e432 add r14, 8
14123e436 mov rdi, rdx
14123e439 jmp 0x14123e446
14123e43b nop dword ptr [rax + rax]
14123e440 mov rdi, r14
14123e443 mov r14, rdx
14123e446 movss xmm8, dword ptr [rdi]
14123e44b movss xmm9, dword ptr [rcx]
14123e450 movaps xmm11, xmm8
14123e454 subss xmm11, xmm9
14123e459 movaps xmm0, xmm11
14123e45d divss xmm0, dword ptr [rip + 0x17622ff]
14123e465 call 0x142923ee0
14123e46a cvttss2si rax, xmm0
14123e46f mov rcx, rax
14123e472 sar rcx, 0x3f
14123e476 movaps xmm1, xmm0
14123e479 subss xmm1, dword ptr [rip + 0x171bc5b]
14123e481 cvttss2si rbx, xmm1
14123e486 and rbx, rcx
14123e489 or rbx, rax
14123e48c ucomiss xmm0, xmm10
14123e490 mov eax, 0
14123e495 cmovb rbx, rax
14123e499 ucomiss xmm0, dword ptr [rip + 0x171bc40]
14123e4a0 mov rax, 0xffffffffffffffff
14123e4a7 cmova rbx, rax
14123e4ab cmp rbx, 3
14123e4af mov eax, 2
14123e4b4 cmovb rbx, rax
14123e4b8 inc rbx
14123e4bb movaps xmm2, xmm12
14123e4bf divss xmm2, xmm11
14123e4c4 lea rax, [rip + 0x19e0d1d]
14123e4cb mov qword ptr [rsp + 0x20], rax
14123e4d0 lea rcx, [rbp - 0x28]
14123e4d4 mov rdx, rbx
14123e4d7 lea r9, [rbp - 0x10]
14123e4db call 0x140c90350
14123e4e0 mov rax, qword ptr [rbp]
14123e4e4 mov rdx, qword ptr [rbp + 0x10]
14123e4e8 sub rax, rdx
14123e4eb cmp rax, rbx
14123e4ee jb 0x14123e604
14123e4f4 unpcklps xmm8, xmm9
14123e4f8 divps xmm8, xmm6
14123e4fc mov r12, qword ptr [rbp - 0x20]
14123e500 mov rax, qword ptr [rbp - 0x18]
14123e504 lea r15, [r12 + rax*4]
14123e508 mov rbx, r12
14123e50b movaps xmm0, xmm13
14123e50f jmp 0x14123e535
14123e511 nop word ptr cs:[rax + rax]
14123e520 mov rax, qword ptr [rbp + 8]
14123e524 movss dword ptr [rax + r13*4], xmm9
14123e52a inc r13
14123e52d mov qword ptr [rbp + 0x10], r13
14123e531 movaps xmm0, xmm9
14123e535 ucomiss xmm0, xmm10
14123e539 jbe 0x14123e590
14123e53b nop dword ptr [rax + rax]
14123e540 cmp rbx, r15
14123e543 je 0x14123e5e0
14123e549 movss xmm1, dword ptr [rbx]
14123e54d add rbx, 4
14123e551 movaps xmm2, xmm14
14123e555 subss xmm2, xmm1
14123e559 unpcklps xmm1, xmm2
14123e55c mulps xmm1, xmm8
14123e560 movshdup xmm9, xmm1
14123e565 addss xmm9, xmm1
14123e56a movaps xmm1, xmm0
14123e56d subss xmm1, xmm9
14123e572 andps xmm1, xmm15
14123e576 ucomiss xmm7, xmm1
14123e579 ja 0x14123e540
14123e57b mov r13, qword ptr [rbp + 0x10]
14123e57f cmp r13, qword ptr [rbp]
14123e583 jne 0x14123e520
14123e585 jmp 0x14123e5c4
14123e587 nop word ptr [rax + rax]
14123e590 cmp rbx, r15
14123e593 je 0x14123e5e0
14123e595 movss xmm0, dword ptr [rbx]
14123e599 add rbx, 4
14123e59d movaps xmm1, xmm14
14123e5a1 subss xmm1, xmm0
14123e5a5 unpcklps xmm0, xmm1
14123e5a8 mulps xmm0, xmm8
14123e5ac movshdup xmm9, xmm0
14123e5b1 addss xmm9, xmm0
14123e5b6 mov r13, qword ptr [rbp + 0x10]
14123e5ba cmp r13, qword ptr [rbp]
14123e5be jne 0x14123e520
14123e5c4 mov rcx, rbp
14123e5c7 lea rdx, [rip + 0x19e0c32]
14123e5ce call 0x140650050
14123e5d3 jmp 0x14123e520
14123e5d8 nop dword ptr [rax + rax]
14123e5e0 mov rdx, qword ptr [rbp - 0x28]
14123e5e4 test rdx, rdx
14123e5e7 je 0x14123e410
14123e5ed shl rdx, 2
14123e5f1 mov r8d, 4
14123e5f7 mov rcx, r12
14123e5fa call 0x140613c20
14123e5ff jmp 0x14123e410
14123e604 mov qword ptr [rsp + 0x20], 4
14123e60d mov r9d, 4
14123e613 mov rcx, rbp
14123e616 mov r8, rbx
14123e619 call 0x1428a7de0
14123e61e jmp 0x14123e4f4
14123e623 mov rax, qword ptr [rbp + 0x10]
14123e627 mov rsi, qword ptr [rbp - 8]
14123e62b mov qword ptr [rsi + 0x10], rax
14123e62f movups xmm0, xmmword ptr [rbp]
14123e633 movups xmmword ptr [rsi], xmm0
14123e636 cmp byte ptr [rbp + 0x20], 0
14123e63a je 0x14123e67e
14123e63c call 0x140c18130
14123e641 mov rcx, rax
14123e644 mov rax, qword ptr [rax]
14123e647 cmp rax, 1
14123e64b jne 0x14123e6ee
14123e651 add rcx, 8
14123e655 cmp qword ptr [rcx], 0
14123e659 jne 0x14123e712
14123e65f mov qword ptr [rcx], 0xffffffffffffffff
14123e666 mov qword ptr [rbp + 0x20], rcx
14123e66a add rcx, 8
14123e66e mov rdx, qword ptr [rbp + 0x18]
14123e672 call 0x140c170f0
14123e677 mov rax, qword ptr [rbp + 0x20]
14123e67b inc qword ptr [rax]
14123e67e mov rax, rsi
14123e681 movaps xmm6, xmmword ptr [rbp + 0x30]
14123e685 movaps xmm7, xmmword ptr [rbp + 0x40]
14123e689 movaps xmm8, xmmword ptr [rbp + 0x50]
14123e68e movaps xmm9, xmmword ptr [rbp + 0x60]
14123e693 movaps xmm10, xmmword ptr [rbp + 0x70]
14123e698 movaps xmm11, xmmword ptr [rbp + 0x80]
14123e6a0 movaps xmm12, xmmword ptr [rbp + 0x90]
14123e6a8 movaps xmm13, xmmword ptr [rbp + 0xa0]
14123e6b0 movaps xmm14, xmmword ptr [rbp + 0xb0]
14123e6b8 movaps xmm15, xmmword ptr [rbp + 0xc0]
14123e6c0 add rsp, 0x158
14123e6c7 pop rbx
14123e6c8 pop rdi
14123e6c9 pop rsi
14123e6ca pop r12
14123e6cc pop r13
14123e6ce pop r14
14123e6d0 pop r15
14123e6d2 pop rbp
14123e6d3 ret 
14123e6d4 test rax, rax
14123e6d7 jne 0x14123e706
14123e6d9 xor edx, edx
14123e6db call 0x1428f15c0
14123e6e0 mov rcx, rax
14123e6e3 test rax, rax
14123e6e6 jne 0x14123e34a
14123e6ec jmp 0x14123e706
14123e6ee test rax, rax
14123e6f1 jne 0x14123e706
14123e6f3 xor edx, edx
14123e6f5 call 0x1428f15c0
14123e6fa mov rcx, rax
14123e6fd test rax, rax
14123e700 jne 0x14123e655
14123e706 lea rcx, [rip + 0x19dad9b]
14123e70d call 0x1428d80d0
14123e712 lea rcx, [rip + 0x19dadff]
14123e719 call 0x1428d9250
14123e71e int3 
14123e71f nop 
14123e720 mov qword ptr [rsp + 0x10], rdx
14123e725 push rbp
14123e726 push r15
14123e728 push r14
14123e72a push r13
14123e72c push r12
14123e72e push rsi
14123e72f push rdi
14123e730 push rbx
14123e731 sub rsp, 0xd8
14123e738 lea rbp, [rdx + 0x80]
14123e73f movaps xmmword ptr [rsp + 0x30], xmm15
14123e745 movaps xmmword ptr [rsp + 0x40], xmm14
14123e74b movaps xmmword ptr [rsp + 0x50], xmm13
14123e751 movaps xmmword ptr [rsp + 0x60], xmm12
14123e757 movaps xmmword ptr [rsp + 0x70], xmm11
14123e75d movaps xmmword ptr [rsp + 0x80], xmm10
14123e766 movaps xmmword ptr [rsp + 0x90], xmm9
14123e76f movaps xmmword ptr [rsp + 0xa0], xmm8
14123e778 movaps xmmword ptr [rsp + 0xb0], xmm7
14123e780 movaps xmmword ptr [rsp + 0xc0], xmm6
14123e788 mov rax, qword ptr [rbp + 0x18]
14123e78c inc qword ptr [rax]
14123e78f movaps xmm6, xmmword ptr [rsp + 0xc0]
14123e797 movaps xmm7, xmmword ptr [rsp + 0xb0]
14123e79f movaps xmm8, xmmword ptr [rsp + 0xa0]
14123e7a8 movaps xmm9, xmmword ptr [rsp + 0x90]
14123e7b1 movaps xmm10, xmmword ptr [rsp + 0x80]
14123e7ba movaps xmm11, xmmword ptr [rsp + 0x70]
14123e7c0 movaps xmm12, xmmword ptr [rsp + 0x60]
14123e7c6 movaps xmm13, xmmword ptr [rsp + 0x50]
14123e7cc movaps xmm14, xmmword ptr [rsp + 0x40]
14123e7d2 movaps xmm15, xmmword ptr [rsp + 0x30]
14123e7d8 add rsp, 0xd8
14123e7df pop rbx
14123e7e0 pop rdi
14123e7e1 pop rsi
14123e7e2 pop r12
14123e7e4 pop r13
14123e7e6 pop r14
14123e7e8 pop r15
14123e7ea pop rbp
14123e7eb ret 
14123e7ec nop dword ptr [rax]
14123e7f0 mov qword ptr [rsp + 0x10], rdx
14123e7f5 push rbp
14123e7f6 push r15
14123e7f8 push r14
14123e7fa push r13
14123e7fc push r12
14123e7fe push rsi
14123e7ff push rdi
14123e800 push rbx
14123e801 sub rsp, 0xd8
14123e808 lea rbp, [rdx + 0x80]
14123e80f movaps xmmword ptr [rsp + 0x30], xmm15
14123e815 movaps xmmword ptr [rsp + 0x40], xmm14
14123e81b movaps xmmword ptr [rsp + 0x50], xmm13
14123e821 movaps xmmword ptr [rsp + 0x60], xmm12
14123e827 movaps xmmword ptr [rsp + 0x70], xmm11
14123e82d movaps xmmword ptr [rsp + 0x80], xmm10
14123e836 movaps xmmword ptr [rsp + 0x90], xmm9
14123e83f movaps xmmword ptr [rsp + 0xa0], xmm8
14123e848 movaps xmmword ptr [rsp + 0xb0], xmm7
14123e850 movaps xmmword ptr [rsp + 0xc0], xmm6
14123e858 mov rdx, qword ptr [rbp]
14123e85c test rdx, rdx
14123e85f je 0x14123e874
14123e861 mov rcx, qword ptr [rbp + 8]
14123e865 shl rdx, 2
14123e869 mov r8d, 4
14123e86f call 0x140613c20
14123e874 cmp byte ptr [rbp + 0x20], 0
14123e878 je 0x14123e89a
14123e87a lea rax, [rbp - 0x40]
14123e87e mov qword ptr [rbp - 0x30], rax
14123e882 lea rax, [rbp - 0x30]
14123e886 mov qword ptr [rbp - 0x38], rax
14123e88a lea rcx, [rip + 0x19e0847]
14123e891 lea rdx, [rbp - 0x38]
14123e895 call 0x1411fb590
14123e89a movaps xmm6, xmmword ptr [rsp + 0xc0]
14123e8a2 movaps xmm7, xmmword ptr [rsp + 0xb0]
14123e8aa movaps xmm8, xmmword ptr [rsp + 0xa0]
14123e8b3 movaps xmm9, xmmword ptr [rsp + 0x90]
14123e8bc movaps xmm10, xmmword ptr [rsp + 0x80]
14123e8c5 movaps xmm11, xmmword ptr [rsp + 0x70]
14123e8cb movaps xmm12, xmmword ptr [rsp + 0x60]
14123e8d1 movaps xmm13, xmmword ptr [rsp + 0x50]
14123e8d7 movaps xmm14, xmmword ptr [rsp + 0x40]
14123e8dd movaps xmm15, xmmword ptr [rsp + 0x30]
14123e8e3 add rsp, 0xd8
14123e8ea pop rbx
14123e8eb pop rdi
14123e8ec pop rsi
14123e8ed pop r12
14123e8ef pop r13
14123e8f1 pop r14
14123e8f3 pop r15
14123e8f5 pop rbp
14123e8f6 ret 
14123e8f7 nop word ptr [rax + rax]
14123e900 mov qword ptr [rsp + 0x10], rdx
14123e905 push rbp
14123e906 push r15
14123e908 push r14
14123e90a push r13
14123e90c push r12
14123e90e push rsi
14123e90f push rdi
14123e910 push rbx
14123e911 sub rsp, 0xd8
14123e918 lea rbp, [rdx + 0x80]
14123e91f movaps xmmword ptr [rsp + 0x30], xmm15
14123e925 movaps xmmword ptr [rsp + 0x40], xmm14
14123e92b movaps xmmword ptr [rsp + 0x50], xmm13
14123e931 movaps xmmword ptr [rsp + 0x60], xmm12
14123e937 movaps xmmword ptr [rsp + 0x70], xmm11
14123e93d movaps xmmword ptr [rsp + 0x80], xmm10
14123e946 movaps xmmword ptr [rsp + 0x90], xmm9
14123e94f movaps xmmword ptr [rsp + 0xa0], xmm8
14123e958 movaps xmmword ptr [rsp + 0xb0], xmm7
14123e960 movaps xmmword ptr [rsp + 0xc0], xmm6
14123e968 mov rax, qword ptr [rbp + 0x20]
14123e96c inc qword ptr [rax]
14123e96f movaps xmm6, xmmword ptr [rsp + 0xc0]
14123e977 movaps xmm7, xmmword ptr [rsp + 0xb0]
14123e97f movaps xmm8, xmmword ptr [rsp + 0xa0]
14123e988 movaps xmm9, xmmword ptr [rsp + 0x90]
14123e991 movaps xmm10, xmmword ptr [rsp + 0x80]
14123e99a movaps xmm11, xmmword ptr [rsp + 0x70]
14123e9a0 movaps xmm12, xmmword ptr [rsp + 0x60]
14123e9a6 movaps xmm13, xmmword ptr [rsp + 0x50]
14123e9ac movaps xmm14, xmmword ptr [rsp + 0x40]
14123e9b2 movaps xmm15, xmmword ptr [rsp + 0x30]
14123e9b8 add rsp, 0xd8
14123e9bf pop rbx
14123e9c0 pop rdi
14123e9c1 pop rsi
14123e9c2 pop r12
14123e9c4 pop r13
14123e9c6 pop r14
14123e9c8 pop r15
14123e9ca pop rbp
14123e9cb ret 
14123e9cc nop dword ptr [rax]
14123e9d0 mov qword ptr [rsp + 0x10], rdx
14123e9d5 push rbp
14123e9d6 push r15
14123e9d8 push r14
14123e9da push r13
14123e9dc push r12
14123e9de push rsi
14123e9df push rdi
14123e9e0 push rbx
14123e9e1 sub rsp, 0xd8
14123e9e8 lea rbp, [rdx + 0x80]
14123e9ef movaps xmmword ptr [rsp + 0x30], xmm15
14123e9f5 movaps xmmword ptr [rsp + 0x40], xmm14
14123e9fb movaps xmmword ptr [rsp + 0x50], xmm13
14123ea01 movaps xmmword ptr [rsp + 0x60], xmm12
14123ea07 movaps xmmword ptr [rsp + 0x70], xmm11
14123ea0d movaps xmmword ptr [rsp + 0x80], xmm10
14123ea16 movaps xmmword ptr [rsp + 0x90], xmm9
14123ea1f movaps xmmword ptr [rsp + 0xa0], xmm8
14123ea28 movaps xmmword ptr [rsp + 0xb0], xmm7
14123ea30 movaps xmmword ptr [rsp + 0xc0], xmm6
14123ea38 mov rdx, qword ptr [rbp - 0x28]
14123ea3c test rdx, rdx
14123ea3f je 0x14123ea54
14123ea41 mov rcx, qword ptr [rbp - 0x20]
14123ea45 shl rdx, 2
14123ea49 mov r8d, 4
14123ea4f call 0x140613c20
14123ea54 movaps xmm6, xmmword ptr [rsp + 0xc0]
14123ea5c movaps xmm7, xmmword ptr [rsp + 0xb0]
14123ea64 movaps xmm8, xmmword ptr [rsp + 0xa0]
14123ea6d movaps xmm9, xmmword ptr [rsp + 0x90]
14123ea76 movaps xmm10, xmmword ptr [rsp + 0x80]
14123ea7f movaps xmm11, xmmword ptr [rsp + 0x70]
14123ea85 movaps xmm12, xmmword ptr [rsp + 0x60]
14123ea8b movaps xmm13, xmmword ptr [rsp + 0x50]
14123ea91 movaps xmm14, xmmword ptr [rsp + 0x40]
14123ea97 movaps xmm15, xmmword ptr [rsp + 0x30]
14123ea9d add rsp, 0xd8
14123eaa4 pop rbx
14123eaa5 pop rdi
14123eaa6 pop rsi
14123eaa7 pop r12
14123eaa9 pop r13
14123eaab pop r14
14123eaad pop r15
14123eaaf pop rbp
14123eab0 ret 
14123eab1 int3 
14123eab2 int3 
14123eab3 int3 
14123eab4 int3 
14123eab5 int3 
14123eab6 int3 
14123eab7 int3 
14123eab8 int3 
14123eab9 int3 
14123eaba int3 
14123eabb int3 
14123eabc int3 
14123eabd int3 
14123eabe int3 
14123eabf int3 
14123eac0 push rsi
14123eac1 push rdi
14123eac2 sub rsp, 0xb8
14123eac9 mov r8, rcx
14123eacc mov qword ptr [rsp + 0x20], 2
14123ead5 lea rdx, [rip + 0x19e07ec]
14123eadc lea r9, [rip + 0x19e0895]
14123eae3 lea rcx, [rsp + 0x50]
14123eae8 call 0x140d89940
14123eaed test byte ptr [rsp + 0x50], 1
14123eaf2 je 0x14123eb3c
14123eaf4 lea rdx, [rsp + 0x58]
14123eaf9 cmp qword ptr [rsp + 0x88], -1
14123eb02 mov r8, qword ptr [rsp + 0x98]
14123eb0a mov r9, qword ptr [rsp + 0xa0]
14123eb12 mov rax, qword ptr [rsp + 0xa8]
14123eb1a mov rcx, qword ptr [rsp + 0xb0]
14123eb22 je 0x14123ec53
14123eb28 mov qword ptr [rsp + 0x28], rcx
14123eb2d mov qword ptr [rsp + 0x20], rax
14123eb32 mov byte ptr [rsp + 0x30], 0
14123eb37 jmp 0x14123ec62
14123eb3c cmp byte ptr [rsp + 0x6a], 0
14123eb41 je 0x14123eb4b
14123eb43 xor r9d, r9d
14123eb46 jmp 0x14123ec82
14123eb4b mov r9, qword ptr [rsp + 0x60]
14123eb50 movzx r8d, byte ptr [rsp + 0x69]
14123eb56 test r9, r9
14123eb59 je 0x14123ec6e
14123eb5f mov rcx, qword ptr [rsp + 0x98]
14123eb67 mov rdx, qword ptr [rsp + 0xa0]
14123eb6f mov eax, r8d
14123eb72 jmp 0x14123eb99
14123eb74 nop word ptr cs:[rax + rax]
14123eb80 mov r10d, eax
14123eb83 xor r10b, 1
14123eb87 cmp al, 1
14123eb89 setne r8b
14123eb8d mov eax, r10d
14123eb90 add r9, r11
14123eb93 je 0x14123ec6e
14123eb99 cmp r9, rdx
14123eb9c jae 0x14123ebb0
14123eb9e cmp byte ptr [rcx + r9], 0xbf
14123eba3 jg 0x14123ebb6
14123eba5 jmp 0x14123ec9b
14123ebaa nop word ptr [rax + rax]
14123ebb0 jne 0x14123ec9b
14123ebb6 movsx r10d, byte ptr [rcx + r9 - 1]
14123ebbc test r10d, r10d
14123ebbf jns 0x14123ec0e
14123ebc1 movzx r11d, byte ptr [rcx + r9 - 2]
14123ebc7 cmp r11b, 0xc0
14123ebcb jge 0x14123ebec
14123ebcd movzx esi, byte ptr [rcx + r9 - 3]
14123ebd3 cmp sil, 0xc0
14123ebd7 jge 0x14123ebf2
14123ebd9 movzx edi, byte ptr [rcx + r9 - 4]
14123ebdf and edi, 7
14123ebe2 shl edi, 6
14123ebe5 and esi, 0x3f
14123ebe8 or esi, edi
14123ebea jmp 0x14123ebf5
14123ebec and r11d, 0x1f
14123ebf0 jmp 0x14123ebff
14123ebf2 and esi, 0xf
14123ebf5 shl esi, 6
14123ebf8 and r11d, 0x3f
14123ebfc or r11d, esi
14123ebff shl r11d, 6
14123ec03 and r10b, 0x3f
14123ec07 movzx r10d, r10b
14123ec0b or r10d, r11d
14123ec0e test r8b, 1
14123ec12 jne 0x14123ec77
14123ec14 mov r11, 0xffffffffffffffff
14123ec1b cmp r10d, 0x80
14123ec22 jb 0x14123eb80
14123ec28 mov r11, 0xfffffffffffffffe
14123ec2f cmp r10d, 0x800
14123ec36 jb 0x14123eb80
14123ec3c xor r11d, r11d
14123ec3f cmp r10d, 0x10000
14123ec46 setb r11b
14123ec4a or r11, 0xfffffffffffffffc
14123ec4e jmp 0x14123eb80
14123ec53 mov qword ptr [rsp + 0x28], rcx
14123ec58 mov qword ptr [rsp + 0x20], rax
14123ec5d mov byte ptr [rsp + 0x30], 1
14123ec62 lea rcx, [rsp + 0x38]
14123ec67 call 0x14123ecb0
14123ec6c jmp 0x14123ec87
14123ec6e xor r9d, r9d
14123ec71 test r8b, 1
14123ec75 je 0x14123ec82
14123ec77 mov qword ptr [rsp + 0x40], r9
14123ec7c mov r9d, 1
14123ec82 mov qword ptr [rsp + 0x38], r9
14123ec87 mov rax, qword ptr [rsp + 0x38]
14123ec8c mov rdx, qword ptr [rsp + 0x40]
14123ec91 add rsp, 0xb8
14123ec98 pop rdi
14123ec99 pop rsi
14123ec9a ret 
14123ec9b lea rax, [rip + 0x19e070e]
14123eca2 mov qword ptr [rsp + 0x20], rax
14123eca7 xor r8d, r8d
14123ecaa call 0x1428d9940
14123ecaf int3 
14123ecb0 push r15
14123ecb2 push r14
14123ecb4 push r13
14123ecb6 push r12
14123ecb8 push rsi
14123ecb9 push rdi
14123ecba push rbp
14123ecbb push rbx
14123ecbc sub rsp, 0x48
14123ecc0 mov rax, rdx
14123ecc3 mov qword ptr [rsp + 0x30], rcx
14123ecc8 mov rdx, qword ptr [rsp + 0xb8]
14123ecd0 mov rbp, qword ptr [rax + 0x28]
14123ecd4 mov r10, rbp
14123ecd7 sub r10, rdx
14123ecda cmp r10, r9
14123ecdd jae 0x14123ee52
14123ece3 movzx r14d, byte ptr [rsp + 0xc0]
14123ecec mov rdi, qword ptr [rsp + 0xb0]
14123ecf4 mov rbx, qword ptr [rax + 0x18]
14123ecf8 mov rcx, qword ptr [rax + 8]
14123ecfc mov r11, qword ptr [rax + 0x10]
14123ed00 mov qword ptr [rsp + 0x38], r11
14123ed05 mov r13, qword ptr [rax + 0x38]
14123ed09 lea r11, [rdi - 1]
14123ed0d mov qword ptr [rsp + 0x40], r11
14123ed12 mov qword ptr [rsp + 0x28], rbx
14123ed17 jmp 0x14123ed36
14123ed19 nop dword ptr [rax]
14123ed20 mov qword ptr [rax + 0x38], rsi
14123ed24 mov r13, rsi
14123ed27 mov rbp, r10
14123ed2a sub r10, rdx
14123ed2d cmp r10, r9
14123ed30 jae 0x14123ee52
14123ed36 movzx r11d, byte ptr [r8 + r10]
14123ed3b bt rbx, r11
14123ed3f jae 0x14123edc0
14123ed41 cmp r13, rcx
14123ed44 mov r11, rcx
14123ed47 cmovb r11, r13
14123ed4b mov ebx, r14d
14123ed4e test r14b, r14b
14123ed51 cmovne r11, rcx
14123ed55 lea rsi, [r11 - 1]
14123ed59 cmp rsi, rdx
14123ed5c jae 0x14123ede0
14123ed62 mov r12, r11
14123ed65 neg r12
14123ed68 mov rsi, qword ptr [rsp + 0x40]
14123ed6d lea r15, [rsi + r11]
14123ed71 lea rsi, [r11 + r10]
14123ed75 dec rsi
14123ed78 nop dword ptr [rax + rax]
14123ed80 test r12, r12
14123ed83 je 0x14123ede9
14123ed85 cmp rsi, r9
14123ed88 jae 0x14123eec4
14123ed8e movzx r11d, byte ptr [r15]
14123ed92 inc r12
14123ed95 dec r15
14123ed98 lea r14, [rsi - 1]
14123ed9c cmp r11b, byte ptr [r8 + rsi]
14123eda0 mov rsi, r14
14123eda3 je 0x14123ed80
14123eda5 sub rbp, rcx
14123eda8 sub rbp, r12
14123edab mov qword ptr [rax + 0x28], rbp
14123edaf mov rsi, rdx
14123edb2 jmp 0x14123ee3a
14123edb7 nop word ptr [rax + rax]
14123edc0 mov qword ptr [rax + 0x28], r10
14123edc4 mov rsi, rdx
14123edc7 test r14b, r14b
14123edca je 0x14123ed20
14123edd0 jmp 0x14123ed27
14123edd5 nop word ptr cs:[rax + rax]
14123ede0 test r11, r11
14123ede3 jne 0x14123eed6
14123ede9 test bl, bl
14123edeb mov rsi, r13
14123edee cmovne rsi, rdx
14123edf2 lea r15, [r8 + r10]
14123edf6 mov r12, rcx
14123edf9 nop dword ptr [rax]
14123ee00 cmp r12, rsi
14123ee03 jae 0x14123ee63
14123ee05 cmp r12, rdx
14123ee08 jae 0x14123ee95
14123ee0e lea r11, [r10 + r12]
14123ee12 cmp r11, r9
14123ee15 jae 0x14123eea8
14123ee1b movzx r11d, byte ptr [rdi + r12]
14123ee20 lea r14, [r12 + 1]
14123ee25 cmp r11b, byte ptr [r15 + r12]
14123ee29 mov r12, r14
14123ee2c je 0x14123ee00
14123ee2e mov rsi, qword ptr [rsp + 0x38]
14123ee33 sub rbp, rsi
14123ee36 mov qword ptr [rax + 0x28], rbp
14123ee3a mov r10, rbp
14123ee3d mov r14d, ebx
14123ee40 test bl, bl
14123ee42 mov rbx, qword ptr [rsp + 0x28]
14123ee47 je 0x14123ed20
14123ee4d jmp 0x14123ed27
14123ee52 mov qword ptr [rax + 0x28], 0
14123ee5a xor eax, eax
14123ee5c mov rcx, qword ptr [rsp + 0x30]
14123ee61 jmp 0x14123ee81
14123ee63 mov qword ptr [rax + 0x28], r10
14123ee67 test bl, bl
14123ee69 jne 0x14123ee6f
14123ee6b mov qword ptr [rax + 0x38], rdx
14123ee6f mov rcx, qword ptr [rsp + 0x30]
14123ee74 mov qword ptr [rcx + 8], r10
14123ee78 mov qword ptr [rcx + 0x10], rbp
14123ee7c mov eax, 1
14123ee81 mov qword ptr [rcx], rax
14123ee84 add rsp, 0x48
14123ee88 pop rbx
14123ee89 pop rbp
14123ee8a pop rdi
14123ee8b pop rsi
14123ee8c pop r12
14123ee8e pop r13
14123ee90 pop r14
14123ee92 pop r15
14123ee94 ret 
14123ee95 cmp rcx, rdx
14123ee98 cmovbe rcx, rdx
14123ee9c lea r8, [rip + 0x19e03c5]
14123eea3 call 0x1428d9518
14123eea8 add r10, rcx
14123eeab cmp r9, r10
14123eeae cmova r10, r9
14123eeb2 lea r8, [rip + 0x19e03c7]
14123eeb9 mov rcx, r10
14123eebc mov rdx, r9
14123eebf call 0x1428d9518
14123eec4 lea r8, [rip + 0x19e03e5]
14123eecb mov rcx, rsi
14123eece mov rdx, r9
14123eed1 call 0x1428d9518
14123eed6 lea r8, [rip + 0x19e03bb]
14123eedd mov rcx, rsi
14123eee0 call 0x1428d9518
14123eee5 int3 
14123eee6 int3 
14123eee7 int3 
14123eee8 int3 
14123eee9 int3 
14123eeea int3 
14123eeeb int3 
14123eeec int3 
14123eeed int3 
14123eeee int3 
14123eeef int3 