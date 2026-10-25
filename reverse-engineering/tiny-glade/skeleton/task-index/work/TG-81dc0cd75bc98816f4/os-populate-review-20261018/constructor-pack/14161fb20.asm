14161fb20: push rbp
14161fb21: push r15
14161fb23: push r14
14161fb25: push r12
14161fb27: push rsi
14161fb28: push rdi
14161fb29: push rbx
14161fb2a: sub rsp, 0x110
14161fb31: lea rbp, [rsp + 0x80]
14161fb39: mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
14161fb44: mov rdi, rdx
14161fb47: mov rsi, rcx
14161fb4a: test r8, r8
14161fb4d: je 0x14161fb67
14161fb4f: mov rbx, r9
14161fb52: cmp r9, 0x40001
14161fb59: jae 0x14161fd2b
14161fb5f: mov r15, r8
14161fb62: xor r12d, r12d
14161fb65: jmp 0x14161fba1
14161fb67: movzx eax, byte ptr [rip + 0x201509b]
14161fb6e: mov ebx, 0x40000
14161fb73: mov ecx, 0x40000
14161fb78: mov edx, 1
14161fb7d: call 0x140613c10
14161fb82: test rax, rax
14161fb85: je 0x14161fd95
14161fb8b: mov r15, rax
14161fb8e: mov r8d, 0x40000
14161fb94: mov rcx, rax
14161fb97: xor edx, edx
14161fb99: call 0x142923c20
14161fb9e: mov r12b, 1
14161fba1: mov qword ptr [rbp + 0x78], r15
14161fba5: mov byte ptr [rbp + 0x87], r12b
14161fbac: lea rcx, [rbp + 0x20]
14161fbb0: mov edx, 0x40000
14161fbb5: mov r8d, 0x22
14161fbbb: mov r9b, 2
14161fbbe: call 0x140bd4de0
14161fbc3: mov qword ptr [rbp + 0x78], r15
14161fbc7: mov byte ptr [rbp + 0x87], r12b
14161fbce: lea r14, [rbp + 8]
14161fbd2: lea rdx, [rbp + 0x20]
14161fbd6: mov r8d, 1
14161fbdc: mov rcx, r14
14161fbdf: call 0x140bd4f20
14161fbe4: mov qword ptr [rbp + 0x78], r15
14161fbe8: mov byte ptr [rbp + 0x87], r12b
14161fbef: mov qword ptr [rsp + 0x30], rbx
14161fbf4: mov qword ptr [rsp + 0x28], r15
14161fbf9: mov qword ptr [rsp + 0x20], r14
14161fbfe: lea r8, [rip + 0x1660531]
14161fc05: lea rcx, [rbp - 0x48]
14161fc09: mov r9d, 0x15
14161fc0f: mov rdx, rdi
14161fc12: call 0x1416a2dc0
14161fc17: cmp dword ptr [rbp - 0x48], 0xb
14161fc1b: jne 0x14161fcbd
14161fc21: mov rax, qword ptr [rbp - 0x20]
14161fc25: mov qword ptr [rbp + 0x40], rax
14161fc29: movups xmm0, xmmword ptr [rbp - 0x40]
14161fc2d: movups xmm1, xmmword ptr [rbp - 0x30]
14161fc31: movaps xmmword ptr [rbp + 0x30], xmm1
14161fc35: movaps xmmword ptr [rbp + 0x20], xmm0
14161fc39: test r12b, r12b
14161fc3c: je 0x14161fc51
14161fc3e: mov edx, 0x40000
14161fc43: mov r8d, 1
14161fc49: mov rcx, r15
14161fc4c: call 0x140613c20
14161fc51: lea rcx, [rbp + 0x20]
14161fc55: mov rdx, rdi
14161fc58: call 0x140bd4cd0
14161fc5d: test rax, rax
14161fc60: je 0x14161fd1a
14161fc66: cmp rdx, qword ptr [rbp + 0x38]
14161fc6a: jb 0x14161fca0
14161fc6c: mov rcx, qword ptr [rbp + 0x40]
14161fc70: mov qword ptr [rsi + 0x30], rcx
14161fc74: movaps xmm0, xmmword ptr [rbp + 0x20]
14161fc78: movaps xmm1, xmmword ptr [rbp + 0x30]
14161fc7c: movups xmmword ptr [rsi + 0x20], xmm1
14161fc80: movups xmmword ptr [rsi + 0x10], xmm0
14161fc84: mov qword ptr [rsi], rax
14161fc87: mov qword ptr [rsi + 8], rdx
14161fc8b: mov rax, rsi
14161fc8e: add rsp, 0x110
14161fc95: pop rbx
14161fc96: pop rdi
14161fc97: pop rsi
14161fc98: pop r12
14161fc9a: pop r14
14161fc9c: pop r15
14161fc9e: pop rbp
14161fc9f: ret 
14161fca0: lea rcx, [rip + 0x1660549]
14161fca7: lea r8, [rip + 0x166057a]
14161fcae: mov edx, 0x37
14161fcb3: call 0x1428d9430
14161fcb8: jmp 0x14161fdb2
14161fcbd: mov dword ptr [rbp + 0x74], r12d
14161fcc1: mov qword ptr [rbp + 0x18], r15
14161fcc5: movups xmm0, xmmword ptr [rbp - 8]
14161fcc9: movaps xmmword ptr [rbp + 0x60], xmm0
14161fccd: movups xmm0, xmmword ptr [rbp - 0x48]
14161fcd1: movups xmm1, xmmword ptr [rbp - 0x38]
14161fcd5: movups xmm2, xmmword ptr [rbp - 0x28]
14161fcd9: movups xmm3, xmmword ptr [rbp - 0x18]
14161fcdd: movaps xmmword ptr [rbp + 0x50], xmm3
14161fce1: movaps xmmword ptr [rbp + 0x40], xmm2
14161fce5: movaps xmmword ptr [rbp + 0x30], xmm1
14161fce9: movaps xmmword ptr [rbp + 0x20], xmm0
14161fced: lea rax, [rip + 0x16604b4]
14161fcf4: mov qword ptr [rsp + 0x20], rax
14161fcf9: lea rcx, [rip + 0x165ede8]
14161fd00: lea r9, [rip + 0x165edc1]
14161fd07: lea r8, [rbp + 0x20]
14161fd0b: mov edx, 0x2b
14161fd10: call 0x1428d9760
14161fd15: jmp 0x14161fdb2
14161fd1a: lea rcx, [rip + 0x16604b7]
14161fd21: call 0x1428d9310
14161fd26: jmp 0x14161fdb2
14161fd2b: lea rax, [rip + 0x147d92b]
14161fd32: mov qword ptr [rbp - 0x48], rax
14161fd36: mov qword ptr [rbp - 0x40], 0x41
14161fd3e: lea rax, [rbp - 0x48]
14161fd42: mov qword ptr [rbp + 8], rax
14161fd46: lea rax, [rip - 0x1515ebd]
14161fd4d: mov qword ptr [rbp + 0x10], rax
14161fd51: lea rax, [rip + 0x165fa70]
14161fd58: mov qword ptr [rbp + 0x20], rax
14161fd5c: mov qword ptr [rbp + 0x28], 1
14161fd64: mov qword ptr [rbp + 0x40], 0
14161fd6c: lea rax, [rbp + 8]
14161fd70: mov qword ptr [rbp + 0x30], rax
14161fd74: mov qword ptr [rbp + 0x38], 1
14161fd7c: mov byte ptr [rbp + 0x87], 0
14161fd83: lea rdx, [rip + 0x165fa4e]
14161fd8a: lea rcx, [rbp + 0x20]
14161fd8e: call 0x1428d9390
14161fd93: jmp 0x14161fdb2
14161fd95: mov byte ptr [rbp + 0x87], 0
14161fd9c: lea r8, [rip + 0x165ed0d]
14161fda3: mov ecx, 1
14161fda8: mov edx, 0x40000
14161fdad: call 0x1428d8fe3
14161fdb2: ud2 
14161fdb4: nop word ptr cs:[rax + rax]
14161fdc0: mov qword ptr [rsp + 0x10], rdx
14161fdc5: push rbp
14161fdc6: push r15
14161fdc8: push r14
14161fdca: push r12
14161fdcc: push rsi
14161fdcd: push rdi
14161fdce: push rbx
14161fdcf: sub rsp, 0x40
14161fdd3: lea rbp, [rdx + 0x80]
14161fdda: test byte ptr [rbp + 0x87], 1
14161fde1: je 0x14161fdf7
14161fde3: mov rcx, qword ptr [rbp + 0x78]
14161fde7: mov edx, 0x40000
14161fdec: mov r8d, 1
14161fdf2: call 0x140613c20
14161fdf7: nop 
14161fdf8: add rsp, 0x40
14161fdfc: pop rbx
14161fdfd: pop rdi
14161fdfe: pop rsi
14161fdff: pop r12
14161fe01: pop r14
14161fe03: pop r15
14161fe05: pop rbp
14161fe06: ret 
14161fe07: nop word ptr [rax + rax]
14161fe10: mov qword ptr [rsp + 0x10], rdx
14161fe15: push rbp
14161fe16: push r15
14161fe18: push r14
14161fe1a: push r12
14161fe1c: push rsi
14161fe1d: push rdi
14161fe1e: push rbx
14161fe1f: sub rsp, 0x40
14161fe23: lea rbp, [rdx + 0x80]
14161fe2a: lea rcx, [rbp + 0x20]
14161fe2e: call 0x140be24b0
14161fe33: nop 
14161fe34: add rsp, 0x40
14161fe38: pop rbx
14161fe39: pop rdi
14161fe3a: pop rsi
14161fe3b: pop r12
14161fe3d: pop r14
14161fe3f: pop r15
14161fe41: pop rbp
14161fe42: ret 
14161fe43: nop word ptr cs:[rax + rax]
14161fe50: mov qword ptr [rsp + 0x10], rdx
14161fe55: push rbp
14161fe56: push r15
14161fe58: push r14
14161fe5a: push r12
14161fe5c: push rsi
14161fe5d: push rdi
14161fe5e: push rbx
14161fe5f: sub rsp, 0x40
14161fe63: lea rbp, [rdx + 0x80]
14161fe6a: mov rsi, qword ptr [rbp + 0x18]
14161fe6e: mov qword ptr [rbp + 0x78], rsi
14161fe72: mov edi, dword ptr [rbp + 0x74]
14161fe75: mov byte ptr [rbp + 0x87], dil
14161fe7c: lea rcx, [rbp + 0x20]
14161fe80: call 0x1407cea70
14161fe85: mov qword ptr [rbp + 0x78], rsi
14161fe89: mov byte ptr [rbp + 0x87], dil
14161fe90: add rsp, 0x40
14161fe94: pop rbx
14161fe95: pop rdi
14161fe96: pop rsi
14161fe97: pop r12
14161fe99: pop r14
14161fe9b: pop r15
14161fe9d: pop rbp
14161fe9e: ret 
