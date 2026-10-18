0x140c97540 4889c8 mov rax, rcx
0x140c97543 f30f1109 movss dword ptr [rcx], xmm1
0x140c97547 f30f115104 movss dword ptr [rcx + 4], xmm2
0x140c9754c f30f115908 movss dword ptr [rcx + 8], xmm3
0x140c97551 c7410c00000000 mov dword ptr [rcx + 0xc], 0
0x140c97558 c3 ret 
