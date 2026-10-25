14145c350: 55                           push rbp
14145c351: 4157                         push r15
14145c353: 4156                         push r14
14145c355: 4155                         push r13
14145c357: 4154                         push r12
14145c359: 56                           push rsi
14145c35a: 57                           push rdi
14145c35b: 53                           push rbx
14145c35c: 4881ecd8000000               sub rsp, 0xd8
14145c363: 488dac2480000000             lea rbp, [rsp + 0x80]
14145c36b: 440f297540                   movaps xmmword ptr [rbp + 0x40], xmm14
14145c370: 440f296d30                   movaps xmmword ptr [rbp + 0x30], xmm13
14145c375: 440f296520                   movaps xmmword ptr [rbp + 0x20], xmm12
14145c37a: 440f295d10                   movaps xmmword ptr [rbp + 0x10], xmm11
14145c37f: 440f295500                   movaps xmmword ptr [rbp], xmm10
14145c384: 440f294df0                   movaps xmmword ptr [rbp - 0x10], xmm9
14145c389: 440f2945e0                   movaps xmmword ptr [rbp - 0x20], xmm8
14145c38e: 0f297dd0                     movaps xmmword ptr [rbp - 0x30], xmm7
14145c392: 0f2975c0                     movaps xmmword ptr [rbp - 0x40], xmm6
14145c396: 48c745b8feffffff             mov qword ptr [rbp - 0x48], 0xfffffffffffffffe
14145c39e: 4889d7                       mov rdi, rdx
14145c3a1: 4889ce                       mov rsi, rcx
14145c3a4: 488b85c8000000               mov rax, qword ptr [rbp + 0xc8]
14145c3ab: 488b95c0000000               mov rdx, qword ptr [rbp + 0xc0]
14145c3b2: f3410f1038                   movss xmm7, dword ptr [r8]
14145c3b7: f3450f104808                 movss xmm9, dword ptr [r8 + 8]
14145c3bd: 448b27                       mov r12d, dword ptr [rdi]
14145c3c0: f3440f7e4704                 movq xmm8, qword ptr [rdi + 4]
14145c3c6: 4d8b7918                     mov r15, qword ptr [r9 + 0x18]
14145c3ca: 48c745a000000000             mov qword ptr [rbp - 0x60], 0
14145c3d2: 48c745a804000000             mov qword ptr [rbp - 0x58], 4
14145c3da: 48c745b000000000             mov qword ptr [rbp - 0x50], 0
14145c3e2: f3410f104804                 movss xmm1, dword ptr [r8 + 4]
14145c3e8: f30f108288000000             movss xmm0, dword ptr [rdx + 0x88]
14145c3f0: f30f59c7                     mulss xmm0, xmm7
14145c3f4: f30f598a98000000             mulss xmm1, dword ptr [rdx + 0x98]
14145c3fc: f30f58c8                     addss xmm1, xmm0
14145c400: f30f1082a8000000             movss xmm0, dword ptr [rdx + 0xa8]
14145c408: f3410f59c1                   mulss xmm0, xmm9
14145c40d: f30f58c1                     addss xmm0, xmm1
14145c411: f30f5882b8000000             addss xmm0, dword ptr [rdx + 0xb8]
14145c419: 0f5705d0944c01               xorps xmm0, xmmword ptr [rip + 0x14c94d0]
14145c420: 8b4004                       mov eax, dword ptr [rax + 4]
14145c423: f3480f2ad0                   cvtsi2ss xmm2, rax
14145c428: e82305e2ff                   call 0x14127c950
14145c42d: 0f28f0                       movaps xmm6, xmm0
14145c430: 66410f6ec4                   movd xmm0, r12d
14145c435: 660f70c050                   pshufd xmm0, xmm0, 0x50
14145c43a: 660f72f01f                   pslld xmm0, 0x1f
14145c43f: 660f72e01f                   psrad xmm0, 0x1f
14145c444: 410f14f9                     unpcklps xmm7, xmm9
14145c448: 66450f6fc8                   movdqa xmm9, xmm8
14145c44d: 66440fdbc8                   pand xmm9, xmm0
14145c452: 660fdfc7                     pandn xmm0, xmm7
14145c456: 4d85ff                       test r15, r15
14145c459: 7807                         js 0x14145c462
14145c45b: f34d0f2ad7                   cvtsi2ss xmm10, r15
14145c460: eb19                         jmp 0x14145c47b
14145c462: 4c89f8                       mov rax, r15
14145c465: 48d1e8                       shr rax, 1
14145c468: 4489f9                       mov ecx, r15d
14145c46b: 83e101                       and ecx, 1
14145c46e: 4809c1                       or rcx, rax
14145c471: f34c0f2ad1                   cvtsi2ss xmm10, rcx
14145c476: f3450f58d2                   addss xmm10, xmm10
14145c47b: 66440febc8                   por xmm9, xmm0
14145c480: 4531ed                       xor r13d, r13d
14145c483: f3440f101d78944c01           movss xmm11, dword ptr [rip + 0x14c9478]
14145c48c: f3440f1025df415001           movss xmm12, dword ptr [rip + 0x15041df]
14145c495: f3440f102dda0b5701           movss xmm13, dword ptr [rip + 0x1570bda]
14145c49e: 488d15cb5f8001               lea rdx, [rip + 0x1805fcb]
14145c4a5: 4c8d75a0                     lea r14, [rbp - 0x60]
14145c4a9: 31db                         xor ebx, ebx
14145c4ab: eb23                         jmp 0x14145c4d0
14145c4ad: 488b45a8                     mov rax, qword ptr [rbp - 0x58]
14145c4b1: 460f1334e8                   movlps qword ptr [rax + r13*8], xmm14
14145c4b6: 49ffc5                       inc r13
14145c4b9: 4c896db0                     mov qword ptr [rbp - 0x50], r13
14145c4bd: c70701000000                 mov dword ptr [rdi], 1
14145c4c3: 440f137704                   movlps qword ptr [rdi + 4], xmm14
14145c4c8: 41b401                       mov r12b, 1
14145c4cb: 450f28c6                     movaps xmm8, xmm14
14145c4cf: 90                           nop 
14145c4d0: 4c39fb                       cmp rbx, r15
14145c4d3: 0f83a8000000                 jae 0x14145c581
14145c4d9: 48ffc3                       inc rbx
14145c4dc: 7812                         js 0x14145c4f0
14145c4de: 0f57c0                       xorps xmm0, xmm0
14145c4e1: f3480f2ac3                   cvtsi2ss xmm0, rbx
14145c4e6: eb22                         jmp 0x14145c50a
14145c4e8: 0f1f840000000000             nop dword ptr [rax + rax]
14145c4f0: 4889d8                       mov rax, rbx
14145c4f3: 48d1e8                       shr rax, 1
14145c4f6: 89d9                         mov ecx, ebx
14145c4f8: 83e101                       and ecx, 1
14145c4fb: 4809c1                       or rcx, rax
14145c4fe: 0f57c0                       xorps xmm0, xmm0
14145c501: f3480f2ac1                   cvtsi2ss xmm0, rcx
14145c506: f30f58c0                     addss xmm0, xmm0
14145c50a: f3410f5ec2                   divss xmm0, xmm10
14145c50f: 410f28cb                     movaps xmm1, xmm11
14145c513: f30f5cc8                     subss xmm1, xmm0
14145c517: f30f12c9                     movsldup xmm1, xmm1
14145c51b: 410f59c9                     mulps xmm1, xmm9
14145c51f: f3440f12f0                   movsldup xmm14, xmm0
14145c524: 440f59f7                     mulps xmm14, xmm7
14145c528: 440f58f1                     addps xmm14, xmm1
14145c52c: 41f6c401                     test r12b, 1
14145c530: 7432                         je 0x14145c564
14145c532: 410f28c6                     movaps xmm0, xmm14
14145c536: 410f5cc0                     subps xmm0, xmm8
14145c53a: 0f59c0                       mulps xmm0, xmm0
14145c53d: f30f16c8                     movshdup xmm1, xmm0
14145c541: f30f58c8                     addss xmm1, xmm0
14145c545: 0f57c0                       xorps xmm0, xmm0
14145c548: f30f51c1                     sqrtss xmm0, xmm1
14145c54c: 410f2ec4                     ucomiss xmm0, xmm12
14145c550: 0f867affffff                 jbe 0x14145c4d0
14145c556: f30f59c6                     mulss xmm0, xmm6
14145c55a: 410f2ec5                     ucomiss xmm0, xmm13
14145c55e: 0f866cffffff                 jbe 0x14145c4d0
14145c564: 4c3b6da0                     cmp r13, qword ptr [rbp - 0x60]
14145c568: 0f853fffffff                 jne 0x14145c4ad
14145c56e: 4c89f1                       mov rcx, r14
14145c571: 4989d4                       mov r12, rdx
14145c574: e8175a72ff                   call 0x140b81f90
14145c579: 4c89e2                       mov rdx, r12
14145c57c: e92cffffff                   jmp 0x14145c4ad
14145c581: 488b45b0                     mov rax, qword ptr [rbp - 0x50]
14145c585: 48894610                     mov qword ptr [rsi + 0x10], rax
14145c589: 0f1045a0                     movups xmm0, xmmword ptr [rbp - 0x60]
14145c58d: 0f1106                       movups xmmword ptr [rsi], xmm0
14145c590: 4889f0                       mov rax, rsi
14145c593: 0f2875c0                     movaps xmm6, xmmword ptr [rbp - 0x40]
14145c597: 0f287dd0                     movaps xmm7, xmmword ptr [rbp - 0x30]
14145c59b: 440f2845e0                   movaps xmm8, xmmword ptr [rbp - 0x20]
14145c5a0: 440f284df0                   movaps xmm9, xmmword ptr [rbp - 0x10]
14145c5a5: 440f285500                   movaps xmm10, xmmword ptr [rbp]
14145c5aa: 440f285d10                   movaps xmm11, xmmword ptr [rbp + 0x10]
14145c5af: 440f286520                   movaps xmm12, xmmword ptr [rbp + 0x20]
14145c5b4: 440f286d30                   movaps xmm13, xmmword ptr [rbp + 0x30]
14145c5b9: 440f287540                   movaps xmm14, xmmword ptr [rbp + 0x40]
14145c5be: 4881c4d8000000               add rsp, 0xd8
14145c5c5: 5b                           pop rbx
14145c5c6: 5f                           pop rdi
14145c5c7: 5e                           pop rsi
14145c5c8: 415c                         pop r12
14145c5ca: 415d                         pop r13
14145c5cc: 415e                         pop r14
14145c5ce: 415f                         pop r15
14145c5d0: 5d                           pop rbp
14145c5d1: c3                           ret 
14145c5d2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14145c5e0: 4889542410                   mov qword ptr [rsp + 0x10], rdx
14145c5e5: 55                           push rbp
14145c5e6: 4157                         push r15
14145c5e8: 4156                         push r14
14145c5ea: 4155                         push r13
14145c5ec: 4154                         push r12
14145c5ee: 56                           push rsi
14145c5ef: 57                           push rdi
14145c5f0: 53                           push rbx
14145c5f1: 4881ecb8000000               sub rsp, 0xb8
14145c5f8: 488daa80000000               lea rbp, [rdx + 0x80]
14145c5ff: 440f29742420                 movaps xmmword ptr [rsp + 0x20], xmm14
14145c605: 440f296c2430                 movaps xmmword ptr [rsp + 0x30], xmm13
14145c60b: 440f29642440                 movaps xmmword ptr [rsp + 0x40], xmm12
14145c611: 440f295c2450                 movaps xmmword ptr [rsp + 0x50], xmm11
14145c617: 440f29542460                 movaps xmmword ptr [rsp + 0x60], xmm10
14145c61d: 440f294c2470                 movaps xmmword ptr [rsp + 0x70], xmm9
14145c623: 66440f7f842480000000         movdqa xmmword ptr [rsp + 0x80], xmm8
14145c62d: 0f29bc2490000000             movaps xmmword ptr [rsp + 0x90], xmm7
14145c635: 0f29b424a0000000             movaps xmmword ptr [rsp + 0xa0], xmm6
14145c63d: 488b55a0                     mov rdx, qword ptr [rbp - 0x60]
14145c641: 4885d2                       test rdx, rdx
14145c644: 7413                         je 0x14145c659
14145c646: 488b4da8                     mov rcx, qword ptr [rbp - 0x58]
14145c64a: 48c1e203                     shl rdx, 3
14145c64e: 41b804000000                 mov r8d, 4
14145c654: e8c7751bff                   call 0x140613c20
14145c659: 0f28b424a0000000             movaps xmm6, xmmword ptr [rsp + 0xa0]
14145c661: 0f28bc2490000000             movaps xmm7, xmmword ptr [rsp + 0x90]
14145c669: 440f28842480000000           movaps xmm8, xmmword ptr [rsp + 0x80]
14145c672: 440f284c2470                 movaps xmm9, xmmword ptr [rsp + 0x70]
14145c678: 440f28542460                 movaps xmm10, xmmword ptr [rsp + 0x60]
14145c67e: 440f285c2450                 movaps xmm11, xmmword ptr [rsp + 0x50]
14145c684: 440f28642440                 movaps xmm12, xmmword ptr [rsp + 0x40]
14145c68a: 440f286c2430                 movaps xmm13, xmmword ptr [rsp + 0x30]
14145c690: 440f28742420                 movaps xmm14, xmmword ptr [rsp + 0x20]
14145c696: 4881c4b8000000               add rsp, 0xb8
14145c69d: 5b                           pop rbx
14145c69e: 5f                           pop rdi
14145c69f: 5e                           pop rsi
14145c6a0: 415c                         pop r12
14145c6a2: 415d                         pop r13
14145c6a4: 415e                         pop r14
14145c6a6: 415f                         pop r15
14145c6a8: 5d                           pop rbp
14145c6a9: c3                           ret 
