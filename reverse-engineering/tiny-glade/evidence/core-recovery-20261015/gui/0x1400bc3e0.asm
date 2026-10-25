0x1400bc3e0 push     rbp
0x1400bc3e1 push     r15
0x1400bc3e3 push     r14
0x1400bc3e5 push     r13
0x1400bc3e7 push     r12
0x1400bc3e9 push     rsi
0x1400bc3ea push     rdi
0x1400bc3eb push     rbx
0x1400bc3ec sub      rsp, 0x388
0x1400bc3f3 lea      rbp, [rsp + 0x80]
0x1400bc3fb movaps   xmmword ptr [rbp + 0x2f0], xmm6
0x1400bc402 mov      qword ptr [rbp + 0x2e8], 0xfffffffffffffffe
0x1400bc40d mov      r15, rdx
0x1400bc410 mov      rdi, rcx
0x1400bc413 mov      rsi, qword ptr [rip + 0x356fca6]
0x1400bc41a movss    xmm0, dword ptr [rsi]
0x1400bc41e divss    xmm0, dword ptr [rip + 0x287824a]
0x1400bc426 call     0x142923f50
0x1400bc42b movss    dword ptr [rbp + 0x1d0], xmm0
0x1400bc433 mov      rbx, qword ptr [rdi]
0x1400bc436 movzx    eax, byte ptr [rip + 0x35787cc]
0x1400bc43d mov      ecx, 0x10
0x1400bc442 mov      edx, 8
0x1400bc447 call     0x140613c10
0x1400bc44c test     rax, rax
0x1400bc44f je       0x1400bcaa0
0x1400bc455 mov      qword ptr [rax], rbx
0x1400bc458 lea      rcx, [rbp + 0x1d0]
0x1400bc45f mov      qword ptr [rax + 8], rcx
0x1400bc463 lea      rcx, [rip + 0x28785c6]
0x1400bc46a mov      qword ptr [rsp + 0x20], rcx
0x1400bc46f lea      rcx, [rbp + 0x28]
0x1400bc473 mov      rdx, r15
0x1400bc476 xor      r8d, r8d
0x1400bc479 mov      r9, rax
0x1400bc47c call     0x1400c4230
0x1400bc481 mov      rax, qword ptr [rbp + 0x38]
0x1400bc485 lock dec qword ptr [rax]
0x1400bc489 jne      0x1400bc494
0x1400bc48b lea      rcx, [rbp + 0x38]
0x1400bc48f call     0x140c41170
0x1400bc494 movss    xmm0, dword ptr [rbp + 0x1d0]
0x1400bc49c call     0x142923eb0
0x1400bc4a1 mulss    xmm0, dword ptr [rip + 0x28781c7]
0x1400bc4a9 movss    dword ptr [rsi], xmm0
0x1400bc4ad mov      rbx, qword ptr [rdi + 8]
0x1400bc4b1 mov      r14, qword ptr [rdi + 0x28]
0x1400bc4b5 movzx    eax, byte ptr [rip + 0x357874d]
0x1400bc4bc mov      ecx, 0x10
0x1400bc4c1 mov      edx, 8
0x1400bc4c6 call     0x140613c10
0x1400bc4cb test     rax, rax
0x1400bc4ce je       0x1400bcaa0
0x1400bc4d4 mov      qword ptr [rax], rbx
0x1400bc4d7 mov      qword ptr [rax + 8], r14
0x1400bc4db lea      rcx, [rip + 0x28784ee]
0x1400bc4e2 mov      qword ptr [rsp + 0x20], rcx
0x1400bc4e7 lea      rcx, [rbp + 0x28]
0x1400bc4eb mov      rdx, r15
0x1400bc4ee xor      r8d, r8d
0x1400bc4f1 mov      r9, rax
0x1400bc4f4 call     0x1400c4230
0x1400bc4f9 mov      rax, qword ptr [rbp + 0x38]
0x1400bc4fd lock dec qword ptr [rax]
0x1400bc501 jne      0x1400bc50c
0x1400bc503 lea      rcx, [rbp + 0x38]
0x1400bc507 call     0x140c41170
0x1400bc50c mov      rsi, qword ptr [rdi + 0x20]
0x1400bc510 movzx    eax, byte ptr [rip + 0x35786f2]
0x1400bc517 mov      ecx, 8
0x1400bc51c mov      edx, 8
0x1400bc521 call     0x140613c10
0x1400bc526 test     rax, rax
0x1400bc529 je       0x1400bcaaf
0x1400bc52f mov      qword ptr [rax], rsi
0x1400bc532 lea      rcx, [rip + 0x28784b7]
0x1400bc539 mov      qword ptr [rsp + 0x20], rcx
0x1400bc53e lea      rcx, [rbp + 0x28]
0x1400bc542 mov      rdx, r15
0x1400bc545 xor      r8d, r8d
0x1400bc548 mov      r9, rax
0x1400bc54b call     0x1400c4230
0x1400bc550 mov      rax, qword ptr [rbp + 0x38]
0x1400bc554 lock dec qword ptr [rax]
0x1400bc558 jne      0x1400bc563
0x1400bc55a lea      rcx, [rbp + 0x38]
0x1400bc55e call     0x140c41170
0x1400bc563 movups   xmm6, xmmword ptr [rdi + 0x10]
0x1400bc567 movzx    eax, byte ptr [rip + 0x357869b]
0x1400bc56e mov      ecx, 0x10
0x1400bc573 mov      edx, 8
0x1400bc578 call     0x140613c10
0x1400bc57d test     rax, rax
0x1400bc580 je       0x1400bcaa0
0x1400bc586 movups   xmmword ptr [rax], xmm6
0x1400bc589 lea      rcx, [rip + 0x2878520]
0x1400bc590 mov      qword ptr [rsp + 0x20], rcx
0x1400bc595 lea      rcx, [rbp + 0x28]
0x1400bc599 mov      rdx, r15
0x1400bc59c xor      r8d, r8d
0x1400bc59f mov      r9, rax
0x1400bc5a2 call     0x1400c4230
0x1400bc5a7 mov      rax, qword ptr [rbp + 0x38]
0x1400bc5ab lock dec qword ptr [rax]
0x1400bc5af jne      0x1400bc5ba
0x1400bc5b1 lea      rcx, [rbp + 0x38]
0x1400bc5b5 call     0x140c41170
0x1400bc5ba mov      dword ptr [rbp + 0x2c0], 2
0x1400bc5c4 mov      r12, qword ptr [rdi + 0x30]
0x1400bc5c8 mov      eax, dword ptr [r12 + 0x1c]
0x1400bc5cd mov      rcx, qword ptr [r12 + 0x10]
0x1400bc5d2 mov      dword ptr [rcx], eax
0x1400bc5d4 mov      rcx, qword ptr [r12]
0x1400bc5d8 mov      rax, qword ptr [rcx + 0x10]
0x1400bc5dc test     rax, rax
0x1400bc5df je       0x1400bc83f
0x1400bc5e5 mov      qword ptr [rbp + 0x2d8], r12
0x1400bc5ec mov      qword ptr [rbp + 0x2d0], rbx
0x1400bc5f3 mov      r13, qword ptr [rcx + 8]
0x1400bc5f7 mov      rsi, qword ptr [rdi + 0x38]
0x1400bc5fb shl      rax, 4
0x1400bc5ff lea      rdi, [rax + rax*4]
0x1400bc603 lea      rbx, [rbp + 0x28]
0x1400bc607 jmp      0x1400bc61e
0x1400bc609 nop      dword ptr [rax]
0x1400bc610 add      r13, 0x50
0x1400bc614 add      rdi, -0x50
0x1400bc618 je       0x1400bc6db
0x1400bc61e mov      r12, r15
0x1400bc621 mov      r15, qword ptr [r13]
0x1400bc625 movzx    eax, byte ptr [rip + 0x35785dd]
0x1400bc62c mov      ecx, 0x20
0x1400bc631 mov      edx, 8
0x1400bc636 call     0x140613c10
0x1400bc63b test     rax, rax
0x1400bc63e je       0x1400bca91
0x1400bc644 mov      r8, rax
0x1400bc647 mov      qword ptr [rax], rsi
0x1400bc64a mov      qword ptr [rax + 8], r13
0x1400bc64e mov      qword ptr [rax + 0x10], r14
0x1400bc652 movabs   rax, 0xbe5466cf34e90c6e
0x1400bc65c xor      r15, rax
0x1400bc65f mov      rax, r15
0x1400bc662 movabs   rcx, 0x5851f42d4c957f2d
0x1400bc66c mul      rcx
0x1400bc66f mov      rcx, rax
0x1400bc672 xor      rcx, rdx
0x1400bc675 mov      rax, rcx
0x1400bc678 movabs   rdx, 0x452821e638d01376
0x1400bc682 mul      rdx
0x1400bc685 xor      rdx, rax
0x1400bc688 mov      rax, rdx
0x1400bc68b rol      rax, cl
0x1400bc68e lea      rcx, [rbp + 0x2c0]
0x1400bc695 mov      qword ptr [r8 + 0x18], rcx
0x1400bc699 test     rdx, rdx
0x1400bc69c mov      ecx, 1
0x1400bc6a1 cmove    rax, rcx
0x1400bc6a5 mov      qword ptr [rsp + 0x20], rax
0x1400bc6aa mov      rcx, rbx
0x1400bc6ad mov      r15, r12
0x1400bc6b0 mov      rdx, r12
0x1400bc6b3 lea      r9, [rip + 0x28785fe]
0x1400bc6ba call     0x1400c4680
0x1400bc6bf mov      rax, qword ptr [rbp + 0x38]
0x1400bc6c3 lock dec qword ptr [rax]
0x1400bc6c7 jne      0x1400bc610
0x1400bc6cd lea      rcx, [rbp + 0x38]
0x1400bc6d1 call     0x140c41170
0x1400bc6d6 jmp      0x1400bc610
0x1400bc6db mov      eax, dword ptr [rbp + 0x2c0]
0x1400bc6e1 test     eax, eax
0x1400bc6e3 je       0x1400bc7f1
0x1400bc6e9 cmp      eax, 1
0x1400bc6ec mov      rbx, qword ptr [rbp + 0x2d0]
0x1400bc6f3 mov      r12, qword ptr [rbp + 0x2d8]
0x1400bc6fa jne      0x1400bc83f
0x1400bc700 mov      rax, qword ptr [r12]
0x1400bc704 cmp      qword ptr [rax + 0x10], 2
0x1400bc709 jb       0x1400bc83f
0x1400bc70f mov      rdx, qword ptr [rbp + 0x2c8]
0x1400bc716 mov      ecx, dword ptr [r12 + 0x1c]
0x1400bc71b mov      r8, qword ptr [r12 + 0x10]
0x1400bc720 mov      dword ptr [r8], ecx
0x1400bc723 mov      rcx, qword ptr [rax + 0x10]
0x1400bc727 test     rcx, rcx
0x1400bc72a je       0x1400bc83f
0x1400bc730 mov      r8, qword ptr [rax + 8]
0x1400bc734 mov      r9d, 1
0x1400bc73a mov      r11d, 1
0x1400bc740 cmp      rdx, qword ptr [r8]
0x1400bc743 je       0x1400bc768
0x1400bc745 lea      r10, [r8 + 0x50]
0x1400bc749 nop      dword ptr [rax]
0x1400bc750 cmp      rcx, r11
0x1400bc753 je       0x1400bc835
0x1400bc759 inc      r11
0x1400bc75c lea      rsi, [r10 + 0x50]
0x1400bc760 cmp      rdx, qword ptr [r10]
0x1400bc763 mov      r10, rsi
0x1400bc766 jne      0x1400bc750
0x1400bc768 mov      r10, rcx
0x1400bc76b sub      r10, r11
0x1400bc76e je       0x1400bc838
0x1400bc774 lea      r9, [r11 + r11*4]
0x1400bc778 shl      r9, 4
0x1400bc77c add      r8, r9
0x1400bc77f mov      r9d, 1
0x1400bc785 jmp      0x1400bc7a0
0x1400bc787 nop      word ptr [rax + rax]
0x1400bc790 inc      r9
0x1400bc793 add      r8, 0x50
0x1400bc797 dec      r10
0x1400bc79a je       0x1400bc838
0x1400bc7a0 cmp      rdx, qword ptr [r8]
0x1400bc7a3 je       0x1400bc790
0x1400bc7a5 mov      r11, r9
0x1400bc7a8 shl      r11, 4
0x1400bc7ac lea      r11, [r11 + r11*4]
0x1400bc7b0 mov      rsi, r8
0x1400bc7b3 sub      rsi, r11
0x1400bc7b6 movups   xmm0, xmmword ptr [r8 + 0x40]
0x1400bc7bb neg      r11
0x1400bc7be movups   xmmword ptr [r8 + r11 + 0x40], xmm0
0x1400bc7c4 movups   xmm0, xmmword ptr [r8]
0x1400bc7c8 movups   xmm1, xmmword ptr [r8 + 0x10]
0x1400bc7cd movups   xmm2, xmmword ptr [r8 + 0x20]
0x1400bc7d2 movups   xmm3, xmmword ptr [r8 + 0x30]
0x1400bc7d7 movups   xmmword ptr [rsi + 0x30], xmm3
0x1400bc7db movups   xmmword ptr [rsi + 0x20], xmm2
0x1400bc7df movups   xmmword ptr [rsi + 0x10], xmm1
0x1400bc7e3 movups   xmmword ptr [rsi], xmm0
0x1400bc7e6 add      r8, 0x50
0x1400bc7ea dec      r10
0x1400bc7ed jne      0x1400bc7a0
0x1400bc7ef jmp      0x1400bc838
0x1400bc7f1 movss    xmm0, dword ptr [rbp + 0x2c4]
0x1400bc7f9 movss    xmm6, dword ptr [rip + 0x2869103]
0x1400bc801 movaps   xmm1, xmm6
0x1400bc804 call     0x142923f10
0x1400bc809 addss    xmm6, xmm0
0x1400bc80d xorps    xmm1, xmm1
0x1400bc810 movaps   xmm2, xmm0
0x1400bc813 cmpltss  xmm2, xmm1
0x1400bc818 andps    xmm6, xmm2
0x1400bc81b andnps   xmm2, xmm0
0x1400bc81e orps     xmm2, xmm6
0x1400bc821 mov      rbx, qword ptr [rbp + 0x2d0]
0x1400bc828 movss    dword ptr [rbx], xmm2
0x1400bc82c mov      r12, qword ptr [rbp + 0x2d8]
0x1400bc833 jmp      0x1400bc83f
0x1400bc835 xor      r9d, r9d
0x1400bc838 sub      rcx, r9
0x1400bc83b mov      qword ptr [rax + 0x10], rcx
0x1400bc83f mov      qword ptr [rbp + 0x1d0], 4
0x1400bc84a mov      byte ptr [rbp + 0x2e7], 0
0x1400bc851 lea      rdx, [rip + 0x28789e8]
0x1400bc858 lea      rcx, [rbp + 0x28]
0x1400bc85c mov      r8d, 3
0x1400bc862 call     0x1400568d0
0x1400bc867 movups   xmm0, xmmword ptr [rbp + 0x88]
0x1400bc86e movaps   xmmword ptr [rbp + 0x10], xmm0
0x1400bc872 movups   xmm0, xmmword ptr [rbp + 0x78]
0x1400bc876 movaps   xmmword ptr [rbp], xmm0
0x1400bc87a movups   xmm0, xmmword ptr [rbp + 0x68]
0x1400bc87e movaps   xmmword ptr [rbp - 0x10], xmm0
0x1400bc882 movups   xmm0, xmmword ptr [rbp + 0x28]
0x1400bc886 movups   xmm1, xmmword ptr [rbp + 0x38]
0x1400bc88a movups   xmm2, xmmword ptr [rbp + 0x48]
0x1400bc88e movups   xmm3, xmmword ptr [rbp + 0x58]
0x1400bc892 movaps   xmmword ptr [rbp - 0x20], xmm3
0x1400bc896 movaps   xmmword ptr [rbp - 0x30], xmm2
0x1400bc89a movaps   xmmword ptr [rbp - 0x40], xmm1
0x1400bc89e movaps   xmmword ptr [rbp - 0x50], xmm0
0x1400bc8a2 mov      byte ptr [rbp + 0x2e7], 1
0x1400bc8a9 lea      rcx, [rbp + 0x28]
0x1400bc8ad lea      rdx, [rbp + 0x1d0]
0x1400bc8b4 lea      rdi, [rbp - 0x50]
0x1400bc8b8 mov      r8, rdi
0x1400bc8bb call     0x140c46bd0
0x1400bc8c0 lea      rcx, [rbp + 0x260]
0x1400bc8c7 lea      rdx, [rbp + 0x28]
0x1400bc8cb mov      r8, r15
0x1400bc8ce call     0x140c46d20
0x1400bc8d3 cmp      byte ptr [rbp + 0x2b4], 0
0x1400bc8da je       0x1400bc999
0x1400bc8e0 mov      rax, qword ptr [rbp + 0x270]
0x1400bc8e7 lock dec qword ptr [rax]
0x1400bc8eb jne      0x1400bc8f9
0x1400bc8ed lea      rcx, [rbp + 0x270]
0x1400bc8f4 call     0x140c41170
0x1400bc8f9 movss    xmm6, dword ptr [rbx]
0x1400bc8fd mov      rdx, qword ptr [r12]
0x1400bc901 lea      rcx, [rbp + 0x1d0]
0x1400bc908 movaps   xmm2, xmm6
0x1400bc90b call     0x140b067d0
0x1400bc910 call     0x140b06790
0x1400bc915 mov      r14, rax
0x1400bc918 mov      eax, dword ptr [r12 + 0x1c]
0x1400bc91d mov      rcx, qword ptr [r12 + 0x10]
0x1400bc922 mov      dword ptr [rcx], eax
0x1400bc924 mov      rbx, qword ptr [r12]
0x1400bc928 mov      rsi, qword ptr [rbx + 0x10]
0x1400bc92c cmp      rsi, qword ptr [rbx]
0x1400bc92f jne      0x1400bc940
0x1400bc931 lea      rdx, [rip + 0x2878910]
0x1400bc938 mov      rcx, rbx
0x1400bc93b call     0x140563540
0x1400bc940 mov      rax, qword ptr [rbx + 8]
0x1400bc944 lea      rcx, [rsi + rsi*4]
0x1400bc948 shl      rcx, 4
0x1400bc94c mov      qword ptr [rax + rcx], r14
0x1400bc950 movss    dword ptr [rax + rcx + 8], xmm6
0x1400bc956 movups   xmm0, xmmword ptr [rbp + 0x1d0]
0x1400bc95d movups   xmm1, xmmword ptr [rbp + 0x1e0]
0x1400bc964 movups   xmm2, xmmword ptr [rbp + 0x1f0]
0x1400bc96b movups   xmm3, xmmword ptr [rbp + 0x200]
0x1400bc972 movups   xmmword ptr [rax + rcx + 0xc], xmm0
0x1400bc977 movups   xmmword ptr [rax + rcx + 0x1c], xmm1
0x1400bc97c movups   xmmword ptr [rax + rcx + 0x2c], xmm2
0x1400bc981 movups   xmmword ptr [rax + rcx + 0x3c], xmm3
0x1400bc986 mov      edx, dword ptr [rbp + 0x210]
0x1400bc98c mov      dword ptr [rax + rcx + 0x4c], edx
0x1400bc990 inc      rsi
0x1400bc993 mov      qword ptr [rbx + 0x10], rsi
0x1400bc997 jmp      0x1400bc9d7
0x1400bc999 mov      byte ptr [rbp + 0x28], 0
0x1400bc99d cmp      byte ptr [rbp + 0x2b3], 0
0x1400bc9a4 je       0x1400bc9be
0x1400bc9a6 lea      rcx, [rbp + 0x270]
0x1400bc9ad lea      rdx, [rbp + 0x28]
0x1400bc9b1 call     0x140042870
0x1400bc9b6 test     al, al
0x1400bc9b8 jne      0x1400bc8e0
0x1400bc9be mov      rax, qword ptr [rbp + 0x270]
0x1400bc9c5 lock dec qword ptr [rax]
0x1400bc9c9 jne      0x1400bc9d7
0x1400bc9cb lea      rcx, [rbp + 0x270]
0x1400bc9d2 call     0x140c41170
0x1400bc9d7 mov      eax, dword ptr [r12 + 0x1c]
0x1400bc9dc mov      rcx, qword ptr [r12 + 0x10]
0x1400bc9e1 mov      dword ptr [rcx], eax
0x1400bc9e3 mov      rax, qword ptr [r12]
0x1400bc9e7 mov      rcx, qword ptr [rax + 8]
0x1400bc9eb mov      rdx, qword ptr [rax + 0x10]
0x1400bc9ef mov      qword ptr [rbp + 0x28], rdi
0x1400bc9f3 cmp      rdx, 2
0x1400bc9f7 jae      0x1400bca77
0x1400bc9f9 mov      eax, dword ptr [r12 + 0x1c]
0x1400bc9fe mov      rcx, qword ptr [r12 + 0x10]
0x1400bca03 mov      dword ptr [rcx], eax
0x1400bca05 mov      rsi, qword ptr [r12]
0x1400bca09 movzx    eax, byte ptr [rip + 0x35781f9]
0x1400bca10 mov      ecx, 8
0x1400bca15 mov      edx, 8
0x1400bca1a call     0x140613c10
0x1400bca1f test     rax, rax
0x1400bca22 je       0x1400bcaaf
0x1400bca28 mov      qword ptr [rax], rsi
0x1400bca2b lea      rcx, [rip + 0x2894586]
0x1400bca32 mov      qword ptr [rsp + 0x20], rcx
0x1400bca37 lea      rcx, [rbp + 0x28]
0x1400bca3b mov      rdx, r15
0x1400bca3e xor      r8d, r8d
0x1400bca41 mov      r9, rax
0x1400bca44 call     0x1400c4230
0x1400bca49 mov      rax, qword ptr [rbp + 0x38]
0x1400bca4d lock dec qword ptr [rax]
0x1400bca51 jne      0x1400bca5c
0x1400bca53 lea      rcx, [rbp + 0x38]
0x1400bca57 call     0x140c41170
0x1400bca5c movaps   xmm6, xmmword ptr [rbp + 0x2f0]
0x1400bca63 add      rsp, 0x388
0x1400bca6a pop      rbx
0x1400bca6b pop      rdi
0x1400bca6c pop      rsi
0x1400bca6d pop      r12
0x1400bca6f pop      r13
0x1400bca71 pop      r14
0x1400bca73 pop      r15
0x1400bca75 pop      rbp
0x1400bca76 ret      
0x1400bca77 cmp      rdx, 0x15
0x1400bca7b jae      0x1400bcabe
0x1400bca7d lea      r9, [rbp + 0x28]
0x1400bca81 mov      r8d, 1
0x1400bca87 call     0x1405c3350
0x1400bca8c jmp      0x1400bc9f9
0x1400bca91 mov      ecx, 8
0x1400bca96 mov      edx, 0x20
0x1400bca9b call     0x1428d9000
0x1400bcaa0 mov      ecx, 8
0x1400bcaa5 mov      edx, 0x10
0x1400bcaaa call     0x1428d9000
0x1400bcaaf mov      ecx, 8
0x1400bcab4 mov      edx, 8