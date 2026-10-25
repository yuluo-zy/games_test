0x14033def0 55 push rbp
0x14033def1 56 push rsi
0x14033def2 57 push rdi
0x14033def3 53 push rbx
0x14033def4 4881ec88050000 sub rsp, 0x588
0x14033defb 488dac2480000000 lea rbp, [rsp + 0x80]
0x14033df03 48c78500050000feffffff mov qword ptr [rbp + 0x500], 0xfffffffffffffffe
0x14033df0e 4889ce mov rsi, rcx
0x14033df11 0fb605f16c2f03 movzx eax, byte ptr [rip + 0x32f6cf1]
0x14033df18 b9f0030000 mov ecx, 0x3f0
0x14033df1d ba08000000 mov edx, 8
0x14033df22 e8e95c2d00 call 0x140613c10
0x14033df27 4885c0 test rax, rax
0x14033df2a 0f8456070000 je 0x14033e686
0x14033df30 488985f8040000 mov qword ptr [rbp + 0x4f8], rax
0x14033df37 0f57c0 xorps xmm0, xmm0
0x14033df3a 0f118540030000 movups xmmword ptr [rbp + 0x340], xmm0
0x14033df41 0f118558030000 movups xmmword ptr [rbp + 0x358], xmm0
0x14033df48 0f118570030000 movups xmmword ptr [rbp + 0x370], xmm0
0x14033df4f 0f118588030000 movups xmmword ptr [rbp + 0x388], xmm0
0x14033df56 0f1185a0030000 movups xmmword ptr [rbp + 0x3a0], xmm0
0x14033df5d c785b003000000000000 mov dword ptr [rbp + 0x3b0], 0
0x14033df67 0f1185d8030000 movups xmmword ptr [rbp + 0x3d8], xmm0
0x14033df6e 0f1185f0030000 movups xmmword ptr [rbp + 0x3f0], xmm0
0x14033df75 0f118508040000 movups xmmword ptr [rbp + 0x408], xmm0
0x14033df7c 0f118520040000 movups xmmword ptr [rbp + 0x420], xmm0
0x14033df83 0f118538040000 movups xmmword ptr [rbp + 0x438], xmm0
0x14033df8a c7854804000000000000 mov dword ptr [rbp + 0x448], 0
0x14033df94 48c7852003000000000000 mov qword ptr [rbp + 0x320], 0
0x14033df9f 48c7852803000008000000 mov qword ptr [rbp + 0x328], 8
0x14033dfaa 48c7853003000000000000 mov qword ptr [rbp + 0x330], 0
0x14033dfb5 48c7853803000010000000 mov qword ptr [rbp + 0x338], 0x10
0x14033dfc0 48c7855003000010000000 mov qword ptr [rbp + 0x350], 0x10
0x14033dfcb 48c7856803000010000000 mov qword ptr [rbp + 0x368], 0x10
0x14033dfd6 48c7858003000010000000 mov qword ptr [rbp + 0x380], 0x10
0x14033dfe1 48c7859803000010000000 mov qword ptr [rbp + 0x398], 0x10
0x14033dfec 48bb0000000000000080 movabs rbx, 0x8000000000000000
0x14033dff6 48899db8030000 mov qword ptr [rbp + 0x3b8], rbx
0x14033dffd 488d05b33c6802 lea rax, [rip + 0x2683cb3]
0x14033e004 488985c0030000 mov qword ptr [rbp + 0x3c0], rax
0x14033e00b 48c785c803000047000000 mov qword ptr [rbp + 0x3c8], 0x47
0x14033e016 48c785d003000010000000 mov qword ptr [rbp + 0x3d0], 0x10
0x14033e021 48c785e803000010000000 mov qword ptr [rbp + 0x3e8], 0x10
0x14033e02c 48c7850004000010000000 mov qword ptr [rbp + 0x400], 0x10
0x14033e037 48c7851804000010000000 mov qword ptr [rbp + 0x418], 0x10
0x14033e042 48c7853004000010000000 mov qword ptr [rbp + 0x430], 0x10
0x14033e04d c7855004000000000000 mov dword ptr [rbp + 0x450], 0
0x14033e057 66c785540400000100 mov word ptr [rbp + 0x454], 1
0x14033e060 48899db8040000 mov qword ptr [rbp + 0x4b8], rbx
0x14033e067 c785f004000000000000 mov dword ptr [rbp + 0x4f0], 0
0x14033e071 0fb605916b2f03 movzx eax, byte ptr [rip + 0x32f6b91]
0x14033e078 b9d8010000 mov ecx, 0x1d8
0x14033e07d ba08000000 mov edx, 8
0x14033e082 e8895b2d00 call 0x140613c10
0x14033e087 4885c0 test rax, rax
0x14033e08a 0f8405060000 je 0x14033e695
0x14033e090 4889c7 mov rdi, rax
0x14033e093 488d9520030000 lea rdx, [rbp + 0x320]
0x14033e09a 41b8d8010000 mov r8d, 0x1d8
0x14033e0a0 4889c1 mov rcx, rax
0x14033e0a3 e8585b5e02 call 0x142923c00
0x14033e0a8 4c8d05f13a6802 lea r8, [rip + 0x2683af1]
0x14033e0af 488d8d40020000 lea rcx, [rbp + 0x240]
0x14033e0b6 4889fa mov rdx, rdi
0x14033e0b9 e8526a9a00 call 0x140ce4b10
0x14033e0be 0f57c0 xorps xmm0, xmm0
0x14033e0c1 0f118540030000 movups xmmword ptr [rbp + 0x340], xmm0
0x14033e0c8 0f118558030000 movups xmmword ptr [rbp + 0x358], xmm0
0x14033e0cf 0f118570030000 movups xmmword ptr [rbp + 0x370], xmm0
0x14033e0d6 0f118588030000 movups xmmword ptr [rbp + 0x388], xmm0
0x14033e0dd 0f1185a0030000 movups xmmword ptr [rbp + 0x3a0], xmm0
0x14033e0e4 c785b003000000000000 mov dword ptr [rbp + 0x3b0], 0
0x14033e0ee 0f1185d8030000 movups xmmword ptr [rbp + 0x3d8], xmm0
0x14033e0f5 0f1185f0030000 movups xmmword ptr [rbp + 0x3f0], xmm0
0x14033e0fc 0f118508040000 movups xmmword ptr [rbp + 0x408], xmm0
0x14033e103 0f118520040000 movups xmmword ptr [rbp + 0x420], xmm0
0x14033e10a 0f118538040000 movups xmmword ptr [rbp + 0x438], xmm0
0x14033e111 c7854804000000000000 mov dword ptr [rbp + 0x448], 0
0x14033e11b 48c7852003000000000000 mov qword ptr [rbp + 0x320], 0
0x14033e126 48c7852803000008000000 mov qword ptr [rbp + 0x328], 8
0x14033e131 48c7853003000000000000 mov qword ptr [rbp + 0x330], 0
0x14033e13c 48c7853803000010000000 mov qword ptr [rbp + 0x338], 0x10
0x14033e147 48c7855003000010000000 mov qword ptr [rbp + 0x350], 0x10
0x14033e152 48c7856803000010000000 mov qword ptr [rbp + 0x368], 0x10
0x14033e15d 48c7858003000010000000 mov qword ptr [rbp + 0x380], 0x10
0x14033e168 48c7859803000010000000 mov qword ptr [rbp + 0x398], 0x10
0x14033e173 48899db8030000 mov qword ptr [rbp + 0x3b8], rbx
0x14033e17a 488d05f43a6802 lea rax, [rip + 0x2683af4]
0x14033e181 488985c0030000 mov qword ptr [rbp + 0x3c0], rax
0x14033e188 48c785c803000042000000 mov qword ptr [rbp + 0x3c8], 0x42
0x14033e193 48c785d003000010000000 mov qword ptr [rbp + 0x3d0], 0x10
0x14033e19e 48c785e803000010000000 mov qword ptr [rbp + 0x3e8], 0x10
0x14033e1a9 48c7850004000010000000 mov qword ptr [rbp + 0x400], 0x10
0x14033e1b4 48c7851804000010000000 mov qword ptr [rbp + 0x418], 0x10
0x14033e1bf 48c7853004000010000000 mov qword ptr [rbp + 0x430], 0x10
0x14033e1ca c7855004000000000000 mov dword ptr [rbp + 0x450], 0
0x14033e1d4 66c785540400000100 mov word ptr [rbp + 0x454], 1
0x14033e1dd 48899d58040000 mov qword ptr [rbp + 0x458], rbx
0x14033e1e4 c785e004000000000000 mov dword ptr [rbp + 0x4e0], 0
0x14033e1ee 0fb605146a2f03 movzx eax, byte ptr [rip + 0x32f6a14]
0x14033e1f5 b9c8010000 mov ecx, 0x1c8
0x14033e1fa ba08000000 mov edx, 8
0x14033e1ff e80c5a2d00 call 0x140613c10
0x14033e204 4885c0 test rax, rax
0x14033e207 0f8499040000 je 0x14033e6a6
0x14033e20d 4889c7 mov rdi, rax
0x14033e210 488d9520030000 lea rdx, [rbp + 0x320]
0x14033e217 41b8c8010000 mov r8d, 0x1c8
0x14033e21d 4889c1 mov rcx, rax
0x14033e220 e8db595e02 call 0x142923c00
0x14033e225 4c8d05bc386802 lea r8, [rip + 0x26838bc]
0x14033e22c 488d8dd0010000 lea rcx, [rbp + 0x1d0]
0x14033e233 4889fa mov rdx, rdi
0x14033e236 e8d5689a00 call 0x140ce4b10
0x14033e23b 488d8d60010000 lea rcx, [rbp + 0x160]
0x14033e242 e8e9c1ffff call 0x14033a430
0x14033e247 488d8df0000000 lea rcx, [rbp + 0xf0]
0x14033e24e e83d450000 call 0x140342790
0x14033e253 488d8d80000000 lea rcx, [rbp + 0x80]
0x14033e25a e801c0ffff call 0x14033a260
0x14033e25f 488d4d10 lea rcx, [rbp + 0x10]
0x14033e263 e8284accff call 0x140002c90
0x14033e268 488d4da0 lea rcx, [rbp - 0x60]
0x14033e26c e82fc6fdff call 0x14031a8a0
0x14033e271 488d8db0020000 lea rcx, [rbp + 0x2b0]
0x14033e278 e883f2f6ff call 0x1402ad500
0x14033e27d 488d8d20030000 lea rcx, [rbp + 0x320]
0x14033e284 e8f7111d00 call 0x14050f480
0x14033e289 0f1085a0020000 movups xmm0, xmmword ptr [rbp + 0x2a0]
0x14033e290 488b85f8040000 mov rax, qword ptr [rbp + 0x4f8]
0x14033e297 0f114060 movups xmmword ptr [rax + 0x60], xmm0
0x14033e29b 0f108590020000 movups xmm0, xmmword ptr [rbp + 0x290]
0x14033e2a2 0f114050 movups xmmword ptr [rax + 0x50], xmm0
0x14033e2a6 0f108580020000 movups xmm0, xmmword ptr [rbp + 0x280]
0x14033e2ad 0f114040 movups xmmword ptr [rax + 0x40], xmm0
0x14033e2b1 0f108540020000 movups xmm0, xmmword ptr [rbp + 0x240]
0x14033e2b8 0f108d50020000 movups xmm1, xmmword ptr [rbp + 0x250]
0x14033e2bf 0f109560020000 movups xmm2, xmmword ptr [rbp + 0x260]
0x14033e2c6 0f109d70020000 movups xmm3, xmmword ptr [rbp + 0x270]
0x14033e2cd 0f115830 movups xmmword ptr [rax + 0x30], xmm3
0x14033e2d1 0f115020 movups xmmword ptr [rax + 0x20], xmm2
0x14033e2d5 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x14033e2d9 0f1100 movups xmmword ptr [rax], xmm0
0x14033e2dc 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
0x14033e2e3 0f108de0010000 movups xmm1, xmmword ptr [rbp + 0x1e0]
0x14033e2ea 0f1095f0010000 movups xmm2, xmmword ptr [rbp + 0x1f0]
0x14033e2f1 0f109d00020000 movups xmm3, xmmword ptr [rbp + 0x200]
0x14033e2f8 0f114070 movups xmmword ptr [rax + 0x70], xmm0
0x14033e2fc 0f118880000000 movups xmmword ptr [rax + 0x80], xmm1
0x14033e303 0f119090000000 movups xmmword ptr [rax + 0x90], xmm2
0x14033e30a 0f1198a0000000 movups xmmword ptr [rax + 0xa0], xmm3
0x14033e311 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
0x14033e318 0f1180b0000000 movups xmmword ptr [rax + 0xb0], xmm0
0x14033e31f 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
0x14033e326 0f1180c0000000 movups xmmword ptr [rax + 0xc0], xmm0
0x14033e32d 0f108530020000 movups xmm0, xmmword ptr [rbp + 0x230]
0x14033e334 0f1180d0000000 movups xmmword ptr [rax + 0xd0], xmm0
0x14033e33b 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
0x14033e342 0f118040010000 movups xmmword ptr [rax + 0x140], xmm0
0x14033e349 0f1085b0010000 movups xmm0, xmmword ptr [rbp + 0x1b0]
0x14033e350 0f118030010000 movups xmmword ptr [rax + 0x130], xmm0
0x14033e357 0f1085a0010000 movups xmm0, xmmword ptr [rbp + 0x1a0]
0x14033e35e 0f118020010000 movups xmmword ptr [rax + 0x120], xmm0
0x14033e365 0f108560010000 movups xmm0, xmmword ptr [rbp + 0x160]
0x14033e36c 0f108d70010000 movups xmm1, xmmword ptr [rbp + 0x170]
0x14033e373 0f109580010000 movups xmm2, xmmword ptr [rbp + 0x180]
0x14033e37a 0f109d90010000 movups xmm3, xmmword ptr [rbp + 0x190]
0x14033e381 0f119810010000 movups xmmword ptr [rax + 0x110], xmm3
0x14033e388 0f119000010000 movups xmmword ptr [rax + 0x100], xmm2
0x14033e38f 0f1188f0000000 movups xmmword ptr [rax + 0xf0], xmm1
0x14033e396 0f1180e0000000 movups xmmword ptr [rax + 0xe0], xmm0
0x14033e39d 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
0x14033e3a4 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
0x14033e3ab 0f109510010000 movups xmm2, xmmword ptr [rbp + 0x110]
0x14033e3b2 0f109d20010000 movups xmm3, xmmword ptr [rbp + 0x120]
0x14033e3b9 0f118050010000 movups xmmword ptr [rax + 0x150], xmm0
0x14033e3c0 0f118860010000 movups xmmword ptr [rax + 0x160], xmm1
0x14033e3c7 0f119070010000 movups xmmword ptr [rax + 0x170], xmm2
0x14033e3ce 0f119880010000 movups xmmword ptr [rax + 0x180], xmm3
0x14033e3d5 0f108530010000 movups xmm0, xmmword ptr [rbp + 0x130]
0x14033e3dc 0f118090010000 movups xmmword ptr [rax + 0x190], xmm0
0x14033e3e3 0f108540010000 movups xmm0, xmmword ptr [rbp + 0x140]
0x14033e3ea 0f1180a0010000 movups xmmword ptr [rax + 0x1a0], xmm0
0x14033e3f1 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
0x14033e3f8 0f1180b0010000 movups xmmword ptr [rax + 0x1b0], xmm0
0x14033e3ff 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
0x14033e406 0f108d90000000 movups xmm1, xmmword ptr [rbp + 0x90]
0x14033e40d 0f1095a0000000 movups xmm2, xmmword ptr [rbp + 0xa0]
0x14033e414 0f109db0000000 movups xmm3, xmmword ptr [rbp + 0xb0]
0x14033e41b 0f1180c0010000 movups xmmword ptr [rax + 0x1c0], xmm0
0x14033e422 0f1188d0010000 movups xmmword ptr [rax + 0x1d0], xmm1
0x14033e429 0f1190e0010000 movups xmmword ptr [rax + 0x1e0], xmm2
0x14033e430 0f1198f0010000 movups xmmword ptr [rax + 0x1f0], xmm3
0x14033e437 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
0x14033e43e 0f118000020000 movups xmmword ptr [rax + 0x200], xmm0
0x14033e445 0f1085e0000000 movups xmm0, xmmword ptr [rbp + 0xe0]
0x14033e44c 0f118020020000 movups xmmword ptr [rax + 0x220], xmm0
0x14033e453 0f1085d0000000 movups xmm0, xmmword ptr [rbp + 0xd0]
0x14033e45a 0f118010020000 movups xmmword ptr [rax + 0x210], xmm0
0x14033e461 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
0x14033e465 0f118090020000 movups xmmword ptr [rax + 0x290], xmm0
0x14033e46c 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
0x14033e470 0f118080020000 movups xmmword ptr [rax + 0x280], xmm0
0x14033e477 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
0x14033e47b 0f118070020000 movups xmmword ptr [rax + 0x270], xmm0
0x14033e482 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
0x14033e486 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
0x14033e48a 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
0x14033e48e 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
0x14033e492 0f119860020000 movups xmmword ptr [rax + 0x260], xmm3
0x14033e499 0f119050020000 movups xmmword ptr [rax + 0x250], xmm2
0x14033e4a0 0f118840020000 movups xmmword ptr [rax + 0x240], xmm1
0x14033e4a7 0f118030020000 movups xmmword ptr [rax + 0x230], xmm0
0x14033e4ae 0f1045a0 movups xmm0, xmmword ptr [rbp - 0x60]
0x14033e4b2 0f104db0 movups xmm1, xmmword ptr [rbp - 0x50]
0x14033e4b6 0f1055c0 movups xmm2, xmmword ptr [rbp - 0x40]
0x14033e4ba 0f105dd0 movups xmm3, xmmword ptr [rbp - 0x30]
0x14033e4be 0f1180a0020000 movups xmmword ptr [rax + 0x2a0], xmm0
0x14033e4c5 0f1188b0020000 movups xmmword ptr [rax + 0x2b0], xmm1
0x14033e4cc 0f1190c0020000 movups xmmword ptr [rax + 0x2c0], xmm2
0x14033e4d3 0f1198d0020000 movups xmmword ptr [rax + 0x2d0], xmm3
0x14033e4da 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
0x14033e4de 0f1180e0020000 movups xmmword ptr [rax + 0x2e0], xmm0
0x14033e4e5 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
0x14033e4e9 0f1180f0020000 movups xmmword ptr [rax + 0x2f0], xmm0
0x14033e4f0 0f104500 movups xmm0, xmmword ptr [rbp]
0x14033e4f4 0f118000030000 movups xmmword ptr [rax + 0x300], xmm0
0x14033e4fb 0f1085b0020000 movups xmm0, xmmword ptr [rbp + 0x2b0]
0x14033e502 0f108dc0020000 movups xmm1, xmmword ptr [rbp + 0x2c0]
0x14033e509 0f1095d0020000 movups xmm2, xmmword ptr [rbp + 0x2d0]
0x14033e510 0f109de0020000 movups xmm3, xmmword ptr [rbp + 0x2e0]
0x14033e517 0f118010030000 movups xmmword ptr [rax + 0x310], xmm0
0x14033e51e 0f118820030000 movups xmmword ptr [rax + 0x320], xmm1
0x14033e525 0f119030030000 movups xmmword ptr [rax + 0x330], xmm2
0x14033e52c 0f119840030000 movups xmmword ptr [rax + 0x340], xmm3
0x14033e533 0f1085f0020000 movups xmm0, xmmword ptr [rbp + 0x2f0]
0x14033e53a 0f118050030000 movups xmmword ptr [rax + 0x350], xmm0
0x14033e541 0f108500030000 movups xmm0, xmmword ptr [rbp + 0x300]
0x14033e548 0f118060030000 movups xmmword ptr [rax + 0x360], xmm0
0x14033e54f 0f108510030000 movups xmm0, xmmword ptr [rbp + 0x310]
0x14033e556 0f118070030000 movups xmmword ptr [rax + 0x370], xmm0
0x14033e55d 0f108520030000 movups xmm0, xmmword ptr [rbp + 0x320]
0x14033e564 0f108d30030000 movups xmm1, xmmword ptr [rbp + 0x330]
0x14033e56b 0f109540030000 movups xmm2, xmmword ptr [rbp + 0x340]
0x14033e572 0f109d50030000 movups xmm3, xmmword ptr [rbp + 0x350]
0x14033e579 0f118080030000 movups xmmword ptr [rax + 0x380], xmm0
0x14033e580 0f118890030000 movups xmmword ptr [rax + 0x390], xmm1
0x14033e587 0f1190a0030000 movups xmmword ptr [rax + 0x3a0], xmm2
0x14033e58e 0f1198b0030000 movups xmmword ptr [rax + 0x3b0], xmm3
0x14033e595 0f108560030000 movups xmm0, xmmword ptr [rbp + 0x360]
0x14033e59c 0f1180c0030000 movups xmmword ptr [rax + 0x3c0], xmm0
0x14033e5a3 0f108570030000 movups xmm0, xmmword ptr [rbp + 0x370]
0x14033e5aa 0f1180d0030000 movups xmmword ptr [rax + 0x3d0], xmm0
0x14033e5b1 0f108580030000 movups xmm0, xmmword ptr [rbp + 0x380]
0x14033e5b8 0f1180e0030000 movups xmmword ptr [rax + 0x3e0], xmm0
0x14033e5bf 48c7852803000009000000 mov qword ptr [rbp + 0x328], 9
0x14033e5ca 48898530030000 mov qword ptr [rbp + 0x330], rax
0x14033e5d1 48c7853803000009000000 mov qword ptr [rbp + 0x338], 9
0x14033e5dc 48c7854003000000000000 mov qword ptr [rbp + 0x340], 0
0x14033e5e7 48c7854803000008000000 mov qword ptr [rbp + 0x348], 8
0x14033e5f2 0f57c0 xorps xmm0, xmm0
0x14033e5f5 0f118550030000 movups xmmword ptr [rbp + 0x350], xmm0
0x14033e5fc 488b85c0020000 mov rax, qword ptr [rbp + 0x2c0]
0x14033e603 48898570030000 mov qword ptr [rbp + 0x370], rax
0x14033e60a 0f1085b0020000 movups xmm0, xmmword ptr [rbp + 0x2b0]
0x14033e611 0f118560030000 movups xmmword ptr [rbp + 0x360], xmm0
0x14033e618 48899d20030000 mov qword ptr [rbp + 0x320], rbx
0x14033e61f 488d8d58030000 lea rcx, [rbp + 0x358]
0x14033e626 e8a51fe3ff call 0x1401705d0
0x14033e62b 0f108580030000 movups xmm0, xmmword ptr [rbp + 0x380]
0x14033e632 0f114660 movups xmmword ptr [rsi + 0x60], xmm0
0x14033e636 0f108570030000 movups xmm0, xmmword ptr [rbp + 0x370]
0x14033e63d 0f114650 movups xmmword ptr [rsi + 0x50], xmm0
0x14033e641 0f108560030000 movups xmm0, xmmword ptr [rbp + 0x360]
0x14033e648 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
0x14033e64c 0f108520030000 movups xmm0, xmmword ptr [rbp + 0x320]
0x14033e653 0f108d30030000 movups xmm1, xmmword ptr [rbp + 0x330]
0x14033e65a 0f109540030000 movups xmm2, xmmword ptr [rbp + 0x340]
0x14033e661 0f109d50030000 movups xmm3, xmmword ptr [rbp + 0x350]
0x14033e668 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
0x14033e66c 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
0x14033e670 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
0x14033e674 0f1106 movups xmmword ptr [rsi], xmm0
0x14033e677 4889f0 mov rax, rsi
0x14033e67a 4881c488050000 add rsp, 0x588
0x14033e681 5b pop rbx
0x14033e682 5f pop rdi
0x14033e683 5e pop rsi
0x14033e684 5d pop rbp
0x14033e685 c3 ret 
0x14033e686 b908000000 mov ecx, 8
0x14033e68b baf0030000 mov edx, 0x3f0
0x14033e690 e86ba95902 call 0x1428d9000
0x14033e695 b908000000 mov ecx, 8
0x14033e69a bad8010000 mov edx, 0x1d8
0x14033e69f e85ca95902 call 0x1428d9000
0x14033e6a4 eb0f jmp 0x14033e6b5
0x14033e6a6 b908000000 mov ecx, 8
0x14033e6ab bac8010000 mov edx, 0x1c8
0x14033e6b0 e84ba95902 call 0x1428d9000
0x14033e6b5 0f0b ud2 
0x14033e6b7 660f1f840000000000 nop word ptr [rax + rax]
0x14033e6c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14033e6c5 55 push rbp
0x14033e6c6 56 push rsi
0x14033e6c7 57 push rdi
0x14033e6c8 53 push rbx
0x14033e6c9 4883ec28 sub rsp, 0x28
0x14033e6cd 488daa80000000 lea rbp, [rdx + 0x80]
0x14033e6d4 488d8d20030000 lea rcx, [rbp + 0x320]
0x14033e6db e860bf2c00 call 0x14060a640
0x14033e6e0 90 nop 
0x14033e6e1 4883c428 add rsp, 0x28
0x14033e6e5 5b pop rbx
0x14033e6e6 5f pop rdi
0x14033e6e7 5e pop rsi
0x14033e6e8 5d pop rbp
0x14033e6e9 c3 ret 
0x14033e6ea 660f1f440000 nop word ptr [rax + rax]
0x14033e6f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14033e6f5 55 push rbp
0x14033e6f6 56 push rsi
0x14033e6f7 57 push rdi
0x14033e6f8 53 push rbx
0x14033e6f9 4883ec28 sub rsp, 0x28
0x14033e6fd 488daa80000000 lea rbp, [rdx + 0x80]
0x14033e704 baf0030000 mov edx, 0x3f0
0x14033e709 41b808000000 mov r8d, 8
0x14033e70f 488b8df8040000 mov rcx, qword ptr [rbp + 0x4f8]
0x14033e716 e805552d00 call 0x140613c20
0x14033e71b 90 nop 
0x14033e71c 4883c428 add rsp, 0x28
0x14033e720 5b pop rbx
0x14033e721 5f pop rdi
0x14033e722 5e pop rsi
0x14033e723 5d pop rbp
0x14033e724 c3 ret 
0x14033e725 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14033e730 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14033e735 55 push rbp
0x14033e736 56 push rsi
0x14033e737 57 push rdi
0x14033e738 53 push rbx
0x14033e739 4883ec28 sub rsp, 0x28
0x14033e73d 488daa80000000 lea rbp, [rdx + 0x80]
0x14033e744 488d8d20030000 lea rcx, [rbp + 0x320]
0x14033e74b e8906c2900 call 0x1405d53e0
0x14033e750 90 nop 
0x14033e751 4883c428 add rsp, 0x28
0x14033e755 5b pop rbx
0x14033e756 5f pop rdi
0x14033e757 5e pop rsi
0x14033e758 5d pop rbp
0x14033e759 c3 ret 
0x14033e75a 660f1f440000 nop word ptr [rax + rax]
0x14033e760 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14033e765 55 push rbp
0x14033e766 56 push rsi
0x14033e767 57 push rdi
0x14033e768 53 push rbx
0x14033e769 4883ec28 sub rsp, 0x28
0x14033e76d 488daa80000000 lea rbp, [rdx + 0x80]
0x14033e774 488d8d40020000 lea rcx, [rbp + 0x240]
0x14033e77b e800b1ceff call 0x140029880
0x14033e780 90 nop 
0x14033e781 4883c428 add rsp, 0x28
0x14033e785 5b pop rbx
0x14033e786 5f pop rdi
0x14033e787 5e pop rsi
0x14033e788 5d pop rbp
0x14033e789 c3 ret 
0x14033e78a 660f1f440000 nop word ptr [rax + rax]
0x14033e790 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14033e795 55 push rbp
0x14033e796 56 push rsi
0x14033e797 57 push rdi
0x14033e798 53 push rbx
0x14033e799 4883ec28 sub rsp, 0x28
0x14033e79d 488daa80000000 lea rbp, [rdx + 0x80]
0x14033e7a4 488d8dd0010000 lea rcx, [rbp + 0x1d0]
0x14033e7ab e8d0b0ceff call 0x140029880
0x14033e7b0 90 nop 
0x14033e7b1 4883c428 add rsp, 0x28
0x14033e7b5 5b pop rbx
0x14033e7b6 5f pop rdi
0x14033e7b7 5e pop rsi
0x14033e7b8 5d pop rbp
0x14033e7b9 c3 ret 
0x14033e7ba 660f1f440000 nop word ptr [rax + rax]
0x14033e7c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14033e7c5 55 push rbp
0x14033e7c6 56 push rsi
0x14033e7c7 57 push rdi
0x14033e7c8 53 push rbx
0x14033e7c9 4883ec28 sub rsp, 0x28
0x14033e7cd 488daa80000000 lea rbp, [rdx + 0x80]
0x14033e7d4 488d8d60010000 lea rcx, [rbp + 0x160]
0x14033e7db e8a0b0ceff call 0x140029880
0x14033e7e0 90 nop 
0x14033e7e1 4883c428 add rsp, 0x28
0x14033e7e5 5b pop rbx
0x14033e7e6 5f pop rdi
0x14033e7e7 5e pop rsi
0x14033e7e8 5d pop rbp
0x14033e7e9 c3 ret 
0x14033e7ea 660f1f440000 nop word ptr [rax + rax]
0x14033e7f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14033e7f5 55 push rbp
0x14033e7f6 56 push rsi
0x14033e7f7 57 push rdi
0x14033e7f8 53 push rbx
0x14033e7f9 4883ec28 sub rsp, 0x28
0x14033e7fd 488daa80000000 lea rbp, [rdx + 0x80]
0x14033e804 488d8df0000000 lea rcx, [rbp + 0xf0]
0x14033e80b e870b0ceff call 0x140029880
0x14033e810 90 nop 
0x14033e811 4883c428 add rsp, 0x28
0x14033e815 5b pop rbx
0x14033e816 5f pop rdi
0x14033e817 5e pop rsi
0x14033e818 5d pop rbp
0x14033e819 c3 ret 
0x14033e81a 660f1f440000 nop word ptr [rax + rax]
0x14033e820 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14033e825 55 push rbp
0x14033e826 56 push rsi
0x14033e827 57 push rdi
0x14033e828 53 push rbx
0x14033e829 4883ec28 sub rsp, 0x28
0x14033e82d 488daa80000000 lea rbp, [rdx + 0x80]
0x14033e834 488d8d80000000 lea rcx, [rbp + 0x80]
0x14033e83b e840b0ceff call 0x140029880
0x14033e840 90 nop 
0x14033e841 4883c428 add rsp, 0x28
0x14033e845 5b pop rbx
0x14033e846 5f pop rdi
0x14033e847 5e pop rsi
0x14033e848 5d pop rbp
0x14033e849 c3 ret 
0x14033e84a 660f1f440000 nop word ptr [rax + rax]
0x14033e850 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14033e855 55 push rbp
0x14033e856 56 push rsi
0x14033e857 57 push rdi
0x14033e858 53 push rbx
0x14033e859 4883ec28 sub rsp, 0x28
0x14033e85d 488daa80000000 lea rbp, [rdx + 0x80]
0x14033e864 488d4d10 lea rcx, [rbp + 0x10]
0x14033e868 e813b0ceff call 0x140029880
0x14033e86d 90 nop 
0x14033e86e 4883c428 add rsp, 0x28
0x14033e872 5b pop rbx
0x14033e873 5f pop rdi
0x14033e874 5e pop rsi
0x14033e875 5d pop rbp
0x14033e876 c3 ret 
0x14033e877 660f1f840000000000 nop word ptr [rax + rax]
0x14033e880 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14033e885 55 push rbp
0x14033e886 56 push rsi
0x14033e887 57 push rdi
0x14033e888 53 push rbx
0x14033e889 4883ec28 sub rsp, 0x28
0x14033e88d 488daa80000000 lea rbp, [rdx + 0x80]
0x14033e894 488d4da0 lea rcx, [rbp - 0x60]
0x14033e898 e8e3afceff call 0x140029880
0x14033e89d 90 nop 
0x14033e89e 4883c428 add rsp, 0x28
0x14033e8a2 5b pop rbx
0x14033e8a3 5f pop rdi
0x14033e8a4 5e pop rsi
0x14033e8a5 5d pop rbp
0x14033e8a6 c3 ret 
0x14033e8a7 660f1f840000000000 nop word ptr [rax + rax]
0x14033e8b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14033e8b5 55 push rbp
0x14033e8b6 56 push rsi
0x14033e8b7 57 push rdi
0x14033e8b8 53 push rbx
0x14033e8b9 4883ec28 sub rsp, 0x28
0x14033e8bd 488daa80000000 lea rbp, [rdx + 0x80]
0x14033e8c4 488d8db0020000 lea rcx, [rbp + 0x2b0]
0x14033e8cb e8b0afceff call 0x140029880
0x14033e8d0 90 nop 
0x14033e8d1 4883c428 add rsp, 0x28
0x14033e8d5 5b pop rbx
0x14033e8d6 5f pop rdi
0x14033e8d7 5e pop rsi
0x14033e8d8 5d pop rbp
0x14033e8d9 c3 ret 
0x14033e8da 660f1f440000 nop word ptr [rax + rax]
0x14033e8e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14033e8e5 55 push rbp
0x14033e8e6 56 push rsi
0x14033e8e7 57 push rdi
0x14033e8e8 53 push rbx
0x14033e8e9 4883ec28 sub rsp, 0x28
0x14033e8ed 488daa80000000 lea rbp, [rdx + 0x80]
0x14033e8f4 488d8d20030000 lea rcx, [rbp + 0x320]
0x14033e8fb e880afceff call 0x140029880
0x14033e900 90 nop 
0x14033e901 4883c428 add rsp, 0x28
0x14033e905 5b pop rbx
0x14033e906 5f pop rdi
0x14033e907 5e pop rsi
0x14033e908 5d pop rbp
0x14033e909 c3 ret 
