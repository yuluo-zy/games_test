0x1416feeb0 mov      rcx, qword ptr [rcx]
0x1416feeb3 mov      eax, dword ptr [r8 + 0x1c]
0x1416feeb7 mov      r9, qword ptr [r8 + 0x10]
0x1416feebb mov      dword ptr [r9], eax
0x1416feebe mov      rax, qword ptr [r8]
0x1416feec1 mov      rdx, qword ptr [rdx]
0x1416feec4 movss    xmm0, dword ptr [rdx + 0x58]
0x1416feec9 movsd    xmm1, qword ptr [rcx + 0x58]
0x1416feece movsd    qword ptr [rax + 0x140], xmm1
0x1416feed6 movsd    xmm1, qword ptr [rcx + 0x60]
0x1416feedb movsd    qword ptr [rax + 0x148], xmm1
0x1416feee3 movsd    xmm1, qword ptr [rcx + 0x68]
0x1416feee8 movsd    qword ptr [rax + 0x150], xmm1
0x1416feef0 movsd    xmm1, qword ptr [rcx + 0x78]
0x1416feef5 movsd    qword ptr [rax + 0x160], xmm1
0x1416feefd movsd    xmm1, qword ptr [rcx + 0x70]
0x1416fef02 movsd    qword ptr [rax + 0x158], xmm1
0x1416fef0a movsd    xmm1, qword ptr [rcx + 0x80]
0x1416fef12 movsd    qword ptr [rax + 0x168], xmm1
0x1416fef1a movzx    edx, byte ptr [rcx + 0x88]
0x1416fef21 cmp      byte ptr [rcx + 0x30], 0
0x1416fef25 je       0x1416fef4d
0x1416fef27 mov      r8d, dword ptr [rax]
0x1416fef2a movss    xmm1, dword ptr [rax + 4]
0x1416fef2f test     dl, dl
0x1416fef31 je       0x1416fef58
0x1416fef33 cmp      r8d, 2
0x1416fef37 je       0x1416fef68
0x1416fef39 mov      edx, 1
0x1416fef3e xorps    xmm2, xmm2
0x1416fef41 cmp      r8d, 1
0x1416fef45 jne      0x1416fef8c
0x1416fef47 addss    xmm1, xmm0
0x1416fef4b jmp      0x1416fef89
0x1416fef4d xor      dl, 1
0x1416fef50 movzx    edx, dl
0x1416fef53 shl      edx, 2
0x1416fef56 jmp      0x1416fef8c
0x1416fef58 cmp      r8d, 1
0x1416fef5c je       0x1416fef61
0x1416fef5e xorps    xmm1, xmm1
0x1416fef61 mov      edx, 3
0x1416fef66 jmp      0x1416fef89
0x1416fef68 xorps    xmm2, xmm2
0x1416fef6b ucomiss  xmm2, xmm1
0x1416fef6e jb       0x1416fef80
0x1416fef70 addss    xmm1, dword ptr [rip + 0x1226a0c]
0x1416fef78 maxss    xmm1, dword ptr [rip + 0x1231df0]
0x1416fef80 subss    xmm1, xmm0
0x1416fef84 mov      edx, 2
0x1416fef89 movaps   xmm2, xmm1
0x1416fef8c mov      dword ptr [rax], edx
0x1416fef8e movss    dword ptr [rax + 4], xmm2
0x1416fef93 movzx    edx, byte ptr [rcx + 0x89]
0x1416fef9a cmp      byte ptr [rcx + 0x31], 0
0x1416fef9e je       0x1416fefc7
0x1416fefa0 mov      r8d, dword ptr [rax + 8]
0x1416fefa4 movss    xmm1, dword ptr [rax + 0xc]
0x1416fefa9 test     dl, dl
0x1416fefab je       0x1416fefd2
0x1416fefad cmp      r8d, 2
0x1416fefb1 je       0x1416fefe2
0x1416fefb3 mov      edx, 1
0x1416fefb8 xorps    xmm2, xmm2
0x1416fefbb cmp      r8d, 1
0x1416fefbf jne      0x1416ff006
0x1416fefc1 addss    xmm1, xmm0
0x1416fefc5 jmp      0x1416ff003
0x1416fefc7 xor      dl, 1
0x1416fefca movzx    edx, dl
0x1416fefcd shl      edx, 2
0x1416fefd0 jmp      0x1416ff006
0x1416fefd2 cmp      r8d, 1
0x1416fefd6 je       0x1416fefdb
0x1416fefd8 xorps    xmm1, xmm1
0x1416fefdb mov      edx, 3
0x1416fefe0 jmp      0x1416ff003
0x1416fefe2 xorps    xmm2, xmm2
0x1416fefe5 ucomiss  xmm2, xmm1
0x1416fefe8 jb       0x1416feffa
0x1416fefea addss    xmm1, dword ptr [rip + 0x1226992]
0x1416feff2 maxss    xmm1, dword ptr [rip + 0x1231d76]
0x1416feffa subss    xmm1, xmm0
0x1416feffe mov      edx, 2
0x1416ff003 movaps   xmm2, xmm1
0x1416ff006 mov      dword ptr [rax + 8], edx
0x1416ff009 movss    dword ptr [rax + 0xc], xmm2
0x1416ff00e movzx    edx, byte ptr [rcx + 0x8a]
0x1416ff015 cmp      byte ptr [rcx + 0x32], 0
0x1416ff019 je       0x1416ff042
0x1416ff01b mov      r8d, dword ptr [rax + 0x10]
0x1416ff01f movss    xmm1, dword ptr [rax + 0x14]
0x1416ff024 test     dl, dl
0x1416ff026 je       0x1416ff04d
0x1416ff028 cmp      r8d, 2
0x1416ff02c je       0x1416ff05d
0x1416ff02e mov      edx, 1
0x1416ff033 xorps    xmm2, xmm2
0x1416ff036 cmp      r8d, 1
0x1416ff03a jne      0x1416ff081
0x1416ff03c addss    xmm1, xmm0
0x1416ff040 jmp      0x1416ff07e
0x1416ff042 xor      dl, 1
0x1416ff045 movzx    edx, dl
0x1416ff048 shl      edx, 2
0x1416ff04b jmp      0x1416ff081
0x1416ff04d cmp      r8d, 1
0x1416ff051 je       0x1416ff056
0x1416ff053 xorps    xmm1, xmm1
0x1416ff056 mov      edx, 3
0x1416ff05b jmp      0x1416ff07e
0x1416ff05d xorps    xmm2, xmm2
0x1416ff060 ucomiss  xmm2, xmm1
0x1416ff063 jb       0x1416ff075
0x1416ff065 addss    xmm1, dword ptr [rip + 0x1226917]
0x1416ff06d maxss    xmm1, dword ptr [rip + 0x1231cfb]
0x1416ff075 subss    xmm1, xmm0
0x1416ff079 mov      edx, 2
0x1416ff07e movaps   xmm2, xmm1
0x1416ff081 mov      dword ptr [rax + 0x10], edx
0x1416ff084 movss    dword ptr [rax + 0x14], xmm2
0x1416ff089 movzx    edx, byte ptr [rcx + 0x8b]
0x1416ff090 cmp      byte ptr [rcx + 0x33], 0
0x1416ff094 je       0x1416ff0bd
0x1416ff096 mov      r8d, dword ptr [rax + 0x18]
0x1416ff09a movss    xmm1, dword ptr [rax + 0x1c]
0x1416ff09f test     dl, dl
0x1416ff0a1 je       0x1416ff0c8
0x1416ff0a3 cmp      r8d, 2
0x1416ff0a7 je       0x1416ff0d8
0x1416ff0a9 mov      edx, 1
0x1416ff0ae xorps    xmm2, xmm2
0x1416ff0b1 cmp      r8d, 1
0x1416ff0b5 jne      0x1416ff0fc
0x1416ff0b7 addss    xmm1, xmm0
0x1416ff0bb jmp      0x1416ff0f9
0x1416ff0bd xor      dl, 1