140a7e3c2: mov rax, qword ptr [r13 + 0x28]
140a7e3c6: lea rcx, [r14 + r14*2]
140a7e3ca: mov qword ptr [rax + rcx*8], rdi
140a7e3ce: mov qword ptr [rax + rcx*8 + 8], rsi
140a7e3d3: mov dword ptr [rax + rcx*8 + 0x10], r15d
140a7e3d8: mov byte ptr [rax + rcx*8 + 0x14], 0
140a7e3dd: inc r14
140a7e3e0: mov qword ptr [r13 + 0x30], r14
140a7e3e4: inc qword ptr [r13 + 0x40]
