0x1408ceb40 56 push rsi
0x1408ceb41 57 push rdi
0x1408ceb42 53 push rbx
0x1408ceb43 4881ec80000000 sub rsp, 0x80
0x1408ceb4a 4c89cf mov rdi, r9
0x1408ceb4d 4889d3 mov rbx, rdx
0x1408ceb50 4889ce mov rsi, rcx
0x1408ceb53 f30f108424c0000000 movss xmm0, dword ptr [rsp + 0xc0]
0x1408ceb5c 410f284810 movaps xmm1, xmmword ptr [r8 + 0x10]
0x1408ceb61 f3410f105008 movss xmm2, dword ptr [r8 + 8]
0x1408ceb67 498b00 mov rax, qword ptr [r8]
0x1408ceb6a 0f294c2430 movaps xmmword ptr [rsp + 0x30], xmm1
0x1408ceb6f 4889442440 mov qword ptr [rsp + 0x40], rax
0x1408ceb74 f30f11542448 movss dword ptr [rsp + 0x48], xmm2
0x1408ceb7a 488b4218 mov rax, qword ptr [rdx + 0x18]
0x1408ceb7e 488b4830 mov rcx, qword ptr [rax + 0x30]
0x1408ceb82 488b4038 mov rax, qword ptr [rax + 0x38]
0x1408ceb86 488b5010 mov rdx, qword ptr [rax + 0x10]
0x1408ceb8a 48ffca dec rdx
0x1408ceb8d 4883e2f0 and rdx, 0xfffffffffffffff0
0x1408ceb91 4801ca add rdx, rcx
0x1408ceb94 4883c210 add rdx, 0x10
0x1408ceb98 410f1009 movups xmm1, xmmword ptr [r9]
0x1408ceb9c 0f294c2450 movaps xmmword ptr [rsp + 0x50], xmm1
0x1408ceba1 f2410f104910 movsd xmm1, qword ptr [r9 + 0x10]
0x1408ceba7 f20f114c2460 movsd qword ptr [rsp + 0x60], xmm1
0x1408cebad f30f11442420 movss dword ptr [rsp + 0x20], xmm0
0x1408cebb3 c644242801 mov byte ptr [rsp + 0x28], 1
0x1408cebb8 488d4c2468 lea rcx, [rsp + 0x68]
0x1408cebbd 4c8d442430 lea r8, [rsp + 0x30]
0x1408cebc2 4c8d4c2450 lea r9, [rsp + 0x50]
0x1408cebc7 ff5038 call qword ptr [rax + 0x38]
0x1408cebca 837c246804 cmp dword ptr [rsp + 0x68], 4
0x1408cebcf 7509 jne 0x1408cebda
0x1408cebd1 c7463402000000 mov dword ptr [rsi + 0x34], 2
0x1408cebd8 eb1a jmp 0x1408cebf4
0x1408cebda 48895c2420 mov qword ptr [rsp + 0x20], rbx
0x1408cebdf 4c8d442468 lea r8, [rsp + 0x68]
0x1408cebe4 4c8d4c2430 lea r9, [rsp + 0x30]
0x1408cebe9 4889f1 mov rcx, rsi
0x1408cebec 4889fa mov rdx, rdi
0x1408cebef e8acf8ffff call 0x1408ce4a0
0x1408cebf4 4889f0 mov rax, rsi
0x1408cebf7 4881c480000000 add rsp, 0x80
0x1408cebfe 5b pop rbx
0x1408cebff 5f pop rdi
0x1408cec00 5e pop rsi
0x1408cec01 c3 ret
