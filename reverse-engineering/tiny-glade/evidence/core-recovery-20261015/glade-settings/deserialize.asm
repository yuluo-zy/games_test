0x14141f0f0  push     rbp
0x14141f0f1  push     r15
0x14141f0f3  push     r14
0x14141f0f5  push     r13
0x14141f0f7  push     r12
0x14141f0f9  push     rsi
0x14141f0fa  push     rdi
0x14141f0fb  push     rbx
0x14141f0fc  sub      rsp, 0x118
0x14141f103  lea      rbp, [rsp + 0x80]
0x14141f10b  movaps   xmmword ptr [rbp + 0x80], xmm7
0x14141f112  movaps   xmmword ptr [rbp + 0x70], xmm6
0x14141f116  mov      qword ptr [rbp + 0x68], 0xfffffffffffffffe
0x14141f11e  mov      rdi, rdx
0x14141f121  mov      rsi, rcx
0x14141f124  mov      r10, qword ptr [rdx + 0x20]
0x14141f128  mov      rax, qword ptr [rdx + 0x28]
0x14141f12c  cmp      rax, r10
0x14141f12f  jae      0x14141f176
0x14141f131  lea      r14, [rdi + 0x18]
0x14141f135  mov      rdx, qword ptr [rdi + 0x18]
0x14141f139  inc      rax
0x14141f13c  mov      r8, r10
0x14141f13f  neg      r8
0x14141f142  movabs   rbx, 0x100002600
0x14141f14c  nop      dword ptr [rax]
0x14141f150  movzx    r9d, byte ptr [rdx + rax - 1]
0x14141f156  cmp      r9, 0x20
0x14141f15a  ja       0x14141f196
0x14141f15c  bt       rbx, r9
0x14141f160  jae      0x14141f196
0x14141f162  mov      qword ptr [rdi + 0x28], rax
0x14141f166  lea      r9, [r8 + rax]
0x14141f16a  inc      r9
0x14141f16d  inc      rax
0x14141f170  cmp      r9, 1
0x14141f174  jne      0x14141f150
0x14141f176  mov      qword ptr [rbp + 0x38], 5
0x14141f17e  lea      rdx, [rbp + 0x38]
0x14141f182  mov      rcx, rdi
0x14141f185  call     0x1428a0920
0x14141f18a  mov      qword ptr [rsi], rax
0x14141f18d  mov      byte ptr [rsi + 0x20], 2
0x14141f191  jmp      0x14141fd75
0x14141f196  cmp      r9d, 0x5b
0x14141f19a  je       0x14141f921
0x14141f1a0  cmp      r9d, 0x7b
0x14141f1a4  jne      0x14141fb3b
0x14141f1aa  dec      byte ptr [rdi + 0x30]
0x14141f1ad  je       0x14141fdde
0x14141f1b3  mov      qword ptr [rdi + 0x28], rax
0x14141f1b7  cmp      rax, r10
0x14141f1ba  jae      0x14141f914
0x14141f1c0  mov      qword ptr [rbp - 0x30], 0
0x14141f1c8  mov      byte ptr [rbp + 0x67], 2
0x14141f1cc  mov      r13b, 1
0x14141f1cf  lea      rcx, [rbp + 0x38]
0x14141f1d3  mov      byte ptr [rbp + 0x34], 2
0x14141f1d7  mov      dword ptr [rbp - 4], 0
0x14141f1de  mov      byte ptr [rbp + 8], 2
0x14141f1e2  mov      r15b, 2
0x14141f1e5  mov      dword ptr [rbp - 8], 0
0x14141f1ec  mov      byte ptr [rbp + 0x66], 2
0x14141f1f0  mov      dword ptr [rbp - 0xc], 0
0x14141f1f7  mov      dword ptr [rbp + 4], 0
0x14141f1fe  mov      dword ptr [rbp], 0
0x14141f205  mov      rdx, qword ptr [r14]
0x14141f208  nop      dword ptr [rax + rax]
0x14141f210  movzx    r8d, byte ptr [rdx + rax]
0x14141f215  cmp      r8, 0x2c
0x14141f219  ja       0x14141f290
0x14141f21b  bt       rbx, r8
0x14141f21f  jae      0x14141f240
0x14141f221  inc      rax
0x14141f224  mov      qword ptr [rdi + 0x28], rax
0x14141f228  cmp      r10, rax
0x14141f22b  jne      0x14141f210
0x14141f22d  jmp      0x14141f914
0x14141f232  nop      word ptr cs:[rax + rax]
0x14141f240  cmp      r8, 0x2c
0x14141f244  jne      0x14141f290
0x14141f246  test     r13b, 1
0x14141f24a  jne      0x14141fdbd
0x14141f250  inc      rax
0x14141f253  mov      qword ptr [rdi + 0x28], rax
0x14141f257  cmp      rax, r10
0x14141f25a  jae      0x14141fa45
0x14141f260  movzx    r8d, byte ptr [rdx + rax]
0x14141f265  cmp      r8, 0x20
0x14141f269  ja       0x14141f2a7
0x14141f26b  bt       rbx, r8
0x14141f26f  jae      0x14141f2a7
0x14141f271  inc      rax
0x14141f274  mov      qword ptr [rdi + 0x28], rax
0x14141f278  cmp      r10, rax
0x14141f27b  jne      0x14141f260
0x14141f27d  jmp      0x14141fa45
0x14141f282  nop      word ptr cs:[rax + rax]
0x14141f290  cmp      r8d, 0x7d
0x14141f294  je       0x14141f9ad
0x14141f29a  test     r13b, 1
0x14141f29e  je       0x14141fe1b
0x14141f2a4  xor      r13d, r13d
0x14141f2a7  cmp      r8b, 0x22
0x14141f2ab  jne      0x14141fda7
0x14141f2b1  inc      rax
0x14141f2b4  mov      qword ptr [rdi + 0x28], rax
0x14141f2b8  mov      qword ptr [rdi + 0x10], 0
0x14141f2c0  mov      r12, rcx
0x14141f2c3  mov      rdx, r14
0x14141f2c6  mov      r8, rdi
0x14141f2c9  call     0x140b8a910
0x14141f2ce  cmp      dword ptr [rbp + 0x38], 2
0x14141f2d2  mov      rdx, qword ptr [rbp + 0x40]
0x14141f2d6  je       0x14141fbd4
0x14141f2dc  mov      r8, qword ptr [rbp + 0x48]
0x14141f2e0  lea      rcx, [rbp + 0x10]
0x14141f2e4  call     0x140a468b0
0x14141f2e9  test     byte ptr [rbp + 0x10], 1
0x14141f2ed  jne      0x14141fb53
0x14141f2f3  movzx    eax, byte ptr [rbp + 0x11]
0x14141f2f7  lea      rcx, [rip + 0x183e7ca]
0x14141f2fe  movsxd   rax, dword ptr [rcx + rax*4]
0x14141f302  add      rax, rcx
0x14141f305  jmp      rax
0x14141f307  cmp      byte ptr [rbp + 0x67], 2
0x14141f30b  jne      0x14141fe47
0x14141f311  mov      rcx, qword ptr [rdi + 0x20]
0x14141f315  mov      rax, qword ptr [rdi + 0x28]
0x14141f319  cmp      rax, rcx
0x14141f31c  jae      0x14141f914
0x14141f322  mov      rdx, qword ptr [r14]
0x14141f325  inc      rax
0x14141f328  neg      rcx
0x14141f32b  movzx    r8d, byte ptr [rdx + rax - 1]
0x14141f331  cmp      r8, 0x3a
0x14141f335  ja       0x14141fe0e
0x14141f33b  bt       rbx, r8
0x14141f33f  jae      0x14141f7d7
0x14141f345  mov      qword ptr [rdi + 0x28], rax
0x14141f349  lea      r8, [rcx + rax]
0x14141f34d  inc      r8
0x14141f350  inc      rax
0x14141f353  cmp      r8, 1
0x14141f357  jne      0x14141f32b
0x14141f359  jmp      0x14141f914
0x14141f35e  mov      rcx, rdi
0x14141f361  call     0x141409100
0x14141f366  test     rax, rax
0x14141f369  je       0x14141f900
0x14141f36f  jmp      0x14141fdca
0x14141f374  cmp      byte ptr [rbp + 8], 2
0x14141f378  jne      0x14141fe88
0x14141f37e  mov      rcx, qword ptr [rdi + 0x20]
0x14141f382  mov      rax, qword ptr [rdi + 0x28]
0x14141f386  cmp      rax, rcx
0x14141f389  jae      0x14141f914
0x14141f38f  mov      rdx, qword ptr [r14]
0x14141f392  inc      rax
0x14141f395  neg      rcx
0x14141f398  movzx    r8d, byte ptr [rdx + rax - 1]
0x14141f39e  cmp      r8, 0x3a
0x14141f3a2  ja       0x14141fe0e
0x14141f3a8  bt       rbx, r8
0x14141f3ac  jae      0x14141f6db
0x14141f3b2  mov      qword ptr [rdi + 0x28], rax
0x14141f3b6  lea      r8, [rcx + rax]
0x14141f3ba  inc      r8
0x14141f3bd  inc      rax
0x14141f3c0  cmp      r8, 1
0x14141f3c4  jne      0x14141f398
0x14141f3c6  jmp      0x14141f914
0x14141f3cb  cmp      dword ptr [rbp + 4], 0
0x14141f3cf  jne      0x14141fe63
0x14141f3d5  mov      rcx, qword ptr [rdi + 0x20]
0x14141f3d9  mov      rax, qword ptr [rdi + 0x28]
0x14141f3dd  cmp      rax, rcx
0x14141f3e0  jae      0x14141f914
0x14141f3e6  mov      rdx, qword ptr [r14]
0x14141f3e9  inc      rax
0x14141f3ec  neg      rcx
0x14141f3ef  movzx    r8d, byte ptr [rdx + rax - 1]
0x14141f3f5  cmp      r8, 0x3a
0x14141f3f9  ja       0x14141fe0e
0x14141f3ff  bt       rbx, r8
0x14141f403  jae      0x14141f70b
0x14141f409  mov      qword ptr [rdi + 0x28], rax
0x14141f40d  lea      r8, [rcx + rax]
0x14141f411  inc      r8
0x14141f414  inc      rax
0x14141f417  cmp      r8, 1
0x14141f41b  jne      0x14141f3ef
0x14141f41d  jmp      0x14141f914
0x14141f422  cmp      qword ptr [rbp - 0x30], 0
0x14141f427  jne      0x14141fe55
0x14141f42d  mov      rcx, qword ptr [rdi + 0x20]
0x14141f431  mov      rax, qword ptr [rdi + 0x28]
0x14141f435  cmp      rax, rcx
0x14141f438  jae      0x14141f914
0x14141f43e  mov      rdx, qword ptr [r14]
0x14141f441  inc      rax
0x14141f444  neg      rcx
0x14141f447  movzx    r8d, byte ptr [rdx + rax - 1]
0x14141f44d  cmp      r8, 0x3a
0x14141f451  ja       0x14141fe0e
0x14141f457  bt       rbx, r8
0x14141f45b  jae      0x14141f741
0x14141f461  mov      qword ptr [rdi + 0x28], rax
0x14141f465  lea      r8, [rcx + rax]
0x14141f469  inc      r8
0x14141f46c  inc      rax
0x14141f46f  cmp      r8, 1
0x14141f473  jne      0x14141f447
0x14141f475  jmp      0x14141f914
0x14141f47a  cmp      dword ptr [rbp - 4], 0
0x14141f47e  jne      0x14141fead
0x14141f484  mov      rcx, qword ptr [rdi + 0x20]
0x14141f488  mov      rax, qword ptr [rdi + 0x28]
0x14141f48c  cmp      rax, rcx
0x14141f48f  jae      0x14141f914
0x14141f495  mov      rdx, qword ptr [r14]
0x14141f498  inc      rax
0x14141f49b  neg      rcx
0x14141f49e  movzx    r8d, byte ptr [rdx + rax - 1]
0x14141f4a4  cmp      r8, 0x3a
0x14141f4a8  ja       0x14141fe0e
0x14141f4ae  bt       rbx, r8
0x14141f4b2  jae      0x14141f772
0x14141f4b8  mov      qword ptr [rdi + 0x28], rax
0x14141f4bc  lea      r8, [rcx + rax]
0x14141f4c0  inc      r8
0x14141f4c3  inc      rax
0x14141f4c6  cmp      r8, 1
0x14141f4ca  jne      0x14141f49e
0x14141f4cc  jmp      0x14141f914
0x14141f4d1  cmp      byte ptr [rbp + 0x66], 2
0x14141f4d5  jne      0x14141fe9f
0x14141f4db  mov      rcx, qword ptr [rdi + 0x20]
0x14141f4df  mov      rax, qword ptr [rdi + 0x28]
0x14141f4e3  cmp      rax, rcx
0x14141f4e6  jae      0x14141f914
0x14141f4ec  mov      rdx, qword ptr [r14]
0x14141f4ef  inc      rax
0x14141f4f2  neg      rcx
0x14141f4f5  movzx    r8d, byte ptr [rdx + rax - 1]
0x14141f4fb  cmp      r8, 0x3a
0x14141f4ff  ja       0x14141fe0e
0x14141f505  bt       rbx, r8
0x14141f509  jae      0x14141f7a7
0x14141f50f  mov      qword ptr [rdi + 0x28], rax
0x14141f513  lea      r8, [rcx + rax]
0x14141f517  inc      r8
0x14141f51a  inc      rax
0x14141f51d  cmp      r8, 1
0x14141f521  jne      0x14141f4f5
0x14141f523  jmp      0x14141f914
0x14141f528  cmp      byte ptr [rbp + 0x34], 2
0x14141f52c  jne      0x14141fe96
0x14141f532  mov      rcx, qword ptr [rdi + 0x20]
0x14141f536  mov      rax, qword ptr [rdi + 0x28]
0x14141f53a  cmp      rax, rcx
0x14141f53d  jae      0x14141f914
0x14141f543  mov      rdx, qword ptr [r14]
0x14141f546  inc      rax
0x14141f549  neg      rcx
0x14141f54c  movzx    r8d, byte ptr [rdx + rax - 1]
0x14141f552  cmp      r8, 0x3a
0x14141f556  ja       0x14141fe0e
0x14141f55c  bt       rbx, r8
0x14141f560  jae      0x14141f807
0x14141f566  mov      qword ptr [rdi + 0x28], rax
0x14141f56a  lea      r8, [rcx + rax]
0x14141f56e  inc      r8
0x14141f571  inc      rax
0x14141f574  cmp      r8, 1
0x14141f578  jne      0x14141f54c
0x14141f57a  jmp      0x14141f914
0x14141f57f  cmp      r15b, 2
0x14141f583  jne      0x14141fe71
0x14141f589  mov      rcx, qword ptr [rdi + 0x20]
0x14141f58d  mov      rax, qword ptr [rdi + 0x28]
0x14141f591  cmp      rax, rcx
0x14141f594  jae      0x14141f914
0x14141f59a  mov      rdx, qword ptr [r14]
0x14141f59d  inc      rax
0x14141f5a0  neg      rcx
0x14141f5a3  movzx    r8d, byte ptr [rdx + rax - 1]
0x14141f5a9  cmp      r8, 0x3a
0x14141f5ad  ja       0x14141fe0e
0x14141f5b3  bt       rbx, r8
0x14141f5b7  jae      0x14141f837
0x14141f5bd  mov      qword ptr [rdi + 0x28], rax
0x14141f5c1  lea      r8, [rcx + rax]
0x14141f5c5  inc      r8
0x14141f5c8  inc      rax
0x14141f5cb  cmp      r8, 1
0x14141f5cf  jne      0x14141f5a3
0x14141f5d1  jmp      0x14141f914
0x14141f5d6  cmp      dword ptr [rbp - 8], 0
0x14141f5da  jne      0x14141fe39
0x14141f5e0  mov      rcx, qword ptr [rdi + 0x20]
0x14141f5e4  mov      rax, qword ptr [rdi + 0x28]
0x14141f5e8  cmp      rax, rcx
0x14141f5eb  jae      0x14141f914
0x14141f5f1  mov      rdx, qword ptr [r14]
0x14141f5f4  inc      rax
0x14141f5f7  neg      rcx
0x14141f5fa  movzx    r8d, byte ptr [rdx + rax - 1]
0x14141f600  cmp      r8, 0x3a
0x14141f604  ja       0x14141fe0e
0x14141f60a  bt       rbx, r8
0x14141f60e  jae      0x14141f865
0x14141f614  mov      qword ptr [rdi + 0x28], rax
0x14141f618  lea      r8, [rcx + rax]
0x14141f61c  inc      r8
0x14141f61f  inc      rax
0x14141f622  cmp      r8, 1
0x14141f626  jne      0x14141f5fa
0x14141f628  jmp      0x14141f914
0x14141f62d  cmp      dword ptr [rbp], 0
0x14141f631  jne      0x14141fe28
0x14141f637  mov      rcx, qword ptr [rdi + 0x20]
0x14141f63b  mov      rax, qword ptr [rdi + 0x28]
0x14141f63f  cmp      rax, rcx
0x14141f642  jae      0x14141f914
0x14141f648  mov      rdx, qword ptr [r14]
0x14141f64b  inc      rax
0x14141f64e  neg      rcx
0x14141f651  movzx    r8d, byte ptr [rdx + rax - 1]
0x14141f657  cmp      r8, 0x3a
0x14141f65b  ja       0x14141fe0e
0x14141f661  bt       rbx, r8
0x14141f665  jae      0x14141f897
0x14141f66b  mov      qword ptr [rdi + 0x28], rax
0x14141f66f  lea      r8, [rcx + rax]
0x14141f673  inc      r8
0x14141f676  inc      rax
0x14141f679  cmp      r8, 1
0x14141f67d  jne      0x14141f651
0x14141f67f  jmp      0x14141f914
0x14141f684  cmp      dword ptr [rbp - 0xc], 0
0x14141f688  jne      0x14141fe7a
0x14141f68e  mov      rcx, qword ptr [rdi + 0x20]
0x14141f692  mov      rax, qword ptr [rdi + 0x28]
0x14141f696  cmp      rax, rcx
0x14141f699  jae      0x14141f914
0x14141f69f  mov      rdx, qword ptr [r14]
0x14141f6a2  inc      rax
0x14141f6a5  neg      rcx
0x14141f6a8  movzx    r8d, byte ptr [rdx + rax - 1]
0x14141f6ae  cmp      r8, 0x3a
0x14141f6b2  ja       0x14141fe0e
0x14141f6b8  bt       rbx, r8
0x14141f6bc  jae      0x14141f8ca
0x14141f6c2  mov      qword ptr [rdi + 0x28], rax
0x14141f6c6  lea      r8, [rcx + rax]
0x14141f6ca  inc      r8
0x14141f6cd  inc      rax
0x14141f6d0  cmp      r8, 1
0x14141f6d4  jne      0x14141f6a8
0x14141f6d6  jmp      0x14141f914
0x14141f6db  cmp      r8, 0x3a
0x14141f6df  jne      0x14141fe0e
0x14141f6e5  mov      qword ptr [rdi + 0x28], rax
0x14141f6e9  lea      rcx, [rbp + 0x10]
0x14141f6ed  mov      rdx, rdi
0x14141f6f0  call     0x1414130c0
0x14141f6f5  test     byte ptr [rbp + 0x10], 1
0x14141f6f9  jne      0x14141fb53
0x14141f6ff  movzx    eax, byte ptr [rbp + 0x11]
0x14141f703  mov      byte ptr [rbp + 8], al
0x14141f706  jmp      0x14141f900
0x14141f70b  cmp      r8, 0x3a
0x14141f70f  jne      0x14141fe0e
0x14141f715  mov      qword ptr [rdi + 0x28], rax
0x14141f719  lea      rcx, [rbp + 0x10]
0x14141f71d  mov      rdx, rdi
0x14141f720  call     0x141412c00
0x14141f725  test     byte ptr [rbp + 0x10], 1
0x14141f729  jne      0x14141fb53
0x14141f72f  mov      eax, dword ptr [rbp + 0x14]
0x14141f732  mov      dword ptr [rbp - 0x38], eax
0x14141f735  mov      dword ptr [rbp + 4], 1
0x14141f73c  jmp      0x14141f900
0x14141f741  cmp      r8, 0x3a
0x14141f745  jne      0x14141fe0e
0x14141f74b  mov      qword ptr [rdi + 0x28], rax
0x14141f74f  mov      rcx, rdi
0x14141f752  call     0x141412f50
0x14141f757  mov      qword ptr [rbp - 0x48], rdx
0x14141f75b  test     rax, rax
0x14141f75e  jne      0x14141fdd5
0x14141f764  mov      eax, 1
0x14141f769  mov      qword ptr [rbp - 0x30], rax
0x14141f76d  jmp      0x14141f900
0x14141f772  cmp      r8, 0x3a
0x14141f776  jne      0x14141fe0e
0x14141f77c  mov      qword ptr [rdi + 0x28], rax
0x14141f780  lea      rcx, [rbp + 0x10]
0x14141f784  mov      rdx, rdi
0x14141f787  call     0x14140cfd0
0x14141f78c  test     byte ptr [rbp + 0x10], 1
0x14141f790  jne      0x14141fb53
0x14141f796  movss    xmm7, dword ptr [rbp + 0x14]
0x14141f79b  mov      dword ptr [rbp - 4], 1
0x14141f7a2  jmp      0x14141f900
0x14141f7a7  cmp      r8, 0x3a
0x14141f7ab  jne      0x14141fe0e
0x14141f7b1  mov      qword ptr [rdi + 0x28], rax
0x14141f7b5  lea      rcx, [rbp + 0x10]
0x14141f7b9  mov      rdx, rdi
0x14141f7bc  call     0x1414130c0
0x14141f7c1  test     byte ptr [rbp + 0x10], 1
0x14141f7c5  jne      0x14141fb53
0x14141f7cb  movzx    eax, byte ptr [rbp + 0x11]
0x14141f7cf  mov      byte ptr [rbp + 0x66], al
0x14141f7d2  jmp      0x14141f900
0x14141f7d7  cmp      r8, 0x3a
0x14141f7db  jne      0x14141fe0e
0x14141f7e1  mov      qword ptr [rdi + 0x28], rax
0x14141f7e5  lea      rcx, [rbp + 0x10]
0x14141f7e9  mov      rdx, rdi
0x14141f7ec  call     0x1414130c0
0x14141f7f1  test     byte ptr [rbp + 0x10], 1
0x14141f7f5  jne      0x14141fb53
0x14141f7fb  movzx    eax, byte ptr [rbp + 0x11]
0x14141f7ff  mov      byte ptr [rbp + 0x67], al
0x14141f802  jmp      0x14141f900
0x14141f807  cmp      r8, 0x3a
0x14141f80b  jne      0x14141fe0e
0x14141f811  mov      qword ptr [rdi + 0x28], rax
0x14141f815  lea      rcx, [rbp + 0x10]
0x14141f819  mov      rdx, rdi
0x14141f81c  call     0x1414130c0
0x14141f821  test     byte ptr [rbp + 0x10], 1
0x14141f825  jne      0x14141fb53
0x14141f82b  movzx    eax, byte ptr [rbp + 0x11]
0x14141f82f  mov      byte ptr [rbp + 0x34], al
0x14141f832  jmp      0x14141f900
0x14141f837  cmp      r8, 0x3a
0x14141f83b  jne      0x14141fe0e
0x14141f841  mov      qword ptr [rdi + 0x28], rax
0x14141f845  lea      rcx, [rbp + 0x10]
0x14141f849  mov      rdx, rdi
0x14141f84c  call     0x1414130c0
0x14141f851  test     byte ptr [rbp + 0x10], 1
0x14141f855  jne      0x14141fb53
0x14141f85b  movzx    r15d, byte ptr [rbp + 0x11]
0x14141f860  jmp      0x14141f900
0x14141f865  cmp      r8, 0x3a
0x14141f869  jne      0x14141fe0e
0x14141f86f  mov      qword ptr [rdi + 0x28], rax
0x14141f873  lea      rcx, [rbp + 0x10]
0x14141f877  mov      rdx, rdi
0x14141f87a  call     0x14140cfd0
0x14141f87f  test     byte ptr [rbp + 0x10], 1
0x14141f883  jne      0x14141fb53
0x14141f889  movss    xmm6, dword ptr [rbp + 0x14]
0x14141f88e  mov      dword ptr [rbp - 8], 1
0x14141f895  jmp      0x14141f900
0x14141f897  cmp      r8, 0x3a
0x14141f89b  jne      0x14141fe0e
0x14141f8a1  mov      qword ptr [rdi + 0x28], rax
0x14141f8a5  lea      rcx, [rbp + 0x10]
0x14141f8a9  mov      rdx, rdi
0x14141f8ac  call     0x141412c00
0x14141f8b1  test     byte ptr [rbp + 0x10], 1
0x14141f8b5  jne      0x14141fb53
0x14141f8bb  mov      eax, dword ptr [rbp + 0x14]
0x14141f8be  mov      dword ptr [rbp - 0x3c], eax
0x14141f8c1  mov      dword ptr [rbp], 1
0x14141f8c8  jmp      0x14141f900
0x14141f8ca  cmp      r8, 0x3a
0x14141f8ce  jne      0x14141fe0e
0x14141f8d4  mov      qword ptr [rdi + 0x28], rax
0x14141f8d8  lea      rcx, [rbp + 0x10]
0x14141f8dc  mov      rdx, rdi
0x14141f8df  call     0x141412c00
0x14141f8e4  test     byte ptr [rbp + 0x10], 1
0x14141f8e8  jne      0x14141fb53
0x14141f8ee  mov      eax, dword ptr [rbp + 0x14]
0x14141f8f1  mov      dword ptr [rbp - 0x34], eax
0x14141f8f4  mov      dword ptr [rbp - 0xc], 1
0x14141f8fb  nop      dword ptr [rax + rax]
0x14141f900  mov      r10, qword ptr [rdi + 0x20]
0x14141f904  mov      rax, qword ptr [rdi + 0x28]
0x14141f908  cmp      rax, r10
0x14141f90b  mov      rcx, r12
0x14141f90e  jb       0x14141f205
0x14141f914  mov      qword ptr [rbp + 0x38], 3
0x14141f91c  jmp      0x14141fa4d
0x14141f921  dec      byte ptr [rdi + 0x30]
0x14141f924  je       0x14141fdde
0x14141f92a  mov      qword ptr [rdi + 0x28], rax
0x14141f92e  mov      qword ptr [rbp + 0x10], rdi
0x14141f932  mov      byte ptr [rbp + 0x18], 1
0x14141f936  lea      rcx, [rbp + 0x38]
0x14141f93a  lea      rdx, [rbp + 0x10]
0x14141f93e  call     0x14140aec0
0x14141f943  cmp      byte ptr [rbp + 0x38], 0
0x14141f947  jne      0x14141fc5c
0x14141f94d  movzx    eax, byte ptr [rbp + 0x39]
0x14141f951  cmp      al, 2
0x14141f953  mov      r14d, 1
0x14141f959  cmovne   r14d, eax
0x14141f95d  lea      rcx, [rbp + 0x38]
0x14141f961  lea      rdx, [rbp + 0x10]
0x14141f965  call     0x14140aec0
0x14141f96a  cmp      byte ptr [rbp + 0x38], 0
0x14141f96e  jne      0x14141fc5c
0x14141f974  movzx    eax, byte ptr [rbp + 0x39]
0x14141f978  xor      r15d, r15d
0x14141f97b  cmp      al, 2
0x14141f97d  cmovne   r15d, eax
0x14141f981  lea      rcx, [rbp + 0x38]
0x14141f985  lea      rdx, [rbp + 0x10]
0x14141f989  call     0x14140ac00
0x14141f98e  mov      rax, qword ptr [rbp + 0x38]
0x14141f992  test     rax, rax
0x14141f995  je       0x14141fb12
0x14141f99b  mov      rbx, qword ptr [rbp + 0x40]
0x14141f99f  cmp      eax, 1
0x14141f9a2  je       0x14141fb14
0x14141f9a8  jmp      0x14141fc60
0x14141f9ad  movzx    ecx, byte ptr [rbp + 0x67]
0x14141f9b1  mov      eax, ecx
0x14141f9b3  and      al, 1
0x14141f9b5  cmp      cl, 2
0x14141f9b8  movzx    eax, al
0x14141f9bb  mov      ecx, 1
0x14141f9c0  cmove    eax, ecx
0x14141f9c3  xor      edx, edx
0x14141f9c5  test     byte ptr [rbp - 0x30], 1
0x14141f9c9  mov      rbx, qword ptr [rbp - 0x48]
0x14141f9cd  cmove    rbx, rdx
0x14141f9d1  test     byte ptr [rbp - 4], 1
0x14141f9d5  jne      0x14141f9df
0x14141f9d7  movss    xmm7, dword ptr [rip + 0x1540c95]
0x14141f9df  test     byte ptr [rbp - 8], 1
0x14141f9e3  mov      r9d, dword ptr [rbp - 0x38]
0x14141f9e7  movzx    r10d, byte ptr [rbp + 0x34]
0x14141f9ec  movzx    r11d, byte ptr [rbp + 8]
0x14141f9f1  jne      0x14141f9fb
0x14141f9f3  movss    xmm6, dword ptr [rip + 0x1505f09]
0x14141f9fb  and      r10b, 1
0x14141f9ff  and      r11b, 1
0x14141fa03  and      r15b, 1
0x14141fa07  movzx    r14d, byte ptr [rbp + 0x66]
0x14141fa0c  mov      r8d, r14d
0x14141fa0f  and      r8b, 1
0x14141fa13  cmp      r14b, 2
0x14141fa17  movzx    r14d, r8b
0x14141fa1b  cmove    r14d, ecx
0x14141fa1f  test     byte ptr [rbp - 0xc], 1
0x14141fa23  mov      ecx, 8
0x14141fa28  cmovne   ecx, dword ptr [rbp - 0x34]
0x14141fa2c  test     byte ptr [rbp + 4], 1
0x14141fa30  cmove    r9d, edx
0x14141fa34  test     byte ptr [rbp], 1
0x14141fa38  mov      r8d, dword ptr [rbp - 0x3c]
0x14141fa3c  cmove    r8d, edx
0x14141fa40  mov      edx, r8d
0x14141fa43  jmp      0x14141fa5f
0x14141fa45  mov      qword ptr [rbp + 0x38], 5
0x14141fa4d  lea      rdx, [rbp + 0x38]
0x14141fa51  mov      rcx, rdi
0x14141fa54  call     0x1428a0920
0x14141fa59  mov      rbx, rax
0x14141fa5c  mov      r14b, 2
0x14141fa5f  mov      dword ptr [rbp + 0x34], r14d
0x14141fa63  inc      byte ptr [rdi + 0x30]
0x14141fa66  mov      qword ptr [rbp + 0x38], rbx
0x14141fa6a  movss    dword ptr [rbp + 0x40], xmm7
0x14141fa6f  movss    dword ptr [rbp + 0x44], xmm6
0x14141fa74  mov      dword ptr [rbp + 0x48], ecx
0x14141fa77  mov      dword ptr [rbp + 0x4c], r9d
0x14141fa7b  mov      dword ptr [rbp + 0x50], edx
0x14141fa7e  mov      byte ptr [rbp + 0x54], al
0x14141fa81  mov      byte ptr [rbp + 0x55], r10b
0x14141fa85  mov      byte ptr [rbp + 0x56], r11b
0x14141fa89  mov      byte ptr [rbp + 0x57], r15b
0x14141fa8d  mov      byte ptr [rbp + 0x58], r14b
0x14141fa91  mov      eax, dword ptr [rbp - 0x22]
0x14141fa94  mov      ecx, dword ptr [rbp - 0x1f]
0x14141fa97  mov      dword ptr [rbp + 0x59], eax
0x14141fa9a  mov      dword ptr [rbp + 0x5c], ecx
0x14141fa9d  mov      rcx, rdi
0x14141faa0  call     0x140058450
0x14141faa5  lea      rdx, [rbp + 0x40]
0x14141faa9  mov      rcx, qword ptr [rdx + 0x10]
0x14141faad  mov      qword ptr [rbp + 0x20], rcx
0x14141fab1  movups   xmm0, xmmword ptr [rdx]
0x14141fab4  movaps   xmmword ptr [rbp + 0x10], xmm0
0x14141fab8  mov      edx, dword ptr [rbp + 0x34]
0x14141fabb  mov      r14b, 2
0x14141fabe  cmp      dl, 2
0x14141fac1  jne      0x14141faee
0x14141fac3  test     rax, rax
0x14141fac6  je       0x14141fd49
0x14141facc  mov      rcx, qword ptr [rax]
0x14141facf  cmp      rcx, 1
0x14141fad3  mov      qword ptr [rbp + 8], rax
0x14141fad7  jne      0x14141fcd9
0x14141fadd  mov      rcx, rax
0x14141fae0  add      rcx, 8
0x14141fae4  call     0x1405e7a00
0x14141fae9  jmp      0x14141fd35
0x14141faee  test     rax, rax
0x14141faf1  jne      0x14141fd4c
0x14141faf7  mov      rax, qword ptr [rbp + 0x20]
0x14141fafb  mov      qword ptr [rbp - 0x50], rax
0x14141faff  movaps   xmm0, xmmword ptr [rbp + 0x10]
0x14141fb03  movaps   xmmword ptr [rbp - 0x60], xmm0
0x14141fb07  mov      eax, dword ptr [rbp - 0x22]
0x14141fb0a  mov      ecx, dword ptr [rbp - 0x1f]
0x14141fb0d  jmp      0x14141fd1b
0x14141fb12  xor      ebx, ebx
0x14141fb14  lea      rcx, [rbp + 0x38]
0x14141fb18  lea      rdx, [rbp + 0x10]
0x14141fb1c  call     0x14140c050
0x14141fb21  test     byte ptr [rbp + 0x38], 1
0x14141fb25  jne      0x14141fc5c
0x14141fb2b  cmp      byte ptr [rbp + 0x3c], 0
0x14141fb2f  jne      0x14141fb5c
0x14141fb31  movss    xmm6, dword ptr [rip + 0x1540b3b]
0x14141fb39  jmp      0x14141fb61
0x14141fb3b  lea      r8, [rip + 0x183c94e]
0x14141fb42  lea      rdx, [rbp - 0xd]
0x14141fb46  mov      rcx, rdi
0x14141fb49  call     0x1428f8c00
0x14141fb4e  jmp      0x14141fd97
0x14141fb53  mov      rbx, qword ptr [rbp + 0x18]
0x14141fb57  jmp      0x14141fa5c
0x14141fb5c  movss    xmm6, dword ptr [rbp + 0x40]
0x14141fb61  lea      rcx, [rbp + 0x38]
0x14141fb65  lea      rdx, [rbp + 0x10]
0x14141fb69  call     0x14140aec0
0x14141fb6e  cmp      byte ptr [rbp + 0x38], 0
0x14141fb72  jne      0x14141fc5c
0x14141fb78  movzx    eax, byte ptr [rbp + 0x39]
0x14141fb7c  xor      r12d, r12d
0x14141fb7f  cmp      al, 2
0x14141fb81  cmove    eax, r12d
0x14141fb85  mov      dword ptr [rbp + 8], eax
0x14141fb88  lea      rcx, [rbp + 0x38]
0x14141fb8c  lea      rdx, [rbp + 0x10]
0x14141fb90  call     0x14140aec0
0x14141fb95  cmp      byte ptr [rbp + 0x38], 0
0x14141fb99  jne      0x14141fc5c
0x14141fb9f  movzx    eax, byte ptr [rbp + 0x39]
0x14141fba3  cmp      al, 2
0x14141fba5  cmovne   r12d, eax
0x14141fba9  mov      dword ptr [rbp], r12d
0x14141fbad  lea      rcx, [rbp + 0x38]
0x14141fbb1  lea      rdx, [rbp + 0x10]
0x14141fbb5  call     0x14140c050
0x14141fbba  test     byte ptr [rbp + 0x38], 1
0x14141fbbe  jne      0x14141fc5c
0x14141fbc4  cmp      byte ptr [rbp + 0x3c], 0
0x14141fbc8  jne      0x14141fbdc
0x14141fbca  movss    xmm7, dword ptr [rip + 0x1505d32]
0x14141fbd2  jmp      0x14141fbe1
0x14141fbd4  mov      rbx, rdx
0x14141fbd7  jmp      0x14141fa5c
0x14141fbdc  movss    xmm7, dword ptr [rbp + 0x40]
0x14141fbe1  lea      rcx, [rbp + 0x38]
0x14141fbe5  lea      rdx, [rbp + 0x10]
0x14141fbe9  call     0x14140aec0
0x14141fbee  cmp      byte ptr [rbp + 0x38], 0
0x14141fbf2  jne      0x14141fc5c
0x14141fbf4  movzx    eax, byte ptr [rbp + 0x39]
0x14141fbf8  cmp      al, 2
0x14141fbfa  mov      ecx, 1
0x14141fbff  cmovne   ecx, eax
0x14141fc02  mov      dword ptr [rbp + 0x34], ecx
0x14141fc05  lea      rcx, [rbp + 0x38]
0x14141fc09  lea      rdx, [rbp + 0x10]
0x14141fc0d  call     0x14140aaa0
0x14141fc12  test     byte ptr [rbp + 0x38], 1
0x14141fc16  jne      0x14141fc5c
0x14141fc18  cmp      byte ptr [rbp + 0x3c], 0
0x14141fc1c  mov      r13d, 8
0x14141fc22  je       0x14141fc28
0x14141fc24  mov      r13d, dword ptr [rbp + 0x40]
0x14141fc28  lea      rcx, [rbp + 0x38]
0x14141fc2c  lea      rdx, [rbp + 0x10]
0x14141fc30  call     0x14140aaa0
0x14141fc35  test     byte ptr [rbp + 0x38], 1
0x14141fc39  jne      0x14141fc5c
0x14141fc3b  mov      eax, dword ptr [rbp + 0x3c]
0x14141fc3e  mov      dword ptr [rbp + 4], eax
0x14141fc41  mov      r12d, dword ptr [rbp + 0x40]
0x14141fc45  lea      rcx, [rbp + 0x38]
0x14141fc49  lea      rdx, [rbp + 0x10]
0x14141fc4d  call     0x14140aaa0
0x14141fc52  test     byte ptr [rbp + 0x38], 1
0x14141fc56  je       0x14141fdeb
0x14141fc5c  mov      rbx, qword ptr [rbp + 0x40]
0x14141fc60  mov      r8b, 2
0x14141fc63  mov      dword ptr [rbp + 0x34], r8d
0x14141fc67  inc      byte ptr [rdi + 0x30]
0x14141fc6a  mov      qword ptr [rbp + 0x38], rbx
0x14141fc6e  movss    dword ptr [rbp + 0x40], xmm6
0x14141fc73  movss    dword ptr [rbp + 0x44], xmm7
0x14141fc78  mov      dword ptr [rbp + 0x48], r13d
0x14141fc7c  mov      dword ptr [rbp + 0x4c], r12d
0x14141fc80  mov      dword ptr [rbp + 0x50], eax
0x14141fc83  mov      byte ptr [rbp + 0x54], r14b
0x14141fc87  mov      byte ptr [rbp + 0x55], r15b
0x14141fc8b  mov      byte ptr [rbp + 0x56], cl
0x14141fc8e  mov      byte ptr [rbp + 0x57], dl
0x14141fc91  mov      byte ptr [rbp + 0x58], r8b
0x14141fc95  mov      eax, dword ptr [rbp - 0x1b]
0x14141fc98  mov      ecx, dword ptr [rbp - 0x18]
0x14141fc9b  mov      dword ptr [rbp + 0x59], eax
0x14141fc9e  mov      dword ptr [rbp + 0x5c], ecx
0x14141fca1  mov      rcx, rdi
0x14141fca4  call     0x1400584f0
0x14141fca9  lea      rdx, [rbp + 0x40]
0x14141fcad  mov      rcx, qword ptr [rdx + 0x10]
0x14141fcb1  mov      qword ptr [rbp + 0x20], rcx
0x14141fcb5  movups   xmm0, xmmword ptr [rdx]
0x14141fcb8  movaps   xmmword ptr [rbp + 0x10], xmm0
0x14141fcbc  mov      edx, dword ptr [rbp + 0x34]
0x14141fcbf  mov      r14b, 2
0x14141fcc2  cmp      dl, 2
0x14141fcc5  jne      0x14141fd00
0x14141fcc7  test     rax, rax
0x14141fcca  je       0x14141fd49
0x14141fccc  mov      rcx, qword ptr [rax]
0x14141fccf  cmp      rcx, 1
0x14141fcd3  mov      qword ptr [rbp + 8], rax
0x14141fcd7  je       0x14141fd29
0x14141fcd9  test     rcx, rcx
0x14141fcdc  jne      0x14141fd35
0x14141fcde  mov      rax, qword ptr [rbp + 8]
0x14141fce2  mov      rdx, qword ptr [rax + 0x10]
0x14141fce6  test     rdx, rdx
0x14141fce9  je       0x14141fd35
0x14141fceb  mov      rax, qword ptr [rbp + 8]
0x14141fcef  mov      rcx, qword ptr [rax + 8]
0x14141fcf3  mov      r8d, 1
0x14141fcf9  call     0x140613c20
0x14141fcfe  jmp      0x14141fd35
0x14141fd00  test     rax, rax
0x14141fd03  jne      0x14141fd4c
0x14141fd05  mov      rax, qword ptr [rbp + 0x20]
0x14141fd09  mov      qword ptr [rbp - 0x50], rax
0x14141fd0d  movaps   xmm0, xmmword ptr [rbp + 0x10]
0x14141fd11  movaps   xmmword ptr [rbp - 0x60], xmm0
0x14141fd15  mov      eax, dword ptr [rbp - 0x1b]
0x14141fd18  mov      ecx, dword ptr [rbp - 0x18]
0x14141fd1b  mov      dword ptr [rbp - 0x14], eax
0x14141fd1e  mov      dword ptr [rbp - 0x11], ecx
0x14141fd21  mov      rax, rbx
0x14141fd24  mov      r14d, edx
0x14141fd27  jmp      0x14141fd4c
0x14141fd29  mov      rcx, rax
0x14141fd2c  add      rcx, 8
0x14141fd30  call     0x1405e7a00
0x14141fd35  mov      edx, 0x28
0x14141fd3a  mov      r8d, 8
0x14141fd40  mov      rcx, qword ptr [rbp + 8]
0x14141fd44  call     0x140613c20
0x14141fd49  mov      rax, rbx
0x14141fd4c  cmp      r14b, 2
0x14141fd50  je       0x14141fd97
0x14141fd52  mov      rcx, qword ptr [rbp - 0x50]
0x14141fd56  mov      qword ptr [rsi + 0x18], rcx
0x14141fd5a  movaps   xmm0, xmmword ptr [rbp - 0x60]
0x14141fd5e  movups   xmmword ptr [rsi + 8], xmm0
0x14141fd62  mov      edx, dword ptr [rbp - 0x14]
0x14141fd65  mov      ecx, dword ptr [rbp - 0x11]
0x14141fd68  mov      dword ptr [rsi + 0x21], edx
0x14141fd6b  mov      dword ptr [rsi + 0x24], ecx
0x14141fd6e  mov      qword ptr [rsi], rax
0x14141fd71  mov      byte ptr [rsi + 0x20], r14b
0x14141fd75  mov      rax, rsi
0x14141fd78  movaps   xmm6, xmmword ptr [rbp + 0x70]
0x14141fd7c  movaps   xmm7, xmmword ptr [rbp + 0x80]
0x14141fd83  add      rsp, 0x118
0x14141fd8a  pop      rbx
0x14141fd8b  pop      rdi
0x14141fd8c  pop      rsi
0x14141fd8d  pop      r12
0x14141fd8f  pop      r13
0x14141fd91  pop      r14
0x14141fd93  pop      r15
0x14141fd95  pop      rbp
0x14141fd96  ret      
0x14141fd97  mov      rcx, rax
0x14141fd9a  mov      rdx, rdi
0x14141fd9d  call     0x1428a2200
0x14141fda2  mov      r14b, 2
0x14141fda5  jmp      0x14141fd6e
0x14141fda7  movzx    eax, r8b
0x14141fdab  cmp      eax, 0x7d
0x14141fdae  jne      0x14141fdbd
0x14141fdb0  mov      qword ptr [rbp + 0x38], 0x12
0x14141fdb8  jmp      0x14141fa4d
0x14141fdbd  mov      qword ptr [rbp + 0x38], 0x10
0x14141fdc5  jmp      0x14141fa4d
0x14141fdca  mov      r14b, 2
0x14141fdcd  mov      rbx, rax
0x14141fdd0  jmp      0x14141fa5f
0x14141fdd5  mov      rbx, qword ptr [rbp - 0x48]
0x14141fdd9  jmp      0x14141fa5c
0x14141fdde  mov      qword ptr [rbp + 0x38], 0x15
0x14141fde6  jmp      0x14141f17e
0x14141fdeb  mov      eax, dword ptr [rbp + 4]
0x14141fdee  test     eax, eax
0x14141fdf0  cmove    r12d, eax
0x14141fdf4  xor      eax, eax
0x14141fdf6  cmp      byte ptr [rbp + 0x3c], 0
0x14141fdfa  je       0x14141fdff
0x14141fdfc  mov      eax, dword ptr [rbp + 0x40]
0x14141fdff  mov      ecx, dword ptr [rbp + 8]
0x14141fe02  mov      edx, dword ptr [rbp]
0x14141fe05  mov      r8d, dword ptr [rbp + 0x34]
0x14141fe09  jmp      0x14141fc63
0x14141fe0e  mov      qword ptr [rbp + 0x38], 6
0x14141fe16  jmp      0x14141fa4d
0x14141fe1b  mov      qword ptr [rbp + 0x38], 8
0x14141fe23  jmp      0x14141fa4d
0x14141fe28  lea      rcx, [rip + 0x183d0c5]
0x14141fe2f  mov      edx, 0x15
0x14141fe34  jmp      0x14141feb9
0x14141fe39  lea      rcx, [rip + 0x183d055]
0x14141fe40  mov      edx, 0xd
0x14141fe45  jmp      0x14141feb9
0x14141fe47  lea      rcx, [rip + 0x183d005]
0x14141fe4e  mov      edx, 0xa
0x14141fe53  jmp      0x14141feb9
0x14141fe55  lea      rcx, [rip + 0x183d00c]
0x14141fe5c  mov      edx, 0x13
0x14141fe61  jmp      0x14141feb9
0x14141fe63  lea      rcx, [rip + 0x183d06c]
0x14141fe6a  mov      edx, 0x1e
0x14141fe6f  jmp      0x14141feb9
0x14141fe71  lea      rcx, [rip + 0x183d012]
0x14141fe78  jmp      0x14141feb4
0x14141fe7a  lea      rcx, [rip + 0x183d03c]
0x14141fe81  mov      edx, 0x19
0x14141fe86  jmp      0x14141feb9
0x14141fe88  lea      rcx, [rip + 0x183cff7]
0x14141fe8f  mov      edx, 4
0x14141fe94  jmp      0x14141feb9
0x14141fe96  lea      rcx, [rip + 0x183cfc0]
0x14141fe9d  jmp      0x14141feb4
0x14141fe9f  lea      rcx, [rip + 0x183cffc]
0x14141fea6  mov      edx, 0x1b
0x14141feab  jmp      0x14141feb9
0x14141fead  lea      rcx, [rip + 0x183cfc7]
0x14141feb4  mov      edx, 0xb
0x14141feb9  call     0x1428f9db0
0x14141febe  jmp      0x14141fa59
0x14141fec3  nop      word ptr cs:[rax + rax]
0x14141fed0  mov      qword ptr [rsp + 0x10], rdx
0x14141fed5  push     rbp
0x14141fed6  push     r15
0x14141fed8  push     r14
0x14141feda  push     r13
0x14141fedc  push     r12
0x14141fede  push     rsi
0x14141fedf  push     rdi
0x14141fee0  push     rbx
0x14141fee1  sub      rsp, 0x48
0x14141fee5  lea      rbp, [rdx + 0x80]
0x14141feec  movaps   xmmword ptr [rsp + 0x20], xmm7
0x14141fef1  movaps   xmmword ptr [rsp + 0x30], xmm6
0x14141fef6  mov      edx, 0x28
0x14141fefb  mov      r8d, 8
0x14141ff01  mov      rcx, qword ptr [rbp + 8]
0x14141ff05  call     0x140613c20
0x14141ff0a  movaps   xmm6, xmmword ptr [rsp + 0x30]
0x14141ff0f  movaps   xmm7, xmmword ptr [rsp + 0x20]
0x14141ff14  add      rsp, 0x48
0x14141ff18  pop      rbx
0x14141ff19  pop      rdi
0x14141ff1a  pop      rsi
0x14141ff1b  pop      r12
0x14141ff1d  pop      r13
0x14141ff1f  pop      r14
0x14141ff21  pop      r15
0x14141ff23  pop      rbp
0x14141ff24  ret      
0x14141ff25  nop      word ptr cs:[rax + rax]
0x14141ff30  mov      qword ptr [rsp + 0x10], rdx
0x14141ff35  push     rbp
0x14141ff36  push     r15
0x14141ff38  push     r14
0x14141ff3a  push     r13
0x14141ff3c  push     r12
0x14141ff3e  push     rsi
0x14141ff3f  push     rdi
0x14141ff40  push     rbx
0x14141ff41  sub      rsp, 0x48
0x14141ff45  lea      rbp, [rdx + 0x80]
0x14141ff4c  movaps   xmmword ptr [rsp + 0x20], xmm7
0x14141ff51  movaps   xmmword ptr [rsp + 0x30], xmm6
0x14141ff56  mov      edx, 0x28
0x14141ff5b  mov      r8d, 8
0x14141ff61  mov      rcx, qword ptr [rbp + 8]
0x14141ff65  call     0x140613c20
0x14141ff6a  movaps   xmm6, xmmword ptr [rsp + 0x30]
0x14141ff6f  movaps   xmm7, xmmword ptr [rsp + 0x20]
0x14141ff74  add      rsp, 0x48
0x14141ff78  pop      rbx
0x14141ff79  pop      rdi
0x14141ff7a  pop      rsi
0x14141ff7b  pop      r12
0x14141ff7d  pop      r13
0x14141ff7f  pop      r14
0x14141ff81  pop      r15
0x14141ff83  pop      rbp
0x14141ff84  ret      
0x14141ff85  nop      word ptr cs:[rax + rax]
0x14141ff90  mov      qword ptr [rsp + 0x10], rdx
0x14141ff95  push     rbp
0x14141ff96  push     r15
0x14141ff98  push     r14
0x14141ff9a  push     r13
0x14141ff9c  push     r12
0x14141ff9e  push     rsi
0x14141ff9f  push     rdi
0x14141ffa0  push     rbx
0x14141ffa1  sub      rsp, 0x48
0x14141ffa5  lea      rbp, [rdx + 0x80]
0x14141ffac  movaps   xmmword ptr [rsp + 0x20], xmm7
0x14141ffb1  movaps   xmmword ptr [rsp + 0x30], xmm6
0x14141ffb6  cmp      byte ptr [rbp + 0x34], 2
0x14141ffba  jne      0x14141ffc5
0x14141ffbc  lea      rcx, [rbp + 0x38]
0x14141ffc0  call     0x1405e88a0
0x14141ffc5  movaps   xmm6, xmmword ptr [rsp + 0x30]
0x14141ffca  movaps   xmm7, xmmword ptr [rsp + 0x20]
0x14141ffcf  add      rsp, 0x48
0x14141ffd3  pop      rbx
0x14141ffd4  pop      rdi
0x14141ffd5  pop      rsi
0x14141ffd6  pop      r12
0x14141ffd8  pop      r13
0x14141ffda  pop      r14
0x14141ffdc  pop      r15
0x14141ffde  pop      rbp
0x14141ffdf  ret      
0x14141ffe0  mov      qword ptr [rsp + 0x10], rdx
0x14141ffe5  push     rbp
0x14141ffe6  push     r15
0x14141ffe8  push     r14
0x14141ffea  push     r13
0x14141ffec  push     r12
0x14141ffee  push     rsi
0x14141ffef  push     rdi
0x14141fff0  push     rbx
0x14141fff1  sub      rsp, 0x48
0x14141fff5  lea      rbp, [rdx + 0x80]
0x14141fffc  movaps   xmmword ptr [rsp + 0x20], xmm7
0x141420001  movaps   xmmword ptr [rsp + 0x30], xmm6
0x141420006  cmp      byte ptr [rbp + 0x34], 2
0x14142000a  jne      0x141420015
0x14142000c  lea      rcx, [rbp + 0x38]
0x141420010  call     0x1405e88a0
0x141420015  movaps   xmm6, xmmword ptr [rsp + 0x30]
0x14142001a  movaps   xmm7, xmmword ptr [rsp + 0x20]
0x14142001f  add      rsp, 0x48
0x141420023  pop      rbx
0x141420024  pop      rdi
0x141420025  pop      rsi
0x141420026  pop      r12
0x141420028  pop      r13
0x14142002a  pop      r14
0x14142002c  pop      r15
0x14142002e  pop      rbp
0x14142002f  ret      
0x141420030  push     rbp
0x141420031  push     r15
0x141420033  push     r14
0x141420035  push     r13
0x141420037  push     r12
0x141420039  push     rsi
0x14142003a  push     rdi
0x14142003b  push     rbx
0x14142003c  sub      rsp, 0x108
0x141420043  lea      rbp, [rsp + 0x80]
0x14142004b  mov      qword ptr [rbp + 0x80], 0xfffffffffffffffe
0x141420056  mov      rdi, rdx
0x141420059  mov      rsi, rcx
0x14142005c  mov      r10, qword ptr [rdx + 0x20]
0x141420060  mov      rax, qword ptr [rdx + 0x28]
0x141420064  cmp      rax, r10
0x141420067  jae      0x1414200b6
0x141420069  lea      rbx, [rdi + 0x18]
0x14142006d  mov      rdx, qword ptr [rdi + 0x18]
0x141420071  inc      rax
0x141420074  mov      r8, r10
0x141420077  neg      r8
0x14142007a  movabs   r14, 0x100002600
0x141420084  nop      word ptr cs:[rax + rax]
0x141420090  movzx    r9d, byte ptr [rdx + rax - 1]
0x141420096  cmp      r9, 0x20
0x14142009a  ja       0x1414200cf
0x14142009c  bt       r14, r9
0x1414200a0  jae      0x1414200cf
0x1414200a2  mov      qword ptr [rdi + 0x28], rax
0x1414200a6  lea      r9, [r8 + rax]
0x1414200aa  inc      r9
0x1414200ad  inc      rax
0x1414200b0  cmp      r9, 1
0x1414200b4  jne      0x141420090
0x1414200b6  mov      qword ptr [rbp - 0x28], 5
0x1414200be  lea      rdx, [rbp - 0x28]
0x1414200c2  mov      rcx, rdi
0x1414200c5  call     0x1428a0920
0x1414200ca  jmp      0x141420b6f
0x1414200cf  cmp      r9d, 0x5b
0x1414200d3  je       0x141420513
0x1414200d9  cmp      r9d, 0x7b
0x1414200dd  jne      0x141420930
0x1414200e3  dec      byte ptr [rdi + 0x30]
0x1414200e6  je       0x141420cde
0x1414200ec  mov      qword ptr [rdi + 0x28], rax
0x1414200f0  mov      r9b, 1
0x1414200f3  movabs   r15, 0x8000000000000000
0x1414200fd  lea      rcx, [rbp - 0x28]
0x141420101  lea      r13, [rbp - 0x58]
0x141420105  mov      qword ptr [rbp + 0x58], r15
0x141420109  mov      qword ptr [rbp + 0x70], r15
0x14142010d  mov      qword ptr [rbp + 0x60], r15
0x141420111  cmp      rax, r10
0x141420114  jae      0x141420563
0x14142011a  mov      rdx, qword ptr [rbx]
0x14142011d  nop      dword ptr [rax]
0x141420120  movzx    r8d, byte ptr [rdx + rax]
0x141420125  cmp      r8, 0x2c
0x141420129  ja       0x141420192
0x14142012b  bt       r14, r8
0x14142012f  jae      0x141420142
0x141420131  inc      rax
0x141420134  mov      qword ptr [rdi + 0x28], rax
0x141420138  cmp      r10, rax
0x14142013b  jne      0x141420120
0x14142013d  jmp      0x141420563
0x141420142  cmp      r8, 0x2c
0x141420146  jne      0x141420192
0x141420148  test     r9b, 1
0x14142014c  jne      0x141420b07
0x141420152  inc      rax
0x141420155  mov      qword ptr [rdi + 0x28], rax
0x141420159  cmp      rax, r10
0x14142015c  jae      0x141420831
0x141420162  nop      word ptr cs:[rax + rax]
0x141420170  movzx    r8d, byte ptr [rdx + rax]
0x141420175  cmp      r8, 0x20
0x141420179  ja       0x1414201a9
0x14142017b  bt       r14, r8
0x14142017f  jae      0x1414201a9
0x141420181  inc      rax
0x141420184  mov      qword ptr [rdi + 0x28], rax
0x141420188  cmp      r10, rax
0x14142018b  jne      0x141420170
0x14142018d  jmp      0x141420831
0x141420192  cmp      r8d, 0x7d
0x141420196  je       0x141420738
0x14142019c  test     r9b, 1
0x1414201a0  je       0x141420ceb
0x1414201a6  xor      r9d, r9d
0x1414201a9  cmp      r8b, 0x22
0x1414201ad  jne      0x141420ac1
0x1414201b3  mov      dword ptr [rbp - 0x5c], r9d
0x1414201b7  inc      rax
0x1414201ba  mov      qword ptr [rdi + 0x28], rax
0x1414201be  mov      qword ptr [rdi + 0x10], 0
0x1414201c6  mov      rax, qword ptr [rbp + 0x58]
0x1414201ca  mov      qword ptr [rbp + 0x48], rax
0x1414201ce  mov      rax, qword ptr [rbp + 0x70]
0x1414201d2  mov      qword ptr [rbp + 0x40], rax
0x1414201d6  mov      rax, qword ptr [rbp + 0x60]
0x1414201da  mov      qword ptr [rbp + 0x38], rax
0x1414201de  mov      byte ptr [rbp + 0x7f], 1
0x1414201e2  mov      byte ptr [rbp + 0x7e], 1
0x1414201e6  mov      rdx, rbx
0x1414201e9  mov      r8, rdi
0x1414201ec  call     0x140b8a910
0x1414201f1  cmp      dword ptr [rbp - 0x28], 2
0x1414201f5  mov      rax, qword ptr [rbp - 0x20]
0x1414201f9  mov      qword ptr [rbp + 0x68], rax
0x1414201fd  je       0x14142059b
0x141420203  mov      rax, qword ptr [rbp - 0x18]
0x141420207  cmp      rax, 4
0x14142020b  je       0x14142032f
0x141420211  cmp      rax, 0xa
0x141420215  je       0x1414202ab
0x14142021b  cmp      rax, 6
0x14142021f  jne      0x14142033d
0x141420225  mov      r12, qword ptr [rbp + 0x68]
0x141420229  mov      eax, dword ptr [r12]
0x14142022d  xor      eax, dword ptr [rip + 0x16a7b7c]
0x141420233  movzx    ecx, word ptr [r12 + 4]
0x141420239  xor      cx, word ptr [rip + 0x16a7b73]
0x141420240  movzx    ecx, cx
0x141420243  or       ecx, eax
0x141420245  jne      0x14142033d
0x14142024b  mov      rcx, qword ptr [rbp + 0x58]
0x14142024f  mov      rax, rcx
0x141420252  neg      rax
0x141420255  jno      0x141420d28
0x14142025b  mov      r9, qword ptr [rdi + 0x20]
0x14142025f  mov      rax, qword ptr [rdi + 0x28]
0x141420263  cmp      rax, r9
0x141420266  jae      0x141420c8a
0x14142026c  mov      rcx, r13
0x14142026f  mov      rdx, qword ptr [rbx]
0x141420272  inc      rax
0x141420275  neg      r9
0x141420278  movzx    r8d, byte ptr [rdx + rax - 1]
0x14142027e  cmp      r8, 0x3a
0x141420282  ja       0x141420df5
0x141420288  bt       r14, r8
0x14142028c  jae      0x1414204b4
0x141420292  mov      qword ptr [rdi + 0x28], rax
0x141420296  lea      r8, [r9 + rax]
0x14142029a  inc      r8
0x14142029d  inc      rax
0x1414202a0  cmp      r8, 1
0x1414202a4  jne      0x141420278
0x1414202a6  jmp      0x141420c8a
0x1414202ab  mov      r12, qword ptr [rbp + 0x68]
0x1414202af  mov      rax, qword ptr [r12]
0x1414202b3  xor      rax, qword ptr [rip + 0x16a7aeb]
0x1414202ba  movzx    ecx, word ptr [r12 + 8]
0x1414202c0  xor      cx, word ptr [rip + 0x16a7ae6]
0x1414202c7  movzx    ecx, cx
0x1414202ca  or       rcx, rax
0x1414202cd  jne      0x14142033d
0x1414202cf  mov      rcx, qword ptr [rbp + 0x60]
0x1414202d3  mov      rax, rcx
0x1414202d6  neg      rax
0x1414202d9  jno      0x141420d5e
0x1414202df  mov      r9, qword ptr [rdi + 0x20]
0x1414202e3  mov      rax, qword ptr [rdi + 0x28]
0x1414202e7  cmp      rax, r9
0x1414202ea  jae      0x141420c2b
0x1414202f0  mov      rcx, r13
0x1414202f3  mov      rdx, qword ptr [rbx]
0x1414202f6  inc      rax
0x1414202f9  neg      r9
0x1414202fc  movzx    r8d, byte ptr [rdx + rax - 1]
0x141420302  cmp      r8, 0x3a
0x141420306  ja       0x141420e38
0x14142030c  bt       r14, r8
0x141420310  jae      0x1414203f6
0x141420316  mov      qword ptr [rdi + 0x28], rax
0x14142031a  lea      r8, [r9 + rax]
0x14142031e  inc      r8
0x141420321  inc      rax
0x141420324  cmp      r8, 1
0x141420328  jne      0x1414202fc
0x14142032a  jmp      0x141420c2b
0x14142032f  mov      rax, qword ptr [rbp + 0x68]
0x141420333  mov      eax, dword ptr [rax]
0x141420335  cmp      eax, dword ptr [rip + 0x16a7a7a]
0x14142033b  je       0x14142039a
0x14142033d  mov      rax, qword ptr [rbp + 0x58]
0x141420341  mov      qword ptr [rbp + 0x48], rax
0x141420345  mov      rax, qword ptr [rbp + 0x70]
0x141420349  mov      qword ptr [rbp + 0x40], rax
0x14142034d  mov      rax, qword ptr [rbp + 0x60]
0x141420351  mov      qword ptr [rbp + 0x38], rax
0x141420355  mov      byte ptr [rbp + 0x7f], 1
0x141420359  mov      byte ptr [rbp + 0x7e], 1
0x14142035d  mov      rcx, rdi
0x141420360  call     0x141409100
0x141420365  mov      qword ptr [rbp + 0x68], rax
0x141420369  cmp      qword ptr [rbp + 0x68], 0
0x14142036e  jne      0x14142059b
0x141420374  mov      r12, qword ptr [rbp + 0x28]
0x141420378  mov      r10, qword ptr [rdi + 0x20]
0x14142037c  mov      rax, qword ptr [rdi + 0x28]
0x141420380  mov      qword ptr [rbp + 0x28], r12
0x141420384  lea      rcx, [rbp - 0x28]
0x141420388  mov      r9d, dword ptr [rbp - 0x5c]
0x14142038c  cmp      rax, r10
0x14142038f  jb       0x14142011a
0x141420395  jmp      0x141420563
0x14142039a  mov      rcx, qword ptr [rbp + 0x70]
0x14142039e  mov      rax, rcx
0x1414203a1  neg      rax
0x1414203a4  jno      0x141420dbf
0x1414203aa  mov      r9, qword ptr [rdi + 0x20]
0x1414203ae  mov      rax, qword ptr [rdi + 0x28]
0x1414203b2  cmp      rax, r9
0x1414203b5  jae      0x141420bd1
0x1414203bb  mov      rcx, r13
0x1414203be  mov      rdx, qword ptr [rbx]
0x1414203c1  inc      rax
0x1414203c4  neg      r9
0x1414203c7  movzx    r8d, byte ptr [rdx + rax - 1]
0x1414203cd  cmp      r8, 0x3a
0x1414203d1  ja       0x141420e77
0x1414203d7  bt       r14, r8
0x1414203db  jae      0x141420455
0x1414203dd  mov      qword ptr [rdi + 0x28], rax
0x1414203e1  lea      r8, [r9 + rax]
0x1414203e5  inc      r8
0x1414203e8  inc      rax
0x1414203eb  cmp      r8, 1
0x1414203ef  jne      0x1414203c7
0x1414203f1  jmp      0x141420bd1
0x1414203f6  cmp      r8, 0x3a
0x1414203fa  jne      0x141420e38
0x141420400  mov      qword ptr [rdi + 0x28], rax
0x141420404  mov      rax, qword ptr [rbp + 0x58]
0x141420408  mov      qword ptr [rbp + 0x48], rax
0x14142040c  mov      rax, qword ptr [rbp + 0x70]
0x141420410  mov      qword ptr [rbp + 0x40], rax
0x141420414  mov      qword ptr [rbp + 0x38], r15
0x141420418  mov      byte ptr [rbp + 0x7f], 1
0x14142041c  mov      byte ptr [rbp + 0x7e], 1
0x141420420  mov      r13, rcx
0x141420423  mov      rdx, rdi
0x141420426  call     0x141412a90
0x14142042b  mov      rcx, qword ptr [rbp - 0x58]
0x14142042f  mov      rax, qword ptr [rbp - 0x50]
0x141420433  mov      qword ptr [rbp + 0x60], rcx
0x141420437  neg      rcx
0x14142043a  jo       0x141420c65
0x141420440  mov      rcx, qword ptr [rbp - 0x48]
0x141420444  mov      qword ptr [rbp - 0x40], rcx
0x141420448  mov      r12, qword ptr [rbp + 0x28]
0x14142044c  mov      qword ptr [rbp + 0x30], rax
0x141420450  jmp      0x141420378
0x141420455  cmp      r8, 0x3a
0x141420459  jne      0x141420e77
0x14142045f  mov      qword ptr [rdi + 0x28], rax
0x141420463  mov      rax, qword ptr [rbp + 0x58]
0x141420467  mov      qword ptr [rbp + 0x48], rax
0x14142046b  mov      qword ptr [rbp + 0x40], r15
0x14142046f  mov      rax, qword ptr [rbp + 0x60]
0x141420473  mov      qword ptr [rbp + 0x38], rax
0x141420477  mov      byte ptr [rbp + 0x7f], 1
0x14142047b  mov      byte ptr [rbp + 0x7e], 1
0x14142047f  mov      r13, rcx
0x141420482  mov      rdx, rdi
0x141420485  call     0x141412a90
0x14142048a  mov      rcx, qword ptr [rbp - 0x58]
0x14142048e  mov      rax, qword ptr [rbp - 0x50]
0x141420492  mov      qword ptr [rbp + 0x70], rcx
0x141420496  neg      rcx
0x141420499  jo       0x141420c0b
0x14142049f  mov      rcx, qword ptr [rbp - 0x48]
0x1414204a3  mov      qword ptr [rbp - 0x30], rcx
0x1414204a7  mov      r12, qword ptr [rbp + 0x28]
0x1414204ab  mov      qword ptr [rbp + 0x50], rax
0x1414204af  jmp      0x141420378
0x1414204b4  cmp      r8, 0x3a
0x1414204b8  jne      0x141420df5
0x1414204be  mov      qword ptr [rdi + 0x28], rax
0x1414204c2  mov      qword ptr [rbp + 0x48], r15
0x1414204c6  mov      rax, qword ptr [rbp + 0x70]
0x1414204ca  mov      qword ptr [rbp + 0x40], rax
0x1414204ce  mov      rax, qword ptr [rbp + 0x60]
0x1414204d2  mov      qword ptr [rbp + 0x38], rax
0x1414204d6  mov      byte ptr [rbp + 0x7f], 1
0x1414204da  mov      byte ptr [rbp + 0x7e], 1
0x1414204de  mov      r13, rcx
0x1414204e1  mov      rdx, rdi
0x1414204e4  call     0x141413b20
0x1414204e9  mov      rax, qword ptr [rbp - 0x58]
0x1414204ed  mov      rcx, qword ptr [rbp - 0x50]
0x1414204f1  mov      qword ptr [rbp + 0x68], rcx
0x1414204f5  mov      qword ptr [rbp + 0x58], rax
0x1414204f9  neg      rax
0x1414204fc  jo       0x141420cc8
0x141420502  mov      rax, qword ptr [rbp - 0x48]
0x141420506  mov      qword ptr [rbp - 0x38], rax
0x14142050a  mov      r12, qword ptr [rbp + 0x68]
0x14142050e  jmp      0x141420378
0x141420513  dec      byte ptr [rdi + 0x30]
0x141420516  je       0x141420cde
0x14142051c  mov      qword ptr [rdi + 0x28], rax
0x141420520  mov      qword ptr [rbp - 0x58], rdi
0x141420524  mov      byte ptr [rbp - 0x50], 1
0x141420528  lea      rcx, [rbp - 0x28]
0x14142052c  lea      rdx, [rbp - 0x58]
0x141420530  call     0x14140bee0
0x141420535  mov      rcx, qword ptr [rbp - 0x28]
0x141420539  mov      rax, qword ptr [rbp - 0x20]
0x14142053d  mov      qword ptr [rbp + 0x68], rax
0x141420541  movabs   rbx, 0x8000000000000001
0x14142054b  cmp      rcx, rbx
0x14142054e  jne      0x1414205cf
0x141420550  movabs   rbx, 0x8000000000000000
0x14142055a  mov      rax, qword ptr [rbp + 0x68]
0x14142055e  jmp      0x141420649
0x141420563  mov      qword ptr [rbp - 0x28], 3
0x14142056b  mov      rax, qword ptr [rbp + 0x58]
0x14142056f  mov      qword ptr [rbp + 0x48], rax
0x141420573  mov      rax, qword ptr [rbp + 0x70]
0x141420577  mov      qword ptr [rbp + 0x40], rax
0x14142057b  mov      rax, qword ptr [rbp + 0x60]
0x14142057f  mov      qword ptr [rbp + 0x38], rax
0x141420583  mov      byte ptr [rbp + 0x7f], 1
0x141420587  mov      byte ptr [rbp + 0x7e], 1
0x14142058b  lea      rdx, [rbp - 0x28]
0x14142058f  mov      rcx, rdi
0x141420592  call     0x1428a0920
0x141420597  mov      qword ptr [rbp + 0x68], rax
0x14142059b  mov      r12b, 1
0x14142059e  mov      bl, 1
0x1414205a0  mov      r14, qword ptr [rbp + 0x50]
0x1414205a4  mov      r15, qword ptr [rbp + 0x30]
0x1414205a8  mov      r13, qword ptr [rbp + 0x70]
0x1414205ac  mov      rax, qword ptr [rbp + 0x60]
0x1414205b0  lea      rax, [rax*2]
0x1414205b8  test     rax, rax
0x1414205bb  jne      0x1414206d5
0x1414205c1  test     r12b, r12b
0x1414205c4  jne      0x1414206ec
0x1414205ca  jmp      0x1414206fd
0x1414205cf  mov      rax, rcx
0x1414205d2  mov      qword ptr [rbp + 0x60], rcx
0x1414205d6  neg      rax
0x1414205d9  jo       0x141420a84
0x1414205df  mov      r12, qword ptr [rbp - 0x18]
0x1414205e3  lea      rcx, [rbp - 0x28]
0x1414205e7  lea      rdx, [rbp - 0x58]
0x1414205eb  call     0x14140ad50
0x1414205f0  mov      rcx, qword ptr [rbp - 0x28]
0x1414205f4  mov      r14, qword ptr [rbp - 0x20]
0x1414205f8  cmp      rcx, rbx
0x1414205fb  je       0x141420624
0x1414205fd  mov      rax, rcx
0x141420600  neg      rax
0x141420603  jno      0x14142086e
0x141420609  lea      rdx, [rip + 0x183cf48]
0x141420610  lea      r8, [rip + 0x183b8d9]
0x141420617  mov      ecx, 1
0x14142061c  call     0x1428f9d20
0x141420621  mov      r14, rax
0x141420624  movabs   rbx, 0x8000000000000000
0x14142062e  mov      rdx, qword ptr [rbp + 0x60]
0x141420632  test     rdx, rdx
0x141420635  je       0x141420646
0x141420637  mov      r8d, 1
0x14142063d  mov      rcx, qword ptr [rbp + 0x68]
0x141420641  call     0x140613c20
0x141420646  mov      rax, r14
0x141420649  inc      byte ptr [rdi + 0x30]
0x14142064c  mov      qword ptr [rbp - 0x28], rbx
0x141420650  mov      qword ptr [rbp + 0x68], rax
0x141420654  mov      qword ptr [rbp - 0x20], rax
0x141420658  mov      qword ptr [rbp - 0x38], r8
0x14142065c  mov      qword ptr [rbp - 0x18], r8
0x141420660  mov      qword ptr [rbp - 0x10], r13
0x141420664  mov      qword ptr [rbp - 8], r14
0x141420668  mov      qword ptr [rbp], r12
0x14142066c  mov      qword ptr [rbp + 0x60], rcx
0x141420670  mov      qword ptr [rbp + 8], rcx
0x141420674  mov      qword ptr [rbp + 0x10], r15
0x141420678  mov      qword ptr [rbp - 0x40], rdx
0x14142067c  mov      qword ptr [rbp + 0x18], rdx
0x141420680  mov      rcx, rdi
0x141420683  call     0x1400584f0
0x141420688  mov      rdx, rbx
0x14142068b  neg      rbx
0x14142068e  jo       0x14142094b
0x141420694  test     rax, rax
0x141420697  jne      0x14142080a
0x14142069d  mov      rax, qword ptr [rbp - 0x38]
0x1414206a1  mov      qword ptr [rsi + 0x10], rax
0x1414206a5  mov      qword ptr [rsi + 0x18], r13
0x1414206a9  mov      qword ptr [rsi + 0x20], r14
0x1414206ad  mov      qword ptr [rsi + 0x28], r12
0x1414206b1  mov      rax, qword ptr [rbp + 0x60]
0x1414206b5  mov      qword ptr [rsi + 0x30], rax
0x1414206b9  mov      qword ptr [rsi + 0x38], r15
0x1414206bd  mov      rax, qword ptr [rbp - 0x40]
0x1414206c1  mov      qword ptr [rsi + 0x40], rax
0x1414206c5  mov      qword ptr [rsi], rdx
0x1414206c8  mov      rax, qword ptr [rbp + 0x68]
0x1414206cc  mov      qword ptr [rsi + 8], rax
0x1414206d0  jmp      0x141420b80
0x1414206d5  mov      r8d, 1
0x1414206db  mov      rcx, r15
0x1414206de  mov      rdx, qword ptr [rbp + 0x60]
0x1414206e2  call     0x140613c20
0x1414206e7  test     r12b, r12b
0x1414206ea  je       0x1414206fd
0x1414206ec  lea      rax, [r13*2]
0x1414206f4  test     rax, rax
0x1414206f7  jne      0x141420789
0x1414206fd  test     bl, bl
0x1414206ff  je       0x1414207a2
0x141420705  movabs   rbx, 0x8000000000000000
0x14142070f  mov      rdx, qword ptr [rbp + 0x58]
0x141420713  lea      rax, [rdx*2]
0x14142071b  test     rax, rax
0x14142071e  mov      r12, qword ptr [rbp - 0x30]
0x141420722  je       0x141420733
0x141420724  mov      r8d, 1
0x14142072a  mov      rcx, qword ptr [rbp + 0x28]
0x14142072e  call     0x140613c20
0x141420733  mov      rax, rbx
0x141420736  jmp      0x1414207b0
0x141420738  mov      rax, qword ptr [rbp + 0x58]
0x14142073c  neg      rax
0x14142073f  jno      0x1414208f6
0x141420745  movabs   rcx, 0x8000000000000000
0x14142074f  mov      qword ptr [rbp + 0x58], rcx
0x141420753  mov      qword ptr [rbp + 0x48], rcx
0x141420757  mov      rax, qword ptr [rbp + 0x70]
0x14142075b  mov      qword ptr [rbp + 0x40], rax
0x14142075f  mov      rax, qword ptr [rbp + 0x60]
0x141420763  mov      qword ptr [rbp + 0x38], rax
0x141420767  mov      byte ptr [rbp + 0x7f], 1
0x14142076b  mov      byte ptr [rbp + 0x7e], 1
0x14142076f  lea      rcx, [rip + 0x183cdf2]
0x141420776  mov      edx, 6
0x14142077b  call     0x1428f9bc0
0x141420780  mov      qword ptr [rbp + 0x68], rax
0x141420784  jmp      0x14142059b
0x141420789  mov      r8d, 1
0x14142078f  mov      rcx, r14
0x141420792  mov      rdx, r13
0x141420795  call     0x140613c20
0x14142079a  test     bl, bl
0x14142079c  jne      0x141420705
0x1414207a2  movabs   rax, 0x8000000000000000
0x1414207ac  mov      r12, qword ptr [rbp - 0x30]
0x1414207b0  inc      byte ptr [rdi + 0x30]
0x1414207b3  mov      qword ptr [rbp - 0x28], rax
0x1414207b7  mov      rcx, qword ptr [rbp + 0x68]
0x1414207bb  mov      qword ptr [rbp - 0x20], rcx
0x1414207bf  mov      rbx, rax
0x1414207c2  mov      rax, qword ptr [rbp - 0x38]
0x1414207c6  mov      qword ptr [rbp - 0x18], rax
0x1414207ca  mov      qword ptr [rbp - 0x10], r13
0x1414207ce  mov      qword ptr [rbp - 8], r14
0x1414207d2  mov      qword ptr [rbp], r12
0x1414207d6  mov      rax, qword ptr [rbp + 0x60]
0x1414207da  mov      qword ptr [rbp + 8], rax
0x1414207de  mov      qword ptr [rbp + 0x10], r15
0x1414207e2  mov      rax, qword ptr [rbp - 0x40]
0x1414207e6  mov      qword ptr [rbp + 0x18], rax
0x1414207ea  mov      rcx, rdi
0x1414207ed  call     0x140058450
0x1414207f2  mov      rcx, rbx
0x1414207f5  neg      rcx
0x1414207f8  jo       0x141420976
0x1414207fe  mov      rdx, rbx
0x141420801  test     rax, rax
0x141420804  je       0x14142069d
0x14142080a  test     rdx, rdx
0x14142080d  jne      0x1414209c6
0x141420813  test     r13, r13
0x141420816  jne      0x1414209e4
0x14142081c  mov      rdx, qword ptr [rbp + 0x60]
0x141420820  test     rdx, rdx
0x141420823  jne      0x141420a08
0x141420829  mov      r12, rax
0x14142082c  jmp      0x141420b64
0x141420831  mov      qword ptr [rbp - 0x28], 5
0x141420839  mov      rax, qword ptr [rbp + 0x58]
0x14142083d  mov      qword ptr [rbp + 0x48], rax
0x141420841  mov      rax, qword ptr [rbp + 0x70]
0x141420845  mov      qword ptr [rbp + 0x40], rax
0x141420849  mov      rax, qword ptr [rbp + 0x60]
0x14142084d  mov      qword ptr [rbp + 0x38], rax
0x141420851  mov      byte ptr [rbp + 0x7f], 1
0x141420855  mov      byte ptr [rbp + 0x7e], 1
0x141420859  lea      rdx, [rbp - 0x28]
0x14142085d  mov      rcx, rdi
0x141420860  call     0x1428a0920
0x141420865  mov      qword ptr [rbp + 0x68], rax
0x141420869  jmp      0x14142059b
0x14142086e  mov      qword ptr [rbp + 0x70], rcx
0x141420872  mov      qword ptr [rbp + 0x50], r14
0x141420876  mov      rax, qword ptr [rbp - 0x18]
0x14142087a  mov      qword ptr [rbp - 0x30], rax
0x14142087e  lea      rcx, [rbp - 0x28]
0x141420882  lea      rdx, [rbp - 0x58]
0x141420886  call     0x14140ad50
0x14142088b  mov      rcx, qword ptr [rbp - 0x28]
0x14142088f  mov      r15, qword ptr [rbp - 0x20]
0x141420893  cmp      rcx, rbx
0x141420896  je       0x1414208bf
0x141420898  mov      rax, rcx
0x14142089b  neg      rax
0x14142089e  jno      0x141420a64
0x1414208a4  lea      rdx, [rip + 0x183ccad]
0x1414208ab  lea      r8, [rip + 0x183b63e]
0x1414208b2  mov      ecx, 2
0x1414208b7  call     0x1428f9d20
0x1414208bc  mov      r15, rax
0x1414208bf  mov      rcx, qword ptr [rbp + 0x50]
0x1414208c3  mov      rdx, qword ptr [rbp + 0x70]
0x1414208c7  test     rdx, rdx
0x1414208ca  je       0x1414208d7
0x1414208cc  mov      r8d, 1
0x1414208d2  call     0x140613c20
0x1414208d7  mov      r14, r15
0x1414208da  movabs   rbx, 0x8000000000000000
0x1414208e4  mov      rdx, qword ptr [rbp + 0x60]
0x1414208e8  test     rdx, rdx
0x1414208eb  jne      0x141420637
0x1414208f1  jmp      0x141420646
0x1414208f6  mov      r13, qword ptr [rbp + 0x70]
0x1414208fa  mov      rax, r13
0x1414208fd  neg      rax
0x141420900  seto     byte ptr [rbp + 0x68]
0x141420904  mov      r12, qword ptr [rbp - 0x30]
0x141420908  jno      0x141420a19
0x14142090e  lea      rcx, [rip + 0x183cc59]
0x141420915  mov      edx, 4
0x14142091a  call     0x1428f9bc0
0x14142091f  mov      r14, qword ptr [rbp + 0x50]
0x141420923  mov      r15, qword ptr [rbp + 0x30]
0x141420927  mov      r13, qword ptr [rbp + 0x70]
0x14142092b  jmp      0x141420b9f
0x141420930  lea      r8, [rip + 0x183bb79]
0x141420937  lea      rdx, [rbp + 0x27]
0x14142093b  mov      rcx, rdi
0x14142093e  call     0x1428f8c00
0x141420943  mov      r12, rax
0x141420946  jmp      0x141420b64
0x14142094b  test     rax, rax
0x14142094e  mov      r12, qword ptr [rbp + 0x68]
0x141420952  je       0x141420b64
0x141420958  mov      rcx, qword ptr [rax]
0x14142095b  cmp      rcx, 1
0x14142095f  mov      qword ptr [rbp + 0x70], rax
0x141420963  jne      0x141420994
0x141420965  mov      rcx, rax
0x141420968  add      rcx, 8
0x14142096c  call     0x1405e7a00
0x141420971  jmp      0x141420b50
0x141420976  test     rax, rax
0x141420979  mov      r12, qword ptr [rbp + 0x68]
0x14142097d  je       0x141420b64
0x141420983  mov      rcx, qword ptr [rax]
0x141420986  cmp      rcx, 1
0x14142098a  mov      qword ptr [rbp + 0x70], rax
0x14142098e  je       0x141420b44
0x141420994  test     rcx, rcx
0x141420997  jne      0x141420b50
0x14142099d  mov      rax, qword ptr [rbp + 0x70]
0x1414209a1  mov      rdx, qword ptr [rax + 0x10]
0x1414209a5  test     rdx, rdx
0x1414209a8  je       0x141420b50
0x1414209ae  mov      rax, qword ptr [rbp + 0x70]
0x1414209b2  mov      rcx, qword ptr [rax + 8]
0x1414209b6  mov      r8d, 1
0x1414209bc  call     0x140613c20
0x1414209c1  jmp      0x141420b50
0x1414209c6  mov      r8d, 1
0x1414209cc  mov      rbx, rax
0x1414209cf  mov      rcx, qword ptr [rbp + 0x68]
0x1414209d3  call     0x140613c20
0x1414209d8  mov      rax, rbx
0x1414209db  test     r13, r13
0x1414209de  je       0x14142081c
0x1414209e4  mov      r8d, 1
0x1414209ea  mov      rbx, rax
0x1414209ed  mov      rcx, r14
0x1414209f0  mov      rdx, r13
0x1414209f3  call     0x140613c20
0x1414209f8  mov      rax, rbx
0x1414209fb  mov      rdx, qword ptr [rbp + 0x60]
0x1414209ff  test     rdx, rdx
0x141420a02  je       0x141420829
0x141420a08  mov      r8d, 1
0x141420a0e  mov      r12, rax
0x141420a11  mov      rcx, r15
0x141420a14  jmp      0x141420b5f
0x141420a19  mov      rax, qword ptr [rbp + 0x60]
0x141420a1d  neg      rax
0x141420a20  jno      0x141420aa8
0x141420a26  lea      rcx, [rip + 0x183c9b1]
0x141420a2d  mov      edx, 0xa
0x141420a32  call     0x1428f9bc0
0x141420a37  mov      r13, qword ptr [rbp + 0x70]
0x141420a3b  test     r13, r13
0x141420a3e  je       0x141420b97
0x141420a44  mov      r8d, 1
0x141420a4a  mov      r14, qword ptr [rbp + 0x50]
0x141420a4e  mov      rcx, r14
0x141420a51  mov      rdx, r13
0x141420a54  mov      rbx, rax
0x141420a57  call     0x140613c20
0x141420a5c  mov      rax, rbx
0x141420a5f  jmp      0x141420b9b
0x141420a64  mov      rdx, qword ptr [rbp - 0x18]
0x141420a68  mov      r14, qword ptr [rbp + 0x50]
0x141420a6c  mov      r13, qword ptr [rbp + 0x70]
0x141420a70  mov      r8, r12
0x141420a73  mov      r12, qword ptr [rbp - 0x30]
0x141420a77  mov      rax, qword ptr [rbp + 0x68]
0x141420a7b  mov      rbx, qword ptr [rbp + 0x60]
0x141420a7f  jmp      0x141420649
0x141420a84  lea      rdx, [rip + 0x183cacd]
0x141420a8b  lea      r8, [rip + 0x183b45e]
0x141420a92  xor      ecx, ecx
0x141420a94  call     0x1428f9d20
0x141420a99  movabs   rbx, 0x8000000000000000
0x141420aa3  jmp      0x141420649
0x141420aa8  mov      rax, qword ptr [rbp + 0x58]
0x141420aac  mov      rcx, qword ptr [rbp + 0x28]
0x141420ab0  mov      qword ptr [rbp + 0x68], rcx
0x141420ab4  mov      r14, qword ptr [rbp + 0x50]
0x141420ab8  mov      r15, qword ptr [rbp + 0x30]
0x141420abc  jmp      0x1414207b0
0x141420ac1  movzx    eax, r8b
0x141420ac5  cmp      eax, 0x7d
0x141420ac8  jne      0x141420b07
0x141420aca  mov      qword ptr [rbp - 0x28], 0x12
0x141420ad2  mov      rax, qword ptr [rbp + 0x58]
0x141420ad6  mov      qword ptr [rbp + 0x48], rax
0x141420ada  mov      rax, qword ptr [rbp + 0x70]
0x141420ade  mov      qword ptr [rbp + 0x40], rax
0x141420ae2  mov      rax, qword ptr [rbp + 0x60]
0x141420ae6  mov      qword ptr [rbp + 0x38], rax
0x141420aea  mov      byte ptr [rbp + 0x7f], 1
0x141420aee  mov      byte ptr [rbp + 0x7e], 1
0x141420af2  lea      rdx, [rbp - 0x28]
0x141420af6  mov      rcx, rdi
0x141420af9  call     0x1428a0920
0x141420afe  mov      qword ptr [rbp + 0x68], rax
0x141420b02  jmp      0x14142059b
0x141420b07  mov      qword ptr [rbp - 0x28], 0x10
0x141420b0f  mov      rax, qword ptr [rbp + 0x58]
0x141420b13  mov      qword ptr [rbp + 0x48], rax
0x141420b17  mov      rax, qword ptr [rbp + 0x70]
0x141420b1b  mov      qword ptr [rbp + 0x40], rax
0x141420b1f  mov      rax, qword ptr [rbp + 0x60]
0x141420b23  mov      qword ptr [rbp + 0x38], rax
0x141420b27  mov      byte ptr [rbp + 0x7f], 1
0x141420b2b  mov      byte ptr [rbp + 0x7e], 1
0x141420b2f  lea      rdx, [rbp - 0x28]
0x141420b33  mov      rcx, rdi
0x141420b36  call     0x1428a0920
0x141420b3b  mov      qword ptr [rbp + 0x68], rax
0x141420b3f  jmp      0x14142059b
0x141420b44  mov      rcx, rax
0x141420b47  add      rcx, 8
0x141420b4b  call     0x1405e7a00
0x141420b50  mov      edx, 0x28
0x141420b55  mov      r8d, 8
0x141420b5b  mov      rcx, qword ptr [rbp + 0x70]
0x141420b5f  call     0x140613c20
0x141420b64  mov      rcx, r12
0x141420b67  mov      rdx, rdi
0x141420b6a  call     0x1428a2200
0x141420b6f  mov      qword ptr [rsi + 8], rax
0x141420b73  movabs   rax, 0x8000000000000000
0x141420b7d  mov      qword ptr [rsi], rax
0x141420b80  mov      rax, rsi
0x141420b83  add      rsp, 0x108
0x141420b8a  pop      rbx
0x141420b8b  pop      rdi
0x141420b8c  pop      rsi
0x141420b8d  pop      r12
0x141420b8f  pop      r13
0x141420b91  pop      r14
0x141420b93  pop      r15
0x141420b95  pop      rbp
0x141420b96  ret      
0x141420b97  mov      r14, qword ptr [rbp + 0x50]
0x141420b9b  mov      r15, qword ptr [rbp + 0x30]
0x141420b9f  mov      rdx, qword ptr [rbp + 0x58]
0x141420ba3  movzx    r12d, byte ptr [rbp + 0x68]
0x141420ba8  mov      qword ptr [rbp + 0x68], rax
0x141420bac  test     rdx, rdx
0x141420baf  je       0x141420bc2
0x141420bb1  mov      r8d, 1
0x141420bb7  mov      rcx, qword ptr [rbp + 0x28]
0x141420bbb  call     0x140613c20
0x141420bc0  jmp      0x141420bca
0x141420bc2  mov      qword ptr [rbp + 0x58], 0
0x141420bca  xor      ebx, ebx
0x141420bcc  jmp      0x1414205ac
0x141420bd1  mov      qword ptr [rbp - 0x28], 3
0x141420bd9  mov      rax, qword ptr [rbp + 0x58]
0x141420bdd  mov      qword ptr [rbp + 0x48], rax
0x141420be1  movabs   rax, 0x8000000000000000
0x141420beb  mov      qword ptr [rbp + 0x40], rax
0x141420bef  mov      rax, qword ptr [rbp + 0x60]
0x141420bf3  mov      qword ptr [rbp + 0x38], rax
0x141420bf7  mov      byte ptr [rbp + 0x7f], 1
0x141420bfb  mov      byte ptr [rbp + 0x7e], 1
0x141420bff  lea      rdx, [rbp - 0x28]
0x141420c03  mov      rcx, rdi
0x141420c06  call     0x1428a0920
0x141420c0b  mov      r12b, 1
0x141420c0e  movabs   r13, 0x8000000000000000
0x141420c18  mov      qword ptr [rbp + 0x68], rax
0x141420c1c  mov      bl, 1
0x141420c1e  mov      r14, qword ptr [rbp + 0x50]
0x141420c22  mov      r15, qword ptr [rbp + 0x30]
0x141420c26  jmp      0x1414205ac
0x141420c2b  mov      qword ptr [rbp - 0x28], 3
0x141420c33  mov      rax, qword ptr [rbp + 0x58]
0x141420c37  mov      qword ptr [rbp + 0x48], rax
0x141420c3b  mov      rax, qword ptr [rbp + 0x70]
0x141420c3f  mov      qword ptr [rbp + 0x40], rax
0x141420c43  movabs   rax, 0x8000000000000000
0x141420c4d  mov      qword ptr [rbp + 0x38], rax
0x141420c51  mov      byte ptr [rbp + 0x7f], 1
0x141420c55  mov      byte ptr [rbp + 0x7e], 1
0x141420c59  lea      rdx, [rbp - 0x28]
0x141420c5d  mov      rcx, rdi
0x141420c60  call     0x1428a0920
0x141420c65  movabs   rcx, 0x8000000000000000
0x141420c6f  mov      qword ptr [rbp + 0x60], rcx
0x141420c73  mov      bl, 1
0x141420c75  mov      qword ptr [rbp + 0x68], rax
0x141420c79  mov      r14, qword ptr [rbp + 0x50]
0x141420c7d  mov      r15, qword ptr [rbp + 0x30]
0x141420c81  mov      r13, qword ptr [rbp + 0x70]
0x141420c85  jmp      0x1414206ec
0x141420c8a  mov      qword ptr [rbp - 0x28], 3
0x141420c92  movabs   rax, 0x8000000000000000
0x141420c9c  mov      qword ptr [rbp + 0x48], rax
0x141420ca0  mov      rax, qword ptr [rbp + 0x70]
0x141420ca4  mov      qword ptr [rbp + 0x40], rax
0x141420ca8  mov      rax, qword ptr [rbp + 0x60]
0x141420cac  mov      qword ptr [rbp + 0x38], rax
0x141420cb0  mov      byte ptr [rbp + 0x7f], 1
0x141420cb4  mov      byte ptr [rbp + 0x7e], 1
0x141420cb8  lea      rdx, [rbp - 0x28]
0x141420cbc  mov      rcx, rdi
0x141420cbf  call     0x1428a0920
0x141420cc4  mov      qword ptr [rbp + 0x68], rax
0x141420cc8  mov      r12b, 1
0x141420ccb  movabs   rax, 0x8000000000000000
0x141420cd5  mov      qword ptr [rbp + 0x58], rax
0x141420cd9  jmp      0x14142059e
0x141420cde  mov      qword ptr [rbp - 0x28], 0x15
0x141420ce6  jmp      0x1414200be
0x141420ceb  mov      qword ptr [rbp - 0x28], 8
0x141420cf3  mov      rax, qword ptr [rbp + 0x58]
0x141420cf7  mov      qword ptr [rbp + 0x48], rax
0x141420cfb  mov      rax, qword ptr [rbp + 0x70]
0x141420cff  mov      qword ptr [rbp + 0x40], rax
0x141420d03  mov      rax, qword ptr [rbp + 0x60]
0x141420d07  mov      qword ptr [rbp + 0x38], rax
0x141420d0b  mov      byte ptr [rbp + 0x7f], 1
0x141420d0f  mov      byte ptr [rbp + 0x7e], 1
0x141420d13  lea      rdx, [rbp - 0x28]
0x141420d17  mov      rcx, rdi
0x141420d1a  call     0x1428a0920
0x141420d1f  mov      qword ptr [rbp + 0x68], rax
0x141420d23  jmp      0x14142059b
0x141420d28  mov      qword ptr [rbp + 0x48], rcx
0x141420d2c  mov      rax, qword ptr [rbp + 0x70]
0x141420d30  mov      qword ptr [rbp + 0x40], rax
0x141420d34  mov      rax, qword ptr [rbp + 0x60]
0x141420d38  mov      qword ptr [rbp + 0x38], rax
0x141420d3c  mov      byte ptr [rbp + 0x7f], 1
0x141420d40  mov      byte ptr [rbp + 0x7e], 1
0x141420d44  lea      rcx, [rip + 0x183c81d]
0x141420d4b  mov      edx, 6
0x141420d50  call     0x1428f9db0
0x141420d55  mov      qword ptr [rbp + 0x68], rax
0x141420d59  jmp      0x14142059b
0x141420d5e  mov      rax, qword ptr [rbp + 0x58]
0x141420d62  mov      qword ptr [rbp + 0x48], rax
0x141420d66  mov      rax, qword ptr [rbp + 0x70]
0x141420d6a  mov      qword ptr [rbp + 0x40], rax
0x141420d6e  mov      qword ptr [rbp + 0x38], rcx
0x141420d72  mov      byte ptr [rbp + 0x7f], 1
0x141420d76  mov      byte ptr [rbp + 0x7e], 1
0x141420d7a  lea      rcx, [rip + 0x183c65d]
0x141420d81  mov      edx, 0xa
0x141420d86  call     0x1428f9db0
0x141420d8b  mov      qword ptr [rbp + 0x68], rax
0x141420d8f  mov      bl, 1
0x141420d91  cmp      qword ptr [rbp + 0x60], 0
0x141420d96  je       0x141420eb6
0x141420d9c  mov      r8d, 1
0x141420da2  mov      r15, qword ptr [rbp + 0x30]
0x141420da6  mov      rcx, r15
0x141420da9  mov      rdx, qword ptr [rbp + 0x60]
0x141420dad  call     0x140613c20
0x141420db2  mov      r14, qword ptr [rbp + 0x50]
0x141420db6  mov      r13, qword ptr [rbp + 0x70]
0x141420dba  jmp      0x1414206ec
0x141420dbf  mov      rax, qword ptr [rbp + 0x58]
0x141420dc3  mov      qword ptr [rbp + 0x48], rax
0x141420dc7  mov      qword ptr [rbp + 0x40], rcx
0x141420dcb  mov      rax, qword ptr [rbp + 0x60]
0x141420dcf  mov      qword ptr [rbp + 0x38], rax
0x141420dd3  mov      byte ptr [rbp + 0x7f], 1
0x141420dd7  mov      byte ptr [rbp + 0x7e], 1
0x141420ddb  lea      rcx, [rip + 0x183c78c]
0x141420de2  mov      edx, 4
0x141420de7  call     0x1428f9db0
0x141420dec  mov      qword ptr [rbp + 0x68], rax
0x141420df0  jmp      0x14142059b
0x141420df5  mov      qword ptr [rbp - 0x28], 6
0x141420dfd  movabs   rax, 0x8000000000000000
0x141420e07  mov      qword ptr [rbp + 0x48], rax
0x141420e0b  mov      rax, qword ptr [rbp + 0x70]
0x141420e0f  mov      qword ptr [rbp + 0x40], rax
0x141420e13  mov      rax, qword ptr [rbp + 0x60]
0x141420e17  mov      qword ptr [rbp + 0x38], rax
0x141420e1b  mov      byte ptr [rbp + 0x7f], 1
0x141420e1f  mov      byte ptr [rbp + 0x7e], 1
0x141420e23  lea      rdx, [rbp - 0x28]
0x141420e27  mov      rcx, rdi
0x141420e2a  call     0x1428a0920
0x141420e2f  mov      qword ptr [rbp + 0x68], rax
0x141420e33  jmp      0x141420cc8
0x141420e38  mov      qword ptr [rbp - 0x28], 6
0x141420e40  mov      rax, qword ptr [rbp + 0x58]
0x141420e44  mov      qword ptr [rbp + 0x48], rax
0x141420e48  mov      rax, qword ptr [rbp + 0x70]
0x141420e4c  mov      qword ptr [rbp + 0x40], rax
0x141420e50  movabs   rax, 0x8000000000000000
0x141420e5a  mov      qword ptr [rbp + 0x38], rax
0x141420e5e  mov      byte ptr [rbp + 0x7f], 1
0x141420e62  mov      byte ptr [rbp + 0x7e], 1
0x141420e66  lea      rdx, [rbp - 0x28]
0x141420e6a  mov      rcx, rdi
0x141420e6d  call     0x1428a0920
0x141420e72  jmp      0x141420c65
0x141420e77  mov      qword ptr [rbp - 0x28], 6
0x141420e7f  mov      rax, qword ptr [rbp + 0x58]
0x141420e83  mov      qword ptr [rbp + 0x48], rax
0x141420e87  movabs   rax, 0x8000000000000000
0x141420e91  mov      qword ptr [rbp + 0x40], rax
0x141420e95  mov      rax, qword ptr [rbp + 0x60]
0x141420e99  mov      qword ptr [rbp + 0x38], rax
0x141420e9d  mov      byte ptr [rbp + 0x7f], 1
0x141420ea1  mov      byte ptr [rbp + 0x7e], 1
0x141420ea5  lea      rdx, [rbp - 0x28]
0x141420ea9  mov      rcx, rdi
0x141420eac  call     0x1428a0920
0x141420eb1  jmp      0x141420c0b
0x141420eb6  mov      qword ptr [rbp + 0x60], 0
0x141420ebe  jmp      0x141420c79
0x141420ec3  nop      word ptr cs:[rax + rax]
0x141420ed0  mov      qword ptr [rsp + 0x10], rdx
0x141420ed5  push     rbp
0x141420ed6  push     r15
0x141420ed8  push     r14
0x141420eda  push     r13
0x141420edc  push     r12
0x141420ede  push     rsi
0x141420edf  push     rdi
0x141420ee0  push     rbx
0x141420ee1  sub      rsp, 0x28
0x141420ee5  lea      rbp, [rdx + 0x80]
0x141420eec  cmp      qword ptr [rbp + 0x70], 0
0x141420ef1  je       0x141420f06
0x141420ef3  mov      r8d, 1
0x141420ef9  mov      rcx, qword ptr [rbp + 0x50]
0x141420efd  mov      rdx, qword ptr [rbp + 0x70]
0x141420f01  call     0x140613c20
0x141420f06  nop      
0x141420f07  add      rsp, 0x28
0x141420f0b  pop      rbx
0x141420f0c  pop      rdi
0x141420f0d  pop      rsi
0x141420f0e  pop      r12
0x141420f10  pop      r13
0x141420f12  pop      r14
0x141420f14  pop      r15
0x141420f16  pop      rbp
0x141420f17  ret      
0x141420f18  nop      dword ptr [rax + rax]
0x141420f20  mov      qword ptr [rsp + 0x10], rdx
0x141420f25  push     rbp
0x141420f26  push     r15
0x141420f28  push     r14
0x141420f2a  push     r13
0x141420f2c  push     r12
0x141420f2e  push     rsi
0x141420f2f  push     rdi
0x141420f30  push     rbx
0x141420f31  sub      rsp, 0x28
0x141420f35  lea      rbp, [rdx + 0x80]
0x141420f3c  cmp      qword ptr [rbp + 0x58], 0
0x141420f41  je       0x141420f56
0x141420f43  mov      r8d, 1
0x141420f49  mov      rcx, qword ptr [rbp + 0x28]
0x141420f4d  mov      rdx, qword ptr [rbp + 0x58]
0x141420f51  call     0x140613c20
0x141420f56  mov      rax, qword ptr [rbp + 0x58]
0x141420f5a  mov      qword ptr [rbp + 0x48], rax
0x141420f5e  mov      rax, qword ptr [rbp + 0x70]
0x141420f62  mov      qword ptr [rbp + 0x40], rax
0x141420f66  mov      rax, qword ptr [rbp + 0x60]
0x141420f6a  mov      qword ptr [rbp + 0x38], rax
0x141420f6e  movzx    eax, byte ptr [rbp + 0x68]
0x141420f72  mov      byte ptr [rbp + 0x7f], al
0x141420f75  mov      byte ptr [rbp + 0x7e], 0
0x141420f79  add      rsp, 0x28
0x141420f7d  pop      rbx
0x141420f7e  pop      rdi
0x141420f7f  pop      rsi
0x141420f80  pop      r12
0x141420f82  pop      r13
0x141420f84  pop      r14
0x141420f86  pop      r15
0x141420f88  pop      rbp
0x141420f89  ret      
0x141420f8a  nop      word ptr [rax + rax]
0x141420f90  mov      qword ptr [rsp + 0x10], rdx
0x141420f95  push     rbp
0x141420f96  push     r15
0x141420f98  push     r14
0x141420f9a  push     r13
0x141420f9c  push     r12
0x141420f9e  push     rsi
0x141420f9f  push     rdi
0x141420fa0  push     rbx
0x141420fa1  sub      rsp, 0x28
0x141420fa5  lea      rbp, [rdx + 0x80]
0x141420fac  cmp      qword ptr [rbp + 0x70], 0
0x141420fb1  je       0x141420fc6
0x141420fb3  mov      r8d, 1
0x141420fb9  mov      rcx, qword ptr [rbp + 0x50]
0x141420fbd  mov      rdx, qword ptr [rbp + 0x70]
0x141420fc1  call     0x140613c20
0x141420fc6  nop      
0x141420fc7  add      rsp, 0x28
0x141420fcb  pop      rbx
0x141420fcc  pop      rdi
0x141420fcd  pop      rsi
0x141420fce  pop      r12
0x141420fd0  pop      r13
0x141420fd2  pop      r14
0x141420fd4  pop      r15
0x141420fd6  pop      rbp
0x141420fd7  ret      
0x141420fd8  nop      dword ptr [rax + rax]
0x141420fe0  mov      qword ptr [rsp + 0x10], rdx
0x141420fe5  push     rbp
0x141420fe6  push     r15
0x141420fe8  push     r14
0x141420fea  push     r13
0x141420fec  push     r12
0x141420fee  push     rsi
0x141420fef  push     rdi
0x141420ff0  push     rbx
0x141420ff1  sub      rsp, 0x28
0x141420ff5  lea      rbp, [rdx + 0x80]
0x141420ffc  mov      edx, 0x28
0x141421001  mov      r8d, 8
0x141421007  mov      rcx, qword ptr [rbp + 0x70]
0x14142100b  call     0x140613c20
0x141421010  nop      
0x141421011  add      rsp, 0x28
0x141421015  pop      rbx
0x141421016  pop      rdi
0x141421017  pop      rsi
0x141421018  pop      r12
0x14142101a  pop      r13
0x14142101c  pop      r14
0x14142101e  pop      r15
0x141421020  pop      rbp
0x141421021  ret      
0x141421022  nop      word ptr cs:[rax + rax]
0x141421030  mov      qword ptr [rsp + 0x10], rdx
0x141421035  push     rbp
0x141421036  push     r15
0x141421038  push     r14
0x14142103a  push     r13
0x14142103c  push     r12
0x14142103e  push     rsi
0x14142103f  push     rdi
0x141421040  push     rbx
0x141421041  sub      rsp, 0x28
0x141421045  lea      rbp, [rdx + 0x80]
0x14142104c  mov      edx, 0x28
0x141421051  mov      r8d, 8
0x141421057  mov      rcx, qword ptr [rbp + 0x70]
0x14142105b  call     0x140613c20
0x141421060  nop      
0x141421061  add      rsp, 0x28
0x141421065  pop      rbx
0x141421066  pop      rdi
0x141421067  pop      rsi
0x141421068  pop      r12
0x14142106a  pop      r13
0x14142106c  pop      r14
0x14142106e  pop      r15
0x141421070  pop      rbp
0x141421071  ret      
0x141421072  nop      word ptr cs:[rax + rax]
0x141421080  mov      qword ptr [rsp + 0x10], rdx
0x141421085  push     rbp
0x141421086  push     r15
0x141421088  push     r14
0x14142108a  push     r13
0x14142108c  push     r12
0x14142108e  push     rsi
0x14142108f  push     rdi
0x141421090  push     rbx
0x141421091  sub      rsp, 0x28
0x141421095  lea      rbp, [rdx + 0x80]
0x14142109c  cmp      qword ptr [rbp + 0x60], 0
0x1414210a1  je       0x1414210b6
0x1414210a3  mov      r8d, 1
0x1414210a9  mov      rcx, qword ptr [rbp + 0x68]
0x1414210ad  mov      rdx, qword ptr [rbp + 0x60]
0x1414210b1  call     0x140613c20
0x1414210b6  nop      
0x1414210b7  add      rsp, 0x28
0x1414210bb  pop      rbx
0x1414210bc  pop      rdi
0x1414210bd  pop      rsi
0x1414210be  pop      r12
0x1414210c0  pop      r13
0x1414210c2  pop      r14
0x1414210c4  pop      r15
0x1414210c6  pop      rbp
0x1414210c7  ret      
0x1414210c8  nop      dword ptr [rax + rax]
0x1414210d0  mov      qword ptr [rsp + 0x10], rdx
0x1414210d5  push     rbp
0x1414210d6  push     r15
0x1414210d8  push     r14
0x1414210da  push     r13
0x1414210dc  push     r12
0x1414210de  push     rsi
0x1414210df  push     rdi
0x1414210e0  push     rbx
0x1414210e1  sub      rsp, 0x28
0x1414210e5  lea      rbp, [rdx + 0x80]
0x1414210ec  lea      rcx, [rbp - 0x28]
0x1414210f0  call     0x1414088a0
0x1414210f5  nop      
0x1414210f6  add      rsp, 0x28
0x1414210fa  pop      rbx
0x1414210fb  pop      rdi
0x1414210fc  pop      rsi
0x1414210fd  pop      r12
0x1414210ff  pop      r13
0x141421101  pop      r14
0x141421103  pop      r15
0x141421105  pop      rbp
0x141421106  ret      
0x141421107  nop      word ptr [rax + rax]
0x141421110  mov      qword ptr [rsp + 0x10], rdx
0x141421115  push     rbp
0x141421116  push     r15
0x141421118  push     r14
0x14142111a  push     r13
0x14142111c  push     r12
0x14142111e  push     rsi
0x14142111f  push     rdi
0x141421120  push     rbx
0x141421121  sub      rsp, 0x28
0x141421125  lea      rbp, [rdx + 0x80]
0x14142112c  lea      rcx, [rbp - 0x28]
0x141421130  call     0x1414088a0
0x141421135  nop      
0x141421136  add      rsp, 0x28
0x14142113a  pop      rbx
0x14142113b  pop      rdi
0x14142113c  pop      rsi
0x14142113d  pop      r12
0x14142113f  pop      r13
0x141421141  pop      r14
0x141421143  pop      r15
0x141421145  pop      rbp
0x141421146  ret      
0x141421147  nop      word ptr [rax + rax]
0x141421150  mov      qword ptr [rsp + 0x10], rdx
0x141421155  push     rbp
0x141421156  push     r15
0x141421158  push     r14
0x14142115a  push     r13
0x14142115c  push     r12
0x14142115e  push     rsi
0x14142115f  push     rdi
0x141421160  push     rbx
0x141421161  sub      rsp, 0x28
0x141421165  lea      rbp, [rdx + 0x80]
0x14142116c  movzx    eax, byte ptr [rbp + 0x7e]
0x141421170  mov      byte ptr [rbp - 0x40], al
0x141421173  movzx    eax, byte ptr [rbp + 0x7f]
0x141421177  mov      byte ptr [rbp - 0x38], al
0x14142117a  mov      rdx, qword ptr [rbp + 0x38]
0x14142117e  mov      rax, qword ptr [rbp + 0x40]
0x141421182  mov      qword ptr [rbp + 0x68], rax
0x141421186  mov      rax, qword ptr [rbp + 0x48]
0x14142118a  mov      qword ptr [rbp + 0x60], rax
0x14142118e  mov      rax, rdx
0x141421191  neg      rax
0x141421194  jo       0x1414211b9
0x141421196  jae      0x1414211a7
0x141421198  mov      r8d, 1
0x14142119e  mov      rcx, qword ptr [rbp + 0x30]
0x1414211a2  call     0x140613c20
0x1414211a7  nop      
0x1414211a8  add      rsp, 0x28
0x1414211ac  pop      rbx
0x1414211ad  pop      rdi
0x1414211ae  pop      rsi
0x1414211af  pop      r12
0x1414211b1  pop      r13
0x1414211b3  pop      r14
0x1414211b5  pop      r15
0x1414211b7  pop      rbp
0x1414211b8  ret      
0x1414211b9  add      rsp, 0x28
0x1414211bd  pop      rbx
0x1414211be  pop      rdi
0x1414211bf  pop      rsi
0x1414211c0  pop      r12
0x1414211c2  pop      r13
0x1414211c4  pop      r14
0x1414211c6  pop      r15
0x1414211c8  pop      rbp
0x1414211c9  ret      
0x1414211ca  nop      word ptr [rax + rax]
0x1414211d0  mov      qword ptr [rsp + 0x10], rdx
0x1414211d5  push     rbp
0x1414211d6  push     r15
0x1414211d8  push     r14
0x1414211da  push     r13
0x1414211dc  push     r12
0x1414211de  push     rsi
0x1414211df  push     rdi
0x1414211e0  push     rbx
0x1414211e1  sub      rsp, 0x28
0x1414211e5  lea      rbp, [rdx + 0x80]
0x1414211ec  mov      rax, qword ptr [rbp + 0x68]
0x1414211f0  neg      rax
0x1414211f3  jno      0x141421206
0x1414211f5  add      rsp, 0x28
0x1414211f9  pop      rbx
0x1414211fa  pop      rdi
0x1414211fb  pop      rsi
0x1414211fc  pop      r12
0x1414211fe  pop      r13
0x141421200  pop      r14
0x141421202  pop      r15
0x141421204  pop      rbp
0x141421205  ret      
0x141421206  test     byte ptr [rbp - 0x38], 1
0x14142120a  je       0x141421238
0x14142120c  cmp      qword ptr [rbp + 0x68], 0
0x141421211  je       0x141421226
0x141421213  mov      r8d, 1
0x141421219  mov      rcx, qword ptr [rbp + 0x50]
0x14142121d  mov      rdx, qword ptr [rbp + 0x68]
0x141421221  call     0x140613c20
0x141421226  nop      
0x141421227  add      rsp, 0x28
0x14142122b  pop      rbx
0x14142122c  pop      rdi
0x14142122d  pop      rsi
0x14142122e  pop      r12
0x141421230  pop      r13
0x141421232  pop      r14
0x141421234  pop      r15
0x141421236  pop      rbp
0x141421237  ret      
0x141421238  add      rsp, 0x28
0x14142123c  pop      rbx
0x14142123d  pop      rdi
0x14142123e  pop      rsi
0x14142123f  pop      r12
0x141421241  pop      r13
0x141421243  pop      r14
0x141421245  pop      r15
0x141421247  pop      rbp
0x141421248  ret      
0x141421249  nop      dword ptr [rax]
0x141421250  mov      qword ptr [rsp + 0x10], rdx
0x141421255  push     rbp
0x141421256  push     r15
0x141421258  push     r14
0x14142125a  push     r13
0x14142125c  push     r12
0x14142125e  push     rsi
0x14142125f  push     rdi
0x141421260  push     rbx
0x141421261  sub      rsp, 0x28
0x141421265  lea      rbp, [rdx + 0x80]
0x14142126c  mov      rax, qword ptr [rbp + 0x60]
0x141421270  neg      rax
0x141421273  jo       0x141421295
0x141421275  test     byte ptr [rbp - 0x40], 1
0x141421279  je       0x141421295
0x14142127b  cmp      qword ptr [rbp + 0x60], 0
0x141421280  je       0x141421295
0x141421282  mov      r8d, 1
0x141421288  mov      rcx, qword ptr [rbp + 0x28]
0x14142128c  mov      rdx, qword ptr [rbp + 0x60]
0x141421290  call     0x140613c20
0x141421295  nop      
0x141421296  add      rsp, 0x28
0x14142129a  pop      rbx
0x14142129b  pop      rdi
0x14142129c  pop      rsi
0x14142129d  pop      r12
0x14142129f  pop      r13
0x1414212a1  pop      r14
0x1414212a3  pop      r15
0x1414212a5  pop      rbp
0x1414212a6  ret      
0x1414212a7  int3     
0x1414212a8  int3     
0x1414212a9  int3     
0x1414212aa  int3     
0x1414212ab  int3     
0x1414212ac  int3     
0x1414212ad  int3     
0x1414212ae  int3     
0x1414212af  int3     
0x1414212b0  push     rbp
0x1414212b1  push     r15
0x1414212b3  push     r14
0x1414212b5  push     r13
0x1414212b7  push     r12
0x1414212b9  push     rsi
0x1414212ba  push     rdi
0x1414212bb  push     rbx
0x1414212bc  sub      rsp, 0xc8
0x1414212c3  lea      rbp, [rsp + 0x80]
0x1414212cb  mov      qword ptr [rbp + 0x40], 0xfffffffffffffffe
0x1414212d3  mov      rdi, rdx
0x1414212d6  mov      r12, rcx
0x1414212d9  mov      r10, qword ptr [rdx + 0x20]
0x1414212dd  mov      rax, qword ptr [rdx + 0x28]
0x1414212e1  cmp      rax, r10
0x1414212e4  jae      0x141421336
0x1414212e6  lea      r14, [rdi + 0x18]
0x1414212ea  mov      rdx, qword ptr [rdi + 0x18]
0x1414212ee  inc      rax
0x1414212f1  mov      r8, r10
0x1414212f4  neg      r8
0x1414212f7  movabs   r15, 0x100002600
0x141421301  nop      word ptr cs:[rax + rax]
0x141421310  movzx    r9d, byte ptr [rdx + rax - 1]
0x141421316  cmp      r9, 0x20
0x14142131a  ja       0x14142134f
0x14142131c  bt       r15, r9
0x141421320  jae      0x14142134f
0x141421322  mov      qword ptr [rdi + 0x28], rax
0x141421326  lea      r9, [r8 + rax]
0x14142132a  inc      r9
0x14142132d  inc      rax
0x141421330  cmp      r9, 1
0x141421334  jne      0x141421310
0x141421336  mov      qword ptr [rbp - 0x18], 5
0x14142133e  lea      rdx, [rbp - 0x18]
0x141421342  mov      rcx, rdi
0x141421345  call     0x1428a0920
0x14142134a  jmp      0x141421d0f
0x14142134f  cmp      r9d, 0x5b
0x141421353  mov      qword ptr [rbp - 0x20], r12
0x141421357  je       0x14142163f
0x14142135d  cmp      r9d, 0x7b
0x141421361  jne      0x1414218fd
0x141421367  dec      byte ptr [rdi + 0x30]
0x14142136a  je       0x141421b5a
0x141421370  mov      qword ptr [rdi + 0x28], rax
0x141421374  mov      r9b, 1
0x141421377  movabs   r13, 0x8000000000000000
0x141421381  lea      rcx, [rbp - 0x18]
0x141421385  lea      rsi, [rbp - 0x58]
0x141421389  mov      qword ptr [rbp + 0x30], r13
0x14142138d  mov      rbx, r13
0x141421390  cmp      rax, r10
0x141421393  jae      0x1414216e7
0x141421399  mov      rdx, qword ptr [r14]
0x14142139c  nop      dword ptr [rax]
0x1414213a0  movzx    r8d, byte ptr [rdx + rax]
0x1414213a5  cmp      r8, 0x2c
0x1414213a9  ja       0x141421412
0x1414213ab  bt       r15, r8
0x1414213af  jae      0x1414213c2
0x1414213b1  inc      rax
0x1414213b4  mov      qword ptr [rdi + 0x28], rax
0x1414213b8  cmp      r10, rax
0x1414213bb  jne      0x1414213a0
0x1414213bd  jmp      0x1414216e7
0x1414213c2  cmp      r8, 0x2c
0x1414213c6  jne      0x141421412
0x1414213c8  test     r9b, 1
0x1414213cc  jne      0x141421a77
0x1414213d2  inc      rax
0x1414213d5  mov      qword ptr [rdi + 0x28], rax
0x1414213d9  cmp      rax, r10
0x1414213dc  jae      0x14142188f
0x1414213e2  nop      word ptr cs:[rax + rax]
0x1414213f0  movzx    r8d, byte ptr [rdx + rax]
0x1414213f5  cmp      r8, 0x20
0x1414213f9  ja       0x141421429
0x1414213fb  bt       r15, r8
0x1414213ff  jae      0x141421429
0x141421401  inc      rax
0x141421404  mov      qword ptr [rdi + 0x28], rax
0x141421408  cmp      r10, rax
0x14142140b  jne      0x1414213f0
0x14142140d  jmp      0x14142188f
0x141421412  cmp      r8d, 0x7d
0x141421416  je       0x141421853
0x14142141c  test     r9b, 1
0x141421420  je       0x141421b67
0x141421426  xor      r9d, r9d
0x141421429  cmp      r8b, 0x22
0x14142142d  jne      0x141421a0b
0x141421433  mov      dword ptr [rbp - 0x3c], r9d
0x141421437  inc      rax
0x14142143a  mov      qword ptr [rdi + 0x28], rax
0x14142143e  mov      qword ptr [rdi + 0x10], 0
0x141421446  mov      rax, qword ptr [rbp + 0x30]
0x14142144a  mov      qword ptr [rbp + 0x20], rax
0x14142144e  mov      qword ptr [rbp + 0x18], rbx
0x141421452  mov      byte ptr [rbp + 0x3f], 1
0x141421456  mov      r12, rcx
0x141421459  mov      rdx, r14
0x14142145c  mov      r8, rdi
0x14142145f  call     0x140b8a910
0x141421464  cmp      dword ptr [rbp - 0x18], 2
0x141421468  mov      rcx, qword ptr [rbp - 0x10]
0x14142146c  je       0x141421adb
0x141421472  mov      rax, qword ptr [rbp - 8]
0x141421476  cmp      rax, 9
0x14142147a  je       0x1414214f2
0x14142147c  cmp      rax, 8
0x141421480  jne      0x14142150e
0x141421486  mov      rax, qword ptr [rcx]
0x141421489  cmp      rax, qword ptr [rip + 0x18c0b98]
0x141421490  jne      0x14142150e
0x141421492  mov      rcx, qword ptr [rbp + 0x30]
0x141421496  mov      rax, rcx
0x141421499  neg      rax
0x14142149c  jno      0x141421d39
0x1414214a2  mov      r9, qword ptr [rdi + 0x20]
0x1414214a6  mov      rax, qword ptr [rdi + 0x28]
0x1414214aa  cmp      rax, r9
0x1414214ad  jae      0x141421aa0
0x1414214b3  mov      rcx, rsi
0x1414214b6  mov      rdx, qword ptr [r14]
0x1414214b9  inc      rax
0x1414214bc  neg      r9
0x1414214bf  movzx    r8d, byte ptr [rdx + rax - 1]
0x1414214c5  cmp      r8, 0x3a
0x1414214c9  ja       0x141421d5b
0x1414214cf  bt       r15, r8
0x1414214d3  jae      0x1414215ac
0x1414214d9  mov      qword ptr [rdi + 0x28], rax
0x1414214dd  lea      r8, [r9 + rax]
0x1414214e1  inc      r8
0x1414214e4  inc      rax
0x1414214e7  cmp      r8, 1
0x1414214eb  jne      0x1414214bf
