0x140259550 push r14
0x140259552 push rsi
0x140259553 push rdi
0x140259554 push rbx
0x140259555 sub rsp, 0x28
0x140259559 mov rsi, rdx
0x14025955c mov rdi, rcx
0x14025955f add rdx, 8
0x140259563 test byte ptr [rcx], 1
0x140259566 je 0x1402595de
0x140259568 mov rbx, qword ptr [rsi]
0x14025956b mov rax, qword ptr [rbx + 0x10]
0x14025956f cmp qword ptr [rbx], rax
0x140259572 je 0x140259686
0x140259578 mov rcx, qword ptr [rbx + 8]
0x14025957c mov byte ptr [rcx + rax], 0x7b
0x140259580 inc rax
0x140259583 mov qword ptr [rbx + 0x10], rax
0x140259587 lea r8, [rip + 0x274b640]
0x14025958e mov r9d, 9
0x140259594 mov rcx, rsi
0x140259597 call 0x1404b6100
0x14025959c test rax, rax
0x14025959f jne 0x140259675
0x1402595a5 add rdi, 4
0x1402595a9 mov rbx, qword ptr [rsi]
0x1402595ac mov rdx, qword ptr [rbx + 0x10]
0x1402595b0 cmp qword ptr [rbx], rdx
0x1402595b3 je 0x1402596e4
0x1402595b9 mov rax, qword ptr [rbx + 8]
0x1402595bd mov byte ptr [rax + rdx], 0x3a
0x1402595c1 inc rdx
0x1402595c4 mov qword ptr [rbx + 0x10], rdx
0x1402595c8 mov rcx, rdi
0x1402595cb mov rdx, rsi
0x1402595ce call 0x1400efde0
0x1402595d3 test rax, rax
0x1402595d6 jne 0x14025966b
0x1402595dc jmp 0x14025964a
0x1402595de mov rbx, qword ptr [rsi]
0x1402595e1 mov rax, qword ptr [rbx + 0x10]
0x1402595e5 cmp qword ptr [rbx], rax
0x1402595e8 je 0x1402596b5
0x1402595ee mov rcx, qword ptr [rbx + 8]
0x1402595f2 mov byte ptr [rcx + rax], 0x7b
0x1402595f6 inc rax
0x1402595f9 mov qword ptr [rbx + 0x10], rax
0x1402595fd lea r8, [rip + 0x274b5c4]
0x140259604 mov r9d, 6
0x14025960a mov rcx, rsi
0x14025960d call 0x1404b6100
0x140259612 test rax, rax
0x140259615 jne 0x140259675
0x140259617 add rdi, 4
0x14025961b mov rbx, qword ptr [rsi]
0x14025961e mov rdx, qword ptr [rbx + 0x10]
0x140259622 cmp qword ptr [rbx], rdx
0x140259625 je 0x14025970a
0x14025962b mov rax, qword ptr [rbx + 8]
0x14025962f mov byte ptr [rax + rdx], 0x3a
0x140259633 inc rdx
0x140259636 mov qword ptr [rbx + 0x10], rdx
0x14025963a mov rcx, rdi
0x14025963d mov rdx, rsi
0x140259640 call 0x1403d8a70
0x140259645 test rax, rax
0x140259648 jne 0x14025966b
0x14025964a mov rsi, qword ptr [rsi]
0x14025964d mov rdx, qword ptr [rsi + 0x10]
0x140259651 cmp qword ptr [rsi], rdx
0x140259654 je 0x140259730
0x14025965a mov rax, qword ptr [rsi + 8]
0x14025965e mov byte ptr [rax + rdx], 0x7d
0x140259662 inc rdx
0x140259665 mov qword ptr [rsi + 0x10], rdx
0x140259669 xor eax, eax
0x14025966b add rsp, 0x28
0x14025966f pop rbx
0x140259670 pop rdi
0x140259671 pop rsi
0x140259672 pop r14
0x140259674 ret 
0x140259675 mov rcx, rax
0x140259678 add rsp, 0x28
0x14025967c pop rbx
0x14025967d pop rdi
0x14025967e pop rsi
0x14025967f pop r14
0x140259681 jmp 0x1428c98a0
0x140259686 mov qword ptr [rsp + 0x20], 1
0x14025968f mov r8d, 1
0x140259695 mov r9d, 1
0x14025969b mov rcx, rbx
0x14025969e mov r14, rdx
0x1402596a1 mov rdx, rax
0x1402596a4 call 0x1428a29e0
0x1402596a9 mov rdx, r14
0x1402596ac mov rax, qword ptr [rbx + 0x10]
0x1402596b0 jmp 0x140259578
0x1402596b5 mov qword ptr [rsp + 0x20], 1
0x1402596be mov r8d, 1
0x1402596c4 mov r9d, 1
0x1402596ca mov rcx, rbx
0x1402596cd mov r14, rdx
0x1402596d0 mov rdx, rax
0x1402596d3 call 0x1428a29e0
0x1402596d8 mov rdx, r14
0x1402596db mov rax, qword ptr [rbx + 0x10]
0x1402596df jmp 0x1402595ee
0x1402596e4 mov qword ptr [rsp + 0x20], 1
0x1402596ed mov r8d, 1
0x1402596f3 mov r9d, 1
0x1402596f9 mov rcx, rbx
0x1402596fc call 0x1428a29e0
0x140259701 mov rdx, qword ptr [rbx + 0x10]
0x140259705 jmp 0x1402595b9
0x14025970a mov qword ptr [rsp + 0x20], 1
0x140259713 mov r8d, 1
0x140259719 mov r9d, 1
0x14025971f mov rcx, rbx
0x140259722 call 0x1428a29e0
0x140259727 mov rdx, qword ptr [rbx + 0x10]
0x14025972b jmp 0x14025962b
0x140259730 mov qword ptr [rsp + 0x20], 1
0x140259739 mov r8d, 1
0x14025973f mov r9d, 1
0x140259745 mov rcx, rsi
0x140259748 call 0x1428a29e0
0x14025974d mov rdx, qword ptr [rsi + 0x10]
0x140259751 jmp 0x14025965a
