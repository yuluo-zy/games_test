00000001408bc650  55                       push       rbp
00000001408bc651  4157                     push       r15
00000001408bc653  4156                     push       r14
00000001408bc655  4155                     push       r13
00000001408bc657  4154                     push       r12
00000001408bc659  56                       push       rsi
00000001408bc65a  57                       push       rdi
00000001408bc65b  53                       push       rbx
00000001408bc65c  4881ec58010000           sub        rsp, 0x158
00000001408bc663  488dac2480000000         lea        rbp, [rsp + 0x80]
00000001408bc66b  48c785d0000000feffffff   mov        qword ptr [rbp + 0xd0], 0xfffffffffffffffe
00000001408bc676  4d89ce                   mov        r14, r9
00000001408bc679  4c89c7                   mov        rdi, r8
00000001408bc67c  488955d8                 mov        qword ptr [rbp - 0x28], rdx
00000001408bc680  4889cb                   mov        rbx, rcx
00000001408bc683  e878a33500               call       0x140c16a00
00000001408bc688  84c0                     test       al, al
00000001408bc68a  7460                     je         0x1408bc6ec
00000001408bc68c  4c8d25663f1e02           lea        r12, [rip + 0x21e3f66]
00000001408bc693  41bf2c000000             mov        r15d, 0x2c
00000001408bc699  ba2c000000               mov        edx, 0x2c
00000001408bc69e  4c89e1                   mov        rcx, r12
00000001408bc6a1  e85ad4ffff               call       0x1408b9b00
00000001408bc6a6  4883f801                 cmp        rax, 1
00000001408bc6aa  0f85b2000000             jne        0x1408bc762
00000001408bc6b0  4989d1                   mov        r9, rdx
00000001408bc6b3  4885d2                   test       rdx, rdx
00000001408bc6b6  743d                     je         0x1408bc6f5
00000001408bc6b8  4983f92c                 cmp        r9, 0x2c
00000001408bc6bc  7335                     jae        0x1408bc6f3
00000001408bc6be  488d05343f1e02           lea        rax, [rip + 0x21e3f34]
00000001408bc6c5  41803c01bf               cmp        byte ptr [r9 + rax], 0xbf
00000001408bc6ca  7f29                     jg         0x1408bc6f5
00000001408bc6cc  488d0525401e02           lea        rax, [rip + 0x21e4025]
00000001408bc6d3  4889442420               mov        qword ptr [rsp + 0x20], rax
00000001408bc6d8  488d0d1a3f1e02           lea        rcx, [rip + 0x21e3f1a]
00000001408bc6df  ba2c000000               mov        edx, 0x2c
00000001408bc6e4  4531c0                   xor        r8d, r8d
00000001408bc6e7  e854d20102               call       0x1428d9940
00000001408bc6ec  31c0                     xor        eax, eax
00000001408bc6ee  e9ca010000               jmp        0x1408bc8bd
00000001408bc6f3  75d7                     jne        0x1408bc6cc
00000001408bc6f5  4c8d25fd3e1e02           lea        r12, [rip + 0x21e3efd]
00000001408bc6fc  4c89e1                   mov        rcx, r12
00000001408bc6ff  4c89ca                   mov        rdx, r9
00000001408bc702  e8f9d3ffff               call       0x1408b9b00
00000001408bc707  4883f801                 cmp        rax, 1
00000001408bc70b  7555                     jne        0x1408bc762
00000001408bc70d  4989d0                   mov        r8, rdx
00000001408bc710  4983c002                 add        r8, 2
00000001408bc714  7439                     je         0x1408bc74f
00000001408bc716  4983f82c                 cmp        r8, 0x2c
00000001408bc71a  7331                     jae        0x1408bc74d
00000001408bc71c  488d05d63e1e02           lea        rax, [rip + 0x21e3ed6]
00000001408bc723  41803c00bf               cmp        byte ptr [r8 + rax], 0xbf
00000001408bc728  7f25                     jg         0x1408bc74f
00000001408bc72a  488d05df3f1e02           lea        rax, [rip + 0x21e3fdf]
00000001408bc731  4889442420               mov        qword ptr [rsp + 0x20], rax
00000001408bc736  488d0dbc3e1e02           lea        rcx, [rip + 0x21e3ebc]
00000001408bc73d  ba2c000000               mov        edx, 0x2c
00000001408bc742  41b92c000000             mov        r9d, 0x2c
00000001408bc748  e8f3d10102               call       0x1428d9940
00000001408bc74d  75db                     jne        0x1408bc72a
00000001408bc74f  41bf2a000000             mov        r15d, 0x2a
00000001408bc755  4929d7                   sub        r15, rdx
00000001408bc758  4c8d259a3e1e02           lea        r12, [rip + 0x21e3e9a]
00000001408bc75f  4d01c4                   add        r12, r8
00000001408bc762  488d0557421e02           lea        rax, [rip + 0x21e4257]
00000001408bc769  4c8d2d25421e02           lea        r13, [rip + 0x21e4225]
00000001408bc770  0fbe48ff                 movsx      ecx, byte ptr [rax - 1]
00000001408bc774  85c9                     test       ecx, ecx
00000001408bc776  7818                     js         0x1408bc790
00000001408bc778  48ffc8                   dec        rax
00000001408bc77b  83f95c                   cmp        ecx, 0x5c
00000001408bc77e  7569                     jne        0x1408bc7e9
00000001408bc780  eb7c                     jmp        0x1408bc7fe
00000001408bc782  66666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
00000001408bc790  0fb650fe                 movzx      edx, byte ptr [rax - 2]
00000001408bc794  80fac0                   cmp        dl, 0xc0
00000001408bc797  7d25                     jge        0x1408bc7be
00000001408bc799  440fb640fd               movzx      r8d, byte ptr [rax - 3]
00000001408bc79e  4180f8c0                 cmp        r8b, 0xc0
00000001408bc7a2  7d23                     jge        0x1408bc7c7
00000001408bc7a4  440fb648fc               movzx      r9d, byte ptr [rax - 4]
00000001408bc7a9  4883c0fc                 add        rax, -4
00000001408bc7ad  4183e107                 and        r9d, 7
00000001408bc7b1  41c1e106                 shl        r9d, 6
00000001408bc7b5  4183e03f                 and        r8d, 0x3f
00000001408bc7b9  4509c8                   or         r8d, r9d
00000001408bc7bc  eb11                     jmp        0x1408bc7cf
00000001408bc7be  4883c0fe                 add        rax, -2
00000001408bc7c2  83e21f                   and        edx, 0x1f
00000001408bc7c5  eb12                     jmp        0x1408bc7d9
00000001408bc7c7  4883c0fd                 add        rax, -3
00000001408bc7cb  4183e00f                 and        r8d, 0xf
00000001408bc7cf  41c1e006                 shl        r8d, 6
00000001408bc7d3  83e23f                   and        edx, 0x3f
00000001408bc7d6  4409c2                   or         edx, r8d
00000001408bc7d9  c1e206                   shl        edx, 6
00000001408bc7dc  80e13f                   and        cl, 0x3f
00000001408bc7df  0fb6c9                   movzx      ecx, cl
00000001408bc7e2  09d1                     or         ecx, edx
00000001408bc7e4  83f95c                   cmp        ecx, 0x5c
00000001408bc7e7  7415                     je         0x1408bc7fe
00000001408bc7e9  83f92f                   cmp        ecx, 0x2f
00000001408bc7ec  7410                     je         0x1408bc7fe
00000001408bc7ee  4c39e8                   cmp        rax, r13
00000001408bc7f1  0f8579ffffff             jne        0x1408bc770
00000001408bc7f7  be2b000000               mov        esi, 0x2b
00000001408bc7fc  eb4f                     jmp        0x1408bc84d
00000001408bc7fe  4c8d2d90411e02           lea        r13, [rip + 0x21e4190]
00000001408bc805  4c29e8                   sub        rax, r13
00000001408bc808  4989c0                   mov        r8, rax
00000001408bc80b  49ffc0                   inc        r8
00000001408bc80e  7432                     je         0x1408bc842
00000001408bc810  4983f82b                 cmp        r8, 0x2b
00000001408bc814  732a                     jae        0x1408bc840
00000001408bc816  43803c28bf               cmp        byte ptr [r8 + r13], 0xbf
00000001408bc81b  7f25                     jg         0x1408bc842
00000001408bc81d  488d05b43e1e02           lea        rax, [rip + 0x21e3eb4]
00000001408bc824  4889442420               mov        qword ptr [rsp + 0x20], rax
00000001408bc829  488d0d65411e02           lea        rcx, [rip + 0x21e4165]
00000001408bc830  ba2b000000               mov        edx, 0x2b
00000001408bc835  41b92b000000             mov        r9d, 0x2b
00000001408bc83b  e800d10102               call       0x1428d9940
00000001408bc840  75db                     jne        0x1408bc81d
00000001408bc842  be2a000000               mov        esi, 0x2a
00000001408bc847  4829c6                   sub        rsi, rax
00000001408bc84a  4d01c5                   add        r13, r8
00000001408bc84d  e8deb83500               call       0x140c18130
00000001408bc852  4889c1                   mov        rcx, rax
00000001408bc855  488b00                   mov        rax, qword ptr [rax]
00000001408bc858  4883f801                 cmp        rax, 1
00000001408bc85c  0f8578050000             jne        0x1408bcdda
00000001408bc862  4883c108                 add        rcx, 8
00000001408bc866  48833900                 cmp        qword ptr [rcx], 0
00000001408bc86a  0f859b050000             jne        0x1408bce0b
00000001408bc870  48c701ffffffff           mov        qword ptr [rcx], 0xffffffffffffffff
00000001408bc877  48898dc8000000           mov        qword ptr [rbp + 0xc8], rcx
00000001408bc87e  4883c108                 add        rcx, 8
00000001408bc882  4889742420               mov        qword ptr [rsp + 0x20], rsi
00000001408bc887  48c744243000000000       mov        qword ptr [rsp + 0x30], 0
00000001408bc890  48c744242801000000       mov        qword ptr [rsp + 0x28], 1
00000001408bc899  4c89e2                   mov        rdx, r12
00000001408bc89c  4d89f8                   mov        r8, r15
00000001408bc89f  4d89e9                   mov        r9, r13
00000001408bc8a2  e899a43500               call       0x140c16d40
00000001408bc8a7  488b8dc8000000           mov        rcx, qword ptr [rbp + 0xc8]
00000001408bc8ae  48ff01                   inc        qword ptr [rcx]
00000001408bc8b1  488985c0000000           mov        qword ptr [rbp + 0xc0], rax
00000001408bc8b8  b801000000               mov        eax, 1
00000001408bc8bd  488985b8000000           mov        qword ptr [rbp + 0xb8], rax
00000001408bc8c4  498b06                   mov        rax, qword ptr [r14]
00000001408bc8c7  498b4e08                 mov        rcx, qword ptr [r14 + 8]
00000001408bc8cb  488b10                   mov        rdx, qword ptr [rax]
00000001408bc8ce  4531c0                   xor        r8d, r8d
00000001408bc8d1  4989d1                   mov        r9, rdx
00000001408bc8d4  4c2b4918                 sub        r9, qword ptr [rcx + 0x18]
00000001408bc8d8  4d0f42c8                 cmovb      r9, r8
00000001408bc8dc  482b5138                 sub        rdx, qword ptr [rcx + 0x38]
00000001408bc8e0  490f42d0                 cmovb      rdx, r8
00000001408bc8e4  4c8b5110                 mov        r10, qword ptr [rcx + 0x10]
00000001408bc8e8  4c8b5930                 mov        r11, qword ptr [rcx + 0x30]
00000001408bc8ec  4929d3                   sub        r11, rdx
00000001408bc8ef  4d0f42d8                 cmovb      r11, r8
00000001408bc8f3  0f97c2                   seta       dl
00000001408bc8f6  4d29ca                   sub        r10, r9
00000001408bc8f9  4d0f42d0                 cmovb      r10, r8
00000001408bc8fd  488b4940                 mov        rcx, qword ptr [rcx + 0x40]
00000001408bc901  410f97c6                 seta       r14b
00000001408bc905  4d01da                   add        r10, r11
00000001408bc908  4989c8                   mov        r8, rcx
00000001408bc90b  4d29d0                   sub        r8, r10
00000001408bc90e  4c8900                   mov        qword ptr [rax], r8
00000001408bc911  4108d6                   or         r14b, dl
00000001408bc914  4180fe01                 cmp        r14b, 1
00000001408bc918  0f859f000000             jne        0x1408bc9bd
00000001408bc91e  488908                   mov        qword ptr [rax], rcx
00000001408bc921  488b0580fad602           mov        rax, qword ptr [rip + 0x2d6fa80]
00000001408bc928  488b00                   mov        rax, qword ptr [rax]
00000001408bc92b  4883f805                 cmp        rax, 5
00000001408bc92f  0f8288000000             jb         0x1408bc9bd
00000001408bc935  488d0dcc401e02           lea        rcx, [rip + 0x21e40cc]
00000001408bc93c  e83f14dcff               call       0x14067dd80
00000001408bc941  f30f6f00                 movdqu     xmm0, xmmword ptr [rax]
00000001408bc945  8b4010                   mov        eax, dword ptr [rax + 0x10]
00000001408bc948  48c7451005000000         mov        qword ptr [rbp + 0x10], 5
00000001408bc950  488d0dc9401e02           lea        rcx, [rip + 0x21e40c9]
00000001408bc957  48894d18                 mov        qword ptr [rbp + 0x18], rcx
00000001408bc95b  48c745201b000000         mov        qword ptr [rbp + 0x20], 0x1b
00000001408bc963  488d158e401e02           lea        rdx, [rip + 0x21e408e]
00000001408bc96a  48895530                 mov        qword ptr [rbp + 0x30], rdx
00000001408bc96e  48c7453801000000         mov        qword ptr [rbp + 0x38], 1
00000001408bc976  48c7454008000000         mov        qword ptr [rbp + 0x40], 8
00000001408bc97e  48c745e000000000         mov        qword ptr [rbp - 0x20], 0
00000001408bc986  0f57c9                   xorps      xmm1, xmm1
00000001408bc989  0f114d48                 movups     xmmword ptr [rbp + 0x48], xmm1
00000001408bc98d  48894de8                 mov        qword ptr [rbp - 0x18], rcx
00000001408bc991  48c745f01b000000         mov        qword ptr [rbp - 0x10], 0x1b
00000001408bc999  48c745f800000000         mov        qword ptr [rbp - 8], 0
00000001408bc9a1  f30f7f4500               movdqu     xmmword ptr [rbp], xmm0
00000001408bc9a6  c7452801000000           mov        dword ptr [rbp + 0x28], 1
00000001408bc9ad  89452c                   mov        dword ptr [rbp + 0x2c], eax
00000001408bc9b0  488d4d60                 lea        rcx, [rbp + 0x60]
00000001408bc9b4  488d55e0                 lea        rdx, [rbp - 0x20]
00000001408bc9b8  e873d94800               call       0x140d4a330
00000001408bc9bd  8b431c                   mov        eax, dword ptr [rbx + 0x1c]
00000001408bc9c0  488b4b10                 mov        rcx, qword ptr [rbx + 0x10]
00000001408bc9c4  8985c8000000             mov        dword ptr [rbp + 0xc8], eax
00000001408bc9ca  4889ce                   mov        rsi, rcx
00000001408bc9cd  8901                     mov        dword ptr [rcx], eax
00000001408bc9cf  488b1b                   mov        rbx, qword ptr [rbx]
00000001408bc9d2  44087338                 or         byte ptr [rbx + 0x38], r14b
00000001408bc9d6  4c8b1f                   mov        r11, qword ptr [rdi]
00000001408bc9d9  488b5708                 mov        rdx, qword ptr [rdi + 8]
00000001408bc9dd  4d8b0b                   mov        r9, qword ptr [r11]
00000001408bc9e0  4531c0                   xor        r8d, r8d
00000001408bc9e3  4c89c9                   mov        rcx, r9
00000001408bc9e6  482b4a18                 sub        rcx, qword ptr [rdx + 0x18]
00000001408bc9ea  490f42c8                 cmovb      rcx, r8
00000001408bc9ee  4c2b4a38                 sub        r9, qword ptr [rdx + 0x38]
00000001408bc9f2  4d0f42c8                 cmovb      r9, r8
00000001408bc9f6  4889c8                   mov        rax, rcx
00000001408bc9f9  48c1e004                 shl        rax, 4
00000001408bc9fd  48034208                 add        rax, qword ptr [rdx + 8]
00000001408bca01  488b7a10                 mov        rdi, qword ptr [rdx + 0x10]
00000001408bca05  4c8b7230                 mov        r14, qword ptr [rdx + 0x30]
00000001408bca09  4829cf                   sub        rdi, rcx
00000001408bca0c  490f42f8                 cmovb      rdi, r8
00000001408bca10  41ba08000000             mov        r10d, 8
00000001408bca16  490f42c2                 cmovb      rax, r10
00000001408bca1a  4c89c9                   mov        rcx, r9
00000001408bca1d  48c1e104                 shl        rcx, 4
00000001408bca21  48034a28                 add        rcx, qword ptr [rdx + 0x28]
00000001408bca25  4d29ce                   sub        r14, r9
00000001408bca28  4d0f42f0                 cmovb      r14, r8
00000001408bca2c  490f42ca                 cmovb      rcx, r10
00000001408bca30  488b5240                 mov        rdx, qword ptr [rdx + 0x40]
00000001408bca34  4c29f2                   sub        rdx, r14
00000001408bca37  4829fa                   sub        rdx, rdi
00000001408bca3a  4d89dd                   mov        r13, r11
00000001408bca3d  498913                   mov        qword ptr [r11], rdx
00000001408bca40  48c1e704                 shl        rdi, 4
00000001408bca44  4801c7                   add        rdi, rax
00000001408bca47  48897dc8                 mov        qword ptr [rbp - 0x38], rdi
00000001408bca4b  49c1e604                 shl        r14, 4
00000001408bca4f  4901ce                   add        r14, rcx
00000001408bca52  4c8975d0                 mov        qword ptr [rbp - 0x30], r14
00000001408bca56  488d7b20                 lea        rdi, [rbx + 0x20]
00000001408bca5a  4c8d7560                 lea        r14, [rbp + 0x60]
00000001408bca5e  eb16                     jmp        0x1408bca76
00000001408bca60  c6433801                 mov        byte ptr [rbx + 0x38], 1
00000001408bca64  8b85c8000000             mov        eax, dword ptr [rbp + 0xc8]
00000001408bca6a  8906                     mov        dword ptr [rsi], eax
00000001408bca6c  c6433801                 mov        byte ptr [rbx + 0x38], 1
00000001408bca70  4c89e0                   mov        rax, r12
00000001408bca73  4c89f9                   mov        rcx, r15
00000001408bca76  4885c0                   test       rax, rax
00000001408bca79  7415                     je         0x1408bca90
00000001408bca7b  483b45c8                 cmp        rax, qword ptr [rbp - 0x38]
00000001408bca7f  740f                     je         0x1408bca90
00000001408bca81  4c8d6010                 lea        r12, [rax + 0x10]
00000001408bca85  4989cf                   mov        r15, rcx
00000001408bca88  eb23                     jmp        0x1408bcaad
00000001408bca8a  660f1f440000             nop        word ptr [rax + rax]
00000001408bca90  4885c9                   test       rcx, rcx
00000001408bca93  0f848e000000             je         0x1408bcb27
00000001408bca99  483b4dd0                 cmp        rcx, qword ptr [rbp - 0x30]
00000001408bca9d  0f8484000000             je         0x1408bcb27
00000001408bcaa3  4c8d7910                 lea        r15, [rcx + 0x10]
00000001408bcaa7  4531e4                   xor        r12d, r12d
00000001408bcaaa  4889c8                   mov        rax, rcx
00000001408bcaad  49ff4500                 inc        qword ptr [r13]
00000001408bcab1  8b8dc8000000             mov        ecx, dword ptr [rbp + 0xc8]
00000001408bcab7  890e                     mov        dword ptr [rsi], ecx
00000001408bcab9  488b4008                 mov        rax, qword ptr [rax + 8]
00000001408bcabd  48c7456000000000         mov        qword ptr [rbp + 0x60], 0
00000001408bcac5  48894568                 mov        qword ptr [rbp + 0x68], rax
00000001408bcac9  4889f9                   mov        rcx, rdi
00000001408bcacc  4c89f2                   mov        rdx, r14
00000001408bcacf  e8dc49cfff               call       0x1405b14b0
00000001408bcad4  488d4de0                 lea        rcx, [rbp - 0x20]
00000001408bcad8  4889da                   mov        rdx, rbx
00000001408bcadb  4989c0                   mov        r8, rax
00000001408bcade  4d89f1                   mov        r9, r14
00000001408bcae1  e86aba2b00               call       0x140b78550
00000001408bcae6  48837de003               cmp        qword ptr [rbp - 0x20], 3
00000001408bcaeb  b802000000               mov        eax, 2
00000001408bcaf0  7403                     je         0x1408bcaf5
00000001408bcaf2  8b45f8                   mov        eax, dword ptr [rbp - 8]
00000001408bcaf5  488b4df0                 mov        rcx, qword ptr [rbp - 0x10]
00000001408bcaf9  48898da8000000           mov        qword ptr [rbp + 0xa8], rcx
00000001408bcb00  8885b0000000             mov        byte ptr [rbp + 0xb0], al
00000001408bcb06  0f8458ffffff             je         0x1408bca64
00000001408bcb0c  f048ff09                 lock dec   qword ptr [rcx]
00000001408bcb10  0f854affffff             jne        0x1408bca60
00000001408bcb16  488d8da8000000           lea        rcx, [rbp + 0xa8]
00000001408bcb1d  e8aeff2900               call       0x140b5cad0
00000001408bcb22  e939ffffff               jmp        0x1408bca60
00000001408bcb27  807b3800                 cmp        byte ptr [rbx + 0x38], 0
00000001408bcb2b  0f84f9010000             je         0x1408bcd2a
00000001408bcb31  488b45d8                 mov        rax, qword ptr [rbp - 0x28]
00000001408bcb35  488b00                   mov        rax, qword ptr [rax]
00000001408bcb38  4c8b38                   mov        r15, qword ptr [rax]
00000001408bcb3b  4d8d7710                 lea        r14, [r15 + 0x10]
00000001408bcb3f  b9ffffff3f               mov        ecx, 0x3fffffff
00000001408bcb44  31c0                     xor        eax, eax
00000001408bcb46  f0410fb14f10             lock cmpxchg dword ptr [r15 + 0x10], ecx
00000001408bcb4c  0f85c5020000             jne        0x1408bce17
00000001408bcb52  4c8b25c7f8d602           mov        r12, qword ptr [rip + 0x2d6f8c7]
00000001408bcb59  498b0424                 mov        rax, qword ptr [r12]
00000001408bcb5d  48d1e0                   shl        rax, 1
00000001408bcb60  4885c0                   test       rax, rax
00000001408bcb63  0f85bb020000             jne        0x1408bce24
00000001408bcb69  31ff                     xor        edi, edi
00000001408bcb6b  410fb64718               movzx      eax, byte ptr [r15 + 0x18]
00000001408bcb70  84c0                     test       al, al
00000001408bcb72  0f85c4020000             jne        0x1408bce3c
00000001408bcb78  4c89b5a8000000           mov        qword ptr [rbp + 0xa8], r14
00000001408bcb7f  4088bdb0000000           mov        byte ptr [rbp + 0xb0], dil
00000001408bcb86  8b85c8000000             mov        eax, dword ptr [rbp + 0xc8]
00000001408bcb8c  8906                     mov        dword ptr [rsi], eax
00000001408bcb8e  408a7338                 mov        sil, byte ptr [rbx + 0x38]
00000001408bcb92  c6433800                 mov        byte ptr [rbx + 0x38], 0
00000001408bcb96  488b03                   mov        rax, qword ptr [rbx]
00000001408bcb99  488b4b08                 mov        rcx, qword ptr [rbx + 8]
00000001408bcb9d  488d4c0801               lea        rcx, [rax + rcx + 1]
00000001408bcba2  660f6f00                 movdqa     xmm0, xmmword ptr [rax]
00000001408bcba6  660fd7d0                 pmovmskb   edx, xmm0
00000001408bcbaa  f7d2                     not        edx
00000001408bcbac  66895578                 mov        word ptr [rbp + 0x78], dx
00000001408bcbb0  48894560                 mov        qword ptr [rbp + 0x60], rax
00000001408bcbb4  4883c010                 add        rax, 0x10
00000001408bcbb8  48894568                 mov        qword ptr [rbp + 0x68], rax
00000001408bcbbc  48894d70                 mov        qword ptr [rbp + 0x70], rcx
00000001408bcbc0  488b4318                 mov        rax, qword ptr [rbx + 0x18]
00000001408bcbc4  48898580000000           mov        qword ptr [rbp + 0x80], rax
00000001408bcbcb  488d4de0                 lea        rcx, [rbp - 0x20]
00000001408bcbcf  488d5560                 lea        rdx, [rbp + 0x60]
00000001408bcbd3  e8b8cbb800               call       0x141449790
00000001408bcbd8  488b4330                 mov        rax, qword ptr [rbx + 0x30]
00000001408bcbdc  40887518                 mov        byte ptr [rbp + 0x18], sil
00000001408bcbe0  48894510                 mov        qword ptr [rbp + 0x10], rax
00000001408bcbe4  410f104768               movups     xmm0, xmmword ptr [r15 + 0x68]
00000001408bcbe9  410f104f78               movups     xmm1, xmmword ptr [r15 + 0x78]
00000001408bcbee  410f109788000000         movups     xmm2, xmmword ptr [r15 + 0x88]
00000001408bcbf6  410f109f98000000         movups     xmm3, xmmword ptr [r15 + 0x98]
00000001408bcbfe  0f299d90000000           movaps     xmmword ptr [rbp + 0x90], xmm3
00000001408bcc05  0f299580000000           movaps     xmmword ptr [rbp + 0x80], xmm2
00000001408bcc0c  0f294d70                 movaps     xmmword ptr [rbp + 0x70], xmm1
00000001408bcc10  0f294560                 movaps     xmmword ptr [rbp + 0x60], xmm0
00000001408bcc14  f30f6f45e0               movdqu     xmm0, xmmword ptr [rbp - 0x20]
00000001408bcc19  0f104df0                 movups     xmm1, xmmword ptr [rbp - 0x10]
00000001408bcc1d  0f105500                 movups     xmm2, xmmword ptr [rbp]
00000001408bcc21  410f119788000000         movups     xmmword ptr [r15 + 0x88], xmm2
00000001408bcc29  488b4510                 mov        rax, qword ptr [rbp + 0x10]
00000001408bcc2d  49898798000000           mov        qword ptr [r15 + 0x98], rax
00000001408bcc34  0fb64518                 movzx      eax, byte ptr [rbp + 0x18]
00000001408bcc38  418887a0000000           mov        byte ptr [r15 + 0xa0], al
00000001408bcc3f  8b4519                   mov        eax, dword ptr [rbp + 0x19]
00000001408bcc42  418987a1000000           mov        dword ptr [r15 + 0xa1], eax
00000001408bcc49  0fb7451d                 movzx      eax, word ptr [rbp + 0x1d]
00000001408bcc4d  66418987a5000000         mov        word ptr [r15 + 0xa5], ax
00000001408bcc55  0fb6451f                 movzx      eax, byte ptr [rbp + 0x1f]
00000001408bcc59  418887a7000000           mov        byte ptr [r15 + 0xa7], al
00000001408bcc60  f3410f7f4768             movdqu     xmmword ptr [r15 + 0x68], xmm0
00000001408bcc66  410f114f78               movups     xmmword ptr [r15 + 0x78], xmm1
00000001408bcc6b  80bd9800000002           cmp        byte ptr [rbp + 0x98], 2
00000001408bcc72  741c                     je         0x1408bcc90
00000001408bcc74  488d9580000000           lea        rdx, [rbp + 0x80]
00000001408bcc7b  488d4d60                 lea        rcx, [rbp + 0x60]
00000001408bcc7f  41b820000000             mov        r8d, 0x20
00000001408bcc85  41b910000000             mov        r9d, 0x10
00000001408bcc8b  e8505879ff               call       0x1400524e0
00000001408bcc90  4084ff                   test       dil, dil
00000001408bcc93  7510                     jne        0x1408bcca5
00000001408bcc95  498b0424                 mov        rax, qword ptr [r12]
00000001408bcc99  48d1e0                   shl        rax, 1
00000001408bcc9c  4885c0                   test       rax, rax
00000001408bcc9f  0f85c9010000             jne        0x1408bce6e
00000001408bcca5  ba010000c0               mov        edx, 0xc0000001
00000001408bccaa  f0410fc116               lock xadd  dword ptr [r14], edx
00000001408bccaf  81c2010000c0             add        edx, 0xc0000001
00000001408bccb5  81fa00000040             cmp        edx, 0x40000000
00000001408bccbb  0f833d010000             jae        0x1408bcdfe
00000001408bccc1  4883bdb800000000         cmp        qword ptr [rbp + 0xb8], 0
00000001408bccc9  744b                     je         0x1408bcd16
00000001408bcccb  e860b43500               call       0x140c18130
00000001408bccd0  4889c1                   mov        rcx, rax
00000001408bccd3  488b00                   mov        rax, qword ptr [rax]
00000001408bccd6  4883f801                 cmp        rax, 1
00000001408bccda  0f85e0000000             jne        0x1408bcdc0
00000001408bcce0  4883c108                 add        rcx, 8
00000001408bcce4  48833900                 cmp        qword ptr [rcx], 0
00000001408bcce8  0f851d010000             jne        0x1408bce0b
00000001408bccee  48c701ffffffff           mov        qword ptr [rcx], 0xffffffffffffffff
00000001408bccf5  48898dc8000000           mov        qword ptr [rbp + 0xc8], rcx
00000001408bccfc  4883c108                 add        rcx, 8
00000001408bcd00  488b95c0000000           mov        rdx, qword ptr [rbp + 0xc0]
00000001408bcd07  e8e4a33500               call       0x140c170f0
00000001408bcd0c  488b85c8000000           mov        rax, qword ptr [rbp + 0xc8]
00000001408bcd13  48ff00                   inc        qword ptr [rax]
00000001408bcd16  4881c458010000           add        rsp, 0x158
00000001408bcd1d  5b                       pop        rbx
00000001408bcd1e  5f                       pop        rdi
00000001408bcd1f  5e                       pop        rsi
00000001408bcd20  415c                     pop        r12
00000001408bcd22  415d                     pop        r13
00000001408bcd24  415e                     pop        r14
00000001408bcd26  415f                     pop        r15
00000001408bcd28  5d                       pop        rbp
00000001408bcd29  c3                       ret        
00000001408bcd2a  488b03                   mov        rax, qword ptr [rbx]
00000001408bcd2d  488b4b18                 mov        rcx, qword ptr [rbx + 0x18]
00000001408bcd31  660f6f00                 movdqa     xmm0, xmmword ptr [rax]
00000001408bcd35  66440fd7c0               pmovmskb   r8d, xmm0
00000001408bcd3a  41f7d0                   not        r8d
00000001408bcd3d  488d5010                 lea        rdx, [rax + 0x10]
00000001408bcd41  eb31                     jmp        0x1408bcd74
00000001408bcd43  666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
00000001408bcd50  458d48ff                 lea        r9d, [r8 - 1]
00000001408bcd54  4521c1                   and        r9d, r8d
00000001408bcd57  f3450fbcc0               tzcnt      r8d, r8d
00000001408bcd5c  41c1e005                 shl        r8d, 5
00000001408bcd60  4989c2                   mov        r10, rax
00000001408bcd63  4d29c2                   sub        r10, r8
00000001408bcd66  41807af800               cmp        byte ptr [r10 - 8], 0
00000001408bcd6b  4589c8                   mov        r8d, r9d
00000001408bcd6e  0f85bdfdffff             jne        0x1408bcb31
00000001408bcd74  4883e901                 sub        rcx, 1
00000001408bcd78  0f8243ffffff             jb         0x1408bccc1
00000001408bcd7e  664585c0                 test       r8w, r8w
00000001408bcd82  75cc                     jne        0x1408bcd50
00000001408bcd84  6666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
00000001408bcd90  660f6f02                 movdqa     xmm0, xmmword ptr [rdx]
00000001408bcd94  66440fd7c0               pmovmskb   r8d, xmm0
00000001408bcd99  480500feffff             add        rax, -0x200
00000001408bcd9f  4883c210                 add        rdx, 0x10
00000001408bcda3  4181f8ffff0000           cmp        r8d, 0xffff
00000001408bcdaa  74e4                     je         0x1408bcd90
00000001408bcdac  41bafeffffff             mov        r10d, 0xfffffffe
00000001408bcdb2  4529c2                   sub        r10d, r8d
00000001408bcdb5  41f7d0                   not        r8d
00000001408bcdb8  4589c1                   mov        r9d, r8d
00000001408bcdbb  4521d1                   and        r9d, r10d
00000001408bcdbe  eb97                     jmp        0x1408bcd57
00000001408bcdc0  4885c0                   test       rax, rax
00000001408bcdc3  752d                     jne        0x1408bcdf2
00000001408bcdc5  31d2                     xor        edx, edx
00000001408bcdc7  e804ba0302               call       0x1428f87d0
00000001408bcdcc  4889c1                   mov        rcx, rax
00000001408bcdcf  4885c0                   test       rax, rax
00000001408bcdd2  0f850cffffff             jne        0x1408bcce4
00000001408bcdd8  eb18                     jmp        0x1408bcdf2
00000001408bcdda  4885c0                   test       rax, rax
00000001408bcddd  7513                     jne        0x1408bcdf2
00000001408bcddf  31d2                     xor        edx, edx
00000001408bcde1  e8eab90302               call       0x1428f87d0
00000001408bcde6  4889c1                   mov        rcx, rax
00000001408bcde9  4885c0                   test       rax, rax
00000001408bcdec  0f8574faffff             jne        0x1408bc866
00000001408bcdf2  488d0da7dc3702           lea        rcx, [rip + 0x237dca7]
00000001408bcdf9  e8d2b20102               call       0x1428d80d0
00000001408bcdfe  4c89f1                   mov        rcx, r14
00000001408bce01  e83abf0102               call       0x1428d8d40
00000001408bce06  e9b6feffff               jmp        0x1408bccc1
00000001408bce0b  488d0d5edd3702           lea        rcx, [rip + 0x237dd5e]
00000001408bce12  e839c40102               call       0x1428d9250
00000001408bce17  4c89f1                   mov        rcx, r14
00000001408bce1a  e8f1bd0102               call       0x1428d8c10
00000001408bce1f  e92efdffff               jmp        0x1408bcb52
00000001408bce24  e887b80102               call       0x1428d86b0
00000001408bce29  89c7                     mov        edi, eax
00000001408bce2b  4080f701                 xor        dil, 1
00000001408bce2f  410fb64718               movzx      eax, byte ptr [r15 + 0x18]
00000001408bce34  84c0                     test       al, al
00000001408bce36  0f843cfdffff             je         0x1408bcb78
00000001408bce3c  4c8975e0                 mov        qword ptr [rbp - 0x20], r14
00000001408bce40  40887de8                 mov        byte ptr [rbp - 0x18], dil
00000001408bce44  488d05f53b1e02           lea        rax, [rip + 0x21e3bf5]
00000001408bce4b  4889442420               mov        qword ptr [rsp + 0x20], rax
00000001408bce50  488d0d61351e02           lea        rcx, [rip + 0x21e3561]
00000001408bce57  4c8d0d2a361e02           lea        r9, [rip + 0x21e362a]
00000001408bce5e  4c8d45e0                 lea        r8, [rbp - 0x20]
00000001408bce62  ba2b000000               mov        edx, 0x2b
00000001408bce67  e8f4c80102               call       0x1428d9760
00000001408bce6c  0f0b                     ud2        
00000001408bce6e  e83db80102               call       0x1428d86b0
00000001408bce73  84c0                     test       al, al
00000001408bce75  0f852afeffff             jne        0x1408bcca5
00000001408bce7b  41c6471801               mov        byte ptr [r15 + 0x18], 1
00000001408bce80  e920feffff               jmp        0x1408bcca5
00000001408bce85  66662e0f1f840000000000   nop        word ptr cs:[rax + rax]
00000001408bce90  4889542410               mov        qword ptr [rsp + 0x10], rdx
00000001408bce95  55                       push       rbp
00000001408bce96  4157                     push       r15
00000001408bce98  4156                     push       r14
00000001408bce9a  4155                     push       r13
00000001408bce9c  4154                     push       r12
00000001408bce9e  56                       push       rsi
00000001408bce9f  57                       push       rdi
00000001408bcea0  53                       push       rbx
00000001408bcea1  4883ec38                 sub        rsp, 0x38
00000001408bcea5  488daa80000000           lea        rbp, [rdx + 0x80]
00000001408bceac  488b85c8000000           mov        rax, qword ptr [rbp + 0xc8]
00000001408bceb3  48ff00                   inc        qword ptr [rax]
00000001408bceb6  4883c438                 add        rsp, 0x38
00000001408bceba  5b                       pop        rbx
00000001408bcebb  5f                       pop        rdi
00000001408bcebc  5e                       pop        rsi
00000001408bcebd  415c                     pop        r12
00000001408bcebf  415d                     pop        r13
00000001408bcec1  415e                     pop        r14
00000001408bcec3  415f                     pop        r15
00000001408bcec5  5d                       pop        rbp
00000001408bcec6  c3                       ret        
00000001408bcec7  660f1f840000000000       nop        word ptr [rax + rax]
00000001408bced0  4889542410               mov        qword ptr [rsp + 0x10], rdx
00000001408bced5  55                       push       rbp
00000001408bced6  4157                     push       r15
00000001408bced8  4156                     push       r14
00000001408bceda  4155                     push       r13
00000001408bcedc  4154                     push       r12
00000001408bcede  56                       push       rsi
00000001408bcedf  57                       push       rdi
00000001408bcee0  53                       push       rbx
00000001408bcee1  4883ec38                 sub        rsp, 0x38
00000001408bcee5  488daa80000000           lea        rbp, [rdx + 0x80]
00000001408bceec  4883bdb800000000         cmp        qword ptr [rbp + 0xb8], 0
00000001408bcef4  7423                     je         0x1408bcf19
00000001408bcef6  488d85c0000000           lea        rax, [rbp + 0xc0]
00000001408bcefd  488945c0                 mov        qword ptr [rbp - 0x40], rax
00000001408bcf01  488d45c0                 lea        rax, [rbp - 0x40]
00000001408bcf05  488945b8                 mov        qword ptr [rbp - 0x48], rax
00000001408bcf09  488d0d68371e02           lea        rcx, [rip + 0x21e3768]
00000001408bcf10  488d55b8                 lea        rdx, [rbp - 0x48]
00000001408bcf14  e8b723a700               call       0x14132f2d0
00000001408bcf19  90                       nop        
00000001408bcf1a  4883c438                 add        rsp, 0x38
00000001408bcf1e  5b                       pop        rbx
00000001408bcf1f  5f                       pop        rdi
00000001408bcf20  5e                       pop        rsi
00000001408bcf21  415c                     pop        r12
00000001408bcf23  415d                     pop        r13
00000001408bcf25  415e                     pop        r14
00000001408bcf27  415f                     pop        r15
00000001408bcf29  5d                       pop        rbp
00000001408bcf2a  c3                       ret        
00000001408bcf2b  0f1f440000               nop        dword ptr [rax + rax]
00000001408bcf30  4889542410               mov        qword ptr [rsp + 0x10], rdx
00000001408bcf35  55                       push       rbp
00000001408bcf36  4157                     push       r15
00000001408bcf38  4156                     push       r14
00000001408bcf3a  4155                     push       r13
00000001408bcf3c  4154                     push       r12
00000001408bcf3e  56                       push       rsi
00000001408bcf3f  57                       push       rdi
00000001408bcf40  53                       push       rbx
00000001408bcf41  4883ec38                 sub        rsp, 0x38
00000001408bcf45  488daa80000000           lea        rbp, [rdx + 0x80]
00000001408bcf4c  488b85c8000000           mov        rax, qword ptr [rbp + 0xc8]
00000001408bcf53  48ff00                   inc        qword ptr [rax]
00000001408bcf56  4883c438                 add        rsp, 0x38
00000001408bcf5a  5b                       pop        rbx
00000001408bcf5b  5f                       pop        rdi
00000001408bcf5c  5e                       pop        rsi
00000001408bcf5d  415c                     pop        r12
00000001408bcf5f  415d                     pop        r13
00000001408bcf61  415e                     pop        r14
00000001408bcf63  415f                     pop        r15
00000001408bcf65  5d                       pop        rbp
00000001408bcf66  c3                       ret        
00000001408bcf67  660f1f840000000000       nop        word ptr [rax + rax]
00000001408bcf70  4889542410               mov        qword ptr [rsp + 0x10], rdx
00000001408bcf75  55                       push       rbp
00000001408bcf76  4157                     push       r15
00000001408bcf78  4156                     push       r14
00000001408bcf7a  4155                     push       r13
00000001408bcf7c  4154                     push       r12
00000001408bcf7e  56                       push       rsi
00000001408bcf7f  57                       push       rdi
00000001408bcf80  53                       push       rbx
00000001408bcf81  4883ec38                 sub        rsp, 0x38
00000001408bcf85  488daa80000000           lea        rbp, [rdx + 0x80]
00000001408bcf8c  488d8da8000000           lea        rcx, [rbp + 0xa8]
00000001408bcf93  e8583ed1ff               call       0x1405d0df0
00000001408bcf98  90                       nop        
00000001408bcf99  4883c438                 add        rsp, 0x38
00000001408bcf9d  5b                       pop        rbx
00000001408bcf9e  5f                       pop        rdi
00000001408bcf9f  5e                       pop        rsi
00000001408bcfa0  415c                     pop        r12
00000001408bcfa2  415d                     pop        r13
00000001408bcfa4  415e                     pop        r14
00000001408bcfa6  415f                     pop        r15
00000001408bcfa8  5d                       pop        rbp
00000001408bcfa9  c3                       ret        
00000001408bcfaa  660f1f440000             nop        word ptr [rax + rax]
00000001408bcfb0  4889542410               mov        qword ptr [rsp + 0x10], rdx
00000001408bcfb5  55                       push       rbp
00000001408bcfb6  4157                     push       r15
00000001408bcfb8  4156                     push       r14
00000001408bcfba  4155                     push       r13
00000001408bcfbc  4154                     push       r12
00000001408bcfbe  56                       push       rsi
00000001408bcfbf  57                       push       rdi
00000001408bcfc0  53                       push       rbx
00000001408bcfc1  4883ec38                 sub        rsp, 0x38
00000001408bcfc5  488daa80000000           lea        rbp, [rdx + 0x80]
00000001408bcfcc  488d4de0                 lea        rcx, [rbp - 0x20]
00000001408bcfd0  e81b3ed1ff               call       0x1405d0df0
00000001408bcfd5  90                       nop        
00000001408bcfd6  4883c438                 add        rsp, 0x38
00000001408bcfda  5b                       pop        rbx
00000001408bcfdb  5f                       pop        rdi
00000001408bcfdc  5e                       pop        rsi
00000001408bcfdd  415c                     pop        r12
00000001408bcfdf  415d                     pop        r13
00000001408bcfe1  415e                     pop        r14
00000001408bcfe3  415f                     pop        r15
00000001408bcfe5  5d                       pop        rbp
00000001408bcfe6  c3                       ret        
