1408da1b0: push rbp
1408da1b1: sub rsp, 0x50
1408da1b5: lea rbp, [rsp + 0x50]
1408da1ba: mov qword ptr [rbp - 8], 0xfffffffffffffffe
1408da1c2: mov rax, qword ptr [rcx]
1408da1c5: mov rcx, qword ptr [rcx + 8]
1408da1c9: mov r9d, 0x3b9aca00
1408da1cf: test rax, rax
1408da1d2: je 0x1408da1e6
1408da1d4: cmp eax, 1
1408da1d7: jne 0x1408da1f9
1408da1d9: call 0x1416764f0
1408da1de: cmp rax, 2
1408da1e2: je 0x1408da1f1
1408da1e4: jmp 0x1408da204
1408da1e6: call 0x14167a890
1408da1eb: cmp rax, 2
1408da1ef: jne 0x1408da204
1408da1f1: xor eax, eax
1408da1f3: add rsp, 0x50
1408da1f7: pop rbp
1408da1f8: ret 
1408da1f9: call 0x1416d1eb0
1408da1fe: cmp rax, 2
1408da202: je 0x1408da1f1
1408da204: mov qword ptr [rbp - 0x28], rax
1408da208: mov qword ptr [rbp - 0x20], rdx
1408da20c: test al, 1
1408da20e: je 0x1408da21b
1408da210: mov eax, 1
1408da215: add rsp, 0x50
1408da219: pop rbp
1408da21a: ret 
1408da21b: mov qword ptr [rbp - 0x10], rax
1408da21f: mov qword ptr [rbp - 0x18], rdx
1408da223: lea rcx, [rip + 0x21ca336]
1408da22a: lea r8, [rip + 0x21ca3a7]
1408da231: mov edx, 0x28
1408da236: call 0x1428d9430
1408da23b: ud2 
1408da23d: nop dword ptr [rax]
1408da240: mov qword ptr [rsp + 0x10], rdx
1408da245: push rbp
1408da246: sub rsp, 0x20
1408da24a: lea rbp, [rdx + 0x50]
1408da24e: cmp qword ptr [rbp - 0x10], 0
1408da253: setne al
1408da256: cmp qword ptr [rbp - 0x18], 0
1408da25b: sete cl
1408da25e: or cl, al
1408da260: jne 0x1408da26b
1408da262: lea rcx, [rbp - 0x20]
1408da266: call 0x140d4cc30
1408da26b: nop 
1408da26c: add rsp, 0x20
1408da270: pop rbp
1408da271: ret 
