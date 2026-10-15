0x140b596b0 sub      rsp, 0x58
0x140b596b4 movzx    eax, byte ptr [rcx]
0x140b596b7 lea      rcx, [rip + 0x1f97bba]
0x140b596be movsxd   rax, dword ptr [rcx + rax*4]
0x140b596c2 add      rax, rcx
0x140b596c5 jmp      rax
0x140b596c7 mov      al, 1
0x140b596c9 add      rsp, 0x58
0x140b596cd ret      
0x140b596ce xor      eax, eax
0x140b596d0 add      rsp, 0x58
0x140b596d4 ret      
0x140b596d5 mov      al, 2
0x140b596d7 add      rsp, 0x58
0x140b596db ret      
0x140b596dc lea      rax, [rip + 0x1f97aa5]
0x140b596e3 mov      qword ptr [rsp + 0x28], rax
0x140b596e8 mov      qword ptr [rsp + 0x30], 1
0x140b596f1 lea      rax, [rsp + 0x20]
0x140b596f6 mov      qword ptr [rsp + 0x38], rax
0x140b596fb xorps    xmm0, xmm0
0x140b596fe movups   xmmword ptr [rsp + 0x40], xmm0
0x140b59703 lea      rdx, [rip + 0x1f97abe]
0x140b5970a lea      rcx, [rsp + 0x28]
0x140b5970f call     0x1428d9390 ; core::panicking::panic_fmt::h57d10e7f426973d3
0x140b59714 int3     