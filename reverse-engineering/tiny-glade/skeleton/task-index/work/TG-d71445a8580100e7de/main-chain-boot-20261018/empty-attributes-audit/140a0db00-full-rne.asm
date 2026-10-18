140a0db00: push rbp
140a0db01: sub rsp, 0x30
140a0db05: lea rbp, [rsp + 0x30]
140a0db0a: mov qword ptr [rbp - 8], 0xfffffffffffffffe
140a0db12: cmp qword ptr [rdx + 0x10], 0
140a0db17: je 0x140a0db38
140a0db19: mov rax, qword ptr [rdx + 0x10]
140a0db1d: mov qword ptr [rcx + 0x18], rax
140a0db21: movups xmm0, xmmword ptr [rdx]
140a0db24: movups xmmword ptr [rcx + 8], xmm0
140a0db28: mov qword ptr [rcx], 2
140a0db2f: mov rax, rcx
140a0db32: add rsp, 0x30
140a0db36: pop rbp
140a0db37: ret 
140a0db38: mov qword ptr [rbp - 0x10], rdx
140a0db3c: lea rcx, [rip + 0x20b949a]
140a0db43: mov edx, 0x1f
140a0db48: call 0x1428d9430 ; core::panicking::panic
140a0db4d: ud2 
140a0db4f: nop 
140a0db50: mov qword ptr [rsp + 0x10], rdx
140a0db55: push rbp
140a0db56: sub rsp, 0x20
140a0db5a: lea rbp, [rdx + 0x30]
140a0db5e: mov rax, qword ptr [rbp - 0x10]
140a0db62: mov rdx, qword ptr [rax]
140a0db65: test rdx, rdx
140a0db68: je 0x140a0db7d
140a0db6a: mov rcx, qword ptr [rax + 8]
140a0db6e: shl rdx, 3
140a0db72: mov r8d, 4
140a0db78: call 0x140613c20
140a0db7d: nop 
140a0db7e: add rsp, 0x20
140a0db82: pop rbp
140a0db83: ret 
