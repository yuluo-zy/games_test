140b46000: push rbp
140b46001: push r14
140b46003: push rsi
140b46004: push rdi
140b46005: push rbx
140b46006: sub rsp, 0xa0
140b4600d: lea rbp, [rsp + 0x80]
140b46015: mov qword ptr [rbp + 0x18], 0xfffffffffffffffe
140b4601d: mov r14, rdx
140b46020: mov rsi, rcx
140b46023: call 0x140b9db60
140b46028: mov rdi, rax
140b4602b: mov rbx, rdx
140b4602e: mov qword ptr [rbp - 0x18], 0
140b46036: mov qword ptr [rbp - 0x10], 1
140b4603e: mov qword ptr [rbp - 8], 0
140b46046: mov qword ptr [rbp - 0x58], 0
140b4604e: mov qword ptr [rbp - 0x48], 0
140b46056: mov qword ptr [rbp - 0x38], 0x20
140b4605e: mov byte ptr [rbp - 0x30], 3
140b46062: lea rax, [rbp - 0x18]
140b46066: mov qword ptr [rbp - 0x28], rax
140b4606a: lea rax, [rip + 0x1fa92b7]
140b46071: mov qword ptr [rbp - 0x20], rax
140b46075: lea rdx, [rbp - 0x58]
140b46079: mov rcx, r14
140b4607c: call 0x140c10290
140b46081: test al, al
140b46083: jne 0x140b460d5
140b46085: mov rax, qword ptr [rbp - 0x18]
140b46089: mov qword ptr [rbp + 8], rax
140b4608d: mov r9, qword ptr [rbp - 0x10]
140b46091: mov rax, qword ptr [rbp - 8]
140b46095: mov qword ptr [rsp + 0x20], rax
140b4609a: mov rcx, rsi
140b4609d: mov rdx, rdi
140b460a0: mov r8, rbx
140b460a3: mov qword ptr [rbp], r9
140b460a7: call 0x140d62810
140b460ac: mov rdx, qword ptr [rbp + 8]
140b460b0: test rdx, rdx
140b460b3: je 0x140b460c4
140b460b5: mov r8d, 1
140b460bb: mov rcx, qword ptr [rbp]
140b460bf: call 0x140613c20
140b460c4: mov rax, rsi
140b460c7: add rsp, 0xa0
140b460ce: pop rbx
140b460cf: pop rdi
140b460d0: pop rsi
140b460d1: pop r14
140b460d3: pop rbp
140b460d4: ret 
140b460d5: lea rax, [rip + 0x1fa92b4]
140b460dc: mov qword ptr [rsp + 0x20], rax
140b460e1: lea rcx, [rip + 0x1fa9270]
140b460e8: lea r9, [rip + 0x1fa9329]
140b460ef: lea r8, [rbp + 0x17]
140b460f3: mov edx, 0x37
140b460f8: call 0x1428d9760
140b460fd: ud2 
140b460ff: nop 
140b46100: mov qword ptr [rsp + 0x10], rdx
140b46105: push rbp
140b46106: push r14
140b46108: push rsi
140b46109: push rdi
140b4610a: push rbx
140b4610b: sub rsp, 0x30
140b4610f: lea rbp, [rdx + 0x80]
140b46116: mov rdx, qword ptr [rbp - 0x18]
140b4611a: test rdx, rdx
140b4611d: je 0x140b4612e
140b4611f: mov rcx, qword ptr [rbp - 0x10]
140b46123: mov r8d, 1
140b46129: call 0x140613c20
140b4612e: nop 
140b4612f: add rsp, 0x30
140b46133: pop rbx
140b46134: pop rdi
140b46135: pop rsi
140b46136: pop r14
140b46138: pop rbp
140b46139: ret 
140b4613a: nop word ptr [rax + rax]
140b46140: mov qword ptr [rsp + 0x10], rdx
140b46145: push rbp
140b46146: push r14
140b46148: push rsi
140b46149: push rdi
140b4614a: push rbx
140b4614b: sub rsp, 0x30
140b4614f: lea rbp, [rdx + 0x80]
140b46156: cmp qword ptr [rbp + 8], 0
140b4615b: je 0x140b46170
140b4615d: mov r8d, 1
140b46163: mov rcx, qword ptr [rbp]
140b46167: mov rdx, qword ptr [rbp + 8]
140b4616b: call 0x140613c20
140b46170: nop 
140b46171: add rsp, 0x30
140b46175: pop rbx
140b46176: pop rdi
140b46177: pop rsi
140b46178: pop r14
140b4617a: pop rbp
140b4617b: ret 
