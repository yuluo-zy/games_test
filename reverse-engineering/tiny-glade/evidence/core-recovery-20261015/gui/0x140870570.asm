0x140870570 cmp      byte ptr [rip + 0x2dc4819], 0
0x140870577 jne      0x140870583
0x140870579 movss    xmm0, dword ptr [rip + 0x215caf7]
0x140870581 jmp      0x14087058b ; 
0x140870583 movss    xmm0, dword ptr [rip + 0x20b5379]
0x14087058b movss    dword ptr [rcx], xmm0
0x14087058f mov      dword ptr [rcx + 4], 0xffffffff
0x140870596 mov      byte ptr [rcx + 8], 0
0x14087059a mov      byte ptr [rcx + 0xa], 0
0x14087059e mov      byte ptr [rcx + 0xc], 0
0x1408705a2 ret      