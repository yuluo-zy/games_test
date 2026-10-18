0x14068eb50 push rbp
0x14068eb51 push r15
0x14068eb53 push r14
0x14068eb55 push r13
0x14068eb57 push r12
0x14068eb59 push rsi
0x14068eb5a push rdi
0x14068eb5b push rbx
0x14068eb5c sub rsp, 0x1e8
0x14068eb63 lea rbp, [rsp + 0x80]
0x14068eb6b movaps xmmword ptr [rbp + 0x150], xmm9
0x14068eb73 movaps xmmword ptr [rbp + 0x140], xmm8
0x14068eb7b movaps xmmword ptr [rbp + 0x130], xmm7
0x14068eb82 movaps xmmword ptr [rbp + 0x120], xmm6
0x14068eb89 mov qword ptr [rbp + 0x118], 0xfffffffffffffffe
0x14068eb94 mov rsi, r9
0x14068eb97 mov qword ptr [rbp + 0x18], rdx
0x14068eb9b mov rdi, rcx
0x14068eb9e xor r14d, r14d
0x14068eba1 cmp r14, qword ptr [r9 + 0x30]
0x14068eba5 jo 0x14068ebf9
0x14068eba7 movups xmm0, xmmword ptr [rsi + 0x38]
0x14068ebab movups xmmword ptr [rbp + 0x50], xmm0
0x14068ebaf lea rcx, [rbp + 0x50]
0x14068ebb3 call 0x1405bdda0
0x14068ebb8 mov qword ptr [rbp + 0x50], rax
0x14068ebbc movabs rax, 0x41223169ff28813b
0x14068ebc6 mov qword ptr [rbp + 0x58], rax
0x14068ebca movabs rax, 0xa79b7268a2a968d9
0x14068ebd4 mov qword ptr [rbp + 0x60], rax
0x14068ebd8 lea rcx, [rbp + 0x50]
0x14068ebdc call 0x1405bdf20
0x14068ebe1 mov rcx, qword ptr [rbp + 0x18]
0x14068ebe5 mov qword ptr [rbp + 0x50], rcx
0x14068ebe9 mov qword ptr [rbp + 0x58], rax
0x14068ebed lea rcx, [rbp + 0x50]
0x14068ebf1 call 0x1405bd910
0x14068ebf6 mov r8, rax
0x14068ebf9 lea rbx, [rbp + 0x50]
0x14068ebfd mov rcx, rbx
0x14068ec00 mov rdx, rdi
0x14068ec03 mov qword ptr [rbp - 8], r8
0x14068ec07 call 0x1406861b0
0x14068ec0c mov rcx, rbx
0x14068ec0f xor edx, edx
0x14068ec11 call 0x140f9d080
0x14068ec16 movups xmm0, xmmword ptr [rbp + 0x50]
0x14068ec1a movaps xmmword ptr [rbp + 0xe0], xmm0
0x14068ec21 mov rax, qword ptr [rbp + 0x60]
0x14068ec25 mov qword ptr [rbp + 0xf0], rax
0x14068ec2c lea r8, [rip + 0x2389885] ; 0x142a184b8 6084a142010000005200000000000000 f32=80.758544921875
0x14068ec33 lea rdi, [rbp + 0x50]
0x14068ec37 mov rcx, rdi
0x14068ec3a mov rdx, rsi
0x14068ec3d call 0x140fcdfd0
0x14068ec42 lea rcx, [rbp + 0xc8]
0x14068ec49 lea rdx, [rbp + 0xe0]
0x14068ec50 mov r8, rdi
0x14068ec53 call 0x1406803f0
0x14068ec58 movups xmm0, xmmword ptr [rbp + 0xc8]
0x14068ec5f movaps xmmword ptr [rbp + 0x20], xmm0
0x14068ec63 mov rax, qword ptr [rbp + 0xd8]
0x14068ec6a mov qword ptr [rbp + 0x30], rax
0x14068ec6e xorps xmm0, xmm0
0x14068ec71 movaps xmmword ptr [rbp - 0x30], xmm0
0x14068ec75 movaps xmmword ptr [rbp - 0x20], xmm0
0x14068ec79 mov rax, qword ptr [rsi + 0x28]
0x14068ec7d test rax, rax
0x14068ec80 je 0x14068ecd5
0x14068ec82 mov rcx, qword ptr [rsi + 0x20]
0x14068ec86 shl rax, 5
0x14068ec8a mov bl, 4
0x14068ec8c xor edi, edi
0x14068ec8e xorps xmm6, xmm6
0x14068ec91 mov edx, dword ptr [rcx + rdi]
0x14068ec94 add edx, -2
0x14068ec97 cmp edx, 7
0x14068ec9a mov r8d, 7
0x14068eca0 cmovb r8d, edx
0x14068eca4 xor r13d, r13d
0x14068eca7 lea rdx, [rip + 0x2389976] ; 0x142a18624 8c74c7fdc374c7fdf274c7fde274c7fd f32=-3.31401896417666e+37
0x14068ecae movsxd r11, dword ptr [rdx + r8*4]
0x14068ecb2 add r11, rdx
0x14068ecb5 xor r12d, r12d
0x14068ecb8 xorps xmm8, xmm8
0x14068ecbc xor r15d, r15d
0x14068ecbf xor r14d, r14d
0x14068ecc2 xorps xmm9, xmm9
0x14068ecc6 xorps xmm7, xmm7
0x14068ecc9 xor r8d, r8d
0x14068eccc xor r9d, r9d
0x14068eccf mov r10, rcx
0x14068ecd2 jmp r11
0x14068ecd5 mov byte ptr [rbp + 0x110], 4
0x14068ecdc xorps xmm7, xmm7
0x14068ecdf xorps xmm9, xmm9
0x14068ece3 xor r15d, r15d
0x14068ece6 xorps xmm8, xmm8
0x14068ecea xor r12d, r12d
0x14068eced xorps xmm6, xmm6
0x14068ecf0 xor edi, edi
0x14068ecf2 xor r13d, r13d
0x14068ecf5 movdqa xmm0, xmmword ptr [rbp + 0x20]
0x14068ecfa movdqa xmmword ptr [rbp + 0x50], xmm0
0x14068ecff mov rax, qword ptr [rbp + 0x30]
0x14068ed03 mov qword ptr [rbp + 0x60], rax
0x14068ed07 mov rbx, qword ptr [rbp + 0x50]
0x14068ed0b cmp qword ptr [rbx + 0x1b8], 0
0x14068ed13 je 0x14068f975
0x14068ed19 lea rdx, [rbp + 0x58]
0x14068ed1d lea rcx, [rbx + 0x1c0]
0x14068ed24 call 0x1405b0d70
0x14068ed29 mov rcx, qword ptr [rbx + 0x1a0]
0x14068ed30 mov rdx, qword ptr [rbx + 0x1a8]
0x14068ed37 mov r8, rdx
0x14068ed3a and r8, rax
0x14068ed3d shr rax, 0x39
0x14068ed41 movd xmm0, eax
0x14068ed45 punpcklbw xmm0, xmm0
0x14068ed49 pshuflw xmm0, xmm0, 0
0x14068ed4e pshufd xmm0, xmm0, 0
0x14068ed53 lea rax, [rcx - 0xc0]
0x14068ed5a xor r9d, r9d
0x14068ed5d mov r10, qword ptr [rbp + 0x58]
0x14068ed61 pcmpeqd xmm1, xmm1
0x14068ed65 movdqu xmm2, xmmword ptr [rcx + r8]
0x14068ed6b movdqa xmm3, xmm2
0x14068ed6f pcmpeqb xmm3, xmm0
0x14068ed73 pmovmskb r11d, xmm3
0x14068ed78 test r11d, r11d
0x14068ed7b je 0x14068edb0
0x14068ed7d tzcnt ebx, r11d
0x14068ed82 add rbx, r8
0x14068ed85 and rbx, rdx
0x14068ed88 neg rbx
0x14068ed8b lea rbx, [rbx + rbx*2]
0x14068ed8f shl rbx, 6
0x14068ed93 cmp r10, qword ptr [rax + rbx]
0x14068ed97 je 0x14068edd2
0x14068ed99 lea ebx, [r11 - 1]
0x14068ed9d and bx, r11w
0x14068eda1 mov r11d, ebx
0x14068eda4 jne 0x14068ed7d
0x14068eda6 nop word ptr cs:[rax + rax]
0x14068edb0 pcmpeqb xmm2, xmm1
0x14068edb4 pmovmskb r11d, xmm2
0x14068edb9 test r11d, r11d
0x14068edbc jne 0x14068f975
0x14068edc2 add r8, r9
0x14068edc5 add r8, 0x10
0x14068edc9 add r9, 0x10
0x14068edcd and r8, rdx
0x14068edd0 jmp 0x14068ed65
0x14068edd2 movaps xmm0, xmmword ptr [rbp - 0x30]
0x14068edd6 movaps xmm1, xmmword ptr [rbp - 0x20]
0x14068edda movups xmmword ptr [rcx + rbx - 0x90], xmm1
0x14068ede2 movups xmmword ptr [rcx + rbx - 0xa0], xmm0
0x14068edea mov dword ptr [rcx + rbx - 0x80], r14d
0x14068edef movss dword ptr [rcx + rbx - 0x7c], xmm9
0x14068edf6 mov dword ptr [rcx + rbx - 0x78], r15d
0x14068edfb movss dword ptr [rcx + rbx - 0x74], xmm8
0x14068ee02 mov dword ptr [rcx + rbx - 0x70], r12d
0x14068ee07 movss dword ptr [rcx + rbx - 0x6c], xmm6
0x14068ee0d movss dword ptr [rcx + rbx - 0x68], xmm7
0x14068ee13 mov qword ptr [rcx + rbx - 0x64], 0
0x14068ee1c mov byte ptr [rcx + rbx - 0x5c], r13b
0x14068ee21 mov byte ptr [rcx + rbx - 0x5b], dil
0x14068ee26 movzx eax, byte ptr [rbp + 0x110]
0x14068ee2d mov byte ptr [rcx + rbx - 0x5a], al
0x14068ee31 mov rax, qword ptr [rbp + 0x60]
0x14068ee35 mov qword ptr [rbp + 0x30], rax
0x14068ee39 movdqa xmm0, xmmword ptr [rbp + 0x50]
0x14068ee3e movdqa xmmword ptr [rbp + 0x20], xmm0
0x14068ee43 mov r12, qword ptr [rsi + 0x50]
0x14068ee47 mov rax, qword ptr [rsi + 0x58]
0x14068ee4b lea rsi, [rax + rax*2]
0x14068ee4f shl rsi, 5
0x14068ee53 add rsi, r12
0x14068ee56 xor edi, edi
0x14068ee58 lea rbx, [rbp + 0x50]
0x14068ee5c xorps xmm7, xmm7
0x14068ee5f xor eax, eax
0x14068ee61 mov qword ptr [rbp], rsi
0x14068ee65 cmp r12, rsi
0x14068ee68 jne 0x14068eefe
0x14068ee6e movaps xmm6, xmmword ptr [rbp + 0x120]
0x14068ee75 movaps xmm7, xmmword ptr [rbp + 0x130]
0x14068ee7c movaps xmm8, xmmword ptr [rbp + 0x140]
0x14068ee84 movaps xmm9, xmmword ptr [rbp + 0x150]
0x14068ee8c add rsp, 0x1e8
0x14068ee93 pop rbx
0x14068ee94 pop rdi
0x14068ee95 pop rsi
0x14068ee96 pop r12
0x14068ee98 pop r13
0x14068ee9a pop r14
0x14068ee9c pop r15
0x14068ee9e pop rbp
0x14068ee9f ret 
0x14068eea0 mov qword ptr [rbp + 0x50], rax
0x14068eea4 movabs rax, 0x41223169ff28813b
0x14068eeae mov qword ptr [rbp + 0x58], rax
0x14068eeb2 movabs rax, 0xa79b7268a2a968d9
0x14068eebc mov qword ptr [rbp + 0x60], rax
0x14068eec0 mov rcx, rbx
0x14068eec3 call 0x1405bdf20
0x14068eec8 mov rcx, qword ptr [rbp - 8]
0x14068eecc mov qword ptr [rbp + 0x50], rcx
0x14068eed0 mov qword ptr [rbp + 0x58], rax
0x14068eed4 mov rcx, rbx
0x14068eed7 call 0x1405bd910
0x14068eedc lea rcx, [rbp + 0x20]
0x14068eee0 mov rdx, qword ptr [rbp + 0x18]
0x14068eee4 mov r8, rax
0x14068eee7 mov r9, r12
0x14068eeea call 0x14068eb50
0x14068eeef mov r12, r15
0x14068eef2 mov rax, r14
0x14068eef5 cmp r12, rsi
0x14068eef8 je 0x14068ee6e
0x14068eefe lea r15, [r12 + 0x60]
0x14068ef03 lea r14, [rax + 1]
0x14068ef07 cmp rdi, qword ptr [r12]
0x14068ef0b jno 0x14068eea0
0x14068ef0d mov rdx, qword ptr [r12 + 0x10]
0x14068ef12 mov rbx, qword ptr [r12 + 0x18]
0x14068ef17 mov qword ptr [rbp - 0x40], rdx
0x14068ef1b mov qword ptr [rbp - 0x38], rbx
0x14068ef1f mov qword ptr [rbp + 0xc8], 0
0x14068ef2a mov qword ptr [rbp + 0xd0], 1
0x14068ef35 mov eax, 1
0x14068ef3a mov qword ptr [rbp + 0xb8], rax
0x14068ef41 xor eax, eax
0x14068ef43 mov qword ptr [rbp + 0x110], r14
0x14068ef4a mov qword ptr [rbp + 0x100], r15
0x14068ef51 mov qword ptr [rbp + 0x48], r12
0x14068ef55 mov qword ptr [rbp + 0x108], rax
0x14068ef5c mov qword ptr [rbp + 0xd8], rax
0x14068ef63 mov qword ptr [rsp + 0x20], 3
0x14068ef6c lea rcx, [rbp + 0x50]
0x14068ef70 mov qword ptr [rbp + 0xc0], rdx
0x14068ef77 mov r8, rbx
0x14068ef7a lea r9, [rip + 0x2389577] ; 0x142a184f8 7421286372617465732f73797374656d f32=3.1014635560837547e+21
0x14068ef81 call 0x140d89940
0x14068ef86 test byte ptr [rbp + 0x50], 1
0x14068ef8a je 0x14068f0c0
0x14068ef90 mov rdi, qword ptr [rbp + 0x88]
0x14068ef97 mov r8, qword ptr [rbp + 0x98]
0x14068ef9e mov rax, qword ptr [rbp + 0xb0]
0x14068efa5 lea rcx, [rax - 1]
0x14068efa9 mov rdx, qword ptr [rbp + 0xa0]
0x14068efb0 mov r12, qword ptr [rbp + 0xa8]
0x14068efb7 cmp rdi, -1
0x14068efbb mov qword ptr [rbp + 0x10], rcx
0x14068efbf je 0x14068f20d
0x14068efc5 mov r10, qword ptr [rbp + 0x78]
0x14068efc9 add rcx, r10
0x14068efcc cmp rcx, rdx
0x14068efcf jae 0x14068f8db
0x14068efd5 mov r13, qword ptr [rbp + 0x70]
0x14068efd9 mov r11, qword ptr [rbp + 0x58]
0x14068efdd mov r9, qword ptr [rbp + 0x68]
0x14068efe1 mov rsi, rax
0x14068efe4 mov qword ptr [rbp + 0x38], r9
0x14068efe8 sub rsi, r9
0x14068efeb mov qword ptr [rbp + 0x40], rsi
0x14068efef nop 
0x14068eff0 movzx ecx, byte ptr [r8 + rcx]
0x14068eff5 bt r13, rcx
0x14068eff9 jae 0x14068f040
0x14068effb cmp rdi, r11
0x14068effe mov r15, r11
0x14068f001 cmova r15, rdi
0x14068f005 lea r9, [r8 + r10]
0x14068f009 mov rcx, r15
0x14068f00c nop dword ptr [rax]
0x14068f010 cmp rcx, rax
0x14068f013 jae 0x14068f060
0x14068f015 lea rsi, [r10 + rcx]
0x14068f019 cmp rsi, rdx
0x14068f01c jae 0x14068f981
0x14068f022 lea rsi, [rcx + 1]
0x14068f026 movzx r14d, byte ptr [r12 + rcx]
0x14068f02b cmp r14b, byte ptr [r9 + rcx]
0x14068f02f mov rcx, rsi
0x14068f032 je 0x14068f010
0x14068f034 sub r10, r11
0x14068f037 add r10, rsi
0x14068f03a jmp 0x14068f043
0x14068f03c nop dword ptr [rax]
0x14068f040 add r10, rax
0x14068f043 xor edi, edi
0x14068f045 mov rcx, qword ptr [rbp + 0x10]
0x14068f049 add rcx, r10
0x14068f04c cmp rcx, rdx
0x14068f04f jb 0x14068eff0
0x14068f051 jmp 0x14068f8db
0x14068f056 nop word ptr cs:[rax + rax]
0x14068f060 mov rcx, r11
0x14068f063 nop word ptr cs:[rax + rax]
0x14068f070 cmp rdi, rcx
0x14068f073 jae 0x14068f310
0x14068f079 dec rcx
0x14068f07c cmp rcx, rax
0x14068f07f jae 0x14068f9bd
0x14068f085 lea r9, [rcx + r10]
0x14068f089 cmp r9, rdx
0x14068f08c jae 0x14068f9d1
0x14068f092 movzx esi, byte ptr [r12 + rcx]
0x14068f097 cmp sil, byte ptr [r8 + r9]
0x14068f09b je 0x14068f070
0x14068f09d add r10, qword ptr [rbp + 0x38]
0x14068f0a1 mov rdi, qword ptr [rbp + 0x40]
0x14068f0a5 mov rcx, qword ptr [rbp + 0x10]
0x14068f0a9 add rcx, r10
0x14068f0ac cmp rcx, rdx
0x14068f0af jb 0x14068eff0
0x14068f0b5 jmp 0x14068f8db
0x14068f0ba nop word ptr [rax + rax]
0x14068f0c0 cmp byte ptr [rbp + 0x6a], 0
0x14068f0c4 jne 0x14068f8db
0x14068f0ca mov r10, qword ptr [rbp + 0x58]
0x14068f0ce mov rcx, qword ptr [rbp + 0x98]
0x14068f0d5 mov r12, qword ptr [rbp + 0xa0]
0x14068f0dc movzx eax, byte ptr [rbp + 0x68]
0x14068f0e0 mov rdx, qword ptr [rbp + 0xc0]
0x14068f0e7 jmp 0x14068f0f5
0x14068f0e9 nop dword ptr [rax]
0x14068f0f0 xor al, 1
0x14068f0f2 add r10, r8
0x14068f0f5 test r10, r10
0x14068f0f8 je 0x14068f116
0x14068f0fa cmp r10, r12
0x14068f0fd jae 0x14068f110
0x14068f0ff cmp byte ptr [rcx + r10], 0xbf
0x14068f104 jg 0x14068f116
0x14068f106 jmp 0x14068f9e5
0x14068f10b nop dword ptr [rax + rax]
0x14068f110 jne 0x14068f9e5
0x14068f116 cmp r10, r12
0x14068f119 mov r14, qword ptr [rbp + 0x110]
0x14068f120 mov r15, qword ptr [rbp + 0x100]
0x14068f127 mov r13, qword ptr [rbp + 0x48]
0x14068f12b je 0x14068f200
0x14068f131 movzx r9d, byte ptr [rcx + r10]
0x14068f136 movzx esi, r9b
0x14068f13a test sil, sil
0x14068f13d jns 0x14068f1a0
0x14068f13f mov r8d, esi
0x14068f142 and r8d, 0x1f
0x14068f146 movzx r11d, byte ptr [rcx + r10 + 1]
0x14068f14c and r11d, 0x3f
0x14068f150 cmp sil, 0xdf
0x14068f154 jbe 0x14068f193
0x14068f156 movzx esi, byte ptr [rcx + r10 + 2]
0x14068f15c shl r11d, 6
0x14068f160 and esi, 0x3f
0x14068f163 or esi, r11d
0x14068f166 cmp r9b, 0xf0
0x14068f16a jb 0x14068f1e1
0x14068f16c movzx r9d, byte ptr [rcx + r10 + 3]
0x14068f172 and r8d, 7
0x14068f176 shl r8d, 0x12
0x14068f17a shl esi, 6
0x14068f17d and r9d, 0x3f
0x14068f181 or r9d, esi
0x14068f184 or r9d, r8d
0x14068f187 mov esi, r9d
0x14068f18a test al, 1
0x14068f18c je 0x14068f1a8
0x14068f18e jmp 0x14068f360
0x14068f193 shl r8d, 6
0x14068f197 or r8d, r11d
0x14068f19a mov esi, r8d
0x14068f19d nop dword ptr [rax]
0x14068f1a0 test al, 1
0x14068f1a2 jne 0x14068f360
0x14068f1a8 mov r8d, 1
0x14068f1ae cmp esi, 0x80
0x14068f1b4 jb 0x14068f0f0
0x14068f1ba mov r8d, 2
0x14068f1c0 cmp esi, 0x800
0x14068f1c6 jb 0x14068f0f0
0x14068f1cc cmp esi, 0x10000
0x14068f1d2 mov r8d, 4
0x14068f1d8 sbb r8, 0
0x14068f1dc jmp 0x14068f0f0
0x14068f1e1 shl r8d, 0xc
0x14068f1e5 or esi, r8d
0x14068f1e8 test al, 1
0x14068f1ea je 0x14068f1a8
0x14068f1ec jmp 0x14068f360
0x14068f1f1 nop word ptr cs:[rax + rax]
0x14068f200 test al, 1
0x14068f202 jne 0x14068f32c
0x14068f208 jmp 0x14068f8f4
0x14068f20d mov r10, qword ptr [rbp + 0x78]
0x14068f211 lea r9, [r10 + rcx]
0x14068f215 cmp r9, rdx
0x14068f218 jae 0x14068f8db
0x14068f21e mov r14, qword ptr [rbp + 0x70]
0x14068f222 mov r13, qword ptr [rbp + 0x58]
0x14068f226 mov rcx, qword ptr [rbp + 0x68]
0x14068f22a mov qword ptr [rbp + 8], rcx
0x14068f22e lea rcx, [r13 - 1]
0x14068f232 mov qword ptr [rbp + 0x38], rcx
0x14068f236 lea r15, [r8 + r13]
0x14068f23a lea rcx, [r12 + r13]
0x14068f23e dec rcx
0x14068f241 mov qword ptr [rbp + 0x40], rcx
0x14068f245 add r12, r13
0x14068f248 jmp 0x14068f269
0x14068f24a nop word ptr [rax + rax]
0x14068f250 add r10, rax
0x14068f253 mov r11, r10
0x14068f256 mov r9, qword ptr [rbp + 0x10]
0x14068f25a add r9, r11
0x14068f25d mov r10, r11
0x14068f260 cmp r9, rdx
0x14068f263 jae 0x14068f8db
0x14068f269 movzx r9d, byte ptr [r8 + r9]
0x14068f26e bt r14, r9
0x14068f272 jae 0x14068f250
0x14068f274 xor r9d, r9d
0x14068f277 mov rdi, r10
0x14068f27a nop word ptr [rax + rax]
0x14068f280 lea r11, [r9 + r13]
0x14068f284 cmp r11, rax
0x14068f287 jae 0x14068f2b0
0x14068f289 lea r11, [rdi + r13]
0x14068f28d cmp r11, rdx
0x14068f290 jae 0x14068f99f
0x14068f296 movzx esi, byte ptr [r12 + r9]
0x14068f29b lea r11, [rdi + 1]
0x14068f29f inc r9
0x14068f2a2 cmp sil, byte ptr [r15 + rdi]
0x14068f2a6 mov rdi, r11
0x14068f2a9 je 0x14068f280
0x14068f2ab jmp 0x14068f256
0x14068f2ad nop dword ptr [rax]
0x14068f2b0 mov r9, qword ptr [rbp + 0x38]
0x14068f2b4 cmp r9, rax
0x14068f2b7 jae 0x14068f333
0x14068f2b9 mov rcx, r14
0x14068f2bc add r9, r10
0x14068f2bf mov r14, qword ptr [rbp + 0x40]
0x14068f2c3 mov rdi, r13
0x14068f2c6 nop word ptr cs:[rax + rax]
0x14068f2d0 sub rdi, 1
0x14068f2d4 jb 0x14068f310
0x14068f2d6 cmp r9, rdx
0x14068f2d9 jae 0x14068fa04
0x14068f2df movzx r11d, byte ptr [r14]
0x14068f2e3 dec r14
0x14068f2e6 lea rsi, [r9 - 1]
0x14068f2ea cmp r11b, byte ptr [r8 + r9]
0x14068f2ee mov r9, rsi
0x14068f2f1 je 0x14068f2d0
0x14068f2f3 add r10, qword ptr [rbp + 8]
0x14068f2f7 mov r11, r10
0x14068f2fa mov r14, rcx
0x14068f2fd jmp 0x14068f256
0x14068f302 nop word ptr cs:[rax + rax]
0x14068f310 mov r14, qword ptr [rbp + 0x110]
0x14068f317 mov r15, qword ptr [rbp + 0x100]
0x14068f31e mov rdx, qword ptr [rbp + 0xc0]
0x14068f325 mov r13, qword ptr [rbp + 0x48]
0x14068f329 mov r12, r10
0x14068f32c test r12, r12
0x14068f32f jne 0x14068f368
0x14068f331 jmp 0x14068f386
0x14068f333 test r13, r13
0x14068f336 mov r14, qword ptr [rbp + 0x110]
0x14068f33d mov r15, qword ptr [rbp + 0x100]
0x14068f344 mov rdx, qword ptr [rbp + 0xc0]
0x14068f34b mov r13, qword ptr [rbp + 0x48]
0x14068f34f jne 0x14068fa97
0x14068f355 nop word ptr cs:[rax + rax]
0x14068f360 mov r12, r10
0x14068f363 test r12, r12
0x14068f366 je 0x14068f386
0x14068f368 cmp r12, rbx
0x14068f36b jae 0x14068f380
0x14068f36d cmp byte ptr [rdx + r12], 0xbf
0x14068f372 jg 0x14068f386
0x14068f374 jmp 0x14068fa37
0x14068f379 nop dword ptr [rax]
0x14068f380 jne 0x14068fa37
0x14068f386 mov rax, qword ptr [rbp + 0xc8]
0x14068f38d sub rax, qword ptr [rbp + 0x108]
0x14068f394 cmp rax, r12
0x14068f397 jb 0x14068f510
0x14068f39d mov rdi, qword ptr [rbp + 0x108]
0x14068f3a4 mov rax, qword ptr [rbp + 0xb8]
0x14068f3ab lea rcx, [rax + rdi]
0x14068f3af mov rsi, qword ptr [rbp + 0xc0]
0x14068f3b6 mov rdx, rsi
0x14068f3b9 mov r8, r12
0x14068f3bc call 0x142923c00
0x14068f3c1 add rdi, r12
0x14068f3c4 mov qword ptr [rbp + 0x108], rdi
0x14068f3cb mov qword ptr [rbp + 0xd8], rdi
0x14068f3d2 add r12, 3
0x14068f3d6 je 0x14068f3f6
0x14068f3d8 cmp rbx, r12
0x14068f3db jbe 0x14068f3f0
0x14068f3dd cmp byte ptr [rsi + r12], 0xbf
0x14068f3e2 jg 0x14068f3f6
0x14068f3e4 jmp 0x14068fa18
0x14068f3e9 nop dword ptr [rax]
0x14068f3f0 jne 0x14068fa18
0x14068f3f6 sub rbx, r12
0x14068f3f9 add rsi, r12
0x14068f3fc xor r12d, r12d
0x14068f3ff mov rdi, rsi
0x14068f402 jmp 0x14068f419
0x14068f404 nop word ptr cs:[rax + rax]
0x14068f410 cmp rbx, r12
0x14068f413 jb 0x14068f590
0x14068f419 mov r8, rbx
0x14068f41c sub r8, r12
0x14068f41f lea rax, [rsi + r12]
0x14068f423 cmp r8, 0xf
0x14068f427 ja 0x14068f460
0x14068f429 cmp rbx, r12
0x14068f42c je 0x14068f590
0x14068f432 xor edx, edx
0x14068f434 nop word ptr cs:[rax + rax]
0x14068f440 cmp byte ptr [rax + rdx], 0x29
0x14068f444 je 0x14068f477
0x14068f446 inc rdx
0x14068f449 cmp r8, rdx
0x14068f44c jne 0x14068f440
0x14068f44e jmp 0x14068f590
0x14068f453 nop word ptr cs:[rax + rax]
0x14068f460 mov cl, 0x29
0x14068f462 mov rdx, rax
0x14068f465 call 0x140d88a40
0x14068f46a cmp rax, 1
0x14068f46e jne 0x14068f590
0x14068f474 mov rsi, rdi
0x14068f477 lea r9, [rdx + r12]
0x14068f47b add r12, rdx
0x14068f47e inc r12
0x14068f481 cmp r9, rbx
0x14068f484 jae 0x14068f410
0x14068f486 cmp byte ptr [rsi + r9], 0x29
0x14068f48b jne 0x14068f410
0x14068f48d cmp r9, rbx
0x14068f490 jae 0x14068fa56
0x14068f496 cmp r12, rbx
0x14068f499 jae 0x14068f4ad
0x14068f49b cmp byte ptr [rsi + r12], 0xbf
0x14068f4a0 mov rcx, rsi
0x14068f4a3 mov r14, r12
0x14068f4a6 jg 0x14068f4b9
0x14068f4a8 jmp 0x14068fa5f
0x14068f4ad mov rcx, rsi
0x14068f4b0 mov r14, rbx
0x14068f4b3 jne 0x14068fa5f
0x14068f4b9 mov rdx, r9
0x14068f4bc call 0x140b9d480
0x14068f4c1 mov r15, rax
0x14068f4c4 mov r12, rdx
0x14068f4c7 mov rax, qword ptr [rbp + 0xc8]
0x14068f4ce sub rax, qword ptr [rbp + 0x108]
0x14068f4d5 cmp rax, rdx
0x14068f4d8 jb 0x14068f556
0x14068f4da add rdi, r14
0x14068f4dd sub rbx, r14
0x14068f4e0 mov rax, qword ptr [rbp + 0xd0]
0x14068f4e7 mov rsi, qword ptr [rbp + 0x108]
0x14068f4ee mov qword ptr [rbp + 0xb8], rax
0x14068f4f5 lea rcx, [rax + rsi]
0x14068f4f9 mov rdx, r15
0x14068f4fc mov r8, r12
0x14068f4ff call 0x142923c00
0x14068f504 mov rdx, rdi
0x14068f507 lea rax, [r12 + rsi]
0x14068f50b jmp 0x14068ef55
0x14068f510 mov qword ptr [rsp + 0x20], 1
0x14068f519 mov r9d, 1
0x14068f51f lea rcx, [rbp + 0xc8]
0x14068f526 mov rdx, qword ptr [rbp + 0x108]
0x14068f52d mov r8, r12
0x14068f530 call 0x1428a58d0
0x14068f535 mov rax, qword ptr [rbp + 0xd0]
0x14068f53c mov qword ptr [rbp + 0xb8], rax
0x14068f543 mov rax, qword ptr [rbp + 0xd8]
0x14068f54a mov qword ptr [rbp + 0x108], rax
0x14068f551 jmp 0x14068f39d
0x14068f556 mov qword ptr [rsp + 0x20], 1
0x14068f55f mov r9d, 1
0x14068f565 lea rcx, [rbp + 0xc8]
0x14068f56c mov rdx, qword ptr [rbp + 0x108]
0x14068f573 mov r8, r12
0x14068f576 call 0x1428a58d0
0x14068f57b mov rax, qword ptr [rbp + 0xd8]
0x14068f582 mov qword ptr [rbp + 0x108], rax
0x14068f589 jmp 0x14068f4da
0x14068f58e nop 
0x14068f590 lea rax, [rbp - 0x40]
0x14068f594 mov qword ptr [rbp - 0x50], rax
0x14068f598 lea rax, [rip - 0x15bcff] ; 0x1405338a0 4989d0488b01488b51084889c1e9ae4a f32=427082.28125
0x14068f59f mov qword ptr [rbp - 0x48], rax
0x14068f5a3 lea rax, [rip + 0x2388ff6] ; 0x142a185a0 8885a142010000001300000000000000 f32=80.76080322265625
0x14068f5aa mov qword ptr [rbp + 0x50], rax
0x14068f5ae mov qword ptr [rbp + 0x58], 1
0x14068f5b6 lea rax, [rbp - 0x50]
0x14068f5ba mov qword ptr [rbp + 0x60], rax
0x14068f5be mov qword ptr [rbp + 0x68], 1
0x14068f5c6 mov qword ptr [rbp + 0x70], 0
0x14068f5ce lea rcx, [rbp + 0xe0]
0x14068f5d5 lea rdx, [rbp + 0x50]
0x14068f5d9 call 0x140d79f90
0x14068f5de mov rax, qword ptr [rbp + 0xe0]
0x14068f5e5 mov qword ptr [rbp + 0x100], rax
0x14068f5ec mov rbx, qword ptr [rbp + 0xe8]
0x14068f5f3 mov rsi, qword ptr [rbp + 0xf0]
0x14068f5fa mov rdx, qword ptr [rbp + 0xc8]
0x14068f601 test rdx, rdx
0x14068f604 je 0x14068f618
0x14068f606 mov r8d, 1
0x14068f60c mov rcx, qword ptr [rbp + 0xb8]
0x14068f613 call 0x140613c20
0x14068f618 lea rdi, [rbp + 0x50]
0x14068f61c mov qword ptr [rbp + 0x110], rbx
0x14068f623 mov qword ptr [rbp + 0xc8], rbx
0x14068f62a mov qword ptr [rbp + 0xd0], rsi
0x14068f631 lea rcx, [rbp + 0xc8]
0x14068f638 call 0x1405bdda0
0x14068f63d mov qword ptr [rbp + 0x50], rax
0x14068f641 movabs rax, 0xdc603d951b9380b2
0x14068f64b mov qword ptr [rbp + 0x58], rax
0x14068f64f movabs rax, 0xce69dbf0aee13da2
0x14068f659 mov qword ptr [rbp + 0x60], rax
0x14068f65d mov rcx, rdi
0x14068f660 call 0x1405bdf20
0x14068f665 mov rcx, qword ptr [rbp + 0x28]
0x14068f669 mov qword ptr [rbp + 0x50], rcx
0x14068f66d mov qword ptr [rbp + 0x58], rax
0x14068f671 mov rcx, rdi
0x14068f674 call 0x1405bd910
0x14068f679 mov rcx, rdi
0x14068f67c lea rdx, [rbp + 0x20]
0x14068f680 mov r8, rax
0x14068f683 call 0x1406861b0
0x14068f688 mov rcx, rdi
0x14068f68b xor edx, edx
0x14068f68d call 0x140f9d930
0x14068f692 mov rcx, rax
0x14068f695 mov rdx, qword ptr [rbp + 0x110]
0x14068f69c mov r8, rsi
0x14068f69f call 0x140fac4c0
0x14068f6a4 mov rax, qword ptr [rbp + 0x60]
0x14068f6a8 mov qword ptr [rbp + 0xf0], rax
0x14068f6af movups xmm0, xmmword ptr [rbp + 0x50]
0x14068f6b3 movaps xmmword ptr [rbp + 0xe0], xmm0
0x14068f6ba mov rdx, qword ptr [rbp + 0x100]
0x14068f6c1 test rdx, rdx
0x14068f6c4 je 0x14068f6d8
0x14068f6c6 mov r8d, 1
0x14068f6cc mov rcx, qword ptr [rbp + 0x110]
0x14068f6d3 call 0x140613c20
0x14068f6d8 cmp dword ptr [r13 + 0x20], 1
0x14068f6dd je 0x14068f7e0
0x14068f6e3 cmp dword ptr [r13 + 0x28], 1
0x14068f6e8 je 0x14068f82b
0x14068f6ee cmp dword ptr [r13 + 0x30], 1
0x14068f6f3 je 0x14068f87d
0x14068f6f9 movzx ebx, byte ptr [r13 + 0x38]
0x14068f6fe cmp bl, 4
0x14068f701 je 0x14068f744
0x14068f703 mov rax, qword ptr [rbp + 0xf0]
0x14068f70a mov qword ptr [rbp + 0x60], rax
0x14068f70e movaps xmm0, xmmword ptr [rbp + 0xe0]
0x14068f715 movaps xmmword ptr [rbp + 0x50], xmm0
0x14068f719 lea rcx, [rbp + 0x50]
0x14068f71d mov dl, 1
0x14068f71f call 0x140f9d930
0x14068f724 mov rcx, rax
0x14068f727 mov edx, ebx
0x14068f729 call 0x140b90c80
0x14068f72e mov rax, qword ptr [rbp + 0x60]
0x14068f732 mov qword ptr [rbp + 0xf0], rax
0x14068f739 movaps xmm0, xmmword ptr [rbp + 0x50]
0x14068f73d movaps xmmword ptr [rbp + 0xe0], xmm0
0x14068f744 mov rax, qword ptr [rbp + 0xf0]
0x14068f74b mov qword ptr [rbp + 0x60], rax
0x14068f74f movaps xmm0, xmmword ptr [rbp + 0xe0]
0x14068f756 movaps xmmword ptr [rbp + 0x50], xmm0
0x14068f75a movzx ebx, byte ptr [r13 + 0x39]
0x14068f75f lea rsi, [rbp + 0x50]
0x14068f763 mov rcx, rsi
0x14068f766 mov dl, 1
0x14068f768 call 0x140f9d930
0x14068f76d mov rcx, rax
0x14068f770 mov edx, ebx
0x14068f772 mov rbx, rsi
0x14068f775 call 0x140b90f40
0x14068f77a mov rax, qword ptr [rbp + 0x60]
0x14068f77e mov qword ptr [rbp + 0xf0], rax
0x14068f785 movdqa xmm0, xmmword ptr [rbp + 0x50]
0x14068f78a movdqa xmmword ptr [rbp + 0xe0], xmm0
0x14068f792 mov qword ptr [rbp + 0x60], rax
0x14068f796 movdqa xmmword ptr [rbp + 0x50], xmm0
0x14068f79b movzx eax, byte ptr [r13 + 0x3a]
0x14068f7a0 lea rcx, [rip + 0x2388e69] ; 0x142a18610 6400c8002c019001f4015802bc022003 f32=1.8367239361444675e-38
0x14068f7a7 movzx esi, word ptr [rcx + rax*2]
0x14068f7ab mov rcx, rbx
0x14068f7ae mov dl, 1
0x14068f7b0 call 0x140f9d930
0x14068f7b5 mov rcx, rax
0x14068f7b8 mov edx, esi
0x14068f7ba call 0x140b90ea0
0x14068f7bf mov r12, r15
0x14068f7c2 mov rax, r14
0x14068f7c5 mov rsi, qword ptr [rbp]
0x14068f7c9 xor edi, edi
0x14068f7cb cmp r12, rsi
0x14068f7ce jne 0x14068eefe
0x14068f7d4 jmp 0x14068ee6e
0x14068f7d9 nop dword ptr [rax]
0x14068f7e0 mov esi, dword ptr [r13 + 0x24]
0x14068f7e4 mov rax, qword ptr [rbp + 0xf0]
0x14068f7eb mov qword ptr [rbp + 0x60], rax
0x14068f7ef movaps xmm0, xmmword ptr [rbp + 0xe0]
0x14068f7f6 movaps xmmword ptr [rbp + 0x50], xmm0
0x14068f7fa mov rcx, rdi
0x14068f7fd mov dl, 1
0x14068f7ff call 0x140f9d930
0x14068f804 mov dword ptr [rax + 0x258], esi
0x14068f80a movaps xmm0, xmmword ptr [rbp + 0x50]
0x14068f80e movaps xmmword ptr [rbp + 0xe0], xmm0
0x14068f815 mov rax, qword ptr [rbp + 0x60]
0x14068f819 mov qword ptr [rbp + 0xf0], rax
0x14068f820 cmp dword ptr [r13 + 0x28], 1
0x14068f825 jne 0x14068f6ee
0x14068f82b movss xmm6, dword ptr [r13 + 0x2c]
0x14068f831 mov rax, qword ptr [rbp + 0xf0]
0x14068f838 mov qword ptr [rbp + 0x60], rax
0x14068f83c movaps xmm0, xmmword ptr [rbp + 0xe0]
0x14068f843 movaps xmmword ptr [rbp + 0x50], xmm0
0x14068f847 mov rcx, rdi
0x14068f84a mov dl, 1
0x14068f84c call 0x140f9d930
0x14068f851 mov rcx, rax
0x14068f854 movaps xmm1, xmm6
0x14068f857 call 0x140fac150
0x14068f85c mov rax, qword ptr [rbp + 0x60]
0x14068f860 mov qword ptr [rbp + 0xf0], rax
0x14068f867 movaps xmm0, xmmword ptr [rbp + 0x50]
0x14068f86b movaps xmmword ptr [rbp + 0xe0], xmm0
0x14068f872 cmp dword ptr [r13 + 0x30], 1
0x14068f877 jne 0x14068f6f9
0x14068f87d movss xmm6, dword ptr [r13 + 0x34]
0x14068f883 maxss xmm6, xmm7
0x14068f887 mov rax, qword ptr [rbp + 0xf0]
0x14068f88e mov qword ptr [rbp + 0x60], rax
0x14068f892 movaps xmm0, xmmword ptr [rbp + 0xe0]
0x14068f899 movaps xmmword ptr [rbp + 0x50], xmm0
0x14068f89d mov rcx, rdi
0x14068f8a0 mov dl, 1
0x14068f8a2 call 0x140f9d930
0x14068f8a7 mov rcx, rax
0x14068f8aa movaps xmm1, xmm6
0x14068f8ad call 0x140b90ec0
0x14068f8b2 mov rax, qword ptr [rbp + 0x60]
0x14068f8b6 mov qword ptr [rbp + 0xf0], rax
0x14068f8bd movaps xmm0, xmmword ptr [rbp + 0x50]
0x14068f8c1 movaps xmmword ptr [rbp + 0xe0], xmm0
0x14068f8c8 movzx ebx, byte ptr [r13 + 0x38]
0x14068f8cd cmp bl, 4
0x14068f8d0 jne 0x14068f703
0x14068f8d6 jmp 0x14068f744
0x14068f8db mov r14, qword ptr [rbp + 0x110]
0x14068f8e2 mov r15, qword ptr [rbp + 0x100]
0x14068f8e9 mov rdx, qword ptr [rbp + 0xc0]
0x14068f8f0 mov r13, qword ptr [rbp + 0x48]
0x14068f8f4 mov rax, qword ptr [rbp + 0xc8]
0x14068f8fb mov rsi, qword ptr [rbp + 0x108]
0x14068f902 sub rax, rsi
0x14068f905 cmp rax, rbx
0x14068f908 jb 0x14068f936
0x14068f90a mov rdi, qword ptr [rbp + 0xb8]
0x14068f911 lea rcx, [rdi + rsi]
0x14068f915 mov r8, rbx
0x14068f918 call 0x142923c00
0x14068f91d add rsi, rbx
0x14068f920 mov rax, qword ptr [rbp + 0xc8]
0x14068f927 mov qword ptr [rbp + 0x100], rax
0x14068f92e mov rbx, rdi
0x14068f931 jmp 0x14068f618
0x14068f936 mov qword ptr [rsp + 0x20], 1
0x14068f93f mov r9d, 1
0x14068f945 lea rcx, [rbp + 0xc8]
0x14068f94c mov rdx, rsi
0x14068f94f mov r8, rbx
0x14068f952 call 0x1428a58d0
0x14068f957 mov rax, qword ptr [rbp + 0xd0]
0x14068f95e mov qword ptr [rbp + 0xb8], rax
0x14068f965 mov rsi, qword ptr [rbp + 0xd8]
0x14068f96c mov rdx, qword ptr [rbp + 0xc0]
0x14068f973 jmp 0x14068f90a
0x14068f975 lea rcx, [rip + 0x23885e4] ; 0x142a17f60 f87ba142010000002600000000000000 f32=80.74212646484375
0x14068f97c call 0x1428d9310
0x14068f981 add r15, r10
0x14068f984 cmp rdx, r15
0x14068f987 cmova r15, rdx
0x14068f98b lea r8, [rip + 0x2388ab6] ; 0x142a18448 c283a142010000004f00000000000000 f32=80.75733947753906
0x14068f992 mov rcx, r15
0x14068f995 call 0x1428d9518
0x14068f99a jmp 0x14068faa9
0x14068f99f add r10, r13
0x14068f9a2 cmp rdx, r10
0x14068f9a5 cmova r10, rdx
0x14068f9a9 lea r8, [rip + 0x2388a98] ; 0x142a18448 c283a142010000004f00000000000000 f32=80.75733947753906
0x14068f9b0 mov rcx, r10
0x14068f9b3 call 0x1428d9518
0x14068f9b8 jmp 0x14068faa9
0x14068f9bd lea r8, [rip + 0x2388a54] ; 0x142a18418 c283a142010000004f00000000000000 f32=80.75733947753906
0x14068f9c4 mov rdx, rax
0x14068f9c7 call 0x1428d9518
0x14068f9cc jmp 0x14068faa9
0x14068f9d1 lea r8, [rip + 0x2388a58] ; 0x142a18430 c283a142010000004f00000000000000 f32=80.75733947753906
0x14068f9d8 mov rcx, r9
0x14068f9db call 0x1428d9518
0x14068f9e0 jmp 0x14068faa9
0x14068f9e5 lea rax, [rip + 0x2388ae4] ; 0x142a184d0 c283a142010000004f00000000000000 f32=80.75733947753906
0x14068f9ec mov qword ptr [rsp + 0x20], rax
0x14068f9f1 mov rdx, r12
0x14068f9f4 mov r8, r10
0x14068f9f7 mov r9, r12
0x14068f9fa call 0x1428d9940
0x14068f9ff jmp 0x14068faa9
0x14068fa04 lea r8, [rip + 0x2388a25] ; 0x142a18430 c283a142010000004f00000000000000 f32=80.75733947753906
0x14068fa0b mov rcx, r9
0x14068fa0e call 0x1428d9518
0x14068fa13 jmp 0x14068faa9
0x14068fa18 lea rax, [rip + 0x2388b21] ; 0x142a18540 fb84a142010000002800000000000000 f32=80.75972747802734
0x14068fa1f mov qword ptr [rsp + 0x20], rax
0x14068fa24 mov rcx, rsi
0x14068fa27 mov rdx, rbx
0x14068fa2a mov r8, r12
0x14068fa2d mov r9, rbx
0x14068fa30 call 0x1428d9940
0x14068fa35 jmp 0x14068faa9
0x14068fa37 lea rax, [rip + 0x2388aea] ; 0x142a18528 fb84a142010000002800000000000000 f32=80.75972747802734
0x14068fa3e mov qword ptr [rsp + 0x20], rax
0x14068fa43 mov rcx, rdx
0x14068fa46 mov rdx, rbx
0x14068fa49 xor r8d, r8d
0x14068fa4c mov r9, r12
0x14068fa4f call 0x1428d9940
0x14068fa54 jmp 0x14068faa9
0x14068fa56 jne 0x14068fa7b
0x14068fa58 lea r12, [rbx + 1]
0x14068fa5c mov rcx, rdi
0x14068fa5f lea rax, [rip + 0x2388b0a] ; 0x142a18570 fb84a142010000002800000000000000 f32=80.75972747802734
0x14068fa66 mov qword ptr [rsp + 0x20], rax
0x14068fa6b mov rdx, rbx
0x14068fa6e mov r8, r12
0x14068fa71 mov r9, rbx
0x14068fa74 call 0x1428d9940
0x14068fa79 jmp 0x14068faa9
0x14068fa7b lea rax, [rip + 0x2388ad6] ; 0x142a18558 fb84a142010000002800000000000000 f32=80.75972747802734
0x14068fa82 mov qword ptr [rsp + 0x20], rax
0x14068fa87 mov rcx, rdi
0x14068fa8a mov rdx, rbx
0x14068fa8d xor r8d, r8d
0x14068fa90 call 0x1428d9940
0x14068fa95 jmp 0x14068faa9
0x14068fa97 mov rcx, r9
0x14068fa9a lea r8, [rip + 0x2388977] ; 0x142a18418 c283a142010000004f00000000000000 f32=80.75733947753906
0x14068faa1 mov rdx, rax
0x14068faa4 call 0x1428d9518
0x14068faa9 ud2 
0x14068faab nop dword ptr [rax + rax]
0x14068fab0 add r8, 0x20
0x14068fab4 cmp rax, r8
0x14068fab7 je 0x14068fb49
0x14068fabd lea r10, [rcx + r8]
0x14068fac1 mov r9d, dword ptr [rcx + r8]
0x14068fac5 add r9d, -2
0x14068fac9 cmp r9d, 7
0x14068facd mov r11d, 7
0x14068fad3 cmovb r11d, r9d
0x14068fad7 movsxd r11, dword ptr [rdx + r11*4]
0x14068fadb add r11, rdx
0x14068fade mov r9d, r13d
0x14068fae1 xor r13d, r13d
0x14068fae4 jmp r11
0x14068fae7 mov r13b, 1
0x14068faea jmp 0x14068fab0
0x14068faec mov r15d, dword ptr [rcx + r8 + 4]
0x14068faf1 movss xmm8, dword ptr [rcx + r8 + 8]
0x14068faf8 mov r12d, dword ptr [rcx + r8 + 0xc]
0x14068fafd movss xmm6, dword ptr [rcx + r8 + 0x10]
0x14068fb04 jmp 0x14068fb41
0x14068fb06 movzx ebx, byte ptr [rcx + r8 + 4]
0x14068fb0c jmp 0x14068fb41
0x14068fb0e movzx edi, byte ptr [rcx + r8 + 4]
0x14068fb14 jmp 0x14068fb41
0x14068fb16 movss xmm7, dword ptr [rip + 0x2295de6] ; 0x142925904 0000803f9a99193f00000000feffffff f32=1.0
0x14068fb1e jmp 0x14068fb41
0x14068fb20 mov r14d, dword ptr [rcx + r8 + 4]
0x14068fb25 movss xmm9, dword ptr [rcx + r8 + 8]
0x14068fb2c jmp 0x14068fb41
0x14068fb2e movups xmm0, xmmword ptr [r10]
0x14068fb32 movdqu xmm1, xmmword ptr [r10 + 0x10]
0x14068fb38 movdqa xmmword ptr [rbp - 0x20], xmm1
0x14068fb3d movaps xmmword ptr [rbp - 0x30], xmm0
0x14068fb41 mov r13d, r9d
0x14068fb44 jmp 0x14068fab0
0x14068fb49 mov byte ptr [rbp + 0x110], bl
0x14068fb4f jmp 0x14068ecf5
0x14068fb54 nop word ptr cs:[rax + rax]
0x14068fb60 mov qword ptr [rsp + 0x10], rdx
0x14068fb65 push rbp
0x14068fb66 push r15
0x14068fb68 push r14
0x14068fb6a push r13
0x14068fb6c push r12
0x14068fb6e push rsi
0x14068fb6f push rdi
0x14068fb70 push rbx
0x14068fb71 sub rsp, 0x68
0x14068fb75 lea rbp, [rdx + 0x80]
0x14068fb7c movaps xmmword ptr [rsp + 0x20], xmm9
0x14068fb82 movaps xmmword ptr [rsp + 0x30], xmm8
0x14068fb88 movaps xmmword ptr [rsp + 0x40], xmm7
0x14068fb8d movaps xmmword ptr [rsp + 0x50], xmm6
0x14068fb92 mov rdx, qword ptr [rbp + 0xc8]
0x14068fb99 test rdx, rdx
0x14068fb9c je 0x14068fbb0
0x14068fb9e mov rcx, qword ptr [rbp + 0xd0]
0x14068fba5 mov r8d, 1
0x14068fbab call 0x140613c20
0x14068fbb0 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14068fbb5 movaps xmm7, xmmword ptr [rsp + 0x40]
0x14068fbba movaps xmm8, xmmword ptr [rsp + 0x30]
0x14068fbc0 movaps xmm9, xmmword ptr [rsp + 0x20]
0x14068fbc6 add rsp, 0x68
0x14068fbca pop rbx
0x14068fbcb pop rdi
0x14068fbcc pop rsi
0x14068fbcd pop r12
0x14068fbcf pop r13
0x14068fbd1 pop r14
0x14068fbd3 pop r15
0x14068fbd5 pop rbp
0x14068fbd6 ret 
0x14068fbd7 nop word ptr [rax + rax]
0x14068fbe0 mov qword ptr [rsp + 0x10], rdx
0x14068fbe5 push rbp
0x14068fbe6 push r15
0x14068fbe8 push r14
0x14068fbea push r13
0x14068fbec push r12
0x14068fbee push rsi
0x14068fbef push rdi
0x14068fbf0 push rbx
0x14068fbf1 sub rsp, 0x68
0x14068fbf5 lea rbp, [rdx + 0x80]
0x14068fbfc movaps xmmword ptr [rsp + 0x20], xmm9
0x14068fc02 movaps xmmword ptr [rsp + 0x30], xmm8
0x14068fc08 movaps xmmword ptr [rsp + 0x40], xmm7
0x14068fc0d movaps xmmword ptr [rsp + 0x50], xmm6
0x14068fc12 cmp qword ptr [rbp + 0x100], 0
0x14068fc1a je 0x14068fc35
0x14068fc1c mov r8d, 1
0x14068fc22 mov rcx, qword ptr [rbp + 0x110]
0x14068fc29 mov rdx, qword ptr [rbp + 0x100]
0x14068fc30 call 0x140613c20
0x14068fc35 movaps xmm6, xmmword ptr [rsp + 0x50]
0x14068fc3a movaps xmm7, xmmword ptr [rsp + 0x40]
0x14068fc3f movaps xmm8, xmmword ptr [rsp + 0x30]
0x14068fc45 movaps xmm9, xmmword ptr [rsp + 0x20]
0x14068fc4b add rsp, 0x68
0x14068fc4f pop rbx
0x14068fc50 pop rdi
0x14068fc51 pop rsi
0x14068fc52 pop r12
0x14068fc54 pop r13
0x14068fc56 pop r14
0x14068fc58 pop r15
0x14068fc5a pop rbp
0x14068fc5b ret 