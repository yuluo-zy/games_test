0x14140cd50 56 push rsi
0x14140cd51 57 push rdi
0x14140cd52 53 push rbx
0x14140cd53 4883ec50 sub rsp, 0x50
0x14140cd57 4889d7 mov rdi, rdx
0x14140cd5a 4889ce mov rsi, rcx
0x14140cd5d 488b4a20 mov rcx, qword ptr [rdx + 0x20]
0x14140cd61 488b4228 mov rax, qword ptr [rdx + 0x28]
0x14140cd65 4839c8 cmp rax, rcx
0x14140cd68 734c jae 0x14140cdb6
0x14140cd6a 488d5718 lea rdx, [rdi + 0x18]
0x14140cd6e 4c8b4718 mov r8, qword ptr [rdi + 0x18]
0x14140cd72 48ffc0 inc rax
0x14140cd75 48f7d9 neg rcx
0x14140cd78 49b90026000001000000 movabs r9, 0x100002600
0x14140cd82 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14140cd90 450fb65400ff movzx r10d, byte ptr [r8 + rax - 1]
0x14140cd96 4983fa20 cmp r10, 0x20
0x14140cd9a 7739 ja 0x14140cdd5
0x14140cd9c 4d0fa3d1 bt r9, r10
0x14140cda0 7333 jae 0x14140cdd5
0x14140cda2 48894728 mov qword ptr [rdi + 0x28], rax
0x14140cda6 4c8d1401 lea r10, [rcx + rax]
0x14140cdaa 49ffc2 inc r10
0x14140cdad 48ffc0 inc rax
0x14140cdb0 4983fa01 cmp r10, 1
0x14140cdb4 75da jne 0x14140cd90
0x14140cdb6 48c744243805000000 mov qword ptr [rsp + 0x38], 5
0x14140cdbf 488d542438 lea rdx, [rsp + 0x38]
0x14140cdc4 4889f9 mov rcx, rdi
0x14140cdc7 e8543b4901 call 0x1428a0920
0x14140cdcc 48894608 mov qword ptr [rsi + 8], rax
0x14140cdd0 c60601 mov byte ptr [rsi], 1
0x14140cdd3 eb5e jmp 0x14140ce33
0x14140cdd5 4180fa22 cmp r10b, 0x22
0x14140cdd9 7560 jne 0x14140ce3b
0x14140cddb 48894728 mov qword ptr [rdi + 0x28], rax
0x14140cddf 48c7471000000000 mov qword ptr [rdi + 0x10], 0
0x14140cde7 488d4c2438 lea rcx, [rsp + 0x38]
0x14140cdec 4989f8 mov r8, rdi
0x14140cdef e81cdb77ff call 0x140b8a910
0x14140cdf4 488b5c2438 mov rbx, qword ptr [rsp + 0x38]
0x14140cdf9 488b542440 mov rdx, qword ptr [rsp + 0x40]
0x14140cdfe 4883fb02 cmp rbx, 2
0x14140ce02 7509 jne 0x14140ce0d
0x14140ce04 48895608 mov qword ptr [rsi + 8], rdx
0x14140ce08 c60601 mov byte ptr [rsi], 1
0x14140ce0b eb26 jmp 0x14140ce33
0x14140ce0d 4c8b442448 mov r8, qword ptr [rsp + 0x48]
0x14140ce12 488d4c2428 lea rcx, [rsp + 0x28]
0x14140ce17 e854ea3800 call 0x14179b870
0x14140ce1c 0fb6442428 movzx eax, byte ptr [rsp + 0x28]
0x14140ce21 4885db test rbx, rbx
0x14140ce24 84c0 test al, al
0x14140ce26 752c jne 0x14140ce54
0x14140ce28 0fb6442429 movzx eax, byte ptr [rsp + 0x29]
0x14140ce2d 884601 mov byte ptr [rsi + 1], al
0x14140ce30 c60600 mov byte ptr [rsi], 0
0x14140ce33 4883c450 add rsp, 0x50
0x14140ce37 5b pop rbx
0x14140ce38 5f pop rdi
0x14140ce39 5e pop rsi
0x14140ce3a c3 ret 
0x14140ce3b 4c8d05aef48401 lea r8, [rip + 0x184f4ae]
0x14140ce42 488d542438 lea rdx, [rsp + 0x38]
0x14140ce47 4889f9 mov rcx, rdi
0x14140ce4a e8b1bd4e01 call 0x1428f8c00
0x14140ce4f 4889442430 mov qword ptr [rsp + 0x30], rax
0x14140ce54 488b4c2430 mov rcx, qword ptr [rsp + 0x30]
0x14140ce59 4889fa mov rdx, rdi
0x14140ce5c e89f534901 call 0x1428a2200
0x14140ce61 e966ffffff jmp 0x14140cdcc
