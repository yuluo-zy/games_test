1412c3a50: push rbp
1412c3a51: push rsi
1412c3a52: push rdi
1412c3a53: sub rsp, 0x140
1412c3a5a: lea rbp, [rsp + 0x80]
1412c3a62: mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
1412c3a6d: mov rsi, rcx
1412c3a70: lea rcx, [rbp - 0x58]
1412c3a74: xor r8d, r8d
1412c3a77: call 0x1412e5f50
1412c3a7c: movzx eax, byte ptr [rip + 0x2371186]
1412c3a83: mov ecx, 0x90
1412c3a88: mov edx, 8
1412c3a8d: call 0x140613c10
1412c3a92: test rax, rax
1412c3a95: je 0x1412c3b62
1412c3a9b: mov rdi, rax
1412c3a9e: lea rdx, [rbp - 0x58]
1412c3aa2: mov r8d, 0x90
1412c3aa8: mov rcx, rax
1412c3aab: call 0x142923c00
1412c3ab0: lea rax, [rip + 0x17dab05]
1412c3ab7: mov qword ptr [rbp + 0x68], rax
1412c3abb: mov qword ptr [rbp + 0x70], 0x2d
1412c3ac3: mov qword ptr [rbp + 0x78], rdi
1412c3ac7: lea rcx, [rip + 0x1969532]
1412c3ace: mov qword ptr [rbp + 0x80], rcx
1412c3ad5: mov qword ptr [rbp + 0x98], 0
1412c3ae0: mov byte ptr [rbp + 0xae], 5
1412c3ae7: mov qword ptr [rbp + 0x88], rax
1412c3aee: mov qword ptr [rbp + 0x90], 0x2d
1412c3af9: movzx eax, byte ptr [rip + 0x2371109]
1412c3b00: mov ecx, 0x40
1412c3b05: mov edx, 8
1412c3b0a: call 0x140613c10
1412c3b0f: test rax, rax
1412c3b12: je 0x1412c3b73
1412c3b14: movups xmm0, xmmword ptr [rbp + 0x78]
1412c3b18: movups xmm1, xmmword ptr [rbp + 0x88]
1412c3b1f: movups xmm2, xmmword ptr [rbp + 0x98]
1412c3b26: movups xmm3, xmmword ptr [rbp + 0xa8]
1412c3b2d: movups xmmword ptr [rax + 0x30], xmm3
1412c3b31: movups xmmword ptr [rax + 0x20], xmm2
1412c3b35: movups xmmword ptr [rax + 0x10], xmm1
1412c3b39: movups xmmword ptr [rax], xmm0
1412c3b3c: movabs rdx, 0x2bf3b1a4ac675864
1412c3b46: mov rcx, rsi
1412c3b49: mov r8, rax
1412c3b4c: call 0x140969bf0
1412c3b51: cmp rax, 1
1412c3b55: je 0x1412c3b84
1412c3b57: add rsp, 0x140
1412c3b5e: pop rdi
1412c3b5f: pop rsi
1412c3b60: pop rbp
1412c3b61: ret 
1412c3b62: mov ecx, 8
1412c3b67: mov edx, 0x90
1412c3b6c: call 0x1428d9000
1412c3b71: jmp 0x1412c3b82
1412c3b73: mov ecx, 8
1412c3b78: mov edx, 0x40
1412c3b7d: call 0x1428d9000
1412c3b82: ud2 
1412c3b84: mov rax, rdx
1412c3b87: add rdx, 0x10
1412c3b8b: mov rsi, rdx
1412c3b8e: cmp qword ptr [rax + 0x18], 0x2d
1412c3b93: jne 0x1412c3c22
1412c3b99: mov rdx, qword ptr [rsi]
1412c3b9c: lea rcx, [rip + 0x17daa19]
1412c3ba3: mov r8d, 0x2d
1412c3ba9: call 0x142923bf0
1412c3bae: test eax, eax
1412c3bb0: jne 0x1412c3c22
1412c3bb2: lea rax, [rip + 0x196a33f]
1412c3bb9: mov qword ptr [rbp + 0x38], rax
1412c3bbd: lea rax, [rip - 0xd90324]
1412c3bc4: mov qword ptr [rbp + 0x40], rax
1412c3bc8: lea rax, [rbp + 0x68]
1412c3bcc: mov qword ptr [rbp + 0x48], rax
1412c3bd0: lea rax, [rip - 0x11b9d47]
1412c3bd7: mov qword ptr [rbp + 0x50], rax
1412c3bdb: lea rax, [rip + 0x1969a46]
1412c3be2: mov qword ptr [rbp + 0x78], rax
1412c3be6: mov qword ptr [rbp + 0x80], 3
1412c3bf1: mov qword ptr [rbp + 0x98], 0
1412c3bfc: lea rax, [rbp + 0x38]
1412c3c00: mov qword ptr [rbp + 0x88], rax
1412c3c07: mov qword ptr [rbp + 0x90], 2
1412c3c12: lea rdx, [rip + 0x1969a3f]
1412c3c19: lea rcx, [rbp + 0x78]
1412c3c1d: call 0x1428d9390
1412c3c22: lea rax, [rip + 0x196a2cf]
1412c3c29: mov qword ptr [rbp + 0x78], rax
1412c3c2d: lea rax, [rip - 0xd90394]
1412c3c34: mov qword ptr [rbp + 0x80], rax
1412c3c3b: mov qword ptr [rbp + 0x88], rsi
1412c3c42: lea rax, [rip - 0x11b9db9]
1412c3c49: mov qword ptr [rbp + 0x90], rax
1412c3c50: lea rcx, [rbp + 0x68]
1412c3c54: mov qword ptr [rbp + 0x98], rcx
1412c3c5b: mov qword ptr [rbp + 0xa0], rax
1412c3c62: lea rax, [rip + 0x19698ff]
1412c3c69: mov qword ptr [rbp + 0x38], rax
1412c3c6d: mov qword ptr [rbp + 0x40], 3
1412c3c75: mov qword ptr [rbp + 0x58], 0
1412c3c7d: lea rax, [rbp + 0x78]
1412c3c81: mov qword ptr [rbp + 0x48], rax
1412c3c85: mov qword ptr [rbp + 0x50], 3
1412c3c8d: lea rdx, [rip + 0x196993c]
1412c3c94: lea rcx, [rbp + 0x38]
1412c3c98: call 0x1428d9390
1412c3c9d: int3 
1412c3c9e: nop 
1412c3ca0: mov qword ptr [rsp + 0x10], rdx
1412c3ca5: push rbp
1412c3ca6: push rsi
1412c3ca7: push rdi
1412c3ca8: sub rsp, 0x20
1412c3cac: lea rbp, [rdx + 0x80]
1412c3cb3: lea rcx, [rbp - 0x58]
1412c3cb7: call 0x14087e420
1412c3cbc: nop 
1412c3cbd: add rsp, 0x20
1412c3cc1: pop rdi
1412c3cc2: pop rsi
1412c3cc3: pop rbp
1412c3cc4: ret 
1412c3cc5: nop word ptr cs:[rax + rax]
1412c3cd0: mov qword ptr [rsp + 0x10], rdx
1412c3cd5: push rbp
1412c3cd6: push rsi
1412c3cd7: push rdi
1412c3cd8: sub rsp, 0x20
1412c3cdc: lea rbp, [rdx + 0x80]
1412c3ce3: lea rcx, [rbp + 0x78]
1412c3ce7: call 0x1400c04b0
1412c3cec: nop 
1412c3ced: add rsp, 0x20
1412c3cf1: pop rdi
1412c3cf2: pop rsi
1412c3cf3: pop rbp
1412c3cf4: ret 
