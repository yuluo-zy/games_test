00000001425df320: 4157                           push r15
00000001425df322: 4156                           push r14
00000001425df324: 4155                           push r13
00000001425df326: 4154                           push r12
00000001425df328: 56                             push rsi
00000001425df329: 57                             push rdi
00000001425df32a: 55                             push rbp
00000001425df32b: 53                             push rbx
00000001425df32c: 4881ecd8000000                 sub rsp, 0xd8
00000001425df333: 440f299424c0000000             movaps xmmword ptr [rsp + 0xc0], xmm10
00000001425df33c: 440f298c24b0000000             movaps xmmword ptr [rsp + 0xb0], xmm9
00000001425df345: 440f298424a0000000             movaps xmmword ptr [rsp + 0xa0], xmm8
00000001425df34e: 0f29bc2490000000               movaps xmmword ptr [rsp + 0x90], xmm7
00000001425df356: 0f29b42480000000               movaps xmmword ptr [rsp + 0x80], xmm6
00000001425df35e: 4c89c6                         mov rsi, r8
00000001425df361: 4889d7                         mov rdi, rdx
00000001425df364: 4889cb                         mov rbx, rcx
00000001425df367: 488d4c2448                     lea rcx, [rsp + 0x48]
00000001425df36c: 4889da                         mov rdx, rbx
00000001425df36f: e88c724cfe                     call 0x140aa6600
00000001425df374: f2440f10542448                 movsd xmm10, qword ptr [rsp + 0x48]
00000001425df37b: f2440f104c2450                 movsd xmm9, qword ptr [rsp + 0x50]
00000001425df382: 488d4c2470                     lea rcx, [rsp + 0x70]
00000001425df387: 48897c2468                     mov qword ptr [rsp + 0x68], rdi
00000001425df38c: 4889fa                         mov rdx, rdi
00000001425df38f: e8ac8d6bfe                     call 0x140c98140
00000001425df394: 4889d9                         mov rcx, rbx
00000001425df397: e844724cfe                     call 0x140aa65e0
00000001425df39c: 0f28f0                         movaps xmm6, xmm0
00000001425df39f: f2440f10442470                 movsd xmm8, qword ptr [rsp + 0x70]
00000001425df3a6: f20f107c2478                   movsd xmm7, qword ptr [rsp + 0x78]
00000001425df3ac: 410f28c2                       movaps xmm0, xmm10
00000001425df3b0: 410f5fc0                       maxps xmm0, xmm8
00000001425df3b4: 450fc2c003                     cmpunordps xmm8, xmm8
00000001425df3b9: 450f54d0                       andps xmm10, xmm8
00000001425df3bd: 440f55c0                       andnps xmm8, xmm0
00000001425df3c1: 450f56c2                       orps xmm8, xmm10
00000001425df3c5: 488d4c2448                     lea rcx, [rsp + 0x48]
00000001425df3ca: 4889da                         mov rdx, rbx
00000001425df3cd: e82e724cfe                     call 0x140aa6600
00000001425df3d2: f20f10442448                   movsd xmm0, qword ptr [rsp + 0x48]
00000001425df3d8: 410fc2c002                     cmpleps xmm0, xmm8
00000001425df3dd: 0f14c0                         unpcklps xmm0, xmm0
00000001425df3e0: 660f50c0                       movmskpd eax, xmm0
00000001425df3e4: 83f803                         cmp eax, 3
00000001425df3e7: 0f850d030000                   jne 0x1425df6fa
00000001425df3ed: 410f28c1                       movaps xmm0, xmm9
00000001425df3f1: 0f5dc7                         minps xmm0, xmm7
00000001425df3f4: 0fc2ff03                       cmpunordps xmm7, xmm7
00000001425df3f8: 440f54cf                       andps xmm9, xmm7
00000001425df3fc: 0f55f8                         andnps xmm7, xmm0
00000001425df3ff: 410f56f9                       orps xmm7, xmm9
00000001425df403: f20f10442450                   movsd xmm0, qword ptr [rsp + 0x50]
00000001425df409: 0f28cf                         movaps xmm1, xmm7
00000001425df40c: 0fc2c802                       cmpleps xmm1, xmm0
00000001425df410: 0f14c9                         unpcklps xmm1, xmm1
00000001425df413: 660f50c1                       movmskpd eax, xmm1
00000001425df417: 83f803                         cmp eax, 3
00000001425df41a: 0f85f2020000                   jne 0x1425df712
00000001425df420: 4889d9                         mov rcx, rbx
00000001425df423: e8b8714cfe                     call 0x140aa65e0
00000001425df428: 0f2e0541643400                 ucomiss xmm0, dword ptr [rip + 0x346441]
00000001425df42f: 0f86f5020000                   jbe 0x1425df72a
00000001425df435: f3410f16d0                     movshdup xmm2, xmm8
00000001425df43a: 4889d9                         mov rcx, rbx
00000001425df43d: 410f28c8                       movaps xmm1, xmm8
00000001425df441: e8da704cfe                     call 0x140aa6520
00000001425df446: 4189c5                         mov r13d, eax
00000001425df449: 4531e4                         xor r12d, r12d
00000001425df44c: 85c0                           test eax, eax
00000001425df44e: 450f4eec                       cmovle r13d, r12d
00000001425df452: 85d2                           test edx, edx
00000001425df454: 440f4fe2                       cmovg r12d, edx
00000001425df458: 0f57c0                         xorps xmm0, xmm0
00000001425df45b: f30f16d7                       movshdup xmm2, xmm7
00000001425df45f: f30f58f8                       addss xmm7, xmm0
00000001425df463: f30f58d0                       addss xmm2, xmm0
00000001425df467: 4889d9                         mov rcx, rbx
00000001425df46a: 0f28cf                         movaps xmm1, xmm7
00000001425df46d: e8ae704cfe                     call 0x140aa6520
00000001425df472: 4189c6                         mov r14d, eax
00000001425df475: 488b6b48                       mov rbp, qword ptr [rbx + 0x48]
00000001425df479: 8d45ff                         lea eax, [rbp - 1]
00000001425df47c: 48c1ed20                       shr rbp, 0x20
00000001425df480: ffcd                           dec ebp
00000001425df482: 4439f0                         cmp eax, r14d
00000001425df485: 440f4cf0                       cmovl r14d, eax
00000001425df489: 39d5                           cmp ebp, edx
00000001425df48b: 0f4dea                         cmovge ebp, edx
00000001425df48e: 4139ec                         cmp r12d, ebp
00000001425df491: 0f8f24020000                   jg 0x1425df6bb
00000001425df497: 4539f5                         cmp r13d, r14d
00000001425df49a: 7e21                           jle 0x1425df4bd
00000001425df49c: 0f1f4000                       nop dword ptr [rax]
00000001425df4a0: 31c0                           xor eax, eax
00000001425df4a2: 4139ec                         cmp r12d, ebp
00000001425df4a5: 0f9cc1                         setl cl
00000001425df4a8: 0f8d0d020000                   jge 0x1425df6bb
00000001425df4ae: 88c8                           mov al, cl
00000001425df4b0: 4101c4                         add r12d, eax
00000001425df4b3: 4139ec                         cmp r12d, ebp
00000001425df4b6: 7ee8                           jle 0x1425df4a0
00000001425df4b8: e9fe010000                     jmp 0x1425df6bb
00000001425df4bd: f30f5935ab633400               mulss xmm6, dword ptr [rip + 0x3463ab]
00000001425df4c5: f30f593527f23400               mulss xmm6, dword ptr [rip + 0x34f227]
00000001425df4cd: 488b4308                       mov rax, qword ptr [rbx + 8]
00000001425df4d1: 4889442440                     mov qword ptr [rsp + 0x40], rax
00000001425df4d6: 488b4310                       mov rax, qword ptr [rbx + 0x10]
00000001425df4da: 4889442438                     mov qword ptr [rsp + 0x38], rax
00000001425df4df: 488b06                         mov rax, qword ptr [rsi]
00000001425df4e2: 4889442460                     mov qword ptr [rsp + 0x60], rax
00000001425df4e7: 4c8b7e08                       mov r15, qword ptr [rsi + 8]
00000001425df4eb: f30f103dadc54900               movss xmm7, dword ptr [rip + 0x49c5ad]
00000001425df4f3: f3440f100508643400             movss xmm8, dword ptr [rip + 0x346408]
00000001425df4fc: f3440f100d6b643400             movss xmm9, dword ptr [rip + 0x34646b]
00000001425df505: 4889742458                     mov qword ptr [rsp + 0x58], rsi
00000001425df50a: 44896c2434                     mov dword ptr [rsp + 0x34], r13d
00000001425df50f: 4489e0                         mov eax, r12d
00000001425df512: 4531e4                         xor r12d, r12d
00000001425df515: 39e8                           cmp eax, ebp
00000001425df517: 410f9cc4                       setl r12b
00000001425df51b: 89442430                       mov dword ptr [rsp + 0x30], eax
00000001425df51f: 4101c4                         add r12d, eax
00000001425df522: 66666666662e0f1f840000000000   nop word ptr cs:[rax + rax]
00000001425df530: 31ff                           xor edi, edi
00000001425df532: 4539f5                         cmp r13d, r14d
00000001425df535: 0f9c44242f                     setl byte ptr [rsp + 0x2f]
00000001425df53a: 4889d9                         mov rcx, rbx
00000001425df53d: 4489ea                         mov edx, r13d
00000001425df540: 448b442430                     mov r8d, dword ptr [rsp + 0x30]
00000001425df545: e886704cfe                     call 0x140aa65d0
00000001425df54a: 4889c6                         mov rsi, rax
00000001425df54d: 4889d9                         mov rcx, rbx
00000001425df550: 4889c2                         mov rdx, rax
00000001425df553: e818674cfe                     call 0x140aa5c70
00000001425df558: 488b442468                     mov rax, qword ptr [rsp + 0x68]
00000001425df55d: f20f1010                       movsd xmm2, qword ptr [rax]
00000001425df561: 0f14c1                         unpcklps xmm0, xmm1
00000001425df564: 0f5cc2                         subps xmm0, xmm2
00000001425df567: 0f59c0                         mulps xmm0, xmm0
00000001425df56a: f30f16c8                       movshdup xmm1, xmm0
00000001425df56e: f30f58c8                       addss xmm1, xmm0
00000001425df572: 0f57c0                         xorps xmm0, xmm0
00000001425df575: f30f51c1                       sqrtss xmm0, xmm1
00000001425df579: f30f104808                     movss xmm1, dword ptr [rax + 8]
00000001425df57e: f30f58ce                       addss xmm1, xmm6
00000001425df582: 0f2ec8                         ucomiss xmm1, xmm0
00000001425df585: 0f86f5000000                   jbe 0x1425df680
00000001425df58b: 4839742438                     cmp qword ptr [rsp + 0x38], rsi
00000001425df590: 0f86ac010000                   jbe 0x1425df742
00000001425df596: 488d0476                       lea rax, [rsi + rsi*2]
00000001425df59a: 48c1e004                       shl rax, 4
00000001425df59e: 488b4c2440                     mov rcx, qword ptr [rsp + 0x40]
00000001425df5a3: 488b540110                     mov rdx, qword ptr [rcx + rax + 0x10]
00000001425df5a8: 4885d2                         test rdx, rdx
00000001425df5ab: 0f84cf000000                   je 0x1425df680
00000001425df5b1: 4803442440                     add rax, qword ptr [rsp + 0x40]
00000001425df5b6: 488b4008                       mov rax, qword ptr [rax + 8]
00000001425df5ba: 488b4c2460                     mov rcx, qword ptr [rsp + 0x60]
00000001425df5bf: 488b09                         mov rcx, qword ptr [rcx]
00000001425df5c2: 486bd238                       imul rdx, rdx, 0x38
00000001425df5c6: 4531c0                         xor r8d, r8d
00000001425df5c9: eb12                           jmp 0x1425df5dd
00000001425df5cb: 0f1f440000                     nop dword ptr [rax + rax]
00000001425df5d0: 4983c038                       add r8, 0x38
00000001425df5d4: 4c39c2                         cmp rdx, r8
00000001425df5d7: 0f84a3000000                   je 0x1425df680
00000001425df5dd: 4a394c0028                     cmp qword ptr [rax + r8 + 0x28], rcx
00000001425df5e2: 75ec                           jne 0x1425df5d0
00000001425df5e4: f2420f101400                   movsd xmm2, qword ptr [rax + r8]
00000001425df5ea: f2420f104c0008                 movsd xmm1, qword ptr [rax + r8 + 8]
00000001425df5f1: f3410f1007                     movss xmm0, dword ptr [r15]
00000001425df5f6: f3410f105f08                   movss xmm3, dword ptr [r15 + 8]
00000001425df5fc: 0f14c3                         unpcklps xmm0, xmm3
00000001425df5ff: 0f28d8                         movaps xmm3, xmm0
00000001425df602: 0f5cda                         subps xmm3, xmm2
00000001425df605: 0f28e1                         movaps xmm4, xmm1
00000001425df608: 0f5ce2                         subps xmm4, xmm2
00000001425df60b: 0f59dc                         mulps xmm3, xmm4
00000001425df60e: f30f16eb                       movshdup xmm5, xmm3
00000001425df612: f30f58eb                       addss xmm5, xmm3
00000001425df616: 0f59e4                         mulps xmm4, xmm4
00000001425df619: f30f16dc                       movshdup xmm3, xmm4
00000001425df61d: f30f58dc                       addss xmm3, xmm4
00000001425df621: f30f5fdf                       maxss xmm3, xmm7
00000001425df625: f30f5eeb                       divss xmm5, xmm3
00000001425df629: 0f57db                         xorps xmm3, xmm3
00000001425df62c: f30f5fdd                       maxss xmm3, xmm5
00000001425df630: 410f28e0                       movaps xmm4, xmm8
00000001425df634: f30f5de3                       minss xmm4, xmm3
00000001425df638: 410f28d8                       movaps xmm3, xmm8
00000001425df63c: f30f5cdc                       subss xmm3, xmm4
00000001425df640: f30f12db                       movsldup xmm3, xmm3
00000001425df644: 0f59da                         mulps xmm3, xmm2
00000001425df647: f30f12d4                       movsldup xmm2, xmm4
00000001425df64b: 0f59d1                         mulps xmm2, xmm1
00000001425df64e: 0f58d3                         addps xmm2, xmm3
00000001425df651: 0f5cc2                         subps xmm0, xmm2
00000001425df654: 0f59c0                         mulps xmm0, xmm0
00000001425df657: f30f16c8                       movshdup xmm1, xmm0
00000001425df65b: f30f58c8                       addss xmm1, xmm0
00000001425df65f: 0f57c0                         xorps xmm0, xmm0
00000001425df662: f30f51c1                       sqrtss xmm0, xmm1
00000001425df666: 440f2ec8                       ucomiss xmm9, xmm0
00000001425df66a: 0f8660ffffff                   jbe 0x1425df5d0
00000001425df670: eb3d                           jmp 0x1425df6af
00000001425df672: 66666666662e0f1f840000000000   nop word ptr cs:[rax + rax]
00000001425df680: 4539f5                         cmp r13d, r14d
00000001425df683: 7d14                           jge 0x1425df699
00000001425df685: 0fb644242f                     movzx eax, byte ptr [rsp + 0x2f]
00000001425df68a: 4088c7                         mov dil, al
00000001425df68d: 4101fd                         add r13d, edi
00000001425df690: 4539f5                         cmp r13d, r14d
00000001425df693: 0f8e97feffff                   jle 0x1425df530
00000001425df699: 396c2430                       cmp dword ptr [rsp + 0x30], ebp
00000001425df69d: 448b6c2434                     mov r13d, dword ptr [rsp + 0x34]
00000001425df6a2: 7d17                           jge 0x1425df6bb
00000001425df6a4: 4139ec                         cmp r12d, ebp
00000001425df6a7: 0f8e62feffff                   jle 0x1425df50f
00000001425df6ad: eb0c                           jmp 0x1425df6bb
00000001425df6af: 488b442458                     mov rax, qword ptr [rsp + 0x58]
00000001425df6b4: 488b4010                       mov rax, qword ptr [rax + 0x10]
00000001425df6b8: c60000                         mov byte ptr [rax], 0
00000001425df6bb: 0f28b42480000000               movaps xmm6, xmmword ptr [rsp + 0x80]
00000001425df6c3: 0f28bc2490000000               movaps xmm7, xmmword ptr [rsp + 0x90]
00000001425df6cb: 440f288424a0000000             movaps xmm8, xmmword ptr [rsp + 0xa0]
00000001425df6d4: 440f288c24b0000000             movaps xmm9, xmmword ptr [rsp + 0xb0]
00000001425df6dd: 440f289424c0000000             movaps xmm10, xmmword ptr [rsp + 0xc0]
00000001425df6e6: 4881c4d8000000                 add rsp, 0xd8
00000001425df6ed: 5b                             pop rbx
00000001425df6ee: 5d                             pop rbp
00000001425df6ef: 5f                             pop rdi
00000001425df6f0: 5e                             pop rsi
00000001425df6f1: 415c                           pop r12
00000001425df6f3: 415d                           pop r13
00000001425df6f5: 415e                           pop r14
00000001425df6f7: 415f                           pop r15
00000001425df6f9: c3                             ret 
00000001425df6fa: 488d0de757b500                 lea rcx, [rip + 0xb557e7]
00000001425df701: 4c8d059858b500                 lea r8, [rip + 0xb55898]
00000001425df708: ba44000000                     mov edx, 0x44
00000001425df70d: e81e9d2f00                     call 0x1428d9430
00000001425df712: 488d0d9f58b500                 lea rcx, [rip + 0xb5589f]
00000001425df719: 4c8d05e058b500                 lea r8, [rip + 0xb558e0]
00000001425df720: ba44000000                     mov edx, 0x44
00000001425df725: e8069d2f00                     call 0x1428d9430
00000001425df72a: 488d0de758b500                 lea rcx, [rip + 0xb558e7]
00000001425df731: 4c8d051859b500                 lea r8, [rip + 0xb55918]
00000001425df738: ba31000000                     mov edx, 0x31
00000001425df73d: e8ee9c2f00                     call 0x1428d9430
00000001425df742: 4c8d051f59b500                 lea r8, [rip + 0xb5591f]
00000001425df749: 4889f1                         mov rcx, rsi
00000001425df74c: 488b542438                     mov rdx, qword ptr [rsp + 0x38]
00000001425df751: e8c29d2f00                     call 0x1428d9518
00000001425df756: cc                             int3 
