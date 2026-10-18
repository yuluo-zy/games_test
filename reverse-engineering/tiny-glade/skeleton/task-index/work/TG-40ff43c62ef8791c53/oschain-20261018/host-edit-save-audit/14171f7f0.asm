14171f7f0: push rbp
14171f7f1: push r15
14171f7f3: push r14
14171f7f5: push rsi
14171f7f6: push rdi
14171f7f7: push rbx
14171f7f8: sub rsp, 0x88
14171f7ff: lea rbp, [rsp + 0x80]
14171f807: mov qword ptr [rbp], 0xfffffffffffffffe
14171f80f: mov rsi, rcx
14171f812: call 0x140c16a00
14171f817: test al, al
14171f819: je 0x14171f87a
14171f81b: lea rbx, [rip + 0x15aaaed]
14171f822: mov edi, 0x52
14171f827: mov edx, 0x52
14171f82c: mov rcx, rbx
14171f82f: call 0x14171e730
14171f834: cmp rax, 1
14171f838: jne 0x14171f8ef
14171f83e: mov r9, rdx
14171f841: test rdx, rdx
14171f844: je 0x14171f883
14171f846: cmp r9, 0x52
14171f84a: jae 0x14171f881
14171f84c: lea rax, [rip + 0x15aaabc]
14171f853: cmp byte ptr [r9 + rax], 0xbf
14171f858: jg 0x14171f883
14171f85a: lea rax, [rip + 0x15aabcf]
14171f861: mov qword ptr [rsp + 0x20], rax
14171f866: lea rcx, [rip + 0x15aaaa2]
14171f86d: mov edx, 0x52
14171f872: xor r8d, r8d
14171f875: call 0x1428d9940
14171f87a: xor eax, eax
14171f87c: jmp 0x14171fa3a
14171f881: jne 0x14171f85a
14171f883: lea rbx, [rip + 0x15aaa85]
14171f88a: mov rcx, rbx
14171f88d: mov rdx, r9
14171f890: call 0x14171e730
14171f895: cmp rax, 1
14171f899: jne 0x14171f8ef
14171f89b: mov r8, rdx
14171f89e: add r8, 2
14171f8a2: je 0x14171f8dd
14171f8a4: cmp r8, 0x52
14171f8a8: jae 0x14171f8db
14171f8aa: lea rax, [rip + 0x15aaa5e]
14171f8b1: cmp byte ptr [r8 + rax], 0xbf
14171f8b6: jg 0x14171f8dd
14171f8b8: lea rax, [rip + 0x15aab89]
14171f8bf: mov qword ptr [rsp + 0x20], rax
14171f8c4: lea rcx, [rip + 0x15aaa44]
14171f8cb: mov edx, 0x52
14171f8d0: mov r9d, 0x52
14171f8d6: call 0x1428d9940
14171f8db: jne 0x14171f8b8
14171f8dd: mov edi, 0x50
14171f8e2: sub rdi, rdx
14171f8e5: lea rbx, [rip + 0x15aaa23]
14171f8ec: add rbx, r8
14171f8ef: lea r8, [rip + 0x15aabb5]
14171f8f6: lea r14, [rip + 0x15aab7b]
14171f8fd: nop dword ptr [rax]
14171f900: movsx eax, byte ptr [r8 - 1]
14171f905: test eax, eax
14171f907: js 0x14171f920
14171f909: dec r8
14171f90c: cmp eax, 0x5c
14171f90f: jne 0x14171f974
14171f911: jmp 0x14171f986
14171f913: nop word ptr cs:[rax + rax]
14171f920: movzx ecx, byte ptr [r8 - 2]
14171f925: cmp cl, 0xc0
14171f928: jge 0x14171f94d
14171f92a: movzx edx, byte ptr [r8 - 3]
14171f92f: cmp dl, 0xc0
14171f932: jge 0x14171f956
14171f934: movzx r9d, byte ptr [r8 - 4]
14171f939: add r8, -4
14171f93d: and r9d, 7
14171f941: shl r9d, 6
14171f945: and edx, 0x3f
14171f948: or edx, r9d
14171f94b: jmp 0x14171f95d
14171f94d: add r8, -2
14171f951: and ecx, 0x1f
14171f954: jmp 0x14171f965
14171f956: add r8, -3
14171f95a: and edx, 0xf
14171f95d: shl edx, 6
14171f960: and ecx, 0x3f
14171f963: or ecx, edx
14171f965: shl ecx, 6
14171f968: and al, 0x3f
14171f96a: movzx eax, al
14171f96d: or eax, ecx
14171f96f: cmp eax, 0x5c
14171f972: je 0x14171f986
14171f974: cmp eax, 0x2f
14171f977: je 0x14171f986
14171f979: cmp r8, r14
14171f97c: jne 0x14171f900
14171f97e: mov r15d, 0x33
14171f984: jmp 0x14171f9d3
14171f986: lea r14, [rip + 0x15aaaeb]
14171f98d: sub r8, r14
14171f990: inc r8
14171f993: je 0x14171f9c7
14171f995: cmp r8, 0x33
14171f999: jae 0x14171f9c5
14171f99b: cmp byte ptr [r8 + r14], 0xbf
14171f9a0: jg 0x14171f9c7
14171f9a2: lea rax, [rip + 0x15aaa67]
14171f9a9: mov qword ptr [rsp + 0x20], rax
14171f9ae: lea rcx, [rip + 0x15aaac3]
14171f9b5: mov edx, 0x33
14171f9ba: mov r9d, 0x33
14171f9c0: call 0x1428d9940
14171f9c5: jne 0x14171f9a2
14171f9c7: mov r15d, 0x33
14171f9cd: sub r15, r8
14171f9d0: add r14, r8
14171f9d3: call 0x140c18130
14171f9d8: mov rcx, rax
14171f9db: mov rax, qword ptr [rax]
14171f9de: cmp rax, 1
14171f9e2: jne 0x14171fd70
14171f9e8: add rcx, 8
14171f9ec: cmp qword ptr [rcx], 0
14171f9f0: jne 0x14171fd94
14171f9f6: mov qword ptr [rcx], 0xffffffffffffffff
14171f9fd: mov qword ptr [rbp - 8], rcx
14171fa01: add rcx, 8
14171fa05: mov qword ptr [rsp + 0x20], r15
14171fa0a: mov qword ptr [rsp + 0x30], 0
14171fa13: mov qword ptr [rsp + 0x28], 1
14171fa1c: mov rdx, rbx
14171fa1f: mov r8, rdi
14171fa22: mov r9, r14
14171fa25: call 0x140c16d40
14171fa2a: mov rcx, qword ptr [rbp - 8]
14171fa2e: inc qword ptr [rcx]
14171fa31: mov qword ptr [rbp - 0x28], rax
14171fa35: mov eax, 1
14171fa3a: mov qword ptr [rbp - 0x30], rax
14171fa3e: lea rdi, [rsi + 0xd8]
14171fa45: xor r15d, r15d
14171fa48: lea rbx, [rip + 0x15ab949]
14171fa4f: movabs r14, 0x8000000000000000
14171fa59: nop dword ptr [rax]
14171fa60: movzx eax, r15b
14171fa64: movsxd rcx, dword ptr [rbx + rax*4]
14171fa68: add rcx, rbx
14171fa6b: mov eax, r15d
14171fa6e: mov edx, r15d
14171fa71: mov r15b, 2
14171fa74: jmp rcx
14171fa76: mov r15b, 1
14171fa79: nop dword ptr [rax]
14171fa80: mov edx, eax
14171fa82: movzx eax, dl
14171fa85: lea rcx, [rax + rax*4]
14171fa89: lea rax, [rdi + rcx*8]
14171fa8d: mov r8, qword ptr [rdi + rcx*8]
14171fa91: mov qword ptr [rdi + rcx*8], r14
14171fa95: mov rcx, r8
14171fa98: neg rcx
14171fa9b: jo 0x14171fa60
14171fa9d: mov qword ptr [rbp - 8], r8
14171faa1: mov r8, qword ptr [rax + 8]
14171faa5: mov r9, qword ptr [rax + 0x10]
14171faa9: mov rcx, rsi
14171faac: mov qword ptr [rbp - 0x10], r8
14171fab0: call 0x14171ece0
14171fab5: mov rdx, qword ptr [rbp - 8]
14171fab9: test rdx, rdx
14171fabc: je 0x14171fa60
14171fabe: mov r8d, 1
14171fac4: mov rcx, qword ptr [rbp - 0x10]
14171fac8: call 0x140613c20
14171facd: movzx eax, r15b
14171fad1: movsxd rcx, dword ptr [rbx + rax*4]
14171fad5: add rcx, rbx
14171fad8: mov edx, eax
14171fada: mov r15b, 2
14171fadd: jmp rcx
14171fadf: mov r15b, 3
14171fae2: jmp 0x14171fa80
14171fae4: mov r15b, 7
14171fae7: jmp 0x14171fa80
14171fae9: mov r15b, 5
14171faec: jmp 0x14171fa80
14171faee: mov r15b, 6
14171faf1: jmp 0x14171fa80
14171faf3: mov r15b, 0xa
14171faf6: jmp 0x14171fa80
14171faf8: mov r15b, 4
14171fafb: jmp 0x14171fa80
14171fafd: mov r15b, 8
14171fb00: jmp 0x14171fa80
14171fb05: mov r15b, 9
14171fb08: jmp 0x14171fa80
14171fb0d: call 0x140c16a00
14171fb12: test al, al
14171fb14: mov byte ptr [rbp - 8], al
14171fb17: je 0x14171fbb5
14171fb1d: lea r8, [rip + 0x15aa987]
14171fb24: lea rdi, [rip + 0x15aa94d]
14171fb2b: nop dword ptr [rax + rax]
14171fb30: movsx eax, byte ptr [r8 - 1]
14171fb35: test eax, eax
14171fb37: js 0x14171fb50
14171fb39: dec r8
14171fb3c: cmp eax, 0x5c
14171fb3f: jne 0x14171fba4
14171fb41: jmp 0x14171fbbc
14171fb43: nop word ptr cs:[rax + rax]
14171fb50: movzx ecx, byte ptr [r8 - 2]
14171fb55: cmp cl, 0xc0
14171fb58: jge 0x14171fb7d
14171fb5a: movzx edx, byte ptr [r8 - 3]
14171fb5f: cmp dl, 0xc0
14171fb62: jge 0x14171fb86
14171fb64: movzx r9d, byte ptr [r8 - 4]
14171fb69: add r8, -4
14171fb6d: and r9d, 7
14171fb71: shl r9d, 6
14171fb75: and edx, 0x3f
14171fb78: or edx, r9d
14171fb7b: jmp 0x14171fb8d
14171fb7d: add r8, -2
14171fb81: and ecx, 0x1f
14171fb84: jmp 0x14171fb95
14171fb86: add r8, -3
14171fb8a: and edx, 0xf
14171fb8d: shl edx, 6
14171fb90: and ecx, 0x3f
14171fb93: or ecx, edx
14171fb95: shl ecx, 6
14171fb98: and al, 0x3f
14171fb9a: movzx eax, al
14171fb9d: or eax, ecx
14171fb9f: cmp eax, 0x5c
14171fba2: je 0x14171fbbc
14171fba4: cmp eax, 0x2f
14171fba7: je 0x14171fbbc
14171fba9: cmp r8, rdi
14171fbac: jne 0x14171fb30
14171fbae: mov ebx, 0x33
14171fbb3: jmp 0x14171fc0d
14171fbb5: xor eax, eax
14171fbb7: jmp 0x14171fc94
14171fbbc: lea rdi, [rip + 0x15aa8b5]
14171fbc3: sub r8, rdi
14171fbc6: inc r8
14171fbc9: je 0x14171fc02
14171fbcb: cmp r8, 0x33
14171fbcf: jae 0x14171fc00
14171fbd1: cmp byte ptr [r8 + rdi], 0xbf
14171fbd6: jg 0x14171fc02
14171fbd8: lea rax, [rip + 0x15aa831]
14171fbdf: mov qword ptr [rsp + 0x20], rax
14171fbe4: lea rcx, [rip + 0x15aa88d]
14171fbeb: mov edx, 0x33
14171fbf0: mov r9d, 0x33
14171fbf6: call 0x1428d9940
14171fbfb: jmp 0x14171fdd6
14171fc00: jne 0x14171fbd8
14171fc02: mov ebx, 0x33
14171fc07: sub rbx, r8
14171fc0a: add rdi, r8
14171fc0d: call 0x140c18130
14171fc12: mov rcx, rax
14171fc15: mov rax, qword ptr [rax]
14171fc18: test rax, rax
14171fc1b: je 0x14171fc2c
14171fc1d: cmp eax, 1
14171fc20: jne 0x14171fdbc
14171fc26: add rcx, 8
14171fc2a: jmp 0x14171fc3f
14171fc2c: xor edx, edx
14171fc2e: call 0x1428f87d0
14171fc33: mov rcx, rax
14171fc36: test rax, rax
14171fc39: je 0x14171fdbc
14171fc3f: cmp qword ptr [rcx], 0
14171fc43: jne 0x14171fda0
14171fc49: mov qword ptr [rcx], 0xffffffffffffffff
14171fc50: mov qword ptr [rbp - 0x10], rcx
14171fc54: add rcx, 8
14171fc58: mov qword ptr [rsp + 0x20], rbx
14171fc5d: mov qword ptr [rsp + 0x30], 0
14171fc66: mov qword ptr [rsp + 0x28], 1
14171fc6f: lea rdx, [rip + 0x15aaa2a]
14171fc76: mov r8d, 8
14171fc7c: mov r9, rdi
14171fc7f: call 0x140c16d40
14171fc84: mov rcx, qword ptr [rbp - 0x10]
14171fc88: inc qword ptr [rcx]
14171fc8b: mov qword ptr [rbp - 0x38], rax
14171fc8f: mov eax, 1
14171fc94: mov qword ptr [rbp - 0x40], rax
14171fc98: add rsi, 0x10
14171fc9c: mov rcx, rsi
14171fc9f: call 0x14091a230
14171fca4: mov ebx, eax
14171fca6: cmp byte ptr [rbp - 8], 0
14171fcaa: je 0x14171fd07
14171fcac: call 0x140c18130
14171fcb1: mov rcx, rax
14171fcb4: mov rax, qword ptr [rax]
14171fcb7: test rax, rax
14171fcba: je 0x14171fccb
14171fcbc: cmp eax, 1
14171fcbf: jne 0x14171fdca
14171fcc5: add rcx, 8
14171fcc9: jmp 0x14171fcde
14171fccb: xor edx, edx
14171fccd: call 0x1428f87d0
14171fcd2: mov rcx, rax
14171fcd5: test rax, rax
14171fcd8: je 0x14171fdca
14171fcde: cmp qword ptr [rcx], 0
14171fce2: jne 0x14171fdae
14171fce8: mov qword ptr [rcx], 0xffffffffffffffff
14171fcef: mov qword ptr [rbp - 8], rcx
14171fcf3: add rcx, 8
14171fcf7: mov rdx, qword ptr [rbp - 0x38]
14171fcfb: call 0x140c170f0
14171fd00: mov rax, qword ptr [rbp - 8]
14171fd04: inc qword ptr [rax]
14171fd07: cmp qword ptr [rbp - 0x30], 0
14171fd0c: je 0x14171fd48
14171fd0e: call 0x140c18130
14171fd13: mov rcx, rax
14171fd16: mov rax, qword ptr [rax]
14171fd19: cmp rax, 1
14171fd1d: jne 0x14171fd5a
14171fd1f: add rcx, 8
14171fd23: cmp qword ptr [rcx], 0
14171fd27: jne 0x14171fd94
14171fd29: mov qword ptr [rcx], 0xffffffffffffffff
14171fd30: mov qword ptr [rbp - 8], rcx
14171fd34: add rcx, 8
14171fd38: mov rdx, qword ptr [rbp - 0x28]
14171fd3c: call 0x140c170f0
14171fd41: mov rax, qword ptr [rbp - 8]
14171fd45: inc qword ptr [rax]
14171fd48: mov eax, ebx
14171fd4a: add rsp, 0x88
14171fd51: pop rbx
14171fd52: pop rdi
14171fd53: pop rsi
14171fd54: pop r14
14171fd56: pop r15
14171fd58: pop rbp
14171fd59: ret 
14171fd5a: test rax, rax
14171fd5d: jne 0x14171fd88
14171fd5f: xor edx, edx
14171fd61: call 0x1428f87d0
14171fd66: mov rcx, rax
14171fd69: test rax, rax
14171fd6c: jne 0x14171fd23
14171fd6e: jmp 0x14171fd88
14171fd70: test rax, rax
14171fd73: jne 0x14171fd88
14171fd75: xor edx, edx
14171fd77: call 0x1428f87d0
14171fd7c: mov rcx, rax
14171fd7f: test rax, rax
14171fd82: jne 0x14171f9ec
14171fd88: lea rcx, [rip + 0x151ad11]
14171fd8f: call 0x1428d80d0
14171fd94: lea rcx, [rip + 0x151add5]
14171fd9b: call 0x1428d9250
14171fda0: lea rcx, [rip + 0x151adc9]
14171fda7: call 0x1428d9250
14171fdac: jmp 0x14171fdd6
14171fdae: lea rcx, [rip + 0x151adbb]
14171fdb5: call 0x1428d9250
14171fdba: jmp 0x14171fdd6
14171fdbc: lea rcx, [rip + 0x151acdd]
14171fdc3: call 0x1428d80d0
14171fdc8: jmp 0x14171fdd6
14171fdca: lea rcx, [rip + 0x151accf]
14171fdd1: call 0x1428d80d0
14171fdd6: ud2 
14171fdd8: nop dword ptr [rax + rax]
14171fde0: mov qword ptr [rsp + 0x10], rdx
14171fde5: push rbp
14171fde6: push r15
14171fde8: push r14
14171fdea: push rsi
14171fdeb: push rdi
14171fdec: push rbx
14171fded: sub rsp, 0x38
14171fdf1: lea rbp, [rdx + 0x80]
14171fdf8: mov rax, qword ptr [rbp - 8]
14171fdfc: inc qword ptr [rax]
14171fdff: add rsp, 0x38
14171fe03: pop rbx
14171fe04: pop rdi
14171fe05: pop rsi
14171fe06: pop r14
14171fe08: pop r15
14171fe0a: pop rbp
14171fe0b: ret 
14171fe0c: nop dword ptr [rax]
14171fe10: mov qword ptr [rsp + 0x10], rdx
14171fe15: push rbp
14171fe16: push r15
14171fe18: push r14
14171fe1a: push rsi
14171fe1b: push rdi
14171fe1c: push rbx
14171fe1d: sub rsp, 0x38
14171fe21: lea rbp, [rdx + 0x80]
14171fe28: cmp qword ptr [rbp - 0x30], 0
14171fe2d: je 0x14171fe4f
14171fe2f: lea rax, [rbp - 0x28]
14171fe33: mov qword ptr [rbp - 0x18], rax
14171fe37: lea rax, [rbp - 0x18]
14171fe3b: mov qword ptr [rbp - 0x20], rax
14171fe3f: lea rcx, [rip + 0x15aa56a]
14171fe46: lea rdx, [rbp - 0x20]
14171fe4a: call 0x14132f2d0
14171fe4f: nop 
14171fe50: add rsp, 0x38
14171fe54: pop rbx
14171fe55: pop rdi
14171fe56: pop rsi
14171fe57: pop r14
14171fe59: pop r15
14171fe5b: pop rbp
14171fe5c: ret 
14171fe5d: nop dword ptr [rax]
14171fe60: mov qword ptr [rsp + 0x10], rdx
14171fe65: push rbp
14171fe66: push r15
14171fe68: push r14
14171fe6a: push rsi
14171fe6b: push rdi
14171fe6c: push rbx
14171fe6d: sub rsp, 0x38
14171fe71: lea rbp, [rdx + 0x80]
14171fe78: mov rax, qword ptr [rbp - 0x10]
14171fe7c: inc qword ptr [rax]
14171fe7f: add rsp, 0x38
14171fe83: pop rbx
14171fe84: pop rdi
14171fe85: pop rsi
14171fe86: pop r14
14171fe88: pop r15
14171fe8a: pop rbp
14171fe8b: ret 
14171fe8c: nop dword ptr [rax]
14171fe90: mov qword ptr [rsp + 0x10], rdx
14171fe95: push rbp
14171fe96: push r15
14171fe98: push r14
14171fe9a: push rsi
14171fe9b: push rdi
14171fe9c: push rbx
14171fe9d: sub rsp, 0x38
14171fea1: lea rbp, [rdx + 0x80]
14171fea8: cmp byte ptr [rbp - 8], 0
14171feac: je 0x14171fece
14171feae: lea rax, [rbp - 0x38]
14171feb2: mov qword ptr [rbp - 0x18], rax
14171feb6: lea rax, [rbp - 0x18]
14171feba: mov qword ptr [rbp - 0x20], rax
14171febe: lea rcx, [rip + 0x15aa4eb]
14171fec5: lea rdx, [rbp - 0x20]
14171fec9: call 0x14132f2d0
14171fece: nop 
14171fecf: add rsp, 0x38
14171fed3: pop rbx
14171fed4: pop rdi
14171fed5: pop rsi
14171fed6: pop r14
14171fed8: pop r15
14171feda: pop rbp
14171fedb: ret 
14171fedc: nop dword ptr [rax]
14171fee0: mov qword ptr [rsp + 0x10], rdx
14171fee5: push rbp
14171fee6: push r15
14171fee8: push r14
14171feea: push rsi
14171feeb: push rdi
14171feec: push rbx
14171feed: sub rsp, 0x38
14171fef1: lea rbp, [rdx + 0x80]
14171fef8: mov rax, qword ptr [rbp - 8]
14171fefc: inc qword ptr [rax]
14171feff: add rsp, 0x38
14171ff03: pop rbx
14171ff04: pop rdi
14171ff05: pop rsi
14171ff06: pop r14
14171ff08: pop r15
14171ff0a: pop rbp
14171ff0b: ret 
14171ff0c: nop dword ptr [rax]
14171ff10: mov qword ptr [rsp + 0x10], rdx
14171ff15: push rbp
14171ff16: push r15
14171ff18: push r14
14171ff1a: push rsi
14171ff1b: push rdi
14171ff1c: push rbx
14171ff1d: sub rsp, 0x38
14171ff21: lea rbp, [rdx + 0x80]
14171ff28: mov rax, qword ptr [rbp - 8]
14171ff2c: inc qword ptr [rax]
14171ff2f: add rsp, 0x38
14171ff33: pop rbx
14171ff34: pop rdi
14171ff35: pop rsi
14171ff36: pop r14
14171ff38: pop r15
14171ff3a: pop rbp
14171ff3b: ret 
14171ff3c: nop dword ptr [rax]
14171ff40: mov qword ptr [rsp + 0x10], rdx
14171ff45: push rbp
14171ff46: push r15
14171ff48: push r14
14171ff4a: push rsi
14171ff4b: push rdi
14171ff4c: push rbx
14171ff4d: sub rsp, 0x38
14171ff51: lea rbp, [rdx + 0x80]
14171ff58: mov rdx, qword ptr [rbp - 8]
14171ff5c: test rdx, rdx
14171ff5f: je 0x14171ff70
14171ff61: mov r8d, 1
14171ff67: mov rcx, qword ptr [rbp - 0x10]
14171ff6b: call 0x140613c20
14171ff70: nop 
14171ff71: add rsp, 0x38
14171ff75: pop rbx
14171ff76: pop rdi
14171ff77: pop rsi
14171ff78: pop r14
14171ff7a: pop r15
14171ff7c: pop rbp
14171ff7d: ret 
