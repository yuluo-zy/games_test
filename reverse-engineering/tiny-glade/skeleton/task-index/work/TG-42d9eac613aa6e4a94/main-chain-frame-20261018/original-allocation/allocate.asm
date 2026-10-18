140c02740: 55                               push     rbp
140c02741: 4157                             push     r15
140c02743: 4156                             push     r14
140c02745: 56                               push     rsi
140c02746: 57                               push     rdi
140c02747: 53                               push     rbx
140c02748: 4883ec68                         sub      rsp, 0x68
140c0274c: 488d6c2460                       lea      rbp, [rsp + 0x60]
140c02751: 48c74500feffffff                 mov      qword ptr [rbp], 0xfffffffffffffffe
140c02759: 4981f901000400                   cmp      r9, 0x40001
140c02760: 0f8399010000                     jae      0x140c028ff
140c02766: 4c89cf                           mov      rdi, r9
140c02769: 4889d6                           mov      rsi, rdx
140c0276c: 4889cb                           mov      rbx, rcx
140c0276f: 488b4d68                         mov      rcx, qword ptr [rbp + 0x68]
140c02773: 488b4560                         mov      rax, qword ptr [rbp + 0x60]
140c02777: 4883f911                         cmp      rcx, 0x11
140c0277b: ba10000000                       mov      edx, 0x10
140c02780: 480f42ca                         cmovb    rcx, rdx
140c02784: a801                             test     al, 1
140c02786: 480f44ca                         cmove    rcx, rdx
140c0278a: 488b4620                         mov      rax, qword ptr [rsi + 0x20]
140c0278e: 4839c8                           cmp      rax, rcx
140c02791: 480f47c8                         cmova    rcx, rax
140c02795: 488b4610                         mov      rax, qword ptr [rsi + 0x10]
140c02799: 4801c8                           add      rax, rcx
140c0279c: 48ffc8                           dec      rax
140c0279f: 48f7d9                           neg      rcx
140c027a2: 4821c1                           and      rcx, rax
140c027a5: 48894e10                         mov      qword ptr [rsi + 0x10], rcx
140c027a9: 4a8d1409                         lea      rdx, [rcx + r9]
140c027ad: 4881fa00000400                   cmp      rdx, 0x40000
140c027b4: 0f86c9000000                     jbe      0x140c02883
140c027ba: 4d89c6                           mov      r14, r8
140c027bd: 488d4dc8                         lea      rcx, [rbp - 0x38]
140c027c1: 4c89c2                           mov      rdx, r8
140c027c4: e827020000                       call     0x140c029f0
140c027c9: 488d4e28                         lea      rcx, [rsi + 0x28]
140c027cd: 48894df8                         mov      qword ptr [rbp - 8], rcx
140c027d1: e8dafcfdff                       call     0x140be24b0
140c027d6: 488b45e8                         mov      rax, qword ptr [rbp - 0x18]
140c027da: 4c8b7df8                         mov      r15, qword ptr [rbp - 8]
140c027de: 49894720                         mov      qword ptr [r15 + 0x20], rax
140c027e2: 0f1045c8                         movups   xmm0, xmmword ptr [rbp - 0x38]
140c027e6: 0f104dd8                         movups   xmm1, xmmword ptr [rbp - 0x28]
140c027ea: 410f114f10                       movups   xmmword ptr [r15 + 0x10], xmm1
140c027ef: 410f1107                         movups   xmmword ptr [r15], xmm0
140c027f3: 48c7461000000000                 mov      qword ptr [rsi + 0x10], 0
140c027fb: 4d8b8648060000                   mov      r8, qword ptr [r14 + 0x648]
140c02802: 0fb7463c                         movzx    eax, word ptr [rsi + 0x3c]
140c02806: 66413b80500a0000                 cmp      ax, word ptr [r8 + 0xa50]
140c0280e: 0f8521010000                     jne      0x140c02935
140c02814: 807e3e01                         cmp      byte ptr [rsi + 0x3e], 1
140c02818: 0f8544010000                     jne      0x140c02962
140c0281e: 488b4630                         mov      rax, qword ptr [rsi + 0x30]
140c02822: 8b480c                           mov      ecx, dword ptr [rax + 0xc]
140c02825: 3b4e38                           cmp      ecx, dword ptr [rsi + 0x38]
140c02828: 0f8561010000                     jne      0x140c0298f
140c0282e: 488b4030                         mov      rax, qword ptr [rax + 0x30]
140c02832: c745c821839e3b                   mov      dword ptr [rbp - 0x38], 0x3b9e8321
140c02839: 48c745d000000000                 mov      qword ptr [rbp - 0x30], 0
140c02841: 488945d8                         mov      qword ptr [rbp - 0x28], rax
140c02845: 498b8e40060000                   mov      rcx, qword ptr [r14 + 0x640]
140c0284c: 488d55c8                         lea      rdx, [rbp - 0x38]
140c02850: 41ff9608050000                   call     qword ptr [r14 + 0x508]
140c02857: 48894618                         mov      qword ptr [rsi + 0x18], rax
140c0285b: 4c89f9                           mov      rcx, r15
140c0285e: 4c89f2                           mov      rdx, r14
140c02861: e86a24fdff                       call     0x140bd4cd0
140c02866: 4885c0                           test     rax, rax
140c02869: 0f842c010000                     je       0x140c0299b
140c0286f: 4989d1                           mov      r9, rdx
140c02872: 488906                           mov      qword ptr [rsi], rax
140c02875: 48895608                         mov      qword ptr [rsi + 8], rdx
140c02879: 488b4e10                         mov      rcx, qword ptr [rsi + 0x10]
140c0287d: 488d1439                         lea      rdx, [rcx + rdi]
140c02881: eb07                             jmp      0x140c0288a
140c02883: 488b06                           mov      rax, qword ptr [rsi]
140c02886: 4c8b4e08                         mov      r9, qword ptr [rsi + 8]
140c0288a: 4839ca                           cmp      rdx, rcx
140c0288d: 0f8284000000                     jb       0x140c02917
140c02893: 4c39ca                           cmp      rdx, r9
140c02896: 0f8787000000                     ja       0x140c02923
140c0289c: 48895610                         mov      qword ptr [rsi + 0x10], rdx
140c028a0: 488b5630                         mov      rdx, qword ptr [rsi + 0x30]
140c028a4: f0ff4208                         lock inc dword ptr [rdx + 8]
140c028a8: 4801c8                           add      rax, rcx
140c028ab: 448b4638                         mov      r8d, dword ptr [rsi + 0x38]
140c028af: 440fb64e3e                       movzx    r9d, byte ptr [rsi + 0x3e]
140c028b4: 440fb7563c                       movzx    r10d, word ptr [rsi + 0x3c]
140c028b9: 4c8b5e28                         mov      r11, qword ptr [rsi + 0x28]
140c028bd: 4c895b20                         mov      qword ptr [rbx + 0x20], r11
140c028c1: 48895328                         mov      qword ptr [rbx + 0x28], rdx
140c028c5: 44894330                         mov      dword ptr [rbx + 0x30], r8d
140c028c9: 6644895334                       mov      word ptr [rbx + 0x34], r10w
140c028ce: 44884b36                         mov      byte ptr [rbx + 0x36], r9b
140c028d2: c6433800                         mov      byte ptr [rbx + 0x38], 0
140c028d6: 488b5618                         mov      rdx, qword ptr [rsi + 0x18]
140c028da: 4801ca                           add      rdx, rcx
140c028dd: 488903                           mov      qword ptr [rbx], rax
140c028e0: 48897b08                         mov      qword ptr [rbx + 8], rdi
140c028e4: 48895310                         mov      qword ptr [rbx + 0x10], rdx
140c028e8: 894b40                           mov      dword ptr [rbx + 0x40], ecx
140c028eb: 48897b18                         mov      qword ptr [rbx + 0x18], rdi
140c028ef: 4889d8                           mov      rax, rbx
140c028f2: 4883c468                         add      rsp, 0x68
140c028f6: 5b                               pop      rbx
140c028f7: 5f                               pop      rdi
140c028f8: 5e                               pop      rsi
140c028f9: 415e                             pop      r14
140c028fb: 415f                             pop      r15
140c028fd: 5d                               pop      rbp
140c028fe: c3                               ret      
140c028ff: 488d0d5a56f101                   lea      rcx, [rip + 0x1f1565a]
140c02906: 4c8d059356f101                   lea      r8, [rip + 0x1f15693]
140c0290d: ba3d000000                       mov      edx, 0x3d
140c02912: e8196bcd01                       call     0x1428d9430
140c02917: 4c8d05b256f101                   lea      r8, [rip + 0x1f156b2]
140c0291e: e82d6fcd01                       call     0x1428d9850
140c02923: 4c8d05a656f101                   lea      r8, [rip + 0x1f156a6]
140c0292a: 4889d1                           mov      rcx, rdx
140c0292d: 4c89ca                           mov      rdx, r9
140c02930: e80b6fcd01                       call     0x1428d9840
140c02935: 48c745c800000000                 mov      qword ptr [rbp - 0x38], 0
140c0293d: 4883c63c                         add      rsi, 0x3c
140c02941: 4981c0500a0000                   add      r8, 0xa50
140c02948: 488d0551f90f02                   lea      rax, [rip + 0x20ff951]
140c0294f: 4889442420                       mov      qword ptr [rsp + 0x20], rax
140c02954: 4c8d4dc8                         lea      r9, [rbp - 0x38]
140c02958: 31c9                             xor      ecx, ecx
140c0295a: 4889f2                           mov      rdx, rsi
140c0295d: e8b47ecc01                       call     0x1428ca816
140c02962: 48c745c800000000                 mov      qword ptr [rbp - 0x38], 0
140c0296a: 4883c63e                         add      rsi, 0x3e
140c0296e: 488d054bf90f02                   lea      rax, [rip + 0x20ff94b]
140c02975: 4889442420                       mov      qword ptr [rsp + 0x20], rax
140c0297a: 4c8d0558f90f02                   lea      r8, [rip + 0x20ff958]
140c02981: 4c8d4dc8                         lea      r9, [rbp - 0x38]
140c02985: 31c9                             xor      ecx, ecx
140c02987: 4889f2                           mov      rdx, rsi
140c0298a: e8737fcc01                       call     0x1428ca902
140c0298f: 488d0d42cef001                   lea      rcx, [rip + 0x1f0ce42]
140c02996: e87569cd01                       call     0x1428d9310
140c0299b: 488d0d1656f101                   lea      rcx, [rip + 0x1f15616]
140c029a2: e86969cd01                       call     0x1428d9310
140c029a7: cc                               int3     
140c029a8: 0f1f840000000000                 nop      dword ptr [rax + rax]
140c029b0: 4889542410                       mov      qword ptr [rsp + 0x10], rdx
140c029b5: 55                               push     rbp
140c029b6: 4157                             push     r15
140c029b8: 4156                             push     r14
140c029ba: 56                               push     rsi
140c029bb: 57                               push     rdi
140c029bc: 53                               push     rbx
140c029bd: 4883ec28                         sub      rsp, 0x28
140c029c1: 488d6a60                         lea      rbp, [rdx + 0x60]
140c029c5: 488b45e8                         mov      rax, qword ptr [rbp - 0x18]
140c029c9: 488b4df8                         mov      rcx, qword ptr [rbp - 8]
140c029cd: 48894120                         mov      qword ptr [rcx + 0x20], rax
140c029d1: 0f1045c8                         movups   xmm0, xmmword ptr [rbp - 0x38]
140c029d5: 0f104dd8                         movups   xmm1, xmmword ptr [rbp - 0x28]
140c029d9: 0f114910                         movups   xmmword ptr [rcx + 0x10], xmm1
140c029dd: 0f1101                           movups   xmmword ptr [rcx], xmm0
140c029e0: 4883c428                         add      rsp, 0x28
140c029e4: 5b                               pop      rbx
140c029e5: 5f                               pop      rdi
140c029e6: 5e                               pop      rsi
140c029e7: 415e                             pop      r14
140c029e9: 415f                             pop      r15
140c029eb: 5d                               pop      rbp
140c029ec: c3                               ret      
