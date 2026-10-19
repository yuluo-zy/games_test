0x140a97b70 55 push rbp
0x140a97b71 4157 push r15
0x140a97b73 4156 push r14
0x140a97b75 4155 push r13
0x140a97b77 4154 push r12
0x140a97b79 56 push rsi
0x140a97b7a 57 push rdi
0x140a97b7b 53 push rbx
0x140a97b7c 4881ec58040000 sub rsp, 0x458
0x140a97b83 488dac2480000000 lea rbp, [rsp + 0x80]
0x140a97b8b 440f29bdc0030000 movaps xmmword ptr [rbp + 0x3c0], xmm15
0x140a97b93 440f29b5b0030000 movaps xmmword ptr [rbp + 0x3b0], xmm14
0x140a97b9b 440f29ada0030000 movaps xmmword ptr [rbp + 0x3a0], xmm13
0x140a97ba3 440f29a590030000 movaps xmmword ptr [rbp + 0x390], xmm12
0x140a97bab 440f299d80030000 movaps xmmword ptr [rbp + 0x380], xmm11
0x140a97bb3 440f299570030000 movaps xmmword ptr [rbp + 0x370], xmm10
0x140a97bbb 440f298d60030000 movaps xmmword ptr [rbp + 0x360], xmm9
0x140a97bc3 440f298550030000 movaps xmmword ptr [rbp + 0x350], xmm8
0x140a97bcb 0f29bd40030000 movaps xmmword ptr [rbp + 0x340], xmm7
0x140a97bd2 0f29b530030000 movaps xmmword ptr [rbp + 0x330], xmm6
0x140a97bd9 48c78528030000feffffff mov qword ptr [rbp + 0x328], 0xfffffffffffffffe
0x140a97be4 4d89cf mov r15, r9
0x140a97be7 4c898590020000 mov qword ptr [rbp + 0x290], r8
0x140a97bee 4889d7 mov rdi, rdx
0x140a97bf1 4989ce mov r14, rcx
0x140a97bf4 e807ee1700 call 0x140c16a00
0x140a97bf9 888521030000 mov byte ptr [rbp + 0x321], al
0x140a97bff 84c0 test al, al
0x140a97c01 0f8431020000 je 0x140a97e38
0x140a97c07 4c8d2531300402 lea r12, [rip + 0x2043031]
0x140a97c0e be50000000 mov esi, 0x50
0x140a97c13 ba50000000 mov edx, 0x50
0x140a97c18 4c89e1 mov rcx, r12
0x140a97c1b e8e0ecffff call 0x140a96900
0x140a97c20 4883f801 cmp rax, 1
0x140a97c24 0f85aa000000 jne 0x140a97cd4
0x140a97c2a 4989d1 mov r9, rdx
0x140a97c2d 4885d2 test rdx, rdx
0x140a97c30 7436 je 0x140a97c68
0x140a97c32 4983f950 cmp r9, 0x50
0x140a97c36 732e jae 0x140a97c66
0x140a97c38 488d0500300402 lea rax, [rip + 0x2043000]
0x140a97c3f 41803c01bf cmp byte ptr [r9 + rax], 0xbf
0x140a97c44 7f22 jg 0x140a97c68
0x140a97c46 488d05cb300402 lea rax, [rip + 0x20430cb]
0x140a97c4d 4889442420 mov qword ptr [rsp + 0x20], rax
0x140a97c52 488d0de62f0402 lea rcx, [rip + 0x2042fe6]
0x140a97c59 ba50000000 mov edx, 0x50
0x140a97c5e 4531c0 xor r8d, r8d
0x140a97c61 e8da1ce401 call 0x1428d9940
0x140a97c66 75de jne 0x140a97c46
0x140a97c68 4c8d25d02f0402 lea r12, [rip + 0x2042fd0]
0x140a97c6f 4c89e1 mov rcx, r12
0x140a97c72 4c89ca mov rdx, r9
0x140a97c75 e886ecffff call 0x140a96900
0x140a97c7a 4883f801 cmp rax, 1
0x140a97c7e 7554 jne 0x140a97cd4
0x140a97c80 4989d0 mov r8, rdx
0x140a97c83 4983c002 add r8, 2
0x140a97c87 7439 je 0x140a97cc2
0x140a97c89 4983f850 cmp r8, 0x50
0x140a97c8d 7331 jae 0x140a97cc0
0x140a97c8f 488d05a92f0402 lea rax, [rip + 0x2042fa9]
0x140a97c96 41803c00bf cmp byte ptr [r8 + rax], 0xbf
0x140a97c9b 7f25 jg 0x140a97cc2
0x140a97c9d 488d058c300402 lea rax, [rip + 0x204308c]
0x140a97ca4 4889442420 mov qword ptr [rsp + 0x20], rax
0x140a97ca9 488d0d8f2f0402 lea rcx, [rip + 0x2042f8f]
0x140a97cb0 ba50000000 mov edx, 0x50
0x140a97cb5 41b950000000 mov r9d, 0x50
0x140a97cbb e8801ce401 call 0x1428d9940
0x140a97cc0 75db jne 0x140a97c9d
0x140a97cc2 be4e000000 mov esi, 0x4e
0x140a97cc7 4829d6 sub rsi, rdx
0x140a97cca 4c8d256e2f0402 lea r12, [rip + 0x2042f6e]
0x140a97cd1 4d01c4 add r12, r8
0x140a97cd4 488d05dc350402 lea rax, [rip + 0x20435dc]
0x140a97cdb 4c8d2d98350402 lea r13, [rip + 0x2043598]
0x140a97ce2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a97cf0 0fbe48ff movsx ecx, byte ptr [rax - 1]
0x140a97cf4 85c9 test ecx, ecx
0x140a97cf6 7818 js 0x140a97d10
0x140a97cf8 48ffc8 dec rax
0x140a97cfb 83f95c cmp ecx, 0x5c
0x140a97cfe 7569 jne 0x140a97d69
0x140a97d00 eb7c jmp 0x140a97d7e
0x140a97d02 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a97d10 0fb650fe movzx edx, byte ptr [rax - 2]
0x140a97d14 80fac0 cmp dl, 0xc0
0x140a97d17 7d25 jge 0x140a97d3e
0x140a97d19 440fb640fd movzx r8d, byte ptr [rax - 3]
0x140a97d1e 4180f8c0 cmp r8b, 0xc0
0x140a97d22 7d23 jge 0x140a97d47
0x140a97d24 440fb648fc movzx r9d, byte ptr [rax - 4]
0x140a97d29 4883c0fc add rax, -4
0x140a97d2d 4183e107 and r9d, 7
0x140a97d31 41c1e106 shl r9d, 6
0x140a97d35 4183e03f and r8d, 0x3f
0x140a97d39 4509c8 or r8d, r9d
0x140a97d3c eb11 jmp 0x140a97d4f
0x140a97d3e 4883c0fe add rax, -2
0x140a97d42 83e21f and edx, 0x1f
0x140a97d45 eb12 jmp 0x140a97d59
0x140a97d47 4883c0fd add rax, -3
0x140a97d4b 4183e00f and r8d, 0xf
0x140a97d4f 41c1e006 shl r8d, 6
0x140a97d53 83e23f and edx, 0x3f
0x140a97d56 4409c2 or edx, r8d
0x140a97d59 c1e206 shl edx, 6
0x140a97d5c 80e13f and cl, 0x3f
0x140a97d5f 0fb6c9 movzx ecx, cl
0x140a97d62 09d1 or ecx, edx
0x140a97d64 83f95c cmp ecx, 0x5c
0x140a97d67 7415 je 0x140a97d7e
0x140a97d69 83f92f cmp ecx, 0x2f
0x140a97d6c 7410 je 0x140a97d7e
0x140a97d6e 4c39e8 cmp rax, r13
0x140a97d71 0f8579ffffff jne 0x140a97cf0
0x140a97d77 bb3d000000 mov ebx, 0x3d
0x140a97d7c eb4f jmp 0x140a97dcd
0x140a97d7e 4c8d2df5340402 lea r13, [rip + 0x20434f5]
0x140a97d85 4c29e8 sub rax, r13
0x140a97d88 4989c0 mov r8, rax
0x140a97d8b 49ffc0 inc r8
0x140a97d8e 7432 je 0x140a97dc2
0x140a97d90 4983f83d cmp r8, 0x3d
0x140a97d94 732a jae 0x140a97dc0
0x140a97d96 43803c28bf cmp byte ptr [r8 + r13], 0xbf
0x140a97d9b 7f25 jg 0x140a97dc2
0x140a97d9d 488d05542f0402 lea rax, [rip + 0x2042f54]
0x140a97da4 4889442420 mov qword ptr [rsp + 0x20], rax
0x140a97da9 488d0dca340402 lea rcx, [rip + 0x20434ca]
0x140a97db0 ba3d000000 mov edx, 0x3d
0x140a97db5 41b93d000000 mov r9d, 0x3d
0x140a97dbb e8801be401 call 0x1428d9940
0x140a97dc0 75db jne 0x140a97d9d
0x140a97dc2 bb3c000000 mov ebx, 0x3c
0x140a97dc7 4829c3 sub rbx, rax
0x140a97dca 4d01c5 add r13, r8
0x140a97dcd e85e031800 call 0x140c18130
0x140a97dd2 4889c1 mov rcx, rax
0x140a97dd5 488b00 mov rax, qword ptr [rax]
0x140a97dd8 4883f801 cmp rax, 1
0x140a97ddc 0f85a4020000 jne 0x140a98086
0x140a97de2 4883c108 add rcx, 8
0x140a97de6 48833900 cmp qword ptr [rcx], 0
0x140a97dea 0f85c81b0000 jne 0x140a999b8
0x140a97df0 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x140a97df7 48898d18030000 mov qword ptr [rbp + 0x318], rcx
0x140a97dfe 4883c108 add rcx, 8
0x140a97e02 48895c2420 mov qword ptr [rsp + 0x20], rbx
0x140a97e07 48c744243000000000 mov qword ptr [rsp + 0x30], 0
0x140a97e10 48c744242801000000 mov qword ptr [rsp + 0x28], 1
0x140a97e19 4c89e2 mov rdx, r12
0x140a97e1c 4989f0 mov r8, rsi
0x140a97e1f 4d89e9 mov r9, r13
0x140a97e22 e819ef1700 call 0x140c16d40
0x140a97e27 48898550020000 mov qword ptr [rbp + 0x250], rax
0x140a97e2e 488b8518030000 mov rax, qword ptr [rbp + 0x318]
0x140a97e35 48ff00 inc qword ptr [rax]
0x140a97e38 488b8540040000 mov rax, qword ptr [rbp + 0x440]
0x140a97e3f 8b481c mov ecx, dword ptr [rax + 0x1c]
0x140a97e42 488b5010 mov rdx, qword ptr [rax + 0x10]
0x140a97e46 890a mov dword ptr [rdx], ecx
0x140a97e48 4c8b28 mov r13, qword ptr [rax]
0x140a97e4b 488b0f mov rcx, qword ptr [rdi]
0x140a97e4e 488d1563641902 lea rdx, [rip + 0x2196463]
0x140a97e55 e8c618edff call 0x140969720
0x140a97e5a 4889c6 mov rsi, rax
0x140a97e5d 4885c0 test rax, rax
0x140a97e60 0f84111c0000 je 0x140a99a77
0x140a97e66 488d0592590002 lea rax, [rip + 0x2005992]
0x140a97e6d 48894538 mov qword ptr [rbp + 0x38], rax
0x140a97e71 48c745403a000000 mov qword ptr [rbp + 0x40], 0x3a
0x140a97e79 4889f7 mov rdi, rsi
0x140a97e7c 4883c710 add rdi, 0x10
0x140a97e80 48837e183a cmp qword ptr [rsi + 0x18], 0x3a
0x140a97e85 0f85ef1a0000 jne 0x140a9997a
0x140a97e8b 488b17 mov rdx, qword ptr [rdi]
0x140a97e8e 488d0d6a590002 lea rcx, [rip + 0x200596a]
0x140a97e95 41b83a000000 mov r8d, 0x3a
0x140a97e9b e850bde801 call 0x142923bf0
0x140a97ea0 85c0 test eax, eax
0x140a97ea2 0f85d21a0000 jne 0x140a9997a
0x140a97ea8 488b0e mov rcx, qword ptr [rsi]
0x140a97eab 488b4608 mov rax, qword ptr [rsi + 8]
0x140a97eaf ff5018 call qword ptr [rax + 0x18]
0x140a97eb2 48898548020000 mov qword ptr [rbp + 0x248], rax
0x140a97eb9 4883bd4802000000 cmp qword ptr [rbp + 0x248], 0
0x140a97ec1 0f84b01b0000 je 0x140a99a77
0x140a97ec7 418b471c mov eax, dword ptr [r15 + 0x1c]
0x140a97ecb 498b4f10 mov rcx, qword ptr [r15 + 0x10]
0x140a97ecf 8901 mov dword ptr [rcx], eax
0x140a97ed1 498b17 mov rdx, qword ptr [r15]
0x140a97ed4 488d4dd0 lea rcx, [rbp - 0x30]
0x140a97ed8 e803ab7f00 call 0x1412929e0
0x140a97edd 48837dd000 cmp qword ptr [rbp - 0x30], 0
0x140a97ee2 4c89ad28010000 mov qword ptr [rbp + 0x128], r13
0x140a97ee9 0f8419010000 je 0x140a98008
0x140a97eef 0f1045d0 movups xmm0, xmmword ptr [rbp - 0x30]
0x140a97ef3 0f2985b0000000 movaps xmmword ptr [rbp + 0xb0], xmm0
0x140a97efa 488b45e0 mov rax, qword ptr [rbp - 0x20]
0x140a97efe 488985c0000000 mov qword ptr [rbp + 0xc0], rax
0x140a97f05 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
0x140a97f0c 488b7030 mov rsi, qword ptr [rax + 0x30]
0x140a97f10 48898508030000 mov qword ptr [rbp + 0x308], rax
0x140a97f17 488b7848 mov rdi, qword ptr [rax + 0x48]
0x140a97f1b 660f6f06 movdqa xmm0, xmmword ptr [rsi]
0x140a97f1f 660fd7c0 pmovmskb eax, xmm0
0x140a97f23 f7d0 not eax
0x140a97f25 488d5e10 lea rbx, [rsi + 0x10]
0x140a97f29 4c8d7d08 lea r15, [rbp + 8]
0x140a97f2d eb31 jmp 0x140a97f60
0x140a97f2f 90 nop
0x140a97f30 f30fbcc8 tzcnt ecx, eax
0x140a97f34 48f7d9 neg rcx
0x140a97f37 488d0c49 lea rcx, [rcx + rcx*2]
0x140a97f3b 448d60ff lea r12d, [rax - 1]
0x140a97f3f 4121c4 and r12d, eax
0x140a97f42 8b448efc mov eax, dword ptr [rsi + rcx*4 - 4]
0x140a97f46 894510 mov dword ptr [rbp + 0x10], eax
0x140a97f49 488b448ef4 mov rax, qword ptr [rsi + rcx*4 - 0xc]
0x140a97f4e 48894508 mov qword ptr [rbp + 8], rax
0x140a97f52 4c89e9 mov rcx, r13
0x140a97f55 4c89fa mov rdx, r15
0x140a97f58 e8e324eaff call 0x14093a440
0x140a97f5d 4489e0 mov eax, r12d
0x140a97f60 4883ef01 sub rdi, 1
0x140a97f64 7228 jb 0x140a97f8e
0x140a97f66 6685c0 test ax, ax
0x140a97f69 75c5 jne 0x140a97f30
0x140a97f6b 0f1f440000 nop dword ptr [rax + rax]
0x140a97f70 660f6f03 movdqa xmm0, xmmword ptr [rbx]
0x140a97f74 660fd7c0 pmovmskb eax, xmm0
0x140a97f78 4881c640ffffff add rsi, -0xc0
0x140a97f7f 4883c310 add rbx, 0x10
0x140a97f83 3dffff0000 cmp eax, 0xffff
0x140a97f88 74e6 je 0x140a97f70
0x140a97f8a f7d0 not eax
0x140a97f8c eba2 jmp 0x140a97f30
0x140a97f8e 498b06 mov rax, qword ptr [r14]
0x140a97f91 488985f0020000 mov qword ptr [rbp + 0x2f0], rax
0x140a97f98 e863ea1700 call 0x140c16a00
0x140a97f9d 89c3 mov ebx, eax
0x140a97f9f 84c0 test al, al
0x140a97fa1 747d je 0x140a98020
0x140a97fa3 4c8d3d3f2c0402 lea r15, [rip + 0x2042c3f]
0x140a97faa 41be53000000 mov r14d, 0x53
0x140a97fb0 ba53000000 mov edx, 0x53
0x140a97fb5 4c89f9 mov rcx, r15
0x140a97fb8 e843e9ffff call 0x140a96900
0x140a97fbd 4883f801 cmp rax, 1
0x140a97fc1 0f85f5000000 jne 0x140a980bc
0x140a97fc7 4989d1 mov r9, rdx
0x140a97fca 4885d2 test rdx, rdx
0x140a97fcd 745a je 0x140a98029
0x140a97fcf 4983f953 cmp r9, 0x53
0x140a97fd3 7352 jae 0x140a98027
0x140a97fd5 488d050d2c0402 lea rax, [rip + 0x2042c0d]
0x140a97fdc 41803c01bf cmp byte ptr [r9 + rax], 0xbf
0x140a97fe1 7f46 jg 0x140a98029
0x140a97fe3 488d052e2d0402 lea rax, [rip + 0x2042d2e]
0x140a97fea 4889442420 mov qword ptr [rsp + 0x20], rax
0x140a97fef 488d0df32b0402 lea rcx, [rip + 0x2042bf3]
0x140a97ff6 ba53000000 mov edx, 0x53
0x140a97ffb 4531c0 xor r8d, r8d
0x140a97ffe e83d19e401 call 0x1428d9940
0x140a98003 e9b31a0000 jmp 0x140a99abb
0x140a98008 b908000000 mov ecx, 8
0x140a9800d 4531f6 xor r14d, r14d
0x140a98010 48c785b802000000000000 mov qword ptr [rbp + 0x2b8], 0
0x140a9801b e974060000 jmp 0x140a98694
0x140a98020 31c0 xor eax, eax
0x140a98022 e911020000 jmp 0x140a98238
0x140a98027 75ba jne 0x140a97fe3
0x140a98029 4c8d3db92b0402 lea r15, [rip + 0x2042bb9]
0x140a98030 4c89f9 mov rcx, r15
0x140a98033 4c89ca mov rdx, r9
0x140a98036 e8c5e8ffff call 0x140a96900
0x140a9803b 4883f801 cmp rax, 1
0x140a9803f 757b jne 0x140a980bc
0x140a98041 4989d0 mov r8, rdx
0x140a98044 4983c002 add r8, 2
0x140a98048 745f je 0x140a980a9
0x140a9804a 4983f853 cmp r8, 0x53
0x140a9804e 7357 jae 0x140a980a7
0x140a98050 488d05922b0402 lea rax, [rip + 0x2042b92]
0x140a98057 41803c00bf cmp byte ptr [r8 + rax], 0xbf
0x140a9805c 7f4b jg 0x140a980a9
0x140a9805e 488d05cb2c0402 lea rax, [rip + 0x2042ccb]
0x140a98065 4889442420 mov qword ptr [rsp + 0x20], rax
0x140a9806a 488d0d782b0402 lea rcx, [rip + 0x2042b78]
0x140a98071 ba53000000 mov edx, 0x53
0x140a98076 41b953000000 mov r9d, 0x53
0x140a9807c e8bf18e401 call 0x1428d9940
0x140a98081 e9351a0000 jmp 0x140a99abb
0x140a98086 4885c0 test rax, rax
0x140a98089 0f85b4180000 jne 0x140a99943
0x140a9808f 31d2 xor edx, edx
0x140a98091 e83a07e601 call 0x1428f87d0
0x140a98096 4889c1 mov rcx, rax
0x140a98099 4885c0 test rax, rax
0x140a9809c 0f8544fdffff jne 0x140a97de6
0x140a980a2 e99c180000 jmp 0x140a99943
0x140a980a7 75b5 jne 0x140a9805e
0x140a980a9 41be51000000 mov r14d, 0x51
0x140a980af 4929d6 sub r14, rdx
0x140a980b2 4c8d3d302b0402 lea r15, [rip + 0x2042b30]
0x140a980b9 4d01c7 add r15, r8
0x140a980bc 488d05f4310402 lea rax, [rip + 0x20431f4]
0x140a980c3 4c8d25b0310402 lea r12, [rip + 0x20431b0]
0x140a980ca 660f1f440000 nop word ptr [rax + rax]
0x140a980d0 0fbe48ff movsx ecx, byte ptr [rax - 1]
0x140a980d4 85c9 test ecx, ecx
0x140a980d6 7818 js 0x140a980f0
0x140a980d8 48ffc8 dec rax
0x140a980db 83f95c cmp ecx, 0x5c
0x140a980de 7569 jne 0x140a98149
0x140a980e0 eb7c jmp 0x140a9815e
0x140a980e2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a980f0 0fb650fe movzx edx, byte ptr [rax - 2]
0x140a980f4 80fac0 cmp dl, 0xc0
0x140a980f7 7d25 jge 0x140a9811e
0x140a980f9 440fb640fd movzx r8d, byte ptr [rax - 3]
0x140a980fe 4180f8c0 cmp r8b, 0xc0
0x140a98102 7d23 jge 0x140a98127
0x140a98104 440fb648fc movzx r9d, byte ptr [rax - 4]
0x140a98109 4883c0fc add rax, -4
0x140a9810d 4183e107 and r9d, 7
0x140a98111 41c1e106 shl r9d, 6
0x140a98115 4183e03f and r8d, 0x3f
0x140a98119 4509c8 or r8d, r9d
0x140a9811c eb11 jmp 0x140a9812f
0x140a9811e 4883c0fe add rax, -2
0x140a98122 83e21f and edx, 0x1f
0x140a98125 eb12 jmp 0x140a98139
0x140a98127 4883c0fd add rax, -3
0x140a9812b 4183e00f and r8d, 0xf
0x140a9812f 41c1e006 shl r8d, 6
0x140a98133 83e23f and edx, 0x3f
0x140a98136 4409c2 or edx, r8d
0x140a98139 c1e206 shl edx, 6
0x140a9813c 80e13f and cl, 0x3f
0x140a9813f 0fb6c9 movzx ecx, cl
0x140a98142 09d1 or ecx, edx
0x140a98144 83f95c cmp ecx, 0x5c
0x140a98147 7415 je 0x140a9815e
0x140a98149 83f92f cmp ecx, 0x2f
0x140a9814c 7410 je 0x140a9815e
0x140a9814e 4c39e0 cmp rax, r12
0x140a98151 0f8579ffffff jne 0x140a980d0
0x140a98157 be3d000000 mov esi, 0x3d
0x140a9815c eb54 jmp 0x140a981b2
0x140a9815e 4c8d2515310402 lea r12, [rip + 0x2043115]
0x140a98165 4c29e0 sub rax, r12
0x140a98168 4989c0 mov r8, rax
0x140a9816b 49ffc0 inc r8
0x140a9816e 7437 je 0x140a981a7
0x140a98170 4983f83d cmp r8, 0x3d
0x140a98174 732f jae 0x140a981a5
0x140a98176 43803c20bf cmp byte ptr [r8 + r12], 0xbf
0x140a9817b 7f2a jg 0x140a981a7
0x140a9817d 488d05742b0402 lea rax, [rip + 0x2042b74]
0x140a98184 4889442420 mov qword ptr [rsp + 0x20], rax
0x140a98189 488d0dea300402 lea rcx, [rip + 0x20430ea]
0x140a98190 ba3d000000 mov edx, 0x3d
0x140a98195 41b93d000000 mov r9d, 0x3d
0x140a9819b e8a017e401 call 0x1428d9940
0x140a981a0 e916190000 jmp 0x140a99abb
0x140a981a5 75d6 jne 0x140a9817d
0x140a981a7 be3c000000 mov esi, 0x3c
0x140a981ac 4829c6 sub rsi, rax
0x140a981af 4d01c4 add r12, r8
0x140a981b2 e879ff1700 call 0x140c18130
0x140a981b7 4889c1 mov rcx, rax
0x140a981ba 488b00 mov rax, qword ptr [rax]
0x140a981bd 4885c0 test rax, rax
0x140a981c0 740f je 0x140a981d1
0x140a981c2 83f801 cmp eax, 1
0x140a981c5 0f85d6180000 jne 0x140a99aa1
0x140a981cb 4883c108 add rcx, 8
0x140a981cf eb13 jmp 0x140a981e4
0x140a981d1 31d2 xor edx, edx
0x140a981d3 e8f805e601 call 0x1428f87d0
0x140a981d8 4889c1 mov rcx, rax
0x140a981db 4885c0 test rax, rax
0x140a981de 0f84bd180000 je 0x140a99aa1
0x140a981e4 48833900 cmp qword ptr [rcx], 0
0x140a981e8 0f85d6170000 jne 0x140a999c4
0x140a981ee 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x140a981f5 48898d18030000 mov qword ptr [rbp + 0x318], rcx
0x140a981fc 4883c108 add rcx, 8
0x140a98200 4889742420 mov qword ptr [rsp + 0x20], rsi
0x140a98205 48c744243000000000 mov qword ptr [rsp + 0x30], 0
0x140a9820e 48c744242801000000 mov qword ptr [rsp + 0x28], 1
0x140a98217 4c89fa mov rdx, r15
0x140a9821a 4d89f0 mov r8, r14
0x140a9821d 4d89e1 mov r9, r12
0x140a98220 e81beb1700 call 0x140c16d40
0x140a98225 488b8d18030000 mov rcx, qword ptr [rbp + 0x318]
0x140a9822c 48ff01 inc qword ptr [rcx]
0x140a9822f 48894500 mov qword ptr [rbp], rax
0x140a98233 b801000000 mov eax, 1
0x140a98238 889d40010000 mov byte ptr [rbp + 0x140], bl
0x140a9823e 488945f8 mov qword ptr [rbp - 8], rax
0x140a98242 48c7857801000000000000 mov qword ptr [rbp + 0x178], 0
0x140a9824d 48c7858001000004000000 mov qword ptr [rbp + 0x180], 4
0x140a98258 48c7858801000000000000 mov qword ptr [rbp + 0x188], 0
0x140a98263 e8d8542b00 call 0x140d4d740
0x140a98268 4889c6 mov rsi, rax
0x140a9826b f60001 test byte ptr [rax], 1
0x140a9826e 7411 je 0x140a98281
0x140a98270 4889f1 mov rcx, rsi
0x140a98273 4883c108 add rcx, 8
0x140a98277 488b4608 mov rax, qword ptr [rsi + 8]
0x140a9827b 488b5610 mov rdx, qword ptr [rsi + 0x10]
0x140a9827f eb1b jmp 0x140a9829c
0x140a98281 e87acb2c00 call 0x140d64e00
0x140a98286 4889f1 mov rcx, rsi
0x140a98289 4883c108 add rcx, 8
0x140a9828d 48c70601000000 mov qword ptr [rsi], 1
0x140a98294 48894608 mov qword ptr [rsi + 8], rax
0x140a98298 48895610 mov qword ptr [rsi + 0x10], rdx
0x140a9829c 4c8d4001 lea r8, [rax + 1]
0x140a982a0 4c8901 mov qword ptr [rcx], r8
0x140a982a3 0f1005162d0402 movups xmm0, xmmword ptr [rip + 0x2042d16]
0x140a982aa 0f1185f0000000 movups xmmword ptr [rbp + 0xf0], xmm0
0x140a982b1 0f1005f82c0402 movups xmm0, xmmword ptr [rip + 0x2042cf8]
0x140a982b8 0f1185e0000000 movups xmmword ptr [rbp + 0xe0], xmm0
0x140a982bf 48898500010000 mov qword ptr [rbp + 0x100], rax
0x140a982c6 48899508010000 mov qword ptr [rbp + 0x108], rdx
0x140a982cd 48c785c800000000000000 mov qword ptr [rbp + 0xc8], 0
0x140a982d8 48c785d000000004000000 mov qword ptr [rbp + 0xd0], 4
0x140a982e3 48c785d800000000000000 mov qword ptr [rbp + 0xd8], 0
0x140a982ee 48c7855802000000000000 mov qword ptr [rbp + 0x258], 0
0x140a982f9 48c7856002000008000000 mov qword ptr [rbp + 0x260], 8
0x140a98304 48c7856802000000000000 mov qword ptr [rbp + 0x268], 0
0x140a9830f 488b8508030000 mov rax, qword ptr [rbp + 0x308]
0x140a98316 4c8b30 mov r14, qword ptr [rax]
0x140a98319 488b5018 mov rdx, qword ptr [rax + 0x18]
0x140a9831d 66410f6f06 movdqa xmm0, xmmword ptr [r14]
0x140a98322 66440fd7e0 pmovmskb r12d, xmm0
0x140a98327 41f7d4 not r12d
0x140a9832a 498d7e10 lea rdi, [r14 + 0x10]
0x140a9832e 488b85f0020000 mov rax, qword ptr [rbp + 0x2f0]
0x140a98335 4883c020 add rax, 0x20
0x140a98339 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
0x140a98340 4885d2 test rdx, rdx
0x140a98343 0f84dd010000 je 0x140a98526
0x140a98349 664585e4 test r12w, r12w
0x140a9834d 7521 jne 0x140a98370
0x140a9834f 90 nop
0x140a98350 660f6f07 movdqa xmm0, xmmword ptr [rdi]
0x140a98354 66440fd7e0 pmovmskb r12d, xmm0
0x140a98359 4981c640ffffff add r14, -0xc0
0x140a98360 4883c710 add rdi, 0x10
0x140a98364 4181fcffff0000 cmp r12d, 0xffff
0x140a9836b 74e3 je 0x140a98350
0x140a9836d 41f7d4 not r12d
0x140a98370 4489e0 mov eax, r12d
0x140a98373 448d60ff lea r12d, [rax - 1]
0x140a98377 4121c4 and r12d, eax
0x140a9837a 48ffca dec rdx
0x140a9837d f30fbcc0 tzcnt eax, eax
0x140a98381 48f7d8 neg rax
0x140a98384 488d0440 lea rax, [rax + rax*2]
0x140a98388 418b4c86fc mov ecx, dword ptr [r14 + rax*4 - 4]
0x140a9838d 898d18020000 mov dword ptr [rbp + 0x218], ecx
0x140a98393 498b4486f4 mov rax, qword ptr [r14 + rax*4 - 0xc]
0x140a98398 48898510020000 mov qword ptr [rbp + 0x210], rax
0x140a9839f 0fb6c1 movzx eax, cl
0x140a983a2 83f802 cmp eax, 2
0x140a983a5 7399 jae 0x140a98340
0x140a983a7 488995f8020000 mov qword ptr [rbp + 0x2f8], rdx
0x140a983ae 4869c890030000 imul rcx, rax, 0x390
0x140a983b5 48038dd0020000 add rcx, qword ptr [rbp + 0x2d0]
0x140a983bc 8b9510020000 mov edx, dword ptr [rbp + 0x210]
0x140a983c2 448b8514020000 mov r8d, dword ptr [rbp + 0x214]
0x140a983c9 e872b50f00 call 0x140b93940
0x140a983ce 48898518030000 mov qword ptr [rbp + 0x318], rax
0x140a983d5 488b9518030000 mov rdx, qword ptr [rbp + 0x318]
0x140a983dc 4885d2 test rdx, rdx
0x140a983df 0f846c160000 je 0x140a99a51
0x140a983e5 488d8d90000000 lea rcx, [rbp + 0x90]
0x140a983ec e86fab0f00 call 0x140b92f60
0x140a983f1 66c785ee0200000001 mov word ptr [rbp + 0x2ee], 0x100
0x140a983fa c6851703000001 mov byte ptr [rbp + 0x317], 1
0x140a98401 31c0 xor eax, eax
0x140a98403 80bd1802000000 cmp byte ptr [rbp + 0x218], 0
0x140a9840a 0f94c0 sete al
0x140a9840d 488d9dee020000 lea rbx, [rbp + 0x2ee]
0x140a98414 488d8d17030000 lea rcx, [rbp + 0x317]
0x140a9841b 480f44d9 cmove rbx, rcx
0x140a9841f 48f7d8 neg rax
0x140a98422 4c8d3c03 lea r15, [rbx + rax]
0x140a98426 4983c702 add r15, 2
0x140a9842a 4c39fb cmp rbx, r15
0x140a9842d 488d8d90010000 lea rcx, [rbp + 0x190]
0x140a98434 0f84d9000000 je 0x140a98513
0x140a9843a 660f1f440000 nop word ptr [rax + rax]
0x140a98440 0fb603 movzx eax, byte ptr [rbx]
0x140a98443 888516030000 mov byte ptr [rbp + 0x316], al
0x140a98449 4869f090030000 imul rsi, rax, 0x390
0x140a98450 4803b5d0020000 add rsi, qword ptr [rbp + 0x2d0]
0x140a98457 488d8516030000 lea rax, [rbp + 0x316]
0x140a9845e 48894538 mov qword ptr [rbp + 0x38], rax
0x140a98462 488d8510020000 lea rax, [rbp + 0x210]
0x140a98469 48894540 mov qword ptr [rbp + 0x40], rax
0x140a9846d 488b8508030000 mov rax, qword ptr [rbp + 0x308]
0x140a98474 48894548 mov qword ptr [rbp + 0x48], rax
0x140a98478 48897550 mov qword ptr [rbp + 0x50], rsi
0x140a9847c 488b8518030000 mov rax, qword ptr [rbp + 0x318]
0x140a98483 48894558 mov qword ptr [rbp + 0x58], rax
0x140a98487 488b85f0020000 mov rax, qword ptr [rbp + 0x2f0]
0x140a9848e 48894560 mov qword ptr [rbp + 0x60], rax
0x140a98492 488d8578010000 lea rax, [rbp + 0x178]
0x140a98499 48894568 mov qword ptr [rbp + 0x68], rax
0x140a9849d 488d85c8000000 lea rax, [rbp + 0xc8]
0x140a984a4 48894570 mov qword ptr [rbp + 0x70], rax
0x140a984a8 488d8558020000 lea rax, [rbp + 0x258]
0x140a984af 48894578 mov qword ptr [rbp + 0x78], rax
0x140a984b3 488b85a0000000 mov rax, qword ptr [rbp + 0xa0]
0x140a984ba 48898580020000 mov qword ptr [rbp + 0x280], rax
0x140a984c1 f30f6f8590000000 movdqu xmm0, xmmword ptr [rbp + 0x90]
0x140a984c9 660f7f8570020000 movdqa xmmword ptr [rbp + 0x270], xmm0
0x140a984d1 488d9570020000 lea rdx, [rbp + 0x270]
0x140a984d8 4989cd mov r13, rcx
0x140a984db e840c70f00 call 0x140b94c20
0x140a984e0 488d4538 lea rax, [rbp + 0x38]
0x140a984e4 488985f0010000 mov qword ptr [rbp + 0x1f0], rax
0x140a984eb 4881c6c0000000 add rsi, 0xc0
0x140a984f2 4889f1 mov rcx, rsi
0x140a984f5 4c89ea mov rdx, r13
0x140a984f8 4531c0 xor r8d, r8d
0x140a984fb e8308cc200 call 0x1416c1130
0x140a98500 48ffc3 inc rbx
0x140a98503 4c39fb cmp rbx, r15
0x140a98506 488d8d90010000 lea rcx, [rbp + 0x190]
0x140a9850d 0f852dffffff jne 0x140a98440
0x140a98513 4c8bad28010000 mov r13, qword ptr [rbp + 0x128]
0x140a9851a 488b95f8020000 mov rdx, qword ptr [rbp + 0x2f8]
0x140a98521 e91afeffff jmp 0x140a98340
0x140a98526 488b9d58020000 mov rbx, qword ptr [rbp + 0x258]
0x140a9852d 488bbd60020000 mov rdi, qword ptr [rbp + 0x260]
0x140a98534 4c8bb568020000 mov r14, qword ptr [rbp + 0x268]
0x140a9853b 488b95e8000000 mov rdx, qword ptr [rbp + 0xe8]
0x140a98542 4885d2 test rdx, rdx
0x140a98545 742c je 0x140a98573
0x140a98547 4889d0 mov rax, rdx
0x140a9854a 48c1e004 shl rax, 4
0x140a9854e 4801c2 add rdx, rax
0x140a98551 4883c221 add rdx, 0x21
0x140a98555 741c je 0x140a98573
0x140a98557 488b8de0000000 mov rcx, qword ptr [rbp + 0xe0]
0x140a9855e 48f7d8 neg rax
0x140a98561 4801c1 add rcx, rax
0x140a98564 4883c1f0 add rcx, -0x10
0x140a98568 41b810000000 mov r8d, 0x10
0x140a9856e e8adb6b7ff call 0x140613c20
0x140a98573 488b95c8000000 mov rdx, qword ptr [rbp + 0xc8]
0x140a9857a 4885d2 test rdx, rdx
0x140a9857d 7416 je 0x140a98595
0x140a9857f 488b8dd0000000 mov rcx, qword ptr [rbp + 0xd0]
0x140a98586 48c1e202 shl rdx, 2
0x140a9858a 41b804000000 mov r8d, 4
0x140a98590 e88bb6b7ff call 0x140613c20
0x140a98595 488b8578010000 mov rax, qword ptr [rbp + 0x178]
0x140a9859c 4885c0 test rax, rax
0x140a9859f 741a je 0x140a985bb
0x140a985a1 488b8d80010000 mov rcx, qword ptr [rbp + 0x180]
0x140a985a8 48c1e002 shl rax, 2
0x140a985ac 488d14c0 lea rdx, [rax + rax*8]
0x140a985b0 41b804000000 mov r8d, 4
0x140a985b6 e865b6b7ff call 0x140613c20
0x140a985bb 80bd4001000000 cmp byte ptr [rbp + 0x140], 0
0x140a985c2 7461 je 0x140a98625
0x140a985c4 e867fb1700 call 0x140c18130
0x140a985c9 4889c1 mov rcx, rax
0x140a985cc 488b00 mov rax, qword ptr [rax]
0x140a985cf 4885c0 test rax, rax
0x140a985d2 740f je 0x140a985e3
0x140a985d4 83f801 cmp eax, 1
0x140a985d7 0f85d2140000 jne 0x140a99aaf
0x140a985dd 4883c108 add rcx, 8
0x140a985e1 eb13 jmp 0x140a985f6
0x140a985e3 31d2 xor edx, edx
0x140a985e5 e8e601e601 call 0x1428f87d0
0x140a985ea 4889c1 mov rcx, rax
0x140a985ed 4885c0 test rax, rax
0x140a985f0 0f84b9140000 je 0x140a99aaf
0x140a985f6 48833900 cmp qword ptr [rcx], 0
0x140a985fa 0f85d5130000 jne 0x140a999d5
0x140a98600 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x140a98607 48898d18030000 mov qword ptr [rbp + 0x318], rcx
0x140a9860e 4883c108 add rcx, 8
0x140a98612 488b5500 mov rdx, qword ptr [rbp]
0x140a98616 e8d5ea1700 call 0x140c170f0
0x140a9861b 488b8518030000 mov rax, qword ptr [rbp + 0x318]
0x140a98622 48ff00 inc qword ptr [rax]
0x140a98625 488bb5b0000000 mov rsi, qword ptr [rbp + 0xb0]
0x140a9862c 4889f1 mov rcx, rsi
0x140a9862f e8dc971e00 call 0x140c81e10
0x140a98634 4889f1 mov rcx, rsi
0x140a98637 4883c901 or rcx, 1
0x140a9863b e8d0971e00 call 0x140c81e10
0x140a98640 48c7c0f0ffffff mov rax, 0xfffffffffffffff0
0x140a98647 f0480fc106 lock xadd qword ptr [rsi], rax
0x140a9864c 4883e0f2 and rax, 0xfffffffffffffff2
0x140a98650 4883f812 cmp rax, 0x12
0x140a98654 0f8451130000 je 0x140a999ab
0x140a9865a 488b85c0000000 mov rax, qword ptr [rbp + 0xc0]
0x140a98661 48c785c000000000000000 mov qword ptr [rbp + 0xc0], 0
0x140a9866c 488d48f0 lea rcx, [rax - 0x10]
0x140a98670 48898d90010000 mov qword ptr [rbp + 0x190], rcx
0x140a98677 f048ff48f0 lock dec qword ptr [rax - 0x10]
0x140a9867c 750c jne 0x140a9868a
0x140a9867e 488d8d90010000 lea rcx, [rbp + 0x190]
0x140a98685 e806460c00 call 0x140b5cc90
0x140a9868a 48899db8020000 mov qword ptr [rbp + 0x2b8], rbx
0x140a98691 4889f9 mov rcx, rdi
0x140a98694 4c897530 mov qword ptr [rbp + 0x30], r14
0x140a98698 49c1e606 shl r14, 6
0x140a9869c 4901ce add r14, rcx
0x140a9869f 4c89b5c8020000 mov qword ptr [rbp + 0x2c8], r14
0x140a986a6 48898db0020000 mov qword ptr [rbp + 0x2b0], rcx
0x140a986ad 4c8bbd48020000 mov r15, qword ptr [rbp + 0x248]
0x140a986b4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a986c0 483b8dc8020000 cmp rcx, qword ptr [rbp + 0x2c8]
0x140a986c7 7415 je 0x140a986de
0x140a986c9 488d7140 lea rsi, [rcx + 0x40]
0x140a986cd c6852503000001 mov byte ptr [rbp + 0x325], 1
0x140a986d4 e877b18500 call 0x1412f3850
0x140a986d9 4889f1 mov rcx, rsi
0x140a986dc ebe2 jmp 0x140a986c0
0x140a986de c6852503000001 mov byte ptr [rbp + 0x325], 1
0x140a986e5 4c89f9 mov rcx, r15
0x140a986e8 e813b68400 call 0x1412e3d00
0x140a986ed 488b8590020000 mov rax, qword ptr [rbp + 0x290]
0x140a986f4 488b00 mov rax, qword ptr [rax]
0x140a986f7 488985a8020000 mov qword ptr [rbp + 0x2a8], rax
0x140a986fe 4883c020 add rax, 0x20
0x140a98702 48898538010000 mov qword ptr [rbp + 0x138], rax
0x140a98709 f3440f1035f2d1e801 movss xmm14, dword ptr [rip + 0x1e8d1f2]
0x140a98712 488b9db0020000 mov rbx, qword ptr [rbp + 0x2b0]
0x140a98719 483b9dc8020000 cmp rbx, qword ptr [rbp + 0x2c8]
0x140a98720 0f84720f0000 je 0x140a99698
0x140a98726 488d4b40 lea rcx, [rbx + 0x40]
0x140a9872a 488b13 mov rdx, qword ptr [rbx]
0x140a9872d 4889d0 mov rax, rdx
0x140a98730 48f7d8 neg rax
0x140a98733 0f805c0f0000 jo 0x140a99695
0x140a98739 48899540020000 mov qword ptr [rbp + 0x240], rdx
0x140a98740 48898d18030000 mov qword ptr [rbp + 0x318], rcx
0x140a98747 488b4308 mov rax, qword ptr [rbx + 8]
0x140a9874b 488985c0020000 mov qword ptr [rbp + 0x2c0], rax
0x140a98752 488b4310 mov rax, qword ptr [rbx + 0x10]
0x140a98756 48898530010000 mov qword ptr [rbp + 0x130], rax
0x140a9875d 488b4318 mov rax, qword ptr [rbx + 0x18]
0x140a98761 488985f8020000 mov qword ptr [rbp + 0x2f8], rax
0x140a98768 488b7b20 mov rdi, qword ptr [rbx + 0x20]
0x140a9876c 8b4330 mov eax, dword ptr [rbx + 0x30]
0x140a9876f 898560010000 mov dword ptr [rbp + 0x160], eax
0x140a98775 488b4328 mov rax, qword ptr [rbx + 0x28]
0x140a98779 48898558010000 mov qword ptr [rbp + 0x158], rax
0x140a98780 488b4334 mov rax, qword ptr [rbx + 0x34]
0x140a98784 48898568010000 mov qword ptr [rbp + 0x168], rax
0x140a9878b 8b433c mov eax, dword ptr [rbx + 0x3c]
0x140a9878e 898570010000 mov dword ptr [rbp + 0x170], eax
0x140a98794 0fb6056ec4b902 movzx eax, byte ptr [rip + 0x2b9c46e]
0x140a9879b b920000000 mov ecx, 0x20
0x140a987a0 ba08000000 mov edx, 8
0x140a987a5 e866b4b7ff call 0x140613c10
0x140a987aa 4885c0 test rax, rax
0x140a987ad 0f84ac120000 je 0x140a99a5f
0x140a987b3 4889c3 mov rbx, rax
0x140a987b6 48c70000000000 mov qword ptr [rax], 0
0x140a987bd 48c7400804000000 mov qword ptr [rax + 8], 4
0x140a987c5 48c7401000000000 mov qword ptr [rax + 0x10], 0
0x140a987cd 4889c1 mov rcx, rax
0x140a987d0 4883c118 add rcx, 0x18
0x140a987d4 c6401800 mov byte ptr [rax + 0x18], 0
0x140a987d8 498b5760 mov rdx, qword ptr [r15 + 0x60]
0x140a987dc eb04 jmp 0x140a987e2
0x140a987de 6690 nop
0x140a987e0 f390 pause
0x140a987e2 488b4208 mov rax, qword ptr [rdx + 8]
0x140a987e6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a987f0 4883f8ff cmp rax, -1
0x140a987f4 74ea je 0x140a987e0
0x140a987f6 4885c0 test rax, rax
0x140a987f9 0f8800110000 js 0x140a998ff
0x140a987ff 4c8d4001 lea r8, [rax + 1]
0x140a98803 f04c0fb14208 lock cmpxchg qword ptr [rdx + 8], r8
0x140a98809 75e5 jne 0x140a987f0
0x140a9880b 48899590000000 mov qword ptr [rbp + 0x90], rdx
0x140a98812 48899d98000000 mov qword ptr [rbp + 0x98], rbx
0x140a98819 31c0 xor eax, eax
0x140a9881b b201 mov dl, 1
0x140a9881d f00fb05318 lock cmpxchg byte ptr [rbx + 0x18], dl
0x140a98822 0f85000e0000 jne 0x140a99628
0x140a98828 c6852403000001 mov byte ptr [rbp + 0x324], 1
0x140a9882f c6852303000001 mov byte ptr [rbp + 0x323], 1
0x140a98836 e835941e00 call 0x140c81c70
0x140a9883b 488b4310 mov rax, qword ptr [rbx + 0x10]
0x140a9883f 4885c0 test rax, rax
0x140a98842 0f94c1 sete cl
0x140a98845 48c1e002 shl rax, 2
0x140a98849 48034308 add rax, qword ptr [rbx + 8]
0x140a9884d 4883c0fc add rax, -4
0x140a98851 0f94c2 sete dl
0x140a98854 08ca or dl, cl
0x140a98856 7418 je 0x140a98870
0x140a98858 498b4728 mov rax, qword ptr [r15 + 0x28]
0x140a9885c 4885c0 test rax, rax
0x140a9885f 0f8420120000 je 0x140a99a85
0x140a98865 8d48ff lea ecx, [rax - 1]
0x140a98868 eb0c jmp 0x140a98876
0x140a9886a 660f1f440000 nop word ptr [rax + rax]
0x140a98870 8b08 mov ecx, dword ptr [rax]
0x140a98872 498b4728 mov rax, qword ptr [r15 + 0x28]
0x140a98876 498b5720 mov rdx, qword ptr [r15 + 0x20]
0x140a9887a f3410f6f4730 movdqu xmm0, xmmword ptr [r15 + 0x30]
0x140a98880 f3410f6f4f40 movdqu xmm1, xmmword ptr [r15 + 0x40]
0x140a98886 f3410f6f5750 movdqu xmm2, xmmword ptr [r15 + 0x50]
0x140a9888c 898d88000000 mov dword ptr [rbp + 0x88], ecx
0x140a98892 48895538 mov qword ptr [rbp + 0x38], rdx
0x140a98896 48894540 mov qword ptr [rbp + 0x40], rax
0x140a9889a f30f7f4548 movdqu xmmword ptr [rbp + 0x48], xmm0
0x140a9889f 48895d58 mov qword ptr [rbp + 0x58], rbx
0x140a988a3 f30f7f4d60 movdqu xmmword ptr [rbp + 0x60], xmm1
0x140a988a8 4c897d70 mov qword ptr [rbp + 0x70], r15
0x140a988ac f30f7f5578 movdqu xmmword ptr [rbp + 0x78], xmm2
0x140a988b1 48c7857002000000000000 mov qword ptr [rbp + 0x270], 0
0x140a988bc 48c7857802000008000000 mov qword ptr [rbp + 0x278], 8
0x140a988c7 48c7858002000000000000 mov qword ptr [rbp + 0x280], 0
0x140a988d2 488b85f8020000 mov rax, qword ptr [rbp + 0x2f8]
0x140a988d9 48898590010000 mov qword ptr [rbp + 0x190], rax
0x140a988e0 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
0x140a988e7 4883781800 cmp qword ptr [rax + 0x18], 0
0x140a988ec 0f84de010000 je 0x140a98ad0
0x140a988f2 c6852703000001 mov byte ptr [rbp + 0x327], 1
0x140a988f9 c6852603000001 mov byte ptr [rbp + 0x326], 1
0x140a98900 488b8d38010000 mov rcx, qword ptr [rbp + 0x138]
0x140a98907 488d9590010000 lea rdx, [rbp + 0x190]
0x140a9890e e85d84b1ff call 0x1405b0d70
0x140a98913 488b95a8020000 mov rdx, qword ptr [rbp + 0x2a8]
0x140a9891a 488b0a mov rcx, qword ptr [rdx]
0x140a9891d 488b5208 mov rdx, qword ptr [rdx + 8]
0x140a98921 4989d0 mov r8, rdx
0x140a98924 4921c0 and r8, rax
0x140a98927 48c1e839 shr rax, 0x39
0x140a9892b 660f6ec0 movd xmm0, eax
0x140a9892f 660f60c0 punpcklbw xmm0, xmm0
0x140a98933 f20f70c000 pshuflw xmm0, xmm0, 0
0x140a98938 660f70c000 pshufd xmm0, xmm0, 0
0x140a9893d 488d8158fdffff lea rax, [rcx - 0x2a8]
0x140a98944 4531c9 xor r9d, r9d
0x140a98947 f3420f6f0c01 movdqu xmm1, xmmword ptr [rcx + r8]
0x140a9894d 660f6fd1 movdqa xmm2, xmm1
0x140a98951 660f74d0 pcmpeqb xmm2, xmm0
0x140a98955 66440fd7d2 pmovmskb r10d, xmm2
0x140a9895a 4585d2 test r10d, r10d
0x140a9895d 7431 je 0x140a98990
0x140a9895f f3450fbcda tzcnt r11d, r10d
0x140a98964 4d01c3 add r11, r8
0x140a98967 4921d3 and r11, rdx
0x140a9896a 49f7db neg r11
0x140a9896d 4d69dba8020000 imul r11, r11, 0x2a8
0x140a98974 488b9df8020000 mov rbx, qword ptr [rbp + 0x2f8]
0x140a9897b 4a391c18 cmp qword ptr [rax + r11], rbx
0x140a9897f 743f je 0x140a989c0
0x140a98981 458d5aff lea r11d, [r10 - 1]
0x140a98985 664521d3 and r11w, r10w
0x140a98989 4589da mov r10d, r11d
0x140a9898c 75d1 jne 0x140a9895f
0x140a9898e 6690 nop
0x140a98990 660f740df8cfe801 pcmpeqb xmm1, xmmword ptr [rip + 0x1e8cff8]
0x140a98998 66440fd7d1 pmovmskb r10d, xmm1
0x140a9899d 4585d2 test r10d, r10d
0x140a989a0 0f852a010000 jne 0x140a98ad0
0x140a989a6 4d01c8 add r8, r9
0x140a989a9 4983c010 add r8, 0x10
0x140a989ad 4983c110 add r9, 0x10
0x140a989b1 4921d0 and r8, rdx
0x140a989b4 eb91 jmp 0x140a98947
0x140a989b6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a989c0 4a83bc1970fdffff02 cmp qword ptr [rcx + r11 - 0x290], 2
0x140a989c9 0f82800f0000 jb 0x140a9994f
0x140a989cf f3420f10841990fdffff movss xmm0, dword ptr [rcx + r11 - 0x270]
0x140a989d9 0f2e05b07cec01 ucomiss xmm0, dword ptr [rip + 0x1ec7cb0]
0x140a989e0 0f86690f0000 jbe 0x140a9994f
0x140a989e6 4c01d9 add rcx, r11
0x140a989e9 8379b801 cmp dword ptr [rcx - 0x48], 1
0x140a989ed 0f87dd000000 ja 0x140a98ad0
0x140a989f3 4889bd90010000 mov qword ptr [rbp + 0x190], rdi
0x140a989fa 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
0x140a98a01 4883781800 cmp qword ptr [rax + 0x18], 0
0x140a98a06 0f84c4000000 je 0x140a98ad0
0x140a98a0c c6852703000001 mov byte ptr [rbp + 0x327], 1
0x140a98a13 c6852603000001 mov byte ptr [rbp + 0x326], 1
0x140a98a1a 488b8d38010000 mov rcx, qword ptr [rbp + 0x138]
0x140a98a21 488d9590010000 lea rdx, [rbp + 0x190]
0x140a98a28 e84383b1ff call 0x1405b0d70
0x140a98a2d 488b95a8020000 mov rdx, qword ptr [rbp + 0x2a8]
0x140a98a34 488b0a mov rcx, qword ptr [rdx]
0x140a98a37 488b5208 mov rdx, qword ptr [rdx + 8]
0x140a98a3b 4989d0 mov r8, rdx
0x140a98a3e 4921c0 and r8, rax
0x140a98a41 48c1e839 shr rax, 0x39
0x140a98a45 660f6ec0 movd xmm0, eax
0x140a98a49 660f60c0 punpcklbw xmm0, xmm0
0x140a98a4d f20f70c000 pshuflw xmm0, xmm0, 0
0x140a98a52 660f70c000 pshufd xmm0, xmm0, 0
0x140a98a57 488d8158fdffff lea rax, [rcx - 0x2a8]
0x140a98a5e 4531c9 xor r9d, r9d
0x140a98a61 f3420f6f0c01 movdqu xmm1, xmmword ptr [rcx + r8]
0x140a98a67 660f6fd1 movdqa xmm2, xmm1
0x140a98a6b 660f74d0 pcmpeqb xmm2, xmm0
0x140a98a6f 66440fd7d2 pmovmskb r10d, xmm2
0x140a98a74 4585d2 test r10d, r10d
0x140a98a77 742c je 0x140a98aa5
0x140a98a79 f3450fbcda tzcnt r11d, r10d
0x140a98a7e 4d01c3 add r11, r8
0x140a98a81 4921d3 and r11, rdx
0x140a98a84 49f7db neg r11
0x140a98a87 4d69dba8020000 imul r11, r11, 0x2a8
0x140a98a8e 4a393c18 cmp qword ptr [rax + r11], rdi
0x140a98a92 0f84550b0000 je 0x140a995ed
0x140a98a98 458d5aff lea r11d, [r10 - 1]
0x140a98a9c 664521d3 and r11w, r10w
0x140a98aa0 4589da mov r10d, r11d
0x140a98aa3 75d4 jne 0x140a98a79
0x140a98aa5 660f740de3cee801 pcmpeqb xmm1, xmmword ptr [rip + 0x1e8cee3]
0x140a98aad 66440fd7d1 pmovmskb r10d, xmm1
0x140a98ab2 4585d2 test r10d, r10d
0x140a98ab5 7519 jne 0x140a98ad0
0x140a98ab7 4d01c8 add r8, r9
0x140a98aba 4983c010 add r8, 0x10
0x140a98abe 4983c110 add r9, 0x10
0x140a98ac2 4921d0 and r8, rdx
0x140a98ac5 eb9a jmp 0x140a98a61
0x140a98ac7 660f1f840000000000 nop word ptr [rax + rax]
0x140a98ad0 c7850403000000000000 mov dword ptr [rbp + 0x304], 0
0x140a98ada 488b8530010000 mov rax, qword ptr [rbp + 0x130]
0x140a98ae1 488d04c500000000 lea rax, [rax*8]
0x140a98ae9 488d1480 lea rdx, [rax + rax*4]
0x140a98aed 488b8dc0020000 mov rcx, qword ptr [rbp + 0x2c0]
0x140a98af4 4801ca add rdx, rcx
0x140a98af7 48899530020000 mov qword ptr [rbp + 0x230], rdx
0x140a98afe 8d976079fe7f lea edx, [rdi + 0x7ffe7960]
0x140a98b04 81ca00000080 or edx, 0x80000000
0x140a98b0a 89950c020000 mov dword ptr [rbp + 0x20c], edx
0x140a98b10 488b95f8020000 mov rdx, qword ptr [rbp + 0x2f8]
0x140a98b17 448daa6079fe7f lea r13d, [rdx + 0x7ffe7960]
0x140a98b1e 4181cd00000080 or r13d, 0x80000000
0x140a98b25 488d0480 lea rax, [rax + rax*4]
0x140a98b29 4883c0d8 add rax, -0x28
0x140a98b2d 48898538020000 mov qword ptr [rbp + 0x238], rax
0x140a98b34 488d5930 lea rbx, [rcx + 0x30]
0x140a98b38 4531ff xor r15d, r15d
0x140a98b3b 4889c8 mov rax, rcx
0x140a98b3e eb1a jmp 0x140a98b5a
0x140a98b40 48838538020000d8 add qword ptr [rbp + 0x238], -0x28
0x140a98b48 488b9d20010000 mov rbx, qword ptr [rbp + 0x120]
0x140a98b4f 4883c328 add rbx, 0x28
0x140a98b53 488b8528020000 mov rax, qword ptr [rbp + 0x228]
0x140a98b5a 483b8530020000 cmp rax, qword ptr [rbp + 0x230]
0x140a98b61 0f84f9080000 je 0x140a99460
0x140a98b67 488d4828 lea rcx, [rax + 0x28]
0x140a98b6b 48898d28020000 mov qword ptr [rbp + 0x228], rcx
0x140a98b72 488b08 mov rcx, qword ptr [rax]
0x140a98b75 48898d20020000 mov qword ptr [rbp + 0x220], rcx
0x140a98b7c 48f7d9 neg rcx
0x140a98b7f 0f807b080000 jo 0x140a99400
0x140a98b85 48899d20010000 mov qword ptr [rbp + 0x120], rbx
0x140a98b8c 488b4808 mov rcx, qword ptr [rax + 8]
0x140a98b90 4c8b4818 mov r9, qword ptr [rax + 0x18]
0x140a98b94 4d85c9 test r9, r9
0x140a98b97 48898da0020000 mov qword ptr [rbp + 0x2a0], rcx
0x140a98b9e 0f846d0e0000 je 0x140a99a11
0x140a98ba4 488b4010 mov rax, qword ptr [rax + 0x10]
0x140a98ba8 31db xor ebx, ebx
0x140a98baa 4c8b9520020000 mov r10, qword ptr [rbp + 0x220]
0x140a98bb1 4c39d0 cmp rax, r10
0x140a98bb4 4c89d2 mov rdx, r10
0x140a98bb7 480f42d3 cmovb rdx, rbx
0x140a98bbb 4829d0 sub rax, rdx
0x140a98bbe 488d1440 lea rdx, [rax + rax*2]
0x140a98bc2 4c8d1cd1 lea r11, [rcx + rdx*8]
0x140a98bc6 f20f100cd1 movsd xmm1, qword ptr [rcx + rdx*8]
0x140a98bcb f30f1044d108 movss xmm0, dword ptr [rcx + rdx*8 + 8]
0x140a98bd1 4d89d0 mov r8, r10
0x140a98bd4 4929c0 sub r8, rax
0x140a98bd7 4c01c8 add rax, r9
0x140a98bda 4d39c8 cmp r8, r9
0x140a98bdd 490f42c2 cmovb rax, r10
0x140a98be1 f30f1054d10c movss xmm2, dword ptr [rcx + rdx*8 + 0xc]
0x140a98be7 4d29c1 sub r9, r8
0x140a98bea 4c0f42cb cmovb r9, rbx
0x140a98bee f20f105cd110 movsd xmm3, qword ptr [rcx + rdx*8 + 0x10]
0x140a98bf4 488d0440 lea rax, [rax + rax*2]
0x140a98bf8 488d14c1 lea rdx, [rcx + rax*8]
0x140a98bfc 4b8d0449 lea rax, [r9 + r9*2]
0x140a98c00 4c8d04c1 lea r8, [rcx + rax*8]
0x140a98c04 0f57e4 xorps xmm4, xmm4
0x140a98c07 b001 mov al, 1
0x140a98c09 4989cc mov r12, rcx
0x140a98c0c 4c894528 mov qword ptr [rbp + 0x28], r8
0x140a98c10 4d89c2 mov r10, r8
0x140a98c13 4c895d18 mov qword ptr [rbp + 0x18], r11
0x140a98c17 48895520 mov qword ptr [rbp + 0x20], rdx
0x140a98c1b 4989d0 mov r8, rdx
0x140a98c1e 0f57ed xorps xmm5, xmm5
0x140a98c21 f30f11ad08030000 movss dword ptr [rbp + 0x308], xmm5
0x140a98c29 eb33 jmp 0x140a98c5e
0x140a98c2b 0f1f440000 nop dword ptr [rax + rax]
0x140a98c30 31c0 xor eax, eax
0x140a98c32 0f2e2d577aec01 ucomiss xmm5, dword ptr [rip + 0x1ec7a57]
0x140a98c39 0f28e5 movaps xmm4, xmm5
0x140a98c3c 488b8da0020000 mov rcx, qword ptr [rbp + 0x2a0]
0x140a98c43 4c8ba598020000 mov r12, qword ptr [rbp + 0x298]
0x140a98c4a 4c8b9d10010000 mov r11, qword ptr [rbp + 0x110]
0x140a98c51 4c8b8518010000 mov r8, qword ptr [rbp + 0x118]
0x140a98c58 0f821a0a0000 jb 0x140a99678
0x140a98c5e 4d39c3 cmp r11, r8
0x140a98c61 741d je 0x140a98c80
0x140a98c63 4d89e1 mov r9, r12
0x140a98c66 4c89d2 mov rdx, r10
0x140a98c69 4d89c2 mov r10, r8
0x140a98c6c 4d89dc mov r12, r11
0x140a98c6f eb1e jmp 0x140a98c8f
0x140a98c71 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a98c80 4d89d9 mov r9, r11
0x140a98c83 4c89da mov rdx, r11
0x140a98c86 4d39d4 cmp r12, r10
0x140a98c89 0f84a4040000 je 0x140a99133
0x140a98c8f 4d8d5c2418 lea r11, [r12 + 0x18]
0x140a98c94 a801 test al, 1
0x140a98c96 7418 je 0x140a98cb0
0x140a98c98 4d39d3 cmp r11, r10
0x140a98c9b 7436 je 0x140a98cd3
0x140a98c9d 4c89c8 mov rax, r9
0x140a98ca0 4989d0 mov r8, rdx
0x140a98ca3 4c89d2 mov rdx, r10
0x140a98ca6 4d89d9 mov r9, r11
0x140a98ca9 eb37 jmp 0x140a98ce2
0x140a98cab 0f1f440000 nop dword ptr [rax + rax]
0x140a98cb0 4c899d10010000 mov qword ptr [rbp + 0x110], r11
0x140a98cb7 4c89e0 mov rax, r12
0x140a98cba 4989f4 mov r12, rsi
0x140a98cbd 4889c6 mov rsi, rax
0x140a98cc0 4c899518010000 mov qword ptr [rbp + 0x118], r10
0x140a98cc7 4989d2 mov r10, rdx
0x140a98cca 4c898d98020000 mov qword ptr [rbp + 0x298], r9
0x140a98cd1 eb37 jmp 0x140a98d0a
0x140a98cd3 4c89d8 mov rax, r11
0x140a98cd6 4d89d0 mov r8, r10
0x140a98cd9 4939d1 cmp r9, rdx
0x140a98cdc 0f8451040000 je 0x140a99133
0x140a98ce2 4d85e4 test r12, r12
0x140a98ce5 0f8445040000 je 0x140a99130
0x140a98ceb 498d4918 lea rcx, [r9 + 0x18]
0x140a98cef 48898d10010000 mov qword ptr [rbp + 0x110], rcx
0x140a98cf6 4c89ce mov rsi, r9
0x140a98cf9 48899518010000 mov qword ptr [rbp + 0x118], rdx
0x140a98d00 4d89c2 mov r10, r8
0x140a98d03 48898598020000 mov qword ptr [rbp + 0x298], rax
0x140a98d0a f3410f102c24 movss xmm5, dword ptr [r12]
0x140a98d10 f3410f10742404 movss xmm6, dword ptr [r12 + 4]
0x140a98d17 f3410f107c2408 movss xmm7, dword ptr [r12 + 8]
0x140a98d1e f30f5c2e subss xmm5, dword ptr [rsi]
0x140a98d22 f30f5c7604 subss xmm6, dword ptr [rsi + 4]
0x140a98d27 f30f5c7e08 subss xmm7, dword ptr [rsi + 8]
0x140a98d2c f30f59ed mulss xmm5, xmm5
0x140a98d30 f30f59f6 mulss xmm6, xmm6
0x140a98d34 f30f58f5 addss xmm6, xmm5
0x140a98d38 f30f59ff mulss xmm7, xmm7
0x140a98d3c f30f58fe addss xmm7, xmm6
0x140a98d40 f30f51ff sqrtss xmm7, xmm7
0x140a98d44 450f57ed xorps xmm13, xmm13
0x140a98d48 4d89d6 mov r14, r10
0x140a98d4b f30f11bdf0020000 movss dword ptr [rbp + 0x2f0], xmm7
0x140a98d53 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a98d60 0f28ef movaps xmm5, xmm7
0x140a98d63 f3410f5ced subss xmm5, xmm13
0x140a98d68 f30f58ec addss xmm5, xmm4
0x140a98d6c 0f2e2dfdcae801 ucomiss xmm5, dword ptr [rip + 0x1e8cafd]
0x140a98d73 0f86b7feffff jbe 0x140a98c30
0x140a98d79 450f57c9 xorps xmm9, xmm9
0x140a98d7d 440f28dc movaps xmm11, xmm4
0x140a98d81 f3440f581d5e13ec01 addss xmm11, dword ptr [rip + 0x1ec135e]
0x140a98d8a f3450f107c240c movss xmm15, dword ptr [r12 + 0xc]
0x140a98d91 f3410f58d7 addss xmm2, xmm15
0x140a98d96 0f28ea movaps xmm5, xmm2
0x140a98d99 f30f59ea mulss xmm5, xmm2
0x140a98d9d f2450f10442410 movsd xmm8, qword ptr [r12 + 0x10]
0x140a98da4 410f58d8 addps xmm3, xmm8
0x140a98da8 0f28f3 movaps xmm6, xmm3
0x140a98dab 0f59f3 mulps xmm6, xmm3
0x140a98dae f30f58ee addss xmm5, xmm6
0x140a98db2 f30f16f6 movshdup xmm6, xmm6
0x140a98db6 f30f58f5 addss xmm6, xmm5
0x140a98dba f30f51f6 sqrtss xmm6, xmm6
0x140a98dbe 410f28ee movaps xmm5, xmm14
0x140a98dc2 f30f5eee divss xmm5, xmm6
0x140a98dc6 660f7ee8 movd eax, xmm5
0x140a98dca 85c0 test eax, eax
0x140a98dcc 0f99c1 setns cl
0x140a98dcf 89c2 mov edx, eax
0x140a98dd1 81e2ffffff7f and edx, 0x7fffffff
0x140a98dd7 81c2000080ff add edx, 0xff800000
0x140a98ddd 81fa0000007f cmp edx, 0x7f000000
0x140a98de3 0f92c2 setb dl
0x140a98de6 20ca and dl, cl
0x140a98de8 ffc8 dec eax
0x140a98dea 3dffff7f00 cmp eax, 0x7fffff
0x140a98def 0f92c0 setb al
0x140a98df2 08d0 or al, dl
0x140a98df4 440f2985d0020000 movaps xmmword ptr [rbp + 0x2d0], xmm8
0x140a98dfc 7512 jne 0x140a98e10
0x140a98dfe 0f57d2 xorps xmm2, xmm2
0x140a98e01 eb11 jmp 0x140a98e14
0x140a98e03 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a98e10 f30f59d5 mulss xmm2, xmm5
0x140a98e14 f30f103554cae801 movss xmm6, dword ptr [rip + 0x1e8ca54]
0x140a98e1c f30f5cf4 subss xmm6, xmm4
0x140a98e20 f3410f5ff1 maxss xmm6, xmm9
0x140a98e25 f3440f58ee addss xmm13, xmm6
0x140a98e2a 450f28c5 movaps xmm8, xmm13
0x140a98e2e f3440f5ec7 divss xmm8, xmm7
0x140a98e33 410f28e6 movaps xmm4, xmm14
0x140a98e37 f3410f5ce0 subss xmm4, xmm8
0x140a98e3c f30f12f4 movsldup xmm6, xmm4
0x140a98e40 f3410f59642408 mulss xmm4, dword ptr [r12 + 8]
0x140a98e47 f3450f12c8 movsldup xmm9, xmm8
0x140a98e4c f3440f594608 mulss xmm8, dword ptr [rsi + 8]
0x140a98e52 f3440f58c4 addss xmm8, xmm4
0x140a98e57 410f28e0 movaps xmm4, xmm8
0x140a98e5b f30f5ce0 subss xmm4, xmm0
0x140a98e5f 440f28d4 movaps xmm10, xmm4
0x140a98e63 f3440f59d4 mulss xmm10, xmm4
0x140a98e68 f2410f103c24 movsd xmm7, qword ptr [r12]
0x140a98e6e f2440f1026 movsd xmm12, qword ptr [rsi]
0x140a98e73 0f59fe mulps xmm7, xmm6
0x140a98e76 450f59e1 mulps xmm12, xmm9
0x140a98e7a 440f58e7 addps xmm12, xmm7
0x140a98e7e 450f28cc movaps xmm9, xmm12
0x140a98e82 440f5cc9 subps xmm9, xmm1
0x140a98e86 410f28f1 movaps xmm6, xmm9
0x140a98e8a 410f59f1 mulps xmm6, xmm9
0x140a98e8e f30f16fe movshdup xmm7, xmm6
0x140a98e92 f30f58fe addss xmm7, xmm6
0x140a98e96 f3410f58fa addss xmm7, xmm10
0x140a98e9b f30f51ff sqrtss xmm7, xmm7
0x140a98e9f 410f28f6 movaps xmm6, xmm14
0x140a98ea3 f30f5ef7 divss xmm6, xmm7
0x140a98ea7 660f7ef1 movd ecx, xmm6
0x140a98eab 85c9 test ecx, ecx
0x140a98ead 0f99c2 setns dl
0x140a98eb0 4189c8 mov r8d, ecx
0x140a98eb3 4181e0ffffff7f and r8d, 0x7fffffff
0x140a98eba 4181c0000080ff add r8d, 0xff800000
0x140a98ec1 4181f80000007f cmp r8d, 0x7f000000
0x140a98ec8 410f92c0 setb r8b
0x140a98ecc 4120d0 and r8b, dl
0x140a98ecf ffc9 dec ecx
0x140a98ed1 81f9ffff7f00 cmp ecx, 0x7fffff
0x140a98ed7 0f92c1 setb cl
0x140a98eda 4408c1 or cl, r8b
0x140a98edd 7511 jne 0x140a98ef0
0x140a98edf 0f57e4 xorps xmm4, xmm4
0x140a98ee2 eb10 jmp 0x140a98ef4
0x140a98ee4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a98ef0 f30f59e6 mulss xmm4, xmm6
0x140a98ef4 f30f12ed movsldup xmm5, xmm5
0x140a98ef8 0f59dd mulps xmm3, xmm5
0x140a98efb 0fb6c0 movzx eax, al
0x140a98efe 660f6ee8 movd xmm5, eax
0x140a98f02 66440f70f550 pshufd xmm14, xmm5, 0x50
0x140a98f08 66410f72f61f pslld xmm14, 0x1f
0x140a98f0e 66410f72e61f psrad xmm14, 0x1f
0x140a98f14 66440fdbf3 pand xmm14, xmm3
0x140a98f19 66450f70d6f5 pshufd xmm10, xmm14, 0xf5
0x140a98f1f f30f12de movsldup xmm3, xmm6
0x140a98f23 440f59cb mulps xmm9, xmm3
0x140a98f27 0fb6c1 movzx eax, cl
0x140a98f2a 660f6ed8 movd xmm3, eax
0x140a98f2e 660f70db50 pshufd xmm3, xmm3, 0x50
0x140a98f33 660f72f31f pslld xmm3, 0x1f
0x140a98f38 660f72e31f psrad xmm3, 0x1f
0x140a98f3d 66410fdbd9 pand xmm3, xmm9
0x140a98f42 410f58cc addps xmm1, xmm12
0x140a98f46 f3410f58c0 addss xmm0, xmm8
0x140a98f4b 0f590d8ec9e801 mulps xmm1, xmmword ptr [rip + 0x1e8c98e]
0x140a98f52 f30f590516c9e801 mulss xmm0, dword ptr [rip + 0x1e8c916]
0x140a98f5a 0f28fa movaps xmm7, xmm2
0x140a98f5d 660fefed pxor xmm5, xmm5
0x140a98f61 f30f59fd mulss xmm7, xmm5
0x140a98f65 f3410f58fe addss xmm7, xmm14
0x140a98f6a 66410f6ff2 movdqa xmm6, xmm10
0x140a98f6f f30f59f5 mulss xmm6, xmm5
0x140a98f73 f30f58f7 addss xmm6, xmm7
0x140a98f77 0f2eb508030000 ucomiss xmm6, dword ptr [rbp + 0x308]
0x140a98f7e 7626 jbe 0x140a98fa6
0x140a98f80 b301 mov bl, 1
0x140a98f82 f30f11a508020000 movss dword ptr [rbp + 0x208], xmm4
0x140a98f8a f30f118590020000 movss dword ptr [rbp + 0x290], xmm0
0x140a98f92 f30f11b508030000 movss dword ptr [rbp + 0x308], xmm6
0x140a98f9a 0f298d40010000 movaps xmmword ptr [rbp + 0x140], xmm1
0x140a98fa1 660f7f5dc0 movdqa xmmword ptr [rbp - 0x40], xmm3
0x140a98fa6 66440f6fcb movdqa xmm9, xmm3
0x140a98fab 440f590d2dc9e801 mulps xmm9, xmmword ptr [rip + 0x1e8c92d]
0x140a98fb3 0f28f4 movaps xmm6, xmm4
0x140a98fb6 f30f5935b2c8e801 mulss xmm6, dword ptr [rip + 0x1e8c8b2]
0x140a98fbe 0f28fa movaps xmm7, xmm2
0x140a98fc1 0f16fe movlhps xmm7, xmm6
0x140a98fc4 440fc6cf24 shufps xmm9, xmm7, 0x24
0x140a98fc9 440f590d2f160402 mulps xmm9, xmmword ptr [rip + 0x204162f]
0x140a98fd1 f30f16f3 movshdup xmm6, xmm3
0x140a98fd5 0f28fe movaps xmm7, xmm6
0x140a98fd8 f3410f59fa mulss xmm7, xmm10
0x140a98fdd 410f28ee movaps xmm5, xmm14
0x140a98fe1 f30f59ec mulss xmm5, xmm4
0x140a98fe5 f30f5cfd subss xmm7, xmm5
0x140a98fe9 f30f59e2 mulss xmm4, xmm2
0x140a98fed 0f28eb movaps xmm5, xmm3
0x140a98ff0 f3410f59ea mulss xmm5, xmm10
0x140a98ff5 f30f5ce5 subss xmm4, xmm5
0x140a98ff9 0f16e7 movlhps xmm4, xmm7
0x140a98ffc f3410f59de mulss xmm3, xmm14
0x140a99001 f30f59f2 mulss xmm6, xmm2
0x140a99005 f30f5cde subss xmm3, xmm6
0x140a99009 440fc6f424 shufps xmm14, xmm4, 0x24
0x140a9900e 440f5935fa150402 mulps xmm14, xmmword ptr [rip + 0x20415fa]
0x140a99016 f30f591dbac8e801 mulss xmm3, dword ptr [rip + 0x1e8c8ba]
0x140a9901e 41f6c701 test r15b, 1
0x140a99022 752c jne 0x140a99050
0x140a99024 488b85f8020000 mov rax, qword ptr [rbp + 0x2f8]
0x140a9902b 8d886079feff lea ecx, [rax - 0x186a0]
0x140a99031 4489e8 mov eax, r13d
0x140a99034 83f904 cmp ecx, 4
0x140a99037 7232 jb 0x140a9906b
0x140a99039 488b8df8020000 mov rcx, qword ptr [rbp + 0x2f8]
0x140a99040 89c8 mov eax, ecx
0x140a99042 81f9ab860100 cmp ecx, 0x186ab
0x140a99048 7521 jne 0x140a9906b
0x140a9904a 4489e8 mov eax, r13d
0x140a9904d eb1c jmp 0x140a9906b
0x140a9904f 90 nop
0x140a99050 8d876079feff lea eax, [rdi - 0x186a0]
0x140a99056 83f804 cmp eax, 4
0x140a99059 720a jb 0x140a99065
0x140a9905b 89f8 mov eax, edi
0x140a9905d 81ffab860100 cmp edi, 0x186ab
0x140a99063 7506 jne 0x140a9906b
0x140a99065 8b850c020000 mov eax, dword ptr [rbp + 0x20c]
0x140a9906b 440f298d90010000 movaps xmmword ptr [rbp + 0x190], xmm9
0x140a99073 440f29b5a0010000 movaps xmmword ptr [rbp + 0x1a0], xmm14
0x140a9907b f30f119db0010000 movss dword ptr [rbp + 0x1b0], xmm3
0x140a99083 0f138db4010000 movlps qword ptr [rbp + 0x1b4], xmm1
0x140a9908a f30f1185bc010000 movss dword ptr [rbp + 0x1bc], xmm0
0x140a99092 488d8dc0010000 lea rcx, [rbp + 0x1c0]
0x140a99099 0f57c0 xorps xmm0, xmm0
0x140a9909c 0f2901 movaps xmmword ptr [rcx], xmm0
0x140a9909f f20f10053935ff01 movsd xmm0, qword ptr [rip + 0x1ff3539]
0x140a990a7 f20f1185d0010000 movsd qword ptr [rbp + 0x1d0], xmm0
0x140a990af c785d80100000000c8c2 mov dword ptr [rbp + 0x1d8], 0xc2c80000
0x140a990b9 4489bddc010000 mov dword ptr [rbp + 0x1dc], r15d
0x140a990c0 8985e0010000 mov dword ptr [rbp + 0x1e0], eax
0x140a990c6 f20f100502f90202 movsd xmm0, qword ptr [rip + 0x202f902]
0x140a990ce f20f1185e4010000 movsd qword ptr [rbp + 0x1e4], xmm0
0x140a990d6 c785ec01000006000000 mov dword ptr [rbp + 0x1ec], 6
0x140a990e0 488d4d38 lea rcx, [rbp + 0x38]
0x140a990e4 488d9590010000 lea rdx, [rbp + 0x190]
0x140a990eb e860b38400 call 0x1412e4450
0x140a990f0 f3440f10350bc8e801 movss xmm14, dword ptr [rip + 0x1e8c80b]
0x140a990f9 f3440f5f1d8e75ec01 maxss xmm11, dword ptr [rip + 0x1ec758e]
0x140a99102 41ffc7 inc r15d
0x140a99105 410f28c0 movaps xmm0, xmm8
0x140a99109 410f28d7 movaps xmm2, xmm15
0x140a9910d 410f28e3 movaps xmm4, xmm11
0x140a99111 410f28cc movaps xmm1, xmm12
0x140a99115 0f289dd0020000 movaps xmm3, xmmword ptr [rbp + 0x2d0]
0x140a9911c 4d89f2 mov r10, r14
0x140a9911f f30f10bdf0020000 movss xmm7, dword ptr [rbp + 0x2f0]
0x140a99127 e934fcffff jmp 0x140a98d60
0x140a9912c 0f1f4000 nop dword ptr [rax]
0x140a99130 4c89ce mov rsi, r9
0x140a99133 f30f108508030000 movss xmm0, dword ptr [rbp + 0x308]
0x140a9913b 0f2e054a37fe01 ucomiss xmm0, dword ptr [rip + 0x1fe374a]
0x140a99142 0f97c0 seta al
0x140a99145 848504030000 test byte ptr [rbp + 0x304], al
0x140a9914b 0f8484020000 je 0x140a993d5
0x140a99151 f6c301 test bl, 1
0x140a99154 0f84d4080000 je 0x140a99a2e
0x140a9915a f30f108590020000 movss xmm0, dword ptr [rbp + 0x290]
0x140a99162 f30f59050236e901 mulss xmm0, dword ptr [rip + 0x1e93602]
0x140a9916a f30f2cc0 cvttss2si eax, xmm0
0x140a9916e f30f100d46b8f001 movss xmm1, dword ptr [rip + 0x1f0b846]
0x140a99176 0f2ec1 ucomiss xmm0, xmm1
0x140a99179 41b9ffffff7f mov r9d, 0x7fffffff
0x140a9917f 410f47c1 cmova eax, r9d
0x140a99183 0f2ec0 ucomiss xmm0, xmm0
0x140a99186 41b800000000 mov r8d, 0
0x140a9918c 410f4ac0 cmovp eax, r8d
0x140a99190 0f288540010000 movaps xmm0, xmmword ptr [rbp + 0x140]
0x140a99197 0f5905a268fe01 mulps xmm0, xmmword ptr [rip + 0x1fe68a2]
0x140a9919e f30f2cc8 cvttss2si ecx, xmm0
0x140a991a2 0f2ec1 ucomiss xmm0, xmm1
0x140a991a5 410f47c9 cmova ecx, r9d
0x140a991a9 0f2ec0 ucomiss xmm0, xmm0
0x140a991ac 410f4ac8 cmovp ecx, r8d
0x140a991b0 f30f16c0 movshdup xmm0, xmm0
0x140a991b4 f30f2cd0 cvttss2si edx, xmm0
0x140a991b8 0f2ec1 ucomiss xmm0, xmm1
0x140a991bb 660f6ec9 movd xmm1, ecx
0x140a991bf 410f47d1 cmova edx, r9d
0x140a991c3 0f2ec0 ucomiss xmm0, xmm0
0x140a991c6 410f4ad0 cmovp edx, r8d
0x140a991ca 660f6ec2 movd xmm0, edx
0x140a991ce 660f62c8 punpckldq xmm1, xmm0
0x140a991d2 660fd68d58020000 movq qword ptr [rbp + 0x258], xmm1
0x140a991da 898560020000 mov dword ptr [rbp + 0x260], eax
0x140a991e0 488d8d58020000 lea rcx, [rbp + 0x258]
0x140a991e7 e8b42f9c00 call 0x14145c1a0
0x140a991ec 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
0x140a991f3 488b4518 mov rax, qword ptr [rbp + 0x18]
0x140a991f7 48898590010000 mov qword ptr [rbp + 0x190], rax
0x140a991fe 488b4520 mov rax, qword ptr [rbp + 0x20]
0x140a99202 48898598010000 mov qword ptr [rbp + 0x198], rax
0x140a99209 488b85a0020000 mov rax, qword ptr [rbp + 0x2a0]
0x140a99210 488985a0010000 mov qword ptr [rbp + 0x1a0], rax
0x140a99217 488b4528 mov rax, qword ptr [rbp + 0x28]
0x140a9921b 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
0x140a99222 488d8dc8000000 lea rcx, [rbp + 0xc8]
0x140a99229 488d9590010000 lea rdx, [rbp + 0x190]
0x140a99230 4c8d0521bc1c02 lea r8, [rip + 0x21cbc21]
0x140a99237 e8d4688600 call 0x1412ffb10
0x140a9923c 4c8ba5c8000000 mov r12, qword ptr [rbp + 0xc8]
0x140a99243 488b9dd0000000 mov rbx, qword ptr [rbp + 0xd0]
0x140a9924a 488b85d8000000 mov rax, qword ptr [rbp + 0xd8]
0x140a99251 48898508030000 mov qword ptr [rbp + 0x308], rax
0x140a99258 488d048500000000 lea rax, [rax*4]
0x140a99260 488d1440 lea rdx, [rax + rax*2]
0x140a99264 b904000000 mov ecx, 4
0x140a99269 4989d6 mov r14, rdx
0x140a9926c e80f2f2e00 call 0x140d7c180
0x140a99271 4885d2 test rdx, rdx
0x140a99274 488995f0020000 mov qword ptr [rbp + 0x2f0], rdx
0x140a9927b 48898598020000 mov qword ptr [rbp + 0x298], rax
0x140a99282 7412 je 0x140a99296
0x140a99284 0fb60d7eb9b902 movzx ecx, byte ptr [rip + 0x2b9b97e]
0x140a9928b 4889d1 mov rcx, rdx
0x140a9928e 4889c2 mov rdx, rax
0x140a99291 e87aa9b7ff call 0x140613c10
0x140a99296 4885c0 test rax, rax
0x140a99299 0f849d070000 je 0x140a99a3c
0x140a9929f 48c70001000000 mov qword ptr [rax], 1
0x140a992a6 48c7400801000000 mov qword ptr [rax + 8], 1
0x140a992ae 488985f0020000 mov qword ptr [rbp + 0x2f0], rax
0x140a992b5 488d4810 lea rcx, [rax + 0x10]
0x140a992b9 4889da mov rdx, rbx
0x140a992bc 4d89f0 mov r8, r14
0x140a992bf e83ca9e801 call 0x142923c00
0x140a992c4 4d85e4 test r12, r12
0x140a992c7 7416 je 0x140a992df
0x140a992c9 49c1e402 shl r12, 2
0x140a992cd 4b8d1464 lea rdx, [r12 + r12*2]
0x140a992d1 41b804000000 mov r8d, 4
0x140a992d7 4889d9 mov rcx, rbx
0x140a992da e841a9b7ff call 0x140613c20
0x140a992df 48b8dd0b57ab0804fa08 movabs rax, 0x8fa0408ab570bdd
0x140a992e9 488b8dd0020000 mov rcx, qword ptr [rbp + 0x2d0]
0x140a992f0 4831c1 xor rcx, rax
0x140a992f3 0f288540010000 movaps xmm0, xmmword ptr [rbp + 0x140]
0x140a992fa 0f1385b8010000 movlps qword ptr [rbp + 0x1b8], xmm0
0x140a99301 f30f108590020000 movss xmm0, dword ptr [rbp + 0x290]
0x140a99309 f30f1185c0010000 movss dword ptr [rbp + 0x1c0], xmm0
0x140a99311 0f2845c0 movaps xmm0, xmmword ptr [rbp - 0x40]
0x140a99315 0f1385c4010000 movlps qword ptr [rbp + 0x1c4], xmm0
0x140a9931c f30f108508020000 movss xmm0, dword ptr [rbp + 0x208]
0x140a99324 f30f1185cc010000 movss dword ptr [rbp + 0x1cc], xmm0
0x140a9932c 48898da0010000 mov qword ptr [rbp + 0x1a0], rcx
0x140a99333 488b85f8020000 mov rax, qword ptr [rbp + 0x2f8]
0x140a9933a 488985a8010000 mov qword ptr [rbp + 0x1a8], rax
0x140a99341 4889bdb0010000 mov qword ptr [rbp + 0x1b0], rdi
0x140a99348 488b85f0020000 mov rax, qword ptr [rbp + 0x2f0]
0x140a9934f 48898590010000 mov qword ptr [rbp + 0x190], rax
0x140a99356 488b8508030000 mov rax, qword ptr [rbp + 0x308]
0x140a9935d 48898598010000 mov qword ptr [rbp + 0x198], rax
0x140a99364 488b9d80020000 mov rbx, qword ptr [rbp + 0x280]
0x140a9936b 483b9d70020000 cmp rbx, qword ptr [rbp + 0x270]
0x140a99372 7513 jne 0x140a99387
0x140a99374 488d8d70020000 lea rcx, [rbp + 0x270]
0x140a9937b 488d15de1f0402 lea rdx, [rip + 0x2041fde]
0x140a99382 e82994acff call 0x1405627b0
0x140a99387 488b8578020000 mov rax, qword ptr [rbp + 0x278]
0x140a9938e 4889d9 mov rcx, rbx
0x140a99391 48c1e106 shl rcx, 6
0x140a99395 0f108590010000 movups xmm0, xmmword ptr [rbp + 0x190]
0x140a9939c 0f108da0010000 movups xmm1, xmmword ptr [rbp + 0x1a0]
0x140a993a3 0f1095b0010000 movups xmm2, xmmword ptr [rbp + 0x1b0]
0x140a993aa 0f109dc0010000 movups xmm3, xmmword ptr [rbp + 0x1c0]
0x140a993b1 0f115c0830 movups xmmword ptr [rax + rcx + 0x30], xmm3
0x140a993b6 0f11540820 movups xmmword ptr [rax + rcx + 0x20], xmm2
0x140a993bb 0f114c0810 movups xmmword ptr [rax + rcx + 0x10], xmm1
0x140a993c0 0f110408 movups xmmword ptr [rax + rcx], xmm0
0x140a993c4 48ffc3 inc rbx
0x140a993c7 48899d80020000 mov qword ptr [rbp + 0x280], rbx
0x140a993ce 488b8da0020000 mov rcx, qword ptr [rbp + 0x2a0]
0x140a993d5 488b8520020000 mov rax, qword ptr [rbp + 0x220]
0x140a993dc 4885c0 test rax, rax
0x140a993df 0f845bf7ffff je 0x140a98b40
0x140a993e5 48c1e003 shl rax, 3
0x140a993e9 488d1440 lea rdx, [rax + rax*2]
0x140a993ed 41b804000000 mov r8d, 4
0x140a993f3 e828a8b7ff call 0x140613c20
0x140a993f8 e943f7ffff jmp 0x140a98b40
0x140a993fd 0f1f00 nop dword ptr [rax]
0x140a99400 488b8528020000 mov rax, qword ptr [rbp + 0x228]
0x140a99407 48398530020000 cmp qword ptr [rbp + 0x230], rax
0x140a9940e 7450 je 0x140a99460
0x140a99410 488b8538020000 mov rax, qword ptr [rbp + 0x238]
0x140a99417 48b9cdcccccccccccccc movabs rcx, 0xcccccccccccccccd
0x140a99421 48f7e1 mul rcx
0x140a99424 4889d7 mov rdi, rdx
0x140a99427 48c1ef05 shr rdi, 5
0x140a9942b eb0c jmp 0x140a99439
0x140a9942d 0f1f00 nop dword ptr [rax]
0x140a99430 4883c328 add rbx, 0x28
0x140a99434 48ffcf dec rdi
0x140a99437 7427 je 0x140a99460
0x140a99439 488b43f8 mov rax, qword ptr [rbx - 8]
0x140a9943d 4885c0 test rax, rax
0x140a99440 74ee je 0x140a99430
0x140a99442 488b0b mov rcx, qword ptr [rbx]
0x140a99445 48c1e003 shl rax, 3
0x140a99449 488d1440 lea rdx, [rax + rax*2]
0x140a9944d 41b804000000 mov r8d, 4
0x140a99453 e8c8a7b7ff call 0x140613c20
0x140a99458 ebd6 jmp 0x140a99430
0x140a9945a 660f1f440000 nop word ptr [rax + rax]
0x140a99460 488b8540020000 mov rax, qword ptr [rbp + 0x240]
0x140a99467 4885c0 test rax, rax
0x140a9946a 741e je 0x140a9948a
0x140a9946c 488d04c500000000 lea rax, [rax*8]
0x140a99474 488d1480 lea rdx, [rax + rax*4]
0x140a99478 41b808000000 mov r8d, 8
0x140a9947e 488b8dc0020000 mov rcx, qword ptr [rbp + 0x2c0]
0x140a99485 e896a7b7ff call 0x140613c20
0x140a9948a c6852703000001 mov byte ptr [rbp + 0x327], 1
0x140a99491 c6852603000000 mov byte ptr [rbp + 0x326], 0
0x140a99498 488d8d90010000 lea rcx, [rbp + 0x190]
0x140a9949f 4c8bad28010000 mov r13, qword ptr [rbp + 0x128]
0x140a994a6 4c89ea mov rdx, r13
0x140a994a9 4c8d8558010000 lea r8, [rbp + 0x158]
0x140a994b0 e80b1beaff call 0x14093afc0
0x140a994b5 4c8bbd48020000 mov r15, qword ptr [rbp + 0x248]
0x140a994bc c6852703000001 mov byte ptr [rbp + 0x327], 1
0x140a994c3 c6852603000000 mov byte ptr [rbp + 0x326], 0
0x140a994ca 488d8d90010000 lea rcx, [rbp + 0x190]
0x140a994d1 4c89ea mov rdx, r13
0x140a994d4 4c8d8568010000 lea r8, [rbp + 0x168]
0x140a994db e8e01aeaff call 0x14093afc0
0x140a994e0 8b8560010000 mov eax, dword ptr [rbp + 0x160]
0x140a994e6 898580010000 mov dword ptr [rbp + 0x180], eax
0x140a994ec 488b8558010000 mov rax, qword ptr [rbp + 0x158]
0x140a994f3 48898578010000 mov qword ptr [rbp + 0x178], rax
0x140a994fa 8b8570010000 mov eax, dword ptr [rbp + 0x170]
0x140a99500 898560020000 mov dword ptr [rbp + 0x260], eax
0x140a99506 488b8568010000 mov rax, qword ptr [rbp + 0x168]
0x140a9950d 48898558020000 mov qword ptr [rbp + 0x258], rax
0x140a99514 488b8580020000 mov rax, qword ptr [rbp + 0x280]
0x140a9951b 488985a0010000 mov qword ptr [rbp + 0x1a0], rax
0x140a99522 0f108570020000 movups xmm0, xmmword ptr [rbp + 0x270]
0x140a99529 0f298590010000 movaps xmmword ptr [rbp + 0x190], xmm0
0x140a99530 660f6f8590000000 movdqa xmm0, xmmword ptr [rbp + 0x90]
0x140a99538 f30f7f85a8010000 movdqu xmmword ptr [rbp + 0x1a8], xmm0
0x140a99540 c6852703000000 mov byte ptr [rbp + 0x327], 0
0x140a99547 c6852603000000 mov byte ptr [rbp + 0x326], 0
0x140a9954e 488d8590010000 lea rax, [rbp + 0x190]
0x140a99555 4889442420 mov qword ptr [rsp + 0x20], rax
0x140a9955a 488d8dc8000000 lea rcx, [rbp + 0xc8]
0x140a99561 4c89ea mov rdx, r13
0x140a99564 4c8d8578010000 lea r8, [rbp + 0x178]
0x140a9956b 4c8d8d58020000 lea r9, [rbp + 0x258]
0x140a99572 e80914eaff call 0x14093a980
0x140a99577 31c0 xor eax, eax
0x140a99579 483b85c8000000 cmp rax, qword ptr [rbp + 0xc8]
0x140a99580 7026 jo 0x140a995a8
0x140a99582 488d8de0000000 lea rcx, [rbp + 0xe0]
0x140a99589 e8025edeff call 0x14087f390
0x140a9958e c6852703000000 mov byte ptr [rbp + 0x327], 0
0x140a99595 c6852603000000 mov byte ptr [rbp + 0x326], 0
0x140a9959c 488d8dc8000000 lea rcx, [rbp + 0xc8]
0x140a995a3 e848d3ddff call 0x1408768f0
0x140a995a8 488b7d58 mov rdi, qword ptr [rbp + 0x58]
0x140a995ac 4883c718 add rdi, 0x18
0x140a995b0 c6852403000000 mov byte ptr [rbp + 0x324], 0
0x140a995b7 c6852303000000 mov byte ptr [rbp + 0x323], 0
0x140a995be 4889f9 mov rcx, rdi
0x140a995c1 e84a881e00 call 0x140c81e10
0x140a995c6 b001 mov al, 1
0x140a995c8 31c9 xor ecx, ecx
0x140a995ca f00fb00f lock cmpxchg byte ptr [rdi], cl
0x140a995ce 0f8587000000 jne 0x140a9965b
0x140a995d4 488b9d18030000 mov rbx, qword ptr [rbp + 0x318]
0x140a995db 483b9dc8020000 cmp rbx, qword ptr [rbp + 0x2c8]
0x140a995e2 0f853ef1ffff jne 0x140a98726
0x140a995e8 e9ab000000 jmp 0x140a99698
0x140a995ed 4a83bc1970fdffff02 cmp qword ptr [rcx + r11 - 0x290], 2
0x140a995f6 0f82ea030000 jb 0x140a999e6
0x140a995fc f3420f10841990fdffff movss xmm0, dword ptr [rcx + r11 - 0x270]
0x140a99606 0f2e058370ec01 ucomiss xmm0, dword ptr [rip + 0x1ec7083]
0x140a9960d 0f86d3030000 jbe 0x140a999e6
0x140a99613 4c01d9 add rcx, r11
0x140a99616 8379b802 cmp dword ptr [rcx - 0x48], 2
0x140a9961a 0f92c0 setb al
0x140a9961d 898504030000 mov dword ptr [rbp + 0x304], eax
0x140a99623 e9b2f4ffff jmp 0x140a98ada
0x140a99628 c6852403000001 mov byte ptr [rbp + 0x324], 1
0x140a9962f c6852303000001 mov byte ptr [rbp + 0x323], 1
0x140a99636 48898dd0020000 mov qword ptr [rbp + 0x2d0], rcx
0x140a9963d 488b8dd0020000 mov rcx, qword ptr [rbp + 0x2d0]
0x140a99644 41b800ca9a3b mov r8d, 0x3b9aca00
0x140a9964a e8c13be301 call 0x1428cd210
0x140a9964f 488b8dd0020000 mov rcx, qword ptr [rbp + 0x2d0]
0x140a99656 e9cdf1ffff jmp 0x140a98828
0x140a9965b c6852403000000 mov byte ptr [rbp + 0x324], 0
0x140a99662 c6852303000000 mov byte ptr [rbp + 0x323], 0
0x140a99669 4889f9 mov rcx, rdi
0x140a9966c 31d2 xor edx, edx
0x140a9966e e81d45e301 call 0x1428cdb90
0x140a99673 e95cffffff jmp 0x140a995d4
0x140a99678 488d0df91c0402 lea rcx, [rip + 0x2041cf9]
0x140a9967f 4c8d051a1d0402 lea r8, [rip + 0x2041d1a]
0x140a99686 ba25000000 mov edx, 0x25
0x140a9968b e8a0fde301 call 0x1428d9430
0x140a99690 e926040000 jmp 0x140a99abb
0x140a99695 4889cb mov rbx, rcx
0x140a99698 48299dc8020000 sub qword ptr [rbp + 0x2c8], rbx
0x140a9969f 0f84a1000000 je 0x140a99746
0x140a996a5 48c1adc802000006 shr qword ptr [rbp + 0x2c8], 6
0x140a996ad 31f6 xor esi, esi
0x140a996af eb1b jmp 0x140a996cc
0x140a996b1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a996c0 48ffc6 inc rsi
0x140a996c3 483bb5c8020000 cmp rsi, qword ptr [rbp + 0x2c8]
0x140a996ca 747a je 0x140a99746
0x140a996cc 4989f6 mov r14, rsi
0x140a996cf 49c1e606 shl r14, 6
0x140a996d3 4a8b7c3308 mov rdi, qword ptr [rbx + r14 + 8]
0x140a996d8 4e8b7c3310 mov r15, qword ptr [rbx + r14 + 0x10]
0x140a996dd 4d85ff test r15, r15
0x140a996e0 743e je 0x140a99720
0x140a996e2 4c8d6708 lea r12, [rdi + 8]
0x140a996e6 eb11 jmp 0x140a996f9
0x140a996e8 0f1f840000000000 nop dword ptr [rax + rax]
0x140a996f0 4983c428 add r12, 0x28
0x140a996f4 49ffcf dec r15
0x140a996f7 7427 je 0x140a99720
0x140a996f9 498b4424f8 mov rax, qword ptr [r12 - 8]
0x140a996fe 4885c0 test rax, rax
0x140a99701 74ed je 0x140a996f0
0x140a99703 498b0c24 mov rcx, qword ptr [r12]
0x140a99707 48c1e003 shl rax, 3
0x140a9970b 488d1440 lea rdx, [rax + rax*2]
0x140a9970f 41b804000000 mov r8d, 4
0x140a99715 e806a5b7ff call 0x140613c20
0x140a9971a ebd4 jmp 0x140a996f0
0x140a9971c 0f1f4000 nop dword ptr [rax]
0x140a99720 4901de add r14, rbx
0x140a99723 498b06 mov rax, qword ptr [r14]
0x140a99726 4885c0 test rax, rax
0x140a99729 7495 je 0x140a996c0
0x140a9972b 48c1e003 shl rax, 3
0x140a9972f 488d1480 lea rdx, [rax + rax*4]
0x140a99733 41b808000000 mov r8d, 8
0x140a99739 4889f9 mov rcx, rdi
0x140a9973c e8dfa4b7ff call 0x140613c20
0x140a99741 e97affffff jmp 0x140a996c0
0x140a99746 488b95b8020000 mov rdx, qword ptr [rbp + 0x2b8]
0x140a9974d 4885d2 test rdx, rdx
0x140a99750 7416 je 0x140a99768
0x140a99752 48c1e206 shl rdx, 6
0x140a99756 41b808000000 mov r8d, 8
0x140a9975c 488b8db0020000 mov rcx, qword ptr [rbp + 0x2b0]
0x140a99763 e8b8a4b7ff call 0x140613c20
0x140a99768 488b8548040000 mov rax, qword ptr [rbp + 0x448]
0x140a9976f 8b581c mov ebx, dword ptr [rax + 0x1c]
0x140a99772 4c8b7010 mov r14, qword ptr [rax + 0x10]
0x140a99776 41891e mov dword ptr [r14], ebx
0x140a99779 488b38 mov rdi, qword ptr [rax]
0x140a9977c 488b4708 mov rax, qword ptr [rdi + 8]
0x140a99780 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
0x140a99787 488b4710 mov rax, qword ptr [rdi + 0x10]
0x140a9978b 48c7471000000000 mov qword ptr [rdi + 0x10], 0
0x140a99793 48898508030000 mov qword ptr [rbp + 0x308], rax
0x140a9979a 4885c0 test rax, rax
0x140a9979d 7456 je 0x140a997f5
0x140a9979f 4c8bbd08030000 mov r15, qword ptr [rbp + 0x308]
0x140a997a6 49f7df neg r15
0x140a997a9 b901000000 mov ecx, 1
0x140a997ae 488bb5d0020000 mov rsi, qword ptr [rbp + 0x2d0]
0x140a997b5 eb24 jmp 0x140a997db
0x140a997b7 660f1f840000000000 nop word ptr [rax + rax]
0x140a997c0 488b8d18030000 mov rcx, qword ptr [rbp + 0x318]
0x140a997c7 498d040f lea rax, [r15 + rcx]
0x140a997cb 48ffc0 inc rax
0x140a997ce 48ffc1 inc rcx
0x140a997d1 4883c640 add rsi, 0x40
0x140a997d5 4883f801 cmp rax, 1
0x140a997d9 741a je 0x140a997f5
0x140a997db 48898d18030000 mov qword ptr [rbp + 0x318], rcx
0x140a997e2 488b06 mov rax, qword ptr [rsi]
0x140a997e5 f048ff08 lock dec qword ptr [rax]
0x140a997e9 75d5 jne 0x140a997c0
0x140a997eb 4889f1 mov rcx, rsi
0x140a997ee e83d390c00 call 0x140b5d130
0x140a997f3 ebcb jmp 0x140a997c0
0x140a997f5 41891e mov dword ptr [r14], ebx
0x140a997f8 498b4550 mov rax, qword ptr [r13 + 0x50]
0x140a997fc 498b4d58 mov rcx, qword ptr [r13 + 0x58]
0x140a99800 488d0cc9 lea rcx, [rcx + rcx*8]
0x140a99804 488d0cc8 lea rcx, [rax + rcx*8]
0x140a99808 48898590010000 mov qword ptr [rbp + 0x190], rax
0x140a9980f 48898d98010000 mov qword ptr [rbp + 0x198], rcx
0x140a99816 48c785a001000000000000 mov qword ptr [rbp + 0x1a0], 0
0x140a99821 48c785b001000000000000 mov qword ptr [rbp + 0x1b0], 0
0x140a9982c c6852503000000 mov byte ptr [rbp + 0x325], 0
0x140a99833 4c8d05ae1a0402 lea r8, [rip + 0x2041aae]
0x140a9983a 488d9590010000 lea rdx, [rbp + 0x190]
0x140a99841 4889f9 mov rcx, rdi
0x140a99844 e8b7ff8500 call 0x1412f9800
0x140a99849 80bd2103000000 cmp byte ptr [rbp + 0x321], 0
0x140a99850 744b je 0x140a9989d
0x140a99852 e8d9e81700 call 0x140c18130
0x140a99857 4889c1 mov rcx, rax
0x140a9985a 488b00 mov rax, qword ptr [rax]
0x140a9985d 4883f801 cmp rax, 1
0x140a99861 0f85b7000000 jne 0x140a9991e
0x140a99867 4883c108 add rcx, 8
0x140a9986b 488b9550020000 mov rdx, qword ptr [rbp + 0x250]
0x140a99872 48833900 cmp qword ptr [rcx], 0
0x140a99876 0f853c010000 jne 0x140a999b8
0x140a9987c 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
0x140a99883 48898d18030000 mov qword ptr [rbp + 0x318], rcx
0x140a9988a 4883c108 add rcx, 8
0x140a9988e e85dd81700 call 0x140c170f0
0x140a99893 488b8518030000 mov rax, qword ptr [rbp + 0x318]
0x140a9989a 48ff00 inc qword ptr [rax]
0x140a9989d 0f28b530030000 movaps xmm6, xmmword ptr [rbp + 0x330]
0x140a998a4 0f28bd40030000 movaps xmm7, xmmword ptr [rbp + 0x340]
0x140a998ab 440f288550030000 movaps xmm8, xmmword ptr [rbp + 0x350]
0x140a998b3 440f288d60030000 movaps xmm9, xmmword ptr [rbp + 0x360]
0x140a998bb 440f289570030000 movaps xmm10, xmmword ptr [rbp + 0x370]
0x140a998c3 440f289d80030000 movaps xmm11, xmmword ptr [rbp + 0x380]
0x140a998cb 440f28a590030000 movaps xmm12, xmmword ptr [rbp + 0x390]
0x140a998d3 440f28ada0030000 movaps xmm13, xmmword ptr [rbp + 0x3a0]
0x140a998db 440f28b5b0030000 movaps xmm14, xmmword ptr [rbp + 0x3b0]
0x140a998e3 440f28bdc0030000 movaps xmm15, xmmword ptr [rbp + 0x3c0]
0x140a998eb 4881c458040000 add rsp, 0x458
0x140a998f2 5b pop rbx
0x140a998f3 5f pop rdi
0x140a998f4 5e pop rsi
0x140a998f5 415c pop r12
0x140a998f7 415d pop r13
0x140a998f9 415e pop r14
0x140a998fb 415f pop r15
0x140a998fd 5d pop rbp
0x140a998fe c3 ret
0x140a998ff c6852203000001 mov byte ptr [rbp + 0x322], 1
0x140a99906 488d0d337a0502 lea rcx, [rip + 0x2057a33]
0x140a9990d 488d158c7a0502 lea rdx, [rip + 0x2057a8c]
0x140a99914 e8e700e101 call 0x1428a9a00
0x140a99919 e99d010000 jmp 0x140a99abb
0x140a9991e 4885c0 test rax, rax
0x140a99921 488b8550020000 mov rax, qword ptr [rbp + 0x250]
0x140a99928 7519 jne 0x140a99943
0x140a9992a 4889c6 mov rsi, rax
0x140a9992d 31d2 xor edx, edx
0x140a9992f e89ceee501 call 0x1428f87d0
0x140a99934 4889f2 mov rdx, rsi
0x140a99937 4889c1 mov rcx, rax
0x140a9993a 4885c0 test rax, rax
0x140a9993d 0f852fffffff jne 0x140a99872
0x140a99943 488d0d56111a02 lea rcx, [rip + 0x21a1156]
0x140a9994a e881e7e301 call 0x1428d80d0
0x140a9994f c6852703000001 mov byte ptr [rbp + 0x327], 1
0x140a99956 c6852603000001 mov byte ptr [rbp + 0x326], 1
0x140a9995d 488d0d2f080402 lea rcx, [rip + 0x204082f]
0x140a99964 4c8d0595190402 lea r8, [rip + 0x2041995]
0x140a9996b ba32000000 mov edx, 0x32
0x140a99970 e8bbfae301 call 0x1428d9430
0x140a99975 e941010000 jmp 0x140a99abb
0x140a9997a 48c7859001000000000000 mov qword ptr [rbp + 0x190], 0
0x140a99985 488d05f4471902 lea rax, [rip + 0x21947f4]
0x140a9998c 4889442420 mov qword ptr [rsp + 0x20], rax
0x140a99991 488d5538 lea rdx, [rbp + 0x38]
0x140a99995 4c8d8d90010000 lea r9, [rbp + 0x190]
0x140a9999c 31c9 xor ecx, ecx
0x140a9999e 4989f8 mov r8, rdi
0x140a999a1 e89b11e301 call 0x1428cab41
0x140a999a6 e910010000 jmp 0x140a99abb
0x140a999ab 4889f1 mov rcx, rsi
0x140a999ae e89d35e301 call 0x1428ccf50
0x140a999b3 e9a2ecffff jmp 0x140a9865a
0x140a999b8 488d0db1111a02 lea rcx, [rip + 0x21a11b1]
0x140a999bf e88cf8e301 call 0x1428d9250
0x140a999c4 488d0da5111a02 lea rcx, [rip + 0x21a11a5]
0x140a999cb e880f8e301 call 0x1428d9250
0x140a999d0 e9e6000000 jmp 0x140a99abb
0x140a999d5 488d0d94111a02 lea rcx, [rip + 0x21a1194]
0x140a999dc e86ff8e301 call 0x1428d9250
0x140a999e1 e9d5000000 jmp 0x140a99abb
0x140a999e6 c6852703000001 mov byte ptr [rbp + 0x327], 1
0x140a999ed c6852603000001 mov byte ptr [rbp + 0x326], 1
0x140a999f4 488d0d98070402 lea rcx, [rip + 0x2040798]
0x140a999fb 4c8d0516190402 lea r8, [rip + 0x2041916]
0x140a99a02 ba32000000 mov edx, 0x32
0x140a99a07 e824fae301 call 0x1428d9430
0x140a99a0c e9aa000000 jmp 0x140a99abb
0x140a99a11 488d0d880d0402 lea rcx, [rip + 0x2040d88]
0x140a99a18 4c8d0511190402 lea r8, [rip + 0x2041911]
0x140a99a1f ba14000000 mov edx, 0x14
0x140a99a24 e807f9e301 call 0x1428d9330
0x140a99a29 e98d000000 jmp 0x140a99abb
0x140a99a2e 488d0d13190402 lea rcx, [rip + 0x2041913]
0x140a99a35 e8d6f8e301 call 0x1428d9310
0x140a99a3a eb7f jmp 0x140a99abb
0x140a99a3c 488b8d98020000 mov rcx, qword ptr [rbp + 0x298]
0x140a99a43 488b95f0020000 mov rdx, qword ptr [rbp + 0x2f0]
0x140a99a4a e8b1f5e301 call 0x1428d9000
0x140a99a4f eb6a jmp 0x140a99abb
0x140a99a51 488d0d78190402 lea rcx, [rip + 0x2041978]
0x140a99a58 e8b3f8e301 call 0x1428d9310
0x140a99a5d eb5c jmp 0x140a99abb
0x140a99a5f c6852203000001 mov byte ptr [rbp + 0x322], 1
0x140a99a66 b908000000 mov ecx, 8
0x140a99a6b ba20000000 mov edx, 0x20
0x140a99a70 e88bf5e301 call 0x1428d9000
0x140a99a75 eb44 jmp 0x140a99abb
0x140a99a77 488d0d52180402 lea rcx, [rip + 0x2041852]
0x140a99a7e e88df8e301 call 0x1428d9310
0x140a99a83 eb36 jmp 0x140a99abb
0x140a99a85 c6852403000001 mov byte ptr [rbp + 0x324], 1
0x140a99a8c c6852303000001 mov byte ptr [rbp + 0x323], 1
0x140a99a93 488d0df6501902 lea rcx, [rip + 0x21950f6]
0x140a99a9a e871f8e301 call 0x1428d9310
0x140a99a9f eb1a jmp 0x140a99abb
0x140a99aa1 488d0df80f1a02 lea rcx, [rip + 0x21a0ff8]
0x140a99aa8 e823e6e301 call 0x1428d80d0
0x140a99aad eb0c jmp 0x140a99abb
0x140a99aaf 488d0dea0f1a02 lea rcx, [rip + 0x21a0fea]
0x140a99ab6 e815e6e301 call 0x1428d80d0
0x140a99abb 0f0b ud2
0x140a99abd 0f1f00 nop dword ptr [rax]
0x140a99ac0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a99ac5 55 push rbp
0x140a99ac6 4157 push r15
0x140a99ac8 4156 push r14
0x140a99aca 4155 push r13
0x140a99acc 4154 push r12
0x140a99ace 56 push rsi
0x140a99acf 57 push rdi
0x140a99ad0 53 push rbx
0x140a99ad1 4881ecd8000000 sub rsp, 0xd8
0x140a99ad8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a99adf 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a99ae5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a99aeb 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a99af1 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a99af7 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a99afd 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a99b06 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a99b0f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a99b18 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a99b20 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a99b28 488b8518030000 mov rax, qword ptr [rbp + 0x318]
0x140a99b2f 48ff00 inc qword ptr [rax]
0x140a99b32 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a99b3a 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a99b42 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a99b4b 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a99b54 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a99b5d 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a99b63 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a99b69 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a99b6f 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a99b75 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a99b7b 4881c4d8000000 add rsp, 0xd8
0x140a99b82 5b pop rbx
0x140a99b83 5f pop rdi
0x140a99b84 5e pop rsi
0x140a99b85 415c pop r12
0x140a99b87 415d pop r13
0x140a99b89 415e pop r14
0x140a99b8b 415f pop r15
0x140a99b8d 5d pop rbp
0x140a99b8e c3 ret
0x140a99b8f 90 nop
0x140a99b90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a99b95 55 push rbp
0x140a99b96 4157 push r15
0x140a99b98 4156 push r14
0x140a99b9a 4155 push r13
0x140a99b9c 4154 push r12
0x140a99b9e 56 push rsi
0x140a99b9f 57 push rdi
0x140a99ba0 53 push rbx
0x140a99ba1 4881ecd8000000 sub rsp, 0xd8
0x140a99ba8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a99baf 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a99bb5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a99bbb 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a99bc1 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a99bc7 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a99bcd 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a99bd6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a99bdf 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a99be8 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a99bf0 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a99bf8 488b8518030000 mov rax, qword ptr [rbp + 0x318]
0x140a99bff 48ff00 inc qword ptr [rax]
0x140a99c02 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a99c0a 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a99c12 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a99c1b 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a99c24 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a99c2d 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a99c33 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a99c39 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a99c3f 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a99c45 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a99c4b 4881c4d8000000 add rsp, 0xd8
0x140a99c52 5b pop rbx
0x140a99c53 5f pop rdi
0x140a99c54 5e pop rsi
0x140a99c55 415c pop r12
0x140a99c57 415d pop r13
0x140a99c59 415e pop r14
0x140a99c5b 415f pop r15
0x140a99c5d 5d pop rbp
0x140a99c5e c3 ret
0x140a99c5f 90 nop
0x140a99c60 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a99c65 55 push rbp
0x140a99c66 4157 push r15
0x140a99c68 4156 push r14
0x140a99c6a 4155 push r13
0x140a99c6c 4154 push r12
0x140a99c6e 56 push rsi
0x140a99c6f 57 push rdi
0x140a99c70 53 push rbx
0x140a99c71 4881ecd8000000 sub rsp, 0xd8
0x140a99c78 488daa80000000 lea rbp, [rdx + 0x80]
0x140a99c7f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a99c85 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a99c8b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a99c91 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a99c97 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a99c9d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a99ca6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a99caf 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a99cb8 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a99cc0 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a99cc8 488b8518030000 mov rax, qword ptr [rbp + 0x318]
0x140a99ccf 48ff00 inc qword ptr [rax]
0x140a99cd2 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a99cda 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a99ce2 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a99ceb 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a99cf4 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a99cfd 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a99d03 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a99d09 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a99d0f 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a99d15 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a99d1b 4881c4d8000000 add rsp, 0xd8
0x140a99d22 5b pop rbx
0x140a99d23 5f pop rdi
0x140a99d24 5e pop rsi
0x140a99d25 415c pop r12
0x140a99d27 415d pop r13
0x140a99d29 415e pop r14
0x140a99d2b 415f pop r15
0x140a99d2d 5d pop rbp
0x140a99d2e c3 ret
0x140a99d2f 90 nop
0x140a99d30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a99d35 55 push rbp
0x140a99d36 4157 push r15
0x140a99d38 4156 push r14
0x140a99d3a 4155 push r13
0x140a99d3c 4154 push r12
0x140a99d3e 56 push rsi
0x140a99d3f 57 push rdi
0x140a99d40 53 push rbx
0x140a99d41 4881ecd8000000 sub rsp, 0xd8
0x140a99d48 488daa80000000 lea rbp, [rdx + 0x80]
0x140a99d4f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a99d55 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a99d5b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a99d61 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a99d67 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a99d6d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a99d76 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a99d7f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a99d88 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a99d90 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a99d98 488b8518030000 mov rax, qword ptr [rbp + 0x318]
0x140a99d9f 48ff00 inc qword ptr [rax]
0x140a99da2 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a99daa 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a99db2 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a99dbb 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a99dc4 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a99dcd 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a99dd3 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a99dd9 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a99ddf 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a99de5 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a99deb 4881c4d8000000 add rsp, 0xd8
0x140a99df2 5b pop rbx
0x140a99df3 5f pop rdi
0x140a99df4 5e pop rsi
0x140a99df5 415c pop r12
0x140a99df7 415d pop r13
0x140a99df9 415e pop r14
0x140a99dfb 415f pop r15
0x140a99dfd 5d pop rbp
0x140a99dfe c3 ret
0x140a99dff 90 nop
0x140a99e00 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a99e05 55 push rbp
0x140a99e06 4157 push r15
0x140a99e08 4156 push r14
0x140a99e0a 4155 push r13
0x140a99e0c 4154 push r12
0x140a99e0e 56 push rsi
0x140a99e0f 57 push rdi
0x140a99e10 53 push rbx
0x140a99e11 4881ecd8000000 sub rsp, 0xd8
0x140a99e18 488daa80000000 lea rbp, [rdx + 0x80]
0x140a99e1f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a99e25 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a99e2b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a99e31 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a99e37 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a99e3d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a99e46 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a99e4f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a99e58 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a99e60 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a99e68 c6852703000000 mov byte ptr [rbp + 0x327], 0
0x140a99e6f c6852603000000 mov byte ptr [rbp + 0x326], 0
0x140a99e76 488d8dc8000000 lea rcx, [rbp + 0xc8]
0x140a99e7d e86ecaddff call 0x1408768f0
0x140a99e82 c6852703000000 mov byte ptr [rbp + 0x327], 0
0x140a99e89 c6852603000000 mov byte ptr [rbp + 0x326], 0
0x140a99e90 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a99e98 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a99ea0 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a99ea9 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a99eb2 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a99ebb 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a99ec1 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a99ec7 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a99ecd 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a99ed3 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a99ed9 4881c4d8000000 add rsp, 0xd8
0x140a99ee0 5b pop rbx
0x140a99ee1 5f pop rdi
0x140a99ee2 5e pop rsi
0x140a99ee3 415c pop r12
0x140a99ee5 415d pop r13
0x140a99ee7 415e pop r14
0x140a99ee9 415f pop r15
0x140a99eeb 5d pop rbp
0x140a99eec c3 ret
0x140a99eed 0f1f00 nop dword ptr [rax]
0x140a99ef0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a99ef5 55 push rbp
0x140a99ef6 4157 push r15
0x140a99ef8 4156 push r14
0x140a99efa 4155 push r13
0x140a99efc 4154 push r12
0x140a99efe 56 push rsi
0x140a99eff 57 push rdi
0x140a99f00 53 push rbx
0x140a99f01 4881ecd8000000 sub rsp, 0xd8
0x140a99f08 488daa80000000 lea rbp, [rdx + 0x80]
0x140a99f0f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a99f15 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a99f1b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a99f21 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a99f27 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a99f2d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a99f36 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a99f3f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a99f48 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a99f50 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a99f58 488bbd08030000 mov rdi, qword ptr [rbp + 0x308]
0x140a99f5f 488b9d18030000 mov rbx, qword ptr [rbp + 0x318]
0x140a99f66 4839fb cmp rbx, rdi
0x140a99f69 488bb5d0020000 mov rsi, qword ptr [rbp + 0x2d0]
0x140a99f70 7576 jne 0x140a99fe8
0x140a99f72 c6852503000000 mov byte ptr [rbp + 0x325], 0
0x140a99f79 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a99f81 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a99f89 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a99f92 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a99f9b 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a99fa4 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a99faa 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a99fb0 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a99fb6 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a99fbc 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a99fc2 4881c4d8000000 add rsp, 0xd8
0x140a99fc9 5b pop rbx
0x140a99fca 5f pop rdi
0x140a99fcb 5e pop rsi
0x140a99fcc 415c pop r12
0x140a99fce 415d pop r13
0x140a99fd0 415e pop r14
0x140a99fd2 415f pop r15
0x140a99fd4 5d pop rbp
0x140a99fd5 c3 ret
0x140a99fd6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a99fe0 48ffc3 inc rbx
0x140a99fe3 4839fb cmp rbx, rdi
0x140a99fe6 748a je 0x140a99f72
0x140a99fe8 4889d9 mov rcx, rbx
0x140a99feb 48c1e106 shl rcx, 6
0x140a99fef 488b040e mov rax, qword ptr [rsi + rcx]
0x140a99ff3 f048ff08 lock dec qword ptr [rax]
0x140a99ff7 75e7 jne 0x140a99fe0
0x140a99ff9 4801f1 add rcx, rsi
0x140a99ffc c6852503000000 mov byte ptr [rbp + 0x325], 0
0x140a9a003 e828310c00 call 0x140b5d130
0x140a9a008 ebd6 jmp 0x140a99fe0
0x140a9a00a 660f1f440000 nop word ptr [rax + rax]
0x140a9a010 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a9a015 55 push rbp
0x140a9a016 4157 push r15
0x140a9a018 4156 push r14
0x140a9a01a 4155 push r13
0x140a9a01c 4154 push r12
0x140a9a01e 56 push rsi
0x140a9a01f 57 push rdi
0x140a9a020 53 push rbx
0x140a9a021 4881ecd8000000 sub rsp, 0xd8
0x140a9a028 488daa80000000 lea rbp, [rdx + 0x80]
0x140a9a02f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a9a035 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a9a03b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a9a041 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a9a047 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a9a04d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a9a056 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a9a05f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a9a068 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a9a070 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a9a078 488b8590010000 mov rax, qword ptr [rbp + 0x190]
0x140a9a07f f048ff08 lock dec qword ptr [rax]
0x140a9a083 750c jne 0x140a9a091
0x140a9a085 488d8d90010000 lea rcx, [rbp + 0x190]
0x140a9a08c e89f300c00 call 0x140b5d130
0x140a9a091 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9a099 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9a0a1 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9a0aa 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9a0b3 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9a0bc 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9a0c2 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9a0c8 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9a0ce 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9a0d4 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9a0da 4881c4d8000000 add rsp, 0xd8
0x140a9a0e1 5b pop rbx
0x140a9a0e2 5f pop rdi
0x140a9a0e3 5e pop rsi
0x140a9a0e4 415c pop r12
0x140a9a0e6 415d pop r13
0x140a9a0e8 415e pop r14
0x140a9a0ea 415f pop r15
0x140a9a0ec 5d pop rbp
0x140a9a0ed c3 ret
0x140a9a0ee 6690 nop
0x140a9a0f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a9a0f5 55 push rbp
0x140a9a0f6 4157 push r15
0x140a9a0f8 4156 push r14
0x140a9a0fa 4155 push r13
0x140a9a0fc 4154 push r12
0x140a9a0fe 56 push rsi
0x140a9a0ff 57 push rdi
0x140a9a100 53 push rbx
0x140a9a101 4881ecd8000000 sub rsp, 0xd8
0x140a9a108 488daa80000000 lea rbp, [rdx + 0x80]
0x140a9a10f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a9a115 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a9a11b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a9a121 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a9a127 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a9a12d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a9a136 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a9a13f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a9a148 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a9a150 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a9a158 488d8d58020000 lea rcx, [rbp + 0x258]
0x140a9a15f e8ecf1ddff call 0x140879350
0x140a9a164 488d8dc8000000 lea rcx, [rbp + 0xc8]
0x140a9a16b e8d0cdddff call 0x140876f40
0x140a9a170 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9a178 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9a180 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9a189 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9a192 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9a19b 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9a1a1 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9a1a7 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9a1ad 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9a1b3 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9a1b9 4881c4d8000000 add rsp, 0xd8
0x140a9a1c0 5b pop rbx
0x140a9a1c1 5f pop rdi
0x140a9a1c2 5e pop rsi
0x140a9a1c3 415c pop r12
0x140a9a1c5 415d pop r13
0x140a9a1c7 415e pop r14
0x140a9a1c9 415f pop r15
0x140a9a1cb 5d pop rbp
0x140a9a1cc c3 ret
0x140a9a1cd 0f1f00 nop dword ptr [rax]
0x140a9a1d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a9a1d5 55 push rbp
0x140a9a1d6 4157 push r15
0x140a9a1d8 4156 push r14
0x140a9a1da 4155 push r13
0x140a9a1dc 4154 push r12
0x140a9a1de 56 push rsi
0x140a9a1df 57 push rdi
0x140a9a1e0 53 push rbx
0x140a9a1e1 4881ecd8000000 sub rsp, 0xd8
0x140a9a1e8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a9a1ef 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a9a1f5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a9a1fb 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a9a201 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a9a207 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a9a20d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a9a216 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a9a21f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a9a228 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a9a230 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a9a238 488b8578010000 mov rax, qword ptr [rbp + 0x178]
0x140a9a23f 4885c0 test rax, rax
0x140a9a242 741a je 0x140a9a25e
0x140a9a244 488b8d80010000 mov rcx, qword ptr [rbp + 0x180]
0x140a9a24b 48c1e002 shl rax, 2
0x140a9a24f 488d14c0 lea rdx, [rax + rax*8]
0x140a9a253 41b804000000 mov r8d, 4
0x140a9a259 e8c299b7ff call 0x140613c20
0x140a9a25e 80bd4001000000 cmp byte ptr [rbp + 0x140], 0
0x140a9a265 7420 je 0x140a9a287
0x140a9a267 488d4500 lea rax, [rbp]
0x140a9a26b 488945f0 mov qword ptr [rbp - 0x10], rax
0x140a9a26f 488d45f0 lea rax, [rbp - 0x10]
0x140a9a273 488945e8 mov qword ptr [rbp - 0x18], rax
0x140a9a277 488d0d1a0a0402 lea rcx, [rip + 0x2040a1a]
0x140a9a27e 488d55e8 lea rdx, [rbp - 0x18]
0x140a9a282 e849508900 call 0x14132f2d0
0x140a9a287 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9a28f 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9a297 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9a2a0 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9a2a9 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9a2b2 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9a2b8 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9a2be 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9a2c4 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9a2ca 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9a2d0 4881c4d8000000 add rsp, 0xd8
0x140a9a2d7 5b pop rbx
0x140a9a2d8 5f pop rdi
0x140a9a2d9 5e pop rsi
0x140a9a2da 415c pop r12
0x140a9a2dc 415d pop r13
0x140a9a2de 415e pop r14
0x140a9a2e0 415f pop r15
0x140a9a2e2 5d pop rbp
0x140a9a2e3 c3 ret
0x140a9a2e4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a9a2f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a9a2f5 55 push rbp
0x140a9a2f6 4157 push r15
0x140a9a2f8 4156 push r14
0x140a9a2fa 4155 push r13
0x140a9a2fc 4154 push r12
0x140a9a2fe 56 push rsi
0x140a9a2ff 57 push rdi
0x140a9a300 53 push rbx
0x140a9a301 4881ecd8000000 sub rsp, 0xd8
0x140a9a308 488daa80000000 lea rbp, [rdx + 0x80]
0x140a9a30f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a9a315 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a9a31b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a9a321 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a9a327 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a9a32d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a9a336 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a9a33f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a9a348 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a9a350 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a9a358 488d8db0000000 lea rcx, [rbp + 0xb0]
0x140a9a35f e85cd9ddff call 0x140877cc0
0x140a9a364 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9a36c 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9a374 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9a37d 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9a386 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9a38f 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9a395 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9a39b 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9a3a1 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9a3a7 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9a3ad 4881c4d8000000 add rsp, 0xd8
0x140a9a3b4 5b pop rbx
0x140a9a3b5 5f pop rdi
0x140a9a3b6 5e pop rsi
0x140a9a3b7 415c pop r12
0x140a9a3b9 415d pop r13
0x140a9a3bb 415e pop r14
0x140a9a3bd 415f pop r15
0x140a9a3bf 5d pop rbp
0x140a9a3c0 c3 ret
0x140a9a3c1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a9a3d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a9a3d5 55 push rbp
0x140a9a3d6 4157 push r15
0x140a9a3d8 4156 push r14
0x140a9a3da 4155 push r13
0x140a9a3dc 4154 push r12
0x140a9a3de 56 push rsi
0x140a9a3df 57 push rdi
0x140a9a3e0 53 push rbx
0x140a9a3e1 4881ecd8000000 sub rsp, 0xd8
0x140a9a3e8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a9a3ef 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a9a3f5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a9a3fb 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a9a401 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a9a407 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a9a40d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a9a416 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a9a41f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a9a428 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a9a430 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a9a438 488b8520020000 mov rax, qword ptr [rbp + 0x220]
0x140a9a43f 4885c0 test rax, rax
0x140a9a442 741a je 0x140a9a45e
0x140a9a444 48c1e003 shl rax, 3
0x140a9a448 488d1440 lea rdx, [rax + rax*2]
0x140a9a44c 41b804000000 mov r8d, 4
0x140a9a452 488b8da0020000 mov rcx, qword ptr [rbp + 0x2a0]
0x140a9a459 e8c297b7ff call 0x140613c20
0x140a9a45e 488b8528020000 mov rax, qword ptr [rbp + 0x228]
0x140a9a465 48398530020000 cmp qword ptr [rbp + 0x230], rax
0x140a9a46c 488bbd40020000 mov rdi, qword ptr [rbp + 0x240]
0x140a9a473 488b9d20010000 mov rbx, qword ptr [rbp + 0x120]
0x140a9a47a 0f858e000000 jne 0x140a9a50e
0x140a9a480 4885ff test rdi, rdi
0x140a9a483 741e je 0x140a9a4a3
0x140a9a485 488d04fd00000000 lea rax, [rdi*8]
0x140a9a48d 488d1480 lea rdx, [rax + rax*4]
0x140a9a491 41b808000000 mov r8d, 8
0x140a9a497 488b8dc0020000 mov rcx, qword ptr [rbp + 0x2c0]
0x140a9a49e e87d97b7ff call 0x140613c20
0x140a9a4a3 c6852703000001 mov byte ptr [rbp + 0x327], 1
0x140a9a4aa c6852603000000 mov byte ptr [rbp + 0x326], 0
0x140a9a4b1 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9a4b9 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9a4c1 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9a4ca 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9a4d3 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9a4dc 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9a4e2 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9a4e8 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9a4ee 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9a4f4 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9a4fa 4881c4d8000000 add rsp, 0xd8
0x140a9a501 5b pop rbx
0x140a9a502 5f pop rdi
0x140a9a503 5e pop rsi
0x140a9a504 415c pop r12
0x140a9a506 415d pop r13
0x140a9a508 415e pop r14
0x140a9a50a 415f pop r15
0x140a9a50c 5d pop rbp
0x140a9a50d c3 ret
0x140a9a50e 488b8538020000 mov rax, qword ptr [rbp + 0x238]
0x140a9a515 48b9cdcccccccccccccc movabs rcx, 0xcccccccccccccccd
0x140a9a51f 48f7e1 mul rcx
0x140a9a522 4889d6 mov rsi, rdx
0x140a9a525 48c1ee05 shr rsi, 5
0x140a9a529 eb12 jmp 0x140a9a53d
0x140a9a52b 0f1f440000 nop dword ptr [rax + rax]
0x140a9a530 4883c328 add rbx, 0x28
0x140a9a534 48ffce dec rsi
0x140a9a537 0f8443ffffff je 0x140a9a480
0x140a9a53d 488b43f8 mov rax, qword ptr [rbx - 8]
0x140a9a541 4885c0 test rax, rax
0x140a9a544 74ea je 0x140a9a530
0x140a9a546 488b0b mov rcx, qword ptr [rbx]
0x140a9a549 48c1e003 shl rax, 3
0x140a9a54d 488d1440 lea rdx, [rax + rax*2]
0x140a9a551 41b804000000 mov r8d, 4
0x140a9a557 e8c496b7ff call 0x140613c20
0x140a9a55c ebd2 jmp 0x140a9a530
0x140a9a55e 6690 nop
0x140a9a560 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a9a565 55 push rbp
0x140a9a566 4157 push r15
0x140a9a568 4156 push r14
0x140a9a56a 4155 push r13
0x140a9a56c 4154 push r12
0x140a9a56e 56 push rsi
0x140a9a56f 57 push rdi
0x140a9a570 53 push rbx
0x140a9a571 4881ecd8000000 sub rsp, 0xd8
0x140a9a578 488daa80000000 lea rbp, [rdx + 0x80]
0x140a9a57f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a9a585 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a9a58b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a9a591 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a9a597 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a9a59d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a9a5a6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a9a5af 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a9a5b8 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a9a5c0 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a9a5c8 0fb68526030000 movzx eax, byte ptr [rbp + 0x326]
0x140a9a5cf 8885d0020000 mov byte ptr [rbp + 0x2d0], al
0x140a9a5d5 0fb68527030000 movzx eax, byte ptr [rbp + 0x327]
0x140a9a5dc 888508030000 mov byte ptr [rbp + 0x308], al
0x140a9a5e2 a801 test al, 1
0x140a9a5e4 7469 je 0x140a9a64f
0x140a9a5e6 488d8d70020000 lea rcx, [rbp + 0x270]
0x140a9a5ed e8fec2ddff call 0x1408768f0
0x140a9a5f2 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9a5fa 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9a602 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9a60b 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9a614 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9a61d 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9a623 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9a629 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9a62f 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9a635 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9a63b 4881c4d8000000 add rsp, 0xd8
0x140a9a642 5b pop rbx
0x140a9a643 5f pop rdi
0x140a9a644 5e pop rsi
0x140a9a645 415c pop r12
0x140a9a647 415d pop r13
0x140a9a649 415e pop r14
0x140a9a64b 415f pop r15
0x140a9a64d 5d pop rbp
0x140a9a64e c3 ret
0x140a9a64f 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9a657 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9a65f 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9a668 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9a671 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9a67a 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9a680 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9a686 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9a68c 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9a692 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9a698 4881c4d8000000 add rsp, 0xd8
0x140a9a69f 5b pop rbx
0x140a9a6a0 5f pop rdi
0x140a9a6a1 5e pop rsi
0x140a9a6a2 415c pop r12
0x140a9a6a4 415d pop r13
0x140a9a6a6 415e pop r14
0x140a9a6a8 415f pop r15
0x140a9a6aa 5d pop rbp
0x140a9a6ab c3 ret
0x140a9a6ac 0f1f4000 nop dword ptr [rax]
0x140a9a6b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a9a6b5 55 push rbp
0x140a9a6b6 4157 push r15
0x140a9a6b8 4156 push r14
0x140a9a6ba 4155 push r13
0x140a9a6bc 4154 push r12
0x140a9a6be 56 push rsi
0x140a9a6bf 57 push rdi
0x140a9a6c0 53 push rbx
0x140a9a6c1 4881ecd8000000 sub rsp, 0xd8
0x140a9a6c8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a9a6cf 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a9a6d5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a9a6db 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a9a6e1 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a9a6e7 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a9a6ed 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a9a6f6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a9a6ff 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a9a708 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a9a710 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a9a718 488b7d58 mov rdi, qword ptr [rbp + 0x58]
0x140a9a71c 488d7718 lea rsi, [rdi + 0x18]
0x140a9a720 4889f1 mov rcx, rsi
0x140a9a723 e8e8761e00 call 0x140c81e10
0x140a9a728 31c9 xor ecx, ecx
0x140a9a72a b001 mov al, 1
0x140a9a72c f00fb04f18 lock cmpxchg byte ptr [rdi + 0x18], cl
0x140a9a731 7579 jne 0x140a9a7ac
0x140a9a733 0fb68508030000 movzx eax, byte ptr [rbp + 0x308]
0x140a9a73a 888524030000 mov byte ptr [rbp + 0x324], al
0x140a9a740 0fb685d0020000 movzx eax, byte ptr [rbp + 0x2d0]
0x140a9a747 2401 and al, 1
0x140a9a749 888523030000 mov byte ptr [rbp + 0x323], al
0x140a9a74f 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9a757 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9a75f 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9a768 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9a771 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9a77a 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9a780 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9a786 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9a78c 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9a792 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9a798 4881c4d8000000 add rsp, 0xd8
0x140a9a79f 5b pop rbx
0x140a9a7a0 5f pop rdi
0x140a9a7a1 5e pop rsi
0x140a9a7a2 415c pop r12
0x140a9a7a4 415d pop r13
0x140a9a7a6 415e pop r14
0x140a9a7a8 415f pop r15
0x140a9a7aa 5d pop rbp
0x140a9a7ab c3 ret
0x140a9a7ac 4889f1 mov rcx, rsi
0x140a9a7af 31d2 xor edx, edx
0x140a9a7b1 e8da33e301 call 0x1428cdb90
0x140a9a7b6 e978ffffff jmp 0x140a9a733
0x140a9a7bb 0f1f440000 nop dword ptr [rax + rax]
0x140a9a7c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a9a7c5 55 push rbp
0x140a9a7c6 4157 push r15
0x140a9a7c8 4156 push r14
0x140a9a7ca 4155 push r13
0x140a9a7cc 4154 push r12
0x140a9a7ce 56 push rsi
0x140a9a7cf 57 push rdi
0x140a9a7d0 53 push rbx
0x140a9a7d1 4881ecd8000000 sub rsp, 0xd8
0x140a9a7d8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a9a7df 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a9a7e5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a9a7eb 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a9a7f1 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a9a7f7 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a9a7fd 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a9a806 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a9a80f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a9a818 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a9a820 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a9a828 0fb69d23030000 movzx ebx, byte ptr [rbp + 0x323]
0x140a9a82f f6852403000001 test byte ptr [rbp + 0x324], 1
0x140a9a836 7472 je 0x140a9a8aa
0x140a9a838 488d8d90000000 lea rcx, [rbp + 0x90]
0x140a9a83f e84c4bdeff call 0x14087f390
0x140a9a844 80e301 and bl, 1
0x140a9a847 889d22030000 mov byte ptr [rbp + 0x322], bl
0x140a9a84d 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9a855 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9a85d 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9a866 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9a86f 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9a878 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9a87e 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9a884 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9a88a 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9a890 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9a896 4881c4d8000000 add rsp, 0xd8
0x140a9a89d 5b pop rbx
0x140a9a89e 5f pop rdi
0x140a9a89f 5e pop rsi
0x140a9a8a0 415c pop r12
0x140a9a8a2 415d pop r13
0x140a9a8a4 415e pop r14
0x140a9a8a6 415f pop r15
0x140a9a8a8 5d pop rbp
0x140a9a8a9 c3 ret
0x140a9a8aa 80e301 and bl, 1
0x140a9a8ad 889d22030000 mov byte ptr [rbp + 0x322], bl
0x140a9a8b3 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9a8bb 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9a8c3 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9a8cc 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9a8d5 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9a8de 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9a8e4 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9a8ea 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9a8f0 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9a8f6 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9a8fc 4881c4d8000000 add rsp, 0xd8
0x140a9a903 5b pop rbx
0x140a9a904 5f pop rdi
0x140a9a905 5e pop rsi
0x140a9a906 415c pop r12
0x140a9a908 415d pop r13
0x140a9a90a 415e pop r14
0x140a9a90c 415f pop r15
0x140a9a90e 5d pop rbp
0x140a9a90f c3 ret
0x140a9a910 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a9a915 55 push rbp
0x140a9a916 4157 push r15
0x140a9a918 4156 push r14
0x140a9a91a 4155 push r13
0x140a9a91c 4154 push r12
0x140a9a91e 56 push rsi
0x140a9a91f 57 push rdi
0x140a9a920 53 push rbx
0x140a9a921 4881ecd8000000 sub rsp, 0xd8
0x140a9a928 488daa80000000 lea rbp, [rdx + 0x80]
0x140a9a92f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a9a935 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a9a93b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a9a941 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a9a947 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a9a94d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a9a956 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a9a95f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a9a968 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a9a970 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a9a978 80bd2203000000 cmp byte ptr [rbp + 0x322], 0
0x140a9a97f 0f8493000000 je 0x140a9aa18
0x140a9a985 488b9d30010000 mov rbx, qword ptr [rbp + 0x130]
0x140a9a98c 4885db test rbx, rbx
0x140a9a98f 488bbd40020000 mov rdi, qword ptr [rbp + 0x240]
0x140a9a996 0f85d9000000 jne 0x140a9aa75
0x140a9a99c 4885ff test rdi, rdi
0x140a9a99f 741a je 0x140a9a9bb
0x140a9a9a1 48c1e703 shl rdi, 3
0x140a9a9a5 488d14bf lea rdx, [rdi + rdi*4]
0x140a9a9a9 41b808000000 mov r8d, 8
0x140a9a9af 488b8dc0020000 mov rcx, qword ptr [rbp + 0x2c0]
0x140a9a9b6 e86592b7ff call 0x140613c20
0x140a9a9bb 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9a9c3 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9a9cb 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9a9d4 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9a9dd 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9a9e6 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9a9ec 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9a9f2 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9a9f8 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9a9fe 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9aa04 4881c4d8000000 add rsp, 0xd8
0x140a9aa0b 5b pop rbx
0x140a9aa0c 5f pop rdi
0x140a9aa0d 5e pop rsi
0x140a9aa0e 415c pop r12
0x140a9aa10 415d pop r13
0x140a9aa12 415e pop r14
0x140a9aa14 415f pop r15
0x140a9aa16 5d pop rbp
0x140a9aa17 c3 ret
0x140a9aa18 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9aa20 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9aa28 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9aa31 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9aa3a 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9aa43 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9aa49 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9aa4f 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9aa55 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9aa5b 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9aa61 4881c4d8000000 add rsp, 0xd8
0x140a9aa68 5b pop rbx
0x140a9aa69 5f pop rdi
0x140a9aa6a 5e pop rsi
0x140a9aa6b 415c pop r12
0x140a9aa6d 415d pop r13
0x140a9aa6f 415e pop r14
0x140a9aa71 415f pop r15
0x140a9aa73 5d pop rbp
0x140a9aa74 c3 ret
0x140a9aa75 488b85c0020000 mov rax, qword ptr [rbp + 0x2c0]
0x140a9aa7c 488d7008 lea rsi, [rax + 8]
0x140a9aa80 eb1b jmp 0x140a9aa9d
0x140a9aa82 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a9aa90 4883c628 add rsi, 0x28
0x140a9aa94 48ffcb dec rbx
0x140a9aa97 0f84fffeffff je 0x140a9a99c
0x140a9aa9d 488b46f8 mov rax, qword ptr [rsi - 8]
0x140a9aaa1 4885c0 test rax, rax
0x140a9aaa4 74ea je 0x140a9aa90
0x140a9aaa6 488b0e mov rcx, qword ptr [rsi]
0x140a9aaa9 48c1e003 shl rax, 3
0x140a9aaad 488d1440 lea rdx, [rax + rax*2]
0x140a9aab1 41b804000000 mov r8d, 4
0x140a9aab7 e86491b7ff call 0x140613c20
0x140a9aabc ebd2 jmp 0x140a9aa90
0x140a9aabe 6690 nop
0x140a9aac0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a9aac5 55 push rbp
0x140a9aac6 4157 push r15
0x140a9aac8 4156 push r14
0x140a9aaca 4155 push r13
0x140a9aacc 4154 push r12
0x140a9aace 56 push rsi
0x140a9aacf 57 push rdi
0x140a9aad0 53 push rbx
0x140a9aad1 4881ecd8000000 sub rsp, 0xd8
0x140a9aad8 488daa80000000 lea rbp, [rdx + 0x80]
0x140a9aadf 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a9aae5 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a9aaeb 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a9aaf1 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a9aaf7 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a9aafd 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a9ab06 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a9ab0f 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a9ab18 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a9ab20 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a9ab28 4c8badc8020000 mov r13, qword ptr [rbp + 0x2c8]
0x140a9ab2f 4c8bb518030000 mov r14, qword ptr [rbp + 0x318]
0x140a9ab36 4d29f5 sub r13, r14
0x140a9ab39 4c8ba5b0020000 mov r12, qword ptr [rbp + 0x2b0]
0x140a9ab40 0f8582000000 jne 0x140a9abc8
0x140a9ab46 488b95b8020000 mov rdx, qword ptr [rbp + 0x2b8]
0x140a9ab4d 4885d2 test rdx, rdx
0x140a9ab50 7412 je 0x140a9ab64
0x140a9ab52 48c1e206 shl rdx, 6
0x140a9ab56 41b808000000 mov r8d, 8
0x140a9ab5c 4c89e1 mov rcx, r12
0x140a9ab5f e8bc90b7ff call 0x140613c20
0x140a9ab64 c6852503000000 mov byte ptr [rbp + 0x325], 0
0x140a9ab6b 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9ab73 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9ab7b 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9ab84 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9ab8d 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9ab96 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9ab9c 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9aba2 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9aba8 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9abae 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9abb4 4881c4d8000000 add rsp, 0xd8
0x140a9abbb 5b pop rbx
0x140a9abbc 5f pop rdi
0x140a9abbd 5e pop rsi
0x140a9abbe 415c pop r12
0x140a9abc0 415d pop r13
0x140a9abc2 415e pop r14
0x140a9abc4 415f pop r15
0x140a9abc6 5d pop rbp
0x140a9abc7 c3 ret
0x140a9abc8 49c1ed06 shr r13, 6
0x140a9abcc 31ff xor edi, edi
0x140a9abce eb0c jmp 0x140a9abdc
0x140a9abd0 48ffc7 inc rdi
0x140a9abd3 4c39ef cmp rdi, r13
0x140a9abd6 0f846affffff je 0x140a9ab46
0x140a9abdc 4889fb mov rbx, rdi
0x140a9abdf 48c1e306 shl rbx, 6
0x140a9abe3 498b741e08 mov rsi, qword ptr [r14 + rbx + 8]
0x140a9abe8 4d8b741e10 mov r14, qword ptr [r14 + rbx + 0x10]
0x140a9abed 4d85f6 test r14, r14
0x140a9abf0 743e je 0x140a9ac30
0x140a9abf2 4c8d7e08 lea r15, [rsi + 8]
0x140a9abf6 eb11 jmp 0x140a9ac09
0x140a9abf8 0f1f840000000000 nop dword ptr [rax + rax]
0x140a9ac00 4983c728 add r15, 0x28
0x140a9ac04 49ffce dec r14
0x140a9ac07 7427 je 0x140a9ac30
0x140a9ac09 498b47f8 mov rax, qword ptr [r15 - 8]
0x140a9ac0d 4885c0 test rax, rax
0x140a9ac10 74ee je 0x140a9ac00
0x140a9ac12 498b0f mov rcx, qword ptr [r15]
0x140a9ac15 48c1e003 shl rax, 3
0x140a9ac19 488d1440 lea rdx, [rax + rax*2]
0x140a9ac1d 41b804000000 mov r8d, 4
0x140a9ac23 e8f88fb7ff call 0x140613c20
0x140a9ac28 ebd6 jmp 0x140a9ac00
0x140a9ac2a 660f1f440000 nop word ptr [rax + rax]
0x140a9ac30 4c8bb518030000 mov r14, qword ptr [rbp + 0x318]
0x140a9ac37 4c01f3 add rbx, r14
0x140a9ac3a 488b03 mov rax, qword ptr [rbx]
0x140a9ac3d 4885c0 test rax, rax
0x140a9ac40 748e je 0x140a9abd0
0x140a9ac42 48c1e003 shl rax, 3
0x140a9ac46 488d1480 lea rdx, [rax + rax*4]
0x140a9ac4a 41b808000000 mov r8d, 8
0x140a9ac50 4889f1 mov rcx, rsi
0x140a9ac53 e8c88fb7ff call 0x140613c20
0x140a9ac58 e973ffffff jmp 0x140a9abd0
0x140a9ac5d 0f1f00 nop dword ptr [rax]
0x140a9ac60 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a9ac65 55 push rbp
0x140a9ac66 4157 push r15
0x140a9ac68 4156 push r14
0x140a9ac6a 4155 push r13
0x140a9ac6c 4154 push r12
0x140a9ac6e 56 push rsi
0x140a9ac6f 57 push rdi
0x140a9ac70 53 push rbx
0x140a9ac71 4881ecd8000000 sub rsp, 0xd8
0x140a9ac78 488daa80000000 lea rbp, [rdx + 0x80]
0x140a9ac7f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a9ac85 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a9ac8b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a9ac91 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a9ac97 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a9ac9d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a9aca6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a9acaf 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a9acb8 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a9acc0 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a9acc8 80bd2503000000 cmp byte ptr [rbp + 0x325], 0
0x140a9accf 0f848f000000 je 0x140a9ad64
0x140a9acd5 4c8b6d30 mov r13, qword ptr [rbp + 0x30]
0x140a9acd9 4d85ed test r13, r13
0x140a9acdc 4c8ba5b0020000 mov r12, qword ptr [rbp + 0x2b0]
0x140a9ace3 0f85d8000000 jne 0x140a9adc1
0x140a9ace9 488b95b8020000 mov rdx, qword ptr [rbp + 0x2b8]
0x140a9acf0 4885d2 test rdx, rdx
0x140a9acf3 7412 je 0x140a9ad07
0x140a9acf5 48c1e206 shl rdx, 6
0x140a9acf9 41b808000000 mov r8d, 8
0x140a9acff 4c89e1 mov rcx, r12
0x140a9ad02 e8198fb7ff call 0x140613c20
0x140a9ad07 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9ad0f 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9ad17 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9ad20 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9ad29 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9ad32 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9ad38 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9ad3e 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9ad44 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9ad4a 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9ad50 4881c4d8000000 add rsp, 0xd8
0x140a9ad57 5b pop rbx
0x140a9ad58 5f pop rdi
0x140a9ad59 5e pop rsi
0x140a9ad5a 415c pop r12
0x140a9ad5c 415d pop r13
0x140a9ad5e 415e pop r14
0x140a9ad60 415f pop r15
0x140a9ad62 5d pop rbp
0x140a9ad63 c3 ret
0x140a9ad64 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9ad6c 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9ad74 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9ad7d 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9ad86 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9ad8f 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9ad95 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9ad9b 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9ada1 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9ada7 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9adad 4881c4d8000000 add rsp, 0xd8
0x140a9adb4 5b pop rbx
0x140a9adb5 5f pop rdi
0x140a9adb6 5e pop rsi
0x140a9adb7 415c pop r12
0x140a9adb9 415d pop r13
0x140a9adbb 415e pop r14
0x140a9adbd 415f pop r15
0x140a9adbf 5d pop rbp
0x140a9adc0 c3 ret
0x140a9adc1 31ff xor edi, edi
0x140a9adc3 eb17 jmp 0x140a9addc
0x140a9adc5 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a9add0 48ffc7 inc rdi
0x140a9add3 4c39ef cmp rdi, r13
0x140a9add6 0f840dffffff je 0x140a9ace9
0x140a9addc 4889fb mov rbx, rdi
0x140a9addf 48c1e306 shl rbx, 6
0x140a9ade3 498b741c08 mov rsi, qword ptr [r12 + rbx + 8]
0x140a9ade8 4d8b741c10 mov r14, qword ptr [r12 + rbx + 0x10]
0x140a9aded 4d85f6 test r14, r14
0x140a9adf0 743e je 0x140a9ae30
0x140a9adf2 4c8d7e08 lea r15, [rsi + 8]
0x140a9adf6 eb11 jmp 0x140a9ae09
0x140a9adf8 0f1f840000000000 nop dword ptr [rax + rax]
0x140a9ae00 4983c728 add r15, 0x28
0x140a9ae04 49ffce dec r14
0x140a9ae07 7427 je 0x140a9ae30
0x140a9ae09 498b47f8 mov rax, qword ptr [r15 - 8]
0x140a9ae0d 4885c0 test rax, rax
0x140a9ae10 74ee je 0x140a9ae00
0x140a9ae12 498b0f mov rcx, qword ptr [r15]
0x140a9ae15 48c1e003 shl rax, 3
0x140a9ae19 488d1440 lea rdx, [rax + rax*2]
0x140a9ae1d 41b804000000 mov r8d, 4
0x140a9ae23 e8f88db7ff call 0x140613c20
0x140a9ae28 ebd6 jmp 0x140a9ae00
0x140a9ae2a 660f1f440000 nop word ptr [rax + rax]
0x140a9ae30 4c01e3 add rbx, r12
0x140a9ae33 488b03 mov rax, qword ptr [rbx]
0x140a9ae36 4885c0 test rax, rax
0x140a9ae39 7495 je 0x140a9add0
0x140a9ae3b 48c1e003 shl rax, 3
0x140a9ae3f 488d1480 lea rdx, [rax + rax*4]
0x140a9ae43 41b808000000 mov r8d, 8
0x140a9ae49 4889f1 mov rcx, rsi
0x140a9ae4c e8cf8db7ff call 0x140613c20
0x140a9ae51 e97affffff jmp 0x140a9add0
0x140a9ae56 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a9ae60 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a9ae65 55 push rbp
0x140a9ae66 4157 push r15
0x140a9ae68 4156 push r14
0x140a9ae6a 4155 push r13
0x140a9ae6c 4154 push r12
0x140a9ae6e 56 push rsi
0x140a9ae6f 57 push rdi
0x140a9ae70 53 push rbx
0x140a9ae71 4881ecd8000000 sub rsp, 0xd8
0x140a9ae78 488daa80000000 lea rbp, [rdx + 0x80]
0x140a9ae7f 440f297c2430 movaps xmmword ptr [rsp + 0x30], xmm15
0x140a9ae85 440f29742440 movaps xmmword ptr [rsp + 0x40], xmm14
0x140a9ae8b 440f296c2450 movaps xmmword ptr [rsp + 0x50], xmm13
0x140a9ae91 440f29642460 movaps xmmword ptr [rsp + 0x60], xmm12
0x140a9ae97 440f295c2470 movaps xmmword ptr [rsp + 0x70], xmm11
0x140a9ae9d 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
0x140a9aea6 440f298c2490000000 movaps xmmword ptr [rsp + 0x90], xmm9
0x140a9aeaf 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
0x140a9aeb8 0f29bc24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm7
0x140a9aec0 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
0x140a9aec8 80bd2103000000 cmp byte ptr [rbp + 0x321], 0
0x140a9aecf 743d je 0x140a9af0e
0x140a9aed1 e85ad21700 call 0x140c18130
0x140a9aed6 4889c6 mov rsi, rax
0x140a9aed9 488b00 mov rax, qword ptr [rax]
0x140a9aedc 4883f801 cmp rax, 1
0x140a9aee0 0f8585000000 jne 0x140a9af6b
0x140a9aee6 4883c608 add rsi, 8
0x140a9aeea 48833e00 cmp qword ptr [rsi], 0
0x140a9aeee 0f859e000000 jne 0x140a9af92
0x140a9aef4 48c706ffffffff mov qword ptr [rsi], 0xffffffffffffffff
0x140a9aefb 488d4e08 lea rcx, [rsi + 8]
0x140a9aeff 488b9550020000 mov rdx, qword ptr [rbp + 0x250]
0x140a9af06 e8e5c11700 call 0x140c170f0
0x140a9af0b 48ff06 inc qword ptr [rsi]
0x140a9af0e 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x140a9af16 0f28bc24b0000000 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x140a9af1e 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x140a9af27 440f288c2490000000 movaps xmm9, xmmword ptr [rsp + 0x90]
0x140a9af30 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
0x140a9af39 440f285c2470 movaps xmm11, xmmword ptr [rsp + 0x70]
0x140a9af3f 440f28642460 movaps xmm12, xmmword ptr [rsp + 0x60]
0x140a9af45 440f286c2450 movaps xmm13, xmmword ptr [rsp + 0x50]
0x140a9af4b 440f28742440 movaps xmm14, xmmword ptr [rsp + 0x40]
0x140a9af51 440f287c2430 movaps xmm15, xmmword ptr [rsp + 0x30]
0x140a9af57 4881c4d8000000 add rsp, 0xd8
0x140a9af5e 5b pop rbx
0x140a9af5f 5f pop rdi
0x140a9af60 5e pop rsi
0x140a9af61 415c pop r12
0x140a9af63 415d pop r13
0x140a9af65 415e pop r14
0x140a9af67 415f pop r15
0x140a9af69 5d pop rbp
0x140a9af6a c3 ret
0x140a9af6b 4885c0 test rax, rax
0x140a9af6e 7516 jne 0x140a9af86
0x140a9af70 4889f1 mov rcx, rsi
0x140a9af73 31d2 xor edx, edx
0x140a9af75 e856d8e501 call 0x1428f87d0
0x140a9af7a 4889c6 mov rsi, rax
0x140a9af7d 4885c0 test rax, rax
0x140a9af80 0f8564ffffff jne 0x140a9aeea
0x140a9af86 488d0d13fb1902 lea rcx, [rip + 0x219fb13]
0x140a9af8d e83ed1e301 call 0x1428d80d0
0x140a9af92 488d0dd7fb1902 lea rcx, [rip + 0x219fbd7]
0x140a9af99 e8b2e2e301 call 0x1428d9250
0x140a9af9e cc int3
