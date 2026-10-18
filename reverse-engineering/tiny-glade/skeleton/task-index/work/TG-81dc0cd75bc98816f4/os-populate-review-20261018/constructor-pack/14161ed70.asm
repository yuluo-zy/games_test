14161ed70: push rbp
14161ed71: push r15
14161ed73: push r14
14161ed75: push r12
14161ed77: push rsi
14161ed78: push rdi
14161ed79: push rbx
14161ed7a: sub rsp, 0x130
14161ed81: lea rbp, [rsp + 0x80]
14161ed89: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
14161ed94: mov rdi, rdx
14161ed97: mov rsi, rcx
14161ed9a: test r8, r8
14161ed9d: je 0x14161edd6
14161ed9f: mov rbx, r9
14161eda2: mov rax, r9
14161eda5: and rax, 0xf
14161eda9: mov qword ptr [rbp - 0x40], rax
14161edad: jne 0x14161efb1
14161edb3: mov r15, r8
14161edb6: mov rax, r8
14161edb9: and rax, 3
14161edbd: mov qword ptr [rbp - 0x40], rax
14161edc1: jne 0x14161efc2
14161edc7: cmp rbx, 0x20
14161edcb: jae 0x14161f05d
14161edd1: xor r12d, r12d
14161edd4: jmp 0x14161ee06
14161edd6: movzx eax, byte ptr [rip + 0x2015e2c]
14161eddd: mov ebx, 0x10
14161ede2: mov ecx, 0x10
14161ede7: mov edx, 4
14161edec: call 0x140613c10
14161edf1: test rax, rax
14161edf4: je 0x14161f0c7
14161edfa: mov r15, rax
14161edfd: xorps xmm0, xmm0
14161ee00: movups xmmword ptr [rax], xmm0
14161ee03: mov r12b, 1
14161ee06: mov qword ptr [rbp + 0x98], r15
14161ee0d: mov byte ptr [rbp + 0xa7], r12b
14161ee14: lea rcx, [rbp + 0x10]
14161ee18: mov edx, 0x10
14161ee1d: mov r8d, 0x22
14161ee23: mov r9b, 4
14161ee26: call 0x140bd4de0
14161ee2b: mov qword ptr [rbp + 0x98], r15
14161ee32: mov byte ptr [rbp + 0xa7], r12b
14161ee39: lea r14, [rbp + 0x60]
14161ee3d: lea rdx, [rbp + 0x10]
14161ee41: mov r8d, 4
14161ee47: mov rcx, r14
14161ee4a: call 0x140bd4f20
14161ee4f: mov qword ptr [rbp + 0x98], r15
14161ee56: mov byte ptr [rbp + 0xa7], r12b
14161ee5d: mov qword ptr [rsp + 0x30], rbx
14161ee62: mov qword ptr [rsp + 0x28], r15
14161ee67: mov qword ptr [rsp + 0x20], r14
14161ee6c: lea r8, [rip + 0x166120d]
14161ee73: lea rcx, [rbp - 0x40]
14161ee77: mov r9d, 0x14
14161ee7d: mov rdx, rdi
14161ee80: call 0x1416a2dc0
14161ee85: cmp dword ptr [rbp - 0x40], 0xb
14161ee89: jne 0x14161efec
14161ee8f: mov rax, qword ptr [rbp - 0x18]
14161ee93: mov qword ptr [rbp + 0x80], rax
14161ee9a: movups xmm0, xmmword ptr [rbp - 0x38]
14161ee9e: movups xmm1, xmmword ptr [rbp - 0x28]
14161eea2: movaps xmmword ptr [rbp + 0x70], xmm1
14161eea6: movaps xmmword ptr [rbp + 0x60], xmm0
14161eeaa: test r12b, r12b
14161eead: je 0x14161eec2
14161eeaf: mov edx, 0x10
14161eeb4: mov r8d, 4
14161eeba: mov rcx, r15
14161eebd: call 0x140613c20
14161eec2: lea rcx, [rbp + 0x60]
14161eec6: mov rdx, rdi
14161eec9: call 0x140bd4cd0
14161eece: test rax, rax
14161eed1: je 0x14161f04c
14161eed7: cmp rdx, qword ptr [rbp + 0x78]
14161eedb: jb 0x14161ef36
14161eedd: mov rcx, rdx
14161eee0: and rcx, 0xf
14161eee4: mov qword ptr [rbp - 0x40], rcx
14161eee8: jne 0x14161ef53
14161eeea: mov rcx, rax
14161eeed: and rcx, 3
14161eef1: mov qword ptr [rbp - 0x40], rcx
14161eef5: jne 0x14161ef82
14161eefb: mov rcx, qword ptr [rbp + 0x80]
14161ef02: mov qword ptr [rsi + 0x30], rcx
14161ef06: movaps xmm0, xmmword ptr [rbp + 0x60]
14161ef0a: movaps xmm1, xmmword ptr [rbp + 0x70]
14161ef0e: movups xmmword ptr [rsi + 0x20], xmm1
14161ef12: movups xmmword ptr [rsi + 0x10], xmm0
14161ef16: shr rdx, 4
14161ef1a: mov qword ptr [rsi], rax
14161ef1d: mov qword ptr [rsi + 8], rdx
14161ef21: mov rax, rsi
14161ef24: add rsp, 0x130
14161ef2b: pop rbx
14161ef2c: pop rdi
14161ef2d: pop rsi
14161ef2e: pop r12
14161ef30: pop r14
14161ef32: pop r15
14161ef34: pop rbp
14161ef35: ret 
14161ef36: lea rcx, [rip + 0x16612b3]
14161ef3d: lea r8, [rip + 0x16612e4]
14161ef44: mov edx, 0x37
14161ef49: call 0x1428d9430
14161ef4e: jmp 0x14161f0e4
14161ef53: mov qword ptr [rbp + 0x10], 0
14161ef5b: lea rax, [rip + 0x16612e6]
14161ef62: mov qword ptr [rsp + 0x20], rax
14161ef67: lea r8, [rip + 0x16612d2]
14161ef6e: lea rdx, [rbp - 0x40]
14161ef72: lea r9, [rbp + 0x10]
14161ef76: xor ecx, ecx
14161ef78: call 0x1428d9596
14161ef7d: jmp 0x14161f0e4
14161ef82: mov qword ptr [rbp + 0x10], 0
14161ef8a: lea rax, [rip + 0x16612cf]
14161ef91: mov qword ptr [rsp + 0x20], rax
14161ef96: lea r8, [rip + 0x16612a3]
14161ef9d: lea rdx, [rbp - 0x40]
14161efa1: lea r9, [rbp + 0x10]
14161efa5: xor ecx, ecx
14161efa7: call 0x1428d9596
14161efac: jmp 0x14161f0e4
14161efb1: mov qword ptr [rbp + 0x10], 0
14161efb9: lea rax, [rip + 0x16aaf88]
14161efc0: jmp 0x14161efd1
14161efc2: mov qword ptr [rbp + 0x10], 0
14161efca: lea rax, [rip + 0x16aaf8f]
14161efd1: mov qword ptr [rsp + 0x20], rax
14161efd6: lea r8, [rip + 0x16aaf63]
14161efdd: lea rdx, [rbp - 0x40]
14161efe1: lea r9, [rbp + 0x10]
14161efe5: xor ecx, ecx
14161efe7: call 0x1428d9596
14161efec: mov dword ptr [rbp + 0x94], r12d
14161eff3: mov qword ptr [rbp - 0x48], r15
14161eff7: movups xmm0, xmmword ptr [rbp]
14161effb: movaps xmmword ptr [rbp + 0x50], xmm0
14161efff: movups xmm0, xmmword ptr [rbp - 0x40]
14161f003: movups xmm1, xmmword ptr [rbp - 0x30]
14161f007: movups xmm2, xmmword ptr [rbp - 0x20]
14161f00b: movups xmm3, xmmword ptr [rbp - 0x10]
14161f00f: movaps xmmword ptr [rbp + 0x40], xmm3
14161f013: movaps xmmword ptr [rbp + 0x30], xmm2
14161f017: movaps xmmword ptr [rbp + 0x20], xmm1
14161f01b: movaps xmmword ptr [rbp + 0x10], xmm0
14161f01f: lea rax, [rip + 0x1661182]
14161f026: mov qword ptr [rsp + 0x20], rax
14161f02b: lea rcx, [rip + 0x165fab6]
14161f032: lea r9, [rip + 0x165fa8f]
14161f039: lea r8, [rbp + 0x10]
14161f03d: mov edx, 0x2b
14161f042: call 0x1428d9760
14161f047: jmp 0x14161f0e4
14161f04c: lea rcx, [rip + 0x1661185]
14161f053: call 0x1428d9310
14161f058: jmp 0x14161f0e4
14161f05d: lea rax, [rip + 0x147f7ab]
14161f064: mov qword ptr [rbp - 0x40], rax
14161f068: mov qword ptr [rbp - 0x38], 0x57
14161f070: lea rax, [rbp - 0x40]
14161f074: mov qword ptr [rbp + 0x60], rax
14161f078: lea rax, [rip - 0x15151ef]
14161f07f: mov qword ptr [rbp + 0x68], rax
14161f083: lea rax, [rip + 0x166073e]
14161f08a: mov qword ptr [rbp + 0x10], rax
14161f08e: mov qword ptr [rbp + 0x18], 1
14161f096: mov qword ptr [rbp + 0x30], 0
14161f09e: lea rax, [rbp + 0x60]
14161f0a2: mov qword ptr [rbp + 0x20], rax
14161f0a6: mov qword ptr [rbp + 0x28], 1
14161f0ae: mov byte ptr [rbp + 0xa7], 0
14161f0b5: lea rdx, [rip + 0x166071c]
14161f0bc: lea rcx, [rbp + 0x10]
14161f0c0: call 0x1428d9390
14161f0c5: jmp 0x14161f0e4
14161f0c7: mov byte ptr [rbp + 0xa7], 0
14161f0ce: lea r8, [rip + 0x165f9db]
14161f0d5: mov ecx, 4
14161f0da: mov edx, 0x10
14161f0df: call 0x1428d8fe3
14161f0e4: ud2 
14161f0e6: nop word ptr cs:[rax + rax]
14161f0f0: mov qword ptr [rsp + 0x10], rdx
14161f0f5: push rbp
14161f0f6: push r15
14161f0f8: push r14
14161f0fa: push r12
14161f0fc: push rsi
14161f0fd: push rdi
14161f0fe: push rbx
14161f0ff: sub rsp, 0x40
14161f103: lea rbp, [rdx + 0x80]
14161f10a: mov rsi, qword ptr [rbp - 0x48]
14161f10e: mov qword ptr [rbp + 0x98], rsi
14161f115: mov edi, dword ptr [rbp + 0x94]
14161f11b: mov byte ptr [rbp + 0xa7], dil
14161f122: lea rcx, [rbp + 0x10]
14161f126: call 0x1407cea70
14161f12b: mov qword ptr [rbp + 0x98], rsi
14161f132: mov byte ptr [rbp + 0xa7], dil
14161f139: add rsp, 0x40
14161f13d: pop rbx
14161f13e: pop rdi
14161f13f: pop rsi
14161f140: pop r12
14161f142: pop r14
14161f144: pop r15
14161f146: pop rbp
14161f147: ret 
14161f148: nop dword ptr [rax + rax]
14161f150: mov qword ptr [rsp + 0x10], rdx
14161f155: push rbp
14161f156: push r15
14161f158: push r14
14161f15a: push r12
14161f15c: push rsi
14161f15d: push rdi
14161f15e: push rbx
14161f15f: sub rsp, 0x40
14161f163: lea rbp, [rdx + 0x80]
14161f16a: test byte ptr [rbp + 0xa7], 1
14161f171: je 0x14161f18a
14161f173: mov rcx, qword ptr [rbp + 0x98]
14161f17a: mov edx, 0x10
14161f17f: mov r8d, 4
14161f185: call 0x140613c20
14161f18a: nop 
14161f18b: add rsp, 0x40
14161f18f: pop rbx
14161f190: pop rdi
14161f191: pop rsi
14161f192: pop r12
14161f194: pop r14
14161f196: pop r15
14161f198: pop rbp
14161f199: ret 
14161f19a: nop word ptr [rax + rax]
14161f1a0: mov qword ptr [rsp + 0x10], rdx
14161f1a5: push rbp
14161f1a6: push r15
14161f1a8: push r14
14161f1aa: push r12
14161f1ac: push rsi
14161f1ad: push rdi
14161f1ae: push rbx
14161f1af: sub rsp, 0x40
14161f1b3: lea rbp, [rdx + 0x80]
14161f1ba: lea rcx, [rbp + 0x60]
14161f1be: call 0x140be24b0
14161f1c3: nop 
14161f1c4: add rsp, 0x40
14161f1c8: pop rbx
14161f1c9: pop rdi
14161f1ca: pop rsi
14161f1cb: pop r12
14161f1cd: pop r14
14161f1cf: pop r15
14161f1d1: pop rbp
14161f1d2: ret 
