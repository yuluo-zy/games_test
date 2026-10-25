0000000140b24d50  55                       push       rbp
0000000140b24d51  4157                     push       r15
0000000140b24d53  4156                     push       r14
0000000140b24d55  4155                     push       r13
0000000140b24d57  4154                     push       r12
0000000140b24d59  56                       push       rsi
0000000140b24d5a  57                       push       rdi
0000000140b24d5b  53                       push       rbx
0000000140b24d5c  4881ec08020000           sub        rsp, 0x208
0000000140b24d63  488dac2480000000         lea        rbp, [rsp + 0x80]
0000000140b24d6b  48c78580010000feffffff   mov        qword ptr [rbp + 0x180], 0xfffffffffffffffe
0000000140b24d76  4989cd                   mov        r13, rcx
0000000140b24d79  48898d70010000           mov        qword ptr [rbp + 0x170], rcx
0000000140b24d80  4c8db5f9000000           lea        r14, [rbp + 0xf9]
0000000140b24d87  beffffff3f               mov        esi, 0x3fffffff
0000000140b24d8c  488b3d8d76b002           mov        rdi, qword ptr [rip + 0x2b0768d]
0000000140b24d93  488d9d50010000           lea        rbx, [rbp + 0x150]
0000000140b24d9a  660f1f440000             nop        word ptr [rax + rax]
0000000140b24da0  4d8d6510                 lea        r12, [r13 + 0x10]
0000000140b24da4  31c0                     xor        eax, eax
0000000140b24da6  f0410fb17510             lock cmpxchg dword ptr [r13 + 0x10], esi
0000000140b24dac  0f852f040000             jne        0x140b251e1
0000000140b24db2  488b07                   mov        rax, qword ptr [rdi]
0000000140b24db5  48d1e0                   shl        rax, 1
0000000140b24db8  4885c0                   test       rax, rax
0000000140b24dbb  0f852d040000             jne        0x140b251ee
0000000140b24dc1  31c0                     xor        eax, eax
0000000140b24dc3  410fb64d18               movzx      ecx, byte ptr [r13 + 0x18]
0000000140b24dc8  84c9                     test       cl, cl
0000000140b24dca  0f853a050000             jne        0x140b2530a
0000000140b24dd0  410fb68da0000000         movzx      ecx, byte ptr [r13 + 0xa0]
0000000140b24dd8  41c685a000000002         mov        byte ptr [r13 + 0xa0], 2
0000000140b24de0  80f902                   cmp        cl, 2
0000000140b24de3  754b                     jne        0x140b24e30
0000000140b24de5  84c0                     test       al, al
0000000140b24de7  750f                     jne        0x140b24df8
0000000140b24de9  488b07                   mov        rax, qword ptr [rdi]
0000000140b24dec  48d1e0                   shl        rax, 1
0000000140b24def  4885c0                   test       rax, rax
0000000140b24df2  0f8576040000             jne        0x140b2526e
0000000140b24df8  ba010000c0               mov        edx, 0xc0000001
0000000140b24dfd  f0410fc11424             lock xadd  dword ptr [r12], edx
0000000140b24e03  81c2010000c0             add        edx, 0xc0000001
0000000140b24e09  81fa00000040             cmp        edx, 0x40000000
0000000140b24e0f  0f83f2030000             jae        0x140b25207
0000000140b24e15  31c9                     xor        ecx, ecx
0000000140b24e17  ba40420f00               mov        edx, 0xf4240
0000000140b24e1c  e85f0b2300               call       0x140d55980
0000000140b24e21  4c8bad70010000           mov        r13, qword ptr [rbp + 0x170]
0000000140b24e28  e973ffffff               jmp        0x140b24da0
0000000140b24e2d  0f1f00                   nop        dword ptr [rax]
0000000140b24e30  498b9598000000           mov        rdx, qword ptr [r13 + 0x98]
0000000140b24e37  488995f0000000           mov        qword ptr [rbp + 0xf0], rdx
0000000140b24e3e  f3410f6f4568             movdqu     xmm0, xmmword ptr [r13 + 0x68]
0000000140b24e44  410f104d78               movups     xmm1, xmmword ptr [r13 + 0x78]
0000000140b24e49  410f109588000000         movups     xmm2, xmmword ptr [r13 + 0x88]
0000000140b24e51  0f2995e0000000           movaps     xmmword ptr [rbp + 0xe0], xmm2
0000000140b24e58  0f298dd0000000           movaps     xmmword ptr [rbp + 0xd0], xmm1
0000000140b24e5f  660f7f85c0000000         movdqa     xmmword ptr [rbp + 0xc0], xmm0
0000000140b24e67  418b95a1000000           mov        edx, dword ptr [r13 + 0xa1]
0000000140b24e6e  458b85a4000000           mov        r8d, dword ptr [r13 + 0xa4]
0000000140b24e75  45894603                 mov        dword ptr [r14 + 3], r8d
0000000140b24e79  418916                   mov        dword ptr [r14], edx
0000000140b24e7c  41c685a800000001         mov        byte ptr [r13 + 0xa8], 1
0000000140b24e84  888df8000000             mov        byte ptr [rbp + 0xf8], cl
0000000140b24e8a  84c0                     test       al, al
0000000140b24e8c  750f                     jne        0x140b24e9d
0000000140b24e8e  488b07                   mov        rax, qword ptr [rdi]
0000000140b24e91  48d1e0                   shl        rax, 1
0000000140b24e94  4885c0                   test       rax, rax
0000000140b24e97  0f85e8030000             jne        0x140b25285
0000000140b24e9d  ba010000c0               mov        edx, 0xc0000001
0000000140b24ea2  f0410fc11424             lock xadd  dword ptr [r12], edx
0000000140b24ea8  81c2010000c0             add        edx, 0xc0000001
0000000140b24eae  81fa00000040             cmp        edx, 0x40000000
0000000140b24eb4  0f835a030000             jae        0x140b25214
0000000140b24eba  488b85c0000000           mov        rax, qword ptr [rbp + 0xc0]
0000000140b24ec1  488b8dc8000000           mov        rcx, qword ptr [rbp + 0xc8]
0000000140b24ec8  4801c1                   add        rcx, rax
0000000140b24ecb  48ffc1                   inc        rcx
0000000140b24ece  660f6f00                 movdqa     xmm0, xmmword ptr [rax]
0000000140b24ed2  660fd7d0                 pmovmskb   edx, xmm0
0000000140b24ed6  f7d2                     not        edx
0000000140b24ed8  4c8b85d8000000           mov        r8, qword ptr [rbp + 0xd8]
0000000140b24edf  48898590000000           mov        qword ptr [rbp + 0x90], rax
0000000140b24ee6  4883c010                 add        rax, 0x10
0000000140b24eea  48898598000000           mov        qword ptr [rbp + 0x98], rax
0000000140b24ef1  48898da0000000           mov        qword ptr [rbp + 0xa0], rcx
0000000140b24ef8  668995a8000000           mov        word ptr [rbp + 0xa8], dx
0000000140b24eff  4c8985b0000000           mov        qword ptr [rbp + 0xb0], r8
0000000140b24f06  48c745f000000000         mov        qword ptr [rbp - 0x10], 0
0000000140b24f0e  48c7454000000000         mov        qword ptr [rbp + 0x40], 0
0000000140b24f16  4889d9                   mov        rcx, rbx
0000000140b24f19  488d55f0                 lea        rdx, [rbp - 0x10]
0000000140b24f1d  4c8d05bc47fc01           lea        r8, [rip + 0x1fc47bc]
0000000140b24f24  e887827e00               call       0x14130d1b0
0000000140b24f29  488b9558010000           mov        rdx, qword ptr [rbp + 0x158]
0000000140b24f30  4c8b8560010000           mov        r8, qword ptr [rbp + 0x160]
0000000140b24f37  488d4da8                 lea        rcx, [rbp - 0x58]
0000000140b24f3b  e8108bfeff               call       0x140b0da50
0000000140b24f40  488b45e8                 mov        rax, qword ptr [rbp - 0x18]
0000000140b24f44  48894530                 mov        qword ptr [rbp + 0x30], rax
0000000140b24f48  f30f6f45a8               movdqu     xmm0, xmmword ptr [rbp - 0x58]
0000000140b24f4d  0f104db8                 movups     xmm1, xmmword ptr [rbp - 0x48]
0000000140b24f51  0f1055c8                 movups     xmm2, xmmword ptr [rbp - 0x38]
0000000140b24f55  0f105dd8                 movups     xmm3, xmmword ptr [rbp - 0x28]
0000000140b24f59  0f295d20                 movaps     xmmword ptr [rbp + 0x20], xmm3
0000000140b24f5d  0f295510                 movaps     xmmword ptr [rbp + 0x10], xmm2
0000000140b24f61  0f294d00                 movaps     xmmword ptr [rbp], xmm1
0000000140b24f65  660f7f45f0               movdqa     xmmword ptr [rbp - 0x10], xmm0
0000000140b24f6a  4c8bbd70010000           mov        r15, qword ptr [rbp + 0x170]
0000000140b24f71  4d8d6710                 lea        r12, [r15 + 0x10]
0000000140b24f75  31c0                     xor        eax, eax
0000000140b24f77  f0410fb17710             lock cmpxchg dword ptr [r15 + 0x10], esi
0000000140b24f7d  0f85b2020000             jne        0x140b25235
0000000140b24f83  488b07                   mov        rax, qword ptr [rdi]
0000000140b24f86  48d1e0                   shl        rax, 1
0000000140b24f89  4885c0                   test       rax, rax
0000000140b24f8c  0f85b7020000             jne        0x140b25249
0000000140b24f92  4531ed                   xor        r13d, r13d
0000000140b24f95  410fb64718               movzx      eax, byte ptr [r15 + 0x18]
0000000140b24f9a  31c9                     xor        ecx, ecx
0000000140b24f9c  84c0                     test       al, al
0000000140b24f9e  0f95c1                   setne      cl
0000000140b24fa1  4c89a508010000           mov        qword ptr [rbp + 0x108], r12
0000000140b24fa8  4488ad10010000           mov        byte ptr [rbp + 0x110], r13b
0000000140b24faf  48898d00010000           mov        qword ptr [rbp + 0x100], rcx
0000000140b24fb6  0f8496000000             je         0x140b25052
0000000140b24fbc  4584ed                   test       r13b, r13b
0000000140b24fbf  750f                     jne        0x140b24fd0
0000000140b24fc1  488b07                   mov        rax, qword ptr [rdi]
0000000140b24fc4  48d1e0                   shl        rax, 1
0000000140b24fc7  4885c0                   test       rax, rax
0000000140b24fca  0f85dd020000             jne        0x140b252ad
0000000140b24fd0  ba010000c0               mov        edx, 0xc0000001
0000000140b24fd5  f0410fc11424             lock xadd  dword ptr [r12], edx
0000000140b24fdb  81c2010000c0             add        edx, 0xc0000001
0000000140b24fe1  81fa00000040             cmp        edx, 0x40000000
0000000140b24fe7  0f8334020000             jae        0x140b25221
0000000140b24fed  488b45f0                 mov        rax, qword ptr [rbp - 0x10]
0000000140b24ff1  4885c0                   test       rax, rax
0000000140b24ff4  7417                     je         0x140b2500d
0000000140b24ff6  488b4df8                 mov        rcx, qword ptr [rbp - 8]
0000000140b24ffa  48c1e004                 shl        rax, 4
0000000140b24ffe  488d1480                 lea        rdx, [rax + rax*4]
0000000140b25002  41b804000000             mov        r8d, 4
0000000140b25008  e813ecaeff               call       0x140613c20
0000000140b2500d  488b4508                 mov        rax, qword ptr [rbp + 8]
0000000140b25011  4885c0                   test       rax, rax
0000000140b25014  7417                     je         0x140b2502d
0000000140b25016  488b4d10                 mov        rcx, qword ptr [rbp + 0x10]
0000000140b2501a  48c1e003                 shl        rax, 3
0000000140b2501e  488d1440                 lea        rdx, [rax + rax*2]
0000000140b25022  41b804000000             mov        r8d, 4
0000000140b25028  e8f3ebaeff               call       0x140613c20
0000000140b2502d  488b5520                 mov        rdx, qword ptr [rbp + 0x20]
0000000140b25031  4885d2                   test       rdx, rdx
0000000140b25034  0f8456010000             je         0x140b25190
0000000140b2503a  488b4d28                 mov        rcx, qword ptr [rbp + 0x28]
0000000140b2503e  48c1e202                 shl        rdx, 2
0000000140b25042  41b804000000             mov        r8d, 4
0000000140b25048  e8d3ebaeff               call       0x140613c20
0000000140b2504d  e93e010000               jmp        0x140b25190
0000000140b25052  498b4720                 mov        rax, qword ptr [r15 + 0x20]
0000000140b25056  498b4f28                 mov        rcx, qword ptr [r15 + 0x28]
0000000140b2505a  4d8b7738                 mov        r14, qword ptr [r15 + 0x38]
0000000140b2505e  498b5f40                 mov        rbx, qword ptr [r15 + 0x40]
0000000140b25062  498b7750                 mov        rsi, qword ptr [r15 + 0x50]
0000000140b25066  498b5758                 mov        rdx, qword ptr [r15 + 0x58]
0000000140b2506a  48899538010000           mov        qword ptr [rbp + 0x138], rdx
0000000140b25071  660f6f45f0               movdqa     xmm0, xmmword ptr [rbp - 0x10]
0000000140b25076  0f284d00                 movaps     xmm1, xmmword ptr [rbp]
0000000140b2507a  0f285510                 movaps     xmm2, xmmword ptr [rbp + 0x10]
0000000140b2507e  0f285d20                 movaps     xmm3, xmmword ptr [rbp + 0x20]
0000000140b25082  488b5530                 mov        rdx, qword ptr [rbp + 0x30]
0000000140b25086  49895760                 mov        qword ptr [r15 + 0x60], rdx
0000000140b2508a  f3410f7f4720             movdqu     xmmword ptr [r15 + 0x20], xmm0
0000000140b25090  410f114f30               movups     xmmword ptr [r15 + 0x30], xmm1
0000000140b25095  410f115740               movups     xmmword ptr [r15 + 0x40], xmm2
0000000140b2509a  410f115f50               movups     xmmword ptr [r15 + 0x50], xmm3
0000000140b2509f  4889c2                   mov        rdx, rax
0000000140b250a2  48f7da                   neg        rdx
0000000140b250a5  704e                     jo         0x140b250f5
0000000140b250a7  7313                     jae        0x140b250bc
0000000140b250a9  48c1e004                 shl        rax, 4
0000000140b250ad  488d1480                 lea        rdx, [rax + rax*4]
0000000140b250b1  41b804000000             mov        r8d, 4
0000000140b250b7  e864ebaeff               call       0x140613c20
0000000140b250bc  4d85f6                   test       r14, r14
0000000140b250bf  7416                     je         0x140b250d7
0000000140b250c1  49c1e603                 shl        r14, 3
0000000140b250c5  4b8d1476                 lea        rdx, [r14 + r14*2]
0000000140b250c9  41b804000000             mov        r8d, 4
0000000140b250cf  4889d9                   mov        rcx, rbx
0000000140b250d2  e849ebaeff               call       0x140613c20
0000000140b250d7  4885f6                   test       rsi, rsi
0000000140b250da  7419                     je         0x140b250f5
0000000140b250dc  48c1e602                 shl        rsi, 2
0000000140b250e0  41b804000000             mov        r8d, 4
0000000140b250e6  488b8d38010000           mov        rcx, qword ptr [rbp + 0x138]
0000000140b250ed  4889f2                   mov        rdx, rsi
0000000140b250f0  e82bebaeff               call       0x140613c20
0000000140b250f5  41c687a800000000         mov        byte ptr [r15 + 0xa8], 0
0000000140b250fd  4584ed                   test       r13b, r13b
0000000140b25100  488d9d50010000           lea        rbx, [rbp + 0x150]
0000000140b25107  4c8db5f9000000           lea        r14, [rbp + 0xf9]
0000000140b2510e  beffffff3f               mov        esi, 0x3fffffff
0000000140b25113  750f                     jne        0x140b25124
0000000140b25115  488b07                   mov        rax, qword ptr [rdi]
0000000140b25118  48d1e0                   shl        rax, 1
0000000140b2511b  4885c0                   test       rax, rax
0000000140b2511e  0f85a7010000             jne        0x140b252cb
0000000140b25124  ba010000c0               mov        edx, 0xc0000001
0000000140b25129  f0410fc11424             lock xadd  dword ptr [r12], edx
0000000140b2512f  81c2010000c0             add        edx, 0xc0000001
0000000140b25135  81fa00000040             cmp        edx, 0x40000000
0000000140b2513b  0f8320010000             jae        0x140b25261
0000000140b25141  f6850001000001           test       byte ptr [rbp + 0x100], 1
0000000140b25148  7446                     je         0x140b25190
0000000140b2514a  488b8d08010000           mov        rcx, qword ptr [rbp + 0x108]
0000000140b25151  80bd1001000000           cmp        byte ptr [rbp + 0x110], 0
0000000140b25158  750f                     jne        0x140b25169
0000000140b2515a  488b07                   mov        rax, qword ptr [rdi]
0000000140b2515d  48d1e0                   shl        rax, 1
0000000140b25160  4885c0                   test       rax, rax
0000000140b25163  0f8579010000             jne        0x140b252e2
0000000140b25169  ba010000c0               mov        edx, 0xc0000001
0000000140b2516e  f00fc111                 lock xadd  dword ptr [rcx], edx
0000000140b25172  81c2010000c0             add        edx, 0xc0000001
0000000140b25178  81fa00000040             cmp        edx, 0x40000000
0000000140b2517e  0f8318010000             jae        0x140b2529c
0000000140b25184  6666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b25190  488b8550010000           mov        rax, qword ptr [rbp + 0x150]
0000000140b25197  4885c0                   test       rax, rax
0000000140b2519a  741a                     je         0x140b251b6
0000000140b2519c  488b8d58010000           mov        rcx, qword ptr [rbp + 0x158]
0000000140b251a3  48c1e003                 shl        rax, 3
0000000140b251a7  488d1480                 lea        rdx, [rax + rax*4]
0000000140b251ab  41b804000000             mov        r8d, 4
0000000140b251b1  e86aeaaeff               call       0x140613c20
0000000140b251b6  41b820000000             mov        r8d, 0x20
0000000140b251bc  41b910000000             mov        r9d, 0x10
0000000140b251c2  488d8dc0000000           lea        rcx, [rbp + 0xc0]
0000000140b251c9  488d95e0000000           lea        rdx, [rbp + 0xe0]
0000000140b251d0  e80bd352ff               call       0x1400524e0
0000000140b251d5  4c8bad70010000           mov        r13, qword ptr [rbp + 0x170]
0000000140b251dc  e9bffbffff               jmp        0x140b24da0
0000000140b251e1  4c89e1                   mov        rcx, r12
0000000140b251e4  e8273adb01               call       0x1428d8c10
0000000140b251e9  e9c4fbffff               jmp        0x140b24db2
0000000140b251ee  e8bd34db01               call       0x1428d86b0
0000000140b251f3  3401                     xor        al, 1
0000000140b251f5  410fb64d18               movzx      ecx, byte ptr [r13 + 0x18]
0000000140b251fa  84c9                     test       cl, cl
0000000140b251fc  0f84cefbffff             je         0x140b24dd0
0000000140b25202  e903010000               jmp        0x140b2530a
0000000140b25207  4c89e1                   mov        rcx, r12
0000000140b2520a  e8313bdb01               call       0x1428d8d40
0000000140b2520f  e901fcffff               jmp        0x140b24e15
0000000140b25214  4c89e1                   mov        rcx, r12
0000000140b25217  e8243bdb01               call       0x1428d8d40
0000000140b2521c  e999fcffff               jmp        0x140b24eba
0000000140b25221  c6857f01000001           mov        byte ptr [rbp + 0x17f], 1
0000000140b25228  4c89e1                   mov        rcx, r12
0000000140b2522b  e8103bdb01               call       0x1428d8d40
0000000140b25230  e9b8fdffff               jmp        0x140b24fed
0000000140b25235  c6857f01000001           mov        byte ptr [rbp + 0x17f], 1
0000000140b2523c  4c89e1                   mov        rcx, r12
0000000140b2523f  e8cc39db01               call       0x1428d8c10
0000000140b25244  e93afdffff               jmp        0x140b24f83
0000000140b25249  c6857f01000001           mov        byte ptr [rbp + 0x17f], 1
0000000140b25250  e85b34db01               call       0x1428d86b0
0000000140b25255  4189c5                   mov        r13d, eax
0000000140b25258  4180f501                 xor        r13b, 1
0000000140b2525c  e934fdffff               jmp        0x140b24f95
0000000140b25261  4c89e1                   mov        rcx, r12
0000000140b25264  e8d73adb01               call       0x1428d8d40
0000000140b25269  e9d3feffff               jmp        0x140b25141
0000000140b2526e  e83d34db01               call       0x1428d86b0
0000000140b25273  84c0                     test       al, al
0000000140b25275  0f857dfbffff             jne        0x140b24df8
0000000140b2527b  41c6451801               mov        byte ptr [r13 + 0x18], 1
0000000140b25280  e973fbffff               jmp        0x140b24df8
0000000140b25285  e82634db01               call       0x1428d86b0
0000000140b2528a  84c0                     test       al, al
0000000140b2528c  0f850bfcffff             jne        0x140b24e9d
0000000140b25292  41c6451801               mov        byte ptr [r13 + 0x18], 1
0000000140b25297  e901fcffff               jmp        0x140b24e9d
0000000140b2529c  c6857f01000000           mov        byte ptr [rbp + 0x17f], 0
0000000140b252a3  e8983adb01               call       0x1428d8d40
0000000140b252a8  e9e3feffff               jmp        0x140b25190
0000000140b252ad  c6857f01000001           mov        byte ptr [rbp + 0x17f], 1
0000000140b252b4  e8f733db01               call       0x1428d86b0
0000000140b252b9  84c0                     test       al, al
0000000140b252bb  0f850ffdffff             jne        0x140b24fd0
0000000140b252c1  41c6471801               mov        byte ptr [r15 + 0x18], 1
0000000140b252c6  e905fdffff               jmp        0x140b24fd0
0000000140b252cb  e8e033db01               call       0x1428d86b0
0000000140b252d0  84c0                     test       al, al
0000000140b252d2  0f854cfeffff             jne        0x140b25124
0000000140b252d8  41c6471801               mov        byte ptr [r15 + 0x18], 1
0000000140b252dd  e942feffff               jmp        0x140b25124
0000000140b252e2  c6857f01000000           mov        byte ptr [rbp + 0x17f], 0
0000000140b252e9  4889ce                   mov        rsi, rcx
0000000140b252ec  e8bf33db01               call       0x1428d86b0
0000000140b252f1  4889f1                   mov        rcx, rsi
0000000140b252f4  beffffff3f               mov        esi, 0x3fffffff
0000000140b252f9  84c0                     test       al, al
0000000140b252fb  0f8568feffff             jne        0x140b25169
0000000140b25301  c6410801                 mov        byte ptr [rcx + 8], 1
0000000140b25305  e95ffeffff               jmp        0x140b25169
0000000140b2530a  4c89a540010000           mov        qword ptr [rbp + 0x140], r12
0000000140b25311  888548010000             mov        byte ptr [rbp + 0x148], al
0000000140b25317  488b0d8a70b002           mov        rcx, qword ptr [rip + 0x2b0708a]
0000000140b2531e  488b09                   mov        rcx, qword ptr [rcx]
0000000140b25321  4883f903                 cmp        rcx, 3
0000000140b25325  0f82fe000000             jb         0x140b25429
0000000140b2532b  488d8540010000           lea        rax, [rbp + 0x140]
0000000140b25332  48898550010000           mov        qword ptr [rbp + 0x150], rax
0000000140b25339  488d05607e0400           lea        rax, [rip + 0x47e60]
0000000140b25340  48898558010000           mov        qword ptr [rbp + 0x158], rax
0000000140b25347  48c7850001000002000000   mov        qword ptr [rbp + 0x100], 2
0000000140b25352  48c7851001000002000000   mov        qword ptr [rbp + 0x110], 2
0000000140b2535d  48c7852001000000000000   mov        qword ptr [rbp + 0x120], 0
0000000140b25368  48b82000000004000000     movabs     rax, 0x400000020
0000000140b25372  48898528010000           mov        qword ptr [rbp + 0x128], rax
0000000140b25379  c6853001000003           mov        byte ptr [rbp + 0x130], 3
0000000140b25380  488d0d2147fc01           lea        rcx, [rip + 0x1fc4721]
0000000140b25387  e8f489b5ff               call       0x14067dd80
0000000140b2538c  f30f6f00                 movdqu     xmm0, xmmword ptr [rax]
0000000140b25390  8b4010                   mov        eax, dword ptr [rax + 0x10]
0000000140b25393  48c7452003000000         mov        qword ptr [rbp + 0x20], 3
0000000140b2539b  488d0d1e47fc01           lea        rcx, [rip + 0x1fc471e]
0000000140b253a2  48894d28                 mov        qword ptr [rbp + 0x28], rcx
0000000140b253a6  48c7453028000000         mov        qword ptr [rbp + 0x30], 0x28
0000000140b253ae  488d15ab46fc01           lea        rdx, [rip + 0x1fc46ab]
0000000140b253b5  48895540                 mov        qword ptr [rbp + 0x40], rdx
0000000140b253b9  48c7454801000000         mov        qword ptr [rbp + 0x48], 1
0000000140b253c1  48895d50                 mov        qword ptr [rbp + 0x50], rbx
0000000140b253c5  48c7455801000000         mov        qword ptr [rbp + 0x58], 1
0000000140b253cd  488d9500010000           lea        rdx, [rbp + 0x100]
0000000140b253d4  48895560                 mov        qword ptr [rbp + 0x60], rdx
0000000140b253d8  48c7456801000000         mov        qword ptr [rbp + 0x68], 1
0000000140b253e0  48c745f000000000         mov        qword ptr [rbp - 0x10], 0
0000000140b253e8  48894df8                 mov        qword ptr [rbp - 8], rcx
0000000140b253ec  48c7450028000000         mov        qword ptr [rbp], 0x28
0000000140b253f4  48c7450800000000         mov        qword ptr [rbp + 8], 0
0000000140b253fc  f30f7f4510               movdqu     xmmword ptr [rbp + 0x10], xmm0
0000000140b25401  c7453801000000           mov        dword ptr [rbp + 0x38], 1
0000000140b25408  89453c                   mov        dword ptr [rbp + 0x3c], eax
0000000140b2540b  488d8d6f010000           lea        rcx, [rbp + 0x16f]
0000000140b25412  488d55f0                 lea        rdx, [rbp - 0x10]
0000000140b25416  e8154f2200               call       0x140d4a330
0000000140b2541b  4c8ba540010000           mov        r12, qword ptr [rbp + 0x140]
0000000140b25422  0fb68548010000           movzx      eax, byte ptr [rbp + 0x148]
0000000140b25429  a801                     test       al, 1
0000000140b2542b  750b                     jne        0x140b25438
0000000140b2542d  488b07                   mov        rax, qword ptr [rdi]
0000000140b25430  48d1e0                   shl        rax, 1
0000000140b25433  4885c0                   test       rax, rax
0000000140b25436  7551                     jne        0x140b25489
0000000140b25438  ba010000c0               mov        edx, 0xc0000001
0000000140b2543d  f0410fc11424             lock xadd  dword ptr [r12], edx
0000000140b25443  81c2010000c0             add        edx, 0xc0000001
0000000140b25449  81fa00000040             cmp        edx, 0x40000000
0000000140b2544f  732e                     jae        0x140b2547f
0000000140b25451  488b8570010000           mov        rax, qword ptr [rbp + 0x170]
0000000140b25458  f048ff08                 lock dec   qword ptr [rax]
0000000140b2545c  750c                     jne        0x140b2546a
0000000140b2545e  488d8d70010000           lea        rcx, [rbp + 0x170]
0000000140b25465  e836730300               call       0x140b5c7a0
0000000140b2546a  90                       nop        
0000000140b2546b  4881c408020000           add        rsp, 0x208
0000000140b25472  5b                       pop        rbx
0000000140b25473  5f                       pop        rdi
0000000140b25474  5e                       pop        rsi
0000000140b25475  415c                     pop        r12
0000000140b25477  415d                     pop        r13
0000000140b25479  415e                     pop        r14
0000000140b2547b  415f                     pop        r15
0000000140b2547d  5d                       pop        rbp
0000000140b2547e  c3                       ret        
0000000140b2547f  4c89e1                   mov        rcx, r12
0000000140b25482  e8b938db01               call       0x1428d8d40
0000000140b25487  ebc8                     jmp        0x140b25451
0000000140b25489  e82232db01               call       0x1428d86b0
0000000140b2548e  84c0                     test       al, al
0000000140b25490  75a6                     jne        0x140b25438
0000000140b25492  41c644240801             mov        byte ptr [r12 + 8], 1
0000000140b25498  eb9e                     jmp        0x140b25438
0000000140b2549a  660f1f440000             nop        word ptr [rax + rax]
0000000140b254a0  4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b254a5  55                       push       rbp
0000000140b254a6  4157                     push       r15
0000000140b254a8  4156                     push       r14
0000000140b254aa  4155                     push       r13
0000000140b254ac  4154                     push       r12
0000000140b254ae  56                       push       rsi
0000000140b254af  57                       push       rdi
0000000140b254b0  53                       push       rbx
0000000140b254b1  4883ec28                 sub        rsp, 0x28
0000000140b254b5  488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b254bc  488b8570010000           mov        rax, qword ptr [rbp + 0x170]
0000000140b254c3  f048ff08                 lock dec   qword ptr [rax]
0000000140b254c7  750c                     jne        0x140b254d5
0000000140b254c9  488d8d70010000           lea        rcx, [rbp + 0x170]
0000000140b254d0  e8cb720300               call       0x140b5c7a0
0000000140b254d5  90                       nop        
0000000140b254d6  4883c428                 add        rsp, 0x28
0000000140b254da  5b                       pop        rbx
0000000140b254db  5f                       pop        rdi
0000000140b254dc  5e                       pop        rsi
0000000140b254dd  415c                     pop        r12
0000000140b254df  415d                     pop        r13
0000000140b254e1  415e                     pop        r14
0000000140b254e3  415f                     pop        r15
0000000140b254e5  5d                       pop        rbp
0000000140b254e6  c3                       ret        
0000000140b254e7  660f1f840000000000       nop        word ptr [rax + rax]
0000000140b254f0  4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b254f5  55                       push       rbp
0000000140b254f6  4157                     push       r15
0000000140b254f8  4156                     push       r14
0000000140b254fa  4155                     push       r13
0000000140b254fc  4154                     push       r12
0000000140b254fe  56                       push       rsi
0000000140b254ff  57                       push       rdi
0000000140b25500  53                       push       rbx
0000000140b25501  4883ec28                 sub        rsp, 0x28
0000000140b25505  488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2550c  488b8550010000           mov        rax, qword ptr [rbp + 0x150]
0000000140b25513  4885c0                   test       rax, rax
0000000140b25516  741a                     je         0x140b25532
0000000140b25518  488b8d58010000           mov        rcx, qword ptr [rbp + 0x158]
0000000140b2551f  48c1e003                 shl        rax, 3
0000000140b25523  488d1480                 lea        rdx, [rax + rax*4]
0000000140b25527  41b804000000             mov        r8d, 4
0000000140b2552d  e8eee6aeff               call       0x140613c20
0000000140b25532  90                       nop        
0000000140b25533  4883c428                 add        rsp, 0x28
0000000140b25537  5b                       pop        rbx
0000000140b25538  5f                       pop        rdi
0000000140b25539  5e                       pop        rsi
0000000140b2553a  415c                     pop        r12
0000000140b2553c  415d                     pop        r13
0000000140b2553e  415e                     pop        r14
0000000140b25540  415f                     pop        r15
0000000140b25542  5d                       pop        rbp
0000000140b25543  c3                       ret        
0000000140b25544  6666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b25550  4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b25555  55                       push       rbp
0000000140b25556  4157                     push       r15
0000000140b25558  4156                     push       r14
0000000140b2555a  4155                     push       r13
0000000140b2555c  4154                     push       r12
0000000140b2555e  56                       push       rsi
0000000140b2555f  57                       push       rdi
0000000140b25560  53                       push       rbx
0000000140b25561  4883ec28                 sub        rsp, 0x28
0000000140b25565  488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2556c  4883bd0001000000         cmp        qword ptr [rbp + 0x100], 0
0000000140b25574  7424                     je         0x140b2559a
0000000140b25576  488d8d00010000           lea        rcx, [rbp + 0x100]
0000000140b2557d  e87eb7ffff               call       0x140b20d00
0000000140b25582  c6857f01000000           mov        byte ptr [rbp + 0x17f], 0
0000000140b25589  4883c428                 add        rsp, 0x28
0000000140b2558d  5b                       pop        rbx
0000000140b2558e  5f                       pop        rdi
0000000140b2558f  5e                       pop        rsi
0000000140b25590  415c                     pop        r12
0000000140b25592  415d                     pop        r13
0000000140b25594  415e                     pop        r14
0000000140b25596  415f                     pop        r15
0000000140b25598  5d                       pop        rbp
0000000140b25599  c3                       ret        
0000000140b2559a  c6857f01000000           mov        byte ptr [rbp + 0x17f], 0
0000000140b255a1  4883c428                 add        rsp, 0x28
0000000140b255a5  5b                       pop        rbx
0000000140b255a6  5f                       pop        rdi
0000000140b255a7  5e                       pop        rsi
0000000140b255a8  415c                     pop        r12
0000000140b255aa  415d                     pop        r13
0000000140b255ac  415e                     pop        r14
0000000140b255ae  415f                     pop        r15
0000000140b255b0  5d                       pop        rbp
0000000140b255b1  c3                       ret        
0000000140b255b2  66666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b255c0  4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b255c5  55                       push       rbp
0000000140b255c6  4157                     push       r15
0000000140b255c8  4156                     push       r14
0000000140b255ca  4155                     push       r13
0000000140b255cc  4154                     push       r12
0000000140b255ce  56                       push       rsi
0000000140b255cf  57                       push       rdi
0000000140b255d0  53                       push       rbx
0000000140b255d1  4883ec28                 sub        rsp, 0x28
0000000140b255d5  488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b255dc  80bd7f01000000           cmp        byte ptr [rbp + 0x17f], 0
0000000140b255e3  741b                     je         0x140b25600
0000000140b255e5  488d4df0                 lea        rcx, [rbp - 0x10]
0000000140b255e9  e8d25ad6ff               call       0x14088b0c0
0000000140b255ee  90                       nop        
0000000140b255ef  4883c428                 add        rsp, 0x28
0000000140b255f3  5b                       pop        rbx
0000000140b255f4  5f                       pop        rdi
0000000140b255f5  5e                       pop        rsi
0000000140b255f6  415c                     pop        r12
0000000140b255f8  415d                     pop        r13
0000000140b255fa  415e                     pop        r14
0000000140b255fc  415f                     pop        r15
0000000140b255fe  5d                       pop        rbp
0000000140b255ff  c3                       ret        
0000000140b25600  4883c428                 add        rsp, 0x28
0000000140b25604  5b                       pop        rbx
0000000140b25605  5f                       pop        rdi
0000000140b25606  5e                       pop        rsi
0000000140b25607  415c                     pop        r12
0000000140b25609  415d                     pop        r13
0000000140b2560b  415e                     pop        r14
0000000140b2560d  415f                     pop        r15
0000000140b2560f  5d                       pop        rbp
0000000140b25610  c3                       ret        
0000000140b25611  6666666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b25620  4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b25625  55                       push       rbp
0000000140b25626  4157                     push       r15
0000000140b25628  4156                     push       r14
0000000140b2562a  4155                     push       r13
0000000140b2562c  4154                     push       r12
0000000140b2562e  56                       push       rsi
0000000140b2562f  57                       push       rdi
0000000140b25630  53                       push       rbx
0000000140b25631  4883ec28                 sub        rsp, 0x28
0000000140b25635  488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2563c  488d8d40010000           lea        rcx, [rbp + 0x140]
0000000140b25643  e8a8b7aaff               call       0x1405d0df0
0000000140b25648  90                       nop        
0000000140b25649  4883c428                 add        rsp, 0x28
0000000140b2564d  5b                       pop        rbx
0000000140b2564e  5f                       pop        rdi
0000000140b2564f  5e                       pop        rsi
0000000140b25650  415c                     pop        r12
0000000140b25652  415d                     pop        r13
0000000140b25654  415e                     pop        r14
0000000140b25656  415f                     pop        r15
0000000140b25658  5d                       pop        rbp
0000000140b25659  c3                       ret        
0000000140b2565a  660f1f440000             nop        word ptr [rax + rax]
0000000140b25660  4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b25665  55                       push       rbp
0000000140b25666  4157                     push       r15
0000000140b25668  4156                     push       r14
0000000140b2566a  4155                     push       r13
0000000140b2566c  4154                     push       r12
0000000140b2566e  56                       push       rsi
0000000140b2566f  57                       push       rdi
0000000140b25670  53                       push       rbx
0000000140b25671  4883ec28                 sub        rsp, 0x28
0000000140b25675  488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2567c  488d8dc0000000           lea        rcx, [rbp + 0xc0]
0000000140b25683  41b820000000             mov        r8d, 0x20
0000000140b25689  41b910000000             mov        r9d, 0x10
0000000140b2568f  488d95e0000000           lea        rdx, [rbp + 0xe0]
0000000140b25696  e845ce52ff               call       0x1400524e0
0000000140b2569b  90                       nop        
0000000140b2569c  4883c428                 add        rsp, 0x28
0000000140b256a0  5b                       pop        rbx
0000000140b256a1  5f                       pop        rdi
0000000140b256a2  5e                       pop        rsi
0000000140b256a3  415c                     pop        r12
0000000140b256a5  415d                     pop        r13
0000000140b256a7  415e                     pop        r14
0000000140b256a9  415f                     pop        r15
0000000140b256ab  5d                       pop        rbp
0000000140b256ac  c3                       ret        
