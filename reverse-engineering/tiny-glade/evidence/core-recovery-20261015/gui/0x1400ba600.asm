0x1400ba600 push     rbp
0x1400ba601 push     r15
0x1400ba603 push     r14
0x1400ba605 push     r13
0x1400ba607 push     r12
0x1400ba609 push     rsi
0x1400ba60a push     rdi
0x1400ba60b push     rbx
0x1400ba60c sub      rsp, 0x3a8
0x1400ba613 lea      rbp, [rsp + 0x80]
0x1400ba61b mov      qword ptr [rbp + 0x320], 0xfffffffffffffffe
0x1400ba626 mov      qword ptr [rbp + 0x308], rdx
0x1400ba62d mov      rbx, qword ptr [rcx]
0x1400ba630 mov      r15, qword ptr [rcx + 8]
0x1400ba634 mov      qword ptr [rbp + 0x148], 4
0x1400ba63f mov      byte ptr [rbp + 0x31d], 0
0x1400ba646 lea      rdx, [rip + 0x287a9bf] ; @0x14293500c 'ALL Cluttercrates/tiny-glade/src/gui/gui_debug.rs'
0x1400ba64d lea      rcx, [rbp - 0x60]
0x1400ba651 mov      r8d, 0xb
0x1400ba657 call     0x1400568d0
0x1400ba65c movups   xmm0, xmmword ptr [rbp]
0x1400ba660 movaps   xmmword ptr [rbp + 0x250], xmm0
0x1400ba667 movups   xmm0, xmmword ptr [rbp - 0x10]
0x1400ba66b movaps   xmmword ptr [rbp + 0x240], xmm0
0x1400ba672 movups   xmm0, xmmword ptr [rbp - 0x20]
0x1400ba676 movaps   xmmword ptr [rbp + 0x230], xmm0
0x1400ba67d movups   xmm0, xmmword ptr [rbp - 0x60]
0x1400ba681 movups   xmm1, xmmword ptr [rbp - 0x50]
0x1400ba685 movups   xmm2, xmmword ptr [rbp - 0x40]
0x1400ba689 movups   xmm3, xmmword ptr [rbp - 0x30]
0x1400ba68d movaps   xmmword ptr [rbp + 0x220], xmm3
0x1400ba694 movaps   xmmword ptr [rbp + 0x210], xmm2
0x1400ba69b movaps   xmmword ptr [rbp + 0x200], xmm1
0x1400ba6a2 movaps   xmmword ptr [rbp + 0x1f0], xmm0
0x1400ba6a9 mov      byte ptr [rbp + 0x31d], 1
0x1400ba6b0 lea      rcx, [rbp - 0x60]
0x1400ba6b4 lea      rdx, [rbp + 0x148]
0x1400ba6bb lea      r8, [rbp + 0x1f0]
0x1400ba6c2 call     0x140c46bd0
0x1400ba6c7 lea      rcx, [rbp + 0x268]
0x1400ba6ce lea      rdx, [rbp - 0x60]
0x1400ba6d2 mov      r8, qword ptr [rbp + 0x308]
0x1400ba6d9 call     0x140c46d20
0x1400ba6de cmp      byte ptr [rbp + 0x2bc], 0
0x1400ba6e5 je       0x1400ba73f
0x1400ba6e7 mov      rax, qword ptr [rbp + 0x278]
0x1400ba6ee lock dec qword ptr [rax]
0x1400ba6f2 jne      0x1400ba700
0x1400ba6f4 lea      rcx, [rbp + 0x278]
0x1400ba6fb call     0x140c41170
0x1400ba700 mov      eax, dword ptr [rbx + 0x1c]
0x1400ba703 mov      rcx, qword ptr [rbx + 0x10]
0x1400ba707 mov      dword ptr [rcx], eax
0x1400ba709 mov      rsi, qword ptr [rbx]
0x1400ba70c mov      rdi, qword ptr [rsi + 0x30]
0x1400ba710 mov      rbx, qword ptr [rsi + 0x40]
0x1400ba714 cmp      rdi, qword ptr [rsi + 0x20]
0x1400ba718 jne      0x1400ba72a
0x1400ba71a lea      rcx, [rsi + 0x20]
0x1400ba71e lea      rdx, [rip + 0x28a2dab]
0x1400ba725 call     0x140562870
0x1400ba72a mov      rax, qword ptr [rsi + 0x28]
0x1400ba72e mov      qword ptr [rax + rdi*8], rbx
0x1400ba732 inc      rdi
0x1400ba735 mov      qword ptr [rsi + 0x30], rdi
0x1400ba739 inc      qword ptr [rsi + 0x40]
0x1400ba73d jmp      0x1400ba779
0x1400ba73f mov      byte ptr [rbp - 0x60], 0
0x1400ba743 cmp      byte ptr [rbp + 0x2bb], 0
0x1400ba74a je       0x1400ba760
0x1400ba74c lea      rcx, [rbp + 0x278]
0x1400ba753 lea      rdx, [rbp - 0x60]
0x1400ba757 call     0x140042870
0x1400ba75c test     al, al
0x1400ba75e jne      0x1400ba6e7
0x1400ba760 mov      rax, qword ptr [rbp + 0x278]
0x1400ba767 lock dec qword ptr [rax]
0x1400ba76b jne      0x1400ba779
0x1400ba76d lea      rcx, [rbp + 0x278]
0x1400ba774 call     0x140c41170
0x1400ba779 call     0x140002370
0x1400ba77e lea      r8, [rip + 0x2879f73]
0x1400ba785 lea      rcx, [rbp + 0x2c8]
0x1400ba78c mov      edx, eax
0x1400ba78e call     0x1402ba660
0x1400ba793 mov      rsi, qword ptr [rbp + 0x2d0]
0x1400ba79a mov      r13, qword ptr [rbp + 0x2d8]
0x1400ba7a1 lea      rax, [rbp + 0x148]
0x1400ba7a8 mov      qword ptr [rbp - 0x60], rax
0x1400ba7ac cmp      r13, 2
0x1400ba7b0 jae      0x1400baa89
0x1400ba7b6 mov      rax, qword ptr [rbp + 0x2c8]
0x1400ba7bd shl      r13, 5
0x1400ba7c1 add      r13, rsi
0x1400ba7c4 mov      qword ptr [rbp + 0x2e0], rsi
0x1400ba7cb mov      qword ptr [rbp + 0x2e8], rsi
0x1400ba7d2 mov      qword ptr [rbp + 0x2f0], rax
0x1400ba7d9 mov      qword ptr [rbp + 0x2f8], r13
0x1400ba7e0 lea      r12, [rbp + 0x220]
0x1400ba7e7 lea      r14, [rbp - 0x60]
0x1400ba7eb jmp      0x1400ba7fe
0x1400ba7ed nop      dword ptr [rax]
0x1400ba7f0 mov      rsi, qword ptr [rbp + 0x2e8]
0x1400ba7f7 mov      r13, qword ptr [rbp + 0x2f8]
0x1400ba7fe cmp      rsi, r13
0x1400ba801 je       0x1400baa17
0x1400ba807 lea      rbx, [rsi + 0x20]
0x1400ba80b mov      qword ptr [rbp + 0x2e8], rbx
0x1400ba812 mov      rcx, qword ptr [rsi]
0x1400ba815 mov      rax, rcx
0x1400ba818 neg      rax
0x1400ba81b jo       0x1400baa1a
0x1400ba821 mov      qword ptr [rbp + 0x310], rcx
0x1400ba828 mov      rax, qword ptr [rsi + 8]
0x1400ba82c mov      qword ptr [rbp + 0x300], rax
0x1400ba833 movzx    eax, byte ptr [rsi + 0x18]
0x1400ba837 mov      byte ptr [rbp + 0x31e], al
0x1400ba83d lea      rax, [rbp + 0x31e]
0x1400ba844 mov      qword ptr [rbp + 0x148], rax
0x1400ba84b lea      rax, [rip + 0xa08e]
0x1400ba852 mov      qword ptr [rbp + 0x150], rax
0x1400ba859 lea      rax, [rip + 0x287a7e0]
0x1400ba860 mov      qword ptr [rbp - 0x60], rax
0x1400ba864 mov      qword ptr [rbp - 0x58], 1
0x1400ba86c lea      rax, [rbp + 0x148]
0x1400ba873 mov      qword ptr [rbp - 0x50], rax
0x1400ba877 mov      qword ptr [rbp - 0x48], 1
0x1400ba87f mov      qword ptr [rbp - 0x40], 0
0x1400ba887 lea      rcx, [rbp + 0x1d8]
0x1400ba88e mov      rdx, r14
0x1400ba891 call     0x140d79f90
0x1400ba896 mov      rax, qword ptr [rbp + 0x1e8]
0x1400ba89d mov      qword ptr [r12 + 0x10], rax
0x1400ba8a2 movups   xmm0, xmmword ptr [rbp + 0x1d8]
0x1400ba8a9 movups   xmmword ptr [r12], xmm0
0x1400ba8ae mov      qword ptr [rbp + 0x148], 4
0x1400ba8b9 mov      qword ptr [rbp + 0x1f0], 3
0x1400ba8c4 mov      qword ptr [rbp + 0x208], 6
0x1400ba8cf mov      dword ptr [rbp + 0x238], 0
0x1400ba8d9 mov      dword ptr [rbp + 0x240], 0
0x1400ba8e3 mov      dword ptr [r12 + 0x38], 0
0x1400ba8ec mov      dword ptr [r12 + 0x35], 0
0x1400ba8f5 mov      byte ptr [r12 + 0x30], 0
0x1400ba8fb mov      qword ptr [r12 + 0x28], 0
0x1400ba904 mov      rcx, r14
0x1400ba907 lea      rdx, [rbp + 0x148]
0x1400ba90e lea      r8, [rbp + 0x1f0]
0x1400ba915 call     0x140c46bd0
0x1400ba91a lea      rcx, [rbp + 0x268]
0x1400ba921 mov      rdx, r14
0x1400ba924 mov      r8, qword ptr [rbp + 0x308]
0x1400ba92b call     0x140c46d20
0x1400ba930 cmp      byte ptr [rbp + 0x2bc], 0
0x1400ba937 je       0x1400ba9b0
0x1400ba939 mov      rax, qword ptr [rbp + 0x278]
0x1400ba940 lock dec qword ptr [rax]
0x1400ba944 jne      0x1400ba952
0x1400ba946 lea      rcx, [rbp + 0x278]
0x1400ba94d call     0x140c41170
0x1400ba952 movzx    ebx, byte ptr [rbp + 0x31e]
0x1400ba959 mov      eax, dword ptr [r15 + 0x1c]
0x1400ba95d mov      rcx, qword ptr [r15 + 0x10]
0x1400ba961 mov      dword ptr [rcx], eax
0x1400ba963 mov      rsi, qword ptr [r15]
0x1400ba966 mov      r13, qword ptr [rsi + 0x30]
0x1400ba96a mov      rdi, qword ptr [rsi + 0x40]
0x1400ba96e cmp      r13, qword ptr [rsi + 0x20]
0x1400ba972 jne      0x1400ba984
0x1400ba974 lea      rcx, [rsi + 0x20]
0x1400ba978 lea      rdx, [rip + 0x28a2b51]
0x1400ba97f call     0x140562630
0x1400ba984 mov      rax, qword ptr [rsi + 0x28]
0x1400ba988 mov      rcx, r13
0x1400ba98b shl      rcx, 4
0x1400ba98f mov      qword ptr [rax + rcx], rdi
0x1400ba993 mov      byte ptr [rax + rcx + 8], bl
0x1400ba997 inc      r13
0x1400ba99a mov      qword ptr [rsi + 0x30], r13
0x1400ba99e inc      qword ptr [rsi + 0x40]
0x1400ba9a2 jmp      0x1400ba9f0
0x1400ba9a4 nop      word ptr cs:[rax + rax]
0x1400ba9b0 mov      byte ptr [rbp - 0x60], 0
0x1400ba9b4 cmp      byte ptr [rbp + 0x2bb], 0
0x1400ba9bb je       0x1400ba9d4
0x1400ba9bd lea      rcx, [rbp + 0x278]
0x1400ba9c4 mov      rdx, r14
0x1400ba9c7 call     0x140042870
0x1400ba9cc test     al, al
0x1400ba9ce jne      0x1400ba939
0x1400ba9d4 mov      rax, qword ptr [rbp + 0x278]
0x1400ba9db lock dec qword ptr [rax]
0x1400ba9df jne      0x1400ba9f0
0x1400ba9e1 lea      rcx, [rbp + 0x278]
0x1400ba9e8 call     0x140c41170
0x1400ba9ed nop      dword ptr [rax]
0x1400ba9f0 mov      rdx, qword ptr [rbp + 0x310]
0x1400ba9f7 test     rdx, rdx
0x1400ba9fa je       0x1400ba7f0
0x1400baa00 mov      r8d, 1
0x1400baa06 mov      rcx, qword ptr [rbp + 0x300]
0x1400baa0d call     0x140613c20
0x1400baa12 jmp      0x1400ba7f0
0x1400baa17 mov      rbx, rsi
0x1400baa1a sub      r13, rbx
0x1400baa1d je       0x1400baa52
0x1400baa1f shr      r13, 5
0x1400baa23 add      rbx, 8
0x1400baa27 jmp      0x1400baa39
0x1400baa29 nop      dword ptr [rax]
0x1400baa30 add      rbx, 0x20
0x1400baa34 dec      r13
0x1400baa37 je       0x1400baa52
0x1400baa39 mov      rdx, qword ptr [rbx - 8]
0x1400baa3d test     rdx, rdx
0x1400baa40 je       0x1400baa30
0x1400baa42 mov      rcx, qword ptr [rbx]
0x1400baa45 mov      r8d, 1
0x1400baa4b call     0x140613c20
0x1400baa50 jmp      0x1400baa30
0x1400baa52 mov      rdx, qword ptr [rbp + 0x2f0]
0x1400baa59 test     rdx, rdx
0x1400baa5c je       0x1400baa74
0x1400baa5e mov      rcx, qword ptr [rbp + 0x2e0]
0x1400baa65 shl      rdx, 5
0x1400baa69 mov      r8d, 8
0x1400baa6f call     0x140613c20
0x1400baa74 nop      
0x1400baa75 add      rsp, 0x3a8
0x1400baa7c pop      rbx
0x1400baa7d pop      rdi
0x1400baa7e pop      rsi
0x1400baa7f pop      r12
0x1400baa81 pop      r13
0x1400baa83 pop      r14
0x1400baa85 pop      r15
0x1400baa87 pop      rbp
0x1400baa88 ret      
0x1400baa89 cmp      r13, 0x15
0x1400baa8d jae      0x1400baabd
0x1400baa8f mov      rbx, r13
0x1400baa92 shl      rbx, 5
0x1400baa96 add      rbx, rsi
0x1400baa99 lea      r14, [rsi + 0x20]
0x1400baa9d mov      byte ptr [rbp + 0x31f], 1
0x1400baaa4 mov      rcx, rsi
0x1400baaa7 mov      rdx, r14
0x1400baaaa call     0x1405c0d20
0x1400baaaf add      r14, 0x20
0x1400baab3 cmp      r14, rbx
0x1400baab6 jne      0x1400baa9d
0x1400baab8 jmp      0x1400ba7b6