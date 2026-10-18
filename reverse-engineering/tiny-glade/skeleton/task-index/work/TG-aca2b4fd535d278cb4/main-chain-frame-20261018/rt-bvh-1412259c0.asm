00000001412259c0 push      rbp
00000001412259c1 push      r15
00000001412259c3 push      r14
00000001412259c5 push      r13
00000001412259c7 push      r12
00000001412259c9 push      rsi
00000001412259ca push      rdi
00000001412259cb push      rbx
00000001412259cc sub       rsp, 0x68
00000001412259d0 lea       rbp, [rsp + 0x60]
00000001412259d5 mov       qword ptr [rbp], 0xfffffffffffffffe
00000001412259dd mov       rsi, rcx
00000001412259e0 mov       rcx, rdx
00000001412259e3 call      0x1408bc520
00000001412259e8 mov       rbx, rax
00000001412259eb mov       rdi, rdx
00000001412259ee mov       qword ptr [rbp - 0x40], rax
00000001412259f2 mov       qword ptr [rbp - 0x38], rdx
00000001412259f6 lea       r14, [rsi + 0x20]
00000001412259fa lea       rdx, [rbp - 0x40]
00000001412259fe mov       rcx, r14
0000000141225a01 call      0x1405b14b0
0000000141225a06 mov       r15, rax
0000000141225a09 shr       r15, 0x39
0000000141225a0d mov       rcx, qword ptr [rsi]
0000000141225a10 mov       rdx, qword ptr [rsi + 8]
0000000141225a14 mov       r8, rdx
0000000141225a17 and       r8, rax
0000000141225a1a movd      xmm0, r15d
0000000141225a1f punpcklbw xmm0, xmm0
0000000141225a23 pshuflw   xmm0, xmm0, 0
0000000141225a28 pshufd    xmm0, xmm0, 0
0000000141225a2d xor       r9d, r9d
0000000141225a30 pcmpeqd   xmm1, xmm1
0000000141225a34 movdqu    xmm2, xmmword ptr [rcx + r8]
0000000141225a3a movdqa    xmm3, xmm2
0000000141225a3e pcmpeqb   xmm3, xmm0
0000000141225a42 pmovmskb  r10d, xmm3
0000000141225a47 test      r10d, r10d
0000000141225a4a je        0x141225a80
0000000141225a4c tzcnt     r12d, r10d
0000000141225a51 add       r12, r8
0000000141225a54 and       r12, rdx
0000000141225a57 shl       r12, 5
0000000141225a5b mov       r11, rcx
0000000141225a5e sub       r11, r12
0000000141225a61 cmp       qword ptr [r11 - 0x20], rbx
0000000141225a65 jne       0x141225a6d
0000000141225a67 cmp       qword ptr [r11 - 0x18], rdi
0000000141225a6b je        0x141225a9e
0000000141225a6d lea       r11d, [r10 - 1]
0000000141225a71 and       r11w, r10w
0000000141225a75 mov       r10d, r11d
0000000141225a78 jne       0x141225a4c
0000000141225a7a nop       word ptr [rax + rax]
0000000141225a80 pcmpeqb   xmm2, xmm1
0000000141225a84 pmovmskb  r10d, xmm2
0000000141225a89 test      r10d, r10d
0000000141225a8c jne       0x141225aa6
0000000141225a8e add       r8, r9
0000000141225a91 add       r8, 0x10
0000000141225a95 add       r9, 0x10
0000000141225a99 and       r8, rdx
0000000141225a9c jmp       0x141225a34
0000000141225a9e mov       rdi, r11
0000000141225aa1 jmp       0x141225bb6
0000000141225aa6 cmp       qword ptr [rsi + 0x10], 0
0000000141225aab je        0x141225e74
0000000141225ab1 cmp       rbx, 3
0000000141225ab5 je        0x141225bb6
0000000141225abb mov       r14, rax
0000000141225abe mov       byte ptr [rsi + 0x38], 1
0000000141225ac2 movzx     eax, byte ptr [rip + 0x240f140]
0000000141225ac9 mov       ecx, 0x70
0000000141225ace mov       edx, 0x10
0000000141225ad3 call      0x140613c10
0000000141225ad8 test      rax, rax
0000000141225adb je        0x141225eed
0000000141225ae1 mov       qword ptr [rax], 1
0000000141225ae8 mov       qword ptr [rax + 8], 1
0000000141225af0 movaps    xmm0, xmmword ptr [rip + 0x19f7829]
0000000141225af7 movaps    xmmword ptr [rax + 0x40], xmm0
0000000141225afb movaps    xmm0, xmmword ptr [rip + 0x19f780e]
0000000141225b02 movaps    xmmword ptr [rax + 0x30], xmm0
0000000141225b06 movaps    xmm0, xmmword ptr [rip + 0x19f77f3]
0000000141225b0d movaps    xmmword ptr [rax + 0x20], xmm0
0000000141225b11 movaps    xmm0, xmmword ptr [rip + 0x19f77d8]
0000000141225b18 movaps    xmmword ptr [rax + 0x10], xmm0
0000000141225b1c mov       qword ptr [rax + 0x50], 0
0000000141225b24 mov       qword ptr [rax + 0x58], 4
0000000141225b2c mov       qword ptr [rax + 0x60], 0
0000000141225b34 mov       dword ptr [rax + 0x68], 0
0000000141225b3b mov       rcx, qword ptr [rsi]
0000000141225b3e mov       rdx, qword ptr [rsi + 8]
0000000141225b42 mov       r8, r14
0000000141225b45 and       r8, rdx
0000000141225b48 movdqu    xmm0, xmmword ptr [rcx + r8]
0000000141225b4e pmovmskb  r9d, xmm0
0000000141225b53 test      r9d, r9d
0000000141225b56 je        0x141225e9a
0000000141225b5c tzcnt     r9d, r9d
0000000141225b61 add       r9, r8
0000000141225b64 and       r9, rdx
0000000141225b67 movzx     r8d, byte ptr [rcx + r9]
0000000141225b6c test      r8b, r8b
0000000141225b6f jns       0x141225ec0
0000000141225b75 lea       r10, [r9 - 0x10]
0000000141225b79 and       r10, rdx
0000000141225b7c mov       byte ptr [rcx + r9], r15b
0000000141225b80 mov       byte ptr [rcx + r10 + 0x10], r15b
0000000141225b85 shl       r9, 5
0000000141225b89 mov       rdx, rcx
0000000141225b8c sub       rdx, r9
0000000141225b8f and       r8b, 1
0000000141225b93 movzx     r8d, r8b
0000000141225b97 sub       qword ptr [rsi + 0x10], r8
0000000141225b9b neg       r9
0000000141225b9e mov       qword ptr [rcx + r9 - 0x20], rbx
0000000141225ba3 mov       qword ptr [rdx - 0x18], rdi
0000000141225ba7 mov       qword ptr [rdx - 0x10], rax
0000000141225bab mov       byte ptr [rdx - 8], 0
0000000141225baf inc       qword ptr [rsi + 0x18]
0000000141225bb3 mov       rdi, rdx
0000000141225bb6 mov       byte ptr [rdi - 8], 1
0000000141225bba mov       rcx, qword ptr [rdi - 0x10]
0000000141225bbe xor       edx, edx
0000000141225bc0 mov       eax, 1
0000000141225bc5 lock cmpxchg qword ptr [rcx], rdx
0000000141225bca lea       r15, [rdi - 0x10]
0000000141225bce mov       r12, qword ptr [rdi - 0x10]
0000000141225bd2 jne       0x141225bf0
0000000141225bd4 mov       rax, qword ptr [r12 + 8]
0000000141225bd9 cmp       rax, 1
0000000141225bdd jne       0x141225cc8
0000000141225be3 mov       qword ptr [r12], 1
0000000141225beb jmp       0x141225e3f
0000000141225bf0 mov       ecx, 0x10
0000000141225bf5 mov       edx, 0x60
0000000141225bfa call      0x140d7c180
0000000141225bff mov       rdi, rax
0000000141225c02 mov       rsi, rdx
0000000141225c05 test      rdx, rdx
0000000141225c08 je        0x141225d9d
0000000141225c0e movzx     eax, byte ptr [rip + 0x240eff4]
0000000141225c15 mov       rcx, rsi
0000000141225c18 mov       rdx, rdi
0000000141225c1b call      0x140613c10
0000000141225c20 mov       r13, rax
0000000141225c23 test      r13, r13
0000000141225c26 je        0x141225da9
0000000141225c2c mov       qword ptr [r13], 1
0000000141225c34 mov       qword ptr [r13 + 8], 1
0000000141225c3c mov       qword ptr [rbp - 0x30], r13
0000000141225c40 mov       qword ptr [rbp - 0x40], 0x10
0000000141225c48 mov       qword ptr [rbp - 0x38], 0x60
0000000141225c50 mov       byte ptr [rbp - 0x28], 1
0000000141225c54 mov       rsi, qword ptr [r12 + 0x60]
0000000141225c59 xor       ecx, ecx
0000000141225c5b mov       edx, 0xc
0000000141225c60 mov       rax, rsi
0000000141225c63 mul       rdx
0000000141225c66 mov       rdi, rax
0000000141225c69 jo        0x141225ed8
0000000141225c6f movabs    rax, 0x7ffffffffffffffc
0000000141225c79 cmp       rdi, rax
0000000141225c7c ja        0x141225ed8
0000000141225c82 mov       rdx, qword ptr [r12 + 0x58]
0000000141225c87 test      rdi, rdi
0000000141225c8a mov       qword ptr [rbp - 0x18], r15
0000000141225c8e je        0x141225db4
0000000141225c94 mov       r14, rdx
0000000141225c97 movzx     eax, byte ptr [rip + 0x240ef6b]
0000000141225c9e mov       edx, 4
0000000141225ca3 mov       rcx, rdi
0000000141225ca6 call      0x140613c10
0000000141225cab mov       ecx, 4
0000000141225cb0 test      rax, rax
0000000141225cb3 je        0x141225ed8
0000000141225cb9 mov       rbx, rax
0000000141225cbc mov       qword ptr [rbp - 0x10], rsi
0000000141225cc0 mov       rdx, r14
0000000141225cc3 jmp       0x141225dc1
0000000141225cc8 mov       qword ptr [rbp - 0x10], r12
0000000141225ccc lea       rax, [r12 + 8]
0000000141225cd1 mov       qword ptr [rbp - 0x20], rax
0000000141225cd5 mov       ecx, 0x10
0000000141225cda mov       edx, 0x60
0000000141225cdf call      0x140d7c180
0000000141225ce4 mov       rdi, rax
0000000141225ce7 mov       rsi, rdx
0000000141225cea test      rdx, rdx
0000000141225ced je        0x141225e57
0000000141225cf3 movzx     eax, byte ptr [rip + 0x240ef0f]
0000000141225cfa mov       rcx, rsi
0000000141225cfd mov       rdx, rdi
0000000141225d00 call      0x140613c10
0000000141225d05 mov       r12, rax
0000000141225d08 mov       rcx, qword ptr [rbp - 0x10]
0000000141225d0c test      r12, r12
0000000141225d0f je        0x141225e67
0000000141225d15 lea       rax, [rcx + 0x10]
0000000141225d19 mov       qword ptr [r12], 1
0000000141225d21 mov       qword ptr [r12 + 8], 1
0000000141225d2a movups    xmm0, xmmword ptr [rax]
0000000141225d2d movdqu    xmm1, xmmword ptr [rax + 0x10]
0000000141225d32 movdqu    xmm2, xmmword ptr [rax + 0x20]
0000000141225d37 movdqu    xmm3, xmmword ptr [rax + 0x30]
0000000141225d3c movups    xmmword ptr [r12 + 0x10], xmm0
0000000141225d42 movdqu    xmmword ptr [r12 + 0x20], xmm1
0000000141225d49 movdqu    xmmword ptr [r12 + 0x30], xmm2
0000000141225d50 movdqu    xmmword ptr [r12 + 0x40], xmm3
0000000141225d57 movups    xmm0, xmmword ptr [rax + 0x40]
0000000141225d5b movups    xmmword ptr [r12 + 0x50], xmm0
0000000141225d61 movdqu    xmm0, xmmword ptr [rax + 0x50]
0000000141225d66 movdqu    xmmword ptr [r12 + 0x60], xmm0
0000000141225d6d mov       qword ptr [r15], r12
0000000141225d70 cmp       rcx, -1
0000000141225d74 je        0x141225e3f
0000000141225d7a mov       rax, qword ptr [rbp - 0x20]
0000000141225d7e lock dec  qword ptr [rax]
0000000141225d82 jne       0x141225e3f
0000000141225d88 mov       edx, 0x70
0000000141225d8d mov       r8d, 0x10
0000000141225d93 call      0x140613c20
0000000141225d98 jmp       0x141225e3f
0000000141225d9d mov       r13, rdi
0000000141225da0 test      r13, r13
0000000141225da3 jne       0x141225c2c
0000000141225da9 mov       rcx, rdi
0000000141225dac mov       rdx, rsi
0000000141225daf call      0x1428d9000
0000000141225db4 mov       ebx, 4
0000000141225db9 mov       qword ptr [rbp - 0x10], 0
0000000141225dc1 mov       r14, r12
0000000141225dc4 add       r12, 0x10
0000000141225dc8 lea       r15, [r13 + 0x10]
0000000141225dcc mov       rcx, rbx
0000000141225dcf mov       r8, rdi
0000000141225dd2 call      0x142923c00
0000000141225dd7 mov       eax, dword ptr [r14 + 0x68]
0000000141225ddb movdqa    xmm0, xmmword ptr [r12]
0000000141225de1 movdqa    xmm1, xmmword ptr [r12 + 0x10]
0000000141225de8 movdqa    xmm2, xmmword ptr [r12 + 0x20]
0000000141225def movdqa    xmm3, xmmword ptr [r12 + 0x30]
0000000141225df6 movdqa    xmmword ptr [r15], xmm0
0000000141225dfb movdqa    xmmword ptr [r15 + 0x10], xmm1
0000000141225e01 movdqa    xmmword ptr [r15 + 0x20], xmm2
0000000141225e07 movdqa    xmmword ptr [r15 + 0x30], xmm3
0000000141225e0d mov       rcx, qword ptr [rbp - 0x10]
0000000141225e11 mov       qword ptr [r13 + 0x50], rcx
0000000141225e15 mov       qword ptr [r13 + 0x58], rbx
0000000141225e19 mov       qword ptr [r13 + 0x60], rsi
0000000141225e1d mov       qword ptr [rbp - 0x10], r13
0000000141225e21 mov       dword ptr [r13 + 0x68], eax
0000000141225e25 lock dec  qword ptr [r14]
0000000141225e29 jne       0x141225e34
0000000141225e2b mov       rcx, qword ptr [rbp - 0x18]
0000000141225e2f call      0x140b5cad0
0000000141225e34 mov       rax, qword ptr [rbp - 0x18]
0000000141225e38 mov       r12, qword ptr [rbp - 0x10]
0000000141225e3c mov       qword ptr [rax], r12
0000000141225e3f add       r12, 0x10
0000000141225e43 mov       rax, r12
0000000141225e46 add       rsp, 0x68
0000000141225e4a pop       rbx
0000000141225e4b pop       rdi
0000000141225e4c pop       rsi
0000000141225e4d pop       r12
0000000141225e4f pop       r13
0000000141225e51 pop       r14
0000000141225e53 pop       r15
0000000141225e55 pop       rbp
0000000141225e56 ret       
0000000141225e57 mov       r12, rdi
0000000141225e5a mov       rcx, qword ptr [rbp - 0x10]
0000000141225e5e test      r12, r12
0000000141225e61 jne       0x141225d15
0000000141225e67 mov       rcx, rdi
0000000141225e6a mov       rdx, rsi
0000000141225e6d call      0x1428d9000
0000000141225e72 jmp       0x141225eeb
0000000141225e74 mov       edx, 1
0000000141225e79 mov       rcx, rsi
0000000141225e7c mov       r8, r14
0000000141225e7f mov       r9b, 1
0000000141225e82 mov       rdi, rax
0000000141225e85 call      0x1428c1430
0000000141225e8a mov       rax, rdi
0000000141225e8d mov       rbx, qword ptr [rbp - 0x40]
0000000141225e91 mov       rdi, qword ptr [rbp - 0x38]
0000000141225e95 jmp       0x141225abb
0000000141225e9a mov       r10d, 0x10
0000000141225ea0 add       r8, r10
0000000141225ea3 and       r8, rdx
0000000141225ea6 movdqu    xmm0, xmmword ptr [rcx + r8]
0000000141225eac pmovmskb  r9d, xmm0
0000000141225eb1 add       r10, 0x10
0000000141225eb5 test      r9d, r9d
0000000141225eb8 jne       0x141225b5c
0000000141225ebe jmp       0x141225ea0
0000000141225ec0 movdqa    xmm0, xmmword ptr [rcx]
0000000141225ec4 pmovmskb  r8d, xmm0
0000000141225ec9 tzcnt     r9d, r8d
0000000141225ece movzx     r8d, byte ptr [rcx + r9]
0000000141225ed3 jmp       0x141225b75
0000000141225ed8 mov       byte ptr [rbp - 1], 1
0000000141225edc lea       r8, [rip + 0x19f408d]
0000000141225ee3 mov       rdx, rdi
0000000141225ee6 call      0x1428d8fe3
0000000141225eeb ud2       
0000000141225eed mov       ecx, 0x10
0000000141225ef2 mov       edx, 0x70
0000000141225ef7 call      0x1428d9000
0000000141225efc int3      
0000000141225efd nop       dword ptr [rax]
0000000141225f00 mov       qword ptr [rsp + 0x10], rdx
0000000141225f05 push      rbp
0000000141225f06 push      r15
0000000141225f08 push      r14
0000000141225f0a push      r13
0000000141225f0c push      r12
0000000141225f0e push      rsi
0000000141225f0f push      rdi
0000000141225f10 push      rbx
0000000141225f11 sub       rsp, 0x28
0000000141225f15 lea       rbp, [rdx + 0x60]
0000000141225f19 cmp       qword ptr [rbp - 0x10], -1
0000000141225f1e je        0x141225f3e
0000000141225f20 mov       rax, qword ptr [rbp - 0x20]
0000000141225f24 lock dec  qword ptr [rax]
0000000141225f28 jne       0x141225f3e
0000000141225f2a mov       edx, 0x70
0000000141225f2f mov       r8d, 0x10
0000000141225f35 mov       rcx, qword ptr [rbp - 0x10]
0000000141225f39 call      0x140613c20
0000000141225f3e nop       
0000000141225f3f add       rsp, 0x28
0000000141225f43 pop       rbx
0000000141225f44 pop       rdi
0000000141225f45 pop       rsi
0000000141225f46 pop       r12
0000000141225f48 pop       r13
0000000141225f4a pop       r14
0000000141225f4c pop       r15
0000000141225f4e pop       rbp
0000000141225f4f ret       
0000000141225f50 mov       qword ptr [rsp + 0x10], rdx
0000000141225f55 push      rbp
0000000141225f56 push      r15
0000000141225f58 push      r14
0000000141225f5a push      r13
0000000141225f5c push      r12
0000000141225f5e push      rsi
0000000141225f5f push      rdi
0000000141225f60 push      rbx
0000000141225f61 sub       rsp, 0x28
0000000141225f65 lea       rbp, [rdx + 0x60]
0000000141225f69 mov       rax, qword ptr [rbp - 0x18]
0000000141225f6d mov       rcx, qword ptr [rbp - 0x10]
0000000141225f71 mov       qword ptr [rax], rcx
0000000141225f74 mov       byte ptr [rbp - 1], 0
0000000141225f78 add       rsp, 0x28
0000000141225f7c pop       rbx
0000000141225f7d pop       rdi
0000000141225f7e pop       rsi
0000000141225f7f pop       r12
0000000141225f81 pop       r13
0000000141225f83 pop       r14
0000000141225f85 pop       r15
0000000141225f87 pop       rbp
0000000141225f88 ret       
0000000141225f89 nop       dword ptr [rax]
0000000141225f90 mov       qword ptr [rsp + 0x10], rdx
0000000141225f95 push      rbp
0000000141225f96 push      r15
0000000141225f98 push      r14
0000000141225f9a push      r13
0000000141225f9c push      r12
0000000141225f9e push      rsi
0000000141225f9f push      rdi
0000000141225fa0 push      rbx
0000000141225fa1 sub       rsp, 0x28
0000000141225fa5 lea       rbp, [rdx + 0x60]
0000000141225fa9 cmp       byte ptr [rbp - 1], 0
0000000141225fad jne       0x141225fc0
0000000141225faf add       rsp, 0x28
0000000141225fb3 pop       rbx
0000000141225fb4 pop       rdi
0000000141225fb5 pop       rsi
0000000141225fb6 pop       r12
0000000141225fb8 pop       r13
0000000141225fba pop       r14
0000000141225fbc pop       r15
0000000141225fbe pop       rbp
0000000141225fbf ret       
0000000141225fc0 lea       rcx, [rbp - 0x40]
0000000141225fc4 call      0x1411fd460
0000000141225fc9 jmp       0x141225faf
