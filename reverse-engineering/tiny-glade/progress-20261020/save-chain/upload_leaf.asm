140957bf0: push rbp
140957bf1: push r15
140957bf3: push r14
140957bf5: push r13
140957bf7: push r12
140957bf9: push rsi
140957bfa: push rdi
140957bfb: push rbx
140957bfc: sub rsp, 0x78
140957c00: lea rbp, [rsp + 0x70]
140957c05: mov qword ptr [rbp], 0xfffffffffffffffe
140957c0d: mov rbx, r8
140957c10: mov rdi, rdx
140957c13: mov rsi, rcx
140957c16: call 0x140c16a00
140957c1b: test al, al
140957c1d: mov byte ptr [rbp - 8], al
140957c20: je 0x140957c82
140957c22: lea r15, [rip + 0x215c230]
140957c29: mov r14d, 0x4b
140957c2f: mov edx, 0x4b
140957c34: mov rcx, r15
140957c37: call 0x14094f620
140957c3c: cmp rax, 1
140957c40: jne 0x140957cf8
140957c46: mov r9, rdx
140957c49: test rdx, rdx
140957c4c: je 0x140957c8b
140957c4e: cmp r9, 0x4b
140957c52: jae 0x140957c89
140957c54: lea rax, [rip + 0x215c1fe]
140957c5b: cmp byte ptr [r9 + rax], 0xbf
140957c60: jg 0x140957c8b
140957c62: lea rax, [rip + 0x215c2bf]
140957c69: mov qword ptr [rsp + 0x20], rax
140957c6e: lea rcx, [rip + 0x215c1e4]
140957c75: mov edx, 0x4b
140957c7a: xor r8d, r8d
140957c7d: call 0x1428d9940
140957c82: xor eax, eax
140957c84: jmp 0x140957e59
140957c89: jne 0x140957c62
140957c8b: lea r15, [rip + 0x215c1c7]
140957c92: mov rcx, r15
140957c95: mov rdx, r9
140957c98: call 0x14094f620
140957c9d: cmp rax, 1
140957ca1: jne 0x140957cf8
140957ca3: mov r8, rdx
140957ca6: add r8, 2
140957caa: je 0x140957ce5
140957cac: cmp r8, 0x4b
140957cb0: jae 0x140957ce3
140957cb2: lea rax, [rip + 0x215c1a0]
140957cb9: cmp byte ptr [r8 + rax], 0xbf
140957cbe: jg 0x140957ce5
140957cc0: lea rax, [rip + 0x215c279]
140957cc7: mov qword ptr [rsp + 0x20], rax
140957ccc: lea rcx, [rip + 0x215c186]
140957cd3: mov edx, 0x4b
140957cd8: mov r9d, 0x4b
140957cde: call 0x1428d9940
140957ce3: jne 0x140957cc0
140957ce5: mov r14d, 0x49
140957ceb: sub r14, rdx
140957cee: lea r15, [rip + 0x215c164]
140957cf5: add r15, r8
140957cf8: lea rax, [rip + 0x215cbf8]
140957cff: lea r12, [rip + 0x215cbc2]
140957d06: nop word ptr cs:[rax + rax]
140957d10: movsx ecx, byte ptr [rax - 1]
140957d14: test ecx, ecx
140957d16: js 0x140957d30
140957d18: dec rax
140957d1b: cmp ecx, 0x5c
140957d1e: jne 0x140957d89
140957d20: jmp 0x140957d9f
140957d22: nop word ptr cs:[rax + rax]
140957d30: movzx edx, byte ptr [rax - 2]
140957d34: cmp dl, 0xc0
140957d37: jge 0x140957d5e
140957d39: movzx r8d, byte ptr [rax - 3]
140957d3e: cmp r8b, 0xc0
140957d42: jge 0x140957d67
140957d44: movzx r9d, byte ptr [rax - 4]
140957d49: add rax, -4
140957d4d: and r9d, 7
140957d51: shl r9d, 6
140957d55: and r8d, 0x3f
140957d59: or r8d, r9d
140957d5c: jmp 0x140957d6f
140957d5e: add rax, -2
140957d62: and edx, 0x1f
140957d65: jmp 0x140957d79
140957d67: add rax, -3
140957d6b: and r8d, 0xf
140957d6f: shl r8d, 6
140957d73: and edx, 0x3f
140957d76: or edx, r8d
140957d79: shl edx, 6
140957d7c: and cl, 0x3f
140957d7f: movzx ecx, cl
140957d82: or ecx, edx
140957d84: cmp ecx, 0x5c
140957d87: je 0x140957d9f
140957d89: cmp ecx, 0x2f
140957d8c: je 0x140957d9f
140957d8e: cmp rax, r12
140957d91: jne 0x140957d10
140957d97: mov r13d, 0x2f
140957d9d: jmp 0x140957def
140957d9f: lea r12, [rip + 0x215cb22]
140957da6: sub rax, r12
140957da9: mov r8, rax
140957dac: inc r8
140957daf: je 0x140957de3
140957db1: cmp r8, 0x2f
140957db5: jae 0x140957de1
140957db7: cmp byte ptr [r8 + r12], 0xbf
140957dbc: jg 0x140957de3
140957dbe: lea rax, [rip + 0x215c143]
140957dc5: mov qword ptr [rsp + 0x20], rax
140957dca: lea rcx, [rip + 0x215caf7]
140957dd1: mov edx, 0x2f
140957dd6: mov r9d, 0x2f
140957ddc: call 0x1428d9940
140957de1: jne 0x140957dbe
140957de3: mov r13d, 0x2e
140957de9: sub r13, rax
140957dec: add r12, r8
140957def: call 0x140c18130
140957df4: mov rcx, rax
140957df7: mov rax, qword ptr [rax]
140957dfa: cmp rax, 1
140957dfe: jne 0x140957f0d
140957e04: add rcx, 8
140957e08: cmp qword ptr [rcx], 0
140957e0c: jne 0x140957f47
140957e12: mov qword ptr [rcx], 0xffffffffffffffff
140957e19: mov qword ptr [rbp - 0x10], rcx
140957e1d: add rcx, 8
140957e21: mov qword ptr [rsp + 0x20], r13
140957e26: mov qword ptr [rsp + 0x30], 0
140957e2f: mov qword ptr [rsp + 0x28], 1
140957e38: mov rdx, r15
140957e3b: mov r8, r14
140957e3e: mov r9, r12
140957e41: call 0x140c16d40
140957e46: mov r14, rax
140957e49: mov rax, qword ptr [rbp - 0x10]
140957e4d: inc qword ptr [rax]
140957e50: mov qword ptr [rbp - 0x28], r14
140957e54: mov eax, 1
140957e59: mov qword ptr [rbp - 0x30], rax
140957e5d: mov rax, qword ptr [rbx]
140957e60: mov rcx, qword ptr [rbx + 8]
140957e64: mov rdx, qword ptr [rax]
140957e67: xor r8d, r8d
140957e6a: mov r9, rdx
140957e6d: sub r9, qword ptr [rcx + 0x18]
140957e71: cmovb r9, r8
140957e75: sub rdx, qword ptr [rcx + 0x38]
140957e79: cmovb rdx, r8
140957e7d: mov r10, qword ptr [rcx + 0x30]
140957e81: sub r10, rdx
140957e84: cmovb r10, r8
140957e88: seta dl
140957e8b: mov r11, qword ptr [rcx + 0x10]
140957e8f: sub r11, r9
140957e92: cmovb r11, r8
140957e96: mov rcx, qword ptr [rcx + 0x40]
140957e9a: seta r8b
140957e9e: add r11, r10
140957ea1: mov r9, rcx
140957ea4: sub r9, r11
140957ea7: mov qword ptr [rax], r9
140957eaa: or r8b, dl
140957ead: je 0x140957ebd
140957eaf: mov qword ptr [rax], rcx
140957eb2: mov rdx, qword ptr [rdi]
140957eb5: mov rcx, rsi
140957eb8: call 0x14083fa60
140957ebd: cmp byte ptr [rbp - 8], 0
140957ec1: je 0x140957efc
140957ec3: call 0x140c18130
140957ec8: mov rcx, rax
140957ecb: mov rax, qword ptr [rax]
140957ece: cmp rax, 1
140957ed2: jne 0x140957f27
140957ed4: add rcx, 8
140957ed8: cmp qword ptr [rcx], 0
140957edc: jne 0x140957f47
140957ede: mov qword ptr [rcx], 0xffffffffffffffff
140957ee5: mov qword ptr [rbp - 8], rcx
140957ee9: add rcx, 8
140957eed: mov rdx, r14
140957ef0: call 0x140c170f0
140957ef5: mov rax, qword ptr [rbp - 8]
140957ef9: inc qword ptr [rax]
140957efc: add rsp, 0x78
140957f00: pop rbx
140957f01: pop rdi
140957f02: pop rsi
140957f03: pop r12
140957f05: pop r13
140957f07: pop r14
140957f09: pop r15
140957f0b: pop rbp
140957f0c: ret 
140957f0d: test rax, rax
140957f10: jne 0x140957f3b
140957f12: xor edx, edx
140957f14: call 0x1428f87d0
140957f19: mov rcx, rax
140957f1c: test rax, rax
140957f1f: jne 0x140957e08
140957f25: jmp 0x140957f3b
140957f27: test rax, rax
140957f2a: jne 0x140957f3b
140957f2c: xor edx, edx
140957f2e: call 0x1428f87d0
140957f33: mov rcx, rax
140957f36: test rax, rax
140957f39: jne 0x140957ed8
140957f3b: lea rcx, [rip + 0x22e2b5e]
140957f42: call 0x1428d80d0
140957f47: lea rcx, [rip + 0x22e2c22]
140957f4e: call 0x1428d9250
140957f53: int3 