0000000140945c60: 55                       push       rbp
0000000140945c61: 56                       push       rsi
0000000140945c62: 57                       push       rdi
0000000140945c63: 4881ec10020000           sub        rsp, 0x210
0000000140945c6a: 488dac2480000000         lea        rbp, [rsp + 0x80]
0000000140945c72: 48c78588010000feffffff   mov        qword ptr [rbp + 0x188], 0xfffffffffffffffe
0000000140945c7d: 4889ce                   mov        rsi, rcx
0000000140945c80: 488d4da0                 lea        rcx, [rbp - 0x60]
0000000140945c84: e8f746e100               call       0x14175a380
0000000140945c89: c6858701000006           mov        byte ptr [rbp + 0x187], 6
0000000140945c90: 488b0dd966ce02           mov        rcx, qword ptr [rip + 0x2ce66d9]
0000000140945c97: 4c8d05c2a03102           lea        r8, [rip + 0x231a0c2]
0000000140945c9e: 488d9587010000           lea        rdx, [rbp + 0x187]
0000000140945ca5: e8f6bb1d00               call       0x140b218a0
0000000140945caa: 4989d0                   mov        r8, rdx
0000000140945cad: 488d4da0                 lea        rcx, [rbp - 0x60]
0000000140945cb1: 4889c2                   mov        rdx, rax
0000000140945cb4: e847d3b000               call       0x141453000
0000000140945cb9: 0f104500                 movups     xmm0, xmmword ptr [rbp]
0000000140945cbd: 0f298570010000           movaps     xmmword ptr [rbp + 0x170], xmm0
0000000140945cc4: 0f1045f0                 movups     xmm0, xmmword ptr [rbp - 0x10]
0000000140945cc8: 0f298560010000           movaps     xmmword ptr [rbp + 0x160], xmm0
0000000140945ccf: 0f1045e0                 movups     xmm0, xmmword ptr [rbp - 0x20]
0000000140945cd3: 0f298550010000           movaps     xmmword ptr [rbp + 0x150], xmm0
0000000140945cda: 0f1045a0                 movups     xmm0, xmmword ptr [rbp - 0x60]
0000000140945cde: 0f104db0                 movups     xmm1, xmmword ptr [rbp - 0x50]
0000000140945ce2: 0f1055c0                 movups     xmm2, xmmword ptr [rbp - 0x40]
0000000140945ce6: 0f105dd0                 movups     xmm3, xmmword ptr [rbp - 0x30]
0000000140945cea: 0f299d40010000           movaps     xmmword ptr [rbp + 0x140], xmm3
0000000140945cf1: 0f299530010000           movaps     xmmword ptr [rbp + 0x130], xmm2
0000000140945cf8: 0f298d20010000           movaps     xmmword ptr [rbp + 0x120], xmm1
0000000140945cff: 0f298510010000           movaps     xmmword ptr [rbp + 0x110], xmm0
0000000140945d06: 488d9510010000           lea        rdx, [rbp + 0x110]
0000000140945d0d: 4889f1                   mov        rcx, rsi
0000000140945d10: e86b152200               call       0x140b67280
0000000140945d15: 31c9                     xor        ecx, ecx
0000000140945d17: e8043e79ff               call       0x1400d9b20
0000000140945d1c: 4885c0                   test       rax, rax
0000000140945d1f: 0f8464010000             je         0x140945e89
0000000140945d25: 48c7450000000000         mov        qword ptr [rbp], 0
0000000140945d2d: 0f1000                   movups     xmm0, xmmword ptr [rax]
0000000140945d30: 48ff00                   inc        qword ptr [rax]
0000000140945d33: 0f100d369d1602           movups     xmm1, xmmword ptr [rip + 0x2169d36]
0000000140945d3a: 0f114d18                 movups     xmmword ptr [rbp + 0x18], xmm1
0000000140945d3e: 0f100d1b9d1602           movups     xmm1, xmmword ptr [rip + 0x2169d1b]
0000000140945d45: 0f114d08                 movups     xmmword ptr [rbp + 8], xmm1
0000000140945d49: 0f114528                 movups     xmmword ptr [rbp + 0x28], xmm0
0000000140945d4d: 48c745a000000000         mov        qword ptr [rbp - 0x60], 0
0000000140945d55: 48c745a808000000         mov        qword ptr [rbp - 0x58], 8
0000000140945d5d: 48c745b000000000         mov        qword ptr [rbp - 0x50], 0
0000000140945d65: c645b800                 mov        byte ptr [rbp - 0x48], 0
0000000140945d69: 488d55a0                 lea        rdx, [rbp - 0x60]
0000000140945d6d: 4889f1                   mov        rcx, rsi
0000000140945d70: e85b3bbf00               call       0x1415398d0
0000000140945d75: 48c745a000000000         mov        qword ptr [rbp - 0x60], 0
0000000140945d7d: 48c745b000000000         mov        qword ptr [rbp - 0x50], 0
0000000140945d85: c745c800ca9a3b           mov        dword ptr [rbp - 0x38], 0x3b9aca00
0000000140945d8c: c645d000                 mov        byte ptr [rbp - 0x30], 0
0000000140945d90: 488d55a0                 lea        rdx, [rbp - 0x60]
0000000140945d94: 4889c1                   mov        rcx, rax
0000000140945d97: e8e4a5bf00               call       0x141540380
0000000140945d9c: 4889c6                   mov        rsi, rax
0000000140945d9f: 4889c7                   mov        rdi, rax
0000000140945da2: 4883c738                 add        rdi, 0x38
0000000140945da6: 48ba9e57c5fc8e591739     movabs     rdx, 0x3917598efcc5579e
0000000140945db0: 49b837fb5a2f96f5a8ab     movabs     r8, 0xaba8f5962f5afb37
0000000140945dba: 4889f9                   mov        rcx, rdi
0000000140945dbd: e82e7979ff               call       0x1400dd6f0
0000000140945dc2: 4885c0                   test       rax, rax
0000000140945dc5: 7433                     je         0x140945dfa
0000000140945dc7: 48399618020000           cmp        qword ptr [rsi + 0x218], rdx
0000000140945dce: 762a                     jbe        0x140945dfa
0000000140945dd0: 488b8610020000           mov        rax, qword ptr [rsi + 0x210]
0000000140945dd7: 488b04d0                 mov        rax, qword ptr [rax + rdx*8]
0000000140945ddb: 4885c0                   test       rax, rax
0000000140945dde: 741a                     je         0x140945dfa
0000000140945de0: 488b8ee0010000           mov        rcx, qword ptr [rsi + 0x1e0]
0000000140945de7: 48f7d0                   not        rax
0000000140945dea: 488d0440                 lea        rax, [rax + rax*2]
0000000140945dee: 48c1e005                 shl        rax, 5
0000000140945df2: 48837c012000             cmp        qword ptr [rcx + rax + 0x20], 0
0000000140945df8: 7508                     jne        0x140945e02
0000000140945dfa: 4889f1                   mov        rcx, rsi
0000000140945dfd: e8ce9dbc00               call       0x14150fbd0
0000000140945e02: 48ba993ac8411d61882a     movabs     rdx, 0x2a88611d41c83a99
0000000140945e0c: 49b84ed2f85b5225813f     movabs     r8, 0x3f8125525bf8d24e
0000000140945e16: 4889f9                   mov        rcx, rdi
0000000140945e19: e8d27879ff               call       0x1400dd6f0
0000000140945e1e: 4885c0                   test       rax, rax
0000000140945e21: 7433                     je         0x140945e56
0000000140945e23: 48399618020000           cmp        qword ptr [rsi + 0x218], rdx
0000000140945e2a: 762a                     jbe        0x140945e56
0000000140945e2c: 488b8610020000           mov        rax, qword ptr [rsi + 0x210]
0000000140945e33: 488b04d0                 mov        rax, qword ptr [rax + rdx*8]
0000000140945e37: 4885c0                   test       rax, rax
0000000140945e3a: 741a                     je         0x140945e56
0000000140945e3c: 488b8ee0010000           mov        rcx, qword ptr [rsi + 0x1e0]
0000000140945e43: 48f7d0                   not        rax
0000000140945e46: 488d0440                 lea        rax, [rax + rax*2]
0000000140945e4a: 48c1e005                 shl        rax, 5
0000000140945e4e: 48837c012000             cmp        qword ptr [rcx + rax + 0x20], 0
0000000140945e54: 7508                     jne        0x140945e5e
0000000140945e56: 4889f1                   mov        rcx, rsi
0000000140945e59: e8f26bbc00               call       0x14150ca50
0000000140945e5e: 488d7da0                 lea        rdi, [rbp - 0x60]
0000000140945e62: 41b870010000             mov        r8d, 0x170
0000000140945e68: 4889f9                   mov        rcx, rdi
0000000140945e6b: 31d2                     xor        edx, edx
0000000140945e6d: e8aeddfd01               call       0x142923c20
0000000140945e72: 4889f1                   mov        rcx, rsi
0000000140945e75: 4889fa                   mov        rdx, rdi
0000000140945e78: e8330ac000               call       0x1415468b0
0000000140945e7d: 90                       nop        
0000000140945e7e: 4881c410020000           add        rsp, 0x210
0000000140945e85: 5f                       pop        rdi
0000000140945e86: 5e                       pop        rsi
0000000140945e87: 5d                       pop        rbp
0000000140945e88: c3                       ret        
0000000140945e89: 488d0d104c2f02           lea        rcx, [rip + 0x22f4c10]
0000000140945e90: e83b22f901               call       0x1428d80d0
0000000140945e95: cc                       int3       
0000000140945e96: 662e0f1f840000000000     nop        word ptr cs:[rax + rax]
0000000140945ea0: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000140945ea5: 55                       push       rbp
0000000140945ea6: 56                       push       rsi
0000000140945ea7: 57                       push       rdi
0000000140945ea8: 4883ec20                 sub        rsp, 0x20
0000000140945eac: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000140945eb3: 488d4da0                 lea        rcx, [rbp - 0x60]
0000000140945eb7: e8c4396eff               call       0x140029880
0000000140945ebc: 90                       nop        
0000000140945ebd: 4883c420                 add        rsp, 0x20
0000000140945ec1: 5f                       pop        rdi
0000000140945ec2: 5e                       pop        rsi
0000000140945ec3: 5d                       pop        rbp
0000000140945ec4: c3                       ret        