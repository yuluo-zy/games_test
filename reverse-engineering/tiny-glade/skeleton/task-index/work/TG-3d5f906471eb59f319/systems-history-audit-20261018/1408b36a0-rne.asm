0x1408b36a0 push     r15
0x1408b36a2 push     r14
0x1408b36a4 push     r13
0x1408b36a6 push     r12
0x1408b36a8 push     rsi
0x1408b36a9 push     rdi
0x1408b36aa push     rbp
0x1408b36ab push     rbx
0x1408b36ac sub      rsp, 0xb8
0x1408b36b3 mov      rax, qword ptr [rcx]
0x1408b36b6 cmp      dword ptr [rax], 0xd
0x1408b36b9 jne      0x1408b385f
0x1408b36bf mov      qword ptr [rsp + 0xb0], rax
0x1408b36c7 mov      rax, qword ptr [rsp + 0x120]
0x1408b36cf mov      rdi, qword ptr [rax]
0x1408b36d2 mov      rax, qword ptr [rdi + 0x18]
0x1408b36d6 cmp      rax, 1
0x1408b36da jne      0x1408b3770
0x1408b36e0 mov      rax, qword ptr [rdi + 0x10]
0x1408b36e4 mov      r10, qword ptr [rdi]
0x1408b36e7 xor      r11d, r11d
0x1408b36ea cmp      rax, r10
0x1408b36ed cmovae   r11, r10
0x1408b36f1 mov      r10, qword ptr [rdi + 8]
0x1408b36f5 sub      rax, r11
0x1408b36f8 cmp      byte ptr [r10 + rax*4], 1
0x1408b36fd jne      0x1408b378b
0x1408b3703 mov      rax, qword ptr [rsp + 0x140]
0x1408b370b mov      r10d, dword ptr [rax + 0x1c]
0x1408b370f mov      r11, qword ptr [rax + 0x10]
0x1408b3713 mov      dword ptr [r11], r10d
0x1408b3716 mov      rbx, qword ptr [rax]
0x1408b3719 mov      r14, qword ptr [rbx + 0x30]
0x1408b371d mov      r15, qword ptr [rbx + 0x40]
0x1408b3721 cmp      r14, qword ptr [rbx + 0x20]
0x1408b3725 jne      0x1408b3759
0x1408b3727 lea      rax, [rbx + 0x20]
0x1408b372b lea      r10, [rip + 0x2386a16]
0x1408b3732 mov      qword ptr [rsp + 0x28], rcx
0x1408b3737 mov      rcx, rax
0x1408b373a mov      r13, rdx
0x1408b373d mov      rdx, r10
0x1408b3740 mov      rbp, r9
0x1408b3743 mov      r12, r8
0x1408b3746 call     0x140562870
0x1408b374b mov      rdx, r13
0x1408b374e mov      rcx, qword ptr [rsp + 0x28]
0x1408b3753 mov      r8, r12
0x1408b3756 mov      r9, rbp
0x1408b3759 mov      rax, qword ptr [rbx + 0x28]
0x1408b375d mov      qword ptr [rax + r14*8], r15
0x1408b3761 inc      r14
0x1408b3764 mov      qword ptr [rbx + 0x30], r14
0x1408b3768 inc      qword ptr [rbx + 0x40]
0x1408b376c mov      rax, qword ptr [rdi + 0x18]
0x1408b3770 test     rax, rax
0x1408b3773 jne      0x1408b378b
0x1408b3775 mov      rax, qword ptr [rsp + 0x128]
0x1408b377d mov      rax, qword ptr [rax]
0x1408b3780 cmp      qword ptr [rax + 0x18], 0
0x1408b3785 je       0x1408b38f0
0x1408b378b mov      rax, qword ptr [rdx]
0x1408b378e mov      r13, qword ptr [rdx + 8]
0x1408b3792 mov      rdx, qword ptr [rax]
0x1408b3795 xor      ebp, ebp
0x1408b3797 mov      r14, rdx
0x1408b379a sub      r14, qword ptr [r13 + 0x18]
0x1408b379e cmovb    r14, rbp
0x1408b37a2 sub      rdx, qword ptr [r13 + 0x38]
0x1408b37a6 cmovb    rdx, rbp
0x1408b37aa mov      rdi, r14
0x1408b37ad shl      rdi, 4
0x1408b37b1 add      rdi, qword ptr [r13 + 8]
0x1408b37b5 mov      r15, qword ptr [r13 + 0x10]
0x1408b37b9 mov      r10, rdx
0x1408b37bc shl      r10, 4
0x1408b37c0 add      r10, qword ptr [r13 + 0x28]
0x1408b37c4 mov      r11, r15
0x1408b37c7 sub      r11, r14
0x1408b37ca cmovb    r11, rbp
0x1408b37ce mov      r12, qword ptr [r13 + 0x30]
0x1408b37d2 mov      esi, 8
0x1408b37d7 cmovb    rdi, rsi
0x1408b37db cmovb    r14, r15
0x1408b37df mov      rbx, r12
0x1408b37e2 sub      rbx, rdx
0x1408b37e5 cmovb    rbx, rbp
0x1408b37e9 cmovb    r10, rsi
0x1408b37ed mov      rsi, qword ptr [r13 + 0x40]
0x1408b37f1 cmovb    rdx, r12
0x1408b37f5 add      rdx, rsi
0x1408b37f8 sub      rsi, rbx
0x1408b37fb sub      rsi, r11
0x1408b37fe mov      qword ptr [rax], rsi
0x1408b3801 shl      r11, 4
0x1408b3805 add      r11, rdi
0x1408b3808 shl      rbx, 4
0x1408b380c add      rbx, r10
0x1408b380f add      rdx, r14
0x1408b3812 sub      rdx, r12
0x1408b3815 sub      rdx, r15
0x1408b3818 inc      rdx
0x1408b381b jmp      0x1408b3837
0x1408b381d nop      dword ptr [rax]
0x1408b3820 lea      r14, [rdi + 0x10]
0x1408b3824 mov      qword ptr [rax], rdx
0x1408b3827 inc      rdx
0x1408b382a cmp      byte ptr [rdi + 9], 3
0x1408b382e mov      rdi, r14
0x1408b3831 je       0x1408b3abe
0x1408b3837 test     rdi, rdi
0x1408b383a je       0x1408b3841
0x1408b383c cmp      rdi, r11
0x1408b383f jne      0x1408b3820
0x1408b3841 test     r10, r10
0x1408b3844 je       0x1408b3cf1
0x1408b384a cmp      r10, rbx
0x1408b384d je       0x1408b3cf1
0x1408b3853 mov      rdi, r10
0x1408b3856 add      r10, 0x10
0x1408b385a xor      r14d, r14d
0x1408b385d jmp      0x1408b3824
0x1408b385f mov      rsi, qword ptr [r9]
0x1408b3862 cmp      qword ptr [rsi + 0x18], 0
0x1408b3867 je       0x1408b3cf1
0x1408b386d mov      rax, qword ptr [rsi + 0x10]
0x1408b3871 mov      rcx, qword ptr [rsi]
0x1408b3874 mov      rdx, qword ptr [rsi + 8]
0x1408b3878 xor      r8d, r8d
0x1408b387b cmp      rax, rcx
0x1408b387e cmovae   r8, rcx
0x1408b3882 sub      rax, r8
0x1408b3885 imul     rax, rax, 0x38
0x1408b3889 movabs   rcx, 0x8000000000000001
0x1408b3893 cmp      qword ptr [rdx + rax], rcx
0x1408b3897 jne      0x1408b3cf1
0x1408b389d mov      rax, qword ptr [rip + 0x2d78b04]
0x1408b38a4 mov      rax, qword ptr [rax]
0x1408b38a7 cmp      rax, 3
0x1408b38ab jbe      0x1408b3c67
0x1408b38b1 lea      rcx, [rip + 0x21eb9f0]
0x1408b38b8 mov      rdi, r9
0x1408b38bb call     0x14067dd80
0x1408b38c0 movups   xmm0, xmmword ptr [rax]
0x1408b38c3 mov      eax, dword ptr [rax + 0x10]
0x1408b38c6 mov      qword ptr [rsp + 0x60], 4
0x1408b38cf lea      rcx, [rip + 0x21eb95a]
0x1408b38d6 mov      qword ptr [rsp + 0x68], rcx
0x1408b38db mov      qword ptr [rsp + 0x70], 0x2d
0x1408b38e4 lea      rdx, [rip + 0x21eb9ad]
0x1408b38eb jmp      0x1408b3bf9
0x1408b38f0 mov      rax, qword ptr [rsp + 0x130]
0x1408b38f8 mov      r11, qword ptr [rax]
0x1408b38fb cmp      dword ptr [r11 + 0x80], 3
0x1408b3903 jae      0x1408b39df
0x1408b3909 mov      rax, qword ptr [rdx]
0x1408b390c mov      r13, qword ptr [rdx + 8]
0x1408b3910 mov      rdx, qword ptr [rax]
0x1408b3913 xor      ebp, ebp
0x1408b3915 mov      r14, rdx
0x1408b3918 sub      r14, qword ptr [r13 + 0x18]
0x1408b391c cmovb    r14, rbp
0x1408b3920 sub      rdx, qword ptr [r13 + 0x38]
0x1408b3924 cmovb    rdx, rbp
0x1408b3928 mov      rdi, r14
0x1408b392b shl      rdi, 4
0x1408b392f add      rdi, qword ptr [r13 + 8]
0x1408b3933 mov      r15, qword ptr [r13 + 0x10]
0x1408b3937 mov      r10, rdx
0x1408b393a shl      r10, 4
0x1408b393e add      r10, qword ptr [r13 + 0x28]
0x1408b3942 mov      r11, r15
0x1408b3945 sub      r11, r14
0x1408b3948 cmovb    r11, rbp
0x1408b394c mov      r12, qword ptr [r13 + 0x30]
0x1408b3950 mov      esi, 8
0x1408b3955 cmovb    rdi, rsi
0x1408b3959 cmovb    r14, r15
0x1408b395d mov      rbx, r12
0x1408b3960 sub      rbx, rdx
0x1408b3963 cmovb    rbx, rbp
0x1408b3967 cmovb    r10, rsi
0x1408b396b mov      rsi, qword ptr [r13 + 0x40]
0x1408b396f cmovb    rdx, r12
0x1408b3973 add      rdx, rsi
0x1408b3976 sub      rsi, rbx
0x1408b3979 sub      rsi, r11
0x1408b397c mov      qword ptr [rax], rsi
0x1408b397f shl      r11, 4
0x1408b3983 add      r11, rdi
0x1408b3986 shl      rbx, 4
0x1408b398a add      rbx, r10
0x1408b398d add      rdx, r14
0x1408b3990 sub      rdx, r12
0x1408b3993 sub      rdx, r15
0x1408b3996 inc      rdx
0x1408b3999 jmp      0x1408b39b7
0x1408b399b nop      dword ptr [rax + rax]
0x1408b39a0 lea      r14, [rdi + 0x10]
0x1408b39a4 mov      qword ptr [rax], rdx
0x1408b39a7 inc      rdx
0x1408b39aa cmp      byte ptr [rdi + 9], 3
0x1408b39ae mov      rdi, r14
0x1408b39b1 je       0x1408b3abe
0x1408b39b7 test     rdi, rdi
0x1408b39ba je       0x1408b39c1
0x1408b39bc cmp      rdi, r11
0x1408b39bf jne      0x1408b39a0
0x1408b39c1 test     r10, r10
0x1408b39c4 je       0x1408b3cf1
0x1408b39ca cmp      r10, rbx
0x1408b39cd je       0x1408b3cf1
0x1408b39d3 mov      rdi, r10
0x1408b39d6 add      r10, 0x10
0x1408b39da xor      r14d, r14d
0x1408b39dd jmp      0x1408b39a4
0x1408b39df mov      rax, qword ptr [rdx]
0x1408b39e2 mov      r13, qword ptr [rdx + 8]
0x1408b39e6 mov      rdx, qword ptr [rax]
0x1408b39e9 xor      r10d, r10d
0x1408b39ec mov      r15, rdx
0x1408b39ef sub      r15, qword ptr [r13 + 0x18]
0x1408b39f3 cmovb    r15, r10
0x1408b39f7 sub      rdx, qword ptr [r13 + 0x38]
0x1408b39fb mov      r11d, dword ptr [r11 + 0x88]
0x1408b3a02 mov      dword ptr [rsp + 0x28], r11d
0x1408b3a07 cmovb    rdx, r10
0x1408b3a0b mov      rbx, r15
0x1408b3a0e shl      rbx, 4
0x1408b3a12 add      rbx, qword ptr [r13 + 8]
0x1408b3a16 mov      r12, qword ptr [r13 + 0x10]
0x1408b3a1a mov      r11, rdx
0x1408b3a1d shl      r11, 4
0x1408b3a21 add      r11, qword ptr [r13 + 0x28]
0x1408b3a25 mov      rdi, r12
0x1408b3a28 sub      rdi, r15
0x1408b3a2b cmovb    rdi, r10
0x1408b3a2f mov      rbp, qword ptr [r13 + 0x30]
0x1408b3a33 mov      esi, 8
0x1408b3a38 cmovb    rbx, rsi
0x1408b3a3c cmovb    r15, r12
0x1408b3a40 mov      r14, rbp
0x1408b3a43 sub      r14, rdx
0x1408b3a46 cmovb    r14, r10
0x1408b3a4a cmovb    r11, rsi
0x1408b3a4e mov      r10, qword ptr [r13 + 0x40]
0x1408b3a52 cmovb    rdx, rbp
0x1408b3a56 add      rdx, r10
0x1408b3a59 sub      r10, r14
0x1408b3a5c sub      r10, rdi
0x1408b3a5f mov      qword ptr [rax], r10
0x1408b3a62 shl      rdi, 4
0x1408b3a66 add      rdi, rbx
0x1408b3a69 shl      r14, 4
0x1408b3a6d add      r14, r11
0x1408b3a70 add      rdx, r15
0x1408b3a73 sub      rdx, rbp
0x1408b3a76 sub      rdx, r12
0x1408b3a79 inc      rdx
0x1408b3a7c jmp      0x1408b3a93
0x1408b3a7e nop      
0x1408b3a80 lea      r10, [rbx + 0x10]
0x1408b3a84 mov      qword ptr [rax], rdx
0x1408b3a87 inc      rdx
0x1408b3a8a cmp      byte ptr [rbx + 9], 3
0x1408b3a8e mov      rbx, r10
0x1408b3a91 je       0x1408b3abe
0x1408b3a93 test     rbx, rbx
0x1408b3a96 je       0x1408b3a9d
0x1408b3a98 cmp      rbx, rdi
0x1408b3a9b jne      0x1408b3a80
0x1408b3a9d test     r11, r11
0x1408b3aa0 je       0x1408b3ab3
0x1408b3aa2 cmp      r11, r14
0x1408b3aa5 je       0x1408b3ab3
0x1408b3aa7 mov      rbx, r11
0x1408b3aaa add      r11, 0x10
0x1408b3aae xor      r10d, r10d
0x1408b3ab1 jmp      0x1408b3a84
0x1408b3ab3 cmp      dword ptr [rsp + 0x28], 3
0x1408b3ab8 jb       0x1408b3cf1
0x1408b3abe mov      rax, qword ptr [rsp + 0x138]
0x1408b3ac6 mov      edx, dword ptr [rcx + 0x1c]
0x1408b3ac9 mov      rcx, qword ptr [rcx + 0x10]
0x1408b3acd mov      dword ptr [rcx], edx
0x1408b3acf mov      rax, qword ptr [rax]
0x1408b3ad2 movups   xmm0, xmmword ptr [rax]
0x1408b3ad5 movaps   xmmword ptr [rsp + 0x30], xmm0
0x1408b3ada mov      eax, dword ptr [r8 + 0x1c]
0x1408b3ade mov      rcx, qword ptr [r8 + 0x10]
0x1408b3ae2 mov      dword ptr [rcx], eax
0x1408b3ae4 mov      rcx, qword ptr [r8]
0x1408b3ae7 movaps   xmm0, xmmword ptr [rsp + 0x30]
0x1408b3aec mov      rax, qword ptr [rsp + 0xb0]
0x1408b3af4 movups   xmmword ptr [rax], xmm0
0x1408b3af7 mov      edx, dword ptr [rsp + 0x30]
0x1408b3afb lea      r8d, [rdx - 6]
0x1408b3aff add      rdx, -5
0x1408b3b03 xor      eax, eax
0x1408b3b05 cmp      r8d, 0xf
0x1408b3b09 cmovb    rax, rdx
0x1408b3b0d cmp      rax, 0xf
0x1408b3b11 je       0x1408b3b18
0x1408b3b13 test     rax, rax
0x1408b3b16 jne      0x1408b3b5d
0x1408b3b18 cmp      qword ptr [rcx + 0x10], 8
0x1408b3b1d jb       0x1408b3b6d
0x1408b3b1f mov      r8, qword ptr [rcx + 8]
0x1408b3b23 mov      r10, qword ptr [r8 + 0xf0]
0x1408b3b2a test     r10, r10
0x1408b3b2d je       0x1408b3b6d
0x1408b3b2f cmp      byte ptr [rcx + 0x18], 7
0x1408b3b33 jne      0x1408b3d06
0x1408b3b39 mov      rax, qword ptr [r8 + 0xf8]
0x1408b3b40 inc      rax
0x1408b3b43 mov      rdx, rax
0x1408b3b46 or       rdx, r10
0x1408b3b49 shr      rdx, 0x20
0x1408b3b4d je       0x1408b3d24
0x1408b3b53 xor      edx, edx
0x1408b3b55 div      r10
0x1408b3b58 jmp      0x1408b3d29
0x1408b3b5d lea      rdx, [rsp + 0x30]
0x1408b3b62 mov      rsi, r9
0x1408b3b65 call     0x140af9850
0x1408b3b6a mov      r9, rsi
0x1408b3b6d mov      rsi, qword ptr [r9]
0x1408b3b70 cmp      qword ptr [rsi + 0x18], 0
0x1408b3b75 je       0x1408b3cf1
0x1408b3b7b mov      rax, qword ptr [rsi + 0x10]
0x1408b3b7f mov      rcx, qword ptr [rsi]
0x1408b3b82 mov      rdx, qword ptr [rsi + 8]
0x1408b3b86 xor      r8d, r8d
0x1408b3b89 cmp      rax, rcx
0x1408b3b8c cmovae   r8, rcx
0x1408b3b90 sub      rax, r8
0x1408b3b93 imul     rax, rax, 0x38
0x1408b3b97 movabs   rcx, 0x8000000000000001
0x1408b3ba1 cmp      qword ptr [rdx + rax], rcx
0x1408b3ba5 jne      0x1408b3cf1
0x1408b3bab mov      rax, qword ptr [rip + 0x2d787f6]
0x1408b3bb2 mov      rax, qword ptr [rax]
0x1408b3bb5 cmp      rax, 3
0x1408b3bb9 jbe      0x1408b3c67
0x1408b3bbf lea      rcx, [rip + 0x21eb652]
0x1408b3bc6 mov      rdi, r9
0x1408b3bc9 call     0x14067dd80
0x1408b3bce movups   xmm0, xmmword ptr [rax]
0x1408b3bd1 mov      eax, dword ptr [rax + 0x10]
0x1408b3bd4 mov      qword ptr [rsp + 0x60], 4
0x1408b3bdd lea      rcx, [rip + 0x21eb64c]
0x1408b3be4 mov      qword ptr [rsp + 0x68], rcx
0x1408b3be9 mov      qword ptr [rsp + 0x70], 0x2d
0x1408b3bf2 lea      rdx, [rip + 0x21eb60f]
0x1408b3bf9 mov      qword ptr [rsp + 0x80], rdx
0x1408b3c01 mov      qword ptr [rsp + 0x88], 1
0x1408b3c0d mov      qword ptr [rsp + 0x90], 8
0x1408b3c19 mov      qword ptr [rsp + 0x30], 0
0x1408b3c22 xorps    xmm1, xmm1
0x1408b3c25 movups   xmmword ptr [rsp + 0x98], xmm1
0x1408b3c2d mov      qword ptr [rsp + 0x38], rcx
0x1408b3c32 mov      qword ptr [rsp + 0x40], 0x2d
0x1408b3c3b mov      qword ptr [rsp + 0x48], 0
0x1408b3c44 movups   xmmword ptr [rsp + 0x50], xmm0
0x1408b3c49 mov      dword ptr [rsp + 0x78], 1
0x1408b3c51 mov      dword ptr [rsp + 0x7c], eax
0x1408b3c55 lea      rcx, [rsp + 0x27]
0x1408b3c5a lea      rdx, [rsp + 0x30]
0x1408b3c5f call     0x140d4a330
0x1408b3c64 mov      r9, rdi
0x1408b3c67 mov      eax, dword ptr [r9 + 0x1c]
0x1408b3c6b mov      rcx, qword ptr [r9 + 0x10]
0x1408b3c6f mov      dword ptr [rcx], eax
0x1408b3c71 mov      rax, qword ptr [rsi + 0x18]
0x1408b3c75 test     rax, rax
0x1408b3c78 je       0x1408b3cd8
0x1408b3c7a mov      rcx, qword ptr [rsi + 0x10]
0x1408b3c7e lea      rdx, [rcx + 1]
0x1408b3c82 mov      r8, qword ptr [rsi]
0x1408b3c85 mov      r9, qword ptr [rsi + 8]
0x1408b3c89 xor      r10d, r10d
0x1408b3c8c cmp      rdx, r8
0x1408b3c8f cmovae   r10, r8
0x1408b3c93 neg      r10
0x1408b3c96 lea      rdx, [rcx + r10]
0x1408b3c9a inc      rdx
0x1408b3c9d mov      qword ptr [rsi + 0x10], rdx
0x1408b3ca1 dec      rax
0x1408b3ca4 mov      qword ptr [rsi + 0x18], rax
0x1408b3ca8 imul     rax, rcx, 0x38
0x1408b3cac movups   xmm0, xmmword ptr [r9 + rax]
0x1408b3cb1 movups   xmm1, xmmword ptr [r9 + rax + 0x10]
0x1408b3cb7 movups   xmm2, xmmword ptr [r9 + rax + 0x20]
0x1408b3cbd movaps   xmmword ptr [rsp + 0x30], xmm0
0x1408b3cc2 mov      rax, qword ptr [r9 + rax + 0x30]
0x1408b3cc7 mov      qword ptr [rsp + 0x60], rax
0x1408b3ccc movaps   xmmword ptr [rsp + 0x50], xmm2
0x1408b3cd1 movaps   xmmword ptr [rsp + 0x40], xmm1
0x1408b3cd6 jmp      0x1408b3ce7
0x1408b3cd8 movabs   rax, 0x8000000000000014
0x1408b3ce2 mov      qword ptr [rsp + 0x30], rax
0x1408b3ce7 lea      rcx, [rsp + 0x30]
0x1408b3cec call     0x1408b2270
0x1408b3cf1 nop      
0x1408b3cf2 add      rsp, 0xb8
0x1408b3cf9 pop      rbx
0x1408b3cfa pop      rbp
0x1408b3cfb pop      rdi
0x1408b3cfc pop      rsi
0x1408b3cfd pop      r12
0x1408b3cff pop      r13
0x1408b3d01 pop      r14
0x1408b3d03 pop      r15
0x1408b3d05 ret      
0x1408b3d06 mov      rax, qword ptr [r8 + 0xf8]
0x1408b3d0d cmp      rax, r10
0x1408b3d10 jb       0x1408b3d30
0x1408b3d12 lea      r8, [rip + 0x223191f]
0x1408b3d19 mov      rcx, rax
0x1408b3d1c mov      rdx, r10
0x1408b3d1f call     0x1428d9518
0x1408b3d24 xor      edx, edx
0x1408b3d26 div      r10d
0x1408b3d29 mov      qword ptr [r8 + 0xf8], rdx
0x1408b3d30 mov      byte ptr [rcx + 0x18], 7
0x1408b3d34 mov      rsi, qword ptr [r9]
