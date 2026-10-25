140a0db90: push rbp
140a0db91: sub rsp, 0x30
140a0db95: lea rbp, [rsp + 0x30]
140a0db9a: mov qword ptr [rbp - 8], 0xfffffffffffffffe
140a0dba2: cmp qword ptr [rdx + 0x10], 0
140a0dba7: je 0x140a0dbc8
140a0dba9: mov rax, qword ptr [rdx + 0x10]
140a0dbad: mov qword ptr [rcx + 0x18], rax
140a0dbb1: movups xmm0, xmmword ptr [rdx]
140a0dbb4: movups xmmword ptr [rcx + 8], xmm0
140a0dbb8: mov qword ptr [rcx], 3
140a0dbbf: mov rax, rcx
140a0dbc2: add rsp, 0x30
140a0dbc6: pop rbp
140a0dbc7: ret 
140a0dbc8: mov qword ptr [rbp - 0x10], rdx
140a0dbcc: lea rcx, [rip + 0x20b940a]
140a0dbd3: mov edx, 0x1f
140a0dbd8: call 0x1428d9430 ; core::panicking::panic
140a0dbdd: ud2 
140a0dbdf: nop 
140a0dbe0: mov qword ptr [rsp + 0x10], rdx
140a0dbe5: push rbp
140a0dbe6: sub rsp, 0x20
140a0dbea: lea rbp, [rdx + 0x30]
140a0dbee: mov rcx, qword ptr [rbp - 0x10]
140a0dbf2: mov rax, qword ptr [rcx]
140a0dbf5: test rax, rax
140a0dbf8: je 0x140a0dc11
140a0dbfa: mov rcx, qword ptr [rcx + 8]
140a0dbfe: shl rax, 2
140a0dc02: lea rdx, [rax + rax*2]
140a0dc06: mov r8d, 4
140a0dc0c: call 0x140613c20
140a0dc11: nop 
140a0dc12: add rsp, 0x20
140a0dc16: pop rbp
140a0dc17: ret 
