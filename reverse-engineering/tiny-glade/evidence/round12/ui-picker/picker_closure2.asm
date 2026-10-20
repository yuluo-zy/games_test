0000000140ac3f50: 55                       push       rbp
0000000140ac3f51: 4157                     push       r15
0000000140ac3f53: 4156                     push       r14
0000000140ac3f55: 4155                     push       r13
0000000140ac3f57: 4154                     push       r12
0000000140ac3f59: 56                       push       rsi
0000000140ac3f5a: 57                       push       rdi
0000000140ac3f5b: 53                       push       rbx
0000000140ac3f5c: 4881ecb8010000           sub        rsp, 0x1b8
0000000140ac3f63: 488dac2480000000         lea        rbp, [rsp + 0x80]
0000000140ac3f6b: 440f29ad20010000         movaps     xmmword ptr [rbp + 0x120], xmm13
0000000140ac3f73: 440f29a510010000         movaps     xmmword ptr [rbp + 0x110], xmm12
0000000140ac3f7b: 440f299d00010000         movaps     xmmword ptr [rbp + 0x100], xmm11
0000000140ac3f83: 440f2995f0000000         movaps     xmmword ptr [rbp + 0xf0], xmm10
0000000140ac3f8b: 440f298de0000000         movaps     xmmword ptr [rbp + 0xe0], xmm9
0000000140ac3f93: 440f2985d0000000         movaps     xmmword ptr [rbp + 0xd0], xmm8
0000000140ac3f9b: 0f29bdc0000000           movaps     xmmword ptr [rbp + 0xc0], xmm7
0000000140ac3fa2: 0f29b5b0000000           movaps     xmmword ptr [rbp + 0xb0], xmm6
0000000140ac3fa9: 48c785a8000000feffffff   mov        qword ptr [rbp + 0xa8], 0xfffffffffffffffe
0000000140ac3fb4: 0fb64220                 movzx      eax, byte ptr [rdx + 0x20]
0000000140ac3fb8: 85c0                     test       eax, eax
0000000140ac3fba: 0f852b0b0000             jne        0x140ac4aeb
0000000140ac3fc0: 48894dc8                 mov        qword ptr [rbp - 0x38], rcx
0000000140ac3fc4: 48895510                 mov        qword ptr [rbp + 0x10], rdx
0000000140ac3fc8: 488b4210                 mov        rax, qword ptr [rdx + 0x10]
0000000140ac3fcc: 31db                     xor        ebx, ebx
0000000140ac3fce: b938000000               mov        ecx, 0x38
0000000140ac3fd3: 4989c7                   mov        r15, rax
0000000140ac3fd6: 48f7e1                   mul        rcx
0000000140ac3fd9: 4889c6                   mov        rsi, rax
0000000140ac3fdc: 0f802b0b0000             jo         0x140ac4b0d
0000000140ac3fe2: 48b8f8ffffffffffff7f     movabs     rax, 0x7ffffffffffffff8
0000000140ac3fec: 4839c6                   cmp        rsi, rax
0000000140ac3fef: 0f87180b0000             ja         0x140ac4b0d
0000000140ac3ff5: 488b4510                 mov        rax, qword ptr [rbp + 0x10]
0000000140ac3ff9: 488b38                   mov        rdi, qword ptr [rax]
0000000140ac3ffc: 4c8b7008                 mov        r14, qword ptr [rax + 8]
0000000140ac4000: 4885f6                   test       rsi, rsi
0000000140ac4003: 7427                     je         0x140ac402c
0000000140ac4005: 0fb605fd0bb702           movzx      eax, byte ptr [rip + 0x2b70bfd]
0000000140ac400c: bb08000000               mov        ebx, 8
0000000140ac4011: ba08000000               mov        edx, 8
0000000140ac4016: 4889f1                   mov        rcx, rsi
0000000140ac4019: e8f2fbb4ff               call       0x140613c10
0000000140ac401e: 4885c0                   test       rax, rax
0000000140ac4021: 0f84e60a0000             je         0x140ac4b0d
0000000140ac4027: 4c89f9                   mov        rcx, r15
0000000140ac402a: eb07                     jmp        0x140ac4033
0000000140ac402c: b808000000               mov        eax, 8
0000000140ac4031: 31c9                     xor        ecx, ecx
0000000140ac4033: 48898d80000000           mov        qword ptr [rbp + 0x80], rcx
0000000140ac403a: 48898588000000           mov        qword ptr [rbp + 0x88], rax
0000000140ac4041: 48c7859000000000000000   mov        qword ptr [rbp + 0x90], 0
0000000140ac404c: 48c745e800000000         mov        qword ptr [rbp - 0x18], 0
0000000140ac4054: 48c745f008000000         mov        qword ptr [rbp - 0x10], 8
0000000140ac405c: 48c745f800000000         mov        qword ptr [rbp - 8], 0
0000000140ac4064: 49c1e704                 shl        r15, 4
0000000140ac4068: 4d01f7                   add        r15, r14
0000000140ac406b: 4c897dc0                 mov        qword ptr [rbp - 0x40], r15
0000000140ac406f: 488b1f                   mov        rbx, qword ptr [rdi]
0000000140ac4072: 488b4708                 mov        rax, qword ptr [rdi + 8]
0000000140ac4076: 48894508                 mov        qword ptr [rbp + 8], rax
0000000140ac407a: 488b4718                 mov        rax, qword ptr [rdi + 0x18]
0000000140ac407e: 48894500                 mov        qword ptr [rbp], rax
0000000140ac4082: 488b4710                 mov        rax, qword ptr [rdi + 0x10]
0000000140ac4086: 488945d0                 mov        qword ptr [rbp - 0x30], rax
0000000140ac408a: f30f10357218e601         movss      xmm6, dword ptr [rip + 0x1e61872]
0000000140ac4092: eb6b                     jmp        0x140ac40ff
0000000140ac4094: 488b8588000000           mov        rax, qword ptr [rbp + 0x88]
0000000140ac409b: 486bce38                 imul       rcx, rsi, 0x38
0000000140ac409f: c7040800000000           mov        dword ptr [rax + rcx], 0
0000000140ac40a6: 488b55dc                 mov        rdx, qword ptr [rbp - 0x24]
0000000140ac40aa: 4889540804               mov        qword ptr [rax + rcx + 4], rdx
0000000140ac40af: 8b55e4                   mov        edx, dword ptr [rbp - 0x1c]
0000000140ac40b2: 8954080c                 mov        dword ptr [rax + rcx + 0xc], edx
0000000140ac40b6: 488b5560                 mov        rdx, qword ptr [rbp + 0x60]
0000000140ac40ba: 4889540810               mov        qword ptr [rax + rcx + 0x10], rdx
0000000140ac40bf: 48c744081800000000       mov        qword ptr [rax + rcx + 0x18], 0
0000000140ac40c8: f30f117c0820             movss      dword ptr [rax + rcx + 0x20], xmm7
0000000140ac40ce: 488b5568                 mov        rdx, qword ptr [rbp + 0x68]
0000000140ac40d2: 4889540824               mov        qword ptr [rax + rcx + 0x24], rdx
0000000140ac40d7: 8b5570                   mov        edx, dword ptr [rbp + 0x70]
0000000140ac40da: 8954082c                 mov        dword ptr [rax + rcx + 0x2c], edx
0000000140ac40de: c644083000               mov        byte ptr [rax + rcx + 0x30], 0
0000000140ac40e3: 0fb695a7000000           movzx      edx, byte ptr [rbp + 0xa7]
0000000140ac40ea: 88540831                 mov        byte ptr [rax + rcx + 0x31], dl
0000000140ac40ee: 48ffc6                   inc        rsi
0000000140ac40f1: 4889b590000000           mov        qword ptr [rbp + 0x90], rsi
0000000140ac40f8: 4c8bb598000000           mov        r14, qword ptr [rbp + 0x98]
0000000140ac40ff: 4c3b75c0                 cmp        r14, qword ptr [rbp - 0x40]
0000000140ac4103: 0f841c090000             je         0x140ac4a25
0000000140ac4109: 498d5610                 lea        rdx, [r14 + 0x10]
0000000140ac410d: 498b06                   mov        rax, qword ptr [r14]
0000000140ac4110: 48894560                 mov        qword ptr [rbp + 0x60], rax
0000000140ac4114: 498b7e08                 mov        rdi, qword ptr [r14 + 8]
0000000140ac4118: 440fb6bfa8000000         movzx      r15d, byte ptr [rdi + 0xa8]
0000000140ac4120: 0fb687a9000000           movzx      eax, byte ptr [rdi + 0xa9]
0000000140ac4127: 8885a7000000             mov        byte ptr [rbp + 0xa7], al
0000000140ac412d: 0fb68788000000           movzx      eax, byte ptr [rdi + 0x88]
0000000140ac4134: 488d0dedb90102           lea        rcx, [rip + 0x201b9ed]
0000000140ac413b: 48630481                 movsxd     rax, dword ptr [rcx + rax*4]
0000000140ac413f: 4801c8                   add        rax, rcx
0000000140ac4142: 48899598000000           mov        qword ptr [rbp + 0x98], rdx
0000000140ac4149: 4989d6                   mov        r14, rdx
0000000140ac414c: ffe0                     jmp        rax
0000000140ac414e: 4588fe                   mov        r14b, r15b
0000000140ac4151: 8b8798000000             mov        eax, dword ptr [rdi + 0x98]
0000000140ac4157: 894538                   mov        dword ptr [rbp + 0x38], eax
0000000140ac415a: 488b8790000000           mov        rax, qword ptr [rdi + 0x90]
0000000140ac4161: 48894530                 mov        qword ptr [rbp + 0x30], rax
0000000140ac4165: f30f109f8c000000         movss      xmm3, dword ptr [rdi + 0x8c]
0000000140ac416d: 488d4d68                 lea        rcx, [rbp + 0x68]
0000000140ac4171: 4889da                   mov        rdx, rbx
0000000140ac4174: 4c8d4530                 lea        r8, [rbp + 0x30]
0000000140ac4178: e8b3707700               call       0x14123b230
0000000140ac417d: 837d6801                 cmp        dword ptr [rbp + 0x68], 1
0000000140ac4181: 0f85c7050000             jne        0x140ac474e
0000000140ac4187: f20f10456c               movsd      xmm0, qword ptr [rbp + 0x6c]
0000000140ac418c: f20f114530               movsd      qword ptr [rbp + 0x30], xmm0
0000000140ac4191: 488d4d30                 lea        rcx, [rbp + 0x30]
0000000140ac4195: e866707700               call       0x14123b200
0000000140ac419a: 440f28c0                 movaps     xmm8, xmm0
0000000140ac419e: 488b4d08                 mov        rcx, qword ptr [rbp + 8]
0000000140ac41a2: 488b01                   mov        rax, qword ptr [rcx]
0000000140ac41a5: 488b4908                 mov        rcx, qword ptr [rcx + 8]
0000000140ac41a9: f2440f1008               movsd      xmm9, qword ptr [rax]
0000000140ac41ae: f2440f1021               movsd      xmm12, qword ptr [rcx]
0000000140ac41b3: f3440f105108             movss      xmm10, dword ptr [rcx + 8]
0000000140ac41b9: f3440f59d0               mulss      xmm10, xmm0
0000000140ac41be: f3440f585008             addss      xmm10, dword ptr [rax + 8]
0000000140ac41c4: 488d4d30                 lea        rcx, [rbp + 0x30]
0000000140ac41c8: e833707700               call       0x14123b200
0000000140ac41cd: 0f28f8                   movaps     xmm7, xmm0
0000000140ac41d0: 450f28da                 movaps     xmm11, xmm10
0000000140ac41d4: f3440f5c9f98000000       subss      xmm11, dword ptr [rdi + 0x98]
0000000140ac41dd: f2440f10af90000000       movsd      xmm13, qword ptr [rdi + 0x90]
0000000140ac41e6: 488bb590000000           mov        rsi, qword ptr [rbp + 0x90]
0000000140ac41ed: 483bb580000000           cmp        rsi, qword ptr [rbp + 0x80]
0000000140ac41f4: 7513                     jne        0x140ac4209
0000000140ac41f6: 488d8d80000000           lea        rcx, [rbp + 0x80]
0000000140ac41fd: 488d154cb60102           lea        rdx, [rip + 0x201b64c]
0000000140ac4204: e8c7d7b6ff               call       0x1406319d0
0000000140ac4209: f3410f12c0               movsldup   xmm0, xmm8
0000000140ac420e: 410f59c4                 mulps      xmm0, xmm12
0000000140ac4212: 440f58c8                 addps      xmm9, xmm0
0000000140ac4216: 410f28c1                 movaps     xmm0, xmm9
0000000140ac421a: 410f5cc5                 subps      xmm0, xmm13
0000000140ac421e: 0f28c8                   movaps     xmm1, xmm0
0000000140ac4221: 0f59c8                   mulps      xmm1, xmm0
0000000140ac4224: f30f16d1                 movshdup   xmm2, xmm1
0000000140ac4228: f30f58d1                 addss      xmm2, xmm1
0000000140ac422c: 410f28cb                 movaps     xmm1, xmm11
0000000140ac4230: f3410f59cb               mulss      xmm1, xmm11
0000000140ac4235: f30f58ca                 addss      xmm1, xmm2
0000000140ac4239: f30f51c9                 sqrtss     xmm1, xmm1
0000000140ac423d: 0f28d6                   movaps     xmm2, xmm6
0000000140ac4240: f30f5ed1                 divss      xmm2, xmm1
0000000140ac4244: f30f12ca                 movsldup   xmm1, xmm2
0000000140ac4248: 0f59c8                   mulps      xmm1, xmm0
0000000140ac424b: f3410f59d3               mulss      xmm2, xmm11
0000000140ac4250: 488b8588000000           mov        rax, qword ptr [rbp + 0x88]
0000000140ac4257: 486bce38                 imul       rcx, rsi, 0x38
0000000140ac425b: c7040801000000           mov        dword ptr [rax + rcx], 1
0000000140ac4262: 0f134c0804               movlps     qword ptr [rax + rcx + 4], xmm1
0000000140ac4267: f30f1154080c             movss      dword ptr [rax + rcx + 0xc], xmm2
0000000140ac426d: 488b5560                 mov        rdx, qword ptr [rbp + 0x60]
0000000140ac4271: 4889540810               mov        qword ptr [rax + rcx + 0x10], rdx
0000000140ac4276: 48c744081800000000       mov        qword ptr [rax + rcx + 0x18], 0
0000000140ac427f: f30f117c0820             movss      dword ptr [rax + rcx + 0x20], xmm7
0000000140ac4285: 440f134c0824             movlps     qword ptr [rax + rcx + 0x24], xmm9
0000000140ac428b: f3440f1154082c           movss      dword ptr [rax + rcx + 0x2c], xmm10
0000000140ac4292: 4488740830               mov        byte ptr [rax + rcx + 0x30], r14b
0000000140ac4297: e947feffff               jmp        0x140ac40e3
0000000140ac429c: f30f108f8c000000         movss      xmm1, dword ptr [rdi + 0x8c]
0000000140ac42a4: f30f109790000000         movss      xmm2, dword ptr [rdi + 0x90]
0000000140ac42ac: 488d0515b60102           lea        rax, [rip + 0x201b615]
0000000140ac42b3: 4889442438               mov        qword ptr [rsp + 0x38], rax
0000000140ac42b8: c744243000000000         mov        dword ptr [rsp + 0x30], 0
0000000140ac42c0: c744242800000000         mov        dword ptr [rsp + 0x28], 0
0000000140ac42c8: c744242000000000         mov        dword ptr [rsp + 0x20], 0
0000000140ac42d0: 488d4d30                 lea        rcx, [rbp + 0x30]
0000000140ac42d4: 0f28de                   movaps     xmm3, xmm6
0000000140ac42d7: e8a4461d00               call       0x140c98980
0000000140ac42dc: 488b4500                 mov        rax, qword ptr [rbp]
0000000140ac42e0: 488b00                   mov        rax, qword ptr [rax]
0000000140ac42e3: 0f57c9                   xorps      xmm1, xmm1
0000000140ac42e6: f60001                   test       byte ptr [rax], 1
0000000140ac42e9: 0f57d2                   xorps      xmm2, xmm2
0000000140ac42ec: 740a                     je         0x140ac42f8
0000000140ac42ee: f30f104804               movss      xmm1, dword ptr [rax + 4]
0000000140ac42f3: f30f10500c               movss      xmm2, dword ptr [rax + 0xc]
0000000140ac42f8: 488d4d30                 lea        rcx, [rbp + 0x30]
0000000140ac42fc: e82f631d00               call       0x140c9a630
0000000140ac4301: 4c8bb598000000           mov        r14, qword ptr [rbp + 0x98]
0000000140ac4308: 84c0                     test       al, al
0000000140ac430a: 0f84effdffff             je         0x140ac40ff
0000000140ac4310: 488b4500                 mov        rax, qword ptr [rbp]
0000000140ac4314: 488b00                   mov        rax, qword ptr [rax]
0000000140ac4317: f60001                   test       byte ptr [rax], 1
0000000140ac431a: 0f84c4040000             je         0x140ac47e4
0000000140ac4320: 8b480c                   mov        ecx, dword ptr [rax + 0xc]
0000000140ac4323: 894d38                   mov        dword ptr [rbp + 0x38], ecx
0000000140ac4326: 488b4004                 mov        rax, qword ptr [rax + 4]
0000000140ac432a: 48894530                 mov        qword ptr [rbp + 0x30], rax
0000000140ac432e: e9c0040000               jmp        0x140ac47f3
0000000140ac4333: 44887d28                 mov        byte ptr [rbp + 0x28], r15b
0000000140ac4337: 488bb798000000           mov        rsi, qword ptr [rdi + 0x98]
0000000140ac433e: 488b87a0000000           mov        rax, qword ptr [rdi + 0xa0]
0000000140ac4345: 488d04c0                 lea        rax, [rax + rax*8]
0000000140ac4349: 4c8d3486                 lea        r14, [rsi + rax*4]
0000000140ac434d: 4531ed                   xor        r13d, r13d
0000000140ac4350: 4c39f6                   cmp        rsi, r14
0000000140ac4353: 7430                     je         0x140ac4385
0000000140ac4355: 66662e0f1f840000000000   nop        word ptr cs:[rax + rax]
0000000140ac4360: 488d4d30                 lea        rcx, [rbp + 0x30]
0000000140ac4364: 4889da                   mov        rdx, rbx
0000000140ac4367: 4989f0                   mov        r8, rsi
0000000140ac436a: e8316c7700               call       0x14123afa0
0000000140ac436f: 4883c624                 add        rsi, 0x24
0000000140ac4373: f6453001                 test       byte ptr [rbp + 0x30], 1
0000000140ac4377: 0f85e9030000             jne        0x140ac4766
0000000140ac437d: 49ffc5                   inc        r13
0000000140ac4380: 4c39f6                   cmp        rsi, r14
0000000140ac4383: 75db                     jne        0x140ac4360
0000000140ac4385: 4c8bb598000000           mov        r14, qword ptr [rbp + 0x98]
0000000140ac438c: e96efdffff               jmp        0x140ac40ff
0000000140ac4391: f30f108790000000         movss      xmm0, dword ptr [rdi + 0x90]
0000000140ac4399: f30f108f94000000         movss      xmm1, dword ptr [rdi + 0x94]
0000000140ac43a1: f30f109798000000         movss      xmm2, dword ptr [rdi + 0x98]
0000000140ac43a9: f30f109f9c000000         movss      xmm3, dword ptr [rdi + 0x9c]
0000000140ac43b1: 488b45d0                 mov        rax, qword ptr [rbp - 0x30]
0000000140ac43b5: 488b00                   mov        rax, qword ptr [rax]
0000000140ac43b8: f30f1020                 movss      xmm4, dword ptr [rax]
0000000140ac43bc: f30f106804               movss      xmm5, dword ptr [rax + 4]
0000000140ac43c1: f30f116c2428             movss      dword ptr [rsp + 0x28], xmm5
0000000140ac43c7: f30f11642420             movss      dword ptr [rsp + 0x20], xmm4
0000000140ac43cd: e8eebb7700               call       0x14123ffc0
0000000140ac43d2: 4c8bb598000000           mov        r14, qword ptr [rbp + 0x98]
0000000140ac43d9: 84c0                     test       al, al
0000000140ac43db: 0f841efdffff             je         0x140ac40ff
0000000140ac43e1: f30f10bf8c000000         movss      xmm7, dword ptr [rdi + 0x8c]
0000000140ac43e9: 488b45d0                 mov        rax, qword ptr [rbp - 0x30]
0000000140ac43ed: 488b00                   mov        rax, qword ptr [rax]
0000000140ac43f0: f2440f1000               movsd      xmm8, qword ptr [rax]
0000000140ac43f5: 488bb590000000           mov        rsi, qword ptr [rbp + 0x90]
0000000140ac43fc: 483bb580000000           cmp        rsi, qword ptr [rbp + 0x80]
0000000140ac4403: 7513                     jne        0x140ac4418
0000000140ac4405: 488d8d80000000           lea        rcx, [rbp + 0x80]
0000000140ac440c: 488d159db40102           lea        rdx, [rip + 0x201b49d]
0000000140ac4413: e8b8d5b6ff               call       0x1406319d0
0000000140ac4418: 488b8588000000           mov        rax, qword ptr [rbp + 0x88]
0000000140ac441f: 486bce38                 imul       rcx, rsi, 0x38
0000000140ac4423: c7040800000000           mov        dword ptr [rax + rcx], 0
0000000140ac442a: 488b55dc                 mov        rdx, qword ptr [rbp - 0x24]
0000000140ac442e: 4889540804               mov        qword ptr [rax + rcx + 4], rdx
0000000140ac4433: 8b55e4                   mov        edx, dword ptr [rbp - 0x1c]
0000000140ac4436: 8954080c                 mov        dword ptr [rax + rcx + 0xc], edx
0000000140ac443a: 488b5560                 mov        rdx, qword ptr [rbp + 0x60]
0000000140ac443e: 4889540810               mov        qword ptr [rax + rcx + 0x10], rdx
0000000140ac4443: 48c744081800000000       mov        qword ptr [rax + rcx + 0x18], 0
0000000140ac444c: f30f117c0820             movss      dword ptr [rax + rcx + 0x20], xmm7
0000000140ac4452: 440f13440824             movlps     qword ptr [rax + rcx + 0x24], xmm8
0000000140ac4458: f30f117c082c             movss      dword ptr [rax + rcx + 0x2c], xmm7
0000000140ac445e: e9b8050000               jmp        0x140ac4a1b
0000000140ac4463: 488bb798000000           mov        rsi, qword ptr [rdi + 0x98]
0000000140ac446a: 488b87a0000000           mov        rax, qword ptr [rdi + 0xa0]
0000000140ac4471: 488d0440                 lea        rax, [rax + rax*2]
0000000140ac4475: 4c8d2c86                 lea        r13, [rsi + rax*4]
0000000140ac4479: 4531f6                   xor        r14d, r14d
0000000140ac447c: eb05                     jmp        0x140ac4483
0000000140ac447e: 6690                     nop        
0000000140ac4480: 49ffc6                   inc        r14
0000000140ac4483: 4c39ee                   cmp        rsi, r13
0000000140ac4486: 0f8465020000             je         0x140ac46f1
0000000140ac448c: 8b4608                   mov        eax, dword ptr [rsi + 8]
0000000140ac448f: 894538                   mov        dword ptr [rbp + 0x38], eax
0000000140ac4492: 488b06                   mov        rax, qword ptr [rsi]
0000000140ac4495: 48894530                 mov        qword ptr [rbp + 0x30], rax
0000000140ac4499: f30f109f8c000000         movss      xmm3, dword ptr [rdi + 0x8c]
0000000140ac44a1: 488d4d68                 lea        rcx, [rbp + 0x68]
0000000140ac44a5: 4889da                   mov        rdx, rbx
0000000140ac44a8: 4c8d4530                 lea        r8, [rbp + 0x30]
0000000140ac44ac: e87f6d7700               call       0x14123b230
0000000140ac44b1: 4883c60c                 add        rsi, 0xc
0000000140ac44b5: 837d6801                 cmp        dword ptr [rbp + 0x68], 1
0000000140ac44b9: 75c5                     jne        0x140ac4480
0000000140ac44bb: f30f7e456c               movq       xmm0, qword ptr [rbp + 0x6c]
0000000140ac44c0: 660fd64518               movq       qword ptr [rbp + 0x18], xmm0
0000000140ac44c5: 488d4d18                 lea        rcx, [rbp + 0x18]
0000000140ac44c9: e8326d7700               call       0x14123b200
0000000140ac44ce: 660f6ff8                 movdqa     xmm7, xmm0
0000000140ac44d2: 660f7ec0                 movd       eax, xmm0
0000000140ac44d6: 25ffffff7f               and        eax, 0x7fffffff
0000000140ac44db: 3dffff7f7f               cmp        eax, 0x7f7fffff
0000000140ac44e0: 7f9e                     jg         0x140ac4480
0000000140ac44e2: 4c39ee                   cmp        rsi, r13
0000000140ac44e5: 0f8490040000             je         0x140ac497b
0000000140ac44eb: 4929f5                   sub        r13, rsi
0000000140ac44ee: 49c1ed02                 shr        r13, 2
0000000140ac44f2: 48b8abaaaaaaaaaaaaaa     movabs     rax, 0xaaaaaaaaaaaaaaab
0000000140ac44fc: 4c0fafe8                 imul       r13, rax
0000000140ac4500: 4c897528                 mov        qword ptr [rbp + 0x28], r14
0000000140ac4504: 4531e4                   xor        r12d, r12d
0000000140ac4507: eb13                     jmp        0x140ac451c
0000000140ac4509: 0f1f8000000000           nop        dword ptr [rax]
0000000140ac4510: 49ffc4                   inc        r12
0000000140ac4513: 4d39ec                   cmp        r12, r13
0000000140ac4516: 0f8463040000             je         0x140ac497f
0000000140ac451c: 4b8d0464                 lea        rax, [r12 + r12*2]
0000000140ac4520: 8b4c8608                 mov        ecx, dword ptr [rsi + rax*4 + 8]
0000000140ac4524: 894d38                   mov        dword ptr [rbp + 0x38], ecx
0000000140ac4527: 488b0486                 mov        rax, qword ptr [rsi + rax*4]
0000000140ac452b: 48894530                 mov        qword ptr [rbp + 0x30], rax
0000000140ac452f: f30f109f8c000000         movss      xmm3, dword ptr [rdi + 0x8c]
0000000140ac4537: 488d4d68                 lea        rcx, [rbp + 0x68]
0000000140ac453b: 4889da                   mov        rdx, rbx
0000000140ac453e: 4c8d4530                 lea        r8, [rbp + 0x30]
0000000140ac4542: e8e96c7700               call       0x14123b230
0000000140ac4547: 49ffc6                   inc        r14
0000000140ac454a: 837d6801                 cmp        dword ptr [rbp + 0x68], 1
0000000140ac454e: 75c0                     jne        0x140ac4510
0000000140ac4550: f30f7e456c               movq       xmm0, qword ptr [rbp + 0x6c]
0000000140ac4555: 660fd64518               movq       qword ptr [rbp + 0x18], xmm0
0000000140ac455a: 488d4d18                 lea        rcx, [rbp + 0x18]
0000000140ac455e: e89d6c7700               call       0x14123b200
0000000140ac4563: 660f7ec0                 movd       eax, xmm0
0000000140ac4567: 25ffffff7f               and        eax, 0x7fffffff
0000000140ac456c: 3dffff7f7f               cmp        eax, 0x7f7fffff
0000000140ac4571: 7f9d                     jg         0x140ac4510
0000000140ac4573: 0f2eff                   ucomiss    xmm7, xmm7
0000000140ac4576: 0f8a61050000             jp         0x140ac4add
0000000140ac457c: 0f2ef8                   ucomiss    xmm7, xmm0
0000000140ac457f: f30f5dc7                 minss      xmm0, xmm7
0000000140ac4583: 488b4528                 mov        rax, qword ptr [rbp + 0x28]
0000000140ac4587: 490f47c6                 cmova      rax, r14
0000000140ac458b: 48894528                 mov        qword ptr [rbp + 0x28], rax
0000000140ac458f: 0f28f8                   movaps     xmm7, xmm0
0000000140ac4592: e979ffffff               jmp        0x140ac4510
0000000140ac4597: 488b4500                 mov        rax, qword ptr [rbp]
0000000140ac459b: 488b00                   mov        rax, qword ptr [rax]
0000000140ac459e: f60001                   test       byte ptr [rax], 1
0000000140ac45a1: 0f8456010000             je         0x140ac46fd
0000000140ac45a7: 8b480c                   mov        ecx, dword ptr [rax + 0xc]
0000000140ac45aa: 894d38                   mov        dword ptr [rbp + 0x38], ecx
0000000140ac45ad: 488b4004                 mov        rax, qword ptr [rax + 4]
0000000140ac45b1: 48894530                 mov        qword ptr [rbp + 0x30], rax
0000000140ac45b5: e952010000               jmp        0x140ac470c
0000000140ac45ba: 4588fe                   mov        r14b, r15b
0000000140ac45bd: 8b8794000000             mov        eax, dword ptr [rdi + 0x94]
0000000140ac45c3: 894538                   mov        dword ptr [rbp + 0x38], eax
0000000140ac45c6: 488b878c000000           mov        rax, qword ptr [rdi + 0x8c]
0000000140ac45cd: 48894530                 mov        qword ptr [rbp + 0x30], rax
0000000140ac45d1: 8b87a0000000             mov        eax, dword ptr [rdi + 0xa0]
0000000140ac45d7: 488d4d3c                 lea        rcx, [rbp + 0x3c]
0000000140ac45db: 894108                   mov        dword ptr [rcx + 8], eax
0000000140ac45de: 488b8798000000           mov        rax, qword ptr [rdi + 0x98]
0000000140ac45e5: 488901                   mov        qword ptr [rcx], rax
0000000140ac45e8: 0f57db                   xorps      xmm3, xmm3
0000000140ac45eb: 488d4d68                 lea        rcx, [rbp + 0x68]
0000000140ac45ef: 488d5530                 lea        rdx, [rbp + 0x30]
0000000140ac45f3: 4989d8                   mov        r8, rbx
0000000140ac45f6: e8f5bd7b00               call       0x1412803f0
0000000140ac45fb: f6456801                 test       byte ptr [rbp + 0x68], 1
0000000140ac45ff: 0f8455010000             je         0x140ac475a
0000000140ac4605: f30f104574               movss      xmm0, dword ptr [rbp + 0x74]
0000000140ac460a: f30f5c8790000000         subss      xmm0, dword ptr [rdi + 0x90]
0000000140ac4612: 0f107d6c                 movups     xmm7, xmmword ptr [rbp + 0x6c]
0000000140ac4616: f30f59c0                 mulss      xmm0, xmm0
0000000140ac461a: 0f28cf                   movaps     xmm1, xmm7
0000000140ac461d: 0fc6cfed                 shufps     xmm1, xmm7, 0xed
0000000140ac4621: f30f10978c000000         movss      xmm2, dword ptr [rdi + 0x8c]
0000000140ac4629: f30f109f94000000         movss      xmm3, dword ptr [rdi + 0x94]
0000000140ac4631: 0f14d3                   unpcklps   xmm2, xmm3
0000000140ac4634: 0f5cca                   subps      xmm1, xmm2
0000000140ac4637: 0f59c9                   mulps      xmm1, xmm1
0000000140ac463a: f30f58c1                 addss      xmm0, xmm1
0000000140ac463e: f30f16c9                 movshdup   xmm1, xmm1
0000000140ac4642: f30f58c8                 addss      xmm1, xmm0
0000000140ac4646: f30f1087a4000000         movss      xmm0, dword ptr [rdi + 0xa4]
0000000140ac464e: f30f59c0                 mulss      xmm0, xmm0
0000000140ac4652: 0f2ec1                   ucomiss    xmm0, xmm1
0000000140ac4655: 0f868a000000             jbe        0x140ac46e5
0000000140ac465b: 4881c798000000           add        rdi, 0x98
0000000140ac4662: 8b4708                   mov        eax, dword ptr [rdi + 8]
0000000140ac4665: 894520                   mov        dword ptr [rbp + 0x20], eax
0000000140ac4668: 488b07                   mov        rax, qword ptr [rdi]
0000000140ac466b: 48894518                 mov        qword ptr [rbp + 0x18], rax
0000000140ac466f: 488bb590000000           mov        rsi, qword ptr [rbp + 0x90]
0000000140ac4676: 483bb580000000           cmp        rsi, qword ptr [rbp + 0x80]
0000000140ac467d: 7513                     jne        0x140ac4692
0000000140ac467f: 488d8d80000000           lea        rcx, [rbp + 0x80]
0000000140ac4686: 488d1583b20102           lea        rdx, [rip + 0x201b283]
0000000140ac468d: e83ed3b6ff               call       0x1406319d0
0000000140ac4692: 488b8588000000           mov        rax, qword ptr [rbp + 0x88]
0000000140ac4699: 486bce38                 imul       rcx, rsi, 0x38
0000000140ac469d: c7040801000000           mov        dword ptr [rax + rcx], 1
0000000140ac46a4: 488b5518                 mov        rdx, qword ptr [rbp + 0x18]
0000000140ac46a8: 4889540804               mov        qword ptr [rax + rcx + 4], rdx
0000000140ac46ad: 8b5520                   mov        edx, dword ptr [rbp + 0x20]
0000000140ac46b0: 8954080c                 mov        dword ptr [rax + rcx + 0xc], edx
0000000140ac46b4: 488b5560                 mov        rdx, qword ptr [rbp + 0x60]
0000000140ac46b8: 4889540810               mov        qword ptr [rax + rcx + 0x10], rdx
0000000140ac46bd: 48c744081800000000       mov        qword ptr [rax + rcx + 0x18], 0
0000000140ac46c6: 0f117c0820               movups     xmmword ptr [rax + rcx + 0x20], xmm7
0000000140ac46cb: 4488740830               mov        byte ptr [rax + rcx + 0x30], r14b
0000000140ac46d0: 0fb695a7000000           movzx      edx, byte ptr [rbp + 0xa7]
0000000140ac46d7: 88540831                 mov        byte ptr [rax + rcx + 0x31], dl
0000000140ac46db: 48ffc6                   inc        rsi
0000000140ac46de: 4889b590000000           mov        qword ptr [rbp + 0x90], rsi
0000000140ac46e5: 4c8bb598000000           mov        r14, qword ptr [rbp + 0x98]
0000000140ac46ec: e90efaffff               jmp        0x140ac40ff
0000000140ac46f1: 4c8bb598000000           mov        r14, qword ptr [rbp + 0x98]
0000000140ac46f8: e902faffff               jmp        0x140ac40ff
0000000140ac46fd: c7453800000000           mov        dword ptr [rbp + 0x38], 0
0000000140ac4704: 48c7453000000000         mov        qword ptr [rbp + 0x30], 0
0000000140ac470c: f30f10bf8c000000         movss      xmm7, dword ptr [rdi + 0x8c]
0000000140ac4714: 8b4538                   mov        eax, dword ptr [rbp + 0x38]
0000000140ac4717: 894570                   mov        dword ptr [rbp + 0x70], eax
0000000140ac471a: 488b4530                 mov        rax, qword ptr [rbp + 0x30]
0000000140ac471e: 48894568                 mov        qword ptr [rbp + 0x68], rax
0000000140ac4722: 488bb590000000           mov        rsi, qword ptr [rbp + 0x90]
0000000140ac4729: 483bb580000000           cmp        rsi, qword ptr [rbp + 0x80]
0000000140ac4730: 0f855ef9ffff             jne        0x140ac4094
0000000140ac4736: 488d8d80000000           lea        rcx, [rbp + 0x80]
0000000140ac473d: 488d15b4b10102           lea        rdx, [rip + 0x201b1b4]
0000000140ac4744: e887d2b6ff               call       0x1406319d0
0000000140ac4749: e946f9ffff               jmp        0x140ac4094
0000000140ac474e: 4c8bb598000000           mov        r14, qword ptr [rbp + 0x98]
0000000140ac4755: e9a5f9ffff               jmp        0x140ac40ff
0000000140ac475a: 4c8bb598000000           mov        r14, qword ptr [rbp + 0x98]
0000000140ac4761: e999f9ffff               jmp        0x140ac40ff
0000000140ac4766: f30f107d34               movss      xmm7, dword ptr [rbp + 0x34]
0000000140ac476b: 4c39f6                   cmp        rsi, r14
0000000140ac476e: 0f8429010000             je         0x140ac489d
0000000140ac4774: 4929f6                   sub        r14, rsi
0000000140ac4777: 49c1ee02                 shr        r14, 2
0000000140ac477b: 48b8398ee3388ee3388e     movabs     rax, 0x8e38e38e38e38e39
0000000140ac4785: 4c0faff0                 imul       r14, rax
0000000140ac4789: 4d89ec                   mov        r12, r13
0000000140ac478c: 4531ff                   xor        r15d, r15d
0000000140ac478f: eb1b                     jmp        0x140ac47ac
0000000140ac4791: 6666666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140ac47a0: 49ffc7                   inc        r15
0000000140ac47a3: 4d39f7                   cmp        r15, r14
0000000140ac47a6: 0f84f4000000             je         0x140ac48a0
0000000140ac47ac: 4b8d04ff                 lea        rax, [r15 + r15*8]
0000000140ac47b0: 4c8d0486                 lea        r8, [rsi + rax*4]
0000000140ac47b4: 488d4d30                 lea        rcx, [rbp + 0x30]
0000000140ac47b8: 4889da                   mov        rdx, rbx
0000000140ac47bb: e8e0677700               call       0x14123afa0
0000000140ac47c0: 49ffc5                   inc        r13
0000000140ac47c3: f6453001                 test       byte ptr [rbp + 0x30], 1
0000000140ac47c7: 74d7                     je         0x140ac47a0
0000000140ac47c9: f30f104534               movss      xmm0, dword ptr [rbp + 0x34]
0000000140ac47ce: 0f2ef8                   ucomiss    xmm7, xmm0
0000000140ac47d1: 0f8af8020000             jp         0x140ac4acf
0000000140ac47d7: f30f5dc7                 minss      xmm0, xmm7
0000000140ac47db: 4d0f47e5                 cmova      r12, r13
0000000140ac47df: 0f28f8                   movaps     xmm7, xmm0
0000000140ac47e2: ebbc                     jmp        0x140ac47a0
0000000140ac47e4: c7453800000000           mov        dword ptr [rbp + 0x38], 0
0000000140ac47eb: 48c7453000000000         mov        qword ptr [rbp + 0x30], 0
0000000140ac47f3: 8b4538                   mov        eax, dword ptr [rbp + 0x38]
0000000140ac47f6: 894570                   mov        dword ptr [rbp + 0x70], eax
0000000140ac47f9: 488b4530                 mov        rax, qword ptr [rbp + 0x30]
0000000140ac47fd: 48894568                 mov        qword ptr [rbp + 0x68], rax
0000000140ac4801: 488b75f8                 mov        rsi, qword ptr [rbp - 8]
0000000140ac4805: 483b75e8                 cmp        rsi, qword ptr [rbp - 0x18]
0000000140ac4809: 7510                     jne        0x140ac481b
0000000140ac480b: 488d4de8                 lea        rcx, [rbp - 0x18]
0000000140ac480f: 488d15cab00102           lea        rdx, [rip + 0x201b0ca]
0000000140ac4816: e895dfa9ff               call       0x1405627b0
0000000140ac481b: 4881c789000000           add        rdi, 0x89
0000000140ac4822: 488b45f0                 mov        rax, qword ptr [rbp - 0x10]
0000000140ac4826: 4889f1                   mov        rcx, rsi
0000000140ac4829: 48c1e106                 shl        rcx, 6
0000000140ac482d: 48893c08                 mov        qword ptr [rax + rcx], rdi
0000000140ac4831: c744080801000000         mov        dword ptr [rax + rcx + 8], 1
0000000140ac4839: 48ba000000000000803f     movabs     rdx, 0x3f80000000000000
0000000140ac4843: 488954080c               mov        qword ptr [rax + rcx + 0xc], rdx
0000000140ac4848: c744081400000000         mov        dword ptr [rax + rcx + 0x14], 0
0000000140ac4850: 488b5560                 mov        rdx, qword ptr [rbp + 0x60]
0000000140ac4854: 4889540818               mov        qword ptr [rax + rcx + 0x18], rdx
0000000140ac4859: 48c744082000000000       mov        qword ptr [rax + rcx + 0x20], 0
0000000140ac4862: c7440828ffff7f7f         mov        dword ptr [rax + rcx + 0x28], 0x7f7fffff
0000000140ac486a: 488b5568                 mov        rdx, qword ptr [rbp + 0x68]
0000000140ac486e: 488954082c               mov        qword ptr [rax + rcx + 0x2c], rdx
0000000140ac4873: 8b5570                   mov        edx, dword ptr [rbp + 0x70]
0000000140ac4876: 89540834                 mov        dword ptr [rax + rcx + 0x34], edx
0000000140ac487a: c644083800               mov        byte ptr [rax + rcx + 0x38], 0
0000000140ac487f: 0fb695a7000000           movzx      edx, byte ptr [rbp + 0xa7]
0000000140ac4886: 88540839                 mov        byte ptr [rax + rcx + 0x39], dl
0000000140ac488a: 48ffc6                   inc        rsi
0000000140ac488d: 488975f8                 mov        qword ptr [rbp - 8], rsi
0000000140ac4891: 4c8bb598000000           mov        r14, qword ptr [rbp + 0x98]
0000000140ac4898: e962f8ffff               jmp        0x140ac40ff
0000000140ac489d: 4d89ec                   mov        r12, r13
0000000140ac48a0: 488b97a0000000           mov        rdx, qword ptr [rdi + 0xa0]
0000000140ac48a7: 4939d4                   cmp        r12, rdx
0000000140ac48aa: 0f834c020000             jae        0x140ac4afc
0000000140ac48b0: 488b4d08                 mov        rcx, qword ptr [rbp + 8]
0000000140ac48b4: 488b01                   mov        rax, qword ptr [rcx]
0000000140ac48b7: 488b4908                 mov        rcx, qword ptr [rcx + 8]
0000000140ac48bb: f2440f1000               movsd      xmm8, qword ptr [rax]
0000000140ac48c0: f2440f1011               movsd      xmm10, qword ptr [rcx]
0000000140ac48c5: f3440f104908             movss      xmm9, dword ptr [rcx + 8]
0000000140ac48cb: f3440f59cf               mulss      xmm9, xmm7
0000000140ac48d0: f3440f584808             addss      xmm9, dword ptr [rax + 8]
0000000140ac48d6: 4b8d14e4                 lea        rdx, [r12 + r12*8]
0000000140ac48da: 48c1e202                 shl        rdx, 2
0000000140ac48de: 48039798000000           add        rdx, qword ptr [rdi + 0x98]
0000000140ac48e5: 488d4d30                 lea        rcx, [rbp + 0x30]
0000000140ac48e9: e8b2647700               call       0x14123ada0
0000000140ac48ee: 8b4538                   mov        eax, dword ptr [rbp + 0x38]
0000000140ac48f1: 894570                   mov        dword ptr [rbp + 0x70], eax
0000000140ac48f4: 488b4530                 mov        rax, qword ptr [rbp + 0x30]
0000000140ac48f8: 48894568                 mov        qword ptr [rbp + 0x68], rax
0000000140ac48fc: 488bb590000000           mov        rsi, qword ptr [rbp + 0x90]
0000000140ac4903: 483bb580000000           cmp        rsi, qword ptr [rbp + 0x80]
0000000140ac490a: 7513                     jne        0x140ac491f
0000000140ac490c: 488d8d80000000           lea        rcx, [rbp + 0x80]
0000000140ac4913: 488d157eaf0102           lea        rdx, [rip + 0x201af7e]
0000000140ac491a: e8b1d0b6ff               call       0x1406319d0
0000000140ac491f: f30f12c7                 movsldup   xmm0, xmm7
0000000140ac4923: 410f59c2                 mulps      xmm0, xmm10
0000000140ac4927: 440f58c0                 addps      xmm8, xmm0
0000000140ac492b: 488b8588000000           mov        rax, qword ptr [rbp + 0x88]
0000000140ac4932: 486bce38                 imul       rcx, rsi, 0x38
0000000140ac4936: c7040801000000           mov        dword ptr [rax + rcx], 1
0000000140ac493d: 488b5568                 mov        rdx, qword ptr [rbp + 0x68]
0000000140ac4941: 4889540804               mov        qword ptr [rax + rcx + 4], rdx
0000000140ac4946: 8b5570                   mov        edx, dword ptr [rbp + 0x70]
0000000140ac4949: 8954080c                 mov        dword ptr [rax + rcx + 0xc], edx
0000000140ac494d: 488b5560                 mov        rdx, qword ptr [rbp + 0x60]
0000000140ac4951: 4889540810               mov        qword ptr [rax + rcx + 0x10], rdx
0000000140ac4956: 4c89640818               mov        qword ptr [rax + rcx + 0x18], r12
0000000140ac495b: f30f117c0820             movss      dword ptr [rax + rcx + 0x20], xmm7
0000000140ac4961: 440f13440824             movlps     qword ptr [rax + rcx + 0x24], xmm8
0000000140ac4967: f3440f114c082c           movss      dword ptr [rax + rcx + 0x2c], xmm9
0000000140ac496e: 0fb65528                 movzx      edx, byte ptr [rbp + 0x28]
0000000140ac4972: 88540830                 mov        byte ptr [rax + rcx + 0x30], dl
0000000140ac4976: e968f7ffff               jmp        0x140ac40e3
0000000140ac497b: 4c897528                 mov        qword ptr [rbp + 0x28], r14
0000000140ac497f: 488b4d08                 mov        rcx, qword ptr [rbp + 8]
0000000140ac4983: 488b01                   mov        rax, qword ptr [rcx]
0000000140ac4986: 488b4908                 mov        rcx, qword ptr [rcx + 8]
0000000140ac498a: f2440f1000               movsd      xmm8, qword ptr [rax]
0000000140ac498f: f3440f104908             movss      xmm9, dword ptr [rcx + 8]
0000000140ac4995: f3440f59cf               mulss      xmm9, xmm7
0000000140ac499a: f3440f584808             addss      xmm9, dword ptr [rax + 8]
0000000140ac49a0: f2440f1011               movsd      xmm10, qword ptr [rcx]
0000000140ac49a5: 488bb590000000           mov        rsi, qword ptr [rbp + 0x90]
0000000140ac49ac: 483bb580000000           cmp        rsi, qword ptr [rbp + 0x80]
0000000140ac49b3: 7513                     jne        0x140ac49c8
0000000140ac49b5: 488d8d80000000           lea        rcx, [rbp + 0x80]
0000000140ac49bc: 488d15a5ae0102           lea        rdx, [rip + 0x201aea5]
0000000140ac49c3: e808d0b6ff               call       0x1406319d0
0000000140ac49c8: f30f12c7                 movsldup   xmm0, xmm7
0000000140ac49cc: 410f59c2                 mulps      xmm0, xmm10
0000000140ac49d0: 440f58c0                 addps      xmm8, xmm0
0000000140ac49d4: 488b8588000000           mov        rax, qword ptr [rbp + 0x88]
0000000140ac49db: 486bce38                 imul       rcx, rsi, 0x38
0000000140ac49df: c7040800000000           mov        dword ptr [rax + rcx], 0
0000000140ac49e6: 488b55dc                 mov        rdx, qword ptr [rbp - 0x24]
0000000140ac49ea: 4889540804               mov        qword ptr [rax + rcx + 4], rdx
0000000140ac49ef: 8b55e4                   mov        edx, dword ptr [rbp - 0x1c]
0000000140ac49f2: 8954080c                 mov        dword ptr [rax + rcx + 0xc], edx
0000000140ac49f6: 488b5560                 mov        rdx, qword ptr [rbp + 0x60]
0000000140ac49fa: 4889540810               mov        qword ptr [rax + rcx + 0x10], rdx
0000000140ac49ff: 488b5528                 mov        rdx, qword ptr [rbp + 0x28]
0000000140ac4a03: 4889540818               mov        qword ptr [rax + rcx + 0x18], rdx
0000000140ac4a08: f30f117c0820             movss      dword ptr [rax + rcx + 0x20], xmm7
0000000140ac4a0e: 440f13440824             movlps     qword ptr [rax + rcx + 0x24], xmm8
0000000140ac4a14: f3440f114c082c           movss      dword ptr [rax + rcx + 0x2c], xmm9
0000000140ac4a1b: 44887c0830               mov        byte ptr [rax + rcx + 0x30], r15b
0000000140ac4a20: e9bef6ffff               jmp        0x140ac40e3
0000000140ac4a25: 488b8590000000           mov        rax, qword ptr [rbp + 0x90]
0000000140ac4a2c: 48894540                 mov        qword ptr [rbp + 0x40], rax
0000000140ac4a30: 0f108580000000           movups     xmm0, xmmword ptr [rbp + 0x80]
0000000140ac4a37: 0f294530                 movaps     xmmword ptr [rbp + 0x30], xmm0
0000000140ac4a3b: 0f104de8                 movups     xmm1, xmmword ptr [rbp - 0x18]
0000000140ac4a3f: 0f114d48                 movups     xmmword ptr [rbp + 0x48], xmm1
0000000140ac4a43: 488b45f8                 mov        rax, qword ptr [rbp - 8]
0000000140ac4a47: 48894558                 mov        qword ptr [rbp + 0x58], rax
0000000140ac4a4b: 488b4550                 mov        rax, qword ptr [rbp + 0x50]
0000000140ac4a4f: 488b4dc8                 mov        rcx, qword ptr [rbp - 0x38]
0000000140ac4a53: 48894120                 mov        qword ptr [rcx + 0x20], rax
0000000140ac4a57: 488b4558                 mov        rax, qword ptr [rbp + 0x58]
0000000140ac4a5b: 48894128                 mov        qword ptr [rcx + 0x28], rax
0000000140ac4a5f: 488b4540                 mov        rax, qword ptr [rbp + 0x40]
0000000140ac4a63: 48894110                 mov        qword ptr [rcx + 0x10], rax
0000000140ac4a67: 488b4548                 mov        rax, qword ptr [rbp + 0x48]
0000000140ac4a6b: 48894118                 mov        qword ptr [rcx + 0x18], rax
0000000140ac4a6f: 0f1101                   movups     xmmword ptr [rcx], xmm0
0000000140ac4a72: 488b4510                 mov        rax, qword ptr [rbp + 0x10]
0000000140ac4a76: c6402001                 mov        byte ptr [rax + 0x20], 1
0000000140ac4a7a: 4889c8                   mov        rax, rcx
0000000140ac4a7d: 0f28b5b0000000           movaps     xmm6, xmmword ptr [rbp + 0xb0]
0000000140ac4a84: 0f28bdc0000000           movaps     xmm7, xmmword ptr [rbp + 0xc0]
0000000140ac4a8b: 440f2885d0000000         movaps     xmm8, xmmword ptr [rbp + 0xd0]
0000000140ac4a93: 440f288de0000000         movaps     xmm9, xmmword ptr [rbp + 0xe0]
0000000140ac4a9b: 440f2895f0000000         movaps     xmm10, xmmword ptr [rbp + 0xf0]
0000000140ac4aa3: 440f289d00010000         movaps     xmm11, xmmword ptr [rbp + 0x100]
0000000140ac4aab: 440f28a510010000         movaps     xmm12, xmmword ptr [rbp + 0x110]
0000000140ac4ab3: 440f28ad20010000         movaps     xmm13, xmmword ptr [rbp + 0x120]
0000000140ac4abb: 4881c4b8010000           add        rsp, 0x1b8
0000000140ac4ac2: 5b                       pop        rbx
0000000140ac4ac3: 5f                       pop        rdi
0000000140ac4ac4: 5e                       pop        rsi
0000000140ac4ac5: 415c                     pop        r12
0000000140ac4ac7: 415d                     pop        r13
0000000140ac4ac9: 415e                     pop        r14
0000000140ac4acb: 415f                     pop        r15
0000000140ac4acd: 5d                       pop        rbp
0000000140ac4ace: c3                       ret        
0000000140ac4acf: 488d0d120d2102           lea        rcx, [rip + 0x2210d12]
0000000140ac4ad6: e83548e101               call       0x1428d9310
0000000140ac4adb: eb42                     jmp        0x140ac4b1f
0000000140ac4add: 488d0dec0c2102           lea        rcx, [rip + 0x2210cec]
0000000140ac4ae4: e82748e101               call       0x1428d9310
0000000140ac4ae9: eb34                     jmp        0x140ac4b1f
0000000140ac4aeb: 488d0d86aa0102           lea        rcx, [rip + 0x201aa86]
0000000140ac4af2: 83f801                   cmp        eax, 1
0000000140ac4af5: 752a                     jne        0x140ac4b21
0000000140ac4af7: e8644fe101               call       0x1428d9a60
0000000140ac4afc: 4c8d057dad0102           lea        r8, [rip + 0x201ad7d]
0000000140ac4b03: 4c89e1                   mov        rcx, r12
0000000140ac4b06: e80d4ae101               call       0x1428d9518
0000000140ac4b0b: eb12                     jmp        0x140ac4b1f
0000000140ac4b0d: 4c8d0524ad0102           lea        r8, [rip + 0x201ad24]
0000000140ac4b14: 4889d9                   mov        rcx, rbx
0000000140ac4b17: 4889f2                   mov        rdx, rsi
0000000140ac4b1a: e8c444e101               call       0x1428d8fe3
0000000140ac4b1f: 0f0b                     ud2        
0000000140ac4b21: e87a4fe101               call       0x1428d9aa0
0000000140ac4b26: cc                       int3       
0000000140ac4b27: 660f1f840000000000       nop        word ptr [rax + rax]
0000000140ac4b30: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140ac4b35: 55                       push       rbp
0000000140ac4b36: 4157                     push       r15
0000000140ac4b38: 4156                     push       r14
0000000140ac4b3a: 4155                     push       r13
0000000140ac4b3c: 4154                     push       r12
0000000140ac4b3e: 56                       push       rsi
0000000140ac4b3f: 57                       push       rdi
0000000140ac4b40: 53                       push       rbx
0000000140ac4b41: 4881ecc8000000           sub        rsp, 0xc8
0000000140ac4b48: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140ac4b4f: 440f296c2440             movaps     xmmword ptr [rsp + 0x40], xmm13
0000000140ac4b55: 440f29642450             movaps     xmmword ptr [rsp + 0x50], xmm12
0000000140ac4b5b: 440f295c2460             movaps     xmmword ptr [rsp + 0x60], xmm11
0000000140ac4b61: 440f29542470             movaps     xmmword ptr [rsp + 0x70], xmm10
0000000140ac4b67: 440f298c2480000000       movaps     xmmword ptr [rsp + 0x80], xmm9
0000000140ac4b70: 440f29842490000000       movaps     xmmword ptr [rsp + 0x90], xmm8
0000000140ac4b79: 0f29bc24a0000000         movaps     xmmword ptr [rsp + 0xa0], xmm7
0000000140ac4b81: 0f29b424b0000000         movaps     xmmword ptr [rsp + 0xb0], xmm6
0000000140ac4b89: 488b55e8                 mov        rdx, qword ptr [rbp - 0x18]
0000000140ac4b8d: 4885d2                   test       rdx, rdx
0000000140ac4b90: 7413                     je         0x140ac4ba5
0000000140ac4b92: 488b4df0                 mov        rcx, qword ptr [rbp - 0x10]
0000000140ac4b96: 48c1e206                 shl        rdx, 6
0000000140ac4b9a: 41b808000000             mov        r8d, 8
0000000140ac4ba0: e87bf0b4ff               call       0x140613c20
0000000140ac4ba5: 488b8580000000           mov        rax, qword ptr [rbp + 0x80]
0000000140ac4bac: 4885c0                   test       rax, rax
0000000140ac4baf: 7416                     je         0x140ac4bc7
0000000140ac4bb1: 488b8d88000000           mov        rcx, qword ptr [rbp + 0x88]
0000000140ac4bb8: 486bd038                 imul       rdx, rax, 0x38
0000000140ac4bbc: 41b808000000             mov        r8d, 8
0000000140ac4bc2: e859f0b4ff               call       0x140613c20
0000000140ac4bc7: 0f28b424b0000000         movaps     xmm6, xmmword ptr [rsp + 0xb0]
0000000140ac4bcf: 0f28bc24a0000000         movaps     xmm7, xmmword ptr [rsp + 0xa0]
0000000140ac4bd7: 440f28842490000000       movaps     xmm8, xmmword ptr [rsp + 0x90]
0000000140ac4be0: 440f288c2480000000       movaps     xmm9, xmmword ptr [rsp + 0x80]
0000000140ac4be9: 440f28542470             movaps     xmm10, xmmword ptr [rsp + 0x70]
0000000140ac4bef: 440f285c2460             movaps     xmm11, xmmword ptr [rsp + 0x60]
0000000140ac4bf5: 440f28642450             movaps     xmm12, xmmword ptr [rsp + 0x50]
0000000140ac4bfb: 440f286c2440             movaps     xmm13, xmmword ptr [rsp + 0x40]
0000000140ac4c01: 4881c4c8000000           add        rsp, 0xc8
0000000140ac4c08: 5b                       pop        rbx
0000000140ac4c09: 5f                       pop        rdi
0000000140ac4c0a: 5e                       pop        rsi
0000000140ac4c0b: 415c                     pop        r12
0000000140ac4c0d: 415d                     pop        r13
0000000140ac4c0f: 415e                     pop        r14
0000000140ac4c11: 415f                     pop        r15
0000000140ac4c13: 5d                       pop        rbp
0000000140ac4c14: c3                       ret        
0000000140ac4c15: 66662e0f1f840000000000   nop        word ptr cs:[rax + rax]
0000000140ac4c20: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140ac4c25: 55                       push       rbp
0000000140ac4c26: 4157                     push       r15
0000000140ac4c28: 4156                     push       r14
0000000140ac4c2a: 4155                     push       r13
0000000140ac4c2c: 4154                     push       r12
0000000140ac4c2e: 56                       push       rsi
0000000140ac4c2f: 57                       push       rdi
0000000140ac4c30: 53                       push       rbx
0000000140ac4c31: 4881ecc8000000           sub        rsp, 0xc8
0000000140ac4c38: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140ac4c3f: 440f296c2440             movaps     xmmword ptr [rsp + 0x40], xmm13
0000000140ac4c45: 440f29642450             movaps     xmmword ptr [rsp + 0x50], xmm12
0000000140ac4c4b: 440f295c2460             movaps     xmmword ptr [rsp + 0x60], xmm11
0000000140ac4c51: 440f29542470             movaps     xmmword ptr [rsp + 0x70], xmm10
0000000140ac4c57: 440f298c2480000000       movaps     xmmword ptr [rsp + 0x80], xmm9
0000000140ac4c60: 440f29842490000000       movaps     xmmword ptr [rsp + 0x90], xmm8
0000000140ac4c69: 0f29bc24a0000000         movaps     xmmword ptr [rsp + 0xa0], xmm7
0000000140ac4c71: 0f29b424b0000000         movaps     xmmword ptr [rsp + 0xb0], xmm6
0000000140ac4c79: 488b4510                 mov        rax, qword ptr [rbp + 0x10]
0000000140ac4c7d: c6402002                 mov        byte ptr [rax + 0x20], 2
0000000140ac4c81: 0f28b424b0000000         movaps     xmm6, xmmword ptr [rsp + 0xb0]
0000000140ac4c89: 0f28bc24a0000000         movaps     xmm7, xmmword ptr [rsp + 0xa0]
0000000140ac4c91: 440f28842490000000       movaps     xmm8, xmmword ptr [rsp + 0x90]
0000000140ac4c9a: 440f288c2480000000       movaps     xmm9, xmmword ptr [rsp + 0x80]
0000000140ac4ca3: 440f28542470             movaps     xmm10, xmmword ptr [rsp + 0x70]
0000000140ac4ca9: 440f285c2460             movaps     xmm11, xmmword ptr [rsp + 0x60]
0000000140ac4caf: 440f28642450             movaps     xmm12, xmmword ptr [rsp + 0x50]
0000000140ac4cb5: 440f286c2440             movaps     xmm13, xmmword ptr [rsp + 0x40]
0000000140ac4cbb: 4881c4c8000000           add        rsp, 0xc8
0000000140ac4cc2: 5b                       pop        rbx
0000000140ac4cc3: 5f                       pop        rdi
0000000140ac4cc4: 5e                       pop        rsi
0000000140ac4cc5: 415c                     pop        r12
0000000140ac4cc7: 415d                     pop        r13
0000000140ac4cc9: 415e                     pop        r14
0000000140ac4ccb: 415f                     pop        r15
0000000140ac4ccd: 5d                       pop        rbp
0000000140ac4cce: c3                       ret        