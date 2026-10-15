0x1400bf170 push     rbp
0x1400bf171 push     r15
0x1400bf173 push     r14
0x1400bf175 push     r12
0x1400bf177 push     rsi
0x1400bf178 push     rdi
0x1400bf179 push     rbx
0x1400bf17a sub      rsp, 0x350
0x1400bf181 lea      rbp, [rsp + 0x80]
0x1400bf189 mov      qword ptr [rbp + 0x2c8], 0xfffffffffffffffe
0x1400bf194 mov      rsi, rdx
0x1400bf197 mov      rbx, rcx
0x1400bf19a mov      rax, qword ptr [rcx]
0x1400bf19d mov      ecx, dword ptr [rax + 0x1c]
0x1400bf1a0 mov      rdx, qword ptr [rax + 0x10]
0x1400bf1a4 mov      dword ptr [rdx], ecx
0x1400bf1a6 mov      rcx, qword ptr [rax]
0x1400bf1a9 call     0x1408aea50
0x1400bf1ae mov      rdi, rax
0x1400bf1b1 mov      rax, qword ptr [rbx + 8]
0x1400bf1b5 mov      r14, qword ptr [rax]
0x1400bf1b8 xor      eax, eax
0x1400bf1ba cmp      rax, qword ptr [r14]
0x1400bf1bd jno      0x1400bf22e
0x1400bf1bf mov      byte ptr [rbp + 0x18], 0
0x1400bf1c3 movzx    eax, byte ptr [rip + 0x3575a3f]
0x1400bf1ca mov      ecx, 8
0x1400bf1cf mov      edx, 8
0x1400bf1d4 call     0x140613c10
0x1400bf1d9 test     rax, rax
0x1400bf1dc je       0x1400bfbf4
0x1400bf1e2 lea      rcx, [rbp + 0x18]
0x1400bf1e6 mov      qword ptr [rax], rcx
0x1400bf1e9 movabs   rcx, 0x22febc6edc6d0ab6
0x1400bf1f3 mov      qword ptr [rsp + 0x20], rcx
0x1400bf1f8 lea      r9, [rip + 0x2875a79]
0x1400bf1ff lea      rcx, [rbp + 0x160]
0x1400bf206 mov      rdx, rsi
0x1400bf209 mov      r8, rax
0x1400bf20c call     0x1400c4680
0x1400bf211 mov      rax, qword ptr [rbp + 0x170]
0x1400bf218 lock dec qword ptr [rax]
0x1400bf21c jne      0x1400bf3fe
0x1400bf222 lea      rcx, [rbp + 0x170]
0x1400bf229 jmp      0x1400bf3f9
0x1400bf22e mov      r15, qword ptr [rbx + 0x10]
0x1400bf232 mov      eax, dword ptr [r15 + 0x1c]
0x1400bf236 mov      rcx, qword ptr [r15 + 0x10]
0x1400bf23a mov      dword ptr [rcx], eax
0x1400bf23c mov      r12, qword ptr [r15]
0x1400bf23f movzx    eax, byte ptr [rip + 0x35759c3]
0x1400bf246 mov      ecx, 0x14
0x1400bf24b mov      edx, 1
0x1400bf250 call     0x140613c10
0x1400bf255 test     rax, rax
0x1400bf258 je       0x1400bfc03
0x1400bf25e add      r12, 0xc1
0x1400bf265 movups   xmm0, xmmword ptr [rip + 0x2876061] ; @0x1429352cd 'Use color managementICC profile: '
0x1400bf26c movups   xmmword ptr [rax], xmm0
0x1400bf26f mov      dword ptr [rax + 0x10], 0x746e656d
0x1400bf276 mov      qword ptr [rbp + 0x190], 0x14
0x1400bf281 mov      qword ptr [rbp + 0x198], rax
0x1400bf288 mov      qword ptr [rbp + 0x1a0], 0x14
0x1400bf293 mov      dword ptr [rbp + 0x1a8], 0
0x1400bf29d mov      qword ptr [rbp + 0x1b8], 0
0x1400bf2a8 mov      dword ptr [rbp + 0x1b0], 0
0x1400bf2b2 mov      qword ptr [rbp + 0x160], 3
0x1400bf2bd mov      qword ptr [rbp + 0x178], 6
0x1400bf2c8 mov      byte ptr [rbp + 0x1c0], 0
0x1400bf2cf mov      dword ptr [rbp + 0x1c5], 0
0x1400bf2d9 mov      dword ptr [rbp + 0x1c8], 0
0x1400bf2e3 mov      qword ptr [rbp + 0x1d0], r12
0x1400bf2ea mov      byte ptr [rbp + 0x1d8], 0
0x1400bf2f1 lea      rcx, [rbp + 0x18]
0x1400bf2f5 lea      rdx, [rbp + 0x160]
0x1400bf2fc mov      r8, rsi
0x1400bf2ff call     0x140c48930
0x1400bf304 mov      rax, qword ptr [rbp + 0x28]
0x1400bf308 lock dec qword ptr [rax]
0x1400bf30c jne      0x1400bf317
0x1400bf30e lea      rcx, [rbp + 0x28]
0x1400bf312 call     0x140c41170
0x1400bf317 mov      rax, qword ptr [r15]
0x1400bf31a cmp      byte ptr [rax + 0xc1], 0
0x1400bf321 je       0x1400bf3fe
0x1400bf327 mov      qword ptr [rbp - 0x58], r14
0x1400bf32b lea      rax, [rip - 0x83862]
0x1400bf332 mov      qword ptr [rbp - 0x50], rax
0x1400bf336 lea      rax, [rip + 0x2875fb3]
0x1400bf33d mov      qword ptr [rbp + 0x18], rax
0x1400bf341 mov      qword ptr [rbp + 0x20], 1
0x1400bf349 lea      rax, [rbp - 0x58]
0x1400bf34d mov      qword ptr [rbp + 0x28], rax
0x1400bf351 mov      qword ptr [rbp + 0x30], 1
0x1400bf359 mov      qword ptr [rbp + 0x38], 0
0x1400bf361 lea      rcx, [rbp + 0x190]
0x1400bf368 lea      rdx, [rbp + 0x18]
0x1400bf36c call     0x140d79f90
0x1400bf371 mov      byte ptr [rbp + 0x1c0], 0
0x1400bf378 mov      dword ptr [rbp + 0x1c5], 0
0x1400bf382 mov      dword ptr [rbp + 0x1c8], 0
0x1400bf38c mov      qword ptr [rbp + 0x160], 3
0x1400bf397 mov      qword ptr [rbp + 0x178], 6
0x1400bf3a2 mov      dword ptr [rbp + 0x1a8], 0
0x1400bf3ac mov      dword ptr [rbp + 0x1b0], 0
0x1400bf3b6 mov      qword ptr [rbp + 0x1b8], 0
0x1400bf3c1 mov      word ptr [rbp + 0x1d0], 0x200
0x1400bf3ca mov      byte ptr [rbp + 0x1d2], 2
0x1400bf3d1 mov      byte ptr [rbp + 0x1d5], 2
0x1400bf3d8 lea      rcx, [rbp + 0x18]
0x1400bf3dc lea      rdx, [rbp + 0x160]
0x1400bf3e3 mov      r8, rsi
0x1400bf3e6 call     0x140c28d40
0x1400bf3eb mov      rax, qword ptr [rbp + 0x28]
0x1400bf3ef lock dec qword ptr [rax]
0x1400bf3f3 jne      0x1400bf3fe
0x1400bf3f5 lea      rcx, [rbp + 0x28]
0x1400bf3f9 call     0x140c41170
0x1400bf3fe movss    xmm0, dword ptr [rip + 0x287528a]
0x1400bf406 movlps   qword ptr [rbp + 0x18], xmm0
0x1400bf40a mov      byte ptr [rbp + 0x20], 2
0x1400bf40e lea      rcx, [rbp + 0x160]
0x1400bf415 lea      rdx, [rbp + 0x18]
0x1400bf419 mov      r8, rsi
0x1400bf41c call     0x140c29310
0x1400bf421 mov      rax, qword ptr [rbp + 0x170]
0x1400bf428 lock dec qword ptr [rax]
0x1400bf42c jne      0x1400bf43a
0x1400bf42e lea      rcx, [rbp + 0x170]
0x1400bf435 call     0x140c41170
0x1400bf43a movzx    eax, byte ptr [rip + 0x35757c8]
0x1400bf441 mov      ecx, 0x14
0x1400bf446 mov      edx, 1
0x1400bf44b call     0x140613c10
0x1400bf450 test     rax, rax
0x1400bf453 je       0x1400bfc03
0x1400bf459 lea      rcx, [rdi + 0x28]
0x1400bf45d movups   xmm0, xmmword ptr [rip + 0x2875e9c] ; @0x142935300 'Bypass color gradingLUTResetModeSnapshotsDisable terrain hover highlightDisable wall snappingDis'
0x1400bf464 movups   xmmword ptr [rax], xmm0
0x1400bf467 mov      dword ptr [rax + 0x10], 0x676e6964
0x1400bf46e mov      qword ptr [rbp + 0x190], 0x14
0x1400bf479 mov      qword ptr [rbp + 0x198], rax
0x1400bf480 mov      qword ptr [rbp + 0x1a0], 0x14
0x1400bf48b mov      dword ptr [rbp + 0x1a8], 0
0x1400bf495 mov      qword ptr [rbp + 0x1b8], 0
0x1400bf4a0 mov      dword ptr [rbp + 0x1b0], 0
0x1400bf4aa mov      qword ptr [rbp + 0x160], 3
0x1400bf4b5 mov      qword ptr [rbp + 0x178], 6
0x1400bf4c0 mov      byte ptr [rbp + 0x1c0], 0
0x1400bf4c7 mov      dword ptr [rbp + 0x1c5], 0
0x1400bf4d1 mov      dword ptr [rbp + 0x1c8], 0
0x1400bf4db mov      qword ptr [rbp + 0x1d0], rcx
0x1400bf4e2 mov      byte ptr [rbp + 0x1d8], 0
0x1400bf4e9 lea      rcx, [rbp + 0x18]
0x1400bf4ed lea      rdx, [rbp + 0x160]
0x1400bf4f4 mov      r8, rsi
0x1400bf4f7 call     0x140c48930
0x1400bf4fc mov      rax, qword ptr [rbp + 0x28]
0x1400bf500 lock dec qword ptr [rax]
0x1400bf504 jne      0x1400bf50f
0x1400bf506 lea      rcx, [rbp + 0x28]
0x1400bf50a call     0x140c41170
0x1400bf50f lea      rdx, [rip + 0x2875dfe] ; @0x142935314 'LUTResetModeSnapshotsDisable terrain hover highlightDisable wall snappingDisable eraser higlight'
0x1400bf516 lea      rcx, [rbp + 0x160]
0x1400bf51d mov      r8d, 3
0x1400bf523 call     0x1400ff0b0
0x1400bf528 lea      rcx, [rbp + 0x48]
0x1400bf52c mov      rdx, rdi
0x1400bf52f call     0x140d7bd90
0x1400bf534 mov      byte ptr [rbp + 0x78], 0
0x1400bf538 mov      dword ptr [rbp + 0x7d], 0
0x1400bf53f mov      dword ptr [rbp + 0x80], 0
0x1400bf549 mov      qword ptr [rbp + 0x18], 3
0x1400bf551 mov      qword ptr [rbp + 0x30], 6
0x1400bf559 mov      dword ptr [rbp + 0x60], 0
0x1400bf560 mov      dword ptr [rbp + 0x68], 0
0x1400bf567 mov      qword ptr [rbp + 0x70], 0
0x1400bf56f mov      r14, qword ptr [rbp + 0x160]
0x1400bf576 mov      ecx, r14d
0x1400bf579 and      ecx, 0xfffffffe
0x1400bf57c lea      rdx, [r14 - 3]
0x1400bf580 xor      eax, eax
0x1400bf582 cmp      ecx, 4
0x1400bf585 cmove    rax, rdx
0x1400bf589 test     rax, rax
0x1400bf58c je       0x1400bf5c7
0x1400bf58e cmp      rax, 1
0x1400bf592 jne      0x1400bf637
0x1400bf598 mov      rdx, qword ptr [rbp + 0x168]
0x1400bf59f test     rdx, rdx
0x1400bf5a2 je       0x1400bf5b6
0x1400bf5a4 mov      rcx, qword ptr [rbp + 0x170]
0x1400bf5ab mov      r8d, 1
0x1400bf5b1 call     0x140613c20
0x1400bf5b6 lea      rcx, [rbp + 0x180]
0x1400bf5bd call     0x140608f80
0x1400bf5c2 jmp      0x1400bf650
0x1400bf5c7 mov      rdx, qword ptr [rbp + 0x190]
0x1400bf5ce test     rdx, rdx
0x1400bf5d1 je       0x1400bf5e5
0x1400bf5d3 mov      rcx, qword ptr [rbp + 0x198]
0x1400bf5da mov      r8d, 1
0x1400bf5e0 call     0x140613c20
0x1400bf5e5 cmp      r14d, 3
0x1400bf5e9 je       0x1400bf60a
0x1400bf5eb cmp      r14d, 2
0x1400bf5ef jb       0x1400bf60a
0x1400bf5f1 mov      rax, qword ptr [rbp + 0x168]
0x1400bf5f8 lock dec qword ptr [rax]
0x1400bf5fc jne      0x1400bf60a
0x1400bf5fe lea      rcx, [rbp + 0x168]
0x1400bf605 call     0x14064eb30
0x1400bf60a mov      rax, qword ptr [rbp + 0x178]
0x1400bf611 cmp      rax, 6
0x1400bf615 je       0x1400bf650
0x1400bf617 cmp      eax, 5
0x1400bf61a jb       0x1400bf650
0x1400bf61c mov      rax, qword ptr [rbp + 0x180]
0x1400bf623 lock dec qword ptr [rax]
0x1400bf627 jne      0x1400bf650
0x1400bf629 lea      rcx, [rbp + 0x180]
0x1400bf630 call     0x14064eb30
0x1400bf635 jmp      0x1400bf650
0x1400bf637 mov      rax, qword ptr [rbp + 0x168]
0x1400bf63e lock dec qword ptr [rax]
0x1400bf642 jne      0x1400bf650
0x1400bf644 lea      rcx, [rbp + 0x168]
0x1400bf64b call     0x140c7de90
0x1400bf650 movups   xmm0, xmmword ptr [rbp + 0x78]
0x1400bf654 movaps   xmmword ptr [rbp + 0x1c0], xmm0
0x1400bf65b movups   xmm0, xmmword ptr [rbp + 0x68]
0x1400bf65f movaps   xmmword ptr [rbp + 0x1b0], xmm0
0x1400bf666 movups   xmm0, xmmword ptr [rbp + 0x58]
0x1400bf66a movaps   xmmword ptr [rbp + 0x1a0], xmm0
0x1400bf671 movups   xmm0, xmmword ptr [rbp + 0x18]
0x1400bf675 movups   xmm1, xmmword ptr [rbp + 0x28]
0x1400bf679 movups   xmm2, xmmword ptr [rbp + 0x38]
0x1400bf67d movups   xmm3, xmmword ptr [rbp + 0x48]
0x1400bf681 movaps   xmmword ptr [rbp + 0x190], xmm3
0x1400bf688 movaps   xmmword ptr [rbp + 0x180], xmm2
0x1400bf68f movaps   xmmword ptr [rbp + 0x170], xmm1
0x1400bf696 movaps   xmmword ptr [rbp + 0x160], xmm0
0x1400bf69d lea      r14, [rbp + 0x18]
0x1400bf6a1 lea      rdx, [rbp + 0x160]
0x1400bf6a8 mov      r8d, 0x110
0x1400bf6ae mov      rcx, r14
0x1400bf6b1 call     0x142923c00
0x1400bf6b6 mov      rbx, qword ptr [rbx + 0x18]
0x1400bf6ba lea      rcx, [rbp + 0x160]
0x1400bf6c1 mov      r8d, 0x110
0x1400bf6c7 mov      rdx, r14
0x1400bf6ca call     0x142923c00
0x1400bf6cf movzx    eax, byte ptr [rip + 0x3575533]
0x1400bf6d6 mov      ecx, 0x10
0x1400bf6db mov      edx, 8
0x1400bf6e0 call     0x140613c10
0x1400bf6e5 test     rax, rax
0x1400bf6e8 mov      qword ptr [rbp + 0x2c0], rax
0x1400bf6ef je       0x1400bfc19
0x1400bf6f5 mov      r9, rax
0x1400bf6f8 mov      qword ptr [rax], rbx
0x1400bf6fb mov      qword ptr [rax + 8], rdi
0x1400bf6ff lea      rax, [rip + 0x288d4f2]
0x1400bf706 mov      qword ptr [rsp + 0x20], rax
0x1400bf70b lea      rcx, [rbp - 0x58]
0x1400bf70f lea      rdx, [rbp + 0x18]
0x1400bf713 mov      r8, rsi
0x1400bf716 call     0x1400ff3e0
0x1400bf71b mov      rax, qword ptr [rbp - 0x48]
0x1400bf71f lock dec qword ptr [rax]
0x1400bf723 jne      0x1400bf72e
0x1400bf725 lea      rcx, [rbp - 0x48]
0x1400bf729 call     0x140c41170
0x1400bf72e movzx    eax, byte ptr [rip + 0x35754d4]
0x1400bf735 mov      ecx, 8
0x1400bf73a mov      edx, 8
0x1400bf73f call     0x140613c10
0x1400bf744 test     rax, rax
0x1400bf747 je       0x1400bfbf4
0x1400bf74d lea      rcx, [rdi + 0x18]
0x1400bf751 mov      qword ptr [rbp + 0x2c0], rax
0x1400bf758 mov      qword ptr [rax], rcx
0x1400bf75b lea      rcx, [rbp - 0x58]
0x1400bf75f call     0x140c34750
0x1400bf764 mov      qword ptr [rbp + 0x200], 0
0x1400bf76f mov      qword ptr [rbp + 0x208], 1
0x1400bf77a xorps    xmm0, xmm0
0x1400bf77d movups   xmmword ptr [rbp + 0x210], xmm0
0x1400bf784 mov      qword ptr [rbp + 0x220], 1
0x1400bf78f mov      qword ptr [rbp + 0x228], 0
0x1400bf79a movups   xmm1, xmmword ptr [rbp - 0x58]
0x1400bf79e movups   xmm2, xmmword ptr [rbp - 0x48]
0x1400bf7a2 movups   xmm3, xmmword ptr [rbp - 0x38]
0x1400bf7a6 movups   xmm4, xmmword ptr [rbp - 0x28]
0x1400bf7aa movups   xmmword ptr [rbp + 0x20], xmm1
0x1400bf7ae movups   xmmword ptr [rbp + 0x30], xmm2
0x1400bf7b2 movups   xmmword ptr [rbp + 0x40], xmm3
0x1400bf7b6 movups   xmmword ptr [rbp + 0x50], xmm4
0x1400bf7ba movups   xmm1, xmmword ptr [rbp - 0x18]
0x1400bf7be movups   xmmword ptr [rbp + 0x60], xmm1
0x1400bf7c2 movups   xmm1, xmmword ptr [rbp - 8]
0x1400bf7c6 movups   xmmword ptr [rbp + 0x70], xmm1
0x1400bf7ca movups   xmm1, xmmword ptr [rbp + 8]
0x1400bf7ce movups   xmmword ptr [rbp + 0x80], xmm1
0x1400bf7d5 movups   xmmword ptr [rbp + 0x278], xmm0
0x1400bf7dc mov      qword ptr [rbp + 0x160], 0
0x1400bf7e7 mov      qword ptr [rbp + 0x170], 0
0x1400bf7f2 mov      qword ptr [rbp + 0x180], 0
0x1400bf7fd movups   xmm1, xmmword ptr [rbp + 0x18]
0x1400bf801 movups   xmm2, xmmword ptr [rbp + 0x28]
0x1400bf805 movups   xmm3, xmmword ptr [rbp + 0x38]
0x1400bf809 movups   xmm4, xmmword ptr [rbp + 0x48]
0x1400bf80d movups   xmmword ptr [rbp + 0x188], xmm1
0x1400bf814 movups   xmmword ptr [rbp + 0x198], xmm2
0x1400bf81b movups   xmmword ptr [rbp + 0x1a8], xmm3
0x1400bf822 movups   xmmword ptr [rbp + 0x1b8], xmm4
0x1400bf829 movups   xmm1, xmmword ptr [rbp + 0x58]
0x1400bf82d movups   xmmword ptr [rbp + 0x1c8], xmm1
0x1400bf834 movups   xmm1, xmmword ptr [rbp + 0x68]
0x1400bf838 movups   xmmword ptr [rbp + 0x1d8], xmm1
0x1400bf83f movups   xmm1, xmmword ptr [rbp + 0x78]
0x1400bf843 movups   xmmword ptr [rbp + 0x1e8], xmm1
0x1400bf84a mov      rax, qword ptr [rbp + 0x88]
0x1400bf851 mov      qword ptr [rbp + 0x1f8], rax
0x1400bf858 mov      dword ptr [rbp + 0x230], 2
0x1400bf862 movhps   xmm0, qword ptr [rip + 0x2874d7f]
0x1400bf869 movups   xmmword ptr [rbp + 0x238], xmm0
0x1400bf870 mov      byte ptr [rbp + 0x248], 0
0x1400bf877 movaps   xmm0, xmmword ptr [rip + 0x2874d52]
0x1400bf87e movups   xmmword ptr [rbp + 0x250], xmm0
0x1400bf885 mov      byte ptr [rbp + 0x260], 0
0x1400bf88c mov      rax, qword ptr [rbp + 0x2c0]
0x1400bf893 mov      qword ptr [rbp + 0x268], rax
0x1400bf89a lea      rax, [rip + 0x29427f7]
0x1400bf8a1 mov      qword ptr [rbp + 0x270], rax
0x1400bf8a8 mov      qword ptr [rbp + 0x290], 0
0x1400bf8b3 mov      dword ptr [rbp + 0x2a0], 0x10101
0x1400bf8bd mov      byte ptr [rbp + 0x2a4], 2
0x1400bf8c4 movzx    eax, byte ptr [rip + 0x357533e]
0x1400bf8cb mov      ecx, 0xc
0x1400bf8d0 mov      edx, 1
0x1400bf8d5 call     0x140613c10
0x1400bf8da test     rax, rax
0x1400bf8dd je       0x1400bfc2a
0x1400bf8e3 movabs   rcx, 0x657274732054554c
0x1400bf8ed mov      qword ptr [rax], rcx
0x1400bf8f0 mov      dword ptr [rax + 8], 0x6874676e
0x1400bf8f7 mov      dword ptr [rbp + 0x2bc], 0
0x1400bf901 mov      dword ptr [rbp + 0x2b9], 0
0x1400bf90b mov      rbx, qword ptr [rbp + 0x190]
0x1400bf912 mov      ecx, ebx
0x1400bf914 and      ecx, 0xfffffffe
0x1400bf917 lea      rdx, [rbx - 3]
0x1400bf91b xor      r8d, r8d
0x1400bf91e cmp      ecx, 4
0x1400bf921 cmove    r8, rdx
0x1400bf925 test     r8, r8
0x1400bf928 mov      qword ptr [rbp + 0x2c0], rax
0x1400bf92f je       0x1400bf96a
0x1400bf931 cmp      r8, 1
0x1400bf935 jne      0x1400bf9d8
0x1400bf93b mov      rdx, qword ptr [rbp + 0x198]
0x1400bf942 test     rdx, rdx
0x1400bf945 je       0x1400bf959
0x1400bf947 mov      rcx, qword ptr [rbp + 0x1a0]
0x1400bf94e mov      r8d, 1
0x1400bf954 call     0x140613c20
0x1400bf959 lea      rcx, [rbp + 0x1b0]
0x1400bf960 call     0x140608f80
0x1400bf965 jmp      0x1400bf9f1
0x1400bf96a mov      rdx, qword ptr [rbp + 0x1c0]
0x1400bf971 test     rdx, rdx
0x1400bf974 je       0x1400bf988
0x1400bf976 mov      rcx, qword ptr [rbp + 0x1c8]
0x1400bf97d mov      r8d, 1
0x1400bf983 call     0x140613c20
0x1400bf988 cmp      ebx, 3
0x1400bf98b je       0x1400bf9ab
0x1400bf98d cmp      ebx, 2
0x1400bf990 jb       0x1400bf9ab
0x1400bf992 mov      rax, qword ptr [rbp + 0x198]
0x1400bf999 lock dec qword ptr [rax]
0x1400bf99d jne      0x1400bf9ab
0x1400bf99f lea      rcx, [rbp + 0x198]
0x1400bf9a6 call     0x14064eb30
0x1400bf9ab mov      rax, qword ptr [rbp + 0x1a8]
0x1400bf9b2 cmp      rax, 6
0x1400bf9b6 je       0x1400bf9f1
0x1400bf9b8 cmp      eax, 5
0x1400bf9bb jb       0x1400bf9f1
0x1400bf9bd mov      rax, qword ptr [rbp + 0x1b0]
0x1400bf9c4 lock dec qword ptr [rax]
0x1400bf9c8 jne      0x1400bf9f1
0x1400bf9ca lea      rcx, [rbp + 0x1b0]
0x1400bf9d1 call     0x14064eb30
0x1400bf9d6 jmp      0x1400bf9f1
0x1400bf9d8 mov      rax, qword ptr [rbp + 0x198]
0x1400bf9df lock dec qword ptr [rax]
0x1400bf9e3 jne      0x1400bf9f1
0x1400bf9e5 lea      rcx, [rbp + 0x198]
0x1400bf9ec call     0x140c7de90
0x1400bf9f1 mov      qword ptr [rbp + 0x190], 3
0x1400bf9fc movups   xmm0, xmmword ptr [rbp + 0x18]
0x1400bfa00 movups   xmmword ptr [rbp + 0x198], xmm0
0x1400bfa07 mov      qword ptr [rbp + 0x1a8], 6
0x1400bfa12 movups   xmm0, xmmword ptr [rbp - 0x58]
0x1400bfa16 movups   xmmword ptr [rbp + 0x1b0], xmm0
0x1400bfa1d mov      qword ptr [rbp + 0x1c0], 0xc
0x1400bfa28 mov      rax, qword ptr [rbp + 0x2c0]
0x1400bfa2f mov      qword ptr [rbp + 0x1c8], rax
0x1400bfa36 mov      qword ptr [rbp + 0x1d0], 0xc
0x1400bfa41 mov      dword ptr [rbp + 0x1d8], 0
0x1400bfa4b mov      dword ptr [rbp + 0x1e0], 0
0x1400bfa55 mov      qword ptr [rbp + 0x1e8], 0
0x1400bfa60 mov      byte ptr [rbp + 0x1f0], 0
0x1400bfa67 mov      rax, qword ptr [rbp + 0x2b5]
0x1400bfa6e mov      qword ptr [rbp + 0x1f1], rax
0x1400bfa75 mov      eax, dword ptr [rbp + 0x2bc]
0x1400bfa7b mov      dword ptr [rbp + 0x1f8], eax
0x1400bfa81 lea      rbx, [rbp + 0x18]
0x1400bfa85 lea      rdx, [rbp + 0x160]
0x1400bfa8c mov      r8d, 0x148
0x1400bfa92 mov      rcx, rbx
0x1400bfa95 call     0x142923c00
0x1400bfa9a lea      rcx, [rbp + 0x160]
0x1400bfaa1 mov      rdx, rbx
0x1400bfaa4 mov      r8, rsi
0x1400bfaa7 call     0x140c77160
0x1400bfaac mov      rax, qword ptr [rbp + 0x170]
0x1400bfab3 lock dec qword ptr [rax]
0x1400bfab7 jne      0x1400bfac5
0x1400bfab9 lea      rcx, [rbp + 0x170]
0x1400bfac0 call     0x140c41170
0x1400bfac5 movzx    eax, byte ptr [rip + 0x357513d]
0x1400bfacc mov      ecx, 8
0x1400bfad1 mov      edx, 8
0x1400bfad6 call     0x140613c10
0x1400bfadb test     rax, rax
0x1400bfade je       0x1400bfbf4
0x1400bfae4 lea      rcx, [rdi + 0x1c]
0x1400bfae8 mov      qword ptr [rax], rcx
0x1400bfaeb lea      rcx, [rip + 0x2874f9e]
0x1400bfaf2 mov      qword ptr [rsp + 0x20], rcx
0x1400bfaf7 lea      rcx, [rbp + 0x160]
0x1400bfafe mov      rdx, rsi
0x1400bfb01 xor      r8d, r8d
0x1400bfb04 mov      r9, rax
0x1400bfb07 call     0x1400c4230
0x1400bfb0c mov      rax, qword ptr [rbp + 0x170]
0x1400bfb13 lock dec qword ptr [rax]
0x1400bfb17 jne      0x1400bfb25
0x1400bfb19 lea      rcx, [rbp + 0x170]
0x1400bfb20 call     0x140c41170
0x1400bfb25 movzx    eax, byte ptr [rip + 0x35750dd]
0x1400bfb2c mov      ecx, 8
0x1400bfb31 mov      edx, 8
0x1400bfb36 call     0x140613c10
0x1400bfb3b test     rax, rax
0x1400bfb3e je       0x1400bfbf4
0x1400bfb44 lea      rcx, [rdi + 0x20]
0x1400bfb48 mov      qword ptr [rax], rcx
0x1400bfb4b lea      rcx, [rip + 0x2874efe]
0x1400bfb52 mov      qword ptr [rsp + 0x20], rcx
0x1400bfb57 lea      rcx, [rbp + 0x160]
0x1400bfb5e mov      rdx, rsi
0x1400bfb61 xor      r8d, r8d
0x1400bfb64 mov      r9, rax
0x1400bfb67 call     0x1400c4230
0x1400bfb6c mov      rax, qword ptr [rbp + 0x170]
0x1400bfb73 lock dec qword ptr [rax]
0x1400bfb77 jne      0x1400bfb85
0x1400bfb79 lea      rcx, [rbp + 0x170]
0x1400bfb80 call     0x140c41170
0x1400bfb85 movzx    eax, byte ptr [rip + 0x357507d]
0x1400bfb8c mov      ecx, 8
0x1400bfb91 mov      edx, 8
0x1400bfb96 call     0x140613c10
0x1400bfb9b test     rax, rax
0x1400bfb9e je       0x1400bfbf4
0x1400bfba0 add      rdi, 0x24
0x1400bfba4 mov      qword ptr [rax], rdi
0x1400bfba7 lea      rcx, [rip + 0x2874f22]
0x1400bfbae mov      qword ptr [rsp + 0x20], rcx
0x1400bfbb3 lea      rcx, [rbp + 0x160]
0x1400bfbba mov      rdx, rsi
0x1400bfbbd xor      r8d, r8d
0x1400bfbc0 mov      r9, rax
0x1400bfbc3 call     0x1400c4230
0x1400bfbc8 mov      rax, qword ptr [rbp + 0x170]
0x1400bfbcf lock dec qword ptr [rax]
0x1400bfbd3 jne      0x1400bfbe1
0x1400bfbd5 lea      rcx, [rbp + 0x170]
0x1400bfbdc call     0x140c41170
0x1400bfbe1 nop      
0x1400bfbe2 add      rsp, 0x350
0x1400bfbe9 pop      rbx
0x1400bfbea pop      rdi
0x1400bfbeb pop      rsi
0x1400bfbec pop      r12
0x1400bfbee pop      r14
0x1400bfbf0 pop      r15
0x1400bfbf2 pop      rbp
0x1400bfbf3 ret      
0x1400bfbf4 mov      ecx, 8
0x1400bfbf9 mov      edx, 8
0x1400bfbfe call     0x1428d9000
0x1400bfc03 lea      r8, [rip + 0x287513e]
0x1400bfc0a mov      ecx, 1
0x1400bfc0f mov      edx, 0x14
0x1400bfc14 call     0x1428d8fe3
0x1400bfc19 mov      ecx, 8
0x1400bfc1e mov      edx, 0x10
0x1400bfc23 call     0x1428d9000
0x1400bfc28 jmp      0x1400bfc40
0x1400bfc2a lea      r8, [rip + 0x2875117]
0x1400bfc31 mov      ecx, 1
0x1400bfc36 mov      edx, 0xc