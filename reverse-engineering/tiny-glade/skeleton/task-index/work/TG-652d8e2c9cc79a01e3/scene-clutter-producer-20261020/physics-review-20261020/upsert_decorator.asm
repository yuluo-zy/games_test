0x141294980 55 push rbp
0x141294981 4157 push r15
0x141294983 4156 push r14
0x141294985 4155 push r13
0x141294987 4154 push r12
0x141294989 56 push rsi
0x14129498a 57 push rdi
0x14129498b 53 push rbx
0x14129498c 4881ec98030000 sub rsp, 0x398
0x141294993 488dac2480000000 lea rbp, [rsp + 0x80]
0x14129499b 440f29b500030000 movaps xmmword ptr [rbp + 0x300], xmm14
0x1412949a3 440f29adf0020000 movaps xmmword ptr [rbp + 0x2f0], xmm13
0x1412949ab 440f29a5e0020000 movaps xmmword ptr [rbp + 0x2e0], xmm12
0x1412949b3 440f299dd0020000 movaps xmmword ptr [rbp + 0x2d0], xmm11
0x1412949bb 440f2995c0020000 movaps xmmword ptr [rbp + 0x2c0], xmm10
0x1412949c3 440f298db0020000 movaps xmmword ptr [rbp + 0x2b0], xmm9
0x1412949cb 440f2985a0020000 movaps xmmword ptr [rbp + 0x2a0], xmm8
0x1412949d3 0f29bd90020000 movaps xmmword ptr [rbp + 0x290], xmm7
0x1412949da 0f29b580020000 movaps xmmword ptr [rbp + 0x280], xmm6
0x1412949e1 48c78578020000feffffff mov qword ptr [rbp + 0x278], 0xfffffffffffffffe
0x1412949ec 4589ce mov r14d, r9d
0x1412949ef 4d89c7 mov r15, r8
0x1412949f2 4889d7 mov rdi, rdx
0x1412949f5 4889ce mov rsi, rcx
0x1412949f8 4c8bad88030000 mov r13, qword ptr [rbp + 0x388]
0x1412949ff 44898db0010000 mov dword ptr [rbp + 0x1b0], r9d
0x141294a06 4d8d6020 lea r12, [r8 + 0x20]
0x141294a0a 488d95b0010000 lea rdx, [rbp + 0x1b0]
0x141294a11 4c89e1 mov rcx, r12
0x141294a14 e8b7c431ff call 0x1405b0ed0
0x141294a19 4889c3 mov rbx, rax
0x141294a1c 498b07 mov rax, qword ptr [r15]
0x141294a1f 498b4f08 mov rcx, qword ptr [r15 + 8]
0x141294a23 4889ca mov rdx, rcx
0x141294a26 4821da and rdx, rbx
0x141294a29 4989d8 mov r8, rbx
0x141294a2c 49c1e839 shr r8, 0x39
0x141294a30 66410f6ec0 movd xmm0, r8d
0x141294a35 660f60c0 punpcklbw xmm0, xmm0
0x141294a39 f20f70c000 pshuflw xmm0, xmm0, 0
0x141294a3e 660f70c000 pshufd xmm0, xmm0, 0
0x141294a43 4c8d40e8 lea r8, [rax - 0x18]
0x141294a47 4531c9 xor r9d, r9d
0x141294a4a 660f76c9 pcmpeqd xmm1, xmm1
0x141294a4e f30f6f1410 movdqu xmm2, xmmword ptr [rax + rdx]
0x141294a53 660f6fda movdqa xmm3, xmm2
0x141294a57 660f74d8 pcmpeqb xmm3, xmm0
0x141294a5b 66440fd7d3 pmovmskb r10d, xmm3
0x141294a60 4585d2 test r10d, r10d
0x141294a63 742b je 0x141294a90
0x141294a65 f3450fbcda tzcnt r11d, r10d
0x141294a6a 4901d3 add r11, rdx
0x141294a6d 4921cb and r11, rcx
0x141294a70 49f7db neg r11
0x141294a73 4f8d1c5b lea r11, [r11 + r11*2]
0x141294a77 473934d8 cmp dword ptr [r8 + r11*8], r14d
0x141294a7b 7431 je 0x141294aae
0x141294a7d 458d5aff lea r11d, [r10 - 1]
0x141294a81 664521d3 and r11w, r10w
0x141294a85 4589da mov r10d, r11d
0x141294a88 75db jne 0x141294a65
0x141294a8a 660f1f440000 nop word ptr [rax + rax]
0x141294a90 660f74d1 pcmpeqb xmm2, xmm1
0x141294a94 66440fd7d2 pmovmskb r10d, xmm2
0x141294a99 4585d2 test r10d, r10d
0x141294a9c 751c jne 0x141294aba
0x141294a9e 4c01ca add rdx, r9
0x141294aa1 4883c210 add rdx, 0x10
0x141294aa5 4983c110 add r9, 0x10
0x141294aa9 4821ca and rdx, rcx
0x141294aac eba0 jmp 0x141294a4e
0x141294aae 4a8d1cd8 lea rbx, [rax + r11*8]
0x141294ab2 4d89fe mov r14, r15
0x141294ab5 4531ff xor r15d, r15d
0x141294ab8 eb0e jmp 0x141294ac8
0x141294aba 49837f1000 cmp qword ptr [r15 + 0x10], 0
0x141294abf 0f84780f0000 je 0x141295a3d
0x141294ac5 4589f6 mov r14d, r14d
0x141294ac8 488b8580030000 mov rax, qword ptr [rbp + 0x380]
0x141294acf 0f1000 movups xmm0, xmmword ptr [rax]
0x141294ad2 0f104810 movups xmm1, xmmword ptr [rax + 0x10]
0x141294ad6 0f298d90010000 movaps xmmword ptr [rbp + 0x190], xmm1
0x141294add 0f298580010000 movaps xmmword ptr [rbp + 0x180], xmm0
0x141294ae4 410f284500 movaps xmm0, xmmword ptr [r13]
0x141294ae9 66410f6f4d10 movdqa xmm1, xmmword ptr [r13 + 0x10]
0x141294aef 660f7f4d60 movdqa xmmword ptr [rbp + 0x60], xmm1
0x141294af4 0f294550 movaps xmmword ptr [rbp + 0x50], xmm0
0x141294af8 c6857202000001 mov byte ptr [rbp + 0x272], 1
0x141294aff 488d8d80010000 lea rcx, [rbp + 0x180]
0x141294b06 e8159f63ff call 0x1408cea20
0x141294b0b 888571020000 mov byte ptr [rbp + 0x271], al
0x141294b11 4c8b8598010000 mov r8, qword ptr [rbp + 0x198]
0x141294b18 4983c010 add r8, 0x10
0x141294b1c c6857202000001 mov byte ptr [rbp + 0x272], 1
0x141294b23 488d8db0010000 lea rcx, [rbp + 0x1b0]
0x141294b2a 488d5550 lea rdx, [rbp + 0x50]
0x141294b2e e84d1f89ff call 0x140b26a80
0x141294b33 0f28bdb0010000 movaps xmm7, xmmword ptr [rbp + 0x1b0]
0x141294b3a f2440f108dc0010000 movsd xmm9, qword ptr [rbp + 0x1c0]
0x141294b43 f3440f1095c8010000 movss xmm10, dword ptr [rbp + 0x1c8]
0x141294b4c 0f288540020000 movaps xmm0, xmmword ptr [rbp + 0x240]
0x141294b53 0f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm0
0x141294b5a 0f288530020000 movaps xmm0, xmmword ptr [rbp + 0x230]
0x141294b61 0f2985d0000000 movaps xmmword ptr [rbp + 0xd0], xmm0
0x141294b68 0f288520020000 movaps xmm0, xmmword ptr [rbp + 0x220]
0x141294b6f 0f2985c0000000 movaps xmmword ptr [rbp + 0xc0], xmm0
0x141294b76 0f288510020000 movaps xmm0, xmmword ptr [rbp + 0x210]
0x141294b7d 0f2985b0000000 movaps xmmword ptr [rbp + 0xb0], xmm0
0x141294b84 0f288500020000 movaps xmm0, xmmword ptr [rbp + 0x200]
0x141294b8b 0f2985a0000000 movaps xmmword ptr [rbp + 0xa0], xmm0
0x141294b92 0f2885f0010000 movaps xmm0, xmmword ptr [rbp + 0x1f0]
0x141294b99 0f298590000000 movaps xmmword ptr [rbp + 0x90], xmm0
0x141294ba0 0f2885e0010000 movaps xmm0, xmmword ptr [rbp + 0x1e0]
0x141294ba7 0f298580000000 movaps xmmword ptr [rbp + 0x80], xmm0
0x141294bae 0f2885d0010000 movaps xmm0, xmmword ptr [rbp + 0x1d0]
0x141294bb5 0f294570 movaps xmmword ptr [rbp + 0x70], xmm0
0x141294bb9 4d85ff test r15, r15
0x141294bbc 0f8445040000 je 0x141295007
0x141294bc2 488b053f743902 mov rax, qword ptr [rip + 0x239743f]
0x141294bc9 803800 cmp byte ptr [rax], 0
0x141294bcc 0f84e8000000 je 0x141294cba
0x141294bd2 488b05cf773902 mov rax, qword ptr [rip + 0x23977cf]
0x141294bd9 488b00 mov rax, qword ptr [rax]
0x141294bdc 4883f803 cmp rax, 3
0x141294be0 0f82d4000000 jb 0x141294cba
0x141294be6 c6857702000001 mov byte ptr [rbp + 0x277], 1
0x141294bed c6857602000001 mov byte ptr [rbp + 0x276], 1
0x141294bf4 488d0df54d9901 lea rcx, [rip + 0x1994df5]
0x141294bfb e880913eff call 0x14067dd80
0x141294c00 0f1000 movups xmm0, xmmword ptr [rax]
0x141294c03 8b4010 mov eax, dword ptr [rax + 0x10]
0x141294c06 48c785e001000003000000 mov qword ptr [rbp + 0x1e0], 3
0x141294c11 488d0dd04c9901 lea rcx, [rip + 0x1994cd0]
0x141294c18 48898de8010000 mov qword ptr [rbp + 0x1e8], rcx
0x141294c1f 48c785f001000027000000 mov qword ptr [rbp + 0x1f0], 0x27
0x141294c2a 488d15af4d9901 lea rdx, [rip + 0x1994daf]
0x141294c31 48899500020000 mov qword ptr [rbp + 0x200], rdx
0x141294c38 48c7850802000001000000 mov qword ptr [rbp + 0x208], 1
0x141294c43 48c7851002000008000000 mov qword ptr [rbp + 0x210], 8
0x141294c4e 48c785b001000000000000 mov qword ptr [rbp + 0x1b0], 0
0x141294c59 660fefc9 pxor xmm1, xmm1
0x141294c5d f30f7f8d18020000 movdqu xmmword ptr [rbp + 0x218], xmm1
0x141294c65 48898db8010000 mov qword ptr [rbp + 0x1b8], rcx
0x141294c6c 48c785c001000027000000 mov qword ptr [rbp + 0x1c0], 0x27
0x141294c77 48c785c801000000000000 mov qword ptr [rbp + 0x1c8], 0
0x141294c82 0f1185d0010000 movups xmmword ptr [rbp + 0x1d0], xmm0
0x141294c89 c785f801000001000000 mov dword ptr [rbp + 0x1f8], 1
0x141294c93 8985fc010000 mov dword ptr [rbp + 0x1fc], eax
0x141294c99 c6857702000001 mov byte ptr [rbp + 0x277], 1
0x141294ca0 c6857602000001 mov byte ptr [rbp + 0x276], 1
0x141294ca7 488d8d70020000 lea rcx, [rbp + 0x270]
0x141294cae 488d95b0010000 lea rdx, [rbp + 0x1b0]
0x141294cb5 e87656abff call 0x140d4a330
0x141294cba 0f288580010000 movaps xmm0, xmmword ptr [rbp + 0x180]
0x141294cc1 660f6f8d90010000 movdqa xmm1, xmmword ptr [rbp + 0x190]
0x141294cc9 660f7f8dc0010000 movdqa xmmword ptr [rbp + 0x1c0], xmm1
0x141294cd1 0f2985b0010000 movaps xmmword ptr [rbp + 0x1b0], xmm0
0x141294cd8 c6857702000001 mov byte ptr [rbp + 0x277], 1
0x141294cdf c6857602000000 mov byte ptr [rbp + 0x276], 0
0x141294ce6 488d95b0010000 lea rdx, [rbp + 0x1b0]
0x141294ced 4889f9 mov rcx, rdi
0x141294cf0 e84b39d700 call 0x142008640
0x141294cf5 898560020000 mov dword ptr [rbp + 0x260], eax
0x141294cfb 899564020000 mov dword ptr [rbp + 0x264], edx
0x141294d01 0f2885e0000000 movaps xmm0, xmmword ptr [rbp + 0xe0]
0x141294d08 0f298520020000 movaps xmmword ptr [rbp + 0x220], xmm0
0x141294d0f 0f2885d0000000 movaps xmm0, xmmword ptr [rbp + 0xd0]
0x141294d16 0f298510020000 movaps xmmword ptr [rbp + 0x210], xmm0
0x141294d1d 0f2885c0000000 movaps xmm0, xmmword ptr [rbp + 0xc0]
0x141294d24 0f298500020000 movaps xmmword ptr [rbp + 0x200], xmm0
0x141294d2b 0f2885b0000000 movaps xmm0, xmmword ptr [rbp + 0xb0]
0x141294d32 0f2985f0010000 movaps xmmword ptr [rbp + 0x1f0], xmm0
0x141294d39 0f284570 movaps xmm0, xmmword ptr [rbp + 0x70]
0x141294d3d 660f6f8d80000000 movdqa xmm1, xmmword ptr [rbp + 0x80]
0x141294d45 660f6f9590000000 movdqa xmm2, xmmword ptr [rbp + 0x90]
0x141294d4d 660f6f9da0000000 movdqa xmm3, xmmword ptr [rbp + 0xa0]
0x141294d55 660f7f9de0010000 movdqa xmmword ptr [rbp + 0x1e0], xmm3
0x141294d5d 660f7f95d0010000 movdqa xmmword ptr [rbp + 0x1d0], xmm2
0x141294d65 660f7f8dc0010000 movdqa xmmword ptr [rbp + 0x1c0], xmm1
0x141294d6d 0f2985b0010000 movaps xmmword ptr [rbp + 0x1b0], xmm0
0x141294d74 488d8d60020000 lea rcx, [rbp + 0x260]
0x141294d7b e8604c64ff call 0x1408d99e0
0x141294d80 48c1e020 shl rax, 0x20
0x141294d84 89d1 mov ecx, edx
0x141294d86 4809c1 or rcx, rax
0x141294d89 48898dd0010000 mov qword ptr [rbp + 0x1d0], rcx
0x141294d90 48c785d801000000000000 mov qword ptr [rbp + 0x1d8], 0
0x141294d9b 0f2885b0010000 movaps xmm0, xmmword ptr [rbp + 0x1b0]
0x141294da2 660f6f8dc0010000 movdqa xmm1, xmmword ptr [rbp + 0x1c0]
0x141294daa 660f6f95e0010000 movdqa xmm2, xmmword ptr [rbp + 0x1e0]
0x141294db2 660f6f9df0010000 movdqa xmm3, xmmword ptr [rbp + 0x1f0]
0x141294dba 0f298500010000 movaps xmmword ptr [rbp + 0x100], xmm0
0x141294dc1 660f7f8d10010000 movdqa xmmword ptr [rbp + 0x110], xmm1
0x141294dc9 660f7f9530010000 movdqa xmmword ptr [rbp + 0x130], xmm2
0x141294dd1 660f7f9d40010000 movdqa xmmword ptr [rbp + 0x140], xmm3
0x141294dd9 0f288500020000 movaps xmm0, xmmword ptr [rbp + 0x200]
0x141294de0 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
0x141294de7 0f288510020000 movaps xmm0, xmmword ptr [rbp + 0x210]
0x141294dee 0f298560010000 movaps xmmword ptr [rbp + 0x160], xmm0
0x141294df5 0f288520020000 movaps xmm0, xmmword ptr [rbp + 0x220]
0x141294dfc 0f298570010000 movaps xmmword ptr [rbp + 0x170], xmm0
0x141294e03 488b85d0010000 mov rax, qword ptr [rbp + 0x1d0]
0x141294e0a 48898520010000 mov qword ptr [rbp + 0x120], rax
0x141294e11 488b85d8010000 mov rax, qword ptr [rbp + 0x1d8]
0x141294e18 48898528010000 mov qword ptr [rbp + 0x128], rax
0x141294e1f 488d4da0 lea rcx, [rbp - 0x60]
0x141294e23 488d9500010000 lea rdx, [rbp + 0x100]
0x141294e2a e8d1e18fff call 0x140b93000
0x141294e2f 488b8510010000 mov rax, qword ptr [rbp + 0x110]
0x141294e36 f048ff08 lock dec qword ptr [rax]
0x141294e3a 750c jne 0x141294e48
0x141294e3c 488d8d10010000 lea rcx, [rbp + 0x110]
0x141294e43 e888ab3bff call 0x14064f9d0
0x141294e48 83bd0001000002 cmp dword ptr [rbp + 0x100], 2
0x141294e4f 7217 jb 0x141294e68
0x141294e51 488b8d08010000 mov rcx, qword ptr [rbp + 0x108]
0x141294e58 ba2c000000 mov edx, 0x2c
0x141294e5d 41b804000000 mov r8d, 4
0x141294e63 e8b8ed37ff call 0x140613c20
0x141294e68 488d4f20 lea rcx, [rdi + 0x20]
0x141294e6c c6857402000001 mov byte ptr [rbp + 0x274], 1
0x141294e73 4c8d0d8e4b9901 lea r9, [rip + 0x1994b8e]
0x141294e7a ba11000000 mov edx, 0x11
0x141294e7f 440fb68571020000 movzx r8d, byte ptr [rbp + 0x271]
0x141294e87 e8147763ff call 0x1408cc5a0
0x141294e8c 4989c4 mov r12, rax
0x141294e8f c6808803000001 mov byte ptr [rax + 0x388], 1
0x141294e96 c6857402000001 mov byte ptr [rbp + 0x274], 1
0x141294e9d 488d8db0010000 lea rcx, [rbp + 0x1b0]
0x141294ea4 488d55a0 lea rdx, [rbp - 0x60]
0x141294ea8 e8b3e08fff call 0x140b92f60
0x141294ead c6857402000001 mov byte ptr [rbp + 0x274], 1
0x141294eb4 488d8d00010000 lea rcx, [rbp + 0x100]
0x141294ebb 488d95b0010000 lea rdx, [rbp + 0x1b0]
0x141294ec2 e8d94964ff call 0x1408d98a0
0x141294ec7 4c89e1 mov rcx, r12
0x141294eca 4881c188010000 add rcx, 0x188
0x141294ed1 c6857402000001 mov byte ptr [rbp + 0x274], 1
0x141294ed8 488d9500010000 lea rdx, [rbp + 0x100]
0x141294edf e8fcbb6cff call 0x140960ae0
0x141294ee4 4c8dadb0010000 lea r13, [rbp + 0x1b0]
0x141294eeb 488d55a0 lea rdx, [rbp - 0x60]
0x141294eef 41b8b0000000 mov r8d, 0xb0
0x141294ef5 4c89e9 mov rcx, r13
0x141294ef8 e803ed6801 call 0x142923c00
0x141294efd c6857402000000 mov byte ptr [rbp + 0x274], 0
0x141294f04 4c89e1 mov rcx, r12
0x141294f07 4c89ea mov rdx, r13
0x141294f0a e89139d700 call 0x1420088a0
0x141294f0f 4189d1 mov r9d, edx
0x141294f12 c6857402000000 mov byte ptr [rbp + 0x274], 0
0x141294f19 488d8da4010000 lea rcx, [rbp + 0x1a4]
0x141294f20 0fb69571020000 movzx edx, byte ptr [rbp + 0x271]
0x141294f27 4189c0 mov r8d, eax
0x141294f2a e8f17663ff call 0x1408cc620
0x141294f2f 498b07 mov rax, qword ptr [r15]
0x141294f32 498b4f08 mov rcx, qword ptr [r15 + 8]
0x141294f36 4889ca mov rdx, rcx
0x141294f39 4821da and rdx, rbx
0x141294f3c f30f6f0410 movdqu xmm0, xmmword ptr [rax + rdx]
0x141294f41 66440fd7c0 pmovmskb r8d, xmm0
0x141294f46 4585c0 test r8d, r8d
0x141294f49 0f84b20a0000 je 0x141295a01
0x141294f4f f20f108560020000 movsd xmm0, qword ptr [rbp + 0x260]
0x141294f57 f3450fbcc0 tzcnt r8d, r8d
0x141294f5c 4901d0 add r8, rdx
0x141294f5f 4921c8 and r8, rcx
0x141294f62 420fb61400 movzx edx, byte ptr [rax + r8]
0x141294f67 84d2 test dl, dl
0x141294f69 0f89b70a0000 jns 0x141295a26
0x141294f6f 48c1eb39 shr rbx, 0x39
0x141294f73 4d8d48f0 lea r9, [r8 - 0x10]
0x141294f77 4921c9 and r9, rcx
0x141294f7a 42881c00 mov byte ptr [rax + r8], bl
0x141294f7e 42885c0810 mov byte ptr [rax + r9 + 0x10], bl
0x141294f83 80e201 and dl, 1
0x141294f86 0fb6ca movzx ecx, dl
0x141294f89 49294f10 sub qword ptr [r15 + 0x10], rcx
0x141294f8d 49f7d8 neg r8
0x141294f90 4b8d0c40 lea rcx, [r8 + r8*2]
0x141294f94 448974c8e8 mov dword ptr [rax + rcx*8 - 0x18], r14d
0x141294f99 0f1344c8ec movlps qword ptr [rax + rcx*8 - 0x14], xmm0
0x141294f9e 488b95a4010000 mov rdx, qword ptr [rbp + 0x1a4]
0x141294fa5 488954c8f4 mov qword ptr [rax + rcx*8 - 0xc], rdx
0x141294faa 8b95ac010000 mov edx, dword ptr [rbp + 0x1ac]
0x141294fb0 8954c8fc mov dword ptr [rax + rcx*8 - 4], edx
0x141294fb4 49ff4718 inc qword ptr [r15 + 0x18]
0x141294fb8 8b85ac010000 mov eax, dword ptr [rbp + 0x1ac]
0x141294fbe 8985b8010000 mov dword ptr [rbp + 0x1b8], eax
0x141294fc4 488b85a4010000 mov rax, qword ptr [rbp + 0x1a4]
0x141294fcb 488985b0010000 mov qword ptr [rbp + 0x1b0], rax
0x141294fd2 c6857402000000 mov byte ptr [rbp + 0x274], 0
0x141294fd9 488d95b0010000 lea rdx, [rbp + 0x1b0]
0x141294fe0 4889f9 mov rcx, rdi
0x141294fe3 e8789463ff call 0x1408ce460
0x141294fe8 8b85ac010000 mov eax, dword ptr [rbp + 0x1ac]
0x141294fee 894608 mov dword ptr [rsi + 8], eax
0x141294ff1 488b85a4010000 mov rax, qword ptr [rbp + 0x1a4]
0x141294ff8 488906 mov qword ptr [rsi], rax
0x141294ffb c7460c00000000 mov dword ptr [rsi + 0xc], 0
0x141295002 e99d090000 jmp 0x1412959a4
0x141295007 f20f1073f4 movsd xmm6, qword ptr [rbx - 0xc]
0x14129500c 8b43fc mov eax, dword ptr [rbx - 4]
0x14129500f 8985b8010000 mov dword ptr [rbp + 0x1b8], eax
0x141295015 488b43f4 mov rax, qword ptr [rbx - 0xc]
0x141295019 488985b0010000 mov qword ptr [rbp + 0x1b0], rax
0x141295020 c6857702000001 mov byte ptr [rbp + 0x277], 1
0x141295027 c6857602000001 mov byte ptr [rbp + 0x276], 1
0x14129502e 488d4da0 lea rcx, [rbp - 0x60]
0x141295032 4c8d85b0010000 lea r8, [rbp + 0x1b0]
0x141295039 4889fa mov rdx, rdi
0x14129503c e80f8b63ff call 0x1408cdb50
0x141295041 4c8d6bf4 lea r13, [rbx - 0xc]
0x141295045 448b7dc0 mov r15d, dword ptr [rbp - 0x40]
0x141295049 4585ff test r15d, r15d
0x14129504c 0f8441050000 je 0x141295593
0x141295052 440f2845a0 movaps xmm8, xmmword ptr [rbp - 0x60]
0x141295057 f2440f1065b0 movsd xmm12, qword ptr [rbp - 0x50]
0x14129505d f3440f105db8 movss xmm11, dword ptr [rbp - 0x48]
0x141295063 448b65c4 mov r12d, dword ptr [rbp - 0x3c]
0x141295067 450f2ee1 ucomiss xmm12, xmm9
0x14129506b 7506 jne 0x141295073
0x14129506d 0f8b40010000 jnp 0x1412951b3
0x141295073 488b058e6f3902 mov rax, qword ptr [rip + 0x2396f8e]
0x14129507a 803800 cmp byte ptr [rax], 0
0x14129507d 0f8410050000 je 0x141295593
0x141295083 488b051e733902 mov rax, qword ptr [rip + 0x239731e]
0x14129508a 488b00 mov rax, qword ptr [rax]
0x14129508d 4883f803 cmp rax, 3
0x141295091 0f821d020000 jb 0x1412952b4
0x141295097 450f5ccc subps xmm9, xmm12
0x14129509b f3450f5cd3 subss xmm10, xmm11
0x1412950a0 440f138d60020000 movlps qword ptr [rbp + 0x260], xmm9
0x1412950a8 f3440f119568020000 movss dword ptr [rbp + 0x268], xmm10
0x1412950b1 488d8560020000 lea rax, [rbp + 0x260]
0x1412950b8 48898500010000 mov qword ptr [rbp + 0x100], rax
0x1412950bf 488d050a67abff lea rax, [rip - 0x5498f6]
0x1412950c6 48898508010000 mov qword ptr [rbp + 0x108], rax
0x1412950cd c6857702000001 mov byte ptr [rbp + 0x277], 1
0x1412950d4 c6857602000001 mov byte ptr [rbp + 0x276], 1
0x1412950db 488d0dee479901 lea rcx, [rip + 0x19947ee]
0x1412950e2 e8998c3eff call 0x14067dd80
0x1412950e7 0f1000 movups xmm0, xmmword ptr [rax]
0x1412950ea 8b4010 mov eax, dword ptr [rax + 0x10]
0x1412950ed 48c785e001000003000000 mov qword ptr [rbp + 0x1e0], 3
0x1412950f8 488d0de9479901 lea rcx, [rip + 0x19947e9]
0x1412950ff 48898de8010000 mov qword ptr [rbp + 0x1e8], rcx
0x141295106 48c785f001000027000000 mov qword ptr [rbp + 0x1f0], 0x27
0x141295111 488d15a8479901 lea rdx, [rip + 0x19947a8]
0x141295118 48899500020000 mov qword ptr [rbp + 0x200], rdx
0x14129511f 48c7850802000001000000 mov qword ptr [rbp + 0x208], 1
0x14129512a 488d9500010000 lea rdx, [rbp + 0x100]
0x141295131 48899510020000 mov qword ptr [rbp + 0x210], rdx
0x141295138 48c7851802000001000000 mov qword ptr [rbp + 0x218], 1
0x141295143 48c7852002000000000000 mov qword ptr [rbp + 0x220], 0
0x14129514e 48c785b001000000000000 mov qword ptr [rbp + 0x1b0], 0
0x141295159 48898db8010000 mov qword ptr [rbp + 0x1b8], rcx
0x141295160 48c785c001000027000000 mov qword ptr [rbp + 0x1c0], 0x27
0x14129516b 48c785c801000000000000 mov qword ptr [rbp + 0x1c8], 0
0x141295176 0f1185d0010000 movups xmmword ptr [rbp + 0x1d0], xmm0
0x14129517d c785f801000001000000 mov dword ptr [rbp + 0x1f8], 1
0x141295187 8985fc010000 mov dword ptr [rbp + 0x1fc], eax
0x14129518d c6857702000001 mov byte ptr [rbp + 0x277], 1
0x141295194 c6857602000001 mov byte ptr [rbp + 0x276], 1
0x14129519b 488d8d70020000 lea rcx, [rbp + 0x270]
0x1412951a2 488d95b0010000 lea rdx, [rbp + 0x1b0]
0x1412951a9 e88251abff call 0x140d4a330
0x1412951ae e901010000 jmp 0x1412952b4
0x1412951b3 f3450f16e9 movshdup xmm13, xmm9
0x1412951b8 f3450f16f4 movshdup xmm14, xmm12
0x1412951bd 450f2ef5 ucomiss xmm14, xmm13
0x1412951c1 0f85bd000000 jne 0x141295284
0x1412951c7 0f8ab7000000 jp 0x141295284
0x1412951cd 450f2eda ucomiss xmm11, xmm10
0x1412951d1 0f85ad000000 jne 0x141295284
0x1412951d7 0f8aa7000000 jp 0x141295284
0x1412951dd 410f28c0 movaps xmm0, xmm8
0x1412951e1 0fc2c700 cmpeqps xmm0, xmm7
0x1412951e5 0f50c0 movmskps eax, xmm0
0x1412951e8 83f80f cmp eax, 0xf
0x1412951eb 0f8593000000 jne 0x141295284
0x1412951f1 c6857702000001 mov byte ptr [rbp + 0x277], 1
0x1412951f8 c6857602000001 mov byte ptr [rbp + 0x276], 1
0x1412951ff 4889f9 mov rcx, rdi
0x141295202 4489fa mov edx, r15d
0x141295205 4589e0 mov r8d, r12d
0x141295208 e8c38963ff call 0x1408cdbd0
0x14129520d c6857702000001 mov byte ptr [rbp + 0x277], 1
0x141295214 c6857602000001 mov byte ptr [rbp + 0x276], 1
0x14129521b 488d9580010000 lea rdx, [rbp + 0x180]
0x141295222 4889c1 mov rcx, rax
0x141295225 e8069863ff call 0x1408cea30
0x14129522a 84c0 test al, al
0x14129522c 7456 je 0x141295284
0x14129522e 418b4508 mov eax, dword ptr [r13 + 8]
0x141295232 894608 mov dword ptr [rsi + 8], eax
0x141295235 498b4500 mov rax, qword ptr [r13]
0x141295239 488906 mov qword ptr [rsi], rax
0x14129523c c7460c02000000 mov dword ptr [rsi + 0xc], 2
0x141295243 498b4500 mov rax, qword ptr [r13]
0x141295247 48894610 mov qword ptr [rsi + 0x10], rax
0x14129524b c6857202000001 mov byte ptr [rbp + 0x272], 1
0x141295252 488d4d70 lea rcx, [rbp + 0x70]
0x141295256 e8e54b36ff call 0x1405f9e40
0x14129525b 488b8598010000 mov rax, qword ptr [rbp + 0x198]
0x141295262 f048ff08 lock dec qword ptr [rax]
0x141295266 0f8538070000 jne 0x1412959a4
0x14129526c 488d8d98010000 lea rcx, [rbp + 0x198]
0x141295273 c6857302000000 mov byte ptr [rbp + 0x273], 0
0x14129527a e8a1738cff call 0x140b5c620
0x14129527f e920070000 jmp 0x1412959a4
0x141295284 488b057d6d3902 mov rax, qword ptr [rip + 0x2396d7d]
0x14129528b 803800 cmp byte ptr [rax], 0
0x14129528e 0f84ff020000 je 0x141295593
0x141295294 450f2eee ucomiss xmm13, xmm14
0x141295298 0f85e5fdffff jne 0x141295083
0x14129529e 0f8adffdffff jp 0x141295083
0x1412952a4 450f2ed3 ucomiss xmm10, xmm11
0x1412952a8 0f85d5fdffff jne 0x141295083
0x1412952ae 0f8acffdffff jp 0x141295083
0x1412952b4 0f28c7 movaps xmm0, xmm7
0x1412952b7 410fc2c000 cmpeqps xmm0, xmm8
0x1412952bc 0f50c0 movmskps eax, xmm0
0x1412952bf 83f80f cmp eax, 0xf
0x1412952c2 0f8427010000 je 0x1412953ef
0x1412952c8 488b05d9703902 mov rax, qword ptr [rip + 0x23970d9]
0x1412952cf 488b00 mov rax, qword ptr [rax]
0x1412952d2 4883f803 cmp rax, 3
0x1412952d6 0f8213010000 jb 0x1412953ef
0x1412952dc 410f5cf8 subps xmm7, xmm8
0x1412952e0 0f13bd60020000 movlps qword ptr [rbp + 0x260], xmm7
0x1412952e7 0f12ff movhlps xmm7, xmm7
0x1412952ea f30f11bd68020000 movss dword ptr [rbp + 0x268], xmm7
0x1412952f2 488d8560020000 lea rax, [rbp + 0x260]
0x1412952f9 48898500010000 mov qword ptr [rbp + 0x100], rax
0x141295300 488d05c964abff lea rax, [rip - 0x549b37]
0x141295307 48898508010000 mov qword ptr [rbp + 0x108], rax
0x14129530e c6857702000001 mov byte ptr [rbp + 0x277], 1
0x141295315 c6857602000001 mov byte ptr [rbp + 0x276], 1
0x14129531c 488d0ded459901 lea rcx, [rip + 0x19945ed]
0x141295323 e8588a3eff call 0x14067dd80
0x141295328 0f1000 movups xmm0, xmmword ptr [rax]
0x14129532b 8b4010 mov eax, dword ptr [rax + 0x10]
0x14129532e 48c785e001000003000000 mov qword ptr [rbp + 0x1e0], 3
0x141295339 488d0da8459901 lea rcx, [rip + 0x19945a8]
0x141295340 48898de8010000 mov qword ptr [rbp + 0x1e8], rcx
0x141295347 48c785f001000027000000 mov qword ptr [rbp + 0x1f0], 0x27
0x141295352 488d1567459901 lea rdx, [rip + 0x1994567]
0x141295359 48899500020000 mov qword ptr [rbp + 0x200], rdx
0x141295360 48c7850802000001000000 mov qword ptr [rbp + 0x208], 1
0x14129536b 488d9500010000 lea rdx, [rbp + 0x100]
0x141295372 48899510020000 mov qword ptr [rbp + 0x210], rdx
0x141295379 48c7851802000001000000 mov qword ptr [rbp + 0x218], 1
0x141295384 48c7852002000000000000 mov qword ptr [rbp + 0x220], 0
0x14129538f 48c785b001000000000000 mov qword ptr [rbp + 0x1b0], 0
0x14129539a 48898db8010000 mov qword ptr [rbp + 0x1b8], rcx
0x1412953a1 48c785c001000027000000 mov qword ptr [rbp + 0x1c0], 0x27
0x1412953ac 48c785c801000000000000 mov qword ptr [rbp + 0x1c8], 0
0x1412953b7 0f1185d0010000 movups xmmword ptr [rbp + 0x1d0], xmm0
0x1412953be c785f801000001000000 mov dword ptr [rbp + 0x1f8], 1
0x1412953c8 8985fc010000 mov dword ptr [rbp + 0x1fc], eax
0x1412953ce c6857702000001 mov byte ptr [rbp + 0x277], 1
0x1412953d5 c6857602000001 mov byte ptr [rbp + 0x276], 1
0x1412953dc 488d8d70020000 lea rcx, [rbp + 0x270]
0x1412953e3 488d95b0010000 lea rdx, [rbp + 0x1b0]
0x1412953ea e8414fabff call 0x140d4a330
0x1412953ef c6857702000001 mov byte ptr [rbp + 0x277], 1
0x1412953f6 c6857602000001 mov byte ptr [rbp + 0x276], 1
0x1412953fd 4889f9 mov rcx, rdi
0x141295400 4489fa mov edx, r15d
0x141295403 4589e0 mov r8d, r12d
0x141295406 e8c58763ff call 0x1408cdbd0
0x14129540b c6857702000001 mov byte ptr [rbp + 0x277], 1
0x141295412 c6857602000001 mov byte ptr [rbp + 0x276], 1
0x141295419 488d9580010000 lea rdx, [rbp + 0x180]
0x141295420 4889c1 mov rcx, rax
0x141295423 e8089663ff call 0x1408cea30
0x141295428 84c0 test al, al
0x14129542a 0f8563010000 jne 0x141295593
0x141295430 488b05716f3902 mov rax, qword ptr [rip + 0x2396f71]
0x141295437 488b00 mov rax, qword ptr [rax]
0x14129543a 4883f803 cmp rax, 3
0x14129543e 0f824f010000 jb 0x141295593
0x141295444 c6857702000001 mov byte ptr [rbp + 0x277], 1
0x14129544b c6857602000001 mov byte ptr [rbp + 0x276], 1
0x141295452 4889f9 mov rcx, rdi
0x141295455 4489fa mov edx, r15d
0x141295458 4589e0 mov r8d, r12d
0x14129545b e8708763ff call 0x1408cdbd0
0x141295460 488b4018 mov rax, qword ptr [rax + 0x18]
0x141295464 4883c010 add rax, 0x10
0x141295468 488985f8000000 mov qword ptr [rbp + 0xf8], rax
0x14129546f 488b8598010000 mov rax, qword ptr [rbp + 0x198]
0x141295476 4883c010 add rax, 0x10
0x14129547a 48898560020000 mov qword ptr [rbp + 0x260], rax
0x141295481 488d85f8000000 lea rax, [rbp + 0xf8]
0x141295488 48898500010000 mov qword ptr [rbp + 0x100], rax
0x14129548f 488d055a79afff lea rax, [rip - 0x5086a6]
0x141295496 48898508010000 mov qword ptr [rbp + 0x108], rax
0x14129549d 488d8d60020000 lea rcx, [rbp + 0x260]
0x1412954a4 48898d10010000 mov qword ptr [rbp + 0x110], rcx
0x1412954ab 48898518010000 mov qword ptr [rbp + 0x118], rax
0x1412954b2 c6857702000001 mov byte ptr [rbp + 0x277], 1
0x1412954b9 c6857602000001 mov byte ptr [rbp + 0x276], 1
0x1412954c0 488d0d91449901 lea rcx, [rip + 0x1994491]
0x1412954c7 e8b4883eff call 0x14067dd80
0x1412954cc 0f1000 movups xmm0, xmmword ptr [rax]
0x1412954cf 8b4010 mov eax, dword ptr [rax + 0x10]
0x1412954d2 48c785e001000003000000 mov qword ptr [rbp + 0x1e0], 3
0x1412954dd 488d0d04449901 lea rcx, [rip + 0x1994404]
0x1412954e4 48898de8010000 mov qword ptr [rbp + 0x1e8], rcx
0x1412954eb 48c785f001000027000000 mov qword ptr [rbp + 0x1f0], 0x27
0x1412954f6 488d153b449901 lea rdx, [rip + 0x199443b]
0x1412954fd 48899500020000 mov qword ptr [rbp + 0x200], rdx
0x141295504 48c7850802000002000000 mov qword ptr [rbp + 0x208], 2
0x14129550f 488d9500010000 lea rdx, [rbp + 0x100]
0x141295516 48899510020000 mov qword ptr [rbp + 0x210], rdx
0x14129551d 48c7851802000002000000 mov qword ptr [rbp + 0x218], 2
0x141295528 48c7852002000000000000 mov qword ptr [rbp + 0x220], 0
0x141295533 48c785b001000000000000 mov qword ptr [rbp + 0x1b0], 0
0x14129553e 48898db8010000 mov qword ptr [rbp + 0x1b8], rcx
0x141295545 48c785c001000027000000 mov qword ptr [rbp + 0x1c0], 0x27
0x141295550 48c785c801000000000000 mov qword ptr [rbp + 0x1c8], 0
0x14129555b 0f1185d0010000 movups xmmword ptr [rbp + 0x1d0], xmm0
0x141295562 c785f801000001000000 mov dword ptr [rbp + 0x1f8], 1
0x14129556c 8985fc010000 mov dword ptr [rbp + 0x1fc], eax
0x141295572 c6857702000001 mov byte ptr [rbp + 0x277], 1
0x141295579 c6857602000001 mov byte ptr [rbp + 0x276], 1
0x141295580 488d8d70020000 lea rcx, [rbp + 0x270]
0x141295587 488d95b0010000 lea rdx, [rbp + 0x1b0]
0x14129558e e89d4dabff call 0x140d4a330
0x141295593 488b056e6a3902 mov rax, qword ptr [rip + 0x2396a6e]
0x14129559a 803800 cmp byte ptr [rax], 0
0x14129559d 0f84e8000000 je 0x14129568b
0x1412955a3 488b05fe6d3902 mov rax, qword ptr [rip + 0x2396dfe]
0x1412955aa 488b00 mov rax, qword ptr [rax]
0x1412955ad 4883f803 cmp rax, 3
0x1412955b1 0f82d4000000 jb 0x14129568b
0x1412955b7 c6857702000001 mov byte ptr [rbp + 0x277], 1
0x1412955be c6857602000001 mov byte ptr [rbp + 0x276], 1
0x1412955c5 488d0dcc439901 lea rcx, [rip + 0x19943cc]
0x1412955cc e8af873eff call 0x14067dd80
0x1412955d1 0f1000 movups xmm0, xmmword ptr [rax]
0x1412955d4 8b4010 mov eax, dword ptr [rax + 0x10]
0x1412955d7 48c785e001000003000000 mov qword ptr [rbp + 0x1e0], 3
0x1412955e2 488d0dff429901 lea rcx, [rip + 0x19942ff]
0x1412955e9 48898de8010000 mov qword ptr [rbp + 0x1e8], rcx
0x1412955f0 48c785f001000027000000 mov qword ptr [rbp + 0x1f0], 0x27
0x1412955fb 488d1586439901 lea rdx, [rip + 0x1994386]
0x141295602 48899500020000 mov qword ptr [rbp + 0x200], rdx
0x141295609 48c7850802000001000000 mov qword ptr [rbp + 0x208], 1
0x141295614 48c7851002000008000000 mov qword ptr [rbp + 0x210], 8
0x14129561f 48c785b001000000000000 mov qword ptr [rbp + 0x1b0], 0
0x14129562a 660fefc9 pxor xmm1, xmm1
0x14129562e f30f7f8d18020000 movdqu xmmword ptr [rbp + 0x218], xmm1
0x141295636 48898db8010000 mov qword ptr [rbp + 0x1b8], rcx
0x14129563d 48c785c001000027000000 mov qword ptr [rbp + 0x1c0], 0x27
0x141295648 48c785c801000000000000 mov qword ptr [rbp + 0x1c8], 0
0x141295653 0f1185d0010000 movups xmmword ptr [rbp + 0x1d0], xmm0
0x14129565a c785f801000001000000 mov dword ptr [rbp + 0x1f8], 1
0x141295664 8985fc010000 mov dword ptr [rbp + 0x1fc], eax
0x14129566a c6857702000001 mov byte ptr [rbp + 0x277], 1
0x141295671 c6857602000001 mov byte ptr [rbp + 0x276], 1
0x141295678 488d8d70020000 lea rcx, [rbp + 0x270]
0x14129567f 488d95b0010000 lea rdx, [rbp + 0x1b0]
0x141295686 e8a54cabff call 0x140d4a330
0x14129568b 4c8d7bec lea r15, [rbx - 0x14]
0x14129568f c6857702000001 mov byte ptr [rbp + 0x277], 1
0x141295696 c6857602000001 mov byte ptr [rbp + 0x276], 1
0x14129569d 488d8db0010000 lea rcx, [rbp + 0x1b0]
0x1412956a4 4889fa mov rdx, rdi
0x1412956a7 4d89f8 mov r8, r15
0x1412956aa e8e13e64ff call 0x1408d9590
0x1412956af 488b85e8010000 mov rax, qword ptr [rbp + 0x1e8]
0x1412956b6 f048ff08 lock dec qword ptr [rax]
0x1412956ba 751a jne 0x1412956d6
0x1412956bc 488d8de8010000 lea rcx, [rbp + 0x1e8]
0x1412956c3 c6857702000001 mov byte ptr [rbp + 0x277], 1
0x1412956ca c6857602000001 mov byte ptr [rbp + 0x276], 1
0x1412956d1 e84a6f8cff call 0x140b5c620
0x1412956d6 0f288580010000 movaps xmm0, xmmword ptr [rbp + 0x180]
0x1412956dd 660f6f8d90010000 movdqa xmm1, xmmword ptr [rbp + 0x190]
0x1412956e5 660f7f8dc0010000 movdqa xmmword ptr [rbp + 0x1c0], xmm1
0x1412956ed 0f2985b0010000 movaps xmmword ptr [rbp + 0x1b0], xmm0
0x1412956f4 c6857702000001 mov byte ptr [rbp + 0x277], 1
0x1412956fb c6857602000000 mov byte ptr [rbp + 0x276], 0
0x141295702 488d95b0010000 lea rdx, [rbp + 0x1b0]
0x141295709 4889f9 mov rcx, rdi
0x14129570c e82f2fd700 call 0x142008640
0x141295711 8943ec mov dword ptr [rbx - 0x14], eax
0x141295714 8953f0 mov dword ptr [rbx - 0x10], edx
0x141295717 0f2885e0000000 movaps xmm0, xmmword ptr [rbp + 0xe0]
0x14129571e 0f298520020000 movaps xmmword ptr [rbp + 0x220], xmm0
0x141295725 0f2885d0000000 movaps xmm0, xmmword ptr [rbp + 0xd0]
0x14129572c 0f298510020000 movaps xmmword ptr [rbp + 0x210], xmm0
0x141295733 0f2885c0000000 movaps xmm0, xmmword ptr [rbp + 0xc0]
0x14129573a 0f298500020000 movaps xmmword ptr [rbp + 0x200], xmm0
0x141295741 0f2885b0000000 movaps xmm0, xmmword ptr [rbp + 0xb0]
0x141295748 0f2985f0010000 movaps xmmword ptr [rbp + 0x1f0], xmm0
0x14129574f 0f284570 movaps xmm0, xmmword ptr [rbp + 0x70]
0x141295753 660f6f8d80000000 movdqa xmm1, xmmword ptr [rbp + 0x80]
0x14129575b 660f6f9590000000 movdqa xmm2, xmmword ptr [rbp + 0x90]
0x141295763 660f6f9da0000000 movdqa xmm3, xmmword ptr [rbp + 0xa0]
0x14129576b 660f7f9de0010000 movdqa xmmword ptr [rbp + 0x1e0], xmm3
0x141295773 660f7f95d0010000 movdqa xmmword ptr [rbp + 0x1d0], xmm2
0x14129577b 660f7f8dc0010000 movdqa xmmword ptr [rbp + 0x1c0], xmm1
0x141295783 0f2985b0010000 movaps xmmword ptr [rbp + 0x1b0], xmm0
0x14129578a 4c89f9 mov rcx, r15
0x14129578d e84e4264ff call 0x1408d99e0
0x141295792 48c1e020 shl rax, 0x20
0x141295796 89d1 mov ecx, edx
0x141295798 4809c1 or rcx, rax
0x14129579b 48898dd0010000 mov qword ptr [rbp + 0x1d0], rcx
0x1412957a2 48c785d801000000000000 mov qword ptr [rbp + 0x1d8], 0
0x1412957ad 0f2885b0010000 movaps xmm0, xmmword ptr [rbp + 0x1b0]
0x1412957b4 660f6f8dc0010000 movdqa xmm1, xmmword ptr [rbp + 0x1c0]
0x1412957bc 660f6f95e0010000 movdqa xmm2, xmmword ptr [rbp + 0x1e0]
0x1412957c4 660f6f9df0010000 movdqa xmm3, xmmword ptr [rbp + 0x1f0]
0x1412957cc 0f298500010000 movaps xmmword ptr [rbp + 0x100], xmm0
0x1412957d3 660f7f8d10010000 movdqa xmmword ptr [rbp + 0x110], xmm1
0x1412957db 660f7f9530010000 movdqa xmmword ptr [rbp + 0x130], xmm2
0x1412957e3 660f7f9d40010000 movdqa xmmword ptr [rbp + 0x140], xmm3
0x1412957eb 0f288500020000 movaps xmm0, xmmword ptr [rbp + 0x200]
0x1412957f2 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
0x1412957f9 0f288510020000 movaps xmm0, xmmword ptr [rbp + 0x210]
0x141295800 0f298560010000 movaps xmmword ptr [rbp + 0x160], xmm0
0x141295807 0f288520020000 movaps xmm0, xmmword ptr [rbp + 0x220]
0x14129580e 0f298570010000 movaps xmmword ptr [rbp + 0x170], xmm0
0x141295815 488b85d0010000 mov rax, qword ptr [rbp + 0x1d0]
0x14129581c 48898520010000 mov qword ptr [rbp + 0x120], rax
0x141295823 488b85d8010000 mov rax, qword ptr [rbp + 0x1d8]
0x14129582a 48898528010000 mov qword ptr [rbp + 0x128], rax
0x141295831 488d4da0 lea rcx, [rbp - 0x60]
0x141295835 488d9500010000 lea rdx, [rbp + 0x100]
0x14129583c e8bfd78fff call 0x140b93000
0x141295841 488b8510010000 mov rax, qword ptr [rbp + 0x110]
0x141295848 f048ff08 lock dec qword ptr [rax]
0x14129584c 750c jne 0x14129585a
0x14129584e 488d8d10010000 lea rcx, [rbp + 0x110]
0x141295855 e876a13bff call 0x14064f9d0
0x14129585a 83bd0001000002 cmp dword ptr [rbp + 0x100], 2
0x141295861 7217 jb 0x14129587a
0x141295863 488b8d08010000 mov rcx, qword ptr [rbp + 0x108]
0x14129586a ba2c000000 mov edx, 0x2c
0x14129586f 41b804000000 mov r8d, 4
0x141295875 e8a6e337ff call 0x140613c20
0x14129587a 488d4f20 lea rcx, [rdi + 0x20]
0x14129587e c6857502000001 mov byte ptr [rbp + 0x275], 1
0x141295885 4c8d0d24419901 lea r9, [rip + 0x1994124]
0x14129588c ba11000000 mov edx, 0x11
0x141295891 440fb68571020000 movzx r8d, byte ptr [rbp + 0x271]
0x141295899 e8026d63ff call 0x1408cc5a0
0x14129589e 4889c3 mov rbx, rax
0x1412958a1 c6808803000001 mov byte ptr [rax + 0x388], 1
0x1412958a8 c6857502000001 mov byte ptr [rbp + 0x275], 1
0x1412958af 488d8db0010000 lea rcx, [rbp + 0x1b0]
0x1412958b6 488d55a0 lea rdx, [rbp - 0x60]
0x1412958ba e8a1d68fff call 0x140b92f60
0x1412958bf c6857502000001 mov byte ptr [rbp + 0x275], 1
0x1412958c6 488d8d00010000 lea rcx, [rbp + 0x100]
0x1412958cd 488d95b0010000 lea rdx, [rbp + 0x1b0]
0x1412958d4 e8c73f64ff call 0x1408d98a0
0x1412958d9 4889d9 mov rcx, rbx
0x1412958dc 4881c188010000 add rcx, 0x188
0x1412958e3 c6857502000001 mov byte ptr [rbp + 0x275], 1
0x1412958ea 488d9500010000 lea rdx, [rbp + 0x100]
0x1412958f1 e8eab16cff call 0x140960ae0
0x1412958f6 4c8dbdb0010000 lea r15, [rbp + 0x1b0]
0x1412958fd 488d55a0 lea rdx, [rbp - 0x60]
0x141295901 41b8b0000000 mov r8d, 0xb0
0x141295907 4c89f9 mov rcx, r15
0x14129590a e8f1e26801 call 0x142923c00
0x14129590f c6857502000000 mov byte ptr [rbp + 0x275], 0
0x141295916 4889d9 mov rcx, rbx
0x141295919 4c89fa mov rdx, r15
0x14129591c e87f2fd700 call 0x1420088a0
0x141295921 4189d1 mov r9d, edx
0x141295924 c6857502000000 mov byte ptr [rbp + 0x275], 0
0x14129592b 488d8d00010000 lea rcx, [rbp + 0x100]
0x141295932 0fb69571020000 movzx edx, byte ptr [rbp + 0x271]
0x141295939 4189c0 mov r8d, eax
0x14129593c e8df6c63ff call 0x1408cc620
0x141295941 8b8508010000 mov eax, dword ptr [rbp + 0x108]
0x141295947 41894508 mov dword ptr [r13 + 8], eax
0x14129594b 488b8500010000 mov rax, qword ptr [rbp + 0x100]
0x141295952 49894500 mov qword ptr [r13], rax
0x141295956 8b8508010000 mov eax, dword ptr [rbp + 0x108]
0x14129595c 8985b8010000 mov dword ptr [rbp + 0x1b8], eax
0x141295962 488b8500010000 mov rax, qword ptr [rbp + 0x100]
0x141295969 488985b0010000 mov qword ptr [rbp + 0x1b0], rax
0x141295970 c6857502000000 mov byte ptr [rbp + 0x275], 0
0x141295977 488d95b0010000 lea rdx, [rbp + 0x1b0]
0x14129597e 4889f9 mov rcx, rdi
0x141295981 e8da8a63ff call 0x1408ce460
0x141295986 8b8508010000 mov eax, dword ptr [rbp + 0x108]
0x14129598c 894608 mov dword ptr [rsi + 8], eax
0x14129598f 488b8500010000 mov rax, qword ptr [rbp + 0x100]
0x141295996 488906 mov qword ptr [rsi], rax
0x141295999 c7460c01000000 mov dword ptr [rsi + 0xc], 1
0x1412959a0 0f137610 movlps qword ptr [rsi + 0x10], xmm6
0x1412959a4 4889f0 mov rax, rsi
0x1412959a7 0f28b580020000 movaps xmm6, xmmword ptr [rbp + 0x280]
0x1412959ae 0f28bd90020000 movaps xmm7, xmmword ptr [rbp + 0x290]
0x1412959b5 440f2885a0020000 movaps xmm8, xmmword ptr [rbp + 0x2a0]
0x1412959bd 440f288db0020000 movaps xmm9, xmmword ptr [rbp + 0x2b0]
0x1412959c5 440f2895c0020000 movaps xmm10, xmmword ptr [rbp + 0x2c0]
0x1412959cd 440f289dd0020000 movaps xmm11, xmmword ptr [rbp + 0x2d0]
0x1412959d5 440f28a5e0020000 movaps xmm12, xmmword ptr [rbp + 0x2e0]
0x1412959dd 440f28adf0020000 movaps xmm13, xmmword ptr [rbp + 0x2f0]
0x1412959e5 440f28b500030000 movaps xmm14, xmmword ptr [rbp + 0x300]
0x1412959ed 4881c498030000 add rsp, 0x398
0x1412959f4 5b pop rbx
0x1412959f5 5f pop rdi
0x1412959f6 5e pop rsi
0x1412959f7 415c pop r12
0x1412959f9 415d pop r13
0x1412959fb 415e pop r14
0x1412959fd 415f pop r15
0x1412959ff 5d pop rbp
0x141295a00 c3 ret
0x141295a01 41b910000000 mov r9d, 0x10
0x141295a07 4c01ca add rdx, r9
0x141295a0a 4821ca and rdx, rcx
0x141295a0d f30f6f0410 movdqu xmm0, xmmword ptr [rax + rdx]
0x141295a12 66440fd7c0 pmovmskb r8d, xmm0
0x141295a17 4983c110 add r9, 0x10
0x141295a1b 4585c0 test r8d, r8d
0x141295a1e 0f852bf5ffff jne 0x141294f4f
0x141295a24 ebe1 jmp 0x141295a07
0x141295a26 660f6f08 movdqa xmm1, xmmword ptr [rax]
0x141295a2a 660fd7d1 pmovmskb edx, xmm1
0x141295a2e f3440fbcc2 tzcnt r8d, edx
0x141295a33 420fb61400 movzx edx, byte ptr [rax + r8]
0x141295a38 e932f5ffff jmp 0x141294f6f
0x141295a3d c6857302000001 mov byte ptr [rbp + 0x273], 1
0x141295a44 ba01000000 mov edx, 1
0x141295a49 4c89f9 mov rcx, r15
0x141295a4c 4d89e0 mov r8, r12
0x141295a4f 41b101 mov r9b, 1
0x141295a52 e8e9c06101 call 0x1428b1b40
0x141295a57 e969f0ffff jmp 0x141294ac5
0x141295a5c 0f1f4000 nop dword ptr [rax]
0x141295a60 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141295a65 55 push rbp
0x141295a66 4157 push r15
0x141295a68 4156 push r14
0x141295a6a 4155 push r13
0x141295a6c 4154 push r12
0x141295a6e 56 push rsi
0x141295a6f 57 push rdi
0x141295a70 53 push rbx
0x141295a71 4881ecb8000000 sub rsp, 0xb8
0x141295a78 488daa80000000 lea rbp, [rdx + 0x80]
0x141295a7f 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
0x141295a85 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
0x141295a8b 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
0x141295a91 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
0x141295a97 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
0x141295a9d 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
0x141295aa3 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
0x141295aac 0f29bc2490000000 movaps xmmword ptr [rsp + 0x90], xmm7
0x141295ab4 0f29b424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm6
0x141295abc 488d8d00010000 lea rcx, [rbp + 0x100]
0x141295ac3 e8784336ff call 0x1405f9e40
0x141295ac8 c6857702000000 mov byte ptr [rbp + 0x277], 0
0x141295acf c6857602000000 mov byte ptr [rbp + 0x276], 0
0x141295ad6 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
0x141295ade 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
0x141295ae6 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
0x141295aef 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
0x141295af5 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
0x141295afb 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
0x141295b01 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
0x141295b07 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
0x141295b0d 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
0x141295b13 4881c4b8000000 add rsp, 0xb8
0x141295b1a 5b pop rbx
0x141295b1b 5f pop rdi
0x141295b1c 5e pop rsi
0x141295b1d 415c pop r12
0x141295b1f 415d pop r13
0x141295b21 415e pop r14
0x141295b23 415f pop r15
0x141295b25 5d pop rbp
0x141295b26 c3 ret
0x141295b27 660f1f840000000000 nop word ptr [rax + rax]
0x141295b30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141295b35 55 push rbp
0x141295b36 4157 push r15
0x141295b38 4156 push r14
0x141295b3a 4155 push r13
0x141295b3c 4154 push r12
0x141295b3e 56 push rsi
0x141295b3f 57 push rdi
0x141295b40 53 push rbx
0x141295b41 4881ecb8000000 sub rsp, 0xb8
0x141295b48 488daa80000000 lea rbp, [rdx + 0x80]
0x141295b4f 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
0x141295b55 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
0x141295b5b 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
0x141295b61 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
0x141295b67 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
0x141295b6d 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
0x141295b73 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
0x141295b7c 0f29bc2490000000 movaps xmmword ptr [rsp + 0x90], xmm7
0x141295b84 0f29b424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm6
0x141295b8c 83bd0001000002 cmp dword ptr [rbp + 0x100], 2
0x141295b93 7217 jb 0x141295bac
0x141295b95 488b8d08010000 mov rcx, qword ptr [rbp + 0x108]
0x141295b9c ba2c000000 mov edx, 0x2c
0x141295ba1 41b804000000 mov r8d, 4
0x141295ba7 e874e037ff call 0x140613c20
0x141295bac c6857502000001 mov byte ptr [rbp + 0x275], 1
0x141295bb3 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
0x141295bbb 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
0x141295bc3 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
0x141295bcc 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
0x141295bd2 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
0x141295bd8 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
0x141295bde 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
0x141295be4 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
0x141295bea 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
0x141295bf0 4881c4b8000000 add rsp, 0xb8
0x141295bf7 5b pop rbx
0x141295bf8 5f pop rdi
0x141295bf9 5e pop rsi
0x141295bfa 415c pop r12
0x141295bfc 415d pop r13
0x141295bfe 415e pop r14
0x141295c00 415f pop r15
0x141295c02 5d pop rbp
0x141295c03 c3 ret
0x141295c04 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x141295c10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141295c15 55 push rbp
0x141295c16 4157 push r15
0x141295c18 4156 push r14
0x141295c1a 4155 push r13
0x141295c1c 4154 push r12
0x141295c1e 56 push rsi
0x141295c1f 57 push rdi
0x141295c20 53 push rbx
0x141295c21 4881ecb8000000 sub rsp, 0xb8
0x141295c28 488daa80000000 lea rbp, [rdx + 0x80]
0x141295c2f 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
0x141295c35 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
0x141295c3b 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
0x141295c41 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
0x141295c47 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
0x141295c4d 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
0x141295c53 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
0x141295c5c 0f29bc2490000000 movaps xmmword ptr [rsp + 0x90], xmm7
0x141295c64 0f29b424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm6
0x141295c6c 80bd7502000000 cmp byte ptr [rbp + 0x275], 0
0x141295c73 7468 je 0x141295cdd
0x141295c75 488d4da0 lea rcx, [rbp - 0x60]
0x141295c79 e822e835ff call 0x1405f44a0
0x141295c7e c6857702000000 mov byte ptr [rbp + 0x277], 0
0x141295c85 c6857602000000 mov byte ptr [rbp + 0x276], 0
0x141295c8c 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
0x141295c94 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
0x141295c9c 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
0x141295ca5 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
0x141295cab 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
0x141295cb1 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
0x141295cb7 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
0x141295cbd 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
0x141295cc3 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
0x141295cc9 4881c4b8000000 add rsp, 0xb8
0x141295cd0 5b pop rbx
0x141295cd1 5f pop rdi
0x141295cd2 5e pop rsi
0x141295cd3 415c pop r12
0x141295cd5 415d pop r13
0x141295cd7 415e pop r14
0x141295cd9 415f pop r15
0x141295cdb 5d pop rbp
0x141295cdc c3 ret
0x141295cdd c6857702000000 mov byte ptr [rbp + 0x277], 0
0x141295ce4 c6857602000000 mov byte ptr [rbp + 0x276], 0
0x141295ceb 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
0x141295cf3 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
0x141295cfb 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
0x141295d04 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
0x141295d0a 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
0x141295d10 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
0x141295d16 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
0x141295d1c 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
0x141295d22 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
0x141295d28 4881c4b8000000 add rsp, 0xb8
0x141295d2f 5b pop rbx
0x141295d30 5f pop rdi
0x141295d31 5e pop rsi
0x141295d32 415c pop r12
0x141295d34 415d pop r13
0x141295d36 415e pop r14
0x141295d38 415f pop r15
0x141295d3a 5d pop rbp
0x141295d3b c3 ret
0x141295d3c 0f1f4000 nop dword ptr [rax]
0x141295d40 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141295d45 55 push rbp
0x141295d46 4157 push r15
0x141295d48 4156 push r14
0x141295d4a 4155 push r13
0x141295d4c 4154 push r12
0x141295d4e 56 push rsi
0x141295d4f 57 push rdi
0x141295d50 53 push rbx
0x141295d51 4881ecb8000000 sub rsp, 0xb8
0x141295d58 488daa80000000 lea rbp, [rdx + 0x80]
0x141295d5f 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
0x141295d65 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
0x141295d6b 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
0x141295d71 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
0x141295d77 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
0x141295d7d 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
0x141295d83 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
0x141295d8c 0f29bc2490000000 movaps xmmword ptr [rsp + 0x90], xmm7
0x141295d94 0f29b424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm6
0x141295d9c 488d8db0010000 lea rcx, [rbp + 0x1b0]
0x141295da3 e8984036ff call 0x1405f9e40
0x141295da8 c6857702000000 mov byte ptr [rbp + 0x277], 0
0x141295daf c6857602000000 mov byte ptr [rbp + 0x276], 0
0x141295db6 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
0x141295dbe 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
0x141295dc6 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
0x141295dcf 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
0x141295dd5 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
0x141295ddb 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
0x141295de1 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
0x141295de7 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
0x141295ded 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
0x141295df3 4881c4b8000000 add rsp, 0xb8
0x141295dfa 5b pop rbx
0x141295dfb 5f pop rdi
0x141295dfc 5e pop rsi
0x141295dfd 415c pop r12
0x141295dff 415d pop r13
0x141295e01 415e pop r14
0x141295e03 415f pop r15
0x141295e05 5d pop rbp
0x141295e06 c3 ret
0x141295e07 660f1f840000000000 nop word ptr [rax + rax]
0x141295e10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141295e15 55 push rbp
0x141295e16 4157 push r15
0x141295e18 4156 push r14
0x141295e1a 4155 push r13
0x141295e1c 4154 push r12
0x141295e1e 56 push rsi
0x141295e1f 57 push rdi
0x141295e20 53 push rbx
0x141295e21 4881ecb8000000 sub rsp, 0xb8
0x141295e28 488daa80000000 lea rbp, [rdx + 0x80]
0x141295e2f 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
0x141295e35 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
0x141295e3b 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
0x141295e41 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
0x141295e47 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
0x141295e4d 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
0x141295e53 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
0x141295e5c 0f29bc2490000000 movaps xmmword ptr [rsp + 0x90], xmm7
0x141295e64 0f29b424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm6
0x141295e6c 488d8d00010000 lea rcx, [rbp + 0x100]
0x141295e73 e8c83f36ff call 0x1405f9e40
0x141295e78 c6857702000000 mov byte ptr [rbp + 0x277], 0
0x141295e7f c6857602000000 mov byte ptr [rbp + 0x276], 0
0x141295e86 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
0x141295e8e 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
0x141295e96 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
0x141295e9f 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
0x141295ea5 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
0x141295eab 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
0x141295eb1 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
0x141295eb7 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
0x141295ebd 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
0x141295ec3 4881c4b8000000 add rsp, 0xb8
0x141295eca 5b pop rbx
0x141295ecb 5f pop rdi
0x141295ecc 5e pop rsi
0x141295ecd 415c pop r12
0x141295ecf 415d pop r13
0x141295ed1 415e pop r14
0x141295ed3 415f pop r15
0x141295ed5 5d pop rbp
0x141295ed6 c3 ret
0x141295ed7 660f1f840000000000 nop word ptr [rax + rax]
0x141295ee0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141295ee5 55 push rbp
0x141295ee6 4157 push r15
0x141295ee8 4156 push r14
0x141295eea 4155 push r13
0x141295eec 4154 push r12
0x141295eee 56 push rsi
0x141295eef 57 push rdi
0x141295ef0 53 push rbx
0x141295ef1 4881ecb8000000 sub rsp, 0xb8
0x141295ef8 488daa80000000 lea rbp, [rdx + 0x80]
0x141295eff 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
0x141295f05 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
0x141295f0b 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
0x141295f11 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
0x141295f17 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
0x141295f1d 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
0x141295f23 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
0x141295f2c 0f29bc2490000000 movaps xmmword ptr [rsp + 0x90], xmm7
0x141295f34 0f29b424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm6
0x141295f3c 83bd0001000002 cmp dword ptr [rbp + 0x100], 2
0x141295f43 7217 jb 0x141295f5c
0x141295f45 488b8d08010000 mov rcx, qword ptr [rbp + 0x108]
0x141295f4c ba2c000000 mov edx, 0x2c
0x141295f51 41b804000000 mov r8d, 4
0x141295f57 e8c4dc37ff call 0x140613c20
0x141295f5c c6857402000001 mov byte ptr [rbp + 0x274], 1
0x141295f63 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
0x141295f6b 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
0x141295f73 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
0x141295f7c 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
0x141295f82 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
0x141295f88 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
0x141295f8e 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
0x141295f94 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
0x141295f9a 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
0x141295fa0 4881c4b8000000 add rsp, 0xb8
0x141295fa7 5b pop rbx
0x141295fa8 5f pop rdi
0x141295fa9 5e pop rsi
0x141295faa 415c pop r12
0x141295fac 415d pop r13
0x141295fae 415e pop r14
0x141295fb0 415f pop r15
0x141295fb2 5d pop rbp
0x141295fb3 c3 ret
0x141295fb4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x141295fc0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141295fc5 55 push rbp
0x141295fc6 4157 push r15
0x141295fc8 4156 push r14
0x141295fca 4155 push r13
0x141295fcc 4154 push r12
0x141295fce 56 push rsi
0x141295fcf 57 push rdi
0x141295fd0 53 push rbx
0x141295fd1 4881ecb8000000 sub rsp, 0xb8
0x141295fd8 488daa80000000 lea rbp, [rdx + 0x80]
0x141295fdf 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
0x141295fe5 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
0x141295feb 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
0x141295ff1 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
0x141295ff7 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
0x141295ffd 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
0x141296003 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
0x14129600c 0f29bc2490000000 movaps xmmword ptr [rsp + 0x90], xmm7
0x141296014 0f29b424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm6
0x14129601c 80bd7402000000 cmp byte ptr [rbp + 0x274], 0
0x141296023 7468 je 0x14129608d
0x141296025 488d4da0 lea rcx, [rbp - 0x60]
0x141296029 e872e435ff call 0x1405f44a0
0x14129602e c6857702000000 mov byte ptr [rbp + 0x277], 0
0x141296035 c6857602000000 mov byte ptr [rbp + 0x276], 0
0x14129603c 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
0x141296044 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
0x14129604c 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
0x141296055 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
0x14129605b 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
0x141296061 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
0x141296067 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
0x14129606d 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
0x141296073 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
0x141296079 4881c4b8000000 add rsp, 0xb8
0x141296080 5b pop rbx
0x141296081 5f pop rdi
0x141296082 5e pop rsi
0x141296083 415c pop r12
0x141296085 415d pop r13
0x141296087 415e pop r14
0x141296089 415f pop r15
0x14129608b 5d pop rbp
0x14129608c c3 ret
0x14129608d c6857702000000 mov byte ptr [rbp + 0x277], 0
0x141296094 c6857602000000 mov byte ptr [rbp + 0x276], 0
0x14129609b 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
0x1412960a3 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
0x1412960ab 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
0x1412960b4 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
0x1412960ba 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
0x1412960c0 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
0x1412960c6 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
0x1412960cc 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
0x1412960d2 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
0x1412960d8 4881c4b8000000 add rsp, 0xb8
0x1412960df 5b pop rbx
0x1412960e0 5f pop rdi
0x1412960e1 5e pop rsi
0x1412960e2 415c pop r12
0x1412960e4 415d pop r13
0x1412960e6 415e pop r14
0x1412960e8 415f pop r15
0x1412960ea 5d pop rbp
0x1412960eb c3 ret
0x1412960ec 0f1f4000 nop dword ptr [rax]
0x1412960f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1412960f5 55 push rbp
0x1412960f6 4157 push r15
0x1412960f8 4156 push r14
0x1412960fa 4155 push r13
0x1412960fc 4154 push r12
0x1412960fe 56 push rsi
0x1412960ff 57 push rdi
0x141296100 53 push rbx
0x141296101 4881ecb8000000 sub rsp, 0xb8
0x141296108 488daa80000000 lea rbp, [rdx + 0x80]
0x14129610f 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
0x141296115 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
0x14129611b 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
0x141296121 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
0x141296127 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
0x14129612d 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
0x141296133 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
0x14129613c 0f29bc2490000000 movaps xmmword ptr [rsp + 0x90], xmm7
0x141296144 0f29b424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm6
0x14129614c 488d8db0010000 lea rcx, [rbp + 0x1b0]
0x141296153 e8e83c36ff call 0x1405f9e40
0x141296158 c6857702000000 mov byte ptr [rbp + 0x277], 0
0x14129615f c6857602000000 mov byte ptr [rbp + 0x276], 0
0x141296166 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
0x14129616e 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
0x141296176 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
0x14129617f 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
0x141296185 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
0x14129618b 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
0x141296191 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
0x141296197 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
0x14129619d 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
0x1412961a3 4881c4b8000000 add rsp, 0xb8
0x1412961aa 5b pop rbx
0x1412961ab 5f pop rdi
0x1412961ac 5e pop rsi
0x1412961ad 415c pop r12
0x1412961af 415d pop r13
0x1412961b1 415e pop r14
0x1412961b3 415f pop r15
0x1412961b5 5d pop rbp
0x1412961b6 c3 ret
0x1412961b7 660f1f840000000000 nop word ptr [rax + rax]
0x1412961c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1412961c5 55 push rbp
0x1412961c6 4157 push r15
0x1412961c8 4156 push r14
0x1412961ca 4155 push r13
0x1412961cc 4154 push r12
0x1412961ce 56 push rsi
0x1412961cf 57 push rdi
0x1412961d0 53 push rbx
0x1412961d1 4881ecb8000000 sub rsp, 0xb8
0x1412961d8 488daa80000000 lea rbp, [rdx + 0x80]
0x1412961df 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
0x1412961e5 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
0x1412961eb 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
0x1412961f1 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
0x1412961f7 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
0x1412961fd 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
0x141296203 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
0x14129620c 0f29bc2490000000 movaps xmmword ptr [rsp + 0x90], xmm7
0x141296214 0f29b424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm6
0x14129621c 0fb69d76020000 movzx ebx, byte ptr [rbp + 0x276]
0x141296223 80bd7702000000 cmp byte ptr [rbp + 0x277], 0
0x14129622a 7463 je 0x14129628f
0x14129622c 488d4d70 lea rcx, [rbp + 0x70]
0x141296230 e80b3c36ff call 0x1405f9e40
0x141296235 80e301 and bl, 1
0x141296238 889d72020000 mov byte ptr [rbp + 0x272], bl
0x14129623e 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
0x141296246 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
0x14129624e 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
0x141296257 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
0x14129625d 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
0x141296263 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
0x141296269 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
0x14129626f 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
0x141296275 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
0x14129627b 4881c4b8000000 add rsp, 0xb8
0x141296282 5b pop rbx
0x141296283 5f pop rdi
0x141296284 5e pop rsi
0x141296285 415c pop r12
0x141296287 415d pop r13
0x141296289 415e pop r14
0x14129628b 415f pop r15
0x14129628d 5d pop rbp
0x14129628e c3 ret
0x14129628f 80e301 and bl, 1
0x141296292 889d72020000 mov byte ptr [rbp + 0x272], bl
0x141296298 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
0x1412962a0 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
0x1412962a8 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
0x1412962b1 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
0x1412962b7 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
0x1412962bd 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
0x1412962c3 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
0x1412962c9 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
0x1412962cf 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
0x1412962d5 4881c4b8000000 add rsp, 0xb8
0x1412962dc 5b pop rbx
0x1412962dd 5f pop rdi
0x1412962de 5e pop rsi
0x1412962df 415c pop r12
0x1412962e1 415d pop r13
0x1412962e3 415e pop r14
0x1412962e5 415f pop r15
0x1412962e7 5d pop rbp
0x1412962e8 c3 ret
0x1412962e9 0f1f8000000000 nop dword ptr [rax]
0x1412962f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1412962f5 55 push rbp
0x1412962f6 4157 push r15
0x1412962f8 4156 push r14
0x1412962fa 4155 push r13
0x1412962fc 4154 push r12
0x1412962fe 56 push rsi
0x1412962ff 57 push rdi
0x141296300 53 push rbx
0x141296301 4881ecb8000000 sub rsp, 0xb8
0x141296308 488daa80000000 lea rbp, [rdx + 0x80]
0x14129630f 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
0x141296315 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
0x14129631b 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
0x141296321 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
0x141296327 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
0x14129632d 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
0x141296333 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
0x14129633c 0f29bc2490000000 movaps xmmword ptr [rsp + 0x90], xmm7
0x141296344 0f29b424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm6
0x14129634c 80bd7202000000 cmp byte ptr [rbp + 0x272], 0
0x141296353 7478 je 0x1412963cd
0x141296355 488b8598010000 mov rax, qword ptr [rbp + 0x198]
0x14129635c f048ff08 lock dec qword ptr [rax]
0x141296360 7513 jne 0x141296375
0x141296362 488d8d98010000 lea rcx, [rbp + 0x198]
0x141296369 c6857302000000 mov byte ptr [rbp + 0x273], 0
0x141296370 e8ab628cff call 0x140b5c620
0x141296375 c6857302000000 mov byte ptr [rbp + 0x273], 0
0x14129637c 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
0x141296384 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
0x14129638c 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
0x141296395 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
0x14129639b 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
0x1412963a1 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
0x1412963a7 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
0x1412963ad 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
0x1412963b3 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
0x1412963b9 4881c4b8000000 add rsp, 0xb8
0x1412963c0 5b pop rbx
0x1412963c1 5f pop rdi
0x1412963c2 5e pop rsi
0x1412963c3 415c pop r12
0x1412963c5 415d pop r13
0x1412963c7 415e pop r14
0x1412963c9 415f pop r15
0x1412963cb 5d pop rbp
0x1412963cc c3 ret
0x1412963cd c6857302000000 mov byte ptr [rbp + 0x273], 0
0x1412963d4 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
0x1412963dc 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
0x1412963e4 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
0x1412963ed 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
0x1412963f3 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
0x1412963f9 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
0x1412963ff 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
0x141296405 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
0x14129640b 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
0x141296411 4881c4b8000000 add rsp, 0xb8
0x141296418 5b pop rbx
0x141296419 5f pop rdi
0x14129641a 5e pop rsi
0x14129641b 415c pop r12
0x14129641d 415d pop r13
0x14129641f 415e pop r14
0x141296421 415f pop r15
0x141296423 5d pop rbp
0x141296424 c3 ret
0x141296425 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x141296430 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141296435 55 push rbp
0x141296436 4157 push r15
0x141296438 4156 push r14
0x14129643a 4155 push r13
0x14129643c 4154 push r12
0x14129643e 56 push rsi
0x14129643f 57 push rdi
0x141296440 53 push rbx
0x141296441 4881ecb8000000 sub rsp, 0xb8
0x141296448 488daa80000000 lea rbp, [rdx + 0x80]
0x14129644f 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
0x141296455 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
0x14129645b 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
0x141296461 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
0x141296467 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
0x14129646d 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
0x141296473 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
0x14129647c 0f29bc2490000000 movaps xmmword ptr [rsp + 0x90], xmm7
0x141296484 0f29b424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm6
0x14129648c 80bd7302000000 cmp byte ptr [rbp + 0x273], 0
0x141296493 7421 je 0x1412964b6
0x141296495 488b8580030000 mov rax, qword ptr [rbp + 0x380]
0x14129649c 488b4018 mov rax, qword ptr [rax + 0x18]
0x1412964a0 f048ff08 lock dec qword ptr [rax]
0x1412964a4 7510 jne 0x1412964b6
0x1412964a6 488b8d80030000 mov rcx, qword ptr [rbp + 0x380]
0x1412964ad 4883c118 add rcx, 0x18
0x1412964b1 e86a618cff call 0x140b5c620
0x1412964b6 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
0x1412964be 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
0x1412964c6 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
0x1412964cf 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
0x1412964d5 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
0x1412964db 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
0x1412964e1 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
0x1412964e7 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
0x1412964ed 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
0x1412964f3 4881c4b8000000 add rsp, 0xb8
0x1412964fa 5b pop rbx
0x1412964fb 5f pop rdi
0x1412964fc 5e pop rsi
0x1412964fd 415c pop r12
0x1412964ff 415d pop r13
0x141296501 415e pop r14
0x141296503 415f pop r15
0x141296505 5d pop rbp
0x141296506 c3 ret
