00000001408383a0: 55                       push       rbp
00000001408383a1: 4157                     push       r15
00000001408383a3: 4156                     push       r14
00000001408383a5: 4155                     push       r13
00000001408383a7: 4154                     push       r12
00000001408383a9: 56                       push       rsi
00000001408383aa: 57                       push       rdi
00000001408383ab: 53                       push       rbx
00000001408383ac: 4881ec58010000           sub        rsp, 0x158
00000001408383b3: 488dac2480000000         lea        rbp, [rsp + 0x80]
00000001408383bb: 48c785d0000000feffffff   mov        qword ptr [rbp + 0xd0], 0xfffffffffffffffe
00000001408383c6: 4c898da8000000           mov        qword ptr [rbp + 0xa8], r9
00000001408383cd: 4c8985b8000000           mov        qword ptr [rbp + 0xb8], r8
00000001408383d4: 4889d7                   mov        rdi, rdx
00000001408383d7: 4889ce                   mov        rsi, rcx
00000001408383da: e821e63d00               call       0x140c16a00
00000001408383df: 84c0                     test       al, al
00000001408383e1: 745f                     je         0x140838442
00000001408383e3: 4c8d25c6a02502           lea        r12, [rip + 0x225a0c6]
00000001408383ea: bb41000000               mov        ebx, 0x41
00000001408383ef: ba41000000               mov        edx, 0x41
00000001408383f4: 4c89e1                   mov        rcx, r12
00000001408383f7: e824a5ffff               call       0x140832920
00000001408383fc: 4883f801                 cmp        rax, 1
0000000140838400: 0f85b1000000             jne        0x1408384b7
0000000140838406: 4989d1                   mov        r9, rdx
0000000140838409: 4885d2                   test       rdx, rdx
000000014083840c: 743d                     je         0x14083844b
000000014083840e: 4983f941                 cmp        r9, 0x41
0000000140838412: 7335                     jae        0x140838449
0000000140838414: 488d0595a02502           lea        rax, [rip + 0x225a095]
000000014083841b: 41803c01bf               cmp        byte ptr [r9 + rax], 0xbf
0000000140838420: 7f29                     jg         0x14083844b
0000000140838422: 488d05dfa12502           lea        rax, [rip + 0x225a1df]
0000000140838429: 4889442420               mov        qword ptr [rsp + 0x20], rax
000000014083842e: 488d0d7ba02502           lea        rcx, [rip + 0x225a07b]
0000000140838435: ba41000000               mov        edx, 0x41
000000014083843a: 4531c0                   xor        r8d, r8d
000000014083843d: e8fe140a02               call       0x1428d9940
0000000140838442: 31c0                     xor        eax, eax
0000000140838444: e9da010000               jmp        0x140838623
0000000140838449: 75d7                     jne        0x140838422
000000014083844b: 4c8d255ea02502           lea        r12, [rip + 0x225a05e]
0000000140838452: 4c89e1                   mov        rcx, r12
0000000140838455: 4c89ca                   mov        rdx, r9
0000000140838458: e8c3a4ffff               call       0x140832920
000000014083845d: 4883f801                 cmp        rax, 1
0000000140838461: 7554                     jne        0x1408384b7
0000000140838463: 4989d0                   mov        r8, rdx
0000000140838466: 4983c002                 add        r8, 2
000000014083846a: 7439                     je         0x1408384a5
000000014083846c: 4983f841                 cmp        r8, 0x41
0000000140838470: 7331                     jae        0x1408384a3
0000000140838472: 488d0537a02502           lea        rax, [rip + 0x225a037]
0000000140838479: 41803c00bf               cmp        byte ptr [r8 + rax], 0xbf
000000014083847e: 7f25                     jg         0x1408384a5
0000000140838480: 488d0599a12502           lea        rax, [rip + 0x225a199]
0000000140838487: 4889442420               mov        qword ptr [rsp + 0x20], rax
000000014083848c: 488d0d1da02502           lea        rcx, [rip + 0x225a01d]
0000000140838493: ba41000000               mov        edx, 0x41
0000000140838498: 41b941000000             mov        r9d, 0x41
000000014083849e: e89d140a02               call       0x1428d9940
00000001408384a3: 75db                     jne        0x140838480
00000001408384a5: bb3f000000               mov        ebx, 0x3f
00000001408384aa: 4829d3                   sub        rbx, rdx
00000001408384ad: 4c8d25fc9f2502           lea        r12, [rip + 0x2259ffc]
00000001408384b4: 4d01c4                   add        r12, r8
00000001408384b7: 488d055ca72502           lea        rax, [rip + 0x225a75c]
00000001408384be: 4c8d2d1da72502           lea        r13, [rip + 0x225a71d]
00000001408384c5: 66662e0f1f840000000000   nop        word ptr cs:[rax + rax]
00000001408384d0: 0fbe48ff                 movsx      ecx, byte ptr [rax - 1]
00000001408384d4: 85c9                     test       ecx, ecx
00000001408384d6: 7818                     js         0x1408384f0
00000001408384d8: 48ffc8                   dec        rax
00000001408384db: 83f95c                   cmp        ecx, 0x5c
00000001408384de: 7569                     jne        0x140838549
00000001408384e0: eb7f                     jmp        0x140838561
00000001408384e2: 66666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
00000001408384f0: 0fb650fe                 movzx      edx, byte ptr [rax - 2]
00000001408384f4: 80fac0                   cmp        dl, 0xc0
00000001408384f7: 7d25                     jge        0x14083851e
00000001408384f9: 440fb640fd               movzx      r8d, byte ptr [rax - 3]
00000001408384fe: 4180f8c0                 cmp        r8b, 0xc0
0000000140838502: 7d23                     jge        0x140838527
0000000140838504: 440fb648fc               movzx      r9d, byte ptr [rax - 4]
0000000140838509: 4883c0fc                 add        rax, -4
000000014083850d: 4183e107                 and        r9d, 7
0000000140838511: 41c1e106                 shl        r9d, 6
0000000140838515: 4183e03f                 and        r8d, 0x3f
0000000140838519: 4509c8                   or         r8d, r9d
000000014083851c: eb11                     jmp        0x14083852f
000000014083851e: 4883c0fe                 add        rax, -2
0000000140838522: 83e21f                   and        edx, 0x1f
0000000140838525: eb12                     jmp        0x140838539
0000000140838527: 4883c0fd                 add        rax, -3
000000014083852b: 4183e00f                 and        r8d, 0xf
000000014083852f: 41c1e006                 shl        r8d, 6
0000000140838533: 83e23f                   and        edx, 0x3f
0000000140838536: 4409c2                   or         edx, r8d
0000000140838539: c1e206                   shl        edx, 6
000000014083853c: 80e13f                   and        cl, 0x3f
000000014083853f: 0fb6c9                   movzx      ecx, cl
0000000140838542: 09d1                     or         ecx, edx
0000000140838544: 83f95c                   cmp        ecx, 0x5c
0000000140838547: 7418                     je         0x140838561
0000000140838549: 83f92f                   cmp        ecx, 0x2f
000000014083854c: 7413                     je         0x140838561
000000014083854e: 4c39e8                   cmp        rax, r13
0000000140838551: 0f8579ffffff             jne        0x1408384d0
0000000140838557: 4989fe                   mov        r14, rdi
000000014083855a: bf38000000               mov        edi, 0x38
000000014083855f: eb52                     jmp        0x1408385b3
0000000140838561: 4c8d2d7aa62502           lea        r13, [rip + 0x225a67a]
0000000140838568: 4c29e8                   sub        rax, r13
000000014083856b: 4989c0                   mov        r8, rax
000000014083856e: 49ffc0                   inc        r8
0000000140838571: 7432                     je         0x1408385a5
0000000140838573: 4983f838                 cmp        r8, 0x38
0000000140838577: 732a                     jae        0x1408385a3
0000000140838579: 43803c28bf               cmp        byte ptr [r8 + r13], 0xbf
000000014083857e: 7f25                     jg         0x1408385a5
0000000140838580: 488d0561a02502           lea        rax, [rip + 0x225a061]
0000000140838587: 4889442420               mov        qword ptr [rsp + 0x20], rax
000000014083858c: 488d0d4fa62502           lea        rcx, [rip + 0x225a64f]
0000000140838593: ba38000000               mov        edx, 0x38
0000000140838598: 41b938000000             mov        r9d, 0x38
000000014083859e: e89d130a02               call       0x1428d9940
00000001408385a3: 75db                     jne        0x140838580
00000001408385a5: 4989fe                   mov        r14, rdi
00000001408385a8: bf37000000               mov        edi, 0x37
00000001408385ad: 4829c7                   sub        rdi, rax
00000001408385b0: 4d01c5                   add        r13, r8
00000001408385b3: e878fb3d00               call       0x140c18130
00000001408385b8: 4889c1                   mov        rcx, rax
00000001408385bb: 488b00                   mov        rax, qword ptr [rax]
00000001408385be: 4883f801                 cmp        rax, 1
00000001408385c2: 0f85c2070000             jne        0x140838d8a
00000001408385c8: 4883c108                 add        rcx, 8
00000001408385cc: 48833900                 cmp        qword ptr [rcx], 0
00000001408385d0: 0f85b0090000             jne        0x140838f86
00000001408385d6: 48c701ffffffff           mov        qword ptr [rcx], 0xffffffffffffffff
00000001408385dd: 48898dc0000000           mov        qword ptr [rbp + 0xc0], rcx
00000001408385e4: 4883c108                 add        rcx, 8
00000001408385e8: 48897c2420               mov        qword ptr [rsp + 0x20], rdi
00000001408385ed: 48c744243000000000       mov        qword ptr [rsp + 0x30], 0
00000001408385f6: 48c744242801000000       mov        qword ptr [rsp + 0x28], 1
00000001408385ff: 4c89e2                   mov        rdx, r12
0000000140838602: 4989d8                   mov        r8, rbx
0000000140838605: 4d89e9                   mov        r9, r13
0000000140838608: e833e73d00               call       0x140c16d40
000000014083860d: 488b8dc0000000           mov        rcx, qword ptr [rbp + 0xc0]
0000000140838614: 48ff01                   inc        qword ptr [rcx]
0000000140838617: 48894540                 mov        qword ptr [rbp + 0x40], rax
000000014083861b: b801000000               mov        eax, 1
0000000140838620: 4c89f7                   mov        rdi, r14
0000000140838623: 488b8d48010000           mov        rcx, qword ptr [rbp + 0x148]
000000014083862a: 48894538                 mov        qword ptr [rbp + 0x38], rax
000000014083862e: 8b461c                   mov        eax, dword ptr [rsi + 0x1c]
0000000140838631: 488b5610                 mov        rdx, qword ptr [rsi + 0x10]
0000000140838635: 8902                     mov        dword ptr [rdx], eax
0000000140838637: 488b36                   mov        rsi, qword ptr [rsi]
000000014083863a: 488b01                   mov        rax, qword ptr [rcx]
000000014083863d: 8b08                     mov        ecx, dword ptr [rax]
000000014083863f: f3480f2ac1               cvtsi2ss   xmm0, rcx
0000000140838644: 8b4004                   mov        eax, dword ptr [rax + 4]
0000000140838647: f3480f2ad8               cvtsi2ss   xmm3, rax
000000014083864c: 48897d20                 mov        qword ptr [rbp + 0x20], rdi
0000000140838650: 4c8b0f                   mov        r9, qword ptr [rdi]
0000000140838653: 488b85a8000000           mov        rax, qword ptr [rbp + 0xa8]
000000014083865a: 488b00                   mov        rax, qword ptr [rax]
000000014083865d: f30f1008                 movss      xmm1, dword ptr [rax]
0000000140838661: f30f105004               movss      xmm2, dword ptr [rax + 4]
0000000140838666: f30f115c2428             movss      dword ptr [rsp + 0x28], xmm3
000000014083866c: f30f11442420             movss      dword ptr [rsp + 0x20], xmm0
0000000140838672: 488d7dd0                 lea        rdi, [rbp - 0x30]
0000000140838676: 4889f9                   mov        rcx, rdi
0000000140838679: e83244a400               call       0x14127cab0
000000014083867e: 488b8d50010000           mov        rcx, qword ptr [rbp + 0x150]
0000000140838685: 488d45dc                 lea        rax, [rbp - 0x24]
0000000140838689: 48897dc0                 mov        qword ptr [rbp - 0x40], rdi
000000014083868d: 488945c8                 mov        qword ptr [rbp - 0x38], rax
0000000140838691: 48c7411000000000         mov        qword ptr [rcx + 0x10], 0
0000000140838699: 488d7e60                 lea        rdi, [rsi + 0x60]
000000014083869d: 488b4660                 mov        rax, qword ptr [rsi + 0x60]
00000001408386a1: 89c1                     mov        ecx, eax
00000001408386a3: 83e108                   and        ecx, 8
00000001408386a6: c1e903                   shr        ecx, 3
00000001408386a9: 4883f8f0                 cmp        rax, -0x10
00000001408386ad: 0f93c2                   setae      dl
00000001408386b0: 08ca                     or         dl, cl
00000001408386b2: 7412                     je         0x1408386c6
00000001408386b4: 4889f9                   mov        rcx, rdi
00000001408386b7: 31d2                     xor        edx, edx
00000001408386b9: 41b900ca9a3b             mov        r9d, 0x3b9aca00
00000001408386bf: e8ec3d0902               call       0x1428cc4b0
00000001408386c4: eb0b                     jmp        0x1408386d1
00000001408386c6: 488d4810                 lea        rcx, [rax + 0x10]
00000001408386ca: f0480fb10f               lock cmpxchg qword ptr [rdi], rcx
00000001408386cf: 75e3                     jne        0x1408386b4
00000001408386d1: 4889f9                   mov        rcx, rdi
00000001408386d4: e897954400               call       0x140c81c70
00000001408386d9: 4889f9                   mov        rcx, rdi
00000001408386dc: 4883c901                 or         rcx, 1
00000001408386e0: 48898d80000000           mov        qword ptr [rbp + 0x80], rcx
00000001408386e7: e884954400               call       0x140c81c70
00000001408386ec: 4889b5c0000000           mov        qword ptr [rbp + 0xc0], rsi
00000001408386f3: 488bb680000000           mov        rsi, qword ptr [rsi + 0x80]
00000001408386fa: 4885f6                   test       rsi, rsi
00000001408386fd: 0f8407010000             je         0x14083880a
0000000140838703: 488b85c0000000           mov        rax, qword ptr [rbp + 0xc0]
000000014083870a: 488b5868                 mov        rbx, qword ptr [rax + 0x68]
000000014083870e: 660f6f03                 movdqa     xmm0, xmmword ptr [rbx]
0000000140838712: 4c8d7310                 lea        r14, [rbx + 0x10]
0000000140838716: 660fd7c0                 pmovmskb   eax, xmm0
000000014083871a: f7d0                     not        eax
000000014083871c: 4889bdb0000000           mov        qword ptr [rbp + 0xb0], rdi
0000000140838723: eb40                     jmp        0x140838765
0000000140838725: 66662e0f1f840000000000   nop        word ptr cs:[rax + rax]
0000000140838730: 4983c410                 add        r12, 0x10
0000000140838734: 4889ca                   mov        rdx, rcx
0000000140838737: 488b4108                 mov        rax, qword ptr [rcx + 8]
000000014083873b: 4889f9                   mov        rcx, rdi
000000014083873e: 48c1e104                 shl        rcx, 4
0000000140838742: 4c893c08                 mov        qword ptr [rax + rcx], r15
0000000140838746: 4c89640808               mov        qword ptr [rax + rcx + 8], r12
000000014083874b: 48ffc7                   inc        rdi
000000014083874e: 48897a10                 mov        qword ptr [rdx + 0x10], rdi
0000000140838752: 4489e8                   mov        eax, r13d
0000000140838755: 4885f6                   test       rsi, rsi
0000000140838758: 488bbdb0000000           mov        rdi, qword ptr [rbp + 0xb0]
000000014083875f: 0f84a5000000             je         0x14083880a
0000000140838765: 6685c0                   test       ax, ax
0000000140838768: 7416                     je         0x140838780
000000014083876a: 448d68ff                 lea        r13d, [rax - 1]
000000014083876e: 4121c5                   and        r13d, eax
0000000140838771: eb37                     jmp        0x1408387aa
0000000140838773: 666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140838780: 66410f6f06               movdqa     xmm0, xmmword ptr [r14]
0000000140838785: 660fd7c0                 pmovmskb   eax, xmm0
0000000140838789: 4881c300ffffff           add        rbx, -0x100
0000000140838790: 4983c610                 add        r14, 0x10
0000000140838794: 3dffff0000               cmp        eax, 0xffff
0000000140838799: 74e5                     je         0x140838780
000000014083879b: b9feffffff               mov        ecx, 0xfffffffe
00000001408387a0: 29c1                     sub        ecx, eax
00000001408387a2: f7d0                     not        eax
00000001408387a4: 4189c5                   mov        r13d, eax
00000001408387a7: 4121cd                   and        r13d, ecx
00000001408387aa: 48ffce                   dec        rsi
00000001408387ad: f30fbcc0                 tzcnt      eax, eax
00000001408387b1: c1e004                   shl        eax, 4
00000001408387b4: 4889d9                   mov        rcx, rbx
00000001408387b7: 4829c1                   sub        rcx, rax
00000001408387ba: 4c8b79f0                 mov        r15, qword ptr [rcx - 0x10]
00000001408387be: 4c8b61f8                 mov        r12, qword ptr [rcx - 8]
00000001408387c2: 488b8d50010000           mov        rcx, qword ptr [rbp + 0x150]
00000001408387c9: 488b7910                 mov        rdi, qword ptr [rcx + 0x10]
00000001408387cd: 483b39                   cmp        rdi, qword ptr [rcx]
00000001408387d0: 0f855affffff             jne        0x140838730
00000001408387d6: 4989f0                   mov        r8, rsi
00000001408387d9: 49ffc0                   inc        r8
00000001408387dc: 48c7c0ffffffff           mov        rax, 0xffffffffffffffff
00000001408387e3: 4c0f44c0                 cmove      r8, rax
00000001408387e7: 48c744242010000000       mov        qword ptr [rsp + 0x20], 0x10
00000001408387f0: 41b908000000             mov        r9d, 8
00000001408387f6: 4889fa                   mov        rdx, rdi
00000001408387f9: e862120702               call       0x1428a9a60
00000001408387fe: 488b8d50010000           mov        rcx, qword ptr [rbp + 0x150]
0000000140838805: e926ffffff               jmp        0x140838730
000000014083880a: 4889f9                   mov        rcx, rdi
000000014083880d: e8fe954400               call       0x140c81e10
0000000140838812: 488b9dc0000000           mov        rbx, qword ptr [rbp + 0xc0]
0000000140838819: 4c8d75d0                 lea        r14, [rbp - 0x30]
000000014083881d: 488b8d80000000           mov        rcx, qword ptr [rbp + 0x80]
0000000140838824: e8e7954400               call       0x140c81e10
0000000140838829: 48c7c0f0ffffff           mov        rax, 0xfffffffffffffff0
0000000140838830: f0480fc107               lock xadd  qword ptr [rdi], rax
0000000140838835: 4883e0f2                 and        rax, 0xfffffffffffffff2
0000000140838839: 4883f812                 cmp        rax, 0x12
000000014083883d: 0f8468050000             je         0x140838dab
0000000140838843: e8e8ce5000               call       0x140d45730
0000000140838848: 4989c5                   mov        r13, rax
000000014083884b: 488b4810                 mov        rcx, qword ptr [rax + 0x10]
000000014083884f: 4885c9                   test       rcx, rcx
0000000140838852: 0f8448070000             je         0x140838fa0
0000000140838858: 488b8550010000           mov        rax, qword ptr [rbp + 0x150]
000000014083885f: 488b7008                 mov        rsi, qword ptr [rax + 8]
0000000140838863: 488b7810                 mov        rdi, qword ptr [rax + 0x10]
0000000140838867: 4889f8                   mov        rax, rdi
000000014083886a: 4809c8                   or         rax, rcx
000000014083886d: 48c1e820                 shr        rax, 0x20
0000000140838871: 740a                     je         0x14083887d
0000000140838873: 4889f8                   mov        rax, rdi
0000000140838876: 31d2                     xor        edx, edx
0000000140838878: 48f7f1                   div        rcx
000000014083887b: eb06                     jmp        0x140838883
000000014083887d: 89f8                     mov        eax, edi
000000014083887f: 31d2                     xor        edx, edx
0000000140838881: f7f1                     div        ecx
0000000140838883: 31c9                     xor        ecx, ecx
0000000140838885: 4883ffff                 cmp        rdi, -1
0000000140838889: 0f94c1                   sete       cl
000000014083888c: 4839c1                   cmp        rcx, rax
000000014083888f: 480f47c1                 cmova      rax, rcx
0000000140838893: 4883f801                 cmp        rax, 1
0000000140838897: 4883d000                 adc        rax, 0
000000014083889b: 4c897560                 mov        qword ptr [rbp + 0x60], r14
000000014083889f: 488d4dc0                 lea        rcx, [rbp - 0x40]
00000001408388a3: 48894d68                 mov        qword ptr [rbp + 0x68], rcx
00000001408388a7: 488b8da8000000           mov        rcx, qword ptr [rbp + 0xa8]
00000001408388ae: 48894d70                 mov        qword ptr [rbp + 0x70], rcx
00000001408388b2: 488b8db8000000           mov        rcx, qword ptr [rbp + 0xb8]
00000001408388b9: 48894d78                 mov        qword ptr [rbp + 0x78], rcx
00000001408388bd: 48894508                 mov        qword ptr [rbp + 8], rax
00000001408388c1: 488d4560                 lea        rax, [rbp + 0x60]
00000001408388c5: 48894500                 mov        qword ptr [rbp], rax
00000001408388c9: 31c9                     xor        ecx, ecx
00000001408388cb: e8702f0e00               call       0x14091b840
00000001408388d0: 4885c0                   test       rax, rax
00000001408388d3: 0f84b9060000             je         0x140838f92
00000001408388d9: 4889b588000000           mov        qword ptr [rbp + 0x88], rsi
00000001408388e0: 4889bd90000000           mov        qword ptr [rbp + 0x90], rdi
00000001408388e7: 488d4d08                 lea        rcx, [rbp + 8]
00000001408388eb: 48898d98000000           mov        qword ptr [rbp + 0x98], rcx
00000001408388f2: 4889e9                   mov        rcx, rbp
00000001408388f5: 48898da0000000           mov        qword ptr [rbp + 0xa0], rcx
00000001408388fc: 4c8b08                   mov        r9, qword ptr [rax]
00000001408388ff: 4983c110                 add        r9, 0x10
0000000140838903: 488d8588000000           lea        rax, [rbp + 0x88]
000000014083890a: 4889442428               mov        qword ptr [rsp + 0x28], rax
000000014083890f: 4c894c2420               mov        qword ptr [rsp + 0x20], r9
0000000140838914: 488d4de8                 lea        rcx, [rbp - 0x18]
0000000140838918: 4c89ea                   mov        rdx, r13
000000014083891b: 41b001                   mov        r8b, 1
000000014083891e: e8dd6b0700               call       0x1408af500
0000000140838923: 488b55e8                 mov        rdx, qword ptr [rbp - 0x18]
0000000140838927: 4889d0                   mov        rax, rdx
000000014083892a: 48f7d8                   neg        rax
000000014083892d: 0f805f060000             jo         0x140838f92
0000000140838933: 4c8b45f0                 mov        r8, qword ptr [rbp - 0x10]
0000000140838937: 488b45f8                 mov        rax, qword ptr [rbp - 8]
000000014083893b: 488b8d50010000           mov        rcx, qword ptr [rbp + 0x150]
0000000140838942: 48c7411000000000         mov        qword ptr [rcx + 0x10], 0
000000014083894a: 48c7431000000000         mov        qword ptr [rbx + 0x10], 0
0000000140838952: 48c7456000000000         mov        qword ptr [rbp + 0x60], 0
000000014083895a: 48c7456808000000         mov        qword ptr [rbp + 0x68], 8
0000000140838962: 48c7457000000000         mov        qword ptr [rbp + 0x70], 0
000000014083896a: 488d0c40                 lea        rcx, [rax + rax*2]
000000014083896e: 48c1e104                 shl        rcx, 4
0000000140838972: 4c01c1                   add        rcx, r8
0000000140838975: 4c898588000000           mov        qword ptr [rbp + 0x88], r8
000000014083897c: 4c898590000000           mov        qword ptr [rbp + 0x90], r8
0000000140838983: 48895528                 mov        qword ptr [rbp + 0x28], rdx
0000000140838987: 48899598000000           mov        qword ptr [rbp + 0x98], rdx
000000014083898e: 48894d58                 mov        qword ptr [rbp + 0x58], rcx
0000000140838992: 48898da0000000           mov        qword ptr [rbp + 0xa0], rcx
0000000140838999: 4885c0                   test       rax, rax
000000014083899c: 4c894530                 mov        qword ptr [rbp + 0x30], r8
00000001408389a0: 0f8499010000             je         0x140838b3f
00000001408389a6: 4d8d6030                 lea        r12, [r8 + 0x30]
00000001408389aa: b808000000               mov        eax, 8
00000001408389af: 48894548                 mov        qword ptr [rbp + 0x48], rax
00000001408389b3: 4531f6                   xor        r14d, r14d
00000001408389b6: eb21                     jmp        0x1408389d9
00000001408389b8: 0f1f840000000000         nop        dword ptr [rax + rax]
00000001408389c0: 4c8ba5a8000000           mov        r12, qword ptr [rbp + 0xa8]
00000001408389c7: 4983c430                 add        r12, 0x30
00000001408389cb: 4883c630                 add        rsi, 0x30
00000001408389cf: 483b7558                 cmp        rsi, qword ptr [rbp + 0x58]
00000001408389d3: 0f846e010000             je         0x140838b47
00000001408389d9: 498b4c24d0               mov        rcx, qword ptr [r12 - 0x30]
00000001408389de: 4889c8                   mov        rax, rcx
00000001408389e1: 48f7d8                   neg        rax
00000001408389e4: 0f8061010000             jo         0x140838b4b
00000001408389ea: 48894d50                 mov        qword ptr [rbp + 0x50], rcx
00000001408389ee: 4c89a5a8000000           mov        qword ptr [rbp + 0xa8], r12
00000001408389f5: 498d7424d0               lea        rsi, [r12 - 0x30]
00000001408389fa: 488b4608                 mov        rax, qword ptr [rsi + 8]
00000001408389fe: 48898580000000           mov        qword ptr [rbp + 0x80], rax
0000000140838a05: 488b7e10                 mov        rdi, qword ptr [rsi + 0x10]
0000000140838a09: 488b4618                 mov        rax, qword ptr [rsi + 0x18]
0000000140838a0d: 488985b8000000           mov        qword ptr [rbp + 0xb8], rax
0000000140838a14: 488b4620                 mov        rax, qword ptr [rsi + 0x20]
0000000140838a18: 488985b0000000           mov        qword ptr [rbp + 0xb0], rax
0000000140838a1f: 4c8b6628                 mov        r12, qword ptr [rsi + 0x28]
0000000140838a23: 488b8dc0000000           mov        rcx, qword ptr [rbp + 0xc0]
0000000140838a2a: 488b01                   mov        rax, qword ptr [rcx]
0000000140838a2d: 4c8b7910                 mov        r15, qword ptr [rcx + 0x10]
0000000140838a31: 4c29f8                   sub        rax, r15
0000000140838a34: 4839f8                   cmp        rax, rdi
0000000140838a37: 0f829f000000             jb         0x140838adc
0000000140838a3d: 4c6bc738                 imul       r8, rdi, 0x38
0000000140838a41: 496bcf38                 imul       rcx, r15, 0x38
0000000140838a45: 488b9dc0000000           mov        rbx, qword ptr [rbp + 0xc0]
0000000140838a4c: 48034b08                 add        rcx, qword ptr [rbx + 8]
0000000140838a50: 4c8bad80000000           mov        r13, qword ptr [rbp + 0x80]
0000000140838a57: 4c89ea                   mov        rdx, r13
0000000140838a5a: e8a1b10e02               call       0x142923c00
0000000140838a5f: 4901ff                   add        r15, rdi
0000000140838a62: 4c897b10                 mov        qword ptr [rbx + 0x10], r15
0000000140838a66: 488b4550                 mov        rax, qword ptr [rbp + 0x50]
0000000140838a6a: 4885c0                   test       rax, rax
0000000140838a6d: 7412                     je         0x140838a81
0000000140838a6f: 486bd038                 imul       rdx, rax, 0x38
0000000140838a73: 41b808000000             mov        r8d, 8
0000000140838a79: 4c89e9                   mov        rcx, r13
0000000140838a7c: e89fb1ddff               call       0x140613c20
0000000140838a81: 488b4560                 mov        rax, qword ptr [rbp + 0x60]
0000000140838a85: 4c29f0                   sub        rax, r14
0000000140838a88: 4c39e0                   cmp        rax, r12
0000000140838a8b: 488bbdb0000000           mov        rdi, qword ptr [rbp + 0xb0]
0000000140838a92: 7275                     jb         0x140838b09
0000000140838a94: 4d89e0                   mov        r8, r12
0000000140838a97: 49c1e006                 shl        r8, 6
0000000140838a9b: 4c89f1                   mov        rcx, r14
0000000140838a9e: 48c1e106                 shl        rcx, 6
0000000140838aa2: 48034d48                 add        rcx, qword ptr [rbp + 0x48]
0000000140838aa6: 4889fa                   mov        rdx, rdi
0000000140838aa9: e852b10e02               call       0x142923c00
0000000140838aae: 4d01e6                   add        r14, r12
0000000140838ab1: 4c897570                 mov        qword ptr [rbp + 0x70], r14
0000000140838ab5: 488b95b8000000           mov        rdx, qword ptr [rbp + 0xb8]
0000000140838abc: 4885d2                   test       rdx, rdx
0000000140838abf: 0f84fbfeffff             je         0x1408389c0
0000000140838ac5: 48c1e206                 shl        rdx, 6
0000000140838ac9: 41b808000000             mov        r8d, 8
0000000140838acf: 4889f9                   mov        rcx, rdi
0000000140838ad2: e849b1ddff               call       0x140613c20
0000000140838ad7: e9e4feffff               jmp        0x1408389c0
0000000140838adc: 48c744242038000000       mov        qword ptr [rsp + 0x20], 0x38
0000000140838ae5: 41b908000000             mov        r9d, 8
0000000140838aeb: 488b9dc0000000           mov        rbx, qword ptr [rbp + 0xc0]
0000000140838af2: 4889d9                   mov        rcx, rbx
0000000140838af5: 4c89fa                   mov        rdx, r15
0000000140838af8: 4989f8                   mov        r8, rdi
0000000140838afb: e8600f0702               call       0x1428a9a60
0000000140838b00: 4c8b7b10                 mov        r15, qword ptr [rbx + 0x10]
0000000140838b04: e934ffffff               jmp        0x140838a3d
0000000140838b09: 48c744242040000000       mov        qword ptr [rsp + 0x20], 0x40
0000000140838b12: 41b908000000             mov        r9d, 8
0000000140838b18: 488d4d60                 lea        rcx, [rbp + 0x60]
0000000140838b1c: 4c89f2                   mov        rdx, r14
0000000140838b1f: 4d89e0                   mov        r8, r12
0000000140838b22: e8390f0702               call       0x1428a9a60
0000000140838b27: 488b4568                 mov        rax, qword ptr [rbp + 0x68]
0000000140838b2b: 48894548                 mov        qword ptr [rbp + 0x48], rax
0000000140838b2f: 4c8b7570                 mov        r14, qword ptr [rbp + 0x70]
0000000140838b33: 488bbdb0000000           mov        rdi, qword ptr [rbp + 0xb0]
0000000140838b3a: e955ffffff               jmp        0x140838a94
0000000140838b3f: 4531f6                   xor        r14d, r14d
0000000140838b42: 4d89c4                   mov        r12, r8
0000000140838b45: eb12                     jmp        0x140838b59
0000000140838b47: 4c8b6558                 mov        r12, qword ptr [rbp + 0x58]
0000000140838b4b: 4c89a590000000           mov        qword ptr [rbp + 0x90], r12
0000000140838b52: 488b9dc0000000           mov        rbx, qword ptr [rbp + 0xc0]
0000000140838b59: 4c8b4558                 mov        r8, qword ptr [rbp + 0x58]
0000000140838b5d: 4c89c0                   mov        rax, r8
0000000140838b60: 4c29e0                   sub        rax, r12
0000000140838b63: 48b9abaaaaaaaaaaaaaa     movabs     rcx, 0xaaaaaaaaaaaaaaab
0000000140838b6d: 48f7e1                   mul        rcx
0000000140838b70: 4d39e0                   cmp        r8, r12
0000000140838b73: 4c8d7de8                 lea        r15, [rbp - 0x18]
0000000140838b77: 745d                     je         0x140838bd6
0000000140838b79: 4889d7                   mov        rdi, rdx
0000000140838b7c: 48c1ef05                 shr        rdi, 5
0000000140838b80: 4983c420                 add        r12, 0x20
0000000140838b84: eb13                     jmp        0x140838b99
0000000140838b86: 662e0f1f840000000000     nop        word ptr cs:[rax + rax]
0000000140838b90: 4983c430                 add        r12, 0x30
0000000140838b94: 48ffcf                   dec        rdi
0000000140838b97: 743d                     je         0x140838bd6
0000000140838b99: 498b4424e0               mov        rax, qword ptr [r12 - 0x20]
0000000140838b9e: 4885c0                   test       rax, rax
0000000140838ba1: 7414                     je         0x140838bb7
0000000140838ba3: 498b4c24e8               mov        rcx, qword ptr [r12 - 0x18]
0000000140838ba8: 486bd038                 imul       rdx, rax, 0x38
0000000140838bac: 41b808000000             mov        r8d, 8
0000000140838bb2: e869b0ddff               call       0x140613c20
0000000140838bb7: 498b5424f8               mov        rdx, qword ptr [r12 - 8]
0000000140838bbc: 4885d2                   test       rdx, rdx
0000000140838bbf: 74cf                     je         0x140838b90
0000000140838bc1: 498b0c24                 mov        rcx, qword ptr [r12]
0000000140838bc5: 48c1e206                 shl        rdx, 6
0000000140838bc9: 41b808000000             mov        r8d, 8
0000000140838bcf: e84cb0ddff               call       0x140613c20
0000000140838bd4: ebba                     jmp        0x140838b90
0000000140838bd6: 488bb540010000           mov        rsi, qword ptr [rbp + 0x140]
0000000140838bdd: 488b4528                 mov        rax, qword ptr [rbp + 0x28]
0000000140838be1: 4885c0                   test       rax, rax
0000000140838be4: 7417                     je         0x140838bfd
0000000140838be6: 48c1e004                 shl        rax, 4
0000000140838bea: 488d1440                 lea        rdx, [rax + rax*2]
0000000140838bee: 41b808000000             mov        r8d, 8
0000000140838bf4: 488b4d30                 mov        rcx, qword ptr [rbp + 0x30]
0000000140838bf8: e823b0ddff               call       0x140613c20
0000000140838bfd: 488d45d0                 lea        rax, [rbp - 0x30]
0000000140838c01: 48898588000000           mov        qword ptr [rbp + 0x88], rax
0000000140838c08: 488d45dc                 lea        rax, [rbp - 0x24]
0000000140838c0c: 48898590000000           mov        qword ptr [rbp + 0x90], rax
0000000140838c13: 488b4520                 mov        rax, qword ptr [rbp + 0x20]
0000000140838c17: 48898598000000           mov        qword ptr [rbp + 0x98], rax
0000000140838c1e: 4889b5a0000000           mov        qword ptr [rbp + 0xa0], rsi
0000000140838c25: 488d9588000000           lea        rdx, [rbp + 0x88]
0000000140838c2c: 4889d9                   mov        rcx, rbx
0000000140838c2f: e84cdaab00               call       0x1412f6680
0000000140838c34: 488b7d68                 mov        rdi, qword ptr [rbp + 0x68]
0000000140838c38: 4c89bd88000000           mov        qword ptr [rbp + 0x88], r15
0000000140838c3f: 4983fe02                 cmp        r14, 2
0000000140838c43: 0f836f010000             jae        0x140838db8
0000000140838c49: 488b5310                 mov        rdx, qword ptr [rbx + 0x10]
0000000140838c4d: 4885d2                   test       rdx, rdx
0000000140838c50: 0f84f9000000             je         0x140838d4f
0000000140838c56: 488b4b08                 mov        rcx, qword ptr [rbx + 8]
0000000140838c5a: f30f104920               movss      xmm1, dword ptr [rcx + 0x20]
0000000140838c5f: 0f57c0                   xorps      xmm0, xmm0
0000000140838c62: 0f2ec8                   ucomiss    xmm1, xmm0
0000000140838c65: 730e                     jae        0x140838c75
0000000140838c67: 0fb64131                 movzx      eax, byte ptr [rcx + 0x31]
0000000140838c6b: 41b901000000             mov        r9d, 1
0000000140838c71: 2404                     and        al, 4
0000000140838c73: 7443                     je         0x140838cb8
0000000140838c75: 4c8d4158                 lea        r8, [rcx + 0x58]
0000000140838c79: 41b901000000             mov        r9d, 1
0000000140838c7f: 31c0                     xor        eax, eax
0000000140838c81: 6666666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140838c90: 4c39ca                   cmp        rdx, r9
0000000140838c93: 0f849e000000             je         0x140838d37
0000000140838c99: 4d89c2                   mov        r10, r8
0000000140838c9c: f3410f1008               movss      xmm1, dword ptr [r8]
0000000140838ca1: 49ffc1                   inc        r9
0000000140838ca4: 4983c038                 add        r8, 0x38
0000000140838ca8: 0f2ec8                   ucomiss    xmm1, xmm0
0000000140838cab: 73e3                     jae        0x140838c90
0000000140838cad: 450fb65211               movzx      r10d, byte ptr [r10 + 0x11]
0000000140838cb2: 4180e204                 and        r10b, 4
0000000140838cb6: 75d8                     jne        0x140838c90
0000000140838cb8: b801000000               mov        eax, 1
0000000140838cbd: 4989d0                   mov        r8, rdx
0000000140838cc0: 4d29c8                   sub        r8, r9
0000000140838cc3: 7472                     je         0x140838d37
0000000140838cc5: 496bc138                 imul       rax, r9, 0x38
0000000140838cc9: 4801c1                   add        rcx, rax
0000000140838ccc: b801000000               mov        eax, 1
0000000140838cd1: 0f57c0                   xorps      xmm0, xmm0
0000000140838cd4: eb3c                     jmp        0x140838d12
0000000140838cd6: 662e0f1f840000000000     nop        word ptr cs:[rax + rax]
0000000140838ce0: 4c6bc8c8                 imul       r9, rax, -0x38
0000000140838ce4: 4c8b5130                 mov        r10, qword ptr [rcx + 0x30]
0000000140838ce8: 4e89540930               mov        qword ptr [rcx + r9 + 0x30], r10
0000000140838ced: 0f1009                   movups     xmm1, xmmword ptr [rcx]
0000000140838cf0: 0f105110                 movups     xmm2, xmmword ptr [rcx + 0x10]
0000000140838cf4: 0f105920                 movups     xmm3, xmmword ptr [rcx + 0x20]
0000000140838cf8: 420f115c0920             movups     xmmword ptr [rcx + r9 + 0x20], xmm3
0000000140838cfe: 420f11540910             movups     xmmword ptr [rcx + r9 + 0x10], xmm2
0000000140838d04: 420f110c09               movups     xmmword ptr [rcx + r9], xmm1
0000000140838d09: 4883c138                 add        rcx, 0x38
0000000140838d0d: 49ffc8                   dec        r8
0000000140838d10: 7421                     je         0x140838d33
0000000140838d12: f30f104920               movss      xmm1, dword ptr [rcx + 0x20]
0000000140838d17: 0f2ec8                   ucomiss    xmm1, xmm0
0000000140838d1a: 73c4                     jae        0x140838ce0
0000000140838d1c: 440fb64931               movzx      r9d, byte ptr [rcx + 0x31]
0000000140838d21: 4180e104                 and        r9b, 4
0000000140838d25: 75b9                     jne        0x140838ce0
0000000140838d27: 48ffc0                   inc        rax
0000000140838d2a: 4883c138                 add        rcx, 0x38
0000000140838d2e: 49ffc8                   dec        r8
0000000140838d31: 75df                     jne        0x140838d12
0000000140838d33: 488b4b08                 mov        rcx, qword ptr [rbx + 8]
0000000140838d37: 4829c2                   sub        rdx, rax
0000000140838d3a: 48895310                 mov        qword ptr [rbx + 0x10], rdx
0000000140838d3e: 4c89bd88000000           mov        qword ptr [rbp + 0x88], r15
0000000140838d45: 4883fa02                 cmp        rdx, 2
0000000140838d49: 0f8390000000             jae        0x140838ddf
0000000140838d4f: 488b03                   mov        rax, qword ptr [rbx]
0000000140838d52: 488b5310                 mov        rdx, qword ptr [rbx + 0x10]
0000000140838d56: 4829d0                   sub        rax, rdx
0000000140838d59: 4c39f0                   cmp        rax, r14
0000000140838d5c: 0f829e000000             jb         0x140838e00
0000000140838d62: 4d85f6                   test       r14, r14
0000000140838d65: 0f8481010000             je         0x140838eec
0000000140838d6b: 488b4308                 mov        rax, qword ptr [rbx + 8]
0000000140838d6f: 4983fe01                 cmp        r14, 1
0000000140838d73: 0f85b3000000             jne        0x140838e2c
0000000140838d79: 31c9                     xor        ecx, ecx
0000000140838d7b: 41f6c601                 test       r14b, 1
0000000140838d7f: 0f8532010000             jne        0x140838eb7
0000000140838d85: e962010000               jmp        0x140838eec
0000000140838d8a: 4885c0                   test       rax, rax
0000000140838d8d: 0f85e7010000             jne        0x140838f7a
0000000140838d93: 31d2                     xor        edx, edx
0000000140838d95: e836fa0b02               call       0x1428f87d0
0000000140838d9a: 4889c1                   mov        rcx, rax
0000000140838d9d: 4885c0                   test       rax, rax
0000000140838da0: 0f8526f8ffff             jne        0x1408385cc
0000000140838da6: e9cf010000               jmp        0x140838f7a
0000000140838dab: 4889f9                   mov        rcx, rdi
0000000140838dae: e89d410902               call       0x1428ccf50
0000000140838db3: e98bfaffff               jmp        0x140838843
0000000140838db8: 4983fe15                 cmp        r14, 0x15
0000000140838dbc: 0f83ec010000             jae        0x140838fae
0000000140838dc2: 4c8d8d88000000           lea        r9, [rbp + 0x88]
0000000140838dc9: 41b801000000             mov        r8d, 1
0000000140838dcf: 4889f9                   mov        rcx, rdi
0000000140838dd2: 4c89f2                   mov        rdx, r14
0000000140838dd5: e86606b000               call       0x141339440
0000000140838dda: e96afeffff               jmp        0x140838c49
0000000140838ddf: 4883fa15                 cmp        rdx, 0x15
0000000140838de3: 0f83dc010000             jae        0x140838fc5
0000000140838de9: 4c8d8d88000000           lea        r9, [rbp + 0x88]
0000000140838df0: 41b801000000             mov        r8d, 1
0000000140838df6: e8350cb000               call       0x141339a30
0000000140838dfb: e94fffffff               jmp        0x140838d4f
0000000140838e00: 48c744242038000000       mov        qword ptr [rsp + 0x20], 0x38
0000000140838e09: 41b908000000             mov        r9d, 8
0000000140838e0f: 4889d9                   mov        rcx, rbx
0000000140838e12: 4d89f0                   mov        r8, r14
0000000140838e15: e8460c0702               call       0x1428a9a60
0000000140838e1a: 488b5310                 mov        rdx, qword ptr [rbx + 0x10]
0000000140838e1e: 488b4308                 mov        rax, qword ptr [rbx + 8]
0000000140838e22: 4983fe01                 cmp        r14, 1
0000000140838e26: 0f844dffffff             je         0x140838d79
0000000140838e2c: 4c6bc238                 imul       r8, rdx, 0x38
0000000140838e30: 4901c0                   add        r8, rax
0000000140838e33: 4c8d4f48                 lea        r9, [rdi + 0x48]
0000000140838e37: 4d89f2                   mov        r10, r14
0000000140838e3a: 4983e2fe                 and        r10, 0xfffffffffffffffe
0000000140838e3e: 49f7da                   neg        r10
0000000140838e41: 31c9                     xor        ecx, ecx
0000000140838e43: 666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140838e50: 4d8b59f0                 mov        r11, qword ptr [r9 - 0x10]
0000000140838e54: 4d895830                 mov        qword ptr [r8 + 0x30], r11
0000000140838e58: 410f1041c0               movups     xmm0, xmmword ptr [r9 - 0x40]
0000000140838e5d: 410f1049d0               movups     xmm1, xmmword ptr [r9 - 0x30]
0000000140838e62: 410f1051e0               movups     xmm2, xmmword ptr [r9 - 0x20]
0000000140838e67: 410f115020               movups     xmmword ptr [r8 + 0x20], xmm2
0000000140838e6c: 410f114810               movups     xmmword ptr [r8 + 0x10], xmm1
0000000140838e71: 410f1100                 movups     xmmword ptr [r8], xmm0
0000000140838e75: 410f1001                 movups     xmm0, xmmword ptr [r9]
0000000140838e79: 410f104910               movups     xmm1, xmmword ptr [r9 + 0x10]
0000000140838e7e: 410f105120               movups     xmm2, xmmword ptr [r9 + 0x20]
0000000140838e83: 410f114038               movups     xmmword ptr [r8 + 0x38], xmm0
0000000140838e88: 410f114848               movups     xmmword ptr [r8 + 0x48], xmm1
0000000140838e8d: 410f115058               movups     xmmword ptr [r8 + 0x58], xmm2
0000000140838e92: 4d8b5930                 mov        r11, qword ptr [r9 + 0x30]
0000000140838e96: 4d895868                 mov        qword ptr [r8 + 0x68], r11
0000000140838e9a: 4983c070                 add        r8, 0x70
0000000140838e9e: 4883c1fe                 add        rcx, -2
0000000140838ea2: 4983e980                 sub        r9, -0x80
0000000140838ea6: 4939ca                   cmp        r10, rcx
0000000140838ea9: 75a5                     jne        0x140838e50
0000000140838eab: 4829ca                   sub        rdx, rcx
0000000140838eae: 48f7d9                   neg        rcx
0000000140838eb1: 41f6c601                 test       r14b, 1
0000000140838eb5: 7435                     je         0x140838eec
0000000140838eb7: 48c1e106                 shl        rcx, 6
0000000140838ebb: 4c6bc238                 imul       r8, rdx, 0x38
0000000140838ebf: 4c8b4c0f38               mov        r9, qword ptr [rdi + rcx + 0x38]
0000000140838ec4: 4e894c0030               mov        qword ptr [rax + r8 + 0x30], r9
0000000140838ec9: 0f10440f08               movups     xmm0, xmmword ptr [rdi + rcx + 8]
0000000140838ece: 0f104c0f18               movups     xmm1, xmmword ptr [rdi + rcx + 0x18]
0000000140838ed3: 0f10540f28               movups     xmm2, xmmword ptr [rdi + rcx + 0x28]
0000000140838ed8: 420f11540020             movups     xmmword ptr [rax + r8 + 0x20], xmm2
0000000140838ede: 420f114c0010             movups     xmmword ptr [rax + r8 + 0x10], xmm1
0000000140838ee4: 420f110400               movups     xmmword ptr [rax + r8], xmm0
0000000140838ee9: 48ffc2                   inc        rdx
0000000140838eec: 48895310                 mov        qword ptr [rbx + 0x10], rdx
0000000140838ef0: 488b5560                 mov        rdx, qword ptr [rbp + 0x60]
0000000140838ef4: 4885d2                   test       rdx, rdx
0000000140838ef7: 7412                     je         0x140838f0b
0000000140838ef9: 48c1e206                 shl        rdx, 6
0000000140838efd: 41b808000000             mov        r8d, 8
0000000140838f03: 4889f9                   mov        rcx, rdi
0000000140838f06: e815adddff               call       0x140613c20
0000000140838f0b: 48837d3800               cmp        qword ptr [rbp + 0x38], 0
0000000140838f10: 7440                     je         0x140838f52
0000000140838f12: e819f23d00               call       0x140c18130
0000000140838f17: 4889c1                   mov        rcx, rax
0000000140838f1a: 488b00                   mov        rax, qword ptr [rax]
0000000140838f1d: 4883f801                 cmp        rax, 1
0000000140838f21: 7543                     jne        0x140838f66
0000000140838f23: 4883c108                 add        rcx, 8
0000000140838f27: 48833900                 cmp        qword ptr [rcx], 0
0000000140838f2b: 7559                     jne        0x140838f86
0000000140838f2d: 48c701ffffffff           mov        qword ptr [rcx], 0xffffffffffffffff
0000000140838f34: 48898dc0000000           mov        qword ptr [rbp + 0xc0], rcx
0000000140838f3b: 4883c108                 add        rcx, 8
0000000140838f3f: 488b5540                 mov        rdx, qword ptr [rbp + 0x40]
0000000140838f43: e8a8e13d00               call       0x140c170f0
0000000140838f48: 488b85c0000000           mov        rax, qword ptr [rbp + 0xc0]
0000000140838f4f: 48ff00                   inc        qword ptr [rax]
0000000140838f52: 4881c458010000           add        rsp, 0x158
0000000140838f59: 5b                       pop        rbx
0000000140838f5a: 5f                       pop        rdi
0000000140838f5b: 5e                       pop        rsi
0000000140838f5c: 415c                     pop        r12
0000000140838f5e: 415d                     pop        r13
0000000140838f60: 415e                     pop        r14
0000000140838f62: 415f                     pop        r15
0000000140838f64: 5d                       pop        rbp
0000000140838f65: c3                       ret        
0000000140838f66: 4885c0                   test       rax, rax
0000000140838f69: 750f                     jne        0x140838f7a
0000000140838f6b: 31d2                     xor        edx, edx
0000000140838f6d: e85ef80b02               call       0x1428f87d0
0000000140838f72: 4889c1                   mov        rcx, rax
0000000140838f75: 4885c0                   test       rax, rax
0000000140838f78: 75ad                     jne        0x140838f27
0000000140838f7a: 488d0d1f1b4002           lea        rcx, [rip + 0x2401b1f]
0000000140838f81: e84af10902               call       0x1428d80d0
0000000140838f86: 488d0de31b4002           lea        rcx, [rip + 0x2401be3]
0000000140838f8d: e8be020a02               call       0x1428d9250
0000000140838f92: 488d0d071b4002           lea        rcx, [rip + 0x2401b07]
0000000140838f99: e832f10902               call       0x1428d80d0
0000000140838f9e: eb0c                     jmp        0x140838fac
0000000140838fa0: 488d0d59fa3f02           lea        rcx, [rip + 0x23ffa59]
0000000140838fa7: e8340a0a02               call       0x1428d99e0
0000000140838fac: 0f0b                     ud2        
0000000140838fae: 4c8d8588000000           lea        r8, [rbp + 0x88]
0000000140838fb5: 4889f9                   mov        rcx, rdi
0000000140838fb8: 4c89f2                   mov        rdx, r14
0000000140838fbb: e830293100               call       0x140b4b8f0
0000000140838fc0: e984fcffff               jmp        0x140838c49
0000000140838fc5: 4c8d8588000000           lea        r8, [rbp + 0x88]
0000000140838fcc: e8ff3d3100               call       0x140b4cdd0
0000000140838fd1: e979fdffff               jmp        0x140838d4f
0000000140838fd6: 662e0f1f840000000000     nop        word ptr cs:[rax + rax]
0000000140838fe0: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140838fe5: 55                       push       rbp
0000000140838fe6: 4157                     push       r15
0000000140838fe8: 4156                     push       r14
0000000140838fea: 4155                     push       r13
0000000140838fec: 4154                     push       r12
0000000140838fee: 56                       push       rsi
0000000140838fef: 57                       push       rdi
0000000140838ff0: 53                       push       rbx
0000000140838ff1: 4883ec38                 sub        rsp, 0x38
0000000140838ff5: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140838ffc: 488b85c0000000           mov        rax, qword ptr [rbp + 0xc0]
0000000140839003: 48ff00                   inc        qword ptr [rax]
0000000140839006: 4883c438                 add        rsp, 0x38
000000014083900a: 5b                       pop        rbx
000000014083900b: 5f                       pop        rdi
000000014083900c: 5e                       pop        rsi
000000014083900d: 415c                     pop        r12
000000014083900f: 415d                     pop        r13
0000000140839011: 415e                     pop        r14
0000000140839013: 415f                     pop        r15
0000000140839015: 5d                       pop        rbp
0000000140839016: c3                       ret        
0000000140839017: 660f1f840000000000       nop        word ptr [rax + rax]
0000000140839020: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140839025: 55                       push       rbp
0000000140839026: 4157                     push       r15
0000000140839028: 4156                     push       r14
000000014083902a: 4155                     push       r13
000000014083902c: 4154                     push       r12
000000014083902e: 56                       push       rsi
000000014083902f: 57                       push       rdi
0000000140839030: 53                       push       rbx
0000000140839031: 4883ec38                 sub        rsp, 0x38
0000000140839035: 488daa80000000           lea        rbp, [rdx + 0x80]
000000014083903c: 48837d3800               cmp        qword ptr [rbp + 0x38], 0
0000000140839041: 7420                     je         0x140839063
0000000140839043: 488d4540                 lea        rax, [rbp + 0x40]
0000000140839047: 48894518                 mov        qword ptr [rbp + 0x18], rax
000000014083904b: 488d4518                 lea        rax, [rbp + 0x18]
000000014083904f: 48894510                 mov        qword ptr [rbp + 0x10], rax
0000000140839053: 488d0d2e952502           lea        rcx, [rip + 0x225952e]
000000014083905a: 488d5510                 lea        rdx, [rbp + 0x10]
000000014083905e: e86d62af00               call       0x14132f2d0
0000000140839063: 90                       nop        
0000000140839064: 4883c438                 add        rsp, 0x38
0000000140839068: 5b                       pop        rbx
0000000140839069: 5f                       pop        rdi
000000014083906a: 5e                       pop        rsi
000000014083906b: 415c                     pop        r12
000000014083906d: 415d                     pop        r13
000000014083906f: 415e                     pop        r14
0000000140839071: 415f                     pop        r15
0000000140839073: 5d                       pop        rbp
0000000140839074: c3                       ret        
0000000140839075: 66662e0f1f840000000000   nop        word ptr cs:[rax + rax]
0000000140839080: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140839085: 55                       push       rbp
0000000140839086: 4157                     push       r15
0000000140839088: 4156                     push       r14
000000014083908a: 4155                     push       r13
000000014083908c: 4154                     push       r12
000000014083908e: 56                       push       rsi
000000014083908f: 57                       push       rdi
0000000140839090: 53                       push       rbx
0000000140839091: 4883ec38                 sub        rsp, 0x38
0000000140839095: 488daa80000000           lea        rbp, [rdx + 0x80]
000000014083909c: 488b8db0000000           mov        rcx, qword ptr [rbp + 0xb0]
00000001408390a3: e8d88180ff               call       0x140041280
00000001408390a8: 90                       nop        
00000001408390a9: 4883c438                 add        rsp, 0x38
00000001408390ad: 5b                       pop        rbx
00000001408390ae: 5f                       pop        rdi
00000001408390af: 5e                       pop        rsi
00000001408390b0: 415c                     pop        r12
00000001408390b2: 415d                     pop        r13
00000001408390b4: 415e                     pop        r14
00000001408390b6: 415f                     pop        r15
00000001408390b8: 5d                       pop        rbp
00000001408390b9: c3                       ret        
00000001408390ba: 660f1f440000             nop        word ptr [rax + rax]
00000001408390c0: 4889542410               mov        qword ptr [rsp + 0x10], rdx
00000001408390c5: 55                       push       rbp
00000001408390c6: 4157                     push       r15
00000001408390c8: 4156                     push       r14
00000001408390ca: 4155                     push       r13
00000001408390cc: 4154                     push       r12
00000001408390ce: 56                       push       rsi
00000001408390cf: 57                       push       rdi
00000001408390d0: 53                       push       rbx
00000001408390d1: 4883ec38                 sub        rsp, 0x38
00000001408390d5: 488daa80000000           lea        rbp, [rdx + 0x80]
00000001408390dc: 488b5560                 mov        rdx, qword ptr [rbp + 0x60]
00000001408390e0: 4885d2                   test       rdx, rdx
00000001408390e3: 7413                     je         0x1408390f8
00000001408390e5: 488b4d68                 mov        rcx, qword ptr [rbp + 0x68]
00000001408390e9: 48c1e206                 shl        rdx, 6
00000001408390ed: 41b808000000             mov        r8d, 8
00000001408390f3: e828abddff               call       0x140613c20
00000001408390f8: 90                       nop        
00000001408390f9: 4883c438                 add        rsp, 0x38
00000001408390fd: 5b                       pop        rbx
00000001408390fe: 5f                       pop        rdi
00000001408390ff: 5e                       pop        rsi
0000000140839100: 415c                     pop        r12
0000000140839102: 415d                     pop        r13
0000000140839104: 415e                     pop        r14
0000000140839106: 415f                     pop        r15
0000000140839108: 5d                       pop        rbp
0000000140839109: c3                       ret        
000000014083910a: 660f1f440000             nop        word ptr [rax + rax]
0000000140839110: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140839115: 55                       push       rbp
0000000140839116: 4157                     push       r15
0000000140839118: 4156                     push       r14
000000014083911a: 4155                     push       r13
000000014083911c: 4154                     push       r12
000000014083911e: 56                       push       rsi
000000014083911f: 57                       push       rdi
0000000140839120: 53                       push       rbx
0000000140839121: 4883ec38                 sub        rsp, 0x38
0000000140839125: 488daa80000000           lea        rbp, [rdx + 0x80]
000000014083912c: 488d8d88000000           lea        rcx, [rbp + 0x88]
0000000140839133: e808d50400               call       0x140886640
0000000140839138: 90                       nop        
0000000140839139: 4883c438                 add        rsp, 0x38
000000014083913d: 5b                       pop        rbx
000000014083913e: 5f                       pop        rdi
000000014083913f: 5e                       pop        rsi
0000000140839140: 415c                     pop        r12
0000000140839142: 415d                     pop        r13
0000000140839144: 415e                     pop        r14
0000000140839146: 415f                     pop        r15
0000000140839148: 5d                       pop        rbp
0000000140839149: c3                       ret        
000000014083914a: 660f1f440000             nop        word ptr [rax + rax]
0000000140839150: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140839155: 55                       push       rbp
0000000140839156: 4157                     push       r15
0000000140839158: 4156                     push       r14
000000014083915a: 4155                     push       r13
000000014083915c: 4154                     push       r12
000000014083915e: 56                       push       rsi
000000014083915f: 57                       push       rdi
0000000140839160: 53                       push       rbx
0000000140839161: 4883ec38                 sub        rsp, 0x38
0000000140839165: 488daa80000000           lea        rbp, [rdx + 0x80]
000000014083916c: 488b85c0000000           mov        rax, qword ptr [rbp + 0xc0]
0000000140839173: 48ff00                   inc        qword ptr [rax]
0000000140839176: 4883c438                 add        rsp, 0x38
000000014083917a: 5b                       pop        rbx
000000014083917b: 5f                       pop        rdi
000000014083917c: 5e                       pop        rsi
000000014083917d: 415c                     pop        r12
000000014083917f: 415d                     pop        r13
0000000140839181: 415e                     pop        r14
0000000140839183: 415f                     pop        r15
0000000140839185: 5d                       pop        rbp
0000000140839186: c3                       ret        
0000000140839187: 660f1f840000000000       nop        word ptr [rax + rax]
0000000140839190: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140839195: 55                       push       rbp
0000000140839196: 4157                     push       r15
0000000140839198: 4156                     push       r14
000000014083919a: 4155                     push       r13
000000014083919c: 4154                     push       r12
000000014083919e: 56                       push       rsi
000000014083919f: 57                       push       rdi
00000001408391a0: 53                       push       rbx
00000001408391a1: 4883ec38                 sub        rsp, 0x38
00000001408391a5: 488daa80000000           lea        rbp, [rdx + 0x80]
00000001408391ac: 488b85a8000000           mov        rax, qword ptr [rbp + 0xa8]
00000001408391b3: 48898590000000           mov        qword ptr [rbp + 0x90], rax
00000001408391ba: 48837d5000               cmp        qword ptr [rbp + 0x50], 0
00000001408391bf: 7417                     je         0x1408391d8
00000001408391c1: 486b555038               imul       rdx, qword ptr [rbp + 0x50], 0x38
00000001408391c6: 41b808000000             mov        r8d, 8
00000001408391cc: 488b8d80000000           mov        rcx, qword ptr [rbp + 0x80]
00000001408391d3: e848aaddff               call       0x140613c20
00000001408391d8: c685cf00000001           mov        byte ptr [rbp + 0xcf], 1
00000001408391df: 4883c438                 add        rsp, 0x38
00000001408391e3: 5b                       pop        rbx
00000001408391e4: 5f                       pop        rdi
00000001408391e5: 5e                       pop        rsi
00000001408391e6: 415c                     pop        r12
00000001408391e8: 415d                     pop        r13
00000001408391ea: 415e                     pop        r14
00000001408391ec: 415f                     pop        r15
00000001408391ee: 5d                       pop        rbp
00000001408391ef: c3                       ret        
00000001408391f0: 4889542410               mov        qword ptr [rsp + 0x10], rdx
00000001408391f5: 55                       push       rbp
00000001408391f6: 4157                     push       r15
00000001408391f8: 4156                     push       r14
00000001408391fa: 4155                     push       r13
00000001408391fc: 4154                     push       r12
00000001408391fe: 56                       push       rsi
00000001408391ff: 57                       push       rdi
0000000140839200: 53                       push       rbx
0000000140839201: 4883ec38                 sub        rsp, 0x38
0000000140839205: 488daa80000000           lea        rbp, [rdx + 0x80]
000000014083920c: 488b85a8000000           mov        rax, qword ptr [rbp + 0xa8]
0000000140839213: 48898590000000           mov        qword ptr [rbp + 0x90], rax
000000014083921a: 4883bdb800000000         cmp        qword ptr [rbp + 0xb8], 0
0000000140839222: 741d                     je         0x140839241
0000000140839224: 488b95b8000000           mov        rdx, qword ptr [rbp + 0xb8]
000000014083922b: 48c1e206                 shl        rdx, 6
000000014083922f: 41b808000000             mov        r8d, 8
0000000140839235: 488b8db0000000           mov        rcx, qword ptr [rbp + 0xb0]
000000014083923c: e8dfa9ddff               call       0x140613c20
0000000140839241: c685cf00000000           mov        byte ptr [rbp + 0xcf], 0
0000000140839248: 4883c438                 add        rsp, 0x38
000000014083924c: 5b                       pop        rbx
000000014083924d: 5f                       pop        rdi
000000014083924e: 5e                       pop        rsi
000000014083924f: 415c                     pop        r12
0000000140839251: 415d                     pop        r13
0000000140839253: 415e                     pop        r14
0000000140839255: 415f                     pop        r15
0000000140839257: 5d                       pop        rbp
0000000140839258: c3                       ret        
0000000140839259: 0f1f8000000000           nop        dword ptr [rax]
0000000140839260: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140839265: 55                       push       rbp
0000000140839266: 4157                     push       r15
0000000140839268: 4156                     push       r14
000000014083926a: 4155                     push       r13
000000014083926c: 4154                     push       r12
000000014083926e: 56                       push       rsi
000000014083926f: 57                       push       rdi
0000000140839270: 53                       push       rbx
0000000140839271: 4883ec38                 sub        rsp, 0x38
0000000140839275: 488daa80000000           lea        rbp, [rdx + 0x80]
000000014083927c: 80bdcf00000000           cmp        byte ptr [rbp + 0xcf], 0
0000000140839283: 7439                     je         0x1408392be
0000000140839285: 4883bdb800000000         cmp        qword ptr [rbp + 0xb8], 0
000000014083928d: 741d                     je         0x1408392ac
000000014083928f: 488b95b8000000           mov        rdx, qword ptr [rbp + 0xb8]
0000000140839296: 48c1e206                 shl        rdx, 6
000000014083929a: 41b808000000             mov        r8d, 8
00000001408392a0: 488b8db0000000           mov        rcx, qword ptr [rbp + 0xb0]
00000001408392a7: e874a9ddff               call       0x140613c20
00000001408392ac: 90                       nop        
00000001408392ad: 4883c438                 add        rsp, 0x38
00000001408392b1: 5b                       pop        rbx
00000001408392b2: 5f                       pop        rdi
00000001408392b3: 5e                       pop        rsi
00000001408392b4: 415c                     pop        r12
00000001408392b6: 415d                     pop        r13
00000001408392b8: 415e                     pop        r14
00000001408392ba: 415f                     pop        r15
00000001408392bc: 5d                       pop        rbp
00000001408392bd: c3                       ret        
00000001408392be: 4883c438                 add        rsp, 0x38
00000001408392c2: 5b                       pop        rbx
00000001408392c3: 5f                       pop        rdi
00000001408392c4: 5e                       pop        rsi
00000001408392c5: 415c                     pop        r12
00000001408392c7: 415d                     pop        r13
00000001408392c9: 415e                     pop        r14
00000001408392cb: 415f                     pop        r15
00000001408392cd: 5d                       pop        rbp
00000001408392ce: c3                       ret        