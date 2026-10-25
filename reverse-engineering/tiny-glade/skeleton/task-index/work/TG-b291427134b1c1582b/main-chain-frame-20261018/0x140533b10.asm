0x140533b10 4157 push r15
0x140533b12 4156 push r14
0x140533b14 4155 push r13
0x140533b16 4154 push r12
0x140533b18 56 push rsi
0x140533b19 57 push rdi
0x140533b1a 55 push rbp
0x140533b1b 53 push rbx
0x140533b1c 4881ecb8000000 sub rsp, 0xb8
0x140533b23 8954242c mov dword ptr [rsp + 0x2c], edx
0x140533b27 4488442423 mov byte ptr [rsp + 0x23], r8b
0x140533b2c 44894c2424 mov dword ptr [rsp + 0x24], r9d
0x140533b31 4c8b39 mov r15, qword ptr [rcx]
0x140533b34 4c89f8 mov rax, r15
0x140533b37 48f7d8 neg rax
0x140533b3a 0f801f020000 jo 0x140533d5f
0x140533b40 89d7 mov edi, edx
0x140533b42 4889ce mov rsi, rcx
0x140533b45 488b4140 mov rax, qword ptr [rcx + 0x40]
0x140533b49 89d1 mov ecx, edx
0x140533b4b 488b5040 mov rdx, qword ptr [rax + 0x40]
0x140533b4f 4839ca cmp rdx, rcx
0x140533b52 0f8613020000 jbe 0x140533d6b
0x140533b58 4489c3 mov ebx, r8d
0x140533b5b 488b4038 mov rax, qword ptr [rax + 0x38]
0x140533b5f 48c1e104 shl rcx, 4
0x140533b63 488d2c08 lea rbp, [rax + rcx]
0x140533b67 448b6648 mov r12d, dword ptr [rsi + 0x48]
0x140533b6b 44392408 cmp dword ptr [rax + rcx], r12d
0x140533b6f 0f8595000000 jne 0x140533c0a
0x140533b75 897c2428 mov dword ptr [rsp + 0x28], edi
0x140533b79 4c897c2430 mov qword ptr [rsp + 0x30], r15
0x140533b7e 488b4d08 mov rcx, qword ptr [rbp + 8]
0x140533b82 4c8b7610 mov r14, qword ptr [rsi + 0x10]
0x140533b86 4c39f1 cmp rcx, r14
0x140533b89 0f83e8010000 jae 0x140533d77
0x140533b8f 4c8b7e08 mov r15, qword ptr [rsi + 8]
0x140533b93 4c8d2c49 lea r13, [rcx + rcx*2]
0x140533b97 4b8d0caf lea rcx, [r15 + r13*4]
0x140533b9b 4883c104 add rcx, 4
0x140533b9f 43385caf08 cmp byte ptr [r15 + r13*4 + 8], bl
0x140533ba4 7518 jne 0x140533bbe
0x140533ba6 488d542424 lea rdx, [rsp + 0x24]
0x140533bab 4889cf mov rdi, rcx
0x140533bae e8ad926b00 call 0x140bece60
0x140533bb3 4889f9 mov rcx, rdi
0x140533bb6 84c0 test al, al
0x140533bb8 0f85c5000000 jne 0x140533c83
0x140533bbe 4f8d3caf lea r15, [r15 + r13*4]
0x140533bc2 4983c708 add r15, 8
0x140533bc6 4c8d6c2424 lea r13, [rsp + 0x24]
0x140533bcb 4889cf mov rdi, rcx
0x140533bce 4c89ea mov rdx, r13
0x140533bd1 e88a926b00 call 0x140bece60
0x140533bd6 4c89f9 mov rcx, r15
0x140533bd9 84c0 test al, al
0x140533bdb 4c8b7c2430 mov r15, qword ptr [rsp + 0x30]
0x140533be0 742e je 0x140533c10
0x140533be2 0fb601 movzx eax, byte ptr [rcx]
0x140533be5 83f817 cmp eax, 0x17
0x140533be8 0f84b3000000 je 0x140533ca1
0x140533bee 83f81a cmp eax, 0x1a
0x140533bf1 0f84a0000000 je 0x140533c97
0x140533bf7 83f81d cmp eax, 0x1d
0x140533bfa 7514 jne 0x140533c10
0x140533bfc 80fb17 cmp bl, 0x17
0x140533bff 0f84a1000000 je 0x140533ca6
0x140533c05 e9ad000000 jmp 0x140533cb7
0x140533c0a 4c8b7610 mov r14, qword ptr [rsi + 0x10]
0x140533c0e eb2e jmp 0x140533c3e
0x140533c10 80fb37 cmp bl, 0x37
0x140533c13 0f859e000000 jne 0x140533cb7
0x140533c19 0fb601 movzx eax, byte ptr [rcx]
0x140533c1c 04e7 add al, 0xe7
0x140533c1e 3c02 cmp al, 2
0x140533c20 0f8391000000 jae 0x140533cb7
0x140533c26 c60137 mov byte ptr [rcx], 0x37
0x140533c29 c744242400000000 mov dword ptr [rsp + 0x24], 0
0x140533c31 c70700000000 mov dword ptr [rdi], 0
0x140533c37 4531c9 xor r9d, r9d
0x140533c3a 8b7c2428 mov edi, dword ptr [rsp + 0x28]
0x140533c3e 44896500 mov dword ptr [rbp], r12d
0x140533c42 4c897508 mov qword ptr [rbp + 8], r14
0x140533c46 89d9 mov ecx, ebx
0x140533c48 4489ca mov edx, r9d
0x140533c4b e840205f00 call 0x140b25c90
0x140533c50 89c3 mov ebx, eax
0x140533c52 89d5 mov ebp, edx
0x140533c54 4d39fe cmp r14, r15
0x140533c57 750f jne 0x140533c68
0x140533c59 488d1548114c02 lea rdx, [rip + 0x24c1148]
0x140533c60 4889f1 mov rcx, rsi
0x140533c63 e888f00200 call 0x140562cf0
0x140533c68 488b4608 mov rax, qword ptr [rsi + 8]
0x140533c6c 4b8d0c76 lea rcx, [r14 + r14*2]
0x140533c70 893c88 mov dword ptr [rax + rcx*4], edi
0x140533c73 895c8804 mov dword ptr [rax + rcx*4 + 4], ebx
0x140533c77 40886c8808 mov byte ptr [rax + rcx*4 + 8], bpl
0x140533c7c 49ffc6 inc r14
0x140533c7f 4c897610 mov qword ptr [rsi + 0x10], r14
0x140533c83 4881c4b8000000 add rsp, 0xb8
0x140533c8a 5b pop rbx
0x140533c8b 5d pop rbp
0x140533c8c 5f pop rdi
0x140533c8d 5e pop rsi
0x140533c8e 415c pop r12
0x140533c90 415d pop r13
0x140533c92 415e pop r14
0x140533c94 415f pop r15
0x140533c96 c3 ret 
0x140533c97 80fb02 cmp bl, 2
0x140533c9a 750f jne 0x140533cab
0x140533c9c c60102 mov byte ptr [rcx], 2
0x140533c9f ebe2 jmp 0x140533c83
0x140533ca1 80fb1d cmp bl, 0x1d
0x140533ca4 7511 jne 0x140533cb7
0x140533ca6 c6011e mov byte ptr [rcx], 0x1e
0x140533ca9 ebd8 jmp 0x140533c83
0x140533cab 0fb6c3 movzx eax, bl
0x140533cae 83f837 cmp eax, 0x37
0x140533cb1 0f846fffffff je 0x140533c26
0x140533cb7 488d44242c lea rax, [rsp + 0x2c]
0x140533cbc 4889442438 mov qword ptr [rsp + 0x38], rax
0x140533cc1 488d0568fdffff lea rax, [rip - 0x298]
0x140533cc8 4889442440 mov qword ptr [rsp + 0x40], rax
0x140533ccd 48894c2448 mov qword ptr [rsp + 0x48], rcx
0x140533cd2 488d0527fdffff lea rax, [rip - 0x2d9]
0x140533cd9 4889442450 mov qword ptr [rsp + 0x50], rax
0x140533cde 48897c2458 mov qword ptr [rsp + 0x58], rdi
0x140533ce3 488d0d96fdffff lea rcx, [rip - 0x26a]
0x140533cea 48894c2460 mov qword ptr [rsp + 0x60], rcx
0x140533cef 488d542423 lea rdx, [rsp + 0x23]
0x140533cf4 4889542468 mov qword ptr [rsp + 0x68], rdx
0x140533cf9 4889442470 mov qword ptr [rsp + 0x70], rax
0x140533cfe 4c896c2478 mov qword ptr [rsp + 0x78], r13
0x140533d03 48898c2480000000 mov qword ptr [rsp + 0x80], rcx
0x140533d0b 488d053e114c02 lea rax, [rip + 0x24c113e]
0x140533d12 4889842488000000 mov qword ptr [rsp + 0x88], rax
0x140533d1a 48c784249000000006000000 mov qword ptr [rsp + 0x90], 6
0x140533d26 48c78424a800000000000000 mov qword ptr [rsp + 0xa8], 0
0x140533d32 488d442438 lea rax, [rsp + 0x38]
0x140533d37 4889842498000000 mov qword ptr [rsp + 0x98], rax
0x140533d3f 48c78424a000000005000000 mov qword ptr [rsp + 0xa0], 5
0x140533d4b 488d155e114c02 lea rdx, [rip + 0x24c115e]
0x140533d52 488d8c2488000000 lea rcx, [rsp + 0x88]
0x140533d5a e831563a02 call 0x1428d9390
0x140533d5f 488d0dfa0f4c02 lea rcx, [rip + 0x24c0ffa]
0x140533d66 e8a5553a02 call 0x1428d9310
0x140533d6b 4c8d0506104c02 lea r8, [rip + 0x24c1006]
0x140533d72 e8a1573a02 call 0x1428d9518
0x140533d77 4c8d0512104c02 lea r8, [rip + 0x24c1012]
0x140533d7e 4c89f2 mov rdx, r14
0x140533d81 e892573a02 call 0x1428d9518
0x140533d86 cc int3 