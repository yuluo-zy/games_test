140b1b860 4157 push r15
140b1b862 4156 push r14
140b1b864 4155 push r13
140b1b866 4154 push r12
140b1b868 56 push rsi
140b1b869 57 push rdi
140b1b86a 55 push rbp
140b1b86b 53 push rbx
140b1b86c 4881ec58010000 sub rsp, 0x158
140b1b873 440f29bc2440010000 movaps xmmword ptr [rsp + 0x140], xmm15
140b1b87c 440f29b42430010000 movaps xmmword ptr [rsp + 0x130], xmm14
140b1b885 440f29ac2420010000 movaps xmmword ptr [rsp + 0x120], xmm13
140b1b88e 440f29a42410010000 movaps xmmword ptr [rsp + 0x110], xmm12
140b1b897 440f299c2400010000 movaps xmmword ptr [rsp + 0x100], xmm11
140b1b8a0 440f299424f0000000 movaps xmmword ptr [rsp + 0xf0], xmm10
140b1b8a9 440f298c24e0000000 movaps xmmword ptr [rsp + 0xe0], xmm9
140b1b8b2 440f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm8
140b1b8bb 0f29bc24c0000000 movaps xmmword ptr [rsp + 0xc0], xmm7
140b1b8c3 0f29b424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm6
140b1b8cb 4889cf mov rdi, rcx
140b1b8ce 8b421c mov eax, dword ptr [rdx + 0x1c]
140b1b8d1 488b4a10 mov rcx, qword ptr [rdx + 0x10]
140b1b8d5 8901 mov dword ptr [rcx], eax
140b1b8d7 488b02 mov rax, qword ptr [rdx]
140b1b8da 833801 cmp dword ptr [rax], 1
140b1b8dd 0f85b8060000 jne 0x140b1bf9b
140b1b8e3 4c89ce mov rsi, r9
140b1b8e6 4c89c3 mov rbx, r8
140b1b8e9 488b8c24d0010000 mov rcx, qword ptr [rsp + 0x1d0]
140b1b8f1 488b9424c0010000 mov rdx, qword ptr [rsp + 0x1c0]
140b1b8f9 488b12 mov rdx, qword ptr [rdx]
140b1b8fc f30f107228 movss xmm6, dword ptr [rdx + 0x28]
140b1b901 f30f587004 addss xmm6, dword ptr [rax + 4]
140b1b906 f30f117004 movss dword ptr [rax + 4], xmm6
140b1b90b 488b01 mov rax, qword ptr [rcx]
140b1b90e f30f1005760ff601 movss xmm0, dword ptr [rip + 0x1f60f76]
140b1b916 f30f5e4028 divss xmm0, dword ptr [rax + 0x28]
140b1b91b 4c8bb424c8010000 mov r14, qword ptr [rsp + 0x1c8]
140b1b923 f30f5f05459fe001 maxss xmm0, dword ptr [rip + 0x1e09f45]
140b1b92b f30f5ef0 divss xmm6, xmm0
140b1b92f 0f28c6 movaps xmm0, xmm6
140b1b932 e82987e001 call 0x142924060
140b1b937 f30f5cf0 subss xmm6, xmm0
140b1b93b 488b0b mov rcx, qword ptr [rbx]
140b1b93e 488b4308 mov rax, qword ptr [rbx + 8]
140b1b942 4889442450 mov qword ptr [rsp + 0x50], rax
140b1b947 8b4314 mov eax, dword ptr [rbx + 0x14]
140b1b94a 89442424 mov dword ptr [rsp + 0x24], eax
140b1b94e 4c8ba9f8000000 mov r13, qword ptr [rcx + 0xf8]
140b1b955 488b8100010000 mov rax, qword ptr [rcx + 0x100]
140b1b95c 498d5c8500 lea rbx, [r13 + rax*4]
140b1b961 48894c2448 mov qword ptr [rsp + 0x48], rcx
140b1b966 0fb68108010000 movzx eax, byte ptr [rcx + 0x108]
140b1b96d 88442423 mov byte ptr [rsp + 0x23], al
140b1b971 488b3f mov rdi, qword ptr [rdi]
140b1b974 488b06 mov rax, qword ptr [rsi]
140b1b977 4889842488000000 mov qword ptr [rsp + 0x88], rax
140b1b97f 4d8b3e mov r15, qword ptr [r14]
140b1b982 498d4704 lea rax, [r15 + 4]
140b1b986 4889842480000000 mov qword ptr [rsp + 0x80], rax
140b1b98e b808000000 mov eax, 8
140b1b993 4889442470 mov qword ptr [rsp + 0x70], rax
140b1b998 31ed xor ebp, ebp
140b1b99a f3440f100d1dd6fc01 movss xmm9, dword ptr [rip + 0x1fcd61d]
140b1b9a3 f30f103d65b3ef01 movss xmm7, dword ptr [rip + 0x1efb365]
140b1b9ab f3440f101dbc9ee001 movss xmm11, dword ptr [rip + 0x1e09ebc]
140b1b9b4 f3440f102d479fe001 movss xmm13, dword ptr [rip + 0x1e09f47]
140b1b9bd 440f2835fb16eb01 movaps xmm14, xmmword ptr [rip + 0x1eb16fb]
140b1b9c5 f2440f103de28be101 movsd xmm15, qword ptr [rip + 0x1e18be2]
140b1b9ce f3440f10258912f701 movss xmm12, dword ptr [rip + 0x1f71289]
140b1b9d7 440f2815f1d5fc01 movaps xmm10, xmmword ptr [rip + 0x1fcd5f1]
140b1b9df 48c744243800000000 mov qword ptr [rsp + 0x38], 0
140b1b9e8 4531e4 xor r12d, r12d
140b1b9eb 48c744244000000000 mov qword ptr [rsp + 0x40], 0
140b1b9f4 0f28c6 movaps xmm0, xmm6
140b1b9f7 f3410f59c1 mulss xmm0, xmm9
140b1b9fc e8cf85e001 call 0x142923fd0
140b1ba01 440f28c0 movaps xmm8, xmm0
140b1ba05 0f28c6 movaps xmm0, xmm6
140b1ba08 f30f59c7 mulss xmm0, xmm7
140b1ba0c e89f84e001 call 0x142923eb0
140b1ba11 0f28f8 movaps xmm7, xmm0
140b1ba14 f3410f59f8 mulss xmm7, xmm8
140b1ba19 f3410f59fb mulss xmm7, xmm11
140b1ba1e f3440f12c7 movsldup xmm8, xmm7
140b1ba23 f3440f100dd421ed01 movss xmm9, dword ptr [rip + 0x1ed21d4]
140b1ba2c 440f281dbcd5fc01 movaps xmm11, xmmword ptr [rip + 0x1fcd5bc]
140b1ba34 4c8b4c2478 mov r9, qword ptr [rsp + 0x78]
140b1ba39 488b542440 mov rdx, qword ptr [rsp + 0x40]
140b1ba3e 807c242300 cmp byte ptr [rsp + 0x23], 0
140b1ba43 744b je 0x140b1ba90
140b1ba45 4c39e2 cmp rdx, r12
140b1ba48 0f853d010000 jne 0x140b1bb8b
140b1ba4e 4939dd cmp r13, rbx
140b1ba51 0f8444050000 je 0x140b1bf9b
140b1ba57 488b442450 mov rax, qword ptr [rsp + 0x50]
140b1ba5c 488b80a8010000 mov rax, qword ptr [rax + 0x1a8]
140b1ba63 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140b1ba70 418b4d00 mov ecx, dword ptr [r13]
140b1ba74 488d0cc9 lea rcx, [rcx + rcx*8]
140b1ba78 4c8b64c810 mov r12, qword ptr [rax + rcx*8 + 0x10]
140b1ba7d 4d85e4 test r12, r12
140b1ba80 7552 jne 0x140b1bad4
140b1ba82 4983c504 add r13, 4
140b1ba86 4939dd cmp r13, rbx
140b1ba89 75e5 jne 0x140b1ba70
140b1ba8b e90b050000 jmp 0x140b1bf9b
140b1ba90 4c39e2 cmp rdx, r12
140b1ba93 0f8531010000 jne 0x140b1bbca
140b1ba99 4939dd cmp r13, rbx
140b1ba9c 0f84f9040000 je 0x140b1bf9b
140b1baa2 488b442450 mov rax, qword ptr [rsp + 0x50]
140b1baa7 488b8000010000 mov rax, qword ptr [rax + 0x100]
140b1baae 6690 nop 
140b1bab0 418b4d00 mov ecx, dword ptr [r13]
140b1bab4 488d0c89 lea rcx, [rcx + rcx*4]
140b1bab8 48c1e105 shl rcx, 5
140b1babc 4c8b640858 mov r12, qword ptr [rax + rcx + 0x58]
140b1bac1 4d85e4 test r12, r12
140b1bac4 7570 jne 0x140b1bb36
140b1bac6 4983c504 add r13, 4
140b1baca 4939dd cmp r13, rbx
140b1bacd 75e1 jne 0x140b1bab0
140b1bacf e9c7040000 jmp 0x140b1bf9b
140b1bad4 4c8d04c8 lea r8, [rax + rcx*8]
140b1bad8 488b442448 mov rax, qword ptr [rsp + 0x48]
140b1badd 4c8b8810010000 mov r9, qword ptr [rax + 0x110]
140b1bae4 488b8818010000 mov rcx, qword ptr [rax + 0x118]
140b1baeb 498b4018 mov rax, qword ptr [r8 + 0x18]
140b1baef 498b5038 mov rdx, qword ptr [r8 + 0x38]
140b1baf3 4e8b0cca mov r9, qword ptr [rdx + r9*8]
140b1baf7 49f7d1 not r9
140b1bafa 4f8d0c49 lea r9, [r9 + r9*2]
140b1bafe 49c1e104 shl r9, 4
140b1bb02 4e8b540810 mov r10, qword ptr [rax + r9 + 0x10]
140b1bb07 4c89542438 mov qword ptr [rsp + 0x38], r10
140b1bb0c 4e8b4c0828 mov r9, qword ptr [rax + r9 + 0x28]
140b1bb11 4983c504 add r13, 4
140b1bb15 49394840 cmp qword ptr [r8 + 0x40], rcx
140b1bb19 766c jbe 0x140b1bb87
140b1bb1b 488b0cca mov rcx, qword ptr [rdx + rcx*8]
140b1bb1f 4885c9 test rcx, rcx
140b1bb22 7463 je 0x140b1bb87
140b1bb24 48f7d1 not rcx
140b1bb27 488d0c49 lea rcx, [rcx + rcx*2]
140b1bb2b 48c1e104 shl rcx, 4
140b1bb2f 488b6c0810 mov rbp, qword ptr [rax + rcx + 0x10]
140b1bb34 eb53 jmp 0x140b1bb89
140b1bb36 4801c8 add rax, rcx
140b1bb39 8b8894000000 mov ecx, dword ptr [rax + 0x94]
140b1bb3f 488b542450 mov rdx, qword ptr [rsp + 0x50]
140b1bb44 4c8b8aa8010000 mov r9, qword ptr [rdx + 0x1a8]
140b1bb4b 4c8d14c9 lea r10, [rcx + rcx*8]
140b1bb4f 488b4c2448 mov rcx, qword ptr [rsp + 0x48]
140b1bb54 4c8b8118010000 mov r8, qword ptr [rcx + 0x118]
140b1bb5b 4b8b4cd118 mov rcx, qword ptr [r9 + r10*8 + 0x18]
140b1bb60 4b8b54d138 mov rdx, qword ptr [r9 + r10*8 + 0x38]
140b1bb65 4f3944d140 cmp qword ptr [r9 + r10*8 + 0x40], r8
140b1bb6a 7623 jbe 0x140b1bb8f
140b1bb6c 4e8b04c2 mov r8, qword ptr [rdx + r8*8]
140b1bb70 4d85c0 test r8, r8
140b1bb73 741a je 0x140b1bb8f
140b1bb75 49f7d0 not r8
140b1bb78 4f8d0440 lea r8, [r8 + r8*2]
140b1bb7c 49c1e004 shl r8, 4
140b1bb80 4a8b6c0110 mov rbp, qword ptr [rcx + r8 + 0x10]
140b1bb85 eb0a jmp 0x140b1bb91
140b1bb87 31ed xor ebp, ebp
140b1bb89 31d2 xor edx, edx
140b1bb8b 89d0 mov eax, edx
140b1bb8d eb4b jmp 0x140b1bbda
140b1bb8f 31ed xor ebp, ebp
140b1bb91 4c8b442448 mov r8, qword ptr [rsp + 0x48]
140b1bb96 4d8b8010010000 mov r8, qword ptr [r8 + 0x110]
140b1bb9d 4a8b14c2 mov rdx, qword ptr [rdx + r8*8]
140b1bba1 48f7d2 not rdx
140b1bba4 488d1452 lea rdx, [rdx + rdx*2]
140b1bba8 48c1e204 shl rdx, 4
140b1bbac 4c8b441110 mov r8, qword ptr [rcx + rdx + 0x10]
140b1bbb1 4c89442438 mov qword ptr [rsp + 0x38], r8
140b1bbb6 4c8b4c1128 mov r9, qword ptr [rcx + rdx + 0x28]
140b1bbbb 4983c504 add r13, 4
140b1bbbf 488b4050 mov rax, qword ptr [rax + 0x50]
140b1bbc3 4889442470 mov qword ptr [rsp + 0x70], rax
140b1bbc8 31d2 xor edx, edx
140b1bbca 4889d0 mov rax, rdx
140b1bbcd 48c1e004 shl rax, 4
140b1bbd1 488b4c2470 mov rcx, qword ptr [rsp + 0x70]
140b1bbd6 8b440108 mov eax, dword ptr [rcx + rax + 8]
140b1bbda 4c8d3440 lea r14, [rax + rax*2]
140b1bbde 49c1e604 shl r14, 4
140b1bbe2 4c03742438 add r14, qword ptr [rsp + 0x38]
140b1bbe7 4d8d1481 lea r10, [r9 + rax*4]
140b1bbeb 488d34c500000000 lea rsi, [rax*8]
140b1bbf3 4801ee add rsi, rbp
140b1bbf6 48ffc2 inc rdx
140b1bbf9 48837f1800 cmp qword ptr [rdi + 0x18], 0
140b1bbfe 0f847c000000 je 0x140b1bc80
140b1bc04 4889542440 mov qword ptr [rsp + 0x40], rdx
140b1bc09 4c894c2478 mov qword ptr [rsp + 0x78], r9
140b1bc0e 48896c2460 mov qword ptr [rsp + 0x60], rbp
140b1bc13 4c89fd mov rbp, r15
140b1bc16 488b4710 mov rax, qword ptr [rdi + 0x10]
140b1bc1a 488b0f mov rcx, qword ptr [rdi]
140b1bc1d 4989ff mov r15, rdi
140b1bc20 488b5708 mov rdx, qword ptr [rdi + 8]
140b1bc24 4839c8 cmp rax, rcx
140b1bc27 41b800000000 mov r8d, 0
140b1bc2d 4c0f43c1 cmovae r8, rcx
140b1bc31 4c29c0 sub rax, r8
140b1bc34 48c1e006 shl rax, 6
140b1bc38 0fb67c0238 movzx edi, byte ptr [rdx + rax + 0x38]
140b1bc3d 8b442424 mov eax, dword ptr [rsp + 0x24]
140b1bc41 4c89542458 mov qword ptr [rsp + 0x58], r10
140b1bc46 418902 mov dword ptr [r10], eax
140b1bc49 488d8c2494000000 lea rcx, [rsp + 0x94]
140b1bc51 488b942488000000 mov rdx, qword ptr [rsp + 0x88]
140b1bc59 e8e2b01700 call 0x140c96d40
140b1bc5e 4084ff test dil, dil
140b1bc61 744d je 0x140b1bcb0
140b1bc63 f20f10059520ec01 movsd xmm0, qword ptr [rip + 0x1ec2095]
140b1bc6b f20f11442428 movsd qword ptr [rsp + 0x28], xmm0
140b1bc71 c744243000000080 mov dword ptr [rsp + 0x30], 0x80000000
140b1bc79 eb4c jmp 0x140b1bcc7
140b1bc7b 0f1f440000 nop dword ptr [rax + rax]
140b1bc80 41f60701 test byte ptr [r15], 1
140b1bc84 0f8465010000 je 0x140b1bdef
140b1bc8a 488b8c2480000000 mov rcx, qword ptr [rsp + 0x80]
140b1bc92 8b4108 mov eax, dword ptr [rcx + 8]
140b1bc95 89442430 mov dword ptr [rsp + 0x30], eax
140b1bc99 488b01 mov rax, qword ptr [rcx]
140b1bc9c 4889442428 mov qword ptr [rsp + 0x28], rax
140b1bca1 e95a010000 jmp 0x140b1be00
140b1bca6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
140b1bcb0 48b8000000000000803f movabs rax, 0x3f80000000000000
140b1bcba 4889442428 mov qword ptr [rsp + 0x28], rax
140b1bcbf c744243000000000 mov dword ptr [rsp + 0x30], 0
140b1bcc7 4c89f1 mov rcx, r14
140b1bcca 488d942494000000 lea rdx, [rsp + 0x94]
140b1bcd2 4c8d442428 lea r8, [rsp + 0x28]
140b1bcd7 e824111800 call 0x140c9ce00
140b1bcdc 48833e01 cmp qword ptr [rsi], 1
140b1bce0 4c89ff mov rdi, r15
140b1bce3 0f8534010000 jne 0x140b1be1d
140b1bce9 4989ef mov r15, rbp
140b1bcec f6450001 test byte ptr [rbp], 1
140b1bcf0 4c8b4c2478 mov r9, qword ptr [rsp + 0x78]
140b1bcf5 7419 je 0x140b1bd10
140b1bcf7 f2410f104704 movsd xmm0, qword ptr [r15 + 4]
140b1bcfd f3410f10570c movss xmm2, dword ptr [r15 + 0xc]
140b1bd03 eb11 jmp 0x140b1bd16
140b1bd05 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
140b1bd10 0f57c0 xorps xmm0, xmm0
140b1bd13 0f57d2 xorps xmm2, xmm2
140b1bd16 488b6c2460 mov rbp, qword ptr [rsp + 0x60]
140b1bd1b 488b542440 mov rdx, qword ptr [rsp + 0x40]
140b1bd20 488b4c2458 mov rcx, qword ptr [rsp + 0x58]
140b1bd25 410f280e movaps xmm1, xmmword ptr [r14]
140b1bd29 0f28e1 movaps xmm4, xmm1
140b1bd2c 0f59e1 mulps xmm4, xmm1
140b1bd2f f30f16dc movshdup xmm3, xmm4
140b1bd33 f30f58dc addss xmm3, xmm4
140b1bd37 0f28ec movaps xmm5, xmm4
140b1bd3a 660f15ec unpckhpd xmm5, xmm4
140b1bd3e f30f58eb addss xmm5, xmm3
140b1bd42 0fc6ed00 shufps xmm5, xmm5, 0
140b1bd46 0fc6e4ff shufps xmm4, xmm4, 0xff
140b1bd4a 0f5ce5 subps xmm4, xmm5
140b1bd4d 410f59e5 mulps xmm4, xmm13
140b1bd51 0f28d9 movaps xmm3, xmm1
140b1bd54 410f59dd mulps xmm3, xmm13
140b1bd58 f30f16eb movshdup xmm5, xmm3
140b1bd5c f30f58eb addss xmm5, xmm3
140b1bd60 0f12db movhlps xmm3, xmm3
140b1bd63 f30f58dd addss xmm3, xmm5
140b1bd67 f30f58db addss xmm3, xmm3
140b1bd6b 0fc6db00 shufps xmm3, xmm3, 0
140b1bd6f 0f59d9 mulps xmm3, xmm1
140b1bd72 0f58dc addps xmm3, xmm4
140b1bd75 0f28e1 movaps xmm4, xmm1
140b1bd78 0fc6e1d2 shufps xmm4, xmm1, 0xd2
140b1bd7c 410f59e6 mulps xmm4, xmm14
140b1bd80 0f28e9 movaps xmm5, xmm1
140b1bd83 0fc6e9c9 shufps xmm5, xmm1, 0xc9
140b1bd87 410f59ef mulps xmm5, xmm15
140b1bd8b 0f5cec subps xmm5, xmm4
140b1bd8e 0f58c9 addps xmm1, xmm1
140b1bd91 0fc6c9ff shufps xmm1, xmm1, 0xff
140b1bd95 0f59cd mulps xmm1, xmm5
140b1bd98 0f58cb addps xmm1, xmm3
140b1bd9b 0f28d9 movaps xmm3, xmm1
140b1bd9e 660f15d9 unpckhpd xmm3, xmm1
140b1bda2 0f28e3 movaps xmm4, xmm3
140b1bda5 f30f59e7 mulss xmm4, xmm7
140b1bda9 f3410f59e4 mulss xmm4, xmm12
140b1bdae f30f5cd4 subss xmm2, xmm4
140b1bdb2 f3410f59d9 mulss xmm3, xmm9
140b1bdb7 f3410f59dc mulss xmm3, xmm12
140b1bdbc f30f58da addss xmm3, xmm2
140b1bdc0 8b442424 mov eax, dword ptr [rsp + 0x24]
140b1bdc4 8901 mov dword ptr [rcx], eax
140b1bdc6 410f28d0 movaps xmm2, xmm8
140b1bdca 0f59d1 mulps xmm2, xmm1
140b1bdcd 410f59d2 mulps xmm2, xmm10
140b1bdd1 0f5cc2 subps xmm0, xmm2
140b1bdd4 410f59cb mulps xmm1, xmm11
140b1bdd8 410f59ca mulps xmm1, xmm10
140b1bddc 0f58c8 addps xmm1, xmm0
140b1bddf 410f134e10 movlps qword ptr [r14 + 0x10], xmm1
140b1bde4 f3410f115e18 movss dword ptr [r14 + 0x18], xmm3
140b1bdea e94ffcffff jmp 0x140b1ba3e
140b1bdef c744243000000000 mov dword ptr [rsp + 0x30], 0
140b1bdf7 48c744242800000000 mov qword ptr [rsp + 0x28], 0
140b1be00 8b442424 mov eax, dword ptr [rsp + 0x24]
140b1be04 418902 mov dword ptr [r10], eax
140b1be07 8b442430 mov eax, dword ptr [rsp + 0x30]
140b1be0b 41894618 mov dword ptr [r14 + 0x18], eax
140b1be0f 488b442428 mov rax, qword ptr [rsp + 0x28]
140b1be14 49894610 mov qword ptr [r14 + 0x10], rax
140b1be18 e921fcffff jmp 0x140b1ba3e
140b1be1d f30f5835c31eec01 addss xmm6, dword ptr [rip + 0x1ec1ec3]
140b1be25 0f57c9 xorps xmm1, xmm1
140b1be28 0f57c0 xorps xmm0, xmm0
140b1be2b f30f5fc6 maxss xmm0, xmm6
140b1be2f f30f1035cd9ae001 movss xmm6, dword ptr [rip + 0x1e09acd]
140b1be37 f30f5df0 minss xmm6, xmm0
140b1be3b 0f57c0 xorps xmm0, xmm0
140b1be3e 4989ef mov r15, rbp
140b1be41 f6450001 test byte ptr [rbp], 1
140b1be45 740c je 0x140b1be53
140b1be47 f2410f104704 movsd xmm0, qword ptr [r15 + 4]
140b1be4d f3410f104f0c movss xmm1, dword ptr [r15 + 0xc]
140b1be53 f30f114c246c movss dword ptr [rsp + 0x6c], xmm1
140b1be59 0f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm0
140b1be61 450f2806 movaps xmm8, xmmword ptr [r14]
140b1be65 410f28c8 movaps xmm1, xmm8
140b1be69 410f59c8 mulps xmm1, xmm8
140b1be6d f30f16c1 movshdup xmm0, xmm1
140b1be71 f30f58c1 addss xmm0, xmm1
140b1be75 0f28d1 movaps xmm2, xmm1
140b1be78 660f15d1 unpckhpd xmm2, xmm1
140b1be7c f30f58d0 addss xmm2, xmm0
140b1be80 0fc6d200 shufps xmm2, xmm2, 0
140b1be84 0fc6c9ff shufps xmm1, xmm1, 0xff
140b1be88 0f5cca subps xmm1, xmm2
140b1be8b 410f59cd mulps xmm1, xmm13
140b1be8f 410f28c0 movaps xmm0, xmm8
140b1be93 410f59c5 mulps xmm0, xmm13
140b1be97 f30f16d0 movshdup xmm2, xmm0
140b1be9b f30f58d0 addss xmm2, xmm0
140b1be9f 0f12c0 movhlps xmm0, xmm0
140b1bea2 f30f58c2 addss xmm0, xmm2
140b1bea6 f30f58c0 addss xmm0, xmm0
140b1beaa 0fc6c000 shufps xmm0, xmm0, 0
140b1beae 410f59c0 mulps xmm0, xmm8
140b1beb2 0f58c1 addps xmm0, xmm1
140b1beb5 410f28c8 movaps xmm1, xmm8
140b1beb9 410fc6c8d2 shufps xmm1, xmm8, 0xd2
140b1bebe 410f59ce mulps xmm1, xmm14
140b1bec2 410f28d0 movaps xmm2, xmm8
140b1bec6 410fc6d0c9 shufps xmm2, xmm8, 0xc9
140b1becb 410f59d7 mulps xmm2, xmm15
140b1becf 0f5cd1 subps xmm2, xmm1
140b1bed2 450f58c0 addps xmm8, xmm8
140b1bed6 450fc6c0ff shufps xmm8, xmm8, 0xff
140b1bedb 440f59c2 mulps xmm8, xmm2
140b1bedf 440f58c0 addps xmm8, xmm0
140b1bee3 0f28c6 movaps xmm0, xmm6
140b1bee6 f3440f100dd1d0fc01 movss xmm9, dword ptr [rip + 0x1fcd0d1]
140b1beef f3410f59c1 mulss xmm0, xmm9
140b1bef4 e8d780e001 call 0x142923fd0
140b1bef9 440f28d8 movaps xmm11, xmm0
140b1befd 0f28c6 movaps xmm0, xmm6
140b1bf00 f30f103d08aeef01 movss xmm7, dword ptr [rip + 0x1efae08]
140b1bf08 f30f59c7 mulss xmm0, xmm7
140b1bf0c e89f7fe001 call 0x142923eb0
140b1bf11 f3410f59c3 mulss xmm0, xmm11
140b1bf16 f3440f101d5199e001 movss xmm11, dword ptr [rip + 0x1e09951]
140b1bf1f f3410f59c3 mulss xmm0, xmm11
140b1bf24 410f28c8 movaps xmm1, xmm8
140b1bf28 66410f15c8 unpckhpd xmm1, xmm8
140b1bf2d 0f28d1 movaps xmm2, xmm1
140b1bf30 f30f59d0 mulss xmm2, xmm0
140b1bf34 f3410f59d4 mulss xmm2, xmm12
140b1bf39 f30f105c246c movss xmm3, dword ptr [rsp + 0x6c]
140b1bf3f f30f5cda subss xmm3, xmm2
140b1bf43 f30f590dd599e001 mulss xmm1, dword ptr [rip + 0x1e099d5]
140b1bf4b f3410f59cc mulss xmm1, xmm12
140b1bf50 f30f58cb addss xmm1, xmm3
140b1bf54 8b442424 mov eax, dword ptr [rsp + 0x24]
140b1bf58 488b4c2458 mov rcx, qword ptr [rsp + 0x58]
140b1bf5d 8901 mov dword ptr [rcx], eax
140b1bf5f f30f12c0 movsldup xmm0, xmm0
140b1bf63 410f59c0 mulps xmm0, xmm8
140b1bf67 410f59c2 mulps xmm0, xmm10
140b1bf6b 0f289424a0000000 movaps xmm2, xmmword ptr [rsp + 0xa0]
140b1bf73 0f5cd0 subps xmm2, xmm0
140b1bf76 440f590562d0fc01 mulps xmm8, xmmword ptr [rip + 0x1fcd062]
140b1bf7e 450f59c2 mulps xmm8, xmm10
140b1bf82 440f58c2 addps xmm8, xmm2
140b1bf86 450f134610 movlps qword ptr [r14 + 0x10], xmm8
140b1bf8b f3410f114e18 movss dword ptr [r14 + 0x18], xmm1
140b1bf91 488b6c2460 mov rbp, qword ptr [rsp + 0x60]
140b1bf96 e959faffff jmp 0x140b1b9f4
140b1bf9b 0f28b424b0000000 movaps xmm6, xmmword ptr [rsp + 0xb0]
140b1bfa3 0f28bc24c0000000 movaps xmm7, xmmword ptr [rsp + 0xc0]
140b1bfab 440f288424d0000000 movaps xmm8, xmmword ptr [rsp + 0xd0]
140b1bfb4 440f288c24e0000000 movaps xmm9, xmmword ptr [rsp + 0xe0]
140b1bfbd 440f289424f0000000 movaps xmm10, xmmword ptr [rsp + 0xf0]
140b1bfc6 440f289c2400010000 movaps xmm11, xmmword ptr [rsp + 0x100]
140b1bfcf 440f28a42410010000 movaps xmm12, xmmword ptr [rsp + 0x110]
140b1bfd8 440f28ac2420010000 movaps xmm13, xmmword ptr [rsp + 0x120]
140b1bfe1 440f28b42430010000 movaps xmm14, xmmword ptr [rsp + 0x130]
140b1bfea 440f28bc2440010000 movaps xmm15, xmmword ptr [rsp + 0x140]
140b1bff3 4881c458010000 add rsp, 0x158
140b1bffa 5b pop rbx
140b1bffb 5d pop rbp
140b1bffc 5f pop rdi
140b1bffd 5e pop rsi
140b1bffe 415c pop r12
140b1c000 415d pop r13
140b1c002 415e pop r14
140b1c004 415f pop r15
140b1c006 c3 ret 
