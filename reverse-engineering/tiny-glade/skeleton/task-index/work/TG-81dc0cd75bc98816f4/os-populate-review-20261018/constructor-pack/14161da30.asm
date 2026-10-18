14161da30: push rbp
14161da31: push r15
14161da33: push r14
14161da35: push r12
14161da37: push rsi
14161da38: push rdi
14161da39: push rbx
14161da3a: sub rsp, 0x110
14161da41: lea rbp, [rsp + 0x80]
14161da49: mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
14161da54: mov rdi, rdx
14161da57: mov rsi, rcx
14161da5a: test r8, r8
14161da5d: je 0x14161da77
14161da5f: mov rbx, r9
14161da62: cmp r9, 0x40001
14161da69: jae 0x14161dc3b
14161da6f: mov r15, r8
14161da72: xor r12d, r12d
14161da75: jmp 0x14161dab1
14161da77: movzx eax, byte ptr [rip + 0x201718b]
14161da7e: mov ebx, 0x40000
14161da83: mov ecx, 0x40000
14161da88: mov edx, 1
14161da8d: call 0x140613c10
14161da92: test rax, rax
14161da95: je 0x14161dca5
14161da9b: mov r15, rax
14161da9e: mov r8d, 0x40000
14161daa4: mov rcx, rax
14161daa7: xor edx, edx
14161daa9: call 0x142923c20
14161daae: mov r12b, 1
14161dab1: mov qword ptr [rbp + 0x78], r15
14161dab5: mov byte ptr [rbp + 0x87], r12b
14161dabc: lea rcx, [rbp + 0x20]
14161dac0: mov edx, 0x40000
14161dac5: mov r8d, 0x22
14161dacb: mov r9b, 2
14161dace: call 0x140bd4de0
14161dad3: mov qword ptr [rbp + 0x78], r15
14161dad7: mov byte ptr [rbp + 0x87], r12b
14161dade: lea r14, [rbp + 8]
14161dae2: lea rdx, [rbp + 0x20]
14161dae6: mov r8d, 1
14161daec: mov rcx, r14
14161daef: call 0x140bd4f20
14161daf4: mov qword ptr [rbp + 0x78], r15
14161daf8: mov byte ptr [rbp + 0x87], r12b
14161daff: mov qword ptr [rsp + 0x30], rbx
14161db04: mov qword ptr [rsp + 0x28], r15
14161db09: mov qword ptr [rsp + 0x20], r14
14161db0e: lea r8, [rip + 0x1662229]
14161db15: lea rcx, [rbp - 0x48]
14161db19: mov r9d, 0x1d
14161db1f: mov rdx, rdi
14161db22: call 0x1416a2dc0
14161db27: cmp dword ptr [rbp - 0x48], 0xb
14161db2b: jne 0x14161dbcd
14161db31: mov rax, qword ptr [rbp - 0x20]
14161db35: mov qword ptr [rbp + 0x40], rax
14161db39: movups xmm0, xmmword ptr [rbp - 0x40]
14161db3d: movups xmm1, xmmword ptr [rbp - 0x30]
14161db41: movaps xmmword ptr [rbp + 0x30], xmm1
14161db45: movaps xmmword ptr [rbp + 0x20], xmm0
14161db49: test r12b, r12b
14161db4c: je 0x14161db61
14161db4e: mov edx, 0x40000
14161db53: mov r8d, 1
14161db59: mov rcx, r15
14161db5c: call 0x140613c20
14161db61: lea rcx, [rbp + 0x20]
14161db65: mov rdx, rdi
14161db68: call 0x140bd4cd0
14161db6d: test rax, rax
14161db70: je 0x14161dc2a
14161db76: cmp rdx, qword ptr [rbp + 0x38]
14161db7a: jb 0x14161dbb0
14161db7c: mov rcx, qword ptr [rbp + 0x40]
14161db80: mov qword ptr [rsi + 0x30], rcx
14161db84: movaps xmm0, xmmword ptr [rbp + 0x20]
14161db88: movaps xmm1, xmmword ptr [rbp + 0x30]
14161db8c: movups xmmword ptr [rsi + 0x20], xmm1
14161db90: movups xmmword ptr [rsi + 0x10], xmm0
14161db94: mov qword ptr [rsi], rax
14161db97: mov qword ptr [rsi + 8], rdx
14161db9b: mov rax, rsi
14161db9e: add rsp, 0x110
14161dba5: pop rbx
14161dba6: pop rdi
14161dba7: pop rsi
14161dba8: pop r12
14161dbaa: pop r14
14161dbac: pop r15
14161dbae: pop rbp
14161dbaf: ret 
14161dbb0: lea rcx, [rip + 0x1662639]
14161dbb7: lea r8, [rip + 0x166266a]
14161dbbe: mov edx, 0x37
14161dbc3: call 0x1428d9430
14161dbc8: jmp 0x14161dcc2
14161dbcd: mov dword ptr [rbp + 0x74], r12d
14161dbd1: mov qword ptr [rbp + 0x18], r15
14161dbd5: movups xmm0, xmmword ptr [rbp - 8]
14161dbd9: movaps xmmword ptr [rbp + 0x60], xmm0
14161dbdd: movups xmm0, xmmword ptr [rbp - 0x48]
14161dbe1: movups xmm1, xmmword ptr [rbp - 0x38]
14161dbe5: movups xmm2, xmmword ptr [rbp - 0x28]
14161dbe9: movups xmm3, xmmword ptr [rbp - 0x18]
14161dbed: movaps xmmword ptr [rbp + 0x50], xmm3
14161dbf1: movaps xmmword ptr [rbp + 0x40], xmm2
14161dbf5: movaps xmmword ptr [rbp + 0x30], xmm1
14161dbf9: movaps xmmword ptr [rbp + 0x20], xmm0
14161dbfd: lea rax, [rip + 0x16625a4]
14161dc04: mov qword ptr [rsp + 0x20], rax
14161dc09: lea rcx, [rip + 0x1660ed8]
14161dc10: lea r9, [rip + 0x1660eb1]
14161dc17: lea r8, [rbp + 0x20]
14161dc1b: mov edx, 0x2b
14161dc20: call 0x1428d9760
14161dc25: jmp 0x14161dcc2
14161dc2a: lea rcx, [rip + 0x16625a7]
14161dc31: call 0x1428d9310
14161dc36: jmp 0x14161dcc2
14161dc3b: lea rax, [rip + 0x148063e]
14161dc42: mov qword ptr [rbp - 0x48], rax
14161dc46: mov qword ptr [rbp - 0x40], 0x49
14161dc4e: lea rax, [rbp - 0x48]
14161dc52: mov qword ptr [rbp + 8], rax
14161dc56: lea rax, [rip - 0x1513dcd]
14161dc5d: mov qword ptr [rbp + 0x10], rax
14161dc61: lea rax, [rip + 0x1661b60]
14161dc68: mov qword ptr [rbp + 0x20], rax
14161dc6c: mov qword ptr [rbp + 0x28], 1
14161dc74: mov qword ptr [rbp + 0x40], 0
14161dc7c: lea rax, [rbp + 8]
14161dc80: mov qword ptr [rbp + 0x30], rax
14161dc84: mov qword ptr [rbp + 0x38], 1
14161dc8c: mov byte ptr [rbp + 0x87], 0
14161dc93: lea rdx, [rip + 0x1661b3e]
14161dc9a: lea rcx, [rbp + 0x20]
14161dc9e: call 0x1428d9390
14161dca3: jmp 0x14161dcc2
14161dca5: mov byte ptr [rbp + 0x87], 0
14161dcac: lea r8, [rip + 0x1660dfd]
14161dcb3: mov ecx, 1
14161dcb8: mov edx, 0x40000
14161dcbd: call 0x1428d8fe3
14161dcc2: ud2 
14161dcc4: nop word ptr cs:[rax + rax]
14161dcd0: mov qword ptr [rsp + 0x10], rdx
14161dcd5: push rbp
14161dcd6: push r15
14161dcd8: push r14
14161dcda: push r12
14161dcdc: push rsi
14161dcdd: push rdi
14161dcde: push rbx
14161dcdf: sub rsp, 0x40
14161dce3: lea rbp, [rdx + 0x80]
14161dcea: test byte ptr [rbp + 0x87], 1
14161dcf1: je 0x14161dd07
14161dcf3: mov rcx, qword ptr [rbp + 0x78]
14161dcf7: mov edx, 0x40000
14161dcfc: mov r8d, 1
14161dd02: call 0x140613c20
14161dd07: nop 
14161dd08: add rsp, 0x40
14161dd0c: pop rbx
14161dd0d: pop rdi
14161dd0e: pop rsi
14161dd0f: pop r12
14161dd11: pop r14
14161dd13: pop r15
14161dd15: pop rbp
14161dd16: ret 
14161dd17: nop word ptr [rax + rax]
14161dd20: mov qword ptr [rsp + 0x10], rdx
14161dd25: push rbp
14161dd26: push r15
14161dd28: push r14
14161dd2a: push r12
14161dd2c: push rsi
14161dd2d: push rdi
14161dd2e: push rbx
14161dd2f: sub rsp, 0x40
14161dd33: lea rbp, [rdx + 0x80]
14161dd3a: lea rcx, [rbp + 0x20]
14161dd3e: call 0x140be24b0
14161dd43: nop 
14161dd44: add rsp, 0x40
14161dd48: pop rbx
14161dd49: pop rdi
14161dd4a: pop rsi
14161dd4b: pop r12
14161dd4d: pop r14
14161dd4f: pop r15
14161dd51: pop rbp
14161dd52: ret 
14161dd53: nop word ptr cs:[rax + rax]
14161dd60: mov qword ptr [rsp + 0x10], rdx
14161dd65: push rbp
14161dd66: push r15
14161dd68: push r14
14161dd6a: push r12
14161dd6c: push rsi
14161dd6d: push rdi
14161dd6e: push rbx
14161dd6f: sub rsp, 0x40
14161dd73: lea rbp, [rdx + 0x80]
14161dd7a: mov rsi, qword ptr [rbp + 0x18]
14161dd7e: mov qword ptr [rbp + 0x78], rsi
14161dd82: mov edi, dword ptr [rbp + 0x74]
14161dd85: mov byte ptr [rbp + 0x87], dil
14161dd8c: lea rcx, [rbp + 0x20]
14161dd90: call 0x1407cea70
14161dd95: mov qword ptr [rbp + 0x78], rsi
14161dd99: mov byte ptr [rbp + 0x87], dil
14161dda0: add rsp, 0x40
14161dda4: pop rbx
14161dda5: pop rdi
14161dda6: pop rsi
14161dda7: pop r12
14161dda9: pop r14
14161ddab: pop r15
14161ddad: pop rbp
14161ddae: ret 
