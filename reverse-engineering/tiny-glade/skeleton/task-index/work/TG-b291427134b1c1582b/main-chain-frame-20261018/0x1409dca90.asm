0x1409dca90 4157 push r15
0x1409dca92 4156 push r14
0x1409dca94 4155 push r13
0x1409dca96 4154 push r12
0x1409dca98 56 push rsi
0x1409dca99 57 push rdi
0x1409dca9a 55 push rbp
0x1409dca9b 53 push rbx
0x1409dca9c 4881ecb8000000 sub rsp, 0xb8
0x1409dcaa3 8954242c mov dword ptr [rsp + 0x2c], edx
0x1409dcaa7 4488442423 mov byte ptr [rsp + 0x23], r8b
0x1409dcaac 44894c2424 mov dword ptr [rsp + 0x24], r9d
0x1409dcab1 4c8b39 mov r15, qword ptr [rcx]
0x1409dcab4 4c89f8 mov rax, r15
0x1409dcab7 48f7d8 neg rax
0x1409dcaba 0f801f020000 jo 0x1409dccdf
0x1409dcac0 89d7 mov edi, edx
0x1409dcac2 4889ce mov rsi, rcx
0x1409dcac5 488b4140 mov rax, qword ptr [rcx + 0x40]
0x1409dcac9 89d1 mov ecx, edx
0x1409dcacb 488b5040 mov rdx, qword ptr [rax + 0x40]
0x1409dcacf 4839ca cmp rdx, rcx
0x1409dcad2 0f8613020000 jbe 0x1409dcceb
0x1409dcad8 4489c3 mov ebx, r8d
0x1409dcadb 488b4038 mov rax, qword ptr [rax + 0x38]
0x1409dcadf 48c1e104 shl rcx, 4
0x1409dcae3 488d2c08 lea rbp, [rax + rcx]
0x1409dcae7 448b6648 mov r12d, dword ptr [rsi + 0x48]
0x1409dcaeb 44392408 cmp dword ptr [rax + rcx], r12d
0x1409dcaef 0f8595000000 jne 0x1409dcb8a
0x1409dcaf5 897c2428 mov dword ptr [rsp + 0x28], edi
0x1409dcaf9 4c897c2430 mov qword ptr [rsp + 0x30], r15
0x1409dcafe 488b4d08 mov rcx, qword ptr [rbp + 8]
0x1409dcb02 4c8b7610 mov r14, qword ptr [rsi + 0x10]
0x1409dcb06 4c39f1 cmp rcx, r14
0x1409dcb09 0f83e8010000 jae 0x1409dccf7
0x1409dcb0f 4c8b7e08 mov r15, qword ptr [rsi + 8]
0x1409dcb13 4c8d2c49 lea r13, [rcx + rcx*2]
0x1409dcb17 4b8d0caf lea rcx, [r15 + r13*4]
0x1409dcb1b 4883c104 add rcx, 4
0x1409dcb1f 43385caf08 cmp byte ptr [r15 + r13*4 + 8], bl
0x1409dcb24 7518 jne 0x1409dcb3e
0x1409dcb26 488d542424 lea rdx, [rsp + 0x24]
0x1409dcb2b 4889cf mov rdi, rcx
0x1409dcb2e e82d032100 call 0x140bece60
0x1409dcb33 4889f9 mov rcx, rdi
0x1409dcb36 84c0 test al, al
0x1409dcb38 0f85c5000000 jne 0x1409dcc03
0x1409dcb3e 4f8d3caf lea r15, [r15 + r13*4]
0x1409dcb42 4983c708 add r15, 8
0x1409dcb46 4c8d6c2424 lea r13, [rsp + 0x24]
0x1409dcb4b 4889cf mov rdi, rcx
0x1409dcb4e 4c89ea mov rdx, r13
0x1409dcb51 e80a032100 call 0x140bece60
0x1409dcb56 4c89f9 mov rcx, r15
0x1409dcb59 84c0 test al, al
0x1409dcb5b 4c8b7c2430 mov r15, qword ptr [rsp + 0x30]
0x1409dcb60 742e je 0x1409dcb90
0x1409dcb62 0fb601 movzx eax, byte ptr [rcx]
0x1409dcb65 83f817 cmp eax, 0x17
0x1409dcb68 0f84b3000000 je 0x1409dcc21
0x1409dcb6e 83f81a cmp eax, 0x1a
0x1409dcb71 0f84a0000000 je 0x1409dcc17
0x1409dcb77 83f81d cmp eax, 0x1d
0x1409dcb7a 7514 jne 0x1409dcb90
0x1409dcb7c 80fb17 cmp bl, 0x17
0x1409dcb7f 0f84a1000000 je 0x1409dcc26
0x1409dcb85 e9ad000000 jmp 0x1409dcc37
0x1409dcb8a 4c8b7610 mov r14, qword ptr [rsi + 0x10]
0x1409dcb8e eb2e jmp 0x1409dcbbe
0x1409dcb90 80fb37 cmp bl, 0x37
0x1409dcb93 0f859e000000 jne 0x1409dcc37
0x1409dcb99 0fb601 movzx eax, byte ptr [rcx]
0x1409dcb9c 04e7 add al, 0xe7
0x1409dcb9e 3c02 cmp al, 2
0x1409dcba0 0f8391000000 jae 0x1409dcc37
0x1409dcba6 c60137 mov byte ptr [rcx], 0x37
0x1409dcba9 c744242400000000 mov dword ptr [rsp + 0x24], 0
0x1409dcbb1 c70700000000 mov dword ptr [rdi], 0
0x1409dcbb7 4531c9 xor r9d, r9d
0x1409dcbba 8b7c2428 mov edi, dword ptr [rsp + 0x28]
0x1409dcbbe 44896500 mov dword ptr [rbp], r12d
0x1409dcbc2 4c897508 mov qword ptr [rbp + 8], r14
0x1409dcbc6 89d9 mov ecx, ebx
0x1409dcbc8 4489ca mov edx, r9d
0x1409dcbcb e8c0901400 call 0x140b25c90
0x1409dcbd0 89c3 mov ebx, eax
0x1409dcbd2 89d5 mov ebp, edx
0x1409dcbd4 4d39fe cmp r14, r15
0x1409dcbd7 750f jne 0x1409dcbe8
0x1409dcbd9 488d15786d0e02 lea rdx, [rip + 0x20e6d78]
0x1409dcbe0 4889f1 mov rcx, rsi
0x1409dcbe3 e80861b8ff call 0x140562cf0
0x1409dcbe8 488b4608 mov rax, qword ptr [rsi + 8]
0x1409dcbec 4b8d0c76 lea rcx, [r14 + r14*2]
0x1409dcbf0 893c88 mov dword ptr [rax + rcx*4], edi
0x1409dcbf3 895c8804 mov dword ptr [rax + rcx*4 + 4], ebx
0x1409dcbf7 40886c8808 mov byte ptr [rax + rcx*4 + 8], bpl
0x1409dcbfc 49ffc6 inc r14
0x1409dcbff 4c897610 mov qword ptr [rsi + 0x10], r14
0x1409dcc03 4881c4b8000000 add rsp, 0xb8
0x1409dcc0a 5b pop rbx
0x1409dcc0b 5d pop rbp
0x1409dcc0c 5f pop rdi
0x1409dcc0d 5e pop rsi
0x1409dcc0e 415c pop r12
0x1409dcc10 415d pop r13
0x1409dcc12 415e pop r14
0x1409dcc14 415f pop r15
0x1409dcc16 c3 ret 
0x1409dcc17 80fb02 cmp bl, 2
0x1409dcc1a 750f jne 0x1409dcc2b
0x1409dcc1c c60102 mov byte ptr [rcx], 2
0x1409dcc1f ebe2 jmp 0x1409dcc03
0x1409dcc21 80fb1d cmp bl, 0x1d
0x1409dcc24 7511 jne 0x1409dcc37
0x1409dcc26 c6011e mov byte ptr [rcx], 0x1e
0x1409dcc29 ebd8 jmp 0x1409dcc03
0x1409dcc2b 0fb6c3 movzx eax, bl
0x1409dcc2e 83f837 cmp eax, 0x37
0x1409dcc31 0f846fffffff je 0x1409dcba6
0x1409dcc37 488d44242c lea rax, [rsp + 0x2c]
0x1409dcc3c 4889442438 mov qword ptr [rsp + 0x38], rax
0x1409dcc41 488d0588f9ffff lea rax, [rip - 0x678]
0x1409dcc48 4889442440 mov qword ptr [rsp + 0x40], rax
0x1409dcc4d 48894c2448 mov qword ptr [rsp + 0x48], rcx
0x1409dcc52 488d0547f9ffff lea rax, [rip - 0x6b9]
0x1409dcc59 4889442450 mov qword ptr [rsp + 0x50], rax
0x1409dcc5e 48897c2458 mov qword ptr [rsp + 0x58], rdi
0x1409dcc63 488d0d76faffff lea rcx, [rip - 0x58a]
0x1409dcc6a 48894c2460 mov qword ptr [rsp + 0x60], rcx
0x1409dcc6f 488d542423 lea rdx, [rsp + 0x23]
0x1409dcc74 4889542468 mov qword ptr [rsp + 0x68], rdx
0x1409dcc79 4889442470 mov qword ptr [rsp + 0x70], rax
0x1409dcc7e 4c896c2478 mov qword ptr [rsp + 0x78], r13
0x1409dcc83 48898c2480000000 mov qword ptr [rsp + 0x80], rcx
0x1409dcc8b 488d056e6d0e02 lea rax, [rip + 0x20e6d6e]
0x1409dcc92 4889842488000000 mov qword ptr [rsp + 0x88], rax
0x1409dcc9a 48c784249000000006000000 mov qword ptr [rsp + 0x90], 6
0x1409dcca6 48c78424a800000000000000 mov qword ptr [rsp + 0xa8], 0
0x1409dccb2 488d442438 lea rax, [rsp + 0x38]
0x1409dccb7 4889842498000000 mov qword ptr [rsp + 0x98], rax
0x1409dccbf 48c78424a000000005000000 mov qword ptr [rsp + 0xa0], 5
0x1409dcccb 488d158e6d0e02 lea rdx, [rip + 0x20e6d8e]
0x1409dccd2 488d8c2488000000 lea rcx, [rsp + 0x88]
0x1409dccda e8b1c6ef01 call 0x1428d9390
0x1409dccdf 488d0d2a6c0e02 lea rcx, [rip + 0x20e6c2a]
0x1409dcce6 e825c6ef01 call 0x1428d9310
0x1409dcceb 4c8d05366c0e02 lea r8, [rip + 0x20e6c36]
0x1409dccf2 e821c8ef01 call 0x1428d9518
0x1409dccf7 4c8d05426c0e02 lea r8, [rip + 0x20e6c42]
0x1409dccfe 4c89f2 mov rdx, r14
0x1409dcd01 e812c8ef01 call 0x1428d9518
0x1409dcd06 cc int3 