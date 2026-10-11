14076d9f0 push rbp
14076d9f1 push r15
14076d9f3 push r14
14076d9f5 push r13
14076d9f7 push r12
14076d9f9 push rsi
14076d9fa push rdi
14076d9fb push rbx
14076d9fc sub rsp, 0x388
14076da03 lea rbp, [rsp + 0x80]
14076da0b movaps xmmword ptr [rbp + 0x2f0], xmm15
14076da13 movaps xmmword ptr [rbp + 0x2e0], xmm14
14076da1b movaps xmmword ptr [rbp + 0x2d0], xmm13
14076da23 movaps xmmword ptr [rbp + 0x2c0], xmm12
14076da2b movaps xmmword ptr [rbp + 0x2b0], xmm11
14076da33 movaps xmmword ptr [rbp + 0x2a0], xmm10
14076da3b movaps xmmword ptr [rbp + 0x290], xmm9
14076da43 movaps xmmword ptr [rbp + 0x280], xmm8
14076da4b movaps xmmword ptr [rbp + 0x270], xmm7
14076da52 movdqa xmmword ptr [rbp + 0x260], xmm6
14076da5a mov qword ptr [rbp + 0x258], 0xfffffffffffffffe
14076da65 mov qword ptr [rbp + 0x248], r9
14076da6c mov r12, r8
14076da6f mov rbx, rdx
14076da72 mov rdi, rcx
14076da75 call 0x140c16a00
14076da7a mov byte ptr [rbp + 0x256], al
14076da80 test al, al
14076da82 je 0x14076dc9e
14076da88 mov esi, 0x3d
14076da8d mov ecx, 0x3d
14076da92 call 0x14076d5c0
14076da97 lea r14, [rip + 0x2310dfa]
14076da9e cmp rax, 1
14076daa2 jne 0x14076db48
14076daa8 mov r9, rdx
14076daab test rdx, rdx
14076daae je 0x14076dae6
14076dab0 cmp r9, 0x3d
14076dab4 jae 0x14076dae4
14076dab6 lea rax, [rip + 0x2310ddb]
14076dabd cmp byte ptr [r9 + rax], 0xbf
14076dac2 jg 0x14076dae6
14076dac4 lea rax, [rip + 0x2310e85]
14076dacb mov qword ptr [rsp + 0x20], rax
14076dad0 lea rcx, [rip + 0x2310dc1]
14076dad7 mov edx, 0x3d
14076dadc xor r8d, r8d
14076dadf call 0x1428d9940
14076dae4 jne 0x14076dac4
14076dae6 mov rcx, r9
14076dae9 call 0x14076d5c0
14076daee cmp rax, 1
14076daf2 jne 0x14076db48
14076daf4 mov r8, rdx
14076daf7 add r8, 2
14076dafb je 0x14076db36
14076dafd cmp r8, 0x3d
14076db01 jae 0x14076db34
14076db03 lea rax, [rip + 0x2310d8e]
14076db0a cmp byte ptr [r8 + rax], 0xbf
14076db0f jg 0x14076db36
14076db11 lea rax, [rip + 0x2310e50]
14076db18 mov qword ptr [rsp + 0x20], rax
14076db1d lea rcx, [rip + 0x2310d74]
14076db24 mov edx, 0x3d
14076db29 mov r9d, 0x3d
14076db2f call 0x1428d9940
14076db34 jne 0x14076db11
14076db36 mov esi, 0x3b
14076db3b sub rsi, rdx
14076db3e lea r14, [rip + 0x2310d53]
14076db45 add r14, r8
14076db48 lea r8, [rip + 0x2310e85]
14076db4f lea r13, [rip + 0x2310e42]
14076db56 nop word ptr cs:[rax + rax]
14076db60 movsx eax, byte ptr [r8 - 1]
14076db65 test eax, eax
14076db67 js 0x14076db80
14076db69 dec r8
14076db6c cmp eax, 0x5c
14076db6f jne 0x14076dbd4
14076db71 jmp 0x14076dbe6
14076db73 nop word ptr cs:[rax + rax]
14076db80 movzx ecx, byte ptr [r8 - 2]
14076db85 cmp cl, 0xc0
14076db88 jge 0x14076dbad
14076db8a movzx edx, byte ptr [r8 - 3]
14076db8f cmp dl, 0xc0
14076db92 jge 0x14076dbb6
14076db94 movzx r9d, byte ptr [r8 - 4]
14076db99 add r8, -4
14076db9d and r9d, 7
14076dba1 shl r9d, 6
14076dba5 and edx, 0x3f
14076dba8 or edx, r9d
14076dbab jmp 0x14076dbbd
14076dbad add r8, -2
14076dbb1 and ecx, 0x1f
14076dbb4 jmp 0x14076dbc5
14076dbb6 add r8, -3
14076dbba and edx, 0xf
14076dbbd shl edx, 6
14076dbc0 and ecx, 0x3f
14076dbc3 or ecx, edx
14076dbc5 shl ecx, 6
14076dbc8 and al, 0x3f
14076dbca movzx eax, al
14076dbcd or eax, ecx
14076dbcf cmp eax, 0x5c
14076dbd2 je 0x14076dbe6
14076dbd4 cmp eax, 0x2f
14076dbd7 je 0x14076dbe6
14076dbd9 cmp r8, r13
14076dbdc jne 0x14076db60
14076dbde mov r15d, 0x3c
14076dbe4 jmp 0x14076dc33
14076dbe6 lea r13, [rip + 0x2310dab]
14076dbed sub r8, r13
14076dbf0 inc r8
14076dbf3 je 0x14076dc27
14076dbf5 cmp r8, 0x3c
14076dbf9 jae 0x14076dc25
14076dbfb cmp byte ptr [r8 + r13], 0xbf
14076dc00 jg 0x14076dc27
14076dc02 lea rax, [rip + 0x2310d27]
14076dc09 mov qword ptr [rsp + 0x20], rax
14076dc0e lea rcx, [rip + 0x2310d83]
14076dc15 mov edx, 0x3c
14076dc1a mov r9d, 0x3c
14076dc20 call 0x1428d9940
14076dc25 jne 0x14076dc02
14076dc27 mov r15d, 0x3c
14076dc2d sub r15, r8
14076dc30 add r13, r8
14076dc33 call 0x140c18130
14076dc38 mov rcx, rax
14076dc3b mov rax, qword ptr [rax]
14076dc3e cmp rax, 1
14076dc42 jne 0x14076f5a7
14076dc48 add rcx, 8
14076dc4c cmp qword ptr [rcx], 0
14076dc50 jne 0x14076f6ae
14076dc56 mov qword ptr [rcx], 0xffffffffffffffff
14076dc5d mov qword ptr [rbp + 0x240], rcx
14076dc64 add rcx, 8
14076dc68 mov qword ptr [rsp + 0x20], r15
14076dc6d mov qword ptr [rsp + 0x30], 0
14076dc76 mov qword ptr [rsp + 0x28], 1
14076dc7f mov rdx, r14
14076dc82 mov r8, rsi
14076dc85 mov r9, r13
14076dc88 call 0x140c16d40
14076dc8d mov qword ptr [rbp + 0x170], rax
14076dc94 mov rax, qword ptr [rbp + 0x240]
14076dc9b inc qword ptr [rax]
14076dc9e mov rax, qword ptr [rbp + 0x370]
14076dca5 mov rcx, qword ptr [rax]
14076dca8 lea rdx, [rip + 0x24acb21]
14076dcaf call 0x141209c40
14076dcb4 mov rsi, rax
14076dcb7 test rax, rax
14076dcba je 0x14076f852
14076dcc0 lea rax, [rip + 0x24acca1]
14076dcc7 mov qword ptr [rbp], rax
14076dccb mov qword ptr [rbp + 8], 0x3d
14076dcd3 mov r14, rsi
14076dcd6 add r14, 0x10
14076dcda cmp qword ptr [rsi + 0x18], 0x3d
14076dcdf jne 0x14076f5ec
14076dce5 mov rdx, qword ptr [r14]
14076dce8 lea rcx, [rip + 0x24acc79]
14076dcef mov r8d, 0x3d
14076dcf5 call 0x142923bf0
14076dcfa test eax, eax
14076dcfc jne 0x14076f5ec
14076dd02 mov rcx, qword ptr [rsi]
14076dd05 mov rax, qword ptr [rsi + 8]
14076dd09 call qword ptr [rax + 0x18]
14076dd0c mov r14, rax
14076dd0f test rax, rax
14076dd12 je 0x14076f852
14076dd18 mov byte ptr [rbp + 0x255], 0
14076dd1f mov rax, qword ptr [rdi]
14076dd22 mov rdx, qword ptr [rdi + 8]
14076dd26 mov r10, qword ptr [rax]
14076dd29 xor r9d, r9d
14076dd2c mov r11, r10
14076dd2f sub r11, qword ptr [rdx + 0x18]
14076dd33 cmovb r11, r9
14076dd37 sub r10, qword ptr [rdx + 0x38]
14076dd3b cmovb r10, r9
14076dd3f mov rcx, r11
14076dd42 shl rcx, 4
14076dd46 add rcx, qword ptr [rdx + 8]
14076dd4a mov r8, qword ptr [rdx + 0x10]
14076dd4e sub r8, r11
14076dd51 cmovb r8, r9
14076dd55 mov esi, 8
14076dd5a cmovb rcx, rsi
14076dd5e mov r11, r10
14076dd61 shl r11, 4
14076dd65 add r11, qword ptr [rdx + 0x28]
14076dd69 mov rdi, qword ptr [rdx + 0x30]
14076dd6d sub rdi, r10
14076dd70 cmovb rdi, r9
14076dd74 cmovb r11, rsi
14076dd78 lea r9, [rdi + r8]
14076dd7c mov rdx, qword ptr [rdx + 0x40]
14076dd80 sub rdx, r9
14076dd83 mov qword ptr [rax], rdx
14076dd86 shl r8, 4
14076dd8a add r8, rcx
14076dd8d shl rdi, 4
14076dd91 add rdi, r11
14076dd94 mov qword ptr [rbp + 0x180], rax
14076dd9b mov qword ptr [rbp + 0x188], rcx
14076dda2 mov qword ptr [rbp + 0x190], r8
14076dda9 mov qword ptr [rbp + 0x198], r11
14076ddb0 mov qword ptr [rbp + 0x1a0], rdi
14076ddb7 mov qword ptr [rbp + 0x1a8], r9
14076ddbe lea r8, [rip + 0x2310abb]
14076ddc5 lea rcx, [rbp + 0x158]
14076ddcc lea rdx, [rbp + 0x180]
14076ddd3 call 0x1412036d0
14076ddd8 mov qword ptr [rbp + 0x240], r12
14076dddf mov qword ptr [rbp + 0xa8], r14
14076dde6 mov r13, qword ptr [rbx]
14076dde9 mov rdx, qword ptr [rbx + 8]
14076dded mov r9, qword ptr [r13]
14076ddf1 xor r8d, r8d
14076ddf4 mov rcx, r9
14076ddf7 sub rcx, qword ptr [rdx + 0x18]
14076ddfb cmovb rcx, r8
14076ddff sub r9, qword ptr [rdx + 0x38]
14076de03 cmovb r9, r8
14076de07 mov rax, rcx
14076de0a shl rax, 6
14076de0e add rax, qword ptr [rdx + 8]
14076de12 mov r15, qword ptr [rdx + 0x10]
14076de16 sub r15, rcx
14076de19 cmovb r15, r8
14076de1d cmovb rax, rsi
14076de21 mov rcx, r9
14076de24 shl rcx, 6
14076de28 add rcx, qword ptr [rdx + 0x28]
14076de2c mov r12, qword ptr [rdx + 0x30]
14076de30 sub r12, r9
14076de33 cmovb r12, r8
14076de37 cmovb rcx, rsi
14076de3b mov rdx, qword ptr [rdx + 0x40]
14076de3f sub rdx, r12
14076de42 sub rdx, r15
14076de45 mov qword ptr [r13], rdx
14076de49 shl r15, 6
14076de4d add r15, rax
14076de50 shl r12, 6
14076de54 add r12, rcx
14076de57 jmp 0x14076dea8
14076de59 test rdi, rdi
14076de5c je 0x14076df1a
14076de62 mov r8, r12
14076de65 sub r8, rdi
14076de68 shr r8, 6
14076de6c inc r8
14076de6f mov qword ptr [rsp + 0x20], 8
14076de78 mov r9d, 8
14076de7e lea rcx, [rbp + 0x158]
14076de85 mov rdx, rbx
14076de88 call 0x1428a7de0
14076de8d mov rax, qword ptr [rbp + 0x160]
14076de94 mov qword ptr [rax + rbx*8], rsi
14076de98 inc rbx
14076de9b mov qword ptr [rbp + 0x168], rbx
14076dea2 mov rcx, rdi
14076dea5 mov rax, r14
14076dea8 test rax, rax
14076deab je 0x14076dec0
14076dead cmp rax, r15
14076deb0 je 0x14076dec0
14076deb2 lea r14, [rax + 0x40]
14076deb6 mov rdi, rcx
14076deb9 jmp 0x14076ded4
14076debb nop dword ptr [rax + rax]
14076dec0 test rcx, rcx
14076dec3 je 0x14076df22
14076dec5 cmp rcx, r12
14076dec8 je 0x14076df22
14076deca lea rdi, [rcx + 0x40]
14076dece xor r14d, r14d
14076ded1 mov rax, rcx
14076ded4 inc qword ptr [r13]
14076ded8 mov rsi, qword ptr [rax + 8]
14076dedc mov rbx, qword ptr [rbp + 0x168]
14076dee3 cmp rbx, qword ptr [rbp + 0x158]
14076deea jne 0x14076de8d
14076deec test r14, r14
14076deef je 0x14076de59
14076def5 mov r8, r15
14076def8 sub r8, r14
14076defb shr r8, 6
14076deff test rdi, rdi
14076df02 je 0x14076de6c
14076df08 mov rax, r12
14076df0b sub rax, rdi
14076df0e shr rax, 6
14076df12 add r8, rax
14076df15 jmp 0x14076de6c
14076df1a xor r8d, r8d
14076df1d jmp 0x14076de6c
14076df22 mov r12, qword ptr [rbp + 0x160]
14076df29 mov rsi, qword ptr [rbp + 0x168]
14076df30 cmp rsi, 1
14076df34 ja 0x14076f61c
14076df3a lea r13, [r12 + rsi*8]
14076df3e mov rdx, qword ptr [rbp + 0x240]
14076df45 mov r15, qword ptr [rdx]
14076df48 mov rax, qword ptr [rdx + 0x10]
14076df4c mov qword ptr [rbp + 0x58], rax
14076df50 lea rax, [r15 + 0x20]
14076df54 mov qword ptr [rbp + 0xa0], rax
14076df5b mov rcx, qword ptr [rbp + 0x248]
14076df62 mov eax, dword ptr [rcx + 0x1c]
14076df65 mov dword ptr [rbp + 0x104], eax
14076df6b mov rax, qword ptr [rcx]
14076df6e mov rcx, qword ptr [rcx + 0x10]
14076df72 mov qword ptr [rbp + 0x68], rcx
14076df76 mov qword ptr [rbp + 0x60], rax
14076df7a lea rax, [rax + 0x20]
14076df7e mov qword ptr [rbp + 0x70], rax
14076df82 mov eax, dword ptr [rdx + 0x1c]
14076df85 mov dword ptr [rbp + 0x100], eax
14076df8b mov rcx, qword ptr [rbp + 0x380]
14076df92 mov eax, dword ptr [rcx + 0x1c]
14076df95 mov dword ptr [rbp + 0x108], eax
14076df9b mov rax, qword ptr [rcx]
14076df9e mov qword ptr [rbp + 0x78], rax
14076dfa2 mov rax, qword ptr [rcx + 0x10]
14076dfa6 mov qword ptr [rbp + 0x80], rax
14076dfad pcmpeqd xmm6, xmm6
14076dfb1 movss xmm11, dword ptr [rip + 0x22711b2]
14076dfba xorps xmm12, xmm12
14076dfbe xorps xmm13, xmm13
14076dfc2 movaps xmm14, xmmword ptr [rip + 0x2310796]
14076dfca movss xmm15, dword ptr [rip + 0x23107a1]
14076dfd3 cmp r12, r13
14076dfd6 je 0x14076f440
14076dfdc mov rsi, qword ptr [r12]
14076dfe0 mov qword ptr [rbp + 0x228], rsi
14076dfe7 cmp qword ptr [r15 + 0x18], 0
14076dfec je 0x14076f3e7
14076dff2 add r12, 8
14076dff6 mov rcx, qword ptr [rbp + 0xa0]
14076dffd lea rdx, [rbp + 0x228]
14076e004 call 0x1405b0d70
14076e009 mov rbx, qword ptr [r15]
14076e00c mov rcx, qword ptr [r15 + 8]
14076e010 mov rdx, rcx
14076e013 and rdx, rax
14076e016 shr rax, 0x39
14076e01a movd xmm0, eax
14076e01e punpcklbw xmm0, xmm0
14076e022 pshuflw xmm0, xmm0, 0
14076e027 pshufd xmm0, xmm0, 0
14076e02c lea rax, [rbx - 0x2a8]
14076e033 xor r8d, r8d
14076e036 movdqu xmm1, xmmword ptr [rbx + rdx]
14076e03b movdqa xmm2, xmm1
14076e03f pcmpeqb xmm2, xmm0
14076e043 pmovmskb r9d, xmm2
14076e048 test r9d, r9d
14076e04b je 0x14076e080
14076e04d tzcnt r10d, r9d
14076e052 add r10, rdx
14076e055 and r10, rcx
14076e058 neg r10
14076e05b imul rdi, r10, 0x2a8
14076e062 cmp rsi, qword ptr [rax + rdi]
14076e066 je 0x14076e0b0
14076e068 lea r10d, [r9 - 1]
14076e06c and r10w, r9w
14076e070 mov r9d, r10d
14076e073 jne 0x14076e04d
14076e075 nop word ptr cs:[rax + rax]
14076e080 pcmpeqb xmm1, xmm6
14076e084 pmovmskb r9d, xmm1
14076e089 test r9d, r9d
14076e08c jne 0x14076f3e7
14076e092 add rdx, r8
14076e095 add rdx, 0x10
14076e099 add r8, 0x10
14076e09d and rdx, rcx
14076e0a0 jmp 0x14076e036
14076e0a2 nop word ptr cs:[rax + rax]
14076e0b0 mov r14, qword ptr [rbx + rdi - 0x170]
14076e0b8 test r14, r14
14076e0bb je 0x14076f863
14076e0c1 lea rcx, [r14 + 0x18]
14076e0c5 xor eax, eax
14076e0c7 mov dl, 1
14076e0c9 lock cmpxchg byte ptr [r14 + 0x18], dl
14076e0cf jne 0x14076f3d1
14076e0d5 call 0x140c81c70
14076e0da mov rax, qword ptr [r14 + 0x10]
14076e0de test rax, rax
14076e0e1 sete cl
14076e0e4 shl rax, 2
14076e0e8 add rax, qword ptr [r14 + 8]
14076e0ec add rax, -4
14076e0f0 sete dl
14076e0f3 or dl, cl
14076e0f5 je 0x14076e110
14076e0f7 mov r8, qword ptr [rbp + 0xa8]
14076e0fe mov rax, qword ptr [r8 + 0x28]
14076e102 test rax, rax
14076e105 je 0x14076fa35
14076e10b lea ecx, [rax - 1]
14076e10e jmp 0x14076e11d
14076e110 mov ecx, dword ptr [rax]
14076e112 mov r8, qword ptr [rbp + 0xa8]
14076e119 mov rax, qword ptr [r8 + 0x28]
14076e11d lea r9, [rbx + rdi]
14076e121 mov rdx, qword ptr [r8 + 0x20]
14076e125 movups xmm0, xmmword ptr [r8 + 0x30]
14076e12a movups xmm1, xmmword ptr [r8 + 0x40]
14076e12f movups xmm2, xmmword ptr [r8 + 0x50]
14076e134 mov dword ptr [rbp + 0x50], ecx
14076e137 mov qword ptr [rbp], rdx
14076e13b mov qword ptr [rbp + 8], rax
14076e13f movups xmmword ptr [rbp + 0x10], xmm0
14076e143 mov qword ptr [rbp + 0x20], r14
14076e147 movups xmmword ptr [rbp + 0x28], xmm1
14076e14b mov qword ptr [rbp + 0x38], r8
14076e14f movups xmmword ptr [rbp + 0x40], xmm2
14076e153 mov rax, rbp
14076e156 mov qword ptr [rbp - 0x10], rax
14076e15a lea rax, [rbp + 0x255]
14076e161 mov qword ptr [rbp - 8], rax
14076e165 mov qword ptr [rbp + 0x140], 0
14076e170 mov qword ptr [rbp + 0x148], 8
14076e17b mov qword ptr [rbp + 0x150], 0
14076e186 xorps xmm0, xmm0
14076e189 movss dword ptr [rbp + 0x23c], xmm0
14076e191 mov qword ptr [rbp + 0x240], r9
14076e198 cmp byte ptr [r9 - 0xa], 0
14076e19d je 0x14076f0d0
14076e1a3 mov eax, dword ptr [rbp + 0x104]
14076e1a9 mov rcx, qword ptr [rbp + 0x68]
14076e1ad mov dword ptr [rcx], eax
14076e1af mov rsi, qword ptr [rbp + 0x60]
14076e1b3 cmp qword ptr [rsi + 0x18], 0
14076e1b8 je 0x14076f428
14076e1be lea rax, [rbx + rdi]
14076e1c2 add rax, -0x2a0
14076e1c8 mov qword ptr [rbp + 0x1e8], rax
14076e1cf mov rcx, qword ptr [rbp + 0x70]
14076e1d3 lea rdx, [rbp + 0x228]
14076e1da call 0x1405b0d70
14076e1df mov rcx, qword ptr [rsi]
14076e1e2 mov rdx, qword ptr [rsi + 8]
14076e1e6 mov r8, rdx
14076e1e9 and r8, rax
14076e1ec shr rax, 0x39
14076e1f0 movd xmm0, eax
14076e1f4 punpcklbw xmm0, xmm0
14076e1f8 pshuflw xmm0, xmm0, 0
14076e1fd pshufd xmm0, xmm0, 0
14076e202 lea rax, [rcx - 0x20]
14076e206 mov r9, qword ptr [rbp + 0x228]
14076e20d xor r10d, r10d
14076e210 movdqu xmm1, xmmword ptr [rcx + r8]
14076e216 movdqa xmm2, xmm1
14076e21a pcmpeqb xmm2, xmm0
14076e21e pmovmskb r11d, xmm2
14076e223 test r11d, r11d
14076e226 je 0x14076e252
14076e228 tzcnt esi, r11d
14076e22d add rsi, r8
14076e230 and rsi, rdx
14076e233 mov rdi, rsi
14076e236 shl rdi, 5
14076e23a mov rbx, rax
14076e23d sub rbx, rdi
14076e240 cmp r9, qword ptr [rbx]
14076e243 je 0x14076e274
14076e245 lea esi, [r11 - 1]
14076e249 and si, r11w
14076e24d mov r11d, esi
14076e250 jne 0x14076e228
14076e252 pcmpeqb xmm1, xmm6
14076e256 pmovmskb r11d, xmm1
14076e25b test r11d, r11d
14076e25e jne 0x14076f428
14076e264 add r8, r10
14076e267 add r8, 0x10
14076e26b add r10, 0x10
14076e26f and r8, rdx
14076e272 jmp 0x14076e210
14076e274 mov qword ptr [rbp + 0x118], r15
14076e27b neg rsi
14076e27e shl rsi, 5
14076e282 mov rdx, qword ptr [rcx + rsi - 0x10]
14076e287 mov rax, qword ptr [rcx + rsi - 8]
14076e28c mov rcx, rax
14076e28f shl rcx, 7
14076e293 lea rax, [rcx + rax*8]
14076e297 add rax, rdx
14076e29a pxor xmm0, xmm0
14076e29e movd dword ptr [rbp + 0x23c], xmm0
14076e2a6 mov qword ptr [rbp + 0x120], r12
14076e2ad mov qword ptr [rbp + 0x128], r13
14076e2b4 mov qword ptr [rbp + 0x98], rax
14076e2bb cmp rdx, rax
14076e2be je 0x14076f0bc
14076e2c4 mov qword ptr [rbp + 0x248], rdx
14076e2cb mov rax, qword ptr [rbp + 0x150]
14076e2d2 mov qword ptr [rbp + 0x230], rax
14076e2d9 mov rax, qword ptr [rbp + 0x240]
14076e2e0 mov rcx, qword ptr [rax - 0x1d8]
14076e2e7 mov qword ptr [rbp + 0x130], rcx
14076e2ee mov rcx, qword ptr [rax - 0x1d0]
14076e2f5 mov qword ptr [rbp + 0x138], rcx
14076e2fc mov rdi, qword ptr [rax - 0x1c0]
14076e303 mov r15, qword ptr [rax - 0x1b8]
14076e30a mov r12, qword ptr [rax - 0x210]
14076e311 mov r13, qword ptr [rax - 0x208]
14076e318 mov r14, qword ptr [rax - 0x1f8]
14076e31f mov rsi, qword ptr [rax - 0x1f0]
14076e326 mov ebx, dword ptr [rax - 0x48]
14076e329 mov byte ptr [rbp + 0x257], 1
14076e330 mov rcx, qword ptr [rbp + 0x1e8]
14076e337 call 0x140b2a770
14076e33c mov dword ptr [rbp + 0x180], eax
14076e342 movd dword ptr [rbp + 0x184], xmm0
14076e34a mov byte ptr [rbp + 0x257], 1
14076e351 cmp ebx, 2
14076e354 setb byte ptr [rsp + 0x50]
14076e359 lea rax, [rbp + 0x180]
14076e360 mov qword ptr [rsp + 0x58], rax
14076e365 mov qword ptr [rsp + 0x48], rsi
14076e36a mov qword ptr [rsp + 0x40], r14
14076e36f mov qword ptr [rsp + 0x38], r13
14076e374 mov qword ptr [rsp + 0x30], r12
14076e379 mov qword ptr [rsp + 0x28], r15
14076e37e mov qword ptr [rsp + 0x20], rdi
14076e383 lea rcx, [rbp + 0xe8]
14076e38a mov rdx, qword ptr [rbp + 0x248]
14076e391 mov r8, qword ptr [rbp + 0x130]
14076e398 mov r9, qword ptr [rbp + 0x138]
14076e39f call 0x14122ff90
14076e3a4 mov rax, qword ptr [rbp + 0xf0]
14076e3ab mov r8, qword ptr [rbp + 0xf8]
14076e3b2 mov qword ptr [rbp + 0xf8], 0
14076e3bd imul rcx, r8, 0x68
14076e3c1 add rcx, rax
14076e3c4 mov qword ptr [rbp + 0xc0], rax
14076e3cb mov qword ptr [rbp + 0xc8], rcx
14076e3d2 lea rax, [rbp + 0xe8]
14076e3d9 mov qword ptr [rbp + 0xd0], rax
14076e3e0 mov qword ptr [rbp + 0xd8], r8
14076e3e7 mov qword ptr [rbp + 0xe0], 0
14076e3f2 mov rax, qword ptr [rbp + 0x140]
14076e3f9 mov rdx, qword ptr [rbp + 0x230]
14076e400 sub rax, rdx
14076e403 mov rsi, rdx
14076e406 cmp rax, r8
14076e409 jb 0x14076f08e
14076e40f mov rbx, qword ptr [rbp + 0x148]
14076e416 mov rax, qword ptr [rbp + 0xc0]
14076e41d mov rcx, qword ptr [rbp + 0xc8]
14076e424 mov rdx, qword ptr [rbp + 0xd0]
14076e42b mov r8, qword ptr [rbp + 0xd8]
14076e432 mov qword ptr [rbp + 0x1f0], rax
14076e439 mov qword ptr [rbp + 0x1f8], rcx
14076e440 mov rax, qword ptr [rbp + 0xe0]
14076e447 mov qword ptr [rbp + 0x210], rax
14076e44e mov qword ptr [rbp + 0x200], rdx
14076e455 mov qword ptr [rbp + 0x208], r8
14076e45c mov r9, qword ptr [rbp + 0x1f0]
14076e463 mov rax, qword ptr [rbp + 0x1f8]
14076e46a cmp r9, rax
14076e46d je 0x14076e559
14076e473 imul rcx, rsi, 0x68
14076e477 add rcx, rbx
14076e47a add rcx, 8
14076e47e add r9, 0x68
14076e482 nop word ptr cs:[rax + rax]
14076e490 mov rdx, r9
14076e493 mov r9, qword ptr [r9 - 0x68]
14076e497 mov r8, r9
14076e49a neg r8
14076e49d jo 0x14076e552
14076e4a3 lea r8, [rdx - 0x68]
14076e4a7 movups xmm0, xmmword ptr [r8 + 0x58]
14076e4ac movaps xmmword ptr [rbp + 0x1d0], xmm0
14076e4b3 movups xmm0, xmmword ptr [r8 + 0x48]
14076e4b8 movaps xmmword ptr [rbp + 0x1c0], xmm0
14076e4bf movups xmm0, xmmword ptr [r8 + 8]
14076e4c4 movups xmm1, xmmword ptr [r8 + 0x18]
14076e4c9 movups xmm2, xmmword ptr [r8 + 0x28]
14076e4ce movups xmm3, xmmword ptr [r8 + 0x38]
14076e4d3 movaps xmmword ptr [rbp + 0x1b0], xmm3
14076e4da movaps xmmword ptr [rbp + 0x1a0], xmm2
14076e4e1 movaps xmmword ptr [rbp + 0x190], xmm1
14076e4e8 movaps xmmword ptr [rbp + 0x180], xmm0
14076e4ef mov qword ptr [rcx - 8], r9
14076e4f3 movaps xmm0, xmmword ptr [rbp + 0x180]
14076e4fa movdqa xmm1, xmmword ptr [rbp + 0x190]
14076e502 movdqa xmm2, xmmword ptr [rbp + 0x1a0]
14076e50a movaps xmm3, xmmword ptr [rbp + 0x1b0]
14076e511 movups xmmword ptr [rcx], xmm0
14076e514 movdqu xmmword ptr [rcx + 0x10], xmm1
14076e519 movdqu xmmword ptr [rcx + 0x20], xmm2
14076e51e movups xmmword ptr [rcx + 0x30], xmm3
14076e522 movaps xmm0, xmmword ptr [rbp + 0x1c0]
14076e529 movups xmmword ptr [rcx + 0x40], xmm0
14076e52d movdqa xmm0, xmmword ptr [rbp + 0x1d0]
14076e535 movdqu xmmword ptr [rcx + 0x50], xmm0
14076e53a inc rsi
14076e53d add rcx, 0x68
14076e541 lea r9, [rdx + 0x68]
14076e545 add r8, 0x68
14076e549 cmp r8, rax
14076e54c jne 0x14076e490
14076e552 mov qword ptr [rbp + 0x1f0], rdx
14076e559 mov qword ptr [rbp + 0x150], rsi
14076e560 mov byte ptr [rbp + 0x254], 0
14076e567 lea rcx, [rbp + 0x1f0]
14076e56e call 0x1411fd610
14076e573 mov byte ptr [rbp + 0x257], 1
14076e57a lea rcx, [rbp + 0xe8]
14076e581 call 0x1405d3750
14076e586 mov rdx, qword ptr [rbp + 0x230]
14076e58d cmp rdx, rsi
14076e590 ja 0x14076f7ae
14076e596 mov rax, qword ptr [rbp + 0x248]
14076e59d lea rcx, [rax + 0x88]
14076e5a4 mov qword ptr [rbp + 0x90], rcx
14076e5ab movss xmm0, dword ptr [rbp + 0x23c]
14076e5b3 addss xmm0, dword ptr [rax + 0x84]
14076e5bb movss dword ptr [rbp + 0x23c], xmm0
14076e5c3 imul r12, rdx, 0x68
14076e5c7 add r12, rbx
14076e5ca imul r14, rsi, 0x68
14076e5ce add r14, rbx
14076e5d1 xor r8d, r8d
14076e5d4 mov r15, r12
14076e5d7 mov qword ptr [rbp + 0x130], r14
14076e5de cmp r15, r14
14076e5e1 je 0x14076ea8f
14076e5e7 mov rax, qword ptr [r15 + 0x10]
14076e5eb test rax, rax
14076e5ee je 0x14076f8e9
14076e5f4 mov r14, qword ptr [r15 + 0x28]
14076e5f8 test r14, r14
14076e5fb je 0x14076f8cd
14076e601 lea rcx, [rax - 1]
14076e605 movabs rdx, 0x3fffffffffffffff
14076e60f and rcx, rdx
14076e612 cmp r14, rcx
14076e615 jbe 0x14076f9fc
14076e61b mov rdi, r12
14076e61e lea rcx, [r15 + 0x68]
14076e622 mov qword ptr [rbp + 0x88], rcx
14076e629 mov qword ptr [rbp + 0x138], r8
14076e630 lea ecx, [r8 + 1]
14076e634 mov dword ptr [rbp + 0x10c], ecx
14076e63a mov rcx, qword ptr [rbp + 0x240]
14076e641 movss xmm8, dword ptr [rcx - 0x1c]
14076e647 movss xmm9, dword ptr [rcx - 0x18]
14076e64d mov rsi, qword ptr [r15 + 8]
14076e651 mov rdx, qword ptr [r15 + 0x20]
14076e655 lea rcx, [rdx + 4]
14076e659 mov r13, rdx
14076e65c movss xmm7, dword ptr [rdx + 4]
14076e661 movss xmm0, dword ptr [rsi]
14076e665 addss xmm0, xmm7
14076e669 shl rax, 2
14076e66d xor edx, edx
14076e66f nop 
14076e670 movss xmm1, dword ptr [rcx]
14076e674 movss xmm2, dword ptr [rsi + rdx]
14076e679 addss xmm2, xmm1
14076e67d movaps xmm3, xmm0
14076e680 cmpltss xmm3, xmm2
14076e685 andps xmm1, xmm3
14076e688 andnps xmm3, xmm7
14076e68b orps xmm3, xmm1
14076e68e movaps xmm7, xmm3
14076e691 maxss xmm2, xmm0
14076e695 add rcx, 0xc
14076e699 add rdx, 4
14076e69d movaps xmm0, xmm2
14076e6a0 cmp rax, rdx
14076e6a3 jne 0x14076e670
14076e6a5 addss xmm8, dword ptr [rip + 0x21b71c2]
14076e6ae lea r12, [r14*4]
14076e6b6 movzx eax, byte ptr [rip + 0x2ec654c]
14076e6bd mov edx, 4
14076e6c2 mov rcx, r12
14076e6c5 call 0x140613c10
14076e6ca test rax, rax
14076e6cd je 0x14076f8ad
14076e6d3 mov rbx, rax
14076e6d6 movaps xmm0, xmm9
14076e6da cmpunordss xmm0, xmm9
14076e6e0 movaps xmm1, xmm0
14076e6e3 andps xmm1, xmm8
14076e6e7 maxss xmm8, xmm9
14076e6ec andnps xmm0, xmm8
14076e6f0 orps xmm0, xmm1
14076e6f3 cmp r14, 3
14076e6f7 ja 0x14076e710
14076e6f9 xor eax, eax
14076e6fb mov r12, rdi
14076e6fe jmp 0x14076e7b0
14076e703 nop word ptr cs:[rax + rax]
14076e710 lea rax, [rbx + r14*4]
14076e714 lea rcx, [r13 + 4]
14076e718 lea rdx, [r14 + r14*2]
14076e71c lea rdx, [rdx*4 - 4]
14076e724 add rdx, r13
14076e727 cmp rbx, rdx
14076e72a setb dl
14076e72d cmp rcx, rax
14076e730 setb al
14076e733 test dl, al
14076e735 mov r12, rdi
14076e738 je 0x14076e73e
14076e73a xor eax, eax
14076e73c jmp 0x14076e7b0
14076e73e mov rax, r14
14076e741 and rax, 0xfffffffffffffffc
14076e745 movaps xmm1, xmm7
14076e748 shufps xmm1, xmm7, 0
14076e74c movaps xmm2, xmm0
14076e74f shufps xmm2, xmm0, 0
14076e753 lea rcx, [r13 + 0x28]
14076e757 xor edx, edx
14076e759 nop dword ptr [rax]
14076e760 movss xmm3, dword ptr [rcx - 0x24]
14076e765 movss xmm4, dword ptr [rcx - 0x18]
14076e76a unpcklps xmm3, xmm4
14076e76d movss xmm4, dword ptr [rcx - 0xc]
14076e772 movss xmm5, dword ptr [rcx]
14076e776 unpcklps xmm4, xmm5
14076e779 movlhps xmm3, xmm4
14076e77c movaps xmm4, xmm1
14076e77f subps xmm4, xmm3
14076e782 addps xmm4, xmm14
14076e786 maxps xmm4, xmm13
14076e78a addps xmm4, xmm2
14076e78d movups xmmword ptr [rbx + rdx*4], xmm4
14076e791 add rdx, 4
14076e795 add rcx, 0x30
14076e799 cmp rax, rdx
14076e79c jne 0x14076e760
14076e79e cmp r14, rax
14076e7a1 je 0x14076e7e6
14076e7a3 nop word ptr cs:[rax + rax]
14076e7b0 lea rcx, [rax + rax*2]
14076e7b4 lea rcx, [rcx*4 + 4]
14076e7bc add rcx, r13
14076e7bf nop 
14076e7c0 movaps xmm1, xmm7
14076e7c3 subss xmm1, dword ptr [rcx]
14076e7c7 addss xmm1, xmm11
14076e7cc maxss xmm1, xmm12
14076e7d1 addss xmm1, xmm0
14076e7d5 movss dword ptr [rbx + rax*4], xmm1
14076e7da inc rax
14076e7dd add rcx, 0xc
14076e7e1 cmp r14, rax
14076e7e4 jne 0x14076e7c0
14076e7e6 mov rdx, qword ptr [r15]
14076e7e9 test rdx, rdx
14076e7ec je 0x14076e800
14076e7ee shl rdx, 2
14076e7f2 mov r8d, 4
14076e7f8 mov rcx, rsi
14076e7fb call 0x140613c20
14076e800 mov qword ptr [r15], r14
14076e803 mov qword ptr [r15 + 8], rbx
14076e807 mov qword ptr [r15 + 0x10], r14
14076e80b mov eax, dword ptr [r15 + 0x60]
14076e80f lea rcx, [rbp + 0x188]
14076e816 mov dword ptr [rcx + 8], eax
14076e819 mov rax, qword ptr [r15 + 0x58]
14076e81d mov qword ptr [rcx], rax
14076e820 lea rax, [rbx + r14*4]
14076e824 movups xmmword ptr [rcx + 0x20], xmm13
14076e829 mov qword ptr [rcx + 0x30], 0
14076e831 mov rcx, qword ptr [rbp + 0x1e8]
14076e838 mov qword ptr [rbp + 0x180], rcx
14076e83f mov qword ptr [rbp + 0x198], rbx
14076e846 mov qword ptr [rbp + 0x1a0], rax
14076e84d mov byte ptr [rbp + 0x257], 1
14076e854 lea rcx, [rbp + 0xc0]
14076e85b lea rdx, [rbp + 0x180]
14076e862 lea r8, [rip + 0x2310017]
14076e869 call 0x141204400
14076e86e mov r14, qword ptr [rbp + 0x130]
14076e875 mov rdi, qword ptr [r15 + 0x10]
14076e879 test rdi, rdi
14076e87c mov qword ptr [rbp + 0x230], rdi
14076e883 je 0x14076f88d
14076e889 movzx eax, byte ptr [rip + 0x2ec6379]
14076e890 mov edx, 1
14076e895 mov rcx, rdi
14076e898 call 0x140613c40
14076e89d test rax, rax
14076e8a0 je 0x14076f874
14076e8a6 mov r13, rax
14076e8a9 mov byte ptr [rax], 8
14076e8ac mov rcx, qword ptr [r15 + 0x10]
14076e8b0 dec rcx
14076e8b3 cmp rcx, rdi
14076e8b6 jae 0x14076fa1a
14076e8bc mov byte ptr [r13 + rcx], 8
14076e8c2 mov rax, qword ptr [r15 + 0x10]
14076e8c6 mov qword ptr [rbp + 0x110], 0
14076e8d1 mov qword ptr [rbp + 0xe8], rax
14076e8d8 cmp rax, 1
14076e8dc je 0x14076f3f8
14076e8e2 test rax, rax
14076e8e5 je 0x14076f6ba
14076e8eb mov rsi, qword ptr [rbp + 0xd0]
14076e8f2 test rsi, rsi
14076e8f5 je 0x14076e95e
14076e8f7 lea rbx, [rsi*8]
14076e8ff movzx eax, byte ptr [rip + 0x2ec6303]
14076e906 mov edx, 4
14076e90b mov rcx, rbx
14076e90e call 0x140613c10
14076e913 test rax, rax
14076e916 je 0x14076fa7c
14076e91c mov ecx, 1
14076e921 mov qword ptr [rbp + 0x178], rcx
14076e928 mov rcx, rax
14076e92b cmp rsi, 1
14076e92f mov rbx, rax
14076e932 je 0x14076e955
14076e934 lea r8, [rsi*8 - 8]
14076e93c mov rcx, rbx
14076e93f xor edx, edx
14076e941 call 0x142923c20
14076e946 lea rcx, [rbx + rsi*8]
14076e94a add rcx, -8
14076e94e mov qword ptr [rbp + 0x178], rsi
14076e955 mov qword ptr [rcx], 0
14076e95c jmp 0x14076e96e
14076e95e mov ebx, 4
14076e963 mov qword ptr [rbp + 0x178], 0
14076e96e mov rax, qword ptr [r15 + 0x10]
14076e972 mov rcx, rax
14076e975 mov qword ptr [rbp + 0xe8], rax
14076e97c sub rcx, 1
14076e980 mov qword ptr [rbp + 0x110], rcx
14076e987 mov qword ptr [rbp + 0xb0], r13
14076e98e mov qword ptr [rbp + 0xb8], rbx
14076e995 jb 0x14076f7cc
14076e99b je 0x14076fa6e
14076e9a1 cmp rcx, rsi
14076e9a4 jae 0x14076fa5d
14076e9aa add eax, -2
14076e9ad mov dword ptr [rbx + rcx*8], eax
14076e9b0 mov dword ptr [rbx + rcx*8 + 4], 0x3f800000
14076e9b8 mov r8, qword ptr [r15 + 8]
14076e9bc mov r9, qword ptr [r15 + 0x10]
14076e9c0 movups xmm0, xmmword ptr [r15 + 0x20]
14076e9c5 mov rdx, rbx
14076e9c8 mov rbx, qword ptr [rbp + 0xc8]
14076e9cf mov rcx, qword ptr [rbp + 0x228]
14076e9d6 mov rax, qword ptr [rbp + 0x138]
14076e9dd mov dword ptr [rsp + 0x68], eax
14076e9e1 mov qword ptr [rsp + 0x60], rsi
14076e9e6 mov qword ptr [rsp + 0x58], rdx
14076e9eb mov qword ptr [rsp + 0x50], rdi
14076e9f0 mov qword ptr [rsp + 0x48], r13
14076e9f5 lea rax, [rbp - 0x10]
14076e9f9 mov qword ptr [rsp + 0x40], rax
14076e9fe mov qword ptr [rsp + 0x38], rsi
14076ea03 mov qword ptr [rsp + 0x30], rbx
14076ea08 movups xmmword ptr [rsp + 0x20], xmm0
14076ea0d mov rdx, qword ptr [rbp + 0x1e8]
14076ea14 call 0x141242530
14076ea19 mov rdx, qword ptr [rbp + 0x178]
14076ea20 test rdx, rdx
14076ea23 je 0x14076ea3b
14076ea25 shl rdx, 3
14076ea29 mov r8d, 4
14076ea2f mov rcx, qword ptr [rbp + 0xb8]
14076ea36 call 0x140613c20
14076ea3b mov r8d, 1
14076ea41 mov rcx, qword ptr [rbp + 0xb0]
14076ea48 mov rdx, qword ptr [rbp + 0x230]
14076ea4f call 0x140613c20
14076ea54 mov rax, qword ptr [rbp + 0xc0]
14076ea5b test rax, rax
14076ea5e je 0x14076ea76
14076ea60 shl rax, 2
14076ea64 lea rdx, [rax + rax*2]
14076ea68 mov r8d, 4
14076ea6e mov rcx, rbx
14076ea71 call 0x140613c20
14076ea76 mov r15, qword ptr [rbp + 0x88]
14076ea7d mov eax, dword ptr [rbp + 0x10c]
14076ea83 mov r8d, eax
14076ea86 cmp r15, r14
14076ea89 jne 0x14076e5e7
14076ea8f mov r8, qword ptr [rbp + 0x248]
14076ea96 lea rbx, [r8 + 0x30]
14076ea9a lea r15, [r8 + 0x60]
14076ea9e lea r13, [r8 + 0x48]
14076eaa2 pcmpeqd xmm6, xmm6
14076eaa6 cmp r12, r14
14076eaa9 jne 0x14076eb03
14076eaab mov rdx, qword ptr [rbp + 0x90]
14076eab2 mov r12, qword ptr [rbp + 0x120]
14076eab9 mov r13, qword ptr [rbp + 0x128]
14076eac0 mov rax, qword ptr [rbp + 0x98]
14076eac7 cmp rdx, rax
14076eaca jne 0x14076e2c4
14076ead0 jmp 0x14076f0bc
14076ead5 nop word ptr cs:[rax + rax]
14076eae0 mov r8, qword ptr [rbp + 0x248]
14076eae7 mov rax, qword ptr [r8 + 0x68]
14076eaeb mov qword ptr [rax + rdi*8], rsi
14076eaef inc rdi
14076eaf2 mov qword ptr [r8 + 0x70], rdi
14076eaf6 add r12, 0x68
14076eafa pcmpeqd xmm6, xmm6
14076eafe cmp r12, r14
14076eb01 je 0x14076eaab
14076eb03 mov rax, qword ptr [r12 + 0x28]
14076eb08 test rax, rax
14076eb0b je 0x14076f9e0
14076eb11 cmp qword ptr [r12 + 0x10], 0
14076eb17 je 0x14076f9c4
14076eb1d mov rcx, qword ptr [r12 + 8]
14076eb22 movss xmm7, dword ptr [rcx]
14076eb26 addss xmm7, dword ptr [rip + 0x230fc42]
14076eb2e cmp rax, 1
14076eb32 je 0x14076f9a2
14076eb38 cmp qword ptr [r12 + 0x40], 0
14076eb3e je 0x14076f986
14076eb44 mov rsi, qword ptr [r12 + 0x48]
14076eb49 mov rax, qword ptr [r12 + 0x20]
14076eb4e movss xmm0, dword ptr [rax + 0xc]
14076eb53 subss xmm0, dword ptr [rax]
14076eb57 mov rcx, qword ptr [r12 + 0x38]
14076eb5c movaps xmm1, xmm0
14076eb5f mulss xmm1, xmm0
14076eb63 movss xmm10, dword ptr [rax + 0x14]
14076eb69 subss xmm10, dword ptr [rax + 8]
14076eb6f movaps xmm2, xmm10
14076eb73 mulss xmm2, xmm10
14076eb78 addss xmm2, xmm1
14076eb7c xorps xmm1, xmm1
14076eb7f sqrtss xmm1, xmm2
14076eb83 movss xmm9, dword ptr [rip + 0x21b6d78]
14076eb8c divss xmm9, xmm1
14076eb91 mulss xmm10, xmm9
14076eb96 movss xmm8, dword ptr [rcx]
14076eb9b mov ecx, dword ptr [rax + 8]
14076eb9e mov dword ptr [rbp + 0x188], ecx
14076eba4 mov rax, qword ptr [rax]
14076eba7 mov qword ptr [rbp + 0x180], rax
14076ebae mov rax, qword ptr [r8 + 0x70]
14076ebb2 test rax, rax
14076ebb5 sete cl
14076ebb8 shl rax, 3
14076ebbc add rax, qword ptr [r8 + 0x68]
14076ebc0 mulss xmm9, xmm0
14076ebc5 add rax, -8
14076ebc9 sete dl
14076ebcc or dl, cl
14076ebce jne 0x14076ec90
14076ebd4 cmp qword ptr [rax], rsi
14076ebd7 jne 0x14076ec90
14076ebdd mov rax, qword ptr [r8 + 0x58]
14076ebe1 test rax, rax
14076ebe4 je 0x14076faae
14076ebea mov rcx, qword ptr [r8 + 0x50]
14076ebee lea rax, [rcx + rax*4]
14076ebf2 add rax, -4
14076ebf6 test rax, rax
14076ebf9 je 0x14076faae
14076ebff movss xmm0, dword ptr [rax]
14076ec03 movaps xmm1, xmm7
14076ec06 minss xmm1, xmm0
14076ec0a cmpunordss xmm0, xmm0
14076ec0f movaps xmm2, xmm0
14076ec12 andps xmm2, xmm7
14076ec15 andnps xmm0, xmm1
14076ec18 orps xmm0, xmm2
14076ec1b movss dword ptr [rax], xmm0
14076ec1f mov rcx, qword ptr [r8 + 0x40]
14076ec23 test rcx, rcx
14076ec26 je 0x14076fa99
14076ec2c mov rax, qword ptr [r8 + 0x38]
14076ec30 lea rdx, [rcx + rcx*8]
14076ec34 lea rdx, [rdx + rdx*2]
14076ec38 add rdx, rcx
14076ec3b lea rcx, [rax + rdx]
14076ec3f add rcx, -0x1c
14076ec43 test rcx, rcx
14076ec46 je 0x14076fa99
14076ec4c add rax, rdx
14076ec4f ucomiss xmm7, dword ptr [rax - 8]
14076ec53 jbe 0x14076ed6a
14076ec59 movss dword ptr [rax - 0x1c], xmm9
14076ec5f movss dword ptr [rax - 0x18], xmm10
14076ec65 mov rcx, qword ptr [rbp + 0x180]
14076ec6c mov qword ptr [rax - 0x14], rcx
14076ec70 mov ecx, dword ptr [rbp + 0x188]
14076ec76 mov dword ptr [rax - 0xc], ecx
14076ec79 movss dword ptr [rax - 8], xmm7
14076ec7e movss dword ptr [rax - 4], xmm8
14076ec84 jmp 0x14076ed6a
14076ec89 nop dword ptr [rax]
14076ec90 mov rdi, qword ptr [r8 + 0x58]
14076ec94 cmp rdi, qword ptr [r8 + 0x48]
14076ec98 jne 0x14076ecb0
14076ec9a mov byte ptr [rbp + 0x257], 1
14076eca1 mov rcx, r13
14076eca4 lea rdx, [rip + 0x230d7cd]
14076ecab call 0x140650050
14076ecb0 mov rcx, qword ptr [rbp + 0x248]
14076ecb7 mov rax, qword ptr [rcx + 0x50]
14076ecbb movss dword ptr [rax + rdi*4], xmm7
14076ecc0 inc rdi
14076ecc3 mov qword ptr [rcx + 0x58], rdi
14076ecc7 mov rdi, qword ptr [rcx + 0x40]
14076eccb cmp rdi, qword ptr [rcx + 0x30]
14076eccf jne 0x14076ece7
14076ecd1 mov byte ptr [rbp + 0x257], 1
14076ecd8 mov rcx, rbx
14076ecdb lea rdx, [rip + 0x230d7ae]
14076ece2 call 0x14078cb80
14076ece7 mov r8, qword ptr [rbp + 0x248]
14076ecee mov rax, qword ptr [r8 + 0x38]
14076ecf2 lea rcx, [rdi + rdi*8]
14076ecf6 lea rcx, [rcx + rcx*2]
14076ecfa add rcx, rdi
14076ecfd movss dword ptr [rax + rcx], xmm9
14076ed03 movss dword ptr [rax + rcx + 4], xmm10
14076ed0a mov rdx, qword ptr [rbp + 0x180]
14076ed11 mov qword ptr [rax + rcx + 8], rdx
14076ed16 mov edx, dword ptr [rbp + 0x188]
14076ed1c mov dword ptr [rax + rcx + 0x10], edx
14076ed20 movss dword ptr [rax + rcx + 0x14], xmm7
14076ed26 movss dword ptr [rax + rcx + 0x18], xmm8
14076ed2d inc rdi
14076ed30 mov qword ptr [r8 + 0x40], rdi
14076ed34 mov rdi, qword ptr [r8 + 0x70]
14076ed38 cmp rdi, qword ptr [r8 + 0x60]
14076ed3c jne 0x14076ed54
14076ed3e mov byte ptr [rbp + 0x257], 1
14076ed45 mov rcx, r15
14076ed48 lea rdx, [rip + 0x230d759]
14076ed4f call 0x140b81df0
14076ed54 mov rcx, qword ptr [rbp + 0x248]
14076ed5b mov rax, qword ptr [rcx + 0x68]
14076ed5f mov qword ptr [rax + rdi*8], rsi
14076ed63 inc rdi
14076ed66 mov qword ptr [rcx + 0x70], rdi
14076ed6a mov rsi, qword ptr [r12 + 0x50]
14076ed6f mov byte ptr [rbp + 0x257], 1
14076ed76 mov rcx, r12
14076ed79 call 0x141234970
14076ed7e dec rax
14076ed81 mov rdx, qword ptr [r12 + 0x28]
14076ed86 cmp rax, rdx
14076ed89 jae 0x14076f905
14076ed8f mov rcx, qword ptr [r12 + 0x20]
14076ed94 lea rax, [rax + rax*2]
14076ed98 mov edx, dword ptr [rcx + rax*4 + 8]
14076ed9c mov dword ptr [rbp + 0x188], edx
14076eda2 mov rax, qword ptr [rcx + rax*4]
14076eda6 mov qword ptr [rbp + 0x180], rax
14076edad mov byte ptr [rbp + 0x257], 1
14076edb4 mov rcx, r12
14076edb7 call 0x141234970
14076edbc lea rcx, [rax - 1]
14076edc0 mov rdx, qword ptr [r12 + 0x10]
14076edc5 cmp rcx, rdx
14076edc8 jae 0x14076f920
14076edce mov rcx, qword ptr [r12 + 8]
14076edd3 movss xmm10, dword ptr [rcx + rax*4 - 4]
14076edda addss xmm10, dword ptr [rip + 0x230f98d]
14076ede3 mov byte ptr [rbp + 0x257], 1
14076edea mov rcx, r12
14076eded call 0x141234970
14076edf2 dec rax
14076edf5 mov rdx, qword ptr [r12 + 0x28]
14076edfa cmp rax, rdx
14076edfd jae 0x14076f938
14076ee03 mov rcx, qword ptr [r12 + 0x20]
14076ee08 lea rax, [rax + rax*2]
14076ee0c movss xmm7, dword ptr [rcx + rax*4]
14076ee11 movss xmm6, dword ptr [rcx + rax*4 + 8]
14076ee17 mov byte ptr [rbp + 0x257], 1
14076ee1e mov rcx, r12
14076ee21 call 0x141234970
14076ee26 add rax, -2
14076ee2a mov rdx, qword ptr [r12 + 0x28]
14076ee2f cmp rax, rdx
14076ee32 jae 0x14076f953
14076ee38 mov rcx, qword ptr [r12 + 0x20]
14076ee3d lea rax, [rax + rax*2]
14076ee41 subss xmm7, dword ptr [rcx + rax*4]
14076ee46 subss xmm6, dword ptr [rcx + rax*4 + 8]
14076ee4c mov byte ptr [rbp + 0x257], 1
14076ee53 mov rcx, r12
14076ee56 call 0x141234970
14076ee5b lea rcx, [rax - 1]
14076ee5f mov rdx, qword ptr [r12 + 0x40]
14076ee64 cmp rcx, rdx
14076ee67 jae 0x14076f96e
14076ee6d dec rsi
14076ee70 movaps xmm0, xmm7
14076ee73 mulss xmm0, xmm7
14076ee77 movaps xmm1, xmm6
14076ee7a mulss xmm1, xmm6
14076ee7e addss xmm1, xmm0
14076ee82 xorps xmm0, xmm0
14076ee85 sqrtss xmm0, xmm1
14076ee89 movss xmm9, dword ptr [rip + 0x21b6a72]
14076ee92 divss xmm9, xmm0
14076ee97 mulss xmm7, xmm9
14076ee9c mov rcx, qword ptr [r12 + 0x38]
14076eea1 movss xmm8, dword ptr [rcx + rax*4 - 4]
14076eea8 mov eax, dword ptr [rbp + 0x188]
14076eeae mov dword ptr [rbp + 0x1f8], eax
14076eeb4 mov rax, qword ptr [rbp + 0x180]
14076eebb mov qword ptr [rbp + 0x1f0], rax
14076eec2 mov r8, qword ptr [rbp + 0x248]
14076eec9 mov rax, qword ptr [r8 + 0x70]
14076eecd test rax, rax
14076eed0 sete cl
14076eed3 shl rax, 3
14076eed7 add rax, qword ptr [r8 + 0x68]
14076eedb mulss xmm9, xmm6
14076eee0 add rax, -8
14076eee4 sete dl
14076eee7 or dl, cl
14076eee9 jne 0x14076efc0
14076eeef cmp qword ptr [rax], rsi
14076eef2 jne 0x14076efc0
14076eef8 mov rax, qword ptr [r8 + 0x58]
14076eefc test rax, rax
14076eeff je 0x14076fad8
14076ef05 mov rcx, qword ptr [r8 + 0x50]
14076ef09 lea rax, [rcx + rax*4]
14076ef0d add rax, -4
14076ef11 test rax, rax
14076ef14 je 0x14076fad8
14076ef1a movss xmm0, dword ptr [rax]
14076ef1e movaps xmm1, xmm10
14076ef22 minss xmm1, xmm0
14076ef26 cmpunordss xmm0, xmm0
14076ef2b movaps xmm2, xmm0
14076ef2e andps xmm2, xmm10
14076ef32 andnps xmm0, xmm1
14076ef35 orps xmm0, xmm2
14076ef38 movss dword ptr [rax], xmm0
14076ef3c mov rcx, qword ptr [r8 + 0x40]
14076ef40 test rcx, rcx
14076ef43 je 0x14076fac3
14076ef49 mov rax, qword ptr [r8 + 0x38]
14076ef4d lea rdx, [rcx + rcx*8]
14076ef51 lea rdx, [rdx + rdx*2]
14076ef55 add rdx, rcx
14076ef58 lea rcx, [rax + rdx]
14076ef5c add rcx, -0x1c
14076ef60 test rcx, rcx
14076ef63 je 0x14076fac3
14076ef69 add rax, rdx
14076ef6c ucomiss xmm10, dword ptr [rax - 8]
14076ef71 jbe 0x14076ef9e
14076ef73 movss dword ptr [rax - 0x1c], xmm7
14076ef78 movss dword ptr [rax - 0x18], xmm9
14076ef7e mov rcx, qword ptr [rbp + 0x1f0]
14076ef85 mov qword ptr [rax - 0x14], rcx
14076ef89 mov ecx, dword ptr [rbp + 0x1f8]
14076ef8f mov dword ptr [rax - 0xc], ecx
14076ef92 movss dword ptr [rax - 8], xmm10
14076ef98 movss dword ptr [rax - 4], xmm8
14076ef9e add r12, 0x68
14076efa2 pcmpeqd xmm6, xmm6
14076efa6 cmp r12, r14
14076efa9 jne 0x14076eb03
14076efaf jmp 0x14076eaab
14076efb4 nop word ptr cs:[rax + rax]
14076efc0 mov rdi, qword ptr [r8 + 0x58]
14076efc4 cmp rdi, qword ptr [r8 + 0x48]
14076efc8 jne 0x14076efe0
14076efca mov byte ptr [rbp + 0x257], 1
14076efd1 mov rcx, r13
14076efd4 lea rdx, [rip + 0x230d49d]
14076efdb call 0x140650050
14076efe0 mov rcx, qword ptr [rbp + 0x248]
14076efe7 mov rax, qword ptr [rcx + 0x50]
14076efeb movss dword ptr [rax + rdi*4], xmm10
14076eff1 inc rdi
14076eff4 mov qword ptr [rcx + 0x58], rdi
14076eff8 mov rdi, qword ptr [rcx + 0x40]
14076effc cmp rdi, qword ptr [rcx + 0x30]
14076f000 jne 0x14076f018
14076f002 mov byte ptr [rbp + 0x257], 1
14076f009 mov rcx, rbx
14076f00c lea rdx, [rip + 0x230d47d]
14076f013 call 0x14078cb80
14076f018 mov r8, qword ptr [rbp + 0x248]
14076f01f mov rax, qword ptr [r8 + 0x38]
14076f023 lea rcx, [rdi + rdi*8]
14076f027 lea rcx, [rcx + rcx*2]
14076f02b add rcx, rdi
14076f02e movss dword ptr [rax + rcx], xmm7
14076f033 movss dword ptr [rax + rcx + 4], xmm9
14076f03a mov rdx, qword ptr [rbp + 0x1f0]
14076f041 mov qword ptr [rax + rcx + 8], rdx
14076f046 mov edx, dword ptr [rbp + 0x1f8]
14076f04c mov dword ptr [rax + rcx + 0x10], edx
14076f050 movss dword ptr [rax + rcx + 0x14], xmm10
14076f057 movss dword ptr [rax + rcx + 0x18], xmm8
14076f05e inc rdi
14076f061 mov qword ptr [r8 + 0x40], rdi
14076f065 mov rdi, qword ptr [r8 + 0x70]
14076f069 cmp rdi, qword ptr [r8 + 0x60]
14076f06d jne 0x14076eae0
14076f073 mov byte ptr [rbp + 0x257], 1
14076f07a mov rcx, r15
14076f07d lea rdx, [rip + 0x230d424]
14076f084 call 0x140b81df0
14076f089 jmp 0x14076eae0
14076f08e mov byte ptr [rbp + 0x254], 1
14076f095 mov qword ptr [rsp + 0x20], 0x68
14076f09e mov r9d, 8
14076f0a4 lea rcx, [rbp + 0x140]
14076f0ab call 0x1428a7de0
14076f0b0 mov rsi, qword ptr [rbp + 0x150]
14076f0b7 jmp 0x14076e40f
14076f0bc mov r15, qword ptr [rbp + 0x118]
14076f0c3 nop word ptr cs:[rax + rax]
14076f0d0 mov rax, qword ptr [rbp + 0x58]
14076f0d4 mov ecx, dword ptr [rbp + 0x100]
14076f0da mov dword ptr [rax], ecx
14076f0dc cmp qword ptr [r15 + 0x18], 0
14076f0e1 je 0x14076f410
14076f0e7 mov rsi, r15
14076f0ea mov rcx, qword ptr [rbp + 0xa0]
14076f0f1 lea rdx, [rbp + 0x228]
14076f0f8 call 0x1405b0d70
14076f0fd mov r15, qword ptr [r15]
14076f100 mov qword ptr [rbp + 0x118], rsi
14076f107 mov rcx, qword ptr [rsi + 8]
14076f10b mov rdx, rcx
14076f10e and rdx, rax
14076f111 shr rax, 0x39
14076f115 movd xmm0, eax
14076f119 punpcklbw xmm0, xmm0
14076f11d pshuflw xmm0, xmm0, 0
14076f122 pshufd xmm0, xmm0, 0
14076f127 lea rax, [r15 - 0x2a8]
14076f12e mov r11, qword ptr [rbp + 0x228]
14076f135 xor r8d, r8d
14076f138 movdqu xmm1, xmmword ptr [r15 + rdx]
14076f13e movdqa xmm2, xmm1
14076f142 pcmpeqb xmm2, xmm0
14076f146 pmovmskb r9d, xmm2
14076f14b test r9d, r9d
14076f14e je 0x14076f178
14076f150 tzcnt r10d, r9d
14076f155 add r10, rdx
14076f158 and r10, rcx
14076f15b neg r10
14076f15e imul r10, r10, 0x2a8
14076f165 cmp r11, qword ptr [rax + r10]
14076f169 je 0x14076f19a
14076f16b lea r10d, [r9 - 1]
14076f16f and r10w, r9w
14076f173 mov r9d, r10d
14076f176 jne 0x14076f150
14076f178 pcmpeqb xmm1, xmm6
14076f17c pmovmskb r9d, xmm1
14076f181 test r9d, r9d
14076f184 jne 0x14076f410
14076f18a add rdx, r8
14076f18d add rdx, 0x10
14076f191 add r8, 0x10
14076f195 and rdx, rcx
14076f198 jmp 0x14076f138
14076f19a mov qword ptr [rbp + 0x230], r11
14076f1a1 mov qword ptr [rbp + 0x128], r13
14076f1a8 mov qword ptr [rbp + 0x120], r12
14076f1af add r15, r10
14076f1b2 mov rdi, qword ptr [rbp + 0x148]
14076f1b9 mov rax, qword ptr [rbp + 0x150]
14076f1c0 mov qword ptr [rbp + 0x1e8], rax
14076f1c7 imul rax, rax, 0x68
14076f1cb add rax, rdi
14076f1ce mov qword ptr [rbp + 0x240], rax
14076f1d5 jmp 0x14076f1e7
14076f1d7 nop word ptr [rax + rax]
14076f1e0 mov rdi, qword ptr [rbp + 0x248]
14076f1e7 cmp rdi, qword ptr [rbp + 0x240]
14076f1ee je 0x14076f2e7
14076f1f4 lea rax, [rdi + 0x68]
14076f1f8 mov qword ptr [rbp + 0x248], rax
14076f1ff lea rbx, [rdi + 0x58]
14076f203 mov eax, dword ptr [rdi + 0x60]
14076f206 mov dword ptr [rbp + 0x1f8], eax
14076f20c mov rax, qword ptr [rdi + 0x58]
14076f210 mov qword ptr [rbp + 0x1f0], rax
14076f217 nop word ptr [rax + rax]
14076f220 mov byte ptr [rbp + 0x257], 1
14076f227 lea rcx, [rbp + 0x180]
14076f22e lea rdx, [rbp + 0x1f0]
14076f235 call 0x1412002f0
14076f23a test byte ptr [rbp + 0x180], 1
14076f241 je 0x14076f1e0
14076f243 mov r14d, dword ptr [rbp + 0x184]
14076f24a mov esi, dword ptr [rbp + 0x188]
14076f250 mov byte ptr [rbp + 0x257], 1
14076f257 mov ecx, r14d
14076f25a mov edx, esi
14076f25c call 0x141200280
14076f261 mov r12, rax
14076f264 mov rdx, qword ptr [r15 - 0x1a0]
14076f26b cmp rax, rdx
14076f26e jae 0x14076f762
14076f274 mov r13, qword ptr [r15 - 0x1a8]
14076f27b movss xmm6, dword ptr [r13 + r12*4]
14076f282 mov byte ptr [rbp + 0x257], 1
14076f289 mov rcx, rbx
14076f28c call 0x141200370
14076f291 mov r9d, edx
14076f294 mov byte ptr [rbp + 0x257], 1
14076f29b mov ecx, r14d
14076f29e mov edx, esi
14076f2a0 mov r8d, eax
14076f2a3 call 0x1412002e0
14076f2a8 mov rdx, qword ptr [rdi + 0x10]
14076f2ac cmp rax, rdx
14076f2af jae 0x14076f747
14076f2b5 mov rcx, qword ptr [rdi + 8]
14076f2b9 movss xmm0, dword ptr [rcx + rax*4]
14076f2be addss xmm0, xmm15
14076f2c3 movaps xmm1, xmm6
14076f2c6 cmpunordss xmm1, xmm6
14076f2cb movaps xmm2, xmm1
14076f2ce andps xmm2, xmm0
14076f2d1 maxss xmm0, xmm6
14076f2d5 andnps xmm1, xmm0
14076f2d8 orps xmm1, xmm2
14076f2db movss dword ptr [r13 + r12*4], xmm1
14076f2e2 jmp 0x14076f220
14076f2e7 add r15, -0x118
14076f2ee mov byte ptr [rbp + 0x257], 1
14076f2f5 mov rcx, r15
14076f2f8 mov rdx, qword ptr [rbp + 0x1e8]
14076f2ff call 0x140a885e0
14076f304 mov byte ptr [rbp + 0x257], 1
14076f30b mov rcx, r15
14076f30e movd xmm1, dword ptr [rbp + 0x23c]
14076f316 call 0x140a88450
14076f31b mov eax, dword ptr [rbp + 0x108]
14076f321 mov rcx, qword ptr [rbp + 0x80]
14076f328 mov dword ptr [rcx], eax
14076f32a mov rax, qword ptr [rbp + 0x150]
14076f331 mov qword ptr [rbp + 0x190], rax
14076f338 movdqu xmm0, xmmword ptr [rbp + 0x140]
14076f340 movdqa xmmword ptr [rbp + 0x180], xmm0
14076f348 mov byte ptr [rbp + 0x257], 0
14076f34f lea rcx, [rbp + 0x1f0]
14076f356 mov rdx, qword ptr [rbp + 0x78]
14076f35a mov r8, qword ptr [rbp + 0x230]
14076f361 lea r9, [rbp + 0x180]
14076f368 call 0x1411fbd70
14076f36d xor eax, eax
14076f36f cmp rax, qword ptr [rbp + 0x1f0]
14076f376 jo 0x14076f38b
14076f378 mov byte ptr [rbp + 0x257], 0
14076f37f lea rcx, [rbp + 0x1f0]
14076f386 call 0x1405d3750
14076f38b mov rsi, qword ptr [rbp + 0x20]
14076f38f add rsi, 0x18
14076f393 mov rcx, rsi
14076f396 call 0x140c81e10
14076f39b mov r12, qword ptr [rbp + 0x120]
14076f3a2 mov r13, qword ptr [rbp + 0x128]
14076f3a9 mov r15, qword ptr [rbp + 0x118]
14076f3b0 pcmpeqd xmm6, xmm6
14076f3b4 mov al, 1
14076f3b6 xor ecx, ecx
14076f3b8 lock cmpxchg byte ptr [rsi], cl
14076f3bc je 0x14076dfd3
14076f3c2 mov rcx, rsi
14076f3c5 xor edx, edx
14076f3c7 call 0x1428cdb90
14076f3cc jmp 0x14076dfd3
14076f3d1 mov rsi, rcx
14076f3d4 mov r8d, 0x3b9aca00
14076f3da call 0x1428cd210
14076f3df mov rcx, rsi
14076f3e2 jmp 0x14076e0d5
14076f3e7 lea rcx, [rip + 0x230f68a]
14076f3ee call 0x1428d9310
14076f3f3 jmp 0x14076faeb
14076f3f8 mov qword ptr [rbp + 0x220], r13
14076f3ff lea rcx, [rip + 0x24a9f42]
14076f406 call 0x1428d9310
14076f40b jmp 0x14076faeb
14076f410 mov byte ptr [rbp + 0x257], 1
14076f417 lea rcx, [rip + 0x230f7f2]
14076f41e call 0x1428d9310
14076f423 jmp 0x14076faeb
14076f428 mov byte ptr [rbp + 0x257], 1
14076f42f lea rcx, [rip + 0x230f65a]
14076f436 call 0x1428d9310
14076f43b jmp 0x14076faeb
14076f440 cmp byte ptr [rbp + 0x255], 0
14076f447 je 0x14076f4cf
14076f44d mov rax, qword ptr [rip + 0x2ebcf54]
14076f454 mov rax, qword ptr [rax]
14076f457 test rax, rax
14076f45a je 0x14076f4cf
14076f45c lea rax, [rip + 0x230f5bd]
14076f463 mov qword ptr [rbp + 0x180], rax
14076f46a mov qword ptr [rbp + 0x188], 1
14076f475 mov qword ptr [rbp + 0x190], 8
14076f480 pxor xmm0, xmm0
14076f484 movdqu xmmword ptr [rbp + 0x198], xmm0
14076f48c lea rcx, [rip + 0x230f59d]
14076f493 call 0x14067dd80
14076f498 lea rcx, [rip + 0x230f5a9]
14076f49f mov qword ptr [rbp], rcx
14076f4a3 mov qword ptr [rbp + 8], 0x2a
14076f4ab mov qword ptr [rbp + 0x10], rcx
14076f4af mov qword ptr [rbp + 0x18], 0x2a
14076f4b7 mov qword ptr [rbp + 0x20], rax
14076f4bb lea rcx, [rbp + 0x180]
14076f4c2 mov r8, rbp
14076f4c5 mov edx, 1
14076f4ca call 0x1404b1430
14076f4cf mov rdx, qword ptr [rbp + 0x158]
14076f4d6 test rdx, rdx
14076f4d9 je 0x14076f4f1
14076f4db mov rcx, qword ptr [rbp + 0x160]
14076f4e2 shl rdx, 3
14076f4e6 mov r8d, 8
14076f4ec call 0x140613c20
14076f4f1 cmp byte ptr [rbp + 0x256], 0
14076f4f8 je 0x14076f545
14076f4fa call 0x140c18130
14076f4ff mov rcx, rax
14076f502 mov rax, qword ptr [rax]
14076f505 cmp rax, 1
14076f509 jne 0x14076f5c1
14076f50f add rcx, 8
14076f513 mov rdx, qword ptr [rbp + 0x170]
14076f51a cmp qword ptr [rcx], 0
14076f51e jne 0x14076f6ae
14076f524 mov qword ptr [rcx], 0xffffffffffffffff
14076f52b mov qword ptr [rbp + 0x248], rcx
14076f532 add rcx, 8
14076f536 call 0x140c170f0
14076f53b mov rax, qword ptr [rbp + 0x248]
14076f542 inc qword ptr [rax]
14076f545 movaps xmm6, xmmword ptr [rbp + 0x260]
14076f54c movaps xmm7, xmmword ptr [rbp + 0x270]
14076f553 movaps xmm8, xmmword ptr [rbp + 0x280]
14076f55b movaps xmm9, xmmword ptr [rbp + 0x290]
14076f563 movaps xmm10, xmmword ptr [rbp + 0x2a0]
14076f56b movaps xmm11, xmmword ptr [rbp + 0x2b0]
14076f573 movaps xmm12, xmmword ptr [rbp + 0x2c0]
14076f57b movaps xmm13, xmmword ptr [rbp + 0x2d0]
14076f583 movaps xmm14, xmmword ptr [rbp + 0x2e0]
14076f58b movaps xmm15, xmmword ptr [rbp + 0x2f0]
14076f593 add rsp, 0x388
14076f59a pop rbx
14076f59b pop rdi
14076f59c pop rsi
14076f59d pop r12
14076f59f pop r13
14076f5a1 pop r14
14076f5a3 pop r15
14076f5a5 pop rbp
14076f5a6 ret 
14076f5a7 test rax, rax
14076f5aa jne 0x14076f5e0
14076f5ac xor edx, edx
14076f5ae call 0x1428f15c0
14076f5b3 mov rcx, rax
14076f5b6 test rax, rax
14076f5b9 jne 0x14076dc4c
14076f5bf jmp 0x14076f5e0
14076f5c1 test rax, rax
14076f5c4 jne 0x14076f5e0
14076f5c6 xor edx, edx
14076f5c8 call 0x1428f15c0
14076f5cd mov rdx, qword ptr [rbp + 0x170]
14076f5d4 mov rcx, rax
14076f5d7 test rax, rax
14076f5da jne 0x14076f51a
14076f5e0 lea rcx, [rip + 0x24a9ec1]
14076f5e7 call 0x1428d80d0
14076f5ec mov qword ptr [rbp + 0x180], 0
14076f5f7 lea rax, [rip + 0x24ab262]
14076f5fe mov qword ptr [rsp + 0x20], rax
14076f603 mov rdx, rbp
14076f606 lea r9, [rbp + 0x180]
14076f60d xor ecx, ecx
14076f60f mov r8, r14
14076f612 call 0x1428cab41
14076f617 jmp 0x14076faeb
14076f61c cmp rsi, 0x15
14076f620 jae 0x14076fa46
14076f626 lea rax, [r12 + rsi*8]
14076f62a lea rcx, [r12 + 8]
14076f62f mov edx, 8
14076f634 jmp 0x14076f649
14076f636 mov r9, r12
14076f639 mov qword ptr [r9], r8
14076f63c add rcx, 8
14076f640 add rdx, 8
14076f644 cmp rcx, rax
14076f647 je 0x14076f675
14076f649 mov r10, qword ptr [rcx - 8]
14076f64d mov r8, qword ptr [rcx]
14076f650 cmp r8, r10
14076f653 jae 0x14076f63c
14076f655 mov r9, rdx
14076f658 mov qword ptr [r12 + r9], r10
14076f65c cmp r9, 8
14076f660 je 0x14076f636
14076f662 mov r10, qword ptr [r12 + r9 - 0x10]
14076f667 add r9, -8
14076f66b cmp r8, r10
14076f66e jb 0x14076f658
14076f670 add r9, r12
14076f673 jmp 0x14076f639
14076f675 mov r12, qword ptr [rbp + 0x160]
14076f67c mov rsi, qword ptr [rbp + 0x168]
14076f683 cmp rsi, 2
14076f687 jb 0x14076df3a
14076f68d mov eax, 1
14076f692 mov rcx, qword ptr [r12 + rax*8]
14076f696 cmp rcx, qword ptr [r12 + rax*8 - 8]
14076f69b je 0x14076f77d
14076f6a1 inc rax
14076f6a4 cmp rsi, rax
14076f6a7 jne 0x14076f692
14076f6a9 jmp 0x14076df3a
14076f6ae lea rcx, [rip + 0x24a9e63]
14076f6b5 call 0x1428d9250
14076f6ba lea rax, [rbp + 0x110]
14076f6c1 mov qword ptr [rbp + 0x1f0], rax
14076f6c8 lea rax, [rip + 0x61dff1]
14076f6cf mov qword ptr [rbp + 0x1f8], rax
14076f6d6 lea rcx, [rbp + 0xe8]
14076f6dd mov qword ptr [rbp + 0x200], rcx
14076f6e4 mov qword ptr [rbp + 0x208], rax
14076f6eb lea rax, [rip + 0x24a9c06]
14076f6f2 mov qword ptr [rbp + 0x180], rax
14076f6f9 mov qword ptr [rbp + 0x188], 2
14076f704 mov qword ptr [rbp + 0x1a0], 0
14076f70f lea rax, [rbp + 0x1f0]
14076f716 mov qword ptr [rbp + 0x190], rax
14076f71d mov qword ptr [rbp + 0x198], 2
14076f728 mov qword ptr [rbp + 0x220], r13
14076f72f lea rdx, [rip + 0x24a9be2]
14076f736 lea rcx, [rbp + 0x180]
14076f73d call 0x1428d9390
14076f742 jmp 0x14076faeb
14076f747 mov byte ptr [rbp + 0x257], 1
14076f74e lea r8, [rip + 0x230f4eb]
14076f755 mov rcx, rax
14076f758 call 0x1428d9518
14076f75d jmp 0x14076faeb
14076f762 mov byte ptr [rbp + 0x257], 1
14076f769 lea r8, [rip + 0x230f4b8]
14076f770 mov rcx, r12
14076f773 call 0x1428d9518
14076f778 jmp 0x14076faeb
14076f77d lea rcx, [rax + 1]
14076f781 jmp 0x14076f786
14076f783 inc rcx
14076f786 cmp rcx, rsi
14076f789 jae 0x14076f79f
14076f78b mov rdx, qword ptr [r12 + rcx*8]
14076f78f cmp rdx, qword ptr [r12 + rax*8 - 8]
14076f794 je 0x14076f783
14076f796 mov qword ptr [r12 + rax*8], rdx
14076f79a inc rax
14076f79d jmp 0x14076f783
14076f79f mov qword ptr [rbp + 0x168], rax
14076f7a6 mov rsi, rax
14076f7a9 jmp 0x14076df3a
14076f7ae mov byte ptr [rbp + 0x257], 1
14076f7b5 lea r8, [rip + 0x230f2ec]
14076f7bc mov rcx, rdx
14076f7bf mov rdx, rsi
14076f7c2 call 0x1428d9830
14076f7c7 jmp 0x14076faeb
14076f7cc lea rax, [rbp + 0x110]
14076f7d3 mov qword ptr [rbp + 0x1f0], rax
14076f7da lea rax, [rip + 0x61dedf]
14076f7e1 mov qword ptr [rbp + 0x1f8], rax
14076f7e8 lea rcx, [rbp + 0xe8]
14076f7ef mov qword ptr [rbp + 0x200], rcx
14076f7f6 mov qword ptr [rbp + 0x208], rax
14076f7fd lea rax, [rip + 0x24a9af4]
14076f804 mov qword ptr [rbp + 0x180], rax
14076f80b mov qword ptr [rbp + 0x188], 2
14076f816 mov qword ptr [rbp + 0x1a0], 0
14076f821 lea rax, [rbp + 0x1f0]
14076f828 mov qword ptr [rbp + 0x190], rax
14076f82f mov qword ptr [rbp + 0x198], 2
14076f83a lea rdx, [rip + 0x24a9ad7]
14076f841 lea rcx, [rbp + 0x180]
14076f848 call 0x1428d9390
14076f84d jmp 0x14076faeb
14076f852 lea rcx, [rip + 0x230f17f]
14076f859 call 0x1428d9310
14076f85e jmp 0x14076faeb
14076f863 lea rcx, [rip + 0x24ab23e]
14076f86a call 0x1428d9310
14076f86f jmp 0x14076faeb
14076f874 lea r8, [rip + 0x230f31d]
14076f87b mov ecx, 1
14076f880 mov rdx, rdi
14076f883 call 0x1428d8fe3
14076f888 jmp 0x14076faeb
14076f88d mov qword ptr [rbp + 0x220], 1
14076f898 lea r8, [rip + 0x230f311]
14076f89f xor ecx, ecx
14076f8a1 xor edx, edx
14076f8a3 call 0x1428d9518
14076f8a8 jmp 0x14076faeb
14076f8ad mov byte ptr [rbp + 0x257], 1
14076f8b4 lea r8, [rip + 0x24b021d]
14076f8bb mov ecx, 4
14076f8c0 mov rdx, r12
14076f8c3 call 0x1428d8fe3
14076f8c8 jmp 0x14076faeb
14076f8cd mov byte ptr [rbp + 0x257], 1
14076f8d4 lea r8, [rip + 0x24b026d]
14076f8db xor ecx, ecx
14076f8dd xor edx, edx
14076f8df call 0x1428d9518
14076f8e4 jmp 0x14076faeb
14076f8e9 mov byte ptr [rbp + 0x257], 1
14076f8f0 lea r8, [rip + 0x24b0239]
14076f8f7 xor ecx, ecx
14076f8f9 xor edx, edx
14076f8fb call 0x1428d9518
14076f900 jmp 0x14076faeb
14076f905 mov byte ptr [rbp + 0x257], 1
14076f90c lea r8, [rip + 0x230f20d]
14076f913 mov rcx, rax
14076f916 call 0x1428d9518
14076f91b jmp 0x14076faeb
14076f920 mov byte ptr [rbp + 0x257], 1
14076f927 lea r8, [rip + 0x230f20a]
14076f92e call 0x1428d9518
14076f933 jmp 0x14076faeb
14076f938 mov byte ptr [rbp + 0x257], 1
14076f93f lea r8, [rip + 0x230f20a]
14076f946 mov rcx, rax
14076f949 call 0x1428d9518
14076f94e jmp 0x14076faeb
14076f953 mov byte ptr [rbp + 0x257], 1
14076f95a lea r8, [rip + 0x230f207]
14076f961 mov rcx, rax
14076f964 call 0x1428d9518
14076f969 jmp 0x14076faeb
14076f96e mov byte ptr [rbp + 0x257], 1
14076f975 lea r8, [rip + 0x230f204]
14076f97c call 0x1428d9518
14076f981 jmp 0x14076faeb
14076f986 mov byte ptr [rbp + 0x257], 1
14076f98d lea r8, [rip + 0x230f174]
14076f994 xor ecx, ecx
14076f996 xor edx, edx
14076f998 call 0x1428d9518
14076f99d jmp 0x14076faeb
14076f9a2 mov byte ptr [rbp + 0x257], 1
14076f9a9 lea r8, [rip + 0x230f140]
14076f9b0 mov ecx, 1
14076f9b5 mov edx, 1
14076f9ba call 0x1428d9518
14076f9bf jmp 0x14076faeb
14076f9c4 mov byte ptr [rbp + 0x257], 1
14076f9cb lea r8, [rip + 0x230f106]
14076f9d2 xor ecx, ecx
14076f9d4 xor edx, edx
14076f9d6 call 0x1428d9518
14076f9db jmp 0x14076faeb
14076f9e0 mov byte ptr [rbp + 0x257], 1
14076f9e7 lea r8, [rip + 0x230f0d2]
14076f9ee xor ecx, ecx
14076f9f0 xor edx, edx
14076f9f2 call 0x1428d9518
14076f9f7 jmp 0x14076faeb
14076f9fc mov byte ptr [rbp + 0x257], 1
14076fa03 lea r8, [rip + 0x24b0156]
14076fa0a mov rcx, r14
14076fa0d mov rdx, r14
14076fa10 call 0x1428d9518
14076fa15 jmp 0x14076faeb
14076fa1a mov qword ptr [rbp + 0x220], r13
14076fa21 lea r8, [rip + 0x230f1a0]
14076fa28 mov rdx, rdi
14076fa2b call 0x1428d9518
14076fa30 jmp 0x14076faeb
14076fa35 lea rcx, [rip + 0x24ab084]
14076fa3c call 0x1428d9310
14076fa41 jmp 0x14076faeb
14076fa46 lea r8, [rbp + 0x180]
14076fa4d mov rcx, r12
14076fa50 mov rdx, rsi
14076fa53 call 0x140958d60
14076fa58 jmp 0x14076f68d
14076fa5d lea r8, [rip + 0x230f194]
14076fa64 mov rdx, rsi
14076fa67 call 0x1428d9518
14076fa6c jmp 0x14076faeb
14076fa6e lea rcx, [rip + 0x24a98d3]
14076fa75 call 0x1428d9310
14076fa7a jmp 0x14076faeb
14076fa7c mov qword ptr [rbp + 0x220], r13
14076fa83 lea r8, [rip + 0x230f156]
14076fa8a mov ecx, 4
14076fa8f mov rdx, rbx
14076fa92 call 0x1428d8fe3
14076fa97 jmp 0x14076faeb
14076fa99 mov byte ptr [rbp + 0x257], 1
14076faa0 lea rcx, [rip + 0x230ca31]
14076faa7 call 0x1428d9310
14076faac jmp 0x14076faeb
14076faae mov byte ptr [rbp + 0x257], 1
14076fab5 lea rcx, [rip + 0x230ca04]
14076fabc call 0x1428d9310
14076fac1 jmp 0x14076faeb
14076fac3 mov byte ptr [rbp + 0x257], 1
14076faca lea rcx, [rip + 0x230ca07]
14076fad1 call 0x1428d9310
14076fad6 jmp 0x14076faeb
14076fad8 mov byte ptr [rbp + 0x257], 1
14076fadf lea rcx, [rip + 0x230c9da]
14076fae6 call 0x1428d9310
14076faeb ud2 
