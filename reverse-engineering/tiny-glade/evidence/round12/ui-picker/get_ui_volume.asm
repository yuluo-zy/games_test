0000000140b2dc60: 55                       push       rbp
0000000140b2dc61: 4157                     push       r15
0000000140b2dc63: 4156                     push       r14
0000000140b2dc65: 56                       push       rsi
0000000140b2dc66: 57                       push       rdi
0000000140b2dc67: 53                       push       rbx
0000000140b2dc68: 4883ec48                 sub        rsp, 0x48
0000000140b2dc6c: 488d6c2440               lea        rbp, [rsp + 0x40]
0000000140b2dc71: 48c74500feffffff         mov        qword ptr [rbp], 0xfffffffffffffffe
0000000140b2dc79: 4889ce                   mov        rsi, rcx
0000000140b2dc7c: 488955f8                 mov        qword ptr [rbp - 8], rdx
0000000140b2dc80: 4c8d7160                 lea        r14, [rcx + 0x60]
0000000140b2dc84: 488b4160                 mov        rax, qword ptr [rcx + 0x60]
0000000140b2dc88: 89c1                     mov        ecx, eax
0000000140b2dc8a: 83e108                   and        ecx, 8
0000000140b2dc8d: c1e903                   shr        ecx, 3
0000000140b2dc90: 4883f8f0                 cmp        rax, -0x10
0000000140b2dc94: 0f93c2                   setae      dl
0000000140b2dc97: 08ca                     or         dl, cl
0000000140b2dc99: 7412                     je         0x140b2dcad
0000000140b2dc9b: 4c89f1                   mov        rcx, r14
0000000140b2dc9e: 31d2                     xor        edx, edx
0000000140b2dca0: 41b900ca9a3b             mov        r9d, 0x3b9aca00
0000000140b2dca6: e805e8d901               call       0x1428cc4b0
0000000140b2dcab: eb0b                     jmp        0x140b2dcb8
0000000140b2dcad: 488d4810                 lea        rcx, [rax + 0x10]
0000000140b2dcb1: f0490fb10e               lock cmpxchg qword ptr [r14], rcx
0000000140b2dcb6: 75e3                     jne        0x140b2dc9b
0000000140b2dcb8: 4c89f1                   mov        rcx, r14
0000000140b2dcbb: e8b03f1500               call       0x140c81c70
0000000140b2dcc0: 4d89f7                   mov        r15, r14
0000000140b2dcc3: 4983cf01                 or         r15, 1
0000000140b2dcc7: 4c89f9                   mov        rcx, r15
0000000140b2dcca: e8a13f1500               call       0x140c81c70
0000000140b2dccf: 4883be8000000000         cmp        qword ptr [rsi + 0x80], 0
0000000140b2dcd7: 0f84f9000000             je         0x140b2ddd6
0000000140b2dcdd: 488d8e88000000           lea        rcx, [rsi + 0x88]
0000000140b2dce4: 488d55f8                 lea        rdx, [rbp - 8]
0000000140b2dce8: e88330a8ff               call       0x1405b0d70
0000000140b2dced: 488b4e68                 mov        rcx, qword ptr [rsi + 0x68]
0000000140b2dcf1: 488b5670                 mov        rdx, qword ptr [rsi + 0x70]
0000000140b2dcf5: 4989d0                   mov        r8, rdx
0000000140b2dcf8: 4921c0                   and        r8, rax
0000000140b2dcfb: 48c1e839                 shr        rax, 0x39
0000000140b2dcff: 660f6ec0                 movd       xmm0, eax
0000000140b2dd03: 660f60c0                 punpcklbw  xmm0, xmm0
0000000140b2dd07: f20f70c000               pshuflw    xmm0, xmm0, 0
0000000140b2dd0c: 660f70c000               pshufd     xmm0, xmm0, 0
0000000140b2dd11: 488d41f0                 lea        rax, [rcx - 0x10]
0000000140b2dd15: 4531c9                   xor        r9d, r9d
0000000140b2dd18: 4c8b55f8                 mov        r10, qword ptr [rbp - 8]
0000000140b2dd1c: 660f76c9                 pcmpeqd    xmm1, xmm1
0000000140b2dd20: f3420f6f1401             movdqu     xmm2, xmmword ptr [rcx + r8]
0000000140b2dd26: 660f6fda                 movdqa     xmm3, xmm2
0000000140b2dd2a: 660f74d8                 pcmpeqb    xmm3, xmm0
0000000140b2dd2e: 66440fd7db               pmovmskb   r11d, xmm3
0000000140b2dd33: 4585db                   test       r11d, r11d
0000000140b2dd36: 7438                     je         0x140b2dd70
0000000140b2dd38: f3410fbcf3               tzcnt      esi, r11d
0000000140b2dd3d: 4c01c6                   add        rsi, r8
0000000140b2dd40: 4821d6                   and        rsi, rdx
0000000140b2dd43: 4889f7                   mov        rdi, rsi
0000000140b2dd46: 48c1e704                 shl        rdi, 4
0000000140b2dd4a: 4889c3                   mov        rbx, rax
0000000140b2dd4d: 4829fb                   sub        rbx, rdi
0000000140b2dd50: 4c3b13                   cmp        r10, qword ptr [rbx]
0000000140b2dd53: 7439                     je         0x140b2dd8e
0000000140b2dd55: 418d73ff                 lea        esi, [r11 - 1]
0000000140b2dd59: 664421de                 and        si, r11w
0000000140b2dd5d: 4189f3                   mov        r11d, esi
0000000140b2dd60: 75d6                     jne        0x140b2dd38
0000000140b2dd62: 66666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000140b2dd70: 660f74d1                 pcmpeqb    xmm2, xmm1
0000000140b2dd74: 66440fd7da               pmovmskb   r11d, xmm2
0000000140b2dd79: 4585db                   test       r11d, r11d
0000000140b2dd7c: 7558                     jne        0x140b2ddd6
0000000140b2dd7e: 4d01c8                   add        r8, r9
0000000140b2dd81: 4983c010                 add        r8, 0x10
0000000140b2dd85: 4983c110                 add        r9, 0x10
0000000140b2dd89: 4921d0                   and        r8, rdx
0000000140b2dd8c: eb92                     jmp        0x140b2dd20
0000000140b2dd8e: 48f7de                   neg        rsi
0000000140b2dd91: 48c1e604                 shl        rsi, 4
0000000140b2dd95: 488b7431f8               mov        rsi, qword ptr [rcx + rsi - 8]
0000000140b2dd9a: f048ff06                 lock inc   qword ptr [rsi]
0000000140b2dd9e: 7e4a                     jle        0x140b2ddea
0000000140b2dda0: 4c89f1                   mov        rcx, r14
0000000140b2dda3: e868401500               call       0x140c81e10
0000000140b2dda8: 4c89f9                   mov        rcx, r15
0000000140b2ddab: e860401500               call       0x140c81e10
0000000140b2ddb0: 48c7c0f0ffffff           mov        rax, 0xfffffffffffffff0
0000000140b2ddb7: f0490fc106               lock xadd  qword ptr [r14], rax
0000000140b2ddbc: 4883e0f2                 and        rax, 0xfffffffffffffff2
0000000140b2ddc0: 4883f812                 cmp        rax, 0x12
0000000140b2ddc4: 7426                     je         0x140b2ddec
0000000140b2ddc6: 4889f0                   mov        rax, rsi
0000000140b2ddc9: 4883c448                 add        rsp, 0x48
0000000140b2ddcd: 5b                       pop        rbx
0000000140b2ddce: 5f                       pop        rdi
0000000140b2ddcf: 5e                       pop        rsi
0000000140b2ddd0: 415e                     pop        r14
0000000140b2ddd2: 415f                     pop        r15
0000000140b2ddd4: 5d                       pop        rbp
0000000140b2ddd5: c3                       ret        
0000000140b2ddd6: 4c897df0                 mov        qword ptr [rbp - 0x10], r15
0000000140b2ddda: 4c8975e8                 mov        qword ptr [rbp - 0x18], r14
0000000140b2ddde: 488d0ddbdafb01           lea        rcx, [rip + 0x1fbdadb]
0000000140b2dde5: e826b5da01               call       0x1428d9310
0000000140b2ddea: 0f0b                     ud2        
0000000140b2ddec: 4c89f1                   mov        rcx, r14
0000000140b2ddef: e85cf1d901               call       0x1428ccf50
0000000140b2ddf4: ebd0                     jmp        0x140b2ddc6
0000000140b2ddf6: 662e0f1f840000000000     nop        word ptr cs:[rax + rax]
0000000140b2de00: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140b2de05: 55                       push       rbp
0000000140b2de06: 4157                     push       r15
0000000140b2de08: 4156                     push       r14
0000000140b2de0a: 56                       push       rsi
0000000140b2de0b: 57                       push       rdi
0000000140b2de0c: 53                       push       rbx
0000000140b2de0d: 4883ec28                 sub        rsp, 0x28
0000000140b2de11: 488d6a40                 lea        rbp, [rdx + 0x40]
0000000140b2de15: 488b75e8                 mov        rsi, qword ptr [rbp - 0x18]
0000000140b2de19: 4889f1                   mov        rcx, rsi
0000000140b2de1c: e8ef3f1500               call       0x140c81e10
0000000140b2de21: 488b4df0                 mov        rcx, qword ptr [rbp - 0x10]
0000000140b2de25: e8e63f1500               call       0x140c81e10
0000000140b2de2a: 48c7c0f0ffffff           mov        rax, 0xfffffffffffffff0
0000000140b2de31: f0480fc106               lock xadd  qword ptr [rsi], rax
0000000140b2de36: 4883e0f2                 and        rax, 0xfffffffffffffff2
0000000140b2de3a: 4883f812                 cmp        rax, 0x12
0000000140b2de3e: 740d                     je         0x140b2de4d
0000000140b2de40: 4883c428                 add        rsp, 0x28
0000000140b2de44: 5b                       pop        rbx
0000000140b2de45: 5f                       pop        rdi
0000000140b2de46: 5e                       pop        rsi
0000000140b2de47: 415e                     pop        r14
0000000140b2de49: 415f                     pop        r15
0000000140b2de4b: 5d                       pop        rbp
0000000140b2de4c: c3                       ret        
0000000140b2de4d: 4889f1                   mov        rcx, rsi
0000000140b2de50: e8fbf0d901               call       0x1428ccf50
0000000140b2de55: ebe9                     jmp        0x140b2de40