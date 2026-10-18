0x1409ee1e0 55 push rbp
0x1409ee1e1 4157 push r15
0x1409ee1e3 4156 push r14
0x1409ee1e5 4155 push r13
0x1409ee1e7 4154 push r12
0x1409ee1e9 56 push rsi
0x1409ee1ea 57 push rdi
0x1409ee1eb 53 push rbx
0x1409ee1ec 4881ec88070000 sub rsp, 0x788
0x1409ee1f3 488dac2480000000 lea rbp, [rsp + 0x80]
0x1409ee1fb 48c78500070000feffffff mov qword ptr [rbp + 0x700], 0xfffffffffffffffe
0x1409ee206 4c89ce mov rsi, r9
0x1409ee209 4d89c7 mov r15, r8
0x1409ee20c 4989d4 mov r12, rdx
0x1409ee20f 4889cb mov rbx, rcx
0x1409ee212 410f104104 movups xmm0, xmmword ptr [r9 + 4]
0x1409ee217 410f104914 movups xmm1, xmmword ptr [r9 + 0x14]
0x1409ee21c 0f298da0040000 movaps xmmword ptr [rbp + 0x4a0], xmm1
0x1409ee223 0f298590040000 movaps xmmword ptr [rbp + 0x490], xmm0
0x1409ee22a 488dbd00030000 lea rdi, [rbp + 0x300]
0x1409ee231 488d9590040000 lea rdx, [rbp + 0x490]
0x1409ee238 4889f9 mov rcx, rdi
0x1409ee23b e8f0741e00 call 0x140bd5730
0x1409ee240 4c8db590040000 lea r14, [rbp + 0x490]
0x1409ee247 4c89f1 mov rcx, r14
0x1409ee24a 4889fa mov rdx, rdi
0x1409ee24d 41b87a000000 mov r8d, 0x7a
0x1409ee253 e8d8711e00 call 0x140bd5430
0x1409ee258 4889f9 mov rcx, rdi
0x1409ee25b 4c89f2 mov rdx, r14
0x1409ee25e e82d711e00 call 0x140bd5390
0x1409ee263 4c89f1 mov rcx, r14
0x1409ee266 4889fa mov rdx, rdi
0x1409ee269 e802711e00 call 0x140bd5370
0x1409ee26e 488d8dc8060000 lea rcx, [rbp + 0x6c8]
0x1409ee275 4c89f2 mov rdx, r14
0x1409ee278 e823721e00 call 0x140bd54a0
0x1409ee27d 0fb685e4060000 movzx eax, byte ptr [rbp + 0x6e4]
0x1409ee284 fec8 dec al
0x1409ee286 0fb6c0 movzx eax, al
0x1409ee289 3c02 cmp al, 2
0x1409ee28b b901000000 mov ecx, 1
0x1409ee290 0f43c8 cmovae ecx, eax
0x1409ee293 888de4060000 mov byte ptr [rbp + 0x6e4], cl
0x1409ee299 488b85d8060000 mov rax, qword ptr [rbp + 0x6d8]
0x1409ee2a0 48898510030000 mov qword ptr [rbp + 0x310], rax
0x1409ee2a7 8b85e0060000 mov eax, dword ptr [rbp + 0x6e0]
0x1409ee2ad 898518030000 mov dword ptr [rbp + 0x318], eax
0x1409ee2b3 0fb685e4060000 movzx eax, byte ptr [rbp + 0x6e4]
0x1409ee2ba 88851c030000 mov byte ptr [rbp + 0x31c], al
0x1409ee2c0 0fb785e5060000 movzx eax, word ptr [rbp + 0x6e5]
0x1409ee2c7 6689851d030000 mov word ptr [rbp + 0x31d], ax
0x1409ee2ce 0fb685e7060000 movzx eax, byte ptr [rbp + 0x6e7]
0x1409ee2d5 88851f030000 mov byte ptr [rbp + 0x31f], al
0x1409ee2db 0f1085c8060000 movups xmm0, xmmword ptr [rbp + 0x6c8]
0x1409ee2e2 0f298500030000 movaps xmmword ptr [rbp + 0x300], xmm0
0x1409ee2e9 488d8da0040000 lea rcx, [rbp + 0x4a0]
0x1409ee2f0 4889fa mov rdx, rdi
0x1409ee2f3 e8e8891e00 call 0x140bd6ce0
0x1409ee2f8 488d05f96f0d02 lea rax, [rip + 0x20d6ff9]
0x1409ee2ff 48898590040000 mov qword ptr [rbp + 0x490], rax
0x1409ee306 48c7859804000014000000 mov qword ptr [rbp + 0x498], 0x14
0x1409ee311 488d9590040000 lea rdx, [rbp + 0x490]
0x1409ee318 4c89e1 mov rcx, r12
0x1409ee31b e8b0b11f00 call 0x140be94d0
0x1409ee320 0f1085c8060000 movups xmm0, xmmword ptr [rbp + 0x6c8]
0x1409ee327 0f108dd8060000 movups xmm1, xmmword ptr [rbp + 0x6d8]
0x1409ee32e 0f118560060000 movups xmmword ptr [rbp + 0x660], xmm0
0x1409ee335 0f118d70060000 movups xmmword ptr [rbp + 0x670], xmm1
0x1409ee33c 89855c060000 mov dword ptr [rbp + 0x65c], eax
0x1409ee342 0f105604 movups xmm2, xmmword ptr [rsi + 4]
0x1409ee346 0f105e14 movups xmm3, xmmword ptr [rsi + 0x14]
0x1409ee34a 0f299df0020000 movaps xmmword ptr [rbp + 0x2f0], xmm3
0x1409ee351 0f2995e0020000 movaps xmmword ptr [rbp + 0x2e0], xmm2
0x1409ee358 0f2985a0060000 movaps xmmword ptr [rbp + 0x6a0], xmm0
0x1409ee35f 0f298db0060000 movaps xmmword ptr [rbp + 0x6b0], xmm1
0x1409ee366 4c89a590060000 mov qword ptr [rbp + 0x690], r12
0x1409ee36d 4c89642430 mov qword ptr [rsp + 0x30], r12
0x1409ee372 4c89bd88060000 mov qword ptr [rbp + 0x688], r15
0x1409ee379 4c897c2428 mov qword ptr [rsp + 0x28], r15
0x1409ee37e 48c74424201a000000 mov qword ptr [rsp + 0x20], 0x1a
0x1409ee387 488d157e6f0d02 lea rdx, [rip + 0x20d6f7e]
0x1409ee38e 4c8d0d7c6f0d02 lea r9, [rip + 0x20d6f7c]
0x1409ee395 41b805000000 mov r8d, 5
0x1409ee39b 4c89f1 mov rcx, r14
0x1409ee39e e8fd4ec100 call 0x1416032a0
0x1409ee3a3 4889b550010000 mov qword ptr [rbp + 0x150], rsi
0x1409ee3aa c7850003000002000000 mov dword ptr [rbp + 0x300], 2
0x1409ee3b4 48897c2420 mov qword ptr [rsp + 0x20], rdi
0x1409ee3b9 488d8d50010000 lea rcx, [rbp + 0x150]
0x1409ee3c0 49b8ddb340cc9756ba11 movabs r8, 0x11ba5697cc40b3dd
0x1409ee3ca 4c89f2 mov rdx, r14
0x1409ee3cd 41b119 mov r9b, 0x19
0x1409ee3d0 e86bfeefff call 0x1408ee240
0x1409ee3d5 488d8d00030000 lea rcx, [rbp + 0x300]
0x1409ee3dc 488d9590040000 lea rdx, [rbp + 0x490]
0x1409ee3e3 41b890010000 mov r8d, 0x190
0x1409ee3e9 e81258f301 call 0x142923c00
0x1409ee3ee c685fd06000001 mov byte ptr [rbp + 0x6fd], 1
0x1409ee3f5 488d8d90040000 lea rcx, [rbp + 0x490]
0x1409ee3fc e89f731e00 call 0x140bd57a0
0x1409ee401 c685fd06000001 mov byte ptr [rbp + 0x6fd], 1
0x1409ee408 488d8d50010000 lea rcx, [rbp + 0x150]
0x1409ee40f 488d9590040000 lea rdx, [rbp + 0x490]
0x1409ee416 4531c0 xor r8d, r8d
0x1409ee419 e8c2741e00 call 0x140bd58e0
0x1409ee41e c685fd06000001 mov byte ptr [rbp + 0x6fd], 1
0x1409ee425 488d8d90040000 lea rcx, [rbp + 0x490]
0x1409ee42c 488db550010000 lea rsi, [rbp + 0x150]
0x1409ee433 4889f2 mov rdx, rsi
0x1409ee436 41b801000000 mov r8d, 1
0x1409ee43c 41b901000000 mov r9d, 1
0x1409ee442 e8d9741e00 call 0x140bd5920
0x1409ee447 0f108590040000 movups xmm0, xmmword ptr [rbp + 0x490]
0x1409ee44e 0f108da0040000 movups xmm1, xmmword ptr [rbp + 0x4a0]
0x1409ee455 0f1095b0040000 movups xmm2, xmmword ptr [rbp + 0x4b0]
0x1409ee45c 0f109dbc040000 movups xmm3, xmmword ptr [rbp + 0x4bc]
0x1409ee463 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
0x1409ee46a 0f298d60010000 movaps xmmword ptr [rbp + 0x160], xmm1
0x1409ee471 0f299570010000 movaps xmmword ptr [rbp + 0x170], xmm2
0x1409ee478 0f119d7c010000 movups xmmword ptr [rbp + 0x17c], xmm3
0x1409ee47f 488dbd5c060000 lea rdi, [rbp + 0x65c]
0x1409ee486 48897dc0 mov qword ptr [rbp - 0x40], rdi
0x1409ee48a 4889742420 mov qword ptr [rsp + 0x20], rsi
0x1409ee48f 488d4dc0 lea rcx, [rbp - 0x40]
0x1409ee493 488d9500030000 lea rdx, [rbp + 0x300]
0x1409ee49a 49b8b29298e63a9a2188 movabs r8, 0x88219a3ae69892b2
0x1409ee4a4 41b137 mov r9b, 0x37
0x1409ee4a7 e854f8efff call 0x1408edd00
0x1409ee4ac 488d8d90040000 lea rcx, [rbp + 0x490]
0x1409ee4b3 488d9500030000 lea rdx, [rbp + 0x300]
0x1409ee4ba 41b890010000 mov r8d, 0x190
0x1409ee4c0 e83b57f301 call 0x142923c00
0x1409ee4c5 c685f906000001 mov byte ptr [rbp + 0x6f9], 1
0x1409ee4cc 488d8de0020000 lea rcx, [rbp + 0x2e0]
0x1409ee4d3 e8b8721e00 call 0x140bd5790
0x1409ee4d8 48899d98060000 mov qword ptr [rbp + 0x698], rbx
0x1409ee4df 80bdc605000008 cmp byte ptr [rbp + 0x5c6], 8
0x1409ee4e6 7425 je 0x1409ee50d
0x1409ee4e8 488d8da0050000 lea rcx, [rbp + 0x5a0]
0x1409ee4ef bae8000000 mov edx, 0xe8
0x1409ee4f4 480395c8050000 add rdx, qword ptr [rbp + 0x5c8]
0x1409ee4fb 49b8db8b275c97ad604b movabs r8, 0x4b60ad975c278bdb
0x1409ee505 4989c1 mov r9, rax
0x1409ee508 e853d2f7ff call 0x14096b760
0x1409ee50d 4c8dbd00030000 lea r15, [rbp + 0x300]
0x1409ee514 488d9590040000 lea rdx, [rbp + 0x490]
0x1409ee51b 41b890010000 mov r8d, 0x190
0x1409ee521 4c89f9 mov rcx, r15
0x1409ee524 e8d756f301 call 0x142923c00
0x1409ee529 c685fb06000001 mov byte ptr [rbp + 0x6fb], 1
0x1409ee530 488d8da0060000 lea rcx, [rbp + 0x6a0]
0x1409ee537 e854721e00 call 0x140bd5790
0x1409ee53c 4889c1 mov rcx, rax
0x1409ee53f 48c1e920 shr rcx, 0x20
0x1409ee543 0f1085a0030000 movups xmm0, xmmword ptr [rbp + 0x3a0]
0x1409ee54a 0f298590010000 movaps xmmword ptr [rbp + 0x190], xmm0
0x1409ee551 0f108590030000 movups xmm0, xmmword ptr [rbp + 0x390]
0x1409ee558 0f298580010000 movaps xmmword ptr [rbp + 0x180], xmm0
0x1409ee55f 0f108580030000 movups xmm0, xmmword ptr [rbp + 0x380]
0x1409ee566 0f298570010000 movaps xmmword ptr [rbp + 0x170], xmm0
0x1409ee56d 0f108570030000 movups xmm0, xmmword ptr [rbp + 0x370]
0x1409ee574 0f298560010000 movaps xmmword ptr [rbp + 0x160], xmm0
0x1409ee57b 0f108560030000 movups xmm0, xmmword ptr [rbp + 0x360]
0x1409ee582 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
0x1409ee589 488b9538040000 mov rdx, qword ptr [rbp + 0x438]
0x1409ee590 4c8b8580040000 mov r8, qword ptr [rbp + 0x480]
0x1409ee597 4c8b8d10030000 mov r9, qword ptr [rbp + 0x310]
0x1409ee59e 4c898da0040000 mov qword ptr [rbp + 0x4a0], r9
0x1409ee5a5 0f108500030000 movups xmm0, xmmword ptr [rbp + 0x300]
0x1409ee5ac 0f298590040000 movaps xmmword ptr [rbp + 0x490], xmm0
0x1409ee5b3 0f108518030000 movups xmm0, xmmword ptr [rbp + 0x318]
0x1409ee5ba 0f2985c0040000 movaps xmmword ptr [rbp + 0x4c0], xmm0
0x1409ee5c1 4c8b8d28030000 mov r9, qword ptr [rbp + 0x328]
0x1409ee5c8 4c898dd0040000 mov qword ptr [rbp + 0x4d0], r9
0x1409ee5cf 0f108530030000 movups xmm0, xmmword ptr [rbp + 0x330]
0x1409ee5d6 0f1185a8040000 movups xmmword ptr [rbp + 0x4a8], xmm0
0x1409ee5dd 4c8b8d40030000 mov r9, qword ptr [rbp + 0x340]
0x1409ee5e4 4c898db8040000 mov qword ptr [rbp + 0x4b8], r9
0x1409ee5eb 0f1085b0030000 movups xmm0, xmmword ptr [rbp + 0x3b0]
0x1409ee5f2 0f108dc0030000 movups xmm1, xmmword ptr [rbp + 0x3c0]
0x1409ee5f9 0f1095d0030000 movups xmm2, xmmword ptr [rbp + 0x3d0]
0x1409ee600 0f109de0030000 movups xmm3, xmmword ptr [rbp + 0x3e0]
0x1409ee607 0f118508050000 movups xmmword ptr [rbp + 0x508], xmm0
0x1409ee60e 0f118d18050000 movups xmmword ptr [rbp + 0x518], xmm1
0x1409ee615 0f119528050000 movups xmmword ptr [rbp + 0x528], xmm2
0x1409ee61c 0f119d38050000 movups xmmword ptr [rbp + 0x538], xmm3
0x1409ee623 0f1085f0030000 movups xmm0, xmmword ptr [rbp + 0x3f0]
0x1409ee62a 0f118548050000 movups xmmword ptr [rbp + 0x548], xmm0
0x1409ee631 0f108500040000 movups xmm0, xmmword ptr [rbp + 0x400]
0x1409ee638 0f118558050000 movups xmmword ptr [rbp + 0x558], xmm0
0x1409ee63f 4c8b8d58030000 mov r9, qword ptr [rbp + 0x358]
0x1409ee646 4c898de8040000 mov qword ptr [rbp + 0x4e8], r9
0x1409ee64d 0f108548030000 movups xmm0, xmmword ptr [rbp + 0x348]
0x1409ee654 0f1185d8040000 movups xmmword ptr [rbp + 0x4d8], xmm0
0x1409ee65b 4c898570050000 mov qword ptr [rbp + 0x570], r8
0x1409ee662 48899568050000 mov qword ptr [rbp + 0x568], rdx
0x1409ee669 c785f004000001000000 mov dword ptr [rbp + 0x4f0], 1
0x1409ee673 8985f4040000 mov dword ptr [rbp + 0x4f4], eax
0x1409ee679 898df8040000 mov dword ptr [rbp + 0x4f8], ecx
0x1409ee67f c785fc04000001000000 mov dword ptr [rbp + 0x4fc], 1
0x1409ee689 488d8d50010000 lea rcx, [rbp + 0x150]
0x1409ee690 4c8da590040000 lea r12, [rbp + 0x490]
0x1409ee697 4c89e2 mov rdx, r12
0x1409ee69a e831eefeff call 0x1409dd4d0
0x1409ee69f 80bd3604000008 cmp byte ptr [rbp + 0x436], 8
0x1409ee6a6 7413 je 0x1409ee6bb
0x1409ee6a8 488d8d20040000 lea rcx, [rbp + 0x420]
0x1409ee6af c685fb06000000 mov byte ptr [rbp + 0x6fb], 0
0x1409ee6b6 e8f53d1f00 call 0x140be24b0
0x1409ee6bb 0fb685bc060000 movzx eax, byte ptr [rbp + 0x6bc]
0x1409ee6c2 8985f4060000 mov dword ptr [rbp + 0x6f4], eax
0x1409ee6c8 83f802 cmp eax, 2
0x1409ee6cb 0f8262040000 jb 0x1409eeb33
0x1409ee6d1 b801000000 mov eax, 1
0x1409ee6d6 488d9d20060000 lea rbx, [rbp + 0x620]
0x1409ee6dd eb16 jmp 0x1409ee6f5
0x1409ee6df 90 nop 
0x1409ee6e0 488b85e8060000 mov rax, qword ptr [rbp + 0x6e8]
0x1409ee6e7 ffc0 inc eax
0x1409ee6e9 3b85f4060000 cmp eax, dword ptr [rbp + 0x6f4]
0x1409ee6ef 0f843e040000 je 0x1409eeb33
0x1409ee6f5 4989fd mov r13, rdi
0x1409ee6f8 8d78ff lea edi, [rax - 1]
0x1409ee6fb be01000000 mov esi, 1
0x1409ee700 89f9 mov ecx, edi
0x1409ee702 d3e6 shl esi, cl
0x1409ee704 41be01000000 mov r14d, 1
0x1409ee70a 488985e8060000 mov qword ptr [rbp + 0x6e8], rax
0x1409ee711 89c1 mov ecx, eax
0x1409ee713 41d3e6 shl r14d, cl
0x1409ee716 488b8590060000 mov rax, qword ptr [rbp + 0x690]
0x1409ee71d 4889442430 mov qword ptr [rsp + 0x30], rax
0x1409ee722 488b8588060000 mov rax, qword ptr [rbp + 0x688]
0x1409ee729 4889442428 mov qword ptr [rsp + 0x28], rax
0x1409ee72e 48c74424201a000000 mov qword ptr [rsp + 0x20], 0x1a
0x1409ee737 41b805000000 mov r8d, 5
0x1409ee73d 4c89e1 mov rcx, r12
0x1409ee740 488d15e46b0d02 lea rdx, [rip + 0x20d6be4]
0x1409ee747 4c8d0dc36b0d02 lea r9, [rip + 0x20d6bc3]
0x1409ee74e e84d4bc100 call 0x1416032a0
0x1409ee753 c685ff06000001 mov byte ptr [rbp + 0x6ff], 1
0x1409ee75a 4c89f9 mov rcx, r15
0x1409ee75d e83e701e00 call 0x140bd57a0
0x1409ee762 c685ff06000001 mov byte ptr [rbp + 0x6ff], 1
0x1409ee769 4889d9 mov rcx, rbx
0x1409ee76c 4c89fa mov rdx, r15
0x1409ee76f 41b804000000 mov r8d, 4
0x1409ee775 e8a6701e00 call 0x140bd5820
0x1409ee77a c685ff06000001 mov byte ptr [rbp + 0x6ff], 1
0x1409ee781 4c89f9 mov rcx, r15
0x1409ee784 4889da mov rdx, rbx
0x1409ee787 4189f8 mov r8d, edi
0x1409ee78a e851711e00 call 0x140bd58e0
0x1409ee78f c685ff06000001 mov byte ptr [rbp + 0x6ff], 1
0x1409ee796 4889d9 mov rcx, rbx
0x1409ee799 4c89fa mov rdx, r15
0x1409ee79c 41b801000000 mov r8d, 1
0x1409ee7a2 41b901000000 mov r9d, 1
0x1409ee7a8 e873711e00 call 0x140bd5920
0x1409ee7ad 4c89ef mov rdi, r13
0x1409ee7b0 4c89ad80060000 mov qword ptr [rbp + 0x680], r13
0x1409ee7b7 0f108520060000 movups xmm0, xmmword ptr [rbp + 0x620]
0x1409ee7be 0f108d30060000 movups xmm1, xmmword ptr [rbp + 0x630]
0x1409ee7c5 0f109540060000 movups xmm2, xmmword ptr [rbp + 0x640]
0x1409ee7cc 0f109d4c060000 movups xmm3, xmmword ptr [rbp + 0x64c]
0x1409ee7d3 0f119d2c030000 movups xmmword ptr [rbp + 0x32c], xmm3
0x1409ee7da 0f299520030000 movaps xmmword ptr [rbp + 0x320], xmm2
0x1409ee7e1 0f298d10030000 movaps xmmword ptr [rbp + 0x310], xmm1
0x1409ee7e8 0f298500030000 movaps xmmword ptr [rbp + 0x300], xmm0
0x1409ee7ef 4c897c2420 mov qword ptr [rsp + 0x20], r15
0x1409ee7f4 488d8d80060000 lea rcx, [rbp + 0x680]
0x1409ee7fb 4c89e2 mov rdx, r12
0x1409ee7fe 49b8ddb340cc9756ba11 movabs r8, 0x11ba5697cc40b3dd
0x1409ee808 41b119 mov r9b, 0x19
0x1409ee80b e830faefff call 0x1408ee240
0x1409ee810 41b890010000 mov r8d, 0x190
0x1409ee816 4c89f9 mov rcx, r15
0x1409ee819 4c89e2 mov rdx, r12
0x1409ee81c e8df53f301 call 0x142923c00
0x1409ee821 c685fe06000001 mov byte ptr [rbp + 0x6fe], 1
0x1409ee828 4c89e1 mov rcx, r12
0x1409ee82b e8706f1e00 call 0x140bd57a0
0x1409ee830 4c8dad50010000 lea r13, [rbp + 0x150]
0x1409ee837 c685fe06000001 mov byte ptr [rbp + 0x6fe], 1
0x1409ee83e 4889d9 mov rcx, rbx
0x1409ee841 4c89e2 mov rdx, r12
0x1409ee844 41b808000000 mov r8d, 8
0x1409ee84a e8d16f1e00 call 0x140bd5820
0x1409ee84f c685fe06000001 mov byte ptr [rbp + 0x6fe], 1
0x1409ee856 4c89e1 mov rcx, r12
0x1409ee859 4889da mov rdx, rbx
0x1409ee85c 4c8b85e8060000 mov r8, qword ptr [rbp + 0x6e8]
0x1409ee863 e878701e00 call 0x140bd58e0
0x1409ee868 c685fe06000001 mov byte ptr [rbp + 0x6fe], 1
0x1409ee86f 4889d9 mov rcx, rbx
0x1409ee872 4c89e2 mov rdx, r12
0x1409ee875 41b801000000 mov r8d, 1
0x1409ee87b 41b901000000 mov r9d, 1
0x1409ee881 e89a701e00 call 0x140bd5920
0x1409ee886 0f108520060000 movups xmm0, xmmword ptr [rbp + 0x620]
0x1409ee88d 0f108d30060000 movups xmm1, xmmword ptr [rbp + 0x630]
0x1409ee894 0f109540060000 movups xmm2, xmmword ptr [rbp + 0x640]
0x1409ee89b 0f109d4c060000 movups xmm3, xmmword ptr [rbp + 0x64c]
0x1409ee8a2 0f298590040000 movaps xmmword ptr [rbp + 0x490], xmm0
0x1409ee8a9 0f298da0040000 movaps xmmword ptr [rbp + 0x4a0], xmm1
0x1409ee8b0 0f2995b0040000 movaps xmmword ptr [rbp + 0x4b0], xmm2
0x1409ee8b7 0f119dbc040000 movups xmmword ptr [rbp + 0x4bc], xmm3
0x1409ee8be 4889bd20060000 mov qword ptr [rbp + 0x620], rdi
0x1409ee8c5 4c89642420 mov qword ptr [rsp + 0x20], r12
0x1409ee8ca 4889d9 mov rcx, rbx
0x1409ee8cd 4c89fa mov rdx, r15
0x1409ee8d0 49b8b29298e63a9a2188 movabs r8, 0x88219a3ae69892b2
0x1409ee8da 41b137 mov r9b, 0x37
0x1409ee8dd e81ef4efff call 0x1408edd00
0x1409ee8e2 41b890010000 mov r8d, 0x190
0x1409ee8e8 4c89e9 mov rcx, r13
0x1409ee8eb 4c89fa mov rdx, r15
0x1409ee8ee e80d53f301 call 0x142923c00
0x1409ee8f3 89b500030000 mov dword ptr [rbp + 0x300], esi
0x1409ee8f9 89b504030000 mov dword ptr [rbp + 0x304], esi
0x1409ee8ff c7850803000001000000 mov dword ptr [rbp + 0x308], 1
0x1409ee909 0f2885a0060000 movaps xmm0, xmmword ptr [rbp + 0x6a0]
0x1409ee910 0f288db0060000 movaps xmm1, xmmword ptr [rbp + 0x6b0]
0x1409ee917 0f298da0040000 movaps xmmword ptr [rbp + 0x4a0], xmm1
0x1409ee91e 0f298590040000 movaps xmmword ptr [rbp + 0x490], xmm0
0x1409ee925 c685fa06000001 mov byte ptr [rbp + 0x6fa], 1
0x1409ee92c 4889d9 mov rcx, rbx
0x1409ee92f 4c89e2 mov rdx, r12
0x1409ee932 4d89f8 mov r8, r15
0x1409ee935 e8666d1e00 call 0x140bd56a0
0x1409ee93a c685fa06000001 mov byte ptr [rbp + 0x6fa], 1
0x1409ee941 4889d9 mov rcx, rbx
0x1409ee944 e8476e1e00 call 0x140bd5790
0x1409ee949 80bd8602000008 cmp byte ptr [rbp + 0x286], 8
0x1409ee950 7428 je 0x1409ee97a
0x1409ee952 488b9588020000 mov rdx, qword ptr [rbp + 0x288]
0x1409ee959 b9e8000000 mov ecx, 0xe8
0x1409ee95e 4801ca add rdx, rcx
0x1409ee961 488d8d60020000 lea rcx, [rbp + 0x260]
0x1409ee968 49b8db8b275c97ad604b movabs r8, 0x4b60ad975c278bdb
0x1409ee972 4989c1 mov r9, rax
0x1409ee975 e8e6cdf7ff call 0x14096b760
0x1409ee97a 41b890010000 mov r8d, 0x190
0x1409ee980 488d4dc0 lea rcx, [rbp - 0x40]
0x1409ee984 4c89ea mov rdx, r13
0x1409ee987 e87452f301 call 0x142923c00
0x1409ee98c 4489b500030000 mov dword ptr [rbp + 0x300], r14d
0x1409ee993 4489b504030000 mov dword ptr [rbp + 0x304], r14d
0x1409ee99a c7850803000001000000 mov dword ptr [rbp + 0x308], 1
0x1409ee9a4 0f2885a0060000 movaps xmm0, xmmword ptr [rbp + 0x6a0]
0x1409ee9ab 0f288db0060000 movaps xmm1, xmmword ptr [rbp + 0x6b0]
0x1409ee9b2 0f298da0040000 movaps xmmword ptr [rbp + 0x4a0], xmm1
0x1409ee9b9 0f298590040000 movaps xmmword ptr [rbp + 0x490], xmm0
0x1409ee9c0 c685fc06000001 mov byte ptr [rbp + 0x6fc], 1
0x1409ee9c7 4c89e9 mov rcx, r13
0x1409ee9ca 4c89e2 mov rdx, r12
0x1409ee9cd 4d89f8 mov r8, r15
0x1409ee9d0 e8cb6c1e00 call 0x140bd56a0
0x1409ee9d5 c685fc06000001 mov byte ptr [rbp + 0x6fc], 1
0x1409ee9dc 4c89e9 mov rcx, r13
0x1409ee9df e8ac6d1e00 call 0x140bd5790
0x1409ee9e4 4889c1 mov rcx, rax
0x1409ee9e7 48c1e920 shr rcx, 0x20
0x1409ee9eb 4c8d5520 lea r10, [rbp + 0x20]
0x1409ee9ef 410f104240 movups xmm0, xmmword ptr [r10 + 0x40]
0x1409ee9f4 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
0x1409ee9fb 410f1002 movups xmm0, xmmword ptr [r10]
0x1409ee9ff 410f104a10 movups xmm1, xmmword ptr [r10 + 0x10]
0x1409eea04 410f105220 movups xmm2, xmmword ptr [r10 + 0x20]
0x1409eea09 410f105a30 movups xmm3, xmmword ptr [r10 + 0x30]
0x1409eea0e 0f299d30030000 movaps xmmword ptr [rbp + 0x330], xmm3
0x1409eea15 0f299520030000 movaps xmmword ptr [rbp + 0x320], xmm2
0x1409eea1c 0f298d10030000 movaps xmmword ptr [rbp + 0x310], xmm1
0x1409eea23 0f298500030000 movaps xmmword ptr [rbp + 0x300], xmm0
0x1409eea2a 488b95f8000000 mov rdx, qword ptr [rbp + 0xf8]
0x1409eea31 4c8b8540010000 mov r8, qword ptr [rbp + 0x140]
0x1409eea38 4c8b4dd0 mov r9, qword ptr [rbp - 0x30]
0x1409eea3c 4c898da0040000 mov qword ptr [rbp + 0x4a0], r9
0x1409eea43 0f1045c0 movups xmm0, xmmword ptr [rbp - 0x40]
0x1409eea47 0f298590040000 movaps xmmword ptr [rbp + 0x490], xmm0
0x1409eea4e 4d8b4ac8 mov r9, qword ptr [r10 - 0x38]
0x1409eea52 4c8d9dc0040000 lea r11, [rbp + 0x4c0]
0x1409eea59 4d894b10 mov qword ptr [r11 + 0x10], r9
0x1409eea5d 410f1042b8 movups xmm0, xmmword ptr [r10 - 0x48]
0x1409eea62 410f1103 movups xmmword ptr [r11], xmm0
0x1409eea66 4d8b4ae0 mov r9, qword ptr [r10 - 0x20]
0x1409eea6a 4d894bf8 mov qword ptr [r11 - 8], r9
0x1409eea6e 410f1042d0 movups xmm0, xmmword ptr [r10 - 0x30]
0x1409eea73 410f1143e8 movups xmmword ptr [r11 - 0x18], xmm0
0x1409eea78 410f1082a0000000 movups xmm0, xmmword ptr [r10 + 0xa0]
0x1409eea80 410f118398000000 movups xmmword ptr [r11 + 0x98], xmm0
0x1409eea88 410f108290000000 movups xmm0, xmmword ptr [r10 + 0x90]
0x1409eea90 410f118388000000 movups xmmword ptr [r11 + 0x88], xmm0
0x1409eea98 410f104250 movups xmm0, xmmword ptr [r10 + 0x50]
0x1409eea9d 410f104a60 movups xmm1, xmmword ptr [r10 + 0x60]
0x1409eeaa2 410f105270 movups xmm2, xmmword ptr [r10 + 0x70]
0x1409eeaa7 410f109a80000000 movups xmm3, xmmword ptr [r10 + 0x80]
0x1409eeaaf 410f115b78 movups xmmword ptr [r11 + 0x78], xmm3
0x1409eeab4 410f115368 movups xmmword ptr [r11 + 0x68], xmm2
0x1409eeab9 410f114b58 movups xmmword ptr [r11 + 0x58], xmm1
0x1409eeabe 410f114348 movups xmmword ptr [r11 + 0x48], xmm0
0x1409eeac3 4d8b4af8 mov r9, qword ptr [r10 - 8]
0x1409eeac7 4d894b28 mov qword ptr [r11 + 0x28], r9
0x1409eeacb 410f1042e8 movups xmm0, xmmword ptr [r10 - 0x18]
0x1409eead0 410f114318 movups xmmword ptr [r11 + 0x18], xmm0
0x1409eead5 4c898570050000 mov qword ptr [rbp + 0x570], r8
0x1409eeadc 48899568050000 mov qword ptr [rbp + 0x568], rdx
0x1409eeae3 c785f004000001000000 mov dword ptr [rbp + 0x4f0], 1
0x1409eeaed 8985f4040000 mov dword ptr [rbp + 0x4f4], eax
0x1409eeaf3 898df8040000 mov dword ptr [rbp + 0x4f8], ecx
0x1409eeaf9 c785fc04000001000000 mov dword ptr [rbp + 0x4fc], 1
0x1409eeb03 4c89f9 mov rcx, r15
0x1409eeb06 4c89e2 mov rdx, r12
0x1409eeb09 e8c2e9feff call 0x1409dd4d0
0x1409eeb0e 80bdf600000008 cmp byte ptr [rbp + 0xf6], 8
0x1409eeb15 0f84c5fbffff je 0x1409ee6e0
0x1409eeb1b c685fc06000000 mov byte ptr [rbp + 0x6fc], 0
0x1409eeb22 488d8de0000000 lea rcx, [rbp + 0xe0]
0x1409eeb29 e882391f00 call 0x140be24b0
0x1409eeb2e e9adfbffff jmp 0x1409ee6e0
0x1409eeb33 8b8d7c060000 mov ecx, dword ptr [rbp + 0x67c]
0x1409eeb39 488b8598060000 mov rax, qword ptr [rbp + 0x698]
0x1409eeb40 894820 mov dword ptr [rax + 0x20], ecx
0x1409eeb43 0f10855c060000 movups xmm0, xmmword ptr [rbp + 0x65c]
0x1409eeb4a 0f108d6c060000 movups xmm1, xmmword ptr [rbp + 0x66c]
0x1409eeb51 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1409eeb55 0f1100 movups xmmword ptr [rax], xmm0
0x1409eeb58 4881c488070000 add rsp, 0x788
0x1409eeb5f 5b pop rbx
0x1409eeb60 5f pop rdi
0x1409eeb61 5e pop rsi
0x1409eeb62 415c pop r12
0x1409eeb64 415d pop r13
0x1409eeb66 415e pop r14
0x1409eeb68 415f pop r15
0x1409eeb6a 5d pop rbp
0x1409eeb6b c3 ret 
0x1409eeb6c 0f1f4000 nop dword ptr [rax]
0x1409eeb70 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409eeb75 55 push rbp
0x1409eeb76 4157 push r15
0x1409eeb78 4156 push r14
0x1409eeb7a 4155 push r13
0x1409eeb7c 4154 push r12
0x1409eeb7e 56 push rsi
0x1409eeb7f 57 push rdi
0x1409eeb80 53 push rbx
0x1409eeb81 4883ec38 sub rsp, 0x38
0x1409eeb85 488daa80000000 lea rbp, [rdx + 0x80]
0x1409eeb8c 80bd3604000008 cmp byte ptr [rbp + 0x436], 8
0x1409eeb93 7413 je 0x1409eeba8
0x1409eeb95 488d8d20040000 lea rcx, [rbp + 0x420]
0x1409eeb9c c685fb06000000 mov byte ptr [rbp + 0x6fb], 0
0x1409eeba3 e808391f00 call 0x140be24b0
0x1409eeba8 c685fb06000000 mov byte ptr [rbp + 0x6fb], 0
0x1409eebaf 4883c438 add rsp, 0x38
0x1409eebb3 5b pop rbx
0x1409eebb4 5f pop rdi
0x1409eebb5 5e pop rsi
0x1409eebb6 415c pop r12
0x1409eebb8 415d pop r13
0x1409eebba 415e pop r14
0x1409eebbc 415f pop r15
0x1409eebbe 5d pop rbp
0x1409eebbf c3 ret 
0x1409eebc0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409eebc5 55 push rbp
0x1409eebc6 4157 push r15
0x1409eebc8 4156 push r14
0x1409eebca 4155 push r13
0x1409eebcc 4154 push r12
0x1409eebce 56 push rsi
0x1409eebcf 57 push rdi
0x1409eebd0 53 push rbx
0x1409eebd1 4883ec38 sub rsp, 0x38
0x1409eebd5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409eebdc c685ff06000000 mov byte ptr [rbp + 0x6ff], 0
0x1409eebe3 488d8d90040000 lea rcx, [rbp + 0x490]
0x1409eebea e8c183eaff call 0x140896fb0
0x1409eebef c685ff06000000 mov byte ptr [rbp + 0x6ff], 0
0x1409eebf6 4883c438 add rsp, 0x38
0x1409eebfa 5b pop rbx
0x1409eebfb 5f pop rdi
0x1409eebfc 5e pop rsi
0x1409eebfd 415c pop r12
0x1409eebff 415d pop r13
0x1409eec01 415e pop r14
0x1409eec03 415f pop r15
0x1409eec05 5d pop rbp
0x1409eec06 c3 ret 
0x1409eec07 660f1f840000000000 nop word ptr [rax + rax]
0x1409eec10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409eec15 55 push rbp
0x1409eec16 4157 push r15
0x1409eec18 4156 push r14
0x1409eec1a 4155 push r13
0x1409eec1c 4154 push r12
0x1409eec1e 56 push rsi
0x1409eec1f 57 push rdi
0x1409eec20 53 push rbx
0x1409eec21 4883ec38 sub rsp, 0x38
0x1409eec25 488daa80000000 lea rbp, [rdx + 0x80]
0x1409eec2c c685fe06000000 mov byte ptr [rbp + 0x6fe], 0
0x1409eec33 488d8d00030000 lea rcx, [rbp + 0x300]
0x1409eec3a e87183eaff call 0x140896fb0
0x1409eec3f c685fe06000000 mov byte ptr [rbp + 0x6fe], 0
0x1409eec46 4883c438 add rsp, 0x38
0x1409eec4a 5b pop rbx
0x1409eec4b 5f pop rdi
0x1409eec4c 5e pop rsi
0x1409eec4d 415c pop r12
0x1409eec4f 415d pop r13
0x1409eec51 415e pop r14
0x1409eec53 415f pop r15
0x1409eec55 5d pop rbp
0x1409eec56 c3 ret 
0x1409eec57 660f1f840000000000 nop word ptr [rax + rax]
0x1409eec60 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409eec65 55 push rbp
0x1409eec66 4157 push r15
0x1409eec68 4156 push r14
0x1409eec6a 4155 push r13
0x1409eec6c 4154 push r12
0x1409eec6e 56 push rsi
0x1409eec6f 57 push rdi
0x1409eec70 53 push rbx
0x1409eec71 4883ec38 sub rsp, 0x38
0x1409eec75 488daa80000000 lea rbp, [rdx + 0x80]
0x1409eec7c c685fa06000000 mov byte ptr [rbp + 0x6fa], 0
0x1409eec83 488d8d50010000 lea rcx, [rbp + 0x150]
0x1409eec8a e82183eaff call 0x140896fb0
0x1409eec8f c685fa06000000 mov byte ptr [rbp + 0x6fa], 0
0x1409eec96 4883c438 add rsp, 0x38
0x1409eec9a 5b pop rbx
0x1409eec9b 5f pop rdi
0x1409eec9c 5e pop rsi
0x1409eec9d 415c pop r12
0x1409eec9f 415d pop r13
0x1409eeca1 415e pop r14
0x1409eeca3 415f pop r15
0x1409eeca5 5d pop rbp
0x1409eeca6 c3 ret 
0x1409eeca7 660f1f840000000000 nop word ptr [rax + rax]
0x1409eecb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409eecb5 55 push rbp
0x1409eecb6 4157 push r15
0x1409eecb8 4156 push r14
0x1409eecba 4155 push r13
0x1409eecbc 4154 push r12
0x1409eecbe 56 push rsi
0x1409eecbf 57 push rdi
0x1409eecc0 53 push rbx
0x1409eecc1 4883ec38 sub rsp, 0x38
0x1409eecc5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409eeccc 80bdf600000008 cmp byte ptr [rbp + 0xf6], 8
0x1409eecd3 7413 je 0x1409eece8
0x1409eecd5 c685fc06000000 mov byte ptr [rbp + 0x6fc], 0
0x1409eecdc 488d8de0000000 lea rcx, [rbp + 0xe0]
0x1409eece3 e8c8371f00 call 0x140be24b0
0x1409eece8 c685fc06000000 mov byte ptr [rbp + 0x6fc], 0
0x1409eecef 4883c438 add rsp, 0x38
0x1409eecf3 5b pop rbx
0x1409eecf4 5f pop rdi
0x1409eecf5 5e pop rsi
0x1409eecf6 415c pop r12
0x1409eecf8 415d pop r13
0x1409eecfa 415e pop r14
0x1409eecfc 415f pop r15
0x1409eecfe 5d pop rbp
0x1409eecff c3 ret 
0x1409eed00 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409eed05 55 push rbp
0x1409eed06 4157 push r15
0x1409eed08 4156 push r14
0x1409eed0a 4155 push r13
0x1409eed0c 4154 push r12
0x1409eed0e 56 push rsi
0x1409eed0f 57 push rdi
0x1409eed10 53 push rbx
0x1409eed11 4883ec38 sub rsp, 0x38
0x1409eed15 488daa80000000 lea rbp, [rdx + 0x80]
0x1409eed1c 80bdfc06000000 cmp byte ptr [rbp + 0x6fc], 0
0x1409eed23 7511 jne 0x1409eed36
0x1409eed25 4883c438 add rsp, 0x38
0x1409eed29 5b pop rbx
0x1409eed2a 5f pop rdi
0x1409eed2b 5e pop rsi
0x1409eed2c 415c pop r12
0x1409eed2e 415d pop r13
0x1409eed30 415e pop r14
0x1409eed32 415f pop r15
0x1409eed34 5d pop rbp
0x1409eed35 c3 ret 
0x1409eed36 488d4dc0 lea rcx, [rbp - 0x40]
0x1409eed3a e87182eaff call 0x140896fb0
0x1409eed3f ebe4 jmp 0x1409eed25
0x1409eed41 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409eed50 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409eed55 55 push rbp
0x1409eed56 4157 push r15
0x1409eed58 4156 push r14
0x1409eed5a 4155 push r13
0x1409eed5c 4154 push r12
0x1409eed5e 56 push rsi
0x1409eed5f 57 push rdi
0x1409eed60 53 push rbx
0x1409eed61 4883ec38 sub rsp, 0x38
0x1409eed65 488daa80000000 lea rbp, [rdx + 0x80]
0x1409eed6c 80bdfa06000000 cmp byte ptr [rbp + 0x6fa], 0
0x1409eed73 7511 jne 0x1409eed86
0x1409eed75 4883c438 add rsp, 0x38
0x1409eed79 5b pop rbx
0x1409eed7a 5f pop rdi
0x1409eed7b 5e pop rsi
0x1409eed7c 415c pop r12
0x1409eed7e 415d pop r13
0x1409eed80 415e pop r14
0x1409eed82 415f pop r15
0x1409eed84 5d pop rbp
0x1409eed85 c3 ret 
0x1409eed86 488d8d50010000 lea rcx, [rbp + 0x150]
0x1409eed8d e81e82eaff call 0x140896fb0
0x1409eed92 ebe1 jmp 0x1409eed75
0x1409eed94 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409eeda0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409eeda5 55 push rbp
0x1409eeda6 4157 push r15
0x1409eeda8 4156 push r14
0x1409eedaa 4155 push r13
0x1409eedac 4154 push r12
0x1409eedae 56 push rsi
0x1409eedaf 57 push rdi
0x1409eedb0 53 push rbx
0x1409eedb1 4883ec38 sub rsp, 0x38
0x1409eedb5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409eedbc 80bdfe06000000 cmp byte ptr [rbp + 0x6fe], 0
0x1409eedc3 7511 jne 0x1409eedd6
0x1409eedc5 4883c438 add rsp, 0x38
0x1409eedc9 5b pop rbx
0x1409eedca 5f pop rdi
0x1409eedcb 5e pop rsi
0x1409eedcc 415c pop r12
0x1409eedce 415d pop r13
0x1409eedd0 415e pop r14
0x1409eedd2 415f pop r15
0x1409eedd4 5d pop rbp
0x1409eedd5 c3 ret 
0x1409eedd6 488d8d00030000 lea rcx, [rbp + 0x300]
0x1409eeddd e8ce81eaff call 0x140896fb0
0x1409eede2 ebe1 jmp 0x1409eedc5
0x1409eede4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409eedf0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409eedf5 55 push rbp
0x1409eedf6 4157 push r15
0x1409eedf8 4156 push r14
0x1409eedfa 4155 push r13
0x1409eedfc 4154 push r12
0x1409eedfe 56 push rsi
0x1409eedff 57 push rdi
0x1409eee00 53 push rbx
0x1409eee01 4883ec38 sub rsp, 0x38
0x1409eee05 488daa80000000 lea rbp, [rdx + 0x80]
0x1409eee0c 80bdff06000000 cmp byte ptr [rbp + 0x6ff], 0
0x1409eee13 7511 jne 0x1409eee26
0x1409eee15 4883c438 add rsp, 0x38
0x1409eee19 5b pop rbx
0x1409eee1a 5f pop rdi
0x1409eee1b 5e pop rsi
0x1409eee1c 415c pop r12
0x1409eee1e 415d pop r13
0x1409eee20 415e pop r14
0x1409eee22 415f pop r15
0x1409eee24 5d pop rbp
0x1409eee25 c3 ret 
0x1409eee26 488d8d90040000 lea rcx, [rbp + 0x490]
0x1409eee2d e87e81eaff call 0x140896fb0
0x1409eee32 ebe1 jmp 0x1409eee15
0x1409eee34 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409eee40 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409eee45 55 push rbp
0x1409eee46 4157 push r15
0x1409eee48 4156 push r14
0x1409eee4a 4155 push r13
0x1409eee4c 4154 push r12
0x1409eee4e 56 push rsi
0x1409eee4f 57 push rdi
0x1409eee50 53 push rbx
0x1409eee51 4883ec38 sub rsp, 0x38
0x1409eee55 488daa80000000 lea rbp, [rdx + 0x80]
0x1409eee5c 80bdfb06000000 cmp byte ptr [rbp + 0x6fb], 0
0x1409eee63 7511 jne 0x1409eee76
0x1409eee65 4883c438 add rsp, 0x38
0x1409eee69 5b pop rbx
0x1409eee6a 5f pop rdi
0x1409eee6b 5e pop rsi
0x1409eee6c 415c pop r12
0x1409eee6e 415d pop r13
0x1409eee70 415e pop r14
0x1409eee72 415f pop r15
0x1409eee74 5d pop rbp
0x1409eee75 c3 ret 
0x1409eee76 488d8d00030000 lea rcx, [rbp + 0x300]
0x1409eee7d e82e81eaff call 0x140896fb0
0x1409eee82 ebe1 jmp 0x1409eee65
0x1409eee84 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409eee90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409eee95 55 push rbp
0x1409eee96 4157 push r15
0x1409eee98 4156 push r14
0x1409eee9a 4155 push r13
0x1409eee9c 4154 push r12
0x1409eee9e 56 push rsi
0x1409eee9f 57 push rdi
0x1409eeea0 53 push rbx
0x1409eeea1 4883ec38 sub rsp, 0x38
0x1409eeea5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409eeeac 80bdf906000000 cmp byte ptr [rbp + 0x6f9], 0
0x1409eeeb3 7511 jne 0x1409eeec6
0x1409eeeb5 4883c438 add rsp, 0x38
0x1409eeeb9 5b pop rbx
0x1409eeeba 5f pop rdi
0x1409eeebb 5e pop rsi
0x1409eeebc 415c pop r12
0x1409eeebe 415d pop r13
0x1409eeec0 415e pop r14
0x1409eeec2 415f pop r15
0x1409eeec4 5d pop rbp
0x1409eeec5 c3 ret 
0x1409eeec6 488d8d90040000 lea rcx, [rbp + 0x490]
0x1409eeecd e8de80eaff call 0x140896fb0
0x1409eeed2 ebe1 jmp 0x1409eeeb5
0x1409eeed4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409eeee0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409eeee5 55 push rbp
0x1409eeee6 4157 push r15
0x1409eeee8 4156 push r14
0x1409eeeea 4155 push r13
0x1409eeeec 4154 push r12
0x1409eeeee 56 push rsi
0x1409eeeef 57 push rdi
0x1409eeef0 53 push rbx
0x1409eeef1 4883ec38 sub rsp, 0x38
0x1409eeef5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409eeefc 80bdfd06000000 cmp byte ptr [rbp + 0x6fd], 0
0x1409eef03 7511 jne 0x1409eef16
0x1409eef05 4883c438 add rsp, 0x38
0x1409eef09 5b pop rbx
0x1409eef0a 5f pop rdi
0x1409eef0b 5e pop rsi
0x1409eef0c 415c pop r12
0x1409eef0e 415d pop r13
0x1409eef10 415e pop r14
0x1409eef12 415f pop r15
0x1409eef14 5d pop rbp
0x1409eef15 c3 ret 
0x1409eef16 488d8d00030000 lea rcx, [rbp + 0x300]
0x1409eef1d e88e80eaff call 0x140896fb0
0x1409eef22 ebe1 jmp 0x1409eef05
0x1409eef24 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1409eef30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409eef35 55 push rbp
0x1409eef36 4157 push r15
0x1409eef38 4156 push r14
0x1409eef3a 4155 push r13
0x1409eef3c 4154 push r12
0x1409eef3e 56 push rsi
0x1409eef3f 57 push rdi
0x1409eef40 53 push rbx
0x1409eef41 4883ec38 sub rsp, 0x38
0x1409eef45 488daa80000000 lea rbp, [rdx + 0x80]
0x1409eef4c 488d8d90040000 lea rcx, [rbp + 0x490]
0x1409eef53 e85880eaff call 0x140896fb0
0x1409eef58 90 nop 
0x1409eef59 4883c438 add rsp, 0x38
0x1409eef5d 5b pop rbx
0x1409eef5e 5f pop rdi
0x1409eef5f 5e pop rsi
0x1409eef60 415c pop r12
0x1409eef62 415d pop r13
0x1409eef64 415e pop r14
0x1409eef66 415f pop r15
0x1409eef68 5d pop rbp
0x1409eef69 c3 ret 
0x1409eef6a 660f1f440000 nop word ptr [rax + rax]
0x1409eef70 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409eef75 55 push rbp
0x1409eef76 4157 push r15
0x1409eef78 4156 push r14
0x1409eef7a 4155 push r13
0x1409eef7c 4154 push r12
0x1409eef7e 56 push rsi
0x1409eef7f 57 push rdi
0x1409eef80 53 push rbx
0x1409eef81 4883ec38 sub rsp, 0x38
0x1409eef85 488daa80000000 lea rbp, [rdx + 0x80]
0x1409eef8c c685fd06000000 mov byte ptr [rbp + 0x6fd], 0
0x1409eef93 488d8d00030000 lea rcx, [rbp + 0x300]
0x1409eef9a e81180eaff call 0x140896fb0
0x1409eef9f c685fd06000000 mov byte ptr [rbp + 0x6fd], 0
0x1409eefa6 4883c438 add rsp, 0x38
0x1409eefaa 5b pop rbx
0x1409eefab 5f pop rdi
0x1409eefac 5e pop rsi
0x1409eefad 415c pop r12
0x1409eefaf 415d pop r13
0x1409eefb1 415e pop r14
0x1409eefb3 415f pop r15
0x1409eefb5 5d pop rbp
0x1409eefb6 c3 ret 
0x1409eefb7 660f1f840000000000 nop word ptr [rax + rax]
0x1409eefc0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1409eefc5 55 push rbp
0x1409eefc6 4157 push r15
0x1409eefc8 4156 push r14
0x1409eefca 4155 push r13
0x1409eefcc 4154 push r12
0x1409eefce 56 push rsi
0x1409eefcf 57 push rdi
0x1409eefd0 53 push rbx
0x1409eefd1 4883ec38 sub rsp, 0x38
0x1409eefd5 488daa80000000 lea rbp, [rdx + 0x80]
0x1409eefdc c685f906000000 mov byte ptr [rbp + 0x6f9], 0
0x1409eefe3 488d8d90040000 lea rcx, [rbp + 0x490]
0x1409eefea e8c17feaff call 0x140896fb0
0x1409eefef c685f906000000 mov byte ptr [rbp + 0x6f9], 0
0x1409eeff6 4883c438 add rsp, 0x38
0x1409eeffa 5b pop rbx
0x1409eeffb 5f pop rdi
0x1409eeffc 5e pop rsi
0x1409eeffd 415c pop r12
0x1409eefff 415d pop r13
0x1409ef001 415e pop r14
0x1409ef003 415f pop r15
0x1409ef005 5d pop rbp
0x1409ef006 c3 ret 