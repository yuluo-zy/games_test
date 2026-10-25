0x14140bd70 56 push rsi
0x14140bd71 57 push rdi
0x14140bd72 53 push rbx
0x14140bd73 4883ec60 sub rsp, 0x60
0x14140bd77 4889d0 mov rax, rdx
0x14140bd7a 4889ce mov rsi, rcx
0x14140bd7d 488b12 mov rdx, qword ptr [rdx]
0x14140bd80 4c8b5220 mov r10, qword ptr [rdx + 0x20]
0x14140bd84 488b4a28 mov rcx, qword ptr [rdx + 0x28]
0x14140bd88 4c39d1 cmp rcx, r10
0x14140bd8b 734d jae 0x14140bdda
0x14140bd8d 4c8b4218 mov r8, qword ptr [rdx + 0x18]
0x14140bd91 48ffc1 inc rcx
0x14140bd94 4c89d7 mov rdi, r10
0x14140bd97 48f7df neg rdi
0x14140bd9a 49b90026000001000000 movabs r9, 0x100002600
0x14140bda4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14140bdb0 450fb65c08ff movzx r11d, byte ptr [r8 + rcx - 1]
0x14140bdb6 4983fb2c cmp r11, 0x2c
0x14140bdba 0f8791000000 ja 0x14140be51
0x14140bdc0 4d0fa3d9 bt r9, r11
0x14140bdc4 731f jae 0x14140bde5
0x14140bdc6 48894a28 mov qword ptr [rdx + 0x28], rcx
0x14140bdca 4c8d1c0f lea r11, [rdi + rcx]
0x14140bdce 49ffc3 inc r11
0x14140bdd1 48ffc1 inc rcx
0x14140bdd4 4983fb01 cmp r11, 1
0x14140bdd8 75d6 jne 0x14140bdb0
0x14140bdda 48c744243802000000 mov qword ptr [rsp + 0x38], 2
0x14140bde3 eb5a jmp 0x14140be3f
0x14140bde5 4983fb2c cmp r11, 0x2c
0x14140bde9 7566 jne 0x14140be51
0x14140bdeb 0fb65808 movzx ebx, byte ptr [rax + 8]
0x14140bdef 84db test bl, bl
0x14140bdf1 7570 jne 0x14140be63
0x14140bdf3 48894a28 mov qword ptr [rdx + 0x28], rcx
0x14140bdf7 4c39d1 cmp rcx, r10
0x14140bdfa 733a jae 0x14140be36
0x14140bdfc 48ffc1 inc rcx
0x14140bdff b801000000 mov eax, 1
0x14140be04 4c29d0 sub rax, r10
0x14140be07 660f1f840000000000 nop word ptr [rax + rax]
0x14140be10 450fb65c08ff movzx r11d, byte ptr [r8 + rcx - 1]
0x14140be16 4983fb20 cmp r11, 0x20
0x14140be1a 7750 ja 0x14140be6c
0x14140be1c 4d0fa3d9 bt r9, r11
0x14140be20 734a jae 0x14140be6c
0x14140be22 48894a28 mov qword ptr [rdx + 0x28], rcx
0x14140be26 4c8d1408 lea r10, [rax + rcx]
0x14140be2a 49ffc2 inc r10
0x14140be2d 48ffc1 inc rcx
0x14140be30 4983fa02 cmp r10, 2
0x14140be34 75da jne 0x14140be10
0x14140be36 48c744243805000000 mov qword ptr [rsp + 0x38], 5
0x14140be3f 488d442438 lea rax, [rsp + 0x38]
0x14140be44 4889d1 mov rcx, rdx
0x14140be47 4889c2 mov rdx, rax
0x14140be4a e8d14a4901 call 0x1428a0920
0x14140be4f eb37 jmp 0x14140be88
0x14140be51 4183fb5d cmp r11d, 0x5d
0x14140be55 7508 jne 0x14140be5f
0x14140be57 c70600000000 mov dword ptr [rsi], 0
0x14140be5d eb54 jmp 0x14140beb3
0x14140be5f 0fb65808 movzx ebx, byte ptr [rax + 8]
0x14140be63 f6c301 test bl, 1
0x14140be66 7464 je 0x14140becc
0x14140be68 c6400800 mov byte ptr [rax + 8], 0
0x14140be6c 4180fb5d cmp r11b, 0x5d
0x14140be70 744c je 0x14140bebe
0x14140be72 488d4c2438 lea rcx, [rsp + 0x38]
0x14140be77 e824040200 call 0x14142c2a0
0x14140be7c f644243801 test byte ptr [rsp + 0x38], 1
0x14140be81 7411 je 0x14140be94
0x14140be83 488b442440 mov rax, qword ptr [rsp + 0x40]
0x14140be88 48894608 mov qword ptr [rsi + 8], rax
0x14140be8c c70602000000 mov dword ptr [rsi], 2
0x14140be92 eb1f jmp 0x14140beb3
0x14140be94 8b44245c mov eax, dword ptr [rsp + 0x5c]
0x14140be98 894624 mov dword ptr [rsi + 0x24], eax
0x14140be9b 0f1044243c movups xmm0, xmmword ptr [rsp + 0x3c]
0x14140bea0 0f104c244c movups xmm1, xmmword ptr [rsp + 0x4c]
0x14140bea5 0f114e14 movups xmmword ptr [rsi + 0x14], xmm1
0x14140bea9 0f114604 movups xmmword ptr [rsi + 4], xmm0
0x14140bead c70601000000 mov dword ptr [rsi], 1
0x14140beb3 4889f0 mov rax, rsi
0x14140beb6 4883c460 add rsp, 0x60
0x14140beba 5b pop rbx
0x14140bebb 5f pop rdi
0x14140bebc 5e pop rsi
0x14140bebd c3 ret 
0x14140bebe 48c744243812000000 mov qword ptr [rsp + 0x38], 0x12
0x14140bec7 e973ffffff jmp 0x14140be3f
0x14140becc 48c744243807000000 mov qword ptr [rsp + 0x38], 7
0x14140bed5 e965ffffff jmp 0x14140be3f
