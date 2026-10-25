140327c20: push rsi
140327c21: push rdi
140327c22: push rbx
140327c23: sub rsp, 0xc0
140327c2a: mov ebx, edx
140327c2c: mov rsi, rcx
140327c2f: add rcx, 0x38
140327c33: movabs rdx, 0x8e1bd251962ba784
140327c3d: movabs r8, 0x6a782374c0e6529b
140327c47: call 0x1400dd6f0
140327c4c: test rax, rax
140327c4f: je 0x140327cf6
140327c55: cmp qword ptr [rsi + 0x218], rdx
140327c5c: jbe 0x140327cf6
140327c62: mov rax, qword ptr [rsi + 0x210]
140327c69: mov rcx, qword ptr [rax + rdx*8]
140327c6d: test rcx, rcx
140327c70: je 0x140327cf6
140327c76: mov rax, qword ptr [rsi + 0x1e0]
140327c7d: not rcx
140327c80: lea rcx, [rcx + rcx*2]
140327c84: shl rcx, 5
140327c88: cmp qword ptr [rax + rcx + 0x20], 0
140327c8e: je 0x140327cf6
140327c90: add rax, rcx
140327c93: mov rcx, qword ptr [rax + 0x10]
140327c97: mov edx, dword ptr [rsi + 0x5b0]
140327c9d: mov dword ptr [rax + 0x5c], edx
140327ca0: mov edx, ebx
140327ca2: call 0x140171d50
140327ca7: mov rdi, rax
140327caa: lea rbx, [rsp + 0x50]
140327caf: mov rcx, rbx
140327cb2: call 0x1403c0e50
140327cb7: lea rcx, [rsp + 0x30]
140327cbc: mov rdx, rdi
140327cbf: mov r8, rbx
140327cc2: xor r9d, r9d
140327cc5: call 0x14016dee0
140327cca: mov rdx, qword ptr [rsp + 0x30]
140327ccf: test rdx, rdx
140327cd2: je 0x140327ce8
140327cd4: mov rcx, qword ptr [rsp + 0x38]
140327cd9: shl rdx, 4
140327cdd: mov r8d, 8
140327ce3: call 0x140613c20
140327ce8: mov rax, rsi
140327ceb: add rsp, 0xc0
140327cf2: pop rbx
140327cf3: pop rdi
140327cf4: pop rsi
140327cf5: ret 
140327cf6: lea rax, [rip + 0x269653b]
140327cfd: mov qword ptr [rsp + 0x30], rax
140327d02: mov qword ptr [rsp + 0x38], 0x27
140327d0b: lea rax, [rsp + 0x30]
140327d10: mov qword ptr [rsp + 0x20], rax
140327d15: lea rax, [rip - 0x21de8c]
140327d1c: mov qword ptr [rsp + 0x28], rax
140327d21: lea rax, [rip + 0x2696640]
140327d28: mov qword ptr [rsp + 0x50], rax
140327d2d: mov qword ptr [rsp + 0x58], 2
140327d36: mov qword ptr [rsp + 0x70], 0
140327d3f: lea rax, [rsp + 0x20]
140327d44: mov qword ptr [rsp + 0x60], rax
140327d49: mov qword ptr [rsp + 0x68], 1
140327d52: lea rdx, [rip + 0x2696497]
140327d59: lea rcx, [rsp + 0x50]
140327d5e: call 0x1428d9390
140327d63: int3 
