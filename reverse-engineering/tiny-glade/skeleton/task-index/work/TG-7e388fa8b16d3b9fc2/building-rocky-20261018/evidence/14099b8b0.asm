0x14099b8b0 55 push rbp
0x14099b8b1 56 push rsi
0x14099b8b2 57 push rdi
0x14099b8b3 53 push rbx
0x14099b8b4 4881ecb8040000 sub rsp, 0x4b8
0x14099b8bb 488dac2480000000 lea rbp, [rsp + 0x80]
0x14099b8c3 48c78530040000feffffff mov qword ptr [rbp + 0x430], 0xfffffffffffffffe
0x14099b8ce 4889ce mov rsi, rcx
0x14099b8d1 0fb6053193c902 movzx eax, byte ptr [rip + 0x2c99331]
0x14099b8d8 b980030000 mov ecx, 0x380
0x14099b8dd ba08000000 mov edx, 8
0x14099b8e2 e82983c7ff call 0x140613c10
0x14099b8e7 4885c0 test rax, rax
0x14099b8ea 0f84f1040000 je 0x14099bde1
0x14099b8f0 48898528040000 mov qword ptr [rbp + 0x428], rax
0x14099b8f7 0f57c0 xorps xmm0, xmm0
0x14099b8fa 0f118508030000 movups xmmword ptr [rbp + 0x308], xmm0
0x14099b901 0f118520030000 movups xmmword ptr [rbp + 0x320], xmm0
0x14099b908 0f118538030000 movups xmmword ptr [rbp + 0x338], xmm0
0x14099b90f 0f118550030000 movups xmmword ptr [rbp + 0x350], xmm0
0x14099b916 0f118568030000 movups xmmword ptr [rbp + 0x368], xmm0
0x14099b91d c7857803000000000000 mov dword ptr [rbp + 0x378], 0
0x14099b927 0f1185a0030000 movups xmmword ptr [rbp + 0x3a0], xmm0
0x14099b92e 0f1185b8030000 movups xmmword ptr [rbp + 0x3b8], xmm0
0x14099b935 0f1185d0030000 movups xmmword ptr [rbp + 0x3d0], xmm0
0x14099b93c 0f1185e8030000 movups xmmword ptr [rbp + 0x3e8], xmm0
0x14099b943 0f118500040000 movups xmmword ptr [rbp + 0x400], xmm0
0x14099b94a c7851004000000000000 mov dword ptr [rbp + 0x410], 0
0x14099b954 48c785b002000000000000 mov qword ptr [rbp + 0x2b0], 0
0x14099b95f 48c785e802000000000000 mov qword ptr [rbp + 0x2e8], 0
0x14099b96a 48c785f002000008000000 mov qword ptr [rbp + 0x2f0], 8
0x14099b975 48c785f802000000000000 mov qword ptr [rbp + 0x2f8], 0
0x14099b980 48c7850003000010000000 mov qword ptr [rbp + 0x300], 0x10
0x14099b98b 48c7851803000010000000 mov qword ptr [rbp + 0x318], 0x10
0x14099b996 48c7853003000010000000 mov qword ptr [rbp + 0x330], 0x10
0x14099b9a1 48c7854803000010000000 mov qword ptr [rbp + 0x348], 0x10
0x14099b9ac 48c7856003000010000000 mov qword ptr [rbp + 0x360], 0x10
0x14099b9b7 48bb0000000000000080 movabs rbx, 0x8000000000000000
0x14099b9c1 48899d80030000 mov qword ptr [rbp + 0x380], rbx
0x14099b9c8 488d05d9001202 lea rax, [rip + 0x21200d9]
0x14099b9cf 48898588030000 mov qword ptr [rbp + 0x388], rax
0x14099b9d6 48c7859003000045000000 mov qword ptr [rbp + 0x390], 0x45
0x14099b9e1 48c7859803000010000000 mov qword ptr [rbp + 0x398], 0x10
0x14099b9ec 48c785b003000010000000 mov qword ptr [rbp + 0x3b0], 0x10
0x14099b9f7 48c785c803000010000000 mov qword ptr [rbp + 0x3c8], 0x10
0x14099ba02 48c785e003000010000000 mov qword ptr [rbp + 0x3e0], 0x10
0x14099ba0d 48c785f803000010000000 mov qword ptr [rbp + 0x3f8], 0x10
0x14099ba18 c7851804000000000000 mov dword ptr [rbp + 0x418], 0
0x14099ba22 66c7851c0400000100 mov word ptr [rbp + 0x41c], 1
0x14099ba2b c7852004000000000000 mov dword ptr [rbp + 0x420], 0
0x14099ba35 0fb605cd91c902 movzx eax, byte ptr [rip + 0x2c991cd]
0x14099ba3c b978010000 mov ecx, 0x178
0x14099ba41 ba08000000 mov edx, 8
0x14099ba46 e8c581c7ff call 0x140613c10
0x14099ba4b 4885c0 test rax, rax
0x14099ba4e 0f849c030000 je 0x14099bdf0
0x14099ba54 4889c7 mov rdi, rax
0x14099ba57 488d95b0020000 lea rdx, [rbp + 0x2b0]
0x14099ba5e 41b878010000 mov r8d, 0x178
0x14099ba64 4889c1 mov rcx, rax
0x14099ba67 e89481f801 call 0x142923c00
0x14099ba6c 4c8d05cdfc1102 lea r8, [rip + 0x211fccd]
0x14099ba73 488d8d40020000 lea rcx, [rbp + 0x240]
0x14099ba7a 4889fa mov rdx, rdi
0x14099ba7d e88e903400 call 0x140ce4b10
0x14099ba82 488d8dd0010000 lea rcx, [rbp + 0x1d0]
0x14099ba89 e872570d00 call 0x140a71200
0x14099ba8e 488d8d60010000 lea rcx, [rbp + 0x160]
0x14099ba95 e896630700 call 0x140a11e30
0x14099ba9a 488d8df0000000 lea rcx, [rbp + 0xf0]
0x14099baa1 e83a96dc00 call 0x1417650e0
0x14099baa6 488d8d80000000 lea rcx, [rbp + 0x80]
0x14099baad e8bee8dc00 call 0x14176a370
0x14099bab2 488d4d10 lea rcx, [rbp + 0x10]
0x14099bab6 e8550b0b00 call 0x140a4c610
0x14099babb 488d4da0 lea rcx, [rbp - 0x60]
0x14099babf e84cf6cb00 call 0x14165b110
0x14099bac4 488d8db0020000 lea rcx, [rbp + 0x2b0]
0x14099bacb e82096f2ff call 0x1408c50f0
0x14099bad0 0f1085a0020000 movups xmm0, xmmword ptr [rbp + 0x2a0]
0x14099bad7 488b8528040000 mov rax, qword ptr [rbp + 0x428]
0x14099bade 0f114060 movups xmmword ptr [rax + 0x60], xmm0
0x14099bae2 0f108590020000 movups xmm0, xmmword ptr [rbp + 0x290]
0x14099bae9 0f114050 movups xmmword ptr [rax + 0x50], xmm0
0x14099baed 0f108580020000 movups xmm0, xmmword ptr [rbp + 0x280]
0x14099baf4 0f114040 movups xmmword ptr [rax + 0x40], xmm0
0x14099baf8 0f108540020000 movups xmm0, xmmword ptr [rbp + 0x240]
0x14099baff 0f108d50020000 movups xmm1, xmmword ptr [rbp + 0x250]
0x14099bb06 0f109560020000 movups xmm2, xmmword ptr [rbp + 0x260]
0x14099bb0d 0f109d70020000 movups xmm3, xmmword ptr [rbp + 0x270]
0x14099bb14 0f115830 movups xmmword ptr [rax + 0x30], xmm3
0x14099bb18 0f115020 movups xmmword ptr [rax + 0x20], xmm2
0x14099bb1c 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x14099bb20 0f1100 movups xmmword ptr [rax], xmm0
0x14099bb23 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
0x14099bb2a 0f108de0010000 movups xmm1, xmmword ptr [rbp + 0x1e0]
0x14099bb31 0f1095f0010000 movups xmm2, xmmword ptr [rbp + 0x1f0]
0x14099bb38 0f109d00020000 movups xmm3, xmmword ptr [rbp + 0x200]
0x14099bb3f 0f114070 movups xmmword ptr [rax + 0x70], xmm0
0x14099bb43 0f118880000000 movups xmmword ptr [rax + 0x80], xmm1
0x14099bb4a 0f119090000000 movups xmmword ptr [rax + 0x90], xmm2
0x14099bb51 0f1198a0000000 movups xmmword ptr [rax + 0xa0], xmm3
0x14099bb58 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
0x14099bb5f 0f1180b0000000 movups xmmword ptr [rax + 0xb0], xmm0
0x14099bb66 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
0x14099bb6d 0f1180c0000000 movups xmmword ptr [rax + 0xc0], xmm0
0x14099bb74 0f108530020000 movups xmm0, xmmword ptr [rbp + 0x230]
0x14099bb7b 0f1180d0000000 movups xmmword ptr [rax + 0xd0], xmm0
0x14099bb82 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
0x14099bb89 0f118040010000 movups xmmword ptr [rax + 0x140], xmm0
0x14099bb90 0f1085b0010000 movups xmm0, xmmword ptr [rbp + 0x1b0]
0x14099bb97 0f118030010000 movups xmmword ptr [rax + 0x130], xmm0
0x14099bb9e 0f1085a0010000 movups xmm0, xmmword ptr [rbp + 0x1a0]
0x14099bba5 0f118020010000 movups xmmword ptr [rax + 0x120], xmm0
0x14099bbac 0f108560010000 movups xmm0, xmmword ptr [rbp + 0x160]
0x14099bbb3 0f108d70010000 movups xmm1, xmmword ptr [rbp + 0x170]
0x14099bbba 0f109580010000 movups xmm2, xmmword ptr [rbp + 0x180]
0x14099bbc1 0f109d90010000 movups xmm3, xmmword ptr [rbp + 0x190]
0x14099bbc8 0f119810010000 movups xmmword ptr [rax + 0x110], xmm3
0x14099bbcf 0f119000010000 movups xmmword ptr [rax + 0x100], xmm2
0x14099bbd6 0f1188f0000000 movups xmmword ptr [rax + 0xf0], xmm1
0x14099bbdd 0f1180e0000000 movups xmmword ptr [rax + 0xe0], xmm0
0x14099bbe4 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
0x14099bbeb 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
0x14099bbf2 0f109510010000 movups xmm2, xmmword ptr [rbp + 0x110]
0x14099bbf9 0f109d20010000 movups xmm3, xmmword ptr [rbp + 0x120]
0x14099bc00 0f118050010000 movups xmmword ptr [rax + 0x150], xmm0
0x14099bc07 0f118860010000 movups xmmword ptr [rax + 0x160], xmm1
0x14099bc0e 0f119070010000 movups xmmword ptr [rax + 0x170], xmm2
0x14099bc15 0f119880010000 movups xmmword ptr [rax + 0x180], xmm3
0x14099bc1c 0f108530010000 movups xmm0, xmmword ptr [rbp + 0x130]
0x14099bc23 0f118090010000 movups xmmword ptr [rax + 0x190], xmm0
0x14099bc2a 0f108540010000 movups xmm0, xmmword ptr [rbp + 0x140]
0x14099bc31 0f1180a0010000 movups xmmword ptr [rax + 0x1a0], xmm0
0x14099bc38 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
0x14099bc3f 0f1180b0010000 movups xmmword ptr [rax + 0x1b0], xmm0
0x14099bc46 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
0x14099bc4d 0f108d90000000 movups xmm1, xmmword ptr [rbp + 0x90]
0x14099bc54 0f1095a0000000 movups xmm2, xmmword ptr [rbp + 0xa0]
0x14099bc5b 0f109db0000000 movups xmm3, xmmword ptr [rbp + 0xb0]
0x14099bc62 0f1180c0010000 movups xmmword ptr [rax + 0x1c0], xmm0
0x14099bc69 0f1188d0010000 movups xmmword ptr [rax + 0x1d0], xmm1
0x14099bc70 0f1190e0010000 movups xmmword ptr [rax + 0x1e0], xmm2
0x14099bc77 0f1198f0010000 movups xmmword ptr [rax + 0x1f0], xmm3
0x14099bc7e 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
0x14099bc85 0f118000020000 movups xmmword ptr [rax + 0x200], xmm0
0x14099bc8c 0f1085e0000000 movups xmm0, xmmword ptr [rbp + 0xe0]
0x14099bc93 0f118020020000 movups xmmword ptr [rax + 0x220], xmm0
0x14099bc9a 0f1085d0000000 movups xmm0, xmmword ptr [rbp + 0xd0]
0x14099bca1 0f118010020000 movups xmmword ptr [rax + 0x210], xmm0
0x14099bca8 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
0x14099bcac 0f118090020000 movups xmmword ptr [rax + 0x290], xmm0
0x14099bcb3 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x14099bcb7 0f118080020000 movups xmmword ptr [rax + 0x280], xmm0
0x14099bcbe 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
0x14099bcc2 0f118070020000 movups xmmword ptr [rax + 0x270], xmm0
0x14099bcc9 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
0x14099bccd 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
0x14099bcd1 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
0x14099bcd5 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
0x14099bcd9 0f119860020000 movups xmmword ptr [rax + 0x260], xmm3
0x14099bce0 0f119050020000 movups xmmword ptr [rax + 0x250], xmm2
0x14099bce7 0f118840020000 movups xmmword ptr [rax + 0x240], xmm1
0x14099bcee 0f118030020000 movups xmmword ptr [rax + 0x230], xmm0
0x14099bcf5 0f1045a0 movups xmm0, xmmword ptr [rbp - 0x60]
0x14099bcf9 0f104db0 movups xmm1, xmmword ptr [rbp - 0x50]
0x14099bcfd 0f1055c0 movups xmm2, xmmword ptr [rbp - 0x40]
0x14099bd01 0f105dd0 movups xmm3, xmmword ptr [rbp - 0x30]
0x14099bd05 0f1180a0020000 movups xmmword ptr [rax + 0x2a0], xmm0
0x14099bd0c 0f1188b0020000 movups xmmword ptr [rax + 0x2b0], xmm1
0x14099bd13 0f1190c0020000 movups xmmword ptr [rax + 0x2c0], xmm2
0x14099bd1a 0f1198d0020000 movups xmmword ptr [rax + 0x2d0], xmm3
0x14099bd21 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
0x14099bd25 0f1180e0020000 movups xmmword ptr [rax + 0x2e0], xmm0
0x14099bd2c 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
0x14099bd30 0f1180f0020000 movups xmmword ptr [rax + 0x2f0], xmm0
0x14099bd37 0f104500 movups xmm0, xmmword ptr [rbp]
0x14099bd3b 0f118000030000 movups xmmword ptr [rax + 0x300], xmm0
0x14099bd42 0f1085b0020000 movups xmm0, xmmword ptr [rbp + 0x2b0]
0x14099bd49 0f108dc0020000 movups xmm1, xmmword ptr [rbp + 0x2c0]
0x14099bd50 0f1095d0020000 movups xmm2, xmmword ptr [rbp + 0x2d0]
0x14099bd57 0f109de0020000 movups xmm3, xmmword ptr [rbp + 0x2e0]
0x14099bd5e 0f118010030000 movups xmmword ptr [rax + 0x310], xmm0
0x14099bd65 0f118820030000 movups xmmword ptr [rax + 0x320], xmm1
0x14099bd6c 0f119030030000 movups xmmword ptr [rax + 0x330], xmm2
0x14099bd73 0f119840030000 movups xmmword ptr [rax + 0x340], xmm3
0x14099bd7a 0f1085f0020000 movups xmm0, xmmword ptr [rbp + 0x2f0]
0x14099bd81 0f118050030000 movups xmmword ptr [rax + 0x350], xmm0
0x14099bd88 0f108510030000 movups xmm0, xmmword ptr [rbp + 0x310]
0x14099bd8f 0f118070030000 movups xmmword ptr [rax + 0x370], xmm0
0x14099bd96 0f108500030000 movups xmm0, xmmword ptr [rbp + 0x300]
0x14099bd9d 0f118060030000 movups xmmword ptr [rax + 0x360], xmm0
0x14099bda4 48c7460808000000 mov qword ptr [rsi + 8], 8
0x14099bdac 48894610 mov qword ptr [rsi + 0x10], rax
0x14099bdb0 48c7461808000000 mov qword ptr [rsi + 0x18], 8
0x14099bdb8 48c7462000000000 mov qword ptr [rsi + 0x20], 0
0x14099bdc0 48c7462808000000 mov qword ptr [rsi + 0x28], 8
0x14099bdc8 0f57c0 xorps xmm0, xmm0
0x14099bdcb 0f114630 movups xmmword ptr [rsi + 0x30], xmm0
0x14099bdcf 48891e mov qword ptr [rsi], rbx
0x14099bdd2 4889f0 mov rax, rsi
0x14099bdd5 4881c4b8040000 add rsp, 0x4b8
0x14099bddc 5b pop rbx
0x14099bddd 5f pop rdi
0x14099bdde 5e pop rsi
0x14099bddf 5d pop rbp
0x14099bde0 c3 ret 
0x14099bde1 b908000000 mov ecx, 8
0x14099bde6 ba80030000 mov edx, 0x380
0x14099bdeb e810d2f301 call 0x1428d9000
0x14099bdf0 b908000000 mov ecx, 8
0x14099bdf5 ba78010000 mov edx, 0x178
0x14099bdfa e801d2f301 call 0x1428d9000
0x14099bdff 0f0b ud2 
0x14099be01 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14099be10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14099be15 55 push rbp
0x14099be16 56 push rsi
0x14099be17 57 push rdi
0x14099be18 53 push rbx
0x14099be19 4883ec28 sub rsp, 0x28
0x14099be1d 488daa80000000 lea rbp, [rdx + 0x80]
0x14099be24 488d8db0020000 lea rcx, [rbp + 0x2b0]
0x14099be2b e8c0dcc4ff call 0x1405e9af0
0x14099be30 90 nop 
0x14099be31 4883c428 add rsp, 0x28
0x14099be35 5b pop rbx
0x14099be36 5f pop rdi
0x14099be37 5e pop rsi
0x14099be38 5d pop rbp
0x14099be39 c3 ret 
0x14099be3a 660f1f440000 nop word ptr [rax + rax]
0x14099be40 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14099be45 55 push rbp
0x14099be46 56 push rsi
0x14099be47 57 push rdi
0x14099be48 53 push rbx
0x14099be49 4883ec28 sub rsp, 0x28
0x14099be4d 488daa80000000 lea rbp, [rdx + 0x80]
0x14099be54 ba80030000 mov edx, 0x380
0x14099be59 41b808000000 mov r8d, 8
0x14099be5f 488b8d28040000 mov rcx, qword ptr [rbp + 0x428]
0x14099be66 e8b57dc7ff call 0x140613c20
0x14099be6b 90 nop 
0x14099be6c 4883c428 add rsp, 0x28
0x14099be70 5b pop rbx
0x14099be71 5f pop rdi
0x14099be72 5e pop rsi
0x14099be73 5d pop rbp
0x14099be74 c3 ret 
0x14099be75 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14099be80 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14099be85 55 push rbp
0x14099be86 56 push rsi
0x14099be87 57 push rdi
0x14099be88 53 push rbx
0x14099be89 4883ec28 sub rsp, 0x28
0x14099be8d 488daa80000000 lea rbp, [rdx + 0x80]
0x14099be94 488d8d40020000 lea rcx, [rbp + 0x240]
0x14099be9b e8e0d968ff call 0x140029880
0x14099bea0 90 nop 
0x14099bea1 4883c428 add rsp, 0x28
0x14099bea5 5b pop rbx
0x14099bea6 5f pop rdi
0x14099bea7 5e pop rsi
0x14099bea8 5d pop rbp
0x14099bea9 c3 ret 
0x14099beaa 660f1f440000 nop word ptr [rax + rax]
0x14099beb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14099beb5 55 push rbp
0x14099beb6 56 push rsi
0x14099beb7 57 push rdi
0x14099beb8 53 push rbx
0x14099beb9 4883ec28 sub rsp, 0x28
0x14099bebd 488daa80000000 lea rbp, [rdx + 0x80]
0x14099bec4 488d8dd0010000 lea rcx, [rbp + 0x1d0]
0x14099becb e8b0d968ff call 0x140029880
0x14099bed0 90 nop 
0x14099bed1 4883c428 add rsp, 0x28
0x14099bed5 5b pop rbx
0x14099bed6 5f pop rdi
0x14099bed7 5e pop rsi
0x14099bed8 5d pop rbp
0x14099bed9 c3 ret 
0x14099beda 660f1f440000 nop word ptr [rax + rax]
0x14099bee0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14099bee5 55 push rbp
0x14099bee6 56 push rsi
0x14099bee7 57 push rdi
0x14099bee8 53 push rbx
0x14099bee9 4883ec28 sub rsp, 0x28
0x14099beed 488daa80000000 lea rbp, [rdx + 0x80]
0x14099bef4 488d8d60010000 lea rcx, [rbp + 0x160]
0x14099befb e880d968ff call 0x140029880
0x14099bf00 90 nop 
0x14099bf01 4883c428 add rsp, 0x28
0x14099bf05 5b pop rbx
0x14099bf06 5f pop rdi
0x14099bf07 5e pop rsi
0x14099bf08 5d pop rbp
0x14099bf09 c3 ret 
0x14099bf0a 660f1f440000 nop word ptr [rax + rax]
0x14099bf10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14099bf15 55 push rbp
0x14099bf16 56 push rsi
0x14099bf17 57 push rdi
0x14099bf18 53 push rbx
0x14099bf19 4883ec28 sub rsp, 0x28
0x14099bf1d 488daa80000000 lea rbp, [rdx + 0x80]
0x14099bf24 488d8df0000000 lea rcx, [rbp + 0xf0]
0x14099bf2b e850d968ff call 0x140029880
0x14099bf30 90 nop 
0x14099bf31 4883c428 add rsp, 0x28
0x14099bf35 5b pop rbx
0x14099bf36 5f pop rdi
0x14099bf37 5e pop rsi
0x14099bf38 5d pop rbp
0x14099bf39 c3 ret 
0x14099bf3a 660f1f440000 nop word ptr [rax + rax]
0x14099bf40 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14099bf45 55 push rbp
0x14099bf46 56 push rsi
0x14099bf47 57 push rdi
0x14099bf48 53 push rbx
0x14099bf49 4883ec28 sub rsp, 0x28
0x14099bf4d 488daa80000000 lea rbp, [rdx + 0x80]
0x14099bf54 488d8d80000000 lea rcx, [rbp + 0x80]
0x14099bf5b e820d968ff call 0x140029880
0x14099bf60 90 nop 
0x14099bf61 4883c428 add rsp, 0x28
0x14099bf65 5b pop rbx
0x14099bf66 5f pop rdi
0x14099bf67 5e pop rsi
0x14099bf68 5d pop rbp
0x14099bf69 c3 ret 
0x14099bf6a 660f1f440000 nop word ptr [rax + rax]
0x14099bf70 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14099bf75 55 push rbp
0x14099bf76 56 push rsi
0x14099bf77 57 push rdi
0x14099bf78 53 push rbx
0x14099bf79 4883ec28 sub rsp, 0x28
0x14099bf7d 488daa80000000 lea rbp, [rdx + 0x80]
0x14099bf84 488d4d10 lea rcx, [rbp + 0x10]
0x14099bf88 e8f3d868ff call 0x140029880
0x14099bf8d 90 nop 
0x14099bf8e 4883c428 add rsp, 0x28
0x14099bf92 5b pop rbx
0x14099bf93 5f pop rdi
0x14099bf94 5e pop rsi
0x14099bf95 5d pop rbp
0x14099bf96 c3 ret 
0x14099bf97 660f1f840000000000 nop word ptr [rax + rax]
0x14099bfa0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14099bfa5 55 push rbp
0x14099bfa6 56 push rsi
0x14099bfa7 57 push rdi
0x14099bfa8 53 push rbx
0x14099bfa9 4883ec28 sub rsp, 0x28
0x14099bfad 488daa80000000 lea rbp, [rdx + 0x80]
0x14099bfb4 488d4da0 lea rcx, [rbp - 0x60]
0x14099bfb8 e8c3d868ff call 0x140029880
0x14099bfbd 90 nop 
0x14099bfbe 4883c428 add rsp, 0x28
0x14099bfc2 5b pop rbx
0x14099bfc3 5f pop rdi
0x14099bfc4 5e pop rsi
0x14099bfc5 5d pop rbp
0x14099bfc6 c3 ret 
