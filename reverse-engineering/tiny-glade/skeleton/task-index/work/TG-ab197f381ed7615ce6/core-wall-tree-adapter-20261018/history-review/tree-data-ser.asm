14003d4e0: push rsi
14003d4e1: push rdi
14003d4e2: push rbx
14003d4e3: sub rsp, 0x40
14003d4e7: mov rsi, rcx
14003d4ea: mov rdi, qword ptr [rdx]
14003d4ed: mov rax, qword ptr [rdi + 0x10]
14003d4f1: cmp qword ptr [rdi], rax
14003d4f4: je 0x14003d613
14003d4fa: mov rcx, qword ptr [rdi + 8]
14003d4fe: mov byte ptr [rcx + rax], 0x7b
14003d502: inc rax
14003d505: mov qword ptr [rdi + 0x10], rax
14003d509: mov qword ptr [rsp + 0x30], rdx
14003d50e: mov byte ptr [rsp + 0x38], 1
14003d513: lea r9, [rsi + 0x18]
14003d517: lea rdx, [rip + 0x28eeca1]
14003d51e: lea rcx, [rsp + 0x30]
14003d523: mov r8d, 8
14003d529: call 0x1404c0740
14003d52e: test rax, rax
14003d531: jne 0x14003d60b
14003d537: lea r9, [rsi + 0x24]
14003d53b: lea rdx, [rip + 0x28eec85]
14003d542: lea rcx, [rsp + 0x30]
14003d547: mov r8d, 5
14003d54d: call 0x1404bdc80
14003d552: test rax, rax
14003d555: jne 0x14003d60b
14003d55b: lea r9, [rsi + 0x28]
14003d55f: lea rdx, [rip + 0x28eec66]
14003d566: lea rcx, [rsp + 0x30]
14003d56b: mov r8d, 0x10
14003d571: call 0x1404bdc80
14003d576: test rax, rax
14003d579: jne 0x14003d60b
14003d57f: lea r9, [rsi + 0x2c]
14003d583: lea rdx, [rip + 0x28eec33]
14003d58a: lea rcx, [rsp + 0x30]
14003d58f: mov r8d, 2
14003d595: call 0x1404befc0
14003d59a: test rax, rax
14003d59d: jne 0x14003d60b
14003d59f: lea rdx, [rip + 0x28eec36]
14003d5a6: lea rcx, [rsp + 0x30]
14003d5ab: mov r8d, 0xb
14003d5b1: mov r9, rsi
14003d5b4: call 0x1404becc0
14003d5b9: test rax, rax
14003d5bc: jne 0x14003d60b
14003d5be: add rsi, 0x30
14003d5c2: lea rdx, [rip + 0x28eec1e]
14003d5c9: lea rcx, [rsp + 0x30]
14003d5ce: mov r8d, 0xc
14003d5d4: mov r9, rsi
14003d5d7: call 0x1404bc390
14003d5dc: test rax, rax
14003d5df: jne 0x14003d60b
14003d5e1: cmp byte ptr [rsp + 0x38], 0
14003d5e6: je 0x14003d609
14003d5e8: mov rax, qword ptr [rsp + 0x30]
14003d5ed: mov rcx, qword ptr [rax]
14003d5f0: lea rdx, [rip + 0x28eeb19]
14003d5f7: lea r9, [rip + 0x28eec82]
14003d5fe: mov r8d, 1
14003d604: call 0x1402b5870
14003d609: xor eax, eax
14003d60b: add rsp, 0x40
14003d60f: pop rbx
14003d610: pop rdi
14003d611: pop rsi
14003d612: ret 
14003d613: mov qword ptr [rsp + 0x20], 1
14003d61c: mov r8d, 1
14003d622: mov r9d, 1
14003d628: mov rcx, rdi
14003d62b: mov rbx, rdx
14003d62e: mov rdx, rax
14003d631: call 0x1428a29e0
14003d636: mov rdx, rbx
14003d639: mov rax, qword ptr [rdi + 0x10]
14003d63d: jmp 0x14003d4fa
14003d642: int3 
14003d643: int3 
14003d644: int3 
14003d645: int3 
14003d646: int3 
14003d647: int3 
14003d648: int3 
14003d649: int3 
14003d64a: int3 
14003d64b: int3 
14003d64c: int3 
14003d64d: int3 
14003d64e: int3 
14003d64f: int3 
14003d650: push r14
14003d652: push rsi
14003d653: push rdi
14003d654: push rbx
14003d655: sub rsp, 0x38
14003d659: mov rdi, rdx
14003d65c: mov rsi, rcx
14003d65f: mov eax, dword ptr [rcx]
14003d661: add rdx, 8
14003d665: test eax, eax
14003d667: je 0x14003d781
14003d66d: cmp eax, 1
14003d670: jne 0x14003d89f
14003d676: mov rbx, qword ptr [rdi]
14003d679: mov rax, qword ptr [rbx + 0x10]
14003d67d: cmp qword ptr [rbx], rax
14003d680: je 0x14003d9c0
14003d686: mov rcx, qword ptr [rbx + 8]
14003d68a: mov byte ptr [rcx + rax], 0x7b
14003d68e: inc rax
14003d691: mov qword ptr [rbx + 0x10], rax
14003d695: lea r8, [rip + 0x28eeb65]
14003d69c: mov r9d, 4
14003d6a2: mov rcx, rdi
14003d6a5: call 0x1404b6100
14003d6aa: test rax, rax
14003d6ad: jne 0x14003d9ad
14003d6b3: mov rbx, qword ptr [rdi]
14003d6b6: mov rdx, qword ptr [rbx + 0x10]
14003d6ba: cmp qword ptr [rbx], rdx
14003d6bd: je 0x14003da4d
14003d6c3: mov rax, qword ptr [rbx + 8]
14003d6c7: mov byte ptr [rax + rdx], 0x3a
14003d6cb: inc rdx
14003d6ce: mov qword ptr [rbx + 0x10], rdx
14003d6d2: mov rbx, qword ptr [rdi]
14003d6d5: mov rdx, qword ptr [rbx + 0x10]
14003d6d9: cmp qword ptr [rbx], rdx
14003d6dc: je 0x14003da73
14003d6e2: mov rax, qword ptr [rbx + 8]
14003d6e6: mov byte ptr [rax + rdx], 0x7b
14003d6ea: inc rdx
14003d6ed: mov qword ptr [rbx + 0x10], rdx
14003d6f1: lea r9, [rsi + 0x20]
14003d6f5: mov qword ptr [rsp + 0x28], rdi
14003d6fa: mov byte ptr [rsp + 0x30], 1
14003d6ff: lea rdx, [rip + 0x28eeaf2]
14003d706: lea rcx, [rsp + 0x28]
14003d70b: mov r8d, 5
14003d711: call 0x1404beee0
14003d716: test rax, rax
14003d719: jne 0x14003d9b5
14003d71f: lea r9, [rsi + 4]
14003d723: lea rdx, [rip + 0x28ee9ff]
14003d72a: lea rcx, [rsp + 0x28]
14003d72f: mov r8d, 4
14003d735: call 0x1404c0740
14003d73a: test rax, rax
14003d73d: jne 0x14003d9b5
14003d743: add rsi, 0x10
14003d747: lea rdx, [rip + 0x28ee9df]
14003d74e: lea rcx, [rsp + 0x28]
14003d753: mov r8d, 2
14003d759: mov r9, rsi
14003d75c: call 0x1404c0740
14003d761: test rax, rax
14003d764: jne 0x14003d9b5
14003d76a: mov rax, qword ptr [rsp + 0x28]
14003d76f: movzx edx, byte ptr [rsp + 0x30]
14003d774: mov rcx, qword ptr [rax]
14003d777: call 0x14003e0a0
14003d77c: jmp 0x14003d898
14003d781: mov rbx, qword ptr [rdi]
14003d784: mov rax, qword ptr [rbx + 0x10]
14003d788: cmp qword ptr [rbx], rax
14003d78b: je 0x14003d9ef
14003d791: mov rcx, qword ptr [rbx + 8]
14003d795: mov byte ptr [rcx + rax], 0x7b
14003d799: inc rax
14003d79c: mov qword ptr [rbx + 0x10], rax
14003d7a0: lea r8, [rip + 0x28eea4c]
14003d7a7: mov r9d, 5
14003d7ad: mov rcx, rdi
14003d7b0: call 0x1404b6100
14003d7b5: test rax, rax
14003d7b8: jne 0x14003d9ad
14003d7be: mov rbx, qword ptr [rdi]
14003d7c1: mov rdx, qword ptr [rbx + 0x10]
14003d7c5: cmp qword ptr [rbx], rdx
14003d7c8: je 0x14003da99
14003d7ce: mov rax, qword ptr [rbx + 8]
14003d7d2: mov byte ptr [rax + rdx], 0x3a
14003d7d6: inc rdx
14003d7d9: mov qword ptr [rbx + 0x10], rdx
14003d7dd: mov rbx, qword ptr [rdi]
14003d7e0: mov rdx, qword ptr [rbx + 0x10]
14003d7e4: cmp qword ptr [rbx], rdx
14003d7e7: je 0x14003dabf
14003d7ed: mov rax, qword ptr [rbx + 8]
14003d7f1: mov byte ptr [rax + rdx], 0x7b
14003d7f5: inc rdx
14003d7f8: mov qword ptr [rbx + 0x10], rdx
14003d7fc: lea r9, [rsi + 8]
14003d800: mov qword ptr [rsp + 0x28], rdi
14003d805: mov byte ptr [rsp + 0x30], 1
14003d80a: lea rdx, [rip + 0x28ee9e7]
14003d811: lea rcx, [rsp + 0x28]
14003d816: mov r8d, 5
14003d81c: call 0x1404beee0
14003d821: test rax, rax
14003d824: jne 0x14003d9b5
14003d82a: add rsi, 0x10
14003d82e: lea rdx, [rip + 0x28ee9c8]
14003d835: lea rcx, [rsp + 0x28]
14003d83a: mov r8d, 4
14003d840: mov r9, rsi
14003d843: call 0x1404ba7f0
14003d848: test rax, rax
14003d84b: jne 0x14003d9b5
14003d851: mov rax, qword ptr [rsp + 0x28]
14003d856: mov rsi, qword ptr [rax]
14003d859: cmp byte ptr [rsp + 0x30], 0
14003d85e: je 0x14003d97a
14003d864: mov rdx, qword ptr [rsi + 0x10]
14003d868: cmp qword ptr [rsi], rdx
14003d86b: je 0x14003db31
14003d871: mov rax, qword ptr [rsi + 8]
14003d875: mov byte ptr [rax + rdx], 0x7d
14003d879: inc rdx
14003d87c: mov qword ptr [rsi + 0x10], rdx
14003d880: cmp qword ptr [rsi], rdx
14003d883: je 0x14003d987
14003d889: mov rax, qword ptr [rsi + 8]
14003d88d: mov byte ptr [rax + rdx], 0x7d
14003d891: inc rdx
14003d894: mov qword ptr [rsi + 0x10], rdx
14003d898: xor eax, eax
14003d89a: jmp 0x14003d9b5
14003d89f: mov rbx, qword ptr [rdi]
14003d8a2: mov rax, qword ptr [rbx + 0x10]
14003d8a6: cmp qword ptr [rbx], rax
14003d8a9: je 0x14003da1e
14003d8af: mov rcx, qword ptr [rbx + 8]
14003d8b3: mov byte ptr [rcx + rax], 0x7b
14003d8b7: inc rax
14003d8ba: mov qword ptr [rbx + 0x10], rax
14003d8be: lea r8, [rip + 0x28ee8f2]
14003d8c5: mov r9d, 6
14003d8cb: mov rcx, rdi
14003d8ce: call 0x1404b6100
14003d8d3: test rax, rax
14003d8d6: jne 0x14003d9ad
14003d8dc: mov rbx, qword ptr [rdi]
14003d8df: mov rdx, qword ptr [rbx + 0x10]
14003d8e3: cmp qword ptr [rbx], rdx
14003d8e6: je 0x14003dae5
14003d8ec: mov rax, qword ptr [rbx + 8]
14003d8f0: mov byte ptr [rax + rdx], 0x3a
14003d8f4: inc rdx
14003d8f7: mov qword ptr [rbx + 0x10], rdx
14003d8fb: mov rbx, qword ptr [rdi]
14003d8fe: mov rdx, qword ptr [rbx + 0x10]
14003d902: cmp qword ptr [rbx], rdx
14003d905: je 0x14003db0b
14003d90b: mov rax, qword ptr [rbx + 8]
14003d90f: mov byte ptr [rax + rdx], 0x7b
14003d913: inc rdx
14003d916: mov qword ptr [rbx + 0x10], rdx
14003d91a: lea r9, [rsi + 8]
14003d91e: mov qword ptr [rsp + 0x28], rdi
14003d923: mov byte ptr [rsp + 0x30], 1
14003d928: lea rdx, [rip + 0x28ee8c9]
14003d92f: lea rcx, [rsp + 0x28]
14003d934: mov r8d, 5
14003d93a: call 0x1404beee0
14003d93f: test rax, rax
14003d942: jne 0x14003d9b5
14003d944: add rsi, 0x10
14003d948: lea rdx, [rip + 0x28ee8ae]
14003d94f: lea rcx, [rsp + 0x28]
14003d954: mov r8d, 4
14003d95a: mov r9, rsi
14003d95d: call 0x1404ba7f0
14003d962: test rax, rax
14003d965: jne 0x14003d9b5
14003d967: mov rax, qword ptr [rsp + 0x28]
14003d96c: mov rsi, qword ptr [rax]
14003d96f: cmp byte ptr [rsp + 0x30], 0
14003d974: jne 0x14003d864
14003d97a: mov rdx, qword ptr [rsi + 0x10]
14003d97e: cmp qword ptr [rsi], rdx
14003d981: jne 0x14003d889
14003d987: mov qword ptr [rsp + 0x20], 1
14003d990: mov r8d, 1
14003d996: mov r9d, 1
14003d99c: mov rcx, rsi
14003d99f: call 0x1428a29e0
14003d9a4: mov rdx, qword ptr [rsi + 0x10]
14003d9a8: jmp 0x14003d889
14003d9ad: mov rcx, rax
14003d9b0: call 0x1428c98a0
14003d9b5: nop 
14003d9b6: add rsp, 0x38
14003d9ba: pop rbx
14003d9bb: pop rdi
14003d9bc: pop rsi
14003d9bd: pop r14
14003d9bf: ret 
14003d9c0: mov qword ptr [rsp + 0x20], 1
14003d9c9: mov r8d, 1
14003d9cf: mov r9d, 1
14003d9d5: mov rcx, rbx
14003d9d8: mov r14, rdx
14003d9db: mov rdx, rax
14003d9de: call 0x1428a29e0
14003d9e3: mov rdx, r14
14003d9e6: mov rax, qword ptr [rbx + 0x10]
14003d9ea: jmp 0x14003d686
14003d9ef: mov qword ptr [rsp + 0x20], 1
14003d9f8: mov r8d, 1
14003d9fe: mov r9d, 1
14003da04: mov rcx, rbx
14003da07: mov r14, rdx
14003da0a: mov rdx, rax
14003da0d: call 0x1428a29e0
14003da12: mov rdx, r14
14003da15: mov rax, qword ptr [rbx + 0x10]
14003da19: jmp 0x14003d791
14003da1e: mov qword ptr [rsp + 0x20], 1
14003da27: mov r8d, 1
14003da2d: mov r9d, 1
14003da33: mov rcx, rbx
14003da36: mov r14, rdx
14003da39: mov rdx, rax
14003da3c: call 0x1428a29e0
14003da41: mov rdx, r14
14003da44: mov rax, qword ptr [rbx + 0x10]
14003da48: jmp 0x14003d8af
14003da4d: mov qword ptr [rsp + 0x20], 1
14003da56: mov r8d, 1
14003da5c: mov r9d, 1
14003da62: mov rcx, rbx
14003da65: call 0x1428a29e0
14003da6a: mov rdx, qword ptr [rbx + 0x10]
14003da6e: jmp 0x14003d6c3
14003da73: mov qword ptr [rsp + 0x20], 1
14003da7c: mov r8d, 1
14003da82: mov r9d, 1
14003da88: mov rcx, rbx
14003da8b: call 0x1428a29e0
14003da90: mov rdx, qword ptr [rbx + 0x10]
14003da94: jmp 0x14003d6e2
14003da99: mov qword ptr [rsp + 0x20], 1
14003daa2: mov r8d, 1
14003daa8: mov r9d, 1
14003daae: mov rcx, rbx
14003dab1: call 0x1428a29e0
14003dab6: mov rdx, qword ptr [rbx + 0x10]
14003daba: jmp 0x14003d7ce
14003dabf: mov qword ptr [rsp + 0x20], 1
14003dac8: mov r8d, 1
14003dace: mov r9d, 1
14003dad4: mov rcx, rbx
14003dad7: call 0x1428a29e0
14003dadc: mov rdx, qword ptr [rbx + 0x10]
14003dae0: jmp 0x14003d7ed
14003dae5: mov qword ptr [rsp + 0x20], 1
14003daee: mov r8d, 1
14003daf4: mov r9d, 1
14003dafa: mov rcx, rbx
14003dafd: call 0x1428a29e0
14003db02: mov rdx, qword ptr [rbx + 0x10]
14003db06: jmp 0x14003d8ec
14003db0b: mov qword ptr [rsp + 0x20], 1
14003db14: mov r8d, 1
14003db1a: mov r9d, 1
14003db20: mov rcx, rbx
14003db23: call 0x1428a29e0
14003db28: mov rdx, qword ptr [rbx + 0x10]
14003db2c: jmp 0x14003d90b
14003db31: mov qword ptr [rsp + 0x20], 1
14003db3a: mov r8d, 1
14003db40: mov r9d, 1
14003db46: mov rcx, rsi
14003db49: call 0x1428a29e0
14003db4e: mov rdx, qword ptr [rsi + 0x10]
14003db52: jmp 0x14003d871
14003db57: int3 
14003db58: int3 
14003db59: int3 
14003db5a: int3 
14003db5b: int3 
14003db5c: int3 
14003db5d: int3 
14003db5e: int3 
14003db5f: int3 
14003db60: mov rax, rcx
14003db63: add r8, -2
14003db67: cmp r8, 0x11
14003db6b: ja 0x14003dc05
14003db71: lea rcx, [rip + 0x28eeae8]
14003db78: movsxd r8, dword ptr [rcx + r8*4]
14003db7c: add r8, rcx
14003db7f: jmp r8
14003db82: cmp word ptr [rdx], 0x7069
14003db87: jne 0x14003dc05
14003db89: mov byte ptr [rax + 1], 0
14003db8d: mov byte ptr [rax], 0
