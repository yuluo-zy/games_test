1412c4c00: push rbp
1412c4c01: push rsi
1412c4c02: sub rsp, 0xe8
1412c4c09: lea rbp, [rsp + 0x80]
1412c4c11: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412c4c19: mov rsi, rcx
1412c4c1c: lea rcx, [rbp - 0x58]
1412c4c20: xor r8d, r8d
1412c4c23: call 0x14161e8f0
1412c4c28: movzx eax, byte ptr [rip + 0x236ffda]
1412c4c2f: mov ecx, 0x38
1412c4c34: mov edx, 8
1412c4c39: call 0x140613c10
1412c4c3e: test rax, rax
1412c4c41: je 0x1412c4cff
1412c4c47: mov rcx, qword ptr [rbp - 0x28]
1412c4c4b: mov qword ptr [rax + 0x30], rcx
1412c4c4f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c4c53: movups xmm1, xmmword ptr [rbp - 0x48]
1412c4c57: movups xmm2, xmmword ptr [rbp - 0x38]
1412c4c5b: movups xmmword ptr [rax + 0x20], xmm2
1412c4c5f: movups xmmword ptr [rax + 0x10], xmm1
1412c4c63: movups xmmword ptr [rax], xmm0
1412c4c66: lea rcx, [rip + 0x17d9209]
1412c4c6d: mov qword ptr [rbp + 0x10], rcx
1412c4c71: mov qword ptr [rbp + 0x18], 0x47
1412c4c79: mov qword ptr [rbp + 0x20], rax
1412c4c7d: lea rax, [rip + 0x1968504]
1412c4c84: mov qword ptr [rbp + 0x28], rax
1412c4c88: mov qword ptr [rbp + 0x40], 0
1412c4c90: mov byte ptr [rbp + 0x56], 5
1412c4c94: mov qword ptr [rbp + 0x30], rcx
1412c4c98: mov qword ptr [rbp + 0x38], 0x47
1412c4ca0: movzx eax, byte ptr [rip + 0x236ff62]
1412c4ca7: mov ecx, 0x40
1412c4cac: mov edx, 8
1412c4cb1: call 0x140613c10
1412c4cb6: test rax, rax
1412c4cb9: je 0x1412c4d10
1412c4cbb: movups xmm0, xmmword ptr [rbp + 0x20]
1412c4cbf: movups xmm1, xmmword ptr [rbp + 0x30]
1412c4cc3: movups xmm2, xmmword ptr [rbp + 0x40]
1412c4cc7: movups xmm3, xmmword ptr [rbp + 0x50]
1412c4ccb: movups xmmword ptr [rax + 0x30], xmm3
1412c4ccf: movups xmmword ptr [rax + 0x20], xmm2
1412c4cd3: movups xmmword ptr [rax + 0x10], xmm1
1412c4cd7: movups xmmword ptr [rax], xmm0
1412c4cda: movabs rdx, 0x75bd131453883cb7
1412c4ce4: mov rcx, rsi
1412c4ce7: mov r8, rax
1412c4cea: call 0x140969bf0
1412c4cef: cmp rax, 1
1412c4cf3: je 0x1412c4d21
1412c4cf5: add rsp, 0xe8
1412c4cfc: pop rsi
1412c4cfd: pop rbp
1412c4cfe: ret 
1412c4cff: mov ecx, 8
1412c4d04: mov edx, 0x38
1412c4d09: call 0x1428d9000
1412c4d0e: jmp 0x1412c4d1f
1412c4d10: mov ecx, 8
1412c4d15: mov edx, 0x40
1412c4d1a: call 0x1428d9000
1412c4d1f: ud2 
1412c4d21: mov rax, rdx
1412c4d24: add rdx, 0x10
1412c4d28: mov rsi, rdx
1412c4d2b: cmp qword ptr [rax + 0x18], 0x47
1412c4d30: jne 0x1412c4daf
1412c4d32: mov rdx, qword ptr [rsi]
1412c4d35: lea rcx, [rip + 0x17d913a]
1412c4d3c: mov r8d, 0x47
1412c4d42: call 0x142923bf0
1412c4d47: test eax, eax
1412c4d49: jne 0x1412c4daf
1412c4d4b: lea rax, [rip + 0x196892e]
1412c4d52: mov qword ptr [rbp - 0x20], rax
1412c4d56: lea rax, [rip - 0xd914bd]
1412c4d5d: mov qword ptr [rbp - 0x18], rax
1412c4d61: lea rax, [rbp + 0x10]
1412c4d65: mov qword ptr [rbp - 0x10], rax
1412c4d69: lea rax, [rip - 0x11baee0]
1412c4d70: mov qword ptr [rbp - 8], rax
1412c4d74: lea rax, [rip + 0x19688ad]
1412c4d7b: mov qword ptr [rbp + 0x20], rax
1412c4d7f: mov qword ptr [rbp + 0x28], 3
1412c4d87: mov qword ptr [rbp + 0x40], 0
1412c4d8f: lea rax, [rbp - 0x20]
1412c4d93: mov qword ptr [rbp + 0x30], rax
1412c4d97: mov qword ptr [rbp + 0x38], 2
1412c4d9f: lea rdx, [rip + 0x19688b2]
1412c4da6: lea rcx, [rbp + 0x20]
1412c4daa: call 0x1428d9390
1412c4daf: lea rax, [rip + 0x19688ca]
1412c4db6: mov qword ptr [rbp + 0x20], rax
1412c4dba: lea rax, [rip - 0xd91521]
1412c4dc1: mov qword ptr [rbp + 0x28], rax
1412c4dc5: mov qword ptr [rbp + 0x30], rsi
1412c4dc9: lea rax, [rip - 0x11baf40]
1412c4dd0: mov qword ptr [rbp + 0x38], rax
1412c4dd4: lea rcx, [rbp + 0x10]
1412c4dd8: mov qword ptr [rbp + 0x40], rcx
1412c4ddc: mov qword ptr [rbp + 0x48], rax
1412c4de0: lea rax, [rip + 0x1968781]
1412c4de7: mov qword ptr [rbp - 0x20], rax
1412c4deb: mov qword ptr [rbp - 0x18], 3
1412c4df3: mov qword ptr [rbp], 0
1412c4dfb: lea rax, [rbp + 0x20]
1412c4dff: mov qword ptr [rbp - 0x10], rax
1412c4e03: mov qword ptr [rbp - 8], 3
1412c4e0b: lea rdx, [rip + 0x19687be]
1412c4e12: lea rcx, [rbp - 0x20]
1412c4e16: call 0x1428d9390
1412c4e1b: int3 
1412c4e1c: nop dword ptr [rax]
1412c4e20: mov qword ptr [rsp + 0x10], rdx
1412c4e25: push rbp
1412c4e26: push rsi
1412c4e27: sub rsp, 0x28
1412c4e2b: lea rbp, [rdx + 0x80]
1412c4e32: lea rcx, [rbp - 0x48]
1412c4e36: call 0x140be24b0
1412c4e3b: nop 
1412c4e3c: add rsp, 0x28
1412c4e40: pop rsi
1412c4e41: pop rbp
1412c4e42: ret 
1412c4e43: nop word ptr cs:[rax + rax]
1412c4e50: mov qword ptr [rsp + 0x10], rdx
1412c4e55: push rbp
1412c4e56: push rsi
1412c4e57: sub rsp, 0x28
1412c4e5b: lea rbp, [rdx + 0x80]
1412c4e62: lea rcx, [rbp + 0x20]
1412c4e66: call 0x1400c04b0
1412c4e6b: nop 
1412c4e6c: add rsp, 0x28
1412c4e70: pop rsi
1412c4e71: pop rbp
1412c4e72: ret 
