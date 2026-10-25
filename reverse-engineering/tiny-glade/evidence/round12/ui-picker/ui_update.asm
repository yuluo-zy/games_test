0000000140b2de60: 55                       push       rbp
0000000140b2de61: 4157                     push       r15
0000000140b2de63: 4156                     push       r14
0000000140b2de65: 4155                     push       r13
0000000140b2de67: 4154                     push       r12
0000000140b2de69: 56                       push       rsi
0000000140b2de6a: 57                       push       rdi
0000000140b2de6b: 53                       push       rbx
0000000140b2de6c: 4881ec98010000           sub        rsp, 0x198
0000000140b2de73: 488dac2480000000         lea        rbp, [rsp + 0x80]
0000000140b2de7b: 0f29bd00010000           movaps     xmmword ptr [rbp + 0x100], xmm7
0000000140b2de82: 0f29b5f0000000           movaps     xmmword ptr [rbp + 0xf0], xmm6
0000000140b2de89: 48c785e8000000feffffff   mov        qword ptr [rbp + 0xe8], 0xfffffffffffffffe
0000000140b2de94: 4d89ce                   mov        r14, r9
0000000140b2de97: 4c89c3                   mov        rbx, r8
0000000140b2de9a: 4889d7                   mov        rdi, rdx
0000000140b2de9d: 4889ce                   mov        rsi, rcx
0000000140b2dea0: e85b8b0e00               call       0x140c16a00
0000000140b2dea5: 84c0                     test       al, al
0000000140b2dea7: 488975c8                 mov        qword ptr [rbp - 0x38], rsi
0000000140b2deab: 4889bdc0000000           mov        qword ptr [rbp + 0xc0], rdi
0000000140b2deb2: 7460                     je         0x140b2df14
0000000140b2deb4: 4c8d2579d4fb01           lea        r12, [rip + 0x1fbd479]
0000000140b2debb: 41bf2f000000             mov        r15d, 0x2f
0000000140b2dec1: ba2f000000               mov        edx, 0x2f
0000000140b2dec6: 4c89e1                   mov        rcx, r12
0000000140b2dec9: e8a2cfffff               call       0x140b2ae70
0000000140b2dece: 4883f801                 cmp        rax, 1
0000000140b2ded2: 0f85b2000000             jne        0x140b2df8a
0000000140b2ded8: 4989d1                   mov        r9, rdx
0000000140b2dedb: 4885d2                   test       rdx, rdx
0000000140b2dede: 743d                     je         0x140b2df1d
0000000140b2dee0: 4983f92f                 cmp        r9, 0x2f
0000000140b2dee4: 7335                     jae        0x140b2df1b
0000000140b2dee6: 488d0547d4fb01           lea        rax, [rip + 0x1fbd447]
0000000140b2deed: 41803c01bf               cmp        byte ptr [r9 + rax], 0xbf
0000000140b2def2: 7f29                     jg         0x140b2df1d
0000000140b2def4: 488d05edd4fb01           lea        rax, [rip + 0x1fbd4ed]
0000000140b2defb: 4889442420               mov        qword ptr [rsp + 0x20], rax
0000000140b2df00: 488d0d2dd4fb01           lea        rcx, [rip + 0x1fbd42d]
0000000140b2df07: ba2f000000               mov        edx, 0x2f
0000000140b2df0c: 4531c0                   xor        r8d, r8d
0000000140b2df0f: e82cbada01               call       0x1428d9940
0000000140b2df14: 31c0                     xor        eax, eax
0000000140b2df16: e9cf010000               jmp        0x140b2e0ea
0000000140b2df1b: 75d7                     jne        0x140b2def4
0000000140b2df1d: 4c8d2510d4fb01           lea        r12, [rip + 0x1fbd410]
0000000140b2df24: 4c89e1                   mov        rcx, r12
0000000140b2df27: 4c89ca                   mov        rdx, r9
0000000140b2df2a: e841cfffff               call       0x140b2ae70
0000000140b2df2f: 4883f801                 cmp        rax, 1
0000000140b2df33: 7555                     jne        0x140b2df8a
0000000140b2df35: 4989d0                   mov        r8, rdx
0000000140b2df38: 4983c002                 add        r8, 2
0000000140b2df3c: 7439                     je         0x140b2df77
0000000140b2df3e: 4983f82f                 cmp        r8, 0x2f
0000000140b2df42: 7331                     jae        0x140b2df75
0000000140b2df44: 488d05e9d3fb01           lea        rax, [rip + 0x1fbd3e9]
0000000140b2df4b: 41803c00bf               cmp        byte ptr [r8 + rax], 0xbf
0000000140b2df50: 7f25                     jg         0x140b2df77
0000000140b2df52: 488d05a7d4fb01           lea        rax, [rip + 0x1fbd4a7]
0000000140b2df59: 4889442420               mov        qword ptr [rsp + 0x20], rax
0000000140b2df5e: 488d0dcfd3fb01           lea        rcx, [rip + 0x1fbd3cf]
0000000140b2df65: ba2f000000               mov        edx, 0x2f
0000000140b2df6a: 41b92f000000             mov        r9d, 0x2f
0000000140b2df70: e8cbb9da01               call       0x1428d9940
0000000140b2df75: 75db                     jne        0x140b2df52
0000000140b2df77: 41bf2d000000             mov        r15d, 0x2d
0000000140b2df7d: 4929d7                   sub        r15, rdx
0000000140b2df80: 4c8d25add3fb01           lea        r12, [rip + 0x1fbd3ad]
0000000140b2df87: 4d01c4                   add        r12, r8
0000000140b2df8a: 488d052ed9fb01           lea        rax, [rip + 0x1fbd92e]
0000000140b2df91: 4c8d2df8d8fb01           lea        r13, [rip + 0x1fbd8f8]
0000000140b2df98: 0f1f840000000000         nop        dword ptr [rax + rax]
0000000140b2dfa0: 0fbe48ff                 movsx      ecx, byte ptr [rax - 1]
0000000140b2dfa4: 85c9                     test       ecx, ecx
0000000140b2dfa6: 7818                     js         0x140b2dfc0
0000000140b2dfa8: 48ffc8                   dec        rax
0000000140b2dfab: 83f95c                   cmp        ecx, 0x5c
0000000140b2dfae: 7569                     jne        0x140b2e019
0000000140b2dfb0: eb7c                     jmp        0x140b2e02e
0000000140b2dfb2: 66666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b2dfc0: 0fb650fe                 movzx      edx, byte ptr [rax - 2]
0000000140b2dfc4: 80fac0                   cmp        dl, 0xc0
0000000140b2dfc7: 7d25                     jge        0x140b2dfee
0000000140b2dfc9: 440fb640fd               movzx      r8d, byte ptr [rax - 3]
0000000140b2dfce: 4180f8c0                 cmp        r8b, 0xc0
0000000140b2dfd2: 7d23                     jge        0x140b2dff7
0000000140b2dfd4: 440fb648fc               movzx      r9d, byte ptr [rax - 4]
0000000140b2dfd9: 4883c0fc                 add        rax, -4
0000000140b2dfdd: 4183e107                 and        r9d, 7
0000000140b2dfe1: 41c1e106                 shl        r9d, 6
0000000140b2dfe5: 4183e03f                 and        r8d, 0x3f
0000000140b2dfe9: 4509c8                   or         r8d, r9d
0000000140b2dfec: eb11                     jmp        0x140b2dfff
0000000140b2dfee: 4883c0fe                 add        rax, -2
0000000140b2dff2: 83e21f                   and        edx, 0x1f
0000000140b2dff5: eb12                     jmp        0x140b2e009
0000000140b2dff7: 4883c0fd                 add        rax, -3
0000000140b2dffb: 4183e00f                 and        r8d, 0xf
0000000140b2dfff: 41c1e006                 shl        r8d, 6
0000000140b2e003: 83e23f                   and        edx, 0x3f
0000000140b2e006: 4409c2                   or         edx, r8d
0000000140b2e009: c1e206                   shl        edx, 6
0000000140b2e00c: 80e13f                   and        cl, 0x3f
0000000140b2e00f: 0fb6c9                   movzx      ecx, cl
0000000140b2e012: 09d1                     or         ecx, edx
0000000140b2e014: 83f95c                   cmp        ecx, 0x5c
0000000140b2e017: 7415                     je         0x140b2e02e
0000000140b2e019: 83f92f                   cmp        ecx, 0x2f
0000000140b2e01c: 7410                     je         0x140b2e02e
0000000140b2e01e: 4c39e8                   cmp        rax, r13
0000000140b2e021: 0f8579ffffff             jne        0x140b2dfa0
0000000140b2e027: bf2f000000               mov        edi, 0x2f
0000000140b2e02c: eb4f                     jmp        0x140b2e07d
0000000140b2e02e: 4c8d2d5bd8fb01           lea        r13, [rip + 0x1fbd85b]
0000000140b2e035: 4c29e8                   sub        rax, r13
0000000140b2e038: 4989c0                   mov        r8, rax
0000000140b2e03b: 49ffc0                   inc        r8
0000000140b2e03e: 7432                     je         0x140b2e072
0000000140b2e040: 4983f82f                 cmp        r8, 0x2f
0000000140b2e044: 732a                     jae        0x140b2e070
0000000140b2e046: 43803c28bf               cmp        byte ptr [r8 + r13], 0xbf
0000000140b2e04b: 7f25                     jg         0x140b2e072
0000000140b2e04d: 488d0574d3fb01           lea        rax, [rip + 0x1fbd374]
0000000140b2e054: 4889442420               mov        qword ptr [rsp + 0x20], rax
0000000140b2e059: 488d0d30d8fb01           lea        rcx, [rip + 0x1fbd830]
0000000140b2e060: ba2f000000               mov        edx, 0x2f
0000000140b2e065: 41b92f000000             mov        r9d, 0x2f
0000000140b2e06b: e8d0b8da01               call       0x1428d9940
0000000140b2e070: 75db                     jne        0x140b2e04d
0000000140b2e072: bf2e000000               mov        edi, 0x2e
0000000140b2e077: 4829c7                   sub        rdi, rax
0000000140b2e07a: 4d01c5                   add        r13, r8
0000000140b2e07d: e8aea00e00               call       0x140c18130
0000000140b2e082: 4889c1                   mov        rcx, rax
0000000140b2e085: 488b00                   mov        rax, qword ptr [rax]
0000000140b2e088: 4883f801                 cmp        rax, 1
0000000140b2e08c: 0f851b0d0000             jne        0x140b2edad
0000000140b2e092: 4883c108                 add        rcx, 8
0000000140b2e096: 48833900                 cmp        qword ptr [rcx], 0
0000000140b2e09a: 0f85f9110000             jne        0x140b2f299
0000000140b2e0a0: 48c701ffffffff           mov        qword ptr [rcx], 0xffffffffffffffff
0000000140b2e0a7: 48898de0000000           mov        qword ptr [rbp + 0xe0], rcx
0000000140b2e0ae: 4883c108                 add        rcx, 8
0000000140b2e0b2: 48897c2420               mov        qword ptr [rsp + 0x20], rdi
0000000140b2e0b7: 48c744243000000000       mov        qword ptr [rsp + 0x30], 0
0000000140b2e0c0: 48c744242801000000       mov        qword ptr [rsp + 0x28], 1
0000000140b2e0c9: 4c89e2                   mov        rdx, r12
0000000140b2e0cc: 4d89f8                   mov        r8, r15
0000000140b2e0cf: 4d89e9                   mov        r9, r13
0000000140b2e0d2: e8698c0e00               call       0x140c16d40
0000000140b2e0d7: 488b8de0000000           mov        rcx, qword ptr [rbp + 0xe0]
0000000140b2e0de: 48ff01                   inc        qword ptr [rcx]
0000000140b2e0e1: 488945d8                 mov        qword ptr [rbp - 0x28], rax
0000000140b2e0e5: b801000000               mov        eax, 1
0000000140b2e0ea: 488b8d80010000           mov        rcx, qword ptr [rbp + 0x180]
0000000140b2e0f1: 488945d0                 mov        qword ptr [rbp - 0x30], rax
0000000140b2e0f5: 488b01                   mov        rax, qword ptr [rcx]
0000000140b2e0f8: 488b4908                 mov        rcx, qword ptr [rcx + 8]
0000000140b2e0fc: 488b10                   mov        rdx, qword ptr [rax]
0000000140b2e0ff: 4531ed                   xor        r13d, r13d
0000000140b2e102: 4989d0                   mov        r8, rdx
0000000140b2e105: 4c2b4118                 sub        r8, qword ptr [rcx + 0x18]
0000000140b2e109: 4d0f42c5                 cmovb      r8, r13
0000000140b2e10d: 482b5138                 sub        rdx, qword ptr [rcx + 0x38]
0000000140b2e111: 490f42d5                 cmovb      rdx, r13
0000000140b2e115: 4c8b4930                 mov        r9, qword ptr [rcx + 0x30]
0000000140b2e119: 4929d1                   sub        r9, rdx
0000000140b2e11c: 4d0f42cd                 cmovb      r9, r13
0000000140b2e120: 0f97c2                   seta       dl
0000000140b2e123: 4c8b5110                 mov        r10, qword ptr [rcx + 0x10]
0000000140b2e127: 4d29c2                   sub        r10, r8
0000000140b2e12a: 4d0f42d5                 cmovb      r10, r13
0000000140b2e12e: 488b4940                 mov        rcx, qword ptr [rcx + 0x40]
0000000140b2e132: 410f97c0                 seta       r8b
0000000140b2e136: 4d01ca                   add        r10, r9
0000000140b2e139: 4108d0                   or         r8b, dl
0000000140b2e13c: 448885a8000000           mov        byte ptr [rbp + 0xa8], r8b
0000000140b2e143: 4d0f45d5                 cmovne     r10, r13
0000000140b2e147: 4c29d1                   sub        rcx, r10
0000000140b2e14a: 488908                   mov        qword ptr [rax], rcx
0000000140b2e14d: 498b06                   mov        rax, qword ptr [r14]
0000000140b2e150: 498b4e08                 mov        rcx, qword ptr [r14 + 8]
0000000140b2e154: 488b10                   mov        rdx, qword ptr [rax]
0000000140b2e157: 4989d0                   mov        r8, rdx
0000000140b2e15a: 4c2b4118                 sub        r8, qword ptr [rcx + 0x18]
0000000140b2e15e: 4d0f42c5                 cmovb      r8, r13
0000000140b2e162: 482b5138                 sub        rdx, qword ptr [rcx + 0x38]
0000000140b2e166: 490f42d5                 cmovb      rdx, r13
0000000140b2e16a: 4c8b4930                 mov        r9, qword ptr [rcx + 0x30]
0000000140b2e16e: 4929d1                   sub        r9, rdx
0000000140b2e171: 4d0f42cd                 cmovb      r9, r13
0000000140b2e175: 0f97c2                   seta       dl
0000000140b2e178: 4c8b5110                 mov        r10, qword ptr [rcx + 0x10]
0000000140b2e17c: 4d29c2                   sub        r10, r8
0000000140b2e17f: 4d0f42d5                 cmovb      r10, r13
0000000140b2e183: 488b4940                 mov        rcx, qword ptr [rcx + 0x40]
0000000140b2e187: 410f97c0                 seta       r8b
0000000140b2e18b: 4d01ca                   add        r10, r9
0000000140b2e18e: 4108d0                   or         r8b, dl
0000000140b2e191: 44884508                 mov        byte ptr [rbp + 8], r8b
0000000140b2e195: 4d0f45d5                 cmovne     r10, r13
0000000140b2e199: 4c29d1                   sub        rcx, r10
0000000140b2e19c: 488908                   mov        qword ptr [rax], rcx
0000000140b2e19f: 488b03                   mov        rax, qword ptr [rbx]
0000000140b2e1a2: 4c8b7b08                 mov        r15, qword ptr [rbx + 8]
0000000140b2e1a6: 488b08                   mov        rcx, qword ptr [rax]
0000000140b2e1a9: 4889ca                   mov        rdx, rcx
0000000140b2e1ac: 492b5718                 sub        rdx, qword ptr [r15 + 0x18]
0000000140b2e1b0: 490f42d5                 cmovb      rdx, r13
0000000140b2e1b4: 492b4f38                 sub        rcx, qword ptr [r15 + 0x38]
0000000140b2e1b8: 490f42cd                 cmovb      rcx, r13
0000000140b2e1bc: 4d8b5710                 mov        r10, qword ptr [r15 + 0x10]
0000000140b2e1c0: 4d8b4f30                 mov        r9, qword ptr [r15 + 0x30]
0000000140b2e1c4: 4d89c8                   mov        r8, r9
0000000140b2e1c7: 4929c8                   sub        r8, rcx
0000000140b2e1ca: 4d0f42c5                 cmovb      r8, r13
0000000140b2e1ce: 400f96c6                 setbe      sil
0000000140b2e1d2: 4c89d3                   mov        rbx, r10
0000000140b2e1d5: 4829d3                   sub        rbx, rdx
0000000140b2e1d8: 490f42dd                 cmovb      rbx, r13
0000000140b2e1dc: 400f96c7                 setbe      dil
0000000140b2e1e0: 4d8b7740                 mov        r14, qword ptr [r15 + 0x40]
0000000140b2e1e4: 4d89f3                   mov        r11, r14
0000000140b2e1e7: 4d29c3                   sub        r11, r8
0000000140b2e1ea: 4929db                   sub        r11, rbx
0000000140b2e1ed: 4d8b6708                 mov        r12, qword ptr [r15 + 8]
0000000140b2e1f1: 4d8b7f28                 mov        r15, qword ptr [r15 + 0x28]
0000000140b2e1f5: 4c8918                   mov        qword ptr [rax], r11
0000000140b2e1f8: 4084f7                   test       dil, sil
0000000140b2e1fb: 7550                     jne        0x140b2e24d
0000000140b2e1fd: 4989cb                   mov        r11, rcx
0000000140b2e200: 49c1e304                 shl        r11, 4
0000000140b2e204: 4d01df                   add        r15, r11
0000000140b2e207: 49c1e004                 shl        r8, 4
0000000140b2e20b: 4989d3                   mov        r11, rdx
0000000140b2e20e: 49c1e304                 shl        r11, 4
0000000140b2e212: 4d01dc                   add        r12, r11
0000000140b2e215: 48c1e304                 shl        rbx, 4
0000000140b2e219: 4531db                   xor        r11d, r11d
0000000140b2e21c: 4939d2                   cmp        r10, rdx
0000000140b2e21f: ba08000000               mov        edx, 8
0000000140b2e224: 4c0f42e2                 cmovb      r12, rdx
0000000140b2e228: 4d8d541cf0               lea        r10, [r12 + rbx - 0x10]
0000000140b2e22d: 4d0f47da                 cmova      r11, r10
0000000140b2e231: 4939c9                   cmp        r9, rcx
0000000140b2e234: 4c0f42fa                 cmovb      r15, rdx
0000000140b2e238: 4b8d4c07f0               lea        rcx, [r15 + r8 - 0x10]
0000000140b2e23d: 490f46cb                 cmovbe     rcx, r11
0000000140b2e241: 4c8930                   mov        qword ptr [rax], r14
0000000140b2e244: 440fb66908               movzx      r13d, byte ptr [rcx + 8]
0000000140b2e249: 4180f501                 xor        r13b, 1
0000000140b2e24d: 488b75c8                 mov        rsi, qword ptr [rbp - 0x38]
0000000140b2e251: 4c8b16                   mov        r10, qword ptr [rsi]
0000000140b2e254: 49837a1000               cmp        qword ptr [r10 + 0x10], 0
0000000140b2e259: 0f84e8000000             je         0x140b2e347
0000000140b2e25f: 498b4208                 mov        rax, qword ptr [r10 + 8]
0000000140b2e263: 448b30                   mov        r14d, dword ptr [rax]
0000000140b2e266: 8b480c                   mov        ecx, dword ptr [rax + 0xc]
0000000140b2e269: 894d18                   mov        dword ptr [rbp + 0x18], ecx
0000000140b2e26c: 488b4804                 mov        rcx, qword ptr [rax + 4]
0000000140b2e270: 48894d10                 mov        qword ptr [rbp + 0x10], rcx
0000000140b2e274: 488b5810                 mov        rbx, qword ptr [rax + 0x10]
0000000140b2e278: 0f104018                 movups     xmm0, xmmword ptr [rax + 0x18]
0000000140b2e27c: 0f2945e0                 movaps     xmmword ptr [rbp - 0x20], xmm0
0000000140b2e280: 0f104028                 movups     xmm0, xmmword ptr [rax + 0x28]
0000000140b2e284: 0f2945f0                 movaps     xmmword ptr [rbp - 0x10], xmm0
0000000140b2e288: 410fb64218               movzx      eax, byte ptr [r10 + 0x18]
0000000140b2e28d: 83f802                   cmp        eax, 2
0000000140b2e290: 4c8995e0000000           mov        qword ptr [rbp + 0xe0], r10
0000000140b2e297: 0f84c5000000             je         0x140b2e362
0000000140b2e29d: 83f801                   cmp        eax, 1
0000000140b2e2a0: 0f85c5000000             jne        0x140b2e36b
0000000140b2e2a6: 41807a1900               cmp        byte ptr [r10 + 0x19], 0
0000000140b2e2ab: 0f849b030000             je         0x140b2e64c
0000000140b2e2b1: 4183fe02                 cmp        r14d, 2
0000000140b2e2b5: 0f85bc010000             jne        0x140b2e477
0000000140b2e2bb: 498d4219                 lea        rax, [r10 + 0x19]
0000000140b2e2bf: 488985b0000000           mov        qword ptr [rbp + 0xb0], rax
0000000140b2e2c6: 8b461c                   mov        eax, dword ptr [rsi + 0x1c]
0000000140b2e2c9: 488b4e10                 mov        rcx, qword ptr [rsi + 0x10]
0000000140b2e2cd: 8901                     mov        dword ptr [rcx], eax
0000000140b2e2cf: 410fb64218               movzx      eax, byte ptr [r10 + 0x18]
0000000140b2e2d4: 85c0                     test       eax, eax
0000000140b2e2d6: 7424                     je         0x140b2e2fc
0000000140b2e2d8: 83f801                   cmp        eax, 1
0000000140b2e2db: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2e2e2: 488b4020                 mov        rax, qword ptr [rax + 0x20]
0000000140b2e2e6: f048ff08                 lock dec   qword ptr [rax]
0000000140b2e2ea: 7510                     jne        0x140b2e2fc
0000000140b2e2ec: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2e2f3: 488d4820                 lea        rcx, [rax + 0x20]
0000000140b2e2f7: e834e00200               call       0x140b5c330
0000000140b2e2fc: 4c8b95e0000000           mov        r10, qword ptr [rbp + 0xe0]
0000000140b2e303: 41c6421800               mov        byte ptr [r10 + 0x18], 0
0000000140b2e308: 488b859f000000           mov        rax, qword ptr [rbp + 0x9f]
0000000140b2e30f: 488b8db0000000           mov        rcx, qword ptr [rbp + 0xb0]
0000000140b2e316: 4889413f                 mov        qword ptr [rcx + 0x3f], rax
0000000140b2e31a: 0f104560                 movups     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2e31e: f30f6f4d70               movdqu     xmm1, xmmword ptr [rbp + 0x70]
0000000140b2e323: f30f6f9580000000         movdqu     xmm2, xmmword ptr [rbp + 0x80]
0000000140b2e32b: f30f6f9d90000000         movdqu     xmm3, xmmword ptr [rbp + 0x90]
0000000140b2e333: f30f7f5930               movdqu     xmmword ptr [rcx + 0x30], xmm3
0000000140b2e338: f30f7f5120               movdqu     xmmword ptr [rcx + 0x20], xmm2
0000000140b2e33d: f30f7f4910               movdqu     xmmword ptr [rcx + 0x10], xmm1
0000000140b2e342: 0f1101                   movups     xmmword ptr [rcx], xmm0
0000000140b2e345: eb2a                     jmp        0x140b2e371
0000000140b2e347: 41be02000000             mov        r14d, 2
0000000140b2e34d: 410fb64218               movzx      eax, byte ptr [r10 + 0x18]
0000000140b2e352: 83f802                   cmp        eax, 2
0000000140b2e355: 4c8995e0000000           mov        qword ptr [rbp + 0xe0], r10
0000000140b2e35c: 0f853bffffff             jne        0x140b2e29d
0000000140b2e362: 31c9                     xor        ecx, ecx
0000000140b2e364: 31d2                     xor        edx, edx
0000000140b2e366: e9e7020000               jmp        0x140b2e652
0000000140b2e36b: 4183fe02                 cmp        r14d, 2
0000000140b2e36f: 750b                     jne        0x140b2e37c
0000000140b2e371: b101                     mov        cl, 1
0000000140b2e373: 31c0                     xor        eax, eax
0000000140b2e375: b201                     mov        dl, 1
0000000140b2e377: e9d6020000               jmp        0x140b2e652
0000000140b2e37c: 4c89d1                   mov        rcx, r10
0000000140b2e37f: 4889da                   mov        rdx, rbx
0000000140b2e382: e8d9f8ffff               call       0x140b2dc60
0000000140b2e387: 4489b5cc000000           mov        dword ptr [rbp + 0xcc], r14d
0000000140b2e38e: 48899db8000000           mov        qword ptr [rbp + 0xb8], rbx
0000000140b2e395: 488985d0000000           mov        qword ptr [rbp + 0xd0], rax
0000000140b2e39c: 488b4838                 mov        rcx, qword ptr [rax + 0x38]
0000000140b2e3a0: 48894d70                 mov        qword ptr [rbp + 0x70], rcx
0000000140b2e3a4: 488985b0000000           mov        qword ptr [rbp + 0xb0], rax
0000000140b2e3ab: 0f104028                 movups     xmm0, xmmword ptr [rax + 0x28]
0000000140b2e3af: 0f294560                 movaps     xmmword ptr [rbp + 0x60], xmm0
0000000140b2e3b3: 488b9598010000           mov        rdx, qword ptr [rbp + 0x198]
0000000140b2e3ba: 8b421c                   mov        eax, dword ptr [rdx + 0x1c]
0000000140b2e3bd: 488b4a10                 mov        rcx, qword ptr [rdx + 0x10]
0000000140b2e3c1: 8901                     mov        dword ptr [rcx], eax
0000000140b2e3c3: 488b1a                   mov        rbx, qword ptr [rdx]
0000000140b2e3c6: 4c8b7b30                 mov        r15, qword ptr [rbx + 0x30]
0000000140b2e3ca: 4c8b6340                 mov        r12, qword ptr [rbx + 0x40]
0000000140b2e3ce: 4c3b7b20                 cmp        r15, qword ptr [rbx + 0x20]
0000000140b2e3d2: 7510                     jne        0x140b2e3e4
0000000140b2e3d4: 488d4b20                 lea        rcx, [rbx + 0x20]
0000000140b2e3d8: 488d1569bd1002           lea        rdx, [rip + 0x210bd69]
0000000140b2e3df: e84c48a3ff               call       0x140562c30
0000000140b2e3e4: 488b4328                 mov        rax, qword ptr [rbx + 0x28]
0000000140b2e3e8: 4c89f9                   mov        rcx, r15
0000000140b2e3eb: 48c1e105                 shl        rcx, 5
0000000140b2e3ef: 488b5570                 mov        rdx, qword ptr [rbp + 0x70]
0000000140b2e3f3: 4889540810               mov        qword ptr [rax + rcx + 0x10], rdx
0000000140b2e3f8: 0f284560                 movaps     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2e3fc: 0f110408                 movups     xmmword ptr [rax + rcx], xmm0
0000000140b2e400: 4c89640818               mov        qword ptr [rax + rcx + 0x18], r12
0000000140b2e405: 49ffc7                   inc        r15
0000000140b2e408: 4c897b30                 mov        qword ptr [rbx + 0x30], r15
0000000140b2e40c: 48ff4340                 inc        qword ptr [rbx + 0x40]
0000000140b2e410: 488b4510                 mov        rax, qword ptr [rbp + 0x10]
0000000140b2e414: 48894520                 mov        qword ptr [rbp + 0x20], rax
0000000140b2e418: 8b4518                   mov        eax, dword ptr [rbp + 0x18]
0000000140b2e41b: 894528                   mov        dword ptr [rbp + 0x28], eax
0000000140b2e41e: 0f2845e0                 movaps     xmm0, xmmword ptr [rbp - 0x20]
0000000140b2e422: 0f284df0                 movaps     xmm1, xmmword ptr [rbp - 0x10]
0000000140b2e426: 0f294560                 movaps     xmmword ptr [rbp + 0x60], xmm0
0000000140b2e42a: 0f294d70                 movaps     xmmword ptr [rbp + 0x70], xmm1
0000000140b2e42e: 8b461c                   mov        eax, dword ptr [rsi + 0x1c]
0000000140b2e431: 488b4e10                 mov        rcx, qword ptr [rsi + 0x10]
0000000140b2e435: 8901                     mov        dword ptr [rcx], eax
0000000140b2e437: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2e43e: 0fb64018                 movzx      eax, byte ptr [rax + 0x18]
0000000140b2e442: 85c0                     test       eax, eax
0000000140b2e444: 0f849a010000             je         0x140b2e5e4
0000000140b2e44a: 83f801                   cmp        eax, 1
0000000140b2e44d: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2e454: 488b4020                 mov        rax, qword ptr [rax + 0x20]
0000000140b2e458: f048ff08                 lock dec   qword ptr [rax]
0000000140b2e45c: 0f8582010000             jne        0x140b2e5e4
0000000140b2e462: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2e469: 488d4820                 lea        rcx, [rax + 0x20]
0000000140b2e46d: e8bede0200               call       0x140b5c330
0000000140b2e472: e96d010000               jmp        0x140b2e5e4
0000000140b2e477: 49395a38                 cmp        qword ptr [r10 + 0x38], rbx
0000000140b2e47b: 48899db8000000           mov        qword ptr [rbp + 0xb8], rbx
0000000140b2e482: 4489b5cc000000           mov        dword ptr [rbp + 0xcc], r14d
0000000140b2e489: 7579                     jne        0x140b2e504
0000000140b2e48b: 4c89d1                   mov        rcx, r10
0000000140b2e48e: 4889da                   mov        rdx, rbx
0000000140b2e491: e8caf7ffff               call       0x140b2dc60
0000000140b2e496: 488985b0000000           mov        qword ptr [rbp + 0xb0], rax
0000000140b2e49d: 8b4518                   mov        eax, dword ptr [rbp + 0x18]
0000000140b2e4a0: 894528                   mov        dword ptr [rbp + 0x28], eax
0000000140b2e4a3: 488b4510                 mov        rax, qword ptr [rbp + 0x10]
0000000140b2e4a7: 48894520                 mov        qword ptr [rbp + 0x20], rax
0000000140b2e4ab: 0f2845e0                 movaps     xmm0, xmmword ptr [rbp - 0x20]
0000000140b2e4af: 0f284df0                 movaps     xmm1, xmmword ptr [rbp - 0x10]
0000000140b2e4b3: 0f294560                 movaps     xmmword ptr [rbp + 0x60], xmm0
0000000140b2e4b7: 0f294d70                 movaps     xmmword ptr [rbp + 0x70], xmm1
0000000140b2e4bb: 8b461c                   mov        eax, dword ptr [rsi + 0x1c]
0000000140b2e4be: 488b4e10                 mov        rcx, qword ptr [rsi + 0x10]
0000000140b2e4c2: 8901                     mov        dword ptr [rcx], eax
0000000140b2e4c4: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2e4cb: 0fb64018                 movzx      eax, byte ptr [rax + 0x18]
0000000140b2e4cf: 85c0                     test       eax, eax
0000000140b2e4d1: 0f840d010000             je         0x140b2e5e4
0000000140b2e4d7: 83f801                   cmp        eax, 1
0000000140b2e4da: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2e4e1: 488b4020                 mov        rax, qword ptr [rax + 0x20]
0000000140b2e4e5: f048ff08                 lock dec   qword ptr [rax]
0000000140b2e4e9: 0f85f5000000             jne        0x140b2e5e4
0000000140b2e4ef: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2e4f6: 488d4820                 lea        rcx, [rax + 0x20]
0000000140b2e4fa: e831de0200               call       0x140b5c330
0000000140b2e4ff: e9e0000000               jmp        0x140b2e5e4
0000000140b2e504: 4c89d1                   mov        rcx, r10
0000000140b2e507: 4889da                   mov        rdx, rbx
0000000140b2e50a: e851f7ffff               call       0x140b2dc60
0000000140b2e50f: 488985d0000000           mov        qword ptr [rbp + 0xd0], rax
0000000140b2e516: 488b4838                 mov        rcx, qword ptr [rax + 0x38]
0000000140b2e51a: 48894d70                 mov        qword ptr [rbp + 0x70], rcx
0000000140b2e51e: 488985b0000000           mov        qword ptr [rbp + 0xb0], rax
0000000140b2e525: 0f104028                 movups     xmm0, xmmword ptr [rax + 0x28]
0000000140b2e529: 0f294560                 movaps     xmmword ptr [rbp + 0x60], xmm0
0000000140b2e52d: 488b9598010000           mov        rdx, qword ptr [rbp + 0x198]
0000000140b2e534: 8b421c                   mov        eax, dword ptr [rdx + 0x1c]
0000000140b2e537: 488b4a10                 mov        rcx, qword ptr [rdx + 0x10]
0000000140b2e53b: 8901                     mov        dword ptr [rcx], eax
0000000140b2e53d: 488b1a                   mov        rbx, qword ptr [rdx]
0000000140b2e540: 4c8b7b30                 mov        r15, qword ptr [rbx + 0x30]
0000000140b2e544: 4c8b6340                 mov        r12, qword ptr [rbx + 0x40]
0000000140b2e548: 4c3b7b20                 cmp        r15, qword ptr [rbx + 0x20]
0000000140b2e54c: 7510                     jne        0x140b2e55e
0000000140b2e54e: 488d4b20                 lea        rcx, [rbx + 0x20]
0000000140b2e552: 488d15efbb1002           lea        rdx, [rip + 0x210bbef]
0000000140b2e559: e8d246a3ff               call       0x140562c30
0000000140b2e55e: 488b4328                 mov        rax, qword ptr [rbx + 0x28]
0000000140b2e562: 4c89f9                   mov        rcx, r15
0000000140b2e565: 48c1e105                 shl        rcx, 5
0000000140b2e569: 488b5570                 mov        rdx, qword ptr [rbp + 0x70]
0000000140b2e56d: 4889540810               mov        qword ptr [rax + rcx + 0x10], rdx
0000000140b2e572: 0f284560                 movaps     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2e576: 0f110408                 movups     xmmword ptr [rax + rcx], xmm0
0000000140b2e57a: 4c89640818               mov        qword ptr [rax + rcx + 0x18], r12
0000000140b2e57f: 49ffc7                   inc        r15
0000000140b2e582: 4c897b30                 mov        qword ptr [rbx + 0x30], r15
0000000140b2e586: 48ff4340                 inc        qword ptr [rbx + 0x40]
0000000140b2e58a: 488b4510                 mov        rax, qword ptr [rbp + 0x10]
0000000140b2e58e: 48894520                 mov        qword ptr [rbp + 0x20], rax
0000000140b2e592: 8b4518                   mov        eax, dword ptr [rbp + 0x18]
0000000140b2e595: 894528                   mov        dword ptr [rbp + 0x28], eax
0000000140b2e598: 0f2845e0                 movaps     xmm0, xmmword ptr [rbp - 0x20]
0000000140b2e59c: 0f284df0                 movaps     xmm1, xmmword ptr [rbp - 0x10]
0000000140b2e5a0: 0f294560                 movaps     xmmword ptr [rbp + 0x60], xmm0
0000000140b2e5a4: 0f294d70                 movaps     xmmword ptr [rbp + 0x70], xmm1
0000000140b2e5a8: 8b461c                   mov        eax, dword ptr [rsi + 0x1c]
0000000140b2e5ab: 488b4e10                 mov        rcx, qword ptr [rsi + 0x10]
0000000140b2e5af: 8901                     mov        dword ptr [rcx], eax
0000000140b2e5b1: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2e5b8: 0fb64018                 movzx      eax, byte ptr [rax + 0x18]
0000000140b2e5bc: 85c0                     test       eax, eax
0000000140b2e5be: 7424                     je         0x140b2e5e4
0000000140b2e5c0: 83f801                   cmp        eax, 1
0000000140b2e5c3: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2e5ca: 488b4020                 mov        rax, qword ptr [rax + 0x20]
0000000140b2e5ce: f048ff08                 lock dec   qword ptr [rax]
0000000140b2e5d2: 7510                     jne        0x140b2e5e4
0000000140b2e5d4: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2e5db: 488d4820                 lea        rcx, [rax + 0x20]
0000000140b2e5df: e84cdd0200               call       0x140b5c330
0000000140b2e5e4: 4c8b95e0000000           mov        r10, qword ptr [rbp + 0xe0]
0000000140b2e5eb: 6641c742180101           mov        word ptr [r10 + 0x18], 0x101
0000000140b2e5f2: 8b85d0000000             mov        eax, dword ptr [rbp + 0xd0]
0000000140b2e5f8: 4189421a                 mov        dword ptr [r10 + 0x1a], eax
0000000140b2e5fc: 0fb785d4000000           movzx      eax, word ptr [rbp + 0xd4]
0000000140b2e603: 664189421e               mov        word ptr [r10 + 0x1e], ax
0000000140b2e608: 488b85b0000000           mov        rax, qword ptr [rbp + 0xb0]
0000000140b2e60f: 49894220                 mov        qword ptr [r10 + 0x20], rax
0000000140b2e613: 448bb5cc000000           mov        r14d, dword ptr [rbp + 0xcc]
0000000140b2e61a: 45897228                 mov        dword ptr [r10 + 0x28], r14d
0000000140b2e61e: 488b4520                 mov        rax, qword ptr [rbp + 0x20]
0000000140b2e622: 4989422c                 mov        qword ptr [r10 + 0x2c], rax
0000000140b2e626: 8b4528                   mov        eax, dword ptr [rbp + 0x28]
0000000140b2e629: 41894234                 mov        dword ptr [r10 + 0x34], eax
0000000140b2e62d: 488b9db8000000           mov        rbx, qword ptr [rbp + 0xb8]
0000000140b2e634: 49895a38                 mov        qword ptr [r10 + 0x38], rbx
0000000140b2e638: 0f284560                 movaps     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2e63c: 660f6f4d70               movdqa     xmm1, xmmword ptr [rbp + 0x70]
0000000140b2e641: 410f114240               movups     xmmword ptr [r10 + 0x40], xmm0
0000000140b2e646: f3410f7f4a50             movdqu     xmmword ptr [r10 + 0x50], xmm1
0000000140b2e64c: b101                     mov        cl, 1
0000000140b2e64e: 31d2                     xor        edx, edx
0000000140b2e650: b001                     mov        al, 1
0000000140b2e652: 4c8bbd88010000           mov        r15, qword ptr [rbp + 0x188]
0000000140b2e659: 4c8b85c0000000           mov        r8, qword ptr [rbp + 0xc0]
0000000140b2e660: 4d8b20                   mov        r12, qword ptr [r8]
0000000140b2e663: 458b0424                 mov        r8d, dword ptr [r12]
0000000140b2e667: 4183f803                 cmp        r8d, 3
0000000140b2e66b: 410f94c1                 sete       r9b
0000000140b2e66f: 440fb65d08               movzx      r11d, byte ptr [rbp + 8]
0000000140b2e674: 4508cb                   or         r11b, r9b
0000000140b2e677: 440fb68da8000000         movzx      r9d, byte ptr [rbp + 0xa8]
0000000140b2e67f: 4508d9                   or         r9b, r11b
0000000140b2e682: 4508cd                   or         r13b, r9b
0000000140b2e685: 7411                     je         0x140b2e698
0000000140b2e687: 84c9                     test       cl, cl
0000000140b2e689: 7459                     je         0x140b2e6e4
0000000140b2e68b: 84d2                     test       dl, dl
0000000140b2e68d: 0f85d7010000             jne        0x140b2e86a
0000000140b2e693: e94b060000               jmp        0x140b2ece3
0000000140b2e698: 4489c1                   mov        ecx, r8d
0000000140b2e69b: 488d15bad3fb01           lea        rdx, [rip + 0x1fbd3ba]
0000000140b2e6a2: 48630c8a                 movsxd     rcx, dword ptr [rdx + rcx*4]
0000000140b2e6a6: 4801d1                   add        rcx, rdx
0000000140b2e6a9: ffe1                     jmp        rcx
0000000140b2e6ab: 31c9                     xor        ecx, ecx
0000000140b2e6ad: 84c0                     test       al, al
0000000140b2e6af: 7524                     jne        0x140b2e6d5
0000000140b2e6b1: e9b4010000               jmp        0x140b2e86a
0000000140b2e6b6: 0f57c0                   xorps      xmm0, xmm0
0000000140b2e6b9: 410f2e442404             ucomiss    xmm0, dword ptr [r12 + 4]
0000000140b2e6bf: 0f93c1                   setae      cl
0000000140b2e6c2: 84c0                     test       al, al
0000000140b2e6c4: 750f                     jne        0x140b2e6d5
0000000140b2e6c6: e99f010000               jmp        0x140b2e86a
0000000140b2e6cb: b101                     mov        cl, 1
0000000140b2e6cd: 84c0                     test       al, al
0000000140b2e6cf: 0f8495010000             je         0x140b2e86a
0000000140b2e6d5: 0fb6c0                   movzx      eax, al
0000000140b2e6d8: 83f802                   cmp        eax, 2
0000000140b2e6db: 0f85e0010000             jne        0x140b2e8c1
0000000140b2e6e1: 4531ed                   xor        r13d, r13d
0000000140b2e6e4: 410fb64219               movzx      eax, byte ptr [r10 + 0x19]
0000000140b2e6e9: 04fe                     add        al, 0xfe
0000000140b2e6eb: 0fb6c0                   movzx      eax, al
0000000140b2e6ee: 3c03                     cmp        al, 3
0000000140b2e6f0: b903000000               mov        ecx, 3
0000000140b2e6f5: 0f42c8                   cmovb      ecx, eax
0000000140b2e6f8: 498d7a20                 lea        rdi, [r10 + 0x20]
0000000140b2e6fc: 0fb6c1                   movzx      eax, cl
0000000140b2e6ff: 8d48ff                   lea        ecx, [rax - 1]
0000000140b2e702: 83f902                   cmp        ecx, 2
0000000140b2e705: 0f83d4000000             jae        0x140b2e7df
0000000140b2e70b: 41f6c501                 test       r13b, 1
0000000140b2e70f: 0f84a2020000             je         0x140b2e9b7
0000000140b2e715: 498b4258                 mov        rax, qword ptr [r10 + 0x58]
0000000140b2e719: 48898590000000           mov        qword ptr [rbp + 0x90], rax
0000000140b2e720: f3410f6f4228             movdqu     xmm0, xmmword ptr [r10 + 0x28]
0000000140b2e726: f3410f6f4a38             movdqu     xmm1, xmmword ptr [r10 + 0x38]
0000000140b2e72c: f3410f6f5248             movdqu     xmm2, xmmword ptr [r10 + 0x48]
0000000140b2e732: 660f7f9580000000         movdqa     xmmword ptr [rbp + 0x80], xmm2
0000000140b2e73a: 660f7f4d70               movdqa     xmmword ptr [rbp + 0x70], xmm1
0000000140b2e73f: 660f7f4560               movdqa     xmmword ptr [rbp + 0x60], xmm0
0000000140b2e744: 498b4a20                 mov        rcx, qword ptr [r10 + 0x20]
0000000140b2e748: 498b4238                 mov        rax, qword ptr [r10 + 0x38]
0000000140b2e74c: 48898dc0000000           mov        qword ptr [rbp + 0xc0], rcx
0000000140b2e753: f048ff01                 lock inc   qword ptr [rcx]
0000000140b2e757: 0f8e680b0000             jle        0x140b2f2c5
0000000140b2e75d: 498d4a28                 lea        rcx, [r10 + 0x28]
0000000140b2e761: 48898da8000000           mov        qword ptr [rbp + 0xa8], rcx
0000000140b2e768: 4183fe02                 cmp        r14d, 2
0000000140b2e76c: 0f95c1                   setne      cl
0000000140b2e76f: 4839c3                   cmp        rbx, rax
0000000140b2e772: 0f94c0                   sete       al
0000000140b2e775: 20c8                     and        al, cl
0000000140b2e777: 884508                   mov        byte ptr [rbp + 8], al
0000000140b2e77a: 488b8590000000           mov        rax, qword ptr [rbp + 0x90]
0000000140b2e781: 48894550                 mov        qword ptr [rbp + 0x50], rax
0000000140b2e785: 0f284560                 movaps     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2e789: 0f284d70                 movaps     xmm1, xmmword ptr [rbp + 0x70]
0000000140b2e78d: 0f289580000000           movaps     xmm2, xmmword ptr [rbp + 0x80]
0000000140b2e794: 0f295540                 movaps     xmmword ptr [rbp + 0x40], xmm2
0000000140b2e798: 0f294d30                 movaps     xmmword ptr [rbp + 0x30], xmm1
0000000140b2e79c: 0f294520                 movaps     xmmword ptr [rbp + 0x20], xmm0
0000000140b2e7a0: 8b461c                   mov        eax, dword ptr [rsi + 0x1c]
0000000140b2e7a3: 488b4e10                 mov        rcx, qword ptr [rsi + 0x10]
0000000140b2e7a7: 8901                     mov        dword ptr [rcx], eax
0000000140b2e7a9: 410fb64218               movzx      eax, byte ptr [r10 + 0x18]
0000000140b2e7ae: 85c0                     test       eax, eax
0000000140b2e7b0: 7414                     je         0x140b2e7c6
0000000140b2e7b2: 83f801                   cmp        eax, 1
0000000140b2e7b5: 488b07                   mov        rax, qword ptr [rdi]
0000000140b2e7b8: f048ff08                 lock dec   qword ptr [rax]
0000000140b2e7bc: 7508                     jne        0x140b2e7c6
0000000140b2e7be: 4889f9                   mov        rcx, rdi
0000000140b2e7c1: e86adb0200               call       0x140b5c330
0000000140b2e7c6: 4c8b95e0000000           mov        r10, qword ptr [rbp + 0xe0]
0000000140b2e7cd: 41c6421802               mov        byte ptr [r10 + 0x18], 2
0000000140b2e7d2: 0fb64508                 movzx      eax, byte ptr [rbp + 8]
0000000140b2e7d6: 41884219                 mov        byte ptr [r10 + 0x19], al
0000000140b2e7da: e99b010000               jmp        0x140b2e97a
0000000140b2e7df: 85c0                     test       eax, eax
0000000140b2e7e1: 0f84fc040000             je         0x140b2ece3
0000000140b2e7e7: 4183fe02                 cmp        r14d, 2
0000000140b2e7eb: 0f85fa010000             jne        0x140b2e9eb
0000000140b2e7f1: 498d4219                 lea        rax, [r10 + 0x19]
0000000140b2e7f5: 488985c0000000           mov        qword ptr [rbp + 0xc0], rax
0000000140b2e7fc: 8b461c                   mov        eax, dword ptr [rsi + 0x1c]
0000000140b2e7ff: 488b4e10                 mov        rcx, qword ptr [rsi + 0x10]
0000000140b2e803: 8901                     mov        dword ptr [rcx], eax
0000000140b2e805: 410fb64218               movzx      eax, byte ptr [r10 + 0x18]
0000000140b2e80a: 85c0                     test       eax, eax
0000000140b2e80c: 7414                     je         0x140b2e822
0000000140b2e80e: 83f801                   cmp        eax, 1
0000000140b2e811: 488b07                   mov        rax, qword ptr [rdi]
0000000140b2e814: f048ff08                 lock dec   qword ptr [rax]
0000000140b2e818: 7508                     jne        0x140b2e822
0000000140b2e81a: 4889f9                   mov        rcx, rdi
0000000140b2e81d: e80edb0200               call       0x140b5c330
0000000140b2e822: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2e829: c6401800                 mov        byte ptr [rax + 0x18], 0
0000000140b2e82d: 488b859f000000           mov        rax, qword ptr [rbp + 0x9f]
0000000140b2e834: 488b8dc0000000           mov        rcx, qword ptr [rbp + 0xc0]
0000000140b2e83b: 4889413f                 mov        qword ptr [rcx + 0x3f], rax
0000000140b2e83f: 0f104560                 movups     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2e843: f30f6f4d70               movdqu     xmm1, xmmword ptr [rbp + 0x70]
0000000140b2e848: f30f6f9580000000         movdqu     xmm2, xmmword ptr [rbp + 0x80]
0000000140b2e850: f30f6f9d90000000         movdqu     xmm3, xmmword ptr [rbp + 0x90]
0000000140b2e858: f30f7f5930               movdqu     xmmword ptr [rcx + 0x30], xmm3
0000000140b2e85d: f30f7f5120               movdqu     xmmword ptr [rcx + 0x20], xmm2
0000000140b2e862: f30f7f4910               movdqu     xmmword ptr [rcx + 0x10], xmm1
0000000140b2e867: 0f1101                   movups     xmmword ptr [rcx], xmm0
0000000140b2e86a: 418b471c                 mov        eax, dword ptr [r15 + 0x1c]
0000000140b2e86e: 498b4f10                 mov        rcx, qword ptr [r15 + 0x10]
0000000140b2e872: 8901                     mov        dword ptr [rcx], eax
0000000140b2e874: 498b3f                   mov        rdi, qword ptr [r15]
0000000140b2e877: 488b5f30                 mov        rbx, qword ptr [rdi + 0x30]
0000000140b2e87b: 4c8b7740                 mov        r14, qword ptr [rdi + 0x40]
0000000140b2e87f: 483b5f20                 cmp        rbx, qword ptr [rdi + 0x20]
0000000140b2e883: 7510                     jne        0x140b2e895
0000000140b2e885: 488d4f20                 lea        rcx, [rdi + 0x20]
0000000140b2e889: 488d15b8b81002           lea        rdx, [rip + 0x210b8b8]
0000000140b2e890: e89b43a3ff               call       0x140562c30
0000000140b2e895: 488b4728                 mov        rax, qword ptr [rdi + 0x28]
0000000140b2e899: 4889d9                   mov        rcx, rbx
0000000140b2e89c: 48c1e105                 shl        rcx, 5
0000000140b2e8a0: 4c893408                 mov        qword ptr [rax + rcx], r14
0000000140b2e8a4: c744080800000000         mov        dword ptr [rax + rcx + 8], 0
0000000140b2e8ac: c644081800               mov        byte ptr [rax + rcx + 0x18], 0
0000000140b2e8b1: 48ffc3                   inc        rbx
0000000140b2e8b4: 48895f30                 mov        qword ptr [rdi + 0x30], rbx
0000000140b2e8b8: 48ff4740                 inc        qword ptr [rdi + 0x40]
0000000140b2e8bc: e990070000               jmp        0x140b2f051
0000000140b2e8c1: 84c9                     test       cl, cl
0000000140b2e8c3: 0f841a040000             je         0x140b2ece3
0000000140b2e8c9: 498b4258                 mov        rax, qword ptr [r10 + 0x58]
0000000140b2e8cd: 48898590000000           mov        qword ptr [rbp + 0x90], rax
0000000140b2e8d4: f3410f6f4228             movdqu     xmm0, xmmword ptr [r10 + 0x28]
0000000140b2e8da: f3410f6f4a38             movdqu     xmm1, xmmword ptr [r10 + 0x38]
0000000140b2e8e0: f3410f6f5248             movdqu     xmm2, xmmword ptr [r10 + 0x48]
0000000140b2e8e6: 660f7f9580000000         movdqa     xmmword ptr [rbp + 0x80], xmm2
0000000140b2e8ee: 660f7f4d70               movdqa     xmmword ptr [rbp + 0x70], xmm1
0000000140b2e8f3: 660f7f4560               movdqa     xmmword ptr [rbp + 0x60], xmm0
0000000140b2e8f8: 498b4220                 mov        rax, qword ptr [r10 + 0x20]
0000000140b2e8fc: 488985c0000000           mov        qword ptr [rbp + 0xc0], rax
0000000140b2e903: f048ff00                 lock inc   qword ptr [rax]
0000000140b2e907: 0f8eb8090000             jle        0x140b2f2c5
0000000140b2e90d: 498d4228                 lea        rax, [r10 + 0x28]
0000000140b2e911: 488985a8000000           mov        qword ptr [rbp + 0xa8], rax
0000000140b2e918: 488b8590000000           mov        rax, qword ptr [rbp + 0x90]
0000000140b2e91f: 48894550                 mov        qword ptr [rbp + 0x50], rax
0000000140b2e923: 0f284560                 movaps     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2e927: 0f284d70                 movaps     xmm1, xmmword ptr [rbp + 0x70]
0000000140b2e92b: 0f289580000000           movaps     xmm2, xmmword ptr [rbp + 0x80]
0000000140b2e932: 0f295540                 movaps     xmmword ptr [rbp + 0x40], xmm2
0000000140b2e936: 0f294d30                 movaps     xmmword ptr [rbp + 0x30], xmm1
0000000140b2e93a: 0f294520                 movaps     xmmword ptr [rbp + 0x20], xmm0
0000000140b2e93e: 8b461c                   mov        eax, dword ptr [rsi + 0x1c]
0000000140b2e941: 488b4e10                 mov        rcx, qword ptr [rsi + 0x10]
0000000140b2e945: 8901                     mov        dword ptr [rcx], eax
0000000140b2e947: 410fb64218               movzx      eax, byte ptr [r10 + 0x18]
0000000140b2e94c: 85c0                     test       eax, eax
0000000140b2e94e: 741c                     je         0x140b2e96c
0000000140b2e950: 488b8de0000000           mov        rcx, qword ptr [rbp + 0xe0]
0000000140b2e957: 4883c120                 add        rcx, 0x20
0000000140b2e95b: 83f801                   cmp        eax, 1
0000000140b2e95e: 488b01                   mov        rax, qword ptr [rcx]
0000000140b2e961: f048ff08                 lock dec   qword ptr [rax]
0000000140b2e965: 7505                     jne        0x140b2e96c
0000000140b2e967: e8c4d90200               call       0x140b5c330
0000000140b2e96c: 4c8b95e0000000           mov        r10, qword ptr [rbp + 0xe0]
0000000140b2e973: 6641c742180203           mov        word ptr [r10 + 0x18], 0x302
0000000140b2e97a: 8b4560                   mov        eax, dword ptr [rbp + 0x60]
0000000140b2e97d: 4189421a                 mov        dword ptr [r10 + 0x1a], eax
0000000140b2e981: 0fb74564                 movzx      eax, word ptr [rbp + 0x64]
0000000140b2e985: 664189421e               mov        word ptr [r10 + 0x1e], ax
0000000140b2e98a: 488b85c0000000           mov        rax, qword ptr [rbp + 0xc0]
0000000140b2e991: 49894220                 mov        qword ptr [r10 + 0x20], rax
0000000140b2e995: 488b4550                 mov        rax, qword ptr [rbp + 0x50]
0000000140b2e999: 488b8da8000000           mov        rcx, qword ptr [rbp + 0xa8]
0000000140b2e9a0: 48894130                 mov        qword ptr [rcx + 0x30], rax
0000000140b2e9a4: 0f284520                 movaps     xmm0, xmmword ptr [rbp + 0x20]
0000000140b2e9a8: 660f6f4d30               movdqa     xmm1, xmmword ptr [rbp + 0x30]
0000000140b2e9ad: 660f6f5540               movdqa     xmm2, xmmword ptr [rbp + 0x40]
0000000140b2e9b2: e91f030000               jmp        0x140b2ecd6
0000000140b2e9b7: 498d5a60                 lea        rbx, [r10 + 0x60]
0000000140b2e9bb: 498b4260                 mov        rax, qword ptr [r10 + 0x60]
0000000140b2e9bf: 89c1                     mov        ecx, eax
0000000140b2e9c1: 83e108                   and        ecx, 8
0000000140b2e9c4: c1e903                   shr        ecx, 3
0000000140b2e9c7: 4883f8f0                 cmp        rax, -0x10
0000000140b2e9cb: 0f93c2                   setae      dl
0000000140b2e9ce: 08ca                     or         dl, cl
0000000140b2e9d0: 0f8492000000             je         0x140b2ea68
0000000140b2e9d6: 4889d9                   mov        rcx, rbx
0000000140b2e9d9: 31d2                     xor        edx, edx
0000000140b2e9db: 41b900ca9a3b             mov        r9d, 0x3b9aca00
0000000140b2e9e1: e8cadad901               call       0x1428cc4b0
0000000140b2e9e6: e98c000000               jmp        0x140b2ea77
0000000140b2e9eb: 493b5a38                 cmp        rbx, qword ptr [r10 + 0x38]
0000000140b2e9ef: 48899db8000000           mov        qword ptr [rbp + 0xb8], rbx
0000000140b2e9f6: 4489b5cc000000           mov        dword ptr [rbp + 0xcc], r14d
0000000140b2e9fd: 0f85cb030000             jne        0x140b2edce
0000000140b2ea03: 488b07                   mov        rax, qword ptr [rdi]
0000000140b2ea06: 488985c0000000           mov        qword ptr [rbp + 0xc0], rax
0000000140b2ea0d: f048ff00                 lock inc   qword ptr [rax]
0000000140b2ea11: 0f8eae080000             jle        0x140b2f2c5
0000000140b2ea17: 8b4518                   mov        eax, dword ptr [rbp + 0x18]
0000000140b2ea1a: 894528                   mov        dword ptr [rbp + 0x28], eax
0000000140b2ea1d: 488b4510                 mov        rax, qword ptr [rbp + 0x10]
0000000140b2ea21: 48894520                 mov        qword ptr [rbp + 0x20], rax
0000000140b2ea25: 0f2845e0                 movaps     xmm0, xmmword ptr [rbp - 0x20]
0000000140b2ea29: 0f284df0                 movaps     xmm1, xmmword ptr [rbp - 0x10]
0000000140b2ea2d: 0f294560                 movaps     xmmword ptr [rbp + 0x60], xmm0
0000000140b2ea31: 0f294d70                 movaps     xmmword ptr [rbp + 0x70], xmm1
0000000140b2ea35: 8b461c                   mov        eax, dword ptr [rsi + 0x1c]
0000000140b2ea38: 488b4e10                 mov        rcx, qword ptr [rsi + 0x10]
0000000140b2ea3c: 8901                     mov        dword ptr [rcx], eax
0000000140b2ea3e: 410fb64218               movzx      eax, byte ptr [r10 + 0x18]
0000000140b2ea43: 85c0                     test       eax, eax
0000000140b2ea45: 0f8453040000             je         0x140b2ee9e
0000000140b2ea4b: 83f801                   cmp        eax, 1
0000000140b2ea4e: 488b07                   mov        rax, qword ptr [rdi]
0000000140b2ea51: f048ff08                 lock dec   qword ptr [rax]
0000000140b2ea55: 0f8543040000             jne        0x140b2ee9e
0000000140b2ea5b: 4889f9                   mov        rcx, rdi
0000000140b2ea5e: e8cdd80200               call       0x140b5c330
0000000140b2ea63: e936040000               jmp        0x140b2ee9e
0000000140b2ea68: 488d4810                 lea        rcx, [rax + 0x10]
0000000140b2ea6c: f0480fb10b               lock cmpxchg qword ptr [rbx], rcx
0000000140b2ea71: 0f855fffffff             jne        0x140b2e9d6
0000000140b2ea77: 4889d9                   mov        rcx, rbx
0000000140b2ea7a: e8f1311500               call       0x140c81c70
0000000140b2ea7f: 4989de                   mov        r14, rbx
0000000140b2ea82: 4983ce01                 or         r14, 1
0000000140b2ea86: 4c89f1                   mov        rcx, r14
0000000140b2ea89: e8e2311500               call       0x140c81c70
0000000140b2ea8e: 4c8bade0000000           mov        r13, qword ptr [rbp + 0xe0]
0000000140b2ea95: 4983bd8000000000         cmp        qword ptr [r13 + 0x80], 0
0000000140b2ea9d: 0f8424010000             je         0x140b2ebc7
0000000140b2eaa3: 498d5538                 lea        rdx, [r13 + 0x38]
0000000140b2eaa7: 498d8d88000000           lea        rcx, [r13 + 0x88]
0000000140b2eaae: 488995c0000000           mov        qword ptr [rbp + 0xc0], rdx
0000000140b2eab5: e8b622a8ff               call       0x1405b0d70
0000000140b2eaba: 498b4d68                 mov        rcx, qword ptr [r13 + 0x68]
0000000140b2eabe: 498b5570                 mov        rdx, qword ptr [r13 + 0x70]
0000000140b2eac2: 4989d0                   mov        r8, rdx
0000000140b2eac5: 4921c0                   and        r8, rax
0000000140b2eac8: 48c1e839                 shr        rax, 0x39
0000000140b2eacc: 660f6ec0                 movd       xmm0, eax
0000000140b2ead0: 660f60c0                 punpcklbw  xmm0, xmm0
0000000140b2ead4: f20f70c000               pshuflw    xmm0, xmm0, 0
0000000140b2ead9: 660f70c000               pshufd     xmm0, xmm0, 0
0000000140b2eade: 4c8d51f0                 lea        r10, [rcx - 0x10]
0000000140b2eae2: 31c0                     xor        eax, eax
0000000140b2eae4: 660f76c9                 pcmpeqd    xmm1, xmm1
0000000140b2eae8: 4531ed                   xor        r13d, r13d
0000000140b2eaeb: f3420f6f1401             movdqu     xmm2, xmmword ptr [rcx + r8]
0000000140b2eaf1: 660f6fda                 movdqa     xmm3, xmm2
0000000140b2eaf5: 660f74d8                 pcmpeqb    xmm3, xmm0
0000000140b2eaf9: 66440fd7db               pmovmskb   r11d, xmm3
0000000140b2eafe: 4585db                   test       r11d, r11d
0000000140b2eb01: 7476                     je         0x140b2eb79
0000000140b2eb03: 4c8b8dc0000000           mov        r9, qword ptr [rbp + 0xc0]
0000000140b2eb0a: 4d8b09                   mov        r9, qword ptr [r9]
0000000140b2eb0d: 4c894d08                 mov        qword ptr [rbp + 8], r9
0000000140b2eb11: 48899da8000000           mov        qword ptr [rbp + 0xa8], rbx
0000000140b2eb18: 4c89adb8000000           mov        qword ptr [rbp + 0xb8], r13
0000000140b2eb1f: 4c89e3                   mov        rbx, r12
0000000140b2eb22: 4d89fc                   mov        r12, r15
0000000140b2eb25: 4989ff                   mov        r15, rdi
0000000140b2eb28: 4989f5                   mov        r13, rsi
0000000140b2eb2b: 4c89d7                   mov        rdi, r10
0000000140b2eb2e: f3450fbcd3               tzcnt      r10d, r11d
0000000140b2eb33: 4d01c2                   add        r10, r8
0000000140b2eb36: 4921d2                   and        r10, rdx
0000000140b2eb39: 4c89d6                   mov        rsi, r10
0000000140b2eb3c: 48c1e604                 shl        rsi, 4
0000000140b2eb40: 4989f9                   mov        r9, rdi
0000000140b2eb43: 4929f1                   sub        r9, rsi
0000000140b2eb46: 488b7508                 mov        rsi, qword ptr [rbp + 8]
0000000140b2eb4a: 493b31                   cmp        rsi, qword ptr [r9]
0000000140b2eb4d: 744b                     je         0x140b2eb9a
0000000140b2eb4f: 458d4bff                 lea        r9d, [r11 - 1]
0000000140b2eb53: 664521d9                 and        r9w, r11w
0000000140b2eb57: 4589cb                   mov        r11d, r9d
0000000140b2eb5a: 4c89ee                   mov        rsi, r13
0000000140b2eb5d: 4989fa                   mov        r10, rdi
0000000140b2eb60: 4c89ff                   mov        rdi, r15
0000000140b2eb63: 4d89e7                   mov        r15, r12
0000000140b2eb66: 4989dc                   mov        r12, rbx
0000000140b2eb69: 488b9da8000000           mov        rbx, qword ptr [rbp + 0xa8]
0000000140b2eb70: 4c8badb8000000           mov        r13, qword ptr [rbp + 0xb8]
0000000140b2eb77: 75a6                     jne        0x140b2eb1f
0000000140b2eb79: 660f74d1                 pcmpeqb    xmm2, xmm1
0000000140b2eb7d: 66440fd7ca               pmovmskb   r9d, xmm2
0000000140b2eb82: 4585c9                   test       r9d, r9d
0000000140b2eb85: 7533                     jne        0x140b2ebba
0000000140b2eb87: 4d01e8                   add        r8, r13
0000000140b2eb8a: 4983c010                 add        r8, 0x10
0000000140b2eb8e: 4983c510                 add        r13, 0x10
0000000140b2eb92: 4921d0                   and        r8, rdx
0000000140b2eb95: e951ffffff               jmp        0x140b2eaeb
0000000140b2eb9a: 49f7da                   neg        r10
0000000140b2eb9d: 49c1e204                 shl        r10, 4
0000000140b2eba1: 4c01d1                   add        rcx, r10
0000000140b2eba4: 4889c8                   mov        rax, rcx
0000000140b2eba7: 4c89ee                   mov        rsi, r13
0000000140b2ebaa: 4c89ff                   mov        rdi, r15
0000000140b2ebad: 4d89e7                   mov        r15, r12
0000000140b2ebb0: 4989dc                   mov        r12, rbx
0000000140b2ebb3: 488b9da8000000           mov        rbx, qword ptr [rbp + 0xa8]
0000000140b2ebba: 488d48f0                 lea        rcx, [rax - 0x10]
0000000140b2ebbe: 4885c0                   test       rax, rax
0000000140b2ebc1: 480f44c8                 cmove      rcx, rax
0000000140b2ebc5: eb02                     jmp        0x140b2ebc9
0000000140b2ebc7: 31c9                     xor        ecx, ecx
0000000140b2ebc9: 488d4108                 lea        rax, [rcx + 8]
0000000140b2ebcd: 4885c9                   test       rcx, rcx
0000000140b2ebd0: 480f44c7                 cmove      rax, rdi
0000000140b2ebd4: 488b00                   mov        rax, qword ptr [rax]
0000000140b2ebd7: f048ff00                 lock inc   qword ptr [rax]
0000000140b2ebdb: 0f8ee4060000             jle        0x140b2f2c5
0000000140b2ebe1: 48894520                 mov        qword ptr [rbp + 0x20], rax
0000000140b2ebe5: c685df00000001           mov        byte ptr [rbp + 0xdf], 1
0000000140b2ebec: 4889d9                   mov        rcx, rbx
0000000140b2ebef: e81c321500               call       0x140c81e10
0000000140b2ebf4: c685df00000001           mov        byte ptr [rbp + 0xdf], 1
0000000140b2ebfb: 4c89f1                   mov        rcx, r14
0000000140b2ebfe: e80d321500               call       0x140c81e10
0000000140b2ec03: 48c7c0f0ffffff           mov        rax, 0xfffffffffffffff0
0000000140b2ec0a: f0480fc103               lock xadd  qword ptr [rbx], rax
0000000140b2ec0f: 4883e0f2                 and        rax, 0xfffffffffffffff2
0000000140b2ec13: 4883f812                 cmp        rax, 0x12
0000000140b2ec17: 0f8488060000             je         0x140b2f2a5
0000000140b2ec1d: 488b95e0000000           mov        rdx, qword ptr [rbp + 0xe0]
0000000140b2ec24: 488d4a28                 lea        rcx, [rdx + 0x28]
0000000140b2ec28: 488b4130                 mov        rax, qword ptr [rcx + 0x30]
0000000140b2ec2c: 48898590000000           mov        qword ptr [rbp + 0x90], rax
0000000140b2ec33: 0f1001                   movups     xmm0, xmmword ptr [rcx]
0000000140b2ec36: 0f104910                 movups     xmm1, xmmword ptr [rcx + 0x10]
0000000140b2ec3a: 48898dc0000000           mov        qword ptr [rbp + 0xc0], rcx
0000000140b2ec41: 0f105120                 movups     xmm2, xmmword ptr [rcx + 0x20]
0000000140b2ec45: 0f299580000000           movaps     xmmword ptr [rbp + 0x80], xmm2
0000000140b2ec4c: 0f294d70                 movaps     xmmword ptr [rbp + 0x70], xmm1
0000000140b2ec50: 0f294560                 movaps     xmmword ptr [rbp + 0x60], xmm0
0000000140b2ec54: 488b4520                 mov        rax, qword ptr [rbp + 0x20]
0000000140b2ec58: 488985a8000000           mov        qword ptr [rbp + 0xa8], rax
0000000140b2ec5f: 8b461c                   mov        eax, dword ptr [rsi + 0x1c]
0000000140b2ec62: 488b4e10                 mov        rcx, qword ptr [rsi + 0x10]
0000000140b2ec66: 8901                     mov        dword ptr [rcx], eax
0000000140b2ec68: 0fb64218                 movzx      eax, byte ptr [rdx + 0x18]
0000000140b2ec6c: 85c0                     test       eax, eax
0000000140b2ec6e: 7414                     je         0x140b2ec84
0000000140b2ec70: 83f801                   cmp        eax, 1
0000000140b2ec73: 488b07                   mov        rax, qword ptr [rdi]
0000000140b2ec76: f048ff08                 lock dec   qword ptr [rax]
0000000140b2ec7a: 7508                     jne        0x140b2ec84
0000000140b2ec7c: 4889f9                   mov        rcx, rdi
0000000140b2ec7f: e8acd60200               call       0x140b5c330
0000000140b2ec84: 4c8b95e0000000           mov        r10, qword ptr [rbp + 0xe0]
0000000140b2ec8b: 6641c742180204           mov        word ptr [r10 + 0x18], 0x402
0000000140b2ec92: 8b85d0000000             mov        eax, dword ptr [rbp + 0xd0]
0000000140b2ec98: 4189421a                 mov        dword ptr [r10 + 0x1a], eax
0000000140b2ec9c: 0fb785d4000000           movzx      eax, word ptr [rbp + 0xd4]
0000000140b2eca3: 664189421e               mov        word ptr [r10 + 0x1e], ax
0000000140b2eca8: 488b85a8000000           mov        rax, qword ptr [rbp + 0xa8]
0000000140b2ecaf: 49894220                 mov        qword ptr [r10 + 0x20], rax
0000000140b2ecb3: 488b8590000000           mov        rax, qword ptr [rbp + 0x90]
0000000140b2ecba: 488b8dc0000000           mov        rcx, qword ptr [rbp + 0xc0]
0000000140b2ecc1: 48894130                 mov        qword ptr [rcx + 0x30], rax
0000000140b2ecc5: 0f284560                 movaps     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2ecc9: 660f6f4d70               movdqa     xmm1, xmmword ptr [rbp + 0x70]
0000000140b2ecce: 660f6f9580000000         movdqa     xmm2, xmmword ptr [rbp + 0x80]
0000000140b2ecd6: f30f7f5120               movdqu     xmmword ptr [rcx + 0x20], xmm2
0000000140b2ecdb: f30f7f4910               movdqu     xmmword ptr [rcx + 0x10], xmm1
0000000140b2ece0: 0f1101                   movups     xmmword ptr [rcx], xmm0
0000000140b2ece3: 498b4220                 mov        rax, qword ptr [r10 + 0x20]
0000000140b2ece7: 0fb68888000000           movzx      ecx, byte ptr [rax + 0x88]
0000000140b2ecee: 83f902                   cmp        ecx, 2
0000000140b2ecf1: 0f8422020000             je         0x140b2ef19
0000000140b2ecf7: 83f901                   cmp        ecx, 1
0000000140b2ecfa: 755b                     jne        0x140b2ed57
0000000140b2ecfc: 0fb6b889000000           movzx      edi, byte ptr [rax + 0x89]
0000000140b2ed03: 418b471c                 mov        eax, dword ptr [r15 + 0x1c]
0000000140b2ed07: 498b4f10                 mov        rcx, qword ptr [r15 + 0x10]
0000000140b2ed0b: 8901                     mov        dword ptr [rcx], eax
0000000140b2ed0d: 498b1f                   mov        rbx, qword ptr [r15]
0000000140b2ed10: 4c8b7330                 mov        r14, qword ptr [rbx + 0x30]
0000000140b2ed14: 4c8b7b40                 mov        r15, qword ptr [rbx + 0x40]
0000000140b2ed18: 4c3b7320                 cmp        r14, qword ptr [rbx + 0x20]
0000000140b2ed1c: 7510                     jne        0x140b2ed2e
0000000140b2ed1e: 488d4b20                 lea        rcx, [rbx + 0x20]
0000000140b2ed22: 488d151fb41002           lea        rdx, [rip + 0x210b41f]
0000000140b2ed29: e8023fa3ff               call       0x140562c30
0000000140b2ed2e: 488d4340                 lea        rax, [rbx + 0x40]
0000000140b2ed32: 488b5328                 mov        rdx, qword ptr [rbx + 0x28]
0000000140b2ed36: 4883c330                 add        rbx, 0x30
0000000140b2ed3a: 4d89f0                   mov        r8, r14
0000000140b2ed3d: 49c1e005                 shl        r8, 5
0000000140b2ed41: 4a8d0c02                 lea        rcx, [rdx + r8]
0000000140b2ed45: 4e893c02                 mov        qword ptr [rdx + r8], r15
0000000140b2ed49: 42c744020800000000       mov        dword ptr [rdx + r8 + 8], 0
0000000140b2ed52: e99c020000               jmp        0x140b2eff3
0000000140b2ed57: 418b471c                 mov        eax, dword ptr [r15 + 0x1c]
0000000140b2ed5b: 498b4f10                 mov        rcx, qword ptr [r15 + 0x10]
0000000140b2ed5f: 8901                     mov        dword ptr [rcx], eax
0000000140b2ed61: 498b1f                   mov        rbx, qword ptr [r15]
0000000140b2ed64: 4c8b7330                 mov        r14, qword ptr [rbx + 0x30]
0000000140b2ed68: 488b7b40                 mov        rdi, qword ptr [rbx + 0x40]
0000000140b2ed6c: 4c3b7320                 cmp        r14, qword ptr [rbx + 0x20]
0000000140b2ed70: 7510                     jne        0x140b2ed82
0000000140b2ed72: 488d4b20                 lea        rcx, [rbx + 0x20]
0000000140b2ed76: 488d15cbb31002           lea        rdx, [rip + 0x210b3cb]
0000000140b2ed7d: e8ae3ea3ff               call       0x140562c30
0000000140b2ed82: 488d4340                 lea        rax, [rbx + 0x40]
0000000140b2ed86: 488b5328                 mov        rdx, qword ptr [rbx + 0x28]
0000000140b2ed8a: 4883c330                 add        rbx, 0x30
0000000140b2ed8e: 4d89f0                   mov        r8, r14
0000000140b2ed91: 49c1e005                 shl        r8, 5
0000000140b2ed95: 4a8d0c02                 lea        rcx, [rdx + r8]
0000000140b2ed99: 4a893c02                 mov        qword ptr [rdx + r8], rdi
0000000140b2ed9d: 42c744020800000000       mov        dword ptr [rdx + r8 + 8], 0
0000000140b2eda6: 31ff                     xor        edi, edi
0000000140b2eda8: e946020000               jmp        0x140b2eff3
0000000140b2edad: 4885c0                   test       rax, rax
0000000140b2edb0: 0f85d7040000             jne        0x140b2f28d
0000000140b2edb6: 31d2                     xor        edx, edx
0000000140b2edb8: e8139adc01               call       0x1428f87d0
0000000140b2edbd: 4889c1                   mov        rcx, rax
0000000140b2edc0: 4885c0                   test       rax, rax
0000000140b2edc3: 0f85cdf2ffff             jne        0x140b2e096
0000000140b2edc9: e9bf040000               jmp        0x140b2f28d
0000000140b2edce: 4c89d1                   mov        rcx, r10
0000000140b2edd1: 4889da                   mov        rdx, rbx
0000000140b2edd4: e887eeffff               call       0x140b2dc60
0000000140b2edd9: 488985d0000000           mov        qword ptr [rbp + 0xd0], rax
0000000140b2ede0: 488b4838                 mov        rcx, qword ptr [rax + 0x38]
0000000140b2ede4: 48894d70                 mov        qword ptr [rbp + 0x70], rcx
0000000140b2ede8: 488985c0000000           mov        qword ptr [rbp + 0xc0], rax
0000000140b2edef: 0f104028                 movups     xmm0, xmmword ptr [rax + 0x28]
0000000140b2edf3: 0f294560                 movaps     xmmword ptr [rbp + 0x60], xmm0
0000000140b2edf7: 488b9598010000           mov        rdx, qword ptr [rbp + 0x198]
0000000140b2edfe: 8b421c                   mov        eax, dword ptr [rdx + 0x1c]
0000000140b2ee01: 488b4a10                 mov        rcx, qword ptr [rdx + 0x10]
0000000140b2ee05: 8901                     mov        dword ptr [rcx], eax
0000000140b2ee07: 488b1a                   mov        rbx, qword ptr [rdx]
0000000140b2ee0a: 4c8b7330                 mov        r14, qword ptr [rbx + 0x30]
0000000140b2ee0e: 4c8b6b40                 mov        r13, qword ptr [rbx + 0x40]
0000000140b2ee12: 4c3b7320                 cmp        r14, qword ptr [rbx + 0x20]
0000000140b2ee16: 7510                     jne        0x140b2ee28
0000000140b2ee18: 488d4b20                 lea        rcx, [rbx + 0x20]
0000000140b2ee1c: 488d1525b31002           lea        rdx, [rip + 0x210b325]
0000000140b2ee23: e8083ea3ff               call       0x140562c30
0000000140b2ee28: 488b4328                 mov        rax, qword ptr [rbx + 0x28]
0000000140b2ee2c: 4c89f1                   mov        rcx, r14
0000000140b2ee2f: 48c1e105                 shl        rcx, 5
0000000140b2ee33: 488b5570                 mov        rdx, qword ptr [rbp + 0x70]
0000000140b2ee37: 4889540810               mov        qword ptr [rax + rcx + 0x10], rdx
0000000140b2ee3c: 0f284560                 movaps     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2ee40: 0f110408                 movups     xmmword ptr [rax + rcx], xmm0
0000000140b2ee44: 4c896c0818               mov        qword ptr [rax + rcx + 0x18], r13
0000000140b2ee49: 49ffc6                   inc        r14
0000000140b2ee4c: 4c897330                 mov        qword ptr [rbx + 0x30], r14
0000000140b2ee50: 48ff4340                 inc        qword ptr [rbx + 0x40]
0000000140b2ee54: 488b4510                 mov        rax, qword ptr [rbp + 0x10]
0000000140b2ee58: 48894520                 mov        qword ptr [rbp + 0x20], rax
0000000140b2ee5c: 8b4518                   mov        eax, dword ptr [rbp + 0x18]
0000000140b2ee5f: 894528                   mov        dword ptr [rbp + 0x28], eax
0000000140b2ee62: 0f2845e0                 movaps     xmm0, xmmword ptr [rbp - 0x20]
0000000140b2ee66: 0f284df0                 movaps     xmm1, xmmword ptr [rbp - 0x10]
0000000140b2ee6a: 0f294560                 movaps     xmmword ptr [rbp + 0x60], xmm0
0000000140b2ee6e: 0f294d70                 movaps     xmmword ptr [rbp + 0x70], xmm1
0000000140b2ee72: 8b461c                   mov        eax, dword ptr [rsi + 0x1c]
0000000140b2ee75: 488b4e10                 mov        rcx, qword ptr [rsi + 0x10]
0000000140b2ee79: 8901                     mov        dword ptr [rcx], eax
0000000140b2ee7b: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2ee82: 0fb64018                 movzx      eax, byte ptr [rax + 0x18]
0000000140b2ee86: 85c0                     test       eax, eax
0000000140b2ee88: 7414                     je         0x140b2ee9e
0000000140b2ee8a: 83f801                   cmp        eax, 1
0000000140b2ee8d: 488b07                   mov        rax, qword ptr [rdi]
0000000140b2ee90: f048ff08                 lock dec   qword ptr [rax]
0000000140b2ee94: 7508                     jne        0x140b2ee9e
0000000140b2ee96: 4889f9                   mov        rcx, rdi
0000000140b2ee99: e892d40200               call       0x140b5c330
0000000140b2ee9e: 4c8b95e0000000           mov        r10, qword ptr [rbp + 0xe0]
0000000140b2eea5: 6641c742180101           mov        word ptr [r10 + 0x18], 0x101
0000000140b2eeac: 8b85d0000000             mov        eax, dword ptr [rbp + 0xd0]
0000000140b2eeb2: 4189421a                 mov        dword ptr [r10 + 0x1a], eax
0000000140b2eeb6: 0fb785d4000000           movzx      eax, word ptr [rbp + 0xd4]
0000000140b2eebd: 664189421e               mov        word ptr [r10 + 0x1e], ax
0000000140b2eec2: 488b85c0000000           mov        rax, qword ptr [rbp + 0xc0]
0000000140b2eec9: 49894220                 mov        qword ptr [r10 + 0x20], rax
0000000140b2eecd: 8b85cc000000             mov        eax, dword ptr [rbp + 0xcc]
0000000140b2eed3: 41894228                 mov        dword ptr [r10 + 0x28], eax
0000000140b2eed7: 488b4520                 mov        rax, qword ptr [rbp + 0x20]
0000000140b2eedb: 4989422c                 mov        qword ptr [r10 + 0x2c], rax
0000000140b2eedf: 8b4528                   mov        eax, dword ptr [rbp + 0x28]
0000000140b2eee2: 41894234                 mov        dword ptr [r10 + 0x34], eax
0000000140b2eee6: 488b85b8000000           mov        rax, qword ptr [rbp + 0xb8]
0000000140b2eeed: 49894238                 mov        qword ptr [r10 + 0x38], rax
0000000140b2eef1: 0f284560                 movaps     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2eef5: 660f6f4d70               movdqa     xmm1, xmmword ptr [rbp + 0x70]
0000000140b2eefa: 410f114240               movups     xmmword ptr [r10 + 0x40], xmm0
0000000140b2eeff: f3410f7f4a50             movdqu     xmmword ptr [r10 + 0x50], xmm1
0000000140b2ef05: 498b4220                 mov        rax, qword ptr [r10 + 0x20]
0000000140b2ef09: 0fb68888000000           movzx      ecx, byte ptr [rax + 0x88]
0000000140b2ef10: 83f902                   cmp        ecx, 2
0000000140b2ef13: 0f85defdffff             jne        0x140b2ecf7
0000000140b2ef19: 0fb6b889000000           movzx      edi, byte ptr [rax + 0x89]
0000000140b2ef20: f20f10b88c000000         movsd      xmm7, qword ptr [rax + 0x8c]
0000000140b2ef28: f30f10b094000000         movss      xmm6, dword ptr [rax + 0x94]
0000000140b2ef30: 0fb68898000000           movzx      ecx, byte ptr [rax + 0x98]
0000000140b2ef37: 488d1532cbfb01           lea        rdx, [rip + 0x1fbcb32]
0000000140b2ef3e: 48630c8a                 movsxd     rcx, dword ptr [rdx + rcx*4]
0000000140b2ef42: 4801d1                   add        rcx, rdx
0000000140b2ef45: ffe1                     jmp        rcx
0000000140b2ef47: 4531ed                   xor        r13d, r13d
0000000140b2ef4a: eb4f                     jmp        0x140b2ef9b
0000000140b2ef4c: b908000000               mov        ecx, 8
0000000140b2ef51: eb2d                     jmp        0x140b2ef80
0000000140b2ef53: 498b4a40                 mov        rcx, qword ptr [r10 + 0x40]
0000000140b2ef57: 488b90b0000000           mov        rdx, qword ptr [rax + 0xb0]
0000000140b2ef5e: 4839ca                   cmp        rdx, rcx
0000000140b2ef61: 0f8652030000             jbe        0x140b2f2b9
0000000140b2ef67: 488d0c49                 lea        rcx, [rcx + rcx*2]
0000000140b2ef6b: 48c1e102                 shl        rcx, 2
0000000140b2ef6f: 480388a8000000           add        rcx, qword ptr [rax + 0xa8]
0000000140b2ef76: 4889c8                   mov        rax, rcx
0000000140b2ef79: eb0e                     jmp        0x140b2ef89
0000000140b2ef7b: b904000000               mov        ecx, 4
0000000140b2ef80: 480598000000             add        rax, 0x98
0000000140b2ef86: 4801c8                   add        rax, rcx
0000000140b2ef89: f20f1000                 movsd      xmm0, qword ptr [rax]
0000000140b2ef8d: 0f58f8                   addps      xmm7, xmm0
0000000140b2ef90: f30f587008               addss      xmm6, dword ptr [rax + 8]
0000000140b2ef95: 41bd01000000             mov        r13d, 1
0000000140b2ef9b: 418b471c                 mov        eax, dword ptr [r15 + 0x1c]
0000000140b2ef9f: 498b4f10                 mov        rcx, qword ptr [r15 + 0x10]
0000000140b2efa3: 8901                     mov        dword ptr [rcx], eax
0000000140b2efa5: 498b1f                   mov        rbx, qword ptr [r15]
0000000140b2efa8: 4c8b7330                 mov        r14, qword ptr [rbx + 0x30]
0000000140b2efac: 4c8b7b40                 mov        r15, qword ptr [rbx + 0x40]
0000000140b2efb0: 4c3b7320                 cmp        r14, qword ptr [rbx + 0x20]
0000000140b2efb4: 7510                     jne        0x140b2efc6
0000000140b2efb6: 488d4b20                 lea        rcx, [rbx + 0x20]
0000000140b2efba: 488d1587b11002           lea        rdx, [rip + 0x210b187]
0000000140b2efc1: e86a3ca3ff               call       0x140562c30
0000000140b2efc6: 488d4340                 lea        rax, [rbx + 0x40]
0000000140b2efca: 488b5328                 mov        rdx, qword ptr [rbx + 0x28]
0000000140b2efce: 4883c330                 add        rbx, 0x30
0000000140b2efd2: 4d89f0                   mov        r8, r14
0000000140b2efd5: 49c1e005                 shl        r8, 5
0000000140b2efd9: 4a8d0c02                 lea        rcx, [rdx + r8]
0000000140b2efdd: 4e893c02                 mov        qword ptr [rdx + r8], r15
0000000140b2efe1: 46896c0208               mov        dword ptr [rdx + r8 + 8], r13d
0000000140b2efe6: 420f137c020c             movlps     qword ptr [rdx + r8 + 0xc], xmm7
0000000140b2efec: f3420f11740214           movss      dword ptr [rdx + r8 + 0x14], xmm6
0000000140b2eff3: 40887918                 mov        byte ptr [rcx + 0x18], dil
0000000140b2eff7: 49ffc6                   inc        r14
0000000140b2effa: 4c8933                   mov        qword ptr [rbx], r14
0000000140b2effd: 48ff00                   inc        qword ptr [rax]
0000000140b2f000: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2f007: f6405908                 test       byte ptr [rax + 0x59], 8
0000000140b2f00b: 7444                     je         0x140b2f051
0000000140b2f00d: 41833c2403               cmp        dword ptr [r12], 3
0000000140b2f012: 733d                     jae        0x140b2f051
0000000140b2f014: 488b9590010000           mov        rdx, qword ptr [rbp + 0x190]
0000000140b2f01b: 8b421c                   mov        eax, dword ptr [rdx + 0x1c]
0000000140b2f01e: 488b4a10                 mov        rcx, qword ptr [rdx + 0x10]
0000000140b2f022: 8901                     mov        dword ptr [rcx], eax
0000000140b2f024: 488b3a                   mov        rdi, qword ptr [rdx]
0000000140b2f027: 488b5f30                 mov        rbx, qword ptr [rdi + 0x30]
0000000140b2f02b: 4c8b7740                 mov        r14, qword ptr [rdi + 0x40]
0000000140b2f02f: 49bf0200000001000000     movabs     r15, 0x100000002
0000000140b2f039: 483b5f20                 cmp        rbx, qword ptr [rdi + 0x20]
0000000140b2f03d: 754d                     jne        0x140b2f08c
0000000140b2f03f: 488d4f20                 lea        rcx, [rdi + 0x20]
0000000140b2f043: 488d15feb01002           lea        rdx, [rip + 0x210b0fe]
0000000140b2f04a: e8e135a3ff               call       0x140562630
0000000140b2f04f: eb3b                     jmp        0x140b2f08c
0000000140b2f051: 488b9590010000           mov        rdx, qword ptr [rbp + 0x190]
0000000140b2f058: 8b421c                   mov        eax, dword ptr [rdx + 0x1c]
0000000140b2f05b: 488b4a10                 mov        rcx, qword ptr [rdx + 0x10]
0000000140b2f05f: 8901                     mov        dword ptr [rcx], eax
0000000140b2f061: 488b3a                   mov        rdi, qword ptr [rdx]
0000000140b2f064: 488b5f30                 mov        rbx, qword ptr [rdi + 0x30]
0000000140b2f068: 4c8b7740                 mov        r14, qword ptr [rdi + 0x40]
0000000140b2f06c: 49bf0300000001000000     movabs     r15, 0x100000003
0000000140b2f076: 483b5f20                 cmp        rbx, qword ptr [rdi + 0x20]
0000000140b2f07a: 7510                     jne        0x140b2f08c
0000000140b2f07c: 488d4f20                 lea        rcx, [rdi + 0x20]
0000000140b2f080: 488d15c1b01002           lea        rdx, [rip + 0x210b0c1]
0000000140b2f087: e8a435a3ff               call       0x140562630
0000000140b2f08c: 488b4728                 mov        rax, qword ptr [rdi + 0x28]
0000000140b2f090: 4889d9                   mov        rcx, rbx
0000000140b2f093: 48c1e104                 shl        rcx, 4
0000000140b2f097: 4c893c08                 mov        qword ptr [rax + rcx], r15
0000000140b2f09b: 4c89740808               mov        qword ptr [rax + rcx + 8], r14
0000000140b2f0a0: 48ffc3                   inc        rbx
0000000140b2f0a3: 48895f30                 mov        qword ptr [rdi + 0x30], rbx
0000000140b2f0a7: 48ff4740                 inc        qword ptr [rdi + 0x40]
0000000140b2f0ab: 488b8de0000000           mov        rcx, qword ptr [rbp + 0xe0]
0000000140b2f0b2: 80791802                 cmp        byte ptr [rcx + 0x18], 2
0000000140b2f0b6: 0f8242010000             jb         0x140b2f1fe
0000000140b2f0bc: 0fb64119                 movzx      eax, byte ptr [rcx + 0x19]
0000000140b2f0c0: 8d50fe                   lea        edx, [rax - 2]
0000000140b2f0c3: 0fb6d2                   movzx      edx, dl
0000000140b2f0c6: 80fa03                   cmp        dl, 3
0000000140b2f0c9: 41b803000000             mov        r8d, 3
0000000140b2f0cf: 440f42c2                 cmovb      r8d, edx
0000000140b2f0d3: 4180f801                 cmp        r8b, 1
0000000140b2f0d7: 7466                     je         0x140b2f13f
0000000140b2f0d9: 410fb6d0                 movzx      edx, r8b
0000000140b2f0dd: 83fa03                   cmp        edx, 3
0000000140b2f0e0: 0f8518010000             jne        0x140b2f1fe
0000000140b2f0e6: a801                     test       al, 1
0000000140b2f0e8: 0f8498000000             je         0x140b2f186
0000000140b2f0ee: 488b4120                 mov        rax, qword ptr [rcx + 0x20]
0000000140b2f0f2: 488b8880000000           mov        rcx, qword ptr [rax + 0x80]
0000000140b2f0f9: 48894d70                 mov        qword ptr [rbp + 0x70], rcx
0000000140b2f0fd: 0f104070                 movups     xmm0, xmmword ptr [rax + 0x70]
0000000140b2f101: 0f294560                 movaps     xmmword ptr [rbp + 0x60], xmm0
0000000140b2f105: 488b9598010000           mov        rdx, qword ptr [rbp + 0x198]
0000000140b2f10c: 8b421c                   mov        eax, dword ptr [rdx + 0x1c]
0000000140b2f10f: 488b4a10                 mov        rcx, qword ptr [rdx + 0x10]
0000000140b2f113: 8901                     mov        dword ptr [rcx], eax
0000000140b2f115: 488b3a                   mov        rdi, qword ptr [rdx]
0000000140b2f118: 488b5f30                 mov        rbx, qword ptr [rdi + 0x30]
0000000140b2f11c: 4c8b7740                 mov        r14, qword ptr [rdi + 0x40]
0000000140b2f120: 483b5f20                 cmp        rbx, qword ptr [rdi + 0x20]
0000000140b2f124: 0f85a1000000             jne        0x140b2f1cb
0000000140b2f12a: 488d4f20                 lea        rcx, [rdi + 0x20]
0000000140b2f12e: 488d1513b01002           lea        rdx, [rip + 0x210b013]
0000000140b2f135: e8f63aa3ff               call       0x140562c30
0000000140b2f13a: e98c000000               jmp        0x140b2f1cb
0000000140b2f13f: 488b4120                 mov        rax, qword ptr [rcx + 0x20]
0000000140b2f143: 488b4850                 mov        rcx, qword ptr [rax + 0x50]
0000000140b2f147: 48894d70                 mov        qword ptr [rbp + 0x70], rcx
0000000140b2f14b: 0f104040                 movups     xmm0, xmmword ptr [rax + 0x40]
0000000140b2f14f: 0f294560                 movaps     xmmword ptr [rbp + 0x60], xmm0
0000000140b2f153: 488b9598010000           mov        rdx, qword ptr [rbp + 0x198]
0000000140b2f15a: 8b421c                   mov        eax, dword ptr [rdx + 0x1c]
0000000140b2f15d: 488b4a10                 mov        rcx, qword ptr [rdx + 0x10]
0000000140b2f161: 8901                     mov        dword ptr [rcx], eax
0000000140b2f163: 488b3a                   mov        rdi, qword ptr [rdx]
0000000140b2f166: 488b5f30                 mov        rbx, qword ptr [rdi + 0x30]
0000000140b2f16a: 4c8b7740                 mov        r14, qword ptr [rdi + 0x40]
0000000140b2f16e: 483b5f20                 cmp        rbx, qword ptr [rdi + 0x20]
0000000140b2f172: 7557                     jne        0x140b2f1cb
0000000140b2f174: 488d4f20                 lea        rcx, [rdi + 0x20]
0000000140b2f178: 488d15c9af1002           lea        rdx, [rip + 0x210afc9]
0000000140b2f17f: e8ac3aa3ff               call       0x140562c30
0000000140b2f184: eb45                     jmp        0x140b2f1cb
0000000140b2f186: 488b4120                 mov        rax, qword ptr [rcx + 0x20]
0000000140b2f18a: 488b4868                 mov        rcx, qword ptr [rax + 0x68]
0000000140b2f18e: 48894d70                 mov        qword ptr [rbp + 0x70], rcx
0000000140b2f192: 0f104058                 movups     xmm0, xmmword ptr [rax + 0x58]
0000000140b2f196: 0f294560                 movaps     xmmword ptr [rbp + 0x60], xmm0
0000000140b2f19a: 488b9598010000           mov        rdx, qword ptr [rbp + 0x198]
0000000140b2f1a1: 8b421c                   mov        eax, dword ptr [rdx + 0x1c]
0000000140b2f1a4: 488b4a10                 mov        rcx, qword ptr [rdx + 0x10]
0000000140b2f1a8: 8901                     mov        dword ptr [rcx], eax
0000000140b2f1aa: 488b3a                   mov        rdi, qword ptr [rdx]
0000000140b2f1ad: 488b5f30                 mov        rbx, qword ptr [rdi + 0x30]
0000000140b2f1b1: 4c8b7740                 mov        r14, qword ptr [rdi + 0x40]
0000000140b2f1b5: 483b5f20                 cmp        rbx, qword ptr [rdi + 0x20]
0000000140b2f1b9: 7510                     jne        0x140b2f1cb
0000000140b2f1bb: 488d4f20                 lea        rcx, [rdi + 0x20]
0000000140b2f1bf: 488d1582af1002           lea        rdx, [rip + 0x210af82]
0000000140b2f1c6: e8653aa3ff               call       0x140562c30
0000000140b2f1cb: 488b4728                 mov        rax, qword ptr [rdi + 0x28]
0000000140b2f1cf: 4889d9                   mov        rcx, rbx
0000000140b2f1d2: 48c1e105                 shl        rcx, 5
0000000140b2f1d6: 488b5570                 mov        rdx, qword ptr [rbp + 0x70]
0000000140b2f1da: 4889540810               mov        qword ptr [rax + rcx + 0x10], rdx
0000000140b2f1df: 0f284560                 movaps     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2f1e3: 0f110408                 movups     xmmword ptr [rax + rcx], xmm0
0000000140b2f1e7: 4c89740818               mov        qword ptr [rax + rcx + 0x18], r14
0000000140b2f1ec: 48ffc3                   inc        rbx
0000000140b2f1ef: 48895f30                 mov        qword ptr [rdi + 0x30], rbx
0000000140b2f1f3: 48ff4740                 inc        qword ptr [rdi + 0x40]
0000000140b2f1f7: 488b8de0000000           mov        rcx, qword ptr [rbp + 0xe0]
0000000140b2f1fe: 8b461c                   mov        eax, dword ptr [rsi + 0x1c]
0000000140b2f201: 488b5610                 mov        rdx, qword ptr [rsi + 0x10]
0000000140b2f205: 8902                     mov        dword ptr [rdx], eax
0000000140b2f207: 4883c168                 add        rcx, 0x68
0000000140b2f20b: e880ac0400               call       0x140b79e90
0000000140b2f210: 48837dd000               cmp        qword ptr [rbp - 0x30], 0
0000000140b2f215: 7440                     je         0x140b2f257
0000000140b2f217: e8148f0e00               call       0x140c18130
0000000140b2f21c: 4889c1                   mov        rcx, rax
0000000140b2f21f: 488b00                   mov        rax, qword ptr [rax]
0000000140b2f222: 4883f801                 cmp        rax, 1
0000000140b2f226: 7551                     jne        0x140b2f279
0000000140b2f228: 4883c108                 add        rcx, 8
0000000140b2f22c: 48833900                 cmp        qword ptr [rcx], 0
0000000140b2f230: 7567                     jne        0x140b2f299
0000000140b2f232: 48c701ffffffff           mov        qword ptr [rcx], 0xffffffffffffffff
0000000140b2f239: 48898de0000000           mov        qword ptr [rbp + 0xe0], rcx
0000000140b2f240: 4883c108                 add        rcx, 8
0000000140b2f244: 488b55d8                 mov        rdx, qword ptr [rbp - 0x28]
0000000140b2f248: e8a37e0e00               call       0x140c170f0
0000000140b2f24d: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2f254: 48ff00                   inc        qword ptr [rax]
0000000140b2f257: 0f28b5f0000000           movaps     xmm6, xmmword ptr [rbp + 0xf0]
0000000140b2f25e: 0f28bd00010000           movaps     xmm7, xmmword ptr [rbp + 0x100]
0000000140b2f265: 4881c498010000           add        rsp, 0x198
0000000140b2f26c: 5b                       pop        rbx
0000000140b2f26d: 5f                       pop        rdi
0000000140b2f26e: 5e                       pop        rsi
0000000140b2f26f: 415c                     pop        r12
0000000140b2f271: 415d                     pop        r13
0000000140b2f273: 415e                     pop        r14
0000000140b2f275: 415f                     pop        r15
0000000140b2f277: 5d                       pop        rbp
0000000140b2f278: c3                       ret        
0000000140b2f279: 4885c0                   test       rax, rax
0000000140b2f27c: 750f                     jne        0x140b2f28d
0000000140b2f27e: 31d2                     xor        edx, edx
0000000140b2f280: e84b95dc01               call       0x1428f87d0
0000000140b2f285: 4889c1                   mov        rcx, rax
0000000140b2f288: 4885c0                   test       rax, rax
0000000140b2f28b: 759f                     jne        0x140b2f22c
0000000140b2f28d: 488d0d0cb81002           lea        rcx, [rip + 0x210b80c]
0000000140b2f294: e8378eda01               call       0x1428d80d0
0000000140b2f299: 488d0dd0b81002           lea        rcx, [rip + 0x210b8d0]
0000000140b2f2a0: e8ab9fda01               call       0x1428d9250
0000000140b2f2a5: c685df00000001           mov        byte ptr [rbp + 0xdf], 1
0000000140b2f2ac: 4889d9                   mov        rcx, rbx
0000000140b2f2af: e89cdcd901               call       0x1428ccf50
0000000140b2f2b4: e964f9ffff               jmp        0x140b2ec1d
0000000140b2f2b9: 4c8d0568e6f801           lea        r8, [rip + 0x1f8e668]
0000000140b2f2c0: e853a2da01               call       0x1428d9518
0000000140b2f2c5: 0f0b                     ud2        
0000000140b2f2c7: 660f1f840000000000       nop        word ptr [rax + rax]
0000000140b2f2d0: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2f2d5: 55                       push       rbp
0000000140b2f2d6: 4157                     push       r15
0000000140b2f2d8: 4156                     push       r14
0000000140b2f2da: 4155                     push       r13
0000000140b2f2dc: 4154                     push       r12
0000000140b2f2de: 56                       push       rsi
0000000140b2f2df: 57                       push       rdi
0000000140b2f2e0: 53                       push       rbx
0000000140b2f2e1: 4883ec58                 sub        rsp, 0x58
0000000140b2f2e5: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2f2ec: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2f2f1: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2f2f6: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2f2fd: 48ff00                   inc        qword ptr [rax]
0000000140b2f300: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2f305: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2f30a: 4883c458                 add        rsp, 0x58
0000000140b2f30e: 5b                       pop        rbx
0000000140b2f30f: 5f                       pop        rdi
0000000140b2f310: 5e                       pop        rsi
0000000140b2f311: 415c                     pop        r12
0000000140b2f313: 415d                     pop        r13
0000000140b2f315: 415e                     pop        r14
0000000140b2f317: 415f                     pop        r15
0000000140b2f319: 5d                       pop        rbp
0000000140b2f31a: c3                       ret        
0000000140b2f31b: 0f1f440000               nop        dword ptr [rax + rax]
0000000140b2f320: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2f325: 55                       push       rbp
0000000140b2f326: 4157                     push       r15
0000000140b2f328: 4156                     push       r14
0000000140b2f32a: 4155                     push       r13
0000000140b2f32c: 4154                     push       r12
0000000140b2f32e: 56                       push       rsi
0000000140b2f32f: 57                       push       rdi
0000000140b2f330: 53                       push       rbx
0000000140b2f331: 4883ec58                 sub        rsp, 0x58
0000000140b2f335: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2f33c: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2f341: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2f346: 48837dd000               cmp        qword ptr [rbp - 0x30], 0
0000000140b2f34b: 7420                     je         0x140b2f36d
0000000140b2f34d: 488d45d8                 lea        rax, [rbp - 0x28]
0000000140b2f351: 488945c0                 mov        qword ptr [rbp - 0x40], rax
0000000140b2f355: 488d45c0                 lea        rax, [rbp - 0x40]
0000000140b2f359: 488945b8                 mov        qword ptr [rbp - 0x48], rax
0000000140b2f35d: 488d0d04c0fb01           lea        rcx, [rip + 0x1fbc004]
0000000140b2f364: 488d55b8                 lea        rdx, [rbp - 0x48]
0000000140b2f368: e863ff7f00               call       0x14132f2d0
0000000140b2f36d: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2f372: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2f377: 4883c458                 add        rsp, 0x58
0000000140b2f37b: 5b                       pop        rbx
0000000140b2f37c: 5f                       pop        rdi
0000000140b2f37d: 5e                       pop        rsi
0000000140b2f37e: 415c                     pop        r12
0000000140b2f380: 415d                     pop        r13
0000000140b2f382: 415e                     pop        r14
0000000140b2f384: 415f                     pop        r15
0000000140b2f386: 5d                       pop        rbp
0000000140b2f387: c3                       ret        
0000000140b2f388: 0f1f840000000000         nop        dword ptr [rax + rax]
0000000140b2f390: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2f395: 55                       push       rbp
0000000140b2f396: 4157                     push       r15
0000000140b2f398: 4156                     push       r14
0000000140b2f39a: 4155                     push       r13
0000000140b2f39c: 4154                     push       r12
0000000140b2f39e: 56                       push       rsi
0000000140b2f39f: 57                       push       rdi
0000000140b2f3a0: 53                       push       rbx
0000000140b2f3a1: 4883ec58                 sub        rsp, 0x58
0000000140b2f3a5: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2f3ac: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2f3b1: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2f3b6: 488b85b0000000           mov        rax, qword ptr [rbp + 0xb0]
0000000140b2f3bd: f048ff08                 lock dec   qword ptr [rax]
0000000140b2f3c1: 750c                     jne        0x140b2f3cf
0000000140b2f3c3: 488d8dd0000000           lea        rcx, [rbp + 0xd0]
0000000140b2f3ca: e861cf0200               call       0x140b5c330
0000000140b2f3cf: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2f3d4: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2f3d9: 4883c458                 add        rsp, 0x58
0000000140b2f3dd: 5b                       pop        rbx
0000000140b2f3de: 5f                       pop        rdi
0000000140b2f3df: 5e                       pop        rsi
0000000140b2f3e0: 415c                     pop        r12
0000000140b2f3e2: 415d                     pop        r13
0000000140b2f3e4: 415e                     pop        r14
0000000140b2f3e6: 415f                     pop        r15
0000000140b2f3e8: 5d                       pop        rbp
0000000140b2f3e9: c3                       ret        
0000000140b2f3ea: 660f1f440000             nop        word ptr [rax + rax]
0000000140b2f3f0: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2f3f5: 55                       push       rbp
0000000140b2f3f6: 4157                     push       r15
0000000140b2f3f8: 4156                     push       r14
0000000140b2f3fa: 4155                     push       r13
0000000140b2f3fc: 4154                     push       r12
0000000140b2f3fe: 56                       push       rsi
0000000140b2f3ff: 57                       push       rdi
0000000140b2f400: 53                       push       rbx
0000000140b2f401: 4883ec58                 sub        rsp, 0x58
0000000140b2f405: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2f40c: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2f411: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2f416: 488b85b0000000           mov        rax, qword ptr [rbp + 0xb0]
0000000140b2f41d: f048ff08                 lock dec   qword ptr [rax]
0000000140b2f421: 750c                     jne        0x140b2f42f
0000000140b2f423: 488d8dd0000000           lea        rcx, [rbp + 0xd0]
0000000140b2f42a: e801cf0200               call       0x140b5c330
0000000140b2f42f: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2f434: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2f439: 4883c458                 add        rsp, 0x58
0000000140b2f43d: 5b                       pop        rbx
0000000140b2f43e: 5f                       pop        rdi
0000000140b2f43f: 5e                       pop        rsi
0000000140b2f440: 415c                     pop        r12
0000000140b2f442: 415d                     pop        r13
0000000140b2f444: 415e                     pop        r14
0000000140b2f446: 415f                     pop        r15
0000000140b2f448: 5d                       pop        rbp
0000000140b2f449: c3                       ret        
0000000140b2f44a: 660f1f440000             nop        word ptr [rax + rax]
0000000140b2f450: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2f455: 55                       push       rbp
0000000140b2f456: 4157                     push       r15
0000000140b2f458: 4156                     push       r14
0000000140b2f45a: 4155                     push       r13
0000000140b2f45c: 4154                     push       r12
0000000140b2f45e: 56                       push       rsi
0000000140b2f45f: 57                       push       rdi
0000000140b2f460: 53                       push       rbx
0000000140b2f461: 4883ec58                 sub        rsp, 0x58
0000000140b2f465: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2f46c: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2f471: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2f476: 80bddf00000000           cmp        byte ptr [rbp + 0xdf], 0
0000000140b2f47d: 742e                     je         0x140b2f4ad
0000000140b2f47f: 488b4520                 mov        rax, qword ptr [rbp + 0x20]
0000000140b2f483: f048ff08                 lock dec   qword ptr [rax]
0000000140b2f487: 7509                     jne        0x140b2f492
0000000140b2f489: 488d4d20                 lea        rcx, [rbp + 0x20]
0000000140b2f48d: e89ece0200               call       0x140b5c330
0000000140b2f492: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2f497: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2f49c: 4883c458                 add        rsp, 0x58
0000000140b2f4a0: 5b                       pop        rbx
0000000140b2f4a1: 5f                       pop        rdi
0000000140b2f4a2: 5e                       pop        rsi
0000000140b2f4a3: 415c                     pop        r12
0000000140b2f4a5: 415d                     pop        r13
0000000140b2f4a7: 415e                     pop        r14
0000000140b2f4a9: 415f                     pop        r15
0000000140b2f4ab: 5d                       pop        rbp
0000000140b2f4ac: c3                       ret        
0000000140b2f4ad: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2f4b2: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2f4b7: 4883c458                 add        rsp, 0x58
0000000140b2f4bb: 5b                       pop        rbx
0000000140b2f4bc: 5f                       pop        rdi
0000000140b2f4bd: 5e                       pop        rsi
0000000140b2f4be: 415c                     pop        r12
0000000140b2f4c0: 415d                     pop        r13
0000000140b2f4c2: 415e                     pop        r14
0000000140b2f4c4: 415f                     pop        r15
0000000140b2f4c6: 5d                       pop        rbp
0000000140b2f4c7: c3                       ret        
0000000140b2f4c8: 0f1f840000000000         nop        dword ptr [rax + rax]
0000000140b2f4d0: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2f4d5: 55                       push       rbp
0000000140b2f4d6: 4157                     push       r15
0000000140b2f4d8: 4156                     push       r14
0000000140b2f4da: 4155                     push       r13
0000000140b2f4dc: 4154                     push       r12
0000000140b2f4de: 56                       push       rsi
0000000140b2f4df: 57                       push       rdi
0000000140b2f4e0: 53                       push       rbx
0000000140b2f4e1: 4883ec58                 sub        rsp, 0x58
0000000140b2f4e5: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2f4ec: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2f4f1: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2f4f6: 488b85c0000000           mov        rax, qword ptr [rbp + 0xc0]
0000000140b2f4fd: f048ff08                 lock dec   qword ptr [rax]
0000000140b2f501: 750c                     jne        0x140b2f50f
0000000140b2f503: 488d8dd0000000           lea        rcx, [rbp + 0xd0]
0000000140b2f50a: e821ce0200               call       0x140b5c330
0000000140b2f50f: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2f514: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2f519: 4883c458                 add        rsp, 0x58
0000000140b2f51d: 5b                       pop        rbx
0000000140b2f51e: 5f                       pop        rdi
0000000140b2f51f: 5e                       pop        rsi
0000000140b2f520: 415c                     pop        r12
0000000140b2f522: 415d                     pop        r13
0000000140b2f524: 415e                     pop        r14
0000000140b2f526: 415f                     pop        r15
0000000140b2f528: 5d                       pop        rbp
0000000140b2f529: c3                       ret        
0000000140b2f52a: 660f1f440000             nop        word ptr [rax + rax]
0000000140b2f530: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2f535: 55                       push       rbp
0000000140b2f536: 4157                     push       r15
0000000140b2f538: 4156                     push       r14
0000000140b2f53a: 4155                     push       r13
0000000140b2f53c: 4154                     push       r12
0000000140b2f53e: 56                       push       rsi
0000000140b2f53f: 57                       push       rdi
0000000140b2f540: 53                       push       rbx
0000000140b2f541: 4883ec58                 sub        rsp, 0x58
0000000140b2f545: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2f54c: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2f551: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2f556: 488b8de0000000           mov        rcx, qword ptr [rbp + 0xe0]
0000000140b2f55d: 66c741180101             mov        word ptr [rcx + 0x18], 0x101
0000000140b2f563: 8b85d0000000             mov        eax, dword ptr [rbp + 0xd0]
0000000140b2f569: 89411a                   mov        dword ptr [rcx + 0x1a], eax
0000000140b2f56c: 0fb785d4000000           movzx      eax, word ptr [rbp + 0xd4]
0000000140b2f573: 6689411e                 mov        word ptr [rcx + 0x1e], ax
0000000140b2f577: 488b85c0000000           mov        rax, qword ptr [rbp + 0xc0]
0000000140b2f57e: 48894120                 mov        qword ptr [rcx + 0x20], rax
0000000140b2f582: 8b85cc000000             mov        eax, dword ptr [rbp + 0xcc]
0000000140b2f588: 894128                   mov        dword ptr [rcx + 0x28], eax
0000000140b2f58b: 488b4520                 mov        rax, qword ptr [rbp + 0x20]
0000000140b2f58f: 4889412c                 mov        qword ptr [rcx + 0x2c], rax
0000000140b2f593: 8b4528                   mov        eax, dword ptr [rbp + 0x28]
0000000140b2f596: 894134                   mov        dword ptr [rcx + 0x34], eax
0000000140b2f599: 488b85b8000000           mov        rax, qword ptr [rbp + 0xb8]
0000000140b2f5a0: 48894138                 mov        qword ptr [rcx + 0x38], rax
0000000140b2f5a4: 0f284560                 movaps     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2f5a8: 660f6f4d70               movdqa     xmm1, xmmword ptr [rbp + 0x70]
0000000140b2f5ad: 0f114140                 movups     xmmword ptr [rcx + 0x40], xmm0
0000000140b2f5b1: f30f7f4950               movdqu     xmmword ptr [rcx + 0x50], xmm1
0000000140b2f5b6: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2f5bb: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2f5c0: 4883c458                 add        rsp, 0x58
0000000140b2f5c4: 5b                       pop        rbx
0000000140b2f5c5: 5f                       pop        rdi
0000000140b2f5c6: 5e                       pop        rsi
0000000140b2f5c7: 415c                     pop        r12
0000000140b2f5c9: 415d                     pop        r13
0000000140b2f5cb: 415e                     pop        r14
0000000140b2f5cd: 415f                     pop        r15
0000000140b2f5cf: 5d                       pop        rbp
0000000140b2f5d0: c3                       ret        
0000000140b2f5d1: 6666666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b2f5e0: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2f5e5: 55                       push       rbp
0000000140b2f5e6: 4157                     push       r15
0000000140b2f5e8: 4156                     push       r14
0000000140b2f5ea: 4155                     push       r13
0000000140b2f5ec: 4154                     push       r12
0000000140b2f5ee: 56                       push       rsi
0000000140b2f5ef: 57                       push       rdi
0000000140b2f5f0: 53                       push       rbx
0000000140b2f5f1: 4883ec58                 sub        rsp, 0x58
0000000140b2f5f5: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2f5fc: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2f601: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2f606: 488b8de0000000           mov        rcx, qword ptr [rbp + 0xe0]
0000000140b2f60d: 66c741180101             mov        word ptr [rcx + 0x18], 0x101
0000000140b2f613: 8b85d0000000             mov        eax, dword ptr [rbp + 0xd0]
0000000140b2f619: 89411a                   mov        dword ptr [rcx + 0x1a], eax
0000000140b2f61c: 0fb785d4000000           movzx      eax, word ptr [rbp + 0xd4]
0000000140b2f623: 6689411e                 mov        word ptr [rcx + 0x1e], ax
0000000140b2f627: 488b85c0000000           mov        rax, qword ptr [rbp + 0xc0]
0000000140b2f62e: 48894120                 mov        qword ptr [rcx + 0x20], rax
0000000140b2f632: 8b85cc000000             mov        eax, dword ptr [rbp + 0xcc]
0000000140b2f638: 894128                   mov        dword ptr [rcx + 0x28], eax
0000000140b2f63b: 488b4520                 mov        rax, qword ptr [rbp + 0x20]
0000000140b2f63f: 4889412c                 mov        qword ptr [rcx + 0x2c], rax
0000000140b2f643: 8b4528                   mov        eax, dword ptr [rbp + 0x28]
0000000140b2f646: 894134                   mov        dword ptr [rcx + 0x34], eax
0000000140b2f649: 488b85b8000000           mov        rax, qword ptr [rbp + 0xb8]
0000000140b2f650: 48894138                 mov        qword ptr [rcx + 0x38], rax
0000000140b2f654: 0f284560                 movaps     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2f658: 660f6f4d70               movdqa     xmm1, xmmword ptr [rbp + 0x70]
0000000140b2f65d: 0f114140                 movups     xmmword ptr [rcx + 0x40], xmm0
0000000140b2f661: f30f7f4950               movdqu     xmmword ptr [rcx + 0x50], xmm1
0000000140b2f666: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2f66b: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2f670: 4883c458                 add        rsp, 0x58
0000000140b2f674: 5b                       pop        rbx
0000000140b2f675: 5f                       pop        rdi
0000000140b2f676: 5e                       pop        rsi
0000000140b2f677: 415c                     pop        r12
0000000140b2f679: 415d                     pop        r13
0000000140b2f67b: 415e                     pop        r14
0000000140b2f67d: 415f                     pop        r15
0000000140b2f67f: 5d                       pop        rbp
0000000140b2f680: c3                       ret        
0000000140b2f681: 6666666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b2f690: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2f695: 55                       push       rbp
0000000140b2f696: 4157                     push       r15
0000000140b2f698: 4156                     push       r14
0000000140b2f69a: 4155                     push       r13
0000000140b2f69c: 4154                     push       r12
0000000140b2f69e: 56                       push       rsi
0000000140b2f69f: 57                       push       rdi
0000000140b2f6a0: 53                       push       rbx
0000000140b2f6a1: 4883ec58                 sub        rsp, 0x58
0000000140b2f6a5: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2f6ac: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2f6b1: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2f6b6: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2f6bd: 48ff00                   inc        qword ptr [rax]
0000000140b2f6c0: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2f6c5: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2f6ca: 4883c458                 add        rsp, 0x58
0000000140b2f6ce: 5b                       pop        rbx
0000000140b2f6cf: 5f                       pop        rdi
0000000140b2f6d0: 5e                       pop        rsi
0000000140b2f6d1: 415c                     pop        r12
0000000140b2f6d3: 415d                     pop        r13
0000000140b2f6d5: 415e                     pop        r14
0000000140b2f6d7: 415f                     pop        r15
0000000140b2f6d9: 5d                       pop        rbp
0000000140b2f6da: c3                       ret        
0000000140b2f6db: 0f1f440000               nop        dword ptr [rax + rax]
0000000140b2f6e0: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2f6e5: 55                       push       rbp
0000000140b2f6e6: 4157                     push       r15
0000000140b2f6e8: 4156                     push       r14
0000000140b2f6ea: 4155                     push       r13
0000000140b2f6ec: 4154                     push       r12
0000000140b2f6ee: 56                       push       rsi
0000000140b2f6ef: 57                       push       rdi
0000000140b2f6f0: 53                       push       rbx
0000000140b2f6f1: 4883ec58                 sub        rsp, 0x58
0000000140b2f6f5: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2f6fc: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2f701: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2f706: 488b8de0000000           mov        rcx, qword ptr [rbp + 0xe0]
0000000140b2f70d: 66c741180101             mov        word ptr [rcx + 0x18], 0x101
0000000140b2f713: 8b85d0000000             mov        eax, dword ptr [rbp + 0xd0]
0000000140b2f719: 89411a                   mov        dword ptr [rcx + 0x1a], eax
0000000140b2f71c: 0fb785d4000000           movzx      eax, word ptr [rbp + 0xd4]
0000000140b2f723: 6689411e                 mov        word ptr [rcx + 0x1e], ax
0000000140b2f727: 488b85b0000000           mov        rax, qword ptr [rbp + 0xb0]
0000000140b2f72e: 48894120                 mov        qword ptr [rcx + 0x20], rax
0000000140b2f732: 8b85cc000000             mov        eax, dword ptr [rbp + 0xcc]
0000000140b2f738: 894128                   mov        dword ptr [rcx + 0x28], eax
0000000140b2f73b: 488b4520                 mov        rax, qword ptr [rbp + 0x20]
0000000140b2f73f: 4889412c                 mov        qword ptr [rcx + 0x2c], rax
0000000140b2f743: 8b4528                   mov        eax, dword ptr [rbp + 0x28]
0000000140b2f746: 894134                   mov        dword ptr [rcx + 0x34], eax
0000000140b2f749: 488b85b8000000           mov        rax, qword ptr [rbp + 0xb8]
0000000140b2f750: 48894138                 mov        qword ptr [rcx + 0x38], rax
0000000140b2f754: 0f284560                 movaps     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2f758: 660f6f4d70               movdqa     xmm1, xmmword ptr [rbp + 0x70]
0000000140b2f75d: 0f114140                 movups     xmmword ptr [rcx + 0x40], xmm0
0000000140b2f761: f30f7f4950               movdqu     xmmword ptr [rcx + 0x50], xmm1
0000000140b2f766: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2f76b: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2f770: 4883c458                 add        rsp, 0x58
0000000140b2f774: 5b                       pop        rbx
0000000140b2f775: 5f                       pop        rdi
0000000140b2f776: 5e                       pop        rsi
0000000140b2f777: 415c                     pop        r12
0000000140b2f779: 415d                     pop        r13
0000000140b2f77b: 415e                     pop        r14
0000000140b2f77d: 415f                     pop        r15
0000000140b2f77f: 5d                       pop        rbp
0000000140b2f780: c3                       ret        
0000000140b2f781: 6666666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b2f790: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2f795: 55                       push       rbp
0000000140b2f796: 4157                     push       r15
0000000140b2f798: 4156                     push       r14
0000000140b2f79a: 4155                     push       r13
0000000140b2f79c: 4154                     push       r12
0000000140b2f79e: 56                       push       rsi
0000000140b2f79f: 57                       push       rdi
0000000140b2f7a0: 53                       push       rbx
0000000140b2f7a1: 4883ec58                 sub        rsp, 0x58
0000000140b2f7a5: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2f7ac: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2f7b1: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2f7b6: 488b8de0000000           mov        rcx, qword ptr [rbp + 0xe0]
0000000140b2f7bd: 66c741180101             mov        word ptr [rcx + 0x18], 0x101
0000000140b2f7c3: 8b85d0000000             mov        eax, dword ptr [rbp + 0xd0]
0000000140b2f7c9: 89411a                   mov        dword ptr [rcx + 0x1a], eax
0000000140b2f7cc: 0fb785d4000000           movzx      eax, word ptr [rbp + 0xd4]
0000000140b2f7d3: 6689411e                 mov        word ptr [rcx + 0x1e], ax
0000000140b2f7d7: 488b85b0000000           mov        rax, qword ptr [rbp + 0xb0]
0000000140b2f7de: 48894120                 mov        qword ptr [rcx + 0x20], rax
0000000140b2f7e2: 8b85cc000000             mov        eax, dword ptr [rbp + 0xcc]
0000000140b2f7e8: 894128                   mov        dword ptr [rcx + 0x28], eax
0000000140b2f7eb: 488b4520                 mov        rax, qword ptr [rbp + 0x20]
0000000140b2f7ef: 4889412c                 mov        qword ptr [rcx + 0x2c], rax
0000000140b2f7f3: 8b4528                   mov        eax, dword ptr [rbp + 0x28]
0000000140b2f7f6: 894134                   mov        dword ptr [rcx + 0x34], eax
0000000140b2f7f9: 488b85b8000000           mov        rax, qword ptr [rbp + 0xb8]
0000000140b2f800: 48894138                 mov        qword ptr [rcx + 0x38], rax
0000000140b2f804: 0f284560                 movaps     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2f808: 660f6f4d70               movdqa     xmm1, xmmword ptr [rbp + 0x70]
0000000140b2f80d: 0f114140                 movups     xmmword ptr [rcx + 0x40], xmm0
0000000140b2f811: f30f7f4950               movdqu     xmmword ptr [rcx + 0x50], xmm1
0000000140b2f816: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2f81b: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2f820: 4883c458                 add        rsp, 0x58
0000000140b2f824: 5b                       pop        rbx
0000000140b2f825: 5f                       pop        rdi
0000000140b2f826: 5e                       pop        rsi
0000000140b2f827: 415c                     pop        r12
0000000140b2f829: 415d                     pop        r13
0000000140b2f82b: 415e                     pop        r14
0000000140b2f82d: 415f                     pop        r15
0000000140b2f82f: 5d                       pop        rbp
0000000140b2f830: c3                       ret        
0000000140b2f831: 6666666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b2f840: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2f845: 55                       push       rbp
0000000140b2f846: 4157                     push       r15
0000000140b2f848: 4156                     push       r14
0000000140b2f84a: 4155                     push       r13
0000000140b2f84c: 4154                     push       r12
0000000140b2f84e: 56                       push       rsi
0000000140b2f84f: 57                       push       rdi
0000000140b2f850: 53                       push       rbx
0000000140b2f851: 4883ec58                 sub        rsp, 0x58
0000000140b2f855: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2f85c: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2f861: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2f866: 488b8de0000000           mov        rcx, qword ptr [rbp + 0xe0]
0000000140b2f86d: 66c741180101             mov        word ptr [rcx + 0x18], 0x101
0000000140b2f873: 8b85d0000000             mov        eax, dword ptr [rbp + 0xd0]
0000000140b2f879: 89411a                   mov        dword ptr [rcx + 0x1a], eax
0000000140b2f87c: 0fb785d4000000           movzx      eax, word ptr [rbp + 0xd4]
0000000140b2f883: 6689411e                 mov        word ptr [rcx + 0x1e], ax
0000000140b2f887: 488b85b0000000           mov        rax, qword ptr [rbp + 0xb0]
0000000140b2f88e: 48894120                 mov        qword ptr [rcx + 0x20], rax
0000000140b2f892: 8b85cc000000             mov        eax, dword ptr [rbp + 0xcc]
0000000140b2f898: 894128                   mov        dword ptr [rcx + 0x28], eax
0000000140b2f89b: 488b4520                 mov        rax, qword ptr [rbp + 0x20]
0000000140b2f89f: 4889412c                 mov        qword ptr [rcx + 0x2c], rax
0000000140b2f8a3: 8b4528                   mov        eax, dword ptr [rbp + 0x28]
0000000140b2f8a6: 894134                   mov        dword ptr [rcx + 0x34], eax
0000000140b2f8a9: 488b85b8000000           mov        rax, qword ptr [rbp + 0xb8]
0000000140b2f8b0: 48894138                 mov        qword ptr [rcx + 0x38], rax
0000000140b2f8b4: 0f284560                 movaps     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2f8b8: 660f6f4d70               movdqa     xmm1, xmmword ptr [rbp + 0x70]
0000000140b2f8bd: 0f114140                 movups     xmmword ptr [rcx + 0x40], xmm0
0000000140b2f8c1: f30f7f4950               movdqu     xmmword ptr [rcx + 0x50], xmm1
0000000140b2f8c6: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2f8cb: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2f8d0: 4883c458                 add        rsp, 0x58
0000000140b2f8d4: 5b                       pop        rbx
0000000140b2f8d5: 5f                       pop        rdi
0000000140b2f8d6: 5e                       pop        rsi
0000000140b2f8d7: 415c                     pop        r12
0000000140b2f8d9: 415d                     pop        r13
0000000140b2f8db: 415e                     pop        r14
0000000140b2f8dd: 415f                     pop        r15
0000000140b2f8df: 5d                       pop        rbp
0000000140b2f8e0: c3                       ret        
0000000140b2f8e1: 6666666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b2f8f0: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2f8f5: 55                       push       rbp
0000000140b2f8f6: 4157                     push       r15
0000000140b2f8f8: 4156                     push       r14
0000000140b2f8fa: 4155                     push       r13
0000000140b2f8fc: 4154                     push       r12
0000000140b2f8fe: 56                       push       rsi
0000000140b2f8ff: 57                       push       rdi
0000000140b2f900: 53                       push       rbx
0000000140b2f901: 4883ec58                 sub        rsp, 0x58
0000000140b2f905: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2f90c: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2f911: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2f916: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2f91d: c6401800                 mov        byte ptr [rax + 0x18], 0
0000000140b2f921: 488b859f000000           mov        rax, qword ptr [rbp + 0x9f]
0000000140b2f928: 488b8db0000000           mov        rcx, qword ptr [rbp + 0xb0]
0000000140b2f92f: 4889413f                 mov        qword ptr [rcx + 0x3f], rax
0000000140b2f933: 0f104560                 movups     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2f937: f30f6f4d70               movdqu     xmm1, xmmword ptr [rbp + 0x70]
0000000140b2f93c: f30f6f9580000000         movdqu     xmm2, xmmword ptr [rbp + 0x80]
0000000140b2f944: f30f6f9d90000000         movdqu     xmm3, xmmword ptr [rbp + 0x90]
0000000140b2f94c: f30f7f5930               movdqu     xmmword ptr [rcx + 0x30], xmm3
0000000140b2f951: f30f7f5120               movdqu     xmmword ptr [rcx + 0x20], xmm2
0000000140b2f956: f30f7f4910               movdqu     xmmword ptr [rcx + 0x10], xmm1
0000000140b2f95b: 0f1101                   movups     xmmword ptr [rcx], xmm0
0000000140b2f95e: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2f963: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2f968: 4883c458                 add        rsp, 0x58
0000000140b2f96c: 5b                       pop        rbx
0000000140b2f96d: 5f                       pop        rdi
0000000140b2f96e: 5e                       pop        rsi
0000000140b2f96f: 415c                     pop        r12
0000000140b2f971: 415d                     pop        r13
0000000140b2f973: 415e                     pop        r14
0000000140b2f975: 415f                     pop        r15
0000000140b2f977: 5d                       pop        rbp
0000000140b2f978: c3                       ret        
0000000140b2f979: 0f1f8000000000           nop        dword ptr [rax]
0000000140b2f980: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2f985: 55                       push       rbp
0000000140b2f986: 4157                     push       r15
0000000140b2f988: 4156                     push       r14
0000000140b2f98a: 4155                     push       r13
0000000140b2f98c: 4154                     push       r12
0000000140b2f98e: 56                       push       rsi
0000000140b2f98f: 57                       push       rdi
0000000140b2f990: 53                       push       rbx
0000000140b2f991: 4883ec58                 sub        rsp, 0x58
0000000140b2f995: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2f99c: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2f9a1: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2f9a6: 488b8de0000000           mov        rcx, qword ptr [rbp + 0xe0]
0000000140b2f9ad: 66c741180203             mov        word ptr [rcx + 0x18], 0x302
0000000140b2f9b3: 8b4560                   mov        eax, dword ptr [rbp + 0x60]
0000000140b2f9b6: 89411a                   mov        dword ptr [rcx + 0x1a], eax
0000000140b2f9b9: 0fb74564                 movzx      eax, word ptr [rbp + 0x64]
0000000140b2f9bd: 6689411e                 mov        word ptr [rcx + 0x1e], ax
0000000140b2f9c1: 488b85c0000000           mov        rax, qword ptr [rbp + 0xc0]
0000000140b2f9c8: 48894120                 mov        qword ptr [rcx + 0x20], rax
0000000140b2f9cc: 488b4550                 mov        rax, qword ptr [rbp + 0x50]
0000000140b2f9d0: 488b8da8000000           mov        rcx, qword ptr [rbp + 0xa8]
0000000140b2f9d7: 48894130                 mov        qword ptr [rcx + 0x30], rax
0000000140b2f9db: 0f284520                 movaps     xmm0, xmmword ptr [rbp + 0x20]
0000000140b2f9df: 660f6f4d30               movdqa     xmm1, xmmword ptr [rbp + 0x30]
0000000140b2f9e4: 660f6f5540               movdqa     xmm2, xmmword ptr [rbp + 0x40]
0000000140b2f9e9: f30f7f5120               movdqu     xmmword ptr [rcx + 0x20], xmm2
0000000140b2f9ee: f30f7f4910               movdqu     xmmword ptr [rcx + 0x10], xmm1
0000000140b2f9f3: 0f1101                   movups     xmmword ptr [rcx], xmm0
0000000140b2f9f6: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2f9fb: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2fa00: 4883c458                 add        rsp, 0x58
0000000140b2fa04: 5b                       pop        rbx
0000000140b2fa05: 5f                       pop        rdi
0000000140b2fa06: 5e                       pop        rsi
0000000140b2fa07: 415c                     pop        r12
0000000140b2fa09: 415d                     pop        r13
0000000140b2fa0b: 415e                     pop        r14
0000000140b2fa0d: 415f                     pop        r15
0000000140b2fa0f: 5d                       pop        rbp
0000000140b2fa10: c3                       ret        
0000000140b2fa11: 6666666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b2fa20: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2fa25: 55                       push       rbp
0000000140b2fa26: 4157                     push       r15
0000000140b2fa28: 4156                     push       r14
0000000140b2fa2a: 4155                     push       r13
0000000140b2fa2c: 4154                     push       r12
0000000140b2fa2e: 56                       push       rsi
0000000140b2fa2f: 57                       push       rdi
0000000140b2fa30: 53                       push       rbx
0000000140b2fa31: 4883ec58                 sub        rsp, 0x58
0000000140b2fa35: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2fa3c: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2fa41: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2fa46: 488b8de0000000           mov        rcx, qword ptr [rbp + 0xe0]
0000000140b2fa4d: 66c741180204             mov        word ptr [rcx + 0x18], 0x402
0000000140b2fa53: 8b85d0000000             mov        eax, dword ptr [rbp + 0xd0]
0000000140b2fa59: 89411a                   mov        dword ptr [rcx + 0x1a], eax
0000000140b2fa5c: 0fb785d4000000           movzx      eax, word ptr [rbp + 0xd4]
0000000140b2fa63: 6689411e                 mov        word ptr [rcx + 0x1e], ax
0000000140b2fa67: 488b85a8000000           mov        rax, qword ptr [rbp + 0xa8]
0000000140b2fa6e: 48894120                 mov        qword ptr [rcx + 0x20], rax
0000000140b2fa72: 488b8590000000           mov        rax, qword ptr [rbp + 0x90]
0000000140b2fa79: 488b8dc0000000           mov        rcx, qword ptr [rbp + 0xc0]
0000000140b2fa80: 48894130                 mov        qword ptr [rcx + 0x30], rax
0000000140b2fa84: 660f6f4560               movdqa     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2fa89: 660f6f4d70               movdqa     xmm1, xmmword ptr [rbp + 0x70]
0000000140b2fa8e: 660f6f9580000000         movdqa     xmm2, xmmword ptr [rbp + 0x80]
0000000140b2fa96: f30f7f5120               movdqu     xmmword ptr [rcx + 0x20], xmm2
0000000140b2fa9b: f30f7f4910               movdqu     xmmword ptr [rcx + 0x10], xmm1
0000000140b2faa0: f30f7f01                 movdqu     xmmword ptr [rcx], xmm0
0000000140b2faa4: c685df00000000           mov        byte ptr [rbp + 0xdf], 0
0000000140b2faab: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2fab0: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2fab5: 4883c458                 add        rsp, 0x58
0000000140b2fab9: 5b                       pop        rbx
0000000140b2faba: 5f                       pop        rdi
0000000140b2fabb: 5e                       pop        rsi
0000000140b2fabc: 415c                     pop        r12
0000000140b2fabe: 415d                     pop        r13
0000000140b2fac0: 415e                     pop        r14
0000000140b2fac2: 415f                     pop        r15
0000000140b2fac4: 5d                       pop        rbp
0000000140b2fac5: c3                       ret        
0000000140b2fac6: 662e0f1f840000000000     nop        word ptr cs:[rax + rax]
0000000140b2fad0: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2fad5: 55                       push       rbp
0000000140b2fad6: 4157                     push       r15
0000000140b2fad8: 4156                     push       r14
0000000140b2fada: 4155                     push       r13
0000000140b2fadc: 4154                     push       r12
0000000140b2fade: 56                       push       rsi
0000000140b2fadf: 57                       push       rdi
0000000140b2fae0: 53                       push       rbx
0000000140b2fae1: 4883ec58                 sub        rsp, 0x58
0000000140b2fae5: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2faec: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2faf1: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2faf6: 488b8de0000000           mov        rcx, qword ptr [rbp + 0xe0]
0000000140b2fafd: c6411802                 mov        byte ptr [rcx + 0x18], 2
0000000140b2fb01: 0fb64508                 movzx      eax, byte ptr [rbp + 8]
0000000140b2fb05: 884119                   mov        byte ptr [rcx + 0x19], al
0000000140b2fb08: 8b4560                   mov        eax, dword ptr [rbp + 0x60]
0000000140b2fb0b: 89411a                   mov        dword ptr [rcx + 0x1a], eax
0000000140b2fb0e: 0fb74564                 movzx      eax, word ptr [rbp + 0x64]
0000000140b2fb12: 6689411e                 mov        word ptr [rcx + 0x1e], ax
0000000140b2fb16: 488b85c0000000           mov        rax, qword ptr [rbp + 0xc0]
0000000140b2fb1d: 48894120                 mov        qword ptr [rcx + 0x20], rax
0000000140b2fb21: 488b4550                 mov        rax, qword ptr [rbp + 0x50]
0000000140b2fb25: 488b8da8000000           mov        rcx, qword ptr [rbp + 0xa8]
0000000140b2fb2c: 48894130                 mov        qword ptr [rcx + 0x30], rax
0000000140b2fb30: 0f284520                 movaps     xmm0, xmmword ptr [rbp + 0x20]
0000000140b2fb34: 660f6f4d30               movdqa     xmm1, xmmword ptr [rbp + 0x30]
0000000140b2fb39: 660f6f5540               movdqa     xmm2, xmmword ptr [rbp + 0x40]
0000000140b2fb3e: f30f7f5120               movdqu     xmmword ptr [rcx + 0x20], xmm2
0000000140b2fb43: f30f7f4910               movdqu     xmmword ptr [rcx + 0x10], xmm1
0000000140b2fb48: 0f1101                   movups     xmmword ptr [rcx], xmm0
0000000140b2fb4b: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2fb50: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2fb55: 4883c458                 add        rsp, 0x58
0000000140b2fb59: 5b                       pop        rbx
0000000140b2fb5a: 5f                       pop        rdi
0000000140b2fb5b: 5e                       pop        rsi
0000000140b2fb5c: 415c                     pop        r12
0000000140b2fb5e: 415d                     pop        r13
0000000140b2fb60: 415e                     pop        r14
0000000140b2fb62: 415f                     pop        r15
0000000140b2fb64: 5d                       pop        rbp
0000000140b2fb65: c3                       ret        
0000000140b2fb66: 662e0f1f840000000000     nop        word ptr cs:[rax + rax]
0000000140b2fb70: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2fb75: 55                       push       rbp
0000000140b2fb76: 4157                     push       r15
0000000140b2fb78: 4156                     push       r14
0000000140b2fb7a: 4155                     push       r13
0000000140b2fb7c: 4154                     push       r12
0000000140b2fb7e: 56                       push       rsi
0000000140b2fb7f: 57                       push       rdi
0000000140b2fb80: 53                       push       rbx
0000000140b2fb81: 4883ec58                 sub        rsp, 0x58
0000000140b2fb85: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140b2fb8c: 0f297c2430               movaps     xmmword ptr [rsp + 0x30], xmm7
0000000140b2fb91: 0f29742440               movaps     xmmword ptr [rsp + 0x40], xmm6
0000000140b2fb96: 488b85e0000000           mov        rax, qword ptr [rbp + 0xe0]
0000000140b2fb9d: c6401800                 mov        byte ptr [rax + 0x18], 0
0000000140b2fba1: 488b859f000000           mov        rax, qword ptr [rbp + 0x9f]
0000000140b2fba8: 488b8dc0000000           mov        rcx, qword ptr [rbp + 0xc0]
0000000140b2fbaf: 4889413f                 mov        qword ptr [rcx + 0x3f], rax
0000000140b2fbb3: 0f104560                 movups     xmm0, xmmword ptr [rbp + 0x60]
0000000140b2fbb7: f30f6f4d70               movdqu     xmm1, xmmword ptr [rbp + 0x70]
0000000140b2fbbc: f30f6f9580000000         movdqu     xmm2, xmmword ptr [rbp + 0x80]
0000000140b2fbc4: f30f6f9d90000000         movdqu     xmm3, xmmword ptr [rbp + 0x90]
0000000140b2fbcc: f30f7f5930               movdqu     xmmword ptr [rcx + 0x30], xmm3
0000000140b2fbd1: f30f7f5120               movdqu     xmmword ptr [rcx + 0x20], xmm2
0000000140b2fbd6: f30f7f4910               movdqu     xmmword ptr [rcx + 0x10], xmm1
0000000140b2fbdb: 0f1101                   movups     xmmword ptr [rcx], xmm0
0000000140b2fbde: 0f28742440               movaps     xmm6, xmmword ptr [rsp + 0x40]
0000000140b2fbe3: 0f287c2430               movaps     xmm7, xmmword ptr [rsp + 0x30]
0000000140b2fbe8: 4883c458                 add        rsp, 0x58
0000000140b2fbec: 5b                       pop        rbx
0000000140b2fbed: 5f                       pop        rdi
0000000140b2fbee: 5e                       pop        rsi
0000000140b2fbef: 415c                     pop        r12
0000000140b2fbf1: 415d                     pop        r13
0000000140b2fbf3: 415e                     pop        r14
0000000140b2fbf5: 415f                     pop        r15
0000000140b2fbf7: 5d                       pop        rbp
0000000140b2fbf8: c3                       ret        