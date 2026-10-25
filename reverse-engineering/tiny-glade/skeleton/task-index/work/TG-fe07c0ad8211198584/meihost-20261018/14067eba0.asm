0x14067eba0 push rbp
0x14067eba1 push r15
0x14067eba3 push r14
0x14067eba5 push r13
0x14067eba7 push r12
0x14067eba9 push rsi
0x14067ebaa push rdi
0x14067ebab push rbx
0x14067ebac sub rsp, 0x2c8
0x14067ebb3 lea rbp, [rsp + 0x80]
0x14067ebbb movdqa xmmword ptr [rbp + 0x230], xmm8
0x14067ebc4 movdqa xmmword ptr [rbp + 0x220], xmm7
0x14067ebcc movdqa xmmword ptr [rbp + 0x210], xmm6
0x14067ebd4 mov qword ptr [rbp + 0x208], 0xfffffffffffffffe
0x14067ebdf mov rbx, r8
0x14067ebe2 mov rdi, rdx
0x14067ebe5 mov rsi, rcx
0x14067ebe8 mov r15, qword ptr [rdx + 8]
0x14067ebec mov r14, qword ptr [rdx + 0x10]
0x14067ebf0 cmp r14, 5
0x14067ebf4 jb 0x14067ec1b
0x14067ebf6 mov ecx, dword ptr [r15]
0x14067ebf9 bswap ecx
0x14067ebfb cmp ecx, 0x66696c65
0x14067ec01 jne 0x14067ede1
0x14067ec07 movzx ecx, byte ptr [r15 + 4]
0x14067ec0c mov eax, 0x3a
0x14067ec11 sub eax, ecx
0x14067ec13 test eax, eax
0x14067ec15 je 0x14067edf7
0x14067ec1b mov rbx, qword ptr [rbx + 0x10]
0x14067ec1f mov rcx, r15
0x14067ec22 mov rdx, r14
0x14067ec25 call 0x140c9cd80
0x14067ec2a mov rcx, rbx
0x14067ec2d mov rdx, rax
0x14067ec30 call 0x14093c970
0x14067ec35 test rdx, rdx
0x14067ec38 je 0x14067fb8a
0x14067ec3e mov r12, rdx
0x14067ec41 mov rcx, rbx
0x14067ec44 mov rdx, rax
0x14067ec47 call 0x14093d310
0x14067ec4c mov ebx, eax
0x14067ec4e add r12, 0x18
0x14067ec52 mov rcx, r12
0x14067ec55 call 0x140bd5790
0x14067ec5a mov r14, rax
0x14067ec5d mov rax, r14
0x14067ec60 mov ecx, r14d
0x14067ec63 xorps xmm6, xmm6
0x14067ec66 cvtsi2ss xmm6, rcx
0x14067ec6b shr rax, 0x20
0x14067ec6f xorps xmm0, xmm0
0x14067ec72 cvtsi2ss xmm0, rax
0x14067ec77 movss xmm7, dword ptr [rdi + 0x18]
0x14067ec7c divss xmm7, xmm0
0x14067ec80 mulss xmm6, xmm7
0x14067ec84 mulss xmm7, xmm0
0x14067ec88 movss dword ptr [rdi + 0x38], xmm6
0x14067ec8d movss dword ptr [rdi + 0x3c], xmm7
0x14067ec92 lea r14, [rdi + 0x20]
0x14067ec96 mov rcx, r14
0x14067ec99 call 0x140b92590
0x14067ec9e lea r15, [rbp + 0x170]
0x14067eca5 mov rcx, r15
0x14067eca8 call 0x140a82b20
0x14067ecad movzx r9d, byte ptr [rdi + 0x44]
0x14067ecb2 add r9b, r9b
0x14067ecb5 lea r12, [rbp + 0x110]
0x14067ecbc mov rcx, r12
0x14067ecbf mov rdx, r15
0x14067ecc2 mov r8d, ebx
0x14067ecc5 call 0x140fad550
0x14067ecca mov r8d, dword ptr [rdi + 0x40]
0x14067ecce lea rbx, [rbp + 0x170]
0x14067ecd5 mov rcx, rbx
0x14067ecd8 mov rdx, r12
0x14067ecdb call 0x140b920b0
0x14067ece0 lea r9, [rip + 0x2398471] ; 0x142a17158 00000000000000000000000000000000 f32=0.0
0x14067ece7 mov r8d, 1
0x14067eced mov rcx, r14
0x14067ecf0 mov rdx, rbx
0x14067ecf3 call 0x140b92340
0x14067ecf8 mov rbx, qword ptr [rbp + 0x178]
0x14067ecff mov r14, qword ptr [rbp + 0x180]
0x14067ed06 test r14, r14
0x14067ed09 je 0x14067ed6d
0x14067ed0b lea r15, [rbx + 8]
0x14067ed0f movabs r12, 0x7ffffffffffffffe
0x14067ed19 movabs r13, 0x8000000000000002
0x14067ed23 jmp 0x14067ed39
0x14067ed25 nop word ptr cs:[rax + rax]
0x14067ed30 add r15, 0x20
0x14067ed34 dec r14
0x14067ed37 je 0x14067ed6d
0x14067ed39 mov rdx, qword ptr [r15 - 8]
0x14067ed3d lea rcx, [rdx + r12]
0x14067ed41 cmp rcx, 3
0x14067ed45 setb al
0x14067ed48 cmp rcx, 1
0x14067ed4c setne cl
0x14067ed4f cmp rdx, r13
0x14067ed52 jl 0x14067ed30
0x14067ed54 and al, cl
0x14067ed56 jne 0x14067ed30
0x14067ed58 test rdx, rdx
0x14067ed5b je 0x14067ed30
0x14067ed5d mov rcx, qword ptr [r15]
0x14067ed60 mov r8d, 1
0x14067ed66 call 0x140613c20
0x14067ed6b jmp 0x14067ed30
0x14067ed6d mov rdx, qword ptr [rbp + 0x170]
0x14067ed74 test rdx, rdx
0x14067ed77 je 0x14067ed8b
0x14067ed79 shl rdx, 5
0x14067ed7d mov r8d, 8
0x14067ed83 mov rcx, rbx
0x14067ed86 call 0x140613c20
0x14067ed8b cmp byte ptr [rdi + 0x1c], 0
0x14067ed8f xorps xmm0, xmm0
0x14067ed92 xorps xmm1, xmm1
0x14067ed95 jne 0x14067ed9d
0x14067ed97 movaps xmm0, xmm6
0x14067ed9a movaps xmm1, xmm7
0x14067ed9d mov dword ptr [rsi], 0
0x14067eda3 movss dword ptr [rsi + 4], xmm0
0x14067eda8 mov dword ptr [rsi + 8], 0
0x14067edaf movss dword ptr [rsi + 0xc], xmm1
0x14067edb4 mov rax, rsi
0x14067edb7 movaps xmm6, xmmword ptr [rbp + 0x210]
0x14067edbe movaps xmm7, xmmword ptr [rbp + 0x220]
0x14067edc5 movaps xmm8, xmmword ptr [rbp + 0x230]
0x14067edcd add rsp, 0x2c8
0x14067edd4 pop rbx
0x14067edd5 pop rdi
0x14067edd6 pop rsi
0x14067edd7 pop r12
0x14067edd9 pop r13
0x14067eddb pop r14
0x14067eddd pop r15
0x14067eddf pop rbp
0x14067ede0 ret 
0x14067ede1 xor eax, eax
0x14067ede3 cmp ecx, 0x66696c66
0x14067ede9 adc eax, -1
0x14067edec or eax, 1
0x14067edef test eax, eax
0x14067edf1 jne 0x14067ec1b
0x14067edf7 add r15, 5
0x14067edfb add r14, -5
0x14067edff mov qword ptr [rbp + 0xb0], r15
0x14067ee06 mov qword ptr [rbp + 0xb8], r14
0x14067ee0d mov rax, qword ptr [rip + 0x2fac3dc] ; 0x14362b1f0 0000000000000000b0de674001000000 f32=0.0
0x14067ee14 cmp rax, 2
0x14067ee18 jne 0x14067f746
0x14067ee1e mov cl, 1
0x14067ee20 xor eax, eax
0x14067ee22 lock cmpxchg byte ptr [rip + 0x2fac38e], cl ; 0x14362b1b8 00000000000000000000000000000000 f32=0.0
0x14067ee2a jne 0x14067f767
0x14067ee30 lea rcx, [rip + 0x2fac381] ; 0x14362b1b8 00000000000000000000000000000000 f32=0.0
0x14067ee37 call 0x140c81c70
0x14067ee3c mov r12, qword ptr [rbp + 0xb0]
0x14067ee43 mov r15, qword ptr [rbp + 0xb8]
0x14067ee4a cmp qword ptr [rip + 0x2fac386], 0 ; 0x14362b1d8 00000000000000000000000000000000 f32=0.0
0x14067ee52 je 0x14067ef50
0x14067ee58 mov qword ptr [rbp + 0x1f8], rbx
0x14067ee5f lea rcx, [rip + 0x2fac37a] ; 0x14362b1e0 00000000000000000000000000000000 f32=0.0
0x14067ee66 mov rdx, r12
0x14067ee69 mov r8, r15
0x14067ee6c call 0x1405b1a70
0x14067ee71 mov rcx, qword ptr [rip + 0x2fac350] ; 0x14362b1c8 00000000000000000000000000000000 f32=0.0
0x14067ee78 mov r13, rcx
0x14067ee7b and r13, rax
0x14067ee7e shr rax, 0x39
0x14067ee82 mov r14, qword ptr [rip + 0x2fac337] ; 0x14362b1c0 00000000000000000000000000000000 f32=0.0
0x14067ee89 movd xmm0, eax
0x14067ee8d punpcklbw xmm0, xmm0
0x14067ee91 pshuflw xmm0, xmm0, 0
0x14067ee96 pshufd xmm6, xmm0, 0
0x14067ee9b lea rdx, [r14 - 0x28]
0x14067ee9f xor r8d, r8d
0x14067eea2 pcmpeqd xmm7, xmm7
0x14067eea6 movdqu xmm8, xmmword ptr [r14 + r13]
0x14067eeac movdqa xmm0, xmm8
0x14067eeb1 pcmpeqb xmm0, xmm6
0x14067eeb5 pmovmskb eax, xmm0
0x14067eeb9 test eax, eax
0x14067eebb je 0x14067ef28
0x14067eebd mov qword ptr [rbp + 0x1f0], r12
0x14067eec4 mov qword ptr [rbp + 0x70], rcx
0x14067eec8 mov qword ptr [rbp + 0x78], r8
0x14067eecc mov qword ptr [rbp + 0x68], rax
0x14067eed0 tzcnt eax, eax
0x14067eed4 add rax, r13
0x14067eed7 and rax, rcx
0x14067eeda neg rax
0x14067eedd lea r12, [rax + rax*4]
0x14067eee1 cmp qword ptr [rdx + r12*8 + 0x10], r15
0x14067eee6 jne 0x14067ef0d
0x14067eee8 lea rax, [rdx + r12*8]
0x14067eeec mov rbx, rdx
0x14067eeef mov rdx, qword ptr [rax + 8]
0x14067eef3 mov rcx, qword ptr [rbp + 0x1f0]
0x14067eefa mov r8, r15
0x14067eefd call 0x142923bf0
0x14067ef02 mov rdx, rbx
0x14067ef05 test eax, eax
0x14067ef07 je 0x14067f0c8
0x14067ef0d mov rcx, qword ptr [rbp + 0x68]
0x14067ef11 lea eax, [rcx - 1]
0x14067ef14 and ax, cx
0x14067ef17 mov r12, qword ptr [rbp + 0x1f0]
0x14067ef1e mov rcx, qword ptr [rbp + 0x70]
0x14067ef22 mov r8, qword ptr [rbp + 0x78]
0x14067ef26 jne 0x14067eecc
0x14067ef28 pcmpeqb xmm8, xmm7
0x14067ef2d pmovmskb eax, xmm8
0x14067ef32 test eax, eax
0x14067ef34 mov rbx, qword ptr [rbp + 0x1f8]
0x14067ef3b jne 0x14067ef50
0x14067ef3d add r13, r8
0x14067ef40 add r13, 0x10
0x14067ef44 add r8, 0x10
0x14067ef48 and r13, rcx
0x14067ef4b jmp 0x14067eea6
0x14067ef50 mov rax, qword ptr [rip + 0x2fad451] ; 0x14362c3a8 2093634301000000b0c3624301000000 f32=227.57470703125
0x14067ef57 mov rax, qword ptr [rax]
0x14067ef5a cmp rax, 4
0x14067ef5e jb 0x14067f082
0x14067ef64 mov rcx, r12
0x14067ef67 mov rdx, r15
0x14067ef6a call 0x140d61e00
0x14067ef6f test rax, rax
0x14067ef72 lea rcx, [rip + 0x23980ad] ; 0x142a17026 3f3f3f4c6f6164696e67207465787475 f32=50134268.0
0x14067ef79 cmovne rcx, rax
0x14067ef7d mov eax, 3
0x14067ef82 cmove rdx, rax
0x14067ef86 mov qword ptr [rbp + 0x110], rcx
0x14067ef8d mov qword ptr [rbp + 0x118], rdx
0x14067ef94 lea rax, [rbp + 0x110]
0x14067ef9b mov qword ptr [rbp + 0xc0], rax
0x14067efa2 lea rax, [rip - 0x6506d9] ; 0x14002e8d0 4989d0488b01488b51084889c1e94e46 f32=427082.28125
0x14067efa9 mov qword ptr [rbp + 0xc8], rax
0x14067efb0 lea rcx, [rip + 0x23980a1] ; 0x142a17058 f86fa142010000002900000000000000 f32=80.71868896484375
0x14067efb7 call 0x14067dd80
0x14067efbc movdqu xmm0, xmmword ptr [rax]
0x14067efc0 mov eax, dword ptr [rax + 0x10]
0x14067efc3 mov qword ptr [rbp + 0x1a0], 4
0x14067efce lea rcx, [rip + 0x239809b] ; 0x142a17070 746f6f6c6261723a3a6d65693a3a7769 f32=1.1578383735592298e+27
0x14067efd5 mov qword ptr [rbp + 0x1a8], rcx
0x14067efdc mov qword ptr [rbp + 0x1b0], 0x15
0x14067efe7 lea rdx, [rip + 0x239805a] ; 0x142a17048 2970a142010000001b00000000000000 f32=80.71906280517578
0x14067efee mov qword ptr [rbp + 0x1c0], rdx
0x14067eff5 mov qword ptr [rbp + 0x1c8], 1
0x14067f000 lea rdx, [rbp + 0xc0]
0x14067f007 mov qword ptr [rbp + 0x1d0], rdx
0x14067f00e mov qword ptr [rbp + 0x1d8], 1
0x14067f019 mov qword ptr [rbp + 0x1e0], 0
0x14067f024 mov qword ptr [rbp + 0x170], 0
0x14067f02f mov qword ptr [rbp + 0x178], rcx
0x14067f036 mov qword ptr [rbp + 0x180], 0x15
0x14067f041 mov qword ptr [rbp + 0x188], 0
0x14067f04c movdqu xmmword ptr [rbp + 0x190], xmm0
0x14067f054 mov dword ptr [rbp + 0x1b8], 1
0x14067f05e mov dword ptr [rbp + 0x1bc], eax
0x14067f064 lea rcx, [rbp + 0x40]
0x14067f068 lea rdx, [rbp + 0x170]
0x14067f06f call 0x140d4a330
0x14067f074 mov r12, qword ptr [rbp + 0xb0]
0x14067f07b mov r15, qword ptr [rbp + 0xb8]
0x14067f082 lea r14, [rbp + 0x110]
0x14067f089 mov rcx, r14
0x14067f08c mov rdx, r12
0x14067f08f mov r8, r15
0x14067f092 call 0x140ba3a10
0x14067f097 mov rcx, qword ptr [rbp + 0x110]
0x14067f09e mov rax, qword ptr [rbp + 0x118]
0x14067f0a5 cmp rcx, 2
0x14067f0a9 jne 0x14067f0d7
0x14067f0ab movabs rcx, 0x8000000000000009
0x14067f0b5 mov qword ptr [rbp + 0xc0], rcx
0x14067f0bc mov qword ptr [rbp + 0xc8], rax
0x14067f0c3 jmp 0x14067f27f
0x14067f0c8 mov ebx, dword ptr [r14 + r12*8 - 8]
0x14067f0cd mov r14, qword ptr [r14 + r12*8 - 0x10]
0x14067f0d2 jmp 0x14067f715
0x14067f0d7 mov rdx, qword ptr [rbp + 0x160]
0x14067f0de mov qword ptr [rbp + 0x1c0], rdx
0x14067f0e5 movdqu xmm0, xmmword ptr [rbp + 0x120]
0x14067f0ed movups xmm1, xmmword ptr [rbp + 0x130]
0x14067f0f4 movups xmm2, xmmword ptr [rbp + 0x140]
0x14067f0fb movups xmm3, xmmword ptr [rbp + 0x150]
0x14067f102 movups xmmword ptr [rbp + 0x1b0], xmm3
0x14067f109 movups xmmword ptr [rbp + 0x1a0], xmm2
0x14067f110 movups xmmword ptr [rbp + 0x190], xmm1
0x14067f117 movdqu xmmword ptr [rbp + 0x180], xmm0
0x14067f11f mov qword ptr [rbp + 0x170], rcx
0x14067f126 mov qword ptr [rbp + 0x178], rax
0x14067f12d lea rcx, [rbp + 0xc0]
0x14067f134 lea rdx, [rbp + 0x170]
0x14067f13b call 0x140fae290
0x14067f140 movabs rax, 0x800000000000000a
0x14067f14a cmp qword ptr [rbp + 0xc0], rax
0x14067f151 jne 0x14067f27f
0x14067f157 mov rcx, qword ptr [rbp + 0xc8]
0x14067f15e mov rax, qword ptr [rbp + 0xd0]
0x14067f165 movdqu xmm0, xmmword ptr [rbp + 0xd8]
0x14067f16d movdqa xmmword ptr [rbp], xmm0
0x14067f172 mov rdx, qword ptr [rbp + 0xe8]
0x14067f179 mov qword ptr [rbp + 0x10], rdx
0x14067f17d cmp rcx, 0xa
0x14067f181 je 0x14067f371
0x14067f187 mov r15, qword ptr [rbx + 8]
0x14067f18b mov qword ptr [rbp + 0x40], rcx
0x14067f18f mov qword ptr [rbp + 0x48], rax
0x14067f193 movdqa xmm0, xmmword ptr [rbp]
0x14067f198 movdqu xmmword ptr [rbp + 0x50], xmm0
0x14067f19d mov rax, qword ptr [rbp + 0x10]
0x14067f1a1 mov qword ptr [rbp + 0x60], rax
0x14067f1a5 lea rcx, [rbp + 0x20]
0x14067f1a9 lea rdx, [rbp + 0x40]
0x14067f1ad call 0x140bad8d0
0x14067f1b2 mov rdx, qword ptr [rbp + 0x38]
0x14067f1b6 mov byte ptr [rbp + 0x206], 1
0x14067f1bd lea rcx, [rbp + 0x170]
0x14067f1c4 mov r8d, 0x2b
0x14067f1ca call 0x140bd52a0
0x14067f1cf mov byte ptr [rbp + 0x206], 1
0x14067f1d6 lea r12, [rbp + 0x110]
0x14067f1dd lea rdx, [rbp + 0x170]
0x14067f1e4 mov rcx, r12
0x14067f1e7 call 0x140bd5370
0x14067f1ec mov rax, qword ptr [rbp + 0x20]
0x14067f1f0 mov qword ptr [rbp + 0x1f8], rax
0x14067f1f7 mov rcx, qword ptr [rbp + 0x28]
0x14067f1fb mov rax, qword ptr [rbp + 0x30]
0x14067f1ff mov qword ptr [rsp + 0x30], rax
0x14067f204 mov qword ptr [rbp + 0x1f0], rcx
0x14067f20b mov qword ptr [rsp + 0x28], rcx
0x14067f210 mov qword ptr [rsp + 0x20], r12
0x14067f215 lea r8, [rip + 0x2397f5c] ; 0x142a17178 67726164696e67206c75744372656174 f32=1.6635043896910296e+22
0x14067f21c lea rcx, [rbp + 0xc0]
0x14067f223 mov r9d, 0xb
0x14067f229 mov rdx, r15
0x14067f22c call 0x140faeba0
0x14067f231 cmp dword ptr [rbp + 0xc0], 0xb
0x14067f238 jne 0x14067f77e
0x14067f23e mov r13, rbx
0x14067f241 mov rax, qword ptr [rbp + 0xc8]
0x14067f248 movups xmm0, xmmword ptr [rbp + 0xd0]
0x14067f24f movaps xmmword ptr [rbp - 0x30], xmm0
0x14067f253 movups xmm0, xmmword ptr [rbp + 0xe0]
0x14067f25a movaps xmmword ptr [rbp - 0x20], xmm0
0x14067f25e movdqu xmm0, xmmword ptr [rbp + 0xee]
0x14067f266 movdqu xmmword ptr [rbp - 0x12], xmm0
0x14067f26b movzx ebx, byte ptr [rbp + 0xfe]
0x14067f272 movzx r15d, byte ptr [rbp + 0xff]
0x14067f27a jmp 0x14067f883
0x14067f27f mov rax, qword ptr [rbp + 0xc0]
0x14067f286 mov rcx, qword ptr [rbp + 0xc8]
0x14067f28d movdqu xmm0, xmmword ptr [rbp + 0xd0]
0x14067f295 movups xmm1, xmmword ptr [rbp + 0xe0]
0x14067f29c movups xmm2, xmmword ptr [rbp + 0xf0]
0x14067f2a3 movaps xmmword ptr [rbp + 0x140], xmm2
0x14067f2aa movaps xmmword ptr [rbp + 0x130], xmm1
0x14067f2b1 movdqa xmmword ptr [rbp + 0x120], xmm0
0x14067f2b9 mov qword ptr [rbp + 0x110], rax
0x14067f2c0 mov qword ptr [rbp + 0x118], rcx
0x14067f2c7 mov byte ptr [rbp + 0x205], 1
0x14067f2ce lea rcx, [rbp + 0x170]
0x14067f2d5 call 0x140d56700
0x14067f2da movups xmm0, xmmword ptr [rbp + 0x170]
0x14067f2e1 movups xmm1, xmmword ptr [rbp + 0x180]
0x14067f2e8 movups xmm2, xmmword ptr [rbp + 0x190]
0x14067f2ef movaps xmmword ptr [rbp + 0xa0], xmm2
0x14067f2f6 movaps xmmword ptr [rbp + 0x90], xmm1
0x14067f2fd movaps xmmword ptr [rbp + 0x80], xmm0
0x14067f304 movdqa xmm0, xmmword ptr [rbp + 0x110]
0x14067f30c movaps xmm1, xmmword ptr [rbp + 0x120]
0x14067f313 movaps xmm2, xmmword ptr [rbp + 0x130]
0x14067f31a movaps xmm3, xmmword ptr [rbp + 0x140]
0x14067f321 movups xmmword ptr [rbp + 0x1b0], xmm3
0x14067f328 movups xmmword ptr [rbp + 0x1a0], xmm2
0x14067f32f movups xmmword ptr [rbp + 0x190], xmm1
0x14067f336 movdqu xmmword ptr [rbp + 0x180], xmm0
0x14067f33e lea rax, [rip + 0x2397d40] ; 0x142a17085 4c6f6164696e672074686520696d6167 f32=1.6634148806484356e+22
0x14067f345 mov qword ptr [rbp + 0x170], rax
0x14067f34c mov qword ptr [rbp + 0x178], 0x16
0x14067f357 mov byte ptr [rbp + 0x205], 0
0x14067f35e lea rcx, [rbp + 0x170]
0x14067f365 lea rdx, [rbp + 0x80]
0x14067f36c call 0x1428e5d90
0x14067f371 mov qword ptr [rbp + 0x80], rax
0x14067f378 lea rax, [rbp + 0xb0]
0x14067f37f mov qword ptr [rbp + 0x110], rax
0x14067f386 lea rax, [rip - 0x14baed] ; 0x1405338a0 4989d0488b01488b51084889c1e9ae4a f32=427082.28125
0x14067f38d mov qword ptr [rbp + 0x118], rax
0x14067f394 lea rax, [rbp + 0x80]
0x14067f39b mov qword ptr [rbp + 0x120], rax
0x14067f3a2 lea rax, [rip + 0x6cd877] ; 0x140d4cc20 488b09e998030000cccccccccccccccc f32=-1.0392535899618524e+25
0x14067f3a9 mov qword ptr [rbp + 0x128], rax
0x14067f3b0 lea rax, [rip + 0x2397cf9] ; 0x142a170b0 9b70a142010000000e00000000000000 f32=80.71993255615234
0x14067f3b7 mov qword ptr [rbp + 0x170], rax
0x14067f3be mov qword ptr [rbp + 0x178], 2
0x14067f3c9 mov qword ptr [rbp + 0x180], r14
0x14067f3d0 mov qword ptr [rbp + 0x188], 2
0x14067f3db mov qword ptr [rbp + 0x190], 0
0x14067f3e6 lea rcx, [rbp - 0x48]
0x14067f3ea lea rdx, [rbp + 0x170]
0x14067f3f1 call 0x140d79f90
0x14067f3f6 movdqu xmm0, xmmword ptr [rbp - 0x48]
0x14067f3fb movdqa xmmword ptr [rbp + 0xc0], xmm0
0x14067f403 mov rax, qword ptr [rbp - 0x38]
0x14067f407 mov qword ptr [rbp + 0xd0], rax
0x14067f40e mov eax, dword ptr [rip + 0x2fb581c] ; 0x143634c30 f01400002a150000bc5a190330150000 f32=7.51095976878102e-42
0x14067f414 cmp eax, 3
0x14067f417 jne 0x14067fafb
0x14067f41d mov rdx, qword ptr [rip + 0x2fb5804] ; 0x143634c28 ea140000a45a1903f01400002a150000 f32=7.50255197799507e-42
0x14067f424 lea rcx, [rbp + 0x170]
0x14067f42b call 0x140542890
0x14067f430 cmp dword ptr [rbp + 0x170], 1
0x14067f437 je 0x14067fb9c
0x14067f43d mov rax, qword ptr [rbp + 0x178]
0x14067f444 mov qword ptr [rbp + 0x1f8], rax
0x14067f44b movzx eax, byte ptr [rbp + 0x180]
0x14067f452 mov byte ptr [rbp + 0x1f0], al
0x14067f458 movzx eax, byte ptr [rip + 0x2fb57aa] ; 0x143634c09 5a1903301400006a140000745a190370 f32=4.769354911005053e-10
0x14067f45f mov ecx, 0xf
0x14067f464 mov edx, 1
0x14067f469 call 0x140613c10
0x14067f46e test rax, rax
0x14067f471 je 0x14067fbe4
0x14067f477 movabs rcx, 0x2928202168746170
0x14067f481 mov qword ptr [rax + 7], rcx
0x14067f485 movabs rcx, 0x705f656c75646f6d
0x14067f48f mov qword ptr [rax], rcx
0x14067f492 mov qword ptr [rbp + 0x170], 0xf
0x14067f49d mov qword ptr [rbp + 0x178], rax
0x14067f4a4 mov qword ptr [rbp + 0x180], 0xf
0x14067f4af mov qword ptr [rsp + 0x20], 1
0x14067f4b8 lea rcx, [rbp + 0x170]
0x14067f4bf mov edx, 0xf
0x14067f4c4 mov r8d, 0x18
0x14067f4ca mov r9d, 1
0x14067f4d0 call 0x1428a58d0
0x14067f4d5 mov rax, qword ptr [rbp + 0x178]
0x14067f4dc mov r15, qword ptr [rbp + 0x180]
0x14067f4e3 movups xmm0, xmmword ptr [rip + 0x2397c11] ; 0x142a170fb 246372617465203a3a204c6576656c20 f32=2.7945349379666425e+20
0x14067f4ea movups xmmword ptr [rax + r15], xmm0
0x14067f4ef movabs rcx, 0x726f727245203a3a
0x14067f4f9 mov qword ptr [rax + r15 + 0x10], rcx
0x14067f4fe add r15, 0x18
0x14067f502 mov qword ptr [rbp + 0x120], r15
0x14067f509 movdqu xmm0, xmmword ptr [rbp + 0x170]
0x14067f511 movdqa xmmword ptr [rbp + 0x110], xmm0
0x14067f519 mov r13, qword ptr [rbp + 0xc8]
0x14067f520 mov r12, qword ptr [rbp + 0xd0]
0x14067f527 mov rax, qword ptr [rbp + 0x110]
0x14067f52e sub rax, r15
0x14067f531 cmp rax, r12
0x14067f534 jb 0x14067fb36
0x14067f53a mov rcx, qword ptr [rbp + 0x118]
0x14067f541 add rcx, r15
0x14067f544 mov rdx, r13
0x14067f547 mov r8, r12
0x14067f54a call 0x142923c00
0x14067f54f add r15, r12
0x14067f552 mov qword ptr [rbp + 0x120], r15
0x14067f559 mov qword ptr [rbp + 0x180], r15
0x14067f560 movdqa xmm0, xmmword ptr [rbp + 0x110]
0x14067f568 movdqa xmmword ptr [rbp + 0x170], xmm0
0x14067f570 mov rax, qword ptr [rbp + 0x1f8]
0x14067f577 lea rcx, [rax + 8]
0x14067f57b lea rdx, [rbp + 0x170]
0x14067f582 call 0x140fb0060
0x14067f587 test al, al
0x14067f589 jne 0x14067f67b
0x14067f58f mov rax, qword ptr [rip + 0x2face12] ; 0x14362c3a8 2093634301000000b0c3624301000000 f32=227.57470703125
0x14067f596 mov rax, qword ptr [rax]
0x14067f599 test rax, rax
0x14067f59c je 0x14067f67b
0x14067f5a2 lea rax, [rbp + 0xc0]
0x14067f5a9 mov qword ptr [rbp + 0x110], rax
0x14067f5b0 lea rax, [rip - 0x643ae7] ; 0x14003bad0 4989d0488b4108488b51104889c1e90d f32=427082.28125
0x14067f5b7 mov qword ptr [rbp + 0x118], rax
0x14067f5be lea rcx, [rip + 0x2397b0b] ; 0x142a170d0 f86fa142010000002900000000000000 f32=80.71868896484375
0x14067f5c5 call 0x14067dd80
0x14067f5ca movdqu xmm0, xmmword ptr [rax]
0x14067f5ce mov eax, dword ptr [rax + 0x10]
0x14067f5d1 mov qword ptr [rbp + 0x1a0], 1
0x14067f5dc lea rcx, [rip + 0x2397a8d] ; 0x142a17070 746f6f6c6261723a3a6d65693a3a7769 f32=1.1578383735592298e+27
0x14067f5e3 mov qword ptr [rbp + 0x1a8], rcx
0x14067f5ea mov qword ptr [rbp + 0x1b0], 0x15
0x14067f5f5 lea rdx, [rip + 0x2397b1c] ; 0x142a17118 01000000000000000000000000000000 f32=1.401298464324817e-45
0x14067f5fc mov qword ptr [rbp + 0x1c0], rdx
0x14067f603 mov qword ptr [rbp + 0x1c8], 1
0x14067f60e mov qword ptr [rbp + 0x1d0], r14
0x14067f615 mov qword ptr [rbp + 0x1d8], 1
0x14067f620 mov qword ptr [rbp + 0x1e0], 0
0x14067f62b mov qword ptr [rbp + 0x170], 0
0x14067f636 mov qword ptr [rbp + 0x178], rcx
0x14067f63d mov qword ptr [rbp + 0x180], 0x15
0x14067f648 mov qword ptr [rbp + 0x188], 0
0x14067f653 movdqu xmmword ptr [rbp + 0x190], xmm0
0x14067f65b mov dword ptr [rbp + 0x1b8], 1
0x14067f665 mov dword ptr [rbp + 0x1bc], eax
0x14067f66b lea rcx, [rbp + 0x40]
0x14067f66f lea rdx, [rbp + 0x170]
0x14067f676 call 0x140d4a330
0x14067f67b cmp byte ptr [rbp + 0x1f0], 0
0x14067f682 jne 0x14067f69a
0x14067f684 mov rax, qword ptr [rip + 0x2facd95] ; 0x14362c420 f8936343010000007a29b54201000000 f32=227.5780029296875
0x14067f68b mov rax, qword ptr [rax]
0x14067f68e shl rax, 1
0x14067f691 test rax, rax
0x14067f694 jne 0x14067fb6d
0x14067f69a xor eax, eax
0x14067f69c mov rcx, qword ptr [rbp + 0x1f8]
0x14067f6a3 xchg byte ptr [rcx], al
0x14067f6a5 cmp al, 2
0x14067f6a7 je 0x14067fb63
0x14067f6ad mov rdx, qword ptr [rbp + 0xc0]
0x14067f6b4 test rdx, rdx
0x14067f6b7 je 0x14067f6cb
0x14067f6b9 mov rcx, qword ptr [rbp + 0xc8]
0x14067f6c0 mov r8d, 1
0x14067f6c6 call 0x140613c20
0x14067f6cb mov rbx, qword ptr [rbx + 0x10]
0x14067f6cf movabs rdx, 0x806c29bbf7c5030d
0x14067f6d9 mov rcx, rbx
0x14067f6dc call 0x14093c970
0x14067f6e1 mov r14, rdx
0x14067f6e4 test rdx, rdx
0x14067f6e7 je 0x14067fbfc
0x14067f6ed mov rcx, rbx
0x14067f6f0 mov rdx, rax
0x14067f6f3 call 0x14093d310
0x14067f6f8 mov ebx, eax
0x14067f6fa add r14, 0x18
0x14067f6fe mov rcx, r14
0x14067f701 call 0x140bd5790
0x14067f706 mov r14, rax
0x14067f709 lea rcx, [rbp + 0x80]
0x14067f710 call 0x140d4cc30
0x14067f715 lea rcx, [rip + 0x2faba9c] ; 0x14362b1b8 00000000000000000000000000000000 f32=0.0
0x14067f71c call 0x140c81e10
0x14067f721 xor ecx, ecx
0x14067f723 mov al, 1
0x14067f725 lock cmpxchg byte ptr [rip + 0x2faba8b], cl ; 0x14362b1b8 00000000000000000000000000000000 f32=0.0
0x14067f72d je 0x14067ec5d
0x14067f733 lea rcx, [rip + 0x2faba7e] ; 0x14362b1b8 00000000000000000000000000000000 f32=0.0
0x14067f73a xor edx, edx
0x14067f73c call 0x1428cdb90
0x14067f741 jmp 0x14067ec5d
0x14067f746 lea rcx, [rip + 0x2faba63] ; 0x14362b1b0 00000000000000000000000000000000 f32=0.0
0x14067f74d mov rdx, rcx
0x14067f750 call 0x1428e61b0
0x14067f755 mov cl, 1
0x14067f757 xor eax, eax
0x14067f759 lock cmpxchg byte ptr [rip + 0x2faba57], cl ; 0x14362b1b8 00000000000000000000000000000000 f32=0.0
0x14067f761 je 0x14067ee30
0x14067f767 lea rcx, [rip + 0x2faba4a] ; 0x14362b1b8 00000000000000000000000000000000 f32=0.0
0x14067f76e mov r8d, 0x3b9aca00
0x14067f774 call 0x1428cd210
0x14067f779 jmp 0x14067ee30
0x14067f77e mov r13, rbx
0x14067f781 movups xmm0, xmmword ptr [rbp + 0x100]
0x14067f788 movaps xmmword ptr [rbp + 0x150], xmm0
0x14067f78f movdqu xmm0, xmmword ptr [rbp + 0xc0]
0x14067f797 movups xmm1, xmmword ptr [rbp + 0xd0]
0x14067f79e movups xmm2, xmmword ptr [rbp + 0xe0]
0x14067f7a5 movups xmm3, xmmword ptr [rbp + 0xf0]
0x14067f7ac movaps xmmword ptr [rbp + 0x140], xmm3
0x14067f7b3 movaps xmmword ptr [rbp + 0x130], xmm2
0x14067f7ba movaps xmmword ptr [rbp + 0x120], xmm1
0x14067f7c1 movdqa xmmword ptr [rbp + 0x110], xmm0
0x14067f7c9 mov byte ptr [rbp + 0x204], 1
0x14067f7d0 lea rcx, [rbp + 0x170]
0x14067f7d7 call 0x140d56700
0x14067f7dc movups xmm0, xmmword ptr [rbp + 0x170]
0x14067f7e3 movups xmm1, xmmword ptr [rbp + 0x180]
0x14067f7ea movups xmm2, xmmword ptr [rbp + 0x190]
0x14067f7f1 movaps xmmword ptr [rbp + 0xa0], xmm2
0x14067f7f8 movaps xmmword ptr [rbp + 0x90], xmm1
0x14067f7ff movaps xmmword ptr [rbp + 0x80], xmm0
0x14067f806 movaps xmm0, xmmword ptr [rbp + 0x150]
0x14067f80d movups xmmword ptr [rbp + 0x1c0], xmm0
0x14067f814 movdqa xmm0, xmmword ptr [rbp + 0x110]
0x14067f81c movaps xmm1, xmmword ptr [rbp + 0x120]
0x14067f823 movaps xmm2, xmmword ptr [rbp + 0x130]
0x14067f82a movaps xmm3, xmmword ptr [rbp + 0x140]
0x14067f831 movups xmmword ptr [rbp + 0x1b0], xmm3
0x14067f838 movups xmmword ptr [rbp + 0x1a0], xmm2
0x14067f83f movups xmmword ptr [rbp + 0x190], xmm1
0x14067f846 movdqu xmmword ptr [rbp + 0x180], xmm0
0x14067f84e lea rax, [rip + 0x239792e] ; 0x142a17183 4372656174696e672061207465787475 f32=2.6453361358895317e+20
0x14067f855 mov qword ptr [rbp + 0x170], rax
0x14067f85c mov qword ptr [rbp + 0x178], 0x12
0x14067f867 mov byte ptr [rbp + 0x204], 0
0x14067f86e lea rcx, [rbp + 0x170]
0x14067f875 lea rdx, [rbp + 0x80]
0x14067f87c call 0x1428e5a00
0x14067f881 mov bl, 7
0x14067f883 mov r12, rax
0x14067f886 mov rdx, qword ptr [rbp + 0x1f8]
0x14067f88d test rdx, rdx
0x14067f890 je 0x14067f8a4
0x14067f892 mov r8d, 1
0x14067f898 mov rcx, qword ptr [rbp + 0x1f0]
0x14067f89f call 0x140613c20
0x14067f8a4 lea rcx, [rbp + 0x40]
0x14067f8a8 call 0x14067de40
0x14067f8ad cmp bl, 7
0x14067f8b0 jne 0x14067f8bd
0x14067f8b2 mov rax, r12
0x14067f8b5 mov rbx, r13
0x14067f8b8 jmp 0x14067f371
0x14067f8bd mov qword ptr [rbp + 0xc0], r12
0x14067f8c4 movaps xmm0, xmmword ptr [rbp - 0x30]
0x14067f8c8 movaps xmm1, xmmword ptr [rbp - 0x20]
0x14067f8cc movups xmmword ptr [rbp + 0xc8], xmm0
0x14067f8d3 movups xmmword ptr [rbp + 0xd8], xmm1
0x14067f8da movdqu xmm0, xmmword ptr [rbp - 0x12]
0x14067f8df movdqu xmmword ptr [rbp + 0xe6], xmm0
0x14067f8e7 mov byte ptr [rbp + 0xf6], bl
0x14067f8ed mov byte ptr [rbp + 0xf7], r15b
0x14067f8f4 lea rcx, [rbp + 0xd8]
0x14067f8fb mov byte ptr [rbp + 0x207], 1
0x14067f902 call 0x140bd5790
0x14067f907 mov r14, rax
0x14067f90a mov rbx, qword ptr [r13 + 0x10]
0x14067f90e mov rax, qword ptr [rbp + 0xf0]
0x14067f915 mov qword ptr [rbp + 0x1b8], rax
0x14067f91c movdqu xmm0, xmmword ptr [rbp + 0xc0]
0x14067f924 movups xmm1, xmmword ptr [rbp + 0xd0]
0x14067f92b movups xmm2, xmmword ptr [rbp + 0xe0]
0x14067f932 movups xmmword ptr [rbp + 0x1a8], xmm2
0x14067f939 movups xmmword ptr [rbp + 0x198], xmm1
0x14067f940 movdqu xmmword ptr [rbp + 0x188], xmm0
0x14067f948 movabs rax, 0x8000000000000000
0x14067f952 mov qword ptr [rbp + 0x170], rax
0x14067f959 mov r15, qword ptr [rdi + 0x10]
0x14067f95d test r15, r15
0x14067f960 js 0x14067fc12
0x14067f966 mov r12, qword ptr [rdi + 8]
0x14067f96a je 0x14067f99a
0x14067f96c movzx eax, byte ptr [rip + 0x2fb5296] ; 0x143634c09 5a1903301400006a140000745a190370 f32=4.769354911005053e-10
0x14067f973 mov eax, 1
0x14067f978 mov qword ptr [rbp + 0x1f8], rax
0x14067f97f mov edx, 1
0x14067f984 mov rcx, r15
0x14067f987 call 0x140613c10
0x14067f98c test rax, rax
0x14067f98f je 0x14067fc1d
0x14067f995 mov r13, rax
0x14067f998 jmp 0x14067f9a0
0x14067f99a mov r13d, 1
0x14067f9a0 mov rcx, r13
0x14067f9a3 mov rdx, r12
0x14067f9a6 mov r8, r15
0x14067f9a9 call 0x142923c00
0x14067f9ae mov qword ptr [rbp + 0x170], r15
0x14067f9b5 mov qword ptr [rbp + 0x178], r13
0x14067f9bc mov qword ptr [rbp + 0x180], r15
0x14067f9c3 movdqu xmm0, xmmword ptr [rbp + 0x190]
0x14067f9cb movups xmm1, xmmword ptr [rbp + 0x1a0]
0x14067f9d2 movups xmm2, xmmword ptr [rbp + 0x1b0]
0x14067f9d9 movdqa xmmword ptr [rbp + 0x130], xmm0
0x14067f9e1 movaps xmmword ptr [rbp + 0x140], xmm1
0x14067f9e8 movaps xmmword ptr [rbp + 0x150], xmm2
0x14067f9ef mov rax, qword ptr [rbp + 0x170]
0x14067f9f6 mov qword ptr [rbp + 0x110], rax
0x14067f9fd mov rax, qword ptr [rbp + 0x178]
0x14067fa04 mov qword ptr [rbp + 0x118], rax
0x14067fa0b mov rax, qword ptr [rbp + 0x180]
0x14067fa12 mov qword ptr [rbp + 0x120], rax
0x14067fa19 mov rax, qword ptr [rbp + 0x188]
0x14067fa20 mov qword ptr [rbp + 0x128], rax
0x14067fa27 mov byte ptr [rbp + 0x207], 0
0x14067fa2e lea rdx, [rbp + 0x110]
0x14067fa35 mov rcx, rbx
0x14067fa38 call 0x14093c200
0x14067fa3d mov byte ptr [rbp + 0x207], 0
0x14067fa44 mov rcx, rbx
0x14067fa47 mov rdx, rax
0x14067fa4a call 0x14093d310
0x14067fa4f mov r15, qword ptr [rbp + 0xb8]
0x14067fa56 test r15, r15
0x14067fa59 js 0x14067fc35
0x14067fa5f mov ebx, eax
0x14067fa61 mov r12, qword ptr [rbp + 0xb0]
0x14067fa68 je 0x14067fa98
0x14067fa6a movzx eax, byte ptr [rip + 0x2fb5198] ; 0x143634c09 5a1903301400006a140000745a190370 f32=4.769354911005053e-10
0x14067fa71 mov eax, 1
0x14067fa76 mov qword ptr [rbp + 0x1f8], rax
0x14067fa7d mov edx, 1
0x14067fa82 mov rcx, r15
0x14067fa85 call 0x140613c10
0x14067fa8a test rax, rax
0x14067fa8d je 0x14067fc40
0x14067fa93 mov r13, rax
0x14067fa96 jmp 0x14067fa9e
0x14067fa98 mov r13d, 1
0x14067fa9e mov rcx, r13
0x14067faa1 mov rdx, r12
0x14067faa4 mov r8, r15
0x14067faa7 call 0x142923c00
0x14067faac mov qword ptr [rbp + 0x170], r15
0x14067fab3 mov qword ptr [rbp + 0x178], r13
0x14067faba mov qword ptr [rbp + 0x180], r15
0x14067fac1 mov dword ptr [rbp + 0x88], ebx
0x14067fac7 mov qword ptr [rbp + 0x80], r14
0x14067face mov byte ptr [rbp + 0x207], 0
0x14067fad5 lea rdx, [rip + 0x2fab6e4] ; 0x14362b1c0 00000000000000000000000000000000 f32=0.0
0x14067fadc lea rcx, [rbp + 0x110]
0x14067fae3 lea r8, [rbp + 0x170]
0x14067faea lea r9, [rbp + 0x80]
0x14067faf1 call 0x140fb1cc0
0x14067faf6 jmp 0x14067f715
0x14067fafb mov byte ptr [rbp + 0x110], 1
0x14067fb02 mov qword ptr [rbp + 0x170], r14
0x14067fb09 lea rax, [rip + 0x23975c0] ; 0x142a170d0 f86fa142010000002900000000000000 f32=80.71868896484375
0x14067fb10 mov qword ptr [rsp + 0x20], rax
0x14067fb15 lea rcx, [rip + 0x2fb5114] ; 0x143634c30 f01400002a150000bc5a190330150000 f32=7.51095976878102e-42
0x14067fb1c lea r9, [rip + 0x239734d] ; 0x142a16e70 00000000000000000800000000000000 f32=0.0
0x14067fb23 lea r8, [rbp + 0x170]
0x14067fb2a xor edx, edx
0x14067fb2c call 0x1428d8820
0x14067fb31 jmp 0x14067f41d
0x14067fb36 mov qword ptr [rsp + 0x20], 1
0x14067fb3f lea rcx, [rbp + 0x110]
0x14067fb46 mov r9d, 1
0x14067fb4c mov rdx, r15
0x14067fb4f mov r8, r12
0x14067fb52 call 0x1428a58d0
0x14067fb57 mov r15, qword ptr [rbp + 0x120]
0x14067fb5e jmp 0x14067f53a
0x14067fb63 call 0x1428d8810
0x14067fb68 jmp 0x14067f6ad
0x14067fb6d call 0x1428d86b0
0x14067fb72 test al, al
0x14067fb74 jne 0x14067f69a
0x14067fb7a mov rax, qword ptr [rbp + 0x1f8]
0x14067fb81 mov byte ptr [rax + 1], 1
0x14067fb85 jmp 0x14067f69a
0x14067fb8a lea r8, [rip + 0x23975af] ; 0x142a17140 f86fa142010000002900000000000000 f32=80.71868896484375
0x14067fb91 mov rcx, r15
0x14067fb94 mov rdx, r14
0x14067fb97 call 0x1428d9330
0x14067fb9c mov rax, qword ptr [rbp + 0x178]
0x14067fba3 movzx ecx, byte ptr [rbp + 0x180]
0x14067fbaa mov qword ptr [rbp + 0x110], rax
0x14067fbb1 mov byte ptr [rbp + 0x118], cl
0x14067fbb7 lea rax, [rip + 0x2397512] ; 0x142a170d0 f86fa142010000002900000000000000 f32=80.71868896484375
0x14067fbbe mov qword ptr [rsp + 0x20], rax
0x14067fbc3 lea rcx, [rip + 0x239751e] ; 0x142a170e8 4d757465782077617320706f69736f6e f32=7.21513274569724e+22
0x14067fbca lea r9, [rip + 0x2397337] ; 0x142a16f08 70285d40010000001000000000000000 f32=3.4555931091308594
0x14067fbd1 lea r8, [rbp + 0x110]
0x14067fbd8 mov edx, 0x13
0x14067fbdd call 0x1428d9760
0x14067fbe2 jmp 0x14067fc5d
0x14067fbe4 lea r8, [rip + 0x239738d] ; 0x142a16f78 286fa142010000004a00000000000000 f32=80.71710205078125
0x14067fbeb mov ecx, 1
0x14067fbf0 mov edx, 0xf
0x14067fbf5 call 0x1428d8fe3
0x14067fbfa jmp 0x14067fc5d
0x14067fbfc mov rcx, qword ptr [rdi + 8]
0x14067fc00 mov rdx, qword ptr [rdi + 0x10]
0x14067fc04 lea r8, [rip + 0x239751d] ; 0x142a17128 f86fa142010000002900000000000000 f32=80.71868896484375
0x14067fc0b call 0x1428d9330
0x14067fc10 jmp 0x14067fc5d
0x14067fc12 mov qword ptr [rbp + 0x1f8], 0
0x14067fc1d lea r8, [rip + 0x2532b7c] ; 0x142bb27a0 5027bb42010000004a00000000000000 f32=93.5767822265625
0x14067fc24 mov rcx, qword ptr [rbp + 0x1f8]
0x14067fc2b mov rdx, r15
0x14067fc2e call 0x1428d8fe3
0x14067fc33 jmp 0x14067fc5d
0x14067fc35 mov qword ptr [rbp + 0x1f8], 0
0x14067fc40 mov byte ptr [rbp + 0x207], 0
0x14067fc47 lea r8, [rip + 0x239732a] ; 0x142a16f78 286fa142010000004a00000000000000 f32=80.71710205078125
0x14067fc4e mov rcx, qword ptr [rbp + 0x1f8]
0x14067fc55 mov rdx, r15
0x14067fc58 call 0x1428d8fe3
0x14067fc5d ud2 
0x14067fc5f nop 
0x14067fc60 mov qword ptr [rsp + 0x10], rdx
0x14067fc65 push rbp
0x14067fc66 push r15
0x14067fc68 push r14
0x14067fc6a push r13
0x14067fc6c push r12
0x14067fc6e push rsi
0x14067fc6f push rdi
0x14067fc70 push rbx
0x14067fc71 sub rsp, 0x68
0x14067fc75 lea rbp, [rdx + 0x80]
0x14067fc7c movdqa xmmword ptr [rsp + 0x30], xmm8
0x14067fc83 movdqa xmmword ptr [rsp + 0x40], xmm7
0x14067fc89 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14067fc8f lea rcx, [rip + 0x2fab522] ; 0x14362b1b8 00000000000000000000000000000000 f32=0.0
0x14067fc96 call 0x140161c30
0x14067fc9b movaps xmm6, xmmword ptr [rsp + 0x50]
0x14067fca0 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14067fca5 movaps xmm8, xmmword ptr [rsp + 0x30]
0x14067fcab add rsp, 0x68
0x14067fcaf pop rbx
0x14067fcb0 pop rdi
0x14067fcb1 pop rsi
0x14067fcb2 pop r12
0x14067fcb4 pop r13
0x14067fcb6 pop r14
0x14067fcb8 pop r15
0x14067fcba pop rbp
0x14067fcbb ret 
0x14067fcbc nop dword ptr [rax]
0x14067fcc0 mov qword ptr [rsp + 0x10], rdx
0x14067fcc5 push rbp
0x14067fcc6 push r15
0x14067fcc8 push r14
0x14067fcca push r13
0x14067fccc push r12
0x14067fcce push rsi
0x14067fccf push rdi
0x14067fcd0 push rbx
0x14067fcd1 sub rsp, 0x68
0x14067fcd5 lea rbp, [rdx + 0x80]
0x14067fcdc movdqa xmmword ptr [rsp + 0x30], xmm8
0x14067fce3 movdqa xmmword ptr [rsp + 0x40], xmm7
0x14067fce9 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14067fcef cmp byte ptr [rbp + 0x205], 0
0x14067fcf6 je 0x14067fd04
0x14067fcf8 lea rcx, [rbp + 0x110]
0x14067fcff call 0x140fb2da0
0x14067fd04 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14067fd0a movdqa xmm7, xmmword ptr [rsp + 0x40]
0x14067fd10 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14067fd17 add rsp, 0x68
0x14067fd1b pop rbx
0x14067fd1c pop rdi
0x14067fd1d pop rsi
0x14067fd1e pop r12
0x14067fd20 pop r13
0x14067fd22 pop r14
0x14067fd24 pop r15
0x14067fd26 pop rbp
0x14067fd27 ret 
0x14067fd28 nop dword ptr [rax + rax]
0x14067fd30 mov qword ptr [rsp + 0x10], rdx
0x14067fd35 push rbp
0x14067fd36 push r15
0x14067fd38 push r14
0x14067fd3a push r13
0x14067fd3c push r12
0x14067fd3e push rsi
0x14067fd3f push rdi
0x14067fd40 push rbx
0x14067fd41 sub rsp, 0x68
0x14067fd45 lea rbp, [rdx + 0x80]
0x14067fd4c movdqa xmmword ptr [rsp + 0x30], xmm8
0x14067fd53 movdqa xmmword ptr [rsp + 0x40], xmm7
0x14067fd59 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14067fd5f lea rcx, [rbp + 0x40]
0x14067fd63 call 0x14067de40
0x14067fd68 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14067fd6e movdqa xmm7, xmmword ptr [rsp + 0x40]
0x14067fd74 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14067fd7b add rsp, 0x68
0x14067fd7f pop rbx
0x14067fd80 pop rdi
0x14067fd81 pop rsi
0x14067fd82 pop r12
0x14067fd84 pop r13
0x14067fd86 pop r14
0x14067fd88 pop r15
0x14067fd8a pop rbp
0x14067fd8b ret 
0x14067fd8c nop dword ptr [rax]
0x14067fd90 mov qword ptr [rsp + 0x10], rdx
0x14067fd95 push rbp
0x14067fd96 push r15
0x14067fd98 push r14
0x14067fd9a push r13
0x14067fd9c push r12
0x14067fd9e push rsi
0x14067fd9f push rdi
0x14067fda0 push rbx
0x14067fda1 sub rsp, 0x68
0x14067fda5 lea rbp, [rdx + 0x80]
0x14067fdac movdqa xmmword ptr [rsp + 0x30], xmm8
0x14067fdb3 movdqa xmmword ptr [rsp + 0x40], xmm7
0x14067fdb9 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14067fdbf cmp qword ptr [rbp + 0x1f8], 0
0x14067fdc7 je 0x14067fde2
0x14067fdc9 mov r8d, 1
0x14067fdcf mov rcx, qword ptr [rbp + 0x1f0]
0x14067fdd6 mov rdx, qword ptr [rbp + 0x1f8]
0x14067fddd call 0x140613c20
0x14067fde2 mov byte ptr [rbp + 0x206], 0
0x14067fde9 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14067fdef movdqa xmm7, xmmword ptr [rsp + 0x40]
0x14067fdf5 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14067fdfc add rsp, 0x68
0x14067fe00 pop rbx
0x14067fe01 pop rdi
0x14067fe02 pop rsi
0x14067fe03 pop r12
0x14067fe05 pop r13
0x14067fe07 pop r14
0x14067fe09 pop r15
0x14067fe0b pop rbp
0x14067fe0c ret 
0x14067fe0d nop dword ptr [rax]
0x14067fe10 mov qword ptr [rsp + 0x10], rdx
0x14067fe15 push rbp
0x14067fe16 push r15
0x14067fe18 push r14
0x14067fe1a push r13
0x14067fe1c push r12
0x14067fe1e push rsi
0x14067fe1f push rdi
0x14067fe20 push rbx
0x14067fe21 sub rsp, 0x68
0x14067fe25 lea rbp, [rdx + 0x80]
0x14067fe2c movdqa xmmword ptr [rsp + 0x30], xmm8
0x14067fe33 movdqa xmmword ptr [rsp + 0x40], xmm7
0x14067fe39 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14067fe3f cmp byte ptr [rbp + 0x204], 0
0x14067fe46 je 0x14067fe54
0x14067fe48 lea rcx, [rbp + 0x110]
0x14067fe4f call 0x1407cea70
0x14067fe54 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14067fe5a movdqa xmm7, xmmword ptr [rsp + 0x40]
0x14067fe60 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14067fe67 add rsp, 0x68
0x14067fe6b pop rbx
0x14067fe6c pop rdi
0x14067fe6d pop rsi
0x14067fe6e pop r12
0x14067fe70 pop r13
0x14067fe72 pop r14
0x14067fe74 pop r15
0x14067fe76 pop rbp
0x14067fe77 ret 
0x14067fe78 nop dword ptr [rax + rax]
0x14067fe80 mov qword ptr [rsp + 0x10], rdx
0x14067fe85 push rbp
0x14067fe86 push r15
0x14067fe88 push r14
0x14067fe8a push r13
0x14067fe8c push r12
0x14067fe8e push rsi
0x14067fe8f push rdi
0x14067fe90 push rbx
0x14067fe91 sub rsp, 0x68
0x14067fe95 lea rbp, [rdx + 0x80]
0x14067fe9c movdqa xmmword ptr [rsp + 0x30], xmm8
0x14067fea3 movdqa xmmword ptr [rsp + 0x40], xmm7
0x14067fea9 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14067feaf cmp byte ptr [rbp + 0x206], 0
0x14067feb6 je 0x14067fef4
0x14067feb8 mov rdx, qword ptr [rbp + 0x20]
0x14067febc test rdx, rdx
0x14067febf je 0x14067fed0
0x14067fec1 mov rcx, qword ptr [rbp + 0x28]
0x14067fec5 mov r8d, 1
0x14067fecb call 0x140613c20
0x14067fed0 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14067fed6 movdqa xmm7, xmmword ptr [rsp + 0x40]
0x14067fedc movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14067fee3 add rsp, 0x68
0x14067fee7 pop rbx
0x14067fee8 pop rdi
0x14067fee9 pop rsi
0x14067feea pop r12
0x14067feec pop r13
0x14067feee pop r14
0x14067fef0 pop r15
0x14067fef2 pop rbp
0x14067fef3 ret 
0x14067fef4 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14067fefa movdqa xmm7, xmmword ptr [rsp + 0x40]
0x14067ff00 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14067ff07 add rsp, 0x68
0x14067ff0b pop rbx
0x14067ff0c pop rdi
0x14067ff0d pop rsi
0x14067ff0e pop r12
0x14067ff10 pop r13
0x14067ff12 pop r14
0x14067ff14 pop r15
0x14067ff16 pop rbp
0x14067ff17 ret 
0x14067ff18 nop dword ptr [rax + rax]
0x14067ff20 mov qword ptr [rsp + 0x10], rdx
0x14067ff25 push rbp
0x14067ff26 push r15
0x14067ff28 push r14
0x14067ff2a push r13
0x14067ff2c push r12
0x14067ff2e push rsi
0x14067ff2f push rdi
0x14067ff30 push rbx
0x14067ff31 sub rsp, 0x68
0x14067ff35 lea rbp, [rdx + 0x80]
0x14067ff3c movdqa xmmword ptr [rsp + 0x30], xmm8
0x14067ff43 movdqa xmmword ptr [rsp + 0x40], xmm7
0x14067ff49 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14067ff4f cmp byte ptr [rbp + 0x207], 0
0x14067ff56 je 0x14067ff88
0x14067ff58 lea rcx, [rbp + 0xc0]
0x14067ff5f call 0x1405ee5e0
0x14067ff64 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14067ff6a movdqa xmm7, xmmword ptr [rsp + 0x40]
0x14067ff70 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14067ff77 add rsp, 0x68
0x14067ff7b pop rbx
0x14067ff7c pop rdi
0x14067ff7d pop rsi
0x14067ff7e pop r12
0x14067ff80 pop r13
0x14067ff82 pop r14
0x14067ff84 pop r15
0x14067ff86 pop rbp
0x14067ff87 ret 
0x14067ff88 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14067ff8e movdqa xmm7, xmmword ptr [rsp + 0x40]
0x14067ff94 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14067ff9b add rsp, 0x68
0x14067ff9f pop rbx
0x14067ffa0 pop rdi
0x14067ffa1 pop rsi
0x14067ffa2 pop r12
0x14067ffa4 pop r13
0x14067ffa6 pop r14
0x14067ffa8 pop r15
0x14067ffaa pop rbp
0x14067ffab ret 
0x14067ffac nop dword ptr [rax]
0x14067ffb0 mov qword ptr [rsp + 0x10], rdx
0x14067ffb5 push rbp
0x14067ffb6 push r15
0x14067ffb8 push r14
0x14067ffba push r13
0x14067ffbc push r12
0x14067ffbe push rsi
0x14067ffbf push rdi
0x14067ffc0 push rbx
0x14067ffc1 sub rsp, 0x68
0x14067ffc5 lea rbp, [rdx + 0x80]
0x14067ffcc movdqa xmmword ptr [rsp + 0x30], xmm8
0x14067ffd3 movdqa xmmword ptr [rsp + 0x40], xmm7
0x14067ffd9 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14067ffdf lea rcx, [rbp + 0x80]
0x14067ffe6 call 0x140d4cc30
0x14067ffeb movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14067fff1 movdqa xmm7, xmmword ptr [rsp + 0x40]
0x14067fff7 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14067fffe add rsp, 0x68
0x140680002 pop rbx
0x140680003 pop rdi
0x140680004 pop rsi
0x140680005 pop r12
0x140680007 pop r13
0x140680009 pop r14
0x14068000b pop r15
0x14068000d pop rbp
0x14068000e ret 
0x14068000f nop 
0x140680010 mov qword ptr [rsp + 0x10], rdx
0x140680015 push rbp
0x140680016 push r15
0x140680018 push r14
0x14068001a push r13
0x14068001c push r12
0x14068001e push rsi
0x14068001f push rdi
0x140680020 push rbx
0x140680021 sub rsp, 0x68
0x140680025 lea rbp, [rdx + 0x80]
0x14068002c movdqa xmmword ptr [rsp + 0x30], xmm8
0x140680033 movdqa xmmword ptr [rsp + 0x40], xmm7
0x140680039 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14068003f mov rdx, qword ptr [rbp + 0xc0]
0x140680046 test rdx, rdx
0x140680049 je 0x14068005d
0x14068004b mov rcx, qword ptr [rbp + 0xc8]
0x140680052 mov r8d, 1
0x140680058 call 0x140613c20
0x14068005d movdqa xmm6, xmmword ptr [rsp + 0x50]
0x140680063 movdqa xmm7, xmmword ptr [rsp + 0x40]
0x140680069 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x140680070 add rsp, 0x68
0x140680074 pop rbx
0x140680075 pop rdi
0x140680076 pop rsi
0x140680077 pop r12
0x140680079 pop r13
0x14068007b pop r14
0x14068007d pop r15
0x14068007f pop rbp
0x140680080 ret 
0x140680081 nop word ptr cs:[rax + rax]
0x140680090 mov qword ptr [rsp + 0x10], rdx
0x140680095 push rbp
0x140680096 push r15
0x140680098 push r14
0x14068009a push r13
0x14068009c push r12
0x14068009e push rsi
0x14068009f push rdi
0x1406800a0 push rbx
0x1406800a1 sub rsp, 0x68
0x1406800a5 lea rbp, [rdx + 0x80]
0x1406800ac movdqa xmmword ptr [rsp + 0x30], xmm8
0x1406800b3 movdqa xmmword ptr [rsp + 0x40], xmm7
0x1406800b9 movdqa xmmword ptr [rsp + 0x50], xmm6
0x1406800bf lea rcx, [rbp + 0x110]
0x1406800c6 call 0x1405d2870
0x1406800cb movdqa xmm6, xmmword ptr [rsp + 0x50]
0x1406800d1 movdqa xmm7, xmmword ptr [rsp + 0x40]
0x1406800d7 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x1406800de add rsp, 0x68
0x1406800e2 pop rbx
0x1406800e3 pop rdi
0x1406800e4 pop rsi
0x1406800e5 pop r12
0x1406800e7 pop r13
0x1406800e9 pop r14
0x1406800eb pop r15
0x1406800ed pop rbp
0x1406800ee ret 
0x1406800ef nop 
0x1406800f0 mov qword ptr [rsp + 0x10], rdx
0x1406800f5 push rbp
0x1406800f6 push r15
0x1406800f8 push r14
0x1406800fa push r13
0x1406800fc push r12
0x1406800fe push rsi
0x1406800ff push rdi
0x140680100 push rbx
0x140680101 sub rsp, 0x68
0x140680105 lea rbp, [rdx + 0x80]
0x14068010c movdqa xmmword ptr [rsp + 0x30], xmm8
0x140680113 movdqa xmmword ptr [rsp + 0x40], xmm7
0x140680119 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14068011f mov rcx, qword ptr [rbp + 0x1f8]
0x140680126 movzx edx, byte ptr [rbp + 0x1f0]
0x14068012d call 0x140043000
0x140680132 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x140680138 movdqa xmm7, xmmword ptr [rsp + 0x40]
0x14068013e movdqa xmm8, xmmword ptr [rsp + 0x30]
0x140680145 add rsp, 0x68
0x140680149 pop rbx
0x14068014a pop rdi
0x14068014b pop rsi
0x14068014c pop r12
0x14068014e pop r13
0x140680150 pop r14
0x140680152 pop r15
0x140680154 pop rbp
0x140680155 ret 
0x140680156 nop word ptr cs:[rax + rax]
0x140680160 mov qword ptr [rsp + 0x10], rdx
0x140680165 push rbp
0x140680166 push r15
0x140680168 push r14
0x14068016a push r13
0x14068016c push r12
0x14068016e push rsi
0x14068016f push rdi
0x140680170 push rbx
0x140680171 sub rsp, 0x68
0x140680175 lea rbp, [rdx + 0x80]
0x14068017c movdqa xmmword ptr [rsp + 0x30], xmm8
0x140680183 movdqa xmmword ptr [rsp + 0x40], xmm7
0x140680189 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14068018f mov rdx, qword ptr [rbp + 0x170]
0x140680196 test rdx, rdx
0x140680199 je 0x1406801ad
0x14068019b mov rcx, qword ptr [rbp + 0x178]
0x1406801a2 mov r8d, 1
0x1406801a8 call 0x140613c20
0x1406801ad movdqa xmm6, xmmword ptr [rsp + 0x50]
0x1406801b3 movdqa xmm7, xmmword ptr [rsp + 0x40]
0x1406801b9 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x1406801c0 add rsp, 0x68
0x1406801c4 pop rbx
0x1406801c5 pop rdi
0x1406801c6 pop rsi
0x1406801c7 pop r12
0x1406801c9 pop r13
0x1406801cb pop r14
0x1406801cd pop r15
0x1406801cf pop rbp
0x1406801d0 ret 
0x1406801d1 nop word ptr cs:[rax + rax]
0x1406801e0 mov qword ptr [rsp + 0x10], rdx
0x1406801e5 push rbp
0x1406801e6 push r15
0x1406801e8 push r14
0x1406801ea push r13
0x1406801ec push r12
0x1406801ee push rsi
0x1406801ef push rdi
0x1406801f0 push rbx
0x1406801f1 sub rsp, 0x68
0x1406801f5 lea rbp, [rdx + 0x80]
0x1406801fc movdqa xmmword ptr [rsp + 0x30], xmm8
0x140680203 movdqa xmmword ptr [rsp + 0x40], xmm7
0x140680209 movdqa xmmword ptr [rsp + 0x50], xmm6
0x14068020f mov rdx, qword ptr [rbp + 0x110]
0x140680216 test rdx, rdx
0x140680219 je 0x14068022d
0x14068021b mov rcx, qword ptr [rbp + 0x118]
0x140680222 mov r8d, 1
0x140680228 call 0x140613c20
0x14068022d movdqa xmm6, xmmword ptr [rsp + 0x50]
0x140680233 movdqa xmm7, xmmword ptr [rsp + 0x40]
0x140680239 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x140680240 add rsp, 0x68
0x140680244 pop rbx
0x140680245 pop rdi
0x140680246 pop rsi
0x140680247 pop r12
0x140680249 pop r13
0x14068024b pop r14
0x14068024d pop r15
0x14068024f pop rbp
0x140680250 ret 
0x140680251 nop word ptr cs:[rax + rax]
0x140680260 mov qword ptr [rsp + 0x10], rdx
0x140680265 push rbp
0x140680266 push r15
0x140680268 push r14
0x14068026a push r13
0x14068026c push r12
0x14068026e push rsi
0x14068026f push rdi
0x140680270 push rbx
0x140680271 sub rsp, 0x68
0x140680275 lea rbp, [rdx + 0x80]
0x14068027c movdqa xmmword ptr [rsp + 0x30], xmm8
0x140680283 movaps xmmword ptr [rsp + 0x40], xmm7
0x140680288 movaps xmmword ptr [rsp + 0x50], xmm6
0x14068028d lea rcx, [rbp + 0x170]
0x140680294 call 0x1405f11f0
0x140680299 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14068029e movaps xmm7, xmmword ptr [rsp + 0x40]
0x1406802a3 movaps xmm8, xmmword ptr [rsp + 0x30]
0x1406802a9 add rsp, 0x68
0x1406802ad pop rbx
0x1406802ae pop rdi
0x1406802af pop rsi
0x1406802b0 pop r12
0x1406802b2 pop r13
0x1406802b4 pop r14
0x1406802b6 pop r15
0x1406802b8 pop rbp
0x1406802b9 ret 
0x1406802ba nop word ptr [rax + rax]
0x1406802c0 mov qword ptr [rsp + 0x10], rdx
0x1406802c5 push rbp
0x1406802c6 push r15
0x1406802c8 push r14
0x1406802ca push r13
0x1406802cc push r12
0x1406802ce push rsi
0x1406802cf push rdi
0x1406802d0 push rbx
0x1406802d1 sub rsp, 0x68
0x1406802d5 lea rbp, [rdx + 0x80]
0x1406802dc movdqa xmmword ptr [rsp + 0x30], xmm8
0x1406802e3 movdqa xmmword ptr [rsp + 0x40], xmm7
0x1406802e9 movdqa xmmword ptr [rsp + 0x50], xmm6
0x1406802ef mov byte ptr [rbp + 0x207], 0
0x1406802f6 lea rcx, [rbp + 0x170]
0x1406802fd call 0x140898b70
0x140680302 mov byte ptr [rbp + 0x207], 0
0x140680309 movdqa xmm6, xmmword ptr [rsp + 0x50]
0x14068030f movdqa xmm7, xmmword ptr [rsp + 0x40]
0x140680315 movdqa xmm8, xmmword ptr [rsp + 0x30]
0x14068031c add rsp, 0x68
0x140680320 pop rbx
0x140680321 pop rdi
0x140680322 pop rsi
0x140680323 pop r12
0x140680325 pop r13
0x140680327 pop r14
0x140680329 pop r15
0x14068032b pop rbp
0x14068032c ret 