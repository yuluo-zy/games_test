14161ddb0: push rbp
14161ddb1: push r15
14161ddb3: push r14
14161ddb5: push r12
14161ddb7: push rsi
14161ddb8: push rdi
14161ddb9: push rbx
14161ddba: sub rsp, 0x130
14161ddc1: lea rbp, [rsp + 0x80]
14161ddc9: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
14161ddd4: mov rdi, rdx
14161ddd7: mov rsi, rcx
14161ddda: test r8, r8
14161dddd: je 0x14161de16
14161dddf: mov rbx, r9
14161dde2: mov rax, r9
14161dde5: and rax, 3
14161dde9: mov qword ptr [rbp - 0x40], rax
14161dded: jne 0x14161dff1
14161ddf3: mov r15, r8
14161ddf6: mov rax, r8
14161ddf9: and rax, 3
14161ddfd: mov qword ptr [rbp - 0x40], rax
14161de01: jne 0x14161e002
14161de07: cmp rbx, 8
14161de0b: jae 0x14161e09d
14161de11: xor r12d, r12d
14161de14: jmp 0x14161de46
14161de16: movzx eax, byte ptr [rip + 0x2016dec]
14161de1d: mov ebx, 4
14161de22: mov ecx, 4
14161de27: mov edx, 4
14161de2c: call 0x140613c10
14161de31: test rax, rax
14161de34: je 0x14161e107
14161de3a: mov r15, rax
14161de3d: mov dword ptr [rax], 0
14161de43: mov r12b, 1
14161de46: mov qword ptr [rbp + 0x98], r15
14161de4d: mov byte ptr [rbp + 0xa7], r12b
14161de54: lea rcx, [rbp + 0x10]
14161de58: mov edx, 4
14161de5d: mov r8d, 0x22
14161de63: mov r9b, 4
14161de66: call 0x140bd4de0
14161de6b: mov qword ptr [rbp + 0x98], r15
14161de72: mov byte ptr [rbp + 0xa7], r12b
14161de79: lea r14, [rbp + 0x60]
14161de7d: lea rdx, [rbp + 0x10]
14161de81: mov r8d, 4
14161de87: mov rcx, r14
14161de8a: call 0x140bd4f20
14161de8f: mov qword ptr [rbp + 0x98], r15
14161de96: mov byte ptr [rbp + 0xa7], r12b
14161de9d: mov qword ptr [rsp + 0x30], rbx
14161dea2: mov qword ptr [rsp + 0x28], r15
14161dea7: mov qword ptr [rsp + 0x20], r14
14161deac: lea r8, [rip + 0x16619ed]
14161deb3: lea rcx, [rbp - 0x40]
14161deb7: mov r9d, 0xe
14161debd: mov rdx, rdi
14161dec0: call 0x1416a2dc0
14161dec5: cmp dword ptr [rbp - 0x40], 0xb
14161dec9: jne 0x14161e02c
14161decf: mov rax, qword ptr [rbp - 0x18]
14161ded3: mov qword ptr [rbp + 0x80], rax
14161deda: movups xmm0, xmmword ptr [rbp - 0x38]
14161dede: movups xmm1, xmmword ptr [rbp - 0x28]
14161dee2: movaps xmmword ptr [rbp + 0x70], xmm1
14161dee6: movaps xmmword ptr [rbp + 0x60], xmm0
14161deea: test r12b, r12b
14161deed: je 0x14161df02
14161deef: mov edx, 4
14161def4: mov r8d, 4
14161defa: mov rcx, r15
14161defd: call 0x140613c20
14161df02: lea rcx, [rbp + 0x60]
14161df06: mov rdx, rdi
14161df09: call 0x140bd4cd0
14161df0e: test rax, rax
14161df11: je 0x14161e08c
14161df17: cmp rdx, qword ptr [rbp + 0x78]
14161df1b: jb 0x14161df76
14161df1d: mov rcx, rdx
14161df20: and rcx, 3
14161df24: mov qword ptr [rbp - 0x40], rcx
14161df28: jne 0x14161df93
14161df2a: mov rcx, rax
14161df2d: and rcx, 3
14161df31: mov qword ptr [rbp - 0x40], rcx
14161df35: jne 0x14161dfc2
14161df3b: mov rcx, qword ptr [rbp + 0x80]
14161df42: mov qword ptr [rsi + 0x30], rcx
14161df46: movaps xmm0, xmmword ptr [rbp + 0x60]
14161df4a: movaps xmm1, xmmword ptr [rbp + 0x70]
14161df4e: movups xmmword ptr [rsi + 0x20], xmm1
14161df52: movups xmmword ptr [rsi + 0x10], xmm0
14161df56: shr rdx, 2
14161df5a: mov qword ptr [rsi], rax
14161df5d: mov qword ptr [rsi + 8], rdx
14161df61: mov rax, rsi
14161df64: add rsp, 0x130
14161df6b: pop rbx
14161df6c: pop rdi
14161df6d: pop rsi
14161df6e: pop r12
14161df70: pop r14
14161df72: pop r15
14161df74: pop rbp
14161df75: ret 
14161df76: lea rcx, [rip + 0x1662273]
14161df7d: lea r8, [rip + 0x16622a4]
14161df84: mov edx, 0x37
14161df89: call 0x1428d9430
14161df8e: jmp 0x14161e124
14161df93: mov qword ptr [rbp + 0x10], 0
14161df9b: lea rax, [rip + 0x16622a6]
14161dfa2: mov qword ptr [rsp + 0x20], rax
14161dfa7: lea r8, [rip + 0x1662292]
14161dfae: lea rdx, [rbp - 0x40]
14161dfb2: lea r9, [rbp + 0x10]
14161dfb6: xor ecx, ecx
14161dfb8: call 0x1428d9596
14161dfbd: jmp 0x14161e124
14161dfc2: mov qword ptr [rbp + 0x10], 0
14161dfca: lea rax, [rip + 0x166228f]
14161dfd1: mov qword ptr [rsp + 0x20], rax
14161dfd6: lea r8, [rip + 0x1662263]
14161dfdd: lea rdx, [rbp - 0x40]
14161dfe1: lea r9, [rbp + 0x10]
14161dfe5: xor ecx, ecx
14161dfe7: call 0x1428d9596
14161dfec: jmp 0x14161e124
14161dff1: mov qword ptr [rbp + 0x10], 0
14161dff9: lea rax, [rip + 0x16abf48]
14161e000: jmp 0x14161e011
14161e002: mov qword ptr [rbp + 0x10], 0
14161e00a: lea rax, [rip + 0x16abf4f]
14161e011: mov qword ptr [rsp + 0x20], rax
14161e016: lea r8, [rip + 0x16abf23]
14161e01d: lea rdx, [rbp - 0x40]
14161e021: lea r9, [rbp + 0x10]
14161e025: xor ecx, ecx
14161e027: call 0x1428d9596
14161e02c: mov dword ptr [rbp + 0x94], r12d
14161e033: mov qword ptr [rbp - 0x48], r15
14161e037: movups xmm0, xmmword ptr [rbp]
14161e03b: movaps xmmword ptr [rbp + 0x50], xmm0
14161e03f: movups xmm0, xmmword ptr [rbp - 0x40]
14161e043: movups xmm1, xmmword ptr [rbp - 0x30]
14161e047: movups xmm2, xmmword ptr [rbp - 0x20]
14161e04b: movups xmm3, xmmword ptr [rbp - 0x10]
14161e04f: movaps xmmword ptr [rbp + 0x40], xmm3
14161e053: movaps xmmword ptr [rbp + 0x30], xmm2
14161e057: movaps xmmword ptr [rbp + 0x20], xmm1
14161e05b: movaps xmmword ptr [rbp + 0x10], xmm0
14161e05f: lea rax, [rip + 0x1662142]
14161e066: mov qword ptr [rsp + 0x20], rax
14161e06b: lea rcx, [rip + 0x1660a76]
14161e072: lea r9, [rip + 0x1660a4f]
14161e079: lea r8, [rbp + 0x10]
14161e07d: mov edx, 0x2b
14161e082: call 0x1428d9760
14161e087: jmp 0x14161e124
14161e08c: lea rcx, [rip + 0x1662145]
14161e093: call 0x1428d9310
14161e098: jmp 0x14161e124
14161e09d: lea rax, [rip + 0x147ff23]
14161e0a4: mov qword ptr [rbp - 0x40], rax
14161e0a8: mov qword ptr [rbp - 0x38], 0x45
14161e0b0: lea rax, [rbp - 0x40]
14161e0b4: mov qword ptr [rbp + 0x60], rax
14161e0b8: lea rax, [rip - 0x151422f]
14161e0bf: mov qword ptr [rbp + 0x68], rax
14161e0c3: lea rax, [rip + 0x16616fe]
14161e0ca: mov qword ptr [rbp + 0x10], rax
14161e0ce: mov qword ptr [rbp + 0x18], 1
14161e0d6: mov qword ptr [rbp + 0x30], 0
14161e0de: lea rax, [rbp + 0x60]
14161e0e2: mov qword ptr [rbp + 0x20], rax
14161e0e6: mov qword ptr [rbp + 0x28], 1
14161e0ee: mov byte ptr [rbp + 0xa7], 0
14161e0f5: lea rdx, [rip + 0x16616dc]
14161e0fc: lea rcx, [rbp + 0x10]
14161e100: call 0x1428d9390
14161e105: jmp 0x14161e124
14161e107: mov byte ptr [rbp + 0xa7], 0
14161e10e: lea r8, [rip + 0x166099b]
14161e115: mov ecx, 4
14161e11a: mov edx, 4
14161e11f: call 0x1428d8fe3
14161e124: ud2 
14161e126: nop word ptr cs:[rax + rax]
14161e130: mov qword ptr [rsp + 0x10], rdx
14161e135: push rbp
14161e136: push r15
14161e138: push r14
14161e13a: push r12
14161e13c: push rsi
14161e13d: push rdi
14161e13e: push rbx
14161e13f: sub rsp, 0x40
14161e143: lea rbp, [rdx + 0x80]
14161e14a: test byte ptr [rbp + 0xa7], 1
14161e151: je 0x14161e16a
14161e153: mov rcx, qword ptr [rbp + 0x98]
14161e15a: mov edx, 4
14161e15f: mov r8d, 4
14161e165: call 0x140613c20
14161e16a: nop 
14161e16b: add rsp, 0x40
14161e16f: pop rbx
14161e170: pop rdi
14161e171: pop rsi
14161e172: pop r12
14161e174: pop r14
14161e176: pop r15
14161e178: pop rbp
14161e179: ret 
14161e17a: nop word ptr [rax + rax]
14161e180: mov qword ptr [rsp + 0x10], rdx
14161e185: push rbp
14161e186: push r15
14161e188: push r14
14161e18a: push r12
14161e18c: push rsi
14161e18d: push rdi
14161e18e: push rbx
14161e18f: sub rsp, 0x40
14161e193: lea rbp, [rdx + 0x80]
14161e19a: lea rcx, [rbp + 0x60]
14161e19e: call 0x140be24b0
14161e1a3: nop 
14161e1a4: add rsp, 0x40
14161e1a8: pop rbx
14161e1a9: pop rdi
14161e1aa: pop rsi
14161e1ab: pop r12
14161e1ad: pop r14
14161e1af: pop r15
14161e1b1: pop rbp
14161e1b2: ret 
14161e1b3: nop word ptr cs:[rax + rax]
14161e1c0: mov qword ptr [rsp + 0x10], rdx
14161e1c5: push rbp
14161e1c6: push r15
14161e1c8: push r14
14161e1ca: push r12
14161e1cc: push rsi
14161e1cd: push rdi
14161e1ce: push rbx
14161e1cf: sub rsp, 0x40
14161e1d3: lea rbp, [rdx + 0x80]
14161e1da: mov rsi, qword ptr [rbp - 0x48]
14161e1de: mov qword ptr [rbp + 0x98], rsi
14161e1e5: mov edi, dword ptr [rbp + 0x94]
14161e1eb: mov byte ptr [rbp + 0xa7], dil
14161e1f2: lea rcx, [rbp + 0x10]
14161e1f6: call 0x1407cea70
14161e1fb: mov qword ptr [rbp + 0x98], rsi
14161e202: mov byte ptr [rbp + 0xa7], dil
14161e209: add rsp, 0x40
14161e20d: pop rbx
14161e20e: pop rdi
14161e20f: pop rsi
14161e210: pop r12
14161e212: pop r14
14161e214: pop r15
14161e216: pop rbp
14161e217: ret 
