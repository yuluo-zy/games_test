0000000141339a30: 55                       push       rbp
0000000141339a31: 4881ec80000000           sub        rsp, 0x80
0000000141339a38: 488dac2480000000         lea        rbp, [rsp + 0x80]
0000000141339a40: 48c745f8feffffff         mov        qword ptr [rbp - 8], 0xfffffffffffffffe
0000000141339a48: 498d40ff                 lea        rax, [r8 - 1]
0000000141339a4c: 4839d0                   cmp        rax, rdx
0000000141339a4f: 0f8324010000             jae        0x141339b79
0000000141339a55: 4939d0                   cmp        r8, rdx
0000000141339a58: 7509                     jne        0x141339a63
0000000141339a5a: 4881c480000000           add        rsp, 0x80
0000000141339a61: 5d                       pop        rbp
0000000141339a62: c3                       ret        
0000000141339a63: 4989c9                   mov        r9, rcx
0000000141339a66: 486bc238                 imul       rax, rdx, 0x38
0000000141339a6a: 4801c8                   add        rax, rcx
0000000141339a6d: 496bc838                 imul       rcx, r8, 0x38
0000000141339a71: 498d1409                 lea        rdx, [r9 + rcx]
0000000141339a75: eb41                     jmp        0x141339ab8
0000000141339a77: 660f1f840000000000       nop        word ptr [rax + rax]
0000000141339a80: 4d89ca                   mov        r10, r9
0000000141339a83: 0f2845a0                 movaps     xmm0, xmmword ptr [rbp - 0x60]
0000000141339a87: 0f284db0                 movaps     xmm1, xmmword ptr [rbp - 0x50]
0000000141339a8b: 410f114a10               movups     xmmword ptr [r10 + 0x10], xmm1
0000000141339a90: 410f1102                 movups     xmmword ptr [r10], xmm0
0000000141339a94: f3410f115a20             movss      dword ptr [r10 + 0x20], xmm3
0000000141339a9a: 0f2845e0                 movaps     xmm0, xmmword ptr [rbp - 0x20]
0000000141339a9e: 410f114224               movups     xmmword ptr [r10 + 0x24], xmm0
0000000141339aa3: 448b45f0                 mov        r8d, dword ptr [rbp - 0x10]
0000000141339aa7: 45894234                 mov        dword ptr [r10 + 0x34], r8d
0000000141339aab: 4883c238                 add        rdx, 0x38
0000000141339aaf: 4883c138                 add        rcx, 0x38
0000000141339ab3: 4839c2                   cmp        rdx, rax
0000000141339ab6: 74a2                     je         0x141339a5a
0000000141339ab8: f30f1042e8               movss      xmm0, dword ptr [rdx - 0x18]
0000000141339abd: f30f105a20               movss      xmm3, dword ptr [rdx + 0x20]
0000000141339ac2: 0f2ed8                   ucomiss    xmm3, xmm0
0000000141339ac5: 0f8ab0000000             jp         0x141339b7b
0000000141339acb: 0f2ec3                   ucomiss    xmm0, xmm3
0000000141339ace: 76db                     jbe        0x141339aab
0000000141339ad0: 0f1002                   movups     xmm0, xmmword ptr [rdx]
0000000141339ad3: 0f104a10                 movups     xmm1, xmmword ptr [rdx + 0x10]
0000000141339ad7: 0f2945a0                 movaps     xmmword ptr [rbp - 0x60], xmm0
0000000141339adb: 0f294db0                 movaps     xmmword ptr [rbp - 0x50], xmm1
0000000141339adf: 0f104224                 movups     xmm0, xmmword ptr [rdx + 0x24]
0000000141339ae3: 0f2945e0                 movaps     xmmword ptr [rbp - 0x20], xmm0
0000000141339ae7: 448b4234                 mov        r8d, dword ptr [rdx + 0x34]
0000000141339aeb: 448945f0                 mov        dword ptr [rbp - 0x10], r8d
0000000141339aef: 4989ca                   mov        r10, rcx
0000000141339af2: 66666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
0000000141339b00: 4f8b4411f8               mov        r8, qword ptr [r9 + r10 - 8]
0000000141339b05: 4f89441130               mov        qword ptr [r9 + r10 + 0x30], r8
0000000141339b0a: 430f104411c8             movups     xmm0, xmmword ptr [r9 + r10 - 0x38]
0000000141339b10: 430f104c11d8             movups     xmm1, xmmword ptr [r9 + r10 - 0x28]
0000000141339b16: 430f105411e8             movups     xmm2, xmmword ptr [r9 + r10 - 0x18]
0000000141339b1c: 430f11541120             movups     xmmword ptr [r9 + r10 + 0x20], xmm2
0000000141339b22: 430f114c1110             movups     xmmword ptr [r9 + r10 + 0x10], xmm1
0000000141339b28: 430f110411               movups     xmmword ptr [r9 + r10], xmm0
0000000141339b2d: 4983fa38                 cmp        r10, 0x38
0000000141339b31: 0f8449ffffff             je         0x141339a80
0000000141339b37: f3430f104411b0           movss      xmm0, dword ptr [r9 + r10 - 0x50]
0000000141339b3e: 0f2ed8                   ucomiss    xmm3, xmm0
0000000141339b41: 7a11                     jp         0x141339b54
0000000141339b43: 4983c2c8                 add        r10, -0x38
0000000141339b47: 0f2ec3                   ucomiss    xmm0, xmm3
0000000141339b4a: 77b4                     ja         0x141339b00
0000000141339b4c: 4d01ca                   add        r10, r9
0000000141339b4f: e92fffffff               jmp        0x141339a83
0000000141339b54: f30f115df4               movss      dword ptr [rbp - 0xc], xmm3
0000000141339b59: 4c8955c8                 mov        qword ptr [rbp - 0x38], r10
0000000141339b5d: 4b8d0411                 lea        rax, [r9 + r10]
0000000141339b61: 4883c0c8                 add        rax, -0x38
0000000141339b65: 488945d8                 mov        qword ptr [rbp - 0x28], rax
0000000141339b69: 488d0dd8149001           lea        rcx, [rip + 0x19014d8]
0000000141339b70: 4c894dd0                 mov        qword ptr [rbp - 0x30], r9
0000000141339b74: e897f75901               call       0x1428d9310
0000000141339b79: 0f0b                     ud2        
0000000141339b7b: 488d0dc6149001           lea        rcx, [rip + 0x19014c6]
0000000141339b82: e889f75901               call       0x1428d9310
0000000141339b87: cc                       int3       
0000000141339b88: 0f1f840000000000         nop        dword ptr [rax + rax]
0000000141339b90: 4889542410               mov        qword ptr [rsp + 0x10], rdx
0000000141339b95: 55                       push       rbp
0000000141339b96: 4883ec20                 sub        rsp, 0x20
0000000141339b9a: 488daa80000000           lea        rbp, [rdx + 0x80]
0000000141339ba1: 0f2845a0                 movaps     xmm0, xmmword ptr [rbp - 0x60]
0000000141339ba5: 0f284db0                 movaps     xmm1, xmmword ptr [rbp - 0x50]
0000000141339ba9: 488b45d8                 mov        rax, qword ptr [rbp - 0x28]
0000000141339bad: 0f114810                 movups     xmmword ptr [rax + 0x10], xmm1
0000000141339bb1: 0f1100                   movups     xmmword ptr [rax], xmm0
0000000141339bb4: f30f1045f4               movss      xmm0, dword ptr [rbp - 0xc]
0000000141339bb9: 488b4dc8                 mov        rcx, qword ptr [rbp - 0x38]
0000000141339bbd: 488b55d0                 mov        rdx, qword ptr [rbp - 0x30]
0000000141339bc1: f30f11440ae8             movss      dword ptr [rdx + rcx - 0x18], xmm0
0000000141339bc7: 0f2845e0                 movaps     xmm0, xmmword ptr [rbp - 0x20]
0000000141339bcb: 0f11440aec               movups     xmmword ptr [rdx + rcx - 0x14], xmm0
0000000141339bd0: 8b45f0                   mov        eax, dword ptr [rbp - 0x10]
0000000141339bd3: 89440afc                 mov        dword ptr [rdx + rcx - 4], eax
0000000141339bd7: 4883c420                 add        rsp, 0x20
0000000141339bdb: 5d                       pop        rbp
0000000141339bdc: c3                       ret        