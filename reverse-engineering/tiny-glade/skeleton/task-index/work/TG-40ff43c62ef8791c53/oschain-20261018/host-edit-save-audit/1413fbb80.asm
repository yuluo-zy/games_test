1413fbb80: push rbp
1413fbb81: push rsi
1413fbb82: push rdi
1413fbb83: push rbx
1413fbb84: sub rsp, 0x408
1413fbb8b: lea rbp, [rsp + 0x80]
1413fbb93: mov qword ptr [rbp + 0x380], 0xfffffffffffffffe
1413fbb9e: mov rsi, rcx
1413fbba1: movzx eax, byte ptr [rip + 0x2239061]
1413fbba8: mov ecx, 0x150
1413fbbad: mov edx, 8
1413fbbb2: call 0x140613c10
1413fbbb7: test rax, rax
1413fbbba: je 0x1413fbe9e
1413fbbc0: mov qword ptr [rbp + 0x378], rax
1413fbbc7: xorps xmm0, xmm0
1413fbbca: movups xmmword ptr [rbp - 0x40], xmm0
1413fbbce: movups xmmword ptr [rbp - 0x28], xmm0
1413fbbd2: movups xmmword ptr [rbp - 0x10], xmm0
1413fbbd6: movups xmmword ptr [rbp + 8], xmm0
1413fbbda: movups xmmword ptr [rbp + 0x20], xmm0
1413fbbde: mov dword ptr [rbp + 0x30], 0
1413fbbe5: movups xmmword ptr [rbp + 0x58], xmm0
1413fbbe9: movups xmmword ptr [rbp + 0x70], xmm0
1413fbbed: movups xmmword ptr [rbp + 0x88], xmm0
1413fbbf4: movups xmmword ptr [rbp + 0xa0], xmm0
1413fbbfb: movups xmmword ptr [rbp + 0xb8], xmm0
1413fbc02: mov dword ptr [rbp + 0xc8], 0
1413fbc0c: mov qword ptr [rbp - 0x60], 0
1413fbc14: mov qword ptr [rbp - 0x58], 8
1413fbc1c: mov qword ptr [rbp - 0x50], 0
1413fbc24: mov qword ptr [rbp - 0x48], 0x10
1413fbc2c: mov qword ptr [rbp - 0x30], 0x10
1413fbc34: mov qword ptr [rbp - 0x18], 0x10
1413fbc3c: mov qword ptr [rbp], 0x10
1413fbc44: mov qword ptr [rbp + 0x18], 0x10
1413fbc4c: movabs rbx, 0x8000000000000000
1413fbc56: mov qword ptr [rbp + 0x38], rbx
1413fbc5a: lea rax, [rip + 0x185f143]
1413fbc61: mov qword ptr [rbp + 0x40], rax
1413fbc65: mov qword ptr [rbp + 0x48], 0x2e
1413fbc6d: mov qword ptr [rbp + 0x50], 0x10
1413fbc75: mov qword ptr [rbp + 0x68], 0x10
1413fbc7d: mov qword ptr [rbp + 0x80], 0x10
1413fbc88: mov qword ptr [rbp + 0x98], 0x10
1413fbc93: mov qword ptr [rbp + 0xb0], 0x10
1413fbc9e: mov dword ptr [rbp + 0xd0], 0
1413fbca8: mov word ptr [rbp + 0xd4], 1
1413fbcb1: mov qword ptr [rbp + 0x1d0], rbx
1413fbcb8: mov dword ptr [rbp + 0x290], 0
1413fbcc2: movzx eax, byte ptr [rip + 0x2238f40]
1413fbcc9: mov ecx, 0x2f8
1413fbcce: mov edx, 8
1413fbcd3: call 0x140613c10
1413fbcd8: test rax, rax
1413fbcdb: je 0x1413fbead
1413fbce1: mov rdi, rax
1413fbce4: lea rdx, [rbp - 0x60]
1413fbce8: mov r8d, 0x2f8
1413fbcee: mov rcx, rax
1413fbcf1: call 0x142923c00
1413fbcf6: lea r8, [rip + 0x185ee4b]
1413fbcfd: lea rcx, [rbp + 0x308]
1413fbd04: mov rdx, rdi
1413fbd07: call 0x140ce4b10
1413fbd0c: lea rcx, [rbp + 0x298]
1413fbd13: call 0x14168b370
1413fbd18: lea rcx, [rbp - 0x60]
1413fbd1c: call 0x14168ba60
1413fbd21: movups xmm0, xmmword ptr [rbp + 0x368]
1413fbd28: mov rax, qword ptr [rbp + 0x378]
1413fbd2f: movups xmmword ptr [rax + 0x60], xmm0
1413fbd33: movups xmm0, xmmword ptr [rbp + 0x358]
1413fbd3a: movups xmmword ptr [rax + 0x50], xmm0
1413fbd3e: movups xmm0, xmmword ptr [rbp + 0x348]
1413fbd45: movups xmmword ptr [rax + 0x40], xmm0
1413fbd49: movups xmm0, xmmword ptr [rbp + 0x308]
1413fbd50: movups xmm1, xmmword ptr [rbp + 0x318]
1413fbd57: movups xmm2, xmmword ptr [rbp + 0x328]
1413fbd5e: movups xmm3, xmmword ptr [rbp + 0x338]
1413fbd65: movups xmmword ptr [rax + 0x30], xmm3
1413fbd69: movups xmmword ptr [rax + 0x20], xmm2
1413fbd6d: movups xmmword ptr [rax + 0x10], xmm1
1413fbd71: movups xmmword ptr [rax], xmm0
1413fbd74: movups xmm0, xmmword ptr [rbp + 0x298]
1413fbd7b: movups xmm1, xmmword ptr [rbp + 0x2a8]
1413fbd82: movups xmm2, xmmword ptr [rbp + 0x2b8]
1413fbd89: movups xmm3, xmmword ptr [rbp + 0x2c8]
1413fbd90: movups xmmword ptr [rax + 0x70], xmm0
1413fbd94: movups xmmword ptr [rax + 0x80], xmm1
1413fbd9b: movups xmmword ptr [rax + 0x90], xmm2
1413fbda2: movups xmmword ptr [rax + 0xa0], xmm3
1413fbda9: movups xmm0, xmmword ptr [rbp + 0x2d8]
1413fbdb0: movups xmmword ptr [rax + 0xb0], xmm0
1413fbdb7: movups xmm0, xmmword ptr [rbp + 0x2e8]
1413fbdbe: movups xmmword ptr [rax + 0xc0], xmm0
1413fbdc5: movups xmm0, xmmword ptr [rbp + 0x2f8]
1413fbdcc: movups xmmword ptr [rax + 0xd0], xmm0
1413fbdd3: movups xmm0, xmmword ptr [rbp - 0x60]
1413fbdd7: movups xmm1, xmmword ptr [rbp - 0x50]
1413fbddb: movups xmm2, xmmword ptr [rbp - 0x40]
1413fbddf: movups xmm3, xmmword ptr [rbp - 0x30]
1413fbde3: movups xmmword ptr [rax + 0xe0], xmm0
1413fbdea: movups xmmword ptr [rax + 0xf0], xmm1
1413fbdf1: movups xmmword ptr [rax + 0x100], xmm2
1413fbdf8: movups xmmword ptr [rax + 0x110], xmm3
1413fbdff: movups xmm0, xmmword ptr [rbp - 0x20]
1413fbe03: movups xmmword ptr [rax + 0x120], xmm0
1413fbe0a: movups xmm0, xmmword ptr [rbp - 0x10]
1413fbe0e: movups xmmword ptr [rax + 0x130], xmm0
1413fbe15: movups xmm0, xmmword ptr [rbp]
1413fbe19: movups xmmword ptr [rax + 0x140], xmm0
1413fbe20: mov qword ptr [rbp - 0x58], 3
1413fbe28: mov qword ptr [rbp - 0x50], rax
1413fbe2c: mov qword ptr [rbp - 0x48], 3
1413fbe34: mov qword ptr [rbp - 0x40], 0
1413fbe3c: mov qword ptr [rbp - 0x38], 8
1413fbe44: xorps xmm0, xmm0
1413fbe47: movups xmmword ptr [rbp - 0x30], xmm0
1413fbe4b: mov qword ptr [rbp - 0x60], rbx
1413fbe4f: lea rcx, [rbp - 0x28]
1413fbe53: call 0x1416a00a0
1413fbe58: movups xmm0, xmmword ptr [rbp]
1413fbe5c: movups xmmword ptr [rsi + 0x60], xmm0
1413fbe60: movups xmm0, xmmword ptr [rbp - 0x10]
1413fbe64: movups xmmword ptr [rsi + 0x50], xmm0
1413fbe68: movups xmm0, xmmword ptr [rbp - 0x20]
1413fbe6c: movups xmmword ptr [rsi + 0x40], xmm0
1413fbe70: movups xmm0, xmmword ptr [rbp - 0x60]
1413fbe74: movups xmm1, xmmword ptr [rbp - 0x50]
1413fbe78: movups xmm2, xmmword ptr [rbp - 0x40]
1413fbe7c: movups xmm3, xmmword ptr [rbp - 0x30]
1413fbe80: movups xmmword ptr [rsi + 0x30], xmm3
1413fbe84: movups xmmword ptr [rsi + 0x20], xmm2
1413fbe88: movups xmmword ptr [rsi + 0x10], xmm1
1413fbe8c: movups xmmword ptr [rsi], xmm0
1413fbe8f: mov rax, rsi
1413fbe92: add rsp, 0x408
1413fbe99: pop rbx
1413fbe9a: pop rdi
1413fbe9b: pop rsi
1413fbe9c: pop rbp
1413fbe9d: ret 
1413fbe9e: mov ecx, 8
1413fbea3: mov edx, 0x150
1413fbea8: call 0x1428d9000
1413fbead: mov ecx, 8
1413fbeb2: mov edx, 0x2f8
1413fbeb7: call 0x1428d9000
1413fbebc: ud2 
1413fbebe: nop 
1413fbec0: mov qword ptr [rsp + 0x10], rdx
1413fbec5: push rbp
1413fbec6: push rsi
1413fbec7: push rdi
1413fbec8: push rbx
1413fbec9: sub rsp, 0x28
1413fbecd: lea rbp, [rdx + 0x80]
1413fbed4: lea rcx, [rbp - 0x60]
1413fbed8: call 0x14087ffb0
1413fbedd: nop 
1413fbede: add rsp, 0x28
1413fbee2: pop rbx
1413fbee3: pop rdi
1413fbee4: pop rsi
1413fbee5: pop rbp
1413fbee6: ret 
1413fbee7: nop word ptr [rax + rax]
1413fbef0: mov qword ptr [rsp + 0x10], rdx
1413fbef5: push rbp
1413fbef6: push rsi
1413fbef7: push rdi
1413fbef8: push rbx
1413fbef9: sub rsp, 0x28
1413fbefd: lea rbp, [rdx + 0x80]
1413fbf04: mov edx, 0x150
1413fbf09: mov r8d, 8
1413fbf0f: mov rcx, qword ptr [rbp + 0x378]
1413fbf16: call 0x140613c20
1413fbf1b: nop 
1413fbf1c: add rsp, 0x28
1413fbf20: pop rbx
1413fbf21: pop rdi
1413fbf22: pop rsi
1413fbf23: pop rbp
1413fbf24: ret 
1413fbf25: nop word ptr cs:[rax + rax]
1413fbf30: mov qword ptr [rsp + 0x10], rdx
1413fbf35: push rbp
1413fbf36: push rsi
1413fbf37: push rdi
1413fbf38: push rbx
1413fbf39: sub rsp, 0x28
1413fbf3d: lea rbp, [rdx + 0x80]
1413fbf44: lea rcx, [rbp + 0x308]
1413fbf4b: call 0x140029880
1413fbf50: nop 
1413fbf51: add rsp, 0x28
1413fbf55: pop rbx
1413fbf56: pop rdi
1413fbf57: pop rsi
1413fbf58: pop rbp
1413fbf59: ret 
1413fbf5a: nop word ptr [rax + rax]
1413fbf60: mov qword ptr [rsp + 0x10], rdx
1413fbf65: push rbp
1413fbf66: push rsi
1413fbf67: push rdi
1413fbf68: push rbx
1413fbf69: sub rsp, 0x28
1413fbf6d: lea rbp, [rdx + 0x80]
1413fbf74: lea rcx, [rbp + 0x298]
1413fbf7b: call 0x140029880
1413fbf80: nop 
1413fbf81: add rsp, 0x28
1413fbf85: pop rbx
1413fbf86: pop rdi
1413fbf87: pop rsi
1413fbf88: pop rbp
1413fbf89: ret 
1413fbf8a: nop word ptr [rax + rax]
1413fbf90: mov qword ptr [rsp + 0x10], rdx
1413fbf95: push rbp
1413fbf96: push rsi
1413fbf97: push rdi
1413fbf98: push rbx
1413fbf99: sub rsp, 0x28
1413fbf9d: lea rbp, [rdx + 0x80]
1413fbfa4: lea rcx, [rbp - 0x60]
1413fbfa8: call 0x140029880
1413fbfad: nop 
1413fbfae: add rsp, 0x28
1413fbfb2: pop rbx
1413fbfb3: pop rdi
1413fbfb4: pop rsi
1413fbfb5: pop rbp
1413fbfb6: ret 
