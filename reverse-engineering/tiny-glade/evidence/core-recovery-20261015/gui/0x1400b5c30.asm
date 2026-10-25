0x1400b5c30 push     rbp
0x1400b5c31 push     rsi
0x1400b5c32 push     rdi
0x1400b5c33 push     rbx
0x1400b5c34 sub      rsp, 0x348
0x1400b5c3b lea      rbp, [rsp + 0x80]
0x1400b5c43 mov      qword ptr [rbp + 0x2c0], 0xfffffffffffffffe
0x1400b5c4e mov      rsi, rdx
0x1400b5c51 mov      rdi, rcx
0x1400b5c54 mov      rbx, qword ptr [rcx]
0x1400b5c57 movzx    eax, byte ptr [rip + 0x357efab]
0x1400b5c5e mov      ecx, 8
0x1400b5c63 mov      edx, 8
0x1400b5c68 call     0x140613c10
0x1400b5c6d test     rax, rax
0x1400b5c70 je       0x1400b61a4
0x1400b5c76 mov      qword ptr [rax], rbx
0x1400b5c79 lea      rcx, [rip + 0x287ed10]
0x1400b5c80 mov      qword ptr [rsp + 0x20], rcx
0x1400b5c85 lea      rcx, [rbp + 0x160]
0x1400b5c8c mov      rdx, rsi
0x1400b5c8f xor      r8d, r8d
0x1400b5c92 mov      r9, rax
0x1400b5c95 call     0x1400c4230
0x1400b5c9a mov      rax, qword ptr [rbp + 0x170]
0x1400b5ca1 lock dec qword ptr [rax]
0x1400b5ca5 jne      0x1400b5cb3
0x1400b5ca7 lea      rcx, [rbp + 0x170]
0x1400b5cae call     0x140c41170
0x1400b5cb3 mov      rbx, qword ptr [rdi + 0x10]
0x1400b5cb7 movzx    eax, byte ptr [rip + 0x357ef4b]
0x1400b5cbe mov      ecx, 8
0x1400b5cc3 mov      edx, 8
0x1400b5cc8 call     0x140613c10
0x1400b5ccd test     rax, rax
0x1400b5cd0 je       0x1400b61a4
0x1400b5cd6 mov      qword ptr [rax], rbx
0x1400b5cd9 lea      rcx, [rip + 0x287ecd0]
0x1400b5ce0 mov      qword ptr [rsp + 0x20], rcx
0x1400b5ce5 lea      rcx, [rbp + 0x160]
0x1400b5cec mov      rdx, rsi
0x1400b5cef xor      r8d, r8d
0x1400b5cf2 mov      r9, rax
0x1400b5cf5 call     0x1400c4230
0x1400b5cfa mov      rax, qword ptr [rbp + 0x170]
0x1400b5d01 lock dec qword ptr [rax]
0x1400b5d05 jne      0x1400b5d13
0x1400b5d07 lea      rcx, [rbp + 0x170]
0x1400b5d0e call     0x140c41170
0x1400b5d13 mov      rdi, qword ptr [rdi + 8]
0x1400b5d17 movzx    eax, byte ptr [rip + 0x357eeeb]
0x1400b5d1e mov      ecx, 8
0x1400b5d23 mov      edx, 8
0x1400b5d28 call     0x140613c10
0x1400b5d2d test     rax, rax
0x1400b5d30 je       0x1400b61a4
0x1400b5d36 mov      qword ptr [rbp + 0x2b8], rax
0x1400b5d3d mov      qword ptr [rax], rdi
0x1400b5d40 lea      rcx, [rbp + 0xf0]
0x1400b5d47 call     0x140c34750
0x1400b5d4c mov      qword ptr [rbp + 0x200], 0
0x1400b5d57 mov      qword ptr [rbp + 0x208], 1
0x1400b5d62 xorps    xmm0, xmm0
0x1400b5d65 movups   xmmword ptr [rbp + 0x210], xmm0
0x1400b5d6c mov      qword ptr [rbp + 0x220], 1
0x1400b5d77 mov      qword ptr [rbp + 0x228], 0
0x1400b5d82 movups   xmm1, xmmword ptr [rbp + 0xf0]
0x1400b5d89 movups   xmm2, xmmword ptr [rbp + 0x100]
0x1400b5d90 movups   xmm3, xmmword ptr [rbp + 0x110]
0x1400b5d97 movups   xmm4, xmmword ptr [rbp + 0x120]
0x1400b5d9e movups   xmmword ptr [rbp - 0x50], xmm1
0x1400b5da2 movups   xmmword ptr [rbp - 0x40], xmm2
0x1400b5da6 movups   xmmword ptr [rbp - 0x30], xmm3
0x1400b5daa movups   xmmword ptr [rbp - 0x20], xmm4
0x1400b5dae movups   xmm1, xmmword ptr [rbp + 0x130]
0x1400b5db5 movups   xmmword ptr [rbp - 0x10], xmm1
0x1400b5db9 movups   xmm1, xmmword ptr [rbp + 0x140]
0x1400b5dc0 movups   xmmword ptr [rbp], xmm1
0x1400b5dc4 movups   xmm1, xmmword ptr [rbp + 0x150]
0x1400b5dcb movups   xmmword ptr [rbp + 0x10], xmm1
0x1400b5dcf movups   xmmword ptr [rbp + 0x278], xmm0
0x1400b5dd6 mov      qword ptr [rbp + 0x160], 0
0x1400b5de1 mov      qword ptr [rbp + 0x170], 0
0x1400b5dec mov      qword ptr [rbp + 0x180], 0
0x1400b5df7 movups   xmm1, xmmword ptr [rbp - 0x58]
0x1400b5dfb movups   xmm2, xmmword ptr [rbp - 0x48]
0x1400b5dff movups   xmm3, xmmword ptr [rbp - 0x38]
0x1400b5e03 movups   xmm4, xmmword ptr [rbp - 0x28]
0x1400b5e07 movups   xmmword ptr [rbp + 0x188], xmm1
0x1400b5e0e movups   xmmword ptr [rbp + 0x198], xmm2
0x1400b5e15 movups   xmmword ptr [rbp + 0x1a8], xmm3
0x1400b5e1c movups   xmmword ptr [rbp + 0x1b8], xmm4
0x1400b5e23 movups   xmm1, xmmword ptr [rbp - 0x18]
0x1400b5e27 movups   xmmword ptr [rbp + 0x1c8], xmm1
0x1400b5e2e movups   xmm1, xmmword ptr [rbp - 8]
0x1400b5e32 movups   xmmword ptr [rbp + 0x1d8], xmm1
0x1400b5e39 movups   xmm1, xmmword ptr [rbp + 8]
0x1400b5e3d movups   xmmword ptr [rbp + 0x1e8], xmm1
0x1400b5e44 mov      rax, qword ptr [rbp + 0x18]
0x1400b5e48 mov      qword ptr [rbp + 0x1f8], rax
0x1400b5e4f mov      dword ptr [rbp + 0x230], 2
0x1400b5e59 movhps   xmm0, qword ptr [rip + 0x287e7c8]
0x1400b5e60 movups   xmmword ptr [rbp + 0x238], xmm0
0x1400b5e67 mov      byte ptr [rbp + 0x248], 0
0x1400b5e6e movaps   xmm0, xmmword ptr [rip + 0x287e75b]
0x1400b5e75 movups   xmmword ptr [rbp + 0x250], xmm0
0x1400b5e7c mov      byte ptr [rbp + 0x260], 0
0x1400b5e83 mov      rax, qword ptr [rbp + 0x2b8]
0x1400b5e8a mov      qword ptr [rbp + 0x268], rax
0x1400b5e91 lea      rax, [rip + 0x294c200]
0x1400b5e98 mov      qword ptr [rbp + 0x270], rax
0x1400b5e9f mov      qword ptr [rbp + 0x290], 0
0x1400b5eaa mov      dword ptr [rbp + 0x2a0], 0x10101
0x1400b5eb4 mov      byte ptr [rbp + 0x2a4], 2
0x1400b5ebb movzx    eax, byte ptr [rip + 0x357ed47]
0x1400b5ec2 mov      ecx, 8
0x1400b5ec7 mov      edx, 1
0x1400b5ecc call     0x140613c10
0x1400b5ed1 test     rax, rax
0x1400b5ed4 je       0x1400b61b3
0x1400b5eda movabs   rcx, 0x656475746974614c
0x1400b5ee4 mov      qword ptr [rax], rcx
0x1400b5ee7 mov      dword ptr [rbp + 0x2b4], 0
0x1400b5ef1 mov      dword ptr [rbp + 0x2b1], 0
0x1400b5efb mov      rdi, qword ptr [rbp + 0x190]
0x1400b5f02 mov      ecx, edi
0x1400b5f04 and      ecx, 0xfffffffe
0x1400b5f07 lea      rdx, [rdi - 3]
0x1400b5f0b xor      r8d, r8d
0x1400b5f0e cmp      ecx, 4
0x1400b5f11 cmove    r8, rdx
0x1400b5f15 test     r8, r8
0x1400b5f18 mov      qword ptr [rbp + 0x2b8], rax
0x1400b5f1f je       0x1400b5f5a
0x1400b5f21 cmp      r8, 1
0x1400b5f25 jne      0x1400b5fc8
0x1400b5f2b mov      rdx, qword ptr [rbp + 0x198]
0x1400b5f32 test     rdx, rdx
0x1400b5f35 je       0x1400b5f49
0x1400b5f37 mov      rcx, qword ptr [rbp + 0x1a0]
0x1400b5f3e mov      r8d, 1
0x1400b5f44 call     0x140613c20
0x1400b5f49 lea      rcx, [rbp + 0x1b0]
0x1400b5f50 call     0x140608f80
0x1400b5f55 jmp      0x1400b5fe1
0x1400b5f5a mov      rdx, qword ptr [rbp + 0x1c0]
0x1400b5f61 test     rdx, rdx
0x1400b5f64 je       0x1400b5f78
0x1400b5f66 mov      rcx, qword ptr [rbp + 0x1c8]
0x1400b5f6d mov      r8d, 1
0x1400b5f73 call     0x140613c20
0x1400b5f78 cmp      edi, 3
0x1400b5f7b je       0x1400b5f9b
0x1400b5f7d cmp      edi, 2
0x1400b5f80 jb       0x1400b5f9b
0x1400b5f82 mov      rax, qword ptr [rbp + 0x198]
0x1400b5f89 lock dec qword ptr [rax]
0x1400b5f8d jne      0x1400b5f9b
0x1400b5f8f lea      rcx, [rbp + 0x198]
0x1400b5f96 call     0x14064eb30
0x1400b5f9b mov      rax, qword ptr [rbp + 0x1a8]
0x1400b5fa2 cmp      rax, 6
0x1400b5fa6 je       0x1400b5fe1
0x1400b5fa8 cmp      eax, 5
0x1400b5fab jb       0x1400b5fe1
0x1400b5fad mov      rax, qword ptr [rbp + 0x1b0]
0x1400b5fb4 lock dec qword ptr [rax]
0x1400b5fb8 jne      0x1400b5fe1
0x1400b5fba lea      rcx, [rbp + 0x1b0]
0x1400b5fc1 call     0x14064eb30
0x1400b5fc6 jmp      0x1400b5fe1
0x1400b5fc8 mov      rax, qword ptr [rbp + 0x198]
0x1400b5fcf lock dec qword ptr [rax]
0x1400b5fd3 jne      0x1400b5fe1
0x1400b5fd5 lea      rcx, [rbp + 0x198]
0x1400b5fdc call     0x140c7de90
0x1400b5fe1 mov      qword ptr [rbp + 0x190], 3
0x1400b5fec movups   xmm0, xmmword ptr [rbp - 0x58]
0x1400b5ff0 movups   xmmword ptr [rbp + 0x198], xmm0
0x1400b5ff7 mov      qword ptr [rbp + 0x1a8], 6
0x1400b6002 movups   xmm0, xmmword ptr [rbp + 0xf0]
0x1400b6009 movups   xmmword ptr [rbp + 0x1b0], xmm0
0x1400b6010 mov      qword ptr [rbp + 0x1c0], 8
0x1400b601b mov      rax, qword ptr [rbp + 0x2b8]
0x1400b6022 mov      qword ptr [rbp + 0x1c8], rax
0x1400b6029 mov      qword ptr [rbp + 0x1d0], 8
0x1400b6034 mov      dword ptr [rbp + 0x1d8], 0
0x1400b603e mov      dword ptr [rbp + 0x1e0], 0
0x1400b6048 mov      qword ptr [rbp + 0x1e8], 0
0x1400b6053 mov      byte ptr [rbp + 0x1f0], 0
0x1400b605a mov      rax, qword ptr [rbp + 0x2ad]
0x1400b6061 mov      qword ptr [rbp + 0x1f1], rax
0x1400b6068 mov      eax, dword ptr [rbp + 0x2b4]
0x1400b606e mov      dword ptr [rbp + 0x1f8], eax
0x1400b6074 lea      rdi, [rbp - 0x58]
0x1400b6078 lea      rdx, [rbp + 0x160]
0x1400b607f mov      r8d, 0x148
0x1400b6085 mov      rcx, rdi
0x1400b6088 call     0x142923c00
0x1400b608d lea      rcx, [rbp + 0x160]
0x1400b6094 mov      rdx, rdi
0x1400b6097 mov      r8, rsi
0x1400b609a call     0x140c77160
0x1400b609f mov      rax, qword ptr [rbp + 0x170]
0x1400b60a6 lock dec qword ptr [rax]
0x1400b60aa jne      0x1400b60b8
0x1400b60ac lea      rcx, [rbp + 0x170]
0x1400b60b3 call     0x140c41170
0x1400b60b8 movzx    eax, byte ptr [rip + 0x357eb4a]
0x1400b60bf mov      ecx, 0x15
0x1400b60c4 mov      edx, 1
0x1400b60c9 call     0x140613c10
0x1400b60ce test     rax, rax
0x1400b60d1 je       0x1400b61cb
0x1400b60d7 movups   xmm0, xmmword ptr [rip + 0x287f122] ; @0x142935200 'Show ray tracing viewSun date ()'
0x1400b60de movups   xmmword ptr [rax], xmm0
0x1400b60e1 movabs   rcx, 0x7765697620676e69
0x1400b60eb mov      qword ptr [rax + 0xd], rcx
0x1400b60ef mov      qword ptr [rbp + 0x190], 0x15
0x1400b60fa mov      qword ptr [rbp + 0x198], rax
0x1400b6101 mov      qword ptr [rbp + 0x1a0], 0x15
0x1400b610c mov      dword ptr [rbp + 0x1a8], 0
0x1400b6116 mov      qword ptr [rbp + 0x1b8], 0
0x1400b6121 mov      dword ptr [rbp + 0x1b0], 0
0x1400b612b mov      qword ptr [rbp + 0x160], 3
0x1400b6136 mov      qword ptr [rbp + 0x178], 6
0x1400b6141 mov      byte ptr [rbp + 0x1c0], 0
0x1400b6148 mov      dword ptr [rbp + 0x1c5], 0
0x1400b6152 mov      dword ptr [rbp + 0x1c8], 0
0x1400b615c lea      rax, [rip + 0x357ea2d]
0x1400b6163 mov      qword ptr [rbp + 0x1d0], rax
0x1400b616a mov      byte ptr [rbp + 0x1d8], 0
0x1400b6171 lea      rcx, [rbp - 0x58]
0x1400b6175 lea      rdx, [rbp + 0x160]
0x1400b617c mov      r8, rsi
0x1400b617f call     0x140c48930
0x1400b6184 mov      rax, qword ptr [rbp - 0x48]
0x1400b6188 lock dec qword ptr [rax]
0x1400b618c jne      0x1400b6197
0x1400b618e lea      rcx, [rbp - 0x48]
0x1400b6192 call     0x140c41170
0x1400b6197 nop      
0x1400b6198 add      rsp, 0x348
0x1400b619f pop      rbx
0x1400b61a0 pop      rdi
0x1400b61a1 pop      rsi
0x1400b61a2 pop      rbp
0x1400b61a3 ret      
0x1400b61a4 mov      ecx, 8
0x1400b61a9 mov      edx, 8
0x1400b61ae call     0x1428d9000
0x1400b61b3 lea      r8, [rip + 0x287eb8e]
0x1400b61ba mov      ecx, 1
0x1400b61bf mov      edx, 8
0x1400b61c4 call     0x1428d8fe3
0x1400b61c9 ud2      
0x1400b61cb lea      r8, [rip + 0x287eb76]
0x1400b61d2 mov      ecx, 1
0x1400b61d7 mov      edx, 0x15