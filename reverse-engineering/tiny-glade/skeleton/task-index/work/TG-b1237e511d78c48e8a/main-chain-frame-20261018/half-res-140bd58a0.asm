0000000140bd58a0  4889c8                   mov        rax, rcx
0000000140bd58a3  c7421801000000           mov        dword ptr [rdx + 0x18], 1
0000000140bd58aa  4489421c                 mov        dword ptr [rdx + 0x1c], r8d
0000000140bd58ae  0f1002                   movups     xmm0, xmmword ptr [rdx]
0000000140bd58b1  0f104a20                 movups     xmm1, xmmword ptr [rdx + 0x20]
0000000140bd58b5  0f10522c                 movups     xmm2, xmmword ptr [rdx + 0x2c]
0000000140bd58b9  0f11512c                 movups     xmmword ptr [rcx + 0x2c], xmm2
0000000140bd58bd  0f114920                 movups     xmmword ptr [rcx + 0x20], xmm1
0000000140bd58c1  0f1101                   movups     xmmword ptr [rcx], xmm0
0000000140bd58c4  488b4a10                 mov        rcx, qword ptr [rdx + 0x10]
0000000140bd58c8  48894810                 mov        qword ptr [rax + 0x10], rcx
0000000140bd58cc  8b4a18                   mov        ecx, dword ptr [rdx + 0x18]
0000000140bd58cf  894818                   mov        dword ptr [rax + 0x18], ecx
0000000140bd58d2  8b4a1c                   mov        ecx, dword ptr [rdx + 0x1c]
0000000140bd58d5  89481c                   mov        dword ptr [rax + 0x1c], ecx
0000000140bd58d8  c3                       ret        
