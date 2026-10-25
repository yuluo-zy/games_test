0x1408b7620 test     r8, r8
0x1408b7623 je       0x1408b767f
0x1408b7625 movsxd   rax, dword ptr [r8 + 4]
0x1408b7629 movsxd   rcx, dword ptr [r8]
0x1408b762c xor      edx, edx
0x1408b762e cmp      qword ptr [rip + 0x2d7d653], rcx
0x1408b7635 movss    xmm0, dword ptr [r8 + 0xc]
0x1408b763b setne    dl
0x1408b763e add      rdx, qword ptr [rip + 0x2d7d64b]
0x1408b7645 cmp      rcx, rdx
0x1408b7648 cmova    rdx, rcx
0x1408b764c movsxd   r9, dword ptr [r8 + 0x10]
0x1408b7650 movsxd   r8, dword ptr [r8 + 0x14]
0x1408b7654 mov      qword ptr [rip + 0x2d7d635], rdx
0x1408b765b mov      qword ptr [rip + 0x2d7d606], rax
0x1408b7662 mov      qword ptr [rip + 0x2d7d61f], rcx
0x1408b7669 movss    dword ptr [rip + 0x2d7d5ff], xmm0
0x1408b7671 mov      qword ptr [rip + 0x2d7d600], r9
0x1408b7678 mov      qword ptr [rip + 0x2d7d601], r8
0x1408b767f xor      eax, eax
0x1408b7681 ret      