00000001412f2960 push      r15
00000001412f2962 push      r14
00000001412f2964 push      rsi
00000001412f2965 push      rdi
00000001412f2966 push      rbx
00000001412f2967 sub       rsp, 0xd0
00000001412f296e movaps    xmmword ptr [rsp + 0xc0], xmm10
00000001412f2977 movaps    xmmword ptr [rsp + 0xb0], xmm9
00000001412f2980 movaps    xmmword ptr [rsp + 0xa0], xmm8
00000001412f2989 movaps    xmmword ptr [rsp + 0x90], xmm7
00000001412f2991 movaps    xmmword ptr [rsp + 0x80], xmm6
00000001412f2999 mov       rdi, rdx
00000001412f299c mov       rsi, rcx
00000001412f299f lea       rax, [rdx + 0xc8]
00000001412f29a6 mov       qword ptr [rsp + 0x48], rax
00000001412f29ab cmp       qword ptr [rdx], 0
00000001412f29af je        0x1412f2b12
00000001412f29b5 mov       rdx, qword ptr [rdi + 8]
00000001412f29b9 mov       rcx, qword ptr [rdi + 0x20]
00000001412f29bd cmp       rdx, rcx
00000001412f29c0 jb        0x1412f2b12
00000001412f29c6 lea       rbx, [rsp + 0x54]
00000001412f29cb lea       r14, [rsp + 0x48]
00000001412f29d0 lea       r15, [rsp + 0x20]
00000001412f29d5 nop       word ptr cs:[rax + rax]
00000001412f29e0 mov       rax, qword ptr [rdi]
00000001412f29e3 mov       r8, qword ptr [rdi + 0x28]
00000001412f29e7 lea       r9, [rcx + rcx*2]
00000001412f29eb lea       r9, [rax + r9*4]
00000001412f29ef sub       rdx, rcx
00000001412f29f2 mov       qword ptr [rdi], r9
00000001412f29f5 mov       qword ptr [rdi + 8], rdx
00000001412f29f9 test      rcx, rcx
00000001412f29fc je        0x1412f2d6d
00000001412f2a02 cmp       rcx, 1
00000001412f2a06 je        0x1412f2d57
00000001412f2a0c cmp       rcx, 2
00000001412f2a10 jbe       0x1412f2d41
00000001412f2a16 movss     xmm8, dword ptr [rax]
00000001412f2a1b movss     xmm7, dword ptr [rax + 4]
00000001412f2a20 movss     xmm0, dword ptr [rax + 8]
00000001412f2a25 movaps    xmm5, xmmword ptr [r8]
00000001412f2a29 movaps    xmm4, xmmword ptr [r8 + 0x10]
00000001412f2a2e movaps    xmm3, xmmword ptr [r8 + 0x20]
00000001412f2a33 movaps    xmm2, xmmword ptr [r8 + 0x30]
00000001412f2a38 movss     xmm9, dword ptr [rax + 0xc]
00000001412f2a3e movss     xmm10, dword ptr [rax + 0x10]
00000001412f2a44 movss     xmm1, dword ptr [rax + 0x14]
00000001412f2a49 movss     xmm6, dword ptr [rax + 0x18]
00000001412f2a4e shufps    xmm9, xmm9, 0
00000001412f2a53 mulps     xmm9, xmm5
00000001412f2a57 shufps    xmm10, xmm10, 0
00000001412f2a5c mulps     xmm10, xmm4
00000001412f2a60 addps     xmm10, xmm9
00000001412f2a64 shufps    xmm1, xmm1, 0
00000001412f2a68 mulps     xmm1, xmm3
00000001412f2a6b addps     xmm1, xmm10
00000001412f2a6f addps     xmm1, xmm2
00000001412f2a72 shufps    xmm8, xmm8, 0
00000001412f2a77 mulps     xmm8, xmm5
00000001412f2a7b shufps    xmm7, xmm7, 0
00000001412f2a7f mulps     xmm7, xmm4
00000001412f2a82 addps     xmm7, xmm8
00000001412f2a86 shufps    xmm0, xmm0, 0
00000001412f2a8a mulps     xmm0, xmm3
00000001412f2a8d addps     xmm0, xmm7
00000001412f2a90 addps     xmm0, xmm2
00000001412f2a93 shufps    xmm6, xmm6, 0
00000001412f2a97 mulps     xmm6, xmm5
00000001412f2a9a movss     xmm5, dword ptr [rax + 0x1c]
00000001412f2a9f shufps    xmm5, xmm5, 0
00000001412f2aa3 mulps     xmm5, xmm4
00000001412f2aa6 addps     xmm5, xmm6
00000001412f2aa9 movss     xmm4, dword ptr [rax + 0x20]
00000001412f2aae shufps    xmm4, xmm4, 0
00000001412f2ab2 mulps     xmm4, xmm3
00000001412f2ab5 addps     xmm4, xmm5
00000001412f2ab8 addps     xmm4, xmm2
00000001412f2abb mov       eax, dword ptr [rdi + 0x48]
00000001412f2abe movlps    qword ptr [rsp + 0x20], xmm0
00000001412f2ac3 movhlps   xmm0, xmm0
00000001412f2ac6 movss     dword ptr [rsp + 0x28], xmm0
00000001412f2acc movlps    qword ptr [rsp + 0x2c], xmm1
00000001412f2ad1 movhlps   xmm1, xmm1
00000001412f2ad4 movss     dword ptr [rsp + 0x34], xmm1
00000001412f2ada movlps    qword ptr [rsp + 0x38], xmm4
00000001412f2adf movhlps   xmm4, xmm4
00000001412f2ae2 movss     dword ptr [rsp + 0x40], xmm4
00000001412f2ae8 mov       dword ptr [rsp + 0x44], eax
00000001412f2aec mov       rcx, rbx
00000001412f2aef mov       rdx, r14
00000001412f2af2 mov       r8, r15
00000001412f2af5 call      0x141233e50
00000001412f2afa test      byte ptr [rsp + 0x54], 1
00000001412f2aff jne       0x1412f2b43
00000001412f2b01 mov       rdx, qword ptr [rdi + 8]
00000001412f2b05 mov       rcx, qword ptr [rdi + 0x20]
00000001412f2b09 cmp       rdx, rcx
00000001412f2b0c jae       0x1412f29e0
00000001412f2b12 mov       qword ptr [rdi], 0
00000001412f2b19 cmp       qword ptr [rdi + 0xa0], 0
00000001412f2b21 je        0x1412f2b86
00000001412f2b23 lea       rdx, [rdi + 0xa0]
00000001412f2b2a lea       rcx, [rsp + 0x54]
00000001412f2b2f lea       r8, [rsp + 0x48]
00000001412f2b34 mov       r9, rdi
00000001412f2b37 call      0x141479840
00000001412f2b3c test      byte ptr [rsp + 0x54], 1
00000001412f2b41 je        0x1412f2b86
00000001412f2b43 mov       rax, qword ptr [rsp + 0x78]
00000001412f2b48 mov       qword ptr [rsp + 0x40], rax
00000001412f2b4d movups    xmm0, xmmword ptr [rsp + 0x58]
00000001412f2b52 movups    xmm1, xmmword ptr [rsp + 0x68]
00000001412f2b57 movaps    xmmword ptr [rsp + 0x30], xmm1
00000001412f2b5c movaps    xmmword ptr [rsp + 0x20], xmm0
00000001412f2b61 mov       rax, qword ptr [rsp + 0x40]
00000001412f2b66 mov       qword ptr [rsi + 0x24], rax
00000001412f2b6a movaps    xmm0, xmmword ptr [rsp + 0x20]
00000001412f2b6f movaps    xmm1, xmmword ptr [rsp + 0x30]
00000001412f2b74 movups    xmmword ptr [rsi + 0x14], xmm1
00000001412f2b78 movups    xmmword ptr [rsi + 4], xmm0
00000001412f2b7c mov       eax, 1
00000001412f2b81 jmp       0x1412f2d05
00000001412f2b86 mov       qword ptr [rdi], 0
00000001412f2b8d cmp       qword ptr [rdi + 0x50], 0
00000001412f2b92 je        0x1412f2cfb
00000001412f2b98 mov       rdx, qword ptr [rdi + 0x58]
00000001412f2b9c mov       rcx, qword ptr [rdi + 0x70]
00000001412f2ba0 cmp       rdx, rcx
00000001412f2ba3 jb        0x1412f2cfb
00000001412f2ba9 lea       rbx, [rsp + 0x54]
00000001412f2bae lea       r14, [rsp + 0x48]
00000001412f2bb3 lea       r15, [rsp + 0x20]
00000001412f2bb8 nop       dword ptr [rax + rax]
00000001412f2bc0 mov       rax, qword ptr [rdi + 0x50]
00000001412f2bc4 mov       r8, qword ptr [rdi + 0x78]
00000001412f2bc8 lea       r9, [rcx + rcx*2]
00000001412f2bcc lea       r9, [rax + r9*4]
00000001412f2bd0 sub       rdx, rcx
00000001412f2bd3 mov       qword ptr [rdi + 0x50], r9
00000001412f2bd7 mov       qword ptr [rdi + 0x58], rdx
00000001412f2bdb test      rcx, rcx
00000001412f2bde je        0x1412f2d6d
00000001412f2be4 cmp       rcx, 1
00000001412f2be8 je        0x1412f2d57
00000001412f2bee cmp       rcx, 2
00000001412f2bf2 jbe       0x1412f2d41
00000001412f2bf8 movss     xmm8, dword ptr [rax]
00000001412f2bfd movss     xmm7, dword ptr [rax + 4]
00000001412f2c02 movss     xmm0, dword ptr [rax + 8]
00000001412f2c07 movaps    xmm5, xmmword ptr [r8]
00000001412f2c0b movaps    xmm4, xmmword ptr [r8 + 0x10]
00000001412f2c10 movaps    xmm3, xmmword ptr [r8 + 0x20]
00000001412f2c15 movaps    xmm2, xmmword ptr [r8 + 0x30]
00000001412f2c1a movss     xmm9, dword ptr [rax + 0xc]
00000001412f2c20 movss     xmm10, dword ptr [rax + 0x10]
00000001412f2c26 movss     xmm1, dword ptr [rax + 0x14]
00000001412f2c2b movss     xmm6, dword ptr [rax + 0x18]
00000001412f2c30 shufps    xmm9, xmm9, 0
00000001412f2c35 mulps     xmm9, xmm5
00000001412f2c39 shufps    xmm10, xmm10, 0
00000001412f2c3e mulps     xmm10, xmm4
00000001412f2c42 addps     xmm10, xmm9
00000001412f2c46 shufps    xmm1, xmm1, 0
00000001412f2c4a mulps     xmm1, xmm3
00000001412f2c4d addps     xmm1, xmm10
00000001412f2c51 addps     xmm1, xmm2
00000001412f2c54 shufps    xmm8, xmm8, 0
00000001412f2c59 mulps     xmm8, xmm5
00000001412f2c5d shufps    xmm7, xmm7, 0
00000001412f2c61 mulps     xmm7, xmm4
00000001412f2c64 addps     xmm7, xmm8
00000001412f2c68 shufps    xmm0, xmm0, 0
00000001412f2c6c mulps     xmm0, xmm3
00000001412f2c6f addps     xmm0, xmm7
00000001412f2c72 addps     xmm0, xmm2
00000001412f2c75 shufps    xmm6, xmm6, 0
00000001412f2c79 mulps     xmm6, xmm5
00000001412f2c7c movss     xmm5, dword ptr [rax + 0x1c]
00000001412f2c81 shufps    xmm5, xmm5, 0
00000001412f2c85 mulps     xmm5, xmm4
00000001412f2c88 addps     xmm5, xmm6
00000001412f2c8b movss     xmm4, dword ptr [rax + 0x20]
00000001412f2c90 shufps    xmm4, xmm4, 0
00000001412f2c94 mulps     xmm4, xmm3
00000001412f2c97 addps     xmm4, xmm5
00000001412f2c9a addps     xmm4, xmm2
00000001412f2c9d mov       eax, dword ptr [rdi + 0x98]
00000001412f2ca3 movlps    qword ptr [rsp + 0x20], xmm0
00000001412f2ca8 movhlps   xmm0, xmm0
00000001412f2cab movss     dword ptr [rsp + 0x28], xmm0
00000001412f2cb1 movlps    qword ptr [rsp + 0x2c], xmm1
00000001412f2cb6 movhlps   xmm1, xmm1
00000001412f2cb9 movss     dword ptr [rsp + 0x34], xmm1
00000001412f2cbf movlps    qword ptr [rsp + 0x38], xmm4
00000001412f2cc4 movhlps   xmm4, xmm4
00000001412f2cc7 movss     dword ptr [rsp + 0x40], xmm4
00000001412f2ccd mov       dword ptr [rsp + 0x44], eax
00000001412f2cd1 mov       rcx, rbx
00000001412f2cd4 mov       rdx, r14
00000001412f2cd7 mov       r8, r15
00000001412f2cda call      0x141233e50
00000001412f2cdf test      byte ptr [rsp + 0x54], 1
00000001412f2ce4 jne       0x1412f2b43
00000001412f2cea mov       rdx, qword ptr [rdi + 0x58]
00000001412f2cee mov       rcx, qword ptr [rdi + 0x70]
00000001412f2cf2 cmp       rdx, rcx
00000001412f2cf5 jae       0x1412f2bc0
00000001412f2cfb mov       qword ptr [rdi + 0x50], 0
00000001412f2d03 xor       eax, eax
00000001412f2d05 mov       dword ptr [rsi], eax
00000001412f2d07 movaps    xmm6, xmmword ptr [rsp + 0x80]
00000001412f2d0f movaps    xmm7, xmmword ptr [rsp + 0x90]
00000001412f2d17 movaps    xmm8, xmmword ptr [rsp + 0xa0]
00000001412f2d20 movaps    xmm9, xmmword ptr [rsp + 0xb0]
00000001412f2d29 movaps    xmm10, xmmword ptr [rsp + 0xc0]
00000001412f2d32 add       rsp, 0xd0
00000001412f2d39 pop       rbx
00000001412f2d3a pop       rdi
00000001412f2d3b pop       rsi
00000001412f2d3c pop       r14
00000001412f2d3e pop       r15
00000001412f2d40 ret       
00000001412f2d41 lea       r8, [rip + 0x192bb38]
00000001412f2d48 mov       ecx, 2
00000001412f2d4d mov       edx, 2
00000001412f2d52 call      0x1428d9518
00000001412f2d57 lea       r8, [rip + 0x192bb0a]
00000001412f2d5e mov       ecx, 1
00000001412f2d63 mov       edx, 1
00000001412f2d68 call      0x1428d9518
00000001412f2d6d lea       r8, [rip + 0x192badc]
00000001412f2d74 xor       ecx, ecx
00000001412f2d76 xor       edx, edx
00000001412f2d78 call      0x1428d9518
00000001412f2d7d int3      
