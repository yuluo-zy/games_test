0x1408cea30 488b4118 mov rax, qword ptr [rcx + 0x18]
0x1408cea34 488b8080000000 mov rax, qword ptr [rax + 0x80]
0x1408cea3b 4c8b4218 mov r8, qword ptr [rdx + 0x18]
0x1408cea3f 493b8080000000 cmp rax, qword ptr [r8 + 0x80]
0x1408cea46 0f85b8000000 jne 0x1408ceb04
0x1408cea4c 440fb601 movzx r8d, byte ptr [rcx]
0x1408cea50 443a02 cmp r8b, byte ptr [rdx]
0x1408cea53 0f85ab000000 jne 0x1408ceb04
0x1408cea59 b001 mov al, 1
0x1408cea5b 4183f810 cmp r8d, 0x10
0x1408cea5f 0f8782000000 ja 0x1408ceae7
0x1408cea65 4589c0 mov r8d, r8d
0x1408cea68 4c8d0d855a1d02 lea r9, [rip + 0x21d5a85]
0x1408cea6f 4f630481 movsxd r8, dword ptr [r9 + r8*4]
0x1408cea73 4d01c8 add r8, r9
0x1408cea76 41ffe0 jmp r8
0x1408cea79 488b4108 mov rax, qword ptr [rcx + 8]
0x1408cea7d 483b4208 cmp rax, qword ptr [rdx + 8]
0x1408cea81 eb61 jmp 0x1408ceae4
0x1408cea83 448b4104 mov r8d, dword ptr [rcx + 4]
0x1408cea87 443b4204 cmp r8d, dword ptr [rdx + 4]
0x1408cea8b 7577 jne 0x1408ceb04
0x1408cea8d 4c8b4108 mov r8, qword ptr [rcx + 8]
0x1408cea91 4c3b4208 cmp r8, qword ptr [rdx + 8]
0x1408cea95 756d jne 0x1408ceb04
0x1408cea97 4d85c0 test r8, r8
0x1408cea9a 754b jne 0x1408ceae7
0x1408cea9c 488b4110 mov rax, qword ptr [rcx + 0x10]
0x1408ceaa0 483b4210 cmp rax, qword ptr [rdx + 0x10]
0x1408ceaa4 eb3e jmp 0x1408ceae4
0x1408ceaa6 f30f7e4108 movq xmm0, qword ptr [rcx + 8]
0x1408ceaab f30f7e4a08 movq xmm1, qword ptr [rdx + 8]
0x1408ceab0 660f76c8 pcmpeqd xmm1, xmm0
0x1408ceab4 660f70c150 pshufd xmm0, xmm1, 0x50
0x1408ceab9 660f50c0 movmskpd eax, xmm0
0x1408ceabd 83f803 cmp eax, 3
0x1408ceac0 7542 jne 0x1408ceb04
0x1408ceac2 8b4104 mov eax, dword ptr [rcx + 4]
0x1408ceac5 3b4204 cmp eax, dword ptr [rdx + 4]
0x1408ceac8 eb1a jmp 0x1408ceae4
0x1408ceaca f30f7e4104 movq xmm0, qword ptr [rcx + 4]
0x1408ceacf f30f7e4a04 movq xmm1, qword ptr [rdx + 4]
0x1408cead4 660f76c8 pcmpeqd xmm1, xmm0
0x1408cead8 660f70c150 pshufd xmm0, xmm1, 0x50
0x1408ceadd 660f50c0 movmskpd eax, xmm0
0x1408ceae1 83f803 cmp eax, 3
0x1408ceae4 0f94c0 sete al
0x1408ceae7 c3 ret
0x1408ceae8 488b4108 mov rax, qword ptr [rcx + 8]
0x1408ceaec 483b4208 cmp rax, qword ptr [rdx + 8]
0x1408ceaf0 7512 jne 0x1408ceb04
0x1408ceaf2 0fb64101 movzx eax, byte ptr [rcx + 1]
0x1408ceaf6 3a4201 cmp al, byte ptr [rdx + 1]
0x1408ceaf9 7509 jne 0x1408ceb04
0x1408ceafb 0fb64102 movzx eax, byte ptr [rcx + 2]
0x1408ceaff 3a4202 cmp al, byte ptr [rdx + 2]
0x1408ceb02 ebe0 jmp 0x1408ceae4
0x1408ceb04 31c0 xor eax, eax
0x1408ceb06 c3 ret
