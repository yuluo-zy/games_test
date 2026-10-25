14161ca00: push rbp
14161ca01: push r15
14161ca03: push r14
14161ca05: push r12
14161ca07: push rsi
14161ca08: push rdi
14161ca09: push rbx
14161ca0a: sub rsp, 0x110
14161ca11: lea rbp, [rsp + 0x80]
14161ca19: mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
14161ca24: mov rdi, rdx
14161ca27: mov rsi, rcx
14161ca2a: test r8, r8
14161ca2d: je 0x14161ca47
14161ca2f: mov rbx, r9
14161ca32: cmp r9, 0x40001
14161ca39: jae 0x14161cc0b
14161ca3f: mov r15, r8
14161ca42: xor r12d, r12d
14161ca45: jmp 0x14161ca81
14161ca47: movzx eax, byte ptr [rip + 0x20181bb]
14161ca4e: mov ebx, 0x40000
14161ca53: mov ecx, 0x40000
14161ca58: mov edx, 1
14161ca5d: call 0x140613c10
14161ca62: test rax, rax
14161ca65: je 0x14161cc75
14161ca6b: mov r15, rax
14161ca6e: mov r8d, 0x40000
14161ca74: mov rcx, rax
14161ca77: xor edx, edx
14161ca79: call 0x142923c20
14161ca7e: mov r12b, 1
14161ca81: mov qword ptr [rbp + 0x78], r15
14161ca85: mov byte ptr [rbp + 0x87], r12b
14161ca8c: lea rcx, [rbp + 0x20]
14161ca90: mov edx, 0x40000
14161ca95: mov r8d, 0x22
14161ca9b: mov r9b, 2
14161ca9e: call 0x140bd4de0
14161caa3: mov qword ptr [rbp + 0x78], r15
14161caa7: mov byte ptr [rbp + 0x87], r12b
14161caae: lea r14, [rbp + 8]
14161cab2: lea rdx, [rbp + 0x20]
14161cab6: mov r8d, 1
14161cabc: mov rcx, r14
14161cabf: call 0x140bd4f20
14161cac4: mov qword ptr [rbp + 0x78], r15
14161cac8: mov byte ptr [rbp + 0x87], r12b
14161cacf: mov qword ptr [rsp + 0x30], rbx
14161cad4: mov qword ptr [rsp + 0x28], r15
14161cad9: mov qword ptr [rsp + 0x20], r14
14161cade: lea r8, [rip + 0x166327b]
14161cae5: lea rcx, [rbp - 0x48]
14161cae9: mov r9d, 0x17
14161caef: mov rdx, rdi
14161caf2: call 0x1416a2dc0
14161caf7: cmp dword ptr [rbp - 0x48], 0xb
14161cafb: jne 0x14161cb9d
14161cb01: mov rax, qword ptr [rbp - 0x20]
14161cb05: mov qword ptr [rbp + 0x40], rax
14161cb09: movups xmm0, xmmword ptr [rbp - 0x40]
14161cb0d: movups xmm1, xmmword ptr [rbp - 0x30]
14161cb11: movaps xmmword ptr [rbp + 0x30], xmm1
14161cb15: movaps xmmword ptr [rbp + 0x20], xmm0
14161cb19: test r12b, r12b
14161cb1c: je 0x14161cb31
14161cb1e: mov edx, 0x40000
14161cb23: mov r8d, 1
14161cb29: mov rcx, r15
14161cb2c: call 0x140613c20
14161cb31: lea rcx, [rbp + 0x20]
14161cb35: mov rdx, rdi
14161cb38: call 0x140bd4cd0
14161cb3d: test rax, rax
14161cb40: je 0x14161cbfa
14161cb46: cmp rdx, qword ptr [rbp + 0x38]
14161cb4a: jb 0x14161cb80
14161cb4c: mov rcx, qword ptr [rbp + 0x40]
14161cb50: mov qword ptr [rsi + 0x30], rcx
14161cb54: movaps xmm0, xmmword ptr [rbp + 0x20]
14161cb58: movaps xmm1, xmmword ptr [rbp + 0x30]
14161cb5c: movups xmmword ptr [rsi + 0x20], xmm1
14161cb60: movups xmmword ptr [rsi + 0x10], xmm0
14161cb64: mov qword ptr [rsi], rax
14161cb67: mov qword ptr [rsi + 8], rdx
14161cb6b: mov rax, rsi
14161cb6e: add rsp, 0x110
14161cb75: pop rbx
14161cb76: pop rdi
14161cb77: pop rsi
14161cb78: pop r12
14161cb7a: pop r14
14161cb7c: pop r15
14161cb7e: pop rbp
14161cb7f: ret 
14161cb80: lea rcx, [rip + 0x1663669]
14161cb87: lea r8, [rip + 0x166369a]
14161cb8e: mov edx, 0x37
14161cb93: call 0x1428d9430
14161cb98: jmp 0x14161cc92
14161cb9d: mov dword ptr [rbp + 0x74], r12d
14161cba1: mov qword ptr [rbp + 0x18], r15
14161cba5: movups xmm0, xmmword ptr [rbp - 8]
14161cba9: movaps xmmword ptr [rbp + 0x60], xmm0
14161cbad: movups xmm0, xmmword ptr [rbp - 0x48]
14161cbb1: movups xmm1, xmmword ptr [rbp - 0x38]
14161cbb5: movups xmm2, xmmword ptr [rbp - 0x28]
14161cbb9: movups xmm3, xmmword ptr [rbp - 0x18]
14161cbbd: movaps xmmword ptr [rbp + 0x50], xmm3
14161cbc1: movaps xmmword ptr [rbp + 0x40], xmm2
14161cbc5: movaps xmmword ptr [rbp + 0x30], xmm1
14161cbc9: movaps xmmword ptr [rbp + 0x20], xmm0
14161cbcd: lea rax, [rip + 0x16635d4]
14161cbd4: mov qword ptr [rsp + 0x20], rax
14161cbd9: lea rcx, [rip + 0x1661f08]
14161cbe0: lea r9, [rip + 0x1661ee1]
14161cbe7: lea r8, [rbp + 0x20]
14161cbeb: mov edx, 0x2b
14161cbf0: call 0x1428d9760
14161cbf5: jmp 0x14161cc92
14161cbfa: lea rcx, [rip + 0x16635d7]
14161cc01: call 0x1428d9310
14161cc06: jmp 0x14161cc92
14161cc0b: lea rax, [rip + 0x14804a0]
14161cc12: mov qword ptr [rbp - 0x48], rax
14161cc16: mov qword ptr [rbp - 0x40], 0x43
14161cc1e: lea rax, [rbp - 0x48]
14161cc22: mov qword ptr [rbp + 8], rax
14161cc26: lea rax, [rip - 0x1512d9d]
14161cc2d: mov qword ptr [rbp + 0x10], rax
14161cc31: lea rax, [rip + 0x1662b90]
14161cc38: mov qword ptr [rbp + 0x20], rax
14161cc3c: mov qword ptr [rbp + 0x28], 1
14161cc44: mov qword ptr [rbp + 0x40], 0
14161cc4c: lea rax, [rbp + 8]
14161cc50: mov qword ptr [rbp + 0x30], rax
14161cc54: mov qword ptr [rbp + 0x38], 1
14161cc5c: mov byte ptr [rbp + 0x87], 0
14161cc63: lea rdx, [rip + 0x1662b6e]
14161cc6a: lea rcx, [rbp + 0x20]
14161cc6e: call 0x1428d9390
14161cc73: jmp 0x14161cc92
14161cc75: mov byte ptr [rbp + 0x87], 0
14161cc7c: lea r8, [rip + 0x1661e2d]
14161cc83: mov ecx, 1
14161cc88: mov edx, 0x40000
14161cc8d: call 0x1428d8fe3
14161cc92: ud2 
14161cc94: nop word ptr cs:[rax + rax]
14161cca0: mov qword ptr [rsp + 0x10], rdx
14161cca5: push rbp
14161cca6: push r15
14161cca8: push r14
14161ccaa: push r12
14161ccac: push rsi
14161ccad: push rdi
14161ccae: push rbx
14161ccaf: sub rsp, 0x40
14161ccb3: lea rbp, [rdx + 0x80]
14161ccba: test byte ptr [rbp + 0x87], 1
14161ccc1: je 0x14161ccd7
14161ccc3: mov rcx, qword ptr [rbp + 0x78]
14161ccc7: mov edx, 0x40000
14161cccc: mov r8d, 1
14161ccd2: call 0x140613c20
14161ccd7: nop 
14161ccd8: add rsp, 0x40
14161ccdc: pop rbx
14161ccdd: pop rdi
14161ccde: pop rsi
14161ccdf: pop r12
14161cce1: pop r14
14161cce3: pop r15
14161cce5: pop rbp
14161cce6: ret 
14161cce7: nop word ptr [rax + rax]
14161ccf0: mov qword ptr [rsp + 0x10], rdx
14161ccf5: push rbp
14161ccf6: push r15
14161ccf8: push r14
14161ccfa: push r12
14161ccfc: push rsi
14161ccfd: push rdi
14161ccfe: push rbx
14161ccff: sub rsp, 0x40
14161cd03: lea rbp, [rdx + 0x80]
14161cd0a: lea rcx, [rbp + 0x20]
14161cd0e: call 0x140be24b0
14161cd13: nop 
14161cd14: add rsp, 0x40
14161cd18: pop rbx
14161cd19: pop rdi
14161cd1a: pop rsi
14161cd1b: pop r12
14161cd1d: pop r14
14161cd1f: pop r15
14161cd21: pop rbp
14161cd22: ret 
14161cd23: nop word ptr cs:[rax + rax]
14161cd30: mov qword ptr [rsp + 0x10], rdx
14161cd35: push rbp
14161cd36: push r15
14161cd38: push r14
14161cd3a: push r12
14161cd3c: push rsi
14161cd3d: push rdi
14161cd3e: push rbx
14161cd3f: sub rsp, 0x40
14161cd43: lea rbp, [rdx + 0x80]
14161cd4a: mov rsi, qword ptr [rbp + 0x18]
14161cd4e: mov qword ptr [rbp + 0x78], rsi
14161cd52: mov edi, dword ptr [rbp + 0x74]
14161cd55: mov byte ptr [rbp + 0x87], dil
14161cd5c: lea rcx, [rbp + 0x20]
14161cd60: call 0x1407cea70
14161cd65: mov qword ptr [rbp + 0x78], rsi
14161cd69: mov byte ptr [rbp + 0x87], dil
14161cd70: add rsp, 0x40
14161cd74: pop rbx
14161cd75: pop rdi
14161cd76: pop rsi
14161cd77: pop r12
14161cd79: pop r14
14161cd7b: pop r15
14161cd7d: pop rbp
14161cd7e: ret 
