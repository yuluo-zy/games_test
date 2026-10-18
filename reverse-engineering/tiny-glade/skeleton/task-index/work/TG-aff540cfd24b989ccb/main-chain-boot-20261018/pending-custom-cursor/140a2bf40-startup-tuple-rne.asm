0x140a2bf40 55 push rbp
0x140a2bf41 56 push rsi
0x140a2bf42 57 push rdi
0x140a2bf43 53 push rbx
0x140a2bf44 4881ecc8050000 sub rsp, 0x5c8
0x140a2bf4b 488dac2480000000 lea rbp, [rsp + 0x80]
0x140a2bf53 48c78540050000feffffff mov qword ptr [rbp + 0x540], 0xfffffffffffffffe
0x140a2bf5e 4889ce mov rsi, rcx
0x140a2bf61 0fb605a18cc002 movzx eax, byte ptr [rip + 0x2c08ca1]
0x140a2bf68 b960040000 mov ecx, 0x460
0x140a2bf6d ba08000000 mov edx, 8
0x140a2bf72 e8997cbeff call 0x140613c10
0x140a2bf77 4885c0 test rax, rax
0x140a2bf7a 0f84c9050000 je 0x140a2c549
0x140a2bf80 48898538050000 mov qword ptr [rbp + 0x538], rax
0x140a2bf87 0f57c0 xorps xmm0, xmm0
0x140a2bf8a 0f1185b8030000 movups xmmword ptr [rbp + 0x3b8], xmm0
0x140a2bf91 0f1185d0030000 movups xmmword ptr [rbp + 0x3d0], xmm0
0x140a2bf98 0f1185e8030000 movups xmmword ptr [rbp + 0x3e8], xmm0
0x140a2bf9f 0f118500040000 movups xmmword ptr [rbp + 0x400], xmm0
0x140a2bfa6 0f118518040000 movups xmmword ptr [rbp + 0x418], xmm0
0x140a2bfad c7852804000000000000 mov dword ptr [rbp + 0x428], 0
0x140a2bfb7 0f118550040000 movups xmmword ptr [rbp + 0x450], xmm0
0x140a2bfbe 0f118568040000 movups xmmword ptr [rbp + 0x468], xmm0
0x140a2bfc5 0f118580040000 movups xmmword ptr [rbp + 0x480], xmm0
0x140a2bfcc 0f118598040000 movups xmmword ptr [rbp + 0x498], xmm0
0x140a2bfd3 0f1185b0040000 movups xmmword ptr [rbp + 0x4b0], xmm0
0x140a2bfda c785c004000000000000 mov dword ptr [rbp + 0x4c0], 0
0x140a2bfe4 48c7859803000000000000 mov qword ptr [rbp + 0x398], 0
0x140a2bfef 48c785a003000008000000 mov qword ptr [rbp + 0x3a0], 8
0x140a2bffa 48c785a803000000000000 mov qword ptr [rbp + 0x3a8], 0
0x140a2c005 48c785b003000010000000 mov qword ptr [rbp + 0x3b0], 0x10
0x140a2c010 48c785c803000010000000 mov qword ptr [rbp + 0x3c8], 0x10
0x140a2c01b 48c785e003000010000000 mov qword ptr [rbp + 0x3e0], 0x10
0x140a2c026 48c785f803000010000000 mov qword ptr [rbp + 0x3f8], 0x10
0x140a2c031 48c7851004000010000000 mov qword ptr [rbp + 0x410], 0x10
0x140a2c03c 48bb0000000000000080 movabs rbx, 0x8000000000000000
0x140a2c046 48899d30040000 mov qword ptr [rbp + 0x430], rbx
0x140a2c04d 488d051df60902 lea rax, [rip + 0x209f61d]
0x140a2c054 48898538040000 mov qword ptr [rbp + 0x438], rax
0x140a2c05b 48c785400400006e000000 mov qword ptr [rbp + 0x440], 0x6e
0x140a2c066 48c7854804000010000000 mov qword ptr [rbp + 0x448], 0x10
0x140a2c071 48c7856004000010000000 mov qword ptr [rbp + 0x460], 0x10
0x140a2c07c 48c7857804000010000000 mov qword ptr [rbp + 0x478], 0x10
0x140a2c087 48c7859004000010000000 mov qword ptr [rbp + 0x490], 0x10
0x140a2c092 48c785a804000010000000 mov qword ptr [rbp + 0x4a8], 0x10
0x140a2c09d c785c804000000000000 mov dword ptr [rbp + 0x4c8], 0
0x140a2c0a7 66c785cc0400000100 mov word ptr [rbp + 0x4cc], 1
0x140a2c0b0 48899df0040000 mov qword ptr [rbp + 0x4f0], rbx
0x140a2c0b7 c7853005000000000000 mov dword ptr [rbp + 0x530], 0
0x140a2c0c1 0fb605418bc002 movzx eax, byte ptr [rip + 0x2c08b41]
0x140a2c0c8 b9a0010000 mov ecx, 0x1a0
0x140a2c0cd ba08000000 mov edx, 8
0x140a2c0d2 e8397bbeff call 0x140613c10
0x140a2c0d7 4885c0 test rax, rax
0x140a2c0da 0f8478040000 je 0x140a2c558
0x140a2c0e0 4889c7 mov rdi, rax
0x140a2c0e3 488d9598030000 lea rdx, [rbp + 0x398]
0x140a2c0ea 41b8a0010000 mov r8d, 0x1a0
0x140a2c0f0 4889c1 mov rcx, rax
0x140a2c0f3 e8087bef01 call 0x142923c00
0x140a2c0f8 4c8d0549f40902 lea r8, [rip + 0x209f449]
0x140a2c0ff 488d8d28030000 lea rcx, [rbp + 0x328]
0x140a2c106 4889fa mov rdx, rdi
0x140a2c109 e8028a2b00 call 0x140ce4b10
0x140a2c10e 488d8db8020000 lea rcx, [rbp + 0x2b8]
0x140a2c115 e8c633d700 call 0x14179f4e0
0x140a2c11a 488d8d48020000 lea rcx, [rbp + 0x248]
0x140a2c121 e8fa38d700 call 0x14179fa20
0x140a2c126 488d8dd8010000 lea rcx, [rbp + 0x1d8]
0x140a2c12d e80e21e8ff call 0x1408ae240
0x140a2c132 488d8d68010000 lea rcx, [rbp + 0x168]
0x140a2c139 e8f289fdff call 0x140a04b30
0x140a2c13e 488d8df8000000 lea rcx, [rbp + 0xf8]
0x140a2c145 e836f69c00 call 0x1413fb780
0x140a2c14a 488d8d88000000 lea rcx, [rbp + 0x88]
0x140a2c151 e8fa2f0300 call 0x140a5f150
0x140a2c156 488d4d18 lea rcx, [rbp + 0x18]
0x140a2c15a e861790c00 call 0x140af3ac0
0x140a2c15f 488d4da8 lea rcx, [rbp - 0x58]
0x140a2c163 e8f8ace9ff call 0x1408c6e60
0x140a2c168 488d8d98030000 lea rcx, [rbp + 0x398]
0x140a2c16f e84c340700 call 0x140a9f5c0
0x140a2c174 0f108588030000 movups xmm0, xmmword ptr [rbp + 0x388]
0x140a2c17b 488b8538050000 mov rax, qword ptr [rbp + 0x538]
0x140a2c182 0f114060 movups xmmword ptr [rax + 0x60], xmm0
0x140a2c186 0f108578030000 movups xmm0, xmmword ptr [rbp + 0x378]
0x140a2c18d 0f114050 movups xmmword ptr [rax + 0x50], xmm0
0x140a2c191 0f108568030000 movups xmm0, xmmword ptr [rbp + 0x368]
0x140a2c198 0f114040 movups xmmword ptr [rax + 0x40], xmm0
0x140a2c19c 0f108528030000 movups xmm0, xmmword ptr [rbp + 0x328]
0x140a2c1a3 0f108d38030000 movups xmm1, xmmword ptr [rbp + 0x338]
0x140a2c1aa 0f109548030000 movups xmm2, xmmword ptr [rbp + 0x348]
0x140a2c1b1 0f109d58030000 movups xmm3, xmmword ptr [rbp + 0x358]
0x140a2c1b8 0f115830 movups xmmword ptr [rax + 0x30], xmm3
0x140a2c1bc 0f115020 movups xmmword ptr [rax + 0x20], xmm2
0x140a2c1c0 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x140a2c1c4 0f1100 movups xmmword ptr [rax], xmm0
0x140a2c1c7 0f1085b8020000 movups xmm0, xmmword ptr [rbp + 0x2b8]
0x140a2c1ce 0f108dc8020000 movups xmm1, xmmword ptr [rbp + 0x2c8]
0x140a2c1d5 0f1095d8020000 movups xmm2, xmmword ptr [rbp + 0x2d8]
0x140a2c1dc 0f109de8020000 movups xmm3, xmmword ptr [rbp + 0x2e8]
0x140a2c1e3 0f114070 movups xmmword ptr [rax + 0x70], xmm0
0x140a2c1e7 0f118880000000 movups xmmword ptr [rax + 0x80], xmm1
0x140a2c1ee 0f119090000000 movups xmmword ptr [rax + 0x90], xmm2
0x140a2c1f5 0f1198a0000000 movups xmmword ptr [rax + 0xa0], xmm3
0x140a2c1fc 0f1085f8020000 movups xmm0, xmmword ptr [rbp + 0x2f8]
0x140a2c203 0f1180b0000000 movups xmmword ptr [rax + 0xb0], xmm0
0x140a2c20a 0f108508030000 movups xmm0, xmmword ptr [rbp + 0x308]
0x140a2c211 0f1180c0000000 movups xmmword ptr [rax + 0xc0], xmm0
0x140a2c218 0f108518030000 movups xmm0, xmmword ptr [rbp + 0x318]
0x140a2c21f 0f1180d0000000 movups xmmword ptr [rax + 0xd0], xmm0
0x140a2c226 0f1085a8020000 movups xmm0, xmmword ptr [rbp + 0x2a8]
0x140a2c22d 0f118040010000 movups xmmword ptr [rax + 0x140], xmm0
0x140a2c234 0f108598020000 movups xmm0, xmmword ptr [rbp + 0x298]
0x140a2c23b 0f118030010000 movups xmmword ptr [rax + 0x130], xmm0
0x140a2c242 0f108588020000 movups xmm0, xmmword ptr [rbp + 0x288]
0x140a2c249 0f118020010000 movups xmmword ptr [rax + 0x120], xmm0
0x140a2c250 0f108548020000 movups xmm0, xmmword ptr [rbp + 0x248]
0x140a2c257 0f108d58020000 movups xmm1, xmmword ptr [rbp + 0x258]
0x140a2c25e 0f109568020000 movups xmm2, xmmword ptr [rbp + 0x268]
0x140a2c265 0f109d78020000 movups xmm3, xmmword ptr [rbp + 0x278]
0x140a2c26c 0f119810010000 movups xmmword ptr [rax + 0x110], xmm3
0x140a2c273 0f119000010000 movups xmmword ptr [rax + 0x100], xmm2
0x140a2c27a 0f1188f0000000 movups xmmword ptr [rax + 0xf0], xmm1
0x140a2c281 0f1180e0000000 movups xmmword ptr [rax + 0xe0], xmm0
0x140a2c288 0f1085d8010000 movups xmm0, xmmword ptr [rbp + 0x1d8]
0x140a2c28f 0f108de8010000 movups xmm1, xmmword ptr [rbp + 0x1e8]
0x140a2c296 0f1095f8010000 movups xmm2, xmmword ptr [rbp + 0x1f8]
0x140a2c29d 0f109d08020000 movups xmm3, xmmword ptr [rbp + 0x208]
0x140a2c2a4 0f118050010000 movups xmmword ptr [rax + 0x150], xmm0
0x140a2c2ab 0f118860010000 movups xmmword ptr [rax + 0x160], xmm1
0x140a2c2b2 0f119070010000 movups xmmword ptr [rax + 0x170], xmm2
0x140a2c2b9 0f119880010000 movups xmmword ptr [rax + 0x180], xmm3
0x140a2c2c0 0f108518020000 movups xmm0, xmmword ptr [rbp + 0x218]
0x140a2c2c7 0f118090010000 movups xmmword ptr [rax + 0x190], xmm0
0x140a2c2ce 0f108528020000 movups xmm0, xmmword ptr [rbp + 0x228]
0x140a2c2d5 0f1180a0010000 movups xmmword ptr [rax + 0x1a0], xmm0
0x140a2c2dc 0f108538020000 movups xmm0, xmmword ptr [rbp + 0x238]
0x140a2c2e3 0f1180b0010000 movups xmmword ptr [rax + 0x1b0], xmm0
0x140a2c2ea 0f108568010000 movups xmm0, xmmword ptr [rbp + 0x168]
0x140a2c2f1 0f108d78010000 movups xmm1, xmmword ptr [rbp + 0x178]
0x140a2c2f8 0f109588010000 movups xmm2, xmmword ptr [rbp + 0x188]
0x140a2c2ff 0f109d98010000 movups xmm3, xmmword ptr [rbp + 0x198]
0x140a2c306 0f1180c0010000 movups xmmword ptr [rax + 0x1c0], xmm0
0x140a2c30d 0f1188d0010000 movups xmmword ptr [rax + 0x1d0], xmm1
0x140a2c314 0f1190e0010000 movups xmmword ptr [rax + 0x1e0], xmm2
0x140a2c31b 0f1198f0010000 movups xmmword ptr [rax + 0x1f0], xmm3
0x140a2c322 0f1085a8010000 movups xmm0, xmmword ptr [rbp + 0x1a8]
0x140a2c329 0f118000020000 movups xmmword ptr [rax + 0x200], xmm0
0x140a2c330 0f1085c8010000 movups xmm0, xmmword ptr [rbp + 0x1c8]
0x140a2c337 0f118020020000 movups xmmword ptr [rax + 0x220], xmm0
0x140a2c33e 0f1085b8010000 movups xmm0, xmmword ptr [rbp + 0x1b8]
0x140a2c345 0f118010020000 movups xmmword ptr [rax + 0x210], xmm0
0x140a2c34c 0f108558010000 movups xmm0, xmmword ptr [rbp + 0x158]
0x140a2c353 0f118090020000 movups xmmword ptr [rax + 0x290], xmm0
0x140a2c35a 0f108548010000 movups xmm0, xmmword ptr [rbp + 0x148]
0x140a2c361 0f118080020000 movups xmmword ptr [rax + 0x280], xmm0
0x140a2c368 0f108538010000 movups xmm0, xmmword ptr [rbp + 0x138]
0x140a2c36f 0f118070020000 movups xmmword ptr [rax + 0x270], xmm0
0x140a2c376 0f1085f8000000 movups xmm0, xmmword ptr [rbp + 0xf8]
0x140a2c37d 0f108d08010000 movups xmm1, xmmword ptr [rbp + 0x108]
0x140a2c384 0f109518010000 movups xmm2, xmmword ptr [rbp + 0x118]
0x140a2c38b 0f109d28010000 movups xmm3, xmmword ptr [rbp + 0x128]
0x140a2c392 0f119860020000 movups xmmword ptr [rax + 0x260], xmm3
0x140a2c399 0f119050020000 movups xmmword ptr [rax + 0x250], xmm2
0x140a2c3a0 0f118840020000 movups xmmword ptr [rax + 0x240], xmm1
0x140a2c3a7 0f118030020000 movups xmmword ptr [rax + 0x230], xmm0
0x140a2c3ae 0f108588000000 movups xmm0, xmmword ptr [rbp + 0x88]
0x140a2c3b5 0f108d98000000 movups xmm1, xmmword ptr [rbp + 0x98]
0x140a2c3bc 0f1095a8000000 movups xmm2, xmmword ptr [rbp + 0xa8]
0x140a2c3c3 0f109db8000000 movups xmm3, xmmword ptr [rbp + 0xb8]
0x140a2c3ca 0f1180a0020000 movups xmmword ptr [rax + 0x2a0], xmm0
0x140a2c3d1 0f1188b0020000 movups xmmword ptr [rax + 0x2b0], xmm1
0x140a2c3d8 0f1190c0020000 movups xmmword ptr [rax + 0x2c0], xmm2
0x140a2c3df 0f1198d0020000 movups xmmword ptr [rax + 0x2d0], xmm3
0x140a2c3e6 0f1085c8000000 movups xmm0, xmmword ptr [rbp + 0xc8]
0x140a2c3ed 0f1180e0020000 movups xmmword ptr [rax + 0x2e0], xmm0
0x140a2c3f4 0f1085d8000000 movups xmm0, xmmword ptr [rbp + 0xd8]
0x140a2c3fb 0f1180f0020000 movups xmmword ptr [rax + 0x2f0], xmm0
0x140a2c402 0f1085e8000000 movups xmm0, xmmword ptr [rbp + 0xe8]
0x140a2c409 0f118000030000 movups xmmword ptr [rax + 0x300], xmm0
0x140a2c410 0f104518 movups xmm0, xmmword ptr [rbp + 0x18]
0x140a2c414 0f104d28 movups xmm1, xmmword ptr [rbp + 0x28]
0x140a2c418 0f105538 movups xmm2, xmmword ptr [rbp + 0x38]
0x140a2c41c 0f105d48 movups xmm3, xmmword ptr [rbp + 0x48]
0x140a2c420 0f118010030000 movups xmmword ptr [rax + 0x310], xmm0
0x140a2c427 0f118820030000 movups xmmword ptr [rax + 0x320], xmm1
0x140a2c42e 0f119030030000 movups xmmword ptr [rax + 0x330], xmm2
0x140a2c435 0f119840030000 movups xmmword ptr [rax + 0x340], xmm3
0x140a2c43c 0f104558 movups xmm0, xmmword ptr [rbp + 0x58]
0x140a2c440 0f118050030000 movups xmmword ptr [rax + 0x350], xmm0
0x140a2c447 0f104568 movups xmm0, xmmword ptr [rbp + 0x68]
0x140a2c44b 0f118060030000 movups xmmword ptr [rax + 0x360], xmm0
0x140a2c452 0f104578 movups xmm0, xmmword ptr [rbp + 0x78]
0x140a2c456 0f118070030000 movups xmmword ptr [rax + 0x370], xmm0
0x140a2c45d 0f1045a8 movups xmm0, xmmword ptr [rbp - 0x58]
0x140a2c461 0f104db8 movups xmm1, xmmword ptr [rbp - 0x48]
0x140a2c465 0f1055c8 movups xmm2, xmmword ptr [rbp - 0x38]
0x140a2c469 0f105dd8 movups xmm3, xmmword ptr [rbp - 0x28]
0x140a2c46d 0f118080030000 movups xmmword ptr [rax + 0x380], xmm0
0x140a2c474 0f118890030000 movups xmmword ptr [rax + 0x390], xmm1
0x140a2c47b 0f1190a0030000 movups xmmword ptr [rax + 0x3a0], xmm2
0x140a2c482 0f1198b0030000 movups xmmword ptr [rax + 0x3b0], xmm3
0x140a2c489 0f1045e8 movups xmm0, xmmword ptr [rbp - 0x18]
0x140a2c48d 0f1180c0030000 movups xmmword ptr [rax + 0x3c0], xmm0
0x140a2c494 0f1045f8 movups xmm0, xmmword ptr [rbp - 8]
0x140a2c498 0f1180d0030000 movups xmmword ptr [rax + 0x3d0], xmm0
0x140a2c49f 0f104508 movups xmm0, xmmword ptr [rbp + 8]
0x140a2c4a3 0f1180e0030000 movups xmmword ptr [rax + 0x3e0], xmm0
0x140a2c4aa 0f108598030000 movups xmm0, xmmword ptr [rbp + 0x398]
0x140a2c4b1 0f108da8030000 movups xmm1, xmmword ptr [rbp + 0x3a8]
0x140a2c4b8 0f1095b8030000 movups xmm2, xmmword ptr [rbp + 0x3b8]
0x140a2c4bf 0f109dc8030000 movups xmm3, xmmword ptr [rbp + 0x3c8]
0x140a2c4c6 0f1180f0030000 movups xmmword ptr [rax + 0x3f0], xmm0
0x140a2c4cd 0f118800040000 movups xmmword ptr [rax + 0x400], xmm1
0x140a2c4d4 0f119010040000 movups xmmword ptr [rax + 0x410], xmm2
0x140a2c4db 0f119820040000 movups xmmword ptr [rax + 0x420], xmm3
0x140a2c4e2 0f1085d8030000 movups xmm0, xmmword ptr [rbp + 0x3d8]
0x140a2c4e9 0f118030040000 movups xmmword ptr [rax + 0x430], xmm0
0x140a2c4f0 0f1085f8030000 movups xmm0, xmmword ptr [rbp + 0x3f8]
0x140a2c4f7 0f118050040000 movups xmmword ptr [rax + 0x450], xmm0
0x140a2c4fe 0f1085e8030000 movups xmm0, xmmword ptr [rbp + 0x3e8]
0x140a2c505 0f118040040000 movups xmmword ptr [rax + 0x440], xmm0
0x140a2c50c 48c746080a000000 mov qword ptr [rsi + 8], 0xa
0x140a2c514 48894610 mov qword ptr [rsi + 0x10], rax
0x140a2c518 48c746180a000000 mov qword ptr [rsi + 0x18], 0xa
0x140a2c520 48c7462000000000 mov qword ptr [rsi + 0x20], 0
0x140a2c528 48c7462808000000 mov qword ptr [rsi + 0x28], 8
0x140a2c530 0f57c0 xorps xmm0, xmm0
0x140a2c533 0f114630 movups xmmword ptr [rsi + 0x30], xmm0
0x140a2c537 48891e mov qword ptr [rsi], rbx
0x140a2c53a 4889f0 mov rax, rsi
0x140a2c53d 4881c4c8050000 add rsp, 0x5c8
0x140a2c544 5b pop rbx
0x140a2c545 5f pop rdi
0x140a2c546 5e pop rsi
0x140a2c547 5d pop rbp
0x140a2c548 c3 ret 
0x140a2c549 b908000000 mov ecx, 8
0x140a2c54e ba60040000 mov edx, 0x460
0x140a2c553 e8a8caea01 call 0x1428d9000
0x140a2c558 b908000000 mov ecx, 8
0x140a2c55d baa0010000 mov edx, 0x1a0
0x140a2c562 e899caea01 call 0x1428d9000
0x140a2c567 0f0b ud2 
0x140a2c569 0f1f8000000000 nop dword ptr [rax]
0x140a2c570 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a2c575 55 push rbp
0x140a2c576 56 push rsi
0x140a2c577 57 push rdi
0x140a2c578 53 push rbx
0x140a2c579 4883ec28 sub rsp, 0x28
0x140a2c57d 488daa80000000 lea rbp, [rdx + 0x80]
0x140a2c584 488d8d98030000 lea rcx, [rbp + 0x398]
0x140a2c58b e8802ebcff call 0x1405ef410
0x140a2c590 90 nop 
0x140a2c591 4883c428 add rsp, 0x28
0x140a2c595 5b pop rbx
0x140a2c596 5f pop rdi
0x140a2c597 5e pop rsi
0x140a2c598 5d pop rbp
0x140a2c599 c3 ret 
0x140a2c59a 660f1f440000 nop word ptr [rax + rax]
0x140a2c5a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a2c5a5 55 push rbp
0x140a2c5a6 56 push rsi
0x140a2c5a7 57 push rdi
0x140a2c5a8 53 push rbx
0x140a2c5a9 4883ec28 sub rsp, 0x28
0x140a2c5ad 488daa80000000 lea rbp, [rdx + 0x80]
0x140a2c5b4 ba60040000 mov edx, 0x460
0x140a2c5b9 41b808000000 mov r8d, 8
0x140a2c5bf 488b8d38050000 mov rcx, qword ptr [rbp + 0x538]
0x140a2c5c6 e85576beff call 0x140613c20
0x140a2c5cb 90 nop 
0x140a2c5cc 4883c428 add rsp, 0x28
0x140a2c5d0 5b pop rbx
0x140a2c5d1 5f pop rdi
0x140a2c5d2 5e pop rsi
0x140a2c5d3 5d pop rbp
0x140a2c5d4 c3 ret 
0x140a2c5d5 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140a2c5e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a2c5e5 55 push rbp
0x140a2c5e6 56 push rsi
0x140a2c5e7 57 push rdi
0x140a2c5e8 53 push rbx
0x140a2c5e9 4883ec28 sub rsp, 0x28
0x140a2c5ed 488daa80000000 lea rbp, [rdx + 0x80]
0x140a2c5f4 488d8d28030000 lea rcx, [rbp + 0x328]
0x140a2c5fb e880d25fff call 0x140029880
0x140a2c600 90 nop 
0x140a2c601 4883c428 add rsp, 0x28
0x140a2c605 5b pop rbx
0x140a2c606 5f pop rdi
0x140a2c607 5e pop rsi
0x140a2c608 5d pop rbp
0x140a2c609 c3 ret 
0x140a2c60a 660f1f440000 nop word ptr [rax + rax]
0x140a2c610 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a2c615 55 push rbp
0x140a2c616 56 push rsi
0x140a2c617 57 push rdi
0x140a2c618 53 push rbx
0x140a2c619 4883ec28 sub rsp, 0x28
0x140a2c61d 488daa80000000 lea rbp, [rdx + 0x80]
0x140a2c624 488d8db8020000 lea rcx, [rbp + 0x2b8]
0x140a2c62b e850d25fff call 0x140029880
0x140a2c630 90 nop 
0x140a2c631 4883c428 add rsp, 0x28
0x140a2c635 5b pop rbx
0x140a2c636 5f pop rdi
0x140a2c637 5e pop rsi
0x140a2c638 5d pop rbp
0x140a2c639 c3 ret 
0x140a2c63a 660f1f440000 nop word ptr [rax + rax]
0x140a2c640 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a2c645 55 push rbp
0x140a2c646 56 push rsi
0x140a2c647 57 push rdi
0x140a2c648 53 push rbx
0x140a2c649 4883ec28 sub rsp, 0x28
0x140a2c64d 488daa80000000 lea rbp, [rdx + 0x80]
0x140a2c654 488d8d48020000 lea rcx, [rbp + 0x248]
0x140a2c65b e820d25fff call 0x140029880
0x140a2c660 90 nop 
0x140a2c661 4883c428 add rsp, 0x28
0x140a2c665 5b pop rbx
0x140a2c666 5f pop rdi
0x140a2c667 5e pop rsi
0x140a2c668 5d pop rbp
0x140a2c669 c3 ret 
0x140a2c66a 660f1f440000 nop word ptr [rax + rax]
0x140a2c670 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a2c675 55 push rbp
0x140a2c676 56 push rsi
0x140a2c677 57 push rdi
0x140a2c678 53 push rbx
0x140a2c679 4883ec28 sub rsp, 0x28
0x140a2c67d 488daa80000000 lea rbp, [rdx + 0x80]
0x140a2c684 488d8dd8010000 lea rcx, [rbp + 0x1d8]
0x140a2c68b e8f0d15fff call 0x140029880
0x140a2c690 90 nop 
0x140a2c691 4883c428 add rsp, 0x28
0x140a2c695 5b pop rbx
0x140a2c696 5f pop rdi
0x140a2c697 5e pop rsi
0x140a2c698 5d pop rbp
0x140a2c699 c3 ret 
0x140a2c69a 660f1f440000 nop word ptr [rax + rax]
0x140a2c6a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a2c6a5 55 push rbp
0x140a2c6a6 56 push rsi
0x140a2c6a7 57 push rdi
0x140a2c6a8 53 push rbx
0x140a2c6a9 4883ec28 sub rsp, 0x28
0x140a2c6ad 488daa80000000 lea rbp, [rdx + 0x80]
0x140a2c6b4 488d8d68010000 lea rcx, [rbp + 0x168]
0x140a2c6bb e8c0d15fff call 0x140029880
0x140a2c6c0 90 nop 
0x140a2c6c1 4883c428 add rsp, 0x28
0x140a2c6c5 5b pop rbx
0x140a2c6c6 5f pop rdi
0x140a2c6c7 5e pop rsi
0x140a2c6c8 5d pop rbp
0x140a2c6c9 c3 ret 
0x140a2c6ca 660f1f440000 nop word ptr [rax + rax]
0x140a2c6d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a2c6d5 55 push rbp
0x140a2c6d6 56 push rsi
0x140a2c6d7 57 push rdi
0x140a2c6d8 53 push rbx
0x140a2c6d9 4883ec28 sub rsp, 0x28
0x140a2c6dd 488daa80000000 lea rbp, [rdx + 0x80]
0x140a2c6e4 488d8df8000000 lea rcx, [rbp + 0xf8]
0x140a2c6eb e890d15fff call 0x140029880
0x140a2c6f0 90 nop 
0x140a2c6f1 4883c428 add rsp, 0x28
0x140a2c6f5 5b pop rbx
0x140a2c6f6 5f pop rdi
0x140a2c6f7 5e pop rsi
0x140a2c6f8 5d pop rbp
0x140a2c6f9 c3 ret 
0x140a2c6fa 660f1f440000 nop word ptr [rax + rax]
0x140a2c700 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a2c705 55 push rbp
0x140a2c706 56 push rsi
0x140a2c707 57 push rdi
0x140a2c708 53 push rbx
0x140a2c709 4883ec28 sub rsp, 0x28
0x140a2c70d 488daa80000000 lea rbp, [rdx + 0x80]
0x140a2c714 488d8d88000000 lea rcx, [rbp + 0x88]
0x140a2c71b e860d15fff call 0x140029880
0x140a2c720 90 nop 
0x140a2c721 4883c428 add rsp, 0x28
0x140a2c725 5b pop rbx
0x140a2c726 5f pop rdi
0x140a2c727 5e pop rsi
0x140a2c728 5d pop rbp
0x140a2c729 c3 ret 
0x140a2c72a 660f1f440000 nop word ptr [rax + rax]
0x140a2c730 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a2c735 55 push rbp
0x140a2c736 56 push rsi
0x140a2c737 57 push rdi
0x140a2c738 53 push rbx
0x140a2c739 4883ec28 sub rsp, 0x28
0x140a2c73d 488daa80000000 lea rbp, [rdx + 0x80]
0x140a2c744 488d4d18 lea rcx, [rbp + 0x18]
0x140a2c748 e833d15fff call 0x140029880
0x140a2c74d 90 nop 
0x140a2c74e 4883c428 add rsp, 0x28
0x140a2c752 5b pop rbx
0x140a2c753 5f pop rdi
0x140a2c754 5e pop rsi
0x140a2c755 5d pop rbp
0x140a2c756 c3 ret 
0x140a2c757 660f1f840000000000 nop word ptr [rax + rax]
0x140a2c760 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140a2c765 55 push rbp
0x140a2c766 56 push rsi
0x140a2c767 57 push rdi
0x140a2c768 53 push rbx
0x140a2c769 4883ec28 sub rsp, 0x28
0x140a2c76d 488daa80000000 lea rbp, [rdx + 0x80]
0x140a2c774 488d4da8 lea rcx, [rbp - 0x58]
0x140a2c778 e803d15fff call 0x140029880
0x140a2c77d 90 nop 
0x140a2c77e 4883c428 add rsp, 0x28
0x140a2c782 5b pop rbx
0x140a2c783 5f pop rdi
0x140a2c784 5e pop rsi
0x140a2c785 5d pop rbp
0x140a2c786 c3 ret 
