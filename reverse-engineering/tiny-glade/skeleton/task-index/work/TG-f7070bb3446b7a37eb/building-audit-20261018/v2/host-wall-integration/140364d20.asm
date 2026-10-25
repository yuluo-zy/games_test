0x140364d20 55 push rbp
0x140364d21 4157 push r15
0x140364d23 4156 push r14
0x140364d25 4155 push r13
0x140364d27 4154 push r12
0x140364d29 56 push rsi
0x140364d2a 57 push rdi
0x140364d2b 53 push rbx
0x140364d2c 4881ece8000000 sub rsp, 0xe8
0x140364d33 488dac2480000000 lea rbp, [rsp + 0x80]
0x140364d3b 66440f7f4550 movdqa xmmword ptr [rbp + 0x50], xmm8
0x140364d41 660f7f7d40 movdqa xmmword ptr [rbp + 0x40], xmm7
0x140364d46 660f7f7530 movdqa xmmword ptr [rbp + 0x30], xmm6
0x140364d4b 48c74528feffffff mov qword ptr [rbp + 0x28], 0xfffffffffffffffe
0x140364d53 4c89c6 mov rsi, r8
0x140364d56 4889d7 mov rdi, rdx
0x140364d59 4889cb mov rbx, rcx
0x140364d5c 8b01 mov eax, dword ptr [rcx]
0x140364d5e 3dfdffff3f cmp eax, 0x3ffffffd
0x140364d63 0f87a7010000 ja 0x140364f10
0x140364d69 8d4801 lea ecx, [rax + 1]
0x140364d6c f00fb10b lock cmpxchg dword ptr [rbx], ecx
0x140364d70 0f859a010000 jne 0x140364f10
0x140364d76 0fb64308 movzx eax, byte ptr [rbx + 8]
0x140364d7a 48895d20 mov qword ptr [rbp + 0x20], rbx
0x140364d7e 48837b2800 cmp qword ptr [rbx + 0x28], 0
0x140364d83 0f84a2010000 je 0x140364f2b
0x140364d89 48b8f1de68ef5cfec09d movabs rax, 0x9dc0fe5cef68def1
0x140364d93 488945e0 mov qword ptr [rbp - 0x20], rax
0x140364d97 660fefc0 pxor xmm0, xmm0
0x140364d9b 660f7f45d0 movdqa xmmword ptr [rbp - 0x30], xmm0
0x140364da0 c6450800 mov byte ptr [rbp + 8], 0
0x140364da4 48b87713d038e6212845 movabs rax, 0x452821e638d01377
0x140364dae 488945e8 mov qword ptr [rbp - 0x18], rax
0x140364db2 48b86c0ce934cf6654be movabs rax, 0xbe5466cf34e90c6c
0x140364dbc 488945f0 mov qword ptr [rbp - 0x10], rax
0x140364dc0 48b8dd507cc9b729acc0 movabs rax, 0xc0ac29b7c97c50dd
0x140364dca 488945f8 mov qword ptr [rbp - 8], rax
0x140364dce 48b8170947b5b5d5843f movabs rax, 0x3f84d5b5b5470917
0x140364dd8 48894500 mov qword ptr [rbp], rax
0x140364ddc 4c8d0575ac6002 lea r8, [rip + 0x260ac75]
0x140364de3 488d55d0 lea rdx, [rbp - 0x30]
0x140364de7 4889f9 mov rcx, rdi
0x140364dea ff5640 call qword ptr [rsi + 0x40]
0x140364ded 807d0800 cmp byte ptr [rbp + 8], 0
0x140364df1 7418 je 0x140364e0b
0x140364df3 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x140364df7 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x140364dfb 48334dd0 xor rcx, qword ptr [rbp - 0x30]
0x140364dff 483345d8 xor rax, qword ptr [rbp - 0x28]
0x140364e03 48f7e1 mul rcx
0x140364e06 4831c2 xor rdx, rax
0x140364e09 eb04 jmp 0x140364e0f
0x140364e0b 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x140364e0f 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140364e13 4c8b6010 mov r12, qword ptr [rax + 0x10]
0x140364e17 4c8b7018 mov r14, qword ptr [rax + 0x18]
0x140364e1b 4d89f7 mov r15, r14
0x140364e1e 4921d7 and r15, rdx
0x140364e21 48c1ea39 shr rdx, 0x39
0x140364e25 660f6ec2 movd xmm0, edx
0x140364e29 660f60c0 punpcklbw xmm0, xmm0
0x140364e2d f20f70c000 pshuflw xmm0, xmm0, 0
0x140364e32 660f70f000 pshufd xmm6, xmm0, 0
0x140364e37 48c7451800000000 mov qword ptr [rbp + 0x18], 0
0x140364e3f 660f76ff pcmpeqd xmm7, xmm7
0x140364e43 f3470f6f043c movdqu xmm8, xmmword ptr [r12 + r15]
0x140364e49 66410f6fc0 movdqa xmm0, xmm8
0x140364e4e 660f74c6 pcmpeqb xmm0, xmm6
0x140364e52 66440fd7e8 pmovmskb r13d, xmm0
0x140364e57 4585ed test r13d, r13d
0x140364e5a 743d je 0x140364e99
0x140364e5c 0f1f4000 nop dword ptr [rax]
0x140364e60 f3410fbcc5 tzcnt eax, r13d
0x140364e65 4c01f8 add rax, r15
0x140364e68 4c21f0 and rax, r14
0x140364e6b 48c1e004 shl rax, 4
0x140364e6f 4c89e3 mov rbx, r12
0x140364e72 4829c3 sub rbx, rax
0x140364e75 4c8b43f0 mov r8, qword ptr [rbx - 0x10]
0x140364e79 4c8b4bf8 mov r9, qword ptr [rbx - 8]
0x140364e7d 4889f9 mov rcx, rdi
0x140364e80 4889f2 mov rdx, rsi
0x140364e83 e838699a00 call 0x140d0b7c0
0x140364e88 84c0 test al, al
0x140364e8a 7537 jne 0x140364ec3
0x140364e8c 418d45ff lea eax, [r13 - 1]
0x140364e90 664421e8 and ax, r13w
0x140364e94 4189c5 mov r13d, eax
0x140364e97 75c7 jne 0x140364e60
0x140364e99 66440f74c7 pcmpeqb xmm8, xmm7
0x140364e9e 66410fd7c0 pmovmskb eax, xmm8
0x140364ea3 85c0 test eax, eax
0x140364ea5 0f8580000000 jne 0x140364f2b
0x140364eab 488b4518 mov rax, qword ptr [rbp + 0x18]
0x140364eaf 4901c7 add r15, rax
0x140364eb2 4983c710 add r15, 0x10
0x140364eb6 4883c010 add rax, 0x10
0x140364eba 48894518 mov qword ptr [rbp + 0x18], rax
0x140364ebe 4d21f7 and r15, r14
0x140364ec1 eb80 jmp 0x140364e43
0x140364ec3 488b73f0 mov rsi, qword ptr [rbx - 0x10]
0x140364ec7 488b7bf8 mov rdi, qword ptr [rbx - 8]
0x140364ecb baffffffff mov edx, 0xffffffff
0x140364ed0 488b4d20 mov rcx, qword ptr [rbp + 0x20]
0x140364ed4 f00fc111 lock xadd dword ptr [rcx], edx
0x140364ed8 ffca dec edx
0x140364eda 89d0 mov eax, edx
0x140364edc 25ffffffbf and eax, 0xbfffffff
0x140364ee1 f7d8 neg eax
0x140364ee3 0f801c020000 jo 0x140365105
0x140364ee9 4889f0 mov rax, rsi
0x140364eec 4889fa mov rdx, rdi
0x140364eef 0f287530 movaps xmm6, xmmword ptr [rbp + 0x30]
0x140364ef3 0f287d40 movaps xmm7, xmmword ptr [rbp + 0x40]
0x140364ef7 440f284550 movaps xmm8, xmmword ptr [rbp + 0x50]
0x140364efc 4881c4e8000000 add rsp, 0xe8
0x140364f03 5b pop rbx
0x140364f04 5f pop rdi
0x140364f05 5e pop rsi
0x140364f06 415c pop r12
0x140364f08 415d pop r13
0x140364f0a 415e pop r14
0x140364f0c 415f pop r15
0x140364f0e 5d pop rbp
0x140364f0f c3 ret 
0x140364f10 4889d9 mov rcx, rbx
0x140364f13 e8183b5702 call 0x1428d8a30
0x140364f18 0fb64308 movzx eax, byte ptr [rbx + 8]
0x140364f1c 48895d20 mov qword ptr [rbp + 0x20], rbx
0x140364f20 48837b2800 cmp qword ptr [rbx + 0x28], 0
0x140364f25 0f855efeffff jne 0x140364d89
0x140364f2b baffffffff mov edx, 0xffffffff
0x140364f30 488b5d20 mov rbx, qword ptr [rbp + 0x20]
0x140364f34 f00fc113 lock xadd dword ptr [rbx], edx
0x140364f38 ffca dec edx
0x140364f3a 89d0 mov eax, edx
0x140364f3c 25ffffffbf and eax, 0xbfffffff
0x140364f41 f7d8 neg eax
0x140364f43 0f80e9030000 jo 0x140365332
0x140364f49 b9ffffff3f mov ecx, 0x3fffffff
0x140364f4e 31c0 xor eax, eax
0x140364f50 f00fb10b lock cmpxchg dword ptr [rbx], ecx
0x140364f54 0f85f1030000 jne 0x14036534b
0x140364f5a 488b05bf742c03 mov rax, qword ptr [rip + 0x32c74bf]
0x140364f61 488b00 mov rax, qword ptr [rax]
0x140364f64 48d1e0 shl rax, 1
0x140364f67 4885c0 test rax, rax
0x140364f6a 0f85b6030000 jne 0x140365326
0x140364f70 31c0 xor eax, eax
0x140364f72 0fb64b08 movzx ecx, byte ptr [rbx + 8]
0x140364f76 48895dc0 mov qword ptr [rbp - 0x40], rbx
0x140364f7a 8845c8 mov byte ptr [rbp - 0x38], al
0x140364f7d 48837b2800 cmp qword ptr [rbx + 0x28], 0
0x140364f82 0f8487010000 je 0x14036510f
0x140364f88 48b8f1de68ef5cfec09d movabs rax, 0x9dc0fe5cef68def1
0x140364f92 488945e0 mov qword ptr [rbp - 0x20], rax
0x140364f96 660fefc0 pxor xmm0, xmm0
0x140364f9a 660f7f45d0 movdqa xmmword ptr [rbp - 0x30], xmm0
0x140364f9f c6450800 mov byte ptr [rbp + 8], 0
0x140364fa3 48b87713d038e6212845 movabs rax, 0x452821e638d01377
0x140364fad 488945e8 mov qword ptr [rbp - 0x18], rax
0x140364fb1 48b86c0ce934cf6654be movabs rax, 0xbe5466cf34e90c6c
0x140364fbb 488945f0 mov qword ptr [rbp - 0x10], rax
0x140364fbf 48b8dd507cc9b729acc0 movabs rax, 0xc0ac29b7c97c50dd
0x140364fc9 488945f8 mov qword ptr [rbp - 8], rax
0x140364fcd 48b8170947b5b5d5843f movabs rax, 0x3f84d5b5b5470917
0x140364fd7 48894500 mov qword ptr [rbp], rax
0x140364fdb 4c8d0576aa6002 lea r8, [rip + 0x260aa76]
0x140364fe2 488d55d0 lea rdx, [rbp - 0x30]
0x140364fe6 4889f9 mov rcx, rdi
0x140364fe9 ff5640 call qword ptr [rsi + 0x40]
0x140364fec 807d0800 cmp byte ptr [rbp + 8], 0
0x140364ff0 7418 je 0x14036500a
0x140364ff2 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x140364ff6 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x140364ffa 48334dd0 xor rcx, qword ptr [rbp - 0x30]
0x140364ffe 483345d8 xor rax, qword ptr [rbp - 0x28]
0x140365002 48f7e1 mul rcx
0x140365005 4831c2 xor rdx, rax
0x140365008 eb04 jmp 0x14036500e
0x14036500a 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x14036500e 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140365012 4c8b6010 mov r12, qword ptr [rax + 0x10]
0x140365016 4c8b7018 mov r14, qword ptr [rax + 0x18]
0x14036501a 4d89f7 mov r15, r14
0x14036501d 4921d7 and r15, rdx
0x140365020 48c1ea39 shr rdx, 0x39
0x140365024 660f6ec2 movd xmm0, edx
0x140365028 660f60c0 punpcklbw xmm0, xmm0
0x14036502c f20f70c000 pshuflw xmm0, xmm0, 0
0x140365031 660f70f000 pshufd xmm6, xmm0, 0
0x140365036 48c7451800000000 mov qword ptr [rbp + 0x18], 0
0x14036503e 660f76ff pcmpeqd xmm7, xmm7
0x140365042 f3470f6f043c movdqu xmm8, xmmword ptr [r12 + r15]
0x140365048 66410f6fc0 movdqa xmm0, xmm8
0x14036504d 660f74c6 pcmpeqb xmm0, xmm6
0x140365051 66440fd7e8 pmovmskb r13d, xmm0
0x140365056 4585ed test r13d, r13d
0x140365059 743e je 0x140365099
0x14036505b 0f1f440000 nop dword ptr [rax + rax]
0x140365060 f3410fbcc5 tzcnt eax, r13d
0x140365065 4c01f8 add rax, r15
0x140365068 4c21f0 and rax, r14
0x14036506b 48c1e004 shl rax, 4
0x14036506f 4c89e3 mov rbx, r12
0x140365072 4829c3 sub rbx, rax
0x140365075 4c8b43f0 mov r8, qword ptr [rbx - 0x10]
0x140365079 4c8b4bf8 mov r9, qword ptr [rbx - 8]
0x14036507d 4889f9 mov rcx, rdi
0x140365080 4889f2 mov rdx, rsi
0x140365083 e838679a00 call 0x140d0b7c0
0x140365088 84c0 test al, al
0x14036508a 7536 jne 0x1403650c2
0x14036508c 418d45ff lea eax, [r13 - 1]
0x140365090 664421e8 and ax, r13w
0x140365094 4189c5 mov r13d, eax
0x140365097 75c7 jne 0x140365060
0x140365099 66440f74c7 pcmpeqb xmm8, xmm7
0x14036509e 66410fd7c0 pmovmskb eax, xmm8
0x1403650a3 85c0 test eax, eax
0x1403650a5 7568 jne 0x14036510f
0x1403650a7 488b4518 mov rax, qword ptr [rbp + 0x18]
0x1403650ab 4901c7 add r15, rax
0x1403650ae 4983c710 add r15, 0x10
0x1403650b2 4883c010 add rax, 0x10
0x1403650b6 48894518 mov qword ptr [rbp + 0x18], rax
0x1403650ba 4d21f7 and r15, r14
0x1403650bd e980ffffff jmp 0x140365042
0x1403650c2 488b73f0 mov rsi, qword ptr [rbx - 0x10]
0x1403650c6 488b7bf8 mov rdi, qword ptr [rbx - 8]
0x1403650ca 488b4dc0 mov rcx, qword ptr [rbp - 0x40]
0x1403650ce 807dc800 cmp byte ptr [rbp - 0x38], 0
0x1403650d2 7516 jne 0x1403650ea
0x1403650d4 488b0545732c03 mov rax, qword ptr [rip + 0x32c7345]
0x1403650db 488b00 mov rax, qword ptr [rax]
0x1403650de 48d1e0 shl rax, 1
0x1403650e1 4885c0 test rax, rax
0x1403650e4 0f856e020000 jne 0x140365358
0x1403650ea ba010000c0 mov edx, 0xc0000001
0x1403650ef f00fc111 lock xadd dword ptr [rcx], edx
0x1403650f3 81c2010000c0 add edx, 0xc0000001
0x1403650f9 81fa00000040 cmp edx, 0x40000000
0x1403650ff 0f82e4fdffff jb 0x140364ee9
0x140365105 e8363c5702 call 0x1428d8d40
0x14036510a e9dafdffff jmp 0x140364ee9
0x14036510f 4889f9 mov rcx, rdi
0x140365112 4889f2 mov rdx, rsi
0x140365115 e8e6669a00 call 0x140d0b800
0x14036511a 4889c6 mov rsi, rax
0x14036511d 4889d7 mov rdi, rdx
0x140365120 48b8f1de68ef5cfec09d movabs rax, 0x9dc0fe5cef68def1
0x14036512a 488945e0 mov qword ptr [rbp - 0x20], rax
0x14036512e 660fefc0 pxor xmm0, xmm0
0x140365132 660f7f45d0 movdqa xmmword ptr [rbp - 0x30], xmm0
0x140365137 c6450800 mov byte ptr [rbp + 8], 0
0x14036513b 48b87713d038e6212845 movabs rax, 0x452821e638d01377
0x140365145 488945e8 mov qword ptr [rbp - 0x18], rax
0x140365149 48b86c0ce934cf6654be movabs rax, 0xbe5466cf34e90c6c
0x140365153 488945f0 mov qword ptr [rbp - 0x10], rax
0x140365157 48b8dd507cc9b729acc0 movabs rax, 0xc0ac29b7c97c50dd
0x140365161 488945f8 mov qword ptr [rbp - 8], rax
0x140365165 48b8170947b5b5d5843f movabs rax, 0x3f84d5b5b5470917
0x14036516f 48894500 mov qword ptr [rbp], rax
0x140365173 4c8d05dea86002 lea r8, [rip + 0x260a8de]
0x14036517a 488d55d0 lea rdx, [rbp - 0x30]
0x14036517e 4889f1 mov rcx, rsi
0x140365181 ff5740 call qword ptr [rdi + 0x40]
0x140365184 807d0800 cmp byte ptr [rbp + 8], 0
0x140365188 742d je 0x1403651b7
0x14036518a 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x14036518e 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x140365192 48334dd0 xor rcx, qword ptr [rbp - 0x30]
0x140365196 483345d8 xor rax, qword ptr [rbp - 0x28]
0x14036519a 48f7e1 mul rcx
0x14036519d 4889d3 mov rbx, rdx
0x1403651a0 4831c3 xor rbx, rax
0x1403651a3 488b4520 mov rax, qword ptr [rbp + 0x20]
0x1403651a7 4c8d7010 lea r14, [rax + 0x10]
0x1403651ab 4883782000 cmp qword ptr [rax + 0x20], 0
0x1403651b0 751c jne 0x1403651ce
0x1403651b2 e9bd010000 jmp 0x140365374
0x1403651b7 488b5de0 mov rbx, qword ptr [rbp - 0x20]
0x1403651bb 488b4520 mov rax, qword ptr [rbp + 0x20]
0x1403651bf 4c8d7010 lea r14, [rax + 0x10]
0x1403651c3 4883782000 cmp qword ptr [rax + 0x20], 0
0x1403651c8 0f84a6010000 je 0x140365374
0x1403651ce 488b4520 mov rax, qword ptr [rbp + 0x20]
0x1403651d2 488b4810 mov rcx, qword ptr [rax + 0x10]
0x1403651d6 4c8b6018 mov r12, qword ptr [rax + 0x18]
0x1403651da 4889d8 mov rax, rbx
0x1403651dd 48c1e839 shr rax, 0x39
0x1403651e1 488945b8 mov qword ptr [rbp - 0x48], rax
0x1403651e5 660f6ec0 movd xmm0, eax
0x1403651e9 660f60c0 punpcklbw xmm0, xmm0
0x1403651ed f20f70c000 pshuflw xmm0, xmm0, 0
0x1403651f2 660f70f000 pshufd xmm6, xmm0, 0
0x1403651f7 31d2 xor edx, edx
0x1403651f9 660f76ff pcmpeqd xmm7, xmm7
0x1403651fd 4531ed xor r13d, r13d
0x140365200 48894da8 mov qword ptr [rbp - 0x58], rcx
0x140365204 48895518 mov qword ptr [rbp + 0x18], rdx
0x140365208 4c21e3 and rbx, r12
0x14036520b f3440f6f0419 movdqu xmm8, xmmword ptr [rcx + rbx]
0x140365211 66410f6fc0 movdqa xmm0, xmm8
0x140365216 660f74c6 pcmpeqb xmm0, xmm6
0x14036521a 66440fd7f8 pmovmskb r15d, xmm0
0x14036521f 4585ff test r15d, r15d
0x140365222 7450 je 0x140365274
0x140365224 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140365230 f3410fbcc7 tzcnt eax, r15d
0x140365235 4801d8 add rax, rbx
0x140365238 4c21e0 and rax, r12
0x14036523b 498b0e mov rcx, qword ptr [r14]
0x14036523e 48c1e004 shl rax, 4
0x140365242 4889ca mov rdx, rcx
0x140365245 4829c2 sub rdx, rax
0x140365248 4c8b42f0 mov r8, qword ptr [rdx - 0x10]
0x14036524c 48f7d8 neg rax
0x14036524f 4c8b4c01f8 mov r9, qword ptr [rcx + rax - 8]
0x140365254 4889f1 mov rcx, rsi
0x140365257 4889fa mov rdx, rdi
0x14036525a e861659a00 call 0x140d0b7c0
0x14036525f 84c0 test al, al
0x140365261 0f8563feffff jne 0x1403650ca
0x140365267 418d47ff lea eax, [r15 - 1]
0x14036526b 664421f8 and ax, r15w
0x14036526f 4189c7 mov r15d, eax
0x140365272 75bc jne 0x140365230
0x140365274 4983fd01 cmp r13, 1
0x140365278 41bd01000000 mov r13d, 1
0x14036527e 741c je 0x14036529c
0x140365280 66410fd7c0 pmovmskb eax, xmm8
0x140365285 4531ed xor r13d, r13d
0x140365288 85c0 test eax, eax
0x14036528a 410f95c5 setne r13b
0x14036528e f30fbcc0 tzcnt eax, eax
0x140365292 4801d8 add rax, rbx
0x140365295 4c21e0 and rax, r12
0x140365298 488945b0 mov qword ptr [rbp - 0x50], rax
0x14036529c 66440f74c7 pcmpeqb xmm8, xmm7
0x1403652a1 66410fd7c0 pmovmskb eax, xmm8
0x1403652a6 85c0 test eax, eax
0x1403652a8 488b4da8 mov rcx, qword ptr [rbp - 0x58]
0x1403652ac 488b4518 mov rax, qword ptr [rbp + 0x18]
0x1403652b0 7513 jne 0x1403652c5
0x1403652b2 4801c3 add rbx, rax
0x1403652b5 4883c310 add rbx, 0x10
0x1403652b9 4883c010 add rax, 0x10
0x1403652bd 4889c2 mov rdx, rax
0x1403652c0 e93fffffff jmp 0x140365204
0x1403652c5 488b45b0 mov rax, qword ptr [rbp - 0x50]
0x1403652c9 803c0100 cmp byte ptr [rcx + rax], 0
0x1403652cd 780c js 0x1403652db
0x1403652cf 660f6f01 movdqa xmm0, xmmword ptr [rcx]
0x1403652d3 660fd7c0 pmovmskb eax, xmm0
0x1403652d7 f30fbcc0 tzcnt eax, eax
0x1403652db 488b5520 mov rdx, qword ptr [rbp + 0x20]
0x1403652df 4989c1 mov r9, rax
0x1403652e2 488b4210 mov rax, qword ptr [rdx + 0x10]
0x1403652e6 420fb60c08 movzx ecx, byte ptr [rax + r9]
0x1403652eb 83e101 and ecx, 1
0x1403652ee 48294a20 sub qword ptr [rdx + 0x20], rcx
0x1403652f2 498d49f0 lea rcx, [r9 - 0x10]
0x1403652f6 48234a18 and rcx, qword ptr [rdx + 0x18]
0x1403652fa 4c8b45b8 mov r8, qword ptr [rbp - 0x48]
0x1403652fe 46880408 mov byte ptr [rax + r9], r8b
0x140365302 4488440810 mov byte ptr [rax + rcx + 0x10], r8b
0x140365307 48ff4228 inc qword ptr [rdx + 0x28]
0x14036530b 49c1e104 shl r9, 4
0x14036530f 4889c1 mov rcx, rax
0x140365312 4c29c9 sub rcx, r9
0x140365315 49f7d9 neg r9
0x140365318 4a897408f0 mov qword ptr [rax + r9 - 0x10], rsi
0x14036531d 488979f8 mov qword ptr [rcx - 8], rdi
0x140365321 e9a4fdffff jmp 0x1403650ca
0x140365326 e885335702 call 0x1428d86b0
0x14036532b 3401 xor al, 1
0x14036532d e940fcffff jmp 0x140364f72
0x140365332 4889d9 mov rcx, rbx
0x140365335 e8063a5702 call 0x1428d8d40
0x14036533a b9ffffff3f mov ecx, 0x3fffffff
0x14036533f 31c0 xor eax, eax
0x140365341 f00fb10b lock cmpxchg dword ptr [rbx], ecx
0x140365345 0f840ffcffff je 0x140364f5a
0x14036534b 4889d9 mov rcx, rbx
0x14036534e e8bd385702 call 0x1428d8c10
0x140365353 e902fcffff jmp 0x140364f5a
0x140365358 4889cb mov rbx, rcx
0x14036535b e850335702 call 0x1428d86b0
0x140365360 4889d9 mov rcx, rbx
0x140365363 84c0 test al, al
0x140365365 0f857ffdffff jne 0x1403650ea
0x14036536b c6410801 mov byte ptr [rcx + 8], 1
0x14036536f e976fdffff jmp 0x1403650ea
0x140365374 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140365378 4c8d4030 lea r8, [rax + 0x30]
0x14036537c ba01000000 mov edx, 1
0x140365381 4c89f1 mov rcx, r14
0x140365384 41b101 mov r9b, 1
0x140365387 e8a4fe5602 call 0x1428d5230
0x14036538c e93dfeffff jmp 0x1403651ce
0x140365391 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1403653a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1403653a5 55 push rbp
0x1403653a6 4157 push r15
0x1403653a8 4156 push r14
0x1403653aa 4155 push r13
0x1403653ac 4154 push r12
0x1403653ae 56 push rsi
0x1403653af 57 push rdi
0x1403653b0 53 push rbx
0x1403653b1 4883ec58 sub rsp, 0x58
0x1403653b5 488daa80000000 lea rbp, [rdx + 0x80]
0x1403653bc 66440f7f442420 movdqa xmmword ptr [rsp + 0x20], xmm8
0x1403653c3 660f7f7c2430 movdqa xmmword ptr [rsp + 0x30], xmm7
0x1403653c9 660f7f742440 movdqa xmmword ptr [rsp + 0x40], xmm6
0x1403653cf baffffffff mov edx, 0xffffffff
0x1403653d4 488b4520 mov rax, qword ptr [rbp + 0x20]
0x1403653d8 f00fc110 lock xadd dword ptr [rax], edx
0x1403653dc ffca dec edx
0x1403653de 89d0 mov eax, edx
0x1403653e0 25ffffffbf and eax, 0xbfffffff
0x1403653e5 f7d8 neg eax
0x1403653e7 7021 jo 0x14036540a
0x1403653e9 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x1403653ee 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x1403653f3 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x1403653f9 4883c458 add rsp, 0x58
0x1403653fd 5b pop rbx
0x1403653fe 5f pop rdi
0x1403653ff 5e pop rsi
0x140365400 415c pop r12
0x140365402 415d pop r13
0x140365404 415e pop r14
0x140365406 415f pop r15
0x140365408 5d pop rbp
0x140365409 c3 ret 
0x14036540a 488b4d20 mov rcx, qword ptr [rbp + 0x20]
0x14036540e e82d395702 call 0x1428d8d40
0x140365413 ebd4 jmp 0x1403653e9
0x140365415 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140365420 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140365425 55 push rbp
0x140365426 4157 push r15
0x140365428 4156 push r14
0x14036542a 4155 push r13
0x14036542c 4154 push r12
0x14036542e 56 push rsi
0x14036542f 57 push rdi
0x140365430 53 push rbx
0x140365431 4883ec58 sub rsp, 0x58
0x140365435 488daa80000000 lea rbp, [rdx + 0x80]
0x14036543c 66440f7f442420 movdqa xmmword ptr [rsp + 0x20], xmm8
0x140365443 660f7f7c2430 movdqa xmmword ptr [rsp + 0x30], xmm7
0x140365449 660f7f742440 movdqa xmmword ptr [rsp + 0x40], xmm6
0x14036544f 488d4dc0 lea rcx, [rbp - 0x40]
0x140365453 e898b92600 call 0x1405d0df0
0x140365458 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
0x14036545d 0f287c2430 movaps xmm7, xmmword ptr [rsp + 0x30]
0x140365462 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
0x140365468 4883c458 add rsp, 0x58
0x14036546c 5b pop rbx
0x14036546d 5f pop rdi
0x14036546e 5e pop rsi
0x14036546f 415c pop r12
0x140365471 415d pop r13
0x140365473 415e pop r14
0x140365475 415f pop r15
0x140365477 5d pop rbp
0x140365478 c3 ret 
