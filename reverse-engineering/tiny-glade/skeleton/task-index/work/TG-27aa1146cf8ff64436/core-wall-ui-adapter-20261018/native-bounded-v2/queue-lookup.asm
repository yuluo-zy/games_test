140a8cb47: 488b85b80c0000 mov rax, qword ptr [rbp + 0xcb8]
140a8cb4e: 488b95600b0000 mov rdx, qword ptr [rbp + 0xb60]
140a8cb55: 488b4a08 mov rcx, qword ptr [rdx + 8]
140a8cb59: 4869521098000000 imul rdx, qword ptr [rdx + 0x10], 0x98
140a8cb61: 4989d1 mov r9, rdx
140a8cb64: 4885d2 test rdx, rdx
140a8cb67: 7417 je 0x140a8cb80
140a8cb69: 4c8d8198000000 lea r8, [rcx + 0x98]
140a8cb70: 498d9168ffffff lea rdx, [r9 - 0x98]
140a8cb77: 48394170 cmp qword ptr [rcx + 0x70], rax
140a8cb7b: 4c89c1 mov rcx, r8
140a8cb7e: 75e1 jne 0x140a8cb61
140a8cb80: 4c898d880b0000 mov qword ptr [rbp + 0xb88], r9
140a8cb87: 488b85400d0000 mov rax, qword ptr [rbp + 0xd40]
140a8cb8e: 488b78a0 mov rdi, qword ptr [rax - 0x60]
140a8cb92: 4c8d0cfd00000000 lea r9, [rdi*8]
140a8cb9a: 4889f8 mov rax, rdi
140a8cb9d: 48c1e83d shr rax, 0x3d
140a8cba1: 448995a80c0000 mov dword ptr [rbp + 0xca8], r10d
140a8cba8: 0f85f5150000 jne 0x140a8e1a3
140a8cbae: b900000000 mov ecx, 0
140a8cbb3: 48b8fcffffffffffff7f movabs rax, 0x7ffffffffffffffc
