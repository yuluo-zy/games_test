0x1400bb890 push     rbp
0x1400bb891 push     r14
0x1400bb893 push     rsi
0x1400bb894 push     rdi
0x1400bb895 push     rbx
0x1400bb896 sub      rsp, 0x2e0
0x1400bb89d lea      rbp, [rsp + 0x80]
0x1400bb8a5 movaps   xmmword ptr [rbp + 0x250], xmm7
0x1400bb8ac movaps   xmmword ptr [rbp + 0x240], xmm6
0x1400bb8b3 mov      qword ptr [rbp + 0x238], 0xfffffffffffffffe
0x1400bb8be mov      rsi, rdx
0x1400bb8c1 mov      rdi, rcx
0x1400bb8c4 mov      rbx, qword ptr [rcx]
0x1400bb8c7 mov      eax, dword ptr [rbx + 0x1c]
0x1400bb8ca mov      rcx, qword ptr [rbx + 0x10]
0x1400bb8ce mov      dword ptr [rcx], eax
0x1400bb8d0 mov      r14, qword ptr [rbx]
0x1400bb8d3 movzx    eax, byte ptr [rip + 0x357932f]
0x1400bb8da mov      ecx, 0x1b
0x1400bb8df mov      edx, 1
0x1400bb8e4 call     0x140613c10
0x1400bb8e9 test     rax, rax
0x1400bb8ec je       0x1400bbc01
0x1400bb8f2 add      r14, 0x100
0x1400bb8f9 movups   xmm0, xmmword ptr [rip + 0x28797b7] ; @0x1429350b7 ' Richness to 1.0SourcesEvent filterClear filterAnimate LogoForce TAA pass-thruFreeze occlusion c'
0x1400bb900 movups   xmmword ptr [rax + 0xb], xmm0
0x1400bb904 movups   xmm0, xmmword ptr [rip + 0x28797a1] ; @0x1429350ac 'Force Music Richness to 1.0SourcesEvent filterClear filterAnimate LogoForce TAA pass-thruFreeze '
0x1400bb90b movups   xmmword ptr [rax], xmm0
0x1400bb90e mov      qword ptr [rbp + 0x130], 0x1b
0x1400bb919 mov      qword ptr [rbp + 0x138], rax
0x1400bb920 mov      qword ptr [rbp + 0x140], 0x1b
0x1400bb92b mov      dword ptr [rbp + 0x148], 0
0x1400bb935 mov      qword ptr [rbp + 0x158], 0
0x1400bb940 mov      dword ptr [rbp + 0x150], 0
0x1400bb94a mov      qword ptr [rbp + 0x100], 3
0x1400bb955 mov      qword ptr [rbp + 0x118], 6
0x1400bb960 mov      byte ptr [rbp + 0x160], 0
0x1400bb967 mov      dword ptr [rbp + 0x165], 0
0x1400bb971 mov      dword ptr [rbp + 0x168], 0
0x1400bb97b mov      qword ptr [rbp + 0x170], r14
0x1400bb982 mov      byte ptr [rbp + 0x178], 0
0x1400bb989 lea      rcx, [rbp + 0x38]
0x1400bb98d lea      rdx, [rbp + 0x100]
0x1400bb994 mov      r8, rsi
0x1400bb997 call     0x140c48930
0x1400bb99c mov      rax, qword ptr [rbp + 0x48]
0x1400bb9a0 lock dec qword ptr [rax]
0x1400bb9a4 jne      0x1400bb9af
0x1400bb9a6 lea      rcx, [rbp + 0x48]
0x1400bb9aa call     0x140c41170
0x1400bb9af mov      eax, dword ptr [rbx + 0x1c]
0x1400bb9b2 mov      rcx, qword ptr [rbx + 0x10]
0x1400bb9b6 mov      dword ptr [rcx], eax
0x1400bb9b8 mov      rbx, qword ptr [rbx]
0x1400bb9bb movzx    eax, byte ptr [rip + 0x3579247]
0x1400bb9c2 mov      ecx, 0xf
0x1400bb9c7 mov      edx, 1
0x1400bb9cc call     0x140613c10
0x1400bb9d1 test     rax, rax
0x1400bb9d4 je       0x1400bbc17
0x1400bb9da add      rbx, 0xf6
0x1400bb9e1 movabs   rcx, 0x6f666e6920636973
0x1400bb9eb mov      qword ptr [rax + 7], rcx
0x1400bb9ef movabs   rcx, 0x73756d20776f6853
0x1400bb9f9 mov      qword ptr [rax], rcx
0x1400bb9fc mov      qword ptr [rbp + 0x130], 0xf
0x1400bba07 mov      qword ptr [rbp + 0x138], rax
0x1400bba0e mov      qword ptr [rbp + 0x140], 0xf
0x1400bba19 mov      dword ptr [rbp + 0x148], 0
0x1400bba23 mov      qword ptr [rbp + 0x158], 0
0x1400bba2e mov      dword ptr [rbp + 0x150], 0
0x1400bba38 mov      qword ptr [rbp + 0x100], 3
0x1400bba43 mov      qword ptr [rbp + 0x118], 6
0x1400bba4e mov      byte ptr [rbp + 0x160], 0
0x1400bba55 mov      dword ptr [rbp + 0x165], 0
0x1400bba5f mov      dword ptr [rbp + 0x168], 0
0x1400bba69 mov      qword ptr [rbp + 0x170], rbx
0x1400bba70 mov      byte ptr [rbp + 0x178], 0
0x1400bba77 lea      rcx, [rbp + 0x38]
0x1400bba7b lea      rdx, [rbp + 0x100]
0x1400bba82 mov      r8, rsi
0x1400bba85 call     0x140c48930
0x1400bba8a mov      rax, qword ptr [rbp + 0x48]
0x1400bba8e lock dec qword ptr [rax]
0x1400bba92 jne      0x1400bba9d
0x1400bba94 lea      rcx, [rbp + 0x48]
0x1400bba98 call     0x140c41170
0x1400bba9d movups   xmm6, xmmword ptr [rdi + 8]
0x1400bbaa1 movups   xmm7, xmmword ptr [rdi + 0x18]
0x1400bbaa5 lea      rdx, [rip + 0x287961b] ; @0x1429350c7 'SourcesEvent filterClear filterAnimate LogoForce TAA pass-thruFreeze occlusion cullingShow memor'
0x1400bbaac lea      rcx, [rbp - 0x58]
0x1400bbab0 mov      r8d, 7
0x1400bbab6 call     0x14015eea0
0x1400bbabb movzx    eax, byte ptr [rip + 0x3579147]
0x1400bbac2 mov      ecx, 0x20
0x1400bbac7 mov      edx, 8
0x1400bbacc call     0x140613c10
0x1400bbad1 test     rax, rax
0x1400bbad4 mov      qword ptr [rbp + 0x228], rax
0x1400bbadb je       0x1400bbc2d
0x1400bbae1 mov      rdi, rax
0x1400bbae4 movups   xmmword ptr [rax], xmm6
0x1400bbae7 movups   xmmword ptr [rax + 0x10], xmm7
0x1400bbaeb mov      byte ptr [rbp + 0x237], 1
0x1400bbaf2 lea      rcx, [rbp + 0x38]
0x1400bbaf6 lea      rdx, [rbp - 0x58]
0x1400bbafa mov      r8d, 0x90
0x1400bbb00 call     0x142923c00
0x1400bbb05 lea      rax, [rbp + 0x237]
0x1400bbb0c mov      qword ptr [rbp + 0xc8], rax
0x1400bbb13 mov      qword ptr [rbp + 0xd0], rdi
0x1400bbb1a lea      rax, [rip + 0x2878e0f]
0x1400bbb21 mov      qword ptr [rbp + 0xd8], rax
0x1400bbb28 movzx    eax, byte ptr [rip + 0x35790da]
0x1400bbb2f mov      ecx, 0xa8
0x1400bbb34 mov      edx, 8
0x1400bbb39 call     0x140613c10
0x1400bbb3e test     rax, rax
0x1400bbb41 je       0x1400bbc3e
0x1400bbb47 mov      rdi, rax
0x1400bbb4a lea      rdx, [rbp + 0x38]
0x1400bbb4e mov      r8d, 0xa8
0x1400bbb54 mov      rcx, rax
0x1400bbb57 call     0x142923c00
0x1400bbb5c lea      rax, [rip + 0x28a0c9d]
0x1400bbb63 mov      qword ptr [rsp + 0x20], rax
0x1400bbb68 lea      rcx, [rbp + 0x100]
0x1400bbb6f movabs   r8, 0x20001000000
0x1400bbb79 mov      rdx, rsi
0x1400bbb7c mov      r9, rdi
0x1400bbb7f call     0x1400c2f70
0x1400bbb84 lea      rcx, [rbp + 0x38]
0x1400bbb88 lea      rdx, [rbp + 0x100]
0x1400bbb8f mov      r8d, 0xc8
0x1400bbb95 call     0x142923c00
0x1400bbb9a mov      rax, qword ptr [rbp + 0x1d8]
0x1400bbba1 lock dec qword ptr [rax]
0x1400bbba5 jne      0x1400bbbb3
0x1400bbba7 lea      rcx, [rbp + 0x1d8]
0x1400bbbae call     0x140c41170
0x1400bbbb3 mov      rax, qword ptr [rbp + 0xa8]
0x1400bbbba lock dec qword ptr [rax]
0x1400bbbbe jne      0x1400bbbcc
0x1400bbbc0 lea      rcx, [rbp + 0xa8]
0x1400bbbc7 call     0x140c41170
0x1400bbbcc cmp      dword ptr [rbp + 0x78], 2
0x1400bbbd0 je       0x1400bbbe5
0x1400bbbd2 mov      rax, qword ptr [rbp + 0x48]
0x1400bbbd6 lock dec qword ptr [rax]
0x1400bbbda jne      0x1400bbbe5
0x1400bbbdc lea      rcx, [rbp + 0x48]
0x1400bbbe0 call     0x140c41170
0x1400bbbe5 movaps   xmm6, xmmword ptr [rbp + 0x240]
0x1400bbbec movaps   xmm7, xmmword ptr [rbp + 0x250]
0x1400bbbf3 add      rsp, 0x2e0
0x1400bbbfa pop      rbx
0x1400bbbfb pop      rdi
0x1400bbbfc pop      rsi
0x1400bbbfd pop      r14
0x1400bbbff pop      rbp
0x1400bbc00 ret      
0x1400bbc01 lea      r8, [rip + 0x2879140]
0x1400bbc08 mov      ecx, 1
0x1400bbc0d mov      edx, 0x1b
0x1400bbc12 call     0x1428d8fe3
0x1400bbc17 lea      r8, [rip + 0x287912a]
0x1400bbc1e mov      ecx, 1
0x1400bbc23 mov      edx, 0xf
0x1400bbc28 call     0x1428d8fe3
0x1400bbc2d mov      ecx, 8
0x1400bbc32 mov      edx, 0x20
0x1400bbc37 call     0x1428d9000
0x1400bbc3c jmp      0x1400bbc4d
0x1400bbc3e mov      ecx, 8
0x1400bbc43 mov      edx, 0xa8