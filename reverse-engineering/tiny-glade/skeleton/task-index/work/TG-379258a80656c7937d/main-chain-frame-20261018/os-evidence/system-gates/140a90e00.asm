140a90e00: mov rax, qword ptr [rcx]
140a90e03: cmp byte ptr [rax + 0x30], 1
140a90e07: jne 0x140a90e13
140a90e09: mov rax, qword ptr [rdx]
140a90e0c: cmp byte ptr [rax], 1
140a90e0f: sete al
140a90e12: ret 
140a90e13: xor eax, eax
140a90e15: ret 
