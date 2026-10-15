0x1400b7da0 push     rbp
0x1400b7da1 push     r15
0x1400b7da3 push     r14
0x1400b7da5 push     r12
0x1400b7da7 push     rsi
0x1400b7da8 push     rdi
0x1400b7da9 push     rbx
0x1400b7daa sub      rsp, 0x2c0
0x1400b7db1 lea      rbp, [rsp + 0x80]
0x1400b7db9 mov      qword ptr [rbp + 0x238], 0xfffffffffffffffe
0x1400b7dc4 mov      rsi, rdx
0x1400b7dc7 mov      rdi, qword ptr [rcx]
0x1400b7dca mov      rbx, qword ptr [rcx + 8]
0x1400b7dce mov      r15d, dword ptr [rdi + 0x1c]
0x1400b7dd2 mov      r12, qword ptr [rdi + 0x10]
0x1400b7dd6 mov      dword ptr [r12], r15d
0x1400b7dda mov      r14, qword ptr [rdi]
0x1400b7ddd movzx    eax, byte ptr [rip + 0x357ce25]
0x1400b7de4 mov      ecx, 0x1f
0x1400b7de9 mov      edx, 1
0x1400b7dee call     0x140613c10
0x1400b7df3 test     rax, rax
0x1400b7df6 je       0x1400b82ae
0x1400b7dfc lea      rcx, [r14 + 0xf9]
0x1400b7e03 movups   xmm0, xmmword ptr [rip + 0x287d52e] ; @0x142935338 ' hover highlightDisable wall snappingDisable eraser higlightHide brush previewsInput ReplayShow '
0x1400b7e0a movups   xmmword ptr [rax + 0xf], xmm0
0x1400b7e0e movups   xmm0, xmmword ptr [rip + 0x287d514] ; @0x142935329 'Disable terrain hover highlightDisable wall snappingDisable eraser higlightHide brush previewsIn'
0x1400b7e15 movups   xmmword ptr [rax], xmm0
0x1400b7e18 mov      qword ptr [rbp + 0x130], 0x1f
0x1400b7e23 mov      qword ptr [rbp + 0x138], rax
0x1400b7e2a mov      qword ptr [rbp + 0x140], 0x1f
0x1400b7e35 mov      dword ptr [rbp + 0x148], 0
0x1400b7e3f mov      qword ptr [rbp + 0x158], 0
0x1400b7e4a mov      dword ptr [rbp + 0x150], 0
0x1400b7e54 mov      qword ptr [rbp + 0x100], 3
0x1400b7e5f mov      qword ptr [rbp + 0x118], 6
0x1400b7e6a mov      byte ptr [rbp + 0x160], 0
0x1400b7e71 mov      dword ptr [rbp + 0x165], 0
0x1400b7e7b mov      dword ptr [rbp + 0x168], 0
0x1400b7e85 mov      qword ptr [rbp + 0x170], rcx
0x1400b7e8c mov      byte ptr [rbp + 0x178], 0
0x1400b7e93 lea      rcx, [rbp + 0x38]
0x1400b7e97 lea      rdx, [rbp + 0x100]
0x1400b7e9e mov      r8, rsi
0x1400b7ea1 call     0x140c48930
0x1400b7ea6 mov      rax, qword ptr [rbp + 0x48]
0x1400b7eaa lock dec qword ptr [rax]
0x1400b7eae jne      0x1400b7eb9
0x1400b7eb0 lea      rcx, [rbp + 0x48]
0x1400b7eb4 call     0x140c41170
0x1400b7eb9 mov      dword ptr [r12], r15d
0x1400b7ebd movzx    eax, byte ptr [rip + 0x357cd45]
0x1400b7ec4 mov      ecx, 0x15
0x1400b7ec9 mov      edx, 1
0x1400b7ece call     0x140613c10
0x1400b7ed3 test     rax, rax
0x1400b7ed6 je       0x1400b82c4
0x1400b7edc lea      rcx, [r14 + 0xfd]
0x1400b7ee3 movups   xmm0, xmmword ptr [rip + 0x287d45e] ; @0x142935348 'Disable wall snappingDisable eraser higlightHide brush previewsInput ReplayShow sheep controlsBi'
0x1400b7eea movups   xmmword ptr [rax], xmm0
0x1400b7eed movabs   rdx, 0x676e697070616e73
0x1400b7ef7 mov      qword ptr [rax + 0xd], rdx
0x1400b7efb mov      qword ptr [rbp + 0x130], 0x15
0x1400b7f06 mov      qword ptr [rbp + 0x138], rax
0x1400b7f0d mov      qword ptr [rbp + 0x140], 0x15
0x1400b7f18 mov      dword ptr [rbp + 0x148], 0
0x1400b7f22 mov      qword ptr [rbp + 0x158], 0
0x1400b7f2d mov      dword ptr [rbp + 0x150], 0
0x1400b7f37 mov      qword ptr [rbp + 0x100], 3
0x1400b7f42 mov      qword ptr [rbp + 0x118], 6
0x1400b7f4d mov      byte ptr [rbp + 0x160], 0
0x1400b7f54 mov      dword ptr [rbp + 0x165], 0
0x1400b7f5e mov      dword ptr [rbp + 0x168], 0
0x1400b7f68 mov      qword ptr [rbp + 0x170], rcx
0x1400b7f6f mov      byte ptr [rbp + 0x178], 0
0x1400b7f76 lea      rcx, [rbp + 0x38]
0x1400b7f7a lea      rdx, [rbp + 0x100]
0x1400b7f81 mov      r8, rsi
0x1400b7f84 call     0x140c48930
0x1400b7f89 mov      rax, qword ptr [rbp + 0x48]
0x1400b7f8d lock dec qword ptr [rax]
0x1400b7f91 jne      0x1400b7f9c
0x1400b7f93 lea      rcx, [rbp + 0x48]
0x1400b7f97 call     0x140c41170
0x1400b7f9c mov      dword ptr [r12], r15d
0x1400b7fa0 movzx    eax, byte ptr [rip + 0x357cc62]
0x1400b7fa7 mov      ecx, 0x17
0x1400b7fac mov      edx, 1
0x1400b7fb1 call     0x140613c10
0x1400b7fb6 test     rax, rax
0x1400b7fb9 je       0x1400b82da
0x1400b7fbf lea      rcx, [r14 + 0xfe]
0x1400b7fc6 movups   xmm0, xmmword ptr [rip + 0x287d390] ; @0x14293535d 'Disable eraser higlightHide brush previewsInput ReplayShow sheep controlsBirdsProxiesFinish shad'
0x1400b7fcd movups   xmmword ptr [rax], xmm0
0x1400b7fd0 movabs   rdx, 0x746867696c676968
0x1400b7fda mov      qword ptr [rax + 0xf], rdx
0x1400b7fde mov      qword ptr [rbp + 0x130], 0x17
0x1400b7fe9 mov      qword ptr [rbp + 0x138], rax
0x1400b7ff0 mov      qword ptr [rbp + 0x140], 0x17
0x1400b7ffb mov      dword ptr [rbp + 0x148], 0
0x1400b8005 mov      qword ptr [rbp + 0x158], 0
0x1400b8010 mov      dword ptr [rbp + 0x150], 0
0x1400b801a mov      qword ptr [rbp + 0x100], 3
0x1400b8025 mov      qword ptr [rbp + 0x118], 6
0x1400b8030 mov      byte ptr [rbp + 0x160], 0
0x1400b8037 mov      dword ptr [rbp + 0x165], 0
0x1400b8041 mov      dword ptr [rbp + 0x168], 0
0x1400b804b mov      qword ptr [rbp + 0x170], rcx
0x1400b8052 mov      byte ptr [rbp + 0x178], 0
0x1400b8059 lea      rcx, [rbp + 0x38]
0x1400b805d lea      rdx, [rbp + 0x100]
0x1400b8064 mov      r8, rsi
0x1400b8067 call     0x140c48930
0x1400b806c mov      rax, qword ptr [rbp + 0x48]
0x1400b8070 lock dec qword ptr [rax]
0x1400b8074 jne      0x1400b807f
0x1400b8076 lea      rcx, [rbp + 0x48]
0x1400b807a call     0x140c41170
0x1400b807f mov      dword ptr [r12], r15d
0x1400b8083 movzx    eax, byte ptr [rip + 0x357cb7f]
0x1400b808a mov      ecx, 0x13
0x1400b808f mov      edx, 1
0x1400b8094 call     0x140613c10
0x1400b8099 test     rax, rax
0x1400b809c je       0x1400b82f0
0x1400b80a2 add      r14, 0xfa
0x1400b80a9 movups   xmm0, xmmword ptr [rip + 0x287d2c4] ; @0x142935374 'Hide brush previewsInput ReplayShow sheep controlsBirdsProxiesFinish shader refactoringhttps://g'
0x1400b80b0 movups   xmmword ptr [rax], xmm0
0x1400b80b3 mov      dword ptr [rax + 0xf], 0x73776569
0x1400b80ba mov      qword ptr [rbp + 0x130], 0x13
0x1400b80c5 mov      qword ptr [rbp + 0x138], rax
0x1400b80cc mov      qword ptr [rbp + 0x140], 0x13
0x1400b80d7 mov      dword ptr [rbp + 0x148], 0
0x1400b80e1 mov      qword ptr [rbp + 0x158], 0
0x1400b80ec mov      dword ptr [rbp + 0x150], 0
0x1400b80f6 mov      qword ptr [rbp + 0x100], 3
0x1400b8101 mov      qword ptr [rbp + 0x118], 6
0x1400b810c mov      byte ptr [rbp + 0x160], 0
0x1400b8113 mov      dword ptr [rbp + 0x165], 0
0x1400b811d mov      dword ptr [rbp + 0x168], 0
0x1400b8127 mov      qword ptr [rbp + 0x170], r14
0x1400b812e mov      byte ptr [rbp + 0x178], 0
0x1400b8135 lea      rcx, [rbp + 0x38]
0x1400b8139 lea      rdx, [rbp + 0x100]
0x1400b8140 mov      r8, rsi
0x1400b8143 call     0x140c48930
0x1400b8148 mov      rax, qword ptr [rbp + 0x48]
0x1400b814c lock dec qword ptr [rax]
0x1400b8150 jne      0x1400b815b
0x1400b8152 lea      rcx, [rbp + 0x48]
0x1400b8156 call     0x140c41170
0x1400b815b lea      rdx, [rip + 0x287d225] ; @0x142935387 'Input ReplayShow sheep controlsBirdsProxiesFinish shader refactoringhttps://github.com/anopara/c'
0x1400b8162 lea      rcx, [rbp - 0x58]
0x1400b8166 mov      r8d, 0xc
0x1400b816c call     0x14015eea0
0x1400b8171 movzx    eax, byte ptr [rip + 0x357ca91]
0x1400b8178 mov      ecx, 0x10
0x1400b817d mov      edx, 8
0x1400b8182 call     0x140613c10
0x1400b8187 test     rax, rax
0x1400b818a mov      qword ptr [rbp + 0x228], rax
0x1400b8191 je       0x1400b8306
0x1400b8197 mov      r14, rax
0x1400b819a mov      qword ptr [rax], rdi
0x1400b819d mov      qword ptr [rax + 8], rbx
0x1400b81a1 mov      byte ptr [rbp + 0x237], 1
0x1400b81a8 lea      rcx, [rbp + 0x38]
0x1400b81ac lea      rdx, [rbp - 0x58]
0x1400b81b0 mov      r8d, 0x90
0x1400b81b6 call     0x142923c00
0x1400b81bb lea      rax, [rbp + 0x237]
0x1400b81c2 mov      qword ptr [rbp + 0xc8], rax
0x1400b81c9 mov      qword ptr [rbp + 0xd0], r14
0x1400b81d0 lea      rax, [rip + 0x287c619]
0x1400b81d7 mov      qword ptr [rbp + 0xd8], rax
0x1400b81de movzx    eax, byte ptr [rip + 0x357ca24]
0x1400b81e5 mov      ecx, 0xa8
0x1400b81ea mov      edx, 8
0x1400b81ef call     0x140613c10
0x1400b81f4 test     rax, rax
0x1400b81f7 je       0x1400b8317
0x1400b81fd mov      rdi, rax
0x1400b8200 lea      rdx, [rbp + 0x38]
0x1400b8204 mov      r8d, 0xa8
0x1400b820a mov      rcx, rax
0x1400b820d call     0x142923c00
0x1400b8212 lea      rax, [rip + 0x28a45e7]
0x1400b8219 mov      qword ptr [rsp + 0x20], rax
0x1400b821e lea      rcx, [rbp + 0x100]
0x1400b8225 movabs   r8, 0x20001000000
0x1400b822f mov      rdx, rsi
0x1400b8232 mov      r9, rdi
0x1400b8235 call     0x1400c2f70
0x1400b823a lea      rcx, [rbp + 0x38]
0x1400b823e lea      rdx, [rbp + 0x100]
0x1400b8245 mov      r8d, 0xc8
0x1400b824b call     0x142923c00
0x1400b8250 mov      rax, qword ptr [rbp + 0x1d8]
0x1400b8257 lock dec qword ptr [rax]
0x1400b825b jne      0x1400b8269
0x1400b825d lea      rcx, [rbp + 0x1d8]
0x1400b8264 call     0x140c41170
0x1400b8269 mov      rax, qword ptr [rbp + 0xa8]
0x1400b8270 lock dec qword ptr [rax]
0x1400b8274 jne      0x1400b8282
0x1400b8276 lea      rcx, [rbp + 0xa8]
0x1400b827d call     0x140c41170
0x1400b8282 cmp      dword ptr [rbp + 0x78], 2
0x1400b8286 je       0x1400b829b
0x1400b8288 mov      rax, qword ptr [rbp + 0x48]
0x1400b828c lock dec qword ptr [rax]
0x1400b8290 jne      0x1400b829b
0x1400b8292 lea      rcx, [rbp + 0x48]
0x1400b8296 call     0x140c41170
0x1400b829b nop      
0x1400b829c add      rsp, 0x2c0
0x1400b82a3 pop      rbx
0x1400b82a4 pop      rdi
0x1400b82a5 pop      rsi
0x1400b82a6 pop      r12
0x1400b82a8 pop      r14
0x1400b82aa pop      r15
0x1400b82ac pop      rbp
0x1400b82ad ret      
0x1400b82ae lea      r8, [rip + 0x287ca93]
0x1400b82b5 mov      ecx, 1
0x1400b82ba mov      edx, 0x1f
0x1400b82bf call     0x1428d8fe3
0x1400b82c4 lea      r8, [rip + 0x287ca7d]
0x1400b82cb mov      ecx, 1
0x1400b82d0 mov      edx, 0x15
0x1400b82d5 call     0x1428d8fe3
0x1400b82da lea      r8, [rip + 0x287ca67]
0x1400b82e1 mov      ecx, 1
0x1400b82e6 mov      edx, 0x17
0x1400b82eb call     0x1428d8fe3
0x1400b82f0 lea      r8, [rip + 0x287ca51]
0x1400b82f7 mov      ecx, 1
0x1400b82fc mov      edx, 0x13
0x1400b8301 call     0x1428d8fe3
0x1400b8306 mov      ecx, 8
0x1400b830b mov      edx, 0x10
0x1400b8310 call     0x1428d9000
0x1400b8315 jmp      0x1400b8326
0x1400b8317 mov      ecx, 8
0x1400b831c mov      edx, 0xa8