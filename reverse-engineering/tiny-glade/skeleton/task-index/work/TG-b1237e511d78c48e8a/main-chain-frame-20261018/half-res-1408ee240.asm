00000001408ee240  4157                     push       r15
00000001408ee242  4156                     push       r14
00000001408ee244  4155                     push       r13
00000001408ee246  4154                     push       r12
00000001408ee248  56                       push       rsi
00000001408ee249  57                       push       rdi
00000001408ee24a  55                       push       rbp
00000001408ee24b  53                       push       rbx
00000001408ee24c  4881ecd8000000           sub        rsp, 0xd8
00000001408ee253  4489cd                   mov        ebp, r9d
00000001408ee256  4c89c7                   mov        rdi, r8
00000001408ee259  4889d6                   mov        rsi, rdx
00000001408ee25c  4889cb                   mov        rbx, rcx
00000001408ee25f  48898c2488000000         mov        qword ptr [rsp + 0x88], rcx
00000001408ee267  4489c9                   mov        ecx, r9d
00000001408ee26a  e8a12b3200               call       0x140c10e10
00000001408ee26f  84c0                     test       al, al
00000001408ee271  0f85ec040000             jne        0x1408ee763
00000001408ee277  4c8bb638010000           mov        r14, qword ptr [rsi + 0x138]
00000001408ee27e  4983beb000000000         cmp        qword ptr [r14 + 0xb0], 0
00000001408ee286  0f84b2000000             je         0x1408ee33e
00000001408ee28c  4c8ba42440010000         mov        r12, qword ptr [rsp + 0x140]
00000001408ee294  4989f8                   mov        r8, rdi
00000001408ee297  49c1e839                 shr        r8, 0x39
00000001408ee29b  498b8698000000           mov        rax, qword ptr [r14 + 0x98]
00000001408ee2a2  498b8ea0000000           mov        rcx, qword ptr [r14 + 0xa0]
00000001408ee2a9  4889ca                   mov        rdx, rcx
00000001408ee2ac  4821fa                   and        rdx, rdi
00000001408ee2af  66410f6ec0               movd       xmm0, r8d
00000001408ee2b4  660f60c0                 punpcklbw  xmm0, xmm0
00000001408ee2b8  f20f70c000               pshuflw    xmm0, xmm0, 0
00000001408ee2bd  660f70c000               pshufd     xmm0, xmm0, 0
00000001408ee2c2  4c8d40f0                 lea        r8, [rax - 0x10]
00000001408ee2c6  4531c9                   xor        r9d, r9d
00000001408ee2c9  660f76c9                 pcmpeqd    xmm1, xmm1
00000001408ee2cd  f30f6f1410               movdqu     xmm2, xmmword ptr [rax + rdx]
00000001408ee2d2  660f6fda                 movdqa     xmm3, xmm2
00000001408ee2d6  660f74d8                 pcmpeqb    xmm3, xmm0
00000001408ee2da  66440fd7d3               pmovmskb   r10d, xmm3
00000001408ee2df  4585d2                   test       r10d, r10d
00000001408ee2e2  743c                     je         0x1408ee320
00000001408ee2e4  f3450fbcda               tzcnt      r11d, r10d
00000001408ee2e9  4901d3                   add        r11, rdx
00000001408ee2ec  4921cb                   and        r11, rcx
00000001408ee2ef  4d89df                   mov        r15, r11
00000001408ee2f2  49c1e704                 shl        r15, 4
00000001408ee2f6  4d89c5                   mov        r13, r8
00000001408ee2f9  4d29fd                   sub        r13, r15
00000001408ee2fc  49397d00                 cmp        qword ptr [r13], rdi
00000001408ee300  0f840a010000             je         0x1408ee410
00000001408ee306  458d5aff                 lea        r11d, [r10 - 1]
00000001408ee30a  664521d3                 and        r11w, r10w
00000001408ee30e  4589da                   mov        r10d, r11d
00000001408ee311  75d1                     jne        0x1408ee2e4
00000001408ee313  666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
00000001408ee320  660f74d1                 pcmpeqb    xmm2, xmm1
00000001408ee324  66440fd7d2               pmovmskb   r10d, xmm2
00000001408ee329  4585d2                   test       r10d, r10d
00000001408ee32c  7510                     jne        0x1408ee33e
00000001408ee32e  4c01ca                   add        rdx, r9
00000001408ee331  4883c210                 add        rdx, 0x10
00000001408ee335  4983c110                 add        r9, 0x10
00000001408ee339  4821ca                   and        rdx, rcx
00000001408ee33c  eb8f                     jmp        0x1408ee2cd
00000001408ee33e  4889f9                   mov        rcx, rdi
00000001408ee341  e80a933900               call       0x140c87650
00000001408ee346  80be3601000008           cmp        byte ptr [rsi + 0x136], 8
00000001408ee34d  0f84b4030000             je         0x1408ee707
00000001408ee353  4983bee800000000         cmp        qword ptr [r14 + 0xe8], 0
00000001408ee35b  0f84a6030000             je         0x1408ee707
00000001408ee361  4983be3001000000         cmp        qword ptr [r14 + 0x130], 0
00000001408ee369  0f8498030000             je         0x1408ee707
00000001408ee36f  4989c1                   mov        r9, rax
00000001408ee372  49c1e939                 shr        r9, 0x39
00000001408ee376  498b8e18010000           mov        rcx, qword ptr [r14 + 0x118]
00000001408ee37d  498b9620010000           mov        rdx, qword ptr [r14 + 0x120]
00000001408ee384  4989d0                   mov        r8, rdx
00000001408ee387  4921c0                   and        r8, rax
00000001408ee38a  66410f6ec1               movd       xmm0, r9d
00000001408ee38f  660f60c0                 punpcklbw  xmm0, xmm0
00000001408ee393  f20f70c000               pshuflw    xmm0, xmm0, 0
00000001408ee398  660f70c000               pshufd     xmm0, xmm0, 0
00000001408ee39d  4c8d49f0                 lea        r9, [rcx - 0x10]
00000001408ee3a1  4531d2                   xor        r10d, r10d
00000001408ee3a4  660f76c9                 pcmpeqd    xmm1, xmm1
00000001408ee3a8  f3420f6f1401             movdqu     xmm2, xmmword ptr [rcx + r8]
00000001408ee3ae  660f6fda                 movdqa     xmm3, xmm2
00000001408ee3b2  660f74d8                 pcmpeqb    xmm3, xmm0
00000001408ee3b6  66440fd7db               pmovmskb   r11d, xmm3
00000001408ee3bb  4585db                   test       r11d, r11d
00000001408ee3be  742e                     je         0x1408ee3ee
00000001408ee3c0  f3410fbcfb               tzcnt      edi, r11d
00000001408ee3c5  4c01c7                   add        rdi, r8
00000001408ee3c8  4821d7                   and        rdi, rdx
00000001408ee3cb  4989fe                   mov        r14, rdi
00000001408ee3ce  49c1e604                 shl        r14, 4
00000001408ee3d2  4d89cf                   mov        r15, r9
00000001408ee3d5  4d29f7                   sub        r15, r14
00000001408ee3d8  493907                   cmp        qword ptr [r15], rax
00000001408ee3db  0f84d4020000             je         0x1408ee6b5
00000001408ee3e1  418d7bff                 lea        edi, [r11 - 1]
00000001408ee3e5  664421df                 and        di, r11w
00000001408ee3e9  4189fb                   mov        r11d, edi
00000001408ee3ec  75d2                     jne        0x1408ee3c0
00000001408ee3ee  660f74d1                 pcmpeqb    xmm2, xmm1
00000001408ee3f2  66440fd7da               pmovmskb   r11d, xmm2
00000001408ee3f7  4585db                   test       r11d, r11d
00000001408ee3fa  0f8507030000             jne        0x1408ee707
00000001408ee400  4d01d0                   add        r8, r10
00000001408ee403  4983c010                 add        r8, 0x10
00000001408ee407  4983c210                 add        r10, 0x10
00000001408ee40b  4921d0                   and        r8, rdx
00000001408ee40e  eb98                     jmp        0x1408ee3a8
00000001408ee410  49f7db                   neg        r11
00000001408ee413  49c1e304                 shl        r11, 4
00000001408ee417  428b4c18f8               mov        ecx, dword ptr [rax + r11 - 8]
00000001408ee41c  428b4418fc               mov        eax, dword ptr [rax + r11 - 4]
00000001408ee421  894c243c                 mov        dword ptr [rsp + 0x3c], ecx
00000001408ee425  85c9                     test       ecx, ecx
00000001408ee427  0f85ee020000             jne        0x1408ee71b
00000001408ee42d  89442438                 mov        dword ptr [rsp + 0x38], eax
00000001408ee431  488b0b                   mov        rcx, qword ptr [rbx]
00000001408ee434  448b31                   mov        r14d, dword ptr [rcx]
00000001408ee437  4883c104                 add        rcx, 4
00000001408ee43b  e850732e00               call       0x140bd5790
00000001408ee440  4c8b6e10                 mov        r13, qword ptr [rsi + 0x10]
00000001408ee444  4981fd00010000           cmp        r13, 0x100
00000001408ee44b  0f8376030000             jae        0x1408ee7c7
00000001408ee451  4889c3                   mov        rbx, rax
00000001408ee454  89e9                     mov        ecx, ebp
00000001408ee456  e8b5293200               call       0x140c10e10
00000001408ee45b  4c8d7e60                 lea        r15, [rsi + 0x60]
00000001408ee45f  89e9                     mov        ecx, ebp
00000001408ee461  84c0                     test       al, al
00000001408ee463  747f                     je         0x1408ee4e4
00000001408ee465  e816d774ff               call       0x14003bb80
00000001408ee46a  4c89f9                   mov        rcx, r15
00000001408ee46d  4489f2                   mov        edx, r14d
00000001408ee470  4189e8                   mov        r8d, ebp
00000001408ee473  4531c9                   xor        r9d, r9d
00000001408ee476  e815e60e00               call       0x1409dca90
00000001408ee47b  4489742434               mov        dword ptr [rsp + 0x34], r14d
00000001408ee480  410f100424               movups     xmm0, xmmword ptr [r12]
00000001408ee485  410f104c2410             movups     xmm1, xmmword ptr [r12 + 0x10]
00000001408ee48b  410f10542420             movups     xmm2, xmmword ptr [r12 + 0x20]
00000001408ee491  410f105c242c             movups     xmm3, xmmword ptr [r12 + 0x2c]
00000001408ee497  0f119c24bc000000         movups     xmmword ptr [rsp + 0xbc], xmm3
00000001408ee49f  0f299424b0000000         movaps     xmmword ptr [rsp + 0xb0], xmm2
00000001408ee4a7  0f298c24a0000000         movaps     xmmword ptr [rsp + 0xa0], xmm1
00000001408ee4af  0f29842490000000         movaps     xmmword ptr [rsp + 0x90], xmm0
00000001408ee4b7  488d4c2440               lea        rcx, [rsp + 0x40]
00000001408ee4bc  488d542434               lea        rdx, [rsp + 0x34]
00000001408ee4c1  4c8d842490000000         lea        r8, [rsp + 0x90]
00000001408ee4c9  e862872e00               call       0x140bd6c30
00000001408ee4ce  4c8b7610                 mov        r14, qword ptr [rsi + 0x10]
00000001408ee4d2  4c3b36                   cmp        r14, qword ptr [rsi]
00000001408ee4d5  0f858d000000             jne        0x1408ee568
00000001408ee4db  488d15c68c1b02           lea        rdx, [rip + 0x21b8cc6]
00000001408ee4e2  eb7c                     jmp        0x1408ee560
00000001408ee4e4  e897d674ff               call       0x14003bb80
00000001408ee4e9  4c89f9                   mov        rcx, r15
00000001408ee4ec  4489f2                   mov        edx, r14d
00000001408ee4ef  4189e8                   mov        r8d, ebp
00000001408ee4f2  41b901000000             mov        r9d, 1
00000001408ee4f8  e893e50e00               call       0x1409dca90
00000001408ee4fd  4489742434               mov        dword ptr [rsp + 0x34], r14d
00000001408ee502  410f100424               movups     xmm0, xmmword ptr [r12]
00000001408ee507  410f104c2410             movups     xmm1, xmmword ptr [r12 + 0x10]
00000001408ee50d  410f10542420             movups     xmm2, xmmword ptr [r12 + 0x20]
00000001408ee513  410f105c242c             movups     xmm3, xmmword ptr [r12 + 0x2c]
00000001408ee519  0f119c24bc000000         movups     xmmword ptr [rsp + 0xbc], xmm3
00000001408ee521  0f299424b0000000         movaps     xmmword ptr [rsp + 0xb0], xmm2
00000001408ee529  0f298c24a0000000         movaps     xmmword ptr [rsp + 0xa0], xmm1
00000001408ee531  0f29842490000000         movaps     xmmword ptr [rsp + 0x90], xmm0
00000001408ee539  488d4c2440               lea        rcx, [rsp + 0x40]
00000001408ee53e  488d542434               lea        rdx, [rsp + 0x34]
00000001408ee543  4c8d842490000000         lea        r8, [rsp + 0x90]
00000001408ee54b  e8b0862e00               call       0x140bd6c00
00000001408ee550  4c8b7610                 mov        r14, qword ptr [rsi + 0x10]
00000001408ee554  4c3b36                   cmp        r14, qword ptr [rsi]
00000001408ee557  750f                     jne        0x1408ee568
00000001408ee559  488d15308c1b02           lea        rdx, [rip + 0x21b8c30]
00000001408ee560  4889f1                   mov        rcx, rsi
00000001408ee563  e8e81b2700               call       0x140b60150
00000001408ee568  488b4608                 mov        rax, qword ptr [rsi + 8]
00000001408ee56c  4c89f1                   mov        rcx, r14
00000001408ee56f  48c1e106                 shl        rcx, 6
00000001408ee573  4a8d0cb1                 lea        rcx, [rcx + r14*4]
00000001408ee577  8b942480000000           mov        edx, dword ptr [rsp + 0x80]
00000001408ee57e  89540840                 mov        dword ptr [rax + rcx + 0x40], edx
00000001408ee582  0f10442440               movups     xmm0, xmmword ptr [rsp + 0x40]
00000001408ee587  f30f6f4c2450             movdqu     xmm1, xmmword ptr [rsp + 0x50]
00000001408ee58d  0f10542460               movups     xmm2, xmmword ptr [rsp + 0x60]
00000001408ee592  0f105c2470               movups     xmm3, xmmword ptr [rsp + 0x70]
00000001408ee597  0f115c0830               movups     xmmword ptr [rax + rcx + 0x30], xmm3
00000001408ee59c  0f11540820               movups     xmmword ptr [rax + rcx + 0x20], xmm2
00000001408ee5a1  f30f7f4c0810             movdqu     xmmword ptr [rax + rcx + 0x10], xmm1
00000001408ee5a7  0f110408                 movups     xmmword ptr [rax + rcx], xmm0
00000001408ee5ab  49ffc6                   inc        r14
00000001408ee5ae  4c897610                 mov        qword ptr [rsi + 0x10], r14
00000001408ee5b2  8b6c2438                 mov        ebp, dword ptr [rsp + 0x38]
00000001408ee5b6  4c8b7628                 mov        r14, qword ptr [rsi + 0x28]
00000001408ee5ba  4c3b7618                 cmp        r14, qword ptr [rsi + 0x18]
00000001408ee5be  7510                     jne        0x1408ee5d0
00000001408ee5c0  488d4e18                 lea        rcx, [rsi + 0x18]
00000001408ee5c4  488d15f58b1b02           lea        rdx, [rip + 0x21b8bf5]
00000001408ee5cb  e8a048c7ff               call       0x140562e70
00000001408ee5d0  488b4620                 mov        rax, qword ptr [rsi + 0x20]
00000001408ee5d4  42892cf0                 mov        dword ptr [rax + r14*8], ebp
00000001408ee5d8  46886cf004               mov        byte ptr [rax + r14*8 + 4], r13b
00000001408ee5dd  49ffc6                   inc        r14
00000001408ee5e0  4c897628                 mov        qword ptr [rsi + 0x28], r14
00000001408ee5e4  4c8bb638010000           mov        r14, qword ptr [rsi + 0x138]
00000001408ee5eb  4889f9                   mov        rcx, rdi
00000001408ee5ee  e85d903900               call       0x140c87650
00000001408ee5f3  80be3601000008           cmp        byte ptr [rsi + 0x136], 8
00000001408ee5fa  0f8407010000             je         0x1408ee707
00000001408ee600  4983bee800000000         cmp        qword ptr [r14 + 0xe8], 0
00000001408ee608  0f84f9000000             je         0x1408ee707
00000001408ee60e  4983be3001000000         cmp        qword ptr [r14 + 0x130], 0
00000001408ee616  0f84eb000000             je         0x1408ee707
00000001408ee61c  4989c1                   mov        r9, rax
00000001408ee61f  49c1e939                 shr        r9, 0x39
00000001408ee623  498b8e18010000           mov        rcx, qword ptr [r14 + 0x118]
00000001408ee62a  498b9620010000           mov        rdx, qword ptr [r14 + 0x120]
00000001408ee631  4989d0                   mov        r8, rdx
00000001408ee634  4921c0                   and        r8, rax
00000001408ee637  66410f6ec1               movd       xmm0, r9d
00000001408ee63c  660f60c0                 punpcklbw  xmm0, xmm0
00000001408ee640  f20f70c000               pshuflw    xmm0, xmm0, 0
00000001408ee645  660f70c000               pshufd     xmm0, xmm0, 0
00000001408ee64a  4c8d49f0                 lea        r9, [rcx - 0x10]
00000001408ee64e  4531d2                   xor        r10d, r10d
00000001408ee651  660f76c9                 pcmpeqd    xmm1, xmm1
00000001408ee655  f3420f6f1401             movdqu     xmm2, xmmword ptr [rcx + r8]
00000001408ee65b  660f6fda                 movdqa     xmm3, xmm2
00000001408ee65f  660f74d8                 pcmpeqb    xmm3, xmm0
00000001408ee663  66440fd7db               pmovmskb   r11d, xmm3
00000001408ee668  4585db                   test       r11d, r11d
00000001408ee66b  742a                     je         0x1408ee697
00000001408ee66d  f3410fbcfb               tzcnt      edi, r11d
00000001408ee672  4c01c7                   add        rdi, r8
00000001408ee675  4821d7                   and        rdi, rdx
00000001408ee678  4989fe                   mov        r14, rdi
00000001408ee67b  49c1e604                 shl        r14, 4
00000001408ee67f  4d89cf                   mov        r15, r9
00000001408ee682  4d29f7                   sub        r15, r14
00000001408ee685  493907                   cmp        qword ptr [r15], rax
00000001408ee688  745b                     je         0x1408ee6e5
00000001408ee68a  418d7bff                 lea        edi, [r11 - 1]
00000001408ee68e  664421df                 and        di, r11w
00000001408ee692  4189fb                   mov        r11d, edi
00000001408ee695  75d6                     jne        0x1408ee66d
00000001408ee697  660f74d1                 pcmpeqb    xmm2, xmm1
00000001408ee69b  66440fd7da               pmovmskb   r11d, xmm2
00000001408ee6a0  4585db                   test       r11d, r11d
00000001408ee6a3  7562                     jne        0x1408ee707
00000001408ee6a5  4d01d0                   add        r8, r10
00000001408ee6a8  4983c010                 add        r8, 0x10
00000001408ee6ac  4983c210                 add        r10, 0x10
00000001408ee6b0  4921d0                   and        r8, rdx
00000001408ee6b3  eba0                     jmp        0x1408ee655
00000001408ee6b5  48f7df                   neg        rdi
00000001408ee6b8  48c1e704                 shl        rdi, 4
00000001408ee6bc  8b7c39f8                 mov        edi, dword ptr [rcx + rdi - 8]
00000001408ee6c0  488b0b                   mov        rcx, qword ptr [rbx]
00000001408ee6c3  4883c104                 add        rcx, 4
00000001408ee6c7  e8c4702e00               call       0x140bd5790
00000001408ee6cc  488b9618010000           mov        rdx, qword ptr [rsi + 0x118]
00000001408ee6d3  4839fa                   cmp        rdx, rdi
00000001408ee6d6  7270                     jb         0x1408ee748
00000001408ee6d8  488b8e10010000           mov        rcx, qword ptr [rsi + 0x110]
00000001408ee6df  48890439                 mov        qword ptr [rcx + rdi], rax
00000001408ee6e3  eb22                     jmp        0x1408ee707
00000001408ee6e5  48f7df                   neg        rdi
00000001408ee6e8  48c1e704                 shl        rdi, 4
00000001408ee6ec  8b4c39f8                 mov        ecx, dword ptr [rcx + rdi - 8]
00000001408ee6f0  488b9618010000           mov        rdx, qword ptr [rsi + 0x118]
00000001408ee6f7  4839ca                   cmp        rdx, rcx
00000001408ee6fa  725b                     jb         0x1408ee757
00000001408ee6fc  488b8610010000           mov        rax, qword ptr [rsi + 0x110]
00000001408ee703  48891c08                 mov        qword ptr [rax + rcx], rbx
00000001408ee707  4881c4d8000000           add        rsp, 0xd8
00000001408ee70e  5b                       pop        rbx
00000001408ee70f  5d                       pop        rbp
00000001408ee710  5f                       pop        rdi
00000001408ee711  5e                       pop        rsi
00000001408ee712  415c                     pop        r12
00000001408ee714  415d                     pop        r13
00000001408ee716  415e                     pop        r14
00000001408ee718  415f                     pop        r15
00000001408ee71a  c3                       ret        
00000001408ee71b  48c744244000000000       mov        qword ptr [rsp + 0x40], 0
00000001408ee724  488d054d8a1b02           lea        rax, [rip + 0x21b8a4d]
00000001408ee72b  4889442420               mov        qword ptr [rsp + 0x20], rax
00000001408ee730  4c8d05f9891b02           lea        r8, [rip + 0x21b89f9]
00000001408ee737  488d54243c               lea        rdx, [rsp + 0x3c]
00000001408ee73c  4c8d4c2440               lea        r9, [rsp + 0x40]
00000001408ee741  31c9                     xor        ecx, ecx
00000001408ee743  e807b2fb01               call       0x1428a994f
00000001408ee748  4c8d0571741c02           lea        r8, [rip + 0x21c7471]
00000001408ee74f  4889f9                   mov        rcx, rdi
00000001408ee752  e8d9b0fe01               call       0x1428d9830
00000001408ee757  4c8d0562741c02           lea        r8, [rip + 0x21c7462]
00000001408ee75e  e8cdb0fe01               call       0x1428d9830
00000001408ee763  488d842488000000         lea        rax, [rsp + 0x88]
00000001408ee76b  4889842490000000         mov        qword ptr [rsp + 0x90], rax
00000001408ee773  488d0546e6ffff           lea        rax, [rip - 0x19ba]
00000001408ee77a  4889842498000000         mov        qword ptr [rsp + 0x98], rax
00000001408ee782  488d05878a1b02           lea        rax, [rip + 0x21b8a87]
00000001408ee789  4889442440               mov        qword ptr [rsp + 0x40], rax
00000001408ee78e  48c744244802000000       mov        qword ptr [rsp + 0x48], 2
00000001408ee797  48c744246000000000       mov        qword ptr [rsp + 0x60], 0
00000001408ee7a0  488d842490000000         lea        rax, [rsp + 0x90]
00000001408ee7a8  4889442450               mov        qword ptr [rsp + 0x50], rax
00000001408ee7ad  48c744245801000000       mov        qword ptr [rsp + 0x58], 1
00000001408ee7b6  488d15738a1b02           lea        rdx, [rip + 0x21b8a73]
00000001408ee7bd  488d4c2440               lea        rcx, [rsp + 0x40]
00000001408ee7c2  e8c9abfe01               call       0x1428d9390
00000001408ee7c7  488d054a891b02           lea        rax, [rip + 0x21b894a]
00000001408ee7ce  4889442420               mov        qword ptr [rsp + 0x20], rax
00000001408ee7d3  488d0d56821b02           lea        rcx, [rip + 0x21b8256]
00000001408ee7da  4c8d0dbf821b02           lea        r9, [rip + 0x21b82bf]
00000001408ee7e1  4c8d442440               lea        r8, [rsp + 0x40]
00000001408ee7e6  ba2b000000               mov        edx, 0x2b
00000001408ee7eb  e870affe01               call       0x1428d9760
00000001408ee7f0  cc                       int3       
