00000001412f6680: 4156                     push       r14
00000001412f6682: 56                       push       rsi
00000001412f6683: 57                       push       rdi
00000001412f6684: 53                       push       rbx
00000001412f6685: 488b4110                 mov        rax, qword ptr [rcx + 0x10]
00000001412f6689: 4885c0                   test       rax, rax
00000001412f668c: 0f84e5010000             je         0x1412f6877
00000001412f6692: 4c8b4108                 mov        r8, qword ptr [rcx + 8]
00000001412f6696: 4c8b0a                   mov        r9, qword ptr [rdx]
00000001412f6699: 4c8b5208                 mov        r10, qword ptr [rdx + 8]
00000001412f669d: 4c8b5a10                 mov        r11, qword ptr [rdx + 0x10]
00000001412f66a1: 488b5218                 mov        rdx, qword ptr [rdx + 0x18]
00000001412f66a5: 4983c038                 add        r8, 0x38
00000001412f66a9: be01000000               mov        esi, 1
00000001412f66ae: 4829c6                   sub        rsi, rax
00000001412f66b1: 0f280538f26201           movaps     xmm0, xmmword ptr [rip + 0x162f238]
00000001412f66b8: f30f100d44f26201         movss      xmm1, dword ptr [rip + 0x162f244]
00000001412f66c0: 31ff                     xor        edi, edi
00000001412f66c2: eb1d                     jmp        0x1412f66e1
00000001412f66c4: 6666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
00000001412f66d0: 4983c038                 add        r8, 0x38
00000001412f66d4: 48ffc6                   inc        rsi
00000001412f66d7: 4883fe01                 cmp        rsi, 1
00000001412f66db: 0f848f010000             je         0x1412f6870
00000001412f66e1: 418078f801               cmp        byte ptr [r8 - 8], 1
00000001412f66e6: 75e8                     jne        0x1412f66d0
00000001412f66e8: f3410f1050e8             movss      xmm2, dword ptr [r8 - 0x18]
00000001412f66ee: f3410f105a04             movss      xmm3, dword ptr [r10 + 4]
00000001412f66f4: f30f59da                 mulss      xmm3, xmm2
00000001412f66f8: f3410f585904             addss      xmm3, dword ptr [r9 + 4]
00000001412f66fe: 498b1b                   mov        rbx, qword ptr [r11]
00000001412f6701: f30f599b98000000         mulss      xmm3, dword ptr [rbx + 0x98]
00000001412f6709: f30f12d2                 movsldup   xmm2, xmm2
00000001412f670d: f3410f1022               movss      xmm4, dword ptr [r10]
00000001412f6712: f3410f106a08             movss      xmm5, dword ptr [r10 + 8]
00000001412f6718: 0f14e5                   unpcklps   xmm4, xmm5
00000001412f671b: 0f59e2                   mulps      xmm4, xmm2
00000001412f671e: f3410f1011               movss      xmm2, dword ptr [r9]
00000001412f6723: f3410f106908             movss      xmm5, dword ptr [r9 + 8]
00000001412f6729: 0f14d5                   unpcklps   xmm2, xmm5
00000001412f672c: 0f58d4                   addps      xmm2, xmm4
00000001412f672f: f30f10a388000000         movss      xmm4, dword ptr [rbx + 0x88]
00000001412f6737: f30f10aba8000000         movss      xmm5, dword ptr [rbx + 0xa8]
00000001412f673f: 0f14e5                   unpcklps   xmm4, xmm5
00000001412f6742: 0f59e2                   mulps      xmm4, xmm2
00000001412f6745: f30f58dc                 addss      xmm3, xmm4
00000001412f6749: f30f16d4                 movshdup   xmm2, xmm4
00000001412f674d: f30f58d3                 addss      xmm2, xmm3
00000001412f6751: f30f5893b8000000         addss      xmm2, dword ptr [rbx + 0xb8]
00000001412f6759: 0f57d0                   xorps      xmm2, xmm0
00000001412f675c: 488b1a                   mov        rbx, qword ptr [rdx]
00000001412f675f: f30f105b04               movss      xmm3, dword ptr [rbx + 4]
00000001412f6764: f30f58d9                 addss      xmm3, xmm1
00000001412f6768: 0f2eda                   ucomiss    xmm3, xmm2
00000001412f676b: 0f875fffffff             ja         0x1412f66d0
00000001412f6771: 4885f6                   test       rsi, rsi
00000001412f6774: 0f84f1000000             je         0x1412f686b
00000001412f677a: 48f7de                   neg        rsi
00000001412f677d: bf01000000               mov        edi, 1
00000001412f6782: eb45                     jmp        0x1412f67c9
00000001412f6784: 6666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
00000001412f6790: 486bdfc8                 imul       rbx, rdi, -0x38
00000001412f6794: 4d8b7030                 mov        r14, qword ptr [r8 + 0x30]
00000001412f6798: 4d89741830               mov        qword ptr [r8 + rbx + 0x30], r14
00000001412f679d: 410f1010                 movups     xmm2, xmmword ptr [r8]
00000001412f67a1: 410f105810               movups     xmm3, xmmword ptr [r8 + 0x10]
00000001412f67a6: 410f106020               movups     xmm4, xmmword ptr [r8 + 0x20]
00000001412f67ab: 410f11641820             movups     xmmword ptr [r8 + rbx + 0x20], xmm4
00000001412f67b1: 410f115c1810             movups     xmmword ptr [r8 + rbx + 0x10], xmm3
00000001412f67b7: 410f111418               movups     xmmword ptr [r8 + rbx], xmm2
00000001412f67bc: 4983c038                 add        r8, 0x38
00000001412f67c0: 48ffce                   dec        rsi
00000001412f67c3: 0f84a7000000             je         0x1412f6870
00000001412f67c9: 4180783001               cmp        byte ptr [r8 + 0x30], 1
00000001412f67ce: 75c0                     jne        0x1412f6790
00000001412f67d0: f3410f105020             movss      xmm2, dword ptr [r8 + 0x20]
00000001412f67d6: f3410f105a04             movss      xmm3, dword ptr [r10 + 4]
00000001412f67dc: f30f59da                 mulss      xmm3, xmm2
00000001412f67e0: f3410f585904             addss      xmm3, dword ptr [r9 + 4]
00000001412f67e6: 498b1b                   mov        rbx, qword ptr [r11]
00000001412f67e9: f30f599b98000000         mulss      xmm3, dword ptr [rbx + 0x98]
00000001412f67f1: f30f12d2                 movsldup   xmm2, xmm2
00000001412f67f5: f3410f1022               movss      xmm4, dword ptr [r10]
00000001412f67fa: f3410f106a08             movss      xmm5, dword ptr [r10 + 8]
00000001412f6800: 0f14e5                   unpcklps   xmm4, xmm5
00000001412f6803: 0f59e2                   mulps      xmm4, xmm2
00000001412f6806: f3410f1011               movss      xmm2, dword ptr [r9]
00000001412f680b: f3410f106908             movss      xmm5, dword ptr [r9 + 8]
00000001412f6811: 0f14d5                   unpcklps   xmm2, xmm5
00000001412f6814: 0f58d4                   addps      xmm2, xmm4
00000001412f6817: f30f10a388000000         movss      xmm4, dword ptr [rbx + 0x88]
00000001412f681f: f30f10aba8000000         movss      xmm5, dword ptr [rbx + 0xa8]
00000001412f6827: 0f14e5                   unpcklps   xmm4, xmm5
00000001412f682a: 0f59e2                   mulps      xmm4, xmm2
00000001412f682d: f30f58dc                 addss      xmm3, xmm4
00000001412f6831: f30f16d4                 movshdup   xmm2, xmm4
00000001412f6835: f30f58d3                 addss      xmm2, xmm3
00000001412f6839: f30f5893b8000000         addss      xmm2, dword ptr [rbx + 0xb8]
00000001412f6841: 0f57d0                   xorps      xmm2, xmm0
00000001412f6844: 488b1a                   mov        rbx, qword ptr [rdx]
00000001412f6847: f30f105b04               movss      xmm3, dword ptr [rbx + 4]
00000001412f684c: f30f58d9                 addss      xmm3, xmm1
00000001412f6850: 0f2eda                   ucomiss    xmm3, xmm2
00000001412f6853: 0f8737ffffff             ja         0x1412f6790
00000001412f6859: 48ffc7                   inc        rdi
00000001412f685c: 4983c038                 add        r8, 0x38
00000001412f6860: 48ffce                   dec        rsi
00000001412f6863: 0f8560ffffff             jne        0x1412f67c9
00000001412f6869: eb05                     jmp        0x1412f6870
00000001412f686b: bf01000000               mov        edi, 1
00000001412f6870: 4829f8                   sub        rax, rdi
00000001412f6873: 48894110                 mov        qword ptr [rcx + 0x10], rax
00000001412f6877: 5b                       pop        rbx
00000001412f6878: 5f                       pop        rdi
00000001412f6879: 5e                       pop        rsi
00000001412f687a: 415e                     pop        r14
00000001412f687c: c3                       ret        